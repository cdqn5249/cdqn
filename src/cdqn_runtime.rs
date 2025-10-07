// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdqn_runtime.rs

//! The cdqnRuntime, the main orchestrator for the Chronosa ecosystem.

use crate::cdu::{Cdu, CduPayload};
use crate::engine::{Engine, EngineInput};
use crate::executor::Executor;
use crate::reasoning::{PrimeElement, ReasoningProjector, SemiAxiom};
use crate::refinement::RefinementEngine;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

// --- Structs for deserializing genesis.json ---

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GenesisPrimeElement {
    id: String,
    world: String,
    representation: Vec<f64>,
    description: String,
    #[serde(default)]
    symmetric_pair: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GenesisSemiAxiom {
    id: String,
    world: String,
    premises: Vec<String>,
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GenesisAxiom {
    id: String,
    worlds: Vec<String>,
    premises: Vec<String>,
    description: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum GenesisCdu {
    PrimeElement(GenesisPrimeElement),
    SemiAxiom(GenesisSemiAxiom),
    Axiom(GenesisAxiom),
}

/// The main entry point for the cdqnRuntime.
pub fn run() {
    println!("--- cdqnRuntime: Starting ---");
    let log_path = PathBuf::from("runtime.cdqn");
    let _ = fs::remove_file(&log_path);

    // 1. Create the core components.
    let projector = ReasoningProjector::new();
    let (engine, command_receiver) = Engine::new(log_path, Box::new(projector));
    let input_sender = engine.input_sender.clone();
    let shared_state = engine.state.clone();

    // 2. Spawn all background threads.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());
    let refinement_handle = RefinementEngine::spawn(shared_state.clone(), input_sender.clone());
    let engine_handle = thread::spawn(move || engine.run());

    // 3. Read and process the genesis file.
    println!("\n[Runtime] Reading genesis.json to build the universe...");
    let genesis_content = fs::read_to_string("genesis.json").expect("Failed to read genesis.json");

    match serde_json::from_str::<Vec<GenesisCdu>>(&genesis_content) {
        Ok(genesis_cdus) => {
            for genesis_cdu in genesis_cdus {
                let (cdu_payload, subtype) = convert_genesis_cdu(genesis_cdu);
                let cdu = Cdu::from_payload(cdu_payload, &subtype, vec![]);
                println!("[Runtime] Seeding Genesis CDU: {}", cdu.name);
                if input_sender.send(EngineInput::Cdu(cdu)).is_err() {
                    eprintln!("[Runtime] Failed to send genesis CDU, engine may have shut down.");
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("[Runtime] Error parsing genesis.json: {}", e);
        }
    }

    thread::sleep(Duration::from_millis(500)); // Allow seeding to process.

    // 4. Initiate a graceful shutdown.
    println!("\n[Runtime] Genesis complete. Shutting down components.");
    if input_sender.send(EngineInput::Shutdown).is_err() {
        println!("[Runtime] Engine channel was already closed.");
    }

    // 5. Wait for the threads to terminate.
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();
    refinement_handle.join().unwrap();

    println!("\nSession complete.");
}

/// Converts a deserialized GenesisCdu into a real CduPayload and subtype string.
fn convert_genesis_cdu(genesis_cdu: GenesisCdu) -> (CduPayload, String) {
    match genesis_cdu {
        GenesisCdu::PrimeElement(pe) => {
            let prime_element = PrimeElement {
                id: pe.id,
                world: pe.world.clone(),
                representation: pe.representation,
                description: pe.description,
                irreducibility_proof: "Self-evident (Genesis)".to_string(),
                symmetric_pair: pe.symmetric_pair,
                relationships: Default::default(),
            };
            (
                CduPayload::PrimeElement(prime_element),
                format!("prime.element.{}", pe.world),
            )
        }
        GenesisCdu::SemiAxiom(sa) => {
            let semi_axiom = SemiAxiom {
                id: sa.id,
                world: sa.world.clone(),
                prime_elements: sa.premises,
                description: sa.description,
                weight: 1.0,
            };
            (
                CduPayload::SemiAxiom(semi_axiom),
                format!("semi-axiom.{}", sa.world),
            )
        }
        GenesisCdu::Axiom(ax) => {
            let axiom = SemiAxiom {
                id: ax.id,
                world: ax.worlds.join("_"),
                prime_elements: ax.premises,
                description: ax.description,
                weight: 1.0,
            };
            (
                CduPayload::SemiAxiom(axiom),
                format!("axiom.{}", ax.worlds.join("_")),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::rehydrate_from_log;
    use std::io::Write;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn test_genesis_parsing_and_storage() {
        // 1. Define a temporary path for the test files.
        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        let temp_dir = std::env::temp_dir().join(format!("genesis_test_{}", rand_string));
        fs::create_dir_all(&temp_dir).unwrap();
        let genesis_path = temp_dir.join("genesis.json");
        let log_path = temp_dir.join("test_runtime.cdqn");

        // 2. Create a dummy genesis.json file.
        let dummy_genesis_content = r#"[
            {
                "type": "PrimeElement", "id": "pe-test-1", "world": "TestWorld", "representation": [1.0],
                "description": "Test PE 1"
            },
            {
                "type": "SemiAxiom", "id": "sa-test-1", "world": "TestWorld", "premises": ["pe-test-1"],
                "description": "Test SA 1"
            }
        ]"#;
        let mut file = fs::File::create(&genesis_path).unwrap();
        file.write_all(dummy_genesis_content.as_bytes()).unwrap();

        // 3. Run a simplified version of the runtime logic.
        let projector = ReasoningProjector::new();
        let (engine, command_receiver) = Engine::new(log_path.clone(), Box::new(projector));
        let input_sender = engine.input_sender.clone();
        let _executor_handle = Executor::spawn(command_receiver, input_sender.clone());
        let engine_handle = thread::spawn(move || engine.run());

        let genesis_content = fs::read_to_string(genesis_path).unwrap();
        let genesis_cdus: Vec<GenesisCdu> = serde_json::from_str(&genesis_content).unwrap();
        let expected_cdu_count = genesis_cdus.len();

        for genesis_cdu in genesis_cdus {
            let (cdu_payload, subtype) = convert_genesis_cdu(genesis_cdu);
            let cdu = Cdu::from_payload(cdu_payload, &subtype, vec![]);
            input_sender.send(EngineInput::Cdu(cdu)).unwrap();
        }

        thread::sleep(Duration::from_millis(200));
        input_sender.send(EngineInput::Shutdown).unwrap();
        engine_handle.join().unwrap();

        // 4. Rehydrate the log and verify the contents.
        let rehydrated_cdus = rehydrate_from_log(&log_path).unwrap();
        assert_eq!(rehydrated_cdus.len(), expected_cdu_count);

        // FIX: Robustly check the payload content, not the name.
        let pe_found = rehydrated_cdus.iter().any(|c| {
            if let Some(CduPayload::PrimeElement(pe)) = c.extract_payload() {
                pe.id == "pe-test-1"
            } else {
                false
            }
        });
        let sa_found = rehydrated_cdus.iter().any(|c| {
            if let Some(CduPayload::SemiAxiom(sa)) = c.extract_payload() {
                sa.id == "sa-test-1"
            } else {
                false
            }
        });

        assert!(pe_found, "The test PrimeElement was not found in the rehydrated log.");
        assert!(sa_found, "The test SemiAxiom was not found in the rehydrated log.");

        // 5. Clean up.
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
