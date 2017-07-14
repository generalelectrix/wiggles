use std::collections::HashMap;
use serde::Deserializer;
use serde_json::{self, Error as SerdeJsonError};
use self::serde::SerializableClock;
use serde::de::Error;

pub mod clock;
pub mod simple;
pub mod multiplier;
mod serde;

pub use self::clock::{
    Clock,
    ClockId,
    CompleteClock,
    ClockKnobAddr,
    ClockValue,
    ClockNetwork,
    KnobAddr,
    ClockCollection,
};

// Gather every clock declaration up here.
// We could potentially make this mutable and provide a registration function if we want to be able
// to load clock defintions after compile time.  For now, we'll just keep it static.
// This collection serves as both a registry to every kind of clock and how it is created, and
// enables serialization and deserialization of those clocks once they are hidden behind trait
// objects.

lazy_static! {
    pub static ref CLOCKS: Vec<&'static str> = vec!(
        simple::KIND,
        multiplier::KIND,
    );
}

/// Return an initialized clock with the provided name, if the kind matches a registered one.
/// Return None if the kind is unknown.
pub fn new_clock<N: Into<String>>(kind: &str, name: N) -> Option<Box<CompleteClock>> {
    match kind {
        simple::KIND => Some(Box::new(simple::SimpleClock::new(name))),
        multiplier::KIND => Some(Box::new(multiplier::ClockMultiplier::new(name))),
        _ => None,
    }
}

/// Deserialize a clock that has been serialized using our janky mechanism.
pub fn deserialize(clock: SerializableClock) -> Result<Box<CompleteClock>, SerdeJsonError>
{
    match clock.kind.as_str() {
        simple::KIND => {
            let result: Result<simple::SimpleClock, _> = serde_json::from_str(&clock.serialized); 
            handle_deserialize_result(result)
        }
        multiplier::KIND => {
            let result: Result<multiplier::ClockMultiplier, _> = serde_json::from_str(&clock.serialized); 
            handle_deserialize_result(result)
        }
        _ => Err(SerdeJsonError::custom(format!("Unknown clock kind: '{}'.", clock.kind))),
    }
}

fn handle_deserialize_result<T>(
    result: Result<T, SerdeJsonError>) -> Result<Box<CompleteClock>, SerdeJsonError>
    where T: 'static + CompleteClock
{
    match result {
        Ok(deserialized) => Ok(Box::new(deserialized)),
        Err(e) => Err(SerdeJsonError::custom(e)),
    }
}