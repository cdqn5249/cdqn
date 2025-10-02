// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::core::ChronosaCore;

fn main() {
    println!("Initializing Chronosa Core...");
    // `core` is no longer mutable. It's a state that gets replaced.
    let core = ChronosaCore::new();

    println!("Recording genesis event...");
    // The original `core` is consumed, and a new `core` and `cdu1` are created.
    let (core, cdu1) = core.record(b"Chronosa instance created.".to_vec(), "genesis");
    println!("  -> Recorded CDU: {}", cdu1.name);

    println!("Recording a causally linked action...");
    // The updated `core` is consumed to produce the final `core` state.
    let (core, cdu2) = core.record_causal(
        b"Perform initial environmental scan.".to_vec(),
        "action",
        &[&cdu1],
    );
    println!("  -> Recorded CDU: {}", cdu2.name);
    println!("     (Caused by: {})", cdu2.metadata.causes[0]);

    println!(
        "\nChronosa Core is operational and has recorded {} causally linked events.",
        core.log.len() // Read the length from the final core state.
    );
}
