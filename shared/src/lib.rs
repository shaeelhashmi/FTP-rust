use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello {
        client_id: String,
    },
    Welcome {
        session_id: String,
    },

    InitUpload {
        file_name: String,
        total_size: u64,
    },

    // UPDATED: Server now gives a specific Upload ID
    InitAck {
        chunk_size: u64,
        upload_id: String, // <--- NEW
    },

    // UPDATED: Worker must specify which upload this belongs to
    ChunkMeta {
        upload_id: String, // <--- NEW
        chunk_index: u64,
        size: usize,
        hash: String,
    },

    ChunkAck {
        chunk_index: u64,
    },
    ChunkNack {
        chunk_index: u64,
    },

    // UPDATED: Tell server which folder to merge
    Complete {
        upload_id: String, // <--- NEW
        file_name: String,
    },
}
