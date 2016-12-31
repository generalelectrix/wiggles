use datatypes::Rate;

#[derive(Clone, Copy, Debug)]
pub enum KnobValue {
    /// A boolean flag indicating if a button press occurred.
    /// The consumer of a knob is expected to reset this after registering the event.
    Button(bool),
    /// Something with the units of a rate.
    Rate(Rate),
    /// Unrestricted floating point number that must be positive or zero.
    PositiveFloat(f64),
}

impl KnobValue {
    pub fn same_variant(&self, other: &KnobValue) -> bool {
        match (*self, *other) {
            (KnobValue::Rate(_), KnobValue::Rate(_)) => true,
            (KnobValue::Button(_), KnobValue::Button(_)) => true,
            (KnobValue::PositiveFloat(_), KnobValue::PositiveFloat(_)) => true,
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
    value: KnobValue,
}

impl Knob {
    pub fn new(name: &'static str, id: KnobId, initial_value: KnobValue) -> Self {
        Knob {
            name: name,
            id: id,
            value: initial_value}
    }

    /// Assign a new value to this knob.  Only allow if the incoming value is
    /// the same as that which currently contained.
    pub fn set(&mut self, value: KnobValue) -> Result<(), KnobMessage> {
        if self.value.same_variant(&value) {
            self.value = value;
            Ok(())
        } else {
            Err(KnobMessage::TypeMismatch {
                expected: self.value,
                actual: value,
                name: self.name.to_string()})
        }
    }

    /// Get the value of this knob as a button event, or panic.
    pub fn get_button_state(&self) -> bool {
        match self.value {
            KnobValue::Button(state) => state,
            x => panic!("Tried to get a Button value from the knob '{}' whose value is {:?}.",
                        self.name,
                        x)
        }
    }

    /// Set the value of this knob as a button event state.
    /// Panics if this knob isn't a button.
    pub fn set_button_state(&mut self, state: bool) {
        self.set(KnobValue::Button(state))
            .expect("Failed to set state of '{}' as a button");
    }

    /// Get the value of a Rate knob, or panic.
    pub fn rate(&self) -> Rate {
        match self.value {
            KnobValue::Rate(r) => r,
            x => panic!("Tried to get a Rate value from a knob whose value is {:?}.", x)
        }
    }

    /// Get the value of a PositiveFloat knob, or panic.
    pub fn positive_float(&self) -> f64 {
        match self.value {
            KnobValue::PositiveFloat(f) => f,
            x => panic!("Tried to get a PositiveFloat value from a knob whose value is {:?}.", x)
        }
    }
}

#[derive(Debug)]
pub enum KnobMessage {
    TypeMismatch { expected: KnobValue, actual: KnobValue, name: String }
}