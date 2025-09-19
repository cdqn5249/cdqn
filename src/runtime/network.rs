// src/runtime/network.rs

use std::io;
use std::net::{Incoming, TcpListener, TcpStream}; // Import Incoming
use std::thread;

// The NodeServer is responsible for listening for incoming connections.
pub struct NodeServer {
    listener: TcpListener,
}

impl NodeServer {
    // Binds to a local address and starts listening for connections.
    pub fn bind(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        println!("[NodeServer] Listening on {}", addr);
        Ok(NodeServer { listener })
    }

    // --- NEW PUBLIC METHOD ADDED HERE ---
    // This provides a safe, public way to access the incoming connection iterator.
    pub fn incoming(&self) -> Incoming<'_> {
        self.listener.incoming()
    }

    // The main server loop. It accepts connections and handles them.
    pub fn run(self) {
        // accept connections and process them serially
        for stream in self.incoming() { // Now uses the public method
            match stream {
                Ok(stream) => {
                    println!(
                        "[NodeServer] New connection: {}",
                        stream.peer_addr().unwrap()
                    );
                    thread::spawn(move || {
                        handle_client(stream)
                    });
                }
                Err(e) => {
                    println!("[NodeServer] Error: {}", e);
                }
            }
        }
    }
}

// A simple handler function for a new client connection.
fn handle_client(stream: TcpStream) {
    println!(
        "[handle_client] Connection from {} handled and closed.",
        stream.peer_addr().unwrap()
    );
}

// The NodeClient is responsible for initiating connections to a NodeServer.
pub struct NodeClient;

impl NodeClient {
    // Attempts to connect to a given server address.
    pub fn connect(addr: &str) -> io::Result<TcpStream> {
        println!("[NodeClient] Attempting to connect to {}", addr);
        let stream = TcpStream::connect(addr)?;
        println!("[NodeClient] Connection established with {}", addr);
        Ok(stream)
    }
}
