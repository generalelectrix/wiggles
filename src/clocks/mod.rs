//! Namespace for creating clock node behaviors.
mod basic;
mod multiplier;
mod triggered;

use self::basic::Clock;
use self::multiplier::ClockMultiplier;

use clock_network::ClockNodePrototype;

/// Return all of the clock prototypes.
pub fn create_prototypes() -> Box<[ClockNodePrototype]> {
    vec![Clock::create_prototype(),
         ClockMultiplier::create_prototype(),].into_boxed_slice()
}