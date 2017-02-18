//! Types for dataflow.

pub type EnumSize = u32;

/// Tag for describing datatypes in requests or other data structures.
pub enum Datatype {
    UnipolarFloat,
    BipolarFloat,
    IntegerEnum(EnumSize),
}

pub struct Unipolar(f64);

impl From<Bipolar> for Unipolar {
    fn from(bp: Bipolar) -> Self {
        let scaled = (bp.0 + 1.0) / 2.0;
        Unipolar(scaled)
    }
}

impl From<IntegerEnum> for Unipolar {
    fn from(IntegerEnum {value, size}: IntegerEnum) -> Self {
        Unipolar(value as f64 / (size - 1) as f64)
    }
}

pub struct Bipolar(f64);

impl From<Unipolar> for Bipolar {
    fn from(up: Unipolar) -> Self {
        let scaled = (up.0 * 2.0) - 1.0;
        Bipolar(scaled)
    }
}
pub struct IntegerEnum {value: EnumSize, size: EnumSize}

pub enum Data {
    /// A float on the range [0.0, 1.0].
    Unipolar(Unipolar),
    /// A float on the range [-1.0, 1.0].
    Bipolar(Bipolar),
    /// An integer on some finite-size range.
    Integer(IntegerEnum),
}