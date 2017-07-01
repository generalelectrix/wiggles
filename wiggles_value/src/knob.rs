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
pub trait Knobs<A, D>
    where D: KnobData
{
    /// List every knob available and the address at which it can be found.
    fn list_all(&self) -> Vec<(A, KnobDescription<D::Datatype>)>;

    /// Return the native datatype for the knob at this address or an error if it doesn't exist.
    fn native_datatype_for(&self, addr: A) -> Result<D::Datatype, KnobError<A, D>>;

    /// Attempt to set a value on the knob at this address.
    fn try_set(&mut self, addr: A, value: D) -> Result<(), KnobError<A, D>>;
}

pub enum KnobError<A, D>
    where D: KnobData
{
    InvalidAddress(A),
    InvalidDatatype{expected: D::Datatype, provided: D::Datatype},
}