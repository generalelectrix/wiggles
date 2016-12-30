//! Types and traits for clocks and clock signals.
use update::{Update, DeltaT};
use utils::modulo_one;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use clock_network::{
    ClockValue, ClockGraph, ComputeClock, UpdateClock, CompleteClock, ClockInputSocket};
use knob::{Knob, KnobValue};
use datatypes::Rate;

// =================================
// quasi-stateless clock multiplication
// =================================


/// Multiply another clock signal to produce a clock that runs at a different rate.
pub struct ClockMultiplier {
    factor: f64,
    current_value: Cell<Option<ClockValue>>, // we may have computed and memoized the current value
    /// Previous floating-point value of time.
    prev_value: f64,
    prev_value_age: i64, // how many updates have gone by since we last computed a value?
}

impl ClockMultiplier {
    pub fn new(initial_source_value: f64, factor: f64) -> Self {
        // initially set this clock's previous value to the current value of
        // the upstream clock times the multiplier.
        let v = initial_source_value * factor;
        ClockMultiplier {
            factor: factor,
            current_value: Cell::new(None),
            prev_value: v,
            prev_value_age: 1}
    }

    /// Compute the current value of this clock and whether or not it ticks this frame.
    /// Memoize this result, or return an existing memoized result.
    fn compute_current_value(&self, upstream_val: ClockValue) -> ClockValue {
        if let Some(vt) = self.current_value.get() {
            vt
        }
        else {
            let current_value = upstream_val.float_value() * self.factor;
            let delta_v = current_value - self.prev_value;
            // depending on the age of the previous value, crudely calculate how
            // much of the total delta_v accumulated this frame.
            let delta_v_this_frame = delta_v / self.prev_value_age as f64;
            // if the integer portion of the approximate value one update ago
            // and the current value are different, this multiplier ticked.
            let current_tick_number = current_value.trunc();
            let approximate_prev_tick_number = (current_value - delta_v_this_frame).trunc();
            let ticked = current_tick_number != approximate_prev_tick_number;
            let new_value = ClockValue {
                phase: modulo_one(current_value),
                tick_count: current_value.trunc() as i64,
                ticked: ticked,};
            self.current_value.set(Some(new_value));
            new_value
        }
    }
}

impl UpdateClock for ClockMultiplier {
    fn update(&mut self, _: &[Knob], _: DeltaT) {
        // if a current_value is set, pull it out and use it to update prev_value.
        // if not, simply increase the age of the currently held previous value.
        // this implementation assumes that state updates come at a deterministic
        // and constant delta_t.
        if let Some(value) = self.current_value.get() {
            self.prev_value = value.float_value();
            self.prev_value_age = 1;
            self.current_value.set(None);
        }
        else {
            self.prev_value_age += 1;
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use update::*;
    use super::*;
    use datatypes::Rate::Hz;
    use utils::assert_almost_eq;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_clock() {
        let mut source = Clock::new(Hz(1.0));

        // update clock 3/4 of a period
        source.update(0.75);

        assert_almost_eq(0.75, source.phase());
        assert_eq!(0, source.ticks());
        assert!(! source.ticked());

        // update clock another 3/4 of a period
        source.update(0.75);
        assert_almost_eq(0.5, source.phase());
        assert_eq!(1, source.ticks());
        assert!(source.ticked());

    }

    #[test]
    fn test_clock_multiplication() {
        // clock that ticks at 1 Hz.
        let source = Rc::new(RefCell::new(Clock::new(Hz(1.0))));

        // clock that should tick at 2 Hz.
        let mut mult = ClockMultiplier::new(source.clone(), 2.0);

        let dt = 0.75;

        assert_eq!(0.0, mult.phase());

        source.borrow_mut().update(dt);
        mult.update(dt);

        assert_almost_eq(0.5, mult.phase());
        assert!(mult.ticked());

        source.borrow_mut().update(dt);
        mult.update(dt);

        source.borrow_mut().update(dt);
        mult.update(dt);
    }

}