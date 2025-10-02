// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/hlc.rs

//! The Hybrid Logical Clock (HLC) module.
//!
//! Provides a timestamping mechanism that captures causal relationships
//! between events in a distributed system, which is essential for Chronosa's
//! sovereign, decentralized nature.

/// Represents a Hybrid Logical Clock timestamp.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hlc {
    /// The wall-clock time component, typically a Unix timestamp in milliseconds.
    pub timestamp: u64,
    /// A counter to differentiate events with the same wall-clock time.
    pub counter: u16,
}
