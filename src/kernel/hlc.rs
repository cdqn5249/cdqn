// src/kernel/hlc.rs

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// The HLC service. It holds the last known time and a counter.
#[derive(Debug, Clone)]
pub struct HLC {
    // We now store the time as microseconds since the Unix epoch.
    last_time_micros: Arc<Mutex<u128>>,
    counter: Arc<Mutex<u16>>,
}

impl Default for HLC {
    fn default() -> Self {
        Self::new()
    }
}

impl HLC {
    pub fn new() -> Self {
        let now_micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();
        HLC {
            last_time_micros: Arc::new(Mutex::new(now_micros)),
            counter: Arc::new(Mutex::new(0)),
        }
    }

    // Generates a new, unique, and sortable timestamp ID.
    pub fn now(&self) -> (String, String) {
        let mut last_time = self.last_time_micros.lock().unwrap();
        let mut counter = self.counter.lock().unwrap();

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros();

        if current_time > *last_time {
            *last_time = current_time;
            *counter = 0;
        } else {
            *counter += 1;
        }

        // We no longer have a convenient RFC3339 formatter, so we'll just use the micros.
        // This is a temporary simplification.
        let timestamp_utc = format!("{}", *last_time);
        let kdu_id = format!("{}-{:04x}", *last_time, *counter);

        (kdu_id, timestamp_utc)
    }
}
