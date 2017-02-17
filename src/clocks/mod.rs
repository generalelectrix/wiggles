//! Namespace for creating clock node behaviors.
mod basic;
mod multiplier;
mod triggered;

pub use self::basic::Clock;
pub use self::multiplier::ClockMultiplier;
pub use self::triggered::TriggeredClock;

use clock_network::{ClockNodePrototype, ClockNodeIndex, clock_button_update};
use knob::{Knob, KnobId};
use event::Events;

#[cfg(test)]
mod test;

/// Return all of the clock prototypes.
pub fn create_prototypes() -> Vec<ClockNodePrototype> {
    vec![Clock::create_prototype(),
         ClockMultiplier::create_prototype(),
         TriggeredClock::create_prototype(),]
}

/// If the selected knob id was a button press, swap the state of the button to false, run the
/// provided action, and return its Events along with an event indicating that the button state was
/// changed.  Otherwise, return None.
pub fn action_if_button_pressed<F>(id: ClockNodeIndex, knobs: &mut [Knob], knob_id: KnobId, mut action: F) -> Option<Events>
    where F: FnMut() -> Events {
        let knob = &mut knobs[knob_id];
        if knob.get_button_state() {
            let action_events = action();
            knob.set_button_state(false);
            let mut reset_events = Events::single(clock_button_update(id, knob, false));
            reset_events.extend(action_events);
            Some(reset_events)
        }
        else {
            None
        }
}