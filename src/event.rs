//! Master type for dispatching all application events.
use knob::KnobEvent;
use clock_network::ClockResponse;
use std::iter::{FromIterator, IntoIterator};

/// Top-level container for classes of events the dataflow networks may emit.
pub enum Event {
    Knob(KnobEvent),
    ClockResponse(ClockResponse),
}

pub struct Events(Vec<Event>);

impl Events {
    pub fn new() -> Self {
        Events(Vec::new())
    }

    pub fn single(e: Event) -> Self {
        Events(vec!(e))
    }

    pub fn extend(&mut self, es: Events) {
        self.0.extend(es.0);
    }
}

impl From<Option<Event>> for Events {
    fn from(event: Option<Event>) -> Self {
        match event {
            Some(e) => Events::single(e),
            None => Events::new()
        }
    }
}

impl FromIterator<Event> for Events {
    fn from_iter<I: IntoIterator<Item=Event>>(i: I) -> Self {
        let event_vec: Vec<Event> = i.into_iter().collect();
        Events(event_vec)
    }
}

impl IntoIterator for Events {
    type Item = Event;
    type IntoIter = ::std::vec::IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}