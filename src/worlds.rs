// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/worlds.rs

//! The Worlds module, a toolbox for creating the foundational realities of the ecosystem.

use crate::cdu::{Cdu, CduPayload};
use crate::engine::EngineInput;
use crate::metaphysics::{NEGATIVE_POLE, POSITIVE_POLE};
use crate::reasoning::{PrimeElement, SemiAxiom};
use std::sync::mpsc::Sender;

/// Creates the foundational CDUs for Rworld and sends them to the Engine.
/// Rworld is founded on the principle of a fundamental duality.
pub fn create_rworld(input_sender: &Sender<EngineInput>) {
    println!("[Worlds] Creating Rworld based on the fundamental duality...");

    // The First Prime Element: The Negative Pole.
    let pe_neg_pole = PrimeElement::new(
        "pe-r-neg-pole".to_string(),
        "Rworld".to_string(),
        vec![NEGATIVE_POLE], // Use the constant from metaphysics
        "The fundamental pole of negative potential.".to_string(),
        "Self-evident".to_string(),
    );
    if input_sender
        .send(EngineInput::Cdu(pe_neg_pole.to_cdu()))
        .is_err()
    {
        return; // Engine is gone, abort.
    }

    // The Second Prime Element: The Positive Pole.
    let pe_pos_pole = PrimeElement::new(
        "pe-r-pos-pole".to_string(),
        "Rworld".to_string(),
        vec![POSITIVE_POLE], // Use the constant from metaphysics
        "The fundamental pole of positive potential.".to_string(),
        "Self-evident".to_string(),
    );
    if input_sender
        .send(EngineInput::Cdu(pe_pos_pole.to_cdu()))
        .is_err()
    {
        return; // Engine is gone, abort.
    }

    // The First Rule: "Duality". This defines the relationship between the two poles.
    let sa_duality = SemiAxiom::new(
        "sa-r-duality".to_string(),
        "Rworld".to_string(),
        vec!["pe-r-neg-pole".to_string(), "pe-r-pos-pole".to_string()],
        "The principle that positive and negative potentials are fundamental and co-existing."
            .to_string(),
    );
    let axiom_cdu = Cdu::from_payload(
        CduPayload::SemiAxiom(sa_duality),
        "semi-axiom.Rworld",
        vec![],
    );
    if input_sender.send(EngineInput::Cdu(axiom_cdu)).is_err() {
        return; // Engine is gone, abort.
    }

    println!("[Worlds] Rworld created successfully.");
}
