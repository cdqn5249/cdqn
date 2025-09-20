// src/main.rs

use base64::Engine as _;
use cdqn::kernel::factory::KDUFactory;
use cdqn::kernel::KDU;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};
// Import the Verifier trait to bring the .verify() method into scope.
use ed25519_dalek::Verifier;

// --- SHARED CONFIGURATION ---
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/cdqn5249/cdqn/actions/workflows/kdu-pipe.yml/dispatches";

#[derive(Serialize, Deserialize, Debug)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).cloned().unwrap_or_default();

    if mode == "--client" {
        let github_token = args.get(2).expect("Usage: cdqn --client <github_token>");
        run_client(github_token);
    } else if mode == "--process" {
        run_processor();
    } else if mode == "--ci-test" {
        run_ci_test();
    } else {
        println!("Usage:\n  cdqn --client <github_token>\n  cdqn --process\n  cdqn --ci-test");
    }
}

// --- CI TEST MODE ---
fn run_ci_test() {
    println!("--- Running in CI-TEST mode ---");
    let factory = KDUFactory::default();
    let crypto_core = factory.crypto_core();
    let originator_keypair = crypto_core.generate_keypair();
    let originator_fqei = "agent@ci.test".to_string();

    let payload = b"ci-test-payload".to_vec();
    let kdu = factory.create_kdu(
        &originator_keypair,
        originator_fqei,
        "CITest".to_string(),
        &payload,
    );

    println!("KDU created with ID: {}", kdu.kdu_id);

    // --- Verification Step ---
    let content_to_hash = (&kdu.metadata, &kdu.data_payload);
    let content_hash_bytes = cdqn::kernel::crypto::CryptoCore::hash_content(&content_to_hash);
    let signature =
        ed25519_dalek::Signature::from_bytes(kdu.originator_signature.as_slice().try_into().unwrap());
    let verification_result = originator_keypair
        .public
        .verify(&content_hash_bytes, &signature);

    assert!(
        verification_result.is_ok(),
        "FATAL: KDU signature verification failed in CI!"
    );
    println!("SUCCESS: KDU signature verified correctly.");
    println!("\n--- CI Test Passed ---");
}

// --- PROCESSOR MODE (The "Serverless Server") ---
fn run_processor() {
    println!("--- Running in PROCESSOR mode ---");
    // This is the corrected line.
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read KDU from stdin");
    let incoming_kdu: KDU = bincode::deserialize(&buffer).expect("Failed to deserialize KDU");
    println!("Processor received KDU with ID: {}", incoming_kdu.kdu_id);
    let factory = KDUFactory::default();
    let ponger_keypair = factory.crypto_core().generate_keypair();
    let ponger_fqei = "ponger@processor.bot".to_string();
    let response_payload = TestPayload {
        action: "sovereign.pipe.response".to_string(),
        status: "ok".to_string(),
    };
    let response_kdu = factory.create_kdu(
        &ponger_keypair,
        ponger_fqei,
        "PongResponse".to_string(),
        &bincode::serialize(&response_payload).unwrap(),
    );
    println!(
        "Processor created response KDU with ID: {}",
        response_kdu.kdu_id
    );
    let kdu_base64 = base64::engine::general_purpose::STANDARD
        .encode(bincode::serialize(&response_kdu).unwrap());
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found");
    let request_body = ureq::json!({ "ref": "gh-pages", "inputs": { "kdu_filename": "pong.kdu", "kdu_content_base64": kdu_base64 } });
    let response = ureq::post(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("Authorization", &format!("token {}", github_token))
        .send_json(request_body);
    if response.is_ok() && response.unwrap().status() == 204 {
        println!("SUCCESS: Processor triggered pipe with pong.kdu.");
    } else {
        eprintln!("FAILURE: Processor could not trigger pipe workflow.");
    }
}

// --- CLIENT MODE ---
fn run_client(github_token: &str) {
    println!("--- Starting in CLIENT mode ---");
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let pinger_fqei = "pinger@client".to_string();

    let payload_struct = TestPayload {
        action: "sovereign.pipe.test".to_string(),
        status: "ok".to_string(),
    };
    let initial_ping = factory.create_kdu(
        &originator_keypair,
        pinger_fqei,
        "InitialPing".to_string(),
        &bincode::serialize(&payload_struct).unwrap(),
    );
    println!("Client created ping KDU with ID: {}", initial_ping.kdu_id);
    let kdu_base64 = base64::engine::general_purpose::STANDARD
        .encode(bincode::serialize(&initial_ping).unwrap());
    let request_body = ureq::json!({ "ref": "gh-pages", "inputs": { "kdu_filename": "ping.kdu", "kdu_content_base64": kdu_base64 } });
    let response = ureq::post(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("Authorization", &format!("token {}", github_token))
        .send_json(request_body);
    if response.is_ok() && response.unwrap().status() == 204 {
        println!("\nSUCCESS: Workflow triggered successfully.");
        println!("Watch the Actions tab for the 'CDQN KDU Processor' to run.");
    } else {
        eprintln!("\nFAILURE: Could not trigger workflow.");
    }
}
