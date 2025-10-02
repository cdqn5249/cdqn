// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/core.rs

//! The core module for the Chronosa agent.
//!
//! This module defines the `ChronosaCore`, which acts as the central log
//! and state manager for an agent instance.

use crate::cdu::Cdu;
use crate::hlc::Hlc;

/// Represents the core of a Chronosa agent instance.
///
/// It maintains a verifiable, append-only log of Causal Data Units (CDUs),
/// representing the agent's complete history of experiences.
#[derive(Debug)]
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
            hlc: Hlc::new(), // Initialize the core's own clock.
            log: Vec::new(),
        }
    }

    /// Records a new, non-causal event in the agent's log.
    /// This is a convenience wrapper around `record_causal`.
    pub fn record(&mut self, payload: Vec<u8>, subtype: &str) -> &Cdu {
        self.record_causal(payload, subtype, &[])
    }

    /// Records a new event that is causally linked to previous events.
    ///
    /// This is the primary method for creating new experiences in the log. It advances
    /// the core's internal clock and uses the provided causes to build the causal graph.
    /// It returns a reference to the newly created CDU.
    pub fn record_causal(&mut self, payload: Vec<u8>, subtype: &str, causes: &[&Cdu]) -> &Cdu {
        // 1. Advance the core's internal clock.
        self.hlc.tick();

        // 2. Collect the names of the cause CDUs.
        let cause_names = causes.iter().map(|cdu| cdu.name.clone()).collect();

        // 3. Create a new CDU with the collected cause names.
        let mut new_cdu = Cdu::new(payload, subtype, cause_names);

        // 4. Assign the core's official, ticked timestamp to the new CDU.
        new_cdu.metadata.hlc = self.hlc.clone();

        // 5. Push the finalized CDU to the log.
        self.log.push(new_cdu);

        // 6. Return a reference to the last CDU added.
        self.log.last().unwrap()
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
        assert!(core.log.is_empty(), "A new core should have an empty log.");
    }

    #[test]
    fn test_core_record() {
        let mut core = ChronosaCore::new();
        core.record(b"First event".to_vec(), "genesis");
        core.record(b"Second event".to_vec(), "observation");
        assert_eq!(core.log.len(), 2);
        assert!(core.log[1].metadata.hlc > core.log[0].metadata.hlc);
    }

    #[test]
    fn test_core_record_causal() {
        let mut core = ChronosaCore::new();

        // 1. Record a genesis event.
        let cause_cdu = core.record(b"Initial observation".to_vec(), "observation");
        let cause_name = cause_cdu.name.clone(); // Clone name before mutable borrow

        // 2. Record a new event that was caused by the first one.
        let effect_cdu = core.record_causal(
            b"Agent performs an action".to_vec(),
            "action",
            &[&cause_cdu],
        );

        // 3. Verify the log state.
        assert_eq!(core.log.len(), 2);
        assert_eq!(effect_cdu.metadata.causes.len(), 1);
        assert_eq!(effect_cdu.metadata.causes[0], cause_name);
        assert!(effect_cdu.metadata.hlc > core.log[0].metadata.hlc);
    }
}
