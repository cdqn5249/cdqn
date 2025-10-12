// Path: src/modules.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! Modules Registry for CDQN Ecosystem
//!
//! This file defines the central **Modules** registry that provides
//! abstraction and discovery for all independent modules of the CDQN ecosystem.
//!
//! Each module (e.g., `CryptoCore`, `LicenseCore`, `AssetCore`, etc.)
//! is self-contained, independent, and pure in function logic. Chronosa
//! uses these modules via their declared interfaces, but cannot override
//! or alter their behavior directly. Modules never follow arbitrary scripts
//! — they enforce their declared roles within the cdqn network.
//!
//! This registry enables runtime module lookup, version control, and
//! safe concurrent access to module functions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::info;

/// Core module trait — every module implements this.
pub trait Module: Send + Sync {
    /// Unique module identifier (e.g., `"cryptocore"`, `"licensecore"`)
    fn id(&self) -> &str;

    /// Human-readable name or summary
    fn description(&self) -> &str;

    /// Returns the module version
    fn version(&self) -> &str;

    /// Executes a generic module call by name
    fn execute(&self, action: &str, payload: &str) -> Result<String, String>;
}

/// Basic registry structure that holds active modules
#[derive(Clone)]
pub struct ModulesRegistry {
    inner: Arc<RwLock<HashMap<String, Arc<dyn Module>>>>,
}

impl ModulesRegistry {
    /// Create a new, empty registry
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a module into the registry
    pub fn register(&self, module: Arc<dyn Module>) {
        let id = module.id().to_string();
        let mut guard = self.inner.write().unwrap();
        guard.insert(id.clone(), module.clone());
        info!("Module registered: {}", id);
    }

    /// Retrieve a module by its identifier
    pub fn get(&self, id: &str) -> Option<Arc<dyn Module>> {
        let guard = self.inner.read().unwrap();
        guard.get(id).cloned()
    }

    /// Execute a module action
    pub fn call(&self, module_id: &str, action: &str, payload: &str) -> Result<String, String> {
        if let Some(module) = self.get(module_id) {
            module.execute(action, payload)
        } else {
            Err(format!("Module {} not found", module_id))
        }
    }

    /// List all registered modules
    pub fn list(&self) -> Vec<String> {
        let guard = self.inner.read().unwrap();
        guard.keys().cloned().collect()
    }
}

/// Example module: CryptoCore
#[derive(Clone, Serialize, Deserialize)]
pub struct CryptoCore {
    version: String,
}

impl CryptoCore {
    pub fn new() -> Self {
        Self {
            version: "0.1.0".into(),
        }
    }
}

impl Module for CryptoCore {
    fn id(&self) -> &str {
        "cryptocore"
    }

    fn description(&self) -> &str {
        "Lightweight cryptographic operations core"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn execute(&self, action: &str, payload: &str) -> Result<String, String> {
        match action {
            "hash" => {
                use sha3::{Digest, Sha3_256};
                let mut hasher = Sha3_256::new();
                hasher.update(payload.as_bytes());
                let result = hasher.finalize();
                Ok(format!("{:x}", result))
            }
            _ => Err(format!("Unknown action '{}'", action)),
        }
    }
}

/// Example module: LicenseCore
#[derive(Clone, Serialize, Deserialize)]
pub struct LicenseCore {
    version: String,
}

impl LicenseCore {
    pub fn new() -> Self {
        Self {
            version: "0.1.0".into(),
        }
    }
}

impl Module for LicenseCore {
    fn id(&self) -> &str {
        "licensecore"
    }

    fn description(&self) -> &str {
        "Manages license templates and user-defined legal models"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn execute(&self, action: &str, _payload: &str) -> Result<String, String> {
        match action {
            "list_templates" => Ok(String::from("BaDaaS, MIT, Apache-2.0, GPL-3.0")),
            _ => Err(format!("Unknown action '{}'", action)),
        }
    }
}

/// Example module: AssetCore
#[derive(Clone, Serialize, Deserialize)]
pub struct AssetCore {
    version: String,
}

impl AssetCore {
    pub fn new() -> Self {
        Self {
            version: "0.1.0".into(),
        }
    }
}

impl Module for AssetCore {
    fn id(&self) -> &str {
        "assetcore"
    }

    fn description(&self) -> &str {
        "Manages user assets, ownership, and asset trade permission tokens"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn execute(&self, action: &str, payload: &str) -> Result<String, String> {
        match action {
            "mint_asset" => Ok(format!("Asset minted for payload: {}", payload)),
            "transfer_asset" => Ok(format!("Asset transfer executed: {}", payload)),
            _ => Err(format!("Unknown action '{}'", action)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modules_register_and_call() {
        let registry = ModulesRegistry::new();
        let crypto = Arc::new(CryptoCore::new());
        let license = Arc::new(LicenseCore::new());
        let asset = Arc::new(AssetCore::new());

        registry.register(crypto.clone());
        registry.register(license.clone());
        registry.register(asset.clone());

        // Verify listing
        let list = registry.list();
        assert!(list.contains(&"cryptocore".to_string()));
        assert!(list.contains(&"licensecore".to_string()));
        assert!(list.contains(&"assetcore".to_string()));

        // Test crypto hash call
        let hash = registry.call("cryptocore", "hash", "cdqn").unwrap();
        assert!(!hash.is_empty());

        // Test license listing
        let licenses = registry.call("licensecore", "list_templates", "").unwrap();
        assert!(licenses.contains("BaDaaS"));

        // Test asset mint
        let mint = registry.call("assetcore", "mint_asset", "new_artwork").unwrap();
        assert!(mint.contains("new_artwork"));
    }
}
