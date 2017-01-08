//! Trait for the state update mechanism.
use event::Event;

/// Floating-point duration, in units of seconds.
pub type DeltaT = f64;

pub trait Update {
    /// Updating something may optionally return an event.
    fn update(&mut self, delta_t: DeltaT) -> Option<Event>;
}