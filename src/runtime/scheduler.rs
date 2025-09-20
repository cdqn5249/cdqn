// src/runtime/scheduler.rs

// We are allowing this unused import because the next milestone will use it.
#[allow(unused_imports)]
use crate::kernel::KDU;
use std::collections::HashMap;

/// The EntityScheduler is the heart of the runtime's event loop.
/// It is responsible for delivering KDUs to entities and executing their behaviors.
pub struct EntityScheduler {
    // We are allowing this dead code because the next milestone will use it.
    #[allow(dead_code)]
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
