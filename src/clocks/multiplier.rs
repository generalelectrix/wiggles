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
    ClockNodeIndex};
use knob::{Knob, KnobValue, KnobId};
use event::Events;

pub const MULT_KNOB_ID: KnobId = 0;
pub const INIT_MULT_FACTOR: f64 = 1.0;
pub const SOURCE_INPUT_ID: InputId = 0;

/// Multiply another clock signal to produce a clock that runs at a different rate.
pub struct ClockMultiplier {
    // Implementation note: these values are Cells because a clock multiplier
    // must update its internal state lazily when it is called to produce a value,
    // as it is based solely on the upstream state which is not available during
    // timestep updates.

    /// Previous floating-point value of time; if None, it implies this clock has not been
    /// initialized via a first call to update.
    prev_value: Cell<Option<f64>>,
    /// How many updates have gone by since we last computed a value?
    prev_value_age: Cell<i64>,
}

impl ClockMultiplier {
    /// Create a new instance of this clock, and hide it behind the interface trait.
    fn create() -> Box<CompleteClock> {
        Box::new(ClockMultiplier { prev_value: Cell::new(None), prev_value_age: Cell::new(0) })
    }

    /// Produce the prototype for this type of clock.  This should only need to be called once,
    /// during program initialization.
    pub fn create_prototype() -> ClockNodePrototype {
        let multiplier_knob = Knob::new("multiplier",
                                        MULT_KNOB_ID,
                                        KnobValue::PositiveFloat(INIT_MULT_FACTOR));
        let knobs = vec![multiplier_knob].into_boxed_slice();
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

        // now compute our modified time
        let multiplied_value = upstream_val.float_value() * multiplier;

        // determine if we should tick on this update
        let ticked = 
            if let Some(prev_val) = self.prev_value.get() {
                let delta_t = multiplied_value - prev_val;
                // depending on the age of the previous value, crudely calculate how
                // much of the total delta_t accumulated this frame.  We need to do this max check
                // in case we render two frames in a row without an update.
                let age = max(self.prev_value_age.get(), 1);

                let delta_t_this_frame = delta_t / age as f64;
                // if the integer portion of the approximate value one update ago
                // and the current value are different, this multiplier ticked.
                let current_tick_number = multiplied_value.trunc();
                let approximate_prev_tick_number = (multiplied_value - delta_t_this_frame).trunc();

                current_tick_number != approximate_prev_tick_number
            } else { false }; // this multiplier hasn't been run yet, so don't tick
        self.prev_value.set(Some(multiplied_value));
        self.prev_value_age.set(0);
        ClockValue::from_float_value(multiplied_value, ticked)
    }
}

impl UpdateClock for ClockMultiplier {
    fn update(&mut self, _: ClockNodeIndex, _: &mut [Knob], _: DeltaT) -> Events {
        // age our stored previous value by one
        let new_age = self.prev_value_age.get() + 1;
        self.prev_value_age.set(new_age);
        Events::new()
    }
}
