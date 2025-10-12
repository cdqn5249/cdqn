// Path: src/chronosa.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)
//
//! Chronosa — Autonomous Causal Reasoning Agent
//! -------------------------------------------------------------
//! Each Chronosa instance governs one node in the CDQN ecosystem.
//! It acts as the reasoning core, coordinating between modules,
//! managing CDUs, and applying axioms, theorems, and feedback causality.

use crate::{
    cdu::Cdu,
    modules::ModulesRegistry,
    runtime::{spawn_signal_watch, Runtime, RuntimeSignal},
};
use futures_lite::future;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::{debug, error, info};

/// Represents a single Chronosa instance configuration.
#[derive(Debug, Clone)]
pub struct ChronosaConfig {
    pub node_id: String,
    pub max_queue: usize,
}

/// Core Chronosa agent managing causal reasoning and coordination.
#[derive(Clone)]
pub struct Chronosa {
    config: ChronosaConfig,
    runtime: Arc<Runtime>,
    queue: Arc<Mutex<VecDeque<Cdu>>>,
}

impl Chronosa {
    /// Create a new Chronosa instance with its runtime.
    pub fn new(config: ChronosaConfig, runtime: Arc<Runtime>) -> Self {
        Self {
            config,
            runtime,
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Submit a CDU for reasoning and propagation.
    pub fn submit_cdu(&self, cdu: Cdu) {
        let mut q = self.queue.lock().unwrap();
        if q.len() >= self.config.max_queue {
            error!(
                "Chronosa {} queue full; dropping CDU {}",
                self.config.node_id, cdu.id
            );
            return;
        }
        q.push_back(cdu);
    }

    /// Start all Chronosa async workers and signal watchers.
    pub fn start(&self) {
        let this = self.clone();
        spawn_signal_watch(&this.runtime, move |signal| match signal {
            RuntimeSignal::Stop => {
                info!("Chronosa {} stopping...", this.config.node_id);
            }
            RuntimeSignal::Reload => {
                info!("Chronosa {} reloading configuration.", this.config.node_id);
            }
            RuntimeSignal::Custom(msg) => {
                debug!("Chronosa {} received signal: {}", this.config.node_id, msg);
            }
        });

        self.start_workers();
    }

    /// Internal: start reasoning workers (mock parallel processes).
    fn start_workers(&self) {
        let node_id = self.config.node_id.clone();
        let runtime = self.runtime.clone();
        let queue = self.queue.clone();

        runtime.spawn(async move {
            loop {
                let maybe_cdu = {
                    let mut q = queue.lock().unwrap();
                    q.pop_front()
                };

                if let Some(cdu) = maybe_cdu {
                    debug!(
                        "Chronosa {} processing CDU {} from {}",
                        node_id, cdu.id, cdu.author
                    );
                    // Simulated processing delay without async_std
                    let start = std::time::Instant::now();
                    while start.elapsed() < Duration::from_millis(100) {
                        future::yield_now().await;
                    }
                } else {
                    // Yield when idle
                    future::yield_now().await;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cdu::Cdu;

    #[test]
    fn chronosa_can_process_cdus() {
        crate::init();

        let runtime = Arc::new(Runtime::new());
        let config = ChronosaConfig {
            node_id: "chronosa-test".to_string(),
            max_queue: 4,
        };
        let chronosa = Chronosa::new(config, runtime.clone());

        chronosa.start();

        // Submit a CDU to be processed
        let cdu = Cdu::new("cdu1", "alice", "payload", 12345);
        chronosa.submit_cdu(cdu);

        runtime.run_for(Duration::from_millis(150));
    }
}
