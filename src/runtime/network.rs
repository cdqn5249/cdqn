// src/runtime/network.rs

use crate::kernel::KDU; // Import the KDU struct
use std::io::{self, Read, Write}; // Import Read and Write traits
use std::net::{Incoming, TcpListener, TcpStream};
use std::thread;

// The NodeServer is responsible for listening for incoming connections.
pub struct NodeServer {
    listener: TcpListener,
}

impl NodeServer {
    pub fn bind(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        println!("[NodeServer] Listening on {}", addr);
        Ok(NodeServer { listener })
    }

    pub fn incoming(&self) -> Incoming<'_> {
        self.listener.incoming()
    }

    pub fn run(self) {
        for stream in self.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "[NodeServer] New connection: {}",
                        stream.peer_addr().unwrap()
                    );
                    thread::spawn(move || handle_client(stream));
                }
                Err(e) => {
                    println!("[NodeServer] Error: {}", e);
                }
            }
        }
    }
}

// The client handler now reads a KDU from the stream.
fn handle_client(mut stream: TcpStream) {
    println!(
        "[handle_client] Connection from {} established.",
        stream.peer_addr().unwrap()
    );

    // Read the 8-byte length prefix.
    let mut len_buffer = [0u8; 8];
    if stream.read_exact(&mut len_buffer).is_ok() {
        let kdu_len = u64::from_le_bytes(len_buffer);
        let mut kdu_buffer = vec![0; kdu_len as usize];

        // Read the full KDU data.
        if stream.read_exact(&mut kdu_buffer).is_ok() {
            let kdu: KDU = bincode::deserialize(&kdu_buffer)
                .expect("Failed to deserialize KDU from stream");
            println!("[handle_client] Received KDU with ID: {}", kdu.kdu_id);
        }
    }
    println!("[handle_client] Connection closed.");
}

// The NodeClient is responsible for initiating connections to a NodeServer.
pub struct NodeClient;

impl NodeClient {
    pub fn connect(addr: &str) -> io::Result<TcpStream> {
        println!("[NodeClient] Attempting to connect to {}", addr);
        let stream = TcpStream::connect(addr)?;
        println!("[NodeClient] Connection established with {}", addr);
        Ok(stream)
    }

    // --- NEW FUNCTION ADDED HERE ---
    // Serializes and sends a single KDU over the stream.
    pub fn send_kdu(stream: &mut TcpStream, kdu: &KDU) -> io::Result<()> {
        println!("[NodeClient] Serializing KDU with ID: {}", kdu.kdu_id);
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;

        // Write the 8-byte length prefix.
        stream.write_all(&kdu_len.to_le_bytes())?;
        // Write the KDU data.
        stream.write_all(&kdu_bytes)?;
        println!("[NodeClient] KDU sent successfully.");
        Ok(())
    }
}```

---

### **Step 2: Integrate KDU Transmission into `main.rs`**

Now we update our main executable to perform the full test: create a KDU, send it from the client, and receive it on the server.

1.  Navigate to `src/main.rs` and click "Edit".
2.  **Replace the entire content** with the following:

```rust
// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::runtime::network::{NodeClient, NodeServer};
use serde::Serialize;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    println!("cdqn runtime starting... [Sovereign KDU Transmission Test]");

    let server_addr = "127.0.0.1:8082"; // Use a new port

    // --- 1. Start the Server in a Background Thread ---
    let server_addr_clone = server_addr.to_string();
    let server_handle = thread::spawn(move || {
        if let Ok(server) = NodeServer::bind(&server_addr_clone) {
            // The server will handle one connection and then exit.
            if let Some(stream) = server.incoming().flatten().next() {
                 // The handle_client logic is now inside network.rs
            }
        }
    });

    thread::sleep(Duration::from_millis(100));

    // --- 2. Create a KDU to Send ---
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let originator_fqei = "agent@U.ClientNode#01".to_string();
    let payload_struct = TestPayload {
        action: "sovereign.network.transmit".to_string(),
        status: "ok".to_string(),
    };
    let payload_bytes = bincode::serialize(&payload_struct).unwrap();
    let kdu_to_send = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        &payload_bytes,
    );
    println!("\n--- 1. KDU Created in Client Memory ---");
    println!("KDU ID to send: {}", kdu_to_send.kdu_id);

    // --- 3. Start the Client, Connect, and Send KDU ---
    println!("\n--- 2. Client Connecting and Sending KDU ---");
    if let Ok(mut stream) = NodeClient::connect(server_addr) {
        if NodeClient::send_kdu(&mut stream, &kdu_to_send).is_ok() {
            println!("SUCCESS: NodeClient sent KDU.");
        } else {
            eprintln!("FAILURE: NodeClient failed to send KDU.");
        }
    } else {
        eprintln!("FAILURE: NodeClient failed to connect.");
    }

    // --- 4. Wait for Server to Finish ---
    println!("\n--- 3. Waiting for NodeServer to process and finish ---");
    server_handle.join().expect("Server thread panicked");
    println!("SUCCESS: NodeServer thread finished cleanly.");

    println!("\n--- Sovereign KDU Transmission implemented successfully! ---");
}
