//! The definition of a generic dataflow network.
//! Each node can be driven by zero or more clocks as well as upstream dataflow nodes.
//! Nodes have no access to upstream data during the update step, but can access their upstream
//! dependencies during rendering.
use std::fmt::Debug;
use std::ops::Deref;
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
use knob::Knob;
use clock_network::{ClockNetwork, ClockNodeIndex};
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

impl Deref for DataNodeIndex {
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

#[derive(Debug)]
pub enum DataClockInputNode {
    Clock(ClockNodeIndex),
    Data{node: DataNodeIndex, input: InputId, clock_input: bool}
}

pub type DataClockInputSocket = InputSocket<DataClockInputNode>;
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
    clock_inputs: Vec<DataClockInputSocket>,
    /// Named knobs that provide control parameters.
    knobs: Vec<Knob>,
    /// The stored behavior used to provide data.  It may have internal state
    /// that will be updated during the global timestep update.
    data_provider: Box<DataProvider>,
}

impl NetworkNode<DataNodeIndex> for DataNode {
    fn input_sockets(&self) -> &[DataInputSocket] {
        &self.inputs
    }

    fn input_socket(&self, id: InputId) -> Result<&DataInputSocket, DataNetworkError> {
        self.inputs.get(id).ok_or(NetworkError::InvalidInputId(self.id(), id))
    }

    fn input_socket_mut(
            &mut self, id: InputId) -> Result<&mut DataInputSocket, DataNetworkError> {
        self.inputs.get_mut(id).ok_or(NetworkError::InvalidInputId(self.id, id))
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

pub type DataNetwork = Network<DataNode, DataNodeIndex>;

pub struct ComputeDataReqs<'a> {
    clock_inputs: &'a [DataClockInputSocket],
    data_inputs: &'a [DataInputSocket],
    knobs: &'a [Knob],
    cg: &'a ClockNetwork,
    dg: &'a DataNetwork,
}

pub trait DataProvider: ComputeData + UpdateData + Debug {}

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
