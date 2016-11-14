//! Types and traits for clocks and clock signals.
use update::{Update, DeltaT};
use utils::modulo;

#[derive(Clone)]
/// A rate in Hz.
pub struct Rate(pub f64);

#[derive(Clone)]
/// A phase expressed as a unipolar float [0.0, 1.0)
pub struct Phase(pub f64);

impl Copy for Phase {}

pub trait ClockSource {
    /// Get the phase of this clock.
    fn phase(&self) -> Phase;

    /// Get the current tick count of this clock.
    fn ticks(&self) -> i64;

    /// Find out if the clock ticked the last time it updated.
    fn ticked(&self) -> bool;
}

pub struct Clock {
    phase: f64,
    tick_count: i64,
    ticked: bool,
    rate: f64
}

impl Clock {
    pub fn new(rate: Rate) -> Self {
        let Rate(f) = rate;
        Clock {phase: 0.0, tick_count: 0, rate: f, ticked: true}
    }
}

impl ClockSource for Clock {
    fn phase(&self) -> Phase {Phase(self.phase)}
    fn ticks(&self) -> i64 {self.tick_count}
    fn ticked(&self) -> bool {self.ticked}
}

impl Update for Clock {
    fn update(&mut self, delta_t: DeltaT) {
        // determine how much phase has elapsed
        let DeltaT(dt) = delta_t;
        let elapsed_phase = self.rate * dt;
        let phase_unwrapped = self.phase + elapsed_phase;

        // Determine how many ticks have actually elapsed.  It may be more than 1.
        // It may also be negative if this clock has a negative rate.
        let accumulated_ticks = phase_unwrapped.floor() as i64;

        // This clock ticked if we accumulated +-1 or more ticks.
        self.ticked = accumulated_ticks.abs() > 0;
        self.tick_count += accumulated_ticks;

        self.phase = modulo(phase_unwrapped, 1.0);
    }
}