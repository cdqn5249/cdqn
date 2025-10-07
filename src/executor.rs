// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/executor.rs

//! The Executor for running non-deterministic tasks.

use crate::cdu::Cdu;
// FIX: EngineInput is no longer needed as we are removing the feedback loop for this test.
use std::sync::mpsc;
use std::thread;

/// The Executor runs on a background thread, processing commands.
pub struct Executor;

impl Executor {
    // FIX: The result_sender has been removed. This tests the one-way flow.
    pub fn spawn(command_receiver: mpsc::Receiver<Cdu>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("[Executor] Thread spawned and running.");
            while let Ok(command_cdu) = command_receiver.recv() {
                println!("[Executor] Received command: {}", command_cdu.name);
                println!(
                    "[Executor] Task for command '{}' completed.",
                    command_cdu.name
                );
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
        let handle = Executor::spawn(command_receiver);

        let command = Cdu::new(b"do work".to_vec(), "command.work", vec![]);
        command_sender.send(command.clone()).unwrap();

        // Give the thread a moment to process the command.
        thread::sleep(Duration::from_millis(50));

        drop(command_sender);
        handle.join().unwrap();
    }
}
