// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/refinement.rs

//! The Refinement Engine for Chronosa's autonomous learning.

use crate::cdu::{Cdu, CduPayload, Constraint, Theorem};
use crate::state::SharedState;
use std::thread;
use std::time::Duration;

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
            let new_constraints = self.discover_constraints();
            let new_theorems = self.discover_theorems();

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

    /// Analyzes the log to discover new constraints.
    fn discover_constraints(&self) -> Vec<Constraint> {
        let mut new_constraints = Vec::new();
        let state_guard = match self.state.read() {
            Ok(guard) => guard,
            Err(_) => return vec![],
        };
        let log = state_guard.log();

        for cdu in log.iter() {
            if cdu.name.contains("feedback.reputation.negative") {
                if let Some(cause_name) = cdu.metadata.causes.first() {
                    if let Some(cause_cdu) = log.iter().find(|c| c.name == *cause_name) {
                        if let Some(command_name) = cause_cdu.metadata.causes.first() {
                            let context_parts: Vec<&str> = cdu.name.split('.').collect();
                            let inhibiting_context = if context_parts.len() > 2 {
                                context_parts[context_parts.len() - 2].to_string()
                            } else {
                                "unknown".to_string()
                            };

                            let constraint = Constraint {
                                target_path: vec![command_name.clone()],
                                inhibiting_context,
                                reason: "Action led to negative feedback".to_string(),
                                world: "uworld".to_string(),
                            };
                            new_constraints.push(constraint);
                        }
                    }
                }
            }
        }
        new_constraints
    }

    /// Analyzes the log to discover new theorems.
    fn discover_theorems(&self) -> Vec<Theorem> {
        let mut new_theorems = Vec::new();
        let state_guard = match self.state.read() {
            Ok(guard) => guard,
            Err(_) => return vec![],
        };
        let log = state_guard.log();

        // This is a simple pattern search: find a positive feedback, trace it back
        // to the axiom that caused it, and treat that axiom's premises as a theorem.
        for cdu in log.iter() {
            if cdu.name.contains("feedback.reputation.positive") {
                if let Some(result_name) = cdu.metadata.causes.first() {
                    if let Some(result_cdu) = log.iter().find(|c| c.name == *result_name) {
                        if let Some(command_name) = result_cdu.metadata.causes.first() {
                            if let Some(command_cdu) = log.iter().find(|c| c.name == *command_name)
                            {
                                if let Some(axiom_id) = command_cdu.metadata.causes.first() {
                                    if let Some(axiom_cdu) =
                                        log.iter().find(|c| c.name.contains(axiom_id))
                                    {
                                        if let Some(CduPayload::SemiAxiom(axiom)) =
                                            axiom_cdu.extract_payload()
                                        {
                                            let theorem = Theorem {
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
                                                confidence_score: 1.0, // Start with high confidence
                                            };
                                            new_theorems.push(theorem);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        new_theorems
    }
}
