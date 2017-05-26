//! Dmx fixture abstraction.
//! Accepts an arbitrary number of wiggles values as control parameters and
//! opaquely renders these into a DMX buffer.
use wiggles_value::{Datatype, Data};

pub type DmxChannelCount = u16;
pub type DmxValue = u8;

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

type RenderFunc =  fn(&[FixtureControl], &mut [DmxValue]);

pub struct DmxFixture {
    /// What kind of fixture is this?
    kind: String,
    /// The number of DMX channels that this fixture requires.
    channel_count: DmxChannelCount,
    /// Controls for this fixture.
    controls: Vec<FixtureControl>,
    /// Function to render this fixture's control values into a DMX buffer.
    render_func: RenderFunc,
}

impl DmxFixture {
    pub fn new<K: Into<String>>(
            kind: K,
            channel_count: DmxChannelCount,
            controls: Vec<FixtureControl>,
            render_func: RenderFunc) -> Self {
        DmxFixture {
            kind: kind.into(),
            channel_count: channel_count,
            controls: controls,
            render_func: render_func,
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
        (self.render_func)(&self.controls, buffer);
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