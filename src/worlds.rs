// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/worlds.rs

//! The Worlds module, a toolbox for creating the foundational realities of the ecosystem.

use crate::cdu::{Cdu, CduPayload};
use crate::engine::EngineInput;
use crate::metaphysics::{NEGATIVE_POLE, POSITIVE_POLE};
use crate::reasoning::{PrimeElement, SemiAxiom};
use std::sync::mpsc::Sender;

/// Creates the foundational CDUs for Rworld and sends them to the Engine.
/// Rworld is founded on the principle of a fundamental, symmetric duality.
pub fn create_rworld(input_sender: &Sender<EngineInput>) {
    println!("[Worlds] Creating Rworld based on the fundamental duality...");

    // 1. Define the IDs for the symmetric pair.
    let neg_pole_id = "pe-r-neg-pole".to_string();
    let pos_pole_id = "pe-r-pos-pole".to_string();

    // 2. Create the Prime Elements and immediately link them symmetrically.
    let mut pe_neg_pole = PrimeElement::new(
        neg_pole_id.clone(),
        "Rworld".to_string(),
        vec![NEGATIVE_POLE],
        "The fundamental pole of negative potential.".to_string(),
        "Self-evident".to_string(),
    );
    pe_neg_pole.symmetric_pair = Some(pos_pole_id.clone());

    let mut pe_pos_pole = PrimeElement::new(
        pos_pole_id.clone(),
        "Rworld".to_string(),
        vec![POSITIVE_POLE],
        "The fundamental pole of positive potential.".to_string(),
        "Self-evident".to_string(),
    );
    pe_pos_pole.symmetric_pair = Some(neg_pole_id.clone());

    // 3. Send the symmetrically-linked poles to the Engine using the correct method.
    let neg_pole_cdu = Cdu::from_payload(
        CduPayload::PrimeElement(pe_neg_pole),
        "prime.element.Rworld",
        vec![],
    );
    if input_sender.send(EngineInput::Cdu(neg_pole_cdu)).is_err() {
        return;
    }

    let pos_pole_cdu = Cdu::from_payload(
        CduPayload::PrimeElement(pe_pos_pole),
        "prime.element.Rworld",
        vec![],
    );
    if input_sender.send(EngineInput::Cdu(pos_pole_cdu)).is_err() {
        return;
    }

    // 4. The First Rule: "Duality".
    let sa_duality = SemiAxiom::new(
        "sa-r-duality".to_string(),
        "Rworld".to_string(),
        vec![neg_pole_id, pos_pole_id],
        "The principle that positive and negative potentials are fundamental and co-existing."
            .to_string(),
    );
    let axiom_cdu = Cdu::from_payload(
        CduPayload::SemiAxiom(sa_duality),
        "semi-axiom.Rworld",
        vec![],
    );
    if input_sender.send(EngineInput::Cdu(axiom_cdu)).is_err() {
        return;
    }

    println!("[Worlds] Rworld created successfully.");
}
