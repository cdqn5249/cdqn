// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/state.rs

//! Defines the in-memory state of Chronosa and the pure function to evolve it.

use crate::cdu::Cdu;
use crate::hlc::Hlc;

/// The in-memory state of a Chronosa instance.
/// This is a pure projection of the event log.
#[derive(Debug, Clone, Default)]
pub struct ChronosaState {
    pub(crate) hlc: Hlc,
    pub(crate) log: Vec<Cdu>,
}

impl ChronosaState {
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
    pub fn find_last_by_subtype(&self, subtype: &str) -> Option<&Cdu> {
        let pattern = format!(".{}.cdu", subtype);
        self.log
            .iter()
            .rev()
            .find(|cdu| cdu.name.contains(&pattern))
    }
}

/// The "Evolver" or "Reducer" function.
/// A pure function that computes the next state from the current state and one event.
pub fn evolve(mut state: ChronosaState, event: Cdu) -> ChronosaState {
    // Ensure the state's clock is always up-to-date.
    if event.metadata.hlc > state.hlc {
        state.hlc = event.metadata.hlc.clone();
    }
    state.log.push(event);
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdu::Cdu;

    #[test]
    fn test_evolve() {
        let initial_state = ChronosaState::default();
        assert!(initial_state.is_empty());

        let event1 = Cdu::new(b"event 1".to_vec(), "type1", vec![]);
        let state1 = evolve(initial_state, event1);

        assert_eq!(state1.len(), 1);
        assert_eq!(state1.log()[0].payload, b"event 1".to_vec());

        let event2 = Cdu::new(b"event 2".to_vec(), "type2", vec![]);
        let state2 = evolve(state1, event2);
        assert_eq!(state2.len(), 2);
    }
}
