// src/runtime/scheduler.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::entity::{Entity, Mailbox};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// A simple, concrete representation of a running entity.
struct EntityInstance {
    // A function pointer to the entity's behavior function.
    // We use Box<dyn ...> to handle different state types.
    behavior_fn: Box<dyn Fn(&dyn std::any::Any, KDU) -> (Box<dyn std::any::Any + Send>, Vec<KDU>) + Send>,
    state: Box<dyn std::any::Any + Send>,
    mailbox: Mailbox,
}

/// The EntityScheduler is the heart of the runtime's event loop.
pub struct EntityScheduler {
    entities: HashMap<FQEI, EntityInstance>,
}

impl EntityScheduler {
    pub fn new() -> Self {
        EntityScheduler {
            entities: HashMap::new(),
        }
    }

    /// Registers a new entity with the scheduler.
    pub fn register<E>(&mut self, fqei: FQEI, initial_state: E::State)
    where
        E: Entity + Send + 'static,
        E::State: Send + 'static,
    {
        let mailbox = Arc::new(Mutex::new(VecDeque::new()));
        let behavior_fn = |state: &dyn std::any::Any, message: KDU| {
            let new_state_tuple = E::behavior(state.downcast_ref::<E::State>().unwrap(), message);
            (Box::new(new_state_tuple.0), new_state_tuple.1)
        };

        let instance = EntityInstance {
            behavior_fn: Box::new(behavior_fn),
            state: Box::new(initial_state),
            mailbox: mailbox.clone(),
        };

        self.entities.insert(fqei, instance);
    }

    /// Delivers a KDU to the mailbox of a target entity.
    pub fn route(&self, target_fqei: &FQEI, kdu: KDU) {
        if let Some(entity) = self.entities.get(target_fqei) {
            entity.mailbox.lock().unwrap().push_back(kdu);
        } else {
            println!("[EntityScheduler] WARN: No entity found for FQEI: {}", target_fqei);
        }
    }

    /// The main event loop. This function runs one "turn" of the simulation.
    pub fn run_turn(&mut self) {
        let mut outgoing_kuds: Vec<(FQEI, KDU)> = Vec::new();
        let fqei_list: Vec<FQEI> = self.entities.keys().cloned().collect();

        // --- 1. Dequeue and Execute ---
        for fqei in fqei_list {
            let mut entity = self.entities.get_mut(&fqei).unwrap();
            let maybe_message = entity.mailbox.lock().unwrap().pop_front();
            if let Some(message) = maybe_message {
                println!("\n[EntityScheduler] Executing behavior for {}", fqei);
                let originator_fqei = message.originator_fqei.clone();
                let (new_state, new_kuds) = (entity.behavior_fn)(&*entity.state, message);
                entity.state = new_state;

                for kdu in new_kuds {
                    let target_fqei = originator_fqei.clone();
                    outgoing_kuds.push((target_fqei, kdu));
                }
            }
        }

        // --- 2. Route Outgoing Messages ---
        for (target_fqei, kdu) in outgoing_kuds {
            println!("[EntityScheduler] Routing KDU from {} to {}", kdu.originator_fqei, target_fqei);
            self.route(&target_fqei, kdu);
        }
    }
}

impl Default for EntityScheduler {
    fn default() -> Self {
        Self::new()
    }
}
