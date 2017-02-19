//! Traits and types for generic push-based control parameters.
use std::collections::HashMap;
use std::error;
use std::fmt;

use clock_network::{ClockNodeIndex, ClockNode, ClockNetwork};
use data_network::{DataNodeIndex, DataNode, DataNetwork};
use datatypes::{Rate, ErrorMessage};
use event::Event;
use network::{NetworkNode, NetworkNodeId};


#[derive(PartialEq, Debug)]
/// Message enum encompassing knob-related events.
pub enum KnobEvent {
    /// Announce that a knob patch has successfully changed value.
    ValueChanged { patch: KnobPatch, value: KnobValue },
    KnobPatchesAdded(Vec<KnobPatch>),
    KnobPatchesDeleted(Vec<KnobPatch>),
}

/// A trait expressing that an entity exposes a Knob-based interface.
pub trait Knobs {
    /// Get an immutable slice of this entity's knobs.
    fn knobs(&self) -> &[Knob];

    /// Get a mutable slice of this entity's knobs.
    fn knobs_mut(&mut self) -> &mut [Knob];

    /// Set a new value on the given knob id.
    /// Returns an error if the id doesn't exist or the value doesn't have the
    /// right type.
    fn set_knob_value(&mut self, id: KnobId, value: KnobValue) -> Result<(), KnobError> {
        self.knob_mut(id).and_then(|knob| knob.set(value))
    }

    /// Get the current value of a given knob id.
    /// Returns an error if the id doesn't exist.
    fn get_knob_value(&self, id: KnobId) -> Result<KnobValue, KnobError> {
        self.knob(id).map(|ref k| k.value)
    }

    /// Get a reference to a knob, given an id.
    /// Returns an error if the id doesn't exist.
    fn knob(&self, id: KnobId) -> Result<&Knob, KnobError> {
        self.knobs().get(id).ok_or(KnobError::InvalidId(id))
    }

    /// Get a mutable reference to a knob, given an id.
    /// Returns an error if the id doesn't exist.
    fn knob_mut(&mut self, id: KnobId) -> Result<&mut Knob, KnobError> {
        self.knobs_mut().get_mut(id).ok_or(KnobError::InvalidId(id))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    name: &'static str,
    /// An explicit numeric identifier for this knob.
    /// These will be automatically assigned by the entity that this knob becomes
    /// associated with.
    id: KnobId,
    /// The current value of this knob.
    value: KnobValue,
}

impl Knob {
    /// Return the id this knob has been assigned.
    pub fn id(&self) -> KnobId { self.id }

    /// Return this knob's name.
    pub fn name(&self) -> &'static str { self.name }

    pub fn new(name: &'static str, id: KnobId, initial_value: KnobValue) -> Self {
        Knob {
            name: name,
            id: id,
            value: initial_value}
    }

    /// Assign a new value to this knob.  Only allow if the incoming value is
    /// the same as that which currently contained.
    pub fn set(&mut self, value: KnobValue) -> Result<(), KnobError> {
        if self.value.same_variant(&value) {
            self.value = value;
            Ok(())
        } else {
            Err(KnobError::TypeMismatch {
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

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct KnobPatch {
    node: KnobNode,
    id: KnobId,
}

impl KnobPatch {
    pub fn new<N: Into<KnobNode>>(node: N, id: KnobId) -> Self {
        KnobPatch {
            node: node.into(),
            id: id,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum KnobNode {
    Clock(ClockNodeIndex),
    Data(DataNodeIndex),
}

impl From<ClockNodeIndex> for KnobNode {
    fn from(n: ClockNodeIndex) -> Self {
        KnobNode::Clock(n)
    }
}

impl From<DataNodeIndex> for KnobNode {
    fn from(n: DataNodeIndex) -> Self {
        KnobNode::Data(n)
    }
}

#[derive(Debug)]
/// Keep track of all of the knobs and how to find them.
/// Responsible for routing knob updates to the appropriate place, and emitting
/// events to indicate that various things have happened.
pub struct PatchBay {
    /// Hold onto a prototype value for each knob so we can find out its type.
    patches: HashMap<KnobPatch, KnobValue>,
}

impl PatchBay {
    pub fn new() -> Self { PatchBay { patches: HashMap::new() }}

    fn add_node<N: Into<KnobNode>>(&mut self, node_id: N, knobs: &[Knob]) -> KnobEvent {
        let node_id = node_id.into();
        let patches: Vec<_> =
            knobs.iter()
                .map(|ref knob| {
                    let patch = KnobPatch::new(node_id, knob.id);
                    self.patches.insert(patch, knob.value);
                    patch
                }).collect();
        KnobEvent::KnobPatchesAdded(patches)
    }

    pub fn add_clock_node(&mut self, node: &ClockNode) -> KnobEvent {
        self.add_node(node.id(), node.knobs())
    }

    pub fn add_data_node(&mut self, node: &DataNode) -> KnobEvent {
        self.add_node(node.id(), node.knobs())
    }

    /// Remove all patches from a provided clock node, presumably because it has
    /// been removed.
    fn remove_node<N: Into<KnobNode>>(&mut self, node_id: N, knobs: &[Knob]) -> KnobEvent {
        let node_id = node_id.into();
        let patches: Vec<_> =
            knobs.iter()
                .map(|ref knob| {
                    let patch = KnobPatch::new(node_id, knob.id());
                    self.patches.remove(&patch);
                    patch
                }).collect();
        KnobEvent::KnobPatchesDeleted(patches)
    }

    pub fn remove_clock_node(&mut self, node: &ClockNode) -> KnobEvent {
        self.remove_node(node.id(), node.knobs())
    }

    pub fn remove_data_node(&mut self, node: &DataNode) -> KnobEvent {
        self.remove_node(node.id(), node.knobs())
    }

    pub fn set_knob_value(&self,
                          patch: KnobPatch,
                          value: KnobValue,
                          clock_network: &mut ClockNetwork,
                          data_network: &mut DataNetwork,
                          ) -> Result<KnobEvent, ErrorMessage> {
        // determine which graph to patch into
        match patch.node {
            KnobNode::Clock(node) => {
                clock_network.get_node_mut(node)?.set_knob_value(patch.id, value)?;
                Ok(KnobEvent::ValueChanged { patch: patch, value: value })
            }
            KnobNode::Data(node) => {
                data_network.get_node_mut(node)?.set_knob_value(patch.id, value)?;
                Ok(KnobEvent::ValueChanged { patch: patch, value: value })
            }
        }
    }
}

#[derive(Debug)]
pub enum KnobError {
    TypeMismatch { expected: KnobValue, actual: KnobValue, name: String },
    InvalidId(KnobId),
}

impl fmt::Display for KnobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KnobError
        ::TypeMismatch{ref expected, ref actual, ref name} => 
                write!(f,
                       "Type mismatch for knob '{}': knob is a {}, but received a {},",
                       name,
                       expected.type_name(),
                       actual.type_name()),
            KnobError
        ::InvalidId(id) => write!(f, "Invalid knob id: {}", id),
        }
    }
}

impl error::Error for KnobError {
    fn description(&self) -> &str { 
        match *self {
            KnobError
        ::TypeMismatch{..} => "Knob type mismatch.",
            KnobError
        ::InvalidId(_) => "Invalid knob id.",
        }
     }

     fn cause(&self) -> Option<&error::Error> { None }
}

/// Helper function to create an event to register a clock changing the state of
/// a button-type knob as a result of registering a transient button press.
pub fn button_update<N: Into<KnobNode>>(node: N, knob: &Knob, state: bool) -> Event {
    let patch = KnobPatch::new(node, knob.id());
    let value = KnobValue::Button(state);
    KnobEvent::ValueChanged { patch: patch, value: value }.into()
}