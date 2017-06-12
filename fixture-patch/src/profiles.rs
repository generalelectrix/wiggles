//! Precompiled DMX fixture types for general use.
//! Would be good to implement a domain-specific description of these things
//! so that they can be parsed and created dynamically, allowing for the
//! creation of a fixture editor.
use std::cmp::min;
use wiggles_value::{Data, Datatype, Unipolar, Bipolar, IntegerEnum};
use fixture::{DmxFixture, FixtureControl, DmxValue, RenderFunc};

// Helper functions for converting wiggles values into DMX.
fn as_single_channel(data: Data) -> DmxValue {
    // for single channel, interpret as unipolar and do the dumb thing
    let Unipolar(val) = data.into();
    let scaled = (val * 256.0) as usize;
    min(scaled, 255) as u8
}

fn render_dimmer(controls: &[FixtureControl], buffer: &mut [DmxValue]) {
    debug_assert!(controls.len() == 1);
    debug_assert!(buffer.len() == 1);
    let dmx_val = as_single_channel(controls[0].value());
    buffer[0] = dmx_val;
}
/// Basic 1-channel dimmer.
/// Controlled by a single unipolar.
pub fn dimmer() -> DmxFixture {
    let control = FixtureControl::new("level", Datatype::Unipolar, Data::Unipolar(Unipolar(0.0)));
    DmxFixture::new("dimmer", 1, vec!(control), render_dimmer)
}

/// Match a fixture type name to a render function.
/// Used during deserialization of saved states.
pub fn render_func_for_type(name: &str) -> Option<RenderFunc> {
    match name {
        "dimmer" => Some(render_dimmer),
        _ => None,
    }
}