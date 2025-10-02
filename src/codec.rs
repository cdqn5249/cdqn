// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/codec.rs 

//! A custom, minimalist binary codec for Chronosa's data structures.

use crate::cdu::{Cdu, CduMetadata};
use crate::core::ChronosaCore;
use crate::hlc::Hlc;

// A simple trait for encoding an object into a byte buffer.
pub trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>);
}

// A simple trait for decoding an object from a byte slice.
// The slice is advanced as bytes are consumed.
pub trait Decode: Sized {
    fn decode(buffer: &mut &[u8]) -> Self;
}

// --- Primitive Implementations ---

// --- FIX: Add implementation for u8 ---
impl Encode for u8 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self);
    }
}
impl Decode for u8 {
    fn decode(buffer: &mut &[u8]) -> Self {
        let byte = buffer[0];
        *buffer = &buffer[1..];
        byte
    }
}

impl Encode for u16 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl Decode for u16 {
    fn decode(buffer: &mut &[u8]) -> Self {
        let (int_bytes, rest) = buffer.split_at(std::mem::size_of::<u16>());
        *buffer = rest;
        u16::from_be_bytes(int_bytes.try_into().unwrap())
    }
}

impl Encode for u64 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl Decode for u64 {
    fn decode(buffer: &mut &[u8]) -> Self {
        let (int_bytes, rest) = buffer.split_at(std::mem::size_of::<u64>());
        *buffer = rest;
        u64::from_be_bytes(int_bytes.try_into().unwrap())
    }
}

impl Encode for String {
    fn encode(&self, buffer: &mut Vec<u8>) {
        (self.len() as u64).encode(buffer);
        buffer.extend_from_slice(self.as_bytes());
    }
}
impl Decode for String {
    fn decode(buffer: &mut &[u8]) -> Self {
        let len = u64::decode(buffer) as usize;
        let (str_bytes, rest) = buffer.split_at(len);
        *buffer = rest;
        String::from_utf8(str_bytes.to_vec()).unwrap()
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, buffer: &mut Vec<u8>) {
        (self.len() as u64).encode(buffer);
        for item in self {
            item.encode(buffer);
        }
    }
}
impl<T: Decode> Decode for Vec<T> {
    fn decode(buffer: &mut &[u8]) -> Self {
        let len = u64::decode(buffer) as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(buffer));
        }
        vec
    }
}

// --- Struct Implementations ---

impl Encode for Hlc {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.timestamp.encode(buffer);
        self.counter.encode(buffer);
    }
}
impl Decode for Hlc {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            timestamp: u64::decode(buffer),
            counter: u16::decode(buffer),
        }
    }
}

impl Encode for CduMetadata {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.hlc.encode(buffer);
        self.causes.encode(buffer);
        self.tags.encode(buffer);
    }
}
impl Decode for CduMetadata {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            hlc: Hlc::decode(buffer),
            causes: Vec::<String>::decode(buffer),
            tags: Vec::<String>::decode(buffer),
        }
    }
}

impl Encode for Cdu {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.name.encode(buffer);
        self.payload.encode(buffer);
        self.metadata.encode(buffer);
    }
}
impl Decode for Cdu {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            name: String::decode(buffer),
            payload: Vec::<u8>::decode(buffer),
            metadata: CduMetadata::decode(buffer),
        }
    }
}

impl Encode for ChronosaCore {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.hlc.encode(buffer);
        self.log.encode(buffer);
    }
}
impl Decode for ChronosaCore {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            hlc: Hlc::decode(buffer),
            log: Vec::<Cdu>::decode(buffer),
        }
    }
}
