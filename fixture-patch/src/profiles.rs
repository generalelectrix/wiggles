//! Precompiled DMX fixture types for general use.
//! Would be good to implement a domain-specific description of these things
//! so that they can be parsed and created dynamically, allowing for the
//! creation of a fixture editor.
use std::cmp::min;
use std::collections::HashMap;
use wiggles_value::{Data, Datatype, Unipolar, Bipolar, IntegerEnum};
use fixture::{DmxFixture, FixtureControl, DmxValue, RenderFunc, DmxChannelCount};

// Helper functions for converting wiggles values into DMX.
fn as_single_channel(data: Data) -> DmxValue {
    // for single channel, interpret as unipolar and do the dumb thing
    let Unipolar(val) = data.into();
    let scaled = (val * 256.0) as usize;
    min(scaled, 255) as u8
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

