use sha2::{Digest, Sha256};
use shared::Message;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread; // NEW

// ... (keep merge_chunks and send_message exactly as they are) ...
fn merge_chunks(file_name: &str, total_chunks: u64) {
    let output_path = format!("uploads/{}", file_name);
    let mut output_file = File::create(&output_path).unwrap();
    for i in 0..total_chunks {
        let chunk_path = format!("uploads/chunk_{}", i);
        let mut chunk_file = File::open(&chunk_path).unwrap();
        std::io::copy(&mut chunk_file, &mut output_file).unwrap();
        fs::remove_file(chunk_path).unwrap();
    }
    println!(">> File Assembled: {}", output_path);
}

fn send_message(stream: &mut TcpStream, msg: &Message) {
    let json = serde_json::to_string(msg).unwrap();
    let len = (json.len() as u32).to_be_bytes();
    stream.write_all(&len).unwrap();
    stream.write_all(json.as_bytes()).unwrap();
}

fn handle_client(mut stream: TcpStream) {
    loop {
        // ... (Header reading logic same as before) ...
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).is_err() {
            return;
        }
        let len = u32::from_be_bytes(len_bytes) as usize;

        let mut json_buffer = vec![0u8; len];
        if stream.read_exact(&mut json_buffer).is_err() {
            return;
        }
        let received_text = String::from_utf8_lossy(&json_buffer);
        let request: Result<Message, _> = serde_json::from_str(&received_text);

        match request {
            Ok(Message::Hello { .. }) => {
                send_message(
                    &mut stream,
                    &Message::Welcome {
                        session_id: "s1".to_string(),
                    },
                );
            }
            Ok(Message::InitUpload { file_name, .. }) => {
                fs::create_dir_all("uploads").unwrap();
                send_message(&mut stream, &Message::InitAck { chunk_size: 0 });
            }

            Ok(Message::ChunkMeta {
                chunk_index,
                size,
                hash,
            }) => {
                // 1. Read the Data
                let mut file_data = vec![0u8; size];
                if stream.read_exact(&mut file_data).is_err() {
                    return;
                }

                // 2. Calculate Hash Locally
                let mut hasher = Sha256::new();
                hasher.update(&file_data);
                let server_hash = hex::encode(hasher.finalize());

                // 3. Verify
                if server_hash == hash {
                    // MATCH: Save and ACK
                    let safe_name = format!("uploads/chunk_{}", chunk_index);
                    let mut f = File::create(&safe_name).unwrap();
                    f.write_all(&file_data).unwrap();

                    send_message(&mut stream, &Message::ChunkAck { chunk_index });
                } else {
                    // MISMATCH: Send NACK
                    println!("!!! CORRUPTION on Chunk #{} !!! Sending NACK.", chunk_index);
                    // Do NOT save the file.
                    send_message(&mut stream, &Message::ChunkNack { chunk_index });
                }
            }

            Ok(Message::Complete { file_name }) => {
                merge_chunks(&file_name, 13);
            }
            _ => {}
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening...");
    for stream in listener.incoming() {
        if let Ok(s) = stream {
            thread::spawn(|| handle_client(s));
        }
    }
}
