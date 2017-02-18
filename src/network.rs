//! Attempt to generalize some features of dataflow network into generic types.
use std::{ops, fmt, iter, slice, marker};

use petgraph::graph::{NodeIndex, IndexType, DefaultIx};
use petgraph::stable_graph::StableDiGraph;
use petgraph::Direction;
use petgraph::algo::has_path_connecting;

use datatypes::{Update, DeltaT};
use event::Events;
use interconnect::Interconnector;

pub enum NetworkEvent<I> {
    InputSwapped{node: I, input_id: InputId, new_input: I},
}

#[derive(Debug)]
pub enum NetworkError<I> {
    InvalidNodeId(I),
    MessageCollection(Vec<NetworkError<I>>),
    ExistingListenerCollection(I),
    NodeHasListeners(I),
    InvalidInputId(I, InputId),
    WouldCycle { source: I, sink: I }
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

pub trait NetworkNodeId:
    IndexType
    + Copy
    + fmt::Debug
    + ops::Deref<Target = NodeIndex>
    + From<NodeIndex> {}

pub trait NetworkNode<I: NetworkNodeId>: Update {

    /// Return a slice of the intra-network connections from this node.
    fn input_sockets(&self) -> &[InputSocket<I>];

    fn input_socket(&self, id: InputId) -> Result<&InputSocket<I>,NetworkError<I>>;

    fn input_socket_mut(&mut self, id: InputId) -> Result<&mut InputSocket<I>,NetworkError<I>>;

    fn set_input(&mut self, id: InputId, new_source: I) -> Result<(),NetworkError<I>> {
        self.input_socket_mut(id)?.input = new_source;
        Ok(())
    }

    /// Collect up the input node ids and return them,
    fn input_node_ids(&self) -> Vec<I> {
        self.input_sockets().iter().map(|socket| socket.input).collect()
    }

    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
}

#[derive(Debug)]
/// A network is composed of nodes and dumb edges that just act as wires.
pub struct Network<N: NetworkNode<I>, I: NetworkNodeId> {
    /// The backing graph that holds the individual nodes.
    g:  StableDiGraph<N, ()>,
    /// A collection of connections from nodes to other dataflow domains.
    /// Indexed using the same node indices as the main graph.
    external_connections: Interconnector<I, ExternalListener>,
}

impl<N: NetworkNode<I>, I: NetworkNodeId> Network<N, I> {
    /// Create an empty network.
    pub fn new() -> Self {
        Network {
            g: StableDiGraph::new(),
            external_connections: Interconnector::new(),
        }
    }

    /// Return true if this graph contains the provided node id.
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
        let node_id: I = self.g.add_node(node).into();
        
        // If a collection of listeners already exists for this node id, some cleanup failed somewhere.
        // Remove the node we just added.
        // Return an error, though we might want to panic instead.
        if self.external_connections.has_connections(node_id) {
            self.g.remove_node(*node_id);
            return Err(NetworkError::ExistingListenerCollection(node_id));
        }

        // add edges to the input nodes
        for input_node in input_nodes.iter() {
            self.g.add_edge(**input_node, *node_id, ());
        }
        // write the id back into the new node
        let new_node = self.g.node_weight_mut(*node_id).unwrap();
        new_node.set_id(node_id);
        Ok(new_node)
    }

    /// Get a reference to a node in the graph, if it exists.
    pub fn get_node(&self, node: I) -> Result<&N, NetworkError<I>> {
        self.g.node_weight(*node).ok_or(NetworkError::InvalidNodeId(node))
    }

    /// Get a reference to a node in the graph, if it exists.
    pub fn get_node_mut(&mut self, node: I) -> Result<&mut N, NetworkError<I>> {
        self.g.node_weight_mut(*node).ok_or(NetworkError::InvalidNodeId(node))
    }

    /// Remove a node from the graph, including all edges coming in to the node.
    /// If the graph has any *outgoing* edges or external listeners, return an error.
    /// Incoming edges are always safe to eliminate.
    /// The removed node is returned if removal was successful.
    pub fn remove_node(&mut self, node: I) -> Result<N, NetworkError<I>> {
        if self.has_listeners(node) {
            return Err(NetworkError::NodeHasListeners(node));
        }
        // this node isn't feeding anything downstream, so we can safely delete it
        // first eliminate all of the incoming edges
        self.g.remove_node(*node).ok_or(NetworkError::InvalidNodeId(node))
    }

    /// Attempt to swap a particular input of a node for a different input.
    pub fn swap_input(&mut self,
                      node_index: I,
                      id: InputId,
                      new_source: I)
                      -> Result<NetworkEvent<I>, NetworkError<I>> {
        // identify the current node connected to this input
        let current_source = self.get_node(node_index)?.input_socket(id)?.input;

        // make sure this won't create a cycle
        self.check_cycle(new_source, node_index)?;

        // first swap the input at the node level
        self.get_node_mut(node_index)?.set_input(id, new_source)?;

        // remove old edge, create new edge
        // if there wasn't an edge there, the graph state was inconsistent...
        // for now, just ignore this.
        // TODO: log this inconsistency or something
        self.g.find_edge(*current_source, *node_index)
            .map(|edge| self.g.remove_edge(edge));
        
        self.g.add_edge(*new_source, *node_index, ());
        Ok(NetworkEvent::InputSwapped{ node: node_index, input_id: id, new_input: new_source })
    }

    /// Return an error if connecting source to sink would create a cycle.
    fn check_cycle(&self, source: I, sink: I) -> Result<(), NetworkError<I>> {
        let would_cycle =
            // if the source has sources
            self.g.edges_directed(*source, Direction::Incoming).next().is_some()
            // and if the sink has sinks
            && self.g.edges_directed(*sink, Direction::Outgoing).next().is_some()
            // and if there isn't already an edge connecting source to sink
            && self.g.find_edge(*source, *sink).is_none()
            // then we need to check if sink is already upstream from source.
            && has_path_connecting(&self.g, *sink, *source, None);
        if would_cycle {
            Err(NetworkError::WouldCycle {source: source, sink: sink})
        } else {
            Ok(())
        }
    }
}

impl<N: NetworkNode<I>, I: NetworkNodeId> Update for Network<N, I> {
    fn update(&mut self, dt: DeltaT) -> Events {
        let all_indices: Vec<NodeIndex> = self.g.node_indices().collect();
        all_indices.iter()
                   .flat_map(|ni| self.get_node_mut(I::from(*ni)).unwrap().update(dt))
                   .collect()
    }
}