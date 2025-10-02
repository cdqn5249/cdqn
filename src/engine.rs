// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/engine.rs

//! The CDQN Native Execution Engine.

use crate::cdu::Cdu;
use crate::state::{evolve, ChronosaState};
use crate::storage::{append_events_to_log, rehydrate_from_log};
use std::path::PathBuf;
use std::sync::mpsc;

/// The Projector is the pure, deterministic "brain" of the agent.
pub trait Projector: Send {
    fn project(&self, state: &ChronosaState, input: &Cdu) -> Vec<Cdu>;
}

/// The Engine drives the agent's lifecycle.
pub struct Engine {
    state: ChronosaState,
    log_path: PathBuf,
    projector: Box<dyn Projector>,
    // The channel to receive all incoming CDUs.
    input_receiver: mpsc::Receiver<Cdu>,
    // A sender for the input channel, can be cloned and given to Executors.
    pub input_sender: mpsc::Sender<Cdu>,
    // A channel to broadcast command CDUs to Executors.
    pub command_sender: mpsc::Sender<Cdu>,
}

impl Engine {
    pub fn new(log_path: PathBuf, projector: Box<dyn Projector>) -> (Self, mpsc::Receiver<Cdu>) {
        let events = rehydrate_from_log(&log_path).unwrap_or_default();
        let initial_state = events.into_iter().fold(ChronosaState::default(), evolve);

        let (input_sender, input_receiver) = mpsc::channel();
        let (command_sender, command_receiver) = mpsc::channel();

        let engine = Self {
            state: initial_state,
            log_path,
            projector,
            input_receiver,
            input_sender,
            command_sender,
        };

        (engine, command_receiver)
    }

    /// The main run loop. It listens for inputs and drives the state forward.
    pub fn run(mut self) {
        println!("Engine: Running.");
        while let Ok(input_cdu) = self.input_receiver.recv() {
            // 1. Project the input against the current state to get new events.
            let new_events = self.projector.project(&self.state, &input_cdu);

            // 2. Persist the input and all new events to the WAL.
            let mut all_events_to_persist = vec![input_cdu];
            all_events_to_persist.extend(new_events);
            append_events_to_log(&all_events_to_persist, &self.log_path).unwrap();

            // 3. Evolve the in-memory state.
            for event in all_events_to_persist {
                // Send commands to the executor.
                if event.name.contains(".command.") {
                    self.command_sender.send(event.clone()).unwrap();
                }
                self.state = evolve(self.state, event);
            }
            println!(
                "Engine: Processed event. New state has {} CDUs.",
                self.state.len()
            );
        }
        println!("Engine: Shutting down.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ChronosaState;
    use std::fs;

    // A mock projector for testing purposes.
    struct TestProjector;
    impl Projector for TestProjector {
        fn project(&self, _state: &ChronosaState, input: &Cdu) -> Vec<Cdu> {
            // If the input is an observation, create a command.
            if input.name.contains(".observation.") {
                vec![Cdu::new(
                    b"test command".to_vec(),
                    "command.test",
                    vec![input.name.clone()],
                )]
            } else {
                vec![]
            }
        }
    }

    #[test]
    fn test_engine_processing_flow() {
        let temp_log_path = PathBuf::from("engine_test.cdqn");
        let _ = fs::remove_file(&temp_log_path);

        // 1. Create the engine and get the command receiver.
        let (mut engine, command_receiver) =
            Engine::new(temp_log_path.clone(), Box::new(TestProjector));
        let input_sender = engine.input_sender.clone();

        // 2. Send an input observation.
        let observation = Cdu::new(b"test observation".to_vec(), "observation", vec![]);
        input_sender.send(observation).unwrap();

        // 3. Manually run one cycle of the engine's loop logic.
        // We do this instead of spawning a thread to have deterministic control.
        if let Ok(input_cdu) = engine.input_receiver.recv() {
            let new_events = engine.projector.project(&engine.state, &input_cdu);
            let mut all_events = vec![input_cdu];
            all_events.extend(new_events);
            append_events_to_log(&all_events, &engine.log_path).unwrap();
            for event in all_events {
                if event.name.contains(".command.") {
                    engine.command_sender.send(event.clone()).unwrap();
                }
                engine.state = evolve(engine.state, event);
            }
        }

        // 4. Verify the state was updated correctly.
        assert_eq!(engine.state.len(), 2); // observation + command

        // 5. Verify the command was sent to the command channel.
        let command = command_receiver.try_recv().unwrap();
        assert!(command.name.contains(".command.test"));
        assert_eq!(command.payload, b"test command".to_vec());

        // 6. Clean up.
        let _ = fs::remove_file(&temp_log_path);
    }
}
