//! A wiggles node that agglomerates multiple inputs and blends them together using a selected
//! blend mode.
use std::sync::Arc;
use std::time::Duration;
use std::ops::{Add, Mul};
use std::cmp::max;
use std::fmt;
use console_server::reactor::Messages;
use ::network::Inputs;
use wiggles_value::knob::{
    Knobs,
    Datatype as KnobDatatype,
    Data as KnobData,
    KnobDescription,
    Error as KnobError,
    badaddr,
    badtype,
    Message as KnobMessage,
};
use serde_json::{Error as SerdeJsonError, self};
use clocks::clock::{ClockId, ClockProvider, ClockValue};
use super::wiggle::{Wiggle, CompleteWiggle, WiggleId, KnobAddr, WiggleProvider};
use wiggles_value::{Unipolar, Bipolar, Datatype, Data};
use waveforms::sine;

lazy_static! {
    static ref BLEND_MODES: Vec<String> = vec!(
        "add".to_string(), "mult".to_string(), "max".to_string());
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
enum BlendMode {
    Add,
    Multiply,
    Max,
}

impl BlendMode {
    fn to_picker(&self) -> &'static str {
        match *self {
            BlendMode::Add => "add",
            BlendMode::Multiply => "mult",
            BlendMode::Max => "max",
        }
    }

    fn from_picker(s: &str) -> Result<Self, ()> {
        match s {
            "add" => Ok(BlendMode::Add),
            "mult" => Ok(BlendMode::Multiply),
            "max" => Ok(BlendMode::Max),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blender {
    name: String,
    /// Channel fader levels, each controlled by a knob.
    /// Knob addresses start from 1, as blend mode is 0.
    levels: Vec<Unipolar>,
    /// What blend mode is currently active?
    /// This is controlled by knob 0.
    blend_mode: BlendMode,
}

impl Blender {
    pub fn new<N: Into<String>>(name: N) -> Self {
        Blender {
            name: name.into(),
            // start with one channel all the way up
            levels: vec!(Unipolar(1.0)),
            // run in Add mode by default
            blend_mode: BlendMode::Add,
        }
    }
}

pub const CLASS: &'static str = "blender";

fn level_knob_desc(chan: KnobAddr) -> KnobDescription {
    KnobDescription {
        name: Arc::new(format!("channel {} level", chan)),
        datatype: KnobDatatype::Wiggle(Datatype::Unipolar),
    }
}

// Blender has at least one input, and up to an unlimited number of them.
impl Inputs<Messages<KnobMessage<KnobAddr>>> for Blender {
    fn default_input_count(&self) -> u32 {
        1
    }
    fn try_push_input(&mut self) -> Result<Messages<KnobMessage<KnobAddr>>, ()> {
        // add a fresh input, set to 1
        self.levels.push(Unipolar(1.0));
        // tell the world that there's a new knob available
        // level addresses start at 1
        let addr = self.levels.len() as KnobAddr;
        Ok(Messages::one(KnobMessage::KnobAdded {
            addr: addr,
            desc: level_knob_desc(addr),
        }))
    }
    fn try_pop_input(&mut self) -> Result<Messages<KnobMessage<KnobAddr>>, ()> {
        // No use having a mixer with no inputs (perhaps we may want to relax this restriction.)
        if self.levels.len() == 1 {
            return Err(());
        }
        self.levels.pop();
        Ok(Messages::one(KnobMessage::KnobRemoved((self.levels.len() + 1) as KnobAddr)))
    }
}

const BLEND_MODE_KNOB_ADDR: KnobAddr = 0;

fn blend_knob_datatype() -> KnobDatatype {
    KnobDatatype::Picker(BLEND_MODES.clone())
}

// Need to clone the picker types to send them off into the world.
fn blend_knob_desc() -> KnobDescription {
    KnobDescription {
        name: Arc::new("blend mode".to_string()),
        datatype: blend_knob_datatype(),
    }
}

impl Knobs<KnobAddr> for Blender {
    fn knobs(&self) -> Vec<(KnobAddr, KnobDescription)> {
        let mut descs = vec!((BLEND_MODE_KNOB_ADDR, blend_knob_desc()));
        for chan in 1..self.levels.len()+1 {
            descs.push((chan as KnobAddr, level_knob_desc(chan as KnobAddr)));
        }
        descs
    }

    fn knob_datatype(&self, addr: KnobAddr) -> Result<KnobDatatype, KnobError<KnobAddr>> {
        if addr == BLEND_MODE_KNOB_ADDR {
            Ok(KnobDatatype::Picker(BLEND_MODES.clone()))
        }
        else if addr <= (self.levels.len() as KnobAddr) {
            Ok(KnobDatatype::Wiggle(Datatype::Unipolar))
        }
        else {
            Err(badaddr(addr))
        }
    }

    /// Return this knob's current data payload or an error if it doesn't exist.
    fn knob_value(&self, addr: KnobAddr) -> Result<KnobData, KnobError<KnobAddr>> {
        if addr == BLEND_MODE_KNOB_ADDR {
            return Ok(KnobData::Picker(self.blend_mode.to_picker().to_string()));
        }
        match self.levels.get((addr - 1) as usize) {
            Some(level) => Ok(KnobData::Wiggle(Data::Unipolar(*level))),
            None => Err(badaddr(addr)),
        }
    }

    fn set_knob(&mut self, addr: KnobAddr, value: KnobData) -> Result<(), KnobError<KnobAddr>> {
        if addr == BLEND_MODE_KNOB_ADDR {
            self.blend_mode =
                value.as_picker()
                    .and_then(|p| BlendMode::from_picker(&p))
                    .map_err(|()| badtype(blend_knob_datatype(), value))?;
            return Ok(());
        }
        // not the blend knob, should be a level knob
        match self.levels.get_mut((addr - 1) as usize) {
            Some(level) => {
                 *level = value.as_unipolar()?;
                 Ok(())
            }
            None => Err(badaddr(addr)),
        }
    }
}

impl Wiggle for Blender {
    fn class(&self) -> &'static str {
        CLASS
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// Blender is stateless, update does nothing.
    fn update(&mut self, dt: Duration) -> Messages<KnobMessage<KnobAddr>> {
        Messages::none()
    }

    /// Multiply every input by its level and then blend them all.
    fn render(
        &self,
        phase_offset: Unipolar,
        type_hint: Option<Datatype>,
        inputs: &[Option<WiggleId>],
        wiggles: &WiggleProvider,
        _: &ClockProvider)
        -> Data
    {
        // this could probably be refactored to avoid the repetition, but for now just grind out
        // all of the possible combinations.
    }

    fn clock_source(&self) -> Result<Option<ClockId>, ()> {
        Err(())
    }

    fn set_clock(&mut self, _: Option<ClockId>) -> Result<(), ()> {
        Err(())
    }

    fn as_json(&self) -> Result<String, SerdeJsonError> {
        serde_json::to_string(self)
    }
}



