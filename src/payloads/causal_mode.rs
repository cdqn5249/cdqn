// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/causal_mode.rs

//! Defines the CausalMode struct for Causal Tensor Decomposition.

use crate::payloads::read_string;

/// Represents a decomposed vector of a high-level concept for CTD.
#[derive(Debug, Clone)]
pub struct CausalMode {
    /// The type of mode (e.g., "intent", "world_state").
    pub mode_type: String,
    /// The decomposed vector for this mode.
    pub vector: Vec<f64>,
    /// The original CDU that was decomposed to create this mode.
    pub source_cdu: String,
}

impl CausalMode {
    /// Serializes the CausalMode into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.mode_type.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.mode_type.as_bytes());
        bytes.extend_from_slice(&(self.vector.len() as u32).to_le_bytes());
        for val in &self.vector {
            bytes.extend_from_slice(&val.to_le_bytes());
        }
        bytes.extend_from_slice(&(self.source_cdu.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.source_cdu.as_bytes());
        bytes
    }

    /// Deserializes a CausalMode from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let mode_type = read_string(bytes, &mut pos)?;
        if pos + 4 > bytes.len() {
            return None;
        }
        let vec_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        let mut vector = Vec::with_capacity(vec_len);
        for _ in 0..vec_len {
            if pos + 8 > bytes.len() {
                return None;
            }
            let val = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            vector.push(val);
        }
        let source_cdu = read_string(bytes, &mut pos)?;
        Some(CausalMode {
            mode_type,
            vector,
            source_cdu,
        })
    }
}
