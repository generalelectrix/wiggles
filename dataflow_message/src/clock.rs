//! Message-passing API on top of a clock dataflow network.
use std::sync::Arc;
use std::fmt;
use std::error;
use console_server::reactor::Messages;
use console_server::clients::ResponseFilter;
use wiggles_value::knob::{KnobDescription, Message as KnobMessage, Knobs};
use dataflow::network::{InputId, NetworkError};
use dataflow::clocks::{
    ClockNetwork,
    ClockId,
    KnobAddr as ClockNodeKnobAddr,
    ClockKnobAddr,
    new_clock};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetInput {
    clock: ClockId,
    input: InputId,
    target: Option<ClockId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Create{class: String, name: String},
    Remove{id: ClockId, force: bool},
    SetInput(SetInput),
    PushInput(ClockId, Option<ClockId>),
    PopInput(ClockId),
}

#[derive(Debug, Serialize)]
pub struct ClockDescription {
    name: Arc<String>,
    class: Arc<String>,
    inputs: Vec<Option<ClockId>>,
}

#[derive(Debug, Serialize)]
/// Response messages related to clock actions.
pub enum Response {
    New{id: ClockId, desc: ClockDescription},
    Removed(ClockId),
    SetInput(SetInput),
    PushInput(ClockId, Option<ClockId>),
    PopInput(ClockId),
}

#[derive(Debug)]
/// Outer response wrapper; the controlling system should strip this off and list all knob
/// messages into the global knob type for the application at hand.
/// Don't implement serialize or deserialize to ensure that the client application dissects this
/// message into its constituent pieces.
pub enum ResponseWithKnobs {
    Clock(Response),
    Knob(KnobMessage<ClockKnobAddr>),
}

/// Apply the action dictated by a clock command to this clock network.
pub fn handle_message(
    network: &mut ClockNetwork,
    command: Command)
    -> Result<(Messages<ResponseWithKnobs>, Option<ResponseFilter>), Error>
{
    use self::Command::*;
    match command {
        Create{class, name} => {
            let node = new_clock(&class, name).ok_or(Error::UnknownClass(class.clone()))?;
            let (id, node) = network.add(node);
            let clock = node.inner();
            // emit messages for all of the new knobs
            let mut messages = Messages::none();
            for (addr, desc) in clock.knobs() {
                // Lift the knob addr up into the network domain.
                let addr = (id, addr);
                let msg = ResponseWithKnobs::Knob(KnobMessage::Added{
                    addr: addr,
                    desc: desc,
                });
                messages.push(msg);
            }

            // emit a message for the new clock we just added
            let desc = ClockDescription {
                name: Arc::new(clock.name().to_string()),
                class: Arc::new(class),
                inputs: node.inputs().to_vec(),
            };
            messages.push(ResponseWithKnobs::Clock(Response::New{
                id: id,
                desc: desc
            }));
            Ok((messages, Some(ResponseFilter::All)))
        }
        Remove{id, force} => {
            // remove the node itself
            let clock = network.remove(id, force)?;
            // emit messages indicating the removal of its knobs
            let mut messages = Messages::none();
            for (addr, _) in clock.knobs() {
                messages.push(ResponseWithKnobs::Knob(KnobMessage::Removed((id, addr))));
            }
            // emit a message indicating remove of the clock itself
            messages.push(ResponseWithKnobs::Clock(Response::Removed(id)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        SetInput(set) => {
            network.swap_input(set.clock, set.input, set.target)?;
            let msg = ResponseWithKnobs::Clock(Response::SetInput(set));
            Ok((Messages::one(msg), Some(ResponseFilter::All)))
        }
        PushInput(id, target) => {
            let (_, mut knob_messages) = network.push_input(id, target)?;
            let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
            messages.push(ResponseWithKnobs::Clock(Response::PushInput(id, target)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        PopInput(id) => {
            let mut knob_messages = network.pop_input(id)?;
            let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
            messages.push(ResponseWithKnobs::Clock(Response::PopInput(id)));
            Ok((messages, Some(ResponseFilter::All)))
        }
    }

}

#[derive(Debug)]
pub enum Error {
    UnknownClass(String),
    Network(NetworkError<ClockId>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnknownClass(ref class) => write!(f, "Unknown clock class: '{}'.", class),
            Network(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnknownClass(_) => "Unknown clock class.",
            Network(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            UnknownClass(_) => None,
            Network(ref e) => Some(e),
        }
    }
}

impl From<NetworkError<ClockId>> for Error {
    fn from(e: NetworkError<ClockId>) -> Self {
        Error::Network(e)
    }
}