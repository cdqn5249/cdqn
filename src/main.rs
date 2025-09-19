// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::runtime::persistence::Persistence; // Import our new service
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    println!("cdqn runtime starting... [Sovereign Persistence Write]");

    // --- Setup ---
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let originator_fqei = "agent@U.TestNode#01".to_string();
    let db_path = Path::new("./cdqn_sovereign_db_test");

    // --- KDU Creation ---
    let payload_struct = TestPayload {
        action: "sovereign.persistence.write".to_string(),
        status: "ok".to_string(),
    };
    let payload_bytes = bincode::serialize(&payload_struct).unwrap();
    let new_kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "Generic".to_string(),
        &payload_bytes,
    );
    println!("\n--- 1. KDU Created in Memory ---");
    println!("KDU ID: {}", new_kdu.kdu_id);

    // --- Persistence Write ---
    println!("\n--- 2. Writing KDU to Sovereign Journal ---");
    let persistence = Persistence::new(db_path).expect("Failed to create persistence service");
    persistence
        .write_kdu(&new_kdu)
        .expect("Failed to write KDU to journal");
    println!("SUCCESS: KDU written to journal in '{}'", db_path.display());

    // --- Verification (Manual for now) ---
    // We can't read it back yet, but we can check that the file was created.
    let journal_file = db_path.join("00000001.journal");
    assert!(journal_file.exists(), "Journal file was not created!");
    println!("\n--- 3. Verification ---");
    println!("SUCCESS: Journal file exists.");

    // --- Cleanup ---
    fs::remove_dir_all(db_path).expect("Failed to clean up DB directory");
    println!("\n--- 4. Cleanup ---");
    println!("SUCCESS: Database directory cleaned up.");

    println!("\n--- Sovereign JournalWriter implemented successfully! ---");
}
