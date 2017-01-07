//! Traits and types for generic push-based control parameters.
use std::collections::HashMap;
use std::error;
use std::fmt;

use datatypes::Rate;
use clock_network::{ClockNodeIndex, ClockNode, ClockGraph};


/// A trait expressing that an entity exposes a Knob-based interface.
pub trait Knobs {
    /// Get an immutable slice of this entity's knobs.
    fn knobs(&self) -> &[Knob];

    /// Get a mutable slice of this entity's knobs.
    fn knobs_mut(&mut self) -> &mut [Knob];

    /// Set a new value on the given knob id.
    /// Returns an error if the id doesn't exist or the value doesn't have the
    /// right type.
    fn set_knob_value(&mut self, id: KnobId, value: KnobValue) -> Result<(), KnobMessage> {
        if let Some(knob) = self.knobs_mut().get_mut(id) {
            knob.set(value)
        } else {
            Err(KnobMessage::InvalidId(id))
        }
    }

    /// Get the current value of a given knob id.
    /// Returns an error if the id doesn't exist.
    fn get_knob_value(&self, id: KnobId) -> Result<KnobValue, KnobMessage> {
        if let Some(knob) = self.knobs().get(id) {
            Ok(knob.value)
        } else {
            Err(KnobMessage::InvalidId(id))
        }
    }
}

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

    pub fn type_name(&self) -> &'static str {
        match *self {
            KnobValue::Button(_) => "Button",
            KnobValue::Rate(_) => "Rate",
            KnobValue::PositiveFloat(_) => "PositiveFloat",
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
    /// This method should probably only be called by the entity that owns a
    /// particular knob as it should implicitly be aware of its type.
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
    /// This method should probably only be called by the entity that owns a
    /// particular knob as it should implicitly be aware of its type.
    pub fn set_button_state(&mut self, state: bool) {
        self.set(KnobValue::Button(state))
            .expect("Failed to set state of '{}' as a button");
    }

    /// Get the value of a Rate knob, or panic.
    /// This method should probably only be called by the entity that owns a
    /// particular knob as it should implicitly be aware of its type.
    pub fn rate(&self) -> Rate {
        match self.value {
            KnobValue::Rate(r) => r,
            x => panic!("Tried to get a Rate value from a knob whose value is {:?}.", x)
        }
    }

    /// Get the value of a PositiveFloat knob, or panic.
    /// This method should probably only be called by the entity that owns a
    /// particular knob as it should implicitly be aware of its type.
    pub fn positive_float(&self) -> f64 {
        match self.value {
            KnobValue::PositiveFloat(f) => f,
            x => panic!("Tried to get a PositiveFloat value from a knob whose value is {:?}.", x)
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum KnobPatch {
    Clock { node: ClockNodeIndex, id: KnobId },
}

/// Keep track of all of the knobs and how to find them.
pub struct PatchBay {
    /// Hold onto a prototype value for each knob so we can find out its type.
    patches: HashMap<KnobPatch, KnobValue>,
}

impl PatchBay {
    pub fn new() -> Self { PatchBay { patches: HashMap::new() }}

    pub fn add_clock_node(&mut self, node: &ClockNode) {
        for knob in node.knobs.iter() {
            let patch = KnobPatch::Clock { node: node.id, id: knob.id };
            self.patches.insert(patch, knob.value);
        }
    }

    pub fn set_knob_value(&self,
                          patch: KnobPatch,
                          value: KnobValue,
                          cg: &ClockGraph)
                          -> Result<(), KnobMessage> {
        // determine which graph to patch into
        // match patch {
        //     Clock { node: node, id: id } => {
        //         cg.
        //     }
        // }
        Ok(())
    }
}

#[derive(Debug)]
pub enum KnobMessage {
    TypeMismatch { expected: KnobValue, actual: KnobValue, name: String },
    InvalidId(KnobId),
}

impl fmt::Display for KnobMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KnobMessage::TypeMismatch{ref expected, ref actual, ref name} => 
                write!(f,
                       "Type mismatch for knob '{}': knob is a {}, but received a {},",
                       name,
                       expected.type_name(),
                       actual.type_name()),
            KnobMessage::InvalidId(id) => write!(f, "Invalid knob id: {}", id),
        }
    }
}

impl error::Error for KnobMessage {
    fn description(&self) -> &str { 
        match *self {
            KnobMessage::TypeMismatch{..} => "Knob type mismatch.",
            KnobMessage::InvalidId(_) => "Invalid knob id.",
        }
     }

     fn cause(&self) -> Option<&error::Error> { None }
}