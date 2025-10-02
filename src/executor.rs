// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/executor.rs

//! The Executor for running non-deterministic tasks.

use crate::cdu::Cdu;
use std::sync::mpsc;
use std::thread;

/// The Executor runs on a background thread, processing commands.
pub struct Executor;

impl Executor {
    pub fn spawn(
        command_receiver: mpsc::Receiver<Cdu>,
        result_sender: mpsc::Sender<Cdu>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("Executor: Running.");
            while let Ok(command_cdu) = command_receiver.recv() {
                println!("Executor: Received command: {}", command_cdu.name);

                // In a real system, we would look up and run a real task.
                // For now, we simulate a successful task completion.
                let result_payload = b"Task completed successfully".to_vec();
                let result_cdu = Cdu::new(
                    result_payload,
                    "result.task_completed",
                    vec![command_cdu.name],
                );

                // Send the result back to the Engine.
                if result_sender.send(result_cdu).is_err() {
                    // The engine has shut down, so we can exit.
                    break;
                }
            }
            println!("Executor: Shutting down.");
        })
    }
}
