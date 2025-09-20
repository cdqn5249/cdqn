// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::processor::EntityProcessor;

/// The Orchestrator is the physical heart of the runtime.
pub struct Orchestrator {
    processor: EntityProcessor,
}

impl Orchestrator {
    pub fn new() -> Self {
        Orchestrator {
            processor: EntityProcessor::default(),
        }
    }

    pub fn processor_mut(&mut self) -> &mut EntityProcessor {
        &mut self.processor
    }

    pub fn route_initial_kdu(&self, target_fqei: &FQEI, kdu: KDU) {
        self.processor.route_local(target_fqei, kdu);
    }

    pub fn run(&mut self) {
        println!("[Orchestrator] Starting main loop...");
        for turn in 1..=3 {
            println!("\n--- Turn {} ---", turn);
            let outgoing_kuds = self.processor.run_turn();

            if outgoing_kuds.is_empty() {
                println!("[Orchestrator] System is quiet.");
            }

            for kdu in outgoing_kuds {
                // We get the target FQEI before we move the KDU.
                let target_fqei = kdu.originator_fqei.clone();
                println!(
                    "[Orchestrator] Routing KDU from {} to {}",
                    kdu.originator_fqei, target_fqei
                );
                // Now we can safely move the kdu.
                self.processor.route_local(&target_fqei, kdu);
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
