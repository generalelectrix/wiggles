//! Basic tests of the clock network.
use super::*;
use clocks::*;

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
    ClockMultiplier::create_prototype().create_node("mult".to_string(), &vec!(input)).unwrap()
}

fn assert_error<T: Into<ClockError>>(expected: T, actual: T) {
    assert_eq!(expected.into(), actual.into());
}

fn assert_clocks_equal(float: f64, ticked: bool, clock_val: ClockValue) {
    clock_val.assert_almost_eq_to(ClockValue::from_float_value(float, ticked));
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
    let n0id = add_node_get_id(&mut net, create_basic_node());
    let n1id = add_node_get_id(&mut net, create_multiplier(n0id));
    let remove_err = net.remove_node(n0id).unwrap_err();
    assert_error(NetworkError::NodeHasListeners(n0id), remove_err);
}

#[test]
fn test_swap_input() {
    let mut net = ClockNetwork::new();
    let n0id = add_node_get_id(&mut net, create_basic_node());
    let n1id = add_node_get_id(&mut net, create_basic_node());
    let mid = add_node_get_id(&mut net, create_multiplier(n0id));
    // push n1's clock forward by 0.5.
    net.get_node_mut(n1id).unwrap().update(0.5);

    // mult should be 0.0
    assert_clocks_equal(0.0, false, net.get_value_from_node(mid).unwrap());

    // if we swap the inputs and run update with zero timestep, we should get 0.5.
    net.swap_input(mid, 0, n1id).unwrap();
    net.update(0.0);
    assert_clocks_equal(0.5, false, net.get_value_from_node(mid).unwrap());

    // should be unable to remove n1
    let remove_err = net.remove_node(n1id).unwrap_err();
    assert_error(NetworkError::NodeHasListeners(n1id), remove_err);

    // remove n0 should be OK
    let n0 = net.remove_node(n0id).unwrap();
}