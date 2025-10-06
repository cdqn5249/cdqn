// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::{Cdu, CduPayload};
use cdqn::engine::Engine;
use cdqn::executor::Executor;
use cdqn::payloads::Theorem;
use cdqn::reasoning::{PrimeElement, ReasoningProjector};
use cdqn::refinement::RefinementEngine;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Chronosa Full Learning & CTD Cycle Demo ---");
    let log_path = PathBuf::from("ctd_demo.cdqn");
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

    // NEW: Seed an intent-based theorem for the CTD demo.
    let intent_theorem = Theorem {
        premises: vec![], // This theorem is not based on premises, but on modes.
        conclusion: "intent-based: Formulate a plan to find keys".to_string(),
        proof_path: vec![],
        confidence_score: 1.0,
    };
    let intent_theorem_cdu = Cdu::from_payload(
        CduPayload::Theorem(intent_theorem),
        "theorem.uworld",
        vec![],
    );
    input_sender.send(intent_theorem_cdu).unwrap();
    thread::sleep(Duration::from_millis(100));

    // --- SCENARIO 1: Causal Tensor Decomposition ---
    println!("\n[SCENARIO 1] Simulating a user intent to test the CTD workflow.");
    let intent_input = Cdu::new(
        b"Find my keys".to_vec(),
        "observation.intent", // This subtype will trigger the DecompositionStrategy
        vec![],
    );
    input_sender.send(intent_input).unwrap();
    // Give the engine time to process this single event before we check the proof.
    thread::sleep(Duration::from_millis(200));

    // --- The Final Proof ---
    println!("\n[PROOF] Checking final log for CTD-related knowledge...");
    let final_state = shared_state.read().unwrap();
    let mode_found = final_state
        .find_last_by_subtype("causal.mode.intent")
        .is_some();
    let ctd_command_found = final_state
        .find_last_by_subtype("command.theorem.ctd")
        .is_some();

    if mode_found {
        println!("SUCCESS: A new CAUSAL MODE was created by the DecompositionStrategy.");
    } else {
        println!("FAILURE: The DecompositionStrategy did not create a Causal Mode.");
    }

    if ctd_command_found {
        println!("SUCCESS: The TheoremStrategy USED the Causal Mode to apply a relevant theorem.");
    } else {
        println!("FAILURE: The TheoremStrategy did not use the Causal Mode.");
    }

    // --- Graceful Shutdown ---
    println!("\n[SHUTDOWN] Shutting down all components.");
    // FIX: Drop the sender BEFORE joining the handles.
    // This closes the channel and signals the threads to terminate.
    drop(input_sender);

    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}
