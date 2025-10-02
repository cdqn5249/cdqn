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
            println!("Engine: Processed event. New state has {} CDUs.", self.state.len());
        }
        println!("Engine: Shutting down.");
    }
}
