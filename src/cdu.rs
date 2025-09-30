// Source code under BaDaaS license, vibe coding engine: Gemini Flash-Lite Latest, Google
// File path: src/cdu.rs

use serde::{Serialize, Deserialize};
use super::hashing::{self, Hash};

// --- Core Data Types ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hlc(pub u64); 

// --- NEW: CDU Type Enum ---
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CduType {
    Data,
    Axiom,
    // Add more types later (e.g., Procedure, Constraint)
}

// --- CDU Components ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentHash(pub Hash);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImmutablePayload {
    pub cdu_type: CduType, // <-- NEW FIELD
    pub hlc: Hlc,
    pub parent_hash: Hash, // The hash of the parent CDU
    pub content_hash: ContentHash,
}

// The final Causal Data Unit
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cdu {
    pub address: Hash, // The hash of the ImmutablePayload
    pub payload: ImmutablePayload,
}

// --- Core Logic ---

pub const GENESIS_HASH: Hash = [0u8; 32]; 

impl ImmutablePayload {
    /// Calculates the CDU's address by hashing the entire immutable payload.
    pub fn calculate_address(&self) -> Hash {
        hashing::hash_serializable(self)
    }
}

pub fn create_cdu(cdu_type: CduType, hlc: Hlc, parent_hash: Hash, content_hash: ContentHash) -> Cdu {
    let payload = ImmutablePayload {
        cdu_type, // <-- Use the new type field
        hlc,
        parent_hash,
        content_hash,
    };
    let address = payload.calculate_address();
    Cdu { address, payload }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Helper to create a dummy content hash
    fn create_content_hash(data: &str) -> ContentHash {
        let mut hasher = sha2::Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        let mut hash_array: Hash = [0; 32];
        hash_array.copy_from_slice(&result);
        ContentHash(hash_array)
    }

    #[test]
    fn test_cdu_creation_and_addressing() {
        let content = create_content_hash("test_data");
        let hlc = Hlc(100);
        let parent = [1u8; 32];

        let cdu = create_cdu(CduType::Data, hlc, parent, ContentHash(content.0));
        
        // Check that the address is calculated (i.e., not the genesis hash)
        assert_ne!(cdu.address, GENESIS_HASH, "CDU address should be calculated.");
        
        // Check that the payload matches the address calculation
        let recalculated_address = cdu.payload.calculate_address();
        assert_eq!(cdu.address, recalculated_address, "CDU address must match the hash of its payload.");
    }
    
    #[test]
    fn test_axiom_type_check() {
        let content_data = create_content_hash("data_a");
        let content_axiom = create_content_hash("axiom_rule_1");
        let hlc = Hlc(1);
        let parent = GENESIS_HASH;

        let data_cdu = create_cdu(CduType::Data, hlc.clone(), parent, ContentHash(content_data.0));
        let axiom_cdu = create_cdu(CduType::Axiom, hlc, parent, ContentHash(content_axiom.0));

        // Check that two data CDUs are NOT considered axioms by the type check
        assert!(!super::axiom_type_check(&data_cdu, &data_cdu));
        
        // Check that two axiom CDUs ARE considered axioms by the type check
        assert!(super::axiom_type_check(&axiom_cdu, &axiom_cdu));
    }
}
