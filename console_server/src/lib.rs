//! The core structure of the console server, with the explicit message types and application
//! logic abstracted away behind traits and type parameters.
mod show_library;
mod reactor;

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

pub use reactor::*;
pub use show_library::*;