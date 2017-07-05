//! Traits for various supported blending operations.
//! Implementations for all basic Wiggles data types are provided.
//! Note that blending performs no coercions; we allow infinite headroom inside the dataflow
//! networks and avoid clipping until it is absolutely necessary.
use std::f64;
use super::{Data, Unipolar, Bipolar};

pub trait Blend {
    /// Perform additive blending.
    fn add(base: Data, top: Data) -> Data;

    /// Perform multiplicative blending.
    fn mult(base: Data, top: Data) -> Data;

    /// Perform max value blending.
    fn max(base: Data, top: Data) -> Data;
}

/// Interpret a base layer and top layer as Unipolar and blend them.
impl Blend for Unipolar {
    /// Perform additive blending.
    fn add(base: Data, top: Data) -> Data {
        let base: Unipolar = base.into();
        Data::Unipolar(base + top.into())
    }

    /// Perform multiplicative blending.
    fn mult(base: Data, top: Data) -> Data {
        let base: Unipolar = base.into();
        Data::Unipolar(base * top.into())
    }

    /// Perform max value blending.
    fn max(base: Data, top: Data) -> Data {
        let Unipolar(base) = base.into();
        let Unipolar(top) = top.into();
        Data::Unipolar(Unipolar(f64::max(base, top)))
    }
}

/// Interpret a base layer and top layer as Bipolar and blend them.
impl Blend for Bipolar {
    /// Perform additive blending.
    fn add(base: Data, top: Data) -> Data {
        let base: Bipolar = base.into();
        Data::Bipolar(base + top.into())
    }

    /// Perform multiplicative blending.
    fn mult(base: Data, top: Data) -> Data {
        let Bipolar(base) = base.into();
        let Bipolar(top) = top.into();
        Data::Bipolar(Bipolar(base * top))
    }

    /// Perform max value blending.
    /// For bipolar comparison, we take the max of the absolute values of the two.
    fn max(base: Data, top: Data) -> Data {
        let base: Bipolar = base.into();
        let top: Bipolar = top.into();
        let out = if base.0.abs() < top.0.abs() { top } else { base };
        Data::Bipolar(out)
    }
}