//! Trait for the state update mechanism.
use event::Events;

/// Floating-point duration, in units of seconds.
pub type DeltaT = f64;

pub trait Update {
    /// Updating something may optionally return events.
    fn update(&mut self, delta_t: DeltaT) -> Events;
}