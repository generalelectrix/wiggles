//! Clock abstraction for Wiggles.
//! Implementors will be wrapped up as trait objects and injected into a dataflow network.
use util::{modulo_one, almost_eq};
use network::{Network, NodeIndex, GenerationId, NodeId, Inputs};
use console_server::reactor::Messages;
use wiggles_value::knob::{Knobs, Message as KnobMessage};
use std::collections::HashMap;
use std::time::Duration;
use std::fmt;
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
    pub phase: f64,
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
    fn update(&mut self, dt: Duration) -> Messages<Message<KnobAddr>>;

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

impl<N> ClockProvider for Network<N, ClockId, Message<ClockKnobAddr>>
    where N: Clock + fmt::Debug + Inputs<Message<ClockKnobAddr>>
{
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Concrete message type used by the clock network.
/// Includes messages related to the knob system.
pub enum Message<A> {
    Knob(KnobMessage<A>),
}


pub trait CompleteClock: Clock + Inputs<Message<ClockKnobAddr>> + Knobs<KnobAddr> + fmt::Debug {}

impl<T> CompleteClock for T
    where T: Clock + Inputs<Message<ClockKnobAddr>> + Knobs<KnobAddr> + fmt::Debug {}


