// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/lib.rs

//! K.CryptoCore: The Sovereign Root of Trust
//!
//! This module is the isolated, sovereign core for all cryptographic operations
//! in the CDQN ecosystem, as mandated by the CDQN Manifesto (Tenets 4 & 7).
//!
//! Its purpose is to provide the foundational "root of trust" by enabling
//! verifiable identity and immutable history through standard digital signatures.
//! It is the prerequisite for the future implementation of forward-secret
//! cryptography.
//!
//! This module's API is the direct result of a deep consistency check against
//! the manifesto's tenets.

// --- External Crates ---
// We import the necessary, minimal, and audited cryptographic libraries.
use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature as DalekSignature};
use sha2::{Digest, Sha256};
use rand_core::OsRng; // A secure, OS-provided random number generator.

// --- Core Type Definitions ---
// We define clear, fixed-size types for our cryptographic primitives.
// This makes the code safer and easier to read.

/// A 32-byte Ed25519 public key, used as a verifiable, sovereign identity.
pub type PublicKey = [u8; 32];

/// A 32-byte Ed25519 private key, the root of an entity's sovereignty.
pub type PrivateKey = [u8; 32];

/// A 64-byte Ed25519 signature, the proof of a KDU's authenticity.
pub type Signature = [u8; 64];

/// A 32-byte SHA-256 hash, the cryptographic fingerprint of data.
pub type Hash = [u8; 32];

/// The main struct for the CryptoCore module. It is a stateless "engine"
/// that provides the core cryptographic functions as associated functions.
pub struct CryptoCore;

impl CryptoCore {
    /// Generates a new, long-term sovereign identity keypair.
    /// Mandated By: Tenet 2 (Data Sovereignty), Tenet 4 (Verifiable Identity).
    pub fn generate_identity_keypair() -> (PublicKey, PrivateKey) {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let private_key_bytes = signing_key.to_bytes();
        let public_key_bytes = signing_key.verifying_key().to_bytes();
        (public_key_bytes, private_key_bytes)
    }

    /// Generates a new, ephemeral (single-use) keypair for signing.
    /// Mandated By: Tenet 7 (Ephemeral... signatures).
    pub fn generate_ephemeral_keypair() -> (PublicKey, PrivateKey) {
        // For Phase 1, the generation is identical to an identity keypair.
        // The "ephemeral" nature is enforced by the calling logic (i.e., using it once).
        Self::generate_identity_keypair()
    }

    /// Creates a cryptographic hash of any raw data.
    /// Mandated By: Tenet 7 (isolating all cryptographic operations).
    pub fn hash_data(data: &[u8]) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Signs a pre-computed hash with a given private key.
    /// Mandated By: Tenet 4 ("Every KDU is cryptographically signed...").
    pub fn sign_hash(private_key: &PrivateKey, hash: &Hash) -> Signature {
        let signing_key = SigningKey::from_bytes(private_key);
        // The `sign` method from the `ed25519_dalek` crate is specifically
        // designed to sign a pre-hashed message for security and efficiency.
        signing_key.sign(hash).to_bytes()
    }

    /// Verifies a signature against a hash and a public key.
    /// Mandated By: Tenet 4 ("...by a verifiable originator.").
    pub fn verify_signature(public_key: &PublicKey, signature: &Signature, hash: &Hash) -> bool {
        // Attempt to construct a VerifyingKey from the public key bytes.
        // If the bytes are invalid, verification fails immediately.
        if let Ok(verifying_key) = VerifyingKey::from_bytes(public_key) {
            // Attempt to construct a Signature from the signature bytes.
            if let Ok(dalek_signature) = DalekSignature::from_bytes(signature) {
                // The actual verification check.
                return verifying_key.verify(hash, &dalek_signature).is_ok();
            }
        }
        // If any part of the process fails, the signature is invalid.
        false
    }
}


// --- Unit Tests ---
// A critical part of any cryptographic module is a robust test suite
// to prove its correctness.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let (public_key, private_key) = CryptoCore::generate_identity_keypair();
        // A basic sanity check: ensure keys are not all zeros.
        assert_ne!(public_key, [0u8; 32]);
        assert_ne!(private_key, [0u8; 32]);
    }

    #[test]
    fn test_hash_consistency() {
        let data1 = b"This is a test message for the CDQN CryptoCore.";
        let data2 = b"This is a different test message.";

        let hash1 = CryptoCore::hash_data(data1);
        let hash1_again = CryptoCore::hash_data(data1);
        let hash2 = CryptoCore::hash_data(data2);

        // Hashes of the same data must be identical.
        assert_eq!(hash1, hash1_again);
        // Hashes of different data must be different.
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_valid_signature_lifecycle() {
        // 1. Generate an identity.
        let (public_key, private_key) = CryptoCore::generate_identity_keypair();

        // 2. Create some data and hash it.
        let message = b"This KDU payload is authentic.";
        let hash = CryptoCore::hash_data(message);

        // 3. Sign the hash with the private key.
        let signature = CryptoCore::sign_hash(&private_key, &hash);

        // 4. Verify the signature with the public key. It must succeed.
        let is_valid = CryptoCore::verify_signature(&public_key, &signature, &hash);
        assert!(is_valid, "A valid signature should be verified successfully.");
    }

    #[test]
    fn test_invalid_signature_tampered_data() {
        let (public_key, private_key) = CryptoCore::generate_identity_keypair();
        let message = b"Original message.";
        let tampered_message = b"Tampered message!";

        let original_hash = CryptoCore::hash_data(message);
        let tampered_hash = CryptoCore::hash_data(tampered_message);

        let signature = CryptoCore::sign_hash(&private_key, &original_hash);

        // Verification must fail because the signature was for the original hash,
        // not the hash of the tampered data.
        let is_valid = CryptoCore::verify_signature(&public_key, &signature, &tampered_hash);
        assert!(!is_valid, "A signature should not be valid for tampered data.");
    }

    #[test]
    fn test_invalid_signature_wrong_key() {
        let (alice_pk, alice_sk) = CryptoCore::generate_identity_keypair();
        let (eve_pk, _eve_sk) = CryptoCore::generate_identity_keypair(); // Eve's key

        let message = b"A message from Alice.";
        let hash = CryptoCore::hash_data(message);

        // Alice signs the message.
        let signature = CryptoCore::sign_hash(&alice_sk, &hash);

        // We try to verify Alice's signature using EVE's public key.
        // This must fail.
        let is_valid = CryptoCore::verify_signature(&eve_pk, &signature, &hash);
        assert!(!is_valid, "A signature should not be valid for the wrong public key.");
    }
}
