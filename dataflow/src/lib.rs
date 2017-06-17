#![allow(dead_code)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate petgraph;
extern crate itertools;
extern crate wiggles_value;
mod utils;
mod knob;
mod datatypes;
mod clock_network;
mod clocks;
mod data_network;
mod event;
mod interconnect;
mod master;
mod network;