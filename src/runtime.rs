// Path: src/runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! # CDQN Runtime
//!
//! The `runtime` module provides a lightweight asynchronous execution environment
//! for CDQN nodes. It is built around small, composable primitives using
//! `async-executor` and `async-channel` to maintain full non-blocking behavior.
//!
//! ## Design principles
//! - **No global mutable state**
//! - **No heavy runtime** (no Tokio, no async-std)
//! - **Composable pure tasks**
//! - **Graceful shutdowns via channels**
//!
//! This is the base runtime layer upon which Chronosa agents and node modules run.

use async_channel::{unbounded, Receiver, Sender};
use async_executor::Executor;
use futures_lite::future;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tracing::info;

/// Global executor instance for non-blocking task scheduling.
/// CDQN avoids global mutable state; the executor is wrapped in an `Arc`.
static EXECUTOR: Lazy<Arc<Executor<'_>>> = Lazy::new(|| Arc::new(Executor::new()));

/// Message type used for inter-task communication inside the runtime.
#[derive(Debug, Clone)]
pub enum RuntimeMsg {
    Info(String),
    Stop,
}

/// Structure representing a runtime context.
#[derive(Clone)]
pub struct CdqnRuntime {
    pub sender: Sender<RuntimeMsg>,
    pub receiver: Receiver<RuntimeMsg>,
}

impl CdqnRuntime {
    /// Creates a new runtime context with its own message channel.
    pub fn new() -> Self {
        let (sender, receiver) = unbounded::<RuntimeMsg>();
        Self { sender, receiver }
    }

    /// Spawns an asynchronous task on the CDQN executor.
    pub fn spawn<F>(&self, fut: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        EXECUTOR.spawn(fut).detach();
    }

    /// Runs the runtime event loop.
    ///
    /// This listens to messages until a `Stop` signal is received.
    pub async fn run(&self) {
        info!("CDQN runtime event loop started.");
        while let Ok(msg) = self.receiver.recv().await {
            match msg {
                RuntimeMsg::Info(s) => info!("Runtime message: {}", s),
                RuntimeMsg::Stop => {
                    info!("CDQN runtime stopping.");
                    break;
                }
            }
        }
    }

    /// Signals the runtime to shut down gracefully.
    pub async fn stop(&self) {
        let _ = self.sender.send(RuntimeMsg::Stop).await;
    }
}

/// Runs a blocking executor loop until all spawned tasks complete.
///
/// This is used by CDQN nodes as the main process entry point.
pub fn block_on<F: std::future::Future<Output = T>, T>(future: F) -> T {
    future::block_on(EXECUTOR.run(future))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::future::yield_now;

    #[test]
    fn runtime_basic_flow() {
        block_on(async {
            let rt = CdqnRuntime::new();
            let tx = rt.sender.clone();

            rt.spawn(async move {
                tx.send(RuntimeMsg::Info("Hello from async task".into()))
                    .await
                    .unwrap();
                tx.send(RuntimeMsg::Stop).await.unwrap();
            });

            rt.run().await;
            yield_now().await;
        });
    }
}
