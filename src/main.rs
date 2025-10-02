// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::orchestrator::Orchestrator;

fn main() {
    println!("--- Chronosa Agent Simulation ---");

    // 1. Initialize the Orchestrator.
    let orchestrator = Orchestrator::new();

    // 2. Simulate an initial observation.
    println!("Simulating: Agent observes 'see food'");
    let (core, _) = orchestrator
        .core()
        .clone()
        .record(b"see food".to_vec(), "observation");
    let orchestrator = Orchestrator { core }; // Create a new orchestrator with this memory

    // 3. Run the agent's thought cycle.
    println!("Orchestrator is thinking...");
    let orchestrator = orchestrator.tick();

    // 4. Inspect the result.
    let last_action = orchestrator.core().find_last_by_subtype("action");
    if let Some(action) = last_action {
        println!(
            "Orchestrator decided to: {}",
            String::from_utf8_lossy(&action.payload)
        );
        println!("  -> Action CDU: {}", action.name);
        println!("     (Caused by observation: {})", action.metadata.causes[0]);
    } else {
        println!("Orchestrator decided to do nothing.");
    }

    println!("\nSimulation complete.");
}
