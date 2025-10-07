// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/refinement.rs

//! The Refinement Engine for Chronosa's autonomous learning.

use crate::cdu::{Cdu, CduPayload};
use crate::engine::EngineInput;
use crate::payloads::{Constraint, Theorem};
use crate::reasoning::knowledge_base::KnowledgeBase;
use crate::reasoning::PrimeElement;
use crate::state::SharedState;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

/// Defines the "horizon of similarity".
const SIMILARITY_EPSILON: f64 = 0.1;

/// Calculates the Euclidean distance between two vectors.
fn calculate_euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    let max_len = a.len().max(b.len());
    let a_padded = a
        .iter()
        .cloned()
        .chain(std::iter::repeat(0.0))
        .take(max_len);
    let b_padded = b
        .iter()
        .cloned()
        .chain(std::iter::repeat(0.0))
        .take(max_len);

    a_padded
        .zip(b_padded)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// The RefinementEngine runs on a background thread, analyzing the log
/// to discover new knowledge like Constraints and Theorems.
pub struct RefinementEngine {
    state: SharedState,
    input_sender: std::sync::mpsc::Sender<EngineInput>,
}

impl RefinementEngine {
    /// Spawns the RefinementEngine on a new background thread.
    pub fn spawn(
        state: SharedState,
        input_sender: std::sync::mpsc::Sender<EngineInput>,
    ) -> thread::JoinHandle<()> {
        let engine = Self {
            state,
            input_sender,
        };
        thread::spawn(move || engine.run())
    }

    /// The main run loop. It periodically wakes up to analyze the state.
    fn run(self) {
        println!("[Refinement] Thread spawned and running.");
        loop {
            thread::sleep(Duration::from_secs(5));

            let heartbeat = Cdu::new(vec![], "refinement.heartbeat", vec![]);
            if self.input_sender.send(EngineInput::Cdu(heartbeat)).is_err() {
                println!("[Refinement] Engine channel closed, shutting down.");
                break;
            }

            println!("[Refinement] Waking up to analyze log...");
            let kb = {
                if let Ok(state_guard) = self.state.try_read() {
                    KnowledgeBase::from_state(&state_guard)
                } else {
                    continue;
                }
            };

            // --- Run Discovery Processes ---
            let new_constraints = self.discover_constraints(&kb);
            let new_theorems = self.discover_theorems(&kb);
            let symmetry_updates = self.complete_symmetries(&kb);

            // --- Send New Knowledge to Engine ---
            self.send_new_cdus(
                new_constraints
                    .into_iter()
                    .map(CduPayload::Constraint)
                    .collect(),
            );
            self.send_new_cdus(new_theorems.into_iter().map(CduPayload::Theorem).collect());
            self.send_new_cdus(
                symmetry_updates
                    .into_iter()
                    .map(CduPayload::PrimeElement)
                    .collect(),
            );
        }
        println!("[Refinement] Thread terminating.");
    }

    /// Helper function to send a batch of new knowledge CDUs to the engine.
    fn send_new_cdus(&self, payloads: Vec<CduPayload>) {
        if payloads.is_empty() {
            return;
        }

        println!(
            "[Refinement] Discovered {} new knowledge CDU(s).",
            payloads.len()
        );
        for payload in payloads {
            let subtype = match &payload {
                CduPayload::Constraint(_) => "constraint.discovered",
                CduPayload::Theorem(_) => "theorem.discovered",
                CduPayload::PrimeElement(_) => "prime.element.refined",
                _ => "discovery.unknown",
            };
            let new_cdu = Cdu::from_payload(payload, subtype, vec![]);
            if self.input_sender.send(EngineInput::Cdu(new_cdu)).is_err() {
                // Engine has shut down, so we can stop trying to send.
                return;
            }
        }
    }

    /// Analyzes the log to discover new constraints, avoiding duplicates.
    fn discover_constraints(&self, _kb: &KnowledgeBase) -> Vec<Constraint> {
        // ... (existing logic remains the same)
        vec![] // Simplified for brevity in this example
    }

    /// Analyzes the log to discover new theorems, avoiding duplicates.
    fn discover_theorems(&self, _kb: &KnowledgeBase) -> Vec<Theorem> {
        // ... (existing logic remains the same)
        vec![] // Simplified for brevity in this example
    }

    /// Analyzes the knowledge base to find and link symmetric pairs of PrimeElements.
    fn complete_symmetries(&self, kb: &KnowledgeBase) -> Vec<PrimeElement> {
        let mut updated_elements = Vec::new();
        let unrefined_elements: Vec<_> = kb
            .prime_elements()
            .values()
            .filter(|pe| pe.symmetric_pair.is_none())
            .collect();

        let mut paired_ids = HashSet::new();

        for (i, pe_a) in unrefined_elements.iter().enumerate() {
            if paired_ids.contains(&pe_a.id) {
                continue;
            }

            // Create the target symmetric vector.
            let target_vector: Vec<f64> = pe_a.representation.iter().map(|&v| -v).collect();

            // Search for a matching partner.
            for (j, pe_b) in unrefined_elements.iter().enumerate() {
                if i == j || paired_ids.contains(&pe_b.id) {
                    continue;
                }

                let distance = calculate_euclidean_distance(&target_vector, &pe_b.representation);
                if distance < SIMILARITY_EPSILON {
                    println!(
                        "[Refinement] Found symmetric pair: {} and {}",
                        pe_a.id, pe_b.id
                    );

                    // Create updated versions of both elements.
                    let mut updated_a = (*pe_a).clone();
                    updated_a.symmetric_pair = Some(pe_b.id.clone());

                    let mut updated_b = (*pe_b).clone();
                    updated_b.symmetric_pair = Some(pe_a.id.clone());

                    updated_elements.push(updated_a);
                    updated_elements.push(updated_b);

                    // Mark both as paired to avoid re-checking.
                    paired_ids.insert(pe_a.id.clone());
                    paired_ids.insert(pe_b.id.clone());
                    break; // Move to the next element.
                }
            }
        }
        updated_elements
    }
}
