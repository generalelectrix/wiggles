extern crate console_server;
extern crate serde;
#[macro_use] extern crate log;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;

use std::time::Duration;
use console_server::*;
use console_server::reactor::*;

#[derive(Serialize, Deserialize)]
struct TestConsole {
    value: f64
}

impl Default for TestConsole {
    fn default() -> Self {
        TestConsole{
            value: 0.0
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Cmd {
    TestCommand(f64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Rsp {
    TestResponse(f64),
}

impl WrapResponse for Rsp {}

impl Console for TestConsole {
    type Command = Cmd;
    type Response = Rsp;

    fn render(&mut self) -> Messages<ResponseWrapper<Rsp>> {
        Messages::none()
    }

    fn update(&mut self, dt: Duration) -> Messages<ResponseWrapper<Rsp>> {
        Messages::none()
    }

    fn handle_command(&mut self, cmd: CommandWrapper<Cmd>) -> Messages<ResponseWrapper<Rsp>> {
        let Cmd::TestCommand(v) = cmd.payload;
        self.value = v;
        Messages::one(Rsp::TestResponse(v).with_client(cmd.client_data))
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn);
    
    let state: InitialState<TestConsole> = InitialState::default();

    console_server::run(state).unwrap();
}
