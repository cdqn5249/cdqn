// src/runtime/persistence.rs

use crate::kernel::KDU;
use std::fs::OpenOptions; // We only need OpenOptions, not File
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// The Persistence service.
pub struct Persistence {
    journal_path: PathBuf,
}

impl Persistence {
    // Creates a new Persistence service.
    // It takes a path to the directory where the journal files will be stored.
    pub fn new(dir: &Path) -> io::Result<Self> {
        // Create the directory if it doesn't exist.
        std::fs::create_dir_all(dir)?;
        // For now, we'll use a single, hardcoded journal file name.
        let journal_path = dir.join("00000001.journal");
        Ok(Persistence { journal_path })
    }

    // Writes a single KDU to the journal file.
    pub fn write_kdu(&self, kdu: &KDU) -> io::Result<()> {
        // Serialize the KDU into bytes using our standard format.
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;

        // Open the journal file in append mode.
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.journal_path)?;

        // Write the 8-byte length prefix (little-endian).
        file.write_all(&kdu_len.to_le_bytes())?;
        // Write the serialized KDU data.
        file.write_all(&kdu_bytes)?;

        Ok(())
    }
}
