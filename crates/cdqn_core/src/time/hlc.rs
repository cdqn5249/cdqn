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

//! # Hybrid Logical Clock (HLC) - Immutable Implementation
//!
//! This version of the HLC is designed with functional principles in mind.
//! The `tick` method is a pure function that consumes the previous clock state
//! and returns a new, advanced clock state, avoiding in-place mutation.
//!
//! ## Structure
//! - Physical component: 48 bits (microsecond precision, ~8,925 years range)
//! - Logical component: 16 bits (up to 65,536 events per physical tick)

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
    "ptbtime1.ptb.de:123",
];

/// List of secondary NTP pool servers (stratum 2-3).
const SECONDARY_NTP_SERVERS: &[&str] = &[
    "0.pool.ntp.org:123",
    "1.pool.ntp.org:123",
    "2.pool.ntp.org:123",
];

/// Hybrid Logical Clock structure.
///
/// Encoded as a single u64:
/// - Bits 0-47: physical microseconds since UNIX epoch
/// - Bits 48-63: logical counter
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HybridLogicalClock {
    /// Encoded HLC value.
    value: u64,
    /// Max allowed drift (ε) in milliseconds.
    epsilon_ms: u64,
}

impl HybridLogicalClock {
    /// Creates a new HLC with current synchronized time and zero logical counter.
    pub fn new(epsilon_ms: Option<u64>) -> Self {
        let epsilon = epsilon_ms.unwrap_or(DEFAULT_EPSILON_MS);
        let physical_us = Self::sync_physical_time().as_micros() as u64;
        Self {
            value: (physical_us & 0x0000_FFFF_FFFF_FFFF), // Logical is implicitly 0
            epsilon_ms: epsilon,
        }
    }

    /// Creates an HLC from a raw u64 value.
    pub fn from_raw(value: u64, epsilon_ms: Option<u64>) -> Self {
        Self {
            value,
            epsilon_ms: epsilon_ms.unwrap_or(DEFAULT_EPSILON_MS),
        }
    }

    /// Extracts the physical time component in microseconds.
    pub fn physical_us(self) -> u64 {
        self.value & 0x0000_FFFF_FFFF_FFFF
    }

    /// Extracts the logical counter.
    pub fn logical(self) -> u16 {
        (self.value >> 48) as u16
    }

    /// Advances the clock, returning a new HLC instance.
    /// This is a pure function that does not mutate state.
    pub fn tick(self, remote: Option<Self>) -> Self {
        let current_physical_us = Self::sync_physical_time().as_micros() as u64;

        let mut new_physical = self.physical_us().max(current_physical_us);
        let mut new_logical;

        if let Some(r) = remote {
            new_physical = new_physical.max(r.physical_us());
            if new_physical == self.physical_us() && new_physical == r.physical_us() {
                new_logical = self.logical().max(r.logical()) as u64 + 1;
            } else if new_physical == self.physical_us() {
                new_logical = self.logical() as u64 + 1;
            } else if new_physical == r.physical_us() {
                new_logical = r.logical() as u64 + 1;
            } else {
                new_logical = 0;
            }
        } else {
            if new_physical > self.physical_us() {
                new_logical = 0;
            } else {
                new_logical = self.logical() as u64 + 1;
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

        // Handle logical counter overflow by incrementing physical time
        if new_logical > 0xFFFF {
            new_physical += 1;
            new_logical = 0;
        }

        let new_value = (new_physical & 0x0000_FFFF_FFFF_FFFF) | (new_logical << 48);
        Self::from_raw(new_value, Some(self.epsilon_ms))
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
    fn query_ntp(server: &str) -> io::Result<Duration> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        let addr = server.to_socket_addrs()?.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "No address for NTP server")
        })?;
        socket.connect(addr)?;

        let mut request = [0u8; 48];
        request[0] = 0b00_011_011; // LI=0, VN=3, Mode=3 (client)

        socket.send(&request)?;

        let mut response = [0u8; 48];
        socket.recv_from(&mut response)?;

        // NTP Transmit Timestamp is at bytes 40-47
        let tx_secs_bytes: [u8; 4] = response[40..44].try_into().unwrap();
        let tx_frac_bytes: [u8; 4] = response[44..48].try_into().unwrap();

        let ntp_tx_secs = u32::from_be_bytes(tx_secs_bytes) as u64;
        let ntp_tx_frac = u32::from_be_bytes(tx_frac_bytes) as u64;

        // NTP epoch is 1900-01-01, UNIX is 1970-01-01 (difference: 2,208,988,800 seconds)
        const NTP_UNIX_EPOCH_DIFF: u64 = 2_208_988_800;
        let unix_secs = ntp_tx_secs.saturating_sub(NTP_UNIX_EPOCH_DIFF);
        let micros = (ntp_tx_frac * 1_000_000) / 0x1_0000_0000;

        Ok(Duration::new(unix_secs, (micros * 1000) as u32))
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
    fn test_tick_is_pure_and_monotonic() {
        let hlc1 = HybridLogicalClock::new(None);
        let hlc2 = hlc1.tick(None);
        assert!(hlc2 > hlc1, "HLC must be monotonic");
        assert_eq!(hlc1.logical(), 0, "Original HLC should not be mutated");
    }

    #[test]
    fn test_tick_with_remote_is_pure() {
        let local1 = HybridLogicalClock::new(None);
        let remote1 = local1.tick(None).tick(None); // remote is ahead

        let local2 = local1.tick(Some(remote1));

        assert!(local2 > local1, "New local clock must be greater");
        assert!(local2 > remote1, "New local clock must be greater than remote");
        assert_eq!(local1.logical(), 0, "Original local HLC should not be mutated");
    }

    #[test]
    #[ignore] // This is an integration test and may fail without internet
    fn test_ntp_query_returns_valid_time() {
        if let Ok(time) = HybridLogicalClock::query_ntp("time.nist.gov:123") {
            // Check if time is roughly after project start (Oct 2025)
            const PROJECT_START_SECS: u64 = 1_761_308_400;
            assert!(time.as_secs() > PROJECT_START_SECS);
        } else {
            panic!("NTP query failed. This test requires internet access.");
        }
    }
}
