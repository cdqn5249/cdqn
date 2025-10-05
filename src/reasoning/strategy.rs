// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/strategy.rs

//! Defines the Strategy pattern for modular reasoning components.

use crate::cdu::Cdu;
use crate::reasoning::knowledge_base::KnowledgeBase;
use crate::reasoning::SemiAxiom;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;
use std::thread;

/// Defines the "horizon of similarity".
const SIMILARITY_EPSILON: f64 = 0.1;

/// A container for data passed between strategies in the reasoning pipeline.
/// It is mutable, allowing each strategy to modify the state for the next.
pub struct ReasoningContext<'a> {
    /// The immutable knowledge snapshot for this cycle.
    pub kb: &'a KnowledgeBase,
    /// The input CDU that triggered the reasoning.
    pub input: &'a Cdu,
    /// The list of axioms that are candidates for evaluation.
    /// Strategies can filter this list.
    pub candidate_axioms: Vec<SemiAxiom>,
    /// The final list of generated command CDUs.
    pub commands: Vec<Cdu>,
    /// A flag to indicate if a strategy has produced a final result,
    /// allowing the pipeline to terminate early.
    pub is_conclusive: bool,
}

impl<'a> ReasoningContext<'a> {
    pub fn new(kb: &'a KnowledgeBase, input: &'a Cdu) -> Self {
        Self {
            kb,
            input,
            candidate_axioms: kb.semi_axioms().to_vec(),
            commands: Vec::new(),
            is_conclusive: false,
        }
    }
}

/// The trait that all reasoning strategies must implement.
pub trait ReasoningStrategy {
    fn execute(&self, context: &mut ReasoningContext);
}

// --- Concrete Strategy Implementations ---

/// **Strategy 1: Theorem Shortcut**
/// Checks if a theorem's premises are met. If so, it generates a command
/// and sets the context to conclusive to stop the pipeline.
pub struct TheoremStrategy;
impl ReasoningStrategy for TheoremStrategy {
    fn execute(&self, context: &mut ReasoningContext) {
        let known_element_ids: HashSet<_> = context.kb.prime_elements().keys().cloned().collect();
        for theorem in context.kb.theorems() {
            let premises_set: HashSet<_> = theorem.premises.iter().cloned().collect();
            if premises_set.is_subset(&known_element_ids) {
                println!(
                    "Shortcut: Theorem conclusion '{}' applied.",
                    theorem.conclusion
                );
                let command_payload =
                    format!("Command from theorem: {}", theorem.conclusion).into_bytes();
                let command_cdu = Cdu::new(
                    command_payload,
                    "command.theorem",
                    vec![theorem.conclusion.clone()],
                );
                context.commands.push(command_cdu);
                context.is_conclusive = true;
                return; // Stop after the first matching theorem.
            }
        }
    }
}

/// **Strategy 2: Constraint Filtering**
/// Filters the list of candidate axioms based on active constraints.
pub struct ConstraintStrategy;
impl ReasoningStrategy for ConstraintStrategy {
    fn execute(&self, context: &mut ReasoningContext) {
        let mut allowed_axioms = Vec::new();

        // FIX: Robustly extract the last part of the subtype as the context.
        let name_parts: Vec<&str> = context.input.name.split('.').collect();
        let input_context_str = if name_parts.len() > 2 {
            name_parts[name_parts.len() - 2]
        } else {
            "" // No context found
        };

        if let Some(input_context_pe) = context.kb.prime_elements().get(input_context_str) {
            'axiom_loop: for axiom in &context.candidate_axioms {
                for constraint in context.kb.constraints() {
                    if constraint.target_path.contains(&axiom.id) {
                        if let Some(constraint_context_pe) = context
                            .kb
                            .prime_elements()
                            .get(&constraint.inhibiting_context)
                        {
                            let distance = (input_context_pe.representation
                                - constraint_context_pe.representation)
                                .abs();
                            if distance < SIMILARITY_EPSILON {
                                println!(
                                    "Inhibition: Axiom '{}' pruned by constraint for context '{}' (distance: {:.4}).",
                                    axiom.id, constraint.inhibiting_context, distance
                                );
                                continue 'axiom_loop;
                            }
                        }
                    }
                }
                allowed_axioms.push(axiom.clone());
            }
        } else {
            // If the input context is unknown, no constraints can be checked.
            allowed_axioms = context.candidate_axioms.clone();
        }
        context.candidate_axioms = allowed_axioms;
    }
}

/// **Strategy 3: Axiom Evaluation**
/// Takes the final list of candidate axioms and evaluates them concurrently.
pub struct AxiomEvaluationStrategy;
impl ReasoningStrategy for AxiomEvaluationStrategy {
    fn execute(&self, context: &mut ReasoningContext) {
        if context.candidate_axioms.is_empty() {
            return;
        }

        let (sender, receiver) = mpsc::channel();
        let mut handles = vec![];

        for axiom in &context.candidate_axioms {
            let sender_clone = sender.clone();
            let input_clone = context.input.clone();
            // Clone the necessary parts of the KB for the thread.
            let prime_elements_clone = context.kb.prime_elements().clone();
            let axiom_clone = axiom.clone();

            let handle = thread::spawn(move || {
                let results =
                    Self::evaluate_single_axiom(&axiom_clone, &input_clone, &prime_elements_clone);
                for result in results {
                    if sender_clone.send(result).is_err() {
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        drop(sender);

        for command in receiver {
            context.commands.push(command);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

impl AxiomEvaluationStrategy {
    /// The core logic for evaluating a single axiom.
    fn evaluate_single_axiom(
        axiom: &SemiAxiom,
        _input: &Cdu,
        prime_elements: &HashMap<String, crate::reasoning::PrimeElement>,
    ) -> Vec<Cdu> {
        for element_id in &axiom.prime_elements {
            if !prime_elements.contains_key(element_id) {
                return vec![];
            }
        }

        let command_payload = format!("Command from axiom: {}", axiom.description).into_bytes();
        vec![Cdu::new(
            command_payload,
            "command.reasoned",
            vec![axiom.id.clone()],
        )]
    }
}
