// This macro tells wit-bindgen to generate all the necessary "glue" code.
wit_bindgen::generate!({
    path: "wit/worker.wit",
    world: "local:hello-world/hello",
});

// We define a struct that will hold our component's implementation.
struct MyWorker;

// The `generate!` macro creates a trait called `Guest` in the current scope.
// We implement it directly.
impl Guest for MyWorker {
    fn run() {
        // This code will execute inside the sandbox.
        println!("Hello from inside the WASM sandbox! The worker is running!");
    }
}

// The `generate!` macro also creates the `export!` macro in the current scope.
// We call it to link our implementation.
export!(MyWorker);
