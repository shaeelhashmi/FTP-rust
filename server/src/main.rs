use shared::Message;
use std::fs::{self, File}; // Added fs and File
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    loop {
        // 1. Read Length Header
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).is_err() {
            return;
        }
        let len = u32::from_be_bytes(len_bytes) as usize;

        // 2. Read JSON
        let mut json_buffer = vec![0u8; len];
        if stream.read_exact(&mut json_buffer).is_err() {
            return;
        }

        let received_text = String::from_utf8_lossy(&json_buffer);
        let request: Result<Message, _> = serde_json::from_str(&received_text);

        match request {
            Ok(Message::Hello { client_id }) => {
                println!("User logged in: {}", client_id);
                send_message(
                    &mut stream,
                    &Message::Welcome {
                        session_id: "sess_1".to_string(),
                    },
                );
            }

            Ok(Message::InitUpload {
                file_name,
                total_size,
            }) => {
                println!("Upload Start: {} ({} bytes)", file_name, total_size);
                // Create the uploads directory if it doesn't exist
                fs::create_dir_all("uploads").unwrap();
                send_message(
                    &mut stream,
                    &Message::InitAck {
                        chunk_size: 4 * 1024 * 1024,
                    },
                );
            }

            Ok(Message::ChunkMeta { chunk_index, size }) => {
                println!(">> Receiving Chunk #{} ({} bytes)", chunk_index, size);

                // 3. READ BINARY DATA
                let mut file_data = vec![0u8; size];
                if stream.read_exact(&mut file_data).is_err() {
                    return;
                }

                // 4. WRITE TO DISK (New!)
                // We save it as "uploads/filename_part_0"
                let safe_name = format!("uploads/chunk_{}", chunk_index);
                let mut f = File::create(&safe_name).unwrap();
                f.write_all(&file_data).unwrap();

                println!(">> Saved {}", safe_name);
                send_message(&mut stream, &Message::ChunkAck { chunk_index });
            }
            _ => {}
        }
    }
}

fn send_message(stream: &mut TcpStream, msg: &Message) {
    let json = serde_json::to_string(msg).unwrap();
    let len = (json.len() as u32).to_be_bytes();
    stream.write_all(&len).unwrap();
    stream.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on 7878...");
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            thread::spawn(|| handle_client(s));
        }
    }
}
