// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::{Engine, EngineInput};
use cdqn::executor::Executor;
use cdqn::projector::{Rule, RuleBasedProjector};
use cdqn::refinement::RefinementEngine;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Chronosa Verbose Shutdown Test ---");
    let log_path = PathBuf::from("shutdown_test.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. Use a simple projector for this test.
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
    let shared_state = engine.state.clone();

    // 3. Spawn all components.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state, input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // 4. Send a single CDU to ensure the system processes something.
    println!("\n[MAIN] Sending a single test CDU...");
    let observation = Cdu::new(b"test".to_vec(), "observation", vec![]);
    input_sender.send(EngineInput::Cdu(observation)).unwrap();

    // Wait for the CDU to be fully processed through the entire loop.
    thread::sleep(Duration::from_secs(1));

    // 5. Initiate the graceful shutdown.
    println!("\n[MAIN] Sending shutdown signal...");
    if input_sender.send(EngineInput::Shutdown).is_err() {
        println!("[MAIN] Engine channel was already closed.");
    }

    // 6. Wait for all threads to terminate.
    println!("[MAIN] Waiting for Engine thread to join...");
    engine_handle.join().unwrap();
    println!("[MAIN] Engine thread joined.");

    println!("[MAIN] Waiting for Executor thread to join...");
    executor_handle.join().unwrap();
    println!("[MAIN] Executor thread joined.");

    // FIX: We can now safely join the refinement handle because it is guaranteed to terminate.
    println!("[MAIN] Waiting for Refinement thread to join...");
    refinement_handle.join().unwrap();
    println!("[MAIN] Refinement thread joined.");

    println!("\nSession complete. Shutdown was successful.");
}
