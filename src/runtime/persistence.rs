// src/runtime/persistence.rs

use crate::kernel::KDU;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// The Persistence service. This is the simple, sovereign, non-threaded version.
pub struct Persistence {
    journal_path: PathBuf,
    // The index maps a KDU ID (String) to its byte offset (u64) in the journal file.
    #[allow(dead_code)] // Index will be used in a future milestone
    index: HashMap<String, u64>,
}

impl Persistence {
    pub fn new(dir: &Path) -> io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        let journal_path = dir.join("00000001.journal");
        let index = HashMap::new();

        // NOTE: Index-building logic is temporarily removed for this milestone.
        // We are focusing on the Orchestrator/Processor interaction first.

        Ok(Persistence {
            journal_path,
            index,
        })
    }

    /// Writes a single KDU to the journal file.
    pub fn write_kdu(&mut self, kdu: &KDU) -> io::Result<()> {
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.journal_path)?;

        file.write_all(&kdu_len.to_le_bytes())?;
        file.write_all(&kdu_bytes)?;

        Ok(())
    }
}
