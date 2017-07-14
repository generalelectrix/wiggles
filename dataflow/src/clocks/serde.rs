//! Implement serialization and deserialization of clocks hidden behind trait objects.
//! Unfortunately, since erased_serde has not been ported to serde 1.0 yet, there is no general-
//! purpose mechanism for serializing a trait object, as Serialize is not object-safe :(
//! As a shitty, horrible hack, we use the JSON serializer to serialize the object to a string,
//! and then serialize that string inside whatever other serialization format we're using.
//! Using JSON here ensures that all quasi-human-readable saves are still quasi-human-readable.
use super::clock::{CompleteClock, KnobAddr};
use super::{deserialize as deserialize_clock};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::{Error as SerError};
use serde::de::{Error as DeError};

#[derive(Debug, Serialize, Deserialize)]
/// A clock in JSON-serialized form.
pub struct SerializableClock {
    pub kind: String,
    pub serialized: String,
}

impl Serialize for Box<CompleteClock> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {   
        match self.serializable() {
            Err(e) => Err(S::Error::custom(e)),
            Ok(s) => s.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Box<CompleteClock> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let serialized_clock = SerializableClock::deserialize(deserializer)?;
        deserialize_clock(serialized_clock).map_err(D::Error::custom)
    }
}
