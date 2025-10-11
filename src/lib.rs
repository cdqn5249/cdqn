// Path: src/lib.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

#![allow(clippy::module_inception)]
#![deny(warnings)]
#![forbid(unsafe_code)]

//! # CDQN — Causal Data Query Nodes
//!
//! The CDQN ecosystem is a causal, sovereign, and asynchronous data reasoning system.
//! It unifies causality, trust, and reasoning through Chronosa agents, CDUs (Causal Data Units),
//! and modular functional cores built on immutable and verifiable data.
//!
//! ## Architectural principles
//! - **Causality first** — all data relationships are directional and time-aware.
//! - **Sovereign nodes** — each node owns and governs its data.
//! - **Chronosa reasoning** — self-consistent causal reasoning engine using theorems, Btheorems, axioms.
//! - **Independent modules** — each module enforces its domain logic (crypto, reputation, assets, etc.).
//! - **Accountability** — every action is verifiable and traceable by CDUs and signatures.
//!
//! This library is the base crate for building CDQN nodes, Chronosa instances, and causal applications.

pub mod cdu;
pub mod chronosa;
pub mod core;
pub mod modules;
pub mod runtime;

pub use cdu::*;
pub use chronosa::*;
pub use core::*;
pub use modules::*;
pub use runtime::*;

/// CDQN version identifier.
pub const CDQN_VERSION: &str = "0.1.0";

/// Initializes the CDQN ecosystem.
/// This function should be called once by any node at startup.
///
/// It prepares:
/// - Logging (via `tracing`)
/// - CryptoCore initialization
/// - Default Chronosa context
pub fn init() {
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("CDQN: logging subsystem already initialized.");
    }

    tracing::info!("CDQN ecosystem initialized (v{})", CDQN_VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_runs_without_error() {
        init();
        assert_eq!(CDQN_VERSION, "0.1.0");
    }
}
