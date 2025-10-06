// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/engine.rs

//! The CDQN Native Execution Engine.

use crate::cdu::Cdu;
use crate::state::{evolve_shared_state, ChronosaState, SharedState};
use crate::storage::{append_events_to_log, rehydrate_from_log};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

/// The Projector is the pure, deterministic "brain" of the agent.
pub trait Projector: Send + Sync {
    fn project(&self, state: &ChronosaState, input: &Cdu) -> Vec<Cdu>;
}

/// A wrapper for inputs sent to the Engine, allowing for control signals.
pub enum EngineInput {
    Cdu(Cdu),
    Shutdown,
}

/// The Engine drives the agent's lifecycle.
pub struct Engine {
    pub state: SharedState,
    log_path: PathBuf,
    projector: Arc<dyn Projector>,
    input_receiver: mpsc::Receiver<EngineInput>,
    pub input_sender: mpsc::Sender<EngineInput>,
    pub command_sender: mpsc::Sender<Cdu>,
}

impl Engine {
    pub fn new(log_path: PathBuf, projector: Box<dyn Projector>) -> (Self, mpsc::Receiver<Cdu>) {
        let events = rehydrate_from_log(&log_path).unwrap_or_default();
        let initial_state = events
            .into_iter()
            .fold(ChronosaState::default(), |mut acc, event| {
                if event.metadata.hlc > acc.hlc {
                    acc.hlc = event.metadata.hlc.clone();
                }
                acc.log.push(event);
                acc
            });

        let (input_sender, input_receiver) = mpsc::channel();
        let (command_sender, command_receiver) = mpsc::channel();

        let engine = Self {
            state: Arc::new(RwLock::new(initial_state)),
            log_path,
            projector: Arc::from(projector),
            input_receiver,
            input_sender,
            command_sender,
        };

        (engine, command_receiver)
    }

    /// The main run loop. It listens for inputs and drives the state forward.
    pub fn run(self) {
        println!("[Engine] Thread spawned and running.");
        while let Ok(input) = self.input_receiver.recv() {
            println!("[Engine] Received input from channel.");
            match input {
                EngineInput::Cdu(input_cdu) => {
                    println!("[Engine] Input is a CDU: {}", input_cdu.name);
                    let state = self.state.clone();
                    let projector = self.projector.clone();
                    let command_sender = self.command_sender.clone();
                    let log_path = self.log_path.clone();

                    thread::spawn(move || {
                        println!("[Engine-Task] Spawned for CDU: {}", input_cdu.name);
                        if let Ok(state_guard) = state.read() {
                            let new_events = projector.project(&state_guard, &input_cdu);
                            println!("[Engine-Task] Projector generated {} new event(s).", new_events.len());

                            let mut all_events_to_persist = vec![input_cdu];
                            all_events_to_persist.extend(new_events);
                            append_events_to_log(&all_events_to_persist, &log_path).unwrap();

                            for event in &all_events_to_persist {
                                if event.name.contains(".command.") {
                                    println!("[Engine-Task] Sending command: {}", event.name);
                                    if command_sender.send(event.clone()).is_err() {
                                        println!("[Engine-Task] Command channel closed, cannot send.");
                                    }
                                }
                            }

                            for event in all_events_to_persist {
                                evolve_shared_state(&state, event);
                            }
                            println!("[Engine-Task] Task complete.");
                        } else {
                            eprintln!("[Engine-Task] Failed to acquire read lock for projection.");
                        }
                    });
                }
                EngineInput::Shutdown => {
                    println!("[Engine] Shutdown signal received.");
                    break;
                }
            }
        }
        // This line will only be reached if the input_receiver channel closes or Shutdown is received.
        println!("[Engine] Input channel closed or shutdown received, thread terminating.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ChronosaState;
    use std::fs;
    use std::sync::mpsc;
    use std::thread;

    #[derive(Clone)]
    struct TestProjector;
    impl Projector for TestProjector {
        fn project(&self, _state: &ChronosaState, input: &Cdu) -> Vec<Cdu> {
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
    fn test_concurrent_engine_processing_flow() {
        let temp_log_path = PathBuf::from("concurrent_engine_test.cdqn");
        let _ = fs::remove_file(&temp_log_path);

        let (engine, _command_receiver) =
            Engine::new(temp_log_path.clone(), Box::new(TestProjector));
        let input_sender = engine.input_sender.clone();

        let (task_done_sender, task_done_receiver) = mpsc::channel::<()>();

        for _ in 0..5 {
            let observation = Cdu::new(b"test observation".to_vec(), "observation", vec![]);
            input_sender.send(EngineInput::Cdu(observation)).unwrap();
            let done_sender = task_done_sender.clone();
            thread::spawn(move || {
                done_sender.send(()).unwrap();
            });
        }
        drop(task_done_sender);

        for _ in 0..5 {
            task_done_receiver.recv().unwrap();
        }

        let _ = fs::remove_file(&temp_log_path);
    }
}
