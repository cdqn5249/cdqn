// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/constraint.rs

//! Defines the Constraint struct, an emergent guardrail for reasoning.

use crate::payloads::{read_string, read_vec_string, write_vec_string};

/// Represents a guardrail; a rule that inhibits a reasoning path in a specific context.
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The reasoning path that is being inhibited.
    pub target_path: Vec<String>,
    /// The context in which the path is invalid.
    pub inhibiting_context: String,
    /// The reason for the inhibition (e.g., "LeadsToImpossibility").
    pub reason: String,
    /// The world in which this constraint applies.
    pub world: String,
}

impl Constraint {
    /// Serializes the Constraint into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_vec_string(&mut bytes, &self.target_path);
        bytes.extend_from_slice(&(self.inhibiting_context.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.inhibiting_context.as_bytes());
        bytes.extend_from_slice(&(self.reason.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.reason.as_bytes());
        bytes.extend_from_slice(&(self.world.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.world.as_bytes());
        bytes
    }

    /// Deserializes a Constraint from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let target_path = read_vec_string(bytes, &mut pos)?;
        let inhibiting_context = read_string(bytes, &mut pos)?;
        let reason = read_string(bytes, &mut pos)?;
        let world = read_string(bytes, &mut pos)?;
        Some(Constraint {
            target_path,
            inhibiting_context,
            reason,
            world,
        })
    }
}
