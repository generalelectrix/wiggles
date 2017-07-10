//! Message-passing API on top of a wiggle dataflow network.
use std::sync::Arc;
use std::fmt;
use std::error;
use console_server::reactor::Messages;
use console_server::clients::ResponseFilter;
use wiggles_value::knob::{KnobDescription, Response as KnobResponse, Knobs};
use dataflow::network::{InputId, NetworkError};
use dataflow::wiggles::{
    WiggleNetwork,
    WiggleId,
    KnobAddr as WiggleNodeKnobAddr,
    WiggleKnobAddr,
    new_wiggle,
    WIGGLES};
use dataflow::clocks::{ClockId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInput {
    wiggle: WiggleId,
    input: InputId,
    target: Option<WiggleId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Classes,
    Create{class: String, name: String},
    Remove{id: WiggleId, force: bool},
    SetInput(SetInput),
    PushInput(WiggleId, Option<WiggleId>),
    PopInput(WiggleId),
    SetClock(WiggleId, Option<ClockId>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsesClock {
    Yes(Option<ClockId>),
    No,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiggleDescription {
    name: Arc<String>,
    class: Arc<String>,
    inputs: Vec<Option<WiggleId>>,
    clock: UsesClock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response messages related to wiggle actions.
pub enum Response {
    Classes(Arc<Vec<String>>),
    New{id: WiggleId, desc: WiggleDescription},
    Removed(WiggleId),
    SetInput(SetInput),
    PushInput(WiggleId, Option<WiggleId>),
    PopInput(WiggleId),
    SetClock(WiggleId, Option<ClockId>),
}

#[derive(Debug)]
/// Outer response wrapper; the controlling system should strip this off and list all knob
/// messages into the global knob type for the application at hand.
/// Don't implement serialize or deserialize to ensure that the client application dissects this
/// message into its constituent pieces.
pub enum ResponseWithKnobs {
    Wiggle(Response),
    Knob(KnobResponse<WiggleKnobAddr>),
}

lazy_static! {
    static ref CLASSES: Arc<Vec<String>> = Arc::new(WIGGLES.iter().map(|s| s.to_string()).collect());
}

/// Apply the action dictated by a wiggle command to this wiggle network.
pub fn handle_message(
    network: &mut WiggleNetwork,
    command: Command)
    -> Result<(Messages<ResponseWithKnobs>, Option<ResponseFilter>), Error>
{
    use self::Command::*;
    match command {
        Classes => Ok((
            Messages::one(ResponseWithKnobs::Wiggle(Response::Classes(CLASSES.clone()))),
            None)),
        Create{class, name} => {
            let node = new_wiggle(&class, name).ok_or(Error::UnknownClass(class.clone()))?;
            let (id, node) = network.add(node);
            let wiggle = node.inner();
            // emit messages for all of the new knobs
            let mut messages = Messages::none();
            for (addr, desc) in wiggle.knobs() {
                // Lift the knob addr up into the network domain.
                let addr = (id, addr);
                let msg = ResponseWithKnobs::Knob(KnobResponse::Added{
                    addr: addr,
                    desc: desc,
                });
                messages.push(msg);
            }

            let clock_spec = match wiggle.clock_source() {
                Ok(source) => UsesClock::Yes(source),
                Err(_) => UsesClock::No,
            };

            // emit a message for the new wiggle we just added
            let desc = WiggleDescription {
                name: Arc::new(wiggle.name().to_string()),
                class: Arc::new(class),
                inputs: node.inputs().to_vec(),
                clock: clock_spec,
            };
            messages.push(ResponseWithKnobs::Wiggle(Response::New{
                id: id,
                desc: desc,
            }));
            Ok((messages, Some(ResponseFilter::All)))
        }
        Remove{id, force} => {
            // remove the node itself
            let wiggle = network.remove(id, force)?;
            // emit messages indicating the removal of its knobs
            let mut messages = Messages::none();
            for (addr, _) in wiggle.knobs() {
                messages.push(ResponseWithKnobs::Knob(KnobResponse::Removed((id, addr))));
            }
            // emit a message indicating remove of the wiggle itself
            messages.push(ResponseWithKnobs::Wiggle(Response::Removed(id)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        SetInput(set) => {
            network.swap_input(set.wiggle, set.input, set.target)?;
            let msg = ResponseWithKnobs::Wiggle(Response::SetInput(set));
            Ok((Messages::one(msg), Some(ResponseFilter::All)))
        }
        PushInput(id, target) => {
            let (_, mut knob_messages) = network.push_input(id, target)?;
            let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
            messages.push(ResponseWithKnobs::Wiggle(Response::PushInput(id, target)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        PopInput(id) => {
            let mut knob_messages = network.pop_input(id)?;
            let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
            messages.push(ResponseWithKnobs::Wiggle(Response::PopInput(id)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        SetClock(id, target) => {
            match network.node_inner_mut(id)?.set_clock(target) {
                Ok(_) => {
                    let msg = ResponseWithKnobs::Wiggle(Response::SetClock(id, target));
                    Ok((Messages::one(msg), Some(ResponseFilter::All)))
                }
                Err(_) => {
                    Err(Error::NoClock(id))
                }
            }
        }
    }

}

#[derive(Debug)]
pub enum Error {
    UnknownClass(String),
    NoClock(WiggleId),
    Network(NetworkError<WiggleId>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnknownClass(ref class) => write!(f, "Unknown wiggle class: '{}'.", class),
            NoClock(ref id) => write!(f, "Wiggle {} does not use a clock input.", id),
            Network(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnknownClass(_) => "Unknown wiggle class.",
            NoClock(_) => "This wiggle does not use a clock input.",
            Network(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            UnknownClass(_) => None,
            NoClock(_) => None,
            Network(ref e) => Some(e),
        }
    }
}

impl From<NetworkError<WiggleId>> for Error {
    fn from(e: NetworkError<WiggleId>) -> Self {
        Error::Network(e)
    }
}