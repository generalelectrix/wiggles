//! Dmx fixture abstraction.
//! Accepts an arbitrary number of wiggles values as control parameters and
//! opaquely renders these into a DMX buffer.
use std::str::FromStr;
use std::fmt;
use std::marker::PhantomData;
use serde::{Serializer, Deserializer};
use serde::de::{self, Visitor};
use wiggles_value::{Datatype, Data};
use profiles::render_func_for_type;

pub type DmxChannelCount = u16;
pub type DmxValue = u8;

// --------------------
// Wiggles fixture control parameter
// --------------------

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A single generic control for a fixture.
/// A fixture will provide zero or more of these as its interface.
// TODO: some kind of decoration on data type to aid in selection of things from finite range,
// such as gobo name or color name.
pub struct FixtureControl {
    /// A user-friendly name for this control.
    name: String,
    /// The native data type expected by this control.
    /// Input data will be interpreted as this type.
    data_type: Datatype,
    /// The current value of this control.
    value: Data,
}

impl FixtureControl {
    pub fn new<N: Into<String>>(name: N, data_type: Datatype, initial_value: Data) -> Self {
        FixtureControl {
            name: name.into(),
            data_type: data_type,
            value: initial_value.as_type(data_type).coerce(),
        }
    }
    /// Set this fixture control using value.  The data will be reinterpreted as the native
    /// data type specified by this control, and it will be coerced to be in range.
    pub fn set_value(&mut self, value: Data) {
        self.value = value.as_type(self.data_type).coerce();
    }
    /// Get the value of this control.
    pub fn value(&self) -> Data {
        self.value
    }
}

// ------------------
// serialization helper wrapper type for render function
// ------------------

pub type RenderFunc = fn(&[FixtureControl], &mut [DmxValue]);

struct RenderAction {
    /// The name of this render action, probably the same as the associated fixture type.
    /// Used to round-trip this action through serde.
    name: String,
    func: RenderFunc,
}

impl RenderAction {
    fn serialize_to_str<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.name)
    }
}

impl fmt::Debug for RenderAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RenderAction {{ name: {} }}", self.name)
    }
}

impl PartialEq for RenderAction {
    // Compare RenderActions using their unique string name.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RenderAction {}

impl FromStr for RenderAction {
    type Err = String;
    /// Use the precompiled table of render functions to try to look up this render action.
    /// Used during deserialization.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        render_func_for_type(s)
            .map(|func| RenderAction {name: s.to_string(), func: func})
            .ok_or(format!("Unknown fixture type: '{}'.", s))
    }
}

fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de>, T: FromStr<Err = String>
{
    // A visitor that expects a string and uses T's impl of FromStr to create itself.
    struct DeserializeFromString<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for DeserializeFromString<T>
        where T: FromStr<Err = String>
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
            where E: de::Error
        {
            FromStr::from_str(value).map_err(E::custom)
        }
    }

    deserializer.deserialize_string(DeserializeFromString(PhantomData))
}

// ---------------------
// A single DMX-controlled fixture with a wiggles interface
// ---------------------

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DmxFixture {
    /// What kind of fixture is this?
    kind: String,
    /// The number of DMX channels that this fixture requires.
    channel_count: DmxChannelCount,
    /// Controls for this fixture.
    controls: Vec<FixtureControl>,
    #[serde(serialize_with="RenderAction::serialize_to_str")]
    #[serde(deserialize_with="deserialize_from_str")]
    /// Action to render this fixture to DMX.
    render_action: RenderAction,
}

impl DmxFixture {
    pub fn new<K: Into<String>>(
            kind: K,
            channel_count: DmxChannelCount,
            controls: Vec<FixtureControl>,
            render_func: RenderFunc) -> Self {
        let kind = kind.into();
        let render_action = RenderAction {name: kind.clone(), func: render_func};
        DmxFixture {
            kind: kind,
            channel_count: channel_count,
            controls: controls,
            render_action: render_action,
        }
    }
    pub fn kind(&self) -> &str {
        &self.kind
    }
    pub fn channel_count(&self) -> DmxChannelCount {
        self.channel_count
    }
    
    /// Use this fixture's render func and its controls to render into a DMX buffer.
    pub fn render(&self, buffer: &mut [DmxValue]) {
        debug_assert!(buffer.len() == self.channel_count as usize);
        (self.render_action.func)(&self.controls, buffer);
    }
    
    /// Set a control value.
    pub fn set_control(&mut self, control_id: usize, value: Data) -> Result<(), FixtureError> {
        if control_id >= self.controls.len() {
            Err(FixtureError::ControlOutOfRange(control_id))
        } else {
            self.controls[control_id].set_value(value);
            Ok(())
        }
    }
}

pub enum FixtureError {
    ControlOutOfRange(usize),
}