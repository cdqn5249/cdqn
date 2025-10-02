// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/core.rs

//! The core module for the Chronosa agent.
//!
//! This module defines the `ChronosaCore`, which acts as the central log
//! and state manager for an agent instance.

use crate::cdu::Cdu;

/// Represents the core of a Chronosa agent instance.
///
/// It maintains a verifiable, append-only log of Causal Data Units (CDUs),
/// representing the agent's complete history of experiences.
#[derive(Debug)]
pub struct ChronosaCore {
    /// The immutable log of all CDUs ever recorded by this agent.
    log: Vec<Cdu>,
}

impl ChronosaCore {
    /// Creates a new, empty ChronosaCore.
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    /// Records a new event in the agent's log.
    ///
    /// This method takes a payload and subtype, creates a new CDU,
    /// and appends it to the immutable log.
    /// It returns a reference to the newly created CDU.
    pub fn record(&mut self, payload: Vec<u8>, subtype: &str) -> &Cdu {
        // For now, we assume new events are not directly caused by others.
        // We will add more complex causal linking later.
        let new_cdu = Cdu::new(payload, subtype, vec![]);
        self.log.push(new_cdu);
        // Return a reference to the last CDU added.
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

        // Verify that time moves forward
        assert!(core.log[1].metadata.hlc > core.log[0].metadata.hlc);
    }
}
