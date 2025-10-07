// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::{Engine, EngineInput};
use cdqn::executor::Executor;
use cdqn::reasoning::{PrimeElement, ReasoningProjector};
use cdqn::refinement::RefinementEngine;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Chronosa Symmetry Completion Demo ---");
    let log_path = PathBuf::from("symmetry_demo.cdqn");
    let _ = std::fs::remove_file(&log_path);

    // 1. Use the full ReasoningProjector.
    let projector = ReasoningProjector::new();

    // 2. Create the core components.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();
    let shared_state = engine.state.clone();

    // 3. Spawn all background threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state.clone(), input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // --- Seeding Unrefined Symmetric Concepts ---
    println!("\n[SETUP] Seeding two unrefined, symmetric Prime Elements...");

    // The concept of "success"
    let pe_success = PrimeElement::new(
        "pe-u-success".to_string(),
        "uWorld".to_string(),
        vec![1.5], // In the Positive Zone
        "The concept of a successful outcome.".to_string(),
        "Irreducible".to_string(),
    );
    input_sender
        .send(EngineInput::Cdu(pe_success.to_cdu()))
        .unwrap();

    // The concept of "failure", which is the symmetric opposite of "success".
    let pe_failure = PrimeElement::new(
        "pe-u-failure".to_string(),
        "uWorld".to_string(),
        vec![-1.5], // The exact negation of the "success" vector.
        "The concept of a failed outcome.".to_string(),
        "Irreducible".to_string(),
    );
    input_sender
        .send(EngineInput::Cdu(pe_failure.to_cdu()))
        .unwrap();

    // --- The Learning Phase ---
    println!("\n[LEARNING] Pausing for 6 seconds for the RefinementEngine to discover the symmetry...");
    thread::sleep(Duration::from_secs(6));

    // --- The Proof ---
    println!("\n[PROOF] Checking the log for refined, symmetrically-linked elements...");
    let final_state = shared_state.read().unwrap();
    let refined_elements: Vec<_> = final_state
        .log()
        .iter()
        .filter(|c| c.name.contains("prime.element.refined"))
        .collect();

    let success_is_linked = refined_elements.iter().any(|cdu| {
        if let Some(cdqn::cdu::CduPayload::PrimeElement(pe)) = cdu.extract_payload() {
            pe.id == "pe-u-success" && pe.symmetric_pair == Some("pe-u-failure".to_string())
        } else {
            false
        }
    });

    let failure_is_linked = refined_elements.iter().any(|cdu| {
        if let Some(cdqn::cdu::CduPayload::PrimeElement(pe)) = cdu.extract_payload() {
            pe.id == "pe-u-failure" && pe.symmetric_pair == Some("pe-u-success".to_string())
        } else {
            false
        }
    });

    if success_is_linked && failure_is_linked {
        println!("SUCCESS: The RefinementEngine correctly identified and linked the symmetric pair.");
    } else {
        println!("FAILURE: The RefinementEngine did not link the symmetric pair.");
    }

    assert!(success_is_linked);
    assert!(failure_is_linked);

    // --- Graceful Shutdown ---
    println!("\n[SHUTDOWN] Shutting down all components.");
    input_sender.send(EngineInput::Shutdown).unwrap();

    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}
