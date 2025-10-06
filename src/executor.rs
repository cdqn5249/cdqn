// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/executor.rs

//! The Executor for running non-deterministic tasks.

use crate::cdu::Cdu;
use crate::engine::EngineInput;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// The Executor runs on a background thread, processing commands.
pub struct Executor;

impl Executor {
    // The Executor's job is to execute commands, not send CDUs back into the main loop.
    // In a real system, it would interact with the outside world.
    pub fn spawn(command_receiver: mpsc::Receiver<Cdu>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            println!("[Executor] Thread spawned and running.");
            while let Ok(command_cdu) = command_receiver.recv() {
                println!("[Executor] Received command: {}", command_cdu.name);
                // In a real system, this is where you would perform side effects,
                // like making a network call or writing to a file.
                // For this demo, we just print that the task is "done".
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

    #[test]
    fn test_executor_receives_and_sends() {
        let (command_sender, command_receiver) = mpsc::channel();
        // We no longer need a result channel for this test.
        let handle = Executor::spawn(command_receiver);

        let command = Cdu::new(b"do work".to_vec(), "command.work", vec![]);
        command_sender.send(command.clone()).unwrap();

        // Give the thread a moment to process the command.
        thread::sleep(Duration::from_millis(50));

        drop(command_sender);
        handle.join().unwrap();
    }
}
