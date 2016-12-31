//! Type for keeping track of listening connections across disparate dataflow domains.
//! Provides the Interconnector type, intended to interoperate with a dataflow graph to keep track
//! of connections from that dataflow graph to one or more other graphs.  Uses the same node indices
//! as the companion graph.
use std::marker::PhantomData;
use petgraph::graph::IndexType;

pub struct Interconnector<IntId: IndexType, ExtId: Eq + Copy> {
    connections: Vec<Vec<ExtId>>,
    _marker: PhantomData<IntId>,
}

impl<IntId: IndexType, ExtId: Eq + Copy> Interconnector<IntId, ExtId> {
    pub fn new() -> Self { Interconnector { connections: Vec::new(), _marker: PhantomData } }

    /// Ensure that a given index contains an initialized collection of listeners.
    /// Return a reference to that collection.
    fn ensure_collection(&mut self, node: IntId) -> &mut Vec<ExtId> {
        // extend the collection if the new node is larger than the current collection
        let index = node.index();
        if let None = self.connections.get_mut(index) {
            // If this node index is out of bounds, extend the collection.
            // Use zero-length vecs that won't allocate until used.
            self.connections.resize(index, Vec::with_capacity(0));
        }
        self.connections.get_mut(index).unwrap()
    }

    /// Add a new connection from an internal node to an external listener.
    pub fn add(&mut self, node: IntId, connection: ExtId) {
        let conns = self.ensure_collection(node);
        conns.push(connection);
    }

    /// Remove a connection from an internal node to an external listener.
    /// Does nothing if that connection is not present.
    pub fn remove(&mut self, node: IntId, connection: ExtId) {
        let conns = self.ensure_collection(node);
        if let Some(index) = conns.iter().position(|&i| i == connection) {
            conns.swap_remove(index);
        }
    }
}