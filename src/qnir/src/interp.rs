//! The QnIR reference interpreter — conformance §4.3/§4.5, artifact-share
//! increment 3 (R6).
//!
//! Executes the committed transition relation of §2.3 under §2.5's lifecycle
//! algebra and §3's accounting over each committed Family X Intent envelope,
//! and compares the execution against the committed golden expected outcome
//! at the five agreement points of §4.1:
//!   1. value encodings  — the store/argument components of every record's
//!      state roots carry §2.6 word encodings (suite E pins the grammar);
//!   2. record sequence  — every transition record's fourteen fields, in
//!      order, byte-compared against the committed record blocks;
//!   3. state roots      — record fields 1/7/8 (canonical configuration
//!      encodings, P2) and field 2 (program root, P3) byte-compared;
//!   4. terminal receipt — the receipt's canonical CBOR bytes (P8's closed
//!      vocabulary) byte-compared;
//!   5. reconstructed ledger — Work(E), per-class traces, the settlement
//!      trace, and the §3.3 conservation identities compared numerically.
//!
//! The interpreter reads the committed corpus and never generates or edits
//! goldens (§4.2); the transition relation it executes is the committed one,
//! with the artifact-wave parameters of the corpus preamble (P1–P11) and the
//! declared profile `qir-prof-golden-1` (§3.2/§3.6 — profile data, never
//! built-in constants).

use crate::bhat::{self, Profile};
use crate::cbor::{self, Cbor};
use crate::insn::{decode_envelope, op_name, token_type_name, Intent};
use crate::sha256::sha256;
use crate::value::{dec_value, enc_value, skip_value, w, words_of, Value, WordReader};
use std::collections::HashMap;

// ------------------------------------------ artifact-wave parameters (P5–P8)
// Declared by the corpus preamble; §2.6's own receipt vocabulary for the
// state words, §2.5's two tiers, §2.6's closed terminal vocabulary.

const TIER_O: u64 = 1; // P6
const TIER_S: u64 = 2; // P6
const STATE_OUTSTANDING: u64 = 1; // P6
const STATE_CONSUMED: u64 = 2; // P6
const STATE_TERMINALIZED: u64 = 3; // P6

const TT_STATUS_LOOKUP_MISS: u64 = 1; // §2.5 closed vocabulary
const TT_STATUS_NEG_EVEN_ROOT: u64 = 2;
const TT_EXPRESSION_TOO_COMPLEX: u64 = 3;
const TT_STATUS_DOMAIN_ERROR: u64 = 4;

const OUTCOME_NO_SOLUTION: u64 = 2; // P8 (§2.6 closed terminal vocabulary)
const OUTCOME_EXPRESSION_TOO_COMPLEX: u64 = 3;
const OUTCOME_DOMAIN_ERROR: u64 = 4;

fn class_of(tag: u64) -> &'static str {
    match tag {
        1 => "q_const",
        2 => "q_load",
        3 => "q_tuple",
        4 => "q_field",
        5 => "q_add",
        6 => "q_sub",
        7 => "q_mul",
        8 => "q_div",
        9 => "q_mod",
        10 => "q_cmp",
        11 => "q_br",
        12 => "q_jmp",
        13 => "q_obs",
        14 => "q_promote",
        15 => "q_emit",
        16 => "q_halt",
        _ => unreachable!(),
    }
}

fn outcome_name(tag: u64) -> &'static str {
    match tag {
        1 => "SUCCESS",
        2 => "NO_SOLUTION",
        3 => "EXPRESSION_TOO_COMPLEX",
        4 => "DOMAIN_ERROR",
        _ => "?",
    }
}

// ------------------------------------------------------------ exec decoding

/// One instruction of the execution decode: the committed operands, kept,
/// plus the instruction's word span in the stream (record field 5).
#[derive(Clone, Debug)]
enum Exec {
    Const { imm: Value, bound: i128 },
    Cload { addr: u64, fallback: u64, bound: i128 },
    Arith { tag: u64, a: u64, b: u64, bound: i128 },
    Cmp { a: u64, b: u64 },
    Br { cc: u64, cond: u64, t: u64, f: u64 },
    Jmp { target: u64 },
    Obs { ttype: u64 },
    Promote,
    Halt,
}

#[derive(Clone, Debug)]
struct ExecInsn {
    op: Exec,
    start: usize,
    end: usize, // word span [start, end) in the stream's word list
}

#[derive(Clone, Debug)]
struct ExecBlock {
    id: u64,
    insns: Vec<ExecInsn>,
}

fn words_to_bytes(ws: &[u64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(ws.len() * 8);
    for x in ws {
        out.extend_from_slice(&x.to_be_bytes());
    }
    out
}

fn value_as_i128(v: &Value, what: &str) -> Result<i128, String> {
    match v {
        Value::Int { neg, mag } => {
            if mag.len() > 2 {
                return Err(format!("{what} magnitude out of the seed's execution range"));
            }
            let mut m: u128 = 0;
            for word in mag {
                m = (m << 64) | *word as u128;
            }
            if m > i128::MAX as u128 {
                return Err(format!("{what} magnitude out of the seed's execution range"));
            }
            Ok(if *neg { -(m as i128) } else { m as i128 })
        }
        _ => Err(format!("{what} must be an integer")),
    }
}

fn dec_edge_exec(r: &mut WordReader) -> Result<u64, String> {
    let blk = r.next("instruction")?;
    let arity = r.take_count("edge")?;
    for _ in 0..arity {
        r.next("instruction")?;
    }
    Ok(blk)
}

/// Execution decode of the committed instruction stream: per block its id,
/// argument signature, instruction count, and instructions with every
/// operand kept (the transition relation consumes them all).
fn decode_exec_stream(stream: &[u8]) -> Result<Vec<ExecBlock>, String> {
    let words = words_of(stream)?;
    let mut r = WordReader::new(&words);
    let mut blocks = Vec::new();
    while r.i < words.len() {
        let id = r.next("instruction")?;
        let arity = r.take_count("block argument signature")?;
        for _ in 0..arity {
            let t = r.next("instruction")?;
            if t != 1 && t != 3 {
                return Err("unknown result type tag".to_string());
            }
            skip_value(&mut r)?;
        }
        let cnt = r.take_count("instruction count")?;
        let mut insns = Vec::with_capacity(cnt);
        for _ in 0..cnt {
            let start = r.i;
            let tag = r.next("instruction")?;
            let name = op_name(tag).ok_or_else(|| "unknown opcode tag".to_string())?;
            let op = match tag {
                1 => {
                    let imm = dec_value(&mut r)?;
                    let rtag = r.next("instruction")?;
                    if rtag != 1 && rtag != 3 {
                        return Err("unknown result type tag".to_string());
                    }
                    let b = dec_value(&mut r)?;
                    Exec::Const {
                        imm,
                        bound: value_as_i128(&b, "bound")?,
                    }
                }
                2 => {
                    let addr = r.next("instruction")?;
                    let fallback = r.next("instruction")?;
                    let rtag = r.next("instruction")?;
                    if rtag != 1 && rtag != 3 {
                        return Err("unknown result type tag".to_string());
                    }
                    let b = dec_value(&mut r)?;
                    Exec::Cload {
                        addr,
                        fallback,
                        bound: value_as_i128(&b, "bound")?,
                    }
                }
                5 | 6 | 7 | 8 | 9 => {
                    let a = r.next("instruction")?;
                    let b2 = r.next("instruction")?;
                    let rtag = r.next("instruction")?;
                    if rtag != 1 && rtag != 3 {
                        return Err("unknown result type tag".to_string());
                    }
                    let b = dec_value(&mut r)?;
                    Exec::Arith {
                        tag,
                        a,
                        b: b2,
                        bound: value_as_i128(&b, "bound")?,
                    }
                }
                10 => {
                    let a = r.next("instruction")?;
                    let b = r.next("instruction")?;
                    let lo = dec_value(&mut r)?;
                    let hi = dec_value(&mut r)?;
                    value_as_i128(&lo, "range bound")?;
                    value_as_i128(&hi, "range bound")?;
                    Exec::Cmp { a, b }
                }
                11 => {
                    let cc = r.next("instruction")?;
                    if cc > 5 {
                        return Err("unknown condition code".to_string());
                    }
                    let cond = r.next("instruction")?;
                    let t = dec_edge_exec(&mut r)?;
                    let f = dec_edge_exec(&mut r)?;
                    Exec::Br { cc, cond, t, f }
                }
                12 => {
                    let target = dec_edge_exec(&mut r)?;
                    Exec::Jmp { target }
                }
                13 => {
                    let ttype = r.next("instruction")?;
                    if token_type_name(ttype).is_none() {
                        return Err("unknown token type tag".to_string());
                    }
                    let lo = dec_value(&mut r)?;
                    let hi = dec_value(&mut r)?;
                    value_as_i128(&lo, "range bound")?;
                    value_as_i128(&hi, "range bound")?;
                    Exec::Obs { ttype }
                }
                14 => Exec::Promote,
                16 => Exec::Halt,
                _ => {
                    return Err(format!(
                        "opcode {name} is not exercised by the seed corpus"
                    ))
                }
            };
            let _ = r.next("instruction")?; // trailing ext word
            insns.push(ExecInsn {
                op,
                start,
                end: r.i,
            });
        }
        blocks.push(ExecBlock { id, insns });
    }
    if r.i != words.len() {
        return Err("trailing partial instruction".to_string());
    }
    Ok(blocks)
}

// --------------------------------------------------------------- back-edges
// §2.6's back-edge declaration over the static block graph (the same graph
// the §3.4 surrogate reads): BR both edges, JMP its edge, CLOAD its fallback
// edge; a back-edge's target dominates its source.

fn back_edge_set(blocks: &[ExecBlock]) -> std::collections::BTreeSet<(usize, usize)> {
    let n = blocks.len();
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for b in blocks {
        for ins in &b.insns {
            match &ins.op {
                Exec::Br { t, f, .. } => {
                    edges.push((b.id as usize, *t as usize));
                    edges.push((b.id as usize, *f as usize));
                }
                Exec::Jmp { target } => edges.push((b.id as usize, *target as usize)),
                Exec::Cload { fallback, .. } => {
                    edges.push((b.id as usize, *fallback as usize))
                }
                _ => {}
            }
        }
    }
    let mut dom: Vec<Vec<bool>> = (0..n)
        .map(|b| {
            if b == 0 {
                let mut v = vec![false; n];
                v[0] = true;
                v
            } else {
                vec![true; n]
            }
        })
        .collect();
    let mut changed = true;
    while changed {
        changed = false;
        for b in 1..n {
            let preds: Vec<usize> = edges
                .iter()
                .filter(|(_, d)| *d == b)
                .map(|(s, _)| *s)
                .collect();
            if preds.is_empty() {
                continue;
            }
            let mut new = vec![false; n];
            for c in 0..n {
                new[c] = preds.iter().all(|&p| dom[p][c]);
            }
            new[b] = true;
            if new != dom[b] {
                dom[b] = new;
                changed = true;
            }
        }
    }
    edges
        .into_iter()
        .filter(|(s, d)| dom[*s][*d])
        .collect::<std::collections::BTreeSet<_>>()
}

// ----------------------------------------------------------------- machine

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Acc {
    charged: u64,
    reserved: u64,
    refunded: u64,
}

#[derive(Clone, Debug)]
struct Token {
    birth: u64,
    block: u64,
    index: u64,
    serial: u64,
    ttype: u64,
    tier: u64,
    state: u64,
}

fn tid(t: &Token) -> [u64; 5] {
    [t.birth, t.block, t.index, t.serial, t.ttype]
}

fn full_id(t: &Token) -> (u64, u64, u64, u64, u64) {
    (t.birth, t.block, t.index, t.serial, t.ttype)
}

/// One transition record, structured (the fourteen fields of §2.6's F5;
/// fields 1/2/3/13/14 are derivable from the run and carried implicitly:
/// field 1 = the record's pre state, field 2 = the program root, field 3 =
/// the post-transition step index, field 13 = the receipt-order queue,
/// field 14 = the version bindings).
pub(crate) struct Rec {
    pc: (u64, u64),
    decoded: Vec<u8>,
    io_ids: Vec<Vec<u8>>,
    pre: Vec<u8>,
    post: Vec<u8>,
    charged: u64,
    refunded: u64,
    step: u64,
    clock: u64,
    events: Vec<(u64, [u64; 5])>,
}

struct FatalErr {
    ttype: u64,
}

pub(crate) struct RunOutcome {
    pub(crate) outcome: u64,
    pub(crate) records: Vec<Rec>,
    pub(crate) boundaries: Vec<(u64, Acc)>,
    pub(crate) per_class: Vec<(&'static str, u64, u64)>, // class, U, postings
    pub(crate) acc: Acc,
    pub(crate) receipt: Vec<u8>,
    pub(crate) bh: u64,
}

enum Transfer {
    Fall,
    To(u64),
    Miss(Token, u64),
    Halt,
}

fn per_class_add(v: &mut Vec<(&'static str, u64, u64)>, cls: &'static str, amount: u64) {
    for e in v.iter_mut() {
        if e.0 == cls {
            e.1 += amount;
            e.2 += 1;
            return;
        }
    }
    v.push((cls, amount, 1));
}

struct Machine<'a> {
    blocks: Vec<ExecBlock>,
    words: Vec<u64>,
    deps: Vec<Vec<u8>>,
    d: HashMap<Vec<u8>, Vec<u8>>,
    w_budget: u64,
    profile: &'a Profile,
    bh: u64,
    versions: [String; 3],
    back_edges: std::collections::BTreeSet<(usize, usize)>,
    store: HashMap<(u64, u64), Value>,
    current: HashMap<u64, u64>,
    pool: Vec<Token>,
    acc: Acc,
    clock: u64,
    step: u64,
    halt: (u64, u64),
    receipts: Vec<u64>,
    next_iid: u64,
    next_serial: u64,
    escrow_open: bool,
    records: Vec<Rec>,
    boundaries: Vec<(u64, Acc)>,
    per_class: Vec<(&'static str, u64, u64)>,
    traversals: HashMap<(u64, u64, u64), u64>,
    pending_miss: Option<(Token, u64)>,
}

impl<'a> Machine<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        _envelope: &[u8],
        intent: &Intent,
        stream_words: Vec<u64>,
        d: HashMap<Vec<u8>, Vec<u8>>,
        w_budget: u64,
        profile: &'a Profile,
    ) -> Result<Self, String> {
        let blocks = decode_exec_stream(&words_to_bytes(&stream_words))?;
        if blocks.len() as u64 != intent.n_blocks {
            return Err("exec decode block count disagrees with the envelope".to_string());
        }
        let bes = back_edge_set(&blocks);
        let pool: Vec<Token> = intent
            .pool
            .iter()
            .enumerate()
            .map(|(k, &ttype)| Token {
                birth: 0,
                block: 0,
                index: 0,
                serial: k as u64, // serials allocate in declared order (§2.6)
                ttype,
                tier: TIER_S,
                state: STATE_OUTSTANDING,
            })
            .collect();
        Ok(Machine {
            blocks,
            words: stream_words,
            deps: intent.deps.clone(),
            d,
            w_budget,
            profile,
            bh: bhat::b_hat(&intent.blocks, profile),
            versions: [
                intent.sem.clone(),
                intent.enc.clone(),
                intent.prof.clone(),
            ],
            back_edges: bes,
            store: HashMap::new(),
            current: HashMap::new(),
            pool,
            acc: Acc {
                charged: 0,
                reserved: 0,
                refunded: 0,
            },
            clock: 0,
            step: 0,
            halt: (0, 0),
            receipts: Vec::new(),
            next_iid: 0,
            next_serial: intent.pool.len() as u64,
            escrow_open: false,
            records: Vec::new(),
            boundaries: Vec::new(),
            per_class: Vec::new(),
            traversals: HashMap::new(),
            pending_miss: None,
        })
    }

    /// The canonical configuration encoding (P2: §2.2's ten components in
    /// the table's order; a value inside CBOR rides as the byte string of
    /// its §2.6 word encoding).
    fn config_now(&self, pc: (u64, u64)) -> Vec<u8> {
        let mut store_ids: Vec<(u64, u64)> = self.store.keys().copied().collect();
        store_ids.sort();
        let store_c: Vec<Cbor> = store_ids
            .iter()
            .map(|&(sid, iid)| {
                Cbor::Array(vec![
                    Cbor::UInt(sid),
                    Cbor::UInt(iid),
                    Cbor::Bytes(enc_value(&self.store[&(sid, iid)])),
                ])
            })
            .collect();
        let mut pool_sorted: Vec<&Token> = self.pool.iter().collect();
        pool_sorted.sort_by_key(|t| full_id(t));
        let pool_c: Vec<Cbor> = pool_sorted
            .iter()
            .map(|t| {
                Cbor::Array(vec![
                    Cbor::UInt(t.birth),
                    Cbor::UInt(t.block),
                    Cbor::UInt(t.index),
                    Cbor::UInt(t.serial),
                    Cbor::UInt(t.ttype),
                    Cbor::UInt(t.tier),
                    Cbor::UInt(t.state),
                ])
            })
            .collect();
        let comps = Cbor::Array(vec![
            Cbor::Array(vec![Cbor::UInt(pc.0), Cbor::UInt(pc.1)]),
            Cbor::Array(Vec::new()), // block-argument environment (seed: arity 0)
            Cbor::Array(store_c),
            Cbor::Array(pool_c),
            Cbor::Array(vec![
                Cbor::UInt(self.acc.charged),
                Cbor::UInt(self.acc.reserved),
                Cbor::UInt(self.acc.refunded),
            ]),
            Cbor::UInt(self.clock),
            Cbor::UInt(self.step),
            Cbor::Array(vec![Cbor::UInt(self.halt.0), Cbor::UInt(self.halt.1)]),
            Cbor::Array(self.receipts.iter().map(|&r| Cbor::UInt(r)).collect()),
            Cbor::Array(
                self.versions
                    .iter()
                    .map(|v| Cbor::Text(v.clone()))
                    .collect(),
            ),
        ]);
        cbor::encode(&comps)
    }

    /// Record field 5: the full canonical bytes of the executed instruction.
    fn decoded_at(&self, blk: usize, idx: usize) -> Vec<u8> {
        let ins = &self.blocks[blk].insns[idx];
        let mut out = Vec::new();
        for k in ins.start..ins.end {
            out.extend_from_slice(&w(self.words[k]));
        }
        out
    }

    fn outstanding_escrow(&self) -> u64 {
        self.acc.reserved - self.acc.charged - self.acc.refunded
    }

    fn mint(&mut self, sid: u64, value: Value) {
        let iid = self.next_iid;
        self.next_iid += 1;
        self.store.insert((sid, iid), value);
        self.current.insert(sid, iid);
    }

    fn read_i(&self, sid: u64) -> i128 {
        let iid = self.current[&sid];
        match &self.store[&(sid, iid)] {
            Value::Int { neg, mag } => {
                let mut m: u128 = 0;
                for word in mag {
                    m = (m << 64) | *word as u128;
                }
                let v = m as i128;
                if *neg {
                    -v
                } else {
                    v
                }
            }
            _ => panic!("seed execution reads only integer values"),
        }
    }

    fn birth_token(&mut self, ttype: u64, pc: (u64, u64), birth_step: u64, tier: u64) -> Token {
        let t = Token {
            birth: birth_step,
            block: pc.0,
            index: pc.1,
            serial: self.next_serial,
            ttype,
            tier,
            state: STATE_OUTSTANDING,
        };
        self.next_serial += 1;
        self.pool.push(t.clone());
        t
    }

    /// Consumes the pending CLOAD-miss token when its fallback block
    /// completes (§2.5); the consuming transition's events carry the trace.
    fn consume_pending_miss(&mut self, blk: usize, idx: usize, events: &mut Vec<(u64, [u64; 5])>) {
        let pm = self.pending_miss.clone();
        if let Some((tok, fb)) = pm {
            if fb as usize == blk && idx == self.blocks[blk].insns.len() - 1 {
                let id = full_id(&tok);
                if let Some(pt) = self.pool.iter_mut().find(|p| full_id(p) == id) {
                    pt.state = STATE_CONSUMED;
                    events.push((2, tid(pt)));
                }
                self.pending_miss = None;
            }
        }
    }

    fn run(mut self) -> Result<RunOutcome, String> {
        if self.w_budget < self.bh {
            // admission refusal (§3.4): the single rejection transition,
            // zero postings
            let pre = self.config_now((0, 0));
            let decoded = self.decoded_at(0, 0);
            let outcome = self.fatal(TT_EXPRESSION_TOO_COMPLEX, (0, 0), decoded, pre);
            return Ok(self.pack(outcome));
        }
        // static value ids are dense over the Intent's value-defining
        // instructions in block-then-index scan order (§2.6, P10)
        let mut sid_counter: usize = 0;
        let mut sids: Vec<Vec<Option<u64>>> = Vec::new();
        for b in &self.blocks {
            let mut row = Vec::new();
            for ins in &b.insns {
                if matches!(
                    ins.op,
                    Exec::Const { .. }
                        | Exec::Cload { .. }
                        | Exec::Arith { .. }
                        | Exec::Cmp { .. }
                        | Exec::Obs { .. }
                ) {
                    row.push(Some(sid_counter as u64));
                    sid_counter += 1;
                } else {
                    row.push(None);
                }
            }
            sids.push(row);
        }
        let mut pc: (u64, u64) = (0, 0);
        let mut pre = self.config_now(pc);
        loop {
            let blk = pc.0 as usize;
            let idx = pc.1 as usize;
            let ins = self.blocks[blk].insns[idx].clone();
            let decoded = self.decoded_at(blk, idx);
            let tag = words_of(&decoded)?[0];
            let c = self.profile.charge[tag as usize];
            let mut events: Vec<(u64, [u64; 5])> = Vec::new();
            let mut io_ids: Vec<Vec<u8>> = Vec::new();
            let mut gateway = 0u64;
            if !self.escrow_open {
                // admission passed (§3.4); the escrow opens with this
                // transition
                self.escrow_open = true;
                self.acc.reserved += self.bh; // reserve B(P) in full (§3.3)
                gateway = self.profile.gateway;
            } else if self.outstanding_escrow() < c {
                // fuel backstop (§3.4): after admission, defense in depth
                let outcome = self.fatal(TT_EXPRESSION_TOO_COMPLEX, pc, decoded, pre);
                return Ok(self.pack(outcome));
            }
            let transfer =
                match self.dispatch(&ins.op, pc, &mut events, &mut io_ids, &sids) {
                    Ok(t) => t,
                    Err(f) => {
                        // the raised type names the declared rejection surface
                        let outcome = self.fatal(f.ttype, pc, decoded, pre);
                        return Ok(self.pack(outcome));
                    }
                };
            let transfer = transfer.unwrap_or(Transfer::Fall);
            let post_charge = gateway + c;
            self.acc.charged += post_charge;
            per_class_add(&mut self.per_class, class_of(tag), c);
            if gateway > 0 {
                per_class_add(&mut self.per_class, "gateway", gateway);
            }
            self.step += 1;
            self.clock += 1; // one tick per transition (§2.2)
            if matches!(transfer, Transfer::Halt) {
                // fallback-block completion consumption precedes terminalization
                self.consume_pending_miss(blk, idx, &mut events);
                let outcome = self.terminalize(&mut events);
                self.halt = (1, outcome);
                let refund = self.outstanding_escrow();
                if refund > 0 {
                    self.acc.refunded += refund;
                }
                let post = self.config_now(pc);
                self.records.push(Rec {
                    pc,
                    decoded,
                    io_ids,
                    pre,
                    post,
                    charged: post_charge,
                    refunded: refund,
                    step: self.step,
                    clock: self.clock,
                    events,
                });
                self.boundaries.push((self.step, self.acc));
                return Ok(self.pack(outcome));
            }
            // successful completion of the fallback block (its terminator fired)
            self.consume_pending_miss(blk, idx, &mut events);
            let post = self.config_now(pc);
            self.records.push(Rec {
                pc,
                decoded,
                io_ids,
                pre,
                post: post.clone(),
                charged: post_charge,
                refunded: 0,
                step: self.step,
                clock: self.clock,
                events,
            });
            self.boundaries.push((self.step, self.acc));
            pre = post;
            match transfer {
                Transfer::Miss(tok, fb) => {
                    self.pending_miss = Some((tok, fb));
                    pc = (fb, 0);
                }
                Transfer::To(tgt) => pc = (tgt, 0),
                Transfer::Fall => pc = (blk as u64, idx as u64 + 1),
                Transfer::Halt => unreachable!(),
            }
        }
    }

    fn dispatch(
        &mut self,
        op: &Exec,
        pc: (u64, u64),
        events: &mut Vec<(u64, [u64; 5])>,
        io_ids: &mut Vec<Vec<u8>>,
        sids: &[Vec<Option<u64>>],
    ) -> Result<Option<Transfer>, FatalErr> {
        let etc = || FatalErr { ttype: TT_EXPRESSION_TOO_COMPLEX };
        match op {
            Exec::Const { imm, bound } => {
                let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                match value_as_i128(imm, "immediate") {
                    Ok(v) => {
                        if v > *bound {
                            return Err(etc());
                        }
                        self.mint(sid, Value::int(v));
                    }
                    Err(_) => {
                        // a non-integer constant carries no §3.2 bound surface
                        // in the seed; minted as declared
                        self.mint(sid, imm.clone());
                    }
                }
                Ok(None)
            }
            Exec::Cload { addr, fallback, bound } => {
                let addr_bytes = self.deps[*addr as usize].clone();
                io_ids.push(addr_bytes.clone());
                if let Some(vb) = self.d.get(&addr_bytes) {
                    let ws = words_of(vb).map_err(|_| etc())?;
                    let mut r = WordReader::new(&ws);
                    let v = dec_value(&mut r).map_err(|_| etc())?;
                    let vi = value_as_i128(&v, "dependency value").map_err(|_| etc())?;
                    if vi > *bound {
                        return Err(etc());
                    }
                    let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                    self.mint(sid, v);
                    Ok(None)
                } else {
                    // §2.5: the miss status token; the declared Tier O rides
                    // the token's tier word (P6)
                    let tok = self.birth_token(TT_STATUS_LOOKUP_MISS, pc, self.step + 1, TIER_O);
                    events.push((1, tid(&tok)));
                    Ok(Some(Transfer::Miss(tok, *fallback)))
                }
            }
            Exec::Arith { tag, a, b, bound } => {
                let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                if *tag == 8 {
                    // §2.3's division rule: the zero divisor births the
                    // domain error; the quotient truncates toward zero
                    let y = self.read_i(*b);
                    if y == 0 {
                        return Err(FatalErr { ttype: TT_STATUS_DOMAIN_ERROR });
                    }
                    let x = self.read_i(*a);
                    let q = x / y;
                    if q > *bound {
                        return Err(etc());
                    }
                    self.mint(sid, Value::int(q));
                    return Ok(None);
                }
                if *tag == 9 {
                    // §2.3's remainder rule: the zero divisor births the
                    // domain error; the committed remainder is non-negative
                    let y = self.read_i(*b);
                    if y == 0 {
                        return Err(FatalErr { ttype: TT_STATUS_DOMAIN_ERROR });
                    }
                    let x = self.read_i(*a);
                    let r = (x % y).abs();
                    if r > *bound {
                        return Err(etc());
                    }
                    self.mint(sid, Value::int(r));
                    return Ok(None);
                }
                let (x, y) = (self.read_i(*a), self.read_i(*b));
                // §2.3's arithmetic definitions: ADD sums, SUB subtracts,
                // MUL multiplies. (The seed exercises only ADD; the
                // derivation tool's SUB/MUL aliases were seed-harmless —
                // the interpreter follows the committed rule text.)
                let r = match tag {
                    5 => x.checked_add(y),
                    6 => x.checked_sub(y),
                    7 => x.checked_mul(y),
                    _ => unreachable!(),
                };
                let r = match r {
                    Some(v) => v,
                    None => return Err(etc()),
                };
                if r > *bound {
                    return Err(etc());
                }
                let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                self.mint(sid, Value::int(r));
                Ok(None)
            }
            Exec::Cmp { a, b } => {
                let (x, y) = (self.read_i(*a), self.read_i(*b));
                let r = if x < y {
                    -1
                } else if x > y {
                    1
                } else {
                    0
                };
                let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                self.mint(sid, Value::int(r));
                Ok(None)
            }
            Exec::Br { cc, cond, t, f } => {
                let r = self.read_i(*cond);
                let take_true = match cc {
                    0 => r == -1, // LT
                    1 => r <= 0,  // LE
                    2 => r == 0,  // EQ
                    3 => r != 0,  // NE
                    4 => r == 1,  // GT
                    5 => r >= 0,  // GE
                    _ => unreachable!(),
                };
                let tgt = if take_true { *t } else { *f };
                let slot = if take_true { 1u64 } else { 0u64 };
                self.check_ceiling(pc, tgt, slot)?;
                *self.traversals.entry((pc.0, pc.1, slot)).or_insert(0) += 1;
                Ok(Some(Transfer::To(tgt)))
            }
            Exec::Jmp { target } => {
                self.check_ceiling(pc, *target, 0)?;
                *self.traversals.entry((pc.0, pc.1, 0)).or_insert(0) += 1;
                Ok(Some(Transfer::To(*target)))
            }
            Exec::Obs { ttype } => {
                let present = self
                    .pool
                    .iter()
                    .any(|t| t.ttype == *ttype && t.state == STATE_OUTSTANDING);
                let sid = sids[pc.0 as usize][pc.1 as usize].unwrap();
                self.mint(sid, Value::int(if present { 1 } else { 0 }));
                Ok(None)
            }
            Exec::Promote => {
                // §2.5's consume rule: the earliest-birth outstanding NER
                let ners: Vec<(u64, u64, u64, u64, u64)> = self
                    .pool
                    .iter()
                    .filter(|t| {
                        t.ttype == TT_STATUS_NEG_EVEN_ROOT && t.state == STATE_OUTSTANDING
                    })
                    .map(full_id)
                    .collect();
                if let Some(sel) = ners.iter().copied().min() {
                    if let Some(pt) = self.pool.iter_mut().find(|t| full_id(t) == sel) {
                        pt.state = STATE_CONSUMED;
                        events.push((2, tid(pt)));
                    }
                }
                Ok(None)
            }
            Exec::Halt => Ok(Some(Transfer::Halt)),
        }
    }

    fn check_ceiling(&self, pc: (u64, u64), tgt: u64, slot: u64) -> Result<(), FatalErr> {
        if self.back_edges.contains(&(pc.0 as usize, tgt as usize)) {
            let site = (pc.0, pc.1, slot);
            let n = *self.traversals.get(&site).unwrap_or(&0);
            if n + 1 > self.profile.trip_ceiling as u64 {
                return Err(FatalErr { ttype: TT_EXPRESSION_TOO_COMPLEX });
            }
        }
        Ok(())
    }

    /// T (§2.5): the earliest-birth outstanding Tier S token decides the
    /// outcome; every outstanding token terminalizes and is traced.
    fn terminalize(&mut self, events: &mut Vec<(u64, [u64; 5])>) -> u64 {
        let outs: Vec<(u64, u64, u64, u64, u64)> = self
            .pool
            .iter()
            .filter(|t| t.state == STATE_OUTSTANDING && t.tier == TIER_S)
            .map(full_id)
            .collect();
        let outcome = if let Some(sel) = outs.iter().min() {
            match sel.4 {
                TT_STATUS_NEG_EVEN_ROOT => OUTCOME_NO_SOLUTION,
                TT_EXPRESSION_TOO_COMPLEX => OUTCOME_EXPRESSION_TOO_COMPLEX,
                TT_STATUS_DOMAIN_ERROR => OUTCOME_DOMAIN_ERROR,
                _ => panic!("the terminal vocabulary does not cover this token type"),
            }
        } else {
            1 // SUCCESS
        };
        for t in self.pool.iter_mut() {
            if t.state == STATE_OUTSTANDING {
                t.state = STATE_TERMINALIZED;
                events.push((3, tid(t)));
            }
        }
        outcome
    }

    /// A typed rejection (§2.5): posts nothing, births the fatal token,
    /// is final; the terminal settlement posts into this record (§3.3).
    fn fatal(&mut self, ttype: u64, pc: (u64, u64), decoded: Vec<u8>, pre: Vec<u8>) -> u64 {
        self.step += 1;
        self.clock += 1;
        let tok = self.birth_token(ttype, pc, self.step, TIER_S);
        let id = full_id(&tok);
        if let Some(pt) = self.pool.iter_mut().find(|p| full_id(p) == id) {
            pt.state = STATE_TERMINALIZED;
        }
        let outcome = if ttype == TT_EXPRESSION_TOO_COMPLEX {
            OUTCOME_EXPRESSION_TOO_COMPLEX
        } else {
            OUTCOME_DOMAIN_ERROR
        };
        self.halt = (1, outcome);
        let mut refund = 0u64;
        if self.escrow_open {
            refund = self.outstanding_escrow();
            if refund > 0 {
                self.acc.refunded += refund;
            }
        }
        let post = self.config_now(pc);
        self.records.push(Rec {
            pc,
            decoded,
            io_ids: Vec::new(),
            pre,
            post,
            charged: 0,
            refunded: refund,
            step: self.step,
            clock: self.clock,
            events: vec![(1, tid(&tok))],
        });
        self.boundaries.push((self.step, self.acc));
        outcome
    }

    /// The terminal receipt (P7/P8; §2.6's deterministic CBOR map, closed
    /// keys 0–4).
    fn build_receipt(&self, outcome: u64) -> Vec<u8> {
        let mut sorted: Vec<&Token> = self.pool.iter().collect();
        sorted.sort_by_key(|t| full_id(t));
        let item: Vec<Cbor> = sorted
            .iter()
            .map(|t| {
                Cbor::Array(vec![
                    Cbor::UInt(t.birth),
                    Cbor::UInt(t.block),
                    Cbor::UInt(t.index),
                    Cbor::UInt(t.serial),
                    Cbor::UInt(t.ttype),
                    Cbor::UInt(t.tier),
                    Cbor::UInt(t.state),
                ])
            })
            .collect();
        let selected: Cbor =
            if outcome == OUTCOME_NO_SOLUTION || outcome == OUTCOME_DOMAIN_ERROR {
                let cands: Vec<[u64; 5]> = self
                    .pool
                    .iter()
                    .filter(|t| {
                        t.state == STATE_TERMINALIZED
                            && (t.ttype == TT_STATUS_NEG_EVEN_ROOT
                                || t.ttype == TT_STATUS_DOMAIN_ERROR)
                    })
                    .map(|t| [t.birth, t.block, t.index, t.serial, t.ttype])
                    .collect();
                match cands.iter().copied().min() {
                    Some(tid5) => {
                        Cbor::Array(tid5.iter().map(|&v| Cbor::UInt(v)).collect())
                    }
                    None => Cbor::Null,
                }
            } else {
                Cbor::Null
            };
        cbor::encode(&Cbor::Map(vec![
            (Cbor::UInt(0), Cbor::Bytes(b"QnIR-Terminal".to_vec())),
            (Cbor::UInt(1), Cbor::UInt(outcome)),
            (Cbor::UInt(2), Cbor::Array(item)),
            (Cbor::UInt(3), selected),
            (
                Cbor::UInt(4),
                Cbor::Array(
                    self.versions
                        .iter()
                        .map(|v| Cbor::Text(v.clone()))
                        .collect(),
                ),
            ),
        ]))
    }

    fn pack(self, outcome: u64) -> RunOutcome {
        let receipt = self.build_receipt(outcome);
        RunOutcome {
            outcome,
            records: self.records,
            boundaries: self.boundaries,
            per_class: self.per_class,
            acc: self.acc,
            receipt,
            bh: self.bh,
        }
    }
}

fn envelope_stream_words(envelope: &[u8]) -> Result<Vec<u64>, String> {
    let decoded = cbor::decode(envelope).map_err(|e| e.0)?;
    match decoded {
        Cbor::Map(entries) => {
            for (k, v) in entries {
                if k == Cbor::UInt(6) {
                    return match v {
                        Cbor::Bytes(b) => words_of(&b),
                        _ => Err("the instruction stream must be a byte string".to_string()),
                    };
                }
            }
            Err("the Intent envelope has no instruction stream".to_string())
        }
        _ => Err("the Intent envelope is not a map".to_string()),
    }
}

/// Runs one execution vector from its committed envelope bytes against the
/// declared budget and the declared profile; the seed's dependency store is
/// the declared "no entries" table.
pub fn run_vector(
    envelope: &[u8],
    w_budget: u64,
    profile: &Profile,
) -> Result<RunOutcome, String> {
    let intent = decode_envelope(envelope)?;
    let stream_words = envelope_stream_words(envelope)?;
    let machine = Machine::new(
        envelope,
        &intent,
        stream_words,
        HashMap::new(),
        w_budget,
        profile,
    )?;
    machine.run()
}

// =========================================================== expected blocks
// The committed golden expected outcome of a Family X vector (§4.4): the
// ordered transition-record sequence, the terminal receipt with its token
// itemization, and the reconstructed ledger with the §3.3 conservation
// identities. These parsers read the committed text; the comparison is
// field-by-field against the executed run.

struct RecTxt {
    class: String,        // "completed" | "rejection (posts nothing)"
    fields: Vec<String>,  // the 14 field bodies, in committed order
}

fn parse_record_block(text: &str) -> RecTxt {
    let mut class = String::new();
    let mut fields: Vec<String> = Vec::new();
    for line in text.lines() {
        let lead = line.len() - line.trim_start().len();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if class.is_empty() {
            // "record N — completed" / "record N — rejection (posts nothing)"
            class = trimmed
                .splitn(2, '\u{2014}')
                .nth(1)
                .unwrap_or("")
                .trim()
                .to_string();
            continue;
        }
        if lead >= 20 {
            // a wrapped continuation of the previous field's body
            let last = fields
                .last_mut()
                .expect("record continuation without a field");
            last.push(' ');
            last.push_str(trimmed);
        } else {
            // " N  name  body" — split off the number and name, keep the rest
            let num_end = trimmed.find(char::is_whitespace).expect("field number");
            let _num: usize = trimmed[..num_end].parse().expect("record field number");
            let rest = trimmed[num_end..].trim_start();
            let name_end = rest.find(char::is_whitespace).expect("field name");
            let _name = &rest[..name_end];
            fields.push(rest[name_end..].trim_start().to_string());
        }
    }
    assert!(
        fields.len() == 14,
        "a committed record block must carry 14 fields, got {}",
        fields.len()
    );
    RecTxt { class, fields }
}

#[derive(Clone, Debug)]
enum Iv {
    N(i128),
    A(Vec<Iv>),
}

fn parse_iv(s: &str) -> Iv {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix('[') {
        let inner = inner.strip_suffix(']').expect("bracketed int array");
        let mut parts: Vec<String> = Vec::new();
        let mut depth = 0usize;
        let mut cur = String::new();
        for c in inner.chars() {
            match c {
                '[' => {
                    depth += 1;
                    cur.push(c);
                }
                ']' => {
                    depth -= 1;
                    cur.push(c);
                }
                ',' if depth == 0 => {
                    parts.push(std::mem::take(&mut cur));
                }
                _ => cur.push(c),
            }
        }
        if !cur.trim().is_empty() {
            parts.push(cur);
        }
        Iv::A(parts.iter().map(|p| parse_iv(p)).collect())
    } else {
        Iv::N(s.parse().expect("integer literal"))
    }
}

fn iv_nums(v: &Iv) -> Vec<i128> {
    match v {
        Iv::N(n) => vec![*n],
        Iv::A(items) => items.iter().flat_map(iv_nums).collect(),
    }
}

fn iv_elems(v: &Iv) -> Vec<Iv> {
    match v {
        Iv::A(items) => items.clone(),
        _ => panic!("expected an int array"),
    }
}

fn all_nums(s: &str) -> Vec<u64> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            cur.push(c);
        } else if !cur.is_empty() {
            out.push(cur.parse().expect("number"));
            cur.clear();
        }
    }
    if !cur.is_empty() {
        out.push(cur.parse().expect("number"));
    }
    out
}

struct RcptTxt {
    outcome_tag: u64,
    itemization: Vec<[u64; 7]>,
    /// The table's selected-token rendering: a committed tid list, or the
    /// nil-form annotations ("nil"/"true" — the canonical bytes decide).
    selected: Option<[u64; 5]>,
    bytes: Vec<u8>,
}

fn parse_receipt_block(text: &str) -> RcptTxt {
    let mut outcome_tag = 0u64;
    let mut itemization: Vec<[u64; 7]> = Vec::new();
    let mut selected = None;
    let mut bytes = String::new();
    let mut in_bytes = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "canonical bytes:" {
            in_bytes = true;
            continue;
        }
        if in_bytes {
            bytes.push_str(trimmed);
            continue;
        }
        if trimmed.starts_with("1 ") && trimmed.contains("outcome_tag") {
            let after = trimmed.split("outcome_tag").nth(1).unwrap();
            outcome_tag = all_nums(after)[0];
        } else if trimmed.starts_with('[') {
            let nums = iv_nums(&parse_iv(trimmed.split('#').next().unwrap()));
            let a: [u64; 7] = nums
                .iter()
                .map(|&n| n as u64)
                .collect::<Vec<u64>>()
                .try_into()
                .expect("a 7-word token entry");
            itemization.push(a);
        } else if trimmed.starts_with("3 ") && trimmed.contains("selected_token") {
            let rest = trimmed
                .split("selected_token")
                .nth(1)
                .unwrap()
                .trim();
            if rest.starts_with('[') {
                let nums = iv_nums(&parse_iv(rest.split('#').next().unwrap()));
                selected = Some(
                    nums.iter()
                        .map(|&n| n as u64)
                        .collect::<Vec<u64>>()
                        .try_into()
                        .expect("a 5-word token id"),
                );
            }
            // "nil"/"true" stay None: the canonical bytes decide (see below)
        }
    }
    RcptTxt {
        outcome_tag,
        itemization,
        selected,
        bytes: crate::hex_to_bytes(&bytes),
    }
}

struct LedgerTxt {
    work_e: u64,
    per_class: Vec<(String, u64, u64)>, // class, U, postings
    settlement: Vec<(u64, u64, u64, u64)>, // step, charged, reserved, refunded
    ident1: (u64, u64, u64, u64),       // reserved = charged + refunded + outst.
    ident2: (u64, u64, u64),            // W_budget - charged = remaining
    /// Ok((B(P), charged, refund)) or the declared escrow-never-opened form.
    refund_final: Result<(u64, u64, u64), ()>,
    outstanding_final: u64,
}

fn parse_ledger_block(text: &str) -> LedgerTxt {
    let mut work_e = 0u64;
    let mut per_class = Vec::new();
    let mut settlement = Vec::new();
    let mut ident1 = (0u64, 0u64, 0u64, 0u64);
    let mut ident2 = (0u64, 0u64, 0u64);
    let mut refund_final: Result<(u64, u64, u64), ()> = Err(());
    let mut outstanding_final = 0u64;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("Work(E)") {
            work_e = all_nums(trimmed)[0];
        } else if trimmed.starts_with("step ")
            && trimmed.contains(':')
            && trimmed.contains('/')
        {
            let n = all_nums(trimmed);
            settlement.push((n[0], n[1], n[2], n[3]));
        } else if trimmed.starts_with("reserved = charged") {
            let n = all_nums(trimmed);
            ident1 = (n[0], n[1], n[2], n[3]);
        } else if trimmed.starts_with("remaining_budget") {
            let n = all_nums(trimmed);
            ident2 = (n[0], n[1], n[2]);
        } else if trimmed.starts_with("refund_final") {
            let n = all_nums(trimmed);
            if n.len() == 3 {
                refund_final = Ok((n[0], n[1], n[2]));
            } else {
                // the declared escrow-never-opened form (admission refusal)
                refund_final = Err(());
            }
        } else if trimmed.starts_with("reserve_outstanding after") {
            outstanding_final = all_nums(trimmed)[0];
        } else {
            let first = trimmed.split_whitespace().next().unwrap_or("");
            let n = all_nums(trimmed);
            if (first.starts_with("q_") || first == "gateway") && n.len() == 2 {
                per_class.push((first.to_string(), n[0], n[1]));
            }
        }
    }
    LedgerTxt {
        work_e,
        per_class,
        settlement,
        ident1,
        ident2,
        refund_final,
        outstanding_final,
    }
}

fn between<'a>(s: &'a str, start_marker: &str, end_marker: &str) -> &'a str {
    let i = s.find(start_marker).unwrap_or_else(|| panic!("missing marker: {start_marker}"));
    let rest = &s[i..];
    match rest.find(end_marker) {
        Some(j) => &rest[..j],
        None => rest,
    }
}

fn interp_selected(receipt_bytes: &[u8]) -> Result<Option<[u64; 5]>, String> {
    match cbor::decode(receipt_bytes).map_err(|e| e.0)? {
        Cbor::Map(entries) => {
            for (k, v) in entries {
                if k == Cbor::UInt(3) {
                    return match v {
                        Cbor::Null => Ok(None),
                        Cbor::Array(items) => {
                            let mut out = [0u64; 5];
                            for (i, it) in items.iter().enumerate() {
                                match it {
                                    Cbor::UInt(n) => out[i] = *n,
                                    _ => return Err("selected token id must be words".into()),
                                }
                            }
                            Ok(Some(out))
                        }
                        _ => Err("selected token must be an array or nil".into()),
                    };
                }
            }
            Err("the receipt has no selected-token key".to_string())
        }
        _ => Err("the receipt is not a map".to_string()),
    }
}

fn interp_itemization(receipt_bytes: &[u8]) -> Result<Vec<[u64; 7]>, String> {
    match cbor::decode(receipt_bytes).map_err(|e| e.0)? {
        Cbor::Map(entries) => {
            for (k, v) in entries {
                if k == Cbor::UInt(2) {
                    return match v {
                        Cbor::Array(items) => {
                            let mut out = Vec::new();
                            for it in items {
                                if let Cbor::Array(fields) = it {
                                    let mut row = [0u64; 7];
                                    for (i, f) in fields.iter().enumerate() {
                                        match f {
                                            Cbor::UInt(n) => row[i] = *n,
                                            _ => {
                                                return Err(
                                                    "token entries must be words".into()
                                                )
                                            }
                                        }
                                    }
                                    out.push(row);
                                } else {
                                    return Err("token entries must be arrays".into());
                                }
                            }
                            Ok(out)
                        }
                        _ => Err("token itemization must be an array".into()),
                    };
                }
            }
            Err("the receipt has no itemization key".to_string())
        }
        _ => Err("the receipt is not a map".to_string()),
    }
}

/// Every maximal run of exactly 16 hex characters (an io address token).
fn hex_tokens(s: &str) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c.is_ascii_hexdigit() {
            cur.push(c);
        } else {
            if cur.len() == 16 {
                out.push(crate::hex_to_bytes(&cur));
            }
            cur.clear();
        }
    }
    if cur.len() == 16 {
        out.push(crate::hex_to_bytes(&cur));
    }
    out
}

fn cmp_fail(id: &str, name: &str, what: &str, rep: &mut crate::Report) -> bool {
    rep.fail(format!("{id} {name}: {what}"));
    false
}

/// Executes one vector and compares the run against its committed expected
/// outcome (the first divergence is reported; the vector then stops).
fn exec_vector_check(
    id: &str,
    name: &str,
    section: &str,
    envelope: &[u8],
    params: &crate::XParams,
    profile: &Profile,
    rep: &mut crate::Report,
) -> bool {
    // ---- the vector's committed expected blocks (§4.4)
    let recs_region = between(
        section,
        "Expected transition records",
        "Expected terminal receipt:",
    );
    let header = section
        .lines()
        .find(|l| l.trim().starts_with("Expected transition records"))
        .expect("the records header line");
    let declared_count = all_nums(header.split('(').nth(1).expect("(N records"))[0] as usize;
    let rec_fences = crate::fences(recs_region);
    let receipt_fence = crate::fences(between(
        section,
        "Expected terminal receipt:",
        "Expected reconstructed ledger",
    ));
    assert!(receipt_fence.len() == 1, "one committed receipt block");
    let rcpt = parse_receipt_block(&receipt_fence[0].body);
    let ledger_fence = crate::fences(between(
        section,
        "Expected reconstructed ledger",
        "\n### ",
    ));
    assert!(ledger_fence.len() == 1, "one committed ledger block");
    let ledger = parse_ledger_block(&ledger_fence[0].body);

    // ---- run the committed transition relation over the committed envelope
    let outcome = match run_vector(envelope, params.w_budget, profile) {
        Ok(o) => o,
        Err(e) => {
            return cmp_fail(
                id,
                name,
                &format!("the reference interpreter rejected the run — {e}"),
                rep,
            );
        }
    };

    // ---- agreement point: the terminal outcome
    if outcome.outcome != params.expected_outcome_tag {
        return cmp_fail(
            id,
            name,
            &format!(
                "outcome {} (tag {}) but the committed expectation is {} (tag {})",
                outcome_name(outcome.outcome),
                outcome.outcome,
                params.expected_outcome_name,
                params.expected_outcome_tag
            ),
            rep,
        );
    }

    // ---- agreement points: record sequence + state roots (fields 1–14)
    if declared_count != outcome.records.len() || rec_fences.len() != outcome.records.len() {
        return cmp_fail(
            id,
            name,
            &format!(
                "record count: run {}, committed header {}, committed blocks {}",
                outcome.records.len(),
                declared_count,
                rec_fences.len()
            ),
            rep,
        );
    }
    let prog_root = sha256(envelope).to_vec();
    for (ri, rec) in outcome.records.iter().enumerate() {
        let parsed = parse_record_block(&rec_fences[ri].body);
        let f = &parsed.fields;
        let want_class = if rec.charged == 0 {
            // a rejection transition posts nothing (§3.1)
            "rejection (posts nothing)"
        } else {
            "completed"
        };
        if parsed.class != want_class {
            return cmp_fail(
                id,
                name,
                &format!("record {} is a {} but committed as \"{}\"", ri + 1, want_class, parsed.class),
                rep,
            );
        }
        // field 1 prev_root: the record's pre state — for consecutive records
        // the previous record's post is this record's pre, and the first
        // record's prev root is the declared initial configuration (§2.6)
        let checks: [(usize, String); 11] = [
            (1, format!("prev_root {:?}", crate::hex_to_bytes(&f[0]) == rec.pre)),
            (2, format!("program_root {:?}", crate::hex_to_bytes(&f[1]) == prog_root)),
            (3, format!("step_index {}", all_nums(&f[2])[0] == rec.step)),
            (4, format!("pc {:?}", iv_nums(&parse_iv(&f[3])) == vec![rec.pc.0 as i128, rec.pc.1 as i128])),
            (5, format!("decoded_word {:?}", crate::hex_to_bytes(&f[4]) == rec.decoded)),
            (6, format!("input_output_ids {:?}", hex_tokens(&f[5]) == rec.io_ids)),
            (7, format!("pre_state_root {:?}", crate::hex_to_bytes(&f[6]) == rec.pre)),
            (8, format!("post_state_root {:?}", crate::hex_to_bytes(&f[7]) == rec.post)),
            (9, format!("work_charged {}", all_nums(&f[8])[0] == rec.charged)),
            (10, format!("work_refunded {}", all_nums(&f[9])[0] == rec.refunded)),
            (11, format!("clock {}", all_nums(&f[10])[0] == rec.clock)),
        ];
        for (num, verdict) in &checks {
            if !verdict.ends_with("true") {
                return cmp_fail(
                    id,
                    name,
                    &format!("record {} field {} diverges from the committed bytes ({})", ri + 1, num, verdict),
                    rep,
                );
            }
        }
        // field 12 status: the transition's birth/consume/terminalize traces
        let got_status: Vec<(u64, [u64; 5])> = iv_elems(&parse_iv(&f[11]))
            .iter()
            .map(|e| {
                let n = iv_nums(e);
                (
                    n[0] as u64,
                    [
                        n[1] as u64,
                        n[2] as u64,
                        n[3] as u64,
                        n[4] as u64,
                        n[5] as u64,
                    ],
                )
            })
            .collect();
        if got_status != rec.events {
            return cmp_fail(
                id,
                name,
                &format!(
                    "record {} field 12 (status traces) diverges: run {:?} vs committed \"{}\"",
                    ri + 1,
                    rec.events,
                    f[11]
                ),
                rep,
            );
        }
        // field 13 receipt_ids: no seed vector emits a witness (P7)
        if !iv_nums(&parse_iv(&f[12])).is_empty() {
            return cmp_fail(
                id,
                name,
                &format!("record {} field 13 carries receipts — outside the seed's declared scope (P7)", ri + 1),
                rep,
            );
        }
        // field 14 versions: the vector's declared V
        let q: Vec<&str> = f[13].split('"').collect();
        let got_versions = [q[1], q[3], q[5]];
        if got_versions != [params.sem.as_str(), params.enc.as_str(), params.prof.as_str()] {
            return cmp_fail(
                id,
                name,
                &format!("record {} field 14 (versions) diverges from the declared V", ri + 1),
                rep,
            );
        }
    }

    // ---- agreement point: terminal receipt (canonical bytes decide, §4.4)
    if rcpt.bytes != outcome.receipt {
        return cmp_fail(
            id,
            name,
            &format!(
                "terminal receipt canonical bytes diverge (run {} bytes, committed {} bytes)",
                outcome.receipt.len(),
                rcpt.bytes.len()
            ),
            rep,
        );
    }
    if rcpt.outcome_tag != outcome.outcome {
        return cmp_fail(id, name, "receipt outcome_tag disagrees with the run", rep);
    }
    let got_item = interp_itemization(&outcome.receipt).expect("run receipt decodes");
    if got_item != rcpt.itemization {
        return cmp_fail(
            id,
            name,
            "receipt token_itemization diverges from the committed table",
            rep,
        );
    }
    let got_sel = interp_selected(&outcome.receipt).expect("run receipt decodes");
    match (got_sel, rcpt.selected) {
        (Some(a), Some(b)) if a == b => {}
        (None, None) => {
            // both nil-form annotations over bytes already byte-compared
        }
        _ => {
            return cmp_fail(
                id,
                name,
                "receipt selected_token diverges from the committed table",
                rep,
            );
        }
    }

    // ---- agreement point: reconstructed ledger (§3.1/§3.3)
    if ledger.work_e != outcome.acc.charged {
        return cmp_fail(id, name, "Work(E) diverges from the run's charged total", rep);
    }
    let mut committed_pc: Vec<(String, u64, u64)> = ledger.per_class.clone();
    committed_pc.sort();
    let mut run_pc: Vec<(String, u64, u64)> = outcome
        .per_class
        .iter()
        .map(|(c, u, n)| (c.to_string(), *u, *n))
        .collect();
    run_pc.sort();
    if committed_pc != run_pc {
        return cmp_fail(
            id,
            name,
            &format!(
                "per-class traces diverge: run {:?} vs committed {:?}",
                run_pc, committed_pc
            ),
            rep,
        );
    }
    let run_settle: Vec<(u64, u64, u64, u64)> = outcome
        .boundaries
        .iter()
        .map(|(s, a)| (*s, a.charged, a.reserved, a.refunded))
        .collect();
    if ledger.settlement != run_settle {
        return cmp_fail(
            id,
            name,
            "settlement trace diverges from the run's boundaries",
            rep,
        );
    }
    let outstanding = outcome.acc.reserved - outcome.acc.charged - outcome.acc.refunded;
    let (r1, c1, f1, o1) = ledger.ident1;
    if r1 != c1.wrapping_add(f1).wrapping_add(o1)
        || r1 != outcome.acc.reserved
        || c1 != outcome.acc.charged
        || f1 != outcome.acc.refunded
        || o1 != outstanding
    {
        return cmp_fail(
            id,
            name,
            "conservation identity 1 (reserved = charged + refunded + reserve_outstanding) diverges",
            rep,
        );
    }
    let (w2, c2, r2) = ledger.ident2;
    if w2 != params.w_budget || r2 != w2 - c2 || r2 != params.w_budget - outcome.acc.charged {
        return cmp_fail(
            id,
            name,
            "conservation identity 2 (remaining_budget = W_budget - charged) diverges",
            rep,
        );
    }
    match ledger.refund_final {
        Ok((b3, c3, r3)) => {
            if b3 != outcome.bh || r3 != b3 - c3 || r3 != outcome.bh - outcome.acc.charged {
                return cmp_fail(
                    id,
                    name,
                    "conservation identity 3 (refund_final = B(P) - charged) diverges",
                    rep,
                );
            }
        }
        Err(()) => {
            if outcome.acc.charged != 0 || outcome.acc.reserved != 0 || outcome.outcome != 3 {
                return cmp_fail(
                    id,
                    name,
                    "the committed escrow-never-opened form contradicts the run",
                    rep,
                );
            }
        }
    }
    if ledger.outstanding_final != 0 {
        return cmp_fail(
            id,
            name,
            "the reserve outlives the terminal settlement in the committed ledger",
            rep,
        );
    }

    rep.say(format!(
        "  {id} {name}: executed — {}/{} records match (fields 1–14, state roots byte-exact); receipt bytes match ({} (tag {})); ledger holds: Work(E) = {} U; conservation {} = {} + {} + {}",
        outcome.records.len(),
        outcome.records.len(),
        outcome_name(outcome.outcome),
        outcome.outcome,
        outcome.acc.charged,
        outcome.acc.reserved,
        outcome.acc.charged,
        outcome.acc.refunded,
        outstanding
    ));
    true
}

/// The suite: every committed Family X vector is executed and compared at
/// the five agreement points of §4.1.
pub fn suite_x_exec(md: &str, rep: &mut crate::Report) {
    let vectors = crate::parse_vectors(md, "## Family X — Execution Vectors");
    let profile = bhat::golden_profile();
    // committed envelopes first pass (a vector may cite another's, §4.4)
    let mut committed: Vec<(String, Vec<u8>)> = Vec::new();
    for v in &vectors {
        let f = crate::fences(&v.section);
        let env = match crate::fence_after(&f, "Intent envelope") {
            Some(ef) => crate::hex_to_bytes(&ef.body),
            None => Vec::new(),
        };
        committed.push((v.id.clone(), env));
    }
    let mut pass = 0usize;
    for (vi, v) in vectors.iter().enumerate() {
        let params = crate::parse_x_params(&v.section);
        let env_ref: Vec<u8> = if committed[vi].1.is_empty() {
            committed
                .iter()
                .find(|(id, _)| id == "X1")
                .map(|(_, b)| b.clone())
                .expect("the cited envelope's vector must exist")
        } else {
            committed[vi].1.clone()
        };
        if exec_vector_check(&v.id, &v.name, &v.section, &env_ref, &params, &profile, rep) {
            pass += 1;
        }
    }
    rep.say(format!(
        "suite X execution (reference interpreter): {} — {}/{} committed expected outcomes reproduced at the five agreement points of §4.1",
        if pass == vectors.len() { "PASS" } else { "FAIL" },
        pass,
        vectors.len()
    ));
}
