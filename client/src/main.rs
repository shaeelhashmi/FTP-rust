use std::io::{Read, Write};
use std::net::TcpStream; // Note: We use 'TcpStream' here, not 'Listener'

fn main() {
    // 1. CONNECT: Try to dial the server at port 7878.
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server!");

            // 2. WRITE: Send a message (as raw bytes)
            let msg = b"Hello, I am the Client!";
            stream.write_all(msg).unwrap();

            // 3. READ: Wait for the server to reply
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();

            println!("Server replied: {}", String::from_utf8_lossy(&buffer));
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
