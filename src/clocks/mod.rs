//! Namespace for creating clock node behaviors.
mod basic;
mod multiplier;
mod triggered;

pub use self::basic::Clock;
pub use self::multiplier::ClockMultiplier;
pub use self::triggered::TriggeredClock;

use clock_network::ClockNodePrototype;

#[cfg(test)]
mod test;

/// Return all of the clock prototypes.
pub fn create_prototypes() -> Box<[ClockNodePrototype]> {
    vec![Clock::create_prototype(),
         ClockMultiplier::create_prototype(),
         TriggeredClock::create_prototype(),].into_boxed_slice()
}