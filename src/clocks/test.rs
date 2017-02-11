//! Tests of individual clock modules.
use knob::{KnobValue, Knobs};
use update::Update;
use clock_network::{ClockNetwork, ClockNodePrototype, ClockValue, ClockNode};
use super::*;
use super::basic::INIT_RATE;
use super::multiplier::MULT_KNOB_ID;

#[test]
fn test_basic_clock() {
    let mut network = ClockNetwork::new();
    let proto = Clock::create_prototype();
    let node_id = network.add_node(&proto, "test".to_string(), &[]).unwrap().index();

    assert!(INIT_RATE.in_hz() == 1.0);

    let mut update_and_check = |interval, (float_val, ticked)| {
        network.update(interval);
        let res = network.get_value_from_node(node_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(&res);
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
    let basic_id = network.add_node(&bp, "basic".to_string(), &[]).unwrap().index();
    let mult_id = network.add_node(&mp, "mult".to_string(), &vec!(basic_id).into_boxed_slice()).unwrap().index();
    
    let set_mult_factor = |val, cn: &mut ClockNetwork| {
        let node: &mut ClockNode = cn.get_node_mut(mult_id).unwrap();
        node.set_knob_value(MULT_KNOB_ID, KnobValue::PositiveFloat(val)).unwrap();
    };

    let dt = 0.75;

    let update_and_check = |interval, (float_val, ticked), network: &mut ClockNetwork| {
        network.update(interval);
        let res = network.get_value_from_node(mult_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(&res);
    };

    // initially should just track upstream clock
    update_and_check(dt, (0.75, false), &mut network);
    update_and_check(dt, (1.5, true), &mut network);

    // dip in and set the mult factor to 2.0
    set_mult_factor(2.0, &mut network);

    update_and_check(dt, (3.0, true), &mut network);
    update_and_check(dt, (4.5, true), &mut network);

    // now run half-speed
    set_mult_factor(0.5, &mut network);
    update_and_check(0.4, (4.7, false), &mut network);
    update_and_check(0.4, (4.9, false), &mut network);
}