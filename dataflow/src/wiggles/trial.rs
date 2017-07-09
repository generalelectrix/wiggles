//! A wiggle that just produces a sin wave, as a proof of principle.
use std::sync::Arc;
use std::time::Duration;
use console_server::reactor::Messages;
use ::network::Inputs;
use wiggles_value::knob::{
    Knobs,
    Datatype as KnobDatatype,
    Data as KnobData,
    KnobDescription,
    Error as KnobError,
    badaddr,
    Message as KnobMessage,
};
use serde_json::{Error as SerdeJsonError, self};
use clocks::clock::{ClockId, ClockProvider, ClockValue};
use super::wiggle::{Wiggle, CompleteWiggle, WiggleId, KnobAddr, WiggleProvider};
use wiggles_value::{Unipolar, Datatype, Data};
use waveforms::sine;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestWiggle {
    name: String,
    clock: Option<ClockId>,
    duty_cycle: Unipolar,
}

impl TestWiggle {
    pub fn new<N: Into<String>>(name: N) -> Self {
        TestWiggle {
            name: name.into(),
            clock: None,
            duty_cycle: Unipolar(1.0),
        }
    }
}

pub const CLASS: &'static str = "test";

// TestWiggle has no inputs.
impl<M> Inputs<M> for TestWiggle {
    fn default_input_count(&self) -> u32 {
        0
    }
    fn try_push_input(&mut self) -> Result<Messages<M>, ()> {
        Err(())
    }
    fn try_pop_input(&mut self) -> Result<Messages<M>, ()> {
        Err(())
    }
}

const DUTY_KNOB_ADDR: u32 = 0;

// Unchanging collection of knobs.
lazy_static! {
    static ref KNOB_DESC: Vec<(KnobAddr, KnobDescription)> = {
        let ampl_desc = KnobDescription {
            name: Arc::new("duty cycle".to_string()),
            datatype: KnobDatatype::Wiggle(Datatype::Unipolar),
        };
        vec!((DUTY_KNOB_ADDR, ampl_desc),)
    };
}

impl Knobs<KnobAddr> for TestWiggle {
    fn knobs(&self) -> Vec<(KnobAddr, KnobDescription)> {
        KNOB_DESC.clone()
    }

    fn knob_datatype(&self, addr: KnobAddr) -> Result<KnobDatatype, KnobError<KnobAddr>> {
        match addr {
            DUTY_KNOB_ADDR => Ok(KnobDatatype::Wiggle(Datatype::Unipolar)),
            _ => Err(badaddr(addr)),
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: KnobAddr) -> Result<KnobData, KnobError<KnobAddr>> {
        match addr {
            DUTY_KNOB_ADDR => Ok(KnobData::Wiggle(Data::Unipolar(self.duty_cycle))),
            _ => Err(badaddr(addr)),
        }
    }

    fn set_knob(&mut self, addr: KnobAddr, value: KnobData) -> Result<(), KnobError<KnobAddr>> {
        match addr {
            DUTY_KNOB_ADDR => {
                self.duty_cycle = value.as_unipolar()?;
                Ok(())
            }
            _ => {
                Err(badaddr(addr))
            }
        }
    }
}

impl Wiggle for TestWiggle {
    fn class(&self) -> &'static str {
        CLASS
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// Update the state of this wiggle using the provided update interval.
    fn update(&mut self, _: Duration) -> Messages<KnobMessage<KnobAddr>> {
        Messages::none()
    }

    fn render(
        &self,
        phase_offset: Unipolar,
        type_hint: Option<Datatype>,
        _: &[Option<WiggleId>],
        _: &WiggleProvider,
        clocks: &ClockProvider)
        -> Data
    {
        let clock_val = match self.clock {
            Some(cid) => clocks.get_value(cid),
            None => ClockValue::default(),
        };
        // TODO: pass type hint into waveform generator so it can be as clever as possible.
        let value = sine(clock_val.phase_shift(phase_offset), Unipolar(1.0), false);
        Data::Bipolar(value)
    }

    /// Return Ok if this wiggle uses a clock input, and return the current value of it.
    /// If it doesn't use a clock, return Err.
    fn clock_source(&self) -> Result<Option<ClockId>, ()> {
        Ok(self.clock)
    }

    /// Set the clock source for this wiggle.
    /// If this wiggle doesn't use a clock, return Err.
    fn set_clock(&mut self, source: Option<ClockId>) -> Result<(), ()> {
        self.clock = source;
        Ok(())
    }

    fn as_json(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }
}