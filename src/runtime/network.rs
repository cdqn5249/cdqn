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
}
