use thiserror::Error;

/// The primary error type for the `cdqn-types` crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Error during serialization or deserialization of a CDU or its components.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Error related to cryptographic operations, like hashing.
    #[error("Hashing error: {0}")]
    Hashing(String),

    /// Error when parsing an HLC timestamp.
    #[error("Invalid HLC format: {0}")]
    InvalidHlcFormat(String),
}
