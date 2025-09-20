// src/runtime/mod.rs

use crate::kernel::KDU;
use crate::runtime::scheduler::EntityScheduler;
use std::sync::mpsc::Sender;

pub mod entity;
pub mod network;
pub mod persistence;
pub mod scheduler;
pub mod test_entities;

/// Commands that can be sent to the Persistence thread.
#[derive(Debug)]
pub enum PersistenceCommand {
    WriteKdu(KDU),
    Shutdown,
}

/// The main Runtime orchestrator.
/// This struct will own and manage all the core services of a cdqn node.
pub struct Runtime {
    scheduler: EntityScheduler,
    // A channel to send commands to the persistence thread.
    persistence_tx: Sender<PersistenceCommand>,
}

impl Runtime {
    /// Creates a new Runtime.
    /// For now, it just initializes the services. In the future, it will spawn the threads.
    pub fn new() -> Self {
        // In the next step, we will create the real channel and persistence thread.
        // For now, we create a dummy channel to make the code compile.
        let (persistence_tx, _persistence_rx) = std::sync::mpsc::channel();

        Runtime {
            scheduler: EntityScheduler::default(),
            persistence_tx,
        }
    }

    /// The main run loop for the entire runtime.
    pub fn run(&mut self) {
        println!("[Runtime] Running...");
        // In the future, this loop will drive the scheduler and handle I/O.
        // For now, it does nothing.
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
