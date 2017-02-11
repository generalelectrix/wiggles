//! A clock that performs quasi-stateless clock multiplication and division.
use update::DeltaT;
use std::cell::Cell;
use std::cmp::max;
use clock_network::{
    ClockValue,
    ClockNetwork,
    ComputeClock,
    UpdateClock,
    CompleteClock,
    ClockInputSocket,
    ClockNodePrototype,
    InputId,
    ClockNodeIndex,
};
use knob::{Knob, KnobValue, KnobId};
use event::Events;
use super::action_if_button_pressed;

pub const MULT_KNOB_ID: KnobId = 0;
pub const INIT_MULT_FACTOR: f64 = 1.0;
pub const RESET_KNOB_ID: KnobId = 1;
pub const SOURCE_INPUT_ID: InputId = 0;

/// Multiply another clock signal to produce a clock that runs at a different rate.
pub struct ClockMultiplier {
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
    /// Create a new instance of this clock, and hide it behind the interface trait.
    fn create() -> Box<CompleteClock> {
        Box::new(ClockMultiplier {
            prev_upstream: Cell::new(None),
            prev_value: Cell::new(None),
            prev_value_age: Cell::new(0), })
    }

    /// Produce the prototype for this type of clock.  This should only need to be called once,
    /// during program initialization.
    pub fn create_prototype() -> ClockNodePrototype {
        let multiplier_knob = Knob::new("multiplier",
                                        MULT_KNOB_ID,
                                        KnobValue::PositiveFloat(INIT_MULT_FACTOR));
        let reset_knob = Knob::new("reset",
                                   RESET_KNOB_ID,
                                   KnobValue::Button(false));
        let knobs = vec![multiplier_knob, reset_knob].into_boxed_slice();
        let inputs = vec![("source", SOURCE_INPUT_ID)].into_boxed_slice();
        ClockNodePrototype::new("multiplier", inputs, knobs, Box::new(ClockMultiplier::create))
    }
}


impl ComputeClock for ClockMultiplier {
    fn compute_clock(&self,
                     inputs: &[ClockInputSocket],
                     knobs: &[Knob],
                     g: &ClockNetwork)
                     -> ClockValue {
        // get current time from upstream clock
        let upstream_val = inputs[SOURCE_INPUT_ID].get_value(g);
        // get current multiplier from control knob
        let multiplier = knobs[MULT_KNOB_ID].positive_float();

        let upstream_float_val = upstream_val.float_value();

        let prev_upstream_val = self.prev_upstream.get().unwrap_or(0.0);
        let prev_val = self.prev_value.get().unwrap_or(0.0);

        let upstream_dt = upstream_float_val - prev_upstream_val;
        let mult_dt = upstream_dt * multiplier;

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

impl UpdateClock for ClockMultiplier {
    fn update(&mut self, id: ClockNodeIndex, knobs: &mut [Knob], _: DeltaT) -> Events {
        // if someone mashed reset, forget all of our accumulated state so next
        // update syncs back with the master
        if let Some(reset_events) = {
                let reset_action = || {
                    self.prev_upstream.set(None);
                    self.prev_value.set(None);
                    self.prev_value_age.set(0);
                    None.into()
                };
                action_if_button_pressed(id, knobs, RESET_KNOB_ID, reset_action)
            }
        { reset_events }
        else {
            // age our stored previous value by one
            let new_age = self.prev_value_age.get() + 1;
            self.prev_value_age.set(new_age);
            None.into()
        }
    }
}
