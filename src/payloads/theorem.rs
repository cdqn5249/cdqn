// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/theorem.rs

//! Defines the Theorem struct, a proven, reusable reasoning path.

/// Represents a proven, reusable reasoning path.
#[derive(Debug, Clone)]
pub struct Theorem {
    /// The set of PrimeElements that are the starting conditions.
    pub premises: Vec<String>,
    /// The PrimeElement that is the proven result.
    pub conclusion: String,
    /// The full, verifiable reasoning path of intermediate CDUs.
    pub proof_path: Vec<String>,
    /// A score representing the reliability of this theorem.
    pub confidence_score: f64,
}

// All to_bytes and from_bytes logic has been removed.
// This will now be handled by the central codec.
