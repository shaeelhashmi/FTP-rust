// 'use' imports tools from the Standard Library (std).
// TcpListener is the tool that listens for connections.
// Read and Write are "traits" that let us read/write bytes.
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 1. BIND: We ask the OS to give us port 7878.
    // "127.0.0.1" is "localhost" (your own computer).
    // .unwrap() means "If this fails (e.g., port in use), crash immediately."
    // In Rust, we handle errors explicitly. unwrap() is the lazy way to do it.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server is listening on 127.0.0.1:7878...");

    // 2. ACCEPT LOOP: We enter an infinite loop to accept connections.
    // listener.incoming() gives us a "stream" whenever someone connects.
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // We successfully connected!
                println!("A client connected!");

                // Let's create a buffer (an array of 512 zeros) to hold their message.
                let mut buffer = [0; 512];

                // Read data from the client into our buffer.
                stream.read(&mut buffer).unwrap();

                // Convert the raw bytes into a String so we can print it.
                // String::from_utf8_lossy turns bytes like [72, 101, 108, 108, 111] into "Hello"
                println!("Client said: {}", String::from_utf8_lossy(&buffer));

                // Send a reply back
                stream.write_all(b"Hello from Server!").unwrap();
            }
            Err(e) => {
                // If the connection failed, print the error.
                println!("Connection failed: {}", e);
            }
        }
    }
}
