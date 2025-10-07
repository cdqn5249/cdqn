// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/causal_mode.rs

//! Defines the CausalMode struct for Causal Tensor Decomposition.

/// Represents a decomposed vector of a high-level concept for CTD.
#[derive(Debug, Clone)]
pub struct CausalMode {
    /// The type of mode (e.g., "intent", "world_state").
    pub mode_type: String,
    /// The decomposed vector for this mode.
    pub vector: Vec<f64>,
    /// The original CDU that was decomposed to create this mode.
    pub source_cdu: String,
}

// All to_bytes and from_bytes logic has been removed.
// This will now be handled by the central codec.
