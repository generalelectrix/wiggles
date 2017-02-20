//! The definition of a generic dataflow network.
//! Each node can be driven by zero or more clocks as well as upstream dataflow nodes.
//! Nodes have no access to upstream data during the update step, but can access their upstream
//! dependencies during rendering.
use std::{fmt, error, ops};
use petgraph::graph::{NodeIndex, IndexType, DefaultIx};

use datatypes::{DeltaT, Update};
use event::Events;
use network::{
    InputId,
    InputSocket,
    NetworkNode,
    NetworkNodeId,
    Network,
    NetworkError,
    NetworkEvent,
};
use knob::{Knob, Knobs};
use clock_network::{ClockNetwork, ClockNodeIndex, ClockInputSocket, ClockNetworkError};
use self::data::*;

mod data;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct DataNodeIndex(NodeIndex);

impl From<NodeIndex> for DataNodeIndex {
    fn from(n: NodeIndex) -> Self {
        DataNodeIndex(n)
    }
}

impl ops::Deref for DataNodeIndex {
    type Target = NodeIndex;
    fn deref(&self) -> &NodeIndex { &self.0 }
}

/// This implementation is not unsafe, though since the trait is unsafe the unsafety
/// here is only coming from our reliance on the underlying type itself.
unsafe impl IndexType for DataNodeIndex {
    #[inline(always)]
    fn new(x: usize) -> Self { DataNodeIndex(NodeIndex::new(x)) }
    #[inline(always)]
    fn index(&self) -> usize {
        let DataNodeIndex(idx) = *self;
        idx.index() }
    #[inline(always)]
    fn max() -> Self { DataNodeIndex::new(DefaultIx::max() as usize) }
}

pub type DataInputSocket = InputSocket<DataNodeIndex>;

pub type DataNetworkEvent = NetworkEvent<DataNodeIndex>;
pub type DataNetworkError = NetworkError<DataNodeIndex>;

#[derive(Debug)]
/// A single node in an arbitrary data graph, accepting inputs, listening to
/// knobs, and with a stored behavior that uses these values to produce a
/// clock value when called upon.
pub struct DataNode {
    /// Unique name for this node.
    pub name: String,
    /// The index of this node in the enclosing graph.
    /// The graph implementation must ensure that these indices remain stable.
    id: DataNodeIndex,
    /// Named input sockets that connect this node to upstream clocks.
    inputs: Vec<DataInputSocket>,
    /// Named input sockets that provide this node with clock signals.
    clock_inputs: Vec<ClockInputSocket>,
    /// Named knobs that provide control parameters.
    knobs: Vec<Knob>,
    /// The stored behavior used to provide data.  It may have internal state
    /// that will be updated during the global timestep update.
    data_provider: Box<DataProvider>,
}

impl DataNode {
    pub fn clock_input_sockets(&self) -> &[ClockInputSocket] {
        &self.clock_inputs
    }

    pub fn clock_input_socket(&self, id: InputId) -> Result<&ClockInputSocket, DataflowError> {
        self.clock_inputs.get(id).ok_or(DataflowError::InvalidClockInputId(self.id, id))
    }

    pub fn clock_input_socket_mut(
            &mut self, id: InputId) -> Result<&mut ClockInputSocket, DataflowError> {
        self.clock_inputs.get_mut(id).ok_or(DataflowError::InvalidClockInputId(self.id, id))
    }

    pub fn clock_input_node_ids(&self) -> Vec<ClockNodeIndex> {
        self.clock_input_sockets().iter().map(|socket| socket.input).collect()
    }
}

impl Knobs for DataNode {
    fn knobs(&self) -> &[Knob] { &self.knobs }
    fn knobs_mut(&mut self) -> &mut [Knob] { &mut self.knobs }
}

impl NetworkNode<DataNodeIndex> for DataNode {
    fn input_sockets(&self) -> &[DataInputSocket] {
        &self.inputs
    }

    fn input_sockets_mut(&mut self) -> &mut [DataInputSocket] {
        &mut self.inputs
    }

    fn id(&self) -> DataNodeIndex {
        self.id
    }
    fn set_id(&mut self, id: DataNodeIndex) {
        self.id = id
    }
}

impl Update for DataNode {
    fn update(&mut self, dt: DeltaT) -> Events {
        self.data_provider.update(self.id, &mut self.knobs, dt)
    }
}

// placeholder type for registering an external dependency on a dataflow node
pub type DataListener = usize;

pub type DataNetwork = Network<DataNode, DataNodeIndex, DataListener>;

impl DataNetwork {
    pub fn swap_clock_input(
            &mut self,
            node_index: DataNodeIndex,
            id: InputId,
            new_source: ClockNodeIndex,
            clock_network: &mut ClockNetwork)
            -> Result<DataflowEvent, DataflowError> {
        // identify the current node connected to this input
        let current_source = self.get_node(node_index)?.clock_input_socket(id)?.input;

        // register this node with the clock network as a dependency
        clock_network.add_listener(new_source, node_index)?;
        // unregister the old connection
        clock_network.remove_listener(current_source, node_index);
        
        // swap the input at the node level
        self.get_node_mut(node_index)?.clock_input_socket_mut(id)?.input = new_source;

        Ok(DataflowEvent::ClockInputSwapped{ node: node_index, input_id: id, new_input: new_source })
    }

}

pub struct ComputeDataReqs<'a> {
    clock_inputs: &'a [ClockInputSocket],
    data_inputs: &'a [DataInputSocket],
    knobs: &'a [Knob],
    cg: &'a ClockNetwork,
    dg: &'a DataNetwork,
}

pub trait DataProvider: ComputeData + UpdateData + fmt::Debug {}

pub trait ComputeData {
    /// Get this node's value with a phase offset, in whatever internal format makes sense for it,
    /// possibly delegating that format to an upstream source, or to some eventual default.
    fn get(&self, phase: Unipolar, reqs: ComputeDataReqs) -> Data;

    /// Get value with no phase offset.
    fn get_zero(&self, reqs: ComputeDataReqs) -> Data {
        self.get(Unipolar(0.0), reqs)
    }

    /// Get this node's value as a particular datatype.  This type will be passed upstream if
    /// necessary to attempt to preserve type semantics as much as possible.  The return type is
    /// left generic, so this method makes no type-level guarantee about fulfilling this contract.
    /// Actual conversions to explicit types are left up to the generic conversion method on Data;
    /// these should then fall through as no-ops where necessary.
    fn get_as_kind(&self, kind: Datatype, reqs: ComputeDataReqs) -> Data;
}

pub trait UpdateData {
    /// Update this data provider's state by a specified timestep.
    fn update(&mut self, id: DataNodeIndex, knobs: &mut [Knob], dt: DeltaT) -> Events;
}

#[derive(Debug, PartialEq)]
pub enum DataflowEvent {
    ClockInputSwapped{ node: DataNodeIndex, input_id: InputId, new_input: ClockNodeIndex },
    /// A dataflow node has been added.
    NodeAdded { node: DataNodeIndex, name: String },
    /// A node has been removed.
    NodeRemoved { node: DataNodeIndex, name: String },
    /// A node has been renamed.
    NodeRenamed { node: DataNodeIndex, name: String},
    /// An event generated by the underlying network.
    Network(DataNetworkEvent),
}

impl From<DataNetworkEvent> for DataflowEvent {
    fn from(e: DataNetworkEvent) -> Self {
        DataflowEvent::Network(e)
    }
}

#[derive(Debug)]
pub enum DataflowError {
    InvalidClockInputId(DataNodeIndex, InputId),
    Network(DataNetworkError),
    Clock(ClockNetworkError),
}

impl fmt::Display for DataflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataflowError::InvalidClockInputId(node, input) =>
                write!(f, "Invalid clock input id {:?} for data node {}.", node, input),
            DataflowError::Network(ref err) => {
                write!(f, "Data network error. ")?;
                err.fmt(f)
            }
            DataflowError::Clock(ref err) => {
                write!(f, "Clock network error resulting from dataflow operation. ")?;
                err.fmt(f)
            }
        }
    }
}

impl error::Error for DataflowError {
    // TODO: description messages, though we may never need them
    fn description(&self) -> &str { "TODO: descriptions for DataflowError" }
    fn cause(&self) -> Option<&error::Error> { None }
}

impl From<DataNetworkError> for DataflowError {
    fn from(err: DataNetworkError) -> Self {
        DataflowError::Network(err)
    }
}

impl From<ClockNetworkError> for DataflowError {
    fn from(e: ClockNetworkError) -> Self {
        DataflowError::Clock(e)
    }
}