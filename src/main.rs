// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::runtime::network::NodeClient;
use cdqn::runtime::orchestrator::Orchestrator;
use cdqn::runtime::test_entities::{Pinger, Ponger};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).cloned().unwrap_or_default();

    if mode == "--server" {
        run_server();
    } else if mode == "--client" {
        run_client();
    } else {
        println!("Usage: cdqn [--server | --client]");
    }
}

// --- SERVER MODE ---
fn run_server() {
    println!("--- Starting in SERVER mode ---");
    let mut orchestrator = Orchestrator::new("127.0.0.1:8082");
    let ponger_fqei = "ponger@test".to_string();
    orchestrator
        .processor_mut()
        .register::<Ponger>(ponger_fqei, 0);
    
    // The server runs forever, waiting for connections.
    orchestrator.run();
    
    // Cleanup code (won't be reached unless you Ctrl+C and handle shutdown)
    orchestrator.shutdown();
    fs::remove_dir_all(Path::new("./cdqn_runtime_db")).ok();
}

// --- CLIENT MODE ---
fn run_client() {
    println!("--- Starting in CLIENT mode ---");
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let pinger_fqei = "pinger@test".to_string();
    
    let payload_bytes = bincode::serialize("ping").unwrap();
    let initial_ping = factory.create_kdu(
        &originator_keypair,
        pinger_fqei,
        "InitialPing".to_string(),
        &payload_bytes,
    );

    println!("Client sending initial ping with ID: {}", initial_ping.kdu_id);
    if let Ok(mut stream) = NodeClient::connect("127.0.0.1:8082") {
        NodeClient::send_kdu(&mut stream, &initial_ping).expect("Client failed to send KDU");
        println!("Ping sent successfully.");
    } else {
        eprintln!("Client could not connect to server.");
    }
}
