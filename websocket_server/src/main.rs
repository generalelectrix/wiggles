extern crate websocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::thread;
use websocket::OwnedMessage;
use websocket::sync::Server;

#[derive(Debug, Serialize, Deserialize)]
struct PatchRequest {
    name: String,
    kind: String,
    address: Option<(u32, u32)>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Request {
    PatchState,
    NewPatches(Vec<PatchRequest>),
    Rename(u32, String),
    Repatch(u32, Option<(u32, u32)>),
    Remove(u32),
    GetKinds,
}

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            println!("New request using protocols {:?}", request.protocols());

            if !request.protocols().contains(&"wiggles".to_string()) {
                request.reject().unwrap();
                return;
            }

            let mut client = request.use_protocol("wiggles").accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            let message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&message).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Text(m) => {
                        println!("Received message: {}", m);
                        match serde_json::from_str::<Request>(&m) {
                            Ok(msg) => {
                                println!("Deserialized {:?}", msg);
                                let reserialized = serde_json::to_string(&msg).unwrap();
                                sender.send_message(&OwnedMessage::Text(reserialized)).unwrap();
                            }
                            Err(e) => println!("Deserialization error: {}", e),
                        }
                        
                    }
                    x => {
                        println!("Other kind of message: {:?}", x);
                    }
                }
            }
        });
    }
}