// src/kernel/crypto.rs

use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use rand::rngs::OsRng;
use sha2::{Sha512, Digest};

// A simple struct to hold our cryptographic keys.
#[derive(Debug)]
pub struct Keypair {
    pub secret: SigningKey,
    pub public: VerifyingKey,
}

// The CryptoCore service. For now, it's a simple struct.
pub struct CryptoCore;

impl CryptoCore {
    // Creates a new instance of the CryptoCore.
    pub fn new() -> Self {
        CryptoCore
    }

    // Generates a new, cryptographically secure keypair.
    pub fn generate_keypair(&self) -> Keypair {
        let mut csprng = OsRng;
        let secret_key: SigningKey = SigningKey::generate(&mut csprng);
        let public_key: VerifyingKey = (&secret_key).into();
        Keypair {
            secret: secret_key,
            public: public_key,
        }
    }

    // Signs a given message hash with a secret key.
    pub fn sign(&self, message_hash: &[u8], secret_key: &SigningKey) -> Signature {
        secret_key.sign(message_hash)
    }
    
    // A utility function to create a SHA512 hash of any serializable data.
    pub fn hash_content<T: serde::Serialize>(content: &T) -> Vec<u8> {
        let content_bytes = serde_json::to_vec(content).unwrap();
        let mut hasher = Sha512::new();
        hasher.update(&content_bytes);
        hasher.finalize().to_vec()
    }
}
