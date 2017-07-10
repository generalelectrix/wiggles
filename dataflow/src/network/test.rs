//! Tests for the network abstraction itself.
use std::fmt;
use std::collections::HashMap;
use console_server::reactor::Messages;
use network::{Network, Inputs, NodeId, NetworkError};
use log::LogLevel;
use simple_logger;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TestNodeId(u32, u32);

impl fmt::Display for TestNodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl NodeId for TestNodeId {
    fn index(&self) -> u32 {
        self.0
    }

    fn gen_id(&self) -> u32 {
        self.1
    }

    fn new(i: u32, gid: u32) -> Self {
        TestNodeId(i, gid)
    }

}

#[derive(Debug)]
struct EmptyNode;

impl<T> Inputs<T> for EmptyNode {
    fn default_input_count(&self) -> u32 { 1 }
    fn try_push_input(&mut self) -> Result<Messages<T>, ()> { Err(()) }
    fn try_pop_input(&mut self) -> Result<Messages<T>, ()> { Err(()) }
}

type TestNetwork = Network<EmptyNode, TestNodeId, ()>;

fn expect_no_cycle<I: NodeId>(src: I, dest: I, result: Result<(), NetworkError<I>>) {
    match result.expect_err("Network did not return an error.") {
        NetworkError::WouldCycle{source, sink} => {
            assert_eq!(source, src);
            assert_eq!(sink, dest);
        }
        x => panic!("Network returned unexpected error: {}", x),
    }
}

fn add_node_get_id(net: &mut TestNetwork) -> TestNodeId {
    let (id, _) = net.add(EmptyNode);
    id
}

#[test]
fn test_no_cycles() {
    simple_logger::init_with_level(LogLevel::Debug).unwrap();
    let mut net: TestNetwork = Network::new();
    let end = add_node_get_id(&mut net);

    expect_no_cycle(end, end, net.swap_input(end, 0, Some(end)));

    let head = add_node_get_id(&mut net);
    net.swap_input(end, 0, Some(head)).unwrap();
    {
        let mut expected_listeners = HashMap::new();
        expected_listeners.insert(end.index(), 1);
        assert_eq!(net.node(head).unwrap().listeners, expected_listeners);

        assert_eq!(net.node(end).unwrap().inputs(), vec!(Some(head)).as_slice());
    }

    assert!(net.node(head).unwrap().has_listeners());
    assert!(net.node_among_listeners(head.index(), end.index()));
    assert!(net.check_would_cycle(end, head).is_err());
    expect_no_cycle(end, head, net.swap_input(head, 0, Some(end)));

    let middle = add_node_get_id(&mut net);

    net.swap_input(end, 0, Some(middle)).unwrap();
    net.swap_input(middle, 0, Some(head)).unwrap();

    assert!(net.node_among_listeners(head.index(), end.index()));
    assert!(net.check_would_cycle(end, head).is_err());
    expect_no_cycle(end, head, net.swap_input(head, 0, Some(end)));
}