// BaDaaS License: This file is governed by the BaDaaS license.
// Vibe coding engine: Gemini 2.5 Pro, Google
// File Path: /src/kernel/crypto/src/lib.rs

//! K.CryptoCore: The Sovereign Root of Trust
//!
//! This crate is the isolated, sovereign core for all cryptographic operations
//! in the CDQN ecosystem. It is composed of several modules that enforce a
//! clean separation between pure logic and state, as mandated by the manifesto.

// --- Module Declarations ---
// This declares the existence of our submodules. The Rust compiler will look
// for `engine.rs`, `manager.rs`, and `types.rs` in this directory.
pub mod engine;
pub mod manager;
pub mod types;

// --- Public API Re-exports ---
// We re-export the core components from our submodules to create the public
// API for this crate. Other crates will interact with these components.
pub use engine::CryptoCoreEngine;
pub use manager::CryptoCoreManager;
pub use types::{CryptoError, PrivateKey, PublicKey, Signature};

// --- Unit Tests (Private Module) ---
// The tests remain here, allowing us to test the crate's public API as a whole.
#[cfg(test)]
mod tests {
    use super::*;

    // This module groups all the "happy path" tests.
    mod success_cases {
        use super::*;

        #[test]
        fn full_signature_lifecycle_succeeds() {
            let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
            let message = b"test message";
            let digest = CryptoCoreEngine::create_digest(message);
            let signature = CryptoCoreEngine::sign_digest(&private_key, digest.clone()).unwrap();
            let verification_result =
                CryptoCoreEngine::verify_signature(&public_key, &signature, digest);
            assert!(verification_result.is_ok());
        }

        #[test]
        fn manager_signing_and_verification_succeeds() {
            let manager = CryptoCoreManager::new();
            let message = b"a message from the manager";
            let signature = manager.sign(message).unwrap();
            let digest = CryptoCoreEngine::create_digest(message);
            let verification_result =
                CryptoCoreEngine::verify_signature(manager.public_key(), &signature, digest);
            assert!(verification_result.is_ok());
        }
    }

    // This module groups all the error handling tests.
    mod failure_cases {
        use super::*;

        #[test]
        fn tampered_data_fails_verification() {
            let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
            let message = b"original message";
            let tampered_message = b"tampered message";
            let signature = CryptoCoreEngine::sign_digest(
                &private_key,
                CryptoCoreEngine::create_digest(message),
            )
            .unwrap();
            let tampered_digest = CryptoCoreEngine::create_digest(tampered_message);
            let verification_result =
                CryptoCoreEngine::verify_signature(&public_key, &signature, tampered_digest);
            assert!(verification_result.is_err());
            assert!(matches!(
                verification_result,
                Err(CryptoError::LibraryError(_))
            ));
        }

        #[test]
        fn wrong_key_fails_verification() {
            let (_alice_pk, alice_sk) = CryptoCoreEngine::generate_identity_keypair();
            let (eve_pk, _) = CryptoCoreEngine::generate_identity_keypair();
            let message = b"message from alice";
            let digest = CryptoCoreEngine::create_digest(message);
            let signature = CryptoCoreEngine::sign_digest(&alice_sk, digest.clone()).unwrap();
            let verification_result =
                CryptoCoreEngine::verify_signature(&eve_pk, &signature, digest);
            assert!(verification_result.is_err());
        }
    }
}
