//! Generic abstraction for a control knob.
//! Knobs should be able to accept all basic Wiggles values, as well as some additional
//! purpose-specific values.  Knobs have a native datatype, and under some circumstances should
//! attempt to convert other datatypes into their native datatype.  Unlike a generic Wiggles value,
//! this conversion may fail and the knob control action could return an error rather than setting
//! a value.
use std::sync::Arc;
use std::{error, fmt};
use super::{Datatype as WiggleDatatype, Data as WiggleData, Unipolar};
use super::knob_types::Rate;
use console_server::reactor::Messages;

// For now, knob data types will not be extensible but will instead be limited to Wiggles values
// and a few additional types we've defined here, otherwise the generic types get oppressively 
// complex (and we only expect to have one type of knob anyway).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Datatype {
    Wiggle(WiggleDatatype),
    Rate,
    Button,
    UFloat, // floating point number >= 0.0
    // Pick a value from a finite set of named items.
    // Would be better if this were a simpler impl but we'll roll with it and see how it pans out.
    Picker(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Data {
    Wiggle(WiggleData),
    Rate(Rate),
    Button(bool),
    UFloat(f64),
    Picker(String),
}

// Helper conversion functions for standard allowed conversions.
// We may want to be careful with these depending on how we inform clients about knob value changes.
// If we allow implicit conversions, then we should probably explicitly pass a message back up
// containing the exact data we used to set the knob.  This may not end up being important, though.
impl Data {
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
    /// Attempt to express this value as "unsigned float".
    /// If a wiggle is provided, convert it to a unipolar float.
    pub fn as_ufloat<A>(self) -> Result<f64, Error<A>> {
        match self {
            Data::UFloat(u) => Ok(u),
            Data::Wiggle(d) => {
                let Unipolar(u) = d.coerce().into();
                Ok(u)
            }
            _ => Err(badtype(Datatype::UFloat, self)),
        }
    }
    /// Unpack this knob data as a unipolar.
    /// Convert a Wiggle and ensure it is coerced.
    /// All other types are an error.
    pub fn as_unipolar<A>(self) -> Result<Unipolar, Error<A>> {
        match self {
            Data::Wiggle(d) => Ok(d.coerce().into()),
            _ => Err(badtype(Datatype::Wiggle(WiggleDatatype::Unipolar), self)),
        }
    }
    /// Unpack this knob data as a picker variant.
    /// Since we don't have access to the expected variants here, return an empty error and allow
    /// the client to decide what to do.
    pub fn as_picker(&self, ) -> Result<&str, ()> {
        match *self {
            Data::Picker(ref p) => Ok(p),
            _ => Err(()),
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

impl<T, A> Knobs<A> for Box<T> where T: Knobs<A> + ?Sized {
    fn knobs(&self) -> Vec<(A, KnobDescription)> {
        (**self).knobs()
    }
    fn knob_datatype(&self, addr: A) -> Result<Datatype, Error<A>> {
        (**self).knob_datatype(addr)
    }
    fn knob_value(&self, addr: A) -> Result<Data, Error<A>> {
        (**self).knob_value(addr)
    }
    fn set_knob(&mut self, addr: A, value: Data) -> Result<(), Error<A>> {
        (**self).set_knob(addr, value)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Command<A> {
    /// Set the value of a particular knob.
    Set(A, Data),
    /// A summary of the state of every knob in this system.
    State,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Responses related to actions on individual knobs or a knob subsystem.
pub enum Response<A> {
    ValueChange(A, Data),
    State(Vec<(A, KnobDescription)>),
    Added(A, KnobDescription),
    Removed(A),
}

impl<A> Response<A> {
    /// Use the provided function to lift this knob message into a higher address space.
    pub fn lift_address<NewAddr, F>(self, lifter: F) -> Response<NewAddr>
        where F: Fn(A) -> NewAddr
    {
        use self::Response::*;
        match self {
            ValueChange(addr, value) => ValueChange(lifter(addr), value),
            State(mut descs) => {
                let mut lifted = Vec::with_capacity(descs.len());
                for (addr, desc) in descs.drain(..) {
                    lifted.push((lifter(addr), desc))
                }
                State(lifted)
            }
            Added(addr, desc) => Added(lifter(addr), desc),
            Removed(addr) => Removed(lifter(addr)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error<A> {
    InvalidAddress(A),
    InvalidDatatype{expected: Datatype, provided: Data},
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

impl<A: fmt::Debug> fmt::Display for Error<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidAddress(ref a) => write!(f, "Invalid knob address: {:?}.", a),
            Error::InvalidDatatype{ref expected, ref provided} =>
                write!(f, "Knob expected datatype {:?} but received the data {:?}.", expected, provided),
        }
    }
}

impl<A: fmt::Debug> error::Error for Error<A> {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidAddress(_) => "Invalid knob address.",
            Error::InvalidDatatype{..} => "Invalid datatype for knob.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


/// Helper function for KnobError::InvalidDatatype.
pub fn badtype<A>(expected: Datatype, provided: Data) -> Error<A> {
    Error::InvalidDatatype {
        expected: expected,
        provided: provided,
    }
}

/// Shorthand for KnobError::InvalidAddress.
pub fn badaddr<A>(add: A) -> Error<A> {
    Error::InvalidAddress(add)
}
