// Path: src/cdu.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256 as Sha256};
use std::fmt;

/// Represents the lifecycle state of a Causal Data Unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CduState {
    Draft,
    Active,
    Merged,
    Archived,
    Expired,
    Invalid,
}

/// Immutable causal data unit.
///
/// CDUs are signed, hashed, and linked together in Merkle-like chains.
/// They are the atomic proof units of causality in the CDQN ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalDataUnit {
    pub id: String,
    pub author: String,
    pub role: String,
    pub payload: String,
    pub state: CduState,
    pub timestamp: u64,
    pub signature: Option<String>,
    pub hash: String,
}

impl CausalDataUnit {
    /// Creates a new, unsigned CDU with deterministic hash.
    pub fn new(id: &str, author: &str, role: &str, payload: &str, timestamp: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(author.as_bytes());
        hasher.update(role.as_bytes());
        hasher.update(payload.as_bytes());
        hasher.update(timestamp.to_le_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Self {
            id: id.to_string(),
            author: author.to_string(),
            role: role.to_string(),
            payload: payload.to_string(),
            state: CduState::Draft,
            timestamp,
            signature: None,
            hash,
        }
    }

    /// Signs the CDU with the provided Ed25519 private key.
    /// Returns a new CDU instance with an attached signature (immutably).
    pub fn sign(&self, signing_key: &SigningKey) -> Self {
        let mut cdu = self.clone();
        let message = cdu.hash.as_bytes();
        let sig = signing_key.sign(message);
        cdu.signature = Some(hex::encode(sig.to_bytes()));
        cdu
    }

    /// Verifies the CDU signature using the provided Ed25519 public key.
    pub fn verify(&self, verifying_key: &VerifyingKey) -> bool {
        if let Some(sig_hex) = &self.signature {
            if let Ok(sig_bytes) = hex::decode(sig_hex) {
                if let Ok(signature) = Signature::from_bytes(&sig_bytes) {
                    return verifying_key
                        .verify(self.hash.as_bytes(), &signature)
                        .is_ok();
                }
            }
        }
        false
    }

    /// Returns a new CDU that merges two existing CDUs deterministically.
    ///
    /// This operation does not modify either source CDU. The merged hash
    /// is derived from both source hashes and the current timestamp.
    pub fn merge(a: &Self, b: &Self, timestamp: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(a.hash.as_bytes());
        hasher.update(b.hash.as_bytes());
        hasher.update(timestamp.to_le_bytes());
        let merged_hash = format!("{:x}", hasher.finalize());

        Self {
            id: format!("merge:{}+{}", a.id, b.id),
            author: a.author.clone(),
            role: format!("merge({},{})", a.role, b.role),
            payload: format!("{{a:{}, b:{}}}", a.payload, b.payload),
            state: CduState::Merged,
            timestamp,
            signature: None,
            hash: merged_hash,
        }
    }
}

impl fmt::Display for CausalDataUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[CDU {} by {} | role={} | state={:?}]",
            self.id, self.author, self.role, self.state
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
    fn test_cdu_creation_and_naming() {
        let cdu = CausalDataUnit::new("cdu1", "nodeA", "proposer", "payloadX", ts());
        assert_eq!(cdu.state, CduState::Draft);
        assert!(!cdu.hash.is_empty());
    }

    #[test]
    fn test_cdu_causal_link() {
        let a = CausalDataUnit::new("a", "A", "proposer", "p1", ts());
        let b = CausalDataUnit::new("b", "B", "verifier", "p2", ts());
        let m = CausalDataUnit::merge(&a, &b, ts());
        assert!(m.hash.len() > 10);
        assert!(m.payload.contains("a:"));
    }

    #[test]
    fn test_sign_and_verify_cycle() {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let cdu = CausalDataUnit::new("cdu-sign", "userX", "policy", "payloadY", ts());
        let signed = cdu.sign(&signing_key);
        assert!(signed.signature.is_some());
        assert!(signed.verify(&verifying_key));
    }
}
