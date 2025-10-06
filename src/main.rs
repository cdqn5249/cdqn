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
    println!("--- Chronosa Full Learning & Assimilation Cycle Demo ---");
    let log_path = PathBuf::from("assimilation_demo.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. Use the intelligent ReasoningProjector.
    let projector = ReasoningProjector::new();

    // 2. Create the core components.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();
    let shared_state = engine.state.clone();

    // 3. Spawn all background threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state.clone(), input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // --- Seeding the Initial State ---
    println!("\n[SETUP] Seeding initial knowledge...");
    let pe_user = PrimeElement::new(
        "pe-user-present".to_string(),
        "uworld".to_string(),
        vec![1.0],
        "The user is present".to_string(),
        "".to_string(),
    );
    input_sender.send(pe_user.to_cdu()).unwrap();

    let pe_emergency = PrimeElement::new(
        "emergency".to_string(),
        "uworld".to_string(),
        vec![0.9],
        "Emergency context".to_string(),
        "".to_string(),
    );
    input_sender.send(pe_emergency.to_cdu()).unwrap();

    let axiom_greet = SemiAxiom::new(
        "axiom-greet-user".to_string(),
        "uworld".to_string(),
        vec!["pe-user-present".to_string()],
        "If user is present, greet them".to_string(),
    );
    input_sender.send(axiom_greet.to_cdu()).unwrap();
    thread::sleep(Duration::from_millis(100));

    // --- SCENARIO 1: Learning from Failure ---
    println!("\n[SCENARIO 1] Simulating an emergency. Chronosa will make a mistake.");
    let emergency_input = Cdu::new(
        b"User shouts for help".to_vec(),
        "observation.emergency",
        vec![],
    );
    input_sender.send(emergency_input.clone()).unwrap();
    thread::sleep(Duration::from_millis(200));

    println!("\n[FEEDBACK 1] Providing negative feedback for the mistake.");
    if let Some(result) = {
        shared_state
            .read()
            .unwrap()
            .find_last_by_subtype("result.task_completed")
            .cloned()
    } {
        let feedback = Cdu::new(
            b"Greeting during emergency is bad".to_vec(),
            "feedback.reputation.negative.emergency",
            vec![result.name],
        );
        input_sender.send(feedback).unwrap();
    }

    println!("\n[LEARNING 1] Pausing for 6s for RefinementEngine to discover a CONSTRAINT...");
    thread::sleep(Duration::from_secs(6));

    // --- SCENARIO 2: Learning from Success ---
    println!("\n[SCENARIO 2] Simulating a normal situation. Chronosa should act correctly.");
    let normal_input = Cdu::new(b"User waves hello".to_vec(), "observation.normal", vec![]);
    input_sender.send(normal_input.clone()).unwrap();
    thread::sleep(Duration::from_millis(200));

    println!("\n[FEEDBACK 2] Providing positive feedback for the correct action.");
    if let Some(result) = {
        shared_state
            .read()
            .unwrap()
            .find_last_by_subtype("result.task_completed")
            .cloned()
    } {
        let feedback = Cdu::new(
            b"Greeting was appropriate".to_vec(),
            "feedback.reputation.positive.normal",
            vec![result.name],
        );
        input_sender.send(feedback).unwrap();
    }

    println!("\n[LEARNING 2] Pausing for 6s for RefinementEngine to discover a THEOREM...");
    thread::sleep(Duration::from_secs(6));

    // --- SCENARIO 3: Causal Tensor Decomposition ---
    println!("\n[SCENARIO 3] Simulating a user intent to test the DecompositionStrategy.");
    let intent_input = Cdu::new(
        b"Find my keys".to_vec(),
        "observation.intent", // This subtype will trigger the strategy
        vec![],
    );
    input_sender.send(intent_input).unwrap();
    thread::sleep(Duration::from_millis(100));

    // --- The Final Proof ---
    println!("\n[PROOF] Checking final log for all learned knowledge...");
    let final_state = shared_state.read().unwrap();
    let constraint_found = final_state
        .find_last_by_subtype("constraint.discovered")
        .is_some();
    let theorem_found = final_state
        .find_last_by_subtype("theorem.discovered")
        .is_some();
    let mode_found = final_state
        .find_last_by_subtype("causal.mode.intent")
        .is_some();

    if constraint_found {
        println!("SUCCESS: A new CONSTRAINT was discovered.");
    } else {
        println!("FAILURE: No new constraint was discovered.");
    }
    if theorem_found {
        println!("SUCCESS: A new THEOREM was discovered.");
    } else {
        println!("FAILURE: No new theorem was discovered.");
    }
    if mode_found {
        println!("SUCCESS: A new CAUSAL MODE was created by the DecompositionStrategy.");
    } else {
        println!("FAILURE: The DecompositionStrategy did not create a Causal Mode.");
    }

    // --- Graceful Shutdown ---
    println!("\n[SHUTDOWN] Shutting down all components.");
    drop(input_sender);
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}
