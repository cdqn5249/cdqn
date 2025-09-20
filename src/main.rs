// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::kernel::KDU;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};

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
    } else {
        println!("Usage:\n  cdqn --client <github_token>\n  cdqn --process");
    }
}

// --- PROCESSOR MODE (The "Serverless Server") ---
fn run_processor() {
    println!("--- Running in PROCESSOR mode ---");

    // 1. Read the incoming KDU from standard input.
    let mut buffer = Vec::new();
    io::stdin()
        .read_to_end(&mut buffer)
        .expect("Failed to read KDU from stdin");
    let incoming_kdu: KDU = bincode::deserialize(&buffer).expect("Failed to deserialize KDU");
    println!("Processor received KDU with ID: {}", incoming_kdu.kdu_id);

    // 2. Process the KDU (our "Ponger" logic).
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

    // 3. Trigger the pipe workflow to send the response.
    let kdu_base64 = base64::encode(&bincode::serialize(&response_kdu).unwrap());
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found");

    let request_body = ureq::json!({
        "ref": "gh-pages",
        "inputs": {
            "kdu_filename": "pong.kdu",
            "kdu_content_base64": kdu_base64
        }
    });

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
    // This function remains largely the same.
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
    let kdu_base64 = base64::encode(&bincode::serialize(&initial_ping).unwrap());

    let request_body = ureq::json!({
        "ref": "gh-pages",
        "inputs": {
            "kdu_filename": "ping.kdu",
            "kdu_content_base64": kdu_base64
        }
    });

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
