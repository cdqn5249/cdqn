// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/manager.rs

//! Contains the `CryptoCoreManager`, a stateful actor that represents a single
//! sovereign identity, holding a keypair and using the pure `CryptoCoreEngine`.

use super::engine::CryptoCoreEngine;
use super::types::{CryptoError, PrivateKey, PublicKey, Signature};

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
