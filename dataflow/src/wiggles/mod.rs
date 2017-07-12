use std::collections::HashMap;
use serde::Deserializer;
use serde_json::{self, Error as SerdeJsonError};
use self::serde::SerializableWiggle;
use serde::de::Error;

pub mod wiggle;
mod serde;
pub mod trial;
pub mod blender;
pub mod fanner;

pub use self::wiggle::{
    Wiggle,
    WiggleId,
    CompleteWiggle,
    WiggleKnobAddr,
    WiggleNetwork,
    KnobAddr,
    WiggleCollection,
};

// Gather every wiggle declaration up here.
// We could potentially make this mutable and provide a registration function if we want to be able
// to load wiggle defintions after compile time.  For now, we'll just keep it static.
// This collection serves as both a registry to every class of wiggle and how it is created, and
// enables serialization and deserialization of those wiggles once they are hidden behind trait
// objects.

lazy_static! {
    pub static ref WIGGLES: Vec<&'static str> = vec!(
        trial::CLASS,
    );
}

/// Return an initialized wiggle with the provieded name, if the class matches a registered one.
/// Return None if the class is unknown.
pub fn new_wiggle<N: Into<String>>(class: &str, name: N) -> Option<Box<CompleteWiggle>> {
    match class {
        trial::CLASS => {
            Some(Box::new(trial::TestWiggle::new(name)))
        }
        _ => None,
    }
}

/// Deserialize a wiggle that has been serialized using our janky mechanism.
pub fn deserialize(wiggle: SerializableWiggle) -> Result<Box<CompleteWiggle>, SerdeJsonError>
{
    match wiggle.class.as_str() {
        trial::CLASS => {
            let result: Result<trial::TestWiggle, _> = serde_json::from_str(&wiggle.serialized); 
            handle_deserialize_result(result)
        }
        _ => Err(SerdeJsonError::custom(format!("Unknown wiggle class: '{}'.", wiggle.class))),
    }
}

fn handle_deserialize_result<T>(
    result: Result<T, SerdeJsonError>) -> Result<Box<CompleteWiggle>, SerdeJsonError>
    where T: 'static + CompleteWiggle
{
    match result {
        Ok(deserialized) => Ok(Box::new(deserialized)),
        Err(e) => Err(SerdeJsonError::custom(e)),
    }
}