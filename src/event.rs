//! Master type for dispatching all application events.
use knob::KnobEvent;
use clock_network::ClockResponse;

pub enum Event {
    /// Generically allow multiple events to be grouped together.
    /// This allows returning either a single event or multiple events from
    /// APIs without requiring a memory allocation for a single event.
    EventCollection(Vec<Event>),
    Knob(KnobEvent),
    ClockResponse(ClockResponse),
}