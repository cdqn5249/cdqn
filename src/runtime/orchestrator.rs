// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::network::NodeServer;
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
    network_rx: Receiver<KDU>,
    network_handle: Option<JoinHandle<()>>,
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new("127.0.0.1:8082") // Default address for default implementation
    }
}

impl Orchestrator {
    // new() now takes the listen_addr as an argument.
    pub fn new(listen_addr: &str) -> Self {
        let db_path = Path::new("./cdqn_runtime_db");
        let (persistence_handle, persistence_tx) = Persistence::spawn(db_path);
        let (network_tx, network_rx) = std::sync::mpsc::channel();
        let network_handle = NodeServer::spawn(listen_addr, network_tx);

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
        println!("[Orchestrator] Starting main event loop. Press Ctrl+C to exit.");
        loop {
            if let Ok(network_kdu) = self.network_rx.try_recv() {
                // When a KDU comes from the network, assume it's for the first local entity.
                // This is a simplification for our MVP test.
                let target_fqei = self
                    .processor
                    .get_local_fqeis()
                    .into_iter()
                    .next()
                    .unwrap_or_default();
                self.route_initial_kdu(&target_fqei, network_kdu);
            }

            let outgoing_routes = self.processor.run_turn();
            let local_fqeis = self.processor.get_local_fqeis();

            for (target_fqei, kdu) in outgoing_routes {
                self.persistence_tx
                    .send(PersistenceCommand::WriteKdu(Box::new(kdu.clone())))
                    .unwrap();

                if local_fqeis.contains(&target_fqei) {
                    self.processor.route_local(&target_fqei, kdu);
                } else {
                    // This logic is for when we have a real remote target.
                    // For now, we'll hardcode the server address.
                    let remote_addr = "127.0.0.1:8082";
                    println!(
                        "[Orchestrator] KDU is for a remote entity. Sending to {}.",
                        remote_addr
                    );
                    if let Ok(mut stream) =
                        crate::runtime::network::NodeClient::connect(remote_addr)
                    {
                        crate::runtime::network::NodeClient::send_kdu(&mut stream, &kdu).unwrap();
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    pub fn shutdown(&mut self) {
        println!("[Orchestrator] Shutting down...");
        self.persistence_tx
            .send(PersistenceCommand::Shutdown)
            .unwrap();
        if let Some(handle) = self.persistence_handle.take() {
            handle.join().expect("Persistence thread panicked");
        }
        if let Some(_handle) = self.network_handle.take() {
            // Network thread will exit when main exits.
        }
        println!("[Orchestrator] Shutdown complete.");
    }
}
