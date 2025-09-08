// Import the tools we need from the wasmtime library
use wasmtime::{Engine, Linker, Module, Store};
use anyhow::Result;

/// The main entry point for the cdqn Sovereign Runtime.
#[tokio::main]
async fn main() -> Result<()> {
    println!("cdqnRuntime: Sovereign Operating System starting...");

    // --- WASM HOST LOGIC STARTS HERE ---

    // 1. Create a Wasmtime engine. This is the core JIT compiler.
    let engine = Engine::default();
    println!("cdqnRuntime: Wasmtime engine created.");

    // 2. Load our compiled WASM component from the filesystem.
    //    This path points to the standard location where `cargo build`
    //    places the WASM file for our worker.
    let wasm_path = "../target/wasm32-wasi/debug/hello_world_worker.wasm";
    println!("cdqnRuntime: Loading component from '{}'...", wasm_path);
    let module = Module::from_file(&engine, wasm_path)?;
    println!("cdqnRuntime: Component loaded successfully.");

    // 3. Create a "linker". This is used to define what functions the host
    //    (our runtime) provides to the guest (the WASM component).
    //    For now, we provide nothing.
    let mut linker = Linker::new(&engine);

    // 4. Create a "store". This holds all the state for a WASM instance.
    let mut store = Store::new(&engine, ());

    // 5. Instantiate the component. This links the component's imports
    //    (of which there are none) and creates a runnable instance.
    let instance = linker.instantiate(&mut store, &module)?;
    println!("cdqnRuntime: Component instantiated.");

    // 6. Look up the `run` function that our worker exported in its .wit file.
    //    We use `get_typed_func` to ensure it has the correct signature
    //    (no parameters, no return value).
    let run_func = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    println!("cdqnRuntime: Found exported 'run' function.");

    // 7. Call the function! This is the moment the runtime passes control
    //    to the sandboxed WASM code.
    println!("cdqnRuntime: Executing component...");
    println!("-----------------------------------------");

    run_func.call(&mut store, ())?;

    println!("-----------------------------------------");
    println!("cdqnRuntime: Component execution finished.");

    // --- WASM HOST LOGIC ENDS HERE ---

    Ok(())
}
