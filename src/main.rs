// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::runtime::persistence::Persistence; // Import the new service
use std::fs; // Used for cleaning up the database directory

fn main() {
    println!("cdqn runtime starting... [Phase 1, Milestone 1]");

    // --- Setup ---
    let factory = KDUFactory::default(); // Use the Default trait
    let crypto_core = factory.crypto_core();
    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();
    let db_path = "./cdqn_db_test"; // Define a path for our test database

    // --- KDU Creation ---
    let payload = serde_json::json!({"action": "persistence.test"});
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        payload,
    );
    println!("\n--- 1. KDU Created in Memory ---");
    println!("KDU ID: {}", new_kdu.kdu_id);

    // --- Persistence Write ---
    println!("\n--- 2. Writing KDU to Database ---");
    let persistence = Persistence::new(db_path.as_ref()).expect("Failed to open DB");
    persistence.write_kdu(&new_kdu).expect("Failed to write KDU");
    println!("SUCCESS: KDU written to path '{}'", db_path);

    // --- Persistence Read ---
    println!("\n--- 3. Reading KDU from Database ---");
    let retrieved_kdu = persistence
        .read_kdu(&new_kdu.kdu_id)
        .expect("Read operation failed")
        .expect("KDU not found in DB");
    println!("SUCCESS: Retrieved KDU with ID: {}", retrieved_kdu.kdu_id);

    // --- Verification ---
    assert_eq!(new_kdu.content_hash, retrieved_kdu.content_hash);
    println!("\n--- 4. Verification ---");
    println!("SUCCESS: Retrieved KDU matches original KDU.");

    // --- Cleanup ---
    // We drop the persistence object to release the lock on the DB files
    drop(persistence); 
    fs::remove_dir_all(db_path).expect("Failed to clean up DB directory");
    println!("\n--- 5. Cleanup ---");
    println!("SUCCESS: Database directory cleaned up.");
    
    println!("\n--- C.Persistence service implemented successfully! ---");
}
