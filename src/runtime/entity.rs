// src/runtime/entity.rs

use crate::kernel::KDU;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// A Mailbox is a thread-safe queue for incoming KDUs.
/// We use Arc<Mutex<>> to allow multiple entities to safely send messages
/// to this mailbox at the same time.
pub type Mailbox = Arc<Mutex<VecDeque<KDU>>>;

/// The core trait that defines any actor in the cdqn ecosystem.
/// An Entity is a component with its own internal state that reacts to
/// incoming KDUs.
pub trait Entity {
    /// The type of the entity's internal state.
    type State;

    /// The entity's behavior function. This is the heart of the actor model.
    /// It takes the current state and an incoming KDU, and returns the
    /// new state and a list of KDUs to be sent to other entities.
    /// This function MUST be a pure function.
    fn behavior(state: &Self::State, message: KDU) -> (Self::State, Vec<KDU>);
}
