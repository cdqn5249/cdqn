// src/runtime/test_entities.rs

use crate::kernel::KDU;
use crate::runtime::entity::Entity;

// --- Pinger Entity ---
// State: A simple counter for how many pongs it has received.
#[derive(Default)]
pub struct Pinger;
impl Entity for Pinger {
    type State = u32;

    fn behavior(state: &Self::State, message: KDU) -> (Self::State, Vec<KDU>) {
        println!("[Pinger] Received KDU with ID: {}", message.kdu_id);
        // When it receives a "pong", it increments its state.
        let new_state = state + 1;
        println!("[Pinger] New state: {}", new_state);
        (new_state, vec![]) // Returns no new messages
    }
}

// --- Ponger Entity ---
// State: A simple counter for how many pings it has received.
#[derive(Default)]
pub struct Ponger;
impl Entity for Ponger {
    type State = u32;

    fn behavior(state: &Self::State, message: KDU) -> (Self::State, Vec<KDU>) {
        println!("[Ponger] Received KDU with ID: {}", message.kdu_id);
        // When it receives a "ping", it increments its state and creates a "pong" KDU.
        let new_state = state + 1;
        println!("[Ponger] New state: {}", new_state);

        // Create a new KDU to send back to the originator.
        // NOTE: We need a KDUFactory to do this properly. For this test, we'll create a dummy KDU.
        let response_kdu = KDU {
            kdu_spec_version: "2.1.0".to_string(),
            kdu_id: format!("pong-{}", new_state), // Dummy ID
            content_hash: String::new(),
            originator_fqei: "ponger@test".to_string(), // Dummy FQEI
            originator_signature: vec![],
            timestamp_utc: String::new(),
            kdu_type: "Generic".to_string(),
            metadata: message.metadata, // Echo the metadata back
            data_payload: b"pong".to_vec(),
        };

        (new_state, vec![response_kdu])
    }
}
