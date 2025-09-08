// This is the canonical pattern for a cargo-component guest.

// Step 1: Declare a module named `bindings`. The `cargo component build`
// command will automatically generate the code for this module in the
// background based on our WIT files.
mod bindings;

// Step 2: Now that the module is declared, we can `use` the items
// that the build tool will place inside it.
use bindings::Guest;

// We define a struct that will hold our component's implementation.
struct MyWorker;

// We implement the `Guest` trait, which is now correctly in scope.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}
