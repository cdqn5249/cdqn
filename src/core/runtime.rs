// Path: src/core/runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! Lightweight core runtime for CDQN
//!
//! This module provides a compact, dependency-light async runtime abstraction
//! tailored to the CDQN design goals:
//!  - No Tokio, minimal footprint
//!  - Deterministic scheduling via a small executor
//!  - Simple task spawn + graceful shutdown primitives
//!  - Non-blocking channels for inter-task communication
//!
//! The `Runtime` type here is intentionally small and portable. It is meant
//! to be the base runtime used by Chronosa instances and core modules.
//!
//! NOTE: This file is `core/runtime.rs` (part of the `core` module). If the
//! crate also exposes a root `runtime` module, re-export `Runtime` there as
//! appropriate (e.g. `pub use core::runtime::Runtime;`) so other modules can
//! refer to `crate::runtime::Runtime`.

use async_channel::{bounded, unbounded, Receiver, Sender};
use async_executor::Executor;
use futures_lite::future;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, info};

/// Errors produced by the runtime API.
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("task spawn failed: {0}")]
    SpawnError(String),
    #[error("channel send failed: {0}")]
    ChannelSend(String),
}

/// Runtime stop signal message.
#[derive(Debug, Clone)]
pub enum RuntimeSignal {
    Shutdown, // Graceful shutdown
}

/// A lightweight runtime handle.
///
/// - `executor` runs tasks
/// - `signal_tx` allows sending a shutdown signal to workers
#[derive(Clone)]
pub struct Runtime {
    executor: Arc<Executor<'static>>,
    signal_tx: Sender<RuntimeSignal>,
    signal_rx: Receiver<RuntimeSignal>,
}

impl Runtime {
    /// Create a new Runtime instance.
    pub fn new() -> Self {
        let executor = Arc::new(Executor::new());
        let (signal_tx, signal_rx) = bounded::<RuntimeSignal>(1);
        Self {
            executor,
            signal_tx,
            signal_rx,
        }
    }

    /// Spawn a `'static` async task on the runtime.
    ///
    /// The spawned task is detached and will run until completion or the
    /// process exits. Use channels and signals for graceful coordination.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.executor.spawn(fut).detach();
    }

    /// Returns a cloneable sender which can be used to request a shutdown.
    pub fn signal_sender(&self) -> Sender<RuntimeSignal> {
        self.signal_tx.clone()
    }

    /// Returns a receiver that worker loops can use to observe shutdown signals.
    pub fn signal_receiver(&self) -> Receiver<RuntimeSignal> {
        self.signal_rx.clone()
    }

    /// Block the current thread until the provided future completes using the executor.
    ///
    /// Convenient helper used by tests and simple binaries.
    pub fn block_on<F, T>(&self, fut: F) -> T
    where
        F: Future<Output = T>,
    {
        future::block_on(self.executor.run(fut))
    }

    /// Request a graceful shutdown and wait up to `timeout` for cooperative tasks.
    ///
    /// This sends a `Shutdown` signal on the runtime's internal channel and then
    /// sleeps for the given timeout to allow running tasks to exit gracefully.
    pub async fn shutdown_graceful(&self, timeout: Duration) {
        // Best-effort send; ignore send errors (receiver may be dropped)
        let _ = self.signal_tx.send(RuntimeSignal::Shutdown).await;
        // Allow tasks to observe and wind down
        // We use a simple sleep here; more complex coordination (join handles)
        // can be implemented on top of this primitive.
        futures_timer::Delay::new(timeout).await;
        info!("Runtime graceful shutdown requested (waited {:?}).", timeout);
    }
}

/// Small helper: spawn a periodic task which checks for shutdown signals.
///
/// Example usage: monitoring, housekeeping, or cooperative cancellation loops.
pub fn spawn_signal_watch<T, F>(runtime: &Runtime, mut work: F)
where
    T: Send + 'static,
    F: FnMut() -> T + Send + 'static,
    T: Future<Output = ()> + Send + 'static,
{
    let rx = runtime.signal_receiver();
    let ex = runtime.clone();
    ex.spawn(async move {
        loop {
            // Check for shutdown without blocking: try_recv is non-blocking.
            match rx.try_recv() {
                Ok(RuntimeSignal::Shutdown) | Err(async_channel::TryRecvError::Closed) => {
                    debug!("Signal watch observed shutdown; exiting watch loop.");
                    break;
                }
                Err(async_channel::TryRecvError::Empty) => {
                    // Do work
                    work().await;
                    // Yield to allow other tasks to run
                    futures_lite::future::yield_now().await;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn runtime_spawn_and_block_on() {
        let rt = Runtime::new();

        // spawn a short-lived task that sets a flag via a channel
        let (tx, rx) = unbounded::<bool>();
        rt.spawn(async move {
            let _ = tx.send(true).await;
        });

        // block on receiving the message
        rt.block_on(async {
            let v = rx.recv().await.unwrap();
            assert!(v);
        });
    }

    #[test]
    fn graceful_shutdown_signal() {
        let rt = Runtime::new();
        let sender = rt.signal_sender();
        rt.spawn(async move {
            // request shutdown after a short delay
            futures_timer::Delay::new(Duration::from_millis(10)).await;
            let _ = sender.send(RuntimeSignal::Shutdown).await;
        });

        // wait a bit to let the spawned task run
        rt.block_on(async {
            futures_timer::Delay::new(Duration::from_millis(20)).await;
        });
    }
}
