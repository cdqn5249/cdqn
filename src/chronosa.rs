// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/chronosa.rs

use crate::cdu::Cdu;
use crate::janitor::{spawn_janitor, JanitorHandle};
use crate::state::{evolve, ChronosaState};
use crate::storage::{append_events_to_log, rehydrate_from_log, sync_log_to_disk};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Chronosa {
    state: ChronosaState,
    log_path: PathBuf,
    is_dirty: Arc<AtomicBool>,
    janitor: JanitorHandle,
}

pub enum Command {
    RecordObservation { payload: Vec<u8> },
    Commit,
}

impl Chronosa {
    /// Creates a new Chronosa instance, spawning a Janitor for background saves.
    pub fn new(log_path: PathBuf) -> Self {
        let events = rehydrate_from_log(&log_path).unwrap_or_default();
        let initial_state = events.into_iter().fold(ChronosaState::default(), evolve);
        let is_dirty = Arc::new(AtomicBool::new(false));
        // For demonstration, we use a short interval. In production, this would be minutes.
        let janitor = spawn_janitor(log_path.clone(), is_dirty.clone(), Duration::from_secs(300));

        Self {
            state: initial_state,
            log_path,
            is_dirty,
            janitor,
        }
    }

    /// The Command Handler. A pure function that produces events.
    fn handle(&self, command: Command) -> Vec<Cdu> {
        match command {
            Command::RecordObservation { payload } => {
                let mut hlc = self.state.hlc.clone();
                hlc.tick();
                let mut event = Cdu::new(payload, "observation", vec![]);
                event.metadata.hlc = hlc;
                vec![event]
            }
            Command::Commit => vec![],
        }
    }

    /// Processes a command, marking the log as dirty and handling explicit commits.
    pub fn process_command(&mut self, command: Command) {
        if let Command::Commit = command {
            if self.is_dirty.swap(false, Ordering::AcqRel) {
                println!("  -> User ordered Commit, committing to disk...");
                sync_log_to_disk(&self.log_path).unwrap();
            }
            return;
        }

        let events = self.handle(command);
        if events.is_empty() {
            return;
        }

        append_events_to_log(&events, &self.log_path).unwrap();
        self.is_dirty.store(true, Ordering::Relaxed); // Mark log as dirty

        for event in events {
            self.state = evolve(self.state.clone(), event);
        }
    }

    /// Gracefully shuts down the Janitor thread.
    pub fn shutdown(self) {
        // Before shutting down, perform one final commit if needed.
        if self.is_dirty.load(Ordering::Relaxed) {
            println!("  -> Committing dirty log on shutdown...");
            sync_log_to_disk(&self.log_path).unwrap();
        }
        self.janitor.shutdown();
    }

    /// Provides read-only access to the current state.
    pub fn state(&self) -> &ChronosaState {
        &self.state
    }
}
