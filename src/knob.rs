use datatypes::Rate;
use std::cell::Cell;

pub enum KnobValue {
    Button(bool),
    Rate(Rate),
}

impl KnobValue {
    fn same_varaint(&self, other: &KnobValue) {
        match (*self, *other) with
            Rate(_), Rate(_) => true,
    }
}

pub type KnobId = usize;

/// A struct acting as a template for the creation of individual knobs.
pub struct KnobPrototype {
    /// The name of this knob.
    pub name: &'static str,
    /// The initial value of this knob, also serving as a type hint.
    pub value: KnobValue
}

pub struct Knob {
    pub name: &'static str,
    /// An explicit numeric identifier for this knob.
    /// These will be automatically assigned by the entity that this knob becomes
    /// associated with.
    pub id: KnobId,
    /// Provide an instance of a knob value to hint at the expected type.
    /// Also used to initialize the value upon construction.
    pub prototype_value: KnobValue,
    /// The current value of this knob.
    value: Cell<KnobValue>,
}

impl Knob {
    pub fn from_prototype(prototype: KnobPrototype, id: KnobId) -> Self {
        Knob {
            name: prototype.name,
            id: id,
            prototype_value: prototype.value,
            value: Cell::new(*initial_value)}
    }

    pub fn set(&self, value: &KnobValue) -> Result<(), KnobMessage> {
        if self.same_variant(value) {
            self.value.set(*value);
            Ok(())
        } else {
            Err(KnobMessage::TypeMismatch {
                expected: self.prototype_value,
                actual: value,
                name: self.name.to_string()})
        }
    }
}

pub enum KnobMessage {
    TypeMismatch { expected: KnobValue, actual: KnobValue, name: String }
}