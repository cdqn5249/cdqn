// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/orchestrator.rs

//! The orchestrator module for Chronosa.
//!//! This module contains the primary decision-making loop for the agent.

use crate::core::ChronosaCore;

/// The Orchestrator is responsible for the agent's main "thought loop".
/// It owns the agent's memory (the ChronosaCore) and uses it to decide on actions.
#[derive(Debug, Clone)]
pub struct Orchestrator {
    core: ChronosaCore,
}

impl Orchestrator {
    /// Creates a new Orchestrator with a fresh ChronosaCore.
    pub fn new() -> Self {
        Self {
            core: ChronosaCore::new(),
        }
    }

    /// Represents one "thought cycle" of the agent.
    ///
    // This pure function takes the current state of the orchestrator and returns
    /// a new state reflecting the decisions and actions taken during the cycle.
    pub fn tick(self) -> Self {
        // 1. Decide on an action based on the last observation.
        let last_observation = self.core.find_last_by_subtype("observation");

        // If there is a last observation, decide what to do.
        if let Some(observation) = last_observation {
            let action_payload = match observation.payload.as_slice() {
                b"see enemy" => b"attack".to_vec(),
                b"see food" => b"eat".to_vec(),
                _ => b"idle".to_vec(), // Default action
            };

            // 2. Record the chosen action, causally linked to the observation.
            let (new_core, _) =
                self.core
                    .record_causal(action_payload, "action", &[observation]);

            // 3. Return the new state of the orchestrator.
            return Self { core: new_core };
        }

        // If there's nothing to observe, return the unchanged state.
        self
    }

    /// A getter to access the orchestrator's core for inspection.
    pub fn core(&self) -> &ChronosaCore {
        &self.core
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// --- Unit tests for the Orchestrator ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_tick_decision() {
        // 1. Create an orchestrator and record an observation.
        let orchestrator = Orchestrator::new();
        let (core, _) = orchestrator
            .core
            .clone()
            .record(b"see enemy".to_vec(), "observation");
        let orchestrator = Orchestrator { core };

        // 2. Run one thought cycle.
        let new_orchestrator = orchestrator.tick();

        // 3. Verify that the correct action was taken.
        let last_action = new_orchestrator.core.find_last_by_subtype("action");
        assert!(last_action.is_some());
        assert_eq!(last_action.unwrap().payload, b"attack".to_vec());

        // 4. Verify the causal link.
        let last_observation = new_orchestrator.core.find_last_by_subtype("observation");
        assert_eq!(
            last_action.unwrap().metadata.causes[0],
            last_observation.unwrap().name
        );
    }
}
