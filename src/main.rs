// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::core::ChronosaCore;

fn main() {
    println!("Initializing Chronosa Core...");
    let mut core = ChronosaCore::new();

    println!("Recording genesis event...");
    let cdu1 = core.record(b"Chronosa instance created.".to_vec(), "genesis");
    println!("  -> Recorded CDU: {}", cdu1.name);

    println!("Recording a causally linked action...");
    // This action is a direct result of the genesis event.
    let cdu2 = core.record_causal(
        b"Perform initial environmental scan.".to_vec(),
        "action",
        &[&cdu1],
    );
    println!("  -> Recorded CDU: {}", cdu2.name);
    println!("     (Caused by: {})", cdu2.metadata.causes[0]);

    println!(
        "\nChronosa Core is operational and has recorded {} causally linked events.",
        2
    );
}
