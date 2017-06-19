//! The core structure of a Wiggles console.
//! Agnostic about the actual logical console that is running inside it, as well as the types of
//! control clients that are connected to it.
//! Provides a core show reactor running a lightweight event loop which can save and load shows and
//! drives the console logic itself which is hidden behind the Console trait.
mod show_library;
mod reactor;
mod clients;

extern crate event_loop;
extern crate smallvec;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate chrono;
extern crate ordermap;
#[macro_use] extern crate log;

#[cfg(test)] extern crate simple_logger;
#[cfg(test)] extern crate rand;
#[cfg(test)] #[macro_use] extern crate serde_derive;

pub use reactor::*;
pub use show_library::*;
pub use clients::*;