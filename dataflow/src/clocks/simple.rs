//! A basic clock that runs at a rate set by a knob.
//! Also provides a reset button.
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use console_server::reactor::Messages;
use ::util::{secs, modulo_one};
use super::clock::{Clock, ClockValue, ClockId, ClockProvider, Message, KnobAddr};
use ::network::Inputs;
use wiggles_value::knob::{
    Knobs, Datatype, Data, KnobDescription, Error as KnobError, badaddr, Message as KnobMessage};
use wiggles_value::knob_types::Rate;
use serde_json::{Error as SerdeJsonError, self};

pub const INIT_CLOCK_VAL: ClockValue = ClockValue { phase: 0.0, tick_count: 0, ticked: true };
// Run at 1 Hz by default.
pub const INIT_RATE: f64 = 1.0;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SimpleClock {
    name: String,
    value: ClockValue,
    /// Floating-point rate in units of Hz.
    rate: f64, 
    /// If True, this clock will reset on the next update cycle and swap this value back to false.
    should_reset: bool,
}

impl SimpleClock {
    pub fn new<N: Into<String>>(name: N) -> Self {
        SimpleClock {
            name: name.into(),
            value: INIT_CLOCK_VAL,
            rate: INIT_RATE,
            should_reset: false,
        }
    }
}

pub const CLASS: &'static str = "simple";

impl<M> Inputs<M> for SimpleClock {
    /// Simple clock always has no inputs.
    fn default_input_count(&self) -> u32 {
        0
    }
    /// Simple clock always has no inputs.
    fn try_push_input(&mut self) -> Result<M, ()> {
        Err(())
    }

    /// Simple clock always has no inputs.
    fn try_pop_input(&mut self) -> Result<M, ()> {
        Err(())
    }
}

const RATE_KNOB_ADDR: u32 = 0;
const RESET_KNOB_ADDR: u32 = 1;

// Since SimpleClock always has the same number of knobs, use a static for its knob descriptions.
lazy_static! {
    static ref KNOB_DESC: Vec<(KnobAddr, KnobDescription)> = {
        let rate_desc = KnobDescription {
            name: Arc::new("rate".to_string()),
            datatype: Datatype::Rate,
        };
        let reset_desc = KnobDescription {
            name: Arc::new("reset".to_string()),
            datatype: Datatype::Button,
        };
        vec!((RATE_KNOB_ADDR, rate_desc), (RESET_KNOB_ADDR, reset_desc))
    };
}

impl Knobs<KnobAddr> for SimpleClock {
    fn knobs(&self) -> Vec<(KnobAddr, KnobDescription)> {
        KNOB_DESC.clone()
    }

    fn knob_datatype(&self, addr: KnobAddr) -> Result<Datatype, KnobError<KnobAddr>> {
        match addr {
            RATE_KNOB_ADDR => Ok(Datatype::Rate),
            RESET_KNOB_ADDR => Ok(Datatype::Button),
            _ => Err(badaddr(addr)),
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: KnobAddr) -> Result<Data, KnobError<KnobAddr>> {
        match addr {
            RATE_KNOB_ADDR => Ok(Data::Rate(Rate::Hz(self.rate))),
            RESET_KNOB_ADDR => Ok(Data::Button(self.should_reset)),
            _ => Err(badaddr(addr)),
        }
    }

    fn set_knob(&mut self, addr: KnobAddr, value: Data) -> Result<(), KnobError<KnobAddr>> {
        match addr {
            RATE_KNOB_ADDR => {
                self.rate = value.as_rate()?.in_hz();
                Ok(())
            }
            RESET_KNOB_ADDR => {
                // We only pay attention to a "button down" message for reset.
                if value.as_button()? {
                    self.should_reset = true;
                }
                else {
                    debug!("Simple clock ignored a button-up knob message.");
                }
                Ok(())
            }
            _ => {
                Err(badaddr(addr))
            }
        }
    }
}

impl Clock for SimpleClock {
    /// A string name for this class of clock.
    /// This string will be used during serialization and deserialization to uniquely identify
    /// how to reconstruct this clock from a serialized form.
    fn class(&self) -> &'static str {
        CLASS
    }

    /// Return the name that has been assigned to this clock.
    fn name(&self) -> &str {
        &self.name
    }

    /// Update the state of this clock using the provided update interval.
    /// Return a message collection of some kind.
    fn update(&mut self, dt: Duration) -> Messages<Message<KnobAddr>> {
        // if the reset knob was pushed, reset the clock value
        if self.should_reset {
            self.value = INIT_CLOCK_VAL;
            self.should_reset = false;
            // TODO: emit a message that we changed this knob value.
            Messages::one(Message::Knob(
                KnobMessage::ValueChange{addr: RESET_KNOB_ADDR, value: Data::Button(false)}))
        }
        else {
            // determine how much phase has elapsed
            let elapsed_phase = self.rate * secs(dt);
            let phase_unwrapped = self.value.phase + elapsed_phase;

            // Determine how many ticks have actually elapsed.  It may be more than 1.
            // It may also be negative if this clock has a negative rate.
            let accumulated_ticks = phase_unwrapped.floor() as i64;

            // This clock ticked if we accumulated +-1 or more ticks.
            self.value.ticked = accumulated_ticks.abs() > 0;
            self.value.tick_count += accumulated_ticks;

            self.value.phase = modulo_one(phase_unwrapped);
            Messages::none()
        }
    }

    fn render(&self, _: &[Option<ClockId>], _: &ClockProvider) -> ClockValue {
        self.value
    }

    fn as_json(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }
}