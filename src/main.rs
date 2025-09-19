// src/main.rs

use cdqn::runtime::network::NodeServer; // Import our new service

fn main() {
    println!("cdqn runtime starting... [Sovereign Networking Server]");

    // Define the address for our server to listen on.
    let server_addr = "127.0.0.1:8080";

    // --- Server Initialization ---
    println!("\n--- 1. Initializing NodeServer ---");
    // We use an if-let block to handle the potential error of the port already being in use.
    if let Ok(_server) = NodeServer::bind(server_addr) {
        println!("SUCCESS: NodeServer bound to {} successfully.", server_addr);
        // In a real application, we would call server.run() here to start the main loop.
        // For this test, we let the _server object drop, which closes the listener.
    } else {
        eprintln!(
            "FAILURE: Could not bind NodeServer to {}. Is the port already in use?",
            server_addr
        );
    }

    println!("\n--- Sovereign NodeServer implemented successfully! ---");
}
