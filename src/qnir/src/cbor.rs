//! Canonical CBOR (RFC 8949) — the encoding of §2.6's artifacts.
//!
//! Encoder: shortest-form heads, definite lengths, map keys sorted bytewise
//! by their encoded key (§2.6 canonical maps).
//! Decoder: strict — shortest-form heads only (indefinite lengths are not
//! canonical), map keys must be strictly increasing by their encoded bytes
//! (a repeated key is the declared duplicate-key surface; a non-increasing
//! order is non-canonical).

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Cbor {
    UInt(u64),
    Bytes(Vec<u8>),
    Text(String),
    Array(Vec<Cbor>),
    /// Map entries kept in encoded order (order and duplication are checked).
    Map(Vec<(Cbor, Cbor)>),
    Bool(bool),
    Null,
}

#[derive(Clone, Debug)]
pub struct CborError(pub String);

impl fmt::Display for CborError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

pub fn encode(x: &Cbor) -> Vec<u8> {
    match x {
        Cbor::Null => vec![0xf6],
        Cbor::Bool(b) => vec![if *b { 0xf5 } else { 0xf4 }],
        Cbor::UInt(n) => head(0, *n),
        Cbor::Bytes(b) => {
            let mut v = head(2, b.len() as u64);
            v.extend_from_slice(b);
            v
        }
        Cbor::Text(s) => {
            let b = s.as_bytes();
            let mut v = head(3, b.len() as u64);
            v.extend_from_slice(b);
            v
        }
        Cbor::Array(items) => {
            let mut v = head(4, items.len() as u64);
            for it in items {
                v.extend_from_slice(&encode(it));
            }
            v
        }
        Cbor::Map(entries) => {
            // canonical: sort by encoded key bytes (RFC 8949 §4.2.1)
            let mut keyed: Vec<(Vec<u8>, Vec<u8>)> = entries
                .iter()
                .map(|(k, val)| (encode(k), encode(val)))
                .collect();
            keyed.sort_by(|a, b| a.0.cmp(&b.0));
            let mut v = head(5, keyed.len() as u64);
            for (kb, vb) in keyed {
                v.extend_from_slice(&kb);
                v.extend_from_slice(&vb);
            }
            v
        }
    }
}

struct Reader<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Reader<'a> {
    fn need(&self, n: usize) -> Result<(), CborError> {
        if self.i + n > self.b.len() {
            Err(CborError(format!(
                "count exceeds the enclosing artifact's remaining length"
            )))
        } else {
            Ok(())
        }
    }
    fn head_at(&mut self) -> Result<(u8, u64), CborError> {
        self.need(1)?;
        let ib = self.b[self.i];
        let major = ib >> 5;
        let info = ib & 0x1f;
        self.i += 1;
        let n = match info {
            i if i < 24 => i as u64,
            24 => {
                self.need(1)?;
                let v = self.b[self.i] as u64;
                self.i += 1;
                v
            }
            25 => {
                self.need(2)?;
                let v = u64::from_be_bytes([0, 0, 0, 0, 0, 0, self.b[self.i], self.b[self.i + 1]]);
                self.i += 2;
                v
            }
            26 => {
                self.need(4)?;
                let mut w = [0u8; 4];
                w.copy_from_slice(&self.b[self.i..self.i + 4]);
                self.i += 4;
                u32::from_be_bytes(w) as u64
            }
            27 => {
                self.need(8)?;
                let mut w = [0u8; 8];
                w.copy_from_slice(&self.b[self.i..self.i + 8]);
                self.i += 8;
                u64::from_be_bytes(w)
            }
            _ => {
                // indefinite lengths and simple values are not canonical here
                return Err(CborError("indefinite lengths are not canonical".into()));
            }
        };
        // shortest-form check: the head must be the shortest encoding of n
        let canonical = if n < 24 {
            info == n as u8
        } else if n <= 0xff {
            info == 24
        } else if n <= 0xffff {
            info == 25
        } else if n <= 0xffff_ffff {
            info == 26
        } else {
            info == 27
        };
        if !canonical {
            return Err(CborError("non-shortest-form head is not canonical".into()));
        }
        Ok((major, n))
    }
}

pub fn decode(b: &[u8]) -> Result<Cbor, CborError> {
    let mut r = Reader { b, i: 0 };
    let v = dec(&mut r)?;
    if r.i != b.len() {
        return Err(CborError("trailing CBOR bytes".into()));
    }
    Ok(v)
}

fn dec(r: &mut Reader) -> Result<Cbor, CborError> {
    let (major, n) = r.head_at()?;
    match major {
        0 => Ok(Cbor::UInt(n)),
        1 => {
            // negative CBOR ints appear in no committed §2.6 artifact of the
            // corpus; the strict decoder refuses them rather than guessing
            Err(CborError("unsupported CBOR major type".into()))
        }
        2 => {
            let len = n as usize;
            r.need(len)?;
            let out = r.b[r.i..r.i + len].to_vec();
            r.i += len;
            Ok(Cbor::Bytes(out))
        }
        3 => {
            let len = n as usize;
            r.need(len)?;
            let s = std::str::from_utf8(&r.b[r.i..r.i + len])
                .map_err(|_| CborError("invalid UTF-8 in text string".into()))?;
            r.i += len;
            Ok(Cbor::Text(s.to_string()))
        }
        4 => {
            let mut out = Vec::new();
            for _ in 0..n {
                out.push(dec(r)?);
            }
            Ok(Cbor::Array(out))
        }
        5 => {
            let mut entries: Vec<(Cbor, Cbor)> = Vec::new();
            let mut prev_key: Option<Vec<u8>> = None;
            for _ in 0..n {
                let k = dec(r)?;
                let v = dec(r)?;
                let kb = encode(&k);
                if let Some(pk) = &prev_key {
                    if *pk == kb {
                        return Err(CborError("duplicate envelope key".into()));
                    }
                    if *pk > kb {
                        return Err(CborError("map keys are not in canonical order".into()));
                    }
                }
                prev_key = Some(kb);
                entries.push((k, v));
            }
            Ok(Cbor::Map(entries))
        }
        7 => match n {
            // RFC 8949 simple values: false = 20, true = 21, null = 22.
            // (Repaired at v0.3.0-draft.12: the increment-2 table was off by
            // one — 20 decoded as null and 22 as true — a defect the
            // reference interpreter's receipt decode surfaced, since the
            // terminal receipt's nil-selected-token key is the corpus's only
            // committed simple value; see the increment-3 wave record.)
            20 => Ok(Cbor::Bool(false)),
            21 => Ok(Cbor::Bool(true)),
            22 => Ok(Cbor::Null),
            _ => Err(CborError("unsupported simple value".into())),
        },
        _ => Err(CborError("unsupported CBOR major type".into())),
    }
}
