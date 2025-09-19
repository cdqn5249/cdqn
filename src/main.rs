// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use ed25519_dalek::Verifier;
use serde::Serialize; // We need this for our payload struct
use std::convert::TryInto;

// Define a simple struct for our test payload.
// It must derive Serialize to be convertible to bincode.
#[derive(Serialize)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    println!("cdqn runtime starting... [Sovereign Core Baseline]");

    // --- Setup ---
    let factory = KDUFactory::default();
    let crypto_core = factory.crypto_core();
    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();

    // --- KDU Creation ---
    // 1. Create the payload data as a struct.
    let payload_struct = TestPayload {
        action: "sovereign.core.test".to_string(),
        status: "ok".to_string(),
    };
    // 2. Serialize the payload struct into a vector of bytes using bincode.
    let payload_bytes = bincode::serialize(&payload_struct).unwrap();

    // 3. Create the KDU using the serialized byte payload.
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        &payload_bytes,
    );
    println!("\n--- 1. KDU Created in Memory ---");
    // We can't easily print the whole KDU as JSON anymore, so we print its ID.
    println!("KDU ID: {}", new_kdu.kdu_id);
    println!("Payload as bytes: {:?}", new_kdu.data_payload);

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
