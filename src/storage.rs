// File under BaDaaS license, vibe coding engine: ChatGPT-5, OpenAI
// File path: src/storage.rs

use crate::cdu::Cdu;
use crate::codec::{Decode, Encode};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

const MAGIC_BYTES: &[u8; 6] = b"CDQNv1";

/// Global lock to serialize writes across all threads.
/// Prevents interleaved appends that could corrupt the log file.
static FILE_WRITE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Appends a batch of CDUs to the log file.
/// Fully synchronized and cross-platform safe.
pub fn append_events_to_log(events: &[Cdu], path: &Path) -> io::Result<()> {
    // Prepare in-memory buffer
    let mut buffer = Vec::new();
    let is_new_file = !path.exists();
    if is_new_file {
        buffer.extend_from_slice(MAGIC_BYTES);
    }

    for event in events {
        let mut event_buf = Vec::new();
        event.encode(&mut event_buf);
        buffer.extend_from_slice(&(event_buf.len() as u64).to_be_bytes());
        buffer.extend_from_slice(&event_buf);
    }

    // Acquire global mutex
    let lock = FILE_WRITE_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap();

    // Single atomic write + flush
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(&buffer)?;
    file.sync_all()?; // ensure written to disk

    // Windows-specific barrier
    drop(file);
    #[cfg(target_os = "windows")]
    thread::sleep(Duration::from_millis(20));

    Ok(())
}

/// Reads and decodes all events from the log file.
/// Retries on transient truncation errors caused by concurrent writers.
pub fn rehydrate_from_log(path: &Path) -> io::Result<Vec<Cdu>> {
    const MAX_RETRIES: usize = 5;
    const RETRY_DELAY_MS: u64 = 40;

    for attempt in 0..=MAX_RETRIES {
        match try_rehydrate(path) {
            Ok(events) => return Ok(events),
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                if attempt < MAX_RETRIES {
                    eprintln!(
                        "[Rehydrate] Partial file read ({}). Retrying... ({}/{})",
                        e, attempt + 1, MAX_RETRIES
                    );
                    thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
                    continue;
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!(
                            "Truncated payload after {} retries — giving up. Original: {}",
                            MAX_RETRIES, e
                        ),
                    ));
                }
            }
            Err(e) => return Err(e),
        }
    }

    unreachable!()
}

/// Internal helper for actual file reading/decoding.
fn try_rehydrate(path: &Path) -> io::Result<Vec<Cdu>> {
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
    if magic != MAGIC_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or unsupported file format.",
        ));
    }
    slice = rest;

    let mut events = Vec::new();

    while !slice.is_empty() {
        if slice.len() < std::mem::size_of::<u64>() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Truncated length header",
            ));
        }
        let len = u64::decode(&mut slice) as usize;

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
