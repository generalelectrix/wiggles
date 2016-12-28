use petgraph::stable_graph::StableDiGraph;
use petgraph::graph::{IndexType, NodeIndex, EdgeIndex};
use std::marker::PhantomData;

/// Dataflow graphs just use edges as dumb wires, so they carry unit weight.
/// The weight of the node is the particular implementation of dataflow
/// processing, exposing whatever API is represented by the type T.
type DataflowGraph<T> = StableDiGraph<DataflowNode<T>, ()>;

type InputId = usize;

/// A node in a generic dataflow graph.
/// Nodes can
pub struct DataflowNode<T> {
    name: String,
    id: NodeIndex,
    inputs: Box<[InputSocket<T>]>,
    behavior: &'static Fn(&[InputSocket<T>]) -> T,
}

pub enum DataflowMessage {
    WouldCycle { source: NodeIndex, sink: NodeIndex },
    InvalidInputId(InputId),
}

pub struct InputSocket<T> {
    name: &'static str,
    input_edge: EdgeIndex,
    _marker: PhantomData<T>
}

impl<T> InputSocket<T> {
    fn get_source(&self, g: DataflowGraph<T>) -> &DataflowNode<T> {
        let (source_id, _) = g.edge_endpoints(self.input_edge).unwrap();
        g.node_weight(source_id).unwrap()
    }
}
