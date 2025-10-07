// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/semi_axiom.rs

/// Represents a semi-axiom as a prime ideal of prime elements in Chronosa's reasoning model.
#[derive(Debug, Clone)]
pub struct SemiAxiom {
    /// Unique identifier for the semi-axiom
    pub id: String,
    /// World context this semi-axiom belongs to
    pub world: String,
    /// Constituent prime elements that form this prime ideal
    pub prime_elements: Vec<String>, // Storing IDs for now
    /// Description of the semi-axiom's rule or constraint
    pub description: String,
    /// Current weight of the semi-axiom, determined by links
    pub weight: f64,
}

impl SemiAxiom {
    /// Create a new semi-axiom
    pub fn new(
        id: String,
        world: String,
        prime_elements: Vec<String>,
        description: String,
    ) -> Self {
        Self {
            id,
            world,
            prime_elements,
            description,
            weight: 1.0, // Default initial weight
        }
    }

    /// Update the weight of the semi-axiom
    pub fn update_weight(&mut self, new_weight: f64) {
        self.weight = new_weight;
    }

    // The to_cdu method is removed for now as it depends on the old serialization.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semi_axiom_creation() {
        let axiom = SemiAxiom::new(
            "test_axiom".to_string(),
            "test_world".to_string(),
            vec!["element1".to_string(), "element2".to_string()],
            "A test semi-axiom".to_string(),
        );

        assert_eq!(axiom.id, "test_axiom");
        assert_eq!(axiom.world, "test_world");
        assert_eq!(axiom.prime_elements.len(), 2);
        assert_eq!(axiom.weight, 1.0);
    }

    // The serialization test has been removed as it is no longer valid.
}
