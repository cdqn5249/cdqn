// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::engine::{Engine, EngineInput};
use cdqn::executor::Executor;
use cdqn::projector::RuleBasedProjector; // Using a simple projector for this setup
use cdqn::worlds; // Import the new worlds module
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- cdqnRuntime: Genesis ---");
    let log_path = PathBuf::from("genesis.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. For this setup, we only need a minimal projector.
    let projector = RuleBasedProjector::new(vec![]);

    // 2. Create the core components.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();

    // 3. Spawn the essential threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // 4. The Runtime's primary job: Use the 'worlds' toolbox to create Rworld.
    worlds::create_rworld(&input_sender);

    // Allow the seeding to be processed.
    thread::sleep(Duration::from_millis(200));

    // 5. Initiate a graceful shutdown.
    println!("\n[Runtime] Genesis complete. Shutting down components.");
    if input_sender.send(EngineInput::Shutdown).is_err() {
        println!("[Runtime] Engine channel was already closed.");
    }

    // 6. Wait for the threads to terminate.
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();

    println!("\nSession complete. Rworld has been born.");
}
