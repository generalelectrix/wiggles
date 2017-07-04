use std::collections::HashMap;
use self::clock::CompleteClock;
use serde::Deserializer;
use serde_json::{self, Error as SerdeJsonError};
use self::serde::SerializableClock;
use serde::de::Error;

pub mod clock;
pub mod simple;
pub mod multiplier;
mod serde;

// Gather every clock declaration up here.
// We could potentially make this mutable and provide a registration function if we want to be able
// to load clock defintions after compile time.  For now, we'll just keep it static.
// This collection serves as both a registry to every class of clock and how it is created, and
// enables serialization and deserialization of those clocks once they are hidden behind trait
// objects.

lazy_static! {
    static ref CLOCKS: Vec<&'static str> = vec!(
        simple::CLASS,
        multiplier::CLASS,
    );
}

/// Return an initialized clock with the provieded name, if the class matches a registered one.
/// Return None if the class is unknown.
pub fn new_clock<N: Into<String>>(class: &str, name: N) -> Option<Box<CompleteClock>> {
    match class {
        simple::CLASS => Some(Box::new(simple::SimpleClock::new(name))),
        multipler::CLASS => Some(Box::new(multiplier::ClockMultiplier::new(name))),
        _ => None,
    }
}

/// Deserialize a clock that has been serialized using our janky mechanism.
pub fn deserialize(clock: SerializableClock) -> Result<Box<CompleteClock>, SerdeJsonError>
{
    match clock.class.as_str() {
        simple::CLASS => {
            let result: Result<simple::SimpleClock, _> = serde_json::from_str(&clock.serialized); 
            handle_deserialize_result(result)
        }
        multiplier::CLASS => {
            let result: Result<multiplier::ClockMultiplier, _> = serde_json::from_str(&clock.serialized); 
            handle_deserialize_result(result)
        }
        _ => Err(SerdeJsonError::custom(format!("Unknown clock class: '{}'.", clock.class))),
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