// src/runtime/persistence.rs

use crate::kernel::KDU;
use crate::runtime::PersistenceCommand;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

/// The Persistence service. It now runs in its own dedicated thread.
pub struct Persistence {
    journal_path: PathBuf,
    command_rx: Receiver<PersistenceCommand>,
}

impl Persistence {
    /// Spawns the persistence service in a new thread.
    /// Returns a JoinHandle for the thread and a Sender channel to send commands to it.
    pub fn spawn(dir: &Path) -> (JoinHandle<()>, Sender<PersistenceCommand>) {
        let (command_tx, command_rx) = std::sync::mpsc::channel();
        let journal_path = dir.join("00000001.journal");
        std::fs::create_dir_all(dir).expect("Could not create persistence directory");

        let mut persistence = Persistence {
            journal_path,
            command_rx,
        };

        let handle = thread::spawn(move || {
            persistence.run();
        });

        (handle, command_tx)
    }

    /// The main run loop for the persistence thread.
    /// It listens for commands on the channel and processes them.
    fn run(&mut self) {
        println!("[Persistence] Thread started. Waiting for commands.");
        for command in &self.command_rx {
            match command {
                PersistenceCommand::WriteKdu(kdu) => {
                    if self.write_kdu(&kdu).is_err() {
                        eprintln!("[Persistence] ERROR: Failed to write KDU to journal.");
                    }
                }
                PersistenceCommand::Shutdown => {
                    println!("[Persistence] Shutdown command received. Exiting.");
                    break;
                }
            }
        }
    }

    /// Writes a single KDU to the journal file. (This logic is unchanged).
    fn write_kdu(&self, kdu: &KDU) -> io::Result<()> {
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.journal_path)?;

        file.write_all(&kdu_len.to_le_bytes())?;
        file.write_all(&kdu_bytes)?;
        println!("[Persistence] Wrote KDU with ID: {}", kdu.kdu_id);
        Ok(())
    }
}
