// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use ed25519_dalek::Verifier;
use std::convert::TryInto;

fn main() {
    println!("cdqn runtime starting... [Sovereign Core Baseline]");

    // --- Setup ---
    let factory = KDUFactory::default();
    let crypto_core = factory.crypto_core();
    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();

    // --- KDU Creation ---
    let payload = serde_json::json!({"action": "sovereign.core.test"});
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        payload,
    );
    println!("\n--- 1. KDU Created in Memory ---");
    let kdu_as_json = serde_json::to_string_pretty(&new_kdu).unwrap();
    println!("{}", kdu_as_json);


    // --- Verification ---
    println!("\n--- 2. Verifying KDU Signature ---");
    let content_to_hash = (&new_kdu.metadata, &new_kdu.data_payload);
    let content_hash_bytes = cdqn::kernel::crypto::CryptoCore::hash_content(&content_to_hash);

    let signature_bytes: &[u8; 64] = new_kdu
        .originator_signature
        .as_slice()
        .try_into()
        .expect("Signature slice must be 64 bytes long");

    let signature = ed25519_dalek::Signature::from_bytes(signature_bytes);

    let verification_result = originator_keypair
        .public
        .verify(&content_hash_bytes, &signature);

    match verification_result {
        Ok(_) => println!("SUCCESS: Signature is valid."),
        Err(e) => println!("FAILURE: Signature is invalid! Error: {}", e),
    }

    println!("\n--- Sovereign Core Baseline Established ---");
}
