// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/knowledge_base.rs

//! A snapshot of all knowledge relevant to a single reasoning cycle.

use crate::cdu::CduPayload;
use crate::payloads::{Constraint, Theorem}; // FIX: Import directly from payloads
use crate::reasoning::{PrimeElement, SemiAxiom};
use crate::state::ChronosaState;
use std::collections::HashMap;

/// A snapshot of the current state's knowledge, extracted once per reasoning cycle.
/// This struct provides read-only access to the knowledge components.
pub struct KnowledgeBase {
    prime_elements: HashMap<String, PrimeElement>,
    semi_axioms: Vec<SemiAxiom>,
    constraints: Vec<Constraint>,
    theorems: Vec<Theorem>,
}

impl KnowledgeBase {
    /// Creates a new KnowledgeBase by extracting all relevant CDUs from the state.
    pub fn from_state(state: &ChronosaState) -> Self {
        let mut prime_elements = HashMap::new();
        let mut semi_axioms = Vec::new();
        let mut constraints = Vec::new();
        let mut theorems = Vec::new();

        for cdu in state.log() {
            match cdu.extract_payload() {
                Some(CduPayload::PrimeElement(pe)) => {
                    prime_elements.insert(pe.id.clone(), pe);
                }
                Some(CduPayload::SemiAxiom(sa)) => semi_axioms.push(sa),
                Some(CduPayload::Constraint(c)) => constraints.push(c),
                Some(CduPayload::Theorem(t)) => theorems.push(t),
                _ => {} // Ignore other CDU types
            }
        }

        Self {
            prime_elements,
            semi_axioms,
            constraints,
            theorems,
        }
    }

    // --- Accessor methods ---

    pub fn prime_elements(&self) -> &HashMap<String, PrimeElement> {
        &self.prime_elements
    }

    pub fn semi_axioms(&self) -> &[SemiAxiom] {
        &self.semi_axioms
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    pub fn theorems(&self) -> &[Theorem] {
        &self.theorems
    }
}
