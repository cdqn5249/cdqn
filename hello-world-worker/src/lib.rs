// This is the canonical, two-step process for including bindings.

// Step 1: Use the `bindgen!` macro to specify the world we are implementing.
// This macro tells `cargo-component` what bindings to generate *before*
// the main compilation starts.
cargo_component_bindings::bindgen!({
    world: "hello",
    path: "wit",
});

// Step 2: Now that the bindings have been generated, we can `use` them.
// The generated code is placed in a module named after our world.
use hello::Guest;

// We define a struct that will hold our component's implementation.
struct MyWorker;

// We implement the `Guest` trait, which is now correctly in scope.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}
