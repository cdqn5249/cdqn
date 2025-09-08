// This macro tells wit-bindgen to look at our .wit file and generate
// all the necessary "glue" code to make this a valid WASM component.
wit_bindgen::generate!({
    // We point it to the .wit file we just created.
    path: "wit/worker.wit",

    // We are generating code for the component itself (the "guest").
    // The world path now correctly points to the `world hello`.
    world: "local:hello-world/hello",
});

// We define a struct that will hold our component's implementation.
struct MyWorker;

// This is the crucial part. Because our world exports the `worker` interface,
// wit-bindgen generates a `Worker` trait for us to implement.
impl Worker for MyWorker {
    fn run() {
        // When the host runtime calls the `run` function on this component,
        // this is the code that will execute inside the sandbox.
        println!("Hello from inside the WASM sandbox! The worker is running!");
    }
}

// Finally, we tell wit-bindgen that our `MyWorker` struct provides the
// implementation for the exported functions defined in our world.
export!(MyWorker);
