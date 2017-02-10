//! Implementation of a clock that runs at a constant rate, controlled by a knob.
use update::DeltaT;
use utils::modulo_one;
use clock_network::{
    ClockValue,
    ClockNetwork,
    ComputeClock,
    UpdateClock,
    CompleteClock,
    ClockInputSocket,
    ClockNodePrototype,
    ClockNodeIndex,
    clock_button_update};
use knob::{Knob, KnobValue, KnobId, KnobPatch, KnobEvent};
use datatypes::Rate;
use event::Events;

pub const RATE_KNOB_ID: KnobId = 0;
pub const RESET_KNOB_ID: KnobId = 1;
pub const INIT_CLOCK_VAL: ClockValue = ClockValue { phase: 0.0, tick_count: 0, ticked: true };
pub const INIT_RATE: Rate = Rate::Hz(1.0);

/// The most basic clock, which ticks at a rate controlled by a knob.
pub struct Clock {
    value: ClockValue,
}

impl Clock {
    /// Create a new instance of this clock, and hide it behind the interface trait.
    fn create() -> Box<CompleteClock> {
        Box::new(Clock { value: INIT_CLOCK_VAL })
    }

    /// Produce the prototype for this type of clock.  This should only need to be called once,
    /// during program initialization.
    pub fn create_prototype() -> ClockNodePrototype {
        let rate_knob = Knob::new("rate", RATE_KNOB_ID, KnobValue::Rate(INIT_RATE));
        let reset_knob = Knob::new("reset", RESET_KNOB_ID, KnobValue::Button(false));
        let knobs = vec![rate_knob, reset_knob].into_boxed_slice();
        let inputs = Box::new([]);
        ClockNodePrototype::new("basic", inputs, knobs, Box::new(Clock::create))
    }
}

impl ComputeClock for Clock {
    fn compute_clock(&self,
                     _: &[ClockInputSocket],
                     _: &[Knob],
                     _: &ClockNetwork)
                     -> ClockValue { self.value }
}

impl UpdateClock for Clock {
    fn update(&mut self, id: ClockNodeIndex, knobs: &mut [Knob], dt: DeltaT) -> Events {
        debug_assert!(knobs.len() == 2);
        // if someone hit the reset button, register it and swap the knob value
        if knobs[RESET_KNOB_ID].get_button_state() {
            self.value = INIT_CLOCK_VAL;
            // set the knob back to the unpushed state
            let reset_knob = &mut knobs[RESET_KNOB_ID];
            reset_knob.set_button_state(false);

            // emit an event for the knob value change
            Some(clock_button_update(id, reset_knob, false))
        } else {
            // determine how much phase has elapsed
            let rate = knobs[RATE_KNOB_ID].rate().in_hz();
            let elapsed_phase = rate * dt;
            let phase_unwrapped = self.value.phase + elapsed_phase;

            // Determine how many ticks have actually elapsed.  It may be more than 1.
            // It may also be negative if this clock has a negative rate.
            let accumulated_ticks = phase_unwrapped.floor() as i64;

            // This clock ticked if we accumulated +-1 or more ticks.
            self.value.ticked = accumulated_ticks.abs() > 0;
            self.value.tick_count += accumulated_ticks;

            self.value.phase = modulo_one(phase_unwrapped);
            None
        }.into()


    }
}