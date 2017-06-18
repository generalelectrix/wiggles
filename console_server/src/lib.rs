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

use std::path::PathBuf;
use event_loop::{EventLoop, Event};
use std::error::Error;
use std::sync::mpsc::{channel, Sender, Receiver, RecvTimeoutError};
use std::time::Duration;
use smallvec::SmallVec;
use serde::Serialize;
use serde::de::DeserializeOwned;

use show_library::{ShowLibrary, LibraryError, LoadShow};

/// Outer command wrapper for the reactor, exposing administrative commands on top of the internal
/// commands that the console itself provides.  Quitting the console, saving the show, and loading
/// a different show are all considered top-level commands, as they require swapping out the state
/// of the reactor.
pub enum Command<C> {
    /// Load a show using this spec.
    Load(LoadShow),
    /// Save the current state of the show.
    Save,
    /// Quit the console, cleanly closing down every running thread.
    Quit,
    /// A message to be passed into the console logic running in the reactor.
    Console(C),
}

impl<C> From<C> for Command<C> {
    fn from(msg: C) -> Self {
        Command::Console(msg)
    }
}

/// Outer command wrapper for a response from the reactor, exposing messages indicating
/// that administrative actions have occurred, as well as passing on messages from the console
/// logic running in the reactor.
pub enum Response<R> {
    /// A new show was loaded, with this name.
    Loaded(String),
    /// The show was saved successfully.
    Saved,
    /// A show library error occurred.
    ShowLibErr(LibraryError),
    /// The console is going to quit.
    Quit,
    /// A response emanting from the console itself.
    Console(R),
}

impl<R> From<R> for Response<R> {
    fn from(msg: R) -> Self {
        Response::Console(msg)
    }
}

/// Small vector optimization for zero or 1 messages; console logic should use this type to return
/// response messages.
type MessagesInner<T> = [T; 1];
pub struct Messages<T>(SmallVec<MessagesInner<T>>);

impl<T> Messages<T> {
    fn single(m: T) -> Self {
        let mut msgs = SmallVec::new();
        msgs.push(m);
        Messages(msgs)
    }

    /// Empty message collection.
    pub fn none() -> Self {
        Messages(SmallVec::new())
    }
    
    /// Add a message to this collection.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// A collection that contains one message.
    pub fn one<M: Into<T>>(msg: M) -> Self {
        Messages::single(msg.into())
    }

    /// Convert a message collection that can be interpreted as this type.
    pub fn wrap<M: Into<T>>(mut msgs: Messages<M>) -> Self {
        Messages(msgs.0.drain().map(|m| m.into()).collect())
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
    fn render(&mut self) -> Messages<Self::Response>;

    /// Update the show state, potentially emitting messages.
    fn update(&mut self, dt: Duration) -> Messages<Self::Response>;

    /// Handle a command, probably emitting messages.
    fn handle_command(&mut self, command: Self::Command) -> Messages<Self::Response>;
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
    /// The path where this console is storing its saved shows.
    library_path: PathBuf,
    /// The on-disk library of saved states for this show.
    show_lib: ShowLibrary,
    /// The actual core show logic.
    console: C,
    /// 
    event_source: EventLoop,
    cmd_queue: Receiver<Command<C::Command>>,
    resp_queue: Sender<Response<C::Response>>,
    quit: bool,
}

impl<C> Reactor<C>
    where C: Console
        {
    /// Run the console reactor.
    pub fn run(&mut self) {
        'event: loop {
            if self.quit {
                info!("Reactor is quitting.");
                break 'event;
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
                        break 'event;
                    }
                }
            }
        }
    }

    fn poll_command(&mut self, dt: Duration) -> Messages<Response<C::Response>> {
        // we have dt until the next scheduled event.
        // block until we get a command or we time out.
        match self.cmd_queue.recv_timeout(dt) {
            Ok(Command::Console(msg)) => {
                Messages::wrap(self.console.handle_command(msg))
            },
            Ok(Command::Quit) => {
                debug!("Reactor received the quit command.");
                self.quit = true;
                Messages::one(Response::Quit)
            },
            Ok(Command::Save) => {
                match self.save() {
                    Ok(()) => Messages::one(Response::Saved),
                    Err(e) => Messages::one(Response::ShowLibErr(e)),
                }
            }
            Ok(Command::Load(l)) => {
                Messages::one(self.load_show(l))
            },
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
            .map_err(|e| {
                error!("Show save failed: {}", e);
                e
            })
    }

    fn load_show(&mut self, l: LoadShow) -> Response<C::Response> {
        debug!("Reactor is loading a new show: {}", l);
        match ShowLibrary::open_existing(&self.library_path, l.name.as_str()) {
            Err(e) => Response::ShowLibErr(e),
            Ok(show_lib) => {
                // try to load the save specified by the spec
                match show_lib.load(&l.spec) {
                    Err(e) => Response::ShowLibErr(e),
                    Ok(console) => {
                        // we successfully loaded the new show
                        // save our existing show, swap the console state out, and reset the state
                        // of the event loop
                        self.autosave();
                        self.save();
                        self.console = console;
                        self.event_source.reset();
                        Response::Loaded(l.name)
                    }
                }
            }
        }

    }
}