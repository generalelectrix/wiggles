//! The definition of a generic dataflow network.
//! Each node can be driven by zero or more clocks as well as upstream dataflow nodes.
//! Nodes have no access to upstream data during the update step, but can access their upstream
//! dependencies during rendering.
use std::ops::Deref;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, IndexType, DefaultIx};
use petgraph::algo::has_path_connecting;
use petgraph::Direction;

use knob::Knob;
use clock_network::{ClockNetwork, ClockNodeIndex};
use self::data::*;

mod data;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
/// Newtype declaration to ensure we don't mix up nodes between different graph domains.
pub struct DataNodeIndex(NodeIndex);

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

pub type InputId = usize;

pub struct DataInputSocket {
    /// The local name of this input socket.
    name: &'static str,
    /// A locally-unique numeric id for this socket.  For each node, these should
    /// start at 0 and increase monotonically.
    id: InputId,
    /// The index of the source node.
    pub input_node: DataNodeIndex,
}

pub enum DataClockInputNode {
    Clock(ClockNodeIndex),
    Data{node: DataNodeIndex, input: InputId, clock_input: bool}
}

pub struct DataClockInputSocket {
    /// The local name of this input socket.
    name: &'static str,
    /// A locally-unique numeric id for this socket.  For each node, these should
    /// start at 0 and increase monotonically.
    id: InputId,
    /// The index of the source; if a DataNodeIndex, this node is slaving itself
    /// to another node's input.
    pub input_node: DataClockInputNode,
}



pub struct DataNetwork {}

pub struct ComputeDataReqs<'a> {
    clock_inputs: &'a [DataClockInputSocket],
    data_inputs: &'a [DataInputSocket],
    knobs: &'a [Knob],
    cg: &'a ClockNetwork,
    dg: &'a DataNetwork,
}

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

enum Bar {
    Hello,
    Goodbye,
}

impl Bar {
    fn hi(&self) {}
}

struct Test<T> {
    foo: T,
}

fn test() {
    let a = Test { foo: Bar::Hello };
    a.foo.hi();
}
