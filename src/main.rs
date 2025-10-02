// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::core::ChronosaCore;
use cdqn::storage::{load_core, save_core};
use std::path::Path;

fn main() {
    // --- FIX: File path now uses the .cdqn extension ---
    let core_path = Path::new("chronosa_core.cdqn");

    // Try to load an existing core, or create a new one.
    let core = match load_core(core_path) {
        Ok(loaded_core) => {
            println!(
                "Successfully loaded existing Chronosa Core with {} events.",
                loaded_core.len()
            );
            loaded_core
        }
        Err(_) => {
            println!("No existing core found. Initializing a new Chronosa Core...");
            let core = ChronosaCore::new();
            let (core, _) = core.record(b"Chronosa instance created.".to_vec(), "genesis");
            core
        }
    };

    // Record a new event every time the program runs.
    println!("Recording a new session event...");
    let (core, session_cdu) = core.record(b"New session started.".to_vec(), "session");
    println!("  -> Recorded CDU: {}", session_cdu.name);

    // Save the final state.
    println!("Saving Chronosa Core with {} total events...", core.len());
    save_core(&core, core_path).expect("Failed to save core.");

    println!("\nChronosa Core session complete.");
}
