// src/runtime/scheduler.rs

use crate::kernel::{FQEI, KDU};
use crate::runtime::entity::{Entity, Mailbox};
use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// --- TYPE ALIASES to simplify complex types ---
/// A generic, type-erased entity state.
type ErasedState = Box<dyn Any + Send>;
/// A list of KDUs to be sent out.
type OutgoingKdus = Vec<KDU>;
/// A type-erased function pointer to an entity's behavior.
type BehaviorFn = Box<dyn Fn(&ErasedState, KDU) -> (ErasedState, OutgoingKdus) + Send>;

/// A concrete representation of a running entity.
struct EntityInstance {
    behavior_fn: BehaviorFn,
    state: ErasedState,
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

        let behavior_fn = move |state: &ErasedState, message: KDU| {
            // We use .as_ref() to get &dyn Any from &Box<dyn Any> to call downcast_ref.
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

    /// Delivers a KDU to the mailbox of a target entity.
    pub fn route(&self, target_fqei: &FQEI, kdu: KDU) {
        if let Some(entity) = self.entities.get(target_fqei) {
            entity.mailbox.lock().unwrap().push_back(kdu);
        } else {
            println!(
                "[EntityScheduler] WARN: No entity found for FQEI: {}",
                target_fqei
            );
        }
    }

    /// The main event loop. This function runs one "turn" of the simulation.
    pub fn run_turn(&mut self) {
        let mut outgoing_kuds: Vec<(FQEI, KDU)> = Vec::new();
        let fqei_list: Vec<FQEI> = self.entities.keys().cloned().collect();

        for fqei in fqei_list {
            let entity = self.entities.get_mut(&fqei).unwrap();
            let maybe_message = entity.mailbox.lock().unwrap().pop_front();
            if let Some(message) = maybe_message {
                println!("\n[EntityScheduler] Executing behavior for {}", fqei);
                let originator_fqei = message.originator_fqei.clone();
                // We now pass a reference to the Box itself (&entity.state).
                let (new_state, new_kuds) = (entity.behavior_fn)(&entity.state, message);
                entity.state = new_state;

                for kdu in new_kuds {
                    let target_fqei = originator_fqei.clone();
                    outgoing_kuds.push((target_fqei, kdu));
                }
            }
        }

        for (target_fqei, kdu) in outgoing_kuds {
            println!(
                "[EntityScheduler] Routing KDU from {} to {}",
                kdu.originator_fqei, target_fqei
            );
            self.route(&target_fqei, kdu);
        }
    }
}

impl Default for EntityScheduler {
    fn default() -> Self {
        Self::new()
    }
}
