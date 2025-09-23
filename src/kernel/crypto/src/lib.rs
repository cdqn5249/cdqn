// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/lib.rs

//! K.CryptoCore: The Sovereign Root of Trust
//!
//! This module is architected to reflect the core principles of cdqnLang and
//! provides robust, verbose error handling to ensure the system is auditable
//! and can learn from its mistakes (Manifesto Tenets 7 & 9).

// --- External Crates ---
use ed25519_dalek::{SignatureError, SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha512};

// --- Core Type Definitions (Publicly Exported) ---
pub type Signature = ed25519_dalek::Signature;
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];

/// A specific, verbose, and robust error type for all cryptographic operations.
/// The underlying SignatureError does not implement PartialEq or Eq, so we
/// cannot derive them automatically. We only derive Debug.
#[derive(Debug)]
pub enum CryptoError {
    /// The underlying cryptographic library returned an error. This variant
    /// wraps the library's error to preserve the rich, original context.
    LibraryError(SignatureError),
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

    /// A pure, one-shot function to create a SHA-512 digest object from raw data.
    pub fn create_digest(data: &[u8]) -> Sha512 {
        Sha512::new_with_prefix(data)
    }

    /// A pure function to sign a pre-computed digest.
    pub fn sign_digest(private_key: &PrivateKey, digest: Sha512) -> Result<Signature, CryptoError> {
        let signing_key = SigningKey::from_bytes(private_key);
        signing_key
            .sign_prehashed(digest, None)
            .map_err(CryptoError::LibraryError)
    }

    /// A pure function to verify a signature.
    /// This function now returns a Result<(), CryptoError> to provide verbose
    /// error information, instead of a simple boolean.
    pub fn verify_signature(
        public_key: &PublicKey,
        signature: &Signature,
        digest: Sha512,
    ) -> Result<(), CryptoError> {
        let verifying_key =
            VerifyingKey::from_bytes(public_key).map_err(CryptoError::LibraryError)?;
        verifying_key
            .verify_prehashed(digest, None, signature)
            .map_err(CryptoError::LibraryError)
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

    /// Signs a piece of data, returning a Result with our specific error type.
    pub fn sign(&self, data: &[u8]) -> Result<Signature, CryptoError> {
        let digest = CryptoCoreEngine::create_digest(data);
        CryptoCoreEngine::sign_digest(&self.private_key, digest)
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
    fn successful_signature_lifecycle() {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        let message = b"test message";
        let digest = CryptoCoreEngine::create_digest(message);
        let signature = CryptoCoreEngine::sign_digest(&private_key, digest.clone()).unwrap();
        let verification_result =
            CryptoCoreEngine::verify_signature(&public_key, &signature, digest);
        assert!(verification_result.is_ok());
    }

    #[test]
    fn verification_fails_for_tampered_data() {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        let message = b"original message";
        let tampered_message = b"tampered message";
        let signature =
            CryptoCoreEngine::sign_digest(&private_key, CryptoCoreEngine::create_digest(message))
                .unwrap();
        let tampered_digest = CryptoCoreEngine::create_digest(tampered_message);
        // The digest is consumed, so we pass the owned value directly.
        let verification_result =
            CryptoCoreEngine::verify_signature(&public_key, &signature, tampered_digest);
        assert!(verification_result.is_err());
    }

    #[test]
    fn verification_fails_for_wrong_key() {
        let (_alice_pk, alice_sk) = CryptoCoreEngine::generate_identity_keypair();
        let (eve_pk, _) = CryptoCoreEngine::generate_identity_keypair();
        let message = b"message from alice";
        let digest = CryptoCoreEngine::create_digest(message);
        let signature = CryptoCoreEngine::sign_digest(&alice_sk, digest.clone()).unwrap();
        // The digest is consumed, so we pass the owned value directly.
        let verification_result = CryptoCoreEngine::verify_signature(&eve_pk, &signature, digest);
        assert!(verification_result.is_err());
    }

    #[test]
    fn manager_can_sign_and_verify() {
        let manager = CryptoCoreManager::new();
        let message = b"a message from the manager";
        let signature = manager.sign(message).unwrap();
        let digest = CryptoCoreEngine::create_digest(message);
        let verification_result =
            CryptoCoreEngine::verify_signature(manager.public_key(), &signature, digest);
        assert!(verification_result.is_ok());
    }
}
