// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/reasoning/mod.rs

//! The reasoning module for Chronosa.
//!
//! This module contains the core components of Chronosa's reasoning model,
//! including prime elements, semi-axioms, and the reasoning projector.

// New modules for the refactored, modular design.
pub mod knowledge_base;
pub mod strategy;

// Existing and refactored components.
pub mod prime_element;
pub mod reasoning_projector;
pub mod semi_axiom;

// Re-export for easier access from other parts of the application.
pub use prime_element::PrimeElement;
pub use reasoning_projector::ReasoningProjector;
pub use semi_axiom::SemiAxiom;
