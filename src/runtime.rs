// Path: src/runtime.rs
// Original source for CDQN ecosystem — Causal Data Query Nodes
// Generated and maintained by ChatGPT-5, OpenAI
// Licensed under BaDaaS (Build and Develop as a Service)

//! # CDQN Runtime Facade
//!
//! This module re-exports the lightweight async runtime implementation
//! from `core/runtime.rs` for use throughout the CDQN ecosystem.
//!
//! It serves as the public interface for the internal runtime core, providing:
//! - The [`Runtime`] type for spawning and managing tasks.
//! - The [`RuntimeSignal`] enum for graceful shutdown coordination.
//! - The [`spawn_signal_watch`] helper for cooperative task management.

pub use crate::core::runtime::{spawn_signal_watch, Runtime, RuntimeError, RuntimeSignal};

/// Convenience helper to block the current thread on a future.
///
/// This function delegates to the global executor instance through [`Runtime`],
/// and is typically used in tests or single-node orchestration logic.
///
/// # Example
/// ```
/// use cdqn::runtime::block_on;
///
/// block_on(async {
///     println!("Hello from CDQN runtime!");
/// });
/// ```
pub fn block_on<F, T>(fut: F) -> T
where
    F: std::future::Future<Output = T>,
{
    futures_lite::future::block_on(fut)
}
