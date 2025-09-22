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
use ed25519_dalek::{Signature as DalekSignature, SigningKey, VerifyingKey};
use rand_core::OsRng;
// We now import Sha512, as required by the Ed25519ph standard.
use sha2::{Digest, Sha512};

// --- Core Type Definitions ---
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Signature = [u8; 64];
// A hash is now 64 bytes, the output size of SHA-512.
pub type Hash = [u8; 64];

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

    /// A pure function to create a SHA-512 digest of any raw data.
    pub fn create_digest(data: &[u8]) -> Sha512 {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher
    }

    /// A pure function to sign a pre-computed digest with a given private key.
    pub fn sign_digest(private_key: &PrivateKey, digest: Sha512) -> Signature {
        let signing_key = SigningKey::from_bytes(private_key);
        signing_key.sign_prehashed(digest, None).unwrap().to_bytes()
    }

    /// A pure function to verify a signature against a digest and a public key.
    pub fn verify_signature(public_key: &PublicKey, signature: &Signature, digest: Sha512) -> bool {
        // This final, robust version correctly handles all conversions and checks.
        if let Ok(verifying_key) = VerifyingKey::from_bytes(public_key) {
            if let Ok(dalek_signature) = DalekSignature::from_bytes(signature) {
                return verifying_key
                    .verify_prehashed(digest, None, &dalek_signature)
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
        let digest = CryptoCoreEngine::create_digest(data);
        CryptoCoreEngine::sign_digest(&self.private_key, digest)
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
        let digest = CryptoCoreEngine::create_digest(message);
        let signature = CryptoCoreEngine::sign_digest(&private_key, digest.clone());
        let is_valid = CryptoCoreEngine::verify_signature(&public_key, &signature, digest);
        assert!(is_valid);
    }

    #[test]
    fn manager_can_create_and_sign() {
        let alice_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature = alice_manager.sign(message);
        let digest = CryptoCoreEngine::create_digest(message);
        let is_valid =
            CryptoCoreEngine::verify_signature(alice_manager.public_key(), &signature, digest);
        assert!(is_valid);
    }

    #[test]
    fn manager_signature_is_not_valid_for_other_data() {
        let alice_manager = CryptoCoreManager::new();
        let message = b"original message";
        let tampered_message = b"tampered message";
        let signature = alice_manager.sign(message);
        let tampered_digest = CryptoCoreEngine::create_digest(tampered_message);
        let is_valid = CryptoCoreEngine::verify_signature(
            alice_manager.public_key(),
            &signature,
            tampered_digest,
        );
        assert!(!is_valid);
    }

    #[test]
    fn manager_signature_is_not_valid_for_other_manager() {
        let alice_manager = CryptoCoreManager::new();
        let eve_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature = alice_manager.sign(message);
        let digest = CryptoCoreEngine::create_digest(message);
        let is_valid =
            CryptoCoreEngine::verify_signature(eve_manager.public_key(), &signature, digest);
        assert!(!is_valid);
    }
}
