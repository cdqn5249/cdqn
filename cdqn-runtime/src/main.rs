use wasmtime::{Engine, Linker, Module, Store};
use anyhow::Result;

// This is the crucial change. We now `use` our own crate's library
// to get access to the modules defined in `lib.rs`.
use cdqn_runtime::cdu_log::LogWriter;

/// The main entry point for the cdqn Sovereign Runtime.
#[tokio::main]
async fn main() -> Result<()> {
    println!("cdqnRuntime: Sovereign Operating System starting...");

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
