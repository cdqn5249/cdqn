// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/storage.rs

use crate::cdu::Cdu;
use crate::codec::{Decode, Encode};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

const MAGIC_BYTES: &[u8; 6] = b"CDQNv1";

/// Appends a list of serialized CDUs to the end of the log file.
pub fn append_events_to_log(events: &[Cdu], path: &Path) -> io::Result<()> {
    let is_new_file = !path.exists();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    if is_new_file {
        file.write_all(MAGIC_BYTES)?;
    }
    for event in events {
        let mut buffer = Vec::new();
        event.encode(&mut buffer);
        file.write_all(&(buffer.len() as u64).to_be_bytes())?;
        file.write_all(&buffer)?;
    }
    Ok(())
}

/// Rehydrates the state by reading the entire event log file.
pub fn rehydrate_from_log(path: &Path) -> io::Result<Vec<Cdu>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(path)?;
    let mut magic = [0u8; 6];
    file.read_exact(&mut magic)?;

    if magic != *MAGIC_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid or unsupported file format.",
        ));
    }

    let mut events = Vec::new();
    let mut len_buffer = [0u8; 8];
    while let Ok(()) = file.read_exact(&mut len_buffer) {
        let len = u64::from_be_bytes(len_buffer);
        let mut cdu_buffer = vec![0; len as usize];
        file.read_exact(&mut cdu_buffer)?;
        events.push(Cdu::decode(&mut cdu_buffer.as_slice()));
    }

    Ok(events)
}

/// Forces the synchronization of the log file's in-memory buffers to the disk.
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
        // 1. Define a temporary path for the test file.
        let mut temp_path = env::temp_dir();
        temp_path.push("cdqn_storage_rehydration_test.cdqn");
        // Ensure the file doesn't exist from a previous failed run.
        let _ = fs::remove_file(&temp_path);

        // 2. Create some events.
        let event1 = Cdu::new(b"event 1".to_vec(), "type1", vec![]);
        let event2 = Cdu::new(b"event 2".to_vec(), "type2", vec![event1.name.clone()]);
        let events = vec![event1.clone(), event2.clone()];

        // 3. Append the events to the log.
        let append_result = append_events_to_log(&events, &temp_path);
        assert!(append_result.is_ok());

        // 4. Rehydrate the log from the file.
        let rehydrated_events = rehydrate_from_log(&temp_path).expect("Failed to rehydrate log.");

        // 5. Verify that the loaded events are identical to the original.
        assert_eq!(events.len(), rehydrated_events.len());
        assert_eq!(events[0].name, rehydrated_events[0].name);
        assert_eq!(events[1].name, rehydrated_events[1].name);
        assert_eq!(events[1].metadata.causes, rehydrated_events[1].metadata.causes);

        // 6. Clean up the temporary file.
        fs::remove_file(temp_path).unwrap();
    }
}
