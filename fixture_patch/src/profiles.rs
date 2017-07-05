//! Precompiled DMX fixture types for general use.
//! Would be good to implement a domain-specific description of these things
//! so that they can be parsed and created dynamically, allowing for the
//! creation of a fixture editor.
use std::cmp::min;
use std::collections::HashMap;
use wiggles_value::{Data, Datatype, Unipolar, Bipolar};
use fixture::{DmxFixture, FixtureControl, DmxValue, RenderFunc, DmxChannelCount};

// Helper functions for converting wiggles values into DMX.
/// Interpret as unipolar and map directly to dmx values.
fn as_single_channel(data: Data) -> DmxValue {
    unipolar_as_range(data, 0, 255)
}

/// Spread a unipolar value evenly across a DMX interval.
fn unipolar_as_range(data: Data, min_val: DmxValue, max_val: DmxValue) -> DmxValue {
    debug_assert!(max_val > min_val);
    let Unipolar(val) = data.into();
    let range_delta = max_val as usize - min_val as usize + 1;
    let scaled = min_val as usize + (val * range_delta as f64) as usize;
    min(scaled, max_val as usize) as u8
}

/// Spread a bipolar value evenly across a DMX interval.
fn bipolar_as_range(data: Data, min_val: DmxValue, max_val: DmxValue) -> DmxValue {
    // In this case, all we'd do is the same math as unipolar, so delegate to that
    // function.  This is here to make it more clear what's going on.
    unipolar_as_range(data, min_val, max_val)
}

/// Spread a bipolar value unevenly across two DMX intervals, with custom zero-point.
fn bipolar_as_unequal_range(
        data: Data, min_val: DmxValue, max_val: DmxValue, center: DmxValue) -> DmxValue {
    debug_assert!(max_val > center && center > min_val);

    let Bipolar(val) = data.into();
    if val < 0.0 {
        unipolar_as_range(Data::Unipolar(Unipolar(val + 1.0)), min_val, center - 1)
    }
    else {
        unipolar_as_range(Data::Unipolar(Unipolar(val)), center, max_val)
    }
}

mod test_helpers {
    use super::*;
    #[test]
    fn test_unipolar_as_range() {
        fn check_full_range(expected: DmxValue, unipolar: f64) {
            assert_eq!(expected, as_single_channel(Data::Unipolar(Unipolar(unipolar))));
        }
        check_full_range(0, 0.0);
        // make sure the bottom of the range has a full bin
        check_full_range(0, 0.0038);
        check_full_range(1, 0.004);
        check_full_range(255, 1.0);
        // make sure the top of the range has a full bin
        check_full_range(255, 0.997);
        check_full_range(254, 0.996);
    }

    #[test]
    fn test_bipolar_as_range() {
        fn check_full_range(expected: DmxValue, bipolar: f64) {
            assert_eq!(expected, bipolar_as_range(Data::Bipolar(Bipolar(bipolar)), 0, 255));
        }
        check_full_range(0, -1.0);
        check_full_range(255, 1.0);
        check_full_range(128, 0.0);

        fn check_half_range(expected: DmxValue, bipolar: f64) {
            assert_eq!(expected, bipolar_as_range(Data::Bipolar(Bipolar(bipolar)), 128, 255));
        }
        check_half_range(128, -1.0);
        check_half_range(192, 0.0);
        check_half_range(255, 1.0);
    }

    #[test]
    fn test_bipolar_as_unequal_range() {
        // test using full range, bottom 25% vs top 75%.
        fn check(expected: DmxValue, bipolar: f64) {
            assert_eq!(
                expected,
                bipolar_as_unequal_range(Data::Bipolar(Bipolar(bipolar)), 0, 255, 64));
        }

        check(0, -1.0);
        check(63, -0.0001);
        check(64, 0.0);
        check(255, 1.0);
    }
}

type ControlsCreator = fn() -> Vec<FixtureControl>;

/// Roll up all the data needed to instantiate a fixture of a particular type.
pub struct Profile {
    name: &'static str,
    description: &'static str,
    channel_count: DmxChannelCount,
    controls: ControlsCreator,
    render_func: RenderFunc,
}

impl Profile {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn description(&self) -> &'static str {
        self.description
    }

    pub fn channel_count(&self) -> DmxChannelCount {
        self.channel_count
    }

    pub fn create_fixture(&self) -> DmxFixture {
        DmxFixture::new(self.name.clone(), self.channel_count, (self.controls)(), self.render_func)
    }
}

type ProfileMap = HashMap<&'static str, Profile>;

// Define all of the available profiles here.
lazy_static! {
    /// Runtime lookup for every available fixture profile.
    pub static ref PROFILES: ProfileMap = {
        let mut m = HashMap::new();
        {
            let mut add = |profile: Profile| {
                if m.contains_key(profile.name) {
                    panic!("Duplicate declaration of profile {}", profile.name);
                }
                m.insert(profile.name, profile);
            };
            add(dimmer::PROFILE);
            add(apollo_smart_move_dmx::PROFILE);
            add(apollo_roto_q_dmx::PROFILE);
            add(clay_paky_astroraggi_power::PROFILE);
            add(clay_paky_atlas::PROFILE);
        }
        m
    };
}

/// Match a fixture profile name to a RenderFunc.
/// Used during deserialization of saved states.
pub fn render_func_for_type(name: &str) -> Option<RenderFunc> {
    PROFILES.get(name).map(|profile| profile.render_func)
}

// declare profiles in individual modules

/// Basic 1-channel dimmer.
pub mod dimmer {
    use super::*;

    const CHANNEL_COUNT: DmxChannelCount = 1;

    /// Basic 1-channel dimmer.
    /// Controlled by a single unipolar.
    pub const PROFILE: Profile = Profile {
        name: "dimmer",
        description: "1-channel linear dimmer.",
        channel_count: CHANNEL_COUNT,
        controls: controls,
        render_func: render,
    };

    fn controls() -> Vec<FixtureControl> {
        vec!(FixtureControl::new("level", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0))))
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 1);
        debug_assert!(buffer.len() == CHANNEL_COUNT as usize);
        let dmx_val = as_single_channel(controls[0].value());
        buffer[0] = dmx_val;
    }
}

/// Astroraggi Power, eh!?
pub mod clay_paky_astroraggi_power {
    use super::*;

    const CHANNEL_COUNT: DmxChannelCount = 2;

    /// Clay Paky Astroraggi Power.
    /// No dome indexing.
    /// Breaks out shutter and strobe separately, nonzero strobe takes priority over shutter.
    pub const PROFILE: Profile = Profile {
        name: "clay paky:Astroraggi Power",
        description: "The ORIGINAL moonflower.",
        channel_count: CHANNEL_COUNT,
        controls: controls,
        render_func: render,
    };

    fn controls() -> Vec<FixtureControl> {
        vec!(
            FixtureControl::new("shutter", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0))),
            FixtureControl::new("strobe", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0))),
            FixtureControl::new("rotation", Datatype::Bipolar, Data::Bipolar(Bipolar(0.0))),
        )
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 3);
        debug_assert!(buffer.len() == CHANNEL_COUNT as usize);
        let shutter_channel_val = {
            let Unipolar(strobe_rate) = controls[1].value().into();
            if strobe_rate > 0.01 {
                // strobe is active
                // has a slight detent to account for crappy midi faders not being down all the way
                unipolar_as_range(controls[1].value(), 140, 242)
            }
            else {
                // no strobe, regular shutter
                unipolar_as_range(controls[0].value(), 0, 127)
            }
        };
        // channel 0 - rotation
        buffer[0] = bipolar_as_range(controls[2].value(), 128, 255);
        // channel 1 - shutter/strobe
        buffer[1] = shutter_channel_val;
    }
}

/// Atlas - the megaest fan light of them all
pub mod clay_paky_atlas {
    use super::*;

    const CHANNEL_COUNT: DmxChannelCount = 1;

    /// Clay Paky Atlas.
    /// Breaks out shutter and strobe separately, nonzero strobe takes priority over shutter.
    /// The apertures control is all closed at -1, all open at 0.0, and all closed again at +1,
    /// allowing both directions of fanning action.
    pub const PROFILE: Profile = Profile {
        name: "clay paky:Atlas",
        description: "The megaest fan light of them all.",
        channel_count: CHANNEL_COUNT,
        controls: controls,
        render_func: render,
    };

    fn controls() -> Vec<FixtureControl> {
        vec!(
            FixtureControl::new("apertures", Datatype::Bipolar, Data::Bipolar(Bipolar(-1.0))),
            FixtureControl::new("strobe", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0))),
        )
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 2);
        debug_assert!(buffer.len() == CHANNEL_COUNT as usize);
        // channel 0 - shutter/strobe
        buffer[0] = {
            let Unipolar(strobe_rate) = controls[1].value().into();
            if strobe_rate > 0.01 {
                // strobe is active
                // has a slight detent to account for crappy midi faders not being down all the way
                unipolar_as_range(controls[1].value(), 140, 242)
            }
            else {
                // no strobe, do aperture fanning
                bipolar_as_unequal_range(controls[0].value(), 0, 139, 64)
            }
        };
    }
}

/// Apollo Roto-Q DMX.
/// Only provides access to rotating mode.
pub mod apollo_roto_q_dmx {
    use super::*;

    const CHANNEL_COUNT: DmxChannelCount = 2;

    /// Apollo Roto-Q DMX, controlled as a single bipolar.
    pub const PROFILE: Profile = Profile {
        name: "apollo:Roto-Q DMX",
        description: "Not yet implemented. Apollo Roto-Q DMX, rotating mode only.",
        channel_count: CHANNEL_COUNT,
        controls: controls,
        render_func: render,
    };

    fn controls() -> Vec<FixtureControl> {
        vec!(FixtureControl::new("rotation", Datatype::Bipolar, Data::Bipolar(Bipolar(0.0))))
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 1);
        debug_assert!(buffer.len() == CHANNEL_COUNT as usize);
        let dmx_val = 0; // FIXME: need to map exact dmx range for smart moves
        buffer[0] = dmx_val;
        buffer[1] = 0;
    }
}

/// Apollo smart move DMX.
/// Only provides access to rotating mode.
pub mod apollo_smart_move_dmx {
    use super::*;

    const CHANNEL_COUNT: DmxChannelCount = 3;

    /// Apollo smart move DMX, controlled as a single bipolar.
    pub const PROFILE: Profile = Profile {
        name: "apollo:Smart Move DMX",
        description: "Not yet implemented. Apollo Smart Move DMX, rotating mode only.",
        channel_count: CHANNEL_COUNT,
        controls: controls,
        render_func: render,
    };

    fn controls() -> Vec<FixtureControl> {
        vec!(FixtureControl::new("rotation", Datatype::Bipolar, Data::Bipolar(Bipolar(0.0))))
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 1);
        debug_assert!(buffer.len() == CHANNEL_COUNT as usize);
        let dmx_val = 0; // FIXME: need to map exact dmx range for smart moves
        buffer[0] = dmx_val;
        buffer[1] = 0;
        buffer[2] = 0;
    }
}

