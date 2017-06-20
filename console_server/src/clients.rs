//! Logic for creating and managing control clients connected to a Wiggles controller.

use super::reactor::{CommandMessage, ResponseMessage, Command, Response};
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt;
use std::ops::DerefMut;
use ordermap::OrderMap;

pub type ClientId = u32;

pub type ClientCollection<R> = OrderMap<ClientId, Sender<Response<R>>>;

/// A structure processing and relaying messages to various clients.
/// TODO: consider techniques for only serializing each message once using a pre-registered hook,
/// and handing out pre-serialized messages to the clients rather than the in-memory representation.
/// Since we only have a few clients at a time, this is probably overkill.
struct ResponseRouter<R: Send> {
    message_source: Receiver<ResponseMessage<R>>,
    /// The collection of clients, indexed by their numeric ID.
    /// Options are allowed so we can re-use client IDs from clients that have disconnected,
    /// ensuring this collection doesn't grow too large over time.  The upshot is that we avoid
    /// paying any costs associated with storing this data as a map.
    /// An external service that creates new client connections holds onto a clone of this arc, so
    /// it can infrequently inject new clients.
    clients: Arc<Mutex<ClientCollection<R>>>,
    /// The response router will run as long as this is true.
    running: bool,
}

impl<R: Clone + fmt::Debug + Send> ResponseRouter<R> {
    /// Create a new response router which will drain the provided message queue.
    pub fn new(message_source: Receiver<ResponseMessage<R>>) -> Self {
        ResponseRouter {
            message_source: message_source,
            clients: Arc::new(Mutex::new(OrderMap::new())),
            running: false,
        }
    }

    /// Get a clone of the handle to the client collection.
    pub fn clients_handle(&self) -> Arc<Mutex<ClientCollection<R>>> {
        self.clients.clone()
    }

    /// Block until we acquire the lock on the clients collection.
    /// Return the guard.  If the lock is poisoned, abort and return an error.
    fn clients(&mut self) -> Result<MutexGuard<ClientCollection<R>>, ()> {
        if let Ok(guard) = self.clients.lock() {
            return Ok(guard);
        }
        // Someone else panicked while they were holding the mutex.  Gracefully terminate this router.
        error!("Response router client collection mutex is poisoned.  Aborting.");
        self.running = false;
        Err(())
    }

    /// Run the response router.
    pub fn run(&mut self) {
        info!("Response router is starting.");
        self.running = true;
        while self.running {
            self.run_once();
        }
        info!("Response router quit.")
    }

    /// Run one iteration of the response router's action loop.
    /// Return true if the router should continue or false if it should quit.
    fn run_once(&mut self) {
        // block on an incoming message
        match self.message_source.recv() {
            Err(_) => {
                // The event source hung up.  Abort.
                error!("Response router message source hung up.  Aborting.");
                self.running = false;
            }
            Ok(msg) => {
                debug!("Response router is handling a message: {:?}", msg);
                self.handle_message(msg);
            }
        }
    }

    /// Handle a single message, including filtering.
    /// Responds to the Quit message before passing it on.
    fn handle_message(&mut self, msg: ResponseMessage<R>) {
        if let Response::Quit = msg.payload  {
            info!("Response router received the Quit message.");
            self.running = false;
            self.send_to(Filter::All, Response::Quit);
        }
        else {
            // filter messages based on associated client data, if included.
            match msg.client_data {
                Some(client_data) => {
                    // apply fitering criteria

                },
                None => {
                    // forward to every client
                    self.send_to(Filter::All, msg.payload);
                },
            }
        }
    }

    /// Forward a message to some subset of clients.
    fn send_to(&mut self, filter: Filter, msg: Response<R>) {
        if let Ok(mut clients) = self.clients() {
            let mut dead_clients = Vec::new();

            match filter {
                Filter::All => {
                    for (client_id, sender) in clients.iter() {
                        // send the message, and keep track of any clients that have hung up
                        if let Err(_) = sender.send(msg.clone()) {
                            dead_clients.push(*client_id);
                        }
                    }
                },
                Filter::One(id) => {
                    clients.get(&id).map(|sender| {
                        if let Err(_) = sender.send(msg) {
                            dead_clients.push(id);
                        }
                    });
                }
                Filter::AllBut(id) => {
                    for (client_id, sender) in clients.iter() {
                        // Send the message to every other client.
                        if id != *client_id {
                            if let Err(_) = sender.send(msg.clone()) {
                                dead_clients.push(*client_id);
                            }
                        }
                    }
                }
            }

            // Remove dead clients from the collection.
            for dead_client in dead_clients {
                clients.remove(&dead_client);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Metadata attached to commands and optionally attached to responses.
/// Used for filtering response messages as they are send out to clients.
pub struct ClientData {
    /// The id of the client.
    pub id: ClientId,
    pub filter: ResponseFilter,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
/// Message metadata indicating the intended response filter for the result of processing this
/// message.
pub enum ResponseFilter {
    /// Response is of general interest.
    All,
    /// Response is only of interest to this client.
    Exclusive,
    /// Response is probably of interest to every client except the originator.  This option implies
    /// that the originator doesn't want talkback on these commands as it is probably eagerly
    /// updating some state locally and wants to avoid the nasty latency associated with the
    /// state update that will come with some latency.
    AllButSelf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Options for filtering messages sent out to clients.
enum Filter {
    /// Forward message to every client.
    All,
    /// Forward message to just this client.
    One(ClientId),
    /// Forward message to every client except this one.
    AllBut(ClientId),
}

impl From<ClientData> for Filter {
    fn from(data: ClientData) -> Self {
        match data.filter {
            ResponseFilter::All => Filter::All,
            ResponseFilter::Exclusive => Filter::One(data.id),
            ResponseFilter::AllButSelf => Filter::AllBut(data.id),
        }
    }
}

