//! Generic abstraction for a control knob.
//! Knobs should be able to accept all basic Wiggles values, as well as some additional
//! purpose-specific values.  Knobs have a native datatype, and under some circumstances should
//! attempt to convert other datatypes into their native datatype.  Unlike a generic Wiggles value,
//! this conversion may fail and the knob control action could return an error rather than setting
//! a value.
use std::sync::Arc;
use super::{Datatype as WiggleDatatype, Data as WiggleData};
use super::knob_types::Rate;

// For now, knob data types will not be extensible but will instead be limited to Wiggles values
// and a few additional types we've defined here, otherwise the generic types get oppressively 
// complex (and we only expect to have one type of knob anyway).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
    Wiggle(WiggleDatatype),
    Rate,
    Button,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Wiggle(WiggleData),
    Rate(Rate),
    Button(bool),
}

// Helper conversion functions for standard allowed conversions.
// We may want to be careful with these depending on how we inform clients about knob value changes.
// If we allow implicit conversions, then we should probably explicitly pass a message back up
// containing the exact data we used to set the knob.  This may not end up being important, though.
impl Data {
    // Return the datatype matching this data's current value.
    pub fn datatype(&self) -> Datatype {
        match *self {
            Data::Wiggle(ref v) => Datatype::Wiggle(v.datatype()),
            Data::Rate(_) => Datatype::Rate,
            Data::Button(_) => Datatype::Button,
        }
    }
    /// Attempt to express this value as a rate.
    /// Do not convert any other datatype into a rate.
    pub fn as_rate<A>(self) -> Result<Rate, Error<A>> {
        match self {
            Data::Rate(r) => Ok(r),
            _ => Err(badtype(Datatype::Rate, self)),
        }
    }
    /// Attempt to express this value as a button state.
    /// Do not convert any other datatype into a button state.
    pub fn as_button<A>(self) -> Result<bool, Error<A>> {
        match self {
            Data::Button(b) => Ok(b),
            _ => Err(badtype(Datatype::Button, self)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// The description of a knob, for serialization out to clients.
/// Includes a knob name, and expected datatype.
/// We expect the knob address to be provided externally as knobs
/// aren't aware of their own address.
pub struct KnobDescription {
    pub name: Arc<String>,
    pub datatype: Datatype,
}

/// Entities that have a knob interface must implement this trait.
/// Generic over the type of data used to address an individual knob, enabling resuse of this
/// trait across multiple nested layers of knob-based subsystems.
/// Also generic over the particular datatype used.
pub trait Knobs<A> {
    /// List every knob available and the address at which it can be found.
    fn knobs(&self) -> Vec<(A, KnobDescription)>;

    /// Return the native datatype for the knob at this address or an error if it doesn't exist.
    fn knob_datatype(&self, addr: A) -> Result<Datatype, Error<A>>;

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: A) -> Result<Data, Error<A>>;

    /// Attempt to set a value on the knob at this address.
    fn set_knob(&mut self, addr: A, value: Data) -> Result<(), Error<A>>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Messages related to actions on individual knobs or a knob subsystem.
pub enum Message<A> {
    ValueChange{addr: A, value: Data},
    KnobAdded{addr: A, desc: KnobDescription},
    KnobRemoved(A),
}

impl<A> Message<A> {
    /// Use the provided function to lift this knob message into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> Message<NewAddr>
        where F: FnOnce(A) -> NewAddr, NewAddr: Copy
    {
        use self::Message::*;
        match self {
            ValueChange{addr, value} => ValueChange{addr: lifter(addr), value: value},
            KnobAdded{addr, desc} => KnobAdded{addr: lifter(addr), desc: desc},
            KnobRemoved(addr) => KnobRemoved(lifter(addr)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error<A> {
    InvalidAddress(A),
    InvalidDatatype{expected: Datatype, provided: Datatype},
}

impl<A> Error<A> {
    /// Use the provided function to lift this knob error into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> Error<NewAddr>
        where F: FnOnce(A) -> NewAddr
    {
        use self::Error::*;
        match self {
            InvalidAddress(a) => InvalidAddress(lifter(a)),
            InvalidDatatype{expected: e, provided: p} => InvalidDatatype{expected: e, provided: p},
        }
    }
}


/// Helper function for KnobError::InvalidDatatype.
pub fn badtype<A>(expected: Datatype, provided: Data) -> Error<A> {
    Error::InvalidDatatype {
        expected: expected,
        provided: provided.datatype(),
    }
}

/// Shorthand for KnobError::InvalidAddress.
pub fn badaddr<A>(add: A) -> Error<A> {
    Error::InvalidAddress(add)
}