//! Basic tests of the clock network.
use super::*;
use clocks::Clock;

/// Add a node to the network.
/// Assert that the id is written into the node.
/// Return the id.
fn add_node_get_id(network: &mut ClockNetwork, node: ClockNode) -> ClockNodeIndex {
   
    let (node_id, node_ref) = {
        let node = network.add_node(node).unwrap();
        let node_id = node.id();
        let node_ref = node as *const _;
        (node_id, node_ref)
    };
    let also_node = network.get_node(node_id).unwrap();
    assert_eq!(node_ref, also_node as *const _);
    node_id
}

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
    let id = add_node_get_id(&mut net, create_basic_node());
    let _ = net.remove_node(id).unwrap();
    // removing again should be an error
    let remove_err = net.remove_node(id).unwrap_err();
    assert_error(NetworkError::InvalidNodeId(id), remove_err);
}

#[test]
fn test_no_remove_node_with_listener() {
    let mut net = ClockNetwork::new();
    add_node_get_id(&mut net, create_basic_node());
}