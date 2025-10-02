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
    // --- FIX: Make fields public within the crate ---
    /// The agent's internal, stateful Hybrid Logical Clock.
    pub(crate) hlc: Hlc,
    /// The immutable log of all CDUs ever recorded by this agent.
    pub(crate) log: Vec<Cdu>,
}

// (The rest of the file remains exactly the same)
// ...
impl ChronosaCore {
// ...
// ...
}

impl Default for ChronosaCore {
// ...
}

#[cfg(test)]
mod tests {
// ...
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
