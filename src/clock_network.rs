//! Description of the contents and action of a network of related clocks.
//! Clocks can tap the output of other clocks to perform clock mutation or
//! modulation, such as multiplication and division.  Clocks can also declare
//! that they accept input from one or more "knobs", enabling push-based dataflow
//! of control parameters into the clocks.
use std::cell::Cell;
use std::collections::HashMap;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, EdgeIndex, IndexType, DefaultIx};
use utils::modulo_one;
use update::{Update, DeltaT};
use knob::Knob;
use interconnect::Interconnector;


#[derive(Clone, Copy, Debug)]
/// Represent the complete value of the current state of a clock.
pub struct ClockValue {
    pub phase: f64,
    pub tick_count: i64,
    pub ticked: bool,
}

impl ClockValue {
    /// Construct a clock value from a float and whether or not it just ticked.
    pub fn from_float_value(val: f64, ticked: bool) -> Self {
        ClockValue { phase: modulo_one(val), tick_count: val.trunc() as i64, ticked: ticked }
    }
    pub fn float_value(&self) -> f64 { self.tick_count as f64 + self.phase }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct ClockNodeIndex(NodeIndex);

/// This implementation is not unsafe, though since the trait is unsafe the unsafety
/// here is only coming from our reliance on the underlying type itself.
unsafe impl IndexType for ClockNodeIndex {
    #[inline(always)]
    fn new(x: usize) -> Self { ClockNodeIndex(NodeIndex::new(x)) }
    #[inline(always)]
    fn index(&self) -> usize {
        let ClockNodeIndex(idx) = *self;
        idx.index() }
    #[inline(always)]
    fn max() -> Self { ClockNodeIndex::new(DefaultIx::max() as usize) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
/// Newtype declaration to ensure we don't mix up edges between different graph domains.
pub struct ClockEdgeIndex(EdgeIndex);

/// Placeholder type for keeping track of other domains listening to the clock domain.
type ExternalListener = usize;

/// A clock graph is composed of nodes and dumb edges that just act as wires.
pub struct ClockGraph {
    /// The backing graph that holds the individual nodes.
    g: StableDiGraph<ClockNode, ()>,
    /// A hash to loop up clock nodes by name.
    node_lookup: HashMap<String, ClockNodeIndex>,
    /// A collection of connections from nodes to other dataflow domains.
    /// Indexed using the same node indices as the main graph.
    external_connections: Interconnector<ClockNodeIndex, ExternalListener>,
}

fn placeholder_index() -> ClockNodeIndex { ClockNodeIndex(NodeIndex::new(0)) }

impl ClockGraph {
    /// Create an empty clock graph.
    pub fn new() -> Self {
        ClockGraph {
            g: StableDiGraph::new(),
            node_lookup: HashMap::new(),
            external_connections: Interconnector::new() }
    }

    /// Return true if this graph contains the provided node index.
    pub fn contains_node(&self, ClockNodeIndex(idx): ClockNodeIndex) -> bool {
        self.g.contains_node(idx)
    }

    /// Return true if this node has one or more outgoing edges or external listeners.
    /// Returns false if the node does not exist.
    pub fn has_listeners(&self, ClockNodeIndex(idx): ClockNodeIndex) -> bool {
        self.g.edges(idx).next().is_some()
    }

    /// Return an error if any of the provided nodes is not part of this graph.
    pub fn check_nodes(&self, nodes: &[ClockNodeIndex]) -> Result<(), ClockMessage> {
        let bad_nodes =
            nodes.iter().cloned()
                 .filter_map(|ix| {
                     if self.contains_node(ix) { None }
                     else { Some(ClockMessage::InvalidNodeIndex(ix)) }})
                 .collect::<Vec<_>>();
        if bad_nodes.len() == 0 { Err(ClockMessage::MessageCollection(bad_nodes)) }
        else { Ok(()) }
    }

    /// Instantiate a collection of external listeners for a node index.
    /// If this collection already exists and is not empty, panic.
    fn add_external_listener_collection(&mut self, ClockNodeIndex(idx): ClockNodeIndex) {

    }

    /// Add a new node to the graph using a prototype and a list of input
    /// nodes to connect the inputs of the clock.  Initializes a collection
    /// of external listeners for this node as well.
    pub fn add_node(&mut self,
                    prototype: ClockNodePrototype,
                    name: String,
                    input_nodes: &[ClockNodeIndex])
                    -> Result<&ClockNode, ClockMessage> {
        // check that all the input nodes exist
        try!(self.check_nodes(input_nodes));
        // create the node with a placeholder index
        let new_node = try!(prototype.create_node(name, placeholder_index(), input_nodes));
        // add the node to the graph
        let node_index = self.g.add_node(new_node);
        
        // add edges to the input nodes
        for &ClockNodeIndex(input_node) in input_nodes.iter() {
            self.g.add_edge(input_node, node_index, ());
        }
        // write the index back into the new node
        let new_node = self.g.node_weight_mut(node_index).unwrap();
        new_node.id = ClockNodeIndex(node_index);
        Ok(new_node)
    }

    /// Remove a node from the graph, including all edges coming in to the node.
    /// If the graph has any *outgoing* edges, return an error.  Incoming edges
    /// are always safe to eliminate.
    /// The remove node is returned if removal was successful.
    // pub fn remove_node(&mut self, idx: ClockNodeIndex) -> Result<ClockNode, ClockMessage> {
    //     if self.has_listeners(idx) {
    //         return Err(ClockMessage::NodeHasListeners);
    //     }
    //     // this node isn't feeding anything downstream, so we can safely delete it

    // }


    /// Get the current clock value from any node in the graph.
    /// Return an error if the node doesn't exist or is invalid.
    fn get_value_from_node(
            &self,
            ClockNodeIndex(idx): ClockNodeIndex)
            -> Result<ClockValue, ClockMessage> {
        if let Some(node) = self.g.node_weight(idx) {
            Ok(node.get_value(&self))
        } else {
            Err(ClockMessage::InvalidNodeIndex(ClockNodeIndex(idx)))
        }
    }
}

pub type ClockImplProducer = Box<Fn() -> Box<CompleteClock>>;

/// Serve as a persistent prototype which can be used to create new instances of clock nodes.
pub struct ClockNodePrototype {
    /// A name that identifies this particular clock prototype.  For example,
    /// "simple", "multiplier", etc.
    type_name: &'static str,
    /// The names and numeric IDs of the clock input ports.
    inputs: Box<[(&'static str, InputId)]>,
    /// The control knobs that this clock presents.
    knobs: Box<[Knob]>,
    /// A stored procedure that returns a trait object implementing the clock.
    clock: ClockImplProducer,
}

impl ClockNodePrototype {
    pub fn new(type_name: &'static str,
               inputs: Box<[(&'static str, InputId)]>,
               knobs: Box<[Knob]>,
               clock: ClockImplProducer)
               -> Self {
        ClockNodePrototype {
            type_name: type_name, inputs: inputs, knobs: knobs, clock: clock
        }
    }

    pub fn n_inputs(&self) -> usize { self.inputs.len() }

    pub fn create_node(&self,
                       name: String,
                       id: ClockNodeIndex,
                       input_nodes: &[ClockNodeIndex])
                       -> Result<ClockNode, ClockMessage> {
        if input_nodes.len() != self.inputs.len() {
            return Err(
                ClockMessage::MismatchedInputs {
                    expected: self.inputs.len(),
                    provided: input_nodes.len()});
        }
        let connected_inputs =
            self.inputs.iter()
                       .enumerate()
                       .zip(input_nodes)
                       .map(|((i, &(name, input_id)), node_id)| {
                                // make sure the input IDs are consistent and
                                // monotonically increasing.
                                debug_assert!(input_id == i);
                                ClockInputSocket::new(name, input_id, *node_id)
                            })
                       .collect::<Vec<_>>().into_boxed_slice();
        Ok(ClockNode {
            name: name,
            id: id,
            inputs: connected_inputs,
            knobs: self.knobs.clone(),
            current_value: Cell::new(None),
            clock: (self.clock)(),
        })
    }
}

/// A single node in an arbitrary clock graph, accepting inputs, listening to
/// knobs, and with a stored behavior that uses these values to produce a
/// clock value when called upon.
pub struct ClockNode {
    /// Unique name for this clock.
    pub name: String,
    /// The index of this node in the enclosing graph.
    /// The graph implementation must ensure that these indices remain stable.
    pub id: ClockNodeIndex,
    /// Named input sockets that connect this node to upstream clocks.
    inputs: Box<[ClockInputSocket]>,
    /// Named knobs that provide control parameters.
    pub knobs: Box<[Knob]>,
    /// The current, memoized value of this clock.
    current_value: Cell<Option<ClockValue>>,
    /// The stored behavior used to update the current value based on the
    /// current state of the inputs and knobs.  It may have internal state
    /// that will be updated during the global timestep update.
    clock: Box<CompleteClock>,
}

impl ClockNode {
    /// Return the current value of this clock node.
    fn get_value(&self, g: &ClockGraph) -> ClockValue {
        // If we have memoized the value, get it.
        if let Some(v) = self.current_value.get() {
            v
        } else {
            let v = self.clock.compute_clock(&self.inputs, &self.knobs, g);
            self.current_value.set(Some(v));
            v
        }
    }
}

impl Update for ClockNode {
    fn update(&mut self, dt: DeltaT) {
        self.current_value.set(None);
        self.clock.update(&mut self.knobs, dt);
    }
}

/// Given a timestep and the current state of a clock's control knobs, update
/// any internal state of the clock.
pub trait UpdateClock {
    fn update(&mut self, knobs: &mut [Knob], dt: DeltaT);
}

/// Given some inputs and knobs, compute a clock value.
pub trait ComputeClock {
    fn compute_clock(&self,
                     inputs: &[ClockInputSocket],
                     knobs: &[Knob],
                     g: &ClockGraph)
                     -> ClockValue;
}

/// A clock needs to implement these traits to function as a node in the clock network.
pub trait CompleteClock: ComputeClock + UpdateClock {}
impl<T> CompleteClock for T where T: ComputeClock + UpdateClock {}

/// Type alias for indexing input sockets.
pub type InputId = usize;

/// Specify an upstream clock via an incoming edge to this clock graph node.
pub struct ClockInputSocket {
    /// The local name of this input socket.
    pub name: &'static str,
    /// A locally-unique numeric id for this socket.  For each node, these should
    /// start at 0 and increase monotonically.
    id: InputId,
    /// The index of the source node.
    input_node: ClockNodeIndex,
}

impl ClockInputSocket {
    pub fn new(name: &'static str, id: InputId, input_node: ClockNodeIndex) -> Self {
        ClockInputSocket { name: name, id: id, input_node: input_node }
    }

    /// Fetch the upstream value from the source for this socket.
    /// # Panics
    /// 
    /// If the upstream node doesn't exist.
    pub fn get_value(&self, g: &ClockGraph) -> ClockValue {
        g.get_value_from_node(self.input_node).unwrap()
    }
}

#[derive(Debug)]
/// Message type to convey error conditions related to clock network operations.
pub enum ClockMessage {
    MessageCollection(Vec<ClockMessage>),
    WouldCycle { source: ClockNodeIndex, sink: ClockNodeIndex },
    InvalidInputId(InputId),
    InvalidNodeIndex(ClockNodeIndex),
    MismatchedInputs { expected: usize, provided: usize },
    NodeHasListeners,

}