//! A wiggles node that agglomerates multiple inputs and blends them together using a selected
//! blend mode.
use std::sync::Arc;
use std::time::Duration;
use std::ops::{Add, Mul};
use std::cmp::max;
use std::fmt;
use console_server::reactor::Messages;
use ::network::{OutputId, Inputs, Outputs};
use wiggles_value::knob::{
    Knobs,
    Datatype as KnobDatatype,
    Data as KnobData,
    KnobDescription,
    Error as KnobError,
    badaddr,
    badtype,
    Response as KnobResponse,
};
use serde_json::{Error as SerdeJsonError, self};
use clocks::clock::{ClockId, ClockProvider, ClockValue};
use super::wiggle::{Wiggle, CompleteWiggle, WiggleId, KnobAddr, WiggleProvider};
use wiggles_value::{Unipolar, Bipolar, Datatype, Data};
use wiggles_value::blend::Blend;
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
impl Inputs<KnobResponse<KnobAddr>> for Blender {
    fn default_input_count(&self) -> u32 {
        1
    }
    fn try_push_input(&mut self) -> Result<Messages<KnobResponse<KnobAddr>>, ()> {
        // add a fresh input, set to 1
        self.levels.push(Unipolar(1.0));
        // tell the world that there's a new knob available
        // level addresses start at 1
        let addr = self.levels.len() as KnobAddr;
        Ok(Messages::one(KnobResponse::Added {
            addr: addr,
            desc: level_knob_desc(addr),
        }))
    }
    fn try_pop_input(&mut self) -> Result<Messages<KnobResponse<KnobAddr>>, ()> {
        // No use having a mixer with no inputs (perhaps we may want to relax this restriction.)
        if self.levels.len() == 1 {
            return Err(());
        }
        self.levels.pop();
        Ok(Messages::one(KnobResponse::Removed((self.levels.len() + 1) as KnobAddr)))
    }
}

// Blender has a single, fixed output.
impl<M> Outputs<M> for Blender {}

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

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Blender is stateless, update does nothing.
    fn update(&mut self, _: Duration) -> Messages<KnobResponse<KnobAddr>> {
        Messages::none()
    }

    /// Multiply every input by its level and then blend them all.
    fn render(
        &self,
        phase_offset: f64,
        type_hint: Option<Datatype>,
        inputs: &[Option<(WiggleId, OutputId)>],
        _: OutputId,
        wiggles: &WiggleProvider,
        clocks: &ClockProvider)
        -> Data
    {
        let blender = match type_hint {
            Some(Datatype::Unipolar) | None => {
                match self.blend_mode {
                    BlendMode::Add => <Unipolar as Blend>::add,
                    BlendMode::Multiply => <Unipolar as Blend>::mult,
                    BlendMode::Max => <Unipolar as Blend>::max,
                }
            }
            Some(Datatype::Bipolar) => {
                match self.blend_mode {
                    BlendMode::Add => <Bipolar as Blend>::add,
                    BlendMode::Multiply => <Bipolar as Blend>::mult,
                    BlendMode::Max => <Bipolar as Blend>::max,
                }
            }
        };
        let base_layer = match self.blend_mode {
            BlendMode::Add => Data::unipolar(0.0),
            BlendMode::Multiply => Data::unipolar(1.0),
            BlendMode::Max => Data::unipolar(0.0),
        };

        // log an error if we didn't get the right number of inputs, but don't panic
        if inputs.len() != self.levels.len() {
            error!(
                "Blender {} has {} level controls but received {} inputs.",
                self.name,
                self.levels.len(),
                inputs.len());
        }
        // Use the selected blend function to fold over the inputs.
        inputs.iter()
            .zip(self.levels.iter())
            .map(|(input_id_opt, level)| {
                let input_val = match *input_id_opt {
                    Some((id, output)) => wiggles.get_value(id, output, phase_offset, type_hint, clocks),
                    None => Data::default_with_type_hint(type_hint),
                };
                // scale the input value by its level
                input_val * (*level)
            })
            .fold(base_layer, blender)
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



