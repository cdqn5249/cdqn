// We import a type from our `cdqn-types` crate to prove the link is working.
use cdqn_types::CduType;

/// The main entry point for the cdqn Sovereign Runtime.
///
/// The `#[tokio::main]` attribute transforms this asynchronous function
/// into a standard synchronous `main` function, automatically setting up
/// the Tokio runtime. This is the foundation for our "Asynchronous First"
/// architectural constraint.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Print a startup message to the console.
    println!("cdqnRuntime: Sovereign Operating System starting...");

    // Create an instance of a CduType to demonstrate we can use our other crate.
    let example_type = CduType::System;

    // The `{:?}` format specifier is for debug printing.
    println!("Successfully imported a type from cdqn-types: {:?}", example_type);

    println!("cdqnRuntime: Startup complete. Awaiting tasks...");

    // In the future, this is where the main event loop will live,
    // listening for tasks and dispatching them to WASM components.
    // For now, we just exit successfully.

    // `anyhow::Result<()>` requires us to return Ok at the end.
    Ok(())
}
