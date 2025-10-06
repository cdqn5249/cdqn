// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/executor.rs

//! The Executor for running non-deterministic tasks.

use crate::cdu::Cdu;
use crate::engine::EngineInput;
use std::sync::mpsc;
use std::thread;

/// The Executor runs on a background thread, processing commands.
pub struct Executor;

impl Executor {
    pub fn spawn(
        command_receiver: mpsc::Receiver<Cdu>,
        result_sender: mpsc::Sender<EngineInput>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("[Executor] Thread spawned and running.");
            while let Ok(command_cdu) = command_receiver.recv() {
                println!("[Executor] Received command: {}", command_cdu.name);

                let result_cdu = Cdu::new(
                    b"Task completed successfully".to_vec(),
                    "result.task_completed",
                    vec![command_cdu.name],
                );

                println!("[Executor] Sending result for command.");
                if result_sender.send(EngineInput::Cdu(result_cdu)).is_err() {
                    println!(
                        "[Executor] Engine channel closed, cannot send result. Shutting down."
                    );
                    break;
                }
            }
            println!("[Executor] Command channel closed, thread terminating.");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_executor_receives_and_sends() {
        let (command_sender, command_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let handle = Executor::spawn(command_receiver, result_sender);

        let command = Cdu::new(b"do work".to_vec(), "command.work", vec![]);
        command_sender.send(command.clone()).unwrap();

        // Give a moment for the result to be sent back.
        let result_input = result_receiver.recv_timeout(Duration::from_secs(1)).unwrap();
        let result = match result_input {
            EngineInput::Cdu(cdu) => cdu,
            _ => panic!("Expected a CDU from executor"),
        };

        assert!(result.name.contains(".result.task_completed"));
        assert_eq!(result.metadata.causes.len(), 1);
        assert_eq!(result.metadata.causes[0], command.name);

        drop(command_sender);
        handle.join().unwrap();
    }
}
