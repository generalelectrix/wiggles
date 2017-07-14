//! Message-passing API on top of a wiggle dataflow network.
use std::sync::Arc;
use std::fmt;
use std::error;
use console_server::reactor::Messages;
use console_server::clients::ResponseFilter;
use wiggles_value::knob::{KnobDescription, Response as KnobResponse, Knobs};
use dataflow::network::{InputId, NetworkError, OutputId, Node};
use dataflow::wiggles::{
    Wiggle,
    CompleteWiggle,
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
    target: Option<(WiggleId, OutputId)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Kinds,
    State,
    Create{kind: String, name: String},
    Remove{id: WiggleId, force: bool},
    Rename(WiggleId, String),
    SetInput(SetInput),
    PushInput(WiggleId),
    PopInput(WiggleId),
    PushOutput(WiggleId),
    PopOutput(WiggleId),
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
    kind: Arc<String>,
    inputs: Vec<Option<(WiggleId, OutputId)>>,
    outputs: usize,
    clock: UsesClock,
}

impl WiggleDescription {
    fn from_node(node: &Node<Box<CompleteWiggle>, WiggleId, KnobResponse<WiggleKnobAddr>>) -> Self {
        let wiggle = node.inner();
        let clock_spec = match wiggle.clock_source() {
            Ok(source) => UsesClock::Yes(source),
            Err(_) => UsesClock::No,
        };
        WiggleDescription {
            name: Arc::new(wiggle.name().to_string()),
            kind: Arc::new(wiggle.kind().to_string()),
            inputs: node.inputs().to_vec(),
            outputs: node.output_count(),
            clock: clock_spec,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response messages related to wiggle actions.
pub enum Response {
    Kinds(Arc<Vec<String>>),
    State(Vec<(WiggleId, WiggleDescription)>),
    New{id: WiggleId, desc: WiggleDescription},
    Removed(WiggleId),
    Renamed(WiggleId, Arc<String>),
    SetInput(SetInput),
    PushInput(WiggleId),
    PopInput(WiggleId),
    PushOutput(WiggleId),
    PopOutput(WiggleId),
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
    static ref KINDS: Arc<Vec<String>> = Arc::new(WIGGLES.iter().map(|s| s.to_string()).collect());
}

/// Apply the action dictated by a wiggle command to this wiggle network.
pub fn handle_message(
    network: &mut WiggleNetwork,
    command: Command)
    -> Result<(Messages<ResponseWithKnobs>, Option<ResponseFilter>), Error>
{
    use self::Command::*;
    match command {
        Kinds => Ok((
            Messages::one(ResponseWithKnobs::Wiggle(Response::Kinds(KINDS.clone()))),
            None)),
        State => {
            let state = network.nodes()
                .map(|(wiggle_id, node)| (wiggle_id, WiggleDescription::from_node(node)))
                .collect();
            Ok((Messages::one(ResponseWithKnobs::Wiggle(Response::State(state))), None))
        }
        Create{kind, name} => {
            let node = new_wiggle(&kind, name).ok_or(Error::UnknownKind(kind.clone()))?;
            let (id, node) = network.add(node);
            let wiggle = node.inner();
            // emit messages for all of the new knobs
            let mut messages = Messages::none();
            for (addr, desc) in wiggle.knobs() {
                // Lift the knob addr up into the network domain.
                let addr = (id, addr);
                let msg = ResponseWithKnobs::Knob(KnobResponse::Added(addr, desc));
                messages.push(msg);
            }

            messages.push(ResponseWithKnobs::Wiggle(Response::New{
                id: id,
                desc: WiggleDescription::from_node(node),
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
        Rename(wiggle, name) => {
            network.node_inner_mut(wiggle)?.set_name(name.clone());
            Ok((
                Messages::one(ResponseWithKnobs::Wiggle(Response::Renamed(wiggle, Arc::new(name)))),
                Some(ResponseFilter::All)))
        }
        SetInput(set) => {
            network.swap_input(set.wiggle, set.input, set.target)?;
            let msg = ResponseWithKnobs::Wiggle(Response::SetInput(set));
            Ok((Messages::one(msg), Some(ResponseFilter::All)))
        }
        PushInput(id) => {
            let (_, knob_messages) = network.push_input(id)?;
            handle_io_change(id, knob_messages, Response::PushInput)
        }
        PopInput(id) => {
            let knob_messages = network.pop_input(id)?;
            handle_io_change(id, knob_messages, Response::PopInput)
        }
        PushOutput(id) => {
            let (_, knob_messages) = network.push_output(id)?;
            handle_io_change(id, knob_messages, Response::PushOutput)
        }
        PopOutput(id) => {
            let knob_messages = network.pop_output(id)?;
            handle_io_change(id, knob_messages, Response::PopOutput)
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

fn handle_io_change<F>(
    id: WiggleId,
    mut knob_messages: Messages<KnobResponse<WiggleKnobAddr>>,
    resp: F)
    -> Result<(Messages<ResponseWithKnobs>, Option<ResponseFilter>), Error>
        where F: Fn(WiggleId) -> Response
{
    let mut messages = knob_messages.drain().map(ResponseWithKnobs::Knob).collect::<Messages<_>>();
    messages.push(ResponseWithKnobs::Wiggle(resp(id)));
    Ok((messages, Some(ResponseFilter::All)))
}

#[derive(Debug)]
pub enum Error {
    UnknownKind(String),
    NoClock(WiggleId),
    Network(NetworkError<WiggleId>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            UnknownKind(ref kind) => write!(f, "Unknown wiggle kind: '{}'.", kind),
            NoClock(ref id) => write!(f, "Wiggle {} does not use a clock input.", id),
            Network(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;
        match *self {
            UnknownKind(_) => "Unknown wiggle kind.",
            NoClock(_) => "This wiggle does not use a clock input.",
            Network(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::Error::*;
        match *self {
            UnknownKind(_) => None,
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