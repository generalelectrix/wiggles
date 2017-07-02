//! Generic abstraction for a control knob.
//! Knobs should be able to accept all basic Wiggles values, as well as some additional
//! purpose-specific values.  Knobs have a native datatype, and under some circumstances should
//! attempt to convert other datatypes into their native datatype.  Unlike a generic Wiggles value,
//! this conversion may fail and the knob control action could return an error rather than setting
//! a value.
use std::sync::Arc;
use super::{Datatype as WigglesDatatype, Data as WigglesData};

// For now, knob data types will not be extensible but will instead be limited to Wiggles values
// and a few additional types we've defined here, otherwise the generic types get oppressively 
// complex (and we only expect to have one type of knob anyway).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
    Wiggles(WigglesDatatype),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Wiggles(WigglesData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The description of a knob, for serialization out to clients.
/// Includes a knob name, and expected datatype.
/// We expect the knob address to be provided externally as knobs
/// aren't aware of their own address.
pub struct KnobDescription {
    name: Arc<String>,
    datatype: Datatype,
}

/// Entities that have a knob interface must implement this trait.
/// Generic over the type of data used to address an individual knob, enabling resuse of this
/// trait across multiple nested layers of knob-based subsystems.
/// Also generic over the particular datatype used.
pub trait Knobs {
    type Addr: Copy;
    /// List every knob available and the address at which it can be found.
    fn knobs(&self) -> Vec<(Self::Addr, KnobDescription)>;

    /// Return the native datatype for the knob at this address or an error if it doesn't exist.
    //fn native_datatype_for(&self, addr: Self::Addr) -> Result<D::Datatype, KnobError<Self::Addr, D>>;

    /// Attempt to set a value on the knob at this address.
    fn set_knob(&mut self, addr: Self::Addr, value: Data) -> Result<(), KnobError<Self::Addr>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnobError<A> {
    InvalidAddress(A),
    InvalidDatatype{expected: Datatype, provided: Datatype},
}

impl<A> KnobError<A> {
    /// Use the provided function to lift this knob error into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> KnobError<NewAddr>
        where F: FnOnce(A) -> NewAddr, NewAddr: Copy
    {
        use self::KnobError::*;
        match self {
            InvalidAddress(a) => InvalidAddress(lifter(a)),
            InvalidDatatype{expected: e, provided: p} => InvalidDatatype{expected: e, provided: p},
        }
    }
}