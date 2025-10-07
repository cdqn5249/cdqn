// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/mod.rs

//! Defines the concrete data structures for all CDU payload types.

pub mod axiom;
pub mod causal_mode;
pub mod constraint;
pub mod theorem;

pub use axiom::Axiom;
pub use causal_mode::CausalMode;
pub use constraint::Constraint;
pub use theorem::Theorem;

// --- Shared Serialization Helpers ---

// FIX: Use u64 for all length prefixes to be consistent with the main codec.
pub(crate) fn write_vec_string(bytes: &mut Vec<u8>, vec: &Vec<String>) {
    bytes.extend_from_slice(&(vec.len() as u64).to_le_bytes());
    for item in vec {
        bytes.extend_from_slice(&(item.len() as u64).to_le_bytes());
        bytes.extend_from_slice(item.as_bytes());
    }
}

pub(crate) fn read_vec_string(bytes: &[u8], pos: &mut usize) -> Option<Vec<String>> {
    if *pos + 8 > bytes.len() {
        return None;
    }
    let vec_len = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().ok()?) as usize;
    *pos += 8;
    let mut vec = Vec::with_capacity(vec_len);
    for _ in 0..vec_len {
        if *pos + 8 > bytes.len() {
            return None;
        }
        let str_len = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().ok()?) as usize;
        *pos += 8;
        if *pos + str_len > bytes.len() {
            return None;
        }
        let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
        *pos += str_len;
        vec.push(item);
    }
    Some(vec)
}

pub(crate) fn read_string(bytes: &[u8], pos: &mut usize) -> Option<String> {
    if *pos + 8 > bytes.len() {
        return None;
    }
    let str_len = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().ok()?) as usize;
    *pos += 8;
    if *pos + str_len > bytes.len() {
        return None;
    }
    let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
    *pos += str_len;
    Some(item)
}
