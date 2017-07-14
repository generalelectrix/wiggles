//! A wiggles node that fans a single input across multiple outputs, with a phase shift spread
//! equally across them.
use std::sync::Arc;
use std::time::Duration;
use std::ops::{Add, Mul};
use std::cmp::max;
use std::fmt;
use console_server::reactor::Messages;
use ::network::{OutputId, Inputs, Outputs};
use wiggles_value::knob::{
    Knobs,
    Datatype as KnobDatatype,
    Data as KnobData,
    KnobDescription,
    Error as KnobError,
    badaddr,
    badtype,
    Response as KnobResponse,
};
use serde_json::{Error as SerdeJsonError, self};
use clocks::clock::{ClockId, ClockProvider, ClockValue};
use super::wiggle::{Wiggle, CompleteWiggle, WiggleId, KnobAddr, WiggleProvider};
use wiggles_value::{Unipolar, Bipolar, Datatype, Data};
use wiggles_value::blend::Blend;
use waveforms::sine;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fanner {
    name: String,
    /// The total phase shift to spread across the outputs.
    spread: f64,
    output_count: usize,
}

impl Fanner {
    pub fn new<N: Into<String>>(name: N) -> Self {
        Fanner {
            name: name.into(),
            spread: 0.0,
            output_count: 1,
        }
    }
}

pub const KIND: &'static str = "fanner";

lazy_static! {
    static ref SPREAD_KNOB_DESC: KnobDescription = KnobDescription {
        name: Arc::new("spread".to_string()),
        datatype: KnobDatatype::UFloat,
    };
}

// Fanner has exactly one input.
impl<M, I> Inputs<M, I> for Fanner { }

// Fanner has at least one and up to an unlimited number of outputs.
impl<M, I> Outputs<M, I> for Fanner {
    fn try_push_output(&mut self, _: I) -> Result<Messages<M>, ()> {
        self.output_count += 1;
        Ok(Messages::none())
    }

    fn try_pop_output(&mut self, _: I) -> Result<Messages<M>, ()> {
        if self.output_count == 1 {
            Err(())
        }
        else {
            self.output_count -= 1;
            Ok(Messages::none())
        }
    }
}


impl Knobs<KnobAddr> for Fanner {
    fn knobs(&self) -> Vec<(KnobAddr, KnobDescription)> {
        vec!((0, SPREAD_KNOB_DESC.clone()))
    }

    fn knob_datatype(&self, addr: KnobAddr) -> Result<KnobDatatype, KnobError<KnobAddr>> {
        if addr == 0 {
            Ok(SPREAD_KNOB_DESC.datatype.clone())
        }
        else {
            Err(badaddr(addr))
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: KnobAddr) -> Result<KnobData, KnobError<KnobAddr>> {
        if addr == 0 {
            Ok(KnobData::UFloat(self.spread))
        }
        else {
            Err(badaddr(addr))
        }
    }

    fn set_knob(&mut self, addr: KnobAddr, value: KnobData) -> Result<(), KnobError<KnobAddr>> {
        if addr == 0 {
            let v = value.as_ufloat()?;
            self.spread = v;
            Ok(())
        }
        else {
            Err(badaddr(addr))
        }
    }
}

impl Wiggle for Fanner {
    fn kind(&self) -> &'static str {
        KIND
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Fanner is stateless, update does nothing.
    fn update(&mut self, _: Duration) -> Messages<KnobResponse<KnobAddr>> {
        Messages::none()
    }

    /// Apply additional phase offset based on the requested output ID.
    fn render(
        &self,
        phase_offset: f64,
        type_hint: Option<Datatype>,
        inputs: &[Option<(WiggleId, OutputId)>],
        output_id: OutputId,
        wiggles: &WiggleProvider,
        clocks: &ClockProvider)
        -> Data
    {
        match inputs.get(0) {
            None => {
                // For some reason our required input is missing.  Log an error rather than panic.
                error!(
                    "Fanner {} should have one input but was passed an empty inputs collection.",
                    self.name,
                );
                Data::default_with_type_hint(type_hint)
            }
            Some(&None) => {
                // No input set, provide a default.
                Data::default_with_type_hint(type_hint)
            }
            Some(&Some((source, source_output))) => {
                // Get the value from the upstream source with phase offset.
                let delta_phase =
                    if self.output_count == 1 {
                        0.0
                    }
                    else {
                        self.spread / (self.output_count - 1) as f64
                    };
                let phase_spread = delta_phase * output_id.0 as f64;
                wiggles.get_value(source, source_output, phase_offset + phase_spread, type_hint, clocks)
            }
        }


    }

    fn clock_source(&self) -> Result<Option<ClockId>, ()> {
        Err(())
    }

    fn set_clock(&mut self, _: Option<ClockId>) -> Result<(), ()> {
        Err(())
    }

    fn as_json(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }
}
