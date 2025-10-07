// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

//! The main entry point for the cdqn binary.
//! Its only job is to call the main runtime orchestrator.

fn main() {
    // Call the main run function from the cdqn_runtime module.
    cdqn::cdqn_runtime::run();
}
