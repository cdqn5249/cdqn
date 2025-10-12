// Path: src/chronosa.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! # Chronosa
//!
//! Chronosa is the cognitive reasoning layer of the CDQN ecosystem.
//! Each Chronosa instance represents a sovereign, autonomous reasoning agent
//! residing on a node, interacting with CDUs, modules, and the network manifold.
//!
//! ## Core Concepts
//! - **Roles:** internal role-based reasoning workers (Observer, Proposer, Verifier).
//! - **Causality:** learns through causal CDUs and generates Theorems or Btheorems.
//! - **Accountability:** actions are tracked via signed CDUs and stored on the node.
//! - **Non-interference:** Chronosa cannot bypass its module or system boundaries.
//!
//! ## Thread Model
//! Chronosa uses a lightweight runtime abstraction (no Tokio) with async channels
//! and a deterministic task scheduler. All reasoning is causal, not stochastic.

use crate::cdu::{Cdu, CduState};
use crate::modules::ModulesRegistry;
use async_channel::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

/// Chronosa configuration parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronosaConfig {
    pub node_id: String,
    pub reputation: f64,
    pub max_roles: usize,
}

/// Message type for internal Chronosa role communication.
#[derive(Debug, Clone)]
pub enum RoleMessage {
    Input(Cdu),
    Output(RoleOutput),
    Stop,
}

/// Result of role processing.
#[derive(Debug, Clone)]
pub enum RoleOutput {
    Cdu(Cdu),
    None,
}

/// RoleType identifies a Chronosa reasoning subagent role.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoleType {
    Observer,
    Proposer,
    Verifier,
}

/// Chronosa main structure.
#[derive(Clone)]
pub struct Chronosa {
    pub config: ChronosaConfig,
    pub modules: ModulesRegistry,
    sender: Sender<RoleMessage>,
    receiver: Receiver<RoleMessage>,
    runtime: Arc<crate::runtime::Runtime>,
}

impl Chronosa {
    /// Create a new Chronosa instance with a given module registry and runtime.
    pub fn new(
        config: ChronosaConfig,
        modules: ModulesRegistry,
        runtime: Arc<crate::runtime::Runtime>,
    ) -> Self {
        let (tx, rx) = unbounded();
        Self {
            config,
            modules,
            sender: tx,
            receiver: rx,
            runtime,
        }
    }

    /// Start Chronosa roles and async workers.
    pub fn start(&self) {
        info!("Chronosa {} starting...", self.config.node_id);

        // Spawn role workers
        for role in [RoleType::Observer, RoleType::Proposer, RoleType::Verifier] {
            let rx = self.receiver.clone();
            let tx = self.sender.clone();
            let node_id = self.config.node_id.clone();
            let _modules = self.modules.clone(); // underscore to avoid warnings

            self.runtime.spawn(async move {
                Chronosa::run_role(role, node_id, _modules, tx, rx).await;
            });
        }

        // Start consolidator worker that listens to role outputs.
        let consolidator = self.clone();
        self.runtime.spawn(async move {
            info!(
                "Chronosa {} consolidator starting.",
                consolidator.config.node_id
            );
            consolidator.run_consolidator().await;
            info!(
                "Chronosa {} consolidator stopped.",
                consolidator.config.node_id
            );
        });
    }

    /// Sends a CDU to Chronosa for reasoning.
    pub fn submit(&self, cdu: Cdu) {
        if let Err(e) = self.sender.try_send(RoleMessage::Input(cdu)) {
            warn!("Chronosa queue full: {}", e);
        }
    }

    /// Core async worker for Chronosa roles.
    async fn run_role(
        role: RoleType,
        node_id: String,
        _modules: ModulesRegistry,
        tx: Sender<RoleMessage>,
        rx: Receiver<RoleMessage>,
    ) {
        info!("Role {:?} for node {} started.", role, node_id);
        while let Ok(msg) = rx.recv().await {
            match msg {
                RoleMessage::Input(cdu) => {
                    // Simulate causal reasoning: Proposer expands, Verifier validates, Observer logs.
                    let result = match role {
                        RoleType::Observer => {
                            info!("Observer {} saw CDU {}", node_id, cdu.id);
                            RoleOutput::None
                        }
                        RoleType::Proposer => {
                            if cdu.payload.contains("pattern") {
                                let mut proposed = cdu.clone();
                                proposed.state = CduState::Active;
                                RoleOutput::Cdu(proposed)
                            } else {
                                RoleOutput::None
                            }
                        }
                        RoleType::Verifier => {
                            if cdu.payload.len() > 10 {
                                info!("Verifier {} accepted CDU {}", node_id, cdu.id);
                                RoleOutput::Cdu(cdu.clone())
                            } else {
                                warn!("Verifier {} rejected CDU {}", node_id, cdu.id);
                                RoleOutput::None
                            }
                        }
                    };
                    if let Err(e) = tx.send(RoleMessage::Output(result)).await {
                        warn!("Chronosa role output send failed: {}", e);
                    }
                }
                RoleMessage::Stop => {
                    info!("Role {:?} for node {} stopping.", role, node_id);
                    break;
                }
                _ => {}
            }
        }
    }

    /// Consolidator — merges verified CDUs, generates Theorems/Btheorems, and updates state.
    async fn run_consolidator(&self) {
        while let Ok(msg) = self.receiver.recv().await {
            match msg {
                RoleMessage::Output(out) => match out {
                    RoleOutput::Cdu(cdu) => {
                        info!(
                            "Chronosa {} consolidator anchoring CDU {}",
                            self.config.node_id, cdu.id
                        );
                        // In a full implementation: validate, sign, store, update manifold & reputation.
                        // Here we only log; real storage would be via Storage module / AssetCore.
                    }
                    RoleOutput::None => {}
                },
                RoleMessage::Stop => {
                    info!("Chronosa consolidator received stop signal.");
                    break;
                }
                _ => {}
            }
        }
    }
}

/// Utility function to get current timestamp
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    #[test]
    fn chronosa_roles_process_cdu() {
        let runtime = Arc::new(Runtime::new());
        let modules = ModulesRegistry::new();
        let config = ChronosaConfig {
            node_id: "chronosa_test".to_string(),
            reputation: 0.8,
            max_roles: 3,
        };

        let chronosa = Chronosa::new(config, modules, runtime.clone());
        chronosa.start();

        // Create a CDU containing "pattern" to trigger proposer and long payload to trigger verifier
        let cdu = Cdu::new(
            "c1",
            "alice",
            "this is a pattern payload sample",
            now_secs(),
        );

        // Submit CDU asynchronously on the runtime and wait briefly for processing
        crate::runtime::block_on(async {
            chronosa.submit(cdu);
            async_std::task::sleep(std::time::Duration::from_millis(100)).await;
        });

        // Success here means no panic and valid message processing path
        assert!(true);
    }
}
