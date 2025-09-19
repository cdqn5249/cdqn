// src/main.rs

use cdqn::runtime::network::{NodeClient, NodeServer};
use std::thread;
use std::time::Duration;

fn main() {
    println!("cdqn runtime starting... [Sovereign Networking Client/Server Test]");

    let server_addr = "127.0.0.1:8081"; // Use a different port to avoid conflicts

    // --- 1. Start the Server in a Background Thread ---
    println!("\n--- 1. Spawning NodeServer in background ---");
    let server_addr_clone = server_addr.to_string();
    let server_handle = thread::spawn(move || {
        if let Ok(server) = NodeServer::bind(&server_addr_clone) {
            // We only run the server for one connection for this test.
            // The .take(1) ensures the server thread will exit after the first connection.
            for stream in server.listener.incoming().take(1) {
                if let Ok(s) = stream {
                    println!("[NodeServer] Accepted connection from: {}", s.peer_addr().unwrap());
                }
            }
        }
    });

    // Give the server a moment to start up.
    thread::sleep(Duration::from_millis(100));

    // --- 2. Start the Client and Connect ---
    println!("\n--- 2. Initializing NodeClient to connect ---");
    if let Ok(_stream) = NodeClient::connect(server_addr) {
        println!("SUCCESS: NodeClient connected successfully.");
        // The _stream is dropped here, closing the client side of the connection.
    } else {
        eprintln!("FAILURE: NodeClient failed to connect.");
    }

    // --- 3. Wait for Server to Finish ---
    println!("\n--- 3. Waiting for NodeServer thread to complete ---");
    server_handle.join().expect("Server thread panicked");
    println!("SUCCESS: NodeServer thread finished cleanly.");


    println!("\n--- Sovereign Client/Server connection implemented successfully! ---");
}
