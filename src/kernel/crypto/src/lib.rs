// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/lib.rs

//! K.CryptoCore: The Sovereign Root of Trust
//!
//! This module is architected to reflect the core principles of cdqnLang:
//! 1.  **Purity:** The `CryptoCoreEngine` is a stateless collection of pure
//!     cryptographic functions.
//! 2.  **State Management:** The `CryptoCoreManager` is a stateful actor that
//!     holds an entity's keypair and uses the pure `Engine` to perform actions.
//! This enforces a clean separation between pure logic and state.

// --- External Crates ---
use ed25519_dalek::{
    hazmat::{PrehashSigner, PrehashVerifier}, // Correctly import the traits
    Signature as DalekSignature,
    SigningKey,
    VerifyingKey,
};
use rand_core::OsRng;
use sha2::{Digest, Sha256};

// --- Core Type Definitions ---
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Signature = [u8; 64];
pub type Hash = [u8; 32];

// --- 1. The Pure Functional Engine ---
pub struct CryptoCoreEngine;

impl CryptoCoreEngine {
    pub fn generate_identity_keypair() -> (PublicKey, PrivateKey) {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        (
            signing_key.verifying_key().to_bytes(),
            signing_key.to_bytes(),
        )
    }

    pub fn hash_data(data: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// A pure function to sign a pre-computed hash with a given private key.
    pub fn sign_hash(private_key: &PrivateKey, hash: &Hash) -> Signature {
        let signing_key = SigningKey::from_bytes(private_key);
        // The `sign_prehashed` method is provided by the `PrehashSigner` trait.
        signing_key.sign_prehashed(hash, None).unwrap().to_bytes()
    }

    /// A pure function to verify a signature against a hash and a public key.
    pub fn verify_signature(public_key: &PublicKey, signature: &Signature, hash: &Hash) -> bool {
        if let Ok(verifying_key) = VerifyingKey::from_bytes(public_key) {
            if let Ok(dalek_signature) = DalekSignature::from_bytes(signature) {
                // The `verify_prehashed` method is provided by the `PrehashVerifier` trait.
                return verifying_key
                    .verify_prehashed(hash, None, &dalek_signature)
                    .is_ok();
            }
        }
        false
    }
}

// --- 2. The Stateful Manager ---
pub struct CryptoCoreManager {
    public_key: PublicKey,
    private_key: PrivateKey,
}

impl CryptoCoreManager {
    pub fn new() -> Self {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        Self {
            public_key,
            private_key,
        }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn sign(&self, data: &[u8]) -> Signature {
        let hash = CryptoCoreEngine::hash_data(data);
        CryptoCoreEngine::sign_hash(&self.private_key, &hash)
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_functions_are_correct() {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        let message = b"test message";
        let hash = CryptoCoreEngine::hash_data(message);
        let signature = CryptoCoreEngine::sign_hash(&private_key, &hash);
        let is_valid = CryptoCoreEngine::verify_signature(&public_key, &signature, &hash);
        assert!(is_valid);
    }

    #[test]
    fn manager_can_create_and_sign() {
        let alice_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature = alice_manager.sign(message);
        let message_hash = CryptoCoreEngine::hash_data(message);
        let is_valid = CryptoCoreEngine::verify_signature(
            alice_manager.public_key(),
            &signature,
            &message_hash,
        );
        assert!(is_valid);
    }

    #[test]
    fn manager_signature_is_not_valid_for_other_data() {
        let alice_manager = CryptoCoreManager::new();
        let message = b"original message";
        let tampered_message = b"tampered message";
        let signature = alice_manager.sign(message);
        let tampered_hash = CryptoCoreEngine::hash_data(tampered_message);
        let is_valid = CryptoCoreEngine::verify_signature(
            alice_manager.public_key(),
            &signature,
            &tampered_hash,
        );
        assert!(!is_valid);
    }

    #[test]
    fn manager_signature_is_not_valid_for_other_manager() {
        let alice_manager = CryptoCoreManager::new();
        let eve_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature = alice_manager.sign(message);
        let message_hash = CryptoCoreEngine::hash_data(message);
        let is_valid =
            CryptoCoreEngine::verify_signature(eve_manager.public_key(), &signature, &message_hash);
        assert!(!is_valid);
    }
}
