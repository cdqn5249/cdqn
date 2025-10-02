// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/core.rs

//! The core module for the Chronosa agent.
//!
//! This module defines the `ChronosaCore`, which acts as the central log
//! and state manager for an agent instance, using a functional,
//! state-transformational approach.

use crate::cdu::Cdu;
use crate::hlc::Hlc;

/// Represents the core of a Chronosa agent instance.
///
/// It maintains a verifiable, append-only log of Causal Data Units (CDUs),
/// representing the agent's complete history of experiences.
#[derive(Debug, Clone)]
pub struct ChronosaCore {
    /// The agent's internal, stateful Hybrid Logical Clock.
    pub(crate) hlc: Hlc,
    /// The immutable log of all CDUs ever recorded by this agent.
    pub(crate) log: Vec<Cdu>,
}

impl ChronosaCore {
    // ... (keep all existing methods: new, record, record_causal) ...
    // ...

    // --- GETTER AND QUERY METHODS ---

    /// Returns a slice providing read-only access to the entire CDU log.
    pub fn log(&self) -> &[Cdu] {
        &self.log
    }

    /// Returns the number of CDUs currently in the log.
    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// Returns `true` if the log contains no CDUs.
    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    /// Finds the most recent CDU in the log that matches a given subtype.
    ///
    /// This method iterates backward through the log for efficiency.
    pub fn find_last_by_subtype(&self, subtype: &str) -> Option<&Cdu> {
        let pattern = format!(".{}.cdu", subtype);
        self.log.iter().rev().find(|cdu| cdu.name.contains(&pattern))
    }
}

impl Default for ChronosaCore {
    fn default() -> Self {
        Self::new()
    }
}

// --- Unit tests for the ChronosaCore ---
#[cfg(test)]
mod tests {
    use super::*;

    // ... (keep all existing tests: test_core_new, test_core_record, test_core_record_causal) ...
    // ...

    #[test]
    fn test_core_find_last_by_subtype() {
        let core = ChronosaCore::new();
        let (core, _) = core.record(b"Observation 1".to_vec(), "observation");
        let (core, action1) = core.record(b"Action 1".to_vec(), "action");
        let (core, _) = core.record(b"Observation 2".to_vec(), "observation");

        // 1. Find the last action.
        let last_action = core.find_last_by_subtype("action");
        assert!(last_action.is_some());
        assert_eq!(last_action.unwrap().name, action1.name);

        // 2. Find the last observation.
        let last_observation = core.find_last_by_subtype("observation");
        assert!(last_observation.is_some());
        assert_eq!(
            last_observation.unwrap().payload,
            b"Observation 2".to_vec()
        );

        // 3. Search for a subtype that doesn't exist.
        let no_result = core.find_last_by_subtype("nonexistent");
        assert!(no_result.is_none());
    }
}
