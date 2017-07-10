extern crate ordermap;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate wiggles_value;
extern crate console_server;
#[macro_use] extern crate lazy_static;
extern crate serde_json;
extern crate waveforms;
#[cfg(test)] extern crate simple_logger;

pub mod network;
pub mod clocks;
pub mod wiggles;
mod util;
mod test;