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
    InitAck {
        chunk_size: u64,
    },

    // UPDATED: Now includes the hash!
    ChunkMeta {
        chunk_index: u64,
        size: usize,
        hash: String, // <--- NEW FIELD
    },

    ChunkAck {
        chunk_index: u64,
    },

    ChunkNack {
        chunk_index: u64,
    },
    Complete {
        file_name: String,
    },
}
