//! Abstract notion of a collection of "patched" fixtures.
//! The notion of patching should be more generic than just DMX, but provide support for some
//! extensible set of output connections a fixture can produce output on.  Ideally there should
//! be no restriction on how many different output connections a single fixture can have, such that
//! one logical fixture could span multiple control formats.
//! That said, for now we'll just support DMX for expediency.  Non-DMX control is still a pipe
//! dream anyway.

pub type DmxAddress = u16;
pub type DmxValue = u8;
pub type UniverseId = usize;

#[derive(Copy, Clone, Debug)]
pub enum ControlValue {
    
}

impl ControlValue {
    /// Return a new ControlValue if the incoming value is compatible with this one.
    fn compatible(&mut self, new_val: ControlValue) -> Result<ControlValue, PatchError> {
        unimplemented!()
    }
}

pub struct Control {
    name: String,
    value: ControlValue,
}

impl Control {
    /// Set the value of this control.
    fn set_value(&mut self, value: ControlValue) -> Result<(), PatchError> {
        self.value = self.value.compatible(value)?;
        Ok(())
    }
}

pub trait FixturePatch<'a> {
    /// Unique fixture id.
    fn id(&self) -> usize;
    /// The name of this kind of fixture.
    fn kind(&self) -> &'static str;
    /// The nickname of this particular fixture.
    fn name(&self) -> &'a str;
    /// Set the nickname of this fixture.
    fn set_name<N: Into<String>>(&mut self, name: N);
    /// Get the address of this patch.
    fn address(&self) -> DmxAddress;
    /// Get the universe ID that this fixture is patched on.
    fn universe(&self) -> UniverseId;
    /// Get the number of DMX channels that this fixture requires.
    fn channel_count(&self) -> u16;
    /// Immutable access to this fixture's controls.
    fn controls(&self) -> &'a [Control];
    /// Mutable access to this fixture's controls.
    fn controls_mut(&mut self) -> &'a mut [Control];
    /// Set the value of a particular control with a compatible value.
    fn set_control(&mut self, id: usize, value: ControlValue) -> Result<(), PatchError> {
        match self.control_mut(id) {
            Some(c) => c.set_value(value),
            None => Err(PatchError::ControlOutOfRange(id))
        }
    }
    /// Immutable access to a control by id.
    fn control(&self, id: usize) -> Option<&'a Control> {
        self.controls().get(id)
    }
    /// Mutable access to a control by id.
    fn control_mut(&mut self, id: usize) -> Option<&'a mut Control> {
        self.controls_mut().get_mut(id)
    }

    /// Render this fixture into a DMX buffer.
    fn render(&self, buffer: &mut [DmxValue]) -> Result<(), PatchError>;
}

pub enum PatchError {
    ControlOutOfRange(usize),
    IncompatibleControlValue(ControlValue),
}