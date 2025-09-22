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
use ed25519_dalek::{Signature as DalekSignature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha265};

// --- Core Type Definitions ---
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];
pub type Signature = [u8; 64];
pub type Hash = [u8; 32];

// --- 1. The Pure Functional Engine ---

/// The `CryptoCoreEngine` is a stateless namespace for all core cryptographic
/// operations. It is the direct Rust equivalent of a library of pure functions.
/// All its functions are deterministic and have no side effects beyond what is
/// returned.
pub struct CryptoCoreEngine;

impl CryptoCoreEngine {
    /// A pure function to generate a new, sovereign identity keypair.
    /// NOTE: This function is inherently non-deterministic as it must interact
    /// with the OS for entropy. It is the "Big Bang" of identity creation.
    pub fn generate_identity_keypair() -> (PublicKey, PrivateKey) {
        // The only mutable variable is the random number generator, which is
        // required by the underlying library API. It is created and destroyed
        // within this function, preserving the stateless nature of the Engine.
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        (
            signing_key.verifying_key().to_bytes(),
            signing_key.to_bytes(),
        )
    }

    /// A pure function to create a cryptographic hash of any raw data.
    pub fn hash_data(data: &[u8]) -> Hash {
        // The `hasher` is a temporary, mutable variable required for the builder
        // pattern of the hashing algorithm. The function itself remains pure:
        // same input always produces same output.
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// A pure function to sign a pre-computed hash with a given private key.
    pub fn sign_hash(private_key: &PrivateKey, hash: &Hash) -> Signature {
        let signing_key = SigningKey::from_bytes(private_key);
        signing_key.sign(hash).to_bytes()
    }

    /// A pure function to verify a signature against a hash and a public key.
    pub fn verify_signature(public_key: &PublicKey, signature: &Signature, hash: &Hash) -> bool {
        if let (Ok(verifying_key), Ok(dalek_signature)) = (
            VerifyingKey::from_bytes(public_key),
            DalekSignature::from_bytes(signature),
        ) {
            verifying_key.verify(hash, &dalek_signature).is_ok()
        } else {
            false
        }
    }
}

// --- 2. The Stateful Manager ---

/// The `CryptoCoreManager` is a stateful actor that represents a single
/// sovereign identity. It holds the keypair and uses the `CryptoCoreEngine`
/// to perform actions. This is the component an Entity would own and use.
pub struct CryptoCoreManager {
    public_key: PublicKey,
    private_key: PrivateKey,
}

impl CryptoCoreManager {
    /// Creates a new manager for a new identity.
    pub fn new() -> Self {
        let (public_key, private_key) = CryptoCoreEngine::generate_identity_keypair();
        Self {
            public_key,
            private_key,
        }
    }

    /// Returns the public key (the verifiable identity) of this manager.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Signs a piece of data. This method takes the manager's state (`&self`)
    /// and the data, and returns a new, immutable piece of data (the signature).
    /// It does not modify its own state.
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
        // 1. Create a new manager (a new identity).
        let alice_manager = CryptoCoreManager::new();
        let message = b"message from alice";

        // 2. Alice's manager signs the message.
        let signature = alice_manager.sign(message);

        // 3. We can verify the signature using the manager's public key and the pure engine.
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

        // Verify against the tampered message hash. Must fail.
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
        let eve_manager = CryptoCoreManager::new(); // Eve's manager/identity
        let message = b"message from alice";

        // Alice signs the message.
        let signature = alice_manager.sign(message);

        // Try to verify Alice's signature using EVE's public key. Must fail.
        let message_hash = CryptoCoreEngine::hash_data(message);
        let is_valid =
            CryptoCoreEngine::verify_signature(eve_manager.public_key(), &signature, &message_hash);
        assert!(!is_valid);
    }
}
