// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/core.rs

//! The core module for the Chronosa agent.
//!
//! This module defines the `ChronosaCore`, which acts as the central log
//! and state manager for an agent instance.

use crate::cdu::Cdu;
use crate::hlc::Hlc; // Import the Hlc

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

    /// Records a new event in the agent's log.
    ///
    /// This method advances the core's internal clock and uses that new timestamp
    /// to create and record a new CDU.
    /// It returns a reference to the newly created CDU.
    pub fn record(&mut self, payload: Vec<u8>, subtype: &str) -> &Cdu {
        // 1. Advance the core's internal clock. This is the crucial step.
        self.hlc.tick();

        // 2. Create a new CDU. We will then overwrite its placeholder HLC.
        let mut new_cdu = Cdu::new(payload, subtype, vec![]);

        // 3. Assign the core's official, ticked timestamp to the new CDU.
        new_cdu.metadata.hlc = self.hlc.clone();

        // 4. Push the finalized CDU to the log.
        self.log.push(new_cdu);

        // 5. Return a reference to the last CDU added.
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
        assert_eq!(core.log.len(), 0);

        core.record(b"First event".to_vec(), "genesis");
        assert_eq!(core.log.len(), 1);
        assert_eq!(core.log[0].payload, b"First event".to_vec());

        core.record(b"Second event".to_vec(), "observation");
        assert_eq!(core.log.len(), 2);
        assert_eq!(core.log[1].payload, b"Second event".to_vec());

        // This assertion will now pass because the core ticks its internal clock.
        assert!(core.log[1].metadata.hlc > core.log[0].metadata.hlc);
    }
}
