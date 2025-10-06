// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/worlds.rs

//! The Worlds module, a toolbox for creating the foundational realities of the ecosystem.

use crate::cdu::{Cdu, CduPayload};
use crate::engine::EngineInput;
use crate::reasoning::{PrimeElement, SemiAxiom};
use std::sync::mpsc::Sender;

/// Creates the foundational CDUs for Rworld and sends them to the Engine.
/// Rworld is the set of all real numbers, representing the entirety of existence.
pub fn create_rworld(input_sender: &Sender<EngineInput>) {
    println!("[Worlds] Creating Rworld...");

    // The First Concept: "An Entity". The most basic "noun".
    let pe_entity = PrimeElement::new(
        "pe-r-entity".to_string(),
        "Rworld".to_string(),
        vec![1.0], // A single dimension representing "existence".
        "A fundamental, irreducible entity.".to_string(),
        "Self-evident".to_string(),
    );
    if input_sender
        .send(EngineInput::Cdu(pe_entity.to_cdu()))
        .is_err()
    {
        return; // Engine is gone, abort.
    }

    // The First Rule: "Causality". The most basic "verb".
    let sa_causality = SemiAxiom::new(
        "sa-r-causality".to_string(),
        "Rworld".to_string(),
        vec!["pe-r-entity".to_string()],
        "The principle that one entity can be a cause for another.".to_string(),
    );
    if input_sender
        .send(Cdu::from_payload(
            CduPayload::SemiAxiom(sa_causality),
            "semi-axiom.Rworld",
            vec![],
        ).into()) // .into() wraps the Cdu in EngineInput::Cdu
        .is_err()
    {
        return; // Engine is gone, abort.
    }

    println!("[Worlds] Rworld created successfully.");
}
