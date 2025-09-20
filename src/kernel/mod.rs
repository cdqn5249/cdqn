// src/kernel/mod.rs

pub mod crypto;
pub mod factory;
// pub mod hlc; // This line is now removed.

use serde::{Deserialize, Serialize};

pub type FQEI = String;
pub type UnisphereCoordinates = Vec<u16>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_id: String,
    pub licensor_fqei: FQEI,
    pub custom_terms_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_hash: String,
    pub unisphere_coordinates: UnisphereCoordinates,
    pub license: License,
    pub causal_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KDU {
    pub kdu_spec_version: String,
    pub kdu_id: String,
    pub content_hash: String,
    pub originator_fqei: FQEI,
    pub originator_signature: Vec<u8>,
    pub timestamp_utc: String,
    pub kdu_type: String,
    pub metadata: Metadata,
    // The payload is now a truly content-agnostic vector of bytes.
    pub data_payload: Vec<u8>,
}
