//! Clock abstraction for Wiggles.
//! Implementors will be wrapped up as trait objects and injected into a dataflow network.
use super::util::{modulo_one, almost_eq};
use super::network::{Network, NodeIndex, GenerationId, NodeId, Inputs};
use std::time::Duration;
use std::fmt;

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

trait ClockProvider {
    fn get_value(&self, clock_id: ClockId) -> ClockValue;
}

pub trait Clock<Message>: Inputs<Message> + fmt::Debug
{
    /// A string name for this class of clock.
    /// This string will be used during serialization and deserialization to uniquely identify
    /// how to reconstruct this clock from a serialized form.
    fn class(&self) -> &'static str;

    /// Return the name that has been assigned to this clock.
    fn name(&self) -> &str;

    /// Update the state of this clock using the provided update interval.
    /// Return a message collection of some kind.
    fn update(&mut self, dt: Duration) -> Message;

    /// Render the state of this clock, providing its currently-assigned inputs as well as a
    /// function that can be used to retrieve the current value of one of those inputs.
    fn render(&self, inputs: &[Option<ClockId>], network: &ClockProvider) -> ClockValue;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

type ClockNetwork<M> = Network<Box<Clock<M>>, ClockId, M>;