// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use ed25519_dalek::Verifier;
use std::convert::TryInto; // Import the conversion trait

fn main() {
    println!("cdqn runtime starting... [Phase 0, Milestone 3]");

    let factory = KDUFactory::new();
    let crypto_core = factory.crypto_core();

    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();

    let payload = serde_json::json!({
        "action": "self.test",
        "status": "ok"
    });

    println!("\n--- Creating KDU with Factory ---");
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        payload,
    );

    let kdu_as_json = serde_json::to_string_pretty(&new_kdu).unwrap();
    println!("{}", kdu_as_json);

    println!("\n--- Verifying KDU Signature ---");
    
    let content_to_hash = (&new_kdu.metadata, &new_kdu.data_payload);
    let content_hash_bytes = cdqn::kernel::crypto::CryptoCore::hash_content(&content_to_hash);

    // Correctly convert the Vec<u8> slice into a fixed-size array reference.
    let signature_bytes: &[u8; 64] = new_kdu.originator_signature
        .as_slice()
        .try_into()
        .expect("Signature slice must be 64 bytes long");

    // Reconstruct the signature. Note the removal of .unwrap().
    let signature = ed25519_dalek::Signature::from_bytes(signature_bytes);

    // Use the public key from our keypair to verify
    let verification_result = originator_keypair.public.verify(&content_hash_bytes, &signature);

    match verification_result {
        Ok(_) => println!("SUCCESS: Signature is valid."),
        Err(e) => println!("FAILURE: Signature is invalid! Error: {}", e),
    }
    
    println!("\n--- Kernel services implemented successfully! ---");
}
