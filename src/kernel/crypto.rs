// src/kernel/crypto.rs

use ed25519_dalek::{Signer, Signature, Keypair}; // Use the official Keypair
use rand::rngs::OsRng;
use sha2::{Sha512, Digest};

// The CryptoCore service. For now, it's a simple struct.
pub struct CryptoCore;

impl CryptoCore {
    // Creates a new instance of the CryptoCore.
    pub fn new() -> Self {
        CryptoCore
    }

    // Generates a new, cryptographically secure keypair using the library's method.
    pub fn generate_keypair(&self) -> Keypair {
        let mut csprng = OsRng;
        Keypair::generate(&mut csprng)
    }

    // Signs a given message hash with a keypair.
    pub fn sign(&self, message_hash: &[u8], keypair: &Keypair) -> Signature {
        keypair.sign(message_hash)
    }
    
    // A utility function to create a SHA512 hash of any serializable data.
    pub fn hash_content<T: serde::Serialize>(content: &T) -> Vec<u8> {
        let content_bytes = serde_json::to_vec(content).unwrap();
        let mut hasher = Sha512::new();
        hasher.update(&content_bytes);
        hasher.finalize().to_vec()
    }
}
