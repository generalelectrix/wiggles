//! Top-level entity that owns all data and routes events.
use clock_network::{ClockNetwork, ClockResponse};
use event::Events;
use knob::{KnobEvent, PatchBay};

#[derive(Debug)]
pub struct Master {
    patch_bay: PatchBay,
    clock_network: ClockNetwork,
}

