//! A clock that performs quasi-stateless clock multiplication and division.
//! This implementation fundamentally relies on receiving deterministic, equally-sized timesteps
//! during the state update.
use std::sync::Arc;
use std::time::Duration;
use std::cell::Cell;
use std::cmp::max;
use console_server::reactor::Messages;
use ::util::{modulo_one, almost_eq};
use super::clock::{Clock, ClockValue, ClockId, ClockProvider, KnobAddr};
use ::network::Inputs;
use wiggles_value::knob::{
    Knobs, Datatype, Data, KnobDescription, Error as KnobError, badaddr, Message as KnobMessage};
use serde_json::{Error as SerdeJsonError, self};

pub const MULT_KNOB_ADDR: u32 = 0;
pub const INIT_MULT_FACTOR: f64 = 1.0;
pub const RESET_KNOB_ADDR: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClockMultiplier {
    name: String,
    multiplier: f64,
    should_reset: bool,
    // Implementation note: these values are Cells because a clock multiplier
    // must update its internal state lazily when it is called to produce a value,
    // as it is based solely on the upstream state which is not available during
    // timestep updates.

    /// The last value we read from the upstream clock.
    prev_upstream: Cell<Option<f64>>,
    /// Previous floating-point value of time; if None, it implies this clock has not been
    /// initialized via a first call to update.
    prev_value: Cell<Option<f64>>,
    /// How many updates have gone by since we last computed a value?
    prev_value_age: Cell<i64>,
}

impl ClockMultiplier {
    pub fn new<N: Into<String>>(name: N) -> Self {
        ClockMultiplier {
            name: name.into(),
            multiplier: INIT_MULT_FACTOR,
            should_reset: false,
            prev_upstream: Cell::new(None),
            prev_value: Cell::new(None),
            prev_value_age: Cell::new(0),
        }
    }
}

fn same_optional_float(f0: &Option<f64>, f1: &Option<f64>) -> bool {
    match (*f0, *f1) {
        (Some(f0i), Some(f1i)) => almost_eq(f0i, f1i),
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq for ClockMultiplier {
    fn eq(&self, other: &ClockMultiplier) -> bool {
        self.name == other.name
        && almost_eq(self.multiplier, other.multiplier)
        && self.should_reset == other.should_reset
        && same_optional_float(&self.prev_upstream.get(), &other.prev_upstream.get())
        && same_optional_float(&self.prev_value.get(), &other.prev_value.get())
        && self.prev_value_age == other.prev_value_age
    }
}

pub const CLASS: &'static str = "multiplier";

impl<M> Inputs<M> for ClockMultiplier {
    /// Multiplier always multiplies a single input.
    fn default_input_count(&self) -> u32 {
        1
    }
    /// Simple clock always has 1 input.
    fn try_push_input(&mut self) -> Result<M, ()> {
        Err(())
    }

    /// Simple clock always has 1 input.
    fn try_pop_input(&mut self) -> Result<M, ()> {
        Err(())
    }
}

// Since ClockMultiplier always has the same number of knobs, use a static for its knob descriptions.
lazy_static! {
    static ref KNOB_DESC: Vec<(KnobAddr, KnobDescription)> = {
        let mult_desc = KnobDescription {
            name: Arc::new("factor".to_string()),
            datatype: Datatype::UFloat,
        };
        let reset_desc = KnobDescription {
            name: Arc::new("reset".to_string()),
            datatype: Datatype::Button,
        };
        vec!((MULT_KNOB_ADDR, mult_desc), (RESET_KNOB_ADDR, reset_desc))
    };
}

impl Knobs<KnobAddr> for ClockMultiplier {
    fn knobs(&self) -> Vec<(KnobAddr, KnobDescription)> {
        KNOB_DESC.clone()
    }

    fn knob_datatype(&self, addr: KnobAddr) -> Result<Datatype, KnobError<KnobAddr>> {
        match addr {
            MULT_KNOB_ADDR => Ok(Datatype::UFloat),
            RESET_KNOB_ADDR => Ok(Datatype::Button),
            _ => Err(badaddr(addr)),
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: KnobAddr) -> Result<Data, KnobError<KnobAddr>> {
        match addr {
            MULT_KNOB_ADDR => Ok(Data::UFloat(self.multiplier)),
            RESET_KNOB_ADDR => Ok(Data::Button(self.should_reset)),
            _ => Err(badaddr(addr)),
        }
    }

    fn set_knob(&mut self, addr: KnobAddr, value: Data) -> Result<(), KnobError<KnobAddr>> {
        match addr {
            MULT_KNOB_ADDR => {
                self.multiplier = value.as_ufloat()?;
                Ok(())
            }
            RESET_KNOB_ADDR => {
                // We only pay attention to a "button down" message for reset.
                if value.as_button()? {
                    self.should_reset = true;
                }
                else {
                    debug!("Clock multiplier ignored a button-up knob message.");
                }
                Ok(())
            }
            _ => {
                Err(badaddr(addr))
            }
        }
    }
}

impl Clock for ClockMultiplier {
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
    fn update(&mut self, _: Duration) -> Messages<KnobMessage<KnobAddr>> {
        // if the reset knob was pushed, reset the clock value
        if self.should_reset {
            self.prev_upstream.set(None);
            self.prev_value.set(None);
            self.prev_value_age.set(0);
            self.should_reset = false;
            // emit a message that we changed this knob value.
            Messages::one(
                KnobMessage::ValueChange{addr: RESET_KNOB_ADDR, value: Data::Button(false)})
        }
        else {
            // age our stored previous value by one
            let new_age = self.prev_value_age.get() + 1;
            self.prev_value_age.set(new_age);
            Messages::none()
        }
    }

    fn render(&self, inputs: &[Option<ClockId>], clock_network: &ClockProvider) -> ClockValue {
        // If our inputs are the wrong shape, log an error and return the zero value.
        match inputs.get(0) {
            None => {
                error!("Multiplier {} was passed 0 inputs.", self.name);
                return ClockValue::default();
            }
            Some(&None) => {
                // input is disconnected, return 0
                return ClockValue::default();
            }
            Some(&Some(id)) => {
                let upstream_val = clock_network.get_value(id);

                let upstream_float_val = upstream_val.float_value();

                let prev_upstream_val = self.prev_upstream.get().unwrap_or(0.0);
                let prev_val = self.prev_value.get().unwrap_or(0.0);

                let upstream_dt = upstream_float_val - prev_upstream_val;
                let mult_dt = upstream_dt * self.multiplier;

                let new_time = prev_val + mult_dt;

                // determine if we should tick on this update
                let ticked = {
                    // depending on the age of the previous value, crudely calculate how
                    // much of the total delta_t accumulated this frame.  We need to do this max check
                    // in case we render two frames in a row without an update.
                    let age = max(self.prev_value_age.get(), 1);

                    let delta_t_this_frame = mult_dt / age as f64;
                    // if the integer portion of the approximate value one update ago
                    // and the current value are different, this multiplier ticked.
                    let current_tick_number = new_time.trunc();
                    let approximate_prev_tick_number = (new_time - delta_t_this_frame).trunc();

                    current_tick_number != approximate_prev_tick_number
                };
                self.prev_upstream.set(Some(upstream_float_val));
                self.prev_value.set(Some(new_time));
                self.prev_value_age.set(0);
                ClockValue::from_float_value(new_time, ticked)
            }
        }

    }

    fn as_json(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }
}