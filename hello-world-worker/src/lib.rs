// This is the canonical pattern for a cargo-component guest.

// Step 1: Declare the `bindings` module that `cargo-component` will generate.
mod bindings;

// Step 2: Use the `Guest` trait from the generated module.
use bindings::Guest;

// We define a struct that will hold our component's implementation.
struct MyWorker;

// Step 3: We implement the `Guest` trait. `cargo-component` will
// automatically detect this implementation and handle the exports.
// No `export!` macro is needed.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}
