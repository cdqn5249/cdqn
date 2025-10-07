// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::{Engine, EngineInput};
use cdqn::executor::Executor;
use cdqn::projector::{Rule, RuleBasedProjector};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- CASCADING TEST 3: Engine <-> Executor Feedback Loop ---");
    let log_path = PathBuf::from("test3_feedback_loop.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. Use a simple projector.
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

    // 2. Create the core components.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();

    // 3. Spawn the Engine and Executor, now with the feedback loop re-enabled.
    println!("[MAIN] Spawning Executor thread with feedback sender...");
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    println!("[MAIN] Spawning Engine thread...");
    let engine_handle = thread::spawn(move || engine.run());
    println!("[MAIN] All threads spawned.");

    // 4. Send a single CDU.
    println!("\n[MAIN] Sending a single test CDU...");
    let observation = Cdu::new(b"test".to_vec(), "observation", vec![]);
    input_sender.send(EngineInput::Cdu(observation)).unwrap();

    // Give the system time to complete the full loop.
    thread::sleep(Duration::from_millis(500));

    // 5. Initiate the graceful shutdown.
    println!("\n[MAIN] Sending shutdown signal...");
    if input_sender.send(EngineInput::Shutdown).is_err() {
        println!("[MAIN] Engine channel was already closed.");
    }

    // 6. Wait for the threads to terminate.
    println!("[MAIN] Waiting for Engine thread to join...");
    engine_handle.join().unwrap();
    println!("[MAIN] Engine thread joined successfully.");

    println!("[MAIN] Waiting for Executor thread to join...");
    executor_handle.join().unwrap();
    println!("[MAIN] Executor thread joined successfully.");

    println!("\n--- TEST 3 PASSED: Feedback loop shutdown is clean. ---");
}
