//! Master type for dispatching all application events.
use knob::KnobEvent;
use clock_network::ClockResponse;
use std::iter::FromIterator;

/// Top-level container for classes of events the dataflow networks may emit.
pub enum Event {
    /// Generically allow multiple events to be grouped together.
    /// This allows returning either a single event or multiple events from
    /// APIs without requiring a memory allocation for a single event.
    EventCollection(Vec<Event>),
    Knob(KnobEvent),
    ClockResponse(ClockResponse),
}

/// Iterator over a sequence of events.
pub struct Events {

}

/// Collect an iterator of Events into the right variant.
impl FromIterator<Event> for Event {
    fn from_iter<I: IntoIterator<Item=Event>>(iter: I) -> Self {
        let mut events = Vec::new();
        for event in iter:
            match event {
                Event::EventCollection(incoming) => events.append(incoming);,
                single => events.push(single);
            }
        if events.len()
    }
}