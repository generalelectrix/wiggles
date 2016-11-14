//! Trait for the state update mechanism.

/// Floating-point duration, in units of seconds.
pub struct DeltaT(pub f64);

pub trait Update {
    fn update(&mut self, delta_t: DeltaT);
}