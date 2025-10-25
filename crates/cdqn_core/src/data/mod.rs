// BaDaaS License
// File Path: crates/cdqn_core/src/data/mod.rs
//
// Copyright (c) 2025 Christophe Duy Quang Nguyen
//
// This file is part of the CDQN (Causal Data Query Nodes) ecosystem.
//
// Licensed under the BaDaaS (Build and Develop as a Service) License.
// See LICENSE-BaDaaS.md in the project root for terms.
//
// AI-Mediated Access: All interactions with the CDQN network must be
// mediated by an AI entity (ProxyAgent) as per the BaDaaS license.

//! # Data Module
//!
//! Core data structures for the CDQN ecosystem, including the Card Data Unit (CDU),
//! Glyph (visual representation), and World (semantic scoping).
//!
//! Implements the immutable core + dynamic event stream model for scalability.

#![forbid(unsafe_code)]

use crate::identity::cuid::Cuid;
use crate::time::hlc::HybridLogicalClock;
use std::collections::HashMap;

// ──────────────────────────────────────────────────────────────
// Worlds
// ──────────────────────────────────────────────────────────────
pub mod world {
    use super::*;
    use crate::algebra::axiom::Axiom;
    use std::collections::HashMap;

    /// A World scopes the semantic meaning of CDUs, Primes, and Axioms.
    /// Each World has its own Origin Seeds Pool, Prime Elements, and weighted Axioms.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct World {
        /// Unique name of the World (e.g., "UserWorld", "RWorld").
        pub name: String,
        /// Weighted Axioms specific to this World (weight reflects confirmation by CDUs).
        pub axioms: HashMap<Axiom, f32>, // Weight: 0.0 (irrelevant) to 1.0 (absolute)
        /// Prime Elements anchored in this World.
        pub primes: Vec<i32>, // e.g., [-2, 2, -3, 3, ...]
    }

    impl World {
        /// Creates a new World with default empty axioms and primes.
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                axioms: HashMap::new(),
                primes: Vec::new(),
            }
        }

        /// Adds or updates an Axiom's weight based on causal confirmation.
        pub fn update_axiom_weight(&mut self, axiom: Axiom, delta: f32) {
            let entry = self.axioms.entry(axiom).or_insert(0.0);
            *entry = (*entry + delta).clamp(0.0, 1.0);
        }
    }
}

// ──────────────────────────────────────────────────────────────
// Glyphs
// ──────────────────────────────────────────────────────────────
pub mod glyph {
    use super::*;

    /// Universal visual data representation (the "art" of the card).
    #[derive(Debug, Clone)]
    pub struct Glyph {
        /// Raw image bytes (e.g., PNG/JPEG).
        pub data: Vec<u8>,
        /// MIME type hint (e.g., "image/png").
        pub mime_type: String,
    }

    impl Glyph {
        /// Creates a new Glyph from raw bytes and MIME type.
        pub fn new(data: Vec<u8>, mime_type: &str) -> Self {
            Self {
                data,
                mime_type: mime_type.to_string(),
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────
// Card Data Units (CDUs)
// ──────────────────────────────────────────────────────────────
pub mod card {
    use super::*;

    /// Persistence level for Events (optimization to avoid CDU bloat).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PersistenceLevel {
        /// Transient, in-memory only (for micro-calculations).
        Ephemeral,
        /// Persisted as a full CDU (for context shifts or feedback).
        Persistent,
    }

    /// An Event that evolves a CDU's state without mutating the core.
    #[derive(Debug, Clone)]
    pub struct Event {
        /// HLC timestamp of the event.
        pub timestamp: HybridLogicalClock,
        /// Description of the change (e.g., "MetadataUpdated").
        pub kind: String,
        /// Optional target CUID (for updates to existing CDUs).
        pub target: Option<Cuid>,
        /// Persistence level.
        pub persistence: PersistenceLevel,
        /// Arbitrary payload (e.g., new coordinates or worlds).
        pub payload: HashMap<String, String>,
    }

    impl Event {
        /// Creates a new Event with the given parameters.
        pub fn new(
            timestamp: HybridLogicalClock,
            kind: &str,
            target: Option<Cuid>,
            persistence: PersistenceLevel,
            payload: HashMap<String, String>,
        ) -> Self {
            Self {
                timestamp,
                kind: kind.to_string(),
                target,
                persistence,
                payload,
            }
        }
    }

    /// The immutable core of a Card Data Unit (CDU).
    #[derive(Debug, Clone)]
    pub struct CardDataUnit {
        /// Unique, immutable identifier.
        pub cuid: Cuid,
        /// Immutable visual representation.
        pub glyph: glyph::Glyph,
        /// Initial machine-readable payload (UTF-8 bytes).
        pub initial_oracle_text: Vec<u8>,
        /// Causal links to other CDUs.
        pub causal_links: Vec<Cuid>,
    }

    impl CardDataUnit {
        /// Creates a new immutable CDU core.
        pub fn new(
            cuid: Cuid,
            glyph: glyph::Glyph,
            initial_oracle_text: Vec<u8>,
            causal_links: Vec<Cuid>,
        ) -> Self {
            Self {
                cuid,
                glyph,
                initial_oracle_text,
                causal_links,
            }
        }
    }

    /// Represents the dynamic state of a CDU, computed from its event stream.
    #[derive(Debug, Clone, Default)]
    pub struct CduState {
        /// Current Worlds this CDU belongs to.
        pub worlds: Vec<world::World>,
        /// Per-World coordinates (evolvable).
        pub coord_map: HashMap<String, f32>, // Key: World name, Value: Coordinate
        /// Event history (full audit trail).
        pub events: Vec<Event>,
    }

    impl CduState {
        /// Applies an event to compute a new state (pure function, no mutation).
        pub fn apply_event(&self, event: Event) -> Self {
            let mut new_state = self.clone();
            new_state.events.push(event.clone());

            // Example: Handle "MetadataUpdated" by updating coord_map
            if event.kind == "MetadataUpdated" {
                if let Some(coord_str) = event.payload.get("new_coord") {
                    if let Ok(coord) = coord_str.parse::<f32>() {
                        if let Some(world) = event.payload.get("world") {
                            new_state.coord_map.insert(world.clone(), coord);
                        }
                    }
                }
            }

            new_state
        }

        /// Determines if a new CDU should be persisted based on events.
        pub fn should_persist(&self) -> bool {
            self.events.iter().any(|e| e.persistence == PersistenceLevel::Persistent)
        }
    }
}

// ──────────────────────────────────────────────────────────────
// Public Re-exports
// ──────────────────────────────────────────────────────────────
pub use card::{CardDataUnit, CduState, Event, PersistenceLevel};
pub use glyph::Glyph;
pub use world::World;
