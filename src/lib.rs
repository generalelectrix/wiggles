#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate petgraph;
extern crate itertools;
mod utils;
mod knob;
mod datatypes;
mod clock_network;
mod clocks;
mod data_network;
mod event;
mod interconnect;
mod master;