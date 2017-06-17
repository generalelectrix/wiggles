//! An event loop for lighting controllers.
//! Clients provide a struct implementing the required trais and the event loop then drives the
//! controller by calling the appropriate trait methods.
//! Provides debug-level logging for tracing event flow.
//! Uses channels for asynchronous command and response to control actions.

extern crate log;
extern crate smallvec;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::cmp;
use std::error::Error;
use std::sync::mpsc::{channel, Sender, Receiver};
//use std::iter::Iterator;
use smallvec::SmallVec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Event loop settings.
pub struct Settings {
    /// Absolute fixed state update rate in fps.
    pub updates_per_second: u64,
    /// Maximum render rate in fps.
    pub renders_per_second: u64,
    /// Command the application to autosave at this interval (ms).
    /// Setting this to 0 disables autosave.
    pub autosave_interval: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            // update at 100 fps
            updates_per_second: 100,
            // DMX is limited to 50 fps max
            renders_per_second: 50,
            // By default do not autosave.
            autosave_interval: 0,
        }
    }
}

/// Small vector optimization for output messages, ensuring the most common cases of 0 and 1
/// output messages require no allocation.
type Responses<T> = SmallVec<[T; 1]>;

/// Interface provided to the event loop by the show core.
/// The fact that these methods all return events and not Results implies that the show's
/// implementation of these actions must be infalliable, and should report failures via output
/// messages to be communicated back out to other layers of the stack.
pub trait Actor {
    /// Used by the show to define the commands it accepts.
    type Command;
    /// Returned by the show as a result of updating, rendering, or handling user input.
    type Response;

    /// Render the current state of the show.
    fn render(
        &mut self,
        frame_number: u64,
        time_since_update: Duration)
        -> Responses<Self::Response>;

    /// Update the current state of the show.
    fn update(
        &mut self,
        dt: Duration)
        -> Responses<Self::Response>;

    /// Handle a command message.
    fn handle_command(
        &mut self,
        command: Self::Command)
        -> Responses<Self::Response>;
}

pub struct EventLoop<A: Actor> {
    pub settings: Settings,
    actor: A,
    // Channel ends for sending and receiving messages.
    cmd_send: Sender<A::Command>,
    cmd_recv: Receiver<A::Command>,
    resp_recv: Receiver<A::Response>,
    resp_send: Sender<A::Response>,
    
}

impl<A: Actor> EventLoop<A> {
    /// Instantiate the event loop.
    pub fn new(actor: A) -> Self {
        let (cmd_send, cmd_recv) = channel();
        let (resp_send, resp_recv) = channel();
        EventLoop {
            settings: Settings::default(),
            actor: actor,
            cmd_send: cmd_send,
            cmd_recv: cmd_recv,
            resp_recv: resp_recv,
            resp_send: resp_send,
        }
    }

    /// Get a clone of the command sender channel.
    pub fn command_sender(&self) -> Sender<A::Command> {

    }
}