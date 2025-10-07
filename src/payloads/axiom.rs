// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/axiom.rs

//! Defines the Axiom struct, a rule verified across multiple worlds.

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
        write_vec_string(&mut bytes, &vec![self.id.clone()]); // Re-use for single string
        write_vec_string(&mut bytes, &self.worlds);
        write_vec_string(&mut bytes, &self.premises);
        write_vec_string(&mut bytes, &vec![self.description.clone()]);
        bytes.extend_from_slice(&self.weight.to_le_bytes());
        bytes
    }

    /// Deserializes an Axiom from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let id = read_vec_string(bytes, &mut pos)?.remove(0);
        let worlds = read_vec_string(bytes, &mut pos)?;
        let premises = read_vec_string(bytes, &mut pos)?;
        let description = read_vec_string(bytes, &mut pos)?.remove(0);
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
