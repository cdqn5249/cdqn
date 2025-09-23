// BaDaaS License: This file is governed by the BaDaaS license.
// File Path: /src/kernel/crypto/src/types.rs

//! Defines the public types for the K.CryptoCore module, including keys,
//! signatures, and the verbose error enum.

use ed25519_dalek::SignatureError;

// --- Core Type Definitions (Publicly Exported) ---
pub type Signature = ed25519_dalek::Signature;
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];

/// A specific, verbose, and robust error type for all cryptographic operations.
/// The underlying SignatureError does not implement PartialEq or Eq, so we
/// cannot derive them automatically. We only derive Debug.
#[derive(Debug)]
pub enum CryptoError {
    /// The underlying cryptographic library returned an error. This variant
    /// wraps the library's error to preserve the rich, original context.
    LibraryError(SignatureError),
}
