// By using cargo-component, we no longer need the wit-bindgen macro here.
// The build tool will automatically generate and link the necessary code
// based on the WIT files in our `wit` directory.

// We must define a module named after our WIT package (`local:hello-world`).
// The `use` statement brings the generated `Guest` trait into scope.
use crate::hello_world::Guest;

// We define a struct that will hold our component's implementation.
struct MyWorker;

// We implement the `Guest` trait, which cargo-component generates for us.
impl Guest for MyWorker {
    fn run() {
        println!("Hello from inside the WASM Component! The worker is running!");
    }
}

// We export our implementation using the special `export!` macro,
// which is also automatically provided by cargo-component.
export!(MyWorker);
