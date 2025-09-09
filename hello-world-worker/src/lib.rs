// This is the canonical pattern for a cargo-component guest.

// Step 1: Declare the `bindings` module that `cargo-component` will generate.
mod bindings;

// Step 2: Use the items that the build tool will place inside the module.
// We need both the `Guest` trait to implement and the `export!` macro to link.
use bindings::{Guest, export};

// We define a struct that will hold our component's implementation.
struct MyWorker;

// Step 3: We implement the `Guest` trait, providing the logic for our functions.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}

// Step 4: This is the crucial, missing line. We call the `export!` macro
// to officially link our `MyWorker` struct as the implementation of the
// component's exports.
export!(MyWorker);
