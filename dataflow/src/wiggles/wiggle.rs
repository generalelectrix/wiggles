//! Dataflow node that generates/propagates/mutates a wiggle.
use util::{modulo_one, almost_eq, angle_almost_eq};
use network::{Network, NodeIndex, GenerationId, NodeId, Inputs};
use console_server::reactor::Messages;
use wiggles_value::{Data, Unipolar};
use wiggles_value::knob::{Knobs, Message as KnobMessage};
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
    //fn get_value(&self, wiggle_id: WiggleId) -> WiggleValue;
}

pub trait Wiggle {
    /// A string name for this class of wiggle.
    /// This string will be used during serialization and deserialization to uniquely identify
    /// how to reconstruct this wiggle from a serialized form.
    fn class(&self) -> &'static str;

    /// Return the name that has been assigned to this wiggle.
    fn name(&self) -> &str;

    /// Update the state of this wiggle using the provided update interval.
    /// Return a message collection of some kind.
    fn update(&mut self, dt: Duration) -> Messages<Message<KnobAddr>>;

    /// Render the state of this wiggle, providing its currently-assigned inputs as well as a
    /// function that can be used to retrieve the current value of one of those inputs.
    /// Also provide access to the clock network if this node needs it.
    fn render(
        &self,
        phase_offset: Unipolar,
        inputs: &[Option<WiggleId>],
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
            class: self.class().to_string(),
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
pub type WiggleNetwork = Network<Box<CompleteWiggle>, WiggleId, Message<WiggleKnobAddr>>;

// TODO: refactor to eliminate this?  Unclear if we need other messages at this layer of the stack.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Concrete message type used by the wiggle network.
/// Includes messages related to the knob system.
pub enum Message<A> {
    Knob(KnobMessage<A>),
}


impl<A> Message<A> {
    /// Use the provided function to lift this wiggle message into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> Message<NewAddr>
        where F: FnOnce(A) -> NewAddr, NewAddr: Copy
    {
        use self::Message::*;
        match self {
            Knob(ka) => Knob(ka.lift_address(lifter)),
        }
    }
}


pub trait CompleteWiggle: Wiggle + Inputs<Message<WiggleKnobAddr>> + Knobs<KnobAddr> + fmt::Debug {
    fn eq(&self, other: &CompleteWiggle) -> bool;
    fn as_any(&self) -> &Any;
}

impl<T> CompleteWiggle for T
    where T: 'static + Wiggle + Inputs<Message<WiggleKnobAddr>> + Knobs<KnobAddr> + fmt::Debug + PartialEq
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

// TODO: consider generalizing Update and/or Render as traits.
/// Wrapper trait for a wiggle network.
pub trait WiggleCollection {
    fn update(&mut self, dt: Duration) -> Messages<Message<WiggleKnobAddr>>;
}

impl WiggleCollection for WiggleNetwork {
    fn update(&mut self, dt: Duration) -> Messages<Message<WiggleKnobAddr>> {
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
