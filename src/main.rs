// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::core::ChronosaCore;

fn main() {
    println!("Initializing Chronosa Core...");
    let mut core = ChronosaCore::new();

    println!("Recording genesis event...");
    let cdu1 = core.record(b"Chronosa instance created.".to_vec(), "genesis");
    println!("  -> Recorded CDU: {}", cdu1.name);

    println!("Recording observation event...");
    let cdu2 = core.record(
        b"First observation of the environment.".to_vec(),
        "observation",
    );
    println!("  -> Recorded CDU: {}", cdu2.name);

    println!(
        "\nChronosa Core is operational and has recorded {} events.",
        2
    );
}
