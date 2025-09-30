// Source code under BaDaaS license, vibe coding engine: Gemini Flash-Lite Latest, Google
// File path: src/cdu.rs

use serde::{Serialize, Deserialize};
use super::hashing::{self, Hash};

// --- Core Data Types ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hlc(pub u64); 

// --- NEW: CDU Type Enum ---
#[derive(Serialize, Deserialize, Debug, Clone)]
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
// ... (rest of Cdu struct remains the same)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cdu {
    pub address: Hash, // The hash of the ImmutablePayload
    pub payload: ImmutablePayload,
}

// --- Core Logic ---

pub const GENESIS_HASH: Hash = [0u8; 32]; 

impl ImmutablePayload {
    // ... (calculate_address remains the same, as it hashes the whole payload including cdu_type)
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

// ... (rest of cdu.rs)
