//! Master type encalsulating all of the various events which may be emiited
//! as the result of servicing some request.
use knob::KnobEvent;
use clock_network::ClockEvent;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Index;

#[derive(Debug, PartialEq)]
/// Top-level container for classes of events the dataflow networks may emit.
pub enum Event {
    Knob(KnobEvent),
    Clock(ClockEvent),
}

impl From<KnobEvent> for Event {
    fn from(event: KnobEvent) -> Self {
        Event::Knob(event)
    }
}

impl From<ClockEvent> for Event {
    fn from(event: ClockEvent) -> Self {
        Event::Clock(event)
    }
}

#[derive(Debug, PartialEq)]
pub struct Events(Vec<Event>);

impl Events {
    pub fn new() -> Self {
        Events(Vec::new())
    }

    pub fn single<E>(e: E) -> Self 
            where E: Into<Event> {
        Events(vec!(e.into()))
    }

    pub fn extend(&mut self, es: Events) {
        self.0.extend(es.0);
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

impl Index<usize> for Events {
    type Output = Event;
    fn index(&self, i: usize) -> &Event {
        &self.0[i]
    }
}