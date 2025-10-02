// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/projector.rs

//! Defines projectors, the deterministic "brains" of the agent.

use crate::cdu::Cdu;
use crate::engine::Projector;
use crate::state::ChronosaState;

// --- FIX: Define type aliases for complex function traits ---
pub type Predicate = Box<dyn Fn(&ChronosaState, &Cdu) -> bool + Send>;
pub type Mapper = Box<dyn Fn(&Cdu) -> Vec<Cdu> + Send>;

/// A single, stateless rule for the projector to evaluate.
pub struct Rule {
    /// The predicate: a function that checks if the rule should fire.
    pub predicate: Predicate,
    /// The mapper: a function that creates the resulting command CDUs if the predicate is true.
    pub mapper: Mapper,
}

/// A projector that uses a list of rules to make decisions.
pub struct RuleBasedProjector {
    rules: Vec<Rule>,
}

impl RuleBasedProjector {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl Projector for RuleBasedProjector {
    /// Iterates through its rules and executes the first one whose predicate matches.
    fn project(&self, state: &ChronosaState, input: &Cdu) -> Vec<Cdu> {
        for rule in &self.rules {
            if (rule.predicate)(state, input) {
                return (rule.mapper)(input);
            }
        }
        // If no rules match, produce no new events.
        vec![]
    }
}
