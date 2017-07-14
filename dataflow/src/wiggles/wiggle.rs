//! Dataflow node that generates/propagates/mutates a wiggle.
use util::{modulo_one, almost_eq, angle_almost_eq};
use network::{Network, NodeIndex, GenerationId, NodeId, OutputId, Inputs, Outputs};
use console_server::reactor::Messages;
use wiggles_value::{Data, Unipolar, Datatype};
use wiggles_value::knob::{Knobs, Response as KnobResponse};
use std::collections::HashMap;
use std::time::Duration;
use std::fmt;
use std::any::Any;
use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde_json::{Error as SerdeJsonError, self};
use super::serde::SerializableWiggle;
use clocks::clock::{ClockId, ClockProvider};

pub type KnobAddr = u32;

// We need to qualify the knob's address with the wiggle's address to go up into the network.
pub type WiggleKnobAddr = (WiggleId, KnobAddr);

pub trait WiggleProvider {
    fn get_value(
        &self,
        wiggle_id: WiggleId,
        output_id: OutputId,
        phase_offset: f64,
        type_hint: Option<Datatype>,
        clocks: &ClockProvider)
        -> Data;
}

pub trait Wiggle {
    /// A string name for this kind of wiggle.
    /// This string will be used during serialization and deserialization to uniquely identify
    /// how to reconstruct this wiggle from a serialized form.
    fn kind(&self) -> &'static str;

    /// Return the name that has been assigned to this wiggle.
    fn name(&self) -> &str;

    /// Rename this wiggle.
    fn set_name(&mut self, name: String);

    /// Update the state of this wiggle using the provided update interval.
    /// Return a message collection of some kind.
    fn update(&mut self, dt: Duration) -> Messages<KnobResponse<KnobAddr>>;

    /// Render the state of this wiggle, providing its currently-assigned inputs as well as a
    /// function that can be used to retrieve the current value of one of those inputs.
    /// Specify which output port this wiggle should be rendered for.
    /// Also provide access to the clock network if this node needs it.
    fn render(
        &self,
        phase_offset: f64,
        type_hint: Option<Datatype>,
        inputs: &[Option<(WiggleId, OutputId)>],
        output: OutputId,
        network: &WiggleProvider,
        clocks: &ClockProvider)
        -> Data;

    /// Return Ok if this wiggle uses a clock input, and return the current value of it.
    /// If it doesn't use a clock, return Err.
    fn clock_source(&self) -> Result<Option<ClockId>, ()>;

    /// Set the clock source for this wiggle.
    /// If this wiggle doesn't use a clock, return Err.
    fn set_clock(&mut self, source: Option<ClockId>) -> Result<(), ()>;

    /// Serialize yourself into JSON.
    /// Every wiggle must implement this separately until an erased_serde solution is back in
    /// action.
    fn as_json(&self) -> Result<String, SerdeJsonError>;

    fn serializable(&self) -> Result<SerializableWiggle, SerdeJsonError> {
        Ok(SerializableWiggle {
            kind: self.kind().to_string(),
            serialized: self.as_json()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WiggleId(NodeIndex, GenerationId);

impl fmt::Display for WiggleId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wiggle {}, generation {}", self.0, self.1)
    }
}

impl NodeId for WiggleId {
    fn new(idx: NodeIndex, gen_id: GenerationId) -> Self {
        WiggleId(idx, gen_id)
    }
    fn index(&self) -> NodeIndex {
        self.0
    }
    fn gen_id(&self) -> GenerationId {
        self.1
    }
}

/// Type alias for a network of wiggles.
pub type WiggleNetwork = Network<Box<CompleteWiggle>, WiggleId, KnobResponse<WiggleKnobAddr>>;

impl WiggleProvider for WiggleNetwork {
    fn get_value(
        &self,
        wiggle_id: WiggleId,
        output_id: OutputId,
        phase_offset: f64,
        type_hint: Option<Datatype>,
        clocks: &ClockProvider)
        -> Data
    {
        // if we don't have this node, return a default
        match self.node(wiggle_id) {
            Err(e) => {
                error!("Error while trying to get wiggle from {}: {}.", wiggle_id, e);
                Data::default_with_type_hint(type_hint)
            }
            Ok(node) => {
                node.inner().render(phase_offset, type_hint, node.inputs(), output_id, self, clocks)
            }
        }
    }
}

pub trait CompleteWiggle:
    Wiggle
    + Inputs<KnobResponse<WiggleKnobAddr>, WiggleId>
    + Outputs<KnobResponse<WiggleKnobAddr>, WiggleId>
    + Knobs<KnobAddr>
    + fmt::Debug
{
    fn eq(&self, other: &CompleteWiggle) -> bool;
    fn as_any(&self) -> &Any;
}

impl<T> CompleteWiggle for T
    where T: 'static
        + Wiggle
        + Inputs<KnobResponse<WiggleKnobAddr>, WiggleId>
        + Outputs<KnobResponse<WiggleKnobAddr>, WiggleId>
        + Knobs<KnobAddr>
        + fmt::Debug
        + PartialEq
{
    fn eq(&self, other: &CompleteWiggle) -> bool {
        other.as_any().downcast_ref::<T>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any {
        self
    }

}

impl<'a, 'b> PartialEq<CompleteWiggle+'b> for CompleteWiggle + 'a {
    fn eq(&self, other: &(CompleteWiggle+'b)) -> bool {
        CompleteWiggle::eq(self, other)
    }
}

impl Outputs<KnobResponse<WiggleKnobAddr>, WiggleId> for Box<CompleteWiggle> {
    fn default_output_count(&self) -> u32 {
        (**self).default_output_count()
    }
    fn try_push_output(
            &mut self, node_id: WiggleId) -> Result<Messages<KnobResponse<WiggleKnobAddr>>, ()> {
        (**self).try_push_output(node_id)
    }
    fn try_pop_output(
            &mut self, node_id: WiggleId) -> Result<Messages<KnobResponse<WiggleKnobAddr>>, ()> {
        (**self).try_pop_output(node_id)
    }
}

// TODO: consider generalizing Update and/or Render as traits.
/// Wrapper trait for a wiggle network.
pub trait WiggleCollection {
    fn update(&mut self, dt: Duration) -> Messages<KnobResponse<WiggleKnobAddr>>;
}

impl WiggleCollection for WiggleNetwork {
    fn update(&mut self, dt: Duration) -> Messages<KnobResponse<WiggleKnobAddr>> {
        let mut update_messages = Messages::none();
        {
            let update = |node_id: WiggleId, wiggle: &mut Box<CompleteWiggle>| {
                // lift the address of this message up into the network address space
                let address_lifter = |knob_num| (node_id, knob_num);
                let mut messages = wiggle.update(dt);
                for message in messages.drain() {
                    let lifted_message = message.lift_address(&address_lifter);
                    (&mut update_messages).push(lifted_message);
                }
            };
            self.map_inner(update);
        }
        update_messages
    }
}
