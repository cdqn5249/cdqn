// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/reasoning_projector.rs

//! The ReasoningProjector for Chronosa's advanced reasoning model.

use crate::cdu::{Cdu, CduPayload, Constraint};
use crate::engine::Projector;
use crate::reasoning::{PrimeElement, SemiAxiom};
use crate::state::ChronosaState;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

/// The ReasoningProjector uses PrimeElements and SemiAxioms from the state
/// to perform concurrent, history-aware reasoning.
pub struct ReasoningProjector {
    // In the future, this could hold configuration or references to other services.
    // For now, it's a marker struct.
}

impl ReasoningProjector {
    /// Creates a new ReasoningProjector.
    pub fn new() -> Self {
        Self {}
    }

    /// Extracts all prime elements from the current state log.
    fn get_prime_elements(&self, state: &ChronosaState) -> HashMap<String, PrimeElement> {
        let mut elements = HashMap::new();
        for cdu in state.log() {
            if let Some(CduPayload::PrimeElement(element)) = cdu.extract_payload() {
                elements.insert(element.id.clone(), element);
            }
        }
        elements
    }

    /// Extracts all semi-axioms from the current state log.
    fn get_semi_axioms(&self, state: &ChronosaState) -> Vec<SemiAxiom> {
        let mut axioms = Vec::new();
        for cdu in state.log() {
            if let Some(CduPayload::SemiAxiom(axiom)) = cdu.extract_payload() {
                axioms.push(axiom);
            }
        }
        axioms
    }

    /// Extracts all constraints from the current state log.
    fn get_constraints(&self, state: &ChronosaState) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        for cdu in state.log() {
            if let Some(CduPayload::Constraint(constraint)) = cdu.extract_payload() {
                constraints.push(constraint);
            }
        }
        constraints
    }

    /// Evaluates a single semi-axiom against the state and input.
    /// This is a pure function and can be run in a separate thread.
    fn evaluate_axiom(
        axiom: SemiAxiom,
        _input: &Cdu,
        prime_elements: &HashMap<String, PrimeElement>,
    ) -> Vec<Cdu> {
        // Simple reasoning rule: If all prime elements required by the axiom exist,
        // then the axiom "fires" and produces a command.
        for element_id in &axiom.prime_elements {
            if !prime_elements.contains_key(element_id) {
                // A required prime element is missing, so this axiom does not fire.
                return vec![];
            }
        }

        // All required prime elements are present. Generate a command.
        let command_payload = format!("Command from axiom: {}", axiom.description).into_bytes();
        let command_cdu = Cdu::new(
            command_payload,
            "command.reasoned",
            vec![axiom.id], // The axiom is the cause of the command
        );
        vec![command_cdu]
    }
}

impl Default for ReasoningProjector {
    fn default() -> Self {
        Self::new()
    }
}

impl Projector for ReasoningProjector {
    /// Projects the input CDU against the state to generate commands.
    /// This method evaluates relevant semi-axioms concurrently.
    fn project(&self, state: &ChronosaState, input: &Cdu) -> Vec<Cdu> {
        // 1. Extract all necessary knowledge from the state once.
        let prime_elements = self.get_prime_elements(state);
        let all_axioms = self.get_semi_axioms(state);
        let constraints = self.get_constraints(state);

        // 2. Perform the Inhibition Check.
        // For this simple projector, a "path" is just the axiom's ID.
        // A more advanced projector would check multi-step paths.
        let mut allowed_axioms = Vec::new();
        'axiom_loop: for axiom in all_axioms {
            for constraint in &constraints {
                // Check if the constraint's target path contains our axiom
                if constraint.target_path.contains(&axiom.id) {
                    // This axiom is part of a constrained path.
                    // A real similarity engine would go here. For now, we do a simple check:
                    // Does the input CDU's name contain the inhibiting context string?
                    // This is a placeholder for a real contextual check.
                    if input.name.contains(&constraint.inhibiting_context) {
                        // The context matches, so this axiom is inhibited. Skip it.
                        println!(
                            "Inhibition: Axiom '{}' pruned by constraint for context '{}'.",
                            axiom.id, constraint.inhibiting_context
                        );
                        continue 'axiom_loop;
                    }
                }
            }
            // If we get here, the axiom is not inhibited.
            allowed_axioms.push(axiom);
        }

        // 3. Set up concurrency for allowed axioms.
        let (sender, receiver) = mpsc::channel();
        let mut handles = vec![];

        // 4. Spawn a thread for each *allowed* axiom to evaluate it concurrently.
        for axiom in allowed_axioms {
            let sender_clone = sender.clone();
            let input_clone = input.clone();
            let elements_clone = prime_elements.clone();

            let handle = thread::spawn(move || {
                let results = Self::evaluate_axiom(axiom, &input_clone, &elements_clone);
                for result in results {
                    if sender_clone.send(result).is_err() {
                        // The receiver has been dropped, likely because the main thread is shutting down.
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        // Drop the original sender so the receiver doesn't wait for it.
        drop(sender);

        // 5. Collect all results from the concurrent evaluations.
        let mut final_commands = Vec::new();
        for command in receiver {
            final_commands.push(command);
        }

        // 6. Wait for all threads to finish to ensure clean shutdown.
        for handle in handles {
            handle.join().unwrap();
        }

        final_commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdu::{Cdu, Constraint};
    use crate::reasoning::{PrimeElement, SemiAxiom};
    use crate::state::ChronosaState;

    /// Helper function to manually evolve a state for testing purposes.
    fn test_evolve(mut state: ChronosaState, event: Cdu) -> ChronosaState {
        if event.metadata.hlc > state.hlc {
            state.hlc = event.metadata.hlc.clone();
        }
        state.log.push(event);
        state
    }

    #[test]
    fn test_reasoning_projector() {
        // 1. Set up the initial state with a prime element and a semi-axiom.
        let mut initial_state = ChronosaState::default();

        let pe1 = PrimeElement::new(
            "pe-1".to_string(),
            "uworld".to_string(),
            1.0,
            "The user is present".to_string(),
            "Cannot be decomposed".to_string(),
        );
        let pe1_cdu = pe1.to_cdu();
        initial_state = test_evolve(initial_state, pe1_cdu);

        let axiom1 = SemiAxiom::new(
            "axiom-1".to_string(),
            "uworld".to_string(),
            vec!["pe-1".to_string()], // This axiom requires "pe-1"
            "If user is present, greet them".to_string(),
        );
        let axiom1_cdu = axiom1.to_cdu();
        initial_state = test_evolve(initial_state, axiom1_cdu);

        // 2. Create the projector.
        let projector = ReasoningProjector::new();

        // 3. Create an input CDU.
        let input = Cdu::new(
            "User says hello".to_string().into_bytes(),
            "observation.uworld",
            vec![],
        );

        // 4. Project the input against the state.
        let commands = projector.project(&initial_state, &input);

        // 5. Assert that a command was generated.
        assert_eq!(commands.len(), 1);
        assert!(commands[0].name.contains(".command.reasoned"));
        assert_eq!(
            commands[0].metadata.causes[0],
            "axiom-1" // The command was caused by the axiom
        );
    }

    #[test]
    fn test_reasoning_projector_fails_axiom() {
        // 1. Set up state with a semi-axiom that requires a prime element that is NOT in the state.
        let mut initial_state = ChronosaState::default();

        let axiom1 = SemiAxiom::new(
            "axiom-1".to_string(),
            "uworld".to_string(),
            vec!["pe-missing".to_string()], // This axiom requires "pe-missing"
            "If user is present, greet them".to_string(),
        );
        let axiom1_cdu = axiom1.to_cdu();
        initial_state = test_evolve(initial_state, axiom1_cdu);

        // 2. Create the projector.
        let projector = ReasoningProjector::new();

        // 3. Create an input CDU.
        let input = Cdu::new(
            "User says hello".to_string().into_bytes(),
            "observation.uworld",
            vec![],
        );

        // 4. Project the input against the state.
        let commands = projector.project(&initial_state, &input);

        // 5. Assert that NO command was generated because the axiom's condition was not met.
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_reasoning_projector_inhibits_axiom() {
        // 1. Set up state with an axiom and a constraint that forbids it in a specific context.
        let mut initial_state = ChronosaState::default();

        let pe1 = PrimeElement::new(
            "pe-1".to_string(),
            "uworld".to_string(),
            1.0,
            "The user is present".to_string(),
            "Cannot be decomposed".to_string(),
        );
        initial_state = test_evolve(initial_state, pe1.to_cdu());

        let axiom1 = SemiAxiom::new(
            "axiom-1".to_string(),
            "uworld".to_string(),
            vec!["pe-1".to_string()],
            "If user is present, greet them".to_string(),
        );
        initial_state = test_evolve(initial_state, axiom1.to_cdu());

        let constraint = Constraint {
            target_path: vec!["axiom-1".to_string()],
            inhibiting_context: "emergency".to_string(),
            reason: "Do not greet during an emergency".to_string(),
            world: "uworld".to_string(),
        };
        let constraint_cdu = Cdu::from_payload(
            CduPayload::Constraint(constraint),
            "constraint.uworld",
            vec![],
        );
        initial_state = test_evolve(initial_state, constraint_cdu);

        // 2. Create the projector.
        let projector = ReasoningProjector::new();

        // 3. Create an input CDU that MATCHES the inhibiting context.
        let input = Cdu::new(
            "User shouts for help".to_string().into_bytes(),
            "observation.emergency", // The context matches the constraint
            vec![],
        );

        // 4. Project the input against the state.
        let commands = projector.project(&initial_state, &input);

        // 5. Assert that NO command was generated because the axiom was inhibited.
        assert_eq!(commands.len(), 0);
    }
}
