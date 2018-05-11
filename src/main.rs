mod server;
use {server::Server};
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let mut server = Server::new();
    server
        .event_loop
        .run();
}
