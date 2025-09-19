// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::runtime::persistence::Persistence;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    println!("cdqn runtime starting... [Sovereign Persistence Read/Write]");

    // --- Setup ---
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();
    let db_path = Path::new("./cdqn_sovereign_db_test");

    // --- KDU Creation ---
    let payload_struct = TestPayload {
        action: "sovereign.persistence.readwrite".to_string(),
        status: "ok".to_string(),
    };
    let payload_bytes = bincode::serialize(&payload_struct).unwrap();
    let kdu_to_write = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        &payload_bytes,
    );
    println!("\n--- 1. KDU Created in Memory ---");
    println!("KDU ID to write: {}", kdu_to_write.kdu_id);

    // --- Persistence Write ---
    {
        println!("\n--- 2. Writing KDU to Sovereign Journal ---");
        // We need to make persistence mutable to update its index
        let mut persistence =
            Persistence::new(db_path).expect("Failed to create persistence service");
        persistence
            .write_kdu(&kdu_to_write)
            .expect("Failed to write KDU to journal");
        println!("SUCCESS: KDU written to journal.");
        // persistence is dropped here, closing the file.
    }

    // --- Persistence Read ---
    {
        println!("\n--- 3. Reading KDU from Sovereign Journal ---");
        // Create a new persistence service. It will build its index from the existing file.
        let persistence = Persistence::new(db_path).expect("Failed to open persistence service");
        let retrieved_kdu = persistence
            .read_kdu(&kdu_to_write.kdu_id)
            .expect("Read operation failed")
            .expect("KDU not found in journal via index");
        println!("SUCCESS: Retrieved KDU with ID: {}", retrieved_kdu.kdu_id);

        // --- Verification ---
        assert_eq!(kdu_to_write.content_hash, retrieved_kdu.content_hash);
        println!("\n--- 4. Verification ---");
        println!("SUCCESS: Retrieved KDU matches original KDU.");
    }

    // --- Cleanup ---
    fs::remove_dir_all(db_path).expect("Failed to clean up DB directory");
    println!("\n--- 5. Cleanup ---");
    println!("SUCCESS: Database directory cleaned up.");

    println!("\n--- Sovereign Persistence Layer implemented successfully! ---");
}
