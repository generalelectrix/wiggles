//! Types for dataflow.
use std::ops::Deref;

pub enum DataError {
    EnumSizeLessThanTwo,
}

pub type EnumValue = u32;

// must be greater than 1 to make sense
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct EnumSize(EnumValue);

impl EnumSize {
    pub fn create(size: EnumValue) -> Result<Self,DataError> {
        if size < 1 {
            Err(DataError::EnumSizeLessThanTwo)
        }
        else {
            Ok(EnumSize(size))
        }
    }
}

impl Deref for EnumSize {
    type Target = u32;
    fn deref(&self) -> &u32 {
        &self.0
    }
}

/// Tag for describing datatypes in requests or other data structures.
pub enum Datatype {
    Unipolar,
    Bipolar,
    UInt(EnumSize),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Unipolar(pub f64);

impl From<Bipolar> for Unipolar {
    fn from(bp: Bipolar) -> Self {
        Unipolar((bp.0 + 1.0) / 2.0)
    }
}

impl From<IntegerEnum> for Unipolar {
    fn from(IntegerEnum {value, size}: IntegerEnum) -> Self {
        Unipolar(value as f64 / (size.0 - 1) as f64)
    }
}

impl From<Data> for Unipolar {
    fn from(d: Data) -> Self {
        match d {
            Data::Unipolar(up) => up,
            Data::Bipolar(bp) => bp.into(),
            Data::UInt(ie) => ie.into(),
        }
    }
}

impl Unipolar {
    fn into_uint(self, size: EnumSize) -> IntegerEnum {
        let val = (self.0 * (size.0 - 1) as f64) as EnumValue;
        IntegerEnum {value: val, size: size}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Bipolar(pub f64);

impl From<Unipolar> for Bipolar {
    fn from(up: Unipolar) -> Self {
        Bipolar((up.0 * 2.0) - 1.0)
    }
}

impl From<IntegerEnum> for Bipolar {
    fn from(ie: IntegerEnum) -> Self {
        // use our unipolar conversion
        let as_unipolar: Unipolar = ie.into();
        as_unipolar.into()
    }
}

impl Bipolar {
    fn into_uint(self, size: EnumSize) -> IntegerEnum {
        let as_unipolar: Unipolar = self.into();
        as_unipolar.into_uint(size)
    }
}

impl From<Data> for Bipolar {
    fn from(d: Data) -> Self {
        match d {
            Data::Unipolar(up) => up.into(),
            Data::Bipolar(bp) => bp,
            Data::UInt(ie) => ie.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct IntegerEnum {pub value: EnumValue, pub size: EnumSize}

impl IntegerEnum {

    pub fn into_uint(self, size: EnumSize) -> IntegerEnum {
        if size == self.size {
            self
        }
        else {
            let as_unipolar: Unipolar = self.into();
            as_unipolar.into_uint(size)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Data {
    /// A float on the range [0.0, 1.0].
    Unipolar(Unipolar),
    /// A float on the range [-1.0, 1.0].
    Bipolar(Bipolar),
    /// An integer on some finite-size range.
    UInt(IntegerEnum),
}

impl Data {
    pub fn into_uint(self, size: EnumSize) -> IntegerEnum {
        match self {
            Data::Unipolar(up) => up.into_uint(size),
            Data::Bipolar(bp) => bp.into_uint(size),
            Data::UInt(ie) => ie.into_uint(size),
        }
    }
}