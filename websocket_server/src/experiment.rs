extern crate hyper;
extern crate futures;
extern crate websocket;
extern crate fixture_patch;
extern crate local_ip;
extern crate rand;

use rand::Rng;
use std::fmt;
use std::thread;
use std::error::Error;
use std::net::SocketAddr;
use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use websocket::Message;
use websocket::sync::server::upgrade::IntoWs;
use websocket::sync::server::upgrade::HyperRequest;

const HELLO_HTML: &'static str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello, wiggles!</title>
  </head>
  <body>
    <span>Hello, wiggles!</span>
  </body>
</html>";

/// Consoles built with Wiggles must implement this trait to be run by this server.
trait WigglesApp {

}

struct NoApp;

impl WigglesApp for NoApp {}

struct ConsoleService<A: WigglesApp> {
    session_key: String,
    application: A,
}

impl<A: WigglesApp> ConsoleService<A> {
    /// Run the service with a randomly-generated session key and the provided application.
    pub fn new(app: A) -> Self {
        ConsoleService{
            session_key: rand::thread_rng().gen_ascii_chars().take(32).collect(),
            application: app,
        }
    }

    /// Run the service with the specified session key.
    pub fn with_key<K: Into<String>>(app: A, key: K) -> Self {
        ConsoleService {
            session_key: key.into(),
            application: app,
        }
    }

    pub fn session_key(&self) -> &str {
        &self.session_key
    }
}

#[derive(Debug)]
enum ServerError {
    Hyper(hyper::Error),
    WebsocketUpgrade(Option<SocketAddr>),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ServerError::*;
        match *self {
            Hyper(ref err) => err.fmt(f),
            WebsocketUpgrade(ref addr) => {
                match *addr {
                    Some(a) => write!(f, "Request from {:?} upgraded to a WebSocket.", a),
                    None => write!(f, "Request upgraded to a WebSocket."),
                }
            },
        }
    }
}

impl Error for ServerError {
    fn description(&self) -> &str {
        use ServerError::*;
        match *self {
            Hyper(ref err) => err.description(),
            WebsocketUpgrade(_) => "Request upgraded to a WebSocket.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        use ServerError::*;
        match *self {
            Hyper(ref err) => Some(err),
            WebsocketUpgrade(_) => None,
        }
    }
}

impl<A: WigglesApp> Service for ConsoleService<A> {
    type Request = Request;
    type Response = Response;
    type Error = ServerError;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    /// Request handler.  Serves the view application if the request has this session's unique key.
    /// Also sets a cookie for the client with this key, which is used by the view to open a web socket.
    /// Upgrades a web socket request from the view.
    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        let good_code = req.query().map(|query| query.contains(&self.session_key)).unwrap_or(false);
        let req_addr = req.remote_addr();
        match (good_code, HyperRequest(req).into_ws()) {
            (false, _) => {
                println!("Request without correct session key: {:?}", req);
                response.set_status(StatusCode::NotFound);
                futures::future::ok(response)
            },
            (true, Err((request, _))) => {
                // Not a web socket upgrade, serve the view application.
                // TODO: bundle the view into this binary
                response.set_body(HELLO_HTML);
                futures::future::ok(response)
            },
            (true, Ok(upgrade)) => {
                // upgrade responds to the client, do not return a response
                let mut client = match upgrade.accept() {
                    Ok(c) => c,
                    Err(_) => panic!(),
                };
                // TODO: do something with this client
                futures::future::err(ServerError::WebsocketUpgrade(req_addr))
            }
        }

        
    }
}

fn main() {
    // for testing just use hardcoded key
    let service = ConsoleService::with_key(NoApp, "WIGGLES");
    let host_ip = local_ip::get().unwrap();
    let port = 80;
    println!(
        "Access console at https://{}:{}/console?session_key={}",
        host_ip.to_string(),
        port,
        service.session_key);

}