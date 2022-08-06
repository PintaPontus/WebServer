use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;

pub struct Server{
    pub address: String,
    pub handle: JoinHandle<()>,
}

impl Server {
    pub fn start(addr: &str) -> Self {
        let address = addr.to_string();
        let handle = thread::spawn(move || { Self::bind(&address) });
        return Self{
            address: addr.to_string(),
            handle,
        }
    }
    fn bind(addr: &str) {
        let binding = TcpListener::bind(addr);
        match binding {
            Ok(listener) => {
                println!("Server working!");
                Self::listen(listener);
            }
            Err(e) => { println!("Failed to bind port: {addr}"); }
        }
    }
    fn listen(listener: TcpListener) {
        loop {
            let connection = listener.accept();
            match connection {
                Ok((stream, address)) => {
                    println!("Connected with: {}", address);
                    thread::spawn(|| { Self::receive(stream); });
                }
                Err(e) => { println!("Failed to connect: {}", e); }
            }
        }
    }
    fn receive(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("Request: {}", String::from_utf8_lossy(&buffer));
            }
            Err(e) => { println!("Error during read: {}", e); }
        }
    }
}