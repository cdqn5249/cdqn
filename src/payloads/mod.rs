// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/payloads/mod.rs

//! Defines the concrete data structures for all CDU payload types.

// 1. Declare the sub-modules we will create.
pub mod causal_mode;
pub mod constraint;
pub mod theorem;

// 2. Re-export the structs for easy access from other parts of the application.
pub use causal_mode::CausalMode;
pub use constraint::Constraint;
pub use theorem::Theorem;

// 3. Define shared (crate-public) serialization helpers here.

pub(crate) fn write_vec_string(bytes: &mut Vec<u8>, vec: &Vec<String>) {
    bytes.extend_from_slice(&(vec.len() as u32).to_le_bytes());
    for item in vec {
        bytes.extend_from_slice(&(item.len() as u32).to_le_bytes());
        bytes.extend_from_slice(item.as_bytes());
    }
}

pub(crate) fn read_vec_string(bytes: &[u8], pos: &mut usize) -> Option<Vec<String>> {
    if *pos + 4 > bytes.len() {
        return None;
    }
    let vec_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
    *pos += 4;
    let mut vec = Vec::with_capacity(vec_len);
    for _ in 0..vec_len {
        if *pos + 4 > bytes.len() {
            return None;
        }
        let str_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
        *pos += 4;
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
    if *pos + 4 > bytes.len() {
        return None;
    }
    let str_len = u32::from_le_bytes(bytes[*pos..*pos + 4].try_into().ok()?) as usize;
    *pos += 4;
    if *pos + str_len > bytes.len() {
        return None;
    }
    let item = String::from_utf8(bytes[*pos..*pos + str_len].to_vec()).ok()?;
    *pos += str_len;
    Some(item)
}
