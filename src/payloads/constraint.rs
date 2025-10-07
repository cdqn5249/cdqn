// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/constraint.rs

//! Defines the Constraint struct, an emergent guardrail for reasoning.

/// Represents a guardrail; a rule that inhibits a reasoning path in a specific context.
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The reasoning path that is being inhibited.
    pub target_path: Vec<String>,
    /// The context in which the path is invalid.
    pub inhibiting_context: String,
    /// The reason for the inhibition (e.g., "LeadsToImpossibility").
    pub reason: String,
    /// The world in which this constraint applies.
    pub world: String,
}

// All to_bytes and from_bytes logic has been removed.
// This will now be handled by the central codec.
