// File under BaDaaS license, vibe coding engine: ChatGPT-5, OpenAI
// File path: src/storage.rs

use crate::cdu::Cdu;
use crate::codec::{Decode, Encode};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};

const MAGIC_BYTES: &[u8; 6] = b"CDQNv1";

/// A process-global lock to serialize writes to the on-disk log.
/// This prevents concurrent threads from interleaving length+payload
/// writes and corrupting the log.
static FILE_WRITE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Appends a list of serialized CDUs to the end of the log file.
/// This function builds a single buffer for the entire batch
/// and writes it while holding a global mutex to avoid interleaving.
/// It now also forces a `sync_all()` to ensure full flush before rehydration.
pub fn append_events_to_log(events: &[Cdu], path: &Path) -> io::Result<()> {
    // Build a single write buffer containing length+payload for each event.
    let mut buffer = Vec::new();

    // If a new file will be created we need to include the magic bytes first.
    let is_new_file = !path.exists();
    if is_new_file {
        buffer.extend_from_slice(MAGIC_BYTES);
    }

    for event in events {
        let mut event_buf = Vec::new();
        event.encode(&mut event_buf);

        // Prepend length header (u64, big-endian) followed by payload bytes.
        buffer.extend_from_slice(&(event_buf.len() as u64).to_be_bytes());
        buffer.extend_from_slice(&event_buf);
    }

    // Acquire the global write lock to ensure this write is not interleaved
    // with other concurrent writers.
    let lock = FILE_WRITE_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();

    // Open with append mode and perform a single write of our entire buffer.
    // Then flush to disk to guarantee completeness before any rehydration.
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(&buffer)?;
    file.sync_all()?; // ✅ ensures full flush before other threads read it

    Ok(())
}

/// Rehydrate the log file back into a list of CDUs.
/// This function expects a consistent file format produced by append_events_to_log.
pub fn rehydrate_from_log(path: &Path) -> io::Result<Vec<Cdu>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(path)?;
    let mut raw = Vec::new();
    file.read_to_end(&mut raw)?;

    let mut slice: &[u8] = &raw;

    // Verify magic bytes
    if slice.len() < MAGIC_BYTES.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "File too short"));
    }
    let (magic, rest) = slice.split_at(MAGIC_BYTES.len());
    slice = rest;
    if magic != MAGIC_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or unsupported file format.",
        ));
    }

    let mut events = Vec::new();

    while !slice.is_empty() {
        // Read length header (u64)
        if slice.len() < std::mem::size_of::<u64>() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Truncated length header",
            ));
        }
        let len = u64::decode(&mut slice) as usize;

        // Defensive: ensure there are enough bytes left for the payload.
        if slice.len() < len {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Truncated payload",
            ));
        }

        let (cdu_bytes, rest) = slice.split_at(len);
        slice = rest;
        let mut cdu_slice = cdu_bytes;
        let cdu = Cdu::decode(&mut cdu_slice);
        events.push(cdu);
    }

    Ok(events)
}

/// Forces the synchronization of the log file's in-memory buffers to the disk.
/// Useful when external systems need deterministic persistence guarantees.
pub fn sync_log_to_disk(path: &Path) -> io::Result<()> {
    if path.exists() {
        let file = File::open(path)?;
        file.sync_all()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdu::Cdu;
    use std::env;
    use std::fs;

    #[test]
    fn test_log_rehydration_cycle() {
        let mut temp_path = env::temp_dir();
        temp_path.push("cdqn_storage_rehydration_test.cdqn");
        let _ = fs::remove_file(&temp_path);

        let event1 = Cdu::new(b"event 1".to_vec(), "type1", vec![]);
        let event2 = Cdu::new(b"event 2".to_vec(), "type2", vec![event1.name.clone()]);
        let events = vec![event1.clone(), event2.clone()];

        append_events_to_log(&events, &temp_path).unwrap();
        let rehydrated_events = rehydrate_from_log(&temp_path).expect("Failed to rehydrate log.");

        assert_eq!(events.len(), rehydrated_events.len());
        assert_eq!(events[0].name, rehydrated_events[0].name);
        assert_eq!(events[1].name, rehydrated_events[1].name);
        assert_eq!(
            events[1].metadata.causes,
            rehydrated_events[1].metadata.causes
        );

        fs::remove_file(temp_path).unwrap();
    }
}
