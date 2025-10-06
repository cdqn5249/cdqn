// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/refinement.rs

//! The Refinement Engine for Chronosa's autonomous learning.

use crate::cdu::{Cdu, CduPayload, Constraint, Theorem};
use crate::reasoning::knowledge_base::KnowledgeBase;
use crate::state::SharedState;
use std::collections::HashSet;
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
    input_sender: std::sync::mpsc::Sender<Cdu>,
}

impl RefinementEngine {
    /// Spawns the RefinementEngine on a new background thread.
    pub fn spawn(
        state: SharedState,
        input_sender: std::sync::mpsc::Sender<Cdu>,
    ) -> thread::JoinHandle<()> {
        let engine = Self {
            state,
            input_sender,
        };
        thread::spawn(move || engine.run())
    }

    /// The main run loop. It periodically wakes up to analyze the state.
    fn run(self) {
        println!("RefinementEngine: Running.");
        loop {
            thread::sleep(Duration::from_secs(5));

            let heartbeat = Cdu::new(vec![], "refinement.heartbeat", vec![]);
            if self.input_sender.send(heartbeat).is_err() {
                break;
            }

            println!("RefinementEngine: Waking up to analyze log...");
            // Create a single knowledge snapshot for this analysis cycle.
            let kb = {
                let state_guard = self.state.read().unwrap();
                KnowledgeBase::from_state(&state_guard)
            };

            let new_constraints = self.discover_constraints(&kb);
            let new_theorems = self.discover_theorems(&kb);

            if !new_constraints.is_empty() {
                println!(
                    "RefinementEngine: Discovered {} new constraint(s).",
                    new_constraints.len()
                );
                for constraint in new_constraints {
                    let constraint_cdu = Cdu::from_payload(
                        CduPayload::Constraint(constraint),
                        "constraint.discovered",
                        vec![],
                    );
                    if self.input_sender.send(constraint_cdu).is_err() {
                        break;
                    }
                }
            }

            if !new_theorems.is_empty() {
                println!(
                    "RefinementEngine: Discovered {} new theorem(s).",
                    new_theorems.len()
                );
                for theorem in new_theorems {
                    let theorem_cdu = Cdu::from_payload(
                        CduPayload::Theorem(theorem),
                        "theorem.discovered",
                        vec![],
                    );
                    if self.input_sender.send(theorem_cdu).is_err() {
                        break;
                    }
                }
            }
        }
        println!("RefinementEngine: Shutting down.");
    }

    /// Analyzes the log to discover new constraints, avoiding duplicates.
    fn discover_constraints(&self, kb: &KnowledgeBase) -> Vec<Constraint> {
        let mut potential_constraints = Vec::new();
        // This is inefficient but necessary for now without changing the KB.
        let state_guard = self.state.read().unwrap();
        let log_cdu = state_guard.log();

        for cdu in log_cdu.iter() {
            if cdu.name.contains("feedback.reputation.negative") {
                if let Some(cause_name) = cdu.metadata.causes.first() {
                    if let Some(cause_cdu) = log_cdu.iter().find(|c| c.name == *cause_name) {
                        if let Some(command_name) = cause_cdu.metadata.causes.first() {
                            let context_parts: Vec<&str> = cdu.name.split('.').collect();
                            let inhibiting_context = if context_parts.len() > 2 {
                                context_parts[context_parts.len() - 2].to_string()
                            } else {
                                "unknown".to_string()
                            };

                            potential_constraints.push(Constraint {
                                target_path: vec![command_name.clone()],
                                inhibiting_context,
                                reason: "Action led to negative feedback".to_string(),
                                world: "uworld".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Filter out constraints that are too similar to existing ones.
        let mut new_constraints = Vec::new();
        'potential_loop: for p_constraint in &potential_constraints {
            for e_constraint in kb.constraints() {
                if p_constraint.target_path == e_constraint.target_path {
                    if let (Some(p_context_pe), Some(e_context_pe)) = (
                        kb.prime_elements().get(&p_constraint.inhibiting_context),
                        kb.prime_elements().get(&e_constraint.inhibiting_context),
                    ) {
                        let distance = calculate_euclidean_distance(
                            &p_context_pe.representation,
                            &e_context_pe.representation,
                        );
                        if distance < SIMILARITY_EPSILON {
                            continue 'potential_loop; // Similar constraint exists, discard.
                        }
                    }
                }
            }
            new_constraints.push(p_constraint.clone());
        }
        new_constraints
    }

    /// Analyzes the log to discover new theorems, avoiding duplicates.
    fn discover_theorems(&self, kb: &KnowledgeBase) -> Vec<Theorem> {
        let mut potential_theorems = Vec::new();
        let state_guard = self.state.read().unwrap();
        let log_cdu = state_guard.log();

        for cdu in log_cdu.iter() {
            if cdu.name.contains("feedback.reputation.positive") {
                if let Some(result_name) = cdu.metadata.causes.first() {
                    if let Some(result_cdu) = log_cdu.iter().find(|c| c.name == *result_name) {
                        if let Some(command_name) = result_cdu.metadata.causes.first() {
                            if let Some(command_cdu) =
                                log_cdu.iter().find(|c| c.name == *command_name)
                            {
                                if let Some(axiom_id) = command_cdu.metadata.causes.first() {
                                    if let Some(axiom_cdu) =
                                        log_cdu.iter().find(|c| c.name.contains(axiom_id))
                                    {
                                        if let Some(CduPayload::SemiAxiom(axiom)) =
                                            axiom_cdu.extract_payload()
                                        {
                                            potential_theorems.push(Theorem {
                                                premises: axiom.prime_elements.clone(),
                                                conclusion: format!(
                                                    "Successfully apply '{}'",
                                                    axiom.description
                                                ),
                                                proof_path: vec![
                                                    axiom_id.clone(),
                                                    command_name.clone(),
                                                    result_name.clone(),
                                                ],
                                                confidence_score: 1.0,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Filter out theorems that already exist.
        let mut new_theorems = Vec::new();
        'potential_loop: for p_theorem in &potential_theorems {
            for e_theorem in kb.theorems() {
                let p_premises: HashSet<_> = p_theorem.premises.iter().collect();
                let e_premises: HashSet<_> = e_theorem.premises.iter().collect();
                if p_premises == e_premises && p_theorem.conclusion == e_theorem.conclusion {
                    continue 'potential_loop; // Duplicate theorem exists, discard.
                }
            }
            new_theorems.push(p_theorem.clone());
        }
        new_theorems
    }
}
