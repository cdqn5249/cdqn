// src/kernel/factory.rs

use super::{KDU, FQEI, License, Metadata};
use super::crypto::{CryptoCore, Keypair};
use super::hlc::HLC;

// The KDUFactory holds instances of the services it depends on.
pub struct KDUFactory {
    crypto_core: CryptoCore,
    hlc: HLC,
}

impl KDUFactory {
    // Creates a new factory.
    pub fn new() -> Self {
        KDUFactory {
            crypto_core: CryptoCore::new(),
            hlc: HLC::new(),
        }
    }
    
    // A helper function to get a reference to the internal CryptoCore.
    pub fn crypto_core(&self) -> &CryptoCore {
        &self.crypto_core
    }

    // The main function for creating a new KDU.
    pub fn create_kdu(
        &self,
        originator_keypair: &Keypair,
        originator_fqei: FQEI,
        kdu_type: String,
        data_payload: serde_json::Value,
    ) -> KDU {
        // 1. Generate timestamp and ID from HLC
        let (kdu_id, timestamp_utc) = self.hlc.now();

        // 2. Create placeholder metadata and license
        let license = License {
            license_id: "BaDaaS-1.1.0".to_string(),
            licensor_fqei: originator_fqei.clone(),
            custom_terms_hash: None,
        };
        
        let mut metadata = Metadata {
            metadata_hash: String::new(), // Placeholder
            unisphere_coordinates: vec![0; 42],
            license,
            causal_link: None,
        };

        // 3. Hash the metadata (with its placeholder hash)
        let metadata_hash = CryptoCore::hash_content(&metadata);
        metadata.metadata_hash = hex::encode(&metadata_hash); // Use hex for readability

        // 4. Create the full KDU struct with placeholder hash and signature
        let mut kdu = KDU {
            kdu_spec_version: "2.1.0".to_string(),
            kdu_id,
            content_hash: String::new(), // Placeholder
            originator_fqei,
            originator_signature: Vec::new(), // Placeholder
            timestamp_utc,
            kdu_type,
            metadata,
            data_payload,
        };

        // 5. Create the final content hash (of metadata + payload)
        let content_to_hash = (
            &kdu.metadata,
            &kdu.data_payload,
        );
        let content_hash = CryptoCore::hash_content(&content_to_hash);
        kdu.content_hash = hex::encode(&content_hash);

        // 6. Sign the final content hash
        let signature = self.crypto_core.sign(&content_hash, &originator_keypair.secret);
        kdu.originator_signature = signature.to_bytes().to_vec();

        // 7. Return the final, valid KDU
        kdu
    }
}
