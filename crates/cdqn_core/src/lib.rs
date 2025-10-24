// BaDaaS License
// File Path: crates/cdqn_core/src/lib.rs

//! # CDQN Core Crate
//!
//! `cdqn_core` provides the fundamental data structures, traits, and constants
//! for the entire Causal Data Query Nodes (CDQN) ecosystem. It serves as the
//! foundational layer upon which all other reasoning and actor components are built.
//!
//! This crate adheres to a strict "standard Rust only" policy, with the sole
//! exception of audited cryptography primitives, which will be introduced later
//! under specific feature flags. This ensures maximum portability, security, and
//! performance.
//!
//! ## Core Concepts
//!
//! - **`HybridLogicalClock` (HLC):** A distributed timestamping mechanism that
//!   combines physical and logical time to guarantee causal ordering.
//! - **`Cuid` (Chaos Unique ID):** An immutable, content-addressed identifier
//!   composed of an HLC and a content hash.
//! - **`Cdu` (Chaos Data Unit):** The atomic unit of information in the ecosystem,
//!   analogous to a "magic card" with art, text, and context.
//! - **`Glyph`:** A universal, modality-agnostic visual representation of data.
//! - **Prime Element Algebra:** The foundational types for symbolic reasoning.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    unreachable_pub,
    rustdoc::broken_intra_doc_links
)]

// --- Module Declarations ---
// This section declares the module hierarchy of the crate.
// The compiler will expect to find corresponding files, e.g., `time/mod.rs` or `time.rs`.

/// Foundational types for symbolic reasoning (Axioms, Prime Elements).
pub mod algebra;
/// Core data structures (CDU, Glyph, World).
pub mod data;
/// Crate-specific error types.
pub mod error;
/// Primitives for the event-driven actor model.
pub mod event;
/// Identity and content-addressing types (CUID, ContentHash).
pub mod identity;
/// Custom serialization and deserialization logic (standard library only).
pub mod serde;
/// Distributed time and causality primitives (HLC).
pub mod time;
/// Core traits defining behavior for CDQN types.
pub mod traits;


// --- Public API Re-exports ---
// This section exposes the most important types at the top level of the crate
// for cleaner and more convenient usage by other crates in the workspace.

pub use self::algebra::axiom::Axiom;
pub use self::algebra::prime::{Charge, PrimeElement};

pub use self::data::cdu::Cdu;
pub use self::data::glyph::Glyph;
pub use self::data::world::World;

pub use self::error::CoreError;

pub use self::event::Event;

pub use self::identity::cuid::Cuid;
pub use self::identity::hash::ContentHash;

pub use self::time::hlc::HybridLogicalClock;

pub use self::traits::{Causal, Identifiable, Verifiable};
