use datatypes::Rate;
use std::cell::Cell;

pub enum KnobValue {
    Rate(Rate),
}

pub type KnobId = usize;

pub struct Knob {
    name: &'static str,
    /// An explicit numeric identifier for this knob.
    /// These should start at 0 and increase by one for each entity that
    /// provides knobs.
    id: KnobId,
    /// Provide an instance of a knob value to hint at the expected type.
    /// Also used to initialize the value upon construction.
    prototype_value: KnobValue,
    /// The current value of this knob.
    value: Cell<KnobValue>,
}

impl Knob {
    pub fn new(name: &'static str, id: KnobId, initial_value: KnobValue) -> Self {
        Knob {
            name: name,
            id: id,
            prototype_value: initial_value,
            value: Cell::new(initial_value)}
    }
}