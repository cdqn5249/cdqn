// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/engine.rs

//! Contains the `CryptoCoreEngine`, a stateless collection of pure cryptographic
//! functions that form the foundational primitives of the Kernel.

use super::types::{CryptoError, PrivateKey, PublicKey, Signature};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha512};

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
