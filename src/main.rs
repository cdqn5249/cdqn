// src/main.rs

use cdqn::kernel::{License, Metadata, KDU};
use cdqn::runtime::orchestrator::Orchestrator;
use cdqn::runtime::test_entities::{Pinger, Ponger};
use std::fs;
use std::path::Path;

fn main() {
    println!("cdqn runtime starting... [Unified Runtime - Local Lifecycle]");

    // --- 1. Setup ---
    let mut orchestrator = Orchestrator::new();
    let pinger_fqei = "pinger@test".to_string();
    let ponger_fqei = "ponger@test".to_string();

    orchestrator
        .processor_mut()
        .register::<Pinger>(pinger_fqei.clone(), 0);
    orchestrator
        .processor_mut()
        .register::<Ponger>(ponger_fqei.clone(), 0);
    println!("\n--- 1. Entities Registered ---");

    // --- 2. Create and Route Initial KDU ---
    let dummy_metadata = Metadata {
        metadata_hash: String::new(),
        unisphere_coordinates: vec![],
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
        metadata: dummy_metadata,
        data_payload: b"ping".to_vec(),
    };
    orchestrator.route_initial_kdu(&ponger_fqei, initial_ping);
    println!("\n--- 2. Initial Ping KDU Routed & Journaled ---");

    // --- 3. Run the Orchestrator ---
    orchestrator.run();

    // --- 4. Shutdown ---
    orchestrator.shutdown();

    // --- 5. Cleanup ---
    fs::remove_dir_all(Path::new("./cdqn_runtime_db")).expect("Failed to clean up DB directory");
    println!("\n--- 5. Cleanup Complete ---");

    println!("\n--- Unified Runtime local lifecycle implemented successfully! ---");
}
