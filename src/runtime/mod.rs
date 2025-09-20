// src/runtime/mod.rs

use crate::kernel::KDU;

pub mod entity;
pub mod network;
pub mod orchestrator;
pub mod persistence;
pub mod processor;
pub mod test_entities;

/// Commands that can be sent to the Persistence thread.
#[derive(Debug)]
pub enum PersistenceCommand {
    // We Box the KDU to keep the enum's size small and efficient.
    WriteKdu(Box<KDU>),
    Shutdown,
}
