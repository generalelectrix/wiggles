//! Clock abstraction for Wiggles.
//! Implementors will be wrapped up as trait objects and injected into a dataflow network.
use util::{modulo_one, almost_eq, angle_almost_eq};
use network::{Network, NodeIndex, GenerationId, NodeId, Inputs};
use console_server::reactor::Messages;
use wiggles_value::Unipolar;
use wiggles_value::knob::{Knobs, Message as KnobMessage};
use std::collections::HashMap;
use std::time::Duration;
use std::fmt;
use std::any::Any;
use serde::{Serialize, Serializer};
use serde::de::DeserializeOwned;
use serde_json::{Error as SerdeJsonError, self};
use super::serde::SerializableClock;

// Individual clocks just use numeric indices for their knobs.
pub type KnobAddr = u32;

// We need to qualify the knob's address with the clock's address to go up into the network.
pub type ClockKnobAddr = (ClockId, KnobAddr);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
/// Represent the complete value of the current state of a clock.
pub struct ClockValue {
    /// Floating-point phase.  Setter method always ensures this is wrapped, so clients creating
    /// a clock value do not need to ensure it is wrapped as that is handled internally.
    phase: f64,
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
    /// Used for testing clock operations.
    pub fn assert_almost_eq_to(&self, other: ClockValue) {
        let clock_info_dump = format!("clock a: {:?}, clock b: {:?}", *self, other);
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

    /// Get the phase of this clock.
    pub fn phase(&self) -> Unipolar {
        Unipolar(self.phase)
    }

    /// Set the phase of this clock value, wrapping the value to be unit.
    pub fn set_phase(&mut self, phase: f64) {
        self.phase = modulo_one(phase);
    }

    /// Return this clock value's phase, shifted by the provided amount.
    /// The output is always in range.
    pub fn phase_shift(&self, Unipolar(offset): Unipolar) -> Unipolar {
        Unipolar(modulo_one(self.phase + offset))
    }
}

impl Default for ClockValue {
    /// Placeholder default for ClockValue.
    fn default() -> Self {
        ClockValue {
            phase: 0.0,
            tick_count: 0,
            ticked: false,
        }
    }
}

impl PartialEq for ClockValue {
    fn eq(&self, other: &ClockValue) -> bool {
        angle_almost_eq(self.phase, other.phase)
        && self.tick_count == other.tick_count
        && self.ticked == other.ticked
    }
}

pub trait ClockProvider {
    fn get_value(&self, clock_id: ClockId) -> ClockValue;
}

pub trait Clock {
    /// A string name for this class of clock.
    /// This string will be used during serialization and deserialization to uniquely identify
    /// how to reconstruct this clock from a serialized form.
    fn class(&self) -> &'static str;

    /// Return the name that has been assigned to this clock.
    fn name(&self) -> &str;

    /// Update the state of this clock using the provided update interval.
    /// Return a message collection of some kind.
    fn update(&mut self, dt: Duration) -> Messages<KnobMessage<KnobAddr>>;

    /// Render the state of this clock, providing its currently-assigned inputs as well as a
    /// function that can be used to retrieve the current value of one of those inputs.
    fn render(&self, inputs: &[Option<ClockId>], network: &ClockProvider) -> ClockValue;

    /// Serialize yourself into JSON.
    /// Every clock must implement this separately until an erased_serde solution is back in
    /// action.
    fn as_json(&self) -> Result<String, SerdeJsonError>;

    fn serializable(&self) -> Result<SerializableClock, SerdeJsonError> {
        Ok(SerializableClock {
            class: self.class().to_string(),
            serialized: self.as_json()?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockId(NodeIndex, GenerationId);

impl fmt::Display for ClockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "clock node {}, generation {}", self.0, self.1)
    }
}

impl NodeId for ClockId {
    fn new(idx: NodeIndex, gen_id: GenerationId) -> Self {
        ClockId(idx, gen_id)
    }
    fn index(&self) -> NodeIndex {
        self.0
    }
    fn gen_id(&self) -> GenerationId {
        self.1
    }
}

/// Type alias for a network of clocks.
pub type ClockNetwork = Network<Box<CompleteClock>, ClockId, KnobMessage<ClockKnobAddr>>;

impl ClockProvider for ClockNetwork {
    /// Get the value of the requested clock.
    /// If it is missing, log an error and return a default.
    fn get_value(&self, clock_id: ClockId) -> ClockValue {
        match self.node(clock_id) {
            Err(e) => {
                error!("Error while trying to get clock value from {}: {}.", clock_id, e);
                ClockValue::default()
            }
            Ok(node) => {
                node.inner().render(node.inputs(), self)
            }
        }
    }
}

pub trait CompleteClock:
    Clock + Inputs<KnobMessage<ClockKnobAddr>> + Knobs<KnobAddr> + fmt::Debug
{
    fn eq(&self, other: &CompleteClock) -> bool;
    fn as_any(&self) -> &Any;
}

impl<T> CompleteClock for T
    where T: 'static + Clock
        + Inputs<KnobMessage<ClockKnobAddr>>
        + Knobs<KnobAddr>
        + fmt::Debug
        + PartialEq
{
    fn eq(&self, other: &CompleteClock) -> bool {
        other.as_any().downcast_ref::<T>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &Any {
        self
    }

}

impl<'a, 'b> PartialEq<CompleteClock+'b> for CompleteClock + 'a {
    fn eq(&self, other: &(CompleteClock+'b)) -> bool {
        CompleteClock::eq(self, other)
    }
}

// TODO: consider generalizing Update and/or Render as traits.
/// Wrapper trait for a clock network.
pub trait ClockCollection {
    fn update(&mut self, dt: Duration) -> Messages<KnobMessage<ClockKnobAddr>>;
}

impl ClockCollection for ClockNetwork {
    fn update(&mut self, dt: Duration) -> Messages<KnobMessage<ClockKnobAddr>> {
        let mut update_messages = Messages::none();
        {
            let update = |node_id: ClockId, clock: &mut Box<CompleteClock>| {
                // lift the address of this message up into the network address space
                let address_lifter = |knob_num| (node_id, knob_num);
                let mut messages = clock.update(dt);
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

