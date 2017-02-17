//! The definition of a generic dataflow network.
//! Each node can be driven by zero or more clocks as well as upstream dataflow nodes.
//! Nodes have no access to upstream data during the update step, but can access their upstream
//! dependencies during rendering.
use std::ops::Deref;
use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{NodeIndex, IndexType, DefaultIx};
use petgraph::algo::has_path_connecting;
use petgraph::Direction;

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