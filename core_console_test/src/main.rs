extern crate console_server;
extern crate serde;
#[macro_use] extern crate log;
extern crate simple_logger;
#[macro_use] extern crate serde_derive;

use console_server::{Console, Messages, CommandWrapper, ResponseWrapper};

#[derive(Serialize, Deserialize)]
struct NoopConsole;

#[derive(Debug, Serialize, Deserialize)]
enum Cmd {
    Command,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Rsp {
    Response,
}

impl Console for NoopConsole {
    type Command = Cmd;
    type Response = Rsp;

    fn render(&mut self) -> Messages<ResponseWrapper<Rsp>> {
        debug!("Console rendered.");
        Messages::none()
    }

    fn update(&mut self, dt: Duration) -> Messages<ResponseWrapper<Rsp>> {
        debug!("Console updated.");
        Messages::none()
    }
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Debug);
    println!("Hello, world!");
}
