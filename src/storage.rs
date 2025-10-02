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

// --- FIX: Magic bytes now represent the CDQN project ---
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

// (Unit tests for storage can be added here later)
