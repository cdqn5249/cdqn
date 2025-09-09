// This is the canonical pattern for a cargo-component guest,
// based on the official Bytecode Alliance examples.

// Step 1: Use the `generate!` macro from the `wit-bindgen` crate.
// This single macro reads our WIT, generates the bindings, and makes them available.
wit_bindgen::generate!({
    world: "hello",
    path: "wit",
});

// We define a struct that will hold our component's implementation.
struct MyWorker;

// Step 2: The `generate!` macro automatically creates the `Guest` trait
// and the `export!` macro in the current scope. We implement the trait.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}

// Step 3: We call the `export!` macro to link our implementation.
export!(MyWorker);
