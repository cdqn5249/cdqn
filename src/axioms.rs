// Source code under BaDaaS license, vibe coding engine: Gemini Flash-Lite Latest, Google
// File path: src/axioms.rs

use super::cdu::{Cdu, ContentHash, CduType};
use super::hashing::Hash;

/// A simple axiom: If two CDUs share the same content hash, they are logically identical
/// in terms of content, regardless of time (HLC) or parent lineage.
///
/// This function checks if two CDUs are logically identical based on their content hash.
pub fn axiom_content_identity(cdu_a: &Cdu, cdu_b: &Cdu) -> bool {
    cdu_a.payload.content_hash == cdu_b.payload.content_hash
}

/// A new axiom check: Verifies that the two CDUs being compared are both of type Axiom.
pub fn axiom_type_check(cdu_a: &Cdu, cdu_b: &Cdu) -> bool {
    cdu_a.payload.cdu_type == CduType::Axiom && cdu_b.payload.cdu_type == CduType::Axiom
}
