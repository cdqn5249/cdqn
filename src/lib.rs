// Source code under BaDaaS license, vibe coding engine: Gemini Flash-Lite Latest, Google
// File path: src/lib.rs

// Re-export public items from sub-modules
pub mod cdu;
pub mod hashing;
pub mod axioms;

// Re-export core types for external use
pub use cdu::{Cdu, ImmutablePayload, Hlc, GENESIS_HASH, CduType};
pub use hashing::Hash;

// Re-export the main creation function
pub use cdu::create_cdu;

// Re-export the axiom check functions
pub use axioms::{axiom_content_identity, axiom_type_check}; // <-- Both are now exported

// A simple entry point for testing/initialization
pub fn initialize_cdqn() {
    println!("CDQN Initialized: Modular structure loaded with typed CDUs.");
}
