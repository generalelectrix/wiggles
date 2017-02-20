//! A small framework for defining a builder object for a particular kind of clock.
use std::cell::Cell;

use petgraph::graph::NodeIndex;

use knob::Knob;
use network::InputId;
use super::{
    CompleteClock,
    ClockNode,
    ClockNodeIndex,
    ClockError,
    ClockInputSocket,
};

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