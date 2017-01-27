//! A clock that sits doing nothing until it receives a trigger.
//! It then ticks and runs for one period, then stops without ticking again.
//! Receiving a trigger while the clock is running causes it to reset, tick, and run.
use update::DeltaT;
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
use knob::{Knob, KnobValue, KnobId};
use datatypes::Rate;
use event::Event;

const RATE_KNOB_ID: KnobId = 0;
const TRIGGER_KNOB_ID: KnobId = 1;
const INIT_CLOCK_VAL: ClockValue = ClockValue { phase: 0.0, tick_count: 0, ticked: false };
const INIT_RATE: Rate = Rate::Hz(1.0);

/// A clock that runs for one cycle when triggered.
pub struct TriggeredClock {
    value: ClockValue,
    running: bool,
}

impl TriggeredClock {
    /// Create a new instance of this clock, and hide it behind the interface trait.
    fn create() -> Box<CompleteClock> {
        Box::new(TriggeredClock { value: INIT_CLOCK_VAL, running: false })
    }

    /// Produce the prototype for this type of clock.  This should only need to be called once,
    /// during program initialization.
    pub fn create_prototype() -> ClockNodePrototype {
        let rate_knob = Knob::new("rate", RATE_KNOB_ID, KnobValue::Rate(INIT_RATE));
        let trigger_knob = Knob::new("trigger", TRIGGER_KNOB_ID, KnobValue::Button(false));
        let knobs = vec![rate_knob, trigger_knob].into_boxed_slice();
        let inputs = Box::new([]);
        ClockNodePrototype::new("triggered", inputs, knobs, Box::new(TriggeredClock::create))
    }
}

impl ComputeClock for TriggeredClock {
    fn compute_clock(&self,
                     _: &[ClockInputSocket],
                     _: &[Knob],
                     _: &ClockNetwork)
                     -> ClockValue { self.value }
}

impl UpdateClock for TriggeredClock {
    fn update(&mut self, id: ClockNodeIndex, knobs: &mut [Knob], dt: DeltaT) -> Option<Event> {
        debug_assert!(knobs.len() == 2);
        // if someone hit the trigger button, register it and swap the knob value
        // FIXME: should emit an event announcing the change in button state.
        if knobs[TRIGGER_KNOB_ID].get_button_state() {
            self.value.phase = 0.0;
            self.value.ticked = true;
            self.value.tick_count += 1;
            self.running = true;

            let trigger_knob = &mut knobs[TRIGGER_KNOB_ID];
            trigger_knob.set_button_state(false);
            Some(clock_button_update(id, trigger_knob, false))
        } else if self.running {
            // this clock only ever ticks as a result of a button press
            self.value.ticked = false;

            // if the clock is running, evolve the phase.
            let rate = knobs[RATE_KNOB_ID].rate().in_hz();
            let elapsed_phase = rate * dt;
            let phase_unwrapped = self.value.phase + elapsed_phase;

            // if we finished a cycle, stop running and reset to zero
            if phase_unwrapped >= 1.0 {
                self.value.phase = 0.0;
                self.running = false;
            } else {
                self.value.phase = phase_unwrapped;
            }
            None
        } else { None }
    }
}
