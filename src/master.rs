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
                unimplemented!()
            },
            KnobEvent::KnobPatchesAdded(patches) => {
                unimplemented!()
            },
            KnobEvent::KnobPatchesDeleted(patches) => {
                unimplemented!()
            },
        }
    }

    fn handle_clock_response(&mut self, e: ClockResponse) -> EventHandleResult {
        match e {
            ClockResponse::ClockNodeAdded{node, name} => {
                // need to add all of the relevant patches to the patch bay
                let node = self.clock_network.get_node(node)?;
                let knob_event = self.patch_bay.add_clock_node(node);
                // TODO: other actions taken from a new clock node appearing
                Ok(Events::single(knob_event))
            },
            ClockResponse::ClockNodeRemoved{node, name} => {
                // delete all patches from the patch bay
                let node = self.clock_network.get_node(node)?;
                let knob_event = self.patch_bay.remove_clock_node(node);
                // TODO: other actions taken from a clock node disappearing
                Ok(Events::single(knob_event))
            }
            ClockResponse::InputSwapped{node, input_id, new_input} => {
                // inform listeners about the change in input
                unimplemented!()
            }
        }
    }
}