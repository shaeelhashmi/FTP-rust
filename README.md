<p align="center">
<pre>
 ███████████                                ███████████ ████                          
░░███░░░░░███                              ░░███░░░░░░█░░███                          
 ░███    ░███  ██████   ████████   ██████   ░███   █ ░  ░███   ██████  █████ ███ █████
 ░██████████  ░░░░░███ ░░███░░███ ░░░░░███  ░███████    ░███  ███░░███░░███ ░███░░███ 
 ░███░░░░░░    ███████  ░███ ░░░   ███████  ░███░░░█    ░███ ░███ ░███ ░███ ░███ ░███ 
 ░███         ███░░███  ░███      ███░░███  ░███  ░     ░███ ░███ ░███ ░░███████████  
 █████       ░░████████ █████    ░░████████ █████       █████░░██████   ░░████░████   
░░░░░         ░░░░░░░░ ░░░░░      ░░░░░░░░ ░░░░░       ░░░░░  ░░░░░░     ░░░░ ░░░░    
</pre>
</p>

**ParaFlow** is a robust, concurrent file transfer solution engineered in Rust. It utilizes a multi-threaded architecture to split files into data chunks and transmit them in parallel across multiple TCP streams, effectively maximizing bandwidth utilization. The system prioritizes data integrity and fault tolerance through cryptographic verification and automatic error correction protocols.

## Key Features

* **Concurrency & Performance:** Implements a thread-pool architecture to facilitate the parallel transmission of file chunks, significantly reducing transfer times for large datasets.
* **Cryptographic Integrity:** Enforces SHA-256 hash verification for every data packet. Corrupted chunks are automatically detected and re-queued for transmission.
* **Challenge-Response Authentication:** Secures the control channel using a Salted SHA-256 challenge-response mechanism, preventing replay attacks and ensuring zero-knowledge password verification.
* **Session Isolation:** Utilizes UUIDv4-based session management to isolate concurrent uploads, preventing data collision in multi-user environments.
* **Robust Error Handling:** Features a custom binary/JSON hybrid protocol with defined error states for graceful handling of authentication failures, file type restrictions, and network disconnects.

## Installation

Ensure the Rust toolchain (Cargo) is installed on your system.

```bash
# Clone the repository
git clone [https://github.com/yourusername/paraflow.git](https://github.com/yourusername/paraflow.git)
cd paraflow

# Compile the project in release mode for optimal performance
cargo build --release

```

## Usage Guidelines

The system consists of two binaries: `server` (receiver) and `client` (sender).

### Server Configuration

The server initializes a listener for incoming TCP connections and manages the reassembly of file chunks.

```bash
# Start server on default port (7878)
cargo run -p server

# Start server on a specific port
cargo run -p server -- --port 9000

```

### Client Operations

The client handles file segmentation, hashing, and parallel distribution to worker threads.

```bash
# Standard upload
cargo run -p client -- upload --file data.bin

# High-performance upload (8 threads) to a remote host
cargo run -p client -- upload --file video.mp4 --host 192.168.1.50 --port 9000 --threads 8

# Authenticated upload (Default secret: 'secret123')
cargo run -p client -- upload --file sensitive.doc --secret <password>

```

## Architectural Overview

1. **Handshake & Authentication:** The client initiates a connection. The server responds with a cryptographic salt. The client computes the salted hash of the password and returns it for verification.
2. **Session Negotiation:** Upon successful authentication, the server generates a unique Session ID (UUID) and allocates a dedicated staging directory.
3. **Parallel Distribution:** The client splits the source file into 4MB chunks. These tasks are distributed via a mutex-locked job queue to a pool of worker threads.
4. **Integrity Verification:** The server independently calculates the SHA-256 hash of incoming data.
* **ACK:** Hash match. The chunk is committed to disk.
* **NACK:** Hash mismatch. The server rejects the chunk, and the client re-queues it for retry.


5. **Final Assembly:** Once all chunks are successfully acknowledged, the server merges the segments into the final artifact and cleans up the staging area.

## Security Policies

* **Authentication:** The default configuration uses the password `secret123`. In a production environment, this should be replaced with a secure credential store.
* **File Restrictions:** The server enforces a security policy rejecting executable file formats (e.g., `.exe`, `.sh`) to mitigate remote code execution risks.

---


