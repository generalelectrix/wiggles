//! Tests of individual clock modules.
use knob::KnobValue;
use update::Update;
use clock_network::{ClockNetwork, ClockNodePrototype, ClockValue};
use super::*;
use super::basic::INIT_RATE;
use super::multiplier::MULT_KNOB_ID;

#[test]
fn test_basic_clock() {
    let mut network = ClockNetwork::new();
    let proto = Clock::create_prototype();
    let node_id = network.add_node(&proto, "test".to_string(), &[]).unwrap().id;

    assert!(INIT_RATE.in_hz() == 1.0);

    let update_and_check = |interval, (float_val, ticked)| {
        network.update(interval);
        let res = network.get_value_from_node(node_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(res);
    };

    update_and_check(0.75, (0.75, false));
    update_and_check(0.75, (1.5, true));
    update_and_check(0.1, (1.6, false));
    update_and_check(0.0, (1.6, false));
}

#[test]
fn test_multiplier() {
    let mut network = ClockNetwork::new();
    let (bp, mp) = (Clock::create_prototype(), ClockMultiplier::create_prototype());
    let basic_id = network.add_node(&bp, "basic".to_string(), &[]).unwrap().id;
    let mult_id = network.add_node(&mp, "mult".to_string(), vec!(basic_id)).unwrap().id;
    
    let set_mult_factor = |val| {
        network.get_node_mut(mult_id).unwrap()
            .set_knob_value(MULT_KNOB_ID, KnobValue::PositiveFloat(val)).unwrap();
    };

    let dt = 0.75;

    let update_and_check = |interval, (float_val, ticked)| {
        network.update(interval);
        let res = network.get_value_from_node(mult_id).unwrap();
        ClockValue::from_float_val(float_val, ticked).assert_almost_eq_to(res);
    };

    // initially should just track upstream clock
    update_and_check(dt, (0.75, false));
    update_and_check(dt, (1.5, true));

    // dip in and set the mult factor to 2.0
    set_mult_factor(2.0);

    update_and_check(dt, (2.25, true));
    update_and_check(dt, (3.75, true));

    // now run half-speed
    set_mult_factor(0.5);
    update_and_check(0.4, (3.95, false));
    update_and_check(0.4, (4.15, true));
}