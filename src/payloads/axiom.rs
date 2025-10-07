// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/axiom.rs

//! Defines the Axiom struct, a rule verified across multiple worlds.

/// Represents a rule or transformation that has been verified to be valid
/// and consistent across multiple Worlds.
#[derive(Debug, Clone)]
pub struct Axiom {
    /// Unique identifier for the axiom.
    pub id: String,
    /// The list of worlds this axiom bridges.
    pub worlds: Vec<String>,
    /// The set of PrimeElements that act as premises.
    pub premises: Vec<String>,
    /// A description of the universal truth this axiom represents.
    pub description: String,
    /// The weight or confidence in this axiom.
    pub weight: f64,
}

// All to_bytes and from_bytes logic has been removed.
// This will now be handled by the central codec.
