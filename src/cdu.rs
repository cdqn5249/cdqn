// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

use crate::hlc::Hlc;
use crate::payloads::{Axiom, CausalMode, Constraint, Theorem}; // Import the new Axiom struct
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
#[derive(Debug, Clone)]
pub struct Cdu {
    /// The unique, content-addressed name of the CDU.
    pub name: String,
    /// The immutable, verifiable data payload.
    pub payload: Vec<u8>,
    /// The mutable metadata, representing Chronosa's understanding of the payload.
    pub metadata: CduMetadata,
}

/// Represents the structured content within a CDU payload.
#[derive(Debug, Clone)]
pub enum CduPayload {
    Raw(Vec<u8>),
    PrimeElement(crate::reasoning::PrimeElement),
    SemiAxiom(crate::reasoning::SemiAxiom),
    Axiom(Axiom), // New variant
    Theorem(Theorem),
    Constraint(Constraint),
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
    pub fn from_payload(payload: CduPayload, subtype: &str, causes: Vec<String>) -> Self {
        let payload_bytes = match payload {
            CduPayload::Raw(bytes) => bytes,
            CduPayload::PrimeElement(element) => element.to_bytes(),
            CduPayload::SemiAxiom(axiom) => axiom.to_bytes(),
            CduPayload::Axiom(axiom) => axiom.to_bytes(), // New match arm
            CduPayload::Theorem(theorem) => theorem.to_bytes(),
            CduPayload::Constraint(constraint) => constraint.to_bytes(),
            CduPayload::CausalMode(mode) => mode.to_bytes(),
        };
        Self::new(payload_bytes, subtype, causes)
    }

    /// Extracts the subtype string from the CDU name.
    fn get_subtype(&self) -> Option<String> {
        let name_without_suffix = self.name.strip_suffix(".cdu")?;
        let parts: Vec<&str> = name_without_suffix.split('.').collect();
        if parts.len() > 1 {
            Some(parts[1..].join("."))
        } else {
            None
        }
    }

    /// Extracts the structured content from the CDU payload using robust subtype matching.
    pub fn extract_payload(&self) -> Option<CduPayload> {
        let primary_subtype = self
            .get_subtype()
            .and_then(|s| s.split('.').next().map(|s_slice| s_slice.to_string()));

        match primary_subtype.as_deref() {
            Some("prime") => crate::reasoning::PrimeElement::from_bytes(&self.payload)
                .map(CduPayload::PrimeElement),
            Some("semi-axiom") => {
                crate::reasoning::SemiAxiom::from_bytes(&self.payload).map(CduPayload::SemiAxiom)
            }
            Some("axiom") => Axiom::from_bytes(&self.payload).map(CduPayload::Axiom), // New match arm
            Some("theorem") => Theorem::from_bytes(&self.payload).map(CduPayload::Theorem),
            Some("constraint") => Constraint::from_bytes(&self.payload).map(CduPayload::Constraint),
            Some("causal") => CausalMode::from_bytes(&self.payload).map(CduPayload::CausalMode),
            _ => Some(CduPayload::Raw(self.payload.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdu_creation_and_naming() {
        let payload1 = b"Test payload data".to_vec();
        let payload2 = b"Test payload data".to_vec();
        let cdu1 = Cdu::new(payload1, "test", vec![]);
        let cdu2 = Cdu::new(payload2, "test", vec![]);

        assert_eq!(cdu1.name, cdu2.name);
        assert!(cdu1.name.contains(".test.cdu"));
        assert_ne!(cdu1.metadata.hlc.timestamp, 0);
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
    }
}
