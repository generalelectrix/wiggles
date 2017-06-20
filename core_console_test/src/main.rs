extern crate console_server;
extern crate serde;
#[macro_use] extern crate serde_derive;

use console_server::Console;

#[derive(Serialize, Deserialize)]
struct NoopConsole;

enum Cmd {
    Hello
}

enum Rsp {
    HelloResp
}

impl Console for NoopConsole {

}

fn main() {
    println!("Hello, world!");
}
