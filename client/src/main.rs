use shared::Message;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::TcpStream;

const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB

// ... (Keep read_chunk, send_message, read_message functions exactly as they were) ...
fn read_chunk(filename: &str, chunk_index: u64) -> Vec<u8> {
    let mut file = File::open(filename).expect("File not found");
    let start_pos = chunk_index * CHUNK_SIZE;
    file.seek(SeekFrom::Start(start_pos))
        .expect("Could not seek");
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
// ... (End of helper functions) ...

fn main() {
    let filename = "test_file.bin";
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    println!("Connected!");

    // 1. Handshake
    send_message(
        &mut stream,
        &Message::Hello {
            client_id: "RustUser".to_string(),
        },
    );
    read_message(&mut stream); // Read Welcome

    // 2. Init Upload
    let file_size = std::fs::metadata(filename).unwrap().len();
    // Math: Calculate total chunks (ceiling division)
    let total_chunks = (file_size + CHUNK_SIZE - 1) / CHUNK_SIZE;

    println!(
        "Sending {} ({} bytes) in {} chunks...",
        filename, file_size, total_chunks
    );

    send_message(
        &mut stream,
        &Message::InitUpload {
            file_name: filename.to_string(),
            total_size: file_size,
        },
    );
    read_message(&mut stream); // Read InitAck

    // 3. MAIN LOOP: Send every chunk
    for i in 0..total_chunks {
        let chunk_data = read_chunk(filename, i);
        let size = chunk_data.len();

        println!("Uploading Chunk #{} ({} bytes)...", i, size);

        // A. Send Meta
        send_message(
            &mut stream,
            &Message::ChunkMeta {
                chunk_index: i,
                size,
            },
        );

        // B. Send Data
        stream.write_all(&chunk_data).unwrap();

        // C. Wait for Ack
        if let Message::ChunkAck { chunk_index } = read_message(&mut stream) {
            println!("Chunk #{} Ack received.", chunk_index);
        }
    }

    println!("Transfer Complete!");
}
