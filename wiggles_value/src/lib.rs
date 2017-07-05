//! Types for dataflow.
//! TODO: consider how to allow injection of different conversion behaviors for
//! flexibility.  We may want to use abs() or rescale to go from bipolar to
//! unipolar, for example.
use std::cmp::{min, max};
use std::ops::{Deref, Mul, Add};

extern crate serde;
#[macro_use] extern crate serde_derive;

pub mod blend;
pub mod knob_types;
pub mod knob;

/// Return True if two f64 are within 10^-6 of each other.
/// This is OK because all of our floats are on the unit range, so even though
/// this comparison is absolute it should be good enough for art.
#[inline(always)]
fn almost_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// Tag for describing datatypes in requests or other data structures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum Datatype {
    Unipolar,
    Bipolar,
}

#[derive(Clone, Copy, Debug, PartialOrd, Serialize, Deserialize)]
pub struct Unipolar(pub f64);

impl PartialEq for Unipolar {
    fn eq(&self, other: &Unipolar) -> bool {
        almost_eq(self.0, other.0)
    }
}

impl Eq for Unipolar {}

impl Unipolar {
    pub fn coerce(self) -> Self {
        Unipolar(self.0.min(1.0).max(0.0))
    }
}


impl From<Bipolar> for Unipolar {
    /// Convert to unipolar by taking the absolute value.
    fn from(bp: Bipolar) -> Self {
        Unipolar(bp.0.abs())
    }
}

impl From<Data> for Unipolar {
    fn from(d: Data) -> Self {
        match d {
            Data::Unipolar(up) => up,
            Data::Bipolar(bp) => bp.into(),
        }
    }
}

impl Default for Unipolar {
    fn default() -> Self {
        Unipolar(0.0)
    }
}

impl Deref for Unipolar {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Mul for Unipolar {
    type Output = Unipolar;
    fn mul(self, rhs: Unipolar) -> Self::Output {
        Unipolar(self.0 * rhs.0)
    }
}

impl Add for Unipolar {
    type Output = Unipolar;
    fn add(self, rhs: Unipolar) -> Self::Output {
        Unipolar(self.0 + rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, Serialize, Deserialize)]
pub struct Bipolar(pub f64);

impl PartialEq for Bipolar {
    fn eq(&self, other: &Bipolar) -> bool {
        almost_eq(self.0, other.0)
    }
}

impl Eq for Bipolar {}

impl Bipolar {
    pub fn coerce(self) -> Self {
        Bipolar(self.0.min(1.0).max(-1.0))
    }
}

impl From<Unipolar> for Bipolar {
    /// We just take the value and interpret it as bipolar.
    fn from(up: Unipolar) -> Self {
        Bipolar(up.0)
    }
}

impl From<Data> for Bipolar {
    fn from(d: Data) -> Self {
        match d {
            Data::Unipolar(up) => up.into(),
            Data::Bipolar(bp) => bp,
        }
    }
}

impl Default for Bipolar {
    fn default() -> Self {
        Bipolar(0.0)
    }
}

impl Deref for Bipolar {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Allow scaling a bipolar with a unipolar value.
impl Mul<Unipolar> for Bipolar {
    type Output = Bipolar;
    fn mul(self, rhs: Unipolar) -> Self::Output {
        Bipolar(self.0 * rhs.0)
    }
}

impl Mul for Bipolar {
    type Output = Bipolar;
    fn mul(self, rhs: Bipolar) -> Self::Output {
        Bipolar(self.0 * rhs.0)
    }
}

impl Add for Bipolar {
    type Output = Bipolar;
    fn add(self, rhs: Bipolar) -> Self::Output {
        Bipolar(self.0 + rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Data {
    /// A float on the range [0.0, 1.0].
    Unipolar(Unipolar),
    /// A float on the range [-1.0, 1.0].
    Bipolar(Bipolar),
}

impl Data {
    /// Get a copy of this data with its value clipped to be inside the valid range.
    pub fn coerce(&self) -> Self {
        match *self {
            Data::Unipolar(up) => Data::Unipolar(up.coerce()),
            Data::Bipolar(bp) => Data::Bipolar(bp.coerce()),
        }
    }

    pub fn datatype(&self) -> Datatype {
        match *self {
            Data::Unipolar(_) => Datatype::Unipolar,
            Data::Bipolar(_) => Datatype::Bipolar,
        }
    }

    pub fn as_type(&self, datatype: Datatype) -> Self {
        match datatype {
            Datatype::Bipolar => Data::Bipolar((*self).into()),
            Datatype::Unipolar => Data::Unipolar((*self).into()),
        }
    }

    /// Provide a default value based on an optional type hint.
    /// Provide unipolar if no type hint is provided.
    pub fn default_with_type_hint(type_hint: Option<Datatype>) -> Self {
        match type_hint {
            Some(Datatype::Unipolar) | None => Data::Unipolar(Unipolar::default()),
            Some(Datatype::Bipolar) => Data::Bipolar(Bipolar::default()),
        }
    }
}