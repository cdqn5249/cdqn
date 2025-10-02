// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/cdu.rs

//! The Causal Data Unit (CDU) module.
//!
//! This is the fundamental, atomic unit of memory for the Chronosa agent.

// (Keep all the existing code from the previous step here, from line 1 to the end of the `impl Cdu` block)
// ... existing code ...
impl Cdu {
    // ... existing `new` function ...
}


// --- ADD THIS TEST MODULE AT THE END OF THE FILE ---

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module (cdu)

    #[test]
    fn test_cdu_creation_and_naming() {
        // 1. Define a sample payload and subtype.
        let payload = b"Test payload data".to_vec();
        let subtype = "test_event";

        // 2. Create the CDU.
        let cdu = Cdu::new(payload.clone(), subtype, vec![]);

        // 3. Verify the name construction.
        // We don't know the exact hash, but we can verify the structure.
        assert!(cdu.name.contains(subtype));
        assert!(cdu.name.ends_with(".cdu"));
        assert_ne!(cdu.name, format!(".{}.cdu", subtype), "Hash part of the name should not be empty.");

        // 4. Verify the payload was stored correctly.
        assert_eq!(cdu.payload, payload);
    }

    #[test]
    fn test_cdu_causal_link() {
        // 1. Create a "cause" CDU.
        let cause_cdu = Cdu::new(b"First event".to_vec(), "genesis", vec![]);

        // 2. Create a second CDU that is caused by the first.
        let effect_cdu = Cdu::new(
            b"Second event".to_vec(),
            "response",
            vec![cause_cdu.name.clone()],
        );

        // 3. Verify that the causal link was stored correctly.
        assert_eq!(effect_cdu.metadata.causes.len(), 1);
        assert_eq!(effect_cdu.metadata.causes[0], cause_cdu.name);
    }
}
