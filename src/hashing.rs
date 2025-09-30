// Source code under BaDaaS license, vibe coding engine: Gemini Flash-Lite Latest, Google
// File path: src/hashing.rs

use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

pub type Hash = [u8; 32]; // SHA-256 produces a 32-byte hash

/// Hashes any serializable structure into a fixed-size SHA-256 Hash.
/// This is the core function for content-addressing and Merkle tree construction.
pub fn hash_serializable<T: Serialize>(item: &T) -> Hash {
    let mut hasher = Sha256::new();
    // Use bincode for compact serialization before hashing.
    // This ensures that the structure of the data (e.g., HLC, ParentHash, Type) is included in the hash.
    let serialized = bincode::serialize(item).expect("Failed to serialize item for hashing");
    hasher.update(&serialized);
    let result = hasher.finalize();
    
    // Convert GenericArray to a fixed-size array [u8; 32]
    let mut hash_array: Hash = [0; 32];
    hash_array.copy_from_slice(&result);
    hash_array
}
