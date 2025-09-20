// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::network::NodeServer; // Import NodeServer
use crate::runtime::persistence::Persistence;
use crate::runtime::processor::EntityProcessor;
use crate::runtime::PersistenceCommand;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub struct Orchestrator {
    processor: EntityProcessor,
    persistence_tx: Sender<PersistenceCommand>,
    persistence_handle: Option<JoinHandle<()>>,
    // Add handles for the network thread
    network_rx: Receiver<KDU>,
    network_handle: Option<JoinHandle<()>>,
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl Orchestrator {
    pub fn new() -> Self {
        let db_path = Path::new("./cdqn_runtime_db");
        let (persistence_handle, persistence_tx) = Persistence::spawn(db_path);

        // Spawn the network server and get its handles
        let (network_tx, network_rx) = std::sync::mpsc::channel();
        let network_handle = NodeServer::spawn("127.0.0.1:8082", network_tx);

        Orchestrator {
            processor: EntityProcessor::default(),
            persistence_tx,
            persistence_handle: Some(persistence_handle),
            network_rx,
            network_handle: Some(network_handle),
        }
    }

    pub fn processor_mut(&mut self) -> &mut EntityProcessor {
        &mut self.processor
    }

    pub fn route_initial_kdu(&self, target_fqei: &FQEI, kdu: KDU) {
        self.persistence_tx
            .send(PersistenceCommand::WriteKdu(Box::new(kdu.clone())))
            .unwrap();
        self.processor.route_local(target_fqei, kdu);
    }

    pub fn run(&mut self) {
        println!("[Orchestrator] Starting main loop...");
        // The loop will now also check for incoming network messages.
        for turn in 1..=5 { // Run for a few more turns
            println!("\n--- Turn {} ---", turn);

            // 1. Check for and route incoming KDUs from the network
            if let Ok(network_kdu) = self.network_rx.try_recv() {
                println!("[Orchestrator] Received KDU from network with ID: {}", network_kdu.kdu_id);
                // For now, assume all network KDUs are for the ponger
                self.route_initial_kdu(&"ponger@test".to_string(), network_kdu);
            }

            let outgoing_routes = self.processor.run_turn();

            if outgoing_routes.is_empty() {
                println!("[Orchestrator] System is quiet.");
            }

            for (target_fqei, kdu) in outgoing_routes {
                println!(
                    "[Orchestrator] Journaling and routing KDU from {} to {}",
                    kdu.originator_fqei, target_fqei
                );
                self.persistence_tx
                    .send(PersistenceCommand::WriteKdu(Box::new(kdu.clone())))
                    .unwrap();
                
                // In the final step, we will add logic here to check if the target
                // is local or remote. For now, we still route locally.
                self.processor.route_local(&target_fqei, kdu);
            }
            // Give I/O threads a moment to work
            std::thread::sleep(std::time::Duration::from_millis(10));
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
        // We don't have a graceful shutdown for the network thread yet,
        // it will just exit when the program ends. This is ok for now.
        if let Some(handle) = self.network_handle.take() {
            // In a real app, we'd signal the network thread to shut down.
            // For now, we just let it run its course.
        }
        println!("[Orchestrator] Shutdown complete.");
    }
}
