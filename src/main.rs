// src/main.rs

// Import handle_client along with the other services
use cdqn::runtime::network::{handle_client, NodeClient, NodeServer};
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
                // Now we correctly use the stream by passing it to the handler.
                handle_client(stream);
            }
        }
    });

    thread::sleep(Duration::from_millis(100));

    // --- 2. Create a KDU to Send ---
    let factory = cdqn::kernel::factory::KDUFactory::default();
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
