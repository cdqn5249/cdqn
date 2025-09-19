// src/kernel/factory.rs

use super::crypto::{CryptoCore, Keypair};
use super::hlc::HLC;
use super::{License, Metadata, FQEI, KDU};

pub struct KDUFactory {
    crypto_core: CryptoCore,
    hlc: HLC,
}

// Implement the Default trait as suggested by clippy.
impl Default for KDUFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl KDUFactory {
    pub fn new() -> Self {
        KDUFactory {
            crypto_core: CryptoCore::new(),
            hlc: HLC::new(),
        }
    }

    pub fn crypto_core(&self) -> &CryptoCore {
        &self.crypto_core
    }

    pub fn create_kdu(
        &self,
        originator_keypair: &Keypair,
        originator_fqei: FQEI,
        kdu_type: String,
        data_payload: serde_json::Value,
    ) -> KDU {
        let (kdu_id, timestamp_utc) = self.hlc.now();

        let license = License {
            license_id: "BaDaaS-1.1.0".to_string(),
            licensor_fqei: originator_fqei.clone(),
            custom_terms_hash: None,
        };

        let mut metadata = Metadata {
            metadata_hash: String::new(),
            unisphere_coordinates: vec![0; 42],
            license,
            causal_link: None,
        };

        let metadata_hash = CryptoCore::hash_content(&metadata);
        metadata.metadata_hash = hex::encode(&metadata_hash);

        let mut kdu = KDU {
            kdu_spec_version: "2.1.0".to_string(),
            kdu_id,
            content_hash: String::new(),
            originator_fqei,
            originator_signature: Vec::new(),
            timestamp_utc,
            kdu_type,
            metadata,
            data_payload,
        };

        let content_to_hash = (&kdu.metadata, &kdu.data_payload);
        let content_hash = CryptoCore::hash_content(&content_to_hash);
        kdu.content_hash = hex::encode(&content_hash);

        // Sign using the secret key from our keypair
        let signature = self
            .crypto_core
            .sign(&content_hash, &originator_keypair.secret);
        kdu.originator_signature = signature.to_bytes().to_vec();

        kdu
    }
}
