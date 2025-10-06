// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

use crate::hlc::Hlc;
use crate::payloads::{CausalMode, Constraint, Theorem};
use sha2::{Digest, Sha256};

/// The mutable metadata associated with a Causal Data Unit.
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
    pub payload: Vec<u8>,
    /// The mutable metadata, representing Chronosa's understanding of the payload.
    pub metadata: CduMetadata,
}

/// Represents the structured content within a CDU payload.
/// This enum now refers to types defined in the `payloads` module.
#[derive(Debug, Clone)]
pub enum CduPayload {
    /// A raw byte payload for maximum flexibility.
    Raw(Vec<u8>),
    /// A prime element in Chronosa's reasoning model.
    PrimeElement(crate::reasoning::PrimeElement),
    /// A semi-axiom as a prime ideal of prime elements.
    SemiAxiom(crate::reasoning::SemiAxiom),
    /// A proven, reusable reasoning path.
    Theorem(Theorem),
    /// A guardrail that inhibits a reasoning path in a specific context.
    Constraint(Constraint),
    /// A decomposed vector for Causal Tensor Decomposition.
    CausalMode(CausalMode),
}

impl Cdu {
    /// Creates a new Causal Data Unit.
    pub fn new(payload: Vec<u8>, subtype: &str, causes: Vec<String>) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let hash_result = hasher.finalize();
        let payload_hash = format!("{:x}", hash_result);
        let name = format!("{}.{}.cdu", payload_hash, subtype);

        let metadata = CduMetadata {
            hlc: Hlc::new(),
            causes,
            tags: vec![],
        };

        Self {
            name,
            payload,
            metadata,
        }
    }

    /// Creates a new CDU from a CduPayload.
    /// This now calls the `to_bytes` methods on the payload types.
    pub fn from_payload(payload: CduPayload, subtype: &str, causes: Vec<String>) -> Self {
        let payload_bytes = match payload {
            CduPayload::Raw(bytes) => bytes,
            CduPayload::PrimeElement(element) => element.to_bytes(),
            CduPayload::SemiAxiom(axiom) => axiom.to_bytes(),
            CduPayload::Theorem(theorem) => theorem.to_bytes(),
            CduPayload::Constraint(constraint) => constraint.to_bytes(),
            CduPayload::CausalMode(mode) => mode.to_bytes(),
        };
        Self::new(payload_bytes, subtype, causes)
    }

    /// Extracts the structured content from the CDU payload.
    /// This now calls the `from_bytes` methods on the payload types.
    pub fn extract_payload(&self) -> Option<CduPayload> {
        if self.name.contains(".prime.element.") {
            crate::reasoning::PrimeElement::from_bytes(&self.payload).map(CduPayload::PrimeElement)
        } else if self.name.contains(".semi-axiom.") {
            crate::reasoning::SemiAxiom::from_bytes(&self.payload).map(CduPayload::SemiAxiom)
        } else if self.name.contains(".theorem.") {
            Theorem::from_bytes(&self.payload).map(CduPayload::Theorem)
        } else if self.name.contains(".constraint.") {
            Constraint::from_bytes(&self.payload).map(CduPayload::Constraint)
        } else if self.name.contains(".causal.mode.") {
            CausalMode::from_bytes(&self.payload).map(CduPayload::CausalMode)
        } else {
            Some(CduPayload::Raw(self.payload.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdu_creation_and_naming() {
        let payload = b"Test payload data".to_vec();
        let subtype = "test_event";
        let cdu = Cdu::new(payload.clone(), subtype, vec![]);

        assert!(cdu.name.contains(subtype));
        assert!(cdu.name.ends_with(".cdu"));
        assert_ne!(cdu.name, format!(".{}.cdu", subtype));
        assert_eq!(cdu.payload, payload);
        assert_ne!(cdu.metadata.hlc.timestamp, 0);
    }

    #[test]
    fn test_cdu_causal_link() {
        let cause_cdu = Cdu::new(b"First event".to_vec(), "genesis", vec![]);
        let effect_cdu = Cdu::new(
            b"Second event".to_vec(),
            "response",
            vec![cause_cdu.name.clone()],
        );

        assert_eq!(effect_cdu.metadata.causes.len(), 1);
        assert_eq!(effect_cdu.metadata.causes[0], cause_cdu.name);
        assert!(effect_cdu.metadata.hlc >= cause_cdu.metadata.hlc);
    }
}
