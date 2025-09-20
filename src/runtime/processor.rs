// src/runtime/processor.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::entity::{Entity, Mailbox};
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// --- TYPE ALIASES for clarity ---
type ErasedState = Box<dyn Any + Send>;
type OutgoingKdus = Vec<KDU>;
type BehaviorFn = Box<dyn Fn(&ErasedState, KDU) -> (ErasedState, OutgoingKdus) + Send>;

struct EntityInstance {
    behavior_fn: BehaviorFn,
    state: ErasedState,
    mailbox: Mailbox,
}

/// The EntityProcessor is the logical heart of the runtime.
/// It executes the entity behavior functions in a simple, turn-based loop.
pub struct EntityProcessor {
    entities: HashMap<FQEI, EntityInstance>,
    // Add this to src/runtime/processor.rs, inside impl EntityProcessor

    /// Returns a set of all FQEIs for entities registered locally.
    pub fn get_local_fqeis(&self) -> std::collections::HashSet<FQEI> {
        self.entities.keys().cloned().collect()
    }
}

impl EntityProcessor {
    pub fn new() -> Self {
        EntityProcessor {
            entities: HashMap::new(),
        }
    }

    pub fn register<E>(&mut self, fqei: FQEI, initial_state: E::State)
    where
        E: Entity + Send + 'static,
        E::State: Send + 'static,
    {
        let mailbox = Arc::new(Mutex::new(VecDeque::new()));
        let behavior_fn = move |state: &ErasedState, message: KDU| {
            let (new_specific_state, kuds) =
                E::behavior(state.as_ref().downcast_ref::<E::State>().unwrap(), message);
            let new_generic_state: ErasedState = Box::new(new_specific_state);
            (new_generic_state, kuds)
        };
        let instance = EntityInstance {
            behavior_fn: Box::new(behavior_fn),
            state: Box::new(initial_state),
            mailbox: mailbox.clone(),
        };
        self.entities.insert(fqei, instance);
    }

    /// Delivers a KDU to the mailbox of a local entity.
    pub fn route_local(&self, target_fqei: &FQEI, kdu: KDU) {
        if let Some(entity) = self.entities.get(target_fqei) {
            entity.mailbox.lock().unwrap().push_back(kdu);
        }
    }

    /// Runs one "turn" and returns a list of (Target, KDU) tuples.
    pub fn run_turn(&mut self) -> Vec<(FQEI, KDU)> {
        let mut outgoing_routes: Vec<(FQEI, KDU)> = Vec::new();
        let fqei_list: Vec<FQEI> = self.entities.keys().cloned().collect();

        for fqei in fqei_list {
            let entity = self.entities.get_mut(&fqei).unwrap();
            if let Some(message) = entity.mailbox.lock().unwrap().pop_front() {
                // The originator of the incoming message is the target for any responses.
                let response_target = message.originator_fqei.clone();

                let (new_state, outgoing_kuds) = (entity.behavior_fn)(&entity.state, message);
                entity.state = new_state;

                for kdu in outgoing_kuds {
                    outgoing_routes.push((response_target.clone(), kdu));
                }
            }
        }
        outgoing_routes
    }
}

impl Default for EntityProcessor {
    fn default() -> Self {
        Self::new()
    }
}
