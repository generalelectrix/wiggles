//! Generic abstraction for a control knob.
//! Knobs should be able to accept all basic Wiggles values, as well as some additional
//! purpose-specific values.  Knobs have a native datatype, and under some circumstances should
//! attempt to convert other datatypes into their native datatype.  Unlike a generic Wiggles value,
//! this conversion may fail and the knob control action could return an error rather than setting
//! a value.
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The description of a knob, for serialization out to clients.
/// Current value of the knob is not included.
pub struct KnobDescription<D: Clone> {
    name: Arc<String>,
    datatype: D,
}

/// Marker trait for knob data, attaching the valueless type that describes the expected datatype
/// of a knob without including any actual data.
pub trait KnobData: Sized {
    type Datatype: Clone;
}

/// Entities that have a knob interface must implement this trait.
/// Generic over the type of data used to address an individual knob, enabling resuse of this
/// trait across multiple nested layers of knob-based subsystems.
/// Also generic over the particular datatype used.
pub trait Knobs<D>
    where D: KnobData
{   
    type Addr: Copy;
    /// List every knob available and the address at which it can be found.
    fn knobs(&self) -> Vec<(Self::Addr, KnobDescription<D::Datatype>)>;

    /// Return the native datatype for the knob at this address or an error if it doesn't exist.
    //fn native_datatype_for(&self, addr: Self::Addr) -> Result<D::Datatype, KnobError<Self::Addr, D>>;

    /// Attempt to set a value on the knob at this address.
    fn set_knob(&mut self, addr: Self::Addr, value: D) -> Result<(), KnobError<Self::Addr, D>>;
}

pub enum KnobError<A, D>
    where D: KnobData, A: Copy
{
    InvalidAddress(A),
    InvalidDatatype{expected: D::Datatype, provided: D::Datatype},
}

impl<A, D> KnobError<A, D>
    where D: KnobData, A: Copy
{
    /// Use the provided function to lift this knob error into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> KnobError<NewAddr, D>
        where F: FnOnce(A) -> NewAddr, NewAddr: Copy
    {
        use self::KnobError::*;
        match self {
            InvalidAddress(a) => InvalidAddress(lifter(a)),
            InvalidDatatype{expected: e, provided: p} => InvalidDatatype{expected: e, provided: p},
        }
    }
}