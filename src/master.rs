//! Top-level entity that owns all data and routes events.
use clock_network::{ClockNetwork, ClockEvent};
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

}