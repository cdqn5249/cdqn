// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/theorem.rs

//! Defines the Theorem struct, a proven, reusable reasoning path.

use crate::payloads::{read_string, read_vec_string, write_vec_string};

/// Represents a proven, reusable reasoning path.
#[derive(Debug, Clone)]
pub struct Theorem {
    /// The set of PrimeElements that are the starting conditions.
    pub premises: Vec<String>,
    /// The PrimeElement that is the proven result.
    pub conclusion: String,
    /// The full, verifiable reasoning path of intermediate CDUs.
    pub proof_path: Vec<String>,
    /// A score representing the reliability of this theorem.
    pub confidence_score: f64,
}

impl Theorem {
    /// Serializes the Theorem into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_string(&mut bytes, &self.premises);
        bytes.extend_from_slice(&(self.conclusion.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.conclusion.as_bytes());
        write_vec_string(&mut bytes, &self.proof_path);
        bytes.extend_from_slice(&self.confidence_score.to_le_bytes());
        bytes
    }

    /// Deserializes a Theorem from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let premises = read_vec_string(bytes, &mut pos)?;
        let conclusion = read_string(bytes, &mut pos)?;
        let proof_path = read_vec_string(bytes, &mut pos)?;
        if pos + 8 > bytes.len() {
            return None;
        }
        let confidence_score = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        // The 'pos' variable is not used after this point, so we don't need to increment it.
        Some(Theorem {
            premises,
            conclusion,
            proof_path,
            confidence_score,
        })
    }
}
