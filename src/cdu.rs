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

impl Cdu {
    /// Creates a new Causal Data Unit.
    ///
    /// The CDU's name is deterministically generated from the SHA-256 hash of its payload,
    /// and its metadata is timestamped with a new Hybrid Logical Clock value.
    pub fn new(payload: Vec<u8>, subtype: &str, causes: Vec<String>) -> Self {
        // 1. Create a new SHA-256 hasher.
        let mut hasher = Sha256::new();
        // 2. Write the payload data into the hasher.
        hasher.update(&payload);
        // 3. Finalize the hash and get the result as a byte array.
        let hash_result = hasher.finalize();
        // 4. Format the hash as a hexadecimal string.
        let payload_hash = format!("{:x}", hash_result);

        // 5. Construct the full CDU name.
        let name = format!("{}.{}.cdu", payload_hash, subtype);

        // 6. Create the metadata, now with a real HLC timestamp.
        let metadata = CduMetadata {
            hlc: Hlc::new(), // <-- This now calls our functional HLC constructor.
            causes,
            tags: vec![],
        };

        // 7. Return the newly constructed CDU.
        Self {
            name,
            payload,
            metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module (cdu)

    #[test]
    fn test_cdu_creation_and_naming() {
        // 1. Define a sample payload and subtype.
        let payload = b"Test payload data".to_vec();
        let subtype = "test_event";

        // 2. Create the CDU.
        let cdu = Cdu::new(payload.clone(), subtype, vec![]);

        // 3. Verify the name construction.
        assert!(cdu.name.contains(subtype));
        assert!(cdu.name.ends_with(".cdu"));
        assert_ne!(
            cdu.name,
            format!(".{}.cdu", subtype),
            "Hash part of the name should not be empty."
        );

        // 4. Verify the payload was stored correctly.
        assert_eq!(cdu.payload, payload);

        // 5. Verify that the HLC timestamp is no longer a placeholder.
        assert_ne!(cdu.metadata.hlc.timestamp, 0, "HLC timestamp should be initialized.");
    }

    #[test]
    fn test_cdu_causal_link() {
        // 1. Create a "cause" CDU.
        let cause_cdu = Cdu::new(b"First event".to_vec(), "genesis", vec![]);

        // 2. Create a second CDU that is caused by the first.
        let effect_cdu = Cdu::new(
            b"Second event".to_vec(),
            "response",
            vec![cause_cdu.name.clone()],
        );

        // 3. Verify that the causal link was stored correctly.
        assert_eq!(effect_cdu.metadata.causes.len(), 1);
        assert_eq!(effect_cdu.metadata.causes[0], cause_cdu.name);

        // 4. Verify that the effect's timestamp is greater than or equal to the cause's.
        assert!(effect_cdu.metadata.hlc >= cause_cdu.metadata.hlc);
    }
}
