// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

// Use the public structs from our cdqn library crate.
use cdqn::cdu::{Cdu, CduMetadata};
use cdqn::hlc::Hlc;

fn main() {
    println!("Chronosa Core: Instantiating core data structures...");

    // 1. Create a sample Hybrid Logical Clock timestamp.
    let hlc = Hlc {
        timestamp: 1672531200000, // Example: Jan 1, 2023
        counter: 0,
    };

    // 2. Create sample metadata for a CDU.
    let metadata = CduMetadata {
        hlc: hlc.clone(), // Clone the hlc for the metadata
        causes: vec![],   // This is the first event, so it has no causes.
        tags: vec!["initial_event".to_string()],
    };

    // 3. Create a sample Causal Data Unit.
    let cdu = Cdu {
        name: "00000000.genesis.cdu".to_string(), // A placeholder name for this example
        payload: vec![1, 2, 3, 4],               // Some example byte data
        metadata,
    };

    // By creating these instances, we are now "using" the imports.
    // We print them using the debug formatter to show they were created.
    println!("\nSuccessfully created a sample CDU:");
    println!("{:#?}", cdu);

    println!("\nValidation complete. The modular structure is operational.");
}
