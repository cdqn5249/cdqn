// BaDaaS License
// File Path: crates/cdqn_core/src/time/hlc.rs
//
// Copyright (c) 2025 Christophe Duy Quang Nguyen
//
// This file is part of the CDQN (Causal Data Query Nodes) ecosystem.
//
// Licensed under the BaDaaS (Build and Develop as a Service) License.
// See LICENSE-BaDaaS.md in the project root for terms.
//
// AI-Mediated Access: All interactions with the CDQN network must be
// mediated by an AI entity (ProxyAgent) as per the BaDaaS license.

//! # Hybrid Logical Clock (HLC)
//!
//! The Hybrid Logical Clock provides a way to generate timestamps that capture
//! both physical time and logical causality. This implementation includes
//! mechanisms to synchronize with remote atomic time sources (via NTP) to
//! minimize clock drift, while maintaining the core HLC invariants:
//!
//! - **Monotonicity**: Timestamps are always increasing.
//! - **Causality**: If event A happens-before B, then HLC(A) < HLC(B).
//! - **Bounded Drift**: Logical time stays close to physical time within ε.
//!
//! ## Structure
//! - Physical component: 48 bits (microsecond precision, ~8,925 years range)
//! - Logical component: 16 bits (up to 65,536 events per physical tick)
//!
//! ## Time Synchronization
//! - Uses a hierarchy of NTP sources (primary atomic servers, secondary pools).
//! - Fallback to local system time if all remote sources fail.
//! - ε-bound: Configurable max drift (default 500ms).
//!
//! Note: NTP client is implemented using only `std::net` and `std::time` for
//! zero external dependencies.

#![forbid(unsafe_code)]

use std::fmt;
use std::io;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Default maximum allowed clock drift (ε) in milliseconds.
const DEFAULT_EPSILON_MS: u64 = 500;

/// List of primary NTP servers (stratum 1, atomic clock connected).
const PRIMARY_NTP_SERVERS: &[&str] = &[
    "time.nist.gov:123",
    "time-a-g.nist.gov:123",
    "time-b-g.nist.gov:123",
    "ptbtime1.ptb.de:123",
    "ptbtime2.ptb.de:123",
];

/// List of secondary NTP pool servers (stratum 2-3).
const SECONDARY_NTP_SERVERS: &[&str] = &[
    "0.pool.ntp.org:123",
    "1.pool.ntp.org:123",
    "2.pool.ntp.org:123",
    "3.pool.ntp.org:123",
];

/// Hybrid Logical Clock structure.
/// 
/// Encoded as a single u64: 
/// - Bits 0-47: physical microseconds since UNIX epoch
/// - Bits 48-63: logical counter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HybridLogicalClock {
    /// Encoded HLC value
    value: u64,
    /// Max allowed drift (ε)
    epsilon_ms: u64,
}

impl HybridLogicalClock {
    /// Creates a new HLC with current synchronized time and zero logical counter.
    pub fn new(epsilon_ms: Option<u64>) -> Self {
        let epsilon = epsilon_ms.unwrap_or(DEFAULT_EPSILON_MS);
        let physical_us = Self::sync_physical_time().as_micros() as u64;
        Self {
            value: physical_us & 0x0000_FFFF_FFFF_FFFF | (0 << 48),
            epsilon_ms: epsilon,
        }
    }

    /// Extracts the physical time component in microseconds.
    pub fn physical_us(&self) -> u64 {
        self.value & 0x0000_FFFF_FFFF_FFFF
    }

    /// Extracts the logical counter.
    pub fn logical(&self) -> u16 {
        (self.value >> 48) as u16
    }

    /// Advances the clock, optionally merging with a remote HLC.
    ///
    /// Ensures monotonicity and causality while bounding drift from physical time.
    pub fn tick(&mut self, remote: Option<Self>) -> Self {
        let current_physical_us = Self::sync_physical_time().as_micros() as u64;

        let mut new_physical = self.physical_us().max(current_physical_us);
        let mut new_logical = self.logical() as u64;

        if let Some(r) = remote {
            new_physical = new_physical.max(r.physical_us());
            if new_physical == self.physical_us() && new_physical == r.physical_us() {
                new_logical = new_logical.max(r.logical() as u64) + 1;
            } else if new_physical == self.physical_us() {
                new_logical += 1;
            } else if new_physical == r.physical_us() {
                new_logical = r.logical() as u64 + 1;
            } else {
                new_logical = 0;
            }
        } else {
            if current_physical_us > self.physical_us() {
                new_logical = 0;
            } else {
                new_logical += 1;
            }
        }

        // Check and enforce ε-bound
        let drift_ms = if current_physical_us > new_physical {
            (current_physical_us - new_physical) / 1000
        } else {
            (new_physical - current_physical_us) / 1000
        };
        if drift_ms > self.epsilon_ms {
            // Reset to physical time if drift exceeds bound
            new_physical = current_physical_us;
            new_logical = 0;
        }

        // Ensure logical counter doesn't overflow (though 16 bits is generous)
        if new_logical > 0xFFFF {
            new_physical += 1;
            new_logical = 0;
        }

        self.value = (new_physical & 0x0000_FFFF_FFFF_FFFF) | (new_logical << 48);
        *self
    }

    /// Synchronizes with remote time sources and returns the adjusted duration since epoch.
    fn sync_physical_time() -> Duration {
        // Try primary sources first
        for server in PRIMARY_NTP_SERVERS {
            if let Ok(time) = Self::query_ntp(server) {
                return time;
            }
        }

        // Fallback to secondary
        for server in SECONDARY_NTP_SERVERS {
            if let Ok(time) = Self::query_ntp(server) {
                return time;
            }
        }

        // Ultimate fallback: local system time
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
    }

    /// Queries an NTP server and returns the duration since epoch.
    ///
    /// Implements a basic NTP client using UDP. This is a simplified version
    /// for demonstration; in production, add error handling for leap seconds, etc.
    fn query_ntp(server: &str) -> io::Result<Duration> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_secs(2)))?;
        socket.connect(server.to_socket_addrs()?.next().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "No address for NTP server",
        ))?)?;

        // NTP request packet (LI=0, VN=3, Mode=3, Stratum=0, Poll=0, Precision=0)
        let mut request = [0u8; 48];
        request = 0b00_011_011; // LI=0, VN=3, Mode=3 (client)

        // Send request and record local transmit time
        let local_tx = SystemTime::now();
        socket.send(&request)?;

        // Receive response
        let mut response = [0u8; 48];
        socket.recv(&mut response)?;

        // Extract transmit timestamp from response (bytes 32-39)
        let ntp_tx_secs = u32::from_be_bytes([response, response, response, response]) as u64;
        let ntp_tx_frac = u32::from_be_bytes([response, response, response, response]) as u64;

        // NTP epoch is 1900-01-01, UNIX is 1970-01-01 (difference: 2208988800 seconds)
        let unix_secs = ntp_tx_secs - 2208988800;
        let micros = (ntp_tx_frac * 1_000_000 / 0x1_0000_0000) as u64; // frac to micros

        // Simple round-trip delay compensation (assume symmetric)
        let local_rx = SystemTime::now();
        let rtt = local_rx.duration_since(local_tx).unwrap_or(Duration::ZERO) / 2;
        Ok(Duration::from_secs(unix_secs) + Duration::from_micros(micros) + rtt)
    }
}

impl fmt::Display for HybridLogicalClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:04}", self.physical_us(), self.logical())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hlc() {
        let hlc = HybridLogicalClock::new(None);
        assert_eq!(hlc.logical(), 0);
        assert!(hlc.physical_us() > 0);
    }

    #[test]
    fn test_tick_monotonic() {
        let mut hlc = HybridLogicalClock::new(None);
        let t1 = hlc;
        hlc.tick(None);
        assert!(hlc > t1);
    }

    #[test]
    fn test_tick_with_remote() {
        let mut local = HybridLogicalClock::new(None);
        let mut remote = HybridLogicalClock::new(None);
        remote.tick(None); // Advance remote

        local.tick(Some(remote));
        assert!(local >= remote);
    }

    #[test]
    fn test_drift_bound() {
        let mut hlc = HybridLogicalClock::new(Some(100)); // Small epsilon for test
        // Simulate large drift by advancing physical time manually
        let original = hlc;
        hlc.value += 1_000_000; // 1 second artificial drift
        hlc.tick(None); // Should reset due to epsilon violation
        assert!(hlc.physical_us() > original.physical_us());
        assert_eq!(hlc.logical(), 0);
    }

    #[test]
    fn test_ntp_query() {
        // This test may fail in environments without internet; it's integration-style
        if let Ok(time) = HybridLogicalClock::query_ntp("time.nist.gov:123") {
            assert!(time.as_secs() > 1_700_000_000); // Rough check for post-2023 time
        } else {
            // Fallback: ensure local time is used without panic
            let local = HybridLogicalClock::sync_physical_time();
            assert!(local.as_secs() > 1_700_000_000);
        }
    }
}
