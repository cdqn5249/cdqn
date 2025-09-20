// src/runtime/mod.rs

use crate::kernel::KDU;
use crate::runtime::persistence::Persistence;
use crate::runtime::scheduler::EntityScheduler;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub mod entity;
pub mod network;
pub mod persistence;
pub mod scheduler;
pub mod test_entities;

#[derive(Debug)]
pub enum PersistenceCommand {
    WriteKdu(Box<KDU>),
    Shutdown,
}

pub struct Runtime {
    scheduler: EntityScheduler,
    persistence_tx: Sender<PersistenceCommand>,
    // We need to hold the handle to the thread to shut it down later.
    persistence_handle: Option<JoinHandle<()>>,
}

impl Runtime {
    pub fn new() -> Self {
        let db_path = Path::new("./cdqn_runtime_db");
        let (persistence_handle, persistence_tx) = Persistence::spawn(db_path);

        Runtime {
            scheduler: EntityScheduler::default(),
            persistence_tx,
            persistence_handle: Some(persistence_handle),
        }
    }

    /// The main run loop for the entire runtime.
    pub fn run(&mut self) {
        println!("[Runtime] Running...");
        // For this milestone, the run loop will just set up and run a test simulation.
        self.test_ping_pong();
    }

    /// Cleanly shuts down the runtime and its threads.
    pub fn shutdown(&mut self) {
        println!("[Runtime] Shutting down...");
        self.persistence_tx
            .send(PersistenceCommand::Shutdown)
            .expect("Failed to send shutdown command to persistence thread");

        if let Some(handle) = self.persistence_handle.take() {
            handle.join().expect("Persistence thread panicked");
        }
        println!("[Runtime] Shutdown complete.");
    }
}

// This block contains the test simulation logic, moved from main.rs
impl Runtime {
    fn test_ping_pong(&mut self) {
        // The test logic will go here in the next step.
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
