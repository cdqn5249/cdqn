// This macro tells wit-bindgen to generate all the necessary "glue" code.
wit_bindgen::generate!({
    path: "wit/worker.wit",
    world: "local:hello-world/hello",
});

// We define a struct that will hold our component's implementation.
struct MyWorker;

// This is the crucial fix. The `generate!` macro creates a module named
// after our world (`hello`). The trait we need to implement (`Guest`) is
// INSIDE that module. We must use the full path `hello::Guest`.
impl hello::Guest for MyWorker {
    fn run() {
        // This code will execute inside the sandbox.
        println!("Hello from inside the WASM sandbox! The worker is running!");
    }
}

// The `export!` macro is ALSO inside the generated module.
// We must call it using its full path.
export_hello!(MyWorker);
