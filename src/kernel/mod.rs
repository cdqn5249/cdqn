// src/kernel/mod.rs

use serde::{Deserialize, Serialize};

// FQEI (Fully Qualified Entity Identifier) - A unique, verifiable name for any entity.
// For now, a simple String. Will become more complex later.
pub type FQEI = String;

// A placeholder for the 42-dimensional vector from the Unisphere.
// We use a fixed-size array of 16-bit unsigned integers.
pub type UnisphereCoordinates = [u16; 42];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_id: String,      // e.g., "BaDaaS-1.1.0"
    pub licensor_fqei: FQEI,     // The owner of the content
    pub custom_terms_hash: Option<String>, // Optional hash for custom licenses
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_hash: String,       // Hash of this metadata block
    pub unisphere_coordinates: UnisphereCoordinates, // The 42D vector
    pub license: License,           // The license terms
    pub causal_link: Option<String>, // Optional ID of a preceding KDU
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KDU {
    // --- Universal Header ---
    pub kdu_spec_version: String,
    pub kdu_id: String,            // The HLC timestamp and unique ID
    pub content_hash: String,      // Hash of the payload and metadata
    pub originator_fqei: FQEI,
    pub originator_signature: Vec<u8>, // The forward-secret signature
    pub timestamp_utc: String,
    pub kdu_type: String,          // e.g., "Generic", "Workflow"
    
    // --- Core Blocks ---
    pub metadata: Metadata,
    // The content-agnostic payload, represented as a flexible JSON value for now.
    pub data_payload: serde_json::Value,
}
