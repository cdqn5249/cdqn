//! # CDQN Types Crate
//!
//! This crate provides the foundational data structures for the `cdqn` ecosystem,
//! as defined in the formal specifications. It includes the definitions for the
//! Context Data Unit (`cdu`), `Entity`, Hybrid Logical Clock (`HLC`), and
//! Content Identifier (`CID`).
//!
//! The core principle is to create strongly-typed, serializable, and verifiable
//! data structures that serve as the "atoms" of the entire system.

// Declare the modules that make up this crate.
pub mod cdu;
pub mod entity;
pub mod error;
pub mod hlc;
pub mod cid;

// Re-export the most important types for easy access by other crates.
pub use cdu::{
    Cdu, CduType, IntrinsicMetadata, CreatorInfo, SourceProvenance,
    LicenseType, StandardLicense, GenerationInfo, ConfidenceMetric,
};
pub use entity::{
    EntityId, EntityForm, ExecutionContext, NodeType, NodeProfile,
};
pub use error::Error;
pub use hlc::Hlc;
pub use cid::Cid;
