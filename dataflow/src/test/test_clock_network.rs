//! Tests for the clock network.
//! Primarily here to ensure all necessary traits are implemented.
use std::fmt;
use network::Network;
use clocks::simple::SimpleClock;
use clocks::clock::{CompleteClock, ClockId, KnobAddr, Message, ClockKnobAddr};
use serde_json;

fn box_clock<T: 'static + CompleteClock>(t: T) -> Box<CompleteClock> {
    Box::new(t)
}

type ClockNetwork = Network<Box<CompleteClock>, ClockId, Message<ClockKnobAddr>>;

#[test]
fn test_construct_network() {
    let mut network: ClockNetwork = Network::new();
    let clock = SimpleClock::new("test");
    let boxed = box_clock(clock);
    network.add(boxed);

    // make sure we can serialize and deserialize
    let ser_net = serde_json::to_string(&network).unwrap();
    let de_net: ClockNetwork = serde_json::from_str(&ser_net).unwrap();
    assert_eq!(network, de_net);
}
