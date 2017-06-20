//! The core structure of a Wiggles console.
//! Agnostic about the actual logical console that is running inside it, as well as the types of
//! control clients that are connected to it.
//! Provides a core show reactor running a lightweight event loop which can save and load shows and
//! drives the console logic itself which is hidden behind the Console trait.
pub mod show_library;
pub mod reactor;
pub mod clients;
pub mod socket_server;

extern crate event_loop;
extern crate smallvec;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate chrono;
extern crate ordermap;
extern crate websocket;
#[macro_use] extern crate log;

#[cfg(test)] extern crate simple_logger;
#[cfg(test)] extern crate rand;
#[macro_use] extern crate serde_derive;

use std::path::PathBuf;
use event_loop::Settings;
use std::thread;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::io::Error as IoError;
use std::fmt;
use std::error::Error;
use chrono::prelude::Local;
use serde::Serialize;

use reactor::{Console, Reactor};
use show_library::{ShowLibrary, LoadSpec, LibraryError};
use clients::ResponseRouter;
use socket_server::SocketServer;

/// All of the initial state and parameters needed to run a Wiggles console.
pub struct InitialState<C> {
    library_path: PathBuf,
    console_constructor: Box<Fn()->C>,
    show_library: ShowLibrary,
    load_spec: LoadSpec,
    event_settings: Option<Settings>,
    websocket_addr: SocketAddr,
    websocket_protocol: String,
}

impl<C: Default + Serialize> Default for InitialState<C> {
    /// Create a new show with entirely default parameters.
    /// Panics if the show library cannot be found or some other disk-related error occurs.
    /// Probably best to use the builder which can unpack a library error.
    fn default() -> Self {
        let show_name = format!("New Show {}", Local::now().format("%b %e, %Y %H:%M:%S"));
        let library_path = PathBuf::from("./show_library");
        let console_constructor = Box::new(|| C::default());
        let console = (*console_constructor)();
        let show_lib = ShowLibrary::create_new(&library_path, show_name, &console).unwrap();
        InitialState {
            library_path: library_path,
            console_constructor: console_constructor,
            show_library: show_lib,
            load_spec: LoadSpec::Latest,
            event_settings: None,
            websocket_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80)),
            websocket_protocol: "wiggles".to_string(),
        }
    }
}

// TODO: provide a useful builder for InitialState.

#[derive(Debug)]
/// The errors that could occur during show initialization.
pub enum RunError {
    Reactor(LibraryError),
    SocketServer(IoError)
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunError::Reactor(ref e) => write!(f, "The reactor could not be started: {}", e),
            RunError::SocketServer(ref e) => write!(f, "The client socket server could not be started: {}", e),
        }
    }
}

impl Error for RunError {
    fn description(&self) -> &str {
        match *self {
            RunError::Reactor(_) => "The reactor could not be started due to a show library error.",
            RunError::SocketServer(_) => "The client socket server could not be started.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RunError::Reactor(ref e) => Some(e),
            RunError::SocketServer(ref e) => Some(e),
        }
    }
}

/// Given an initial console state, set up everything and run.
pub fn run<C: Console>(state: InitialState<C>) -> Result<(), RunError> {
    let (mut reactor, cmd_send, resp_recv) = Reactor::create(
        state.console_constructor,
        state.library_path,
        state.show_library,
        state.load_spec,
        state.event_settings,
    ).map_err(RunError::Reactor)?;

    let mut response_router = ResponseRouter::new(resp_recv);
    let client_registry = response_router.client_registry();

    let mut server = SocketServer::new(
        client_registry,
        cmd_send,
        state.websocket_addr,
        state.websocket_protocol
    ).map_err(RunError::SocketServer)?;

    // Fire up the response router.
    thread::spawn(move || response_router.run());

    // Start the client server.
    thread::spawn(move || server.run());

    // Now run the reactor in the main thread.
    reactor.run();

    Ok(())
}

pub use reactor::*;