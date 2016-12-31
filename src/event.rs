//! Master type for dispatching all application events.
use clock_network::ClockNodeIndex;
use knob::KnobPatch;

pub enum Event {
    /// Announce the availability of a new clock node.
    NewClock { name: String, type_name: &'static str, node_id: ClockNodeIndex },
    /// Announce the availability of new knobs.
    NewKnobs { patches: Vec<KnobPatch> },
}