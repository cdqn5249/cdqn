// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

// Import the Hlc struct from our own hlc module.
use crate::hlc::Hlc;

/// The mutable metadata associated with a Causal Data Unit.
/// This contains Chronosa's current interpretation and context of the immutable payload.
#[derive(Debug, Clone)]
pub struct CduMetadata {
    /// The Hybrid Logical Clock timestamp for when this CDU was created or last updated.
    pub hlc: Hlc,
    /// A list of CDU names that are the direct causes for this CDU's existence.
    pub causes: Vec<String>,
    /// User-defined tags for querying and classification.
    pub tags: Vec<String>,
}

/// A Causal Data Unit (CDU).
/// It combines an immutable fact (payload) with mutable, evolving context (metadata).
#[derive(Debug, Clone)]
pub struct Cdu {
    /// The unique, content-addressed name of the CDU.
    /// Format: "<payload_hash>.<subtype>.cdu"
    pub name: String,
    /// The immutable, verifiable data payload. This is the "fact" of what happened.
    /// Represented as a raw byte vector to be maximally generic.
    pub payload: Vec<u8>,
    /// The mutable metadata, representing Chronosa's understanding of the payload.
    pub metadata: CduMetadata,
}
