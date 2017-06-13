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
    pub name: &'static str,
    pub channel_count: DmxChannelCount,
    controls: ControlsCreator,
    render_func: RenderFunc,
}

impl Profile {
    pub fn create_fixture(&self) -> DmxFixture {
        DmxFixture::new(self.name.clone(), self.channel_count, (self.controls)(), self.render_func)
    }
}

// declare profiles in individual modules

mod dimmer {
    use super::*;

    fn controls() -> Vec<FixtureControl> {
        vec!(FixtureControl::new("level", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0))))
    }

    fn render(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
        debug_assert!(controls.len() == 1);
        debug_assert!(buffer.len() == 1);
        let dmx_val = as_single_channel(controls[0].value());
        buffer[0] = dmx_val;
    }

    /// Basic 1-channel dimmer.
    /// Controlled by a single unipolar.
    pub const PROFILE: Profile = Profile {
        name: "dimmer",
        channel_count: 1,
        controls: controls,
        render_func: render,
    };
}

type ProfileMap = HashMap<&'static str, Profile>;

// Define all of the available profiles here.
lazy_static! {
    pub static ref PROFILES: ProfileMap = {
        let mut m = HashMap::new();
        {
            let mut add = |profile: Profile| m.insert(profile.name, profile);
            add(dimmer::PROFILE);
        }
        m
    };
}

/// Match a fixture profile name to a RenderFunc.
/// Used during deserialization of saved states.
pub fn render_func_for_type(name: &str) -> Option<RenderFunc> {
    PROFILES.get(name).map(|profile| profile.render_func)
}

