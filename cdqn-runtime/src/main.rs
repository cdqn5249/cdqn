use wasmtime::{Engine, Linker, Module, Store};
use anyhow::Result;

// 1. Declare that our main application uses the `cdu_log` module.
mod cdu_log;
// 2. Import the LogWriter struct so we can use it.
use cdu_log::LogWriter;

/// The main entry point for the cdqn Sovereign Runtime.
#[tokio::main]
async fn main() -> Result<()> {
    println!("cdqnRuntime: Sovereign Operating System starting...");

    // 3. Create an instance of our LogWriter.
    let _log_writer = LogWriter::new("cdu.log").await?;
    println!("cdqnRuntime: Append-only CDU log is active.");


    // --- WASM HOST LOGIC (from previous step) ---

    let engine = Engine::default();
    println!("cdqnRuntime: Wasmtime engine created.");

    let wasm_path = "../target/wasm32-wasi/debug/hello_world_worker.wasm";
    println!("cdqnRuntime: Loading component from '{}'...", wasm_path);
    let module = Module::from_file(&engine, wasm_path)?;
    println!("cdqnRuntime: Component loaded successfully.");

    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let instance = linker.instantiate(&mut store, &module)?;
    println!("cdqnRuntime: Component instantiated.");

    let run_func = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    println!("cdqnRuntime: Found exported 'run' function.");

    println!("cdqnRuntime: Executing component...");
    println!("-----------------------------------------");

    run_func.call(&mut store, ())?;

    println!("-----------------------------------------");
    println!("cdqnRuntime: Component execution finished.");

    Ok(())
}
