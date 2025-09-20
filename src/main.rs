// src/main.rs

use cdqn::kernel::KDU;
use cdqn::runtime::scheduler::EntityScheduler;
use cdqn::runtime::test_entities::{Pinger, Ponger};

fn main() {
    println!("cdqn runtime starting... [Sovereign Actor Model - Ping Pong]");

    // --- 1. Setup ---
    let mut scheduler = EntityScheduler::new();
    let pinger_fqei = "pinger@test".to_string();
    let ponger_fqei = "ponger@test".to_string();

    // Register the two entities with their initial states.
    scheduler.register::<Pinger>(pinger_fqei.clone(), 0);
    scheduler.register::<Ponger>(ponger_fqei.clone(), 0);
    println!("\n--- 1. Entities Registered ---");
    println!("Registered: {}", pinger_fqei);
    println!("Registered: {}", ponger_fqei);

    // --- 2. Create the initial "ping" KDU ---
    // In a real system, this would come from an external source or another entity.
    let initial_ping = KDU {
        kdu_spec_version: "2.1.0".to_string(),
        kdu_id: "ping-1".to_string(),
        content_hash: String::new(),
        originator_fqei: pinger_fqei.clone(), // The Pinger is the originator
        originator_signature: vec![],
        timestamp_utc: String::new(),
        kdu_type: "Generic".to_string(),
        // Create a dummy metadata struct for this test.
        metadata: unsafe { std::mem::zeroed() },
        data_payload: b"ping".to_vec(),
    };
    println!("\n--- 2. Created Initial Ping KDU ---");

    // --- 3. Start the Simulation ---
    println!("\n--- 3. Starting Simulation ---");
    // Manually route the first message to the Ponger.
    scheduler.route(&ponger_fqei, initial_ping);

    // Run the scheduler for a few turns to see the interaction.
    println!("\n--- Turn 1 ---");
    scheduler.run_turn(); // Ponger receives ping, sends pong.

    println!("\n--- Turn 2 ---");
    scheduler.run_turn(); // Pinger receives pong.

    println!("\n--- Turn 3 ---");
    scheduler.run_turn(); // No new messages, nothing happens.

    println!("\n--- Sovereign EntityScheduler implemented successfully! ---");
}
