//! Tests of individual clock modules.
use knob::{KnobValue, Knobs};
use clock_network::{ClockNetwork, ClockValue, ClockNode, ClockNodePrototype};
use super::*;
use super::basic::{INIT_RATE, RESET_KNOB_ID as BASIC_RESET_KNOB_ID};
use super::multiplier::{MULT_KNOB_ID, RESET_KNOB_ID as MULT_RESET_KNOB_ID};
use super::triggered::{TRIGGER_KNOB_ID};
use event::Event;
use knob::{KnobEvent, KnobPatch};
use datatypes::Update;
use network::{NetworkNode};

#[test]
fn test_basic_clock() {
    let mut network = ClockNetwork::new();
    let proto = Clock::create_prototype();
    let node = proto.create_node("test".to_string(), &[]).unwrap();
    let node_id = network.add_node(node).unwrap().id();

    assert!(INIT_RATE.in_hz() == 1.0);

    let mut update_and_check = |interval, (float_val, ticked)| {
        network.update(interval);
        let res = network.get_value_from_node(node_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(res);
    };

    update_and_check(0.75, (0.75, false));
    update_and_check(0.75, (1.5, true));
    update_and_check(0.1, (1.6, false));
    update_and_check(0.0, (1.6, false));
    update_and_check(0.5, (2.1, true));
    update_and_check(0.0, (2.1, false));
}

#[test]
fn test_basic_clock_reset_emit_event() {
    let mut network = ClockNetwork::new();
    let proto = Clock::create_prototype();
    let node = proto.create_node("test".to_string(), &[]).unwrap();
    let node_id = network.add_node(node).unwrap().id();

    network.get_node_mut(node_id).unwrap().knob_mut(BASIC_RESET_KNOB_ID).unwrap().set_button_state(true);
    //updating should emit a knob state change event
    let events = network.update(0.5);
    let correct_event = Event::Knob(KnobEvent::ValueChanged {
        patch: KnobPatch::new(node_id, BASIC_RESET_KNOB_ID),
        value: KnobValue::Button(false)
    });
    assert_eq!(events, Events::single(correct_event));
}

#[test]
fn test_multiplier() {
    let mut network = ClockNetwork::new();
    let (bp, mp) = (Clock::create_prototype(), ClockMultiplier::create_prototype());
    let basic_id = network.add_node(bp.create_node("basic".to_string(), &[]).unwrap()).unwrap().id();
    let mult_id = network.add_node(
        mp.create_node("mult".to_string(), &vec!(basic_id).into_boxed_slice()).unwrap())
        .unwrap().id();
    
    let set_mult_factor = |val, cn: &mut ClockNetwork| {
        cn.get_node_mut(mult_id).unwrap().set_knob_value(MULT_KNOB_ID, KnobValue::PositiveFloat(val)).unwrap();
    };

    let dt = 0.75;

    let update_and_check = |interval, (float_val, ticked), network: &mut ClockNetwork| {
        network.update(interval);
        let res = network.get_value_from_node(mult_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(res);
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

    // the background clock is at 3.8
    // if we reset, the multiplier should be at half this if we update by dt=0
    network.get_node_mut(mult_id).unwrap().knob_mut(MULT_RESET_KNOB_ID).unwrap().set_button_state(true);
    // hitting reset will cause the multiplier to tick
    // might want to change this if it proves to be artistically annoying
    update_and_check(0.0, (1.9, true), &mut network);
    update_and_check(0.0, (1.9, false), &mut network);
}


#[test]
fn test_triggered_clock() {
    let mut network = ClockNetwork::new();
    let proto = TriggeredClock::create_prototype();
    let node = proto.create_node("test".to_string(), &[]).unwrap();
    let node_id = network.add_node(node).unwrap().id();

    let update_and_check = |interval, (float_val, ticked), network: &mut ClockNetwork| {
        network.update(interval);
        let res = network.get_value_from_node(node_id).unwrap();
        ClockValue::from_float_value(float_val, ticked).assert_almost_eq_to(res);
    };

    let trigger = |network: &mut ClockNetwork| {
        network.get_node_mut(node_id).unwrap().knob_mut(TRIGGER_KNOB_ID).unwrap().set_button_state(true);
    };

    update_and_check(0.25, (0.0, false), &mut network);
    trigger(&mut network);
    update_and_check(0.25, (0.0, true), &mut network);
    update_and_check(0.25, (0.25, false), &mut network);
    update_and_check(0.25, (0.5, false), &mut network);
    update_and_check(0.25, (0.75, false), &mut network);
    update_and_check(0.25, (1.0, false), &mut network);
}