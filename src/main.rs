// src/main.rs

use base64::Engine as _;
use cdqn::kernel::crypto::CryptoCore;
use cdqn::kernel::factory::KDUFactory;
use cdqn::kernel::KDU;
use ed25519_dalek::Verifier;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};

// --- SHARED CONFIGURATION ---
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/cdqn5249/cdqn/actions/workflows/kdu-handler.yml/dispatches";
const USER_AGENT: &str = "cdqn-runtime-mvp-test";

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
    let content_to_hash = (&kdu.metadata, &kdu.data_payload);
    let content_hash_bytes = CryptoCore::hash_content(&content_to_hash);
    let signature = ed25519_dalek::Signature::from_bytes(
        kdu.originator_signature.as_slice().try_into().unwrap(),
    );
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

// --- PROCESSOR MODE ---
fn run_processor() {
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read KDU from stdin");
    let _incoming_kdu: KDU = bincode::deserialize(&buffer).expect("Failed to deserialize KDU");

    let factory = KDUFactory::default();
    let ponger_keypair = factory.crypto_core().generate_keypair();
    let ponger_fqei = "ponger@processor.bot".to_string();
    let response_payload = TestPayload {
        action: "sovereign.handler.response".to_string(),
        status: "ok".to_string(),
    };
    let mut response_kdu = factory.create_kdu(
        &ponger_keypair,
        ponger_fqei,
        "PongResponse".to_string(),
        &bincode::serialize(&response_payload).unwrap(),
    );

    // Re-hash to get the final, correct content hash
    let content_to_hash = (&response_kdu.metadata, &response_kdu.data_payload);
    let final_hash = CryptoCore::hash_content(&content_to_hash);
    response_kdu.content_hash = hex::encode(&final_hash);

    let pong_filename = format!("{}.kdu", response_kdu.content_hash);
    let kdu_base64 = base64::engine::general_purpose::STANDARD
        .encode(bincode::serialize(&response_kdu).unwrap());

    // Output the filename and content, separated by a comma.
    print!("{},{}", pong_filename, kdu_base64);
}

// --- CLIENT MODE ---
fn run_client(github_token: &str) {
    println!("--- Starting in CLIENT mode ---");
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let pinger_fqei = "pinger@client".to_string();
    let payload_struct = TestPayload {
        action: "sovereign.handler.test".to_string(),
        status: "ok".to_string(),
    };
    let initial_ping = factory.create_kdu(
        &originator_keypair,
        pinger_fqei,
        "InitialPing".to_string(),
        &bincode::serialize(&payload_struct).unwrap(),
    );

    let ping_filename = format!("{}.kdu", initial_ping.content_hash);
    println!("Client created ping KDU with filename: {}", ping_filename);

    let kdu_base64 = base64::engine::general_purpose::STANDARD
        .encode(bincode::serialize(&initial_ping).unwrap());

    let request_body = ureq::json!({
        "ref": "main",
        "inputs": {
            "ping_kdu_filename": ping_filename,
            "ping_kdu_base64": kdu_base64,
            "github_token": github_token
        }
    });

    let response = ureq::post(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("Authorization", &format!("Bearer {}", github_token))
        .set("User-Agent", USER_AGENT)
        .send_json(request_body);

    match response {
        Ok(resp) if resp.status() == 204 => {
            println!("\nSUCCESS: Workflow triggered successfully.");
            println!("Watch the Actions tab for the 'CDQN KDU Handler' to run.");
        }
        Ok(resp) => {
            eprintln!("\nFAILURE: Received non-204 status: {}", resp.status());
            eprintln!("Response body: {}", resp.into_string().unwrap_or_default());
        }
        Err(ureq::Error::Status(_code, response)) => {
            eprintln!("\nFAILURE: GitHub API returned an error.");
            eprintln!(
                "Response body: {}",
                response.into_string().unwrap_or_default()
            );
        }
        Err(e) => {
            eprintln!("\nFAILURE: Transport error.");
            eprintln!("Error Details: {}", e);
        }
    }
}
