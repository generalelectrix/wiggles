//! Description of the contents and action of a network of related clocks.
//! Clocks can tap the output of other clocks to perform clock mutation or
//! modulation, such as multiplication and division.  Clocks can also declare
//! that they accept input from one or more "knobs", enabling push-based dataflow
//! of control parameters into the clocks.
use std::cell::Cell;
use std::collections::HashMap;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, EdgeIndex};

use update::{Update, DeltaT};
use knob::Knob;


#[derive(Clone, Copy, Debug)]
/// Represent the complete value of the current state of a clock.
pub struct ClockValue {
    phase: f64,
    tick_count: i64,
    ticked: bool,
}

impl ClockValue {
    fn float_value(&self) -> f64 { self.tick_count as f64 + self.phase }
}

/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct ClockNodeIndex(NodeIndex);
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
    /// Get the current value from any node in the graph.
    /// # Panics
    /// 
    /// If the provided node doesn't exist or is invalid.
    fn get_value_from_node(&self, ClockNodeIndex(idx): ClockNodeIndex) -> ClockValue {
        if let Some(node) = self.g.node_weight(idx) {
            node.get_value(&self)
        } else {
            panic!("The clock node id {} is invalid or does not exist.", idx)
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
    id: NodeIndex,
    /// Named input sockets that connect this node to upstream clocks.
    inputs: Box<[ClockInputSocket]>,
    /// Named knobs that permit threading in control parameters.
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
            self.current_value.set(v);
            v
        }
    }
}

/// Given a timestep and the current state of a clock's control knobs, update
/// any internal state of the clock.
pub trait UpdateClock {
    fn update(&mut self, knobs: &[Knob], dt: DeltaT);
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
    input_node: ClockNodeIndex,
}

impl ClockInputSocket {
    /// Fetch the upstream value from the source for this socket.
    fn get_value(&self, g: &ClockGraph) -> ClockValue {
        g.get_value_from_node(self.input_node)
    }
}


/// Message type to convey error conditions related to clock network operations.
pub enum ClockMessage {
    WouldCycle { source: ClockNodeIndex, sink: ClockNodeIndex },
    InvalidInputId(InputId),
}