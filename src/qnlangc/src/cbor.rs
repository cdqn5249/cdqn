//! Deterministic RFC 8949 CBOR encoder (stdlib only). Definite-length items,
//! shortest unsigned integers, keys sorted by their encoded bytes.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cbor {
    UInt(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Cbor>),
    Map(Vec<(Cbor, Cbor)>),
}

fn head(major: u8, n: u64) -> Vec<u8> {
    let m = major << 5;
    if n < 24 {
        vec![m | n as u8]
    } else if n <= 0xff {
        vec![m | 24, n as u8]
    } else if n <= 0xffff {
        let mut v = vec![m | 25];
        v.extend_from_slice(&(n as u16).to_be_bytes());
        v
    } else if n <= 0xffff_ffff {
        let mut v = vec![m | 26];
        v.extend_from_slice(&(n as u32).to_be_bytes());
        v
    } else {
        let mut v = vec![m | 27];
        v.extend_from_slice(&n.to_be_bytes());
        v
    }
}

impl Cbor {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Cbor::UInt(n) => head(0, *n),
            Cbor::Bytes(b) => {
                let mut v = head(2, b.len() as u64);
                v.extend_from_slice(b);
                v
            }
            Cbor::Text(s) => {
                let mut v = head(3, s.len() as u64);
                v.extend_from_slice(s.as_bytes());
                v
            }
            Cbor::Array(xs) => {
                let mut v = head(4, xs.len() as u64);
                for x in xs {
                    v.extend_from_slice(&x.encode());
                }
                v
            }
            Cbor::Map(entries) => {
                let mut es: Vec<(Vec<u8>, Vec<u8>)> =
                    entries.iter().map(|(k, v)| (k.encode(), v.encode())).collect();
                es.sort_by(|a, b| a.0.cmp(&b.0));
                let mut v = head(5, es.len() as u64);
                for (k, val) in es {
                    v.extend_from_slice(&k);
                    v.extend_from_slice(&val);
                }
                v
            }
        }
    }
}

/// 64-bit big-endian transport words for the instruction stream.
pub fn words_to_bytes(words: &[u64]) -> Vec<u8> {
    let mut v = Vec::with_capacity(words.len() * 8);
    for w in words {
        v.extend_from_slice(&w.to_be_bytes());
    }
    v
}
