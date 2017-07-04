//! Tests for the clock network.
//! Primarily here to ensure all necessary traits are implemented.
use std::fmt;
use network::Network;
use clocks::simple::SimpleClock;
use clocks::multiplier::ClockMultiplier;
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
    let simple_id = {
        let (id, _) = network.add(boxed);
        id
    };
    

    // Add a multiplier linked to the simple clock we just added.
    let mult = ClockMultiplier::new("test mult");
    let boxed = box_clock(mult);
    let mult_id = {
        let (id, _) = network.add(boxed);
        id
    };

    // connect the multiplier's input
    network.swap_input(mult_id, 0, Some(simple_id)).unwrap();

    // make sure we can serialize and deserialize
    let ser_net = serde_json::to_string(&network).unwrap();
    let de_net: ClockNetwork = serde_json::from_str(&ser_net).unwrap();
    assert_eq!(network, de_net);
}
