// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/reasoning_projector.rs

//! The ReasoningProjector for Chronosa's advanced reasoning model.

use crate::cdu::{Cdu, CduPayload, Constraint, Theorem};
use crate::engine::Projector;
use crate::reasoning::{PrimeElement, SemiAxiom};
use crate::state::ChronosaState;
use std::collections::{HashMap, HashSet};
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

    /// Extracts all theorems from the current state log.
    fn get_theorems(&self, state: &ChronosaState) -> Vec<Theorem> {
        let mut theorems = Vec::new();
        for cdu in state.log() {
            if let Some(CduPayload::Theorem(theorem)) = cdu.extract_payload() {
                theorems.push(theorem);
            }
        }
        theorems
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
        let theorems = self.get_theorems(state);

        // 2. Perform the Theorem Check (the "shortcut").
        let known_element_ids: HashSet<_> = prime_elements.keys().cloned().collect();
        for theorem in &theorems {
            let premises_set: HashSet<_> = theorem.premises.iter().cloned().collect();
            if premises_set.is_subset(&known_element_ids) {
                // All premises for this theorem are met.
                println!(
                    "Shortcut: Theorem conclusion '{}' applied, skipping axiom evaluation.",
                    theorem.conclusion
                );
                let command_payload =
                    format!("Command from theorem: {}", theorem.conclusion).into_bytes();
                // We need a way to find the theorem's CDU name to cite it.
                // For now, we'll just use the conclusion as a placeholder cause.
                let command_cdu = Cdu::new(
                    command_payload,
                    "command.theorem",
                    vec![theorem.conclusion.clone()],
                );
                return vec![command_cdu];
            }
        }

        // 3. If no theorem applied, proceed with axiom evaluation.
        let all_axioms = self.get_semi_axioms(state);
        let constraints = self.get_constraints(state);

        // 4. Perform the Inhibition Check.
        let mut allowed_axioms = Vec::new();
        'axiom_loop: for axiom in all_axioms {
            for constraint in &constraints {
                if constraint.target_path.contains(&axiom.id) {
                    if input.name.contains(&constraint.inhibiting_context) {
                        println!(
                            "Inhibition: Axiom '{}' pruned by constraint for context '{}'.",
                            axiom.id, constraint.inhibiting_context
                        );
                        continue 'axiom_loop;
                    }
                }
            }
            allowed_axioms.push(axiom);
        }

        // 5. Set up concurrency for allowed axioms.
        let (sender, receiver) = mpsc::channel();
        let mut handles = vec![];

        for axiom in allowed_axioms {
            let sender_clone = sender.clone();
            let input_clone = input.clone();
            let elements_clone = prime_elements.clone();

            let handle = thread::spawn(move || {
                let results = Self::evaluate_axiom(axiom, &input_clone, &elements_clone);
                for result in results {
                    if sender_clone.send(result).is_err() {
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        drop(sender);

        // 6. Collect all results.
        let mut final_commands = Vec::new();
        for command in receiver {
            final_commands.push(command);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        final_commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdu::{Cdu, Constraint, Theorem};
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
        let mut initial_state = ChronosaState::default();
        let pe1 = PrimeElement::new(
            "pe-1".to_string(),
            "uworld".to_string(),
            1.0,
            "User is present".to_string(),
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
        let projector = ReasoningProjector::new();
        let input = Cdu::new(b"User says hello".to_vec(), "observation.uworld", vec![]);
        let commands = projector.project(&initial_state, &input);
        assert_eq!(commands.len(), 1);
        assert!(commands[0].name.contains(".command.reasoned"));
        assert_eq!(commands[0].metadata.causes[0], "axiom-1");
    }

    #[test]
    fn test_reasoning_projector_fails_axiom() {
        let mut initial_state = ChronosaState::default();
        let axiom1 = SemiAxiom::new(
            "axiom-1".to_string(),
            "uworld".to_string(),
            vec!["pe-missing".to_string()],
            "If user is present, greet them".to_string(),
        );
        initial_state = test_evolve(initial_state, axiom1.to_cdu());
        let projector = ReasoningProjector::new();
        let input = Cdu::new(b"User says hello".to_vec(), "observation.uworld", vec![]);
        let commands = projector.project(&initial_state, &input);
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_reasoning_projector_inhibits_axiom() {
        let mut initial_state = ChronosaState::default();
        let pe1 = PrimeElement::new(
            "pe-1".to_string(),
            "uworld".to_string(),
            1.0,
            "User is present".to_string(),
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
        let projector = ReasoningProjector::new();
        let input = Cdu::new(
            b"User shouts for help".to_vec(),
            "observation.emergency",
            vec![],
        );
        let commands = projector.project(&initial_state, &input);
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_reasoning_projector_uses_theorem() {
        // 1. Set up state with two prime elements and a theorem that requires them.
        let mut initial_state = ChronosaState::default();
        let pe1 = PrimeElement::new(
            "pe-a".to_string(),
            "uworld".to_string(),
            1.0,
            "A".to_string(),
            "".to_string(),
        );
        initial_state = test_evolve(initial_state, pe1.to_cdu());
        let pe2 = PrimeElement::new(
            "pe-b".to_string(),
            "uworld".to_string(),
            1.0,
            "B".to_string(),
            "".to_string(),
        );
        initial_state = test_evolve(initial_state, pe2.to_cdu());

        let theorem = Theorem {
            premises: vec!["pe-a".to_string(), "pe-b".to_string()],
            conclusion: "Conclusion C".to_string(),
            proof_path: vec![],
            confidence_score: 1.0,
        };
        let theorem_cdu = Cdu::from_payload(CduPayload::Theorem(theorem), "theorem.uworld", vec![]);
        initial_state = test_evolve(initial_state, theorem_cdu);

        // Also add an axiom that would otherwise fire, to prove the theorem takes precedence.
        let axiom1 = SemiAxiom::new(
            "axiom-1".to_string(),
            "uworld".to_string(),
            vec!["pe-a".to_string()],
            "This should not fire".to_string(),
        );
        initial_state = test_evolve(initial_state, axiom1.to_cdu());

        // 2. Create the projector.
        let projector = ReasoningProjector::new();
        let input = Cdu::new(b"trigger".to_vec(), "observation.uworld", vec![]);

        // 3. Project the input.
        let commands = projector.project(&initial_state, &input);

        // 4. Assert that exactly one command was generated, and it's from the theorem.
        assert_eq!(commands.len(), 1);
        assert!(commands[0].name.contains(".command.theorem"));
        assert_eq!(commands[0].metadata.causes[0], "Conclusion C");
    }
}
