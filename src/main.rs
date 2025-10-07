// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::{Engine, EngineInput};
use cdqn::projector::{Rule, RuleBasedProjector};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- CASCADING TEST 1: Engine in Isolation ---");
    let log_path = PathBuf::from("test1_engine_isolation.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. Use a simple projector that creates one command, so we can test the child-thread logic.
    let rules = vec![Rule {
        predicate: Box::new(|_, input| input.name.contains("observation")),
        mapper: Box::new(|input| {
            vec![Cdu::new(
                b"test command".to_vec(),
                "command.test",
                vec![input.name.clone()],
            )]
        }),
    }];
    let projector = RuleBasedProjector::new(rules);

    // 2. Create the Engine. We ignore the command_receiver as there is no Executor.
    let (engine, _command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();

    // 3. Spawn ONLY the Engine thread.
    println!("[MAIN] Spawning Engine thread...");
    let engine_handle = thread::spawn(move || engine.run());
    println!("[MAIN] Engine thread spawned.");

    // 4. Send a single CDU.
    println!("\n[MAIN] Sending a single test CDU...");
    let observation = Cdu::new(b"test".to_vec(), "observation", vec![]);
    input_sender.send(EngineInput::Cdu(observation)).unwrap();

    // Give it a moment to process.
    thread::sleep(Duration::from_millis(200));

    // 5. Initiate the graceful shutdown.
    println!("\n[MAIN] Sending shutdown signal...");
    if input_sender.send(EngineInput::Shutdown).is_err() {
        println!("[MAIN] Engine channel was already closed.");
    }

    // 6. Wait for the Engine thread to terminate.
    println!("[MAIN] Waiting for Engine thread to join...");
    engine_handle.join().unwrap();
    println!("[MAIN] Engine thread joined successfully.");

    println!("\n--- TEST 1 PASSED: Engine shutdown is clean in isolation. ---");
}
