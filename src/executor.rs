// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/executor.rs

//! The Executor for running non-deterministic tasks.

use crate::cdu::Cdu;
use crate::engine::EngineInput; // Import the new enum
use std::sync::mpsc;
use std::thread;

/// The Executor runs on a background thread, processing commands.
pub struct Executor;

impl Executor {
    pub fn spawn(
        command_receiver: mpsc::Receiver<Cdu>,
        result_sender: mpsc::Sender<EngineInput>, // Sender type is updated
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

                // Send the result back to the Engine, wrapped in the enum.
                if result_sender.send(EngineInput::Cdu(result_cdu)).is_err() {
                    // The engine has shut down, so we can exit.
                    break;
                }
            }
            println!("Executor: Shutting down.");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_receives_and_sends() {
        let (command_sender, command_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        // 1. Spawn the executor.
        let handle = Executor::spawn(command_receiver, result_sender);

        // 2. Send a command to the executor.
        let command = Cdu::new(b"do work".to_vec(), "command.work", vec![]);
        command_sender.send(command.clone()).unwrap();

        // 3. Wait for the result from the executor.
        let result_input = result_receiver.recv().unwrap();
        let result = match result_input {
            EngineInput::Cdu(cdu) => cdu,
            _ => panic!("Expected a CDU from executor"),
        };

        // 4. Verify the result is correct and causally linked.
        assert!(result.name.contains(".result.task_completed"));
        assert_eq!(result.metadata.causes.len(), 1);
        assert_eq!(result.metadata.causes[0], command.name);

        // 5. Shut down the executor by dropping the sender.
        drop(command_sender);
        handle.join().unwrap();
    }
}
