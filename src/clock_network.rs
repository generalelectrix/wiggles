//! Implementation of a network of potentially dependent clocks.
//! Provides interfaces to modify the network, feed in control parameters,
//! and tap into outputs from individual clocks.
use clock::ClockSource;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::{Rc, Weak};

pub struct ClockNetwork {
    nodes: HashMap<String, Rc<ClockNode>>,
}

/// Things that can go wrong when working with the clock network.
pub enum ClockMessage {
    DuplicateName(ClockNode),
    NameNotFound(String),
}

impl ClockNetwork {
    pub fn new() -> Self {
        ClockNetwork { nodes: HashMap::new() }
    }

    /// Add a clock to the network.  If the clock's name already exists,
    /// return the clock in a resulting error.
    pub fn add(&mut self, clock: ClockNode) -> Result<(),ClockMessage> {
        match self.nodes.entry(clock.name.clone()) {
            Entry::Occupied(_) => Err(ClockMessage::DuplicateName(clock)),
            Entry::Vacant(slot) => {
                slot.insert(Rc::new(clock));
                Ok(())
            }
        }
    }

    /// Remove a named clock from the network.  If no clock exists, an
    /// error is returned.
    pub fn remove(&mut self, name: &str) -> Result<(), ClockMessage> {
        match self.nodes.remove(name) {
            Some(_) => Ok(()),
            None => Err(ClockMessage::NameNotFound(name.to_string()))
        }
    }
}

pub struct ClockNode {
    pub name: String,
    clock: Box<ClockSource>,
    listeners: Vec<Weak<ClockInputSocket>>,
}

/// An input port for receiving a clock signal.
/// Optionally can be named for display purposes.
pub struct ClockInputSocket {
    pub name: Option<&'static str>,
    source: Rc<ClockNode>
}

impl ClockSource for ClockInputSocket {
    fn phase(&self) -> f64 {self.source.clock.phase()}
    fn ticks(&self) -> i64 {self.source.clock.ticks()}
    fn ticked(&self) -> bool {self.source.clock.ticked()}
}