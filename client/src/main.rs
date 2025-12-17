use sha2::{Digest, Sha256};
use shared::Message;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread; // NEW

const CHUNK_SIZE: u64 = 4 * 1024 * 1024;
const WORKER_COUNT: usize = 4;

// ... (keep read_chunk, send_message, read_message, connect_and_auth exactly as they are) ...
fn read_chunk(filename: &str, chunk_index: u64) -> Vec<u8> {
    let mut file = File::open(filename).expect("File not found");
    file.seek(SeekFrom::Start(chunk_index * CHUNK_SIZE))
        .expect("Seek failed");
    let mut buffer = Vec::new();
    let _ = file.take(CHUNK_SIZE).read_to_end(&mut buffer);
    buffer
}

fn send_message(stream: &mut TcpStream, msg: &Message) {
    let json = serde_json::to_string(msg).unwrap();
    let len = (json.len() as u32).to_be_bytes();
    stream.write_all(&len).unwrap();
    stream.write_all(json.as_bytes()).unwrap();
}

fn read_message(stream: &mut TcpStream) -> Message {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).unwrap();
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut json_buf = vec![0u8; len];
    stream.read_exact(&mut json_buf).unwrap();
    let text = String::from_utf8_lossy(&json_buf);
    serde_json::from_str(&text).unwrap()
}

fn connect_and_auth() -> TcpStream {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    send_message(
        &mut stream,
        &Message::Hello {
            client_id: "Worker".to_string(),
        },
    );
    read_message(&mut stream);
    stream
}

fn main() {
    let filename = "test_file.bin";
    let file_size = std::fs::metadata(filename).unwrap().len();
    let total_chunks = (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE;

    // VARIABLE TO STORE THE ID
    let mut current_upload_id = String::new();

    // 1. INITIALIZE (Get the ID)
    {
        println!("Initializing upload with server...");
        let mut setup_stream = connect_and_auth();
        send_message(
            &mut setup_stream,
            &Message::InitUpload {
                file_name: filename.to_string(),
                total_size: file_size,
            },
        );

        let response = read_message(&mut setup_stream);
        if let Message::InitAck { upload_id, .. } = response {
            println!("Server assigned Upload ID: {}", upload_id);
            current_upload_id = upload_id; // Save it!
        } else {
            panic!("Server did not send InitAck!");
        }
    }

    // Share the ID with workers (Arc string)
    let upload_id_arc = Arc::new(current_upload_id.clone());
    let job_queue: Vec<u64> = (0..total_chunks).collect();
    let queue_ptr = Arc::new(Mutex::new(job_queue));
    let mut handles = vec![];

    for worker_id in 0..WORKER_COUNT {
        let queue_ref = Arc::clone(&queue_ptr);
        let id_ref = Arc::clone(&upload_id_arc); // Give worker access to ID
        let fname = filename.to_string();

        let handle = thread::spawn(move || {
            let mut stream = connect_and_auth();
            loop {
                // ... (Pop Job) ...
                let chunk_index = {
                    let mut queue = queue_ref.lock().unwrap();
                    match queue.pop() {
                        Some(idx) => idx,
                        None => break,
                    }
                };

                let mut attempts = 0;
                loop {
                    attempts += 1;
                    // ... (Read chunk logic) ...
                    let chunk_data = read_chunk(&fname, chunk_index);

                    // ... (Hash logic) ...
                    let mut hasher = Sha256::new();
                    hasher.update(&chunk_data);
                    let hash_string = hex::encode(hasher.finalize());

                    // SEND MESSAGE (Using the Upload ID!)
                    send_message(
                        &mut stream,
                        &Message::ChunkMeta {
                            upload_id: id_ref.to_string(), // <--- USE ID
                            chunk_index,
                            size: chunk_data.len(),
                            hash: hash_string,
                        },
                    );

                    stream.write_all(&chunk_data).unwrap();

                    // ... (Ack/Nack logic same as before) ...
                    let response = read_message(&mut stream);
                    match response {
                        Message::ChunkAck { .. } => break,
                        Message::ChunkNack { .. } => println!("Worker {} Retry...", worker_id),
                        _ => {}
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 2. COMPLETE (Send ID)
    let mut stream = connect_and_auth();
    send_message(
        &mut stream,
        &Message::Complete {
            upload_id: current_upload_id, // <--- USE ID
            file_name: filename.to_string(),
        },
    );
    println!("Done.");
}
