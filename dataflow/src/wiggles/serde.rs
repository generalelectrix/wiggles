//! Implement serialization and deserialization of wiggles hidden behind trait objects.
//! See clocks::serde for notes on this hack.
use super::wiggle::{CompleteWiggle, KnobAddr};
use super::{deserialize as deserialize_wiggle};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::{Error as SerError};
use serde::de::{Error as DeError};

#[derive(Debug, Serialize, Deserialize)]
/// A wiggle in JSON-serialized form.
pub struct SerializableWiggle {
    pub class: String,
    pub serialized: String,
}

impl Serialize for Box<CompleteWiggle> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {   
        match self.serializable() {
            Err(e) => Err(S::Error::custom(e)),
            Ok(s) => s.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Box<CompleteWiggle> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let serialized_wiggle = SerializableWiggle::deserialize(deserializer)?;
        deserialize_wiggle(serialized_wiggle).map_err(D::Error::custom)
    }
}
