// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/refinement.rs

//! The Refinement Engine for Chronosa's autonomous learning.

use crate::cdu::{Cdu, CduPayload, Constraint};
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
            // In a real system, this would be triggered by specific events.
            // For now, we'll just poll every few seconds.
            thread::sleep(Duration::from_secs(5));

            // Create a dummy CDU to send. If it fails, the main engine has shut down.
            let heartbeat = Cdu::new(vec![], "refinement.heartbeat", vec![]);
            if self.input_sender.send(heartbeat).is_err() {
                // Engine has shut down, so we can exit.
                break;
            }

            println!("RefinementEngine: Waking up to analyze log...");
            let new_constraints = self.discover_constraints();

            if !new_constraints.is_empty() {
                println!(
                    "RefinementEngine: Discovered {} new constraint(s).",
                    new_constraints.len()
                );
                for constraint in new_constraints {
                    // TODO: Add proper causes for the constraint's discovery.
                    let constraint_cdu = Cdu::from_payload(
                        CduPayload::Constraint(constraint),
                        "constraint.discovered",
                        vec![],
                    );
                    if self.input_sender.send(constraint_cdu).is_err() {
                        // Engine has shut down.
                        break;
                    }
                }
            }

            // TODO: Add theorem discovery logic here.
        }
        println!("RefinementEngine: Shutting down.");
    }

    /// Analyzes the log to discover new constraints.
    /// This is a simplified, inefficient version for demonstration.
    fn discover_constraints(&self) -> Vec<Constraint> {
        let mut new_constraints = Vec::new();
        let state_guard = match self.state.read() {
            Ok(guard) => guard,
            Err(_) => return vec![], // Failed to get lock, try again later.
        };
        let log = state_guard.log();

        // This is a simple pattern search: look for a negative feedback
        // that is causally linked to a specific action's result.
        for cdu in log.iter() {
            if cdu.name.contains("feedback.reputation.negative") {
                if let Some(cause_name) = cdu.metadata.causes.first() {
                    // Find the result CDU that this feedback is about.
                    if let Some(cause_cdu) = log.iter().find(|c| c.name == *cause_name) {
                        // Find the command CDU that caused that result.
                        if let Some(command_name) = cause_cdu.metadata.causes.first() {
                            // We have the command that led to a bad outcome.
                            // Create a constraint to prevent it in the same context.
                            // The "context" is crudely extracted from the feedback CDU's name.
                            let context_parts: Vec<&str> = cdu.name.split('.').collect();
                            let inhibiting_context = if context_parts.len() > 2 {
                                context_parts[1].to_string()
                            } else {
                                "unknown".to_string()
                            };

                            let constraint = Constraint {
                                target_path: vec![command_name.clone()], // Path is just the command for now.
                                inhibiting_context,
                                reason: "Action led to negative feedback".to_string(),
                                world: "uworld".to_string(), // Assuming uworld for now.
                            };

                            // TODO: We need a way to check if this exact constraint already exists
                            // to avoid creating duplicates on every run.
                            new_constraints.push(constraint);
                        }
                    }
                }
            }
        }
        new_constraints
    }

    /// Placeholder for analyzing the log to discover new theorems.
    #[allow(dead_code)]
    fn discover_theorems(&self) -> Vec<Cdu> {
        // In the future, this will look for frequently repeated, successful
        // reasoning paths and abstract them into Theorem CDUs.
        vec![]
    }
}
