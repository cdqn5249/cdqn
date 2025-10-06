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

        // Serialize id
        bytes.extend_from_slice(&(self.id.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.id.as_bytes());

        // Serialize world
        bytes.extend_from_slice(&(self.world.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.world.as_bytes());

        // Serialize representation vector
        bytes.extend_from_slice(&(self.representation.len() as u32).to_le_bytes());
        for val in &self.representation {
            bytes.extend_from_slice(&val.to_le_bytes());
        }

        // Serialize description
        bytes.extend_from_slice(&(self.description.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.description.as_bytes());

        // Serialize irreducibility_proof
        bytes.extend_from_slice(&(self.irreducibility_proof.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.irreducibility_proof.as_bytes());

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

        Some(PrimeElement {
            id,
            world,
            representation,
            description,
            irreducibility_proof,
            relationships: HashMap::new(), // Note: relationships are not serialized for simplicity
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

        // Add to main storage
        self.elements.insert(id.clone(), element);

        // Update world index
        if !self.world_index.contains_key(&world) {
            self.world_index.insert(world.clone(), Vec::new());
        }
        self.world_index.get_mut(&world).unwrap().push(id);
    }

    /// Get a prime element by ID
    pub fn get_element(&self, id: &str) -> Option<&PrimeElement> {
        self.elements.get(id)
    }

    /// Get all prime elements for a specific world
    pub fn get_elements_by_world(&self, world: &str) -> Vec<&PrimeElement> {
        if let Some(ids) = self.world_index.get(world) {
            ids.iter().filter_map(|id| self.elements.get(id)).collect()
        } else {
            Vec::new()
        }
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
    }

    #[test]
    fn test_prime_element_manager() {
        let mut manager = PrimeElementManager::new();

        let element = PrimeElement::new(
            "test_element".to_string(),
            "test_world".to_string(),
            vec![42.0],
            "A test element".to_string(),
            "Cannot be decomposed".to_string(),
        );

        manager.add_element(element);

        assert!(manager.is_prime("test_element"));
        assert!(manager.get_element("test_element").is_some());

        let world_elements = manager.get_elements_by_world("test_world");
        assert_eq!(world_elements.len(), 1);
    }
}
