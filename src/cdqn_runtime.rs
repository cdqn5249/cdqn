// Path: src/cdqn_runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

use serde::Deserialize;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Lightweight genesis prime element representation.
/// This is intentionally small so it can be used as a compile-time
/// friendly substitute while the rest of the engine is implemented.
#[derive(Clone, Debug, Deserialize)]
pub struct GenesisPrimeElement {
    pub id: String,
    pub world: String,
    pub representation: Vec<f64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub symmetric_pair: Option<String>,
}

/// Lightweight genesis semi-axiom representation.
#[derive(Clone, Debug, Deserialize)]
pub struct GenesisSemiAxiom {
    pub id: String,
    pub world: String,
    #[serde(default)]
    pub premises: Vec<String>,
    #[serde(default)]
    pub conclusion: Option<String>,
}

/// Small runtime configuration parsed from a file or defaults.
/// Intentionally minimal and immutable.
#[derive(Clone, Debug, Deserialize)]
pub struct RuntimeConfig {
    pub node_id: String,
    #[serde(default)]
    pub genesis_primes: Vec<GenesisPrimeElement>,
    #[serde(default)]
    pub genesis_semi_axioms: Vec<GenesisSemiAxiom>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            node_id: "local-node".to_string(),
            genesis_primes: vec![],
            genesis_semi_axioms: vec![],
        }
    }
}

/// Run the CDQN runtime.
///
/// This function is intentionally synchronous and conservative:
/// - it performs minimal initialization
/// - prints a welcome banner and runtime info to stdout
/// - returns immediately so `main()` can control process lifetime or tests can run
///
/// Replace this minimal implementation with the async executor of your choice
/// once the rest of the codebase has been cleaned and compiled.
pub fn run() {
    println!("CDQN runtime (conservative stub) starting.");
    if let Ok(start) = SystemTime::now().duration_since(UNIX_EPOCH) {
        println!("Start epoch (secs): {}", start.as_secs());
    }

    // Try to load optional config file "cdqn_runtime.toml" in current dir.
    // If not present, continue with defaults.
    let cfg = load_runtime_config().unwrap_or_default();

    println!("Node id: {}", cfg.node_id);
    println!(
        "Genesis primes count: {}, semi-axioms: {}",
        cfg.genesis_primes.len(),
        cfg.genesis_semi_axioms.len()
    );

    println!("CDQN runtime stub ready. (No active tasks in stub mode.)");
}

/// Attempt to load a `RuntimeConfig` from `cdqn_runtime.toml` alongside the binary.
/// Errors are logged; `None` indicates no valid config found.
fn load_runtime_config() -> Option<RuntimeConfig> {
    let path = PathBuf::from("cdqn_runtime.toml");
    if !path.exists() {
        return None;
    }

    match std::fs::read_to_string(&path) {
        Ok(text) => {
            // Optional dependency: toml crate (add to Cargo.toml if not yet)
            match toml::from_str::<RuntimeConfig>(&text) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    eprintln!("Failed to parse cdqn_runtime.toml: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read cdqn_runtime.toml: {}", e);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_default_config() {
        let cfg = RuntimeConfig::default();
        assert_eq!(cfg.node_id, "local-node");
        assert!(cfg.genesis_primes.is_empty());
    }

    #[test]
    fn run_does_not_panic() {
        run(); // simple smoke test
    }
}
