// src/runtime/persistence.rs

use crate::kernel::KDU;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// The Persistence service.
pub struct Persistence {
    journal_path: PathBuf,
    // The index maps a KDU ID (String) to its byte offset (u64) in the journal file.
    index: HashMap<String, u64>,
}

impl Persistence {
    // The 'new' function now scans the journal to build the index on startup.
    pub fn new(dir: &Path) -> io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        let journal_path = dir.join("00000001.journal");
        let mut index = HashMap::new();

        // Check if the journal file exists before trying to open it.
        if journal_path.exists() {
            let mut file = File::open(&journal_path)?;
            let mut current_pos = 0;
            let mut len_buffer = [0u8; 8];

            // Read the file record by record to build the index.
            while file.read_exact(&mut len_buffer).is_ok() {
                let kdu_len = u64::from_le_bytes(len_buffer);
                let mut kdu_buffer = vec![0; kdu_len as usize];
                file.read_exact(&mut kdu_buffer)?;

                let kdu: KDU = bincode::deserialize(&kdu_buffer)
                    .expect("Failed to deserialize KDU during index build");

                // The position of this KDU is where its length prefix started.
                index.insert(kdu.kdu_id, current_pos);

                current_pos += 8 + kdu_len;
            }
        }

        Ok(Persistence {
            journal_path,
            index,
        })
    }

    // The write function now also updates the in-memory index.
    pub fn write_kdu(&mut self, kdu: &KDU) -> io::Result<()> {
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.journal_path)?;

        // Get the position of the start of our write.
        let write_pos = file.seek(SeekFrom::End(0))?;

        file.write_all(&kdu_len.to_le_bytes())?;
        file.write_all(&kdu_bytes)?;

        // Add the new KDU to our live index.
        self.index.insert(kdu.kdu_id.clone(), write_pos);

        Ok(())
    }

    // Reads a KDU from the journal using the index.
    pub fn read_kdu(&self, kdu_id: &str) -> io::Result<Option<KDU>> {
        // Look up the KDU's position in the index.
        match self.index.get(kdu_id) {
            Some(&pos) => {
                let mut file = File::open(&self.journal_path)?;
                // Seek directly to the correct position.
                file.seek(SeekFrom::Start(pos))?;

                let mut len_buffer = [0u8; 8];
                file.read_exact(&mut len_buffer)?;
                let kdu_len = u64::from_le_bytes(len_buffer);

                let mut kdu_buffer = vec![0; kdu_len as usize];
                file.read_exact(&mut kdu_buffer)?;

                let kdu = bincode::deserialize(&kdu_buffer)
                    .expect("Failed to deserialize KDU during read");

                Ok(Some(kdu))
            }
            None => Ok(None), // KDU not found in index.
        }
    }
}
