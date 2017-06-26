extern crate console_server;
extern crate serde;
#[macro_use] extern crate log;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;
extern crate fixture_patch;
extern crate fixture_patch_message;

use std::time::Duration;
use console_server::*;
use console_server::clients::{ClientData, ResponseFilter};
use console_server::reactor::*;
use fixture_patch::Patch;
use fixture_patch_message::{
    PatchServerRequest,
    PatchServerResponse,
    handle_message as handle_patch_message};

#[derive(Serialize, Deserialize, Default)]
struct TestConsole {
    patch: Patch
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
                    .map(|m| Response::Patch(m).with_client(client_data))
                    .collect()
            }
            Err(e) => {
                client_data.filter = ResponseFilter::Exclusive;
                Messages::one(Response::Error(format!("{}", e)).with_client(client_data))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    Patch(PatchServerRequest),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Response {
    Error(String),
    Patch(PatchServerResponse),
}

impl From<PatchServerResponse> for Response {
    fn from(r: PatchServerResponse) -> Self {
        Response::Patch(r)
    }
}

impl WrapResponse for Response {}

impl Console for TestConsole {
    type Command = Command;
    type Response = Response;

    fn render(&mut self) -> Messages<ResponseWrapper<Response>> {
        let render_errors = self.patch.render();
        for err in render_errors {
            error!("DMX render error: {}", err);
        }
        Messages::none()
    }

    fn update(&mut self, dt: Duration) -> Messages<ResponseWrapper<Response>> {
        Messages::none()
    }

    fn handle_command(&mut self, cmd: CommandWrapper<Command>) -> Messages<ResponseWrapper<Response>> {
        match cmd.payload {
            Command::Patch(msg) => {
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
