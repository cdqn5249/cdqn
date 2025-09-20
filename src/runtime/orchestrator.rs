// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::persistence::Persistence;
use crate::runtime::processor::EntityProcessor;
use crate::runtime::PersistenceCommand;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub struct Orchestrator {
    processor: EntityProcessor,
    persistence_tx: Sender<PersistenceCommand>,
    persistence_handle: Option<JoinHandle<()>>,
}

// Implement the Default trait as suggested by clippy.
impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl Orchestrator {
    pub fn new() -> Self {
        let db_path = Path::new("./cdqn_runtime_db");
        let (persistence_handle, persistence_tx) = Persistence::spawn(db_path);
        Orchestrator {
            processor: EntityProcessor::default(),
            persistence_tx,
            persistence_handle: Some(persistence_handle),
        }
    }

    pub fn processor_mut(&mut self) -> &mut EntityProcessor {
        &mut self.processor
    }

    pub fn route_initial_kdu(&self, target_fqei: &FQEI, kdu: KDU) {
        // Journal the initial KDU before routing it.
        self.persistence_tx
            .send(PersistenceCommand::WriteKdu(Box::new(kdu.clone())))
            .unwrap();
        self.processor.route_local(target_fqei, kdu);
    }

    pub fn run(&mut self) {
        println!("[Orchestrator] Starting main loop...");
        for turn in 1..=3 {
            println!("\n--- Turn {} ---", turn);
            let outgoing_routes = self.processor.run_turn();

            if outgoing_routes.is_empty() {
                println!("[Orchestrator] System is quiet.");
            }

            for (target_fqei, kdu) in outgoing_routes {
                println!(
                    "[Orchestrator] Journaling and routing KDU from {} to {}",
                    kdu.originator_fqei, target_fqei
                );
                // 1. Journal the outgoing KDU.
                self.persistence_tx
                    .send(PersistenceCommand::WriteKdu(Box::new(kdu.clone())))
                    .unwrap();
                // 2. Route it to its local target.
                self.processor.route_local(&target_fqei, kdu);
            }
        }
        println!("\n[Orchestrator] Simulation finished.");
    }

    pub fn shutdown(&mut self) {
        println!("[Orchestrator] Shutting down...");
        self.persistence_tx
            .send(PersistenceCommand::Shutdown)
            .unwrap();
        if let Some(handle) = self.persistence_handle.take() {
            handle.join().expect("Persistence thread panicked");
        }
        println!("[Orchestrator] Shutdown complete.");
    }
}
