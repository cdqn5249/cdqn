// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/storage.rs

//! The storage module for the cdqn project.
//!
//! Provides functionality to persist and load the `ChronosaCore` state
//! using a custom binary format.

use crate::codec::{Decode, Encode};
use crate::core::ChronosaCore;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const MAGIC_BYTES: &[u8; 6] = b"CDQNv1";

/// Saves the state of a ChronosaCore to a file.
pub fn save_core(core: &ChronosaCore, path: &Path) -> std::io::Result<()> {
    let mut buffer = Vec::new();
    core.encode(&mut buffer);

    let mut file = File::create(path)?;
    file.write_all(MAGIC_BYTES)?;
    file.write_all(&buffer)?;
    Ok(())
}

/// Loads the state of a ChronosaCore from a file.
pub fn load_core(path: &Path) -> std::io::Result<ChronosaCore> {
    let mut file = File::open(path)?;
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;

    if magic != *MAGIC_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid or unsupported file format.",
        ));
    }

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(ChronosaCore::decode(&mut buffer.as_slice()))
}

// --- Unit tests for the storage module ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    // Helper function to create a core with some data for testing.
    fn create_test_core() -> ChronosaCore {
        let core = ChronosaCore::new();
        let (core, cdu1) = core.record(b"Event 1".to_vec(), "type1");
        let (core, _) = core.record_causal(b"Event 2".to_vec(), "type2", &[&cdu1]);
        core
    }

    #[test]
    fn test_save_and_load_cycle() {
        // 1. Create a core with some data.
        let original_core = create_test_core();

        // 2. Define a temporary path for the test file.
        let mut temp_path = env::temp_dir();
        temp_path.push("cdqn_storage_test.cdqn");

        // 3. Save the core.
        let save_result = save_core(&original_core, &temp_path);
        assert!(save_result.is_ok());

        // 4. Load the core back from the file.
        let loaded_core = load_core(&temp_path).expect("Failed to load core.");

        // 5. Verify that the loaded core is identical to the original.
        // We compare essential properties to confirm correctness.
        assert_eq!(original_core.len(), loaded_core.len());
        assert_eq!(original_core.log()[1].name, loaded_core.log()[1].name);
        assert_eq!(
            original_core.log()[1].metadata.causes,
            loaded_core.log()[1].metadata.causes
        );
        assert_eq!(
            original_core.log()[1].metadata.hlc,
            loaded_core.log()[1].metadata.hlc
        );

        // 6. Clean up the temporary file.
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_load_invalid_magic_bytes() {
        // 1. Create a dummy file with incorrect magic bytes.
        let mut temp_path = env::temp_dir();
        temp_path.push("cdqn_invalid_test.cdqn");
        let mut file = File::create(&temp_path).unwrap();
        file.write_all(b"WRONGv1").unwrap();

        // 2. Attempt to load the core and verify it fails with the correct error.
        let load_result = load_core(&temp_path);
        assert!(load_result.is_err());
        assert_eq!(
            load_result.err().unwrap().kind(),
            std::io::ErrorKind::InvalidData
        );

        // 3. Clean up.
        fs::remove_file(temp_path).unwrap();
    }
}
