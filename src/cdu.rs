// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

// Import the Hlc struct from our own hlc module using the 'crate' path.
use crate::hlc::Hlc;
// Import the necessary traits for hashing from the sha2 crate.
use sha2::{Digest, Sha256};

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

/// Represents the structured content within a CDU payload.
/// This enum allows for type-safe handling of different kinds of information.
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
}

impl Cdu {
    /// Creates a new Causal Data Unit.
    ///
    /// The CDU's name is deterministically generated from the SHA-256 hash of its payload,
    /// and its metadata is timestamped with a new Hybrid Logical Clock value.
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

    /// Creates a new CDU from a CduPayload
    pub fn from_payload(payload: CduPayload, subtype: &str, causes: Vec<String>) -> Self {
        let payload_bytes = match payload {
            CduPayload::Raw(bytes) => bytes,
            CduPayload::PrimeElement(element) => element.to_bytes(),
            CduPayload::SemiAxiom(axiom) => axiom.to_bytes(),
            CduPayload::Theorem(theorem) => theorem.to_bytes(),
            CduPayload::Constraint(constraint) => constraint.to_bytes(),
        };
        Self::new(payload_bytes, subtype, causes)
    }

    /// Extracts the structured content from the CDU payload
    pub fn extract_payload(&self) -> Option<CduPayload> {
        if self.name.contains(".prime.element.") {
            crate::reasoning::PrimeElement::from_bytes(&self.payload).map(CduPayload::PrimeElement)
        } else if self.name.contains(".semi-axiom.") {
            crate::reasoning::SemiAxiom::from_bytes(&self.payload).map(CduPayload::SemiAxiom)
        } else if self.name.contains(".theorem.") {
            Theorem::from_bytes(&self.payload).map(CduPayload::Theorem)
        } else if self.name.contains(".constraint.") {
            Constraint::from_bytes(&self.payload).map(CduPayload::Constraint)
        } else {
            Some(CduPayload::Raw(self.payload.clone()))
        }
    }
}

// --- Serialization for new structs ---

impl Theorem {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // A simple helper for writing a Vec<String>
        let write_vec_string = |bytes: &mut Vec<u8>, vec: &Vec<String>| {
            bytes.extend_from_slice(&(vec.len() as u32).to_le_bytes());
            for item in vec {
                bytes.extend_from_slice(&(item.len() as u32).to_le_bytes());
                bytes.extend_from_slice(item.as_bytes());
            }
        };
        write_vec_string(&mut bytes, &self.premises);
        bytes.extend_from_slice(&(self.conclusion.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.conclusion.as_bytes());
        write_vec_string(&mut bytes, &self.proof_path);
        bytes.extend_from_slice(&self.confidence_score.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        // A simple helper for reading a Vec<String>
        let read_vec_string = |bytes: &[u8], pos: &mut usize| -> Option<Vec<String>> {
            if *pos + 4 > bytes.len() { return None; }
            let vec_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
            *pos += 4;
            let mut vec = Vec::with_capacity(vec_len);
            for _ in 0..vec_len {
                if *pos + 4 > bytes.len() { return None; }
                let str_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
                *pos += 4;
                if *pos + str_len > bytes.len() { return None; }
                let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
                *pos += str_len;
                vec.push(item);
            }
            Some(vec)
        };
        let premises = read_vec_string(bytes, &mut pos)?;
        if pos + 4 > bytes.len() { return None; }
        let conclusion_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        if pos + conclusion_len > bytes.len() { return None; }
        let conclusion = String::from_utf8(bytes[pos..pos + conclusion_len].to_vec()).ok()?;
        pos += conclusion_len;
        let proof_path = read_vec_string(bytes, &mut pos)?;
        if pos + 8 > bytes.len() { return None; }
        let confidence_score = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        Some(Theorem { premises, conclusion, proof_path, confidence_score })
    }
}

impl Constraint {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let write_vec_string = |bytes: &mut Vec<u8>, vec: &Vec<String>| {
            bytes.extend_from_slice(&(vec.len() as u32).to_le_bytes());
            for item in vec {
                bytes.extend_from_slice(&(item.len() as u32).to_le_bytes());
                bytes.extend_from_slice(item.as_bytes());
            }
        };
        write_vec_string(&mut bytes, &self.target_path);
        bytes.extend_from_slice(&(self.inhibiting_context.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.inhibiting_context.as_bytes());
        bytes.extend_from_slice(&(self.reason.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.reason.as_bytes());
        bytes.extend_from_slice(&(self.world.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.world.as_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let read_vec_string = |bytes: &[u8], pos: &mut usize| -> Option<Vec<String>> {
            if *pos + 4 > bytes.len() { return None; }
            let vec_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
            *pos += 4;
            let mut vec = Vec::with_capacity(vec_len);
            for _ in 0..vec_len {
                if *pos + 4 > bytes.len() { return None; }
                let str_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
                *pos += 4;
                if *pos + str_len > bytes.len() { return None; }
                let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
                *pos += str_len;
                vec.push(item);
            }
            Some(vec)
        };
        let read_string = |bytes: &[u8], pos: &mut usize| -> Option<String> {
            if *pos + 4 > bytes.len() { return None; }
            let str_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
            *pos += 4;
            if *pos + str_len > bytes.len() { return None; }
            let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
            *pos += str_len;
            Some(item)
        };
        let target_path = read_vec_string(bytes, &mut pos)?;
        let inhibiting_context = read_string(bytes, &mut pos)?;
        let reason = read_string(bytes, &mut pos)?;
        let world = read_string(bytes, &mut pos)?;
        Some(Constraint { target_path, inhibiting_context, reason, world })
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
