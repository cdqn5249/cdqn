// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/main.rs

use cdqn::cdu::Cdu;
use cdqn::engine::{Engine, Projector};
use cdqn::executor::Executor;
use cdqn::state::ChronosaState;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

/// A simple, rule-based Projector for our demo.
/// This is the "brain" of our agent for this simulation.
struct SimpleProjector;
impl Projector for SimpleProjector {
    fn project(&self, state: &ChronosaState, input: &Cdu) -> Vec<Cdu> {
        // If the input is an observation of "see enemy",
        // and we haven't already decided to attack, create a command to attack.
        if input.name.contains(".observation.") && input.payload == b"see enemy" {
            if state
                .find_last_by_subtype("command.schedule_task")
                .is_none()
            {
                println!("Projector: Saw enemy, creating attack command.");
                return vec![Cdu::new(
                    b"{\"task\":\"attack\"}".to_vec(),
                    "command.schedule_task",
                    vec![input.name.clone()],
                )];
            }
        }
        // Otherwise, produce no new events.
        vec![]
    }
}

fn main() {
    println!("--- CDQN Native Engine Demo ---");
    let log_path = PathBuf::from("native_engine.cdqn");
    let _ = std::fs::remove_file(&log_path); // Clean up from previous runs

    // 1. Create the Engine with our application's logic (the Projector).
    //    This also gives us the receiver for commands that the Executor will use.
    let (engine, command_receiver) = Engine::new(log_path, Box::new(SimpleProjector));

    // 2. The Engine's input sender is our handle to the outside world.
    //    We clone it to send results from the executor and inputs from the client.
    let input_sender = engine.input_sender.clone();

    // 3. Spawn the Executor on a background thread.
    let executor_handle = Executor::spawn(command_receiver, input_sender.clone());

    // 4. Spawn the Engine on its own background thread.
    let engine_handle = thread::spawn(move || engine.run());

    // 5. The main thread is now the "client", feeding inputs to the Engine.
    println!("Client: Simulating an observation...");
    let observation = Cdu::new(b"see enemy".to_vec(), "observation", vec![]);
    input_sender.send(observation).unwrap();

    // Give the system a moment to process the event and the resulting command.
    thread::sleep(Duration::from_millis(100));

    // 6. Gracefully shut down the system by dropping the sender.
    //    This will cause the Engine's loop to exit, which in turn will
    //    cause the Executor's loop to exit.
    println!("Client: Shutting down.");
    drop(input_sender);

    // 7. Wait for the threads to finish cleanly.
    engine_handle.join().unwrap();
    executor_handle.join().unwrap();

    println!("\nSession complete.");
}
