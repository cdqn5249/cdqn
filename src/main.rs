// src/main.rs

use cdqn::runtime::Runtime;

fn main() {
    println!("cdqn runtime starting... [Phase 4, Milestone 1 - Orchestrator Init]");

    // --- 1. Create the Runtime ---
    // This single line now encapsulates the creation of all our core services.
    let mut runtime = Runtime::new();
    println!("\n--- 1. Runtime Created ---");
    println!("SUCCESS: Runtime and its services have been initialized.");

    // --- 2. Run the Runtime ---
    // This will eventually be the main blocking call that runs the node.
    runtime.run();
    println!("\n--- 2. Runtime Finished ---");


    println!("\n--- Runtime Orchestrator structure implemented successfully! ---");
}
