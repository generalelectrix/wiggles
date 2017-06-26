extern crate console_server;
extern crate serde;
#[macro_use] extern crate log;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;
extern crate fixture_patch;
extern crate fixture_patch_message;

use std::time::Duration;
use console_server::*;
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
    fn handle_patch_message(&mut self, message: PatchServerRequest) -> Messages<Response> {
        match handle_patch_message(&mut self.patch, message) {
            Ok(resp) => Messages::wrap(resp),
            Err(e) => Messages::one(Response::Error(format!("{}", e))),
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
        let client_data = cmd.client_data;
        match cmd.payload {
            Command::Patch(msg) => {
                let mut responses = self.handle_patch_message(msg);
                let responses = responses.drain().map(|r| r.with_client(client_data)).collect();
                responses
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn);
    
    let state: InitialState<TestConsole> = InitialState::default();

    console_server::run(state).unwrap();
}
