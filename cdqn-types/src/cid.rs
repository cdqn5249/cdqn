use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

/// Represents a Content Identifier (CID).
/// A CID is a self-describing content-addressed identifier, derived from
/// the cryptographic hash of the data it refers to.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cid(String);

impl Cid {
    pub fn new(hash_str: String) -> Self {
        Cid(hash_str)
    }
}

impl fmt::Display for Cid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Cid {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cid(s.to_string()))
    }
}

impl AsRef<str> for Cid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
