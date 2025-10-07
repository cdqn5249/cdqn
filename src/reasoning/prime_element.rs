// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/prime_element.rs

use crate::cdu::Cdu;
use std::collections::HashMap;

/// Represents a prime element in Chronosa's reasoning model.
#[derive(Debug, Clone)]
pub struct PrimeElement {
    /// Unique identifier for the prime element
    pub id: String,
    /// World context this prime element belongs to
    pub world: String,
    /// Mathematical representation as a vector in a multi-dimensional space.
    pub representation: Vec<f64>,
    /// Conceptual description
    pub description: String,
    /// Proof of irreducibility
    pub irreducibility_proof: String,
    /// The ID of the corresponding symmetric prime element, if known.
    pub symmetric_pair: Option<String>,
    /// Relationships to other prime elements
    pub relationships: HashMap<String, String>,
}

impl PrimeElement {
    /// Create a new prime element
    pub fn new(
        id: String,
        world: String,
        representation: Vec<f64>,
        description: String,
        irreducibility_proof: String,
    ) -> Self {
        Self {
            id,
            world,
            representation,
            description,
            irreducibility_proof,
            symmetric_pair: None, // A new element's pair is initially unknown.
            relationships: HashMap::new(),
        }
    }

    /// Add a relationship to another prime element
    pub fn add_relationship(&mut self, target_id: String, relationship_type: String) {
        self.relationships.insert(target_id, relationship_type);
    }

    /// Convert to CDU
    pub fn to_cdu(&self) -> Cdu {
        let payload_bytes = self.to_bytes();
        let subtype = format!("prime.element.{}", self.world);
        Cdu::new(payload_bytes, &subtype, vec![])
    }

    /// Convert the PrimeElement to a byte representation for storage
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Helper to write a string
        let write_string = |bytes: &mut Vec<u8>, s: &str| {
            bytes.extend_from_slice(&(s.len() as u32).to_le_bytes());
            bytes.extend_from_slice(s.as_bytes());
        };

        write_string(&mut bytes, &self.id);
        write_string(&mut bytes, &self.world);

        // Serialize representation vector
        bytes.extend_from_slice(&(self.representation.len() as u32).to_le_bytes());
        for val in &self.representation {
            bytes.extend_from_slice(&val.to_le_bytes());
        }

        write_string(&mut bytes, &self.description);
        write_string(&mut bytes, &self.irreducibility_proof);

        // Serialize symmetric_pair
        match &self.symmetric_pair {
            Some(pair_id) => {
                bytes.push(1); // Tag for Some
                write_string(&mut bytes, pair_id);
            }
            None => {
                bytes.push(0); // Tag for None
            }
        }

        bytes
    }

    /// Create a PrimeElement from a byte representation
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;

        // Helper to read a string
        let read_string = |bytes: &[u8], pos: &mut usize| -> Option<String> {
            if *pos + 4 > bytes.len() {
                return None;
            }
            let len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
            *pos += 4;
            if *pos + len > bytes.len() {
                return None;
            }
            let s = String::from_utf8(bytes[*pos..*pos + len].to_vec()).ok()?;
            *pos += len;
            Some(s)
        };

        let id = read_string(bytes, &mut pos)?;
        let world = read_string(bytes, &mut pos)?;

        // Deserialize representation vector
        if pos + 4 > bytes.len() {
            return None;
        }
        let vec_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().ok()?) as usize;
        pos += 4;
        let mut representation = Vec::with_capacity(vec_len);
        for _ in 0..vec_len {
            if pos + 8 > bytes.len() {
                return None;
            }
            let val = f64::from_le_bytes(bytes[pos..pos + 8].try_into().ok()?);
            pos += 8;
            representation.push(val);
        }

        let description = read_string(bytes, &mut pos)?;
        let irreducibility_proof = read_string(bytes, &mut pos)?;

        // Deserialize symmetric_pair
        if pos >= bytes.len() {
            return None;
        }
        let symmetric_pair = match bytes[pos] {
            1 => {
                pos += 1;
                Some(read_string(bytes, &mut pos)?)
            }
            _ => {
                // FIX: The pos increment was unused. The byte is consumed by the match.
                None
            }
        };

        Some(PrimeElement {
            id,
            world,
            representation,
            description,
            irreducibility_proof,
            symmetric_pair,
            relationships: HashMap::new(),
        })
    }
}

/// Manager for prime elements
#[derive(Debug)]
pub struct PrimeElementManager {
    /// Storage for prime elements by ID
    elements: HashMap<String, PrimeElement>,
    /// Index of elements by world
    world_index: HashMap<String, Vec<String>>,
}

impl Default for PrimeElementManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimeElementManager {
    /// Create a new prime element manager
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            world_index: HashMap::new(),
        }
    }

    /// Add a prime element
    pub fn add_element(&mut self, element: PrimeElement) {
        let world = element.world.clone();
        let id = element.id.clone();
        self.elements.insert(id.clone(), element);
        self.world_index.entry(world).or_default().push(id);
    }

    /// Get a prime element by ID
    pub fn get_element(&self, id: &str) -> Option<&PrimeElement> {
        self.elements.get(id)
    }

    /// Get all prime elements for a specific world
    pub fn get_elements_by_world(&self, world: &str) -> Vec<&PrimeElement> {
        self.world_index
            .get(world)
            .map(|ids| ids.iter().filter_map(|id| self.elements.get(id)).collect())
            .unwrap_or_default()
    }

    /// Check if an element is prime (irreducible)
    pub fn is_prime(&self, id: &str) -> bool {
        self.elements.contains_key(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_element_creation() {
        let element = PrimeElement::new(
            "test_element".to_string(),
            "test_world".to_string(),
            vec![42.0, 1.0],
            "A test element".to_string(),
            "Cannot be decomposed".to_string(),
        );

        assert_eq!(element.id, "test_element");
        assert_eq!(element.world, "test_world");
        assert_eq!(element.representation, vec![42.0, 1.0]);
        assert!(element.symmetric_pair.is_none());
    }

    #[test]
    fn test_prime_element_serialization_cycle() {
        let mut element = PrimeElement::new(
            "test_element".to_string(),
            "test_world".to_string(),
            vec![1.0, 2.0],
            "A test element".to_string(),
            "Proof".to_string(),
        );
        element.symmetric_pair = Some("symmetric_id".to_string());

        let bytes = element.to_bytes();
        let deserialized = PrimeElement::from_bytes(&bytes).unwrap();

        assert_eq!(element.id, deserialized.id);
        assert_eq!(element.world, deserialized.world);
        assert_eq!(element.representation, deserialized.representation);
        assert_eq!(element.description, deserialized.description);
        assert_eq!(
            element.irreducibility_proof,
            deserialized.irreducibility_proof
        );
        assert_eq!(element.symmetric_pair, deserialized.symmetric_pair);
    }
}
