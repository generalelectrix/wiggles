//! Description of the contents and action of a network of related clocks.
//! Clocks can tap the output of other clocks to perform clock mutation or
//! modulation, such as multiplication and division.  Clocks can also declare
//! that they accept input from one or more "knobs", enabling push-based dataflow
//! of control parameters into the clocks.
use std::fmt;
use std::error;
use std::cell::Cell;
use std::collections::HashMap;
use std::ops::Deref;
use itertools::Itertools;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, EdgeIndex, IndexType, DefaultIx};
use petgraph::algo::has_path_connecting;
use petgraph::Direction;
use utils::modulo_one;
use update::{Update, DeltaT};
use knob::{Knob, Knobs, KnobId, KnobValue, KnobPatch, KnobEvent};
use interconnect::Interconnector;
use event::Event;

/// Events related to clocks and the clock graph.
pub enum ClockResponse {
    /// Inform the world that this clock node has swapped an input
    InputSwaped { node: ClockNodeIndex, input_id: InputId, new_input: ClockNodeIndex },
}

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

impl Deref for ClockNodeIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &NodeIndex { &self.0 }
}

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
    pub fn contains_node(&self, node: ClockNodeIndex) -> bool {
        self.g.contains_node(*node)
    }

    /// Return true if this node has one or more outgoing edges or external listeners.
    /// Returns false if the node does not exist.
    pub fn has_listeners(&self, node: ClockNodeIndex) -> bool {
        self.g.edges(*node).next().is_some()
        || self.external_connections.has_connections(node)
    }

    /// Return an error if any of the provided nodes is not part of this graph.
    pub fn check_nodes(&self, nodes: &[ClockNodeIndex]) -> Result<(), ClockError> {
        let bad_nodes =
            nodes.iter().cloned()
                 .filter_map(|ix| {
                     if self.contains_node(ix) { None }
                     else { Some(ClockError::InvalidNodeIndex(ix)) }})
                 .collect::<Vec<_>>();
        if bad_nodes.is_empty() { Ok(()) }
        else { Err(ClockError::MessageCollection(bad_nodes)) }
    }

    /// Add a new node to the graph using a prototype and a list of input
    /// nodes to connect the inputs of the clock.  Initializes a collection
    /// of external listeners for this node as well.
    pub fn add_node(&mut self,
                    prototype: &ClockNodePrototype,
                    name: String,
                    input_nodes: &[ClockNodeIndex])
                    -> Result<&ClockNode, ClockError> {
        // check that all the input nodes exist
        try!(self.check_nodes(input_nodes));
        // create the node with a placeholder index
        let new_node = try!(prototype.create_node(name, placeholder_index(), input_nodes));

        // add the node to the graph
        let node_index = ClockNodeIndex(self.g.add_node(new_node));
        
        // If a collection of listeners already exists for this node id, some cleanup failed somewhere.
        // Remove the node we just added.
        // Return an error, though we might want to panic instead.
        if self.external_connections.has_connections(node_index) {
            self.g.remove_node(*node_index);
            return Err(ClockError::ExistingListenerCollection(node_index));
        }

        // add edges to the input nodes
        for input_node in input_nodes.iter() {
            self.g.add_edge(**input_node, *node_index, ());
        }
        // write the index back into the new node
        let new_node = self.g.node_weight_mut(*node_index).unwrap();
        new_node.id = node_index;
        Ok(new_node)
    }

    /// Get a reference to a node in the graph, if it exists.
    pub fn get_node(&self, node: ClockNodeIndex) -> Result<&ClockNode, ClockError> {
        self.g.node_weight(*node).ok_or(ClockError::InvalidNodeIndex(node))
    }

        /// Get a reference to a node in the graph, if it exists.
    pub fn get_node_mut(&mut self, node: ClockNodeIndex) -> Result<&mut ClockNode, ClockError> {
        self.g.node_weight_mut(*node).ok_or(ClockError::InvalidNodeIndex(node))
    }

    /// Remove a node from the graph, including all edges coming in to the node.
    /// If the graph has any *outgoing* edges or external listeners, return an error.
    /// Incoming edges are always safe to eliminate.
    /// The removed node is returned if removal was successful.
    pub fn remove_node(&mut self, node: ClockNodeIndex) -> Result<ClockNode, ClockError> {
        if self.has_listeners(node) {
            return Err(ClockError::NodeHasListeners(node));
        }
        // this node isn't feeding anything downstream, so we can safely delete it
        // first eliminate all of the incoming edges
        self.g.remove_node(*node).ok_or(ClockError::InvalidNodeIndex(node))
    }

    /// Get the current clock value from any node in the graph.
    /// Return an error if the node doesn't exist or is invalid.
    fn get_value_from_node(&self, node: ClockNodeIndex) -> Result<ClockValue, ClockError> {
        Ok(self.get_node(node)?.get_value(&self))
    }

    /// Attempt to swap a particular input of a clock for a different input.
    pub fn swap_input(&mut self,
                      node_index: ClockNodeIndex,
                      id: InputId,
                      new_source: ClockNodeIndex)
                      -> Result<ClockResponse, ClockError> {
        // identify the current node connected to this input
        let current_source = self.get_node(node_index)?.get_input(id)?;

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
        Ok(ClockResponse::InputSwaped{ node: node_index, input_id: id, new_input: new_source })
    }

    /// Return an error if connecting source to sink would create a cycle.
    fn check_cycle(&self, source: ClockNodeIndex, sink: ClockNodeIndex) -> Result<(), ClockError> {
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
            Err(ClockError::WouldCycle {source: source, sink: sink})
        } else {
            Ok(())
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

    pub fn type_name(&self) -> &'static str { self.type_name }

    pub fn n_inputs(&self) -> usize { self.inputs.len() }

    pub fn create_node(&self,
                       name: String,
                       id: ClockNodeIndex,
                       input_nodes: &[ClockNodeIndex])
                       -> Result<ClockNode, ClockError> {
        if input_nodes.len() != self.inputs.len() {
            return Err(
                ClockError::MismatchedInputs {
                    type_name: self.type_name,
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
                       .collect::<Vec<_>>();
        Ok(ClockNode {
            name: name,
            id: id,
            inputs: connected_inputs,
            knobs: self.knobs.clone().into_vec(),
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
    name: String,
    /// The index of this node in the enclosing graph.
    /// The graph implementation must ensure that these indices remain stable.
    id: ClockNodeIndex,
    /// Named input sockets that connect this node to upstream clocks.
    inputs: Vec<ClockInputSocket>,
    /// Named knobs that provide control parameters.
    knobs: Vec<Knob>,
    /// The current, memoized value of this clock.
    current_value: Cell<Option<ClockValue>>,
    /// The stored behavior used to update the current value based on the
    /// current state of the inputs and knobs.  It may have internal state
    /// that will be updated during the global timestep update.
    clock: Box<CompleteClock>,
}

impl ClockNode {
    /// Return the index where this node can be found.
    pub fn index(&self) -> ClockNodeIndex { self.id }

    /// Return the current value of this clock node.
    pub fn get_value(&self, g: &ClockGraph) -> ClockValue {
        // If we have memoized the value, get it.
        if let Some(v) = self.current_value.get() {
            v
        } else {
            let v = self.clock.compute_clock(&self.inputs, &self.knobs, g);
            self.current_value.set(Some(v));
            v
        }
    }
    
    /// Return the node index that an input is listening to.
    pub fn get_input(&self, id: InputId) -> Result<ClockNodeIndex, ClockError> {
        self.inputs.get(id)
                   .map(|input| input.input_node)
                   .ok_or(ClockError::InvalidInputId(self.index(), id))
    }

    /// Set an input to a particular node.
    pub fn set_input(&mut self, id: InputId, source: ClockNodeIndex) -> Result<(), ClockError> {
        self.inputs.get_mut(id)
                   .map(|input| { input.input_node = source; })
                   .ok_or(ClockError::InvalidInputId(self.index(), id))
    }
}

impl Update for ClockNode {
    fn update(&mut self, dt: DeltaT) -> Option<Event> {
        self.current_value.set(None);
        self.clock.update(self.id, &mut self.knobs, dt)
    }
}

impl Knobs for ClockNode {
    fn knobs(&self) -> &[Knob] { &self.knobs }
    fn knobs_mut(&mut self) -> &mut [Knob] { &mut self.knobs }
}

/// Helper function to create an event to register a clock changing the state of
/// a button-type knob as a result of registering a transient button press.
pub fn clock_button_update(node: ClockNodeIndex, knob: &Knob, state: bool) -> Event {
    let patch = KnobPatch::Clock { node: node, id: knob.id() };
    let value = KnobValue::Button(state);
    Event::Knob(KnobEvent::ValueChanged { patch: patch, value: value })
}

/// Given a timestep and the current state of a clock's control knobs, update
/// any internal state of the clock.  Updating a clock may require emitting
/// some kind of state update announcement, so allow returning a collection
/// of events.  The id of the node that owns this clock is passed in as it
/// is needed for emitting certain events.
pub trait UpdateClock {
    fn update(&mut self, id: ClockNodeIndex, knobs: &mut [Knob], dt: DeltaT) -> Option<Event>;
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
    name: &'static str,
    /// A locally-unique numeric id for this socket.  For each node, these should
    /// start at 0 and increase monotonically.
    id: InputId,
    /// The index of the source node.
    pub input_node: ClockNodeIndex,
}

impl ClockInputSocket {
    pub fn new(name: &'static str, id: InputId, input_node: ClockNodeIndex) -> Self {
        ClockInputSocket { name: name, id: id, input_node: input_node }
    }

    pub fn name(&self) -> &'static str { self.name }

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
pub enum ClockError {
    MessageCollection(Vec<ClockError>),
    WouldCycle { source: ClockNodeIndex, sink: ClockNodeIndex },
    InvalidInputId(ClockNodeIndex, InputId),
    InvalidNodeIndex(ClockNodeIndex),
    MismatchedInputs { type_name: &'static str, expected: usize, provided: usize },
    NodeHasListeners(ClockNodeIndex),
    ExistingListenerCollection(ClockNodeIndex),
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClockError::WouldCycle{ source, sink } => 
                write!(f, "Connecting clock node {:?} to {:?} would create a cycle.", source, sink),
            ClockError::InvalidInputId(node, id) =>
                write!(f, "Clock node {:?} has no input with id {:?}.", node, id),
            ClockError::InvalidNodeIndex(node) =>
                write!(f, "Invalid clock node {:?}.", node),
            ClockError::MismatchedInputs { type_name, expected, provided } =>
                write!(f, "Clock type {} expects {} inputs but was provided {}.", type_name, expected, provided),
            ClockError::NodeHasListeners(node) =>
                write!(f, "Clock node {:?} has listeners connected.", node),
            ClockError::ExistingListenerCollection(node) =>
                write!(f, "Tried to create a listener collection for node {:?} but it already has a non-empty one.", node),
            ClockError::MessageCollection(ref msgs) => {
                write!(f, "Multiple messages:\n{}", msgs.iter().format("\n"))
            }
        }
    }
}

impl error::Error for ClockError {
    // TODO: description messages, though we may never need them
    fn description(&self) -> &str { "" }

     fn cause(&self) -> Option<&error::Error> { None }
}