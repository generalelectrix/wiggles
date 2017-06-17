//! The core structure of the console server, with the explicit message types and application
//! logic abstracted away behind traits and type parameters.

extern crate event_loop;
extern crate smallvec;
extern crate serde;

use event_loop::{EventLoop, Event};
use std::error::Error;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use smallvec::SmallVec;
use serde::{Serialize, Deserialize};

struct LoadData;

/// Outer command wrapper for the reactor, exposing administrative commands on top of the internal
/// commands that the console itself provides.  Quitting the console, saving the show, and loading
/// a different show are all considered top-level commands, as they require swapping out the state
/// of the reactor.
pub enum Command<C> {
    /// Load a different show, using a metadata type that is yet to be determined.
    /// This may end up being a true path, or some kind of internal identifier.
    Load(LoadData),
    /// Quit the console, cleanly closing down every running thread.
    Quit,
    /// A message to be passed into the console logic running in the reactor.
    Console(C),
}

/// Outer command wrapper for a response from the reactor, exposing messages indicating
/// that administrative actions have occurred, as well as passing on messages from the console
/// logic running in the reactor.
pub enum Response<R, LE> {
    /// A new show was loaded, with this name.
    Loaded(String),
    /// Show load failed.
    LoadFailed(LE: Error),
    /// The console is going to quit.
    Quit,
    /// A response emanting from the console itself.
    Console(R),
}

/// Small vector optimization for zero or 1 messages; console logic should use this type to return
/// response messages.  May be removed if it turns out that consoles basically only ever emit one
/// response message per command message.
pub type Messages<T> = SmallVec<[T; 1]>;

/// Console logic must implement this trait to be run in the Wiggles reactor.
/// Note that none of these methods return Result; consoles are expected to be unconditionally
/// stable as far as the reactor is concerned.  If they need to indicate expected/safe errors, that
/// should be done in-band as part of the Response type.
pub trait Console<'de>: Serialize + Deserialize<'de> {
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
pub struct Reactor<'de, C, LE>
    where C: Console<'de>, LE: Error
        {
    console: C,
    event_source: EventLoop,
    cmd_queue: Receiver<Command<C::Command>>,
    resp_queue: Sender<Response<C::Response, LE>>,
}

// impl<Cmd, Resp, Save, Load, C> Reactor<Cmd, Resp, Save, Load, C>
//     where C: Console 
//         {
//     /// Run the console reactor.
//     pub fn run(&mut self) {
//         loop {
//             match self.event_source.next() {
//                 Event::Idle(dt) => {
//                     // we have dt until the next scheduled event.  use it to handle commands.
//                     match self.
//                 }
//             }
//         }
//     }

//     fn poll_command(&mut self) -> 
// }