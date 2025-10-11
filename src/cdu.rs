// Path: src/cdu.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! # Causal Data Unit (CDU)
//!
//! CDUs are the fundamental immutable data atoms of the CDQN ecosystem.
//! They represent verifiable, causal relationships between actions, states,
//! and reasoning outcomes. Each CDU is self-contained, signed, and hashed
//! to form a causal chain of trust.
//!
//! ## Principles
//! - **Immutable:** once created, a CDU cannot be altered.
//! - **Causal:** every CDU derives from prior CDUs, linking in a verifiable chain.
//! - **Verifiable:** cryptographically signed and hashed (SHA3 + Ed25519).
//! - **Lightweight:** pure structs, no blocking, no global state.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256 as Sha256};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use std::fmt;

/// Represents the state of a CDU in its lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CduState {
    Draft,
    Active,
    Archived,
    Invalid,
}

/// The immutable Causal Data Unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cdu {
    pub id: String,
    pub author: String,
    pub payload: String,
    pub timestamp: u64,
    pub state: CduState,
    pub hash: String,
    pub signature: Option<String>,
}

impl Cdu {
    /// Creates a new unsigned CDU.
    pub fn new(id: &str, author: &str, payload: &str, timestamp: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(author.as_bytes());
        hasher.update(payload.as_bytes());
        hasher.update(timestamp.to_le_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Self {
            id: id.to_string(),
            author: author.to_string(),
            payload: payload.to_string(),
            timestamp,
            state: CduState::Draft,
            hash,
            signature: None,
        }
    }

    /// Returns a new CDU signed with the given Ed25519 private key.
    pub fn sign(&self, key: &SigningKey) -> Self {
        let mut clone = self.clone();
        let signature = key.sign(self.hash.as_bytes());
        clone.signature = Some(hex::encode(signature.to_bytes()));
        clone
    }

    /// Verifies this CDU’s signature with the given Ed25519 public key.
    pub fn verify(&self, key: &VerifyingKey) -> bool {
        if let Some(sig_hex) = &self.signature {
            if let Ok(sig_bytes) = hex::decode(sig_hex) {
                if sig_bytes.len() == 64 {
                    if let Ok(arr) = sig_bytes.clone().try_into() as Result<[u8; 64], _> {
                        let sig = Signature::from_bytes(&arr);
                        return key.verify(self.hash.as_bytes(), &sig).is_ok();
                    }
                }
            }
        }
        false
    }

    /// Deterministically merges two CDUs into a new one.
    pub fn merge(a: &Self, b: &Self, timestamp: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(a.hash.as_bytes());
        hasher.update(b.hash.as_bytes());
        hasher.update(timestamp.to_le_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Self {
            id: format!("merge:{}+{}", a.id, b.id),
            author: a.author.clone(),
            payload: format!("{{A:{},B:{}}}", a.payload, b.payload),
            timestamp,
            state: CduState::Active,
            hash,
            signature: None,
        }
    }
}

impl fmt::Display for Cdu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[CDU {} | author={} | state={:?}]",
            self.id, self.author, self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn ts() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    #[test]
    fn create_and_sign_verify_cycle() {
        let key_bytes = [1u8; 32];
        let signing_key = SigningKey::from_bytes(&key_bytes);
        let verifying_key = VerifyingKey::from(&signing_key);

        let cdu = Cdu::new("1", "alice", "payload", ts());
        let signed = cdu.sign(&signing_key);
        assert!(signed.signature.is_some());
        assert!(signed.verify(&verifying_key));
    }

    #[test]
    fn merge_two_cdus() {
        let a = Cdu::new("a", "nodeA", "payloadA", ts());
        let b = Cdu::new("b", "nodeB", "payloadB", ts());
        let merged = Cdu::merge(&a, &b, ts());
        assert!(merged.payload.contains("A:"));
        assert_ne!(merged.hash, a.hash);
    }
}
