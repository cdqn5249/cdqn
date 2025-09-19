// src/kernel/crypto.rs

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};

// Our own simple struct to hold the cryptographic keys.
#[derive(Debug)]
pub struct Keypair {
    pub secret: SigningKey,
    pub public: VerifyingKey,
}

// The CryptoCore service.
pub struct CryptoCore;

impl Default for CryptoCore {
    fn default() -> Self {
        Self::new()
    }
}

impl CryptoCore {
    pub fn new() -> Self {
        CryptoCore
    }

    pub fn generate_keypair(&self) -> Keypair {
        let mut csprng = OsRng;
        let secret_key: SigningKey = SigningKey::generate(&mut csprng);
        let public_key: VerifyingKey = (&secret_key).into();
        Keypair {
            secret: secret_key,
            public: public_key,
        }
    }

    pub fn sign(&self, message_hash: &[u8], secret_key: &SigningKey) -> Signature {
        secret_key.sign(message_hash)
    }

    // A utility function to create a SHA512 hash of any serializable data.
    // It now uses bincode for a consistent, sovereign binary format.
    pub fn hash_content<T: serde::Serialize>(content: &T) -> Vec<u8> {
        let content_bytes = bincode::serialize(content).unwrap();
        let mut hasher = Sha512::new();
        hasher.update(&content_bytes);
        hasher.finalize().to_vec()
    }
}
