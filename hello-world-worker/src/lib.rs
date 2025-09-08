// This is the canonical way to generate and include the bindings.
// This single macro call creates a `bindings` module in our crate.
cargo_component_bindings::generate!();

// Now, we can `use` the generated `Guest` trait from the `bindings` module.
use bindings::Guest;

// We define a struct that will hold our component's implementation.
struct MyWorker;

// We implement the `Guest` trait, which is now correctly in scope.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}
