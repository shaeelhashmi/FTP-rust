use shared::Message;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread; // Import the shared Enum

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    return;
                }

                // 1. Convert bytes to String
                let received_text = String::from_utf8_lossy(&buffer[..size]);
                println!("Raw Data: {}", received_text);

                // 2. Deserialize: Turn String into a Rust Enum (Message)
                // We use from_str to parse the JSON.
                let request: Result<Message, _> = serde_json::from_str(&received_text);

                match request {
                    Ok(Message::Hello { client_id }) => {
                        println!("Processing Login for: {}", client_id);

                        // 3. Prepare the Reply (Welcome Message)
                        let reply = Message::Welcome {
                            session_id: "sess_999".to_string(), // Dummy ID for now
                        };

                        // 4. Send the Reply as JSON
                        let json_reply = serde_json::to_string(&reply).unwrap();
                        stream.write_all(json_reply.as_bytes()).unwrap();
                    }
                    Ok(_) => {
                        println!("Received a different message type.");
                    }
                    Err(e) => {
                        println!("Failed to parse JSON: {}", e);
                    }
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
