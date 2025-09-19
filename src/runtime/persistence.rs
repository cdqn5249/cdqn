// src/runtime/persistence.rs

use crate::kernel::KDU;
use rocksdb::{Options, DB};
use std::path::Path;

// The Persistence service. It holds a connection to the RocksDB database.
pub struct Persistence {
    db: DB,
}

impl Persistence {
    // Creates a new Persistence service, opening or creating a database at the given path.
    pub fn new(path: &Path) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Persistence { db })
    }

    // Writes a KDU to the database.
    // The KDU's ID is the key, and its serialized JSON is the value.
    pub fn write_kdu(&self, kdu: &KDU) -> Result<(), rocksdb::Error> {
        let key = kdu.kdu_id.as_bytes();
        let value = serde_json::to_vec(kdu).expect("Failed to serialize KDU");
        self.db.put(key, &value)
    }

    // Reads a KDU from the database using its ID.
    pub fn read_kdu(&self, kdu_id: &str) -> Result<Option<KDU>, String> {
        let key = kdu_id.as_bytes();
        match self.db.get(key) {
            Ok(Some(value)) => {
                let kdu = serde_json::from_slice(&value)
                    .map_err(|e| format!("Failed to deserialize KDU: {}", e))?;
                Ok(Some(kdu))
            }
            Ok(None) => Ok(None), // KDU not found
            Err(e) => Err(format!("Failed to read from DB: {}", e)),
        }
    }
}
