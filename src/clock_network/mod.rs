//! Description of the contents and action of a network of related clocks.
//! Clocks can tap the output of other clocks to perform clock mutation or
//! modulation, such as multiplication and division.  Clocks can also declare
//! that they accept input from one or more "knobs", enabling push-based dataflow
//! of control parameters into the clocks.
use std::fmt;
use std::error;
use std::cell::Cell;
use std::ops::Deref;
use petgraph::graph::{NodeIndex, IndexType, DefaultIx};

use utils::{modulo_one, almost_eq};
use knob::{Knob, Knobs, KnobValue, KnobPatch, KnobEvent};
use event::{Event, Events};
use data_network::DataNodeIndex;
use datatypes::{Update, DeltaT};
use network::{Network, NetworkNode, InputId, InputSocket, NetworkError, NetworkEvent};

#[cfg(test)]
mod test;

/// A clock needs to implement these traits to function as a node in the clock network.
pub trait CompleteClock: ComputeClock + UpdateClock + fmt::Debug {}
impl<T> CompleteClock for T where T: ComputeClock + UpdateClock + fmt::Debug {}

pub type ClockInputSocket = InputSocket<ClockNodeIndex>;
pub type ClockNetworkError = NetworkError<ClockNodeIndex>;
pub type ClockNetworkEvent = NetworkEvent<ClockNodeIndex>;

#[derive(PartialEq, Debug)]
/// Events related to clocks and the clock graph.
pub enum ClockEvent {
    /// An event inherited from the underlying network.
    Network(ClockNetworkEvent),
    /// A new clock node has been added.
    NodeAdded { node: ClockNodeIndex, name: String },
    /// A clock node has been removed.
    NodeRemoved { node: ClockNodeIndex, name: String },
    /// A clock node has been renamed.
    NodeRenamed { node: ClockNodeIndex, name: String},
}

impl From<ClockNetworkEvent> for ClockEvent {
    fn from(e: ClockNetworkEvent) -> Self {
        ClockEvent::Network(e)
    }
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

    /// Assert that this clock value is equivalent to another.
    pub fn assert_almost_eq_to(&self, other: &ClockValue) {
        let clock_info_dump = format!("clock a: {:?}, clock b: {:?}", *self, *other);
        assert!(almost_eq(self.phase, other.phase),
                "clock a phase = {} but clock b phase = {}\n{}",
                self.phase,
                other.phase,
                clock_info_dump);
        assert_eq!(self.tick_count,
                   other.tick_count,
                   "clock a ticks = {} but clock b ticks = {}\n{}",
                   self.tick_count,
                   other.tick_count,
                   clock_info_dump);
        assert_eq!(self.ticked,
                   other.ticked,
                   "clock a ticked: {} but clock b ticked: {}\n{}",
                   self.ticked,
                   other.ticked,
                   clock_info_dump);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct ClockNodeIndex(pub NodeIndex);

impl From<NodeIndex> for ClockNodeIndex {
    fn from(n: NodeIndex) -> Self {
        ClockNodeIndex(n)
    }
}

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ClockListener {
    Dataflow(DataNodeIndex),
    External, // TODO: decide how to keep track/identify external listeners
}

impl From<DataNodeIndex> for ClockListener {
    fn from(node: DataNodeIndex) -> Self {
        ClockListener::Dataflow(node)
    }
}

pub type ClockNetwork = Network<ClockNode, ClockNodeIndex, ClockListener>;

impl ClockNetwork {
    /// Get the current clock value from any node in the graph.
    /// Return an error if the node doesn't exist or is invalid.
    pub fn get_value_from_node(&self, node: ClockNodeIndex) -> Result<ClockValue, ClockNetworkError> {
        Ok(self.get_node(node)?.get_value(&self))
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
                                ClockInputSocket::new(name, *node_id)
                            })
                       .collect::<Vec<_>>();
        Ok(ClockNode {
            name: name,
            id: ClockNodeIndex(NodeIndex::new(0)), // use a placeholder id for now
            inputs: connected_inputs,
            knobs: self.knobs.clone().into_vec(),
            current_value: Cell::new(None),
            clock: (self.clock)(),
        })
    }
}

#[derive(Debug)]
/// A single node in an arbitrary clock graph, accepting inputs, listening to
/// knobs, and with a stored behavior that uses these values to produce a
/// clock value when called upon.
pub struct ClockNode {
    /// Unique name for this clock.
    pub name: String,
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

impl NetworkNode<ClockNodeIndex> for ClockNode {
    fn input_sockets(&self) -> &[ClockInputSocket] {
        &self.inputs
    }

    fn input_socket(&self, id: InputId) -> Result<&ClockInputSocket, ClockNetworkError> {
        self.inputs.get(id).ok_or(NetworkError::InvalidInputId(self.id(), id))
    }

    fn input_socket_mut(
            &mut self, id: InputId) -> Result<&mut ClockInputSocket, ClockNetworkError> {
        self.inputs.get_mut(id).ok_or(NetworkError::InvalidInputId(self.id, id))
    }

    fn id(&self) -> ClockNodeIndex {
        self.id
    }
    fn set_id(&mut self, id: ClockNodeIndex) {
        self.id = id
    }
}

impl ClockNode {
    /// Return the current value of this clock node.
    pub fn get_value(&self, g: &ClockNetwork) -> ClockValue {
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
    fn update(&mut self, dt: DeltaT) -> Events {
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
    let patch = KnobPatch::new(node, knob.id());
    let value = KnobValue::Button(state);
    KnobEvent::ValueChanged { patch: patch, value: value }.into()
}

/// Given a timestep and the current state of a clock's control knobs, update
/// any internal state of the clock.  Updating a clock may require emitting
/// some kind of state update announcement, so allow returning a collection
/// of events.  The id of the node that owns this clock is passed in as it
/// is needed for emitting certain events.
pub trait UpdateClock {
    fn update(&mut self, id: ClockNodeIndex, knobs: &mut [Knob], dt: DeltaT) -> Events;
}

/// Given some inputs and knobs, compute a clock value.
pub trait ComputeClock {
    fn compute_clock(&self,
                     inputs: &[ClockInputSocket],
                     knobs: &[Knob],
                     g: &ClockNetwork)
                     -> ClockValue;
}

#[derive(Debug)]
/// Message type to convey error conditions related to clock network operations.
pub enum ClockError {
    MismatchedInputs { type_name: &'static str, expected: usize, provided: usize },
    UnknownPrototype(String),
    Network(ClockNetworkError),
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClockError::MismatchedInputs { type_name, expected, provided } =>
                write!(f, "Clock type {} expects {} inputs but was provided {}.", type_name, expected, provided),
            ClockError::UnknownPrototype(ref name) =>
                write!(f, "Unknown clock node prototype: '{}'.", name),
            ClockError::Network(ref err) => {
                write!(f, "Clock network error. ")?;
                err.fmt(f)
            }
        }
    }
}

impl error::Error for ClockError {
    // TODO: description messages, though we may never need them
    fn description(&self) -> &str { "TODO: descriptions for ClockError" }
    fn cause(&self) -> Option<&error::Error> { None }
}