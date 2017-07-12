//! Message-passing API on top of a clock dataflow network.
use std::sync::Arc;
use std::fmt;
use std::error;
use console_server::reactor::Messages;
use console_server::clients::ResponseFilter;
use wiggles_value::knob::{KnobDescription, Response as KnobResponse, Knobs};
use dataflow::network::{InputId, NetworkError, OutputId, Node};
use dataflow::clocks::{
    Clock,
    CompleteClock,
    ClockNetwork,
    ClockId,
    KnobAddr as ClockNodeKnobAddr,
    ClockKnobAddr,
    new_clock,
    CLOCKS,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInput {
    clock: ClockId,
    input: InputId,
    target: Option<ClockId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Get a listing of every available type of clock.
    Classes,
    /// Get a summary of the state of every clock.  Used to initialize new clients.
    State,
    /// Create a new clock.
    Create{kind: String, name: String},
    /// Delete an existing clock.
    Remove{id: ClockId, force: bool},
    /// Rename a clock.
    Rename(ClockId, String),
    /// Assign the input of a clock.
    SetInput(SetInput),
    /// Add a new input to a clock.
    PushInput(ClockId),
    /// Remove an input from a clock.
    PopInput(ClockId),
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ClockDescription {
    name: Arc<String>,
    kind: Arc<String>,
    inputs: Vec<Option<ClockId>>,
}

impl ClockDescription {
    fn from_node(node: &Node<Box<CompleteClock>, ClockId, KnobResponse<ClockKnobAddr>>) -> Self {
        let inputs = node.inputs().iter().map(|o| o.map(|(i, _)| i)).collect();
        let clock = node.inner();
        ClockDescription {
            name: Arc::new(clock.name().to_string()),
            kind: Arc::new(clock.class().to_string()),
            inputs: inputs,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
/// Response messages related to clock actions.
pub enum Response {
    /// A listing of every available type of clock.
    Classes(Arc<Vec<String>>),
    /// A summary of the state of every clock.
    State(Vec<(ClockId, ClockDescription)>),
    /// A new clock has been added.
    New(ClockId, ClockDescription),
    /// A clock has been deleted.
    Removed(ClockId),
    /// A clock has been renamed.
    Renamed(ClockId, Arc<String>),
    /// A clock's input has been reassigned.
    SetInput(SetInput),
    /// A clock has had a new input added.
    PushInput(ClockId),
    /// A clock has had an input removed.
    PopInput(ClockId),
}

#[derive(Debug)]
/// Outer response wrapper; the controlling system should strip this off and list all knob
/// messages into the global knob type for the application at hand.
/// Don't implement serialize or deserialize to ensure that the client application dissects this
/// message into its constituent pieces.
pub enum ResponseWithKnobs {
    Clock(Response),
    Knob(KnobResponse<ClockKnobAddr>),
}

lazy_static! {
    static ref CLASSES: Arc<Vec<String>> = Arc::new(CLOCKS.iter().map(|s| s.to_string()).collect());
}

const o0: OutputId = OutputId(0);

/// Apply the action dictated by a clock command to this clock network.
pub fn handle_message(
    network: &mut ClockNetwork,
    command: Command)
    -> Result<(Messages<ResponseWithKnobs>, Option<ResponseFilter>), Error>
{
    use self::Command::*;
    match command {
        State => {
            let state = network.nodes()
                .map(|(clock_id, node)| (clock_id, ClockDescription::from_node(node)))
                .collect();
            Ok((Messages::one(ResponseWithKnobs::Clock(Response::State(state))), None))
        }
        Classes => Ok((
            Messages::one(ResponseWithKnobs::Clock(Response::Classes(CLASSES.clone()))),
            None)),
        Create{kind, name} => {
            let node = new_clock(&kind, name).ok_or(Error::UnknownClass(kind.clone()))?;
            let (id, node) = network.add(node);
            let clock = node.inner();
            // emit messages for all of the new knobs
            let mut messages = Messages::none();
            for (addr, desc) in clock.knobs() {
                // Lift the knob addr up into the network domain.
                let addr = (id, addr);
                let msg = ResponseWithKnobs::Knob(KnobResponse::Added{
                    addr: addr,
                    desc: desc,
                });
                messages.push(msg);
            }

            // emit a message for the new clock we just added
            messages.push(ResponseWithKnobs::Clock(Response::New(
                id, ClockDescription::from_node(node))));
            Ok((messages, Some(ResponseFilter::All)))
        }
        Remove{id, force} => {
            // remove the node itself
            let clock = network.remove(id, force)?;
            // emit messages indicating the removal of its knobs
            let mut messages = Messages::none();
            for (addr, _) in clock.knobs() {
                messages.push(ResponseWithKnobs::Knob(KnobResponse::Removed((id, addr))));
            }
            // emit a message indicating remove of the clock itself
            messages.push(ResponseWithKnobs::Clock(Response::Removed(id)));
            Ok((messages, Some(ResponseFilter::All)))
        }
        Rename(clock, name) => {
            network.node_inner_mut(clock)?.set_name(name.clone());
            Ok((
                Messages::one(ResponseWithKnobs::Clock(Response::Renamed(clock, Arc::new(name)))),
                Some(ResponseFilter::All)))
        }
        SetInput(set) => {
            let target = set.target.map(|t| (t, o0));
            network.swap_input(set.clock, set.input, target)?;
            let msg = ResponseWithKnobs::Clock(Response::SetInput(set));
            Ok((Messages::one(msg), Some(ResponseFilter::All)))
        }
        PushInput(id) => {
            let (_, mut knob_messages) = network.push_input(id)?;
            let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
            messages.push(ResponseWithKnobs::Clock(Response::PushInput(id)));
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