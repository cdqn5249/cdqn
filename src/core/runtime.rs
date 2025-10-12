// Path: src/core/runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)
//
//! Core runtime abstraction for CDQN ecosystem.
//! This module defines a minimal async runtime built on `async-executor`
//! and `futures-lite`, avoiding heavy runtimes like Tokio or async-std.

use async_channel::{bounded, Receiver, Sender};
use async_executor::Executor;
use futures_lite::future;
use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use tracing::{debug, error};

/// Runtime-level errors for CDQN.
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("task spawn failed: {0}")]
    SpawnFailed(String),

    #[error("channel send failed: {0}")]
    ChannelSendFailed(String),
}

/// High-level runtime signal (Stop, Reload, Custom).
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
    /// Create a new runtime with bounded signal channels.
    pub fn new() -> Self {
        let executor = Arc::new(Executor::new());
        let (signal_tx, signal_rx) = bounded(64);
        Self {
            executor,
            signal_tx,
            signal_rx,
        }
    }

    /// Spawn a background async task.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let ex = self.executor.clone();
        ex.spawn(fut).detach();
    }

    /// Send a runtime signal (Stop, Reload, etc.).
    pub async fn send_signal(&self, signal: RuntimeSignal) -> Result<(), RuntimeError> {
        self.signal_tx
            .send(signal)
            .await
            .map_err(|e| RuntimeError::ChannelSendFailed(e.to_string()))
    }

    /// Run the executor for a fixed duration (non-blocking tick loop).
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

    /// Wait for the next runtime signal, with a timeout.
    pub async fn next_signal(&self, timeout: Duration) -> Option<RuntimeSignal> {
        let recv_future = self.signal_rx.recv();
        let timed_future = async {
            let start = Instant::now();
            loop {
                if Instant::now().duration_since(start) >= timeout {
                    return None;
                }
                if let Ok(sig) = recv_future.or(async { Err(async_channel::RecvError) }).await {
                    return Some(sig);
                }
                future::yield_now().await;
            }
        };
        timed_future.await
    }
}

/// Helper for spawning a signal watcher that reacts to runtime events.
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
    fn runtime_spawn_and_signal_cycle() {
        let rt = Runtime::new();
        rt.spawn(async { debug!("runtime test task running") });
        rt.run_for(Duration::from_millis(20));
    }

    #[test]
    fn signal_flow_test() {
        let rt = Runtime::new();
        let rt2 = rt.clone();
        rt.spawn(async move {
            rt2.send_signal(RuntimeSignal::Custom("ping".into()))
                .await
                .unwrap();
        });

        rt.run_for(Duration::from_millis(30));
    }
}
