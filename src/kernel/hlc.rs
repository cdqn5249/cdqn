// src/kernel/hlc.rs

use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

// The HLC service. It holds the last known time and a counter.
#[derive(Debug, Clone)]
pub struct HLC {
    last_time: Arc<Mutex<DateTime<Utc>>>,
    counter: Arc<Mutex<u16>>,
}

// Implement the Default trait as suggested by clippy.
impl Default for HLC {
    fn default() -> Self {
        Self::new()
    }
}

impl HLC {
    // Creates a new HLC instance, initialized to the current time.
    pub fn new() -> Self {
        HLC {
            last_time: Arc::new(Mutex::new(Utc::now())),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    // Generates a new, unique, and sortable timestamp ID.
    pub fn now(&self) -> (String, String) {
        let mut last_time = self.last_time.lock().unwrap();
        let mut counter = self.counter.lock().unwrap();

        let current_time = Utc::now();

        if current_time > *last_time {
            *last_time = current_time;
            *counter = 0;
        } else {
            *counter += 1;
        }

        let timestamp_utc = last_time.to_rfc3339();
        // Format into a sortable string: Time-Counter
        let kdu_id = format!("{}-{:04x}", last_time.timestamp_micros(), *counter);

        (kdu_id, timestamp_utc)
    }
}
