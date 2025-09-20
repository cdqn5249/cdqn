// src/main.rs

use cdqn::kernel::{License, Metadata, KDU}; // Import the necessary structs
use cdqn::runtime::scheduler::EntityScheduler;
use cdqn::runtime::test_entities::{Pinger, Ponger};

fn main() {
    println!("cdqn runtime starting... [Sovereign Actor Model - Ping Pong]");

    // --- 1. Setup ---
    let mut scheduler = EntityScheduler::new();
    let pinger_fqei = "pinger@test".to_string();
    let ponger_fqei = "ponger@test".to_string();

    scheduler.register::<Pinger>(pinger_fqei.clone(), 0);
    scheduler.register::<Ponger>(ponger_fqei.clone(), 0);
    println!("\n--- 1. Entities Registered ---");
    println!("Registered: {}", pinger_fqei);
    println!("Registered: {}", ponger_fqei);

    // --- 2. Create the initial "ping" KDU ---
    // This now creates a valid, safe, dummy Metadata object.
    let dummy_metadata = Metadata {
        metadata_hash: String::new(),
        unisphere_coordinates: vec![], // Empty vec is valid
        license: License {
            license_id: String::new(),
            licensor_fqei: String::new(),
            custom_terms_hash: None,
        },
        causal_link: None,
    };

    let initial_ping = KDU {
        kdu_spec_version: "2.1.0".to_string(),
        kdu_id: "ping-1".to_string(),
        content_hash: String::new(),
        originator_fqei: pinger_fqei.clone(),
        originator_signature: vec![],
        timestamp_utc: String::new(),
        kdu_type: "Generic".to_string(),
        metadata: dummy_metadata, // Use the valid dummy metadata
        data_payload: b"ping".to_vec(),
    };
    println!("\n--- 2. Created Initial Ping KDU ---");

    // --- 3. Start the Simulation ---
    println!("\n--- 3. Starting Simulation ---");
    scheduler.route(&ponger_fqei, initial_ping);

    println!("\n--- Turn 1 ---");
    scheduler.run_turn();

    println!("\n--- Turn 2 ---");
    scheduler.run_turn();

    println!("\n--- Turn 3 ---");
    scheduler.run_turn();

    println!("\n--- Sovereign EntityScheduler implemented successfully! ---");
}
