// src/main.rs

// Use the public items from our library. Note: FQEI is removed as it was unused.
use cdqn::kernel::{License, Metadata, KDU};

fn main() {
    println!("cdqn runtime starting... [Phase 0, Milestone 2]");

    // Create a dummy KDU to demonstrate the data structures.
    let dummy_kdu = KDU {
        kdu_spec_version: "2.1.0".to_string(),
        kdu_id: "01H8XJ...".to_string(),    // Placeholder HLC ID
        content_hash: "zQm...".to_string(), // Placeholder content hash
        originator_fqei: "agent@U.MyProject#01H8XJ...".to_string(),
        originator_signature: vec![0u8; 64], // Placeholder 64-byte signature
        timestamp_utc: "2025-09-19T10:00:00Z".to_string(),
        kdu_type: "Generic".to_string(),
        metadata: Metadata {
            metadata_hash: "zQm...meta".to_string(),
            // Correctly create a Vec<u16> instead of an array.
            unisphere_coordinates: vec![0; 42],
            license: License {
                license_id: "BaDaaS-1.1.0".to_string(),
                licensor_fqei: "user@U.CDQN#...".to_string(),
                custom_terms_hash: None,
            },
            causal_link: None,
        },
        data_payload: serde_json::json!({
            "action": "example.ping",
            "value": 123
        }),
    };

    // Print the KDU in a nicely formatted JSON string.
    let kdu_as_json = serde_json::to_string_pretty(&dummy_kdu).unwrap();
    println!("\n--- Sample KDU ---");
    println!("{}", kdu_as_json);
    println!("\n--- KDU structure implemented successfully! ---");
}
