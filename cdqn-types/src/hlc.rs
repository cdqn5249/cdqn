use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::fmt;
use std::str::FromStr;
use crate::error::Error;

/// Represents a Hybrid Logical Clock timestamp.
/// It combines a physical timestamp (UTC) with a logical counter to ensure
/// a total ordering of events in a distributed system.
///
/// Format: `YYYY-MM-DDTHH:MM:SS.sssssssssZ_CCCC` (counter is hex)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hlc {
    timestamp: DateTime<Utc>,
    counter: u16,
}

impl Hlc {
    /// Creates a new HLC timestamp based on the current UTC time.
    pub fn new() -> Self {
        Hlc {
            timestamp: Utc::now(),
            counter: 0,
        }
    }
}

impl Default for Hlc {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Hlc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}_{:04X}",
            self.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true),
            self.counter
        )
    }
}

impl FromStr for Hlc {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.rsplitn(2, '_').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidHlcFormat("Missing counter".to_string()));
        }
        let timestamp_str = parts[1];
        let counter_str = parts[0];

        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .map_err(|e| Error::InvalidHlcFormat(e.to_string()))?
            .with_timezone(&Utc);

        let counter = u16::from_str_radix(counter_str, 16)
            .map_err(|e| Error::InvalidHlcFormat(e.to_string()))?;

        Ok(Hlc { timestamp, counter })
    }
}

impl Serialize for Hlc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Hlc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        Hlc::from_str(&s).map_err(serde::de::Error::custom)
    }
}
