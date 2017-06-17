//! An event loop for lighting controllers.
//! Clients provide structs implementing the required traits and the event loop then drives the
//! controller by calling the appropriate trait methods.
//! Provides debug-level logging for tracing event flow.
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate log;
extern crate smallvec;

use std::thread::sleep;
use std::time::{Duration, Instant};
use std::cmp;
use std::error::Error;
use smallvec::SmallVec;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// Event loop settings.
pub struct Settings {
    /// Absolute fixed state update rate in fps.
    updates_per_second: u64,
    /// Maximum render rate in fps.
    renders_per_second: u64,
    /// Command the application to autosave at this interval (ms).
    autosave_interval: u64,
}

/// Small vector optimization for output messages, ensuring the most common cases of 0 and 1
/// output messages require no allocation.
type OutputMessages<T> = SmallVec<[T; 1]>;

/// Interface provided to the event loop by the show core.
/// The fact that these methods all return events and not Results implies that the show's
/// implementation of these actions must be infalliable, and should report failures via output
/// messages to be communicated back out to other layers of the stack.
pub trait Actor {
    /// Used by the show to define the commands it accepts.
    type InputMessage;
    /// Returned by the show as a result of updating, rendering, or handling user input.
    type OutputMessage;

    /// Render the current state of the show.
    fn render(
        &mut self,
        frame_number: u64,
        time_since_update: Duration)
        -> OutputMessages<Self::OutputMessage>;

    /// Update the current state of the show.
    fn update(
        &mut self,
        dt: Duration)
        -> OutputMessages<Self::OutputMessage>;

    /// Handle a command message.
    fn handle_command(
        &mut self,
        command: Self::InputMessage)
        -> OutputMessages<Self::OutputMessage>;
}

