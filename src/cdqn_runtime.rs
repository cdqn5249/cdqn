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
