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
    /// ensuring content-addressable, verifiable storage.
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

        // 6. Create placeholder metadata.
        //    (We will implement HLC logic in a future step).
        let metadata = CduMetadata {
            hlc: Hlc {
                timestamp: 0,
                counter: 0,
            },
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
