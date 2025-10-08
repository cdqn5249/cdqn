// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/mod.rs

//! Defines the concrete data structures for all CDU payload types.

// 1. Declare the sub-modules.
pub mod axiom;
pub mod causal_mode;
pub mod constraint;
pub mod theorem;

// 2. Re-export the structs for easy access.
pub use axiom::Axiom;
pub use causal_mode::CausalMode;
pub use constraint::Constraint;
pub use theorem::Theorem;

// 3. All serialization helpers have been removed.
// This logic is now correctly and centrally handled by the traits in `codec.rs`.```
