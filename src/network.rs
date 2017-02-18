//! Attempt to generalize some features of dataflow network into generic types.

pub type InputId = usize;

#[derive(Debug)]
pub struct InputSocket<T> {
    /// The local name of this input socket.
    name: &'static str,
    /// A locally-unique numeric id for this socket.  For each node, these should
    /// start at 0 and increase monotonically.
    id: InputId,
    /// Some identifier providing functionality to perform access on a graph.
    pub input: T,
}

impl<T> InputSocket<T> {
    pub fn new(name: &'static str, id: InputId, input: T) -> Self {
        InputSocket { name: name, id: id, input: input }
    }

    pub fn name(&self) -> &'static str { self.name }
}