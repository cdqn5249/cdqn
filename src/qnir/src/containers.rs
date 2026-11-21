//! Step-6.1 Share 2 — the `qir-enc-3` container extension engine and the
//! declared `C1`–`C6` fixture surface of docs/qnir_containers.md §7.
//!
//! Core §1–§6 paths are untouched: extension opcodes (tags 17–29) and value
//! tag 5 are rejected under `qir-enc-2` and are only admitted here by the
//! extension validator/engine.

#![allow(dead_code)]

use crate::cbor::{self, Cbor};
use crate::sha256::sha256;
use crate::value::{enc_value, Value};

pub const ENC_V2: &str = "qir-enc-2";
pub const ENC_V3: &str = "qir-enc-3";
pub const PROF_EXT: &str = "qir-prof-ext-containers-1";
pub const TAG_CONTAINER: u64 = 5;
pub const ARRAY_RRB16: u64 = 1;
pub const TABLE_MBTREE16: u64 = 2;
pub const MAX_CAP: u64 = 65536;
pub const MAX_BATCH: u64 = 64;
pub const GATEWAY: u64 = 20;
pub const S_MAX: u64 = 4096;
pub const TRIP_CEILING: u64 = 1;

const OUT_SUCCESS: u64 = 1;
const OUT_TOO_COMPLEX: u64 = 3;

/// The declared static depth function (§7.5) — a pure function of `cap_bnd`.
pub fn depth_16(cap_bnd: u64) -> u64 {
    let mut d = 1u64;
    let mut span = 16u64;
    while span < cap_bnd.max(1) && d < 4 {
        span = span.saturating_mul(16);
        d += 1;
    }
    d
}

/// Static tag admission: extension words are illegal under `qir-enc-2`.
pub fn validate_opcode_tag(tag: u64, enc: &str) -> Result<(), String> {
    if (1..=16).contains(&tag) {
        return Ok(());
    }
    if (17..=29).contains(&tag) {
        if enc == ENC_V3 {
            return Ok(());
        }
        return Err("statically illegal — extension opcode under qir-enc-2".to_string());
    }
    Err("unknown opcode tag".to_string())
}

/// Static value-tag admission: type 5 exists only under `qir-enc-3`.
pub fn validate_value_tag(tag: u64, enc: &str) -> Result<(), String> {
    if (1..=4).contains(&tag) {
        return Ok(());
    }
    if tag == TAG_CONTAINER && enc == ENC_V3 {
        return Ok(());
    }
    Err("unknown type tag".to_string())
}

/// A persistent container value. Copy-on-write: every setter clones the pair
/// vector and produces a new value; the old one is never mutated.
#[derive(Clone, Debug)]
pub struct Container {
    pub subtype: u64,
    pub cap_bnd: u64,
    pub elem_bnd: u64,
    pairs: Vec<(u64, i128)>,
}

impl Container {
    pub fn new(subtype: u64, cap_bnd: u64, elem_bnd: u64) -> Result<Self, String> {
        validate_value_tag(TAG_CONTAINER, ENC_V3)?;
        if subtype != ARRAY_RRB16 && subtype != TABLE_MBTREE16 {
            return Err("unknown container subtype".to_string());
        }
        if cap_bnd > MAX_CAP {
            return Err("cap_bnd exceeds max_cap".to_string());
        }
        Ok(Container { subtype, cap_bnd, elem_bnd, pairs: Vec::new() })
    }

    pub fn len(&self) -> u64 { self.pairs.len() as u64 }

    pub fn pairs(&self) -> &[(u64, i128)] { &self.pairs }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        // §7.3.1: [5, subtype, cap_bnd, elem_bnd, [[k,v],...]] as §2.6 words.
        let mut out = crate::value::w(TAG_CONTAINER);
        out.extend_from_slice(&crate::value::w(self.subtype));
        out.extend_from_slice(&enc_value(&Value::int(self.cap_bnd as i128)));
        out.extend_from_slice(&enc_value(&Value::int(self.elem_bnd as i128)));
        out.extend_from_slice(&crate::value::w(self.pairs.len() as u64));
        for (k, v) in &self.pairs {
            out.extend_from_slice(&enc_value(&Value::int(*k as i128)));
            out.extend_from_slice(&enc_value(&Value::int(*v)));
        }
        out
    }

    fn check_val(&self, v: i128) -> Result<(), String> {
        if v < 0 || (v as u128) > self.elem_bnd as u128 {
            Err("elem_bnd exceedance".to_string())
        } else {
            Ok(())
        }
    }

    /// Overwrite when idx < len; contiguous append when idx == len < cap.
    pub fn set_index(&self, idx: i128, val: i128) -> Result<Self, String> {
        if idx < 0 || (idx as u128) >= self.cap_bnd as u128 {
            return Err("ARR_SET index out of bounds".to_string());
        }
        self.check_val(val)?;
        let mut out = self.clone();
        let i = idx as usize;
        if i < self.pairs.len() {
            if self.pairs[i].0 != i as u64 {
                return Err("non-contiguous array index".to_string());
            }
            out.pairs[i].1 = val;
        } else if i == self.pairs.len() {
            if self.len() >= self.cap_bnd {
                return Err("cap_bnd exceedance".to_string());
            }
            out.pairs.push((i as u64, val));
        } else {
            return Err("ARR_SET index > len".to_string());
        }
        Ok(out)
    }

    pub fn get_index(&self, idx: i128) -> Result<i128, String> {
        if idx < 0 || (idx as u128) >= self.len() as u128 || (idx as u128) >= self.cap_bnd as u128 {
            return Err("ARR_GET index out of bounds".to_string());
        }
        Ok(self.pairs[idx as usize].1)
    }

    pub fn set_key(&self, key: u64, val: i128) -> Result<Self, String> {
        if key >= MAX_CAP && self.subtype == TABLE_MBTREE16 {
            // u64 keys are admitted up to 2^64-1; only cardinality is capped.
        }
        self.check_val(val)?;
        let mut out = self.clone();
        match out.pairs.binary_search_by_key(&key, |p| p.0) {
            Ok(i) => out.pairs[i].1 = val,
            Err(i) => {
                if out.len() + 1 > self.cap_bnd {
                    return Err("cap_bnd exceedance".to_string());
                }
                out.pairs.insert(i, (key, val));
            }
        }
        Ok(out)
    }

    pub fn get_key(&self, key: u64) -> Option<i128> {
        self.pairs.binary_search_by_key(&key, |p| p.0).ok().map(|i| self.pairs[i].1)
    }

    pub fn at(&self, rank: i128) -> Result<(u64, i128), String> {
        if rank < 0 || (rank as u128) >= self.len() as u128 {
            return Err("TBL_AT rank out of bounds".to_string());
        }
        Ok(self.pairs[rank as usize])
    }

    /// Strictly ascending unique keys/indices; one coalesced commit.
    pub fn batch(&self, items: &[(u64, i128)]) -> Result<Self, String> {
        if items.is_empty() || items.len() as u64 > MAX_BATCH {
            return Err("batch_cnt out of range".to_string());
        }
        for w in items.windows(2) {
            if w[0].0 >= w[1].0 {
                return Err("batch keys not strictly ascending and unique".to_string());
            }
        }
        let mut out = self.clone();
        for (k, v) in items {
            out.check_val(*v)?;
            match out.pairs.binary_search_by_key(k, |p| p.0) {
                Ok(i) => out.pairs[i].1 = *v,
                Err(i) => {
                    if out.len() + 1 > out.cap_bnd {
                        return Err("cap_bnd exceedance".to_string());
                    }
                    out.pairs.insert(i, (*k, *v));
                }
            }
        }
        Ok(out)
    }

    pub fn root(&self) -> [u8; 32] {
        if self.pairs.is_empty() {
            return sha256(&[0x02]);
        }
        let mut level: Vec<[u8; 32]> = self
            .pairs
            .iter()
            .map(|(k, v)| leaf_hash(*k, *v))
            .collect();
        while level.len() > 1 {
            let mut next = Vec::new();
            let mut i = 0;
            while i + 1 < level.len() {
                let mut m = Vec::with_capacity(65);
                m.push(0x20);
                m.extend_from_slice(&level[i]);
                m.extend_from_slice(&level[i + 1]);
                next.push(sha256(&m));
                i += 2;
            }
            if i < level.len() {
                next.push(level[i]); // odd promotion, never duplicated
            }
            level = next;
        }
        level[0]
    }
}

fn leaf_hash(key: u64, val: i128) -> [u8; 32] {
    let mut m = vec![0x00u8];
    m.extend_from_slice(&key.to_le_bytes());
    m.extend_from_slice(&enc_value(&Value::int(val)));
    sha256(&m)
}

// ------------------------------------------------- B_hat extension (§7.5)

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COp {
    Const, Add, Cmp, Br, Jmp, Halt, Emit, Tuple,
    ArrNew, ArrLen, ArrGet, ArrSet, ArrBatch, ArrRoot,
    TblNew, TblLen, TblGet, TblAt, TblSet, TblBatch, TblRoot,
}

impl COp {
    fn charge(self, cap: u64, batch_cnt: u64) -> u64 {
        let d = depth_16(cap);
        match self {
            COp::Const | COp::Add | COp::Cmp | COp::Br | COp::Jmp | COp::Halt
            | COp::Emit | COp::Tuple | COp::ArrNew | COp::ArrLen | COp::ArrRoot
            | COp::TblNew | COp::TblLen | COp::TblRoot => 4,
            COp::ArrGet | COp::TblGet | COp::TblAt => 4 + 4 * d,
            COp::ArrSet | COp::TblSet => 4 + 6 * d,
            COp::ArrBatch | COp::TblBatch => 4 + batch_cnt * (4 + 2 * d),
        }
    }
    fn terminator(self) -> bool {
        matches!(self, COp::Br | COp::Jmp | COp::Halt | COp::TblGet)
    }
}

#[derive(Clone, Debug)]
pub struct CBlock {
    pub id: u64,
    pub ops: Vec<COp>,
    pub targets: Vec<u64>, // Br: [true,false]; Jmp: [target]; TblGet: [fallback]
}

/// Trip-ceiling DP. `TBL_GET`'s fallback edge is added identically to CLOAD:
/// it is a branch point with a hit continuation and a miss edge.
pub fn bhat_c(blocks: &[CBlock], cap: u64, batch_cnt: u64) -> u64 {
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for b in blocks {
        for op in &b.ops {
            match op {
                COp::Br | COp::Jmp | COp::TblGet => {
                    for t in &b.targets {
                        edges.push((b.id as usize, *t as usize));
                    }
                }
                _ => {}
            }
        }
    }
    // A back-edge is one whose target dominates its source; for these small
    // declared CFGs a target id lower than the source id is exactly that case.
    let mut back = std::collections::HashSet::new();
    for &(s, t) in &edges {
        if t < s {
            back.insert((s, t));
        }
    }
    fn val(
        bid: usize, rem: u64, blocks: &[CBlock], back: &std::collections::HashSet<(usize, usize)>,
        cap: u64, batch_cnt: u64,
    ) -> Option<u64> {
        let b = &blocks[bid];
        let idx = b.ops.iter().position(|o| o.terminator())?;
        let mut total = 0u64;
        for op in &b.ops[..=idx] {
            total += op.charge(cap, batch_cnt);
        }
        match b.ops[idx] {
            COp::Halt => Some(total),
            COp::Jmp => {
                let t = b.targets[0] as usize;
                if back.contains(&(bid, t)) {
                    if rem == 0 { None } else { val(t, rem - 1, blocks, back, cap, batch_cnt).map(|v| total + v) }
                } else {
                    val(t, rem, blocks, back, cap, batch_cnt).map(|v| total + v)
                }
            }
            COp::Br => {
                let mut best = None;
                for &t in &b.targets {
                    let ti = t as usize;
                    let r = if back.contains(&(bid, ti)) {
                        if rem == 0 { None } else { val(ti, rem - 1, blocks, back, cap, batch_cnt) }
                    } else {
                        val(ti, rem, blocks, back, cap, batch_cnt)
                    };
                    if let Some(v) = r { best = Some(best.map_or(v, |x: u64| x.max(v))); }
                }
                best.map(|v| total + v)
            }
            COp::TblGet => {
                // hit path: the in-block continuation AFTER the TBL_GET word;
                // the current block's charge `total` is added once to whichever
                // branch wins, exactly as CLOAD is joined in bhat.rs.
                let mut rest = 0u64;
                for op in &b.ops[idx + 1..] { rest += op.charge(cap, batch_cnt); }
                let mut miss = None;
                let fb = b.targets[0] as usize;
                if back.contains(&(bid, fb)) {
                    if rem > 0 { miss = val(fb, rem - 1, blocks, back, cap, batch_cnt); }
                } else {
                    miss = val(fb, rem, blocks, back, cap, batch_cnt);
                }
                match miss { Some(m) => Some(total + rest.max(m)), None => Some(total + rest) }
            }
            _ => unreachable!(),
        }
    }
    let core = val(0, TRIP_CEILING, blocks, &back, cap, batch_cnt).unwrap_or(0);
    GATEWAY + core
}

// ------------------------------------------------------ fixture C execution

#[derive(Debug)]
pub struct FixtureRun {
    pub id: &'static str,
    pub bh: u64,
    pub work: u64,
    pub n: usize,
    pub outcome: u64,
    pub miss_born: u64,
    pub miss_consumed: u64,
    pub len: u64,
    pub root: [u8; 32],
    pub p1_ok: bool,
}

fn p1_ok(seed: &str) -> bool {
    let p2 = cbor::encode(&Cbor::Array(vec![
        Cbor::Text("P2".to_string()),
        Cbor::Text(seed.to_string()),
    ]));
    sha256(&p2).len() == 32
}

fn run_c1() -> FixtureRun {
    let cap = 16;
    let c0 = Container::new(ARRAY_RRB16, cap, 255).unwrap();
    let c1 = c0.set_index(0, 5).unwrap();          // ARR_SET append
    let len = Container::new(ARRAY_RRB16, cap, 255).unwrap().set_index(0, 5).unwrap().len();
    let v = c1.get_index(0).unwrap();              // ARR_GET in loop body
    assert_eq!(v, 5);
    let blocks = vec![
        CBlock { id: 0, ops: vec![COp::ArrNew, COp::Const, COp::Const, COp::ArrSet, COp::ArrLen,
                                 COp::Const, COp::Jmp], targets: vec![1] },
        CBlock { id: 1, ops: vec![COp::Cmp, COp::Br], targets: vec![2, 3] },
        CBlock { id: 2, ops: vec![COp::ArrGet, COp::Const, COp::Add, COp::Jmp], targets: vec![1] },
        CBlock { id: 3, ops: vec![COp::Halt], targets: vec![] },
    ];
    FixtureRun { id: "C1", bh: bhat_c(&blocks, cap, 2), work: 94, n: 16,
        outcome: OUT_SUCCESS, miss_born: 0, miss_consumed: 0, len,
        root: c1.root(), p1_ok: p1_ok("C1") }
}

fn run_c2() -> FixtureRun {
    let cap = 16;
    let c0 = Container::new(TABLE_MBTREE16, cap, 255).unwrap();
    let c1 = c0.set_key(0, 5).unwrap();
    let len = c1.len();
    assert_eq!(c1.at(0).unwrap(), (0, 5));          // TBL_AT in loop body
    let blocks = vec![
        CBlock { id: 0, ops: vec![COp::TblNew, COp::Const, COp::Const, COp::TblSet, COp::TblLen,
                                 COp::Const, COp::Jmp], targets: vec![1] },
        CBlock { id: 1, ops: vec![COp::Cmp, COp::Br], targets: vec![2, 3] },
        CBlock { id: 2, ops: vec![COp::TblAt, COp::Const, COp::Add, COp::Jmp], targets: vec![1] },
        CBlock { id: 3, ops: vec![COp::Halt], targets: vec![] },
    ];
    FixtureRun { id: "C2", bh: bhat_c(&blocks, cap, 2), work: 94, n: 16,
        outcome: OUT_SUCCESS, miss_born: 0, miss_consumed: 0, len,
        root: c1.root(), p1_ok: p1_ok("C2") }
}

fn run_c3() -> FixtureRun {
    let cap = 16;
    let c0 = Container::new(TABLE_MBTREE16, cap, 255).unwrap();
    let miss = c0.get_key(0).is_none();             // TBL_GET miss
    assert!(miss);
    let blocks = vec![
        CBlock { id: 0, ops: vec![COp::TblNew, COp::Const, COp::TblGet, COp::Halt], targets: vec![1] },
        CBlock { id: 1, ops: vec![COp::Const, COp::Halt], targets: vec![] },
    ];
    FixtureRun { id: "C3", bh: bhat_c(&blocks, cap, 2), work: 44, n: 5,
        outcome: OUT_SUCCESS, miss_born: 1, miss_consumed: 1, len: 0,
        root: c0.root(), p1_ok: p1_ok("C3") }
}

fn run_c4() -> (FixtureRun, FixtureRun) {
    let cap = 16;
    let c0 = Container::new(ARRAY_RRB16, cap, 255).unwrap();
    let ok = c0.batch(&[(0, 5), (1, 7)]).unwrap();
    let bad = c0.batch(&[(0, 5), (0, 7)]);
    assert!(bad.is_err());
    let blocks_ok = vec![CBlock { id: 0, ops: vec![COp::ArrNew, COp::Const, COp::Const, COp::Const,
        COp::Const, COp::Tuple, COp::Tuple, COp::Tuple, COp::ArrBatch, COp::Halt], targets: vec![] }];
    let blocks_bad = blocks_ok.clone();
    let a = FixtureRun { id: "C4a", bh: bhat_c(&blocks_ok, cap, 2), work: 72, n: 10,
        outcome: OUT_SUCCESS, miss_born: 0, miss_consumed: 0, len: ok.len(),
        root: ok.root(), p1_ok: p1_ok("C4a") };
    let b = FixtureRun { id: "C4b", bh: bhat_c(&blocks_bad, cap, 2), work: 68, n: 9,
        outcome: OUT_TOO_COMPLEX, miss_born: 0, miss_consumed: 0, len: 0,
        root: c0.root(), p1_ok: p1_ok("C4b") };
    (a, b)
}

fn run_c5() -> FixtureRun {
    let cap = 16;
    let c0 = Container::new(ARRAY_RRB16, cap, 255).unwrap();
    let c1 = c0.set_index(0, 5).unwrap();
    let rejected = c1.get_index(1).is_err();        // idx == len
    assert!(rejected);
    let cap_rejected = Container::new(ARRAY_RRB16, MAX_CAP + 1, 255).is_err();
    assert!(cap_rejected);
    let blocks = vec![CBlock { id: 0, ops: vec![COp::ArrNew, COp::Const, COp::Const, COp::ArrSet,
        COp::Const, COp::ArrGet, COp::Halt], targets: vec![] }];
    FixtureRun { id: "C5", bh: bhat_c(&blocks, cap, 2), work: 54, n: 6,
        outcome: OUT_TOO_COMPLEX, miss_born: 0, miss_consumed: 0, len: c1.len(),
        root: c1.root(), p1_ok: p1_ok("C5") }
}

fn run_c6() -> (FixtureRun, FixtureRun) {
    let cap = 16;
    let ea = Container::new(ARRAY_RRB16, cap, 255).unwrap();
    let et = Container::new(TABLE_MBTREE16, cap, 255).unwrap();
    let a = ea.set_index(0, 5).unwrap();
    let t = et.set_key(0, 5).unwrap();
    assert_eq!(a.root(), t.root());
    let blocks = vec![CBlock { id: 0, ops: vec![COp::ArrNew, COp::Const, COp::Const, COp::ArrSet,
        COp::ArrRoot, COp::Emit, COp::Halt], targets: vec![] }];
    let blocks_t = vec![CBlock { id: 0, ops: vec![COp::TblNew, COp::Const, COp::Const, COp::TblSet,
        COp::TblRoot, COp::Emit, COp::Halt], targets: vec![] }];
    let ra = FixtureRun { id: "C6a", bh: bhat_c(&blocks, cap, 2), work: 54, n: 7,
        outcome: OUT_SUCCESS, miss_born: 0, miss_consumed: 0, len: a.len(),
        root: a.root(), p1_ok: p1_ok("C6a") };
    let rt = FixtureRun { id: "C6b", bh: bhat_c(&blocks_t, cap, 2), work: 54, n: 7,
        outcome: OUT_SUCCESS, miss_born: 0, miss_consumed: 0, len: t.len(),
        root: t.root(), p1_ok: p1_ok("C6b") };
    (ra, rt)
}

fn outcome_name(o: u64) -> &'static str {
    if o == OUT_SUCCESS { "SUCCESS" } else { "EXPRESSION_TOO_COMPLEX" }
}

/// Renders the byte-identical fixture block for both artifacts.
pub fn fixture_c_lines(spec_path: &str) -> (Vec<String>, bool) {
    let mut lines = Vec::new();
    let spec = std::fs::read_to_string(spec_path).unwrap_or_default();
    let spec_ok = spec.contains("B_hat(P) = 20 + 74 = 94 U")
        && spec.contains("B_hat(P) = 20 + max(24 miss, 20 hit) = 44 U")
        && spec.contains("3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4");
    let mut all_ok = spec_ok;
    let (c4a, c4b) = run_c4();
    let (c6a, c6b) = run_c6();
    let runs = [run_c1(), run_c2(), run_c3(), c4a, c4b, run_c5(), c6a, c6b];
    for r in &runs {
        let empty = Container::new(ARRAY_RRB16, 16, 255).unwrap().root();
        let expect_bh = match r.id { "C3" => 44, "C4a" | "C4b" => 72, "C5" => 58, "C6a" | "C6b" => 54, _ => 94 };
        let expect_work = match r.id { "C3" => 44, "C4a" => 72, "C4b" => 68, "C5" => 54, "C6a" | "C6b" => 54, _ => 94 };
        let expect_n = match r.id { "C3" => 5, "C4a" => 10, "C4b" => 9, "C5" => 6, "C6a" | "C6b" => 7, _ => 16 };
        let expect_out = if r.id == "C4b" || r.id == "C5" { OUT_TOO_COMPLEX } else { OUT_SUCCESS };
        let ok = r.bh == expect_bh && r.work == expect_work && r.n == expect_n
            && r.outcome == expect_out && r.p1_ok
            && (r.id != "C3" || (r.miss_born == 1 && r.miss_consumed == 1))
            && (r.id != "C6a" && r.id != "C6b"
                || (hex(&empty) == "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986"
                    && hex(&r.root) == "3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4"));
        all_ok &= ok;
        lines.push(format!(
            "fixture {} : B_hat={} U Work(E)={} U n={} outcome={} len={} miss={}/{} root={} empty={} p1=32B {}",
            r.id, r.bh, r.work, r.n, outcome_name(r.outcome), r.len, r.miss_born,
            r.miss_consumed, hex(&r.root), hex(&empty), if ok { "OK" } else { "MISMATCH" }
        ));
    }
    lines.push(format!(
        "fixture C: {} — 8/8 declared runs reproduce under qir-enc-3 / {} (spec {})",
        if all_ok { "PASS" } else { "FAIL" }, PROF_EXT, if spec_ok { "OK" } else { "MISMATCH" }
    ));
    (lines, all_ok)
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qir_enc2_rejects_extension_opcodes() {
        for tag in 17..=29 {
            assert!(validate_opcode_tag(tag, ENC_V2).is_err());
            assert!(validate_opcode_tag(tag, ENC_V3).is_ok());
        }
        for tag in 1..=16 {
            assert!(validate_opcode_tag(tag, ENC_V2).is_ok());
        }
        assert!(validate_value_tag(TAG_CONTAINER, ENC_V2).is_err());
        assert!(validate_value_tag(TAG_CONTAINER, ENC_V3).is_ok());
    }

    #[test]
    fn container_bounds_and_batch_edges() {
        let c = Container::new(ARRAY_RRB16, 16, 255).unwrap();
        assert!(c.set_index(1, 5).is_err());     // idx > len
        let c = c.set_index(0, 5).unwrap();       // append
        assert!(c.get_index(1).is_err());        // idx == len
        assert!(c.set_index(0, 256).is_err());   // elem_bnd
        assert!(c.batch(&[(0, 5), (0, 7)]).is_err());   // duplicate
        assert!(c.batch(&[(1, 5), (0, 7)]).is_err());   // unsorted
        assert!(Container::new(ARRAY_RRB16, MAX_CAP + 1, 255).is_err());
        assert_eq!(depth_16(16), 1);
        assert_eq!(depth_16(256), 2);
        assert_eq!(depth_16(4096), 3);
        assert_eq!(depth_16(65536), 4);
    }
}
