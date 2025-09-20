// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::processor::EntityProcessor;

/// The Orchestrator is the physical heart of the runtime.
/// It manages the main event loop and all I/O services.
pub struct Orchestrator {
    processor: EntityProcessor,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            processor: EntityProcessor::default(),
        }
    }

    /// A helper function to directly access the processor for registration.
    pub fn processor_mut(&mut self) -> &mut EntityProcessor {
        &mut self.processor
    }

    /// A helper function to route the very first KDU to start a simulation.
    pub fn route_initial_kdu(&self, target_fqei: &FQEI, kdu: KDU) {
        self.processor.route_local(target_fqei, kdu);
    }

    /// The main run loop for the orchestrator.
    pub fn run(&mut self) {
        println!("[Orchestrator] Starting main loop...");
        // For this test, we will run for a fixed number of turns.
        for turn in 1..=3 {
            println!("\n--- Turn {} ---", turn);
            let outgoing_kuds = self.processor.run_turn();

            if outgoing_kuds.is_empty() {
                println!("[Orchestrator] System is quiet.");
            }

            for kdu in outgoing_kuds {
                // In the next phase, this is where the "switchboard" logic will go.
                // For now, we just route it back locally.
                println!(
                    "[Orchestrator] Routing KDU from {} to {}",
                    kdu.originator_fqei, kdu.originator_fqei
                );
                self.processor.route_local(&kdu.originator_fqei, kdu);
            }
        }
        println!("\n[Orchestrator] Simulation finished.");
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}
