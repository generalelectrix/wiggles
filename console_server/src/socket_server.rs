
use std::thread;
use websocket::OwnedMessage;
use websocket::server::NoTlsAcceptor;
use websocket::WebSocketError;
use websocket::sync::{Client, Server};
use websocket::stream::sync::{TcpStream, Splittable};
use websocket::receiver::Reader;
use websocket::sender::Writer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::ToSocketAddrs;
use std::io::{Error as IoError, ErrorKind};
use std::iter::Iterator;
use super::reactor::{Command, Response, CommandMessage, CommandWrapper};
use super::clients::{ClientData, ClientCollection, ClientId, ResponseFilter};
use serde_json;


/// Owns a websocket server and the requisite information to spin off new clients with unique ids.
/// New clients are added to the collection of clients handled by the response message router.
/// These clients then serialize and deserialize the associated message types.
pub struct SocketServer<C: Send, R: Send> {
    /// The server accepting websocket requests.
    server: Server<NoTlsAcceptor>,
    /// The command queue used to send messages into the reactor.
    command_queue: Sender<CommandMessage<C>>,
    /// The registry of clients used by the response server to do response distribution.
    client_registry: Arc<Mutex<ClientCollection<R>>>,
    /// The protocol this console is running to ensure the wrong frontend client cannot connect.
    protocol: String,
    /// Monotonically increasing IDs to identify clients.
    next_client_id: ClientId,
}

impl<C: Send, R: Send> SocketServer<C, R>
    where R: 'static + Clone + Serialize + fmt::Debug, C: 'static + DeserializeOwned + fmt::Debug
{
    /// Initiate a new socket server.
    /// Requires an existing client registry and reactor command queue, ensuring the rest of the
    /// console machinery is wired up before we start allowing client connections.
    pub fn new<A: ToSocketAddrs, N: Into<String>>(
        client_registry: Arc<Mutex<ClientCollection<R>>>,
        command_queue: Sender<CommandMessage<C>>,
        addr: A,
        protocol: N)
        -> Result<Self, IoError>
    {
        let server = Server::bind(addr)?;
        Ok(SocketServer {
            server: server,
            command_queue: command_queue,
            client_registry: client_registry,
            protocol: protocol.into(),
            next_client_id: 0,
        })
    }

    /// Run this server, accepting requests and spinning off new client threads to handle them.
    /// Runs forever unless something goes wrong with the underlying socket.
    pub fn run(&mut self) {
        info!("Socket server is starting.");
        // TEMP: work around Rust #42852
        let mut cmd_send_clones = Vec::new();
        for _ in 0..10000 {
            cmd_send_clones.push(self.command_queue.clone());
        }
        loop {
            match self.server.next().unwrap() {
                // Log socket errors.
                Err(e) => error!("Socket request error: {}", e.error),
                Ok(request) => {
                    debug!("Client is requesting procotols {:?}.", request.protocols());
                    // make sure the client is running the right protocol.
                    if !request.protocols().contains(&self.protocol) {
                        request.reject();
                        continue;
                    }

                    // Accept the request and create a new websocket client.
                    match request.use_protocol(self.protocol.clone()).accept() {
                        Err(e) => error!("Error on websocket accept: {:?}", e),
                        Ok(client) => {
                            self.new_client(client, &mut cmd_send_clones);
                        }
                    }
                }
            }
        }
    }

    fn register_client_with_router(&self, id: ClientId, resp_sender: Sender<Response<R>>) {
        match self.client_registry.lock() {
            Ok(mut cr) => {
                cr.insert(id, resp_sender);
            }
            Err(_) => {
                // the response router panicked while holding the lock.
                // this should never happen.  We'll log and panic ourselves.
                error!("Client registry lock is poisoned!  Socket server will panic.");
                panic!("Client registry lock is poisoned!");
            }
        }
    }
    
    fn new_client(&mut self, client: Client<TcpStream>, cmd_clones: &mut Vec<Sender<CommandMessage<C>>>) {
        // Generate a new client ID for this client.
        let id = self.next_client_id;
        self.next_client_id += 1;

        client.peer_addr().map(|ip| info!("Websocket connection from {}", ip));

        // Split the client into constituent sender and receiver.
        match client.split() {
            Err(e) => error!("Could not split new websocket client: {}", e),
            Ok((receiver, sender)) => {
                // Create a new channel for the response messages.
                let (resp_send, resp_recv) = channel();
                // Register this new client with the response router.
                self.register_client_with_router(id, resp_send);
                // Get a copy of the reactor command sender.
                let cmd_queue = cmd_clones.pop().expect("Ran out of command clones.");
                // Start the sender and receiver in new threads.
                thread::spawn(move || run_client_receiver(receiver, cmd_queue, id));
                thread::spawn(move || run_client_sender(sender, resp_recv, id));
            }
        }
    }
}

/// Deserialize messages from this client and forward them to the reactor.
fn run_client_receiver<C: DeserializeOwned + fmt::Debug + Send>(
    mut receiver: Reader<<TcpStream as Splittable>::Reader>,
    command_queue: Sender<CommandMessage<C>>,
    id: ClientId)
{
    debug!("Client {} receiver is starting.", id);
    for message in receiver.incoming_messages().filter_map(Result::ok) {
        match message {
            OwnedMessage::Close(_) => {
                info!("Client {} disconnected.", id);
                return;
            }
            OwnedMessage::Text(m) => {
                debug!("Received message from client {}: {}", id, m);
                match serde_json::from_str::<(ResponseFilter, Command<C>)>(&m) {
                    Ok((filter, msg)) => {
                        debug!("Deserialized message from client {}: {:?}", id, msg);
                        // Construct the client data and the command wrapper and send them to the
                        // reactor.
                        let client_data = ClientData {
                            id: id,
                            filter: filter,
                        };
                        let cmd = CommandWrapper {
                            client_data: client_data,
                            payload: msg,
                        };
                        command_queue.send(cmd);
                    }
                    Err(e) => error!("Deserialization error on client {}: {}", id, e),
                }
            }
            x => {
                error!("Incomprehensible message from client: {:?}", x);
            }
        }
    }
}

// TODO: consider locally batching messages over a short period of time to avoid thrashing the
// TCP connection with lots of tiny messages.
/// Serialize and send messages to this client.
fn run_client_sender<R: Serialize + fmt::Debug + Clone + Send>(
    mut sender: Writer<<TcpStream as Splittable>::Writer>,
    message_queue: Receiver<Response<R>>,
    id: ClientId)
{
    debug!("Client {} sender is starting.", id);
    for msg in message_queue.iter() {
        debug!("Client {} is sending a message: {:?}", id, msg);
        match serde_json::to_string(&msg) {
            Err(e) => {
                error!(
                    "Message serialization error, client {}, trying to serialize {:?}: {}",
                    id,
                    msg,
                    e)
            }
            Ok(json) => {
                if let Err(e) = sender.send_message(&OwnedMessage::Text(json)) {
                    match e {
                        WebSocketError::IoError(ref e) if e.kind() == ErrorKind::BrokenPipe => {
                            // Close this client.
                            break;
                        }
                        _ => {
                            error!("Error sending message to client {}: {}\nMessage: {:?}", id, e, msg);
                        }
                    }
                    
                }
            }
        }
    }
    info!("Client {} websocket sender terminated.", id);
}