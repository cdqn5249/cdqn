// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

use crate::codec::{Decode, Encode}; // Import the core traits
use crate::hlc::Hlc;
use crate::payloads::{Axiom, CausalMode, Constraint, Theorem};
use sha3::{Digest, Sha3_256 as Sha256};

/// The mutable metadata associated with a Causal Data Unit.
#[derive(Debug, Clone)]
pub struct CduMetadata {
    pub hlc: Hlc,
    pub causes: Vec<String>,
    pub tags: Vec<String>,
}

/// A Causal Data Unit (CDU).
#[derive(Debug, Clone)]
pub struct Cdu {
    pub name: String,
    pub payload: Vec<u8>,
    pub metadata: CduMetadata,
}

/// Represents the structured content within a CDU payload.
#[derive(Debug, Clone)]
pub enum CduPayload {
    Raw(Vec<u8>),
    PrimeElement(crate::reasoning::PrimeElement),
    SemiAxiom(crate::reasoning::SemiAxiom),
    Axiom(Axiom),
    Theorem(Theorem),
    Constraint(Constraint),
    CausalMode(CausalMode),
}

/// Helper function to correctly serialize any payload type that implements Encode.
fn to_payload_bytes<T: Encode>(payload_struct: &T) -> Vec<u8> {
    let mut buffer = Vec::new();
    payload_struct.encode(&mut buffer);
    buffer
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
        // FIX: Use the new generic helper function.
        let payload_bytes = match &payload {
            CduPayload::Raw(bytes) => bytes.clone(),
            CduPayload::PrimeElement(element) => to_payload_bytes(element),
            CduPayload::SemiAxiom(axiom) => to_payload_bytes(axiom),
            CduPayload::Axiom(axiom) => to_payload_bytes(axiom),
            CduPayload::Theorem(theorem) => to_payload_bytes(theorem),
            CduPayload::Constraint(constraint) => to_payload_bytes(constraint),
            CduPayload::CausalMode(mode) => to_payload_bytes(mode),
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

        // FIX: Use the Decode trait for deserialization.
        let mut payload_slice = self.payload.as_slice();
        match primary_subtype.as_deref() {
            Some("prime") => Some(CduPayload::PrimeElement(
                crate::reasoning::PrimeElement::decode(&mut payload_slice),
            )),
            Some("semi-axiom") => Some(CduPayload::SemiAxiom(crate::reasoning::SemiAxiom::decode(
                &mut payload_slice,
            ))),
            Some("axiom") => Some(CduPayload::Axiom(Axiom::decode(&mut payload_slice))),
            Some("theorem") => Some(CduPayload::Theorem(Theorem::decode(&mut payload_slice))),
            Some("constraint") => Some(CduPayload::Constraint(Constraint::decode(
                &mut payload_slice,
            ))),
            Some("causal") => Some(CduPayload::CausalMode(CausalMode::decode(
                &mut payload_slice,
            ))),
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
