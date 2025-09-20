// src/runtime/scheduler.rs

use crate::kernel::KDU;
use std::collections::HashMap;

/// The EntityScheduler is the heart of the runtime's event loop.
/// It is responsible for delivering KDUs to entities and executing their behaviors.
pub struct EntityScheduler {
    // We will need a way to store and look up our entities and their mailboxes.
    // A HashMap from an entity's FQEI (String) to its Mailbox is a good start.
    mailboxes: HashMap<String, super::entity::Mailbox>,
}

impl EntityScheduler {
    /// Creates a new, empty scheduler.
    pub fn new() -> Self {
        EntityScheduler {
            mailboxes: HashMap::new(),
        }
    }

    /// The main event loop. This function will run forever, processing messages.
    pub fn run(&mut self) {
        println!("[EntityScheduler] Starting main event loop...");
        loop {
            // In the next milestone, we will implement the logic here to:
            // 1. Check all mailboxes for new messages.
            // 2. Execute the entity's behavior for one message.
            // 3. Route the resulting KDUs to their destination mailboxes.

            // For now, we just prevent an infinite busy-loop.
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

// Implement the Default trait as suggested by clippy.
impl Default for EntityScheduler {
    fn default() -> Self {
        Self::new()
    }
}
