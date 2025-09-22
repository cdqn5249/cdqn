// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/lib.rs

//! K.CryptoCore: The Sovereign Root of Trust
//!
//! This module is architected to reflect the core principles of cdqnLang:
//!
//! 1.  **Purity:** The `CryptoCoreEngine` is a stateless collection of pure
//!     cryptographic functions.
//! 2.  **State Management:** The `CryptoCoreManager` is a stateful actor that
//!     holds an entity's keypair and uses the pure `Engine` to perform actions.
//!
//! This enforces a clean separation between pure logic and state.

// --- External Crates ---
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha512};

// --- Core Type Definitions (Publicly Exported) ---
pub type Signature = ed25519_dalek::Signature;
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];

/// A specific, meaningful error type for cryptographic operations.
#[derive(Debug, PartialEq, Eq)]
pub enum CryptoError {
    SignatureCreationFailed,
}

// --- 1. The Pure Functional Engine (Public API) ---
pub struct CryptoCoreEngine;

impl CryptoCoreEngine {
    /// A pure function to generate a new, sovereign identity keypair.
    pub fn generate_identity_keypair() -> (PublicKey, PrivateKey) {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        (
            signing_key.verifying_key().to_bytes(),
            signing_key.to_bytes(),
        )
    }

    /// A pure function to create a SHA-512 digest object from raw data.
    pub fn create_digest(data: &[u8]) -> Sha512 {
        Sha512::new_with_prefix(data)
    }

    /// A pure function to sign a pre-computed digest.
    /// It takes the `Sha512` object directly, preserving type information.
    pub fn sign_digest(private_key: &PrivateKey, digest: Sha512) -> Result<Signature, CryptoError> {
        let signing_key = SigningKey::from_bytes(private_key);
        signing_key
            .sign_prehashed(digest, None)
            .map_err(|_| CryptoError::SignatureCreationFailed)
    }

    /// A pure function to verify a signature against a digest.
    /// It takes the `Sha512` object directly, preserving type information.
    pub fn verify_signature(public_key: &PublicKey, signature: &Signature, digest: Sha512) -> bool {
        let verifying_key_result = VerifyingKey::from_bytes(public_key);
        match verifying_key_result {
            Ok(verifying_key) => verifying_key
                .verify_prehashed(digest, None, signature)
                .is_ok(),
            Err(_) => false,
        }
    }
}

// --- 2. The Stateful Manager (Public API) ---
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

    /// Signs a piece of data, returning an Option of the official Signature object.
    pub fn sign(&self, data: &[u8]) -> Option<Signature> {
        let digest = CryptoCoreEngine::create_digest(data);
        CryptoCoreEngine::sign_digest(&self.private_key, digest).ok()
    }
}

/// Implement the `Default` trait as recommended by Rust best practices.
impl Default for CryptoCoreManager {
    fn default() -> Self {
        Self::new()
    }
}

// --- Unit Tests (Private Module) ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_functions_are_correct() {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        let message = b"test message";
        let digest = CryptoCoreEngine::create_digest(message);
        // We must clone the digest because it's consumed by sign_digest,
        // but we still need it for verification. This is correct ownership.
        let signature_result = CryptoCoreEngine::sign_digest(&private_key, digest.clone());
        assert!(signature_result.is_ok());
        let signature = signature_result.unwrap();
        let is_valid = CryptoCoreEngine::verify_signature(&public_key, &signature, digest);
        assert!(is_valid);
    }

    #[test]
    fn manager_can_create_and_sign() {
        let alice_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature_option = alice_manager.sign(message);
        assert!(signature_option.is_some());
        let signature = signature_option.unwrap();
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
        let signature = alice_manager.sign(message).unwrap();
        let tampered_digest = CryptoCoreEngine::create_digest(tampered_message);
        let is_valid = CryptoCoreEngine::verify_signature(
            alice_manager.public_key(),
            &signature,
            &tampered_digest,
        );
        assert!(!is_valid);
    }

    #[test]
    fn manager_signature_is_not_valid_for_other_manager() {
        let alice_manager = CryptoCoreManager::new();
        let eve_manager = CryptoCoreManager::new();
        let message = b"message from alice";
        let signature = alice_manager.sign(message).unwrap();
        let digest = CryptoCoreEngine::create_digest(message);
        let is_valid =
            CryptoCoreEngine::verify_signature(eve_manager.public_key(), &signature, &digest);
        assert!(!is_valid);
    }
}
