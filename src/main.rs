// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use ed25519_dalek::Verifier;

fn main() {
    println!("cdqn runtime starting... [Phase 0, Milestone 3]");

    // 1. Initialize the KDUFactory. This also initializes CryptoCore and HLC.
    let factory = KDUFactory::new();
    let crypto_core = factory.crypto_core();

    // 2. Generate a new identity for our originator.
    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();

    // 3. Define the payload for our new KDU.
    let payload = serde_json::json!({
        "action": "self.test",
        "status": "ok"
    });

    // 4. Use the factory to create a valid, signed KDU.
    println!("\n--- Creating KDU with Factory ---");
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        payload,
    );

    let kdu_as_json = serde_json::to_string_pretty(&new_kdu).unwrap();
    println!("{}", kdu_as_json);

    // 5. --- Verification Step ---
    // This is the most important part: we prove the signature is valid.
    println!("\n--- Verifying KDU Signature ---");
    
    // Re-hash the content exactly as the factory did.
    let content_to_hash = (
        &new_kdu.metadata,
        &new_kdu.data_payload,
    );
    let content_hash_bytes = cdqn::kernel::crypto::CryptoCore::hash_content(&content_to_hash);

    // Reconstruct the signature from the KDU's bytes.
    let signature = ed25519_dalek::Signature::from_bytes(&new_kdu.originator_signature).unwrap();

    // Use the public key to verify the signature against the hash.
    let verification_result = originator_keypair.public.verify(&content_hash_bytes, &signature);

    match verification_result {
        Ok(_) => println!("SUCCESS: Signature is valid."),
        Err(e) => println!("FAILURE: Signature is invalid! Error: {}", e),
    }
    
    println!("\n--- Kernel services implemented successfully! ---");
}
