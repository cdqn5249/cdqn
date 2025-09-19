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
    // In the next milestone, this is where we will read KDUs from the stream.
    println!(
        "[handle_client] Connection from {} handled and closed.",
        stream.peer_addr().unwrap()
    );
}
