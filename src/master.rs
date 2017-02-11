//! Top-level entity that owns all data and routes events.
use clock_network::{ClockNetwork, ClockResponse};
use event::{Event, Events};
use knob::{KnobEvent, PatchBay};
use datatypes::ErrorMessage;

#[derive(Debug)]
pub struct Master {
    patch_bay: PatchBay,
    clock_network: ClockNetwork,
}

type EventHandleResult = Result<Events,ErrorMessage>;

impl Master {
    /// Handle a single event, provided by some event queue.
    /// Returns any events resulting from processing this event.
    pub fn handle_event(&mut self, e: Event) -> EventHandleResult {
        match e {
            Event::Knob(e) => self.handle_knob_event(e),
            Event::ClockResponse(e) => self.handle_clock_response(e),
        }
    }

    fn handle_knob_event(&mut self, e: KnobEvent) -> EventHandleResult {
        match e {
            KnobEvent::ChangeValue{patch, value} => {
                self.patch_bay.set_knob_value(patch, value, &mut self.clock_network)
                              .map(Events::single)
            },
            KnobEvent::ValueChanged{patch, value} => {
                Ok(None.into())
            },
            KnobEvent::KnobPatchesAdded(patches) => {
                Ok(None.into())
            },
            KnobEvent::KnobPatchesDeleted(patches) => {
                Ok(None.into())
            },
        }
    }

    fn handle_clock_response(&mut self, e: ClockResponse) -> EventHandleResult {
        Ok(None.into())
    }
}