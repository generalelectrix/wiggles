//! Attempt to generalize some features of dataflow network into generic types.
use std::{ops, fmt, iter, slice};

use petgraph::graph::{NodeIndex, IndexType, DefaultIx};
use petgraph::stable_graph::StableDiGraph;

use interconnect::Interconnector;

pub enum NetworkError<I> {
    InvalidNodeId(I),
    MessageCollection(Vec<NetworkError<I>>),
    ExistingListenerCollection(I),
}

pub type InputId = usize;

#[derive(Debug)]
pub struct InputSocket<T> {
    /// The local name of this input socket.
    name: &'static str,
    /// Some identifier providing functionality to perform access on a graph.
    pub input: T,
}

impl<T> InputSocket<T> {
    pub fn new(name: &'static str, input: T) -> Self {
        InputSocket { name: name, input: input }
    }

    pub fn name(&self) -> &'static str { self.name }
}

/// Placeholder type for keeping track of other domains listening to the clock domain.
type ExternalListener = usize;

pub trait NetworkIndex:
    IndexType
    + Copy
    + fmt::Debug
    + ops::Deref<Target = NodeIndex>
    + From<NodeIndex> {}

pub trait InputSockets<I: NetworkIndex> {

    /// Return a slice of the intra-network connections from this node.
    fn input_sockets(&self) -> &[InputSocket<I>];

    /// Collect up the input node ids and return them,
    fn input_node_ids(&self) -> Vec<I> {
        self.input_sockets().iter().map(|socket| socket.input).collect()
    }
}

pub trait NodeId<I: NetworkIndex> {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
}

pub trait NetworkNode<I: NetworkIndex>: InputSockets<I> + NodeId<I> {}

#[derive(Debug)]
/// A network is composed of nodes and dumb edges that just act as wires.
pub struct Network<N: NetworkNode<I>, I: NetworkIndex> {
    /// The backing graph that holds the individual nodes.
    g:  StableDiGraph<N, ()>,
    /// A collection of connections from nodes to other dataflow domains.
    /// Indexed using the same node indices as the main graph.
    external_connections: Interconnector<I, ExternalListener>,
}

impl<N: NetworkNode<I>, I: NetworkIndex> Network<N, I> {
    /// Create an empty network.
    pub fn new() -> Self {
        Network {
            g: StableDiGraph::new(),
            external_connections: Interconnector::new() }
    }

    /// Return true if this graph contains the provided node index.
    pub fn contains_node(&self, node: I) -> bool {
        self.g.contains_node(*node)
    }

    /// Return true if this node has one or more outgoing edges or external listeners.
    /// Return false if the node does not exist.
    pub fn has_listeners(&self, node: I) -> bool {
        self.g.edges(*node).next().is_some()
        || self.external_connections.has_connections(node)
    }

    /// Return an error if any of the provided nodes is not part of this graph.
    pub fn check_nodes<'a, T>(&self, nodes: T) -> Result<(), NetworkError<I>>
            where T: IntoIterator<Item=&'a I> {
        let bad_nodes =
            nodes.into_iter().cloned()
                 .filter_map(|ix| {
                     if self.contains_node(ix) { None }
                     else { Some(NetworkError::InvalidNodeId(ix)) }})
                 .collect::<Vec<_>>();
        if bad_nodes.is_empty() { Ok(()) }
        else { Err(NetworkError::MessageCollection(bad_nodes)) }
    }

    /// Add a new node to the graph.  Initializes a collection of external listeners for this node.
    /// This method is only capable of validating that the inputs from this node to the same graph.
    /// The caller must have already validated that any external connections are valid and properly
    /// registered with the other network domain's interconnector.
    pub fn add_node(&mut self, node: N) -> Result<&N, NetworkError<I>> {
        let input_nodes = node.input_node_ids();
        // check that all the input nodes exist
        self.check_nodes(&input_nodes)?;

        // add the node to the graph
        let node_index: I = self.g.add_node(node).into();
        
        // If a collection of listeners already exists for this node id, some cleanup failed somewhere.
        // Remove the node we just added.
        // Return an error, though we might want to panic instead.
        if self.external_connections.has_connections(node_index) {
            self.g.remove_node(*node_index);
            return Err(NetworkError::ExistingListenerCollection(node_index));
        }

        // add edges to the input nodes
        for input_node in input_nodes.iter() {
            self.g.add_edge(**input_node, *node_index, ());
        }
        // write the index back into the new node
        let new_node = self.g.node_weight_mut(*node_index).unwrap();
        new_node.set_id(node_index);
        Ok(new_node)
    }
}