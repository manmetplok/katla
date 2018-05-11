pub extern crate wayland_server as ways;
use std::ffi::{OsStr, OsString};

pub struct Server {
    pub display: self::ways::Display,
    pub event_loop: self::ways::EventLoop,
    pub socket_name: OsString,
}

impl Server {
    pub fn new() -> Server {

        let (mut display, event_loop) = self::ways::Display::new();
        let socket_name = display
            .add_socket_auto()
            .expect("Failed to create a server socket.");

        println!("have something");
        Server {
            display: display,
            event_loop: event_loop,
            socket_name: socket_name,
        }
    }
}
