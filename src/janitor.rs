// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/janitor.rs

//! The Janitor module for background, periodic disk synchronization.

use crate::storage::sync_log_to_disk;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// A handle to the running Janitor thread.
pub struct JanitorHandle {
    shutdown_signal: Arc<AtomicBool>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl JanitorHandle {
    /// Signals the Janitor to shut down and waits for it to finish.
    pub fn shutdown(mut self) {
        self.shutdown_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}

/// Spawns the Janitor thread.
pub fn spawn_janitor(
    log_path: PathBuf,
    is_dirty: Arc<AtomicBool>,
    interval: Duration,
) -> JanitorHandle {
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown_signal.clone();

    let thread_handle = thread::spawn(move || {
        println!("Janitor thread started. Interval: {:?}", interval);
        while !shutdown_clone.load(Ordering::Relaxed) {
            thread::sleep(interval);
            // If the dirty flag is set, sync the log to disk.
            if is_dirty.swap(false, Ordering::AcqRel) {
                println!("Janitor: Detected dirty log, committing to disk...");
                if let Err(e) = sync_log_to_disk(&log_path) {
                    eprintln!("Janitor: Failed to sync log to disk: {}", e);
                }
            }
        }
        println!("Janitor thread shut down.");
    });

    JanitorHandle {
        shutdown_signal,
        thread_handle: Some(thread_handle),
    }
}
