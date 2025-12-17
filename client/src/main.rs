use shared::Message;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Connected!");

            // 1. Send HELLO
            let msg = Message::Hello {
                client_id: "RustUser".to_string(),
            };
            let json = serde_json::to_string(&msg).unwrap();
            stream.write_all(json.as_bytes()).unwrap();

            // 2. Wait for WELCOME
            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(n) => {
                    let text = String::from_utf8_lossy(&buffer[..n]);

                    // Try to parse the reply
                    let response: Result<Message, _> = serde_json::from_str(&text);

                    if let Ok(Message::Welcome { session_id }) = response {
                        println!("Login Successful! Session ID: {}", session_id);
                    } else {
                        println!("Unexpected response: {}", text);
                    }
                }
                Err(e) => println!("Failed to read: {}", e),
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
}
