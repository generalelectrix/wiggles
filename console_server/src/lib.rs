//! The core structure of the console server, with the explicit message types and application
//! logic abstracted away behind traits and type parameters.
mod show_library;

extern crate event_loop;
extern crate smallvec;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate chrono;
#[macro_use] extern crate log;

#[cfg(test)] extern crate simple_logger;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate serde_derive;

use std::path::PathBuf;
use event_loop::{EventLoop, Event, Settings};
use std::error::Error;
use std::sync::mpsc::{channel, Sender, Receiver, RecvTimeoutError};
use std::time::Duration;
use smallvec::SmallVec;
use serde::Serialize;
use serde::de::DeserializeOwned;

use show_library::{ShowLibrary, LibraryError, LoadShow, LoadSpec};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Metadata attached to commands and optionally attached to responses.
/// Used for filtering response messages as they are send out to clients.
pub struct ClientData {
    /// The id of the client.
    id: u32,
    /// If false, the client does not want to be informed about happy-path messages related to this
    /// one.  This helps avoid jittery sliders due to the round-trip latency from and back to the UI.
    talkback: bool,
    /// If true, the client is suggesting that the result of this command or response is only of
    /// interest to it.  The server may or may not respect this option.
    exclusive: bool,
}

#[derive(Debug)]
/// Outer command wrapper for the reactor, exposing administrative commands on top of the internal
/// commands that the console itself provides.  Quitting the console, saving the show, and loading
/// a different show are all considered top-level commands, as they require swapping out the state
/// of the reactor.
pub enum Command<T> {
    /// Create a new, empty show.
    NewShow(String),
    /// List all available saves and autosaves.
    AvailableSaves,
    /// Load a show using this spec.
    Load(LoadShow),
    /// Save the current state of the show.
    Save,
    /// Save the current show as a new show with a different name.  This show will become the one
    /// running in the reactor.
    SaveAs(String),
    /// Change the name of the currently-running show.  This will move all of the files in the
    /// saved show library.
    Rename(String),
    /// Quit the console, cleanly closing down every running thread.
    Quit,
    /// A message to be passed into the console logic running in the reactor.
    Console(T),
}

impl<T> From<T> for Command<T> {
    fn from(msg: T) -> Self {
        Command::Console(msg)
    }
}

/// Message type with client data.
pub struct CommandWrapper<T> {
    client_data: ClientData,
    payload: T,
}

/// The message type received by the reactor.
type CommandMessage<T> = CommandWrapper<Command<T>>;

/// Outer command wrapper for a response from the reactor, exposing messages indicating
/// that administrative actions have occurred, as well as passing on messages from the console
/// logic running in the reactor.
pub enum Response<T> {
    /// A listing of all available save and autosave files for the running show.
    SavesAvailable{saves: Vec<String>, autosaves: Vec<String>},
    /// A new show was loaded, with this name.
    Loaded(String),
    /// The running show's name changed.
    Renamed(String),
    /// The show was saved successfully.
    Saved,
    /// A show library error occurred.
    ShowLibErr(LibraryError),
    /// The console is going to quit.
    Quit,
    /// A response emanting from the console itself.
    Console(T),
}

impl<T> From<T> for Response<T> {
    fn from(msg: T) -> Self {
        Response::Console(msg)
    }
}

/// Message type for outgoing messages.  Has optional client data.
pub struct ResponseWrapper<T> {
    client_data: Option<ClientData>,
    payload: T,
}

/// The message type sent out by the reactor.
type ResponseMessage<T> = ResponseWrapper<Response<T>>;

/// Small vector optimization for zero or 1 messages; console logic should use this type to return
/// response messages.
pub struct Messages<T>(SmallVec<[T; 1]>);

impl<T> Messages<T> {
    fn one(m: T) -> Self {
        let mut msgs = SmallVec::new();
        msgs.push(m);
        Messages(msgs)
    }

    /// Empty message collection.
    pub fn none() -> Self {
        Messages(SmallVec::new())
    }
    
    /// Add a message to this collection.
    pub fn push(&mut self, item:T) {
        self.0.push(item);
    }

    /// Convert a message collection that can be interpreted as this type.
    pub fn wrap<M: Into<T>>(mut msgs: Messages<M>) -> Messages<T> {
        Messages(
            msgs.0.drain()
                .map(|m| m.into())
                .collect())
    }
}

/// Console logic must implement this trait to be run in the Wiggles reactor.
/// Note that none of these methods return Result; consoles are expected to be unconditionally
/// stable as far as the reactor is concerned.  If they need to indicate expected/safe errors, that
/// should be done in-band as part of the Response type.
pub trait Console: Serialize + DeserializeOwned {
    /// The native command message type used by this console.
    type Command;
    /// The native response message type used by this console.
    type Response;
    /// Render a show frame, potentially emitting messages.
    fn render(&mut self) -> Messages<ResponseWrapper<Self::Response>>;

    /// Update the show state, potentially emitting messages.
    fn update(&mut self, dt: Duration) -> Messages<ResponseWrapper<Self::Response>>;

    /// Handle a command, probably emitting messages.
    fn handle_command(&mut self, command: Self::Command) -> Messages<ResponseWrapper<Self::Response>>;
}

/// The heart of the console.
/// Owns core data such as the fixture patch, dataflow logic, control mappings, etc.
/// Runs an event loop that alternately processes UI commands, updates model state,
/// renders the state of the show, and potentially keeps a store of autosaved data to ensure the
/// console can recover from a crash without total disaster (even if you never remembered to hit
/// save).
pub struct Reactor<C>
    where C: Console
        {
    /// Stored constructor that the reactor can use to create a new, empty show.
    console_constructor: Box<Fn()->C>,
    /// The path where this console is storing its saved shows.
    library_path: PathBuf,
    /// The on-disk library of saved states for this show.
    show_lib: ShowLibrary,
    /// The actual core show logic.
    console: C,
    /// The source of show events, polled by the reactor.
    event_source: EventLoop,
    /// Channel on which the reactor will receive commands.
    cmd_queue: Receiver<CommandMessage<C::Command>>,
    /// Channel on which the reactor will send responses to commands or spontaneously emit messages.
    resp_queue: Sender<ResponseMessage<C::Response>>,
    /// If true, the reactor loop will exit at the start of its next iteration.
    quit: bool,
}

type InitializedReactor<C: Console> = (
    Reactor<C>,
    Sender<Message<Command<C::Command>>>,
    Receiver<Message<Response<C::Response>>>,
);

impl<C> Reactor<C>
    where C: Console
        {
    /// Create a new reactor running the specified show.
    /// Optionally provide overridden settings for the event loop.
    pub fn new(
        console_constructor: Box<Fn()->C>,
        library_path: PathBuf,
        show_library: ShowLibrary,
        load_spec: LoadSpec,
        event_settings: Option<Settings>)
        -> Result<InitializedReactor<C>, LibraryError>
    {
        // make sure we can load the provided show
        let console = show_library.load(load_spec)?;

        // initialize message channels
        let (cmd_send, cmd_recv) = channel();
        let (resp_send, resp_recv) = channel();
        let mut event_source = EventLoop::new();
        if let Some(settings) = event_settings {
            event_source.settings = settings;
        }
        let reactor = Reactor {
            console_constructor: console_constructor,
            library_path: library_path,
            show_lib: show_library,
            console: console,
            event_source: event_source,
            cmd_queue: cmd_recv,
            resp_queue: resp_send,
            quit: false,
        };
        Ok((reactor, cmd_send, resp_recv))
    }

    /// Run the console reactor.
    pub fn run(&mut self) {
        let mut running = true;
        info!("Console reactor is starting.");
        while running {
            running = self.run_one_iteration();
        }
        info!("Console reactor quit.");
    }

    /// Run one iteration of the event loop.
    /// Return true if the loop should run another iteration, or false if we should break.
    fn run_one_iteration(&mut self) -> bool {
        if self.quit {
            info!("Reactor is quitting.");
            return false;
        }
        let msgs = match self.event_source.next() {
            Event::Idle(dt) => {
                self.poll_command(dt)
            },
            Event::Autosave => {
                self.autosave();
                Messages::none()
            },
            Event::Render => { 
                Messages::wrap(self.console.render())
            },
            Event::Update(dt) => {
                Messages::wrap(self.console.update(dt))
            },
        };
        for msg in msgs.0 {
            match self.resp_queue.send(msg) {
                Ok(_) => (),
                Err(_) => {
                    // The response sink hung up.
                    // This should only be able to happen if the control server panicked.
                    // Not much we can do here except autosave and quit.
                    error!("Console response sink hung up.");
                    self.abort();
                    return false;
                }
            }
        }
        true
    }

    fn poll_command(&mut self, dt: Duration) -> Messages<Response<C::Response>> {
        // we have dt until the next scheduled event.
        // block until we get a command or we time out.
        match self.cmd_queue.recv_timeout(dt) {
            Err(RecvTimeoutError::Timeout) => {
                Messages::none()
            },
            Err(RecvTimeoutError::Disconnected) => {
                // The event stream went away.
                // The only way this should be able to happen is if the http server crashed.
                // TODO: attempt to restart a fresh http server thread to continue running the show.
                error!("Console event source hung up.");
                self.abort()
            },
            Ok(message) => self.handle_command(message),
        }
    }

    fn handle_command(&mut self, command: Message<Command<C::Command>>) -> Messages<Response<C::Response>> {
        let client_options = command.client_options;
        match command.payload {
            Command::Console(msg) => {
                Messages::wrap(self.console.handle_command(msg))
            },
            Command::Quit => {
                debug!("Reactor received the quit command.");
                self.quit = true;
                Messages::one(Response::Quit)
            },
            Command::AvailableSaves => {
                debug!("Getting a listing of available saved show states.");
                let saves = self.show_lib.saves().unwrap_or(Vec::new());
                let autosaves = self.show_lib.autosaves().unwrap_or(Vec::new());
                Messages::one(Response::SavesAvailable{saves: saves, autosaves: autosaves})
            }
            Command::Save => {
                info!("Saving show.");
                match self.save() {
                    Ok(()) => Messages::one(Response::Saved),
                    Err(e) => Messages::one(Response::ShowLibErr(e)),
                }
            }
            Command::SaveAs(name) => {
                match ShowLibrary::create_new(&self.library_path, name.clone(), &self.console) {
                    Err(e) => Messages::one(Response::ShowLibErr(e)),
                    Ok(show_lib) => {
                        debug!("Saving show as '{}'.", name);
                        // make an autosave in our current name to be thorough
                        self.autosave();
                        // swap just our show lib, since save as doesn't change the state of the show
                        self.show_lib = show_lib;
                        Messages::one(Response::Renamed(name))
                    }
                }
            }
            Command::Rename(name) => {
                debug!("Renaming show as '{}'.", name);
                match self.show_lib.rename(name.clone()) {
                    Ok(()) => Messages::one(Response::Renamed(name)),
                    Err(e) => Messages::one(Response::ShowLibErr(e)),
                }
            }
            Command::NewShow(name) => {
                debug!("Creating a new show.");
                Messages::one(self.new_show(name))
            }
            Command::Load(l) => {
                Messages::one(self.load_show(l))
            },
        }
    }

    /// If the console needs to crash because one of the other pieces of the application has
    /// panicked and we cannot recover, use this method to quit the event loop.
    /// Autosave, persist the state that the console has crashed, and then quit.
    /// Return a message we can send to the other parts of the application to instruct them to
    /// quit, though we may not be able to do anything with it.
    fn abort(&mut self) -> Messages<Response<C::Response>> {
        error!("Console is aborting!");
        self.autosave();
        self.quit = true;
        Messages::one(Response::Quit)
    }

    /// Save the current state of the show to a fast binary format.
    fn autosave(&self) {
        if let Err(e) = self.show_lib.autosave(&self.console) {
            error!("Autosave failed: {}", e);
        }
    }

    /// Save the current state of the show to a slow but human-readable format.
    fn save(&self) -> Result<(), LibraryError> {
        self.show_lib.save(&self.console)
    }

    /// Create a fresh, empty show, saved with this name.
    fn new_show(&mut self, name: String) -> Response<C::Response> {
        debug!("Reactor is creating a new show: {}.", name);
        let new_console = (self.console_constructor)();
        match ShowLibrary::create_new(&self.library_path, name, &new_console) {
            Err(e) => Response::ShowLibErr(e),
            Ok(show_lib) => {
                // swap the fresh show in
                self.swap_show(show_lib, new_console)
            }
        }
    }

    fn load_show(&mut self, l: LoadShow) -> Response<C::Response> {
        debug!("Reactor is loading a show: {}", l);
        match ShowLibrary::open_existing(&self.library_path, l.name) {
            Err(e) => Response::ShowLibErr(e),
            Ok(show_lib) => {
                // try to load the save specified by the spec
                match show_lib.load(l.spec) {
                    Err(e) => Response::ShowLibErr(e),
                    Ok(console) => {
                        // we successfully loaded the new show
                        // swap out the running show
                        self.swap_show(show_lib, console)
                    }
                }
            }
        }
    }

    /// Swap the show running in the reactor.  Save the running show before doing so.
    fn swap_show(&mut self, show_lib: ShowLibrary, console: C) -> Response<C::Response> {
        self.autosave();
        if let Err(e) = self.save() {
            error!("The running show failed to save before loading a new show.  Error: {}", e);
        }
        self.show_lib = show_lib;
        self.console = console;
        self.event_source.reset();
        Response::Loaded(self.show_lib.name().to_string())
    }
}