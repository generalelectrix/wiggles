extern crate console_server;
extern crate serde;
#[macro_use] extern crate log;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;
extern crate fixture_patch;
extern crate fixture_patch_message;
extern crate rust_dmx;
extern crate dataflow;
extern crate dataflow_message;
extern crate wiggles_value;

use std::fmt;
use std::time::Duration;
use console_server::*;
use console_server::clients::{ClientData, ResponseFilter};
use console_server::reactor::*;
use fixture_patch::{Patch, UniverseId};
use fixture_patch_message::{
    PatchServerRequest,
    PatchServerResponse,
    handle_message as handle_patch_message,
    UnivWithPort};
use rust_dmx::{DmxPort, OfflineDmxPort, Error as DmxError};
use dataflow::clocks::{ClockKnobAddr, ClockNetwork, ClockCollection};
use dataflow::wiggles::{WiggleId, WiggleKnobAddr, WiggleNetwork, WiggleCollection};
use dataflow_message::clock::{
    Command as ClockCommand,
    Response as ClockResponse,
    ResponseWithKnobs as ClockResponseWithKnobs,
    handle_message as handle_clock_message,
};
use dataflow_message::wiggle::{
    Command as WiggleCommand,
    Response as WiggleResponse,
    ResponseWithKnobs as WiggleResponseWithKnobs,
    handle_message as handle_wiggle_message,
};
use wiggles_value::knob::{
    Response as KnobResponse,
    Command as KnobCommand,
    Error as KnobError,
    Knobs,
    KnobDescription,
};

#[derive(Serialize, Deserialize, Default)]
struct TestConsole {
    patch: Patch<WiggleId>,
    clocks: ClockNetwork,
    wiggles: WiggleNetwork,
}

impl TestConsole {
    fn handle_patch_message(
        &mut self,
        message: PatchServerRequest<WiggleId>,
        client_data: ClientData)
        -> Messages<ResponseWrapper<Response>>
    {
        let result = handle_patch_message(&mut self.patch, message);
        handle_error(result, client_data, Response::Patcher)
    }

    /// Swap a port to offline mode if it looks like it has been disconnected.
    fn handle_dmx_port_error(
            &mut self,
            uid: UniverseId,
            err: DmxError)
            -> Messages<ResponseWrapper<Response>>
    {
        match err {
            DmxError::IO(e) => {
                // Interpret OS error code 6 "device not configured" as an unplugged port.
                // Replace it with an offline port and inform the world.
                if let Some(6) = e.raw_os_error() {
                    return self.set_port_offline(uid);
                }
            }
            _ => (),
        }
        Messages::none()
    }

    fn set_port_offline(&mut self, uid: UniverseId) -> Messages<ResponseWrapper<Response>> {
        let offline_port = Box::new(OfflineDmxPort);
        let univ_with_port = UnivWithPort::new(
            uid,
            offline_port.namespace().to_string(),
            offline_port.port_name().to_string());

        if let Err(e) = self.patch.set_universe_port(uid, offline_port) {
            // This should be unreachable.
            error!(
                "An error occurred while trying to set universe {} to offline mode: {}.",
                uid,
                e);
        }
        let mut messages = Messages::none();
        let port_msg = Response::Patcher(PatchServerResponse::UpdateUniverse(univ_with_port));
        let err_msg = Response::Error(
            format!("Universe {}'s DMX port has been disconnected.", uid));
        messages.push(port_msg.no_client());
        messages.push(err_msg.no_client());
        messages
    }

    fn handle_clock_message(
        &mut self,
        message: ClockCommand,
        client_data: ClientData)
        -> Messages<ResponseWrapper<Response>>
    {
        let result = handle_clock_message(&mut self.clocks, message).map(|(mut resp, filter)| {
            let resp = resp.drain().map(|r| {
                match r {
                    ClockResponseWithKnobs::Knob(m) =>
                        Response::Knob(m.lift_address(KnobAddress::Clock)),
                    ClockResponseWithKnobs::Clock(m) =>
                        Response::Clock(m),
                }
            }).collect();
            (resp, filter)
        });
        handle_error(result, client_data, |x| x)
    }

    fn handle_wiggle_message(
        &mut self,
        message: WiggleCommand,
        client_data: ClientData)
        -> Messages<ResponseWrapper<Response>>
    {
        let result = handle_wiggle_message(&mut self.wiggles, message).map(|(mut resp, filter)| {
            let resp = resp.drain().map(|r| {
                match r {
                    WiggleResponseWithKnobs::Knob(m) =>
                        Response::Knob(m.lift_address(KnobAddress::Wiggle)),
                    WiggleResponseWithKnobs::Wiggle(m) =>
                        Response::Wiggle(m),
                }
            }).collect();
            (resp, filter)
        });
        handle_error(result, client_data, |x| x)
    }

    fn handle_knob_message(
        &mut self,
        message: KnobCommand<KnobAddress>,
        client_data: ClientData)
        -> Messages<ResponseWrapper<Response>>
    {
        
        let result = match message {
            KnobCommand::Set(addr, value) => {
                match addr {
                    KnobAddress::Clock(a) => {
                        let result =
                            self.clocks.set_knob(a.clone(), value.clone())
                                .map(|()| Messages::one(KnobResponse::ValueChange(a, value)));
                        lift_knob_result(result, &KnobAddress::Clock)
                    }
                    KnobAddress::Wiggle(a) => {
                        let result =
                            self.wiggles.set_knob(a.clone(), value.clone())
                                .map(|()| Messages::one(KnobResponse::ValueChange(a, value)));
                        lift_knob_result(result, &KnobAddress::Wiggle)
                    }
                }
            }
            KnobCommand::State => {
                fn lift_state<A, K: Knobs<A>, F>(
                    knob_system: &K, lifter: F)
                    -> Vec<(KnobAddress, KnobDescription)>
                    where F: Fn(A) -> KnobAddress
                {
                    let mut knobs = knob_system.knobs();
                    let knobs = knobs.drain(..).map(|(addr, desc)| (lifter(addr), desc)).collect();
                    knobs
                }
                let mut clock_knobs = lift_state(&self.clocks, KnobAddress::Clock);
                let wiggle_knobs = lift_state(&self.wiggles, KnobAddress::Wiggle);
                clock_knobs.extend(wiggle_knobs);
                Ok(Messages::one(KnobResponse::State(clock_knobs)))
            }
        };
        handle_error(result.map(|r| (r, None)), client_data, Response::Knob)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum KnobAddress {
    Clock(ClockKnobAddr),
    Wiggle(WiggleKnobAddr),
}

type KnobResult<A> = Result<Messages<KnobResponse<A>>, KnobError<A>>;

/// Take the result from handling a knob command and lift it up into the global address space.
fn lift_knob_result<A, F>(
    r: KnobResult<A>,
    lifter: &F)
    -> KnobResult<KnobAddress>
    where F: Fn(A) -> KnobAddress
{
    match r {
        Ok(mut responses) => {
            Ok(responses.drain().map(|msg| msg.lift_address(lifter)).collect())
        }
        Err(err) => Err(err.lift_address(lifter))
    }
}

/// Handle a result from handling a message.
/// Convert the error case into the top-level response type.
/// Lift the OK case's messages into the top-level wrapper type.
fn handle_error<M, E, F>(
    r: Result<(Messages<M>, Option<ResponseFilter>), E>,
    mut client_data: ClientData,
    message_wrapper: F)
    -> Messages<ResponseWrapper<Response>>
    where F: Fn(M) -> Response, E: fmt::Display
{
    match r {
        Ok((mut resp, maybe_filter)) => {
            if let Some(filter) = maybe_filter {
                client_data.filter = filter;
            }
            resp.drain()
                .map(|m| message_wrapper(m).with_client(client_data))
                .collect()
        }
        Err(e) => {
            client_data.filter = ResponseFilter::Exclusive;
            Messages::one(Response::Error(format!("{}", e)).with_client(client_data))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Patcher(PatchServerRequest<WiggleId>),
    Clock(ClockCommand),
    Wiggle(WiggleCommand),
    Knob(KnobCommand<KnobAddress>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Response {
    Error(String),
    Patcher(PatchServerResponse<WiggleId>),
    Clock(ClockResponse),
    Wiggle(WiggleResponse),
    Knob(KnobResponse<KnobAddress>),
}

impl WrapResponse for Response {}

impl Console for TestConsole {
    type Command = Command;
    type Response = Response;

    fn render(&mut self) -> Messages<ResponseWrapper<Response>> {
        let render_errors = self.patch.render();
        let mut messages = Messages::none();
        for (uid, err) in render_errors {
            error!("DMX render error, universe {}: {:?}", uid, err);
            let error_messages = self.handle_dmx_port_error(uid, err);
            messages.extend(error_messages);
        }
        messages
    }

    fn update(&mut self, dt: Duration) -> Messages<ResponseWrapper<Response>> {
        // update the clocks
        let mut clock_msgs = self.clocks.update(dt);
        let mut wiggle_msgs = self.wiggles.update(dt);
        let mut messages = Messages::none();
        messages.reserve(clock_msgs.len() + wiggle_msgs.len());
        for msg in clock_msgs.drain() {
            messages.push(Response::Knob(msg.lift_address(KnobAddress::Clock)).no_client());
        }
        for msg in wiggle_msgs.drain() {
            messages.push(Response::Knob(msg.lift_address(KnobAddress::Wiggle)).no_client());
        }
        messages
    }

    fn handle_command(&mut self, cmd: CommandWrapper<Command>) -> Messages<ResponseWrapper<Response>> {
        match cmd.payload {
            Command::Patcher(msg) => {
                self.handle_patch_message(msg, cmd.client_data)
            }
            Command::Knob(msg) => {
                self.handle_knob_message(msg, cmd.client_data)
            }
            Command::Clock(msg) => {
                self.handle_clock_message(msg, cmd.client_data)
            }
            Command::Wiggle(msg) => {
                self.handle_wiggle_message(msg, cmd.client_data)
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();
    
    let state: InitialState<TestConsole> = InitialState::default();

    console_server::run(state).unwrap();
}
