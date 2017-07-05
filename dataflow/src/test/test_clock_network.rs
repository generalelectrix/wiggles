//! Tests for the clock network.
//! Primarily here to ensure all necessary traits are implemented.
use std::fmt;
use std::time::Duration;
use network::Network;
use clocks::simple::SimpleClock;
use clocks::multiplier::ClockMultiplier;
use clocks::clock::{
    ClockNetwork,
    CompleteClock,
    ClockId,
    KnobAddr,
    Message,
    ClockKnobAddr,
    ClockValue,
    ClockProvider,
    ClockCollection,
};
use serde_json;

fn box_clock<T: 'static + CompleteClock>(t: T) -> Box<CompleteClock> {
    Box::new(t)
}

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

    // Run an update step that updates by 1/10th of a second.
    // No messages should have been emitted.
    let messages = network.update(Duration::from_millis(100));
    assert_eq!(0, messages.len());

    // Simple clock should have advanced 1/10th of a period, same with multiplier.
    let simple_val = network.get_value(simple_id);
    assert_eq!(ClockValue::from_float_value(0.1, false), simple_val);

    let mult_val = network.get_value(mult_id);
    assert_eq!(ClockValue::from_float_value(0.1, false), mult_val);

    // make sure we can serialize and deserialize
    let ser_net = serde_json::to_string(&network).unwrap();
    let de_net: ClockNetwork = serde_json::from_str(&ser_net).unwrap();
    assert_eq!(network, de_net);
}
