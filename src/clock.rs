//! Types and traits for clocks and clock signals.
use update::{Update, DeltaT};
use utils::modulo_one;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

#[derive(Clone, Debug)]
pub enum Rate {
    Hz(f64),
    Bpm(f64),
    Period(f64)
}

impl Rate {
    fn in_hz(&self) -> f64 {
        match *self {
            Rate::Hz(v) => v,
            Rate::Bpm(bpm) => bpm / 60.0,
            Rate::Period(seconds) => 1.0 / seconds
        }
    }
}


pub trait ClockSource {
    /// Get the phase of this clock.
    fn phase(&self) -> f64;

    /// Get the current tick count of this clock.
    fn ticks(&self) -> i64;

    /// Find out if the clock ticked the last time it updated.
    fn ticked(&self) -> bool;

    /// Get the tick count and phase of this clock, as a single float.
    /// The integer portion is tick count, while the fractional portion is phase.
    fn value(&self) -> f64 {
        self.ticks() as f64 + self.phase()
    }
}

pub trait SynchronizableClock {
    /// Reset a clock such that it's phase is 0.0 and it behaves as if it just ticked.
    fn reset(&mut self);
}

pub struct Clock {
    phase: f64,
    tick_count: i64,
    ticked: bool,
    rate: f64 // Hz
}

impl Clock {
    pub fn new(rate: Rate) -> Self {
        Clock {phase: 0.0, tick_count: 0, rate: rate.in_hz(), ticked: true}
    }

    pub fn set_rate(&mut self, rate: Rate) {
        self.rate = rate.in_hz();
    }
}

impl ClockSource for Clock {
    fn phase(&self) -> f64 {self.phase}
    fn ticks(&self) -> i64 {self.tick_count}
    fn ticked(&self) -> bool {self.ticked}
}

impl SynchronizableClock for Clock {
    fn reset(&mut self) {
        self.phase = 0.0;
        self.tick_count = 0;
        self.ticked = true;
    }
}

impl Update for Clock {
    fn update(&mut self, DeltaT(dt): DeltaT) {
        // determine how much phase has elapsed
        let elapsed_phase = self.rate * dt;
        let phase_unwrapped = self.phase + elapsed_phase;

        // Determine how many ticks have actually elapsed.  It may be more than 1.
        // It may also be negative if this clock has a negative rate.
        let accumulated_ticks = phase_unwrapped.floor() as i64;

        // This clock ticked if we accumulated +-1 or more ticks.
        self.ticked = accumulated_ticks.abs() > 0;
        self.tick_count += accumulated_ticks;

        self.phase = modulo_one(phase_unwrapped);
    }
}

// =================================
// quasi-stateless clock multiplication
// =================================


/// Multiply another clock signal to produce a clock that runs at a different rate.
pub struct ClockMultiplier<T: ClockSource> {
    source: T,
    factor: f64,
    current_value: Cell<Option<(f64, bool)>>, // we may have computed and memoized the current value
    prev_value: f64,
    prev_value_age: i64, // how many updates have gone by since we last computed a value?
}

impl<T: ClockSource> ClockMultiplier<T> {
    pub fn new(source: T, factor: f64) -> Self {
        // initially set this clock's previous value to the current value of
        // the upstream clock times the multiplier.
        let v = source.value() * factor;
        ClockMultiplier {
            source: source,
            factor: factor,
            current_value: Cell::new(None),
            prev_value: v,
            prev_value_age: 1}
    }

    /// Compute the current value of this clock and whether or not it ticks this frame.
    /// Memoize this result, or return an existing memoized result.
    fn compute_current_value(&self) -> (f64, bool) {
        if let Some(vt) = self.current_value.get() {
            vt
        }
        else {
            let current_value = self.source.value() * self.factor;
            let delta_v = current_value - self.prev_value;
            // depending on the age of the previous value, crudely calculate how
            // much of the total delta_v accumulated this frame.
            let delta_v_this_frame = delta_v / self.prev_value_age as f64;
            // if the integer portion of the approximate value one update ago
            // and the current value are different, this multiplier ticked.
            let current_tick_number = current_value.trunc();
            let approximate_prev_tick_number = (current_value - delta_v_this_frame).trunc();
            let ticked = current_tick_number != approximate_prev_tick_number;
            self.current_value.set(Some((current_value, ticked)));
            (current_value, ticked)
        }
    }
}

impl<T: ClockSource> Update for ClockMultiplier<T> {
    fn update(&mut self, _: DeltaT) {
        // if a current_value is set, pull it out and use it to update prev_value.
        // if not, simply increase the age of the currently held previous value.
        // this implementation assumes that state updates come at a deterministic
        // and constant delta_t.
        if let Some((value, _)) = self.current_value.get() {
            self.prev_value = value;
            self.prev_value_age = 1;
            self.current_value.set(None);
        }
        else {
            self.prev_value_age += 1;
        }
    }
}

impl<T: ClockSource> ClockSource for ClockMultiplier<T> {
    fn phase(&self) -> f64 {
        let (value, _) = self.compute_current_value();
        modulo_one(value)
    }

    fn ticks(&self) -> i64 {
        let (value, _) = self.compute_current_value();
        value.trunc() as i64
    }

    fn ticked(&self) -> bool {
        let (_, ticked) = self.compute_current_value();
        ticked
    }
}


impl<T: ClockSource> ClockSource for Rc<RefCell<T>> {
    fn phase(&self) -> f64 {self.borrow().phase()}
    fn ticks(&self) -> i64 {self.borrow().ticks()}
    fn ticked(&self) -> bool {self.borrow().ticked()}
}

mod tests {
    #![allow(unused_imports)]
    use update::*;
    use super::*;
    use super::Rate::Hz;
    use utils::assert_almost_eq;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_clock() {
        let mut source = Clock::new(Hz(1.0));

        // update clock 3/4 of a period
        source.update(DeltaT(0.75));

        assert_almost_eq(0.75, source.phase());
        assert_eq!(0, source.ticks());
        assert!(! source.ticked());

        // update clock another 3/4 of a period
        source.update(DeltaT(0.75));
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

        let dt = DeltaT(0.75);

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