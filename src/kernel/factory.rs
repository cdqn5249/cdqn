// src/kernel/factory.rs

use crate::kernel::crypto::{CryptoCore, Keypair};
use crate::kernel::{License, Metadata, FQEI, KDU};
use std::time::{SystemTime, UNIX_EPOCH};

/// The KDUFactory is responsible for creating valid, signed KDUs.
/// It now directly owns the state of the Hybrid Logical Clock.
pub struct KDUFactory {
    crypto_core: CryptoCore,
    // HLC state is now part of the factory itself.
    last_time_micros: u128,
    counter: u16,
}

impl Default for KDUFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl KDUFactory {
    /// Creates a new factory, initializing the HLC state.
    pub fn new() -> Self {
        KDUFactory {
            crypto_core: CryptoCore::new(),
            last_time_micros: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_micros(),
            counter: 0,
        }
    }

    pub fn crypto_core(&self) -> &CryptoCore {
        &self.crypto_core
    }

    /// Creates a new, immutable KDU using a pure functional approach.
    pub fn create_kdu(
        &mut self, // <-- This is now mutable
        originator_keypair: &Keypair,
        originator_fqei: FQEI,
        kdu_type: String,
        data_payload: &[u8],
    ) -> KDU {
        // --- 1. Generate all unique and context-dependent components first ---
        let current_time_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();

        if current_time_micros > self.last_time_micros {
            self.last_time_micros = current_time_micros;
            self.counter = 0;
        } else {
            // This correctly handles events faster than the clock's resolution.
            self.counter += 1;
        }

        let kdu_id = format!("{}-{:04x}", self.last_time_micros, self.counter);
        let timestamp_utc = format!("{}", self.last_time_micros);

        // --- The rest of the KDU creation process ---
        let license = License {
            license_id: "BaDaaS-1.1.0".to_string(),
            licensor_fqei: originator_fqei.clone(),
            custom_terms_hash: None,
        };

        let metadata = Metadata {
            // The metadata hash is now a hash of the license, not itself.
            metadata_hash: hex::encode(CryptoCore::hash_content(&license)),
            unisphere_coordinates: vec![0; 42],
            license,
            causal_link: None,
        };

        // --- 2. Create the final content_hash from all components ---
        let content_to_hash = (&kdu_id, &timestamp_utc, &metadata, &data_payload);
        let content_hash_bytes = CryptoCore::hash_content(&content_to_hash);
        let content_hash = hex::encode(&content_hash_bytes);

        // --- 3. Create the signature from the final content_hash ---
        let signature = self
            .crypto_core
            .sign(&content_hash_bytes, &originator_keypair.secret);

        // --- 4. Construct the final, immutable KDU in a single step ---
        KDU {
            kdu_spec_version: "2.1.0".to_string(),
            kdu_id,
            content_hash,
            originator_fqei,
            originator_signature: signature.to_bytes().to_vec(),
            timestamp_utc,
            kdu_type,
            metadata,
            data_payload: data_payload.to_vec(),
        }
    }
}
