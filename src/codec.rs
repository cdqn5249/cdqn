// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/codec.rs

//! A custom, minimalist binary codec for Chronosa's data structures.

use crate::cdu::{Cdu, CduMetadata};
use crate::hlc::Hlc;
use crate::payloads::{Axiom, CausalMode, Constraint, Theorem};
use crate::reasoning::{PrimeElement, SemiAxiom};

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

impl Encode for f64 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_le_bytes());
    }
}
impl Decode for f64 {
    fn decode(buffer: &mut &[u8]) -> Self {
        let (float_bytes, rest) = buffer.split_at(std::mem::size_of::<f64>());
        *buffer = rest;
        f64::from_le_bytes(float_bytes.try_into().unwrap())
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

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            Some(val) => {
                1u8.encode(buffer);
                val.encode(buffer);
            }
            None => 0u8.encode(buffer),
        }
    }
}
impl<T: Decode> Decode for Option<T> {
    fn decode(buffer: &mut &[u8]) -> Self {
        match u8::decode(buffer) {
            1 => Some(T::decode(buffer)),
            _ => None,
        }
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

// --- Payload Struct Implementations ---

impl Encode for PrimeElement {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.id.encode(buffer);
        self.world.encode(buffer);
        self.representation.encode(buffer);
        self.description.encode(buffer);
        self.irreducibility_proof.encode(buffer);
        self.symmetric_pair.encode(buffer);
    }
}
impl Decode for PrimeElement {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            id: String::decode(buffer),
            world: String::decode(buffer),
            representation: Vec::<f64>::decode(buffer),
            description: String::decode(buffer),
            irreducibility_proof: String::decode(buffer),
            symmetric_pair: Option::<String>::decode(buffer),
            relationships: Default::default(),
        }
    }
}

impl Encode for SemiAxiom {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.id.encode(buffer);
        self.world.encode(buffer);
        self.prime_elements.encode(buffer);
        self.description.encode(buffer);
        self.weight.encode(buffer);
    }
}
impl Decode for SemiAxiom {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            id: String::decode(buffer),
            world: String::decode(buffer),
            prime_elements: Vec::<String>::decode(buffer),
            description: String::decode(buffer),
            weight: f64::decode(buffer),
        }
    }
}

impl Encode for Axiom {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.id.encode(buffer);
        self.worlds.encode(buffer);
        self.premises.encode(buffer);
        self.description.encode(buffer);
        self.weight.encode(buffer);
    }
}
impl Decode for Axiom {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            id: String::decode(buffer),
            worlds: Vec::<String>::decode(buffer),
            premises: Vec::<String>::decode(buffer),
            description: String::decode(buffer),
            weight: f64::decode(buffer),
        }
    }
}

impl Encode for Theorem {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.premises.encode(buffer);
        self.conclusion.encode(buffer);
        self.proof_path.encode(buffer);
        self.confidence_score.encode(buffer);
    }
}
impl Decode for Theorem {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            premises: Vec::<String>::decode(buffer),
            conclusion: String::decode(buffer),
            proof_path: Vec::<String>::decode(buffer),
            confidence_score: f64::decode(buffer),
        }
    }
}

impl Encode for Constraint {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.target_path.encode(buffer);
        self.inhibiting_context.encode(buffer);
        self.reason.encode(buffer);
        self.world.encode(buffer);
    }
}
impl Decode for Constraint {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            target_path: Vec::<String>::decode(buffer),
            inhibiting_context: String::decode(buffer),
            reason: String::decode(buffer),
            world: String::decode(buffer),
        }
    }
}

impl Encode for CausalMode {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.mode_type.encode(buffer);
        self.vector.encode(buffer);
        self.source_cdu.encode(buffer);
    }
}
impl Decode for CausalMode {
    fn decode(buffer: &mut &[u8]) -> Self {
        Self {
            mode_type: String::decode(buffer),
            vector: Vec::<f64>::decode(buffer),
            source_cdu: String::decode(buffer),
        }
    }
}
