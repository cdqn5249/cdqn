// src/main.rs

use cdqn::runtime::Runtime;
use std::fs;
use std::path::Path;

fn main() {
    println!("cdqn runtime starting... [Phase 4, Milestone 2 - Threaded I/O]");

    // --- 1. Create the Runtime ---
    // This now spawns the persistence thread.
    let mut runtime = Runtime::new();
    println!("\n--- 1. Runtime Created ---");

    // --- 2. Run the Runtime ---
    // This will execute our test simulation.
    runtime.run();
    println!("\n--- 2. Runtime Finished ---");

    // --- 3. Shutdown the Runtime ---
    // This sends the shutdown command and waits for the thread to exit.
    runtime.shutdown();
    println!("\n--- 3. Runtime Shutdown Complete ---");

    // --- 4. Cleanup ---
    fs::remove_dir_all(Path::new("./cdqn_runtime_db")).expect("Failed to clean up DB directory");
    println!("\n--- 4. Cleanup Complete ---");

    println!("\n--- Threaded I/O implemented successfully! ---");
}
