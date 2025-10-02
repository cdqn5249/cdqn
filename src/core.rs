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
    hlc: Hlc,
    /// The immutable log of all CDUs ever recorded by this agent.
    log: Vec<Cdu>,
}

impl ChronosaCore {
    /// Creates a new, empty ChronosaCore.
    pub fn new() -> Self {
        Self {
            hlc: Hlc::new(),
            log: Vec::new(),
        }
    }

    /// Records a new, non-causal event, returning a new state of the core.
    /// This is a convenience wrapper around `record_causal`.
    pub fn record(self, payload: Vec<u8>, subtype: &str) -> (Self, Cdu) {
        self.record_causal(payload, subtype, &[])
    }

    /// Records a new causal event, returning a new state of the core.
    ///
    /// This is a pure function that takes the current core state and returns a new
    /// core state with the event added. It does not mutate the original core.
    pub fn record_causal(self, payload: Vec<u8>, subtype: &str, causes: &[&Cdu]) -> (Self, Cdu) {
        // 1. Create the next state for the HLC.
        let mut new_hlc = self.hlc;
        new_hlc.tick();

        // 2. Collect the names of the cause CDUs.
        let cause_names = causes.iter().map(|cdu| cdu.name.clone()).collect();

        // 3. Create a new CDU with the collected cause names.
        let mut new_cdu = Cdu::new(payload, subtype, cause_names);

        // 4. Assign the core's official, ticked timestamp to the new CDU.
        new_cdu.metadata.hlc = new_hlc.clone();

        // 5. Create the next state for the log.
        let mut new_log = self.log;
        new_log.push(new_cdu.clone()); // Push a clone of the CDU to the new log

        // 6. Construct and return the new core state and the created CDU.
        (
            Self {
                hlc: new_hlc,
                log: new_log,
            },
            new_cdu,
        )
    }

    // --- GETTER METHODS ---

    /// Returns a slice providing read-only access to the entire CDU log.
    pub fn log(&self) -> &[Cdu] {
        &self.log
    }

    /// Returns the number of CDUs currently in the log.
    pub fn len(&self) -> usize {
        self.log.len()
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

    #[test]
    fn test_core_new() {
        let core = ChronosaCore::new();
        assert!(core.log().is_empty(), "A new core should have an empty log.");
    }

    #[test]
    fn test_core_record() {
        let core = ChronosaCore::new();
        let (core, _) = core.record(b"First event".to_vec(), "genesis");
        let (core, _) = core.record(b"Second event".to_vec(), "observation");

        assert_eq!(core.len(), 2);
        assert!(core.log()[1].metadata.hlc > core.log()[0].metadata.hlc);
    }

    #[test]
    fn test_core_record_causal() {
        let core = ChronosaCore::new();

        // 1. Record a genesis event. `core` is consumed and a new `core` is returned.
        let (core, cause_cdu) = core.record(b"Initial observation".to_vec(), "observation");

        // 2. Record a new event that was caused by the first one.
        let (core, effect_cdu) = core.record_causal(
            b"Agent performs an action".to_vec(),
            "action",
            &[&cause_cdu],
        );

        // 3. Verify the final state of the core.
        assert_eq!(core.len(), 2);
        assert_eq!(effect_cdu.metadata.causes.len(), 1);
        assert_eq!(effect_cdu.metadata.causes[0], cause_cdu.name);
        assert!(effect_cdu.metadata.hlc > core.log()[0].metadata.hlc);
    }
}
