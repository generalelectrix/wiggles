//! A basic clock that runs at a rate set by a knob.
//! Also provides a reset button.
use std::sync::Arc;
use super::{Clock, ClockValue};
use ::network::Inputs;
use wiggles_value::knob::{Knobs, Datatype, Data, KnobDescription, KnobError, badaddr};
use wiggles_value::knob_types::Rate;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleClock {
    name: String,
    value: ClockValue,
    /// Floating-point rate in units of Hz.
    rate: f64, 
    /// If True, this clock will reset on the next update cycle and swap this value back to false.
    should_reset: bool,
}

const CLASS: &'static str = "simple";

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
    static ref KNOB_DESC: Vec<(u32, KnobDescription)> = {
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

impl Knobs for SimpleClock {
    type Addr = u32;
    fn knobs(&self) -> Vec<(Self::Addr, KnobDescription)> {
        KNOB_DESC.clone()
    }

    fn knob_datatype(&self, addr: Self::Addr) -> Result<Datatype, KnobError<Self::Addr>> {
        match addr {
            RATE_KNOB_ADDR => Ok(Datatype::Rate),
            RESET_KNOB_ADDR => Ok(Datatype::Button),
            _ => Err(badaddr(addr)),
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: Self::Addr) -> Result<Data, KnobError<Self::Addr>> {
        match addr {
            RATE_KNOB_ADDR => Ok(Data::Rate(Rate::Hz(self.rate))),
            RESET_KNOB_ADDR => Ok(Data::Button(self.should_reset)),
            _ => Err(badaddr(addr)),
        }
    }

    fn set_knob(&mut self, addr: Self::Addr, value: Data) -> Result<(), KnobError<Self::Addr>> {
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

// impl<M> Clock<M> for SimpleClock {
//     /// A string name for this class of clock.
//     /// This string will be used during serialization and deserialization to uniquely identify
//     /// how to reconstruct this clock from a serialized form.
//     fn class(&self) -> &'static str {
//         CLASS
//     }

//     /// Return the name that has been assigned to this clock.
//     fn name(&self) -> &str {
//         &self.name
//     }

//     /// Update the state of this clock using the provided update interval.
//     /// Return a message collection of some kind.
//     fn update(&mut self, dt: Duration) -> Messages<M>;

//     /// Render the state of this clock, providing its currently-assigned inputs as well as a
//     /// function that can be used to retrieve the current value of one of those inputs.
//     fn render(&self, inputs: &[Option<ClockId>], network: &ClockProvider) -> ClockValue;
// }