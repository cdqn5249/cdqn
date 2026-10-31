//! The §2.6 value grammar — the four closed type tags, the sign convention,
//! minimal big-endian magnitudes, and the §2.3 zoom normalizer N.
//!
//! Malformed surfaces declared by the corpus (family M):
//!   M2 negative-tagged zero · M3 leading zero magnitude word (non-canonical)
//!   M4 non-normalized zoom encoding · M5 count exceeds the enclosing
//!   artifact's remaining length · M6 unknown type tag

pub const TAG_INT_POS: u64 = 1;
pub const TAG_INT_NEG: u64 = 2;
pub const TAG_TUPLE: u64 = 3;
pub const TAG_ZOOM: u64 = 4;

/// One 64-bit big-endian unsigned word (§2.6).
pub fn w(x: u64) -> Vec<u8> {
    x.to_be_bytes().to_vec()
}

pub fn words_of(bs: &[u8]) -> Result<Vec<u64>, String> {
    if bs.len() % 8 != 0 {
        return Err("trailing partial instruction".to_string());
    }
    Ok(bs
        .chunks(8)
        .map(|c| u64::from_be_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]))
        .collect())
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Sign rides the type tag; magnitude words carry the absolute value,
    /// minimal big-endian, no leading zero word; canonical zero is INT_POS
    /// with count 0.
    Int { neg: bool, mag: Vec<u64> },
    Tuple(Vec<Value>),
    /// A zoom pair in the normalized form of §2.3's N.
    Zoom(i128, i128),
}

impl Value {
    pub fn int(v: i128) -> Value {
        let neg = v < 0;
        let mut m = v.unsigned_abs();
        let mut mag: Vec<u64> = Vec::new();
        while m > 0 {
            mag.push((m & 0xFFFF_FFFF_FFFF_FFFF) as u64);
            m >>= 64;
        }
        mag.reverse(); // big-endian: most significant first
        Value::Int { neg, mag }
    }
    /// 2^k for k a multiple of 64 — magnitude [1, 0, ...] (k/64 zero words).
    pub fn pow2(k: u32) -> Value {
        let zeros = (k / 64) as usize;
        let mut mag = vec![1u64];
        mag.extend(std::iter::repeat(0u64).take(zeros));
        Value::Int { neg: false, mag }
    }
}

/// §2.3 minimal-zoom normalizer N: N(0, z) = (0, 0); m = 2^k·q with q odd
/// gives (q, z + k).
pub fn normalize_zoom(m: i128, z: i128) -> (i128, i128) {
    if m == 0 {
        return (0, 0);
    }
    let mut q = m;
    let mut k: i128 = 0;
    while q % 2 == 0 {
        q /= 2;
        k += 1;
    }
    (q, z + k)
}

fn enc_int_parts(neg: bool, mag: &[u64]) -> Vec<u8> {
    let mut out = w(if neg { TAG_INT_NEG } else { TAG_INT_POS });
    out.extend_from_slice(&w(mag.len() as u64));
    for m in mag {
        out.extend_from_slice(&w(*m));
    }
    out
}

/// Canonical encoding of one §2.6 value.
pub fn enc_value(v: &Value) -> Vec<u8> {
    match v {
        Value::Int { neg, mag } => enc_int_parts(*neg, mag),
        Value::Tuple(fields) => {
            let mut out = w(TAG_TUPLE);
            out.extend_from_slice(&w(fields.len() as u64));
            for f in fields {
                out.extend_from_slice(&enc_value(f));
            }
            out
        }
        Value::Zoom(m, z) => {
            let (q, ze) = normalize_zoom(*m, *z);
            let mut out = w(TAG_ZOOM);
            out.extend_from_slice(&enc_value(&Value::int(q)));
            out.extend_from_slice(&enc_value(&Value::int(ze)));
            out
        }
    }
}

/// Word reader over a decoded instruction stream with the declared structural
/// bound: a count must not exceed the remaining length of its enclosing
/// artifact (§2.6, family M5), and an artifact that ends where a declared
/// structure continues is a trailing partial instruction (family M10).
pub struct WordReader<'a> {
    pub words: &'a [u64],
    pub i: usize,
}

impl<'a> WordReader<'a> {
    pub fn new(words: &'a [u64]) -> Self {
        WordReader { words, i: 0 }
    }
    fn oom(&self, _what: &str) -> String {
        // an artifact that ends where a declared structure continues is a
        // trailing partial instruction (family M10); count overruns are
        // reported by take_count (family M5)
        "trailing partial instruction".to_string()
    }
    pub fn next(&mut self, what: &str) -> Result<u64, String> {
        if self.i >= self.words.len() {
            return Err(self.oom(what));
        }
        let v = self.words[self.i];
        self.i += 1;
        Ok(v)
    }
    pub fn take_count(&mut self, what: &str) -> Result<usize, String> {
        let c = self.next(what)? as usize;
        // the count's own declared items must fit in what remains
        if c > self.words.len().saturating_sub(self.i) {
            return Err(
                "count exceeds the enclosing artifact's remaining length".to_string()
            );
        }
        Ok(c)
    }
}

/// Decodes one §2.6 value encoding, enforcing every declared malformed
/// surface of the value grammar.
pub fn dec_value(r: &mut WordReader) -> Result<Value, String> {
    let tag = r.next("value")?;
    match tag {
        TAG_INT_POS | TAG_INT_NEG => {
            let cnt = r.take_count("value")? as u64;
            let mut mag = Vec::with_capacity(cnt as usize);
            for _ in 0..cnt {
                mag.push(r.next("value")?);
            }
            // minimal big-endian: no leading zero magnitude word (M3)
            if mag.len() > 1 && mag[0] == 0 {
                return Err("leading zero magnitude word (non-canonical)".to_string());
            }
            // sign convention: a negative-tagged zero is malformed (M2)
            if tag == TAG_INT_NEG && mag.iter().all(|&m| m == 0) {
                return Err("negative-tagged zero".to_string());
            }
            Ok(Value::Int {
                neg: tag == TAG_INT_NEG,
                mag,
            })
        }
        TAG_TUPLE => {
            let k = r.take_count("value")?;
            let mut fields = Vec::with_capacity(k);
            for _ in 0..k {
                fields.push(dec_value(r)?);
            }
            Ok(Value::Tuple(fields))
        }
        TAG_ZOOM => {
            let m = dec_value(r)?;
            let z = dec_value(r)?;
            let (mv, zv) = match (&m, &z) {
                (Value::Int { neg: false, .. }, Value::Int { .. }) => {
                    (mag_to_i128(&m)?, mag_to_i128(&z)?)
                }
                _ => return Err("zoom pair components must be integers".to_string()),
            };
            let zn = normalize_zoom(mv, zv);
            // a non-normalized zoom encoding is malformed (M4)
            if (mv, zv) != zn {
                return Err("non-normalized zoom encoding".to_string());
            }
            Ok(Value::Zoom(mv, zv))
        }
        _ => Err("unknown type tag".to_string()),
    }
}

fn mag_to_i128(v: &Value) -> Result<i128, String> {
    match v {
        Value::Int { neg, mag } => {
            if mag.len() > 2 {
                return Err("zoom component magnitude out of the seed's range".to_string());
            }
            let mut m: u128 = 0;
            for word in mag {
                m = (m << 64) | *word as u128;
            }
            if m > i128::MAX as u128 {
                return Err("zoom component magnitude out of the seed's range".to_string());
            }
            Ok(if *neg { -(m as i128) } else { m as i128 })
        }
        _ => Err("not an integer".to_string()),
    }
}

/// Skips one value encoding without interpreting it (operand scanning).
pub fn skip_value(r: &mut WordReader) -> Result<(), String> {
    let t = r.next("value")?;
    match t {
        TAG_INT_POS | TAG_INT_NEG => {
            let cnt = r.take_count("value")?;
            r.i += cnt;
            Ok(())
        }
        TAG_TUPLE => {
            let k = r.take_count("value")?;
            for _ in 0..k {
                skip_value(r)?;
            }
            Ok(())
        }
        TAG_ZOOM => {
            skip_value(r)?;
            skip_value(r)
        }
        _ => Err("unknown type tag".to_string()),
    }
}
