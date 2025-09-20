// src/runtime/orchestrator.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::processor::EntityProcessor;

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
            // The processor now returns a list of (Target, KDU) tuples.
            let outgoing_routes = self.processor.run_turn();

            if outgoing_routes.is_empty() {
                println!("[Orchestrator] System is quiet.");
            }

            // We now have the correct target FQEI for each KDU.
            for (target_fqei, kdu) in outgoing_routes {
                println!(
                    "[Orchestrator] Routing KDU from {} to {}",
                    kdu.originator_fqei, target_fqei
                );
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
