// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::chronosa::{Chronosa, Command};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- CDQN Janitor Demo ---");
    let log_path = PathBuf::from("chronosa_events.cdqn");

    // 1. Create the Chronosa instance. It loads past events and starts the Janitor.
    let mut chronosa = Chronosa::new(log_path);
    println!(
        "Initialized. State has {} events.",
        chronosa.state().len()
    );

    // 2. Send a command. This is fast and only marks the log as "dirty".
    println!("Client: Sending observation (buffered)...");
    chronosa.process_command(Command::RecordObservation {
        payload: b"see enemy".to_vec(),
    });
    println!("  -> State updated instantly.");

    // 3. The user can force a save at a critical moment.
    println!("Client: Forcing a save before continuing...");
    chronosa.process_command(Command::Commit);
    println!("  -> Data is now durable.");

    // 4. Gracefully shut down the agent and its background thread.
    chronosa.shutdown();
    println!("\nSession complete.");
}
