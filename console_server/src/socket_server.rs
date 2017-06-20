
use std::thread;
use websocket::OwnedMessage;
use websocket::server::NoTlsAcceptor;
use websocket::server::upgrade::WsUpgrade;
use websocket::sync::{Client, Server};
use websocket::stream::sync::{TcpStream, Splittable};
use websocket::receiver::Reader;
use websocket::sender::Writer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::net::ToSocketAddrs;
use std::io::Error as IoError;
use super::reactor::{Command, Response, CommandMessage, CommandWrapper};
use super::clients::{ClientData, ClientCollection, ClientId, ResponseFilter};
use serde_json;


/// Owns a websocket server and the requisite information to spin off new clients with unique ids.
/// New clients are added to the collection of clients handled by the response message router.
/// These clients then serialize and deserialize the associated message types.
struct SocketServer<C, R>
    where R: Clone + Serialize + fmt::Debug, C: DeserializeOwned + fmt::Debug
{
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

impl<C, R> SocketServer<C, R>
    where R: Clone + Serialize + fmt::Debug, C: DeserializeOwned + fmt::Debug
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
    fn run(&mut self) {
        info!("Socket server is starting.");
        loop {
            for req in self.server {
                match req {
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
                                self.new_client(client);
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn new_client(&mut self, client: Client<TcpStream>) {
        // Generate a new client ID for this client.
        let id = self.next_client_id;
        self.next_client_id += 1;

        client.peer_addr().map(|ip| info!("Websocket connection from {}", ip));

        // Split the client into constituent sender and receiver.
        match client.split() {
            Err(e) => error!("Could not split new websocket client: {}", e),
            Ok((mut receiver, mut sender)) => {

            }
        }
    }
}

/// Deserialize messages from this client and forward them to the reactor.
fn run_client_receiver<C: DeserializeOwned + fmt::Debug>(
    receiver: Reader<<TcpStream as Splittable>::Reader>,
    command_queue: Sender<CommandMessage<C>>,
    id: ClientId)
{
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