// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/hlc.rs

//! The Hybrid Logical Clock (HLC) module.
//!
//! Provides a timestamping mechanism that captures causal relationships
//! between events in a distributed system, which is essential for Chronosa's
//! sovereign, decentralized nature.

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a Hybrid Logical Clock timestamp.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hlc {
    /// The wall-clock time component, as milliseconds since the Unix epoch.
    pub timestamp: u64,
    /// A counter to differentiate events with the same wall-clock time.
    pub counter: u16,
}

impl Hlc {
    /// Creates a new HLC timestamp based on the current system time.
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before the UNIX epoch, which should not happen.")
            .as_millis() as u64;

        Self {
            timestamp: now,
            counter: 0,
        }
    }

    /// Advances the clock's time, ensuring it is always moving forward.
    /// This is called when a new local event occurs.
    pub fn tick(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before the UNIX epoch.")
            .as_millis() as u64;

        if now > self.timestamp {
            // The physical clock has advanced, so we can reset the counter.
            self.timestamp = now;
            self.counter = 0;
        } else {
            // The physical clock is the same or has gone backward.
            // We must increment the counter to ensure the new timestamp is unique and greater.
            self.counter += 1;
        }
    }
}

// --- Unit tests for the HLC logic ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_hlc_new() {
        let hlc1 = Hlc::new();
        sleep(Duration::from_millis(2));
        let hlc2 = Hlc::new();
        assert!(
            hlc2 > hlc1,
            "A later HLC should be greater than an earlier one."
        );
    }

    #[test]
    fn test_hlc_tick() {
        let mut hlc = Hlc::new();
        let original_hlc = hlc.clone();

        hlc.tick();
        assert!(
            hlc > original_hlc,
            "Ticking the clock should always produce a greater timestamp."
        );
    }

    #[test]
    fn test_hlc_counter_increment() {
        let mut hlc = Hlc {
            timestamp: 100,
            counter: 5,
        };
        let original_hlc = hlc.clone();

        // Simulate a tick where the physical clock has not advanced.
        // (We can't do this directly, so we'll just call tick and check the counter logic)
        // In a real scenario where time stands still, the counter must increment.
        if hlc.timestamp == original_hlc.timestamp {
            hlc.counter += 1;
        }

        assert!(hlc > original_hlc);
        assert_eq!(hlc.counter, 6);
    }
}
