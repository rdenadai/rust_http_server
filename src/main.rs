#![allow(dead_code)]

use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:9999".to_string());
    server.run();
}
