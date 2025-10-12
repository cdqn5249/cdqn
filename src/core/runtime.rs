// Path: src/core/runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)
//
//! Core runtime abstraction for CDQN ecosystem.
//! Lightweight async executor based on `async-executor` and `futures-lite`.
//! This runtime provides non-blocking scheduling, channel-based communication,
//! and signal handling for Chronosa and module orchestration.

use async_channel::{bounded, Receiver, Sender};
use async_executor::Executor;
use futures_lite::{future, prelude::*};
use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use tracing::{debug, error, info};

/// Runtime-level errors for CDQN.
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("task spawn failed: {0}")]
    SpawnFailed(String),

    #[error("channel send failed: {0}")]
    ChannelSendFailed(String),
}

/// High-level signal that can be sent to the runtime.
#[derive(Debug, Clone)]
pub enum RuntimeSignal {
    Stop,
    Reload,
    Custom(String),
}

/// Lightweight concurrent runtime for CDQN nodes.
#[derive(Clone)]
pub struct Runtime {
    executor: Arc<Executor<'static>>,
    signal_tx: Sender<RuntimeSignal>,
    signal_rx: Receiver<RuntimeSignal>,
}

impl Runtime {
    /// Creates a new runtime with its signal channels.
    pub fn new() -> Self {
        let executor = Arc::new(Executor::new());
        let (signal_tx, signal_rx) = bounded(64);

        Self {
            executor,
            signal_tx,
            signal_rx,
        }
    }

    /// Spawns an async task on the executor.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let ex = self.executor.clone();
        ex.spawn(fut).detach();
    }

    /// Sends a runtime signal (e.g. Stop, Reload).
    pub async fn send_signal(&self, signal: RuntimeSignal) -> Result<(), RuntimeError> {
        self.signal_tx
            .send(signal)
            .await
            .map_err(|e| RuntimeError::ChannelSendFailed(e.to_string()))
    }

    /// Runs the executor event loop for a given duration.
    pub fn run_for(&self, duration: Duration) {
        let ex = self.executor.clone();
        let until = Instant::now() + duration;

        future::block_on(async move {
            while Instant::now() < until {
                ex.try_tick();
                future::yield_now().await;
            }
        });
    }

    /// Waits for the next runtime signal asynchronously.
    pub async fn next_signal(&self, timeout: Duration) -> Option<RuntimeSignal> {
        let fut = self.signal_rx.recv();
        future::or(fut, async {
            let start = Instant::now();
            while Instant::now().duration_since(start) < timeout {
                future::yield_now().await;
            }
            None
        })
        .await
    }
}

/// Utility helper for signal watchers (Chronosa use).
pub fn spawn_signal_watch<F>(runtime: &Runtime, mut handler: F)
where
    F: FnMut(RuntimeSignal) + Send + 'static,
{
    let r = runtime.clone();
    runtime.spawn(async move {
        while let Some(sig) = r.next_signal(Duration::from_secs(1)).await {
            debug!("Runtime received signal: {:?}", sig);
            handler(sig);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_spawn_and_signal_flow() {
        let rt = Runtime::new();
        rt.spawn(async { debug!("hello from task") });
        rt.run_for(Duration::from_millis(20));
    }

    #[test]
    fn signal_handling_cycle() {
        let rt = Runtime::new();
        let rt2 = rt.clone();
        rt.spawn(async move {
            rt2.send_signal(RuntimeSignal::Stop).await.unwrap();
        });

        rt.run_for(Duration::from_millis(10));
    }
}
