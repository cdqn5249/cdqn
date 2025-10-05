// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::Engine;
use cdqn::executor::Executor;
use cdqn::projector::{Rule, RuleBasedProjector};
use cdqn::refinement::RefinementEngine;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- CDQN Projector Demo ---");
    let log_path = PathBuf::from("projector_demo.cdqn");
    let _ = std::fs::remove_file(&log_path); // Clean up from previous runs

    // 1. Define the agent's "logic" as a set of formal rules.
    let rules = vec![
        // Rule 1: If we see an enemy, and haven't already decided to attack,
        // then create a command to attack.
        Rule {
            predicate: Box::new(|state, input| {
                input.name.contains(".observation.")
                    && input.payload == b"see enemy"
                    && state
                        .find_last_by_subtype("command.schedule_task")
                        .is_none()
            }),
            mapper: Box::new(|input| {
                println!("Projector: Saw enemy, creating attack command.");
                vec![Cdu::new(
                    b"{\"task\":\"attack\"}".to_vec(),
                    "command.schedule_task",
                    vec![input.name.clone()],
                )]
            }),
        },
        // Add more rules here in the future...
    ];

    // 2. Create the Projector with our defined rules.
    let projector = RuleBasedProjector::new(rules);

    // 3. Create the Engine with our Projector.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();
    let shared_state = engine.state.clone(); // Get a handle to the shared state.

    // 4. Spawn the Executor, Engine, and the new RefinementEngine on background threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state, input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // 5. The main thread acts as the "world", feeding an observation to the Engine.
    println!("Client: Simulating an observation...");
    let observation = Cdu::new(b"see enemy".to_vec(), "observation", vec![]);
    input_sender.send(observation).unwrap();

    // Give the system a moment to process.
    thread::sleep(Duration::from_millis(100));

    // 6. Gracefully shut down the system.
    println!("Client: Shutting down.");
    drop(input_sender);
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    // Although the refinement thread will exit when the sender drops,
    // it's good practice to join its handle as well.
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}
