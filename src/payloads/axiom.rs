// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/axiom.rs

//! Defines the Axiom struct, a rule verified across multiple worlds.

// FIX: Correctly import and use read_string.
use crate::payloads::{read_string, read_vec_string, write_vec_string};

/// Represents a rule or transformation that has been verified to be valid
/// and consistent across multiple Worlds.
#[derive(Debug, Clone)]
pub struct Axiom {
    /// Unique identifier for the axiom.
    pub id: String,
    /// The list of worlds this axiom bridges.
    pub worlds: Vec<String>,
    /// The set of PrimeElements that act as premises.
    pub premises: Vec<String>,
    /// A description of the universal truth this axiom represents.
    pub description: String,
    /// The weight or confidence in this axiom.
    pub weight: f64,
}

impl Axiom {
    /// Serializes the Axiom into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Helper to write a single string.
        let write_string = |bytes: &mut Vec<u8>, s: &str| {
            bytes.extend_from_slice(&(s.len() as u32).to_le_bytes());
            bytes.extend_from_slice(s.as_bytes());
        };

        // FIX: Use the correct serialization for single strings.
        write_string(&mut bytes, &self.id);
        write_vec_string(&mut bytes, &self.worlds);
        write_vec_string(&mut bytes, &self.premises);
        write_string(&mut bytes, &self.description);
        bytes.extend_from_slice(&self.weight.to_le_bytes());
        bytes
    }

    /// Deserializes an Axiom from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        // FIX: Use the correct deserialization for single strings.
        let id = read_string(bytes, &mut pos)?;
        let worlds = read_vec_string(bytes, &mut pos)?;
        let premises = read_vec_string(bytes, &mut pos)?;
        let description = read_string(bytes, &mut pos)?;
        if pos + 8 > bytes.len() {
            return None;
        }
        let weight = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
        Some(Axiom {
            id,
            worlds,
            premises,
            description,
            weight,
        })
    }
}
