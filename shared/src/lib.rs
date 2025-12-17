use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    // Handshake
    Hello {
        client_id: String,
    },
    Welcome {
        session_id: String,
    },

    // New: Preparing to upload
    InitUpload {
        file_name: String,
        total_size: u64,
    },

    // New: Server says "Ready to receive"
    InitAck {
        chunk_size: u64,
    },

    // New: The Header for a chunk
    ChunkMeta {
        chunk_index: u64,
        size: usize, // How many bytes follow this message?
    },

    // New: Server confirms receipt
    ChunkAck {
        chunk_index: u64,
    },
}
