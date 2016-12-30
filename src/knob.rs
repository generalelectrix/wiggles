use datatypes::Rate;
use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub enum KnobValue {
    Button(bool),
    Rate(Rate),
}

impl KnobValue {
    pub fn same_variant(&self, other: &KnobValue) -> bool {
        match (*self, *other) {
            (KnobValue::Rate(_), KnobValue::Rate(_)) => true,
            (KnobValue::Button(_), KnobValue::Button(_)) => true,
            (_, _) => false,
        }
    }
}

pub type KnobId = usize;

#[derive(Clone, Debug)]
pub struct Knob {
    pub name: &'static str,
    /// An explicit numeric identifier for this knob.
    /// These will be automatically assigned by the entity that this knob becomes
    /// associated with.
    pub id: KnobId,
    /// The current value of this knob.
    value: Cell<KnobValue>,
}

impl Knob {
    pub fn new(name: &'static str, id: KnobId, initial_value: KnobValue) -> Self {
        Knob {
            name: name,
            id: id,
            value: Cell::new(initial_value)}
    }

    /// Assign a new value to this knob.  Only allow if the incoming value is
    /// the same as that which currently contained.
    pub fn set(&self, value: KnobValue) -> Result<(), KnobMessage> {
        if self.value.get().same_variant(&value) {
            self.value.set(value);
            Ok(())
        } else {
            Err(KnobMessage::TypeMismatch {
                expected: self.value.get(),
                actual: value,
                name: self.name.to_string()})
        }
    }

    /// Get the value of this knob as a button event, or panic.
    pub fn button_state(&self) -> bool {
        match self.value.get() {
            KnobValue::Button(state) => state,
            x => panic!("Tried to get a Button value from a knob whose value is {:?}.", x)
        }
    }

    /// Get the value of a Rate knob, or panic.
    pub fn rate(&self) -> Rate {
        match self.value.get() {
            KnobValue::Rate(r) => r,
            x => panic!("Tried to get a Rate value from a knob whose value is {:?}.", x)
        }
    }
}

pub enum KnobMessage {
    TypeMismatch { expected: KnobValue, actual: KnobValue, name: String }
}