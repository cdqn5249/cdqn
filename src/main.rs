// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::Engine;
use cdqn::executor::Executor;
use cdqn::reasoning::{PrimeElement, ReasoningProjector, SemiAxiom};
use cdqn::refinement::RefinementEngine;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Chronosa Learning Cycle Demo ---");
    let log_path = PathBuf::from("learning_cycle_demo.cdqn");
    let _ = std::fs::remove_file(&log_path); // Clean up from previous runs

    // 1. Use the intelligent ReasoningProjector.
    let projector = ReasoningProjector::new();

    // 2. Create the core components.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();
    let shared_state = engine.state.clone(); // Get a handle to the shared state.

    // 3. Spawn all background threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state.clone(), input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // --- Seeding the Initial State ---
    println!("\n[SETUP] Seeding initial knowledge...");
    // A basic fact: the user is present.
    let pe1 = PrimeElement::new(
        "pe-user-present".to_string(),
        "uworld".to_string(),
        1.0,
        "The user is present".to_string(),
        "Cannot be decomposed".to_string(),
    );
    input_sender.send(pe1.to_cdu()).unwrap();

    // A basic rule: if the user is present, greet them.
    let axiom1 = SemiAxiom::new(
        "axiom-greet-user".to_string(),
        "uworld".to_string(),
        vec!["pe-user-present".to_string()],
        "If user is present, greet them".to_string(),
    );
    input_sender.send(axiom1.to_cdu()).unwrap();
    thread::sleep(Duration::from_millis(100)); // Let state settle.

    // --- The Mistake ---
    println!("\n[SCENARIO 1] Simulating an emergency. Chronosa should make a mistake.");
    let emergency_input = Cdu::new(
        b"User shouts for help".to_vec(),
        "observation.emergency",
        vec![],
    );
    input_sender.send(emergency_input.clone()).unwrap();
    thread::sleep(Duration::from_millis(200)); // Allow time for the command to be processed.

    // --- The Feedback ---
    println!("\n[FEEDBACK] The world provides negative feedback for the mistake.");
    // We need to find the result of the greeting command to link our feedback to it.
    // In a real app, this would be more robust.
    let last_result = {
        // FIX: Use the shared_state handle, not the moved 'engine'.
        let state = shared_state.read().unwrap();
        state.find_last_by_subtype("result.task_completed").cloned()
    };

    if let Some(result) = last_result {
        println!("Feedback linked to result: {}", result.name);
        let feedback = Cdu::new(
            b"Greeting during emergency is bad".to_vec(),
            "feedback.reputation.negative.emergency", // Context is in the name
            vec![result.name],                        // Causal link to the bad result
        );
        input_sender.send(feedback).unwrap();
    } else {
        println!("Warning: Could not find a result to link feedback to. Skipping.");
    }

    // --- The Learning ---
    println!("\n[LEARNING] Pausing for 6 seconds to allow the RefinementEngine to learn...");
    thread::sleep(Duration::from_secs(6));

    // --- The Wisdom ---
    println!("\n[SCENARIO 2] Simulating the same emergency again.");
    println!("Chronosa should now know better and do nothing.");
    input_sender.send(emergency_input).unwrap();
    thread::sleep(Duration::from_millis(200));

    // --- The Proof ---
    println!("\n[PROOF] Checking the final state...");
    // FIX: Use the shared_state handle, not the moved 'engine'.
    let final_state = shared_state.read().unwrap();
    let constraint_found = final_state.find_last_by_subtype("constraint.discovered");
    if let Some(constraint) = constraint_found {
        println!("SUCCESS: A new constraint was discovered and added to the log:");
        println!(" -> {}", constraint.name);
    } else {
        println!("FAILURE: No new constraint was discovered.");
    }

    // --- Graceful Shutdown ---
    println!("\n[SHUTDOWN] Shutting down all components.");
    drop(input_sender);
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    // Although the refinement thread will exit when the sender drops,
    // it's good practice to join its handle as well.
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}
