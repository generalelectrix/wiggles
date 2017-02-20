//! Basic tests of the clock network.
use super::*;
use clocks::Clock;

fn create_basic_node() -> ClockNode {
    Clock::create_prototype().create_node("basic".to_string(), &[]).unwrap()
}

fn create_multiplier(input: ClockNodeIndex) -> ClockNode {
    Clock::create_prototype().create_node("mult".to_string(), &vec!(input)).unwrap()
}

fn assert_error<T: Into<ClockError>>(expected: T, actual: T) {
    assert_eq!(expected.into(), actual.into());
}

#[test]
fn test_add_remove_node() {
    let mut net = ClockNetwork::new();
    let node = create_basic_node();
    let id = net.add_node(node).unwrap().id();
    let _ = net.remove_node(id).unwrap();
    // removing again should be an error
    let remove_err = net.remove_node(id).unwrap_err();
    assert_error(NetworkError::InvalidNodeId(id), remove_err);
}