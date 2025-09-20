// src/main.rs

use cdqn::kernel::factory::KDUFactory;
use cdqn::kernel::KDU;
use serde::Serialize;
use std::env;
use std::thread;
use std::time::Duration;

// --- SHARED CONFIGURATION ---
const GITHUB_API_URL: &str = "https://api.github.com/repos/cdqn5249/cdqn/actions/workflows/kdu-pipe.yml/dispatches";
const GITHUB_PAGES_URL: &str = "https://cdqn5249.github.io/cdqn/";

#[derive(Serialize)]
struct TestPayload {
    action: String,
    status: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).cloned().unwrap_or_default();

    if mode == "--server" {
        run_server();
    } else if mode == "--client" {
        // The client now needs a GitHub Personal Access Token to trigger the workflow.
        let github_token = args.get(2).expect("Usage: cdqn --client <github_token>");
        run_client(github_token);
    } else {
        println!("Usage:\n  cdqn --server\n  cdqn --client <github_token>");
    }
}

// --- SERVER MODE ---
fn run_server() {
    println!("--- Starting in SERVER mode ---");
    println!("Polling for 'ping.kdu' at {}ping.kdu", GITHUB_PAGES_URL);

    loop {
        // Poll the GitHub Pages URL for the ping file.
        let response = ureq::get(&format!("{}ping.kdu", GITHUB_PAGES_URL)).call();
        
        if let Ok(response) = response {
            if response.status() == 200 {
                println!("\nSUCCESS: Found 'ping.kdu'!");
                let mut kdu_bytes = Vec::new();
                response.into_reader().read_to_end(&mut kdu_bytes).unwrap();
                
                let kdu: KDU = bincode::deserialize(&kdu_bytes).expect("Failed to deserialize ping KDU");
                println!("Received and deserialized KDU with ID: {}", kdu.kdu_id);
                
                // In a real system, we would verify the signature here.
                println!("KDU is valid (simulation). Now exiting.");
                break; // Exit after successfully processing one KDU.
            }
        }
        
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(5)); // Wait 5 seconds before polling again.
    }
}

// --- CLIENT MODE ---
fn run_client(github_token: &str) {
    println!("--- Starting in CLIENT mode ---");
    let factory = KDUFactory::default();
    let originator_keypair = factory.crypto_core().generate_keypair();
    let pinger_fqei = "pinger@test".to_string();
    
    let payload_struct = TestPayload {
        action: "sovereign.pipe.test".to_string(),
        status: "ok".to_string(),
    };
    let payload_bytes = bincode::serialize(&payload_struct).unwrap();
    let initial_ping = factory.create_kdu(
        &originator_keypair,
        pinger_fqei,
        "InitialPing".to_string(),
        &payload_bytes,
    );

    println!("Client created ping KDU with ID: {}", initial_ping.kdu_id);
    let kdu_bytes = bincode::serialize(&initial_ping).unwrap();
    let kdu_base64 = base64::encode(&kdu_bytes);

    println!("Sending KDU to GitHub Actions pipe...");

    // Create the JSON payload for the GitHub API.
    let request_body = ureq::json!({
        "ref": "gh-pages",
        "inputs": {
            "kdu_filename": "ping.kdu",
            "kdu_content_base64": kdu_base64
        }
    });

    // Make the authenticated HTTP request to trigger the workflow.
    let response = ureq::post(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("Authorization", &format!("token {}", github_token))
        .send_json(request_body);

    if response.is_ok() && response.unwrap().status() == 204 {
        println!("\nSUCCESS: Workflow triggered successfully.");
        println!("It may take 1-2 minutes for the 'ping.kdu' file to appear on GitHub Pages.");
    } else {
        eprintln!("\nFAILURE: Could not trigger workflow. Check your GitHub token and permissions.");
        if let Some(resp) = response.err() {
            eprintln!("Error details: {}", resp);
        }
    }
}
