// src/runtime/network.rs

use std::io;
use std::net::{TcpListener, TcpStream};
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

    // The main server loop. It accepts connections and handles them.
    pub fn run(self) {
        // accept connections and process them serially
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "[NodeServer] New connection: {}",
                        stream.peer_addr().unwrap()
                    );
                    // For now, we just print a message. In the future, we'll handle KDU exchange.
                    thread::spawn(move || {
                        // connection succeeded
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
    // For this milestone, we do nothing but acknowledge the connection.
    println!(
        "[handle_client] Connection from {} handled and closed.",
        stream.peer_addr().unwrap()
    );
}

// --- NEW CODE ADDED BELOW ---

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
