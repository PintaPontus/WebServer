use std::thread;

use server::Server;

mod server;

fn main() {
    println!("Server started!");
    let server = Server::start("127.0.0.1:80");
    server.handle.join().unwrap();
}