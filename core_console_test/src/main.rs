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
use dataflow::clocks::ClockNetwork;
use dataflow::wiggles::WiggleNetwork;
use dataflow_message::clock::{
    Command as ClockCommand,
    Response as ClockResponse,
    ResponseWithKnobs as ClockResponseWithKnobs};
use dataflow_message::wiggle::{
    Command as WiggleCommand,
    Response as WiggleResponse,
    ResponseWithKnobs as WiggleResponseWithKnobs};
use wiggles_value::knob::{Message as KnobMessage};

#[derive(Serialize, Deserialize, Default)]
struct TestConsole {
    patch: Patch,
    clocks: ClockNetwork,
    wiggles: WiggleNetwork,
}

impl TestConsole {
    fn handle_patch_message(
            &mut self,
            message: PatchServerRequest,
            mut client_data: ClientData)
            -> Messages<ResponseWrapper<Response>>
    {
        match handle_patch_message(&mut self.patch, message) {
            Ok((mut resp, maybe_filter)) => {
                if let Some(filter) = maybe_filter {
                    client_data.filter = filter;
                }
                resp.drain()
                    .map(|m| Response::Patcher(m).with_client(client_data))
                    .collect()
            }
            Err(e) => {
                client_data.filter = ResponseFilter::Exclusive;
                Messages::one(Response::Error(format!("{}", e)).with_client(client_data))
            }
        }
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
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Patcher(PatchServerRequest),
    Clock()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Response {
    Error(String),
    Patcher(PatchServerResponse),
}

impl From<PatchServerResponse> for Response {
    fn from(r: PatchServerResponse) -> Self {
        Response::Patcher(r)
    }
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
        Messages::none()
    }

    fn handle_command(&mut self, cmd: CommandWrapper<Command>) -> Messages<ResponseWrapper<Response>> {
        match cmd.payload {
            Command::Patcher(msg) => {
                self.handle_patch_message(msg, cmd.client_data)
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn);
    
    let state: InitialState<TestConsole> = InitialState::default();

    console_server::run(state).unwrap();
}
