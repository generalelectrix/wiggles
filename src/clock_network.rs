//! Description of the contents and action of a network of related clocks.
//! Clocks can tap the output of other clocks to perform clock mutation or
//! modulation, such as multiplication and division.  Clocks can also declare
//! that they accept input from one or more "knobs", enabling push-based dataflow
//! of control parameters into the clocks.
use std::cell::Cell;
use std::collections::HashMap;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, EdgeIndex};
use utils::modulo_one;
use update::{Update, DeltaT};
use knob::Knob;


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

#[derive(Clone, Copy, Debug)]
/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct ClockNodeIndex(NodeIndex);

#[derive(Clone, Copy, Debug)]
/// Newtype declaration to ensure we don't mix up edges between different graph domains.
pub struct ClockEdgeIndex(EdgeIndex);

/// A clock graph is composed of nodes and dumb edges that just act as wires.
pub struct ClockGraph {
    /// The backing graph that holds the individual nodes.
    g: StableDiGraph<ClockNode, ()>,
    /// A hash to loop up clock nodes by name.
    node_lookup: HashMap<String, ClockNodeIndex>
}

impl ClockGraph {
    pub fn new() -> Self {
        ClockGraph { g: StableDiGraph::new(), node_lookup: HashMap::new() }
    }


    /// Get the current value from any node in the graph.
    /// # Panics
    /// 
    /// If the provided node doesn't exist or is invalid.
    fn get_value_from_node(&self, ClockNodeIndex(idx): ClockNodeIndex) -> ClockValue {
        if let Some(node) = self.g.node_weight(idx) {
            node.get_value(&self)
        } else {
            panic!("The clock node id {:?} is invalid or does not exist.", idx)
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

    pub fn create_node(&self,
                       name: String,
                       id: ClockNodeIndex,
                       input_nodes: &[ClockNodeIndex])
                       -> ClockNode {
        debug_assert!(input_nodes.len() == self.inputs.len());
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
                       .collect::<Vec<_>>()
                       .into_boxed_slice();
        ClockNode {
            name: name,
            id: id,
            inputs: connected_inputs,
            knobs: self.knobs.clone(),
            current_value: Cell::new(None),
            clock: (self.clock)(),
        }
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
    inputs: Box<[ClockInputSocket]>,
    /// Named knobs that provide control parameters.
    knobs: Box<[Knob]>,
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
    pub fn get_value(&self, g: &ClockGraph) -> ClockValue {
        g.get_value_from_node(self.input_node)
    }
}


/// Message type to convey error conditions related to clock network operations.
pub enum ClockMessage {
    WouldCycle { source: ClockNodeIndex, sink: ClockNodeIndex },
    InvalidInputId(InputId),
}