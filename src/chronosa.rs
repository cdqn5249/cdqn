// Path: src/chronosa.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! Chronosa — local reasoning assembly
//!
//! Chronosa is the local reasoning agent (per-node). It is an assembly of
//! lightweight role micro-engines (Proposer, Verifier, BackwardValidator,
//! Policy, Consolidator, ReputationEngine). Chronosa receives CDUs and
//! orchestrates the role-based processing pipeline in an asynchronous,
//! non-blocking way. Most logic is implemented as small pure functions that
//! return new CDUs or status messages; the runtime schedules role tasks.
//!
//! This file intentionally keeps implementations minimal and deterministic
//! so it can serve as a solid scaffold for future rigorous theorem/Btheorem
//! implementations.

use crate::cdu::Cdu;
use crate::runtime::CdqnRuntime;
use async_channel::{unbounded, Receiver, Sender};
use futures_lite::future::yield_now;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info};

/// A lightweight wrapper for role-generated outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoleOutput {
    Cdu(Cdu),
    Info(String),
    None,
}

/// Role identifiers inside a Chronosa instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Proposer,
    Verifier,
    BackwardValidator,
    Policy,
    Consolidator,
    ReputationEngine,
}

/// Messages that Chronosa accepts on its public input channel.
#[derive(Debug, Clone)]
pub enum ChronosaMsg {
    IngestCdu(Cdu),
    Stop,
}

/// Chronosa configuration (immutable)
#[derive(Debug, Clone)]
pub struct ChronosaConfig {
    pub node_id: String,
}

impl Default for ChronosaConfig {
    fn default() -> Self {
        Self {
            node_id: "local-chronosa".to_string(),
        }
    }
}

/// The Chronosa instance. It uses an internal role channel to route role tasks
/// and a public input channel to accept external CDUs.
pub struct Chronosa {
    pub config: ChronosaConfig,
    // Public input channel (external callers push CDUs here).
    pub in_tx: Sender<ChronosaMsg>,
    pub in_rx: Receiver<ChronosaMsg>,
    // Internal role channel (roles and consolidator exchange outputs).
    pub role_tx: Sender<RoleOutput>,
    pub role_rx: Receiver<RoleOutput>,
    // Runtime handle to schedule tasks
    pub runtime: Arc<CdqnRuntime>,
}

impl Chronosa {
    /// Create a new Chronosa bound to a runtime.
    pub fn new(config: ChronosaConfig, runtime: Arc<CdqnRuntime>) -> Self {
        let (in_tx, in_rx) = unbounded::<ChronosaMsg>();
        let (role_tx, role_rx) = unbounded::<RoleOutput>();

        Self {
            config,
            in_tx,
            in_rx,
            role_tx,
            role_rx,
            runtime,
        }
    }

    /// Start Chronosa processing loop on the configured runtime.
    /// This will spawn background tasks for role orchestration.
    pub fn start(self: Arc<Self>) {
        let me = self.clone();
        // Main ingest loop
        self.runtime.spawn(async move {
            info!("Chronosa {} ingest loop starting.", me.config.node_id);
            me.run_ingest_loop().await;
            info!("Chronosa {} ingest loop stopped.", me.config.node_id);
        });

        // Start a consolidator worker that listens to role outputs.
        let consolidator = self.clone();
        self.runtime.spawn(async move {
            info!("Chronosa {} consolidator starting.", consolidator.config.node_id);
            consolidator.run_consolidator().await;
            info!("Chronosa {} consolidator stopped.", consolidator.config.node_id);
        });
    }

    /// Submit a CDU to Chronosa asynchronously.
    pub async fn submit(&self, cdu: Cdu) -> Result<(), async_channel::SendError<ChronosaMsg>> {
        self.in_tx.send(ChronosaMsg::IngestCdu(cdu)).await
    }

    /// Send a stop signal to Chronosa.
    pub async fn stop(&self) -> Result<(), async_channel::SendError<ChronosaMsg>> {
        self.in_tx.send(ChronosaMsg::Stop).await
    }

    /// The ingest loop: receives CDUs and dispatches them to roles.
    async fn run_ingest_loop(&self) {
        while let Ok(msg) = self.in_rx.recv().await {
            match msg {
                ChronosaMsg::IngestCdu(cdu) => {
                    debug!("Chronosa {} received CDU {}", self.config.node_id, cdu.id);
                    // For each incoming CDU, spawn role tasks (Proposer, Verifier, Policy)
                    let roles_sender = self.role_tx.clone();
                    let cdu_clone = cdu.clone();
                    let rt = self.runtime.clone();
                    rt.spawn(async move {
                        // Proposer role proposes candidate theorem (pure function)
                        let proposal = proposer_role(&cdu_clone);
                        if let Some(p) = proposal {
                            let _ = roles_sender.send(RoleOutput::Cdu(p)).await;
                        }

                        // Verifier role checks consistency (pure)
                        let verification = verifier_role(&cdu_clone);
                        if let Some(v) = verification {
                            let _ = roles_sender.send(RoleOutput::Cdu(v)).await;
                        }

                        // Policy role can produce refusals or info
                        let policy_out = policy_role(&cdu_clone);
                        match policy_out {
                            Some(RoleOutput::Info(s)) => {
                                let _ = roles_sender.send(RoleOutput::Info(s)).await;
                            }
                            Some(RoleOutput::Cdu(d)) => {
                                let _ = roles_sender.send(RoleOutput::Cdu(d)).await;
                            }
                            _ => {}
                        }
                    });
                }
                ChronosaMsg::Stop => {
                    debug!("Chronosa {} received Stop", self.config.node_id);
                    break;
                }
            }
            // Yield to let spawned tasks run
            yield_now().await;
        }
    }

    /// Consolidator listens to role outputs and anchors accepted CDUs.
    async fn run_consolidator(&self) {
        while let Ok(out) = self.role_rx.recv().await {
            match out {
                RoleOutput::Cdu(cdu) => {
                    // Simplified consolidation: mark CDU Active and log.
                    info!("Chronosa {} consolidator anchoring CDU {}", self.config.node_id, cdu.id);
                    // In a full implementation: validate, sign, store, update manifold & reputation.
                    // Here we only log; real storage would be via Storage module / AssetCore.
                }
                RoleOutput::Info(s) => {
                    info!("Chronosa {} role info: {}", self.config.node_id, s);
                }
                RoleOutput::None => {}
            }
            yield_now().await;
        }
    }
}

/// -----------------
/// Role implementations (pure/deterministic helpers)
/// -----------------

/// Proposer role: derive a lightweight "theorem" CDU from input CDU.
/// Pure function — no side effects; deterministic based on CDU content.
pub fn proposer_role(input: &Cdu) -> Option<Cdu> {
    // Very small heuristic: if payload contains the word "pattern", propose a theorem
    if input.payload.contains("pattern") {
        let ts = now_secs();
        let id = format!("prop:{}:{}", input.id, ts);
        let payload = format!("theorem: derived from {}", input.id);
        Some(Cdu::new(&id, "chronosa.proposer", &payload, ts))
    } else {
        None
    }
}

/// Verifier role: produce a verification CDU if simple checks pass.
/// Pure and cheap.
pub fn verifier_role(input: &Cdu) -> Option<Cdu> {
    // Example check: payload length threshold
    if input.payload.len() > 10 {
        let ts = now_secs();
        let id = format!("verif:{}:{}", input.id, ts);
        let payload = format!("verification: payload_len={}", input.payload.len());
        Some(Cdu::new(&id, "chronosa.verifier", &payload, ts))
    } else {
        None
    }
}

/// Policy role: checks axioms/semi-axioms and may emit refusal info.
/// Returns an optional RoleOutput (Info or CDU)
pub fn policy_role(input: &Cdu) -> Option<RoleOutput> {
    // Example policy: refuse CDUs that include banned token "malicious"
    if input.payload.contains("malicious") {
        Some(RoleOutput::Info(format!(
            "refusal: CDU {} contains banned token",
            input.id
        )))
    } else {
        None
    }
}

/// Backward validator role stub — attempts to produce Btheorem CDU.
/// In real system this would attempt a backward trace to axioms.
pub fn backward_validator_role(_input: &Cdu) -> Option<Cdu> {
    // Stub: return None (rare)
    None
}

/// Simple helper: current epoch seconds (deterministic enough for IDs)
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime;
    use futures_lite::future::yield_now;
    use std::sync::Arc;

    #[test]
    fn chronosa_roundtrip_flow() {
        // Build runtime and chronosa
        let rt = Arc::new(runtime::CdqnRuntime::new());
        let cfg = ChronosaConfig {
            node_id: "test-node".to_string(),
        };
        let chronosa = Arc::new(Chronosa::new(cfg, rt.clone()));
        chronosa.start();

        // Create a CDU containing "pattern" to trigger proposer and long payload to trigger verifier
        let cdu = Cdu::new("c1", "alice", "this is a pattern payload sample", now_secs());

        // Submit CDU asynchronously on the runtime and wait briefly for processing
        crate::runtime::block_on(async {
            chronosa.submit(cdu).await.unwrap();
            // allow tasks to run
            yield_now().await;
            // stop chronosa
            chronosa.stop().await.unwrap();
            // ensure some time for background consolidator to process Stop
            yield_now().await;
        });
    }
}
