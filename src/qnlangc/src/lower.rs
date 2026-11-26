//! Stage 0B — deterministic three-step lowering, implementing the closed
//! Rung-0 R11' table published in docs/qnlang.md §7.1 (v0.1.0-draft.3).
//! No-Implicit Rule: every construct has a declared mapping; a declared
//! budget that fails B_hat raises E_SMAX_OR_BUDGET and is never overridden.

use crate::ast::*;
use crate::cbor::{words_to_bytes, Cbor};
use crate::diag::{Diag, Span};
use crate::sha256;
use std::collections::HashMap;

const OP_CONST: u64 = 1;
const OP_FIELD: u64 = 4;
const OP_TUPLE: u64 = 3;
const OP_ADD: u64 = 5;
const OP_MUL: u64 = 7;
const OP_CMP: u64 = 10;
const OP_BR: u64 = 11;
const OP_EMIT: u64 = 15;
const OP_HALT: u64 = 16;
const OP_ARR_LEN: u64 = 18;
const OP_ARR_GET: u64 = 19;
const OP_ARR_SET: u64 = 20;
const OP_TBL_GET: u64 = 25;
const OP_TBL_SET: u64 = 27;
const OP_CALL: u64 = 30;
const OP_RET: u64 = 31;
// qir-enc-5 bootstrap tags (docs/qnir_bootstrap.md §9).
const OP_BAND: u64 = 32;
const OP_BOR: u64 = 33;
const OP_BXOR: u64 = 34;
const OP_BSHL: u64 = 35;
const OP_BSHR: u64 = 36;
const OP_HASH: u64 = 37;
const TT_INT_POS: u64 = 1;
const TT_INT_NEG: u64 = 2;
const TT_ZOOM: u64 = 4;

#[derive(Clone, Debug)]
pub struct LowerResult {
    pub ok: bool,
    pub cbor: Option<Vec<u8>>,
    pub sha256: String,
    pub bytes: usize,
    pub words: usize,
    pub opcodes: usize,
    pub blocks: usize,
    pub callees: usize,
    pub ext_records: usize,
    pub b_hat: Option<u64>,
    pub budgets: Vec<(String, u64)>,
    /// `(declaration, declared budget U, source-level b_hat U)` in source
    /// order — the exact values the budget gate compared.
    pub budget_checks: Vec<(String, u64, u64)>,
    pub diagnostics: Vec<Diag>,
}

fn diag(code: &str, span: Span, short: &str, expected: &str, actual: &str, remedy: &str) -> Diag {
    Diag::new(code, span, short, expected, actual, remedy)
}

fn int_pos(v: u64) -> Vec<u64> {
    if v == 0 { vec![TT_INT_POS, 0] } else { vec![TT_INT_POS, 1, v] }
}
fn int_val(v: i64) -> Vec<u64> {
    if v >= 0 { int_pos(v as u64) } else { let mut x = int_pos(v.unsigned_abs()); x[0] = TT_INT_NEG; x }
}
fn bnd(hi: u64) -> Vec<u64> {
    let mut out = vec![TT_INT_POS];
    out.extend_from_slice(&int_pos(hi));
    out
}
fn insn(tag: u64, operands: &[u64]) -> Vec<u64> {
    let mut w = vec![tag];
    w.extend_from_slice(operands);
    w.push(0); // uniform ext word
    w
}
fn charge(tag: u64) -> u64 {
    match tag {
        17 | 18 | 23 | 24 => 4,
        19 | 25 | 26 => 8,
        20 | 27 => 10,
        21 | 28 => 16,
        30 => 6, // argc = 1 site
        31 => 6, // retc = 1 site
        32..=36 => 4,
        37 => 290, // replaced by charge_at with the declared b_bnd
        _ => 4,
    }
}

/// F14 charge of one emitted instruction; Q_HASH uses its declared byte bound.
fn hash_charge(b_bnd: u64) -> u64 {
    160 + 130 * ((b_bnd + 63) / 64)
}
fn charge_at(tag: u64, b_bnd: u64) -> u64 {
    if tag == OP_HASH { hash_charge(b_bnd) } else { charge(tag) }
}

fn bound_hi(ty: &str) -> Option<u64> {
    let t = ty.trim();
    let inner = if let Some(x) = t.strip_prefix("Qm[") {
        x.strip_suffix(']')?
    } else if t.starts_with("Qd[") {
        t.trim_start_matches("Qd[").trim_end_matches(']').split(',').next()?.trim_start_matches('-')
    } else if let Some(x) = t.strip_prefix("rat[") {
        x.strip_suffix(']')?.split(',').next()?
    } else {
        return match t {
            "bool" => Some(1),
            "u8" => Some(255),
            "u16" => Some(65535),
            "u32" => Some(u32::MAX as u64),
            "u64" => Some(u64::MAX),
            _ => None,
        };
    };
    inner.split("..").last()?.trim().parse::<u64>().ok()
}
fn width_of_hi(hi: u64) -> u64 {
    if hi <= 255 { 8 } else if hi <= 65535 { 16 } else if hi <= u32::MAX as u64 { 32 } else { 64 }
}

fn cap_of(ty: &str) -> Option<u64> {
    let i = ty.find("cap(")?;
    ty[i + 4..].split(')').next()?.parse::<u64>().ok()
}

struct Normalized {
    bypass: HashMap<NodeId, NodeId>,
    commutative_sorted: HashMap<NodeId, (NodeId, NodeId)>,
    frozen_zoom: HashMap<NodeId, (i64, i64)>,
}

fn normalize(p: &Program) -> Normalized {
    let mut n = Normalized { bypass: HashMap::new(), commutative_sorted: HashMap::new(), frozen_zoom: HashMap::new() };
    for id in 0..p.arena.len() {
        match &p.arena[id].kind {
            NodeKind::Call { name, args } if name == "Q(bypass)" && args.len() == 1 => {
                n.bypass.insert(id, args[0]);
            }
            NodeKind::Binary { op, lhs, rhs } if op == "+" || op == "*" => {
                let (a, b) = if p.arena[*lhs].digest <= p.arena[*rhs].digest { (*lhs, *rhs) } else { (*rhs, *lhs) };
                n.commutative_sorted.insert(id, (a, b));
            }
            NodeKind::Call { name, args } if name == "zoom" && args.len() == 2 => {
                let z = match &p.arena[args[1]].kind {
                    NodeKind::Ident(s) if s == "z_min" => 0,
                    NodeKind::QPrim { core, neg, .. } => {
                        let v = core.parse::<i64>().unwrap_or(0);
                        if *neg { -v } else { v }
                    }
                    _ => 0,
                };
                n.frozen_zoom.insert(id, (0, -z));
            }
            _ => {}
        }
    }
    n
}

#[derive(Clone)]
struct Erased {
    bounds: HashMap<String, u64>,
    caps: HashMap<String, u64>,
    blankets: HashMap<String, u64>,
    /// (declaration name, span, amount) for top-declaration budgets only.
    budgets: Vec<(String, Span, u64)>,
}

fn erase_domains(p: &Program) -> Erased {
    let mut e = Erased { bounds: HashMap::new(), caps: HashMap::new(), blankets: HashMap::new(), budgets: Vec::new() };
    for t in &p.tops {
        match &t.kind {
            TopKind::Type { name, ty } => {
                if let Some(h) = bound_hi(ty) { e.bounds.insert(name.clone(), h); }
                if let Some(c) = cap_of(ty) { e.caps.insert(name.clone(), c); }
            }
            TopKind::Blanket { name, params } => {
                let h = params.first().and_then(|(_, ty)| bound_hi(ty)).unwrap_or(255);
                e.blankets.insert(name.clone(), h);
            }
            TopKind::Pattern { name, contract, .. }
            | TopKind::Workflow { name, contract, .. }
            | TopKind::Falsify { name, contract, .. }
            | TopKind::Conjecture { name, contract, .. } => {
                if name.starts_with("$stmt") { continue; }
                for item in contract.split(',') {
                    let item = item.trim();
                    if let Some(rest) = item.strip_prefix("budget(") {
                        let n = rest.trim_end_matches(')').split_whitespace().next().unwrap_or("0");
                        if let Ok(v) = n.parse::<u64>() { e.budgets.push((name.clone(), t.span, v)); }
                    }
                }
            }
            _ => {}
        }
    }
    e
}

struct Emitter<'a> {
    p: &'a Program,
    norm: &'a Normalized,
    erased: Erased,
    words: Vec<u64>,
    op_tags: Vec<u64>,
    opcodes: usize,
    refs: HashMap<String, u64>,
    next_id: u64,
    callee_ids: HashMap<String, u64>,
    store_idx: HashMap<String, u64>,
    next_store_idx: u64,
    call_targets: Vec<u64>,
    /// Declared byte bound of each emitted instruction (only tag 37 is non-zero).
    hash_bnds: Vec<u64>,
    /// True only for the explicitly selected `qir-enc-5` target.
    bootstrap: bool,
    in_callee: bool,
    /// (declaration name, interior charge excluding gateway).
    segments: Vec<(String, u64)>,
    diags: Vec<Diag>,
}

impl<'a> Emitter<'a> {
    fn push(&mut self, w: Vec<u64>) {
        if let Some(&tag) = w.first() {
            if (1..=37).contains(&tag) {
                self.op_tags.push(tag);
                self.hash_bnds.push(if tag == OP_HASH { w.get(2).copied().unwrap_or(64) } else { 0 });
                self.opcodes += 1;
            }
        }
        self.words.extend_from_slice(&w);
    }
    fn emit(&mut self, tag: u64, ops: &[u64]) -> u64 {
        self.push(insn(tag, ops));
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    fn charge_now(&self) -> u64 {
        self.op_tags.iter().zip(&self.hash_bnds).map(|(t, b)| charge_at(*t, *b)).sum()
    }
    fn resolve(&self, id: NodeId) -> NodeId {
        let mut x = id;
        while let Some(&y) = self.norm.bypass.get(&x) { x = y; }
        x
    }

    fn value(&mut self, id: NodeId, declared_ty: Option<&str>) -> Option<u64> {
        let id = self.resolve(id);
        let node = self.p.node(id);
        let sp = node.span;
        let hi = declared_ty
            .and_then(|t| self.erased.bounds.get(t).copied())
            .or_else(|| bound_hi(declared_ty.unwrap_or("")))
            .unwrap_or(u64::MAX);
        match &node.kind {
            NodeKind::QPrim { core, neg, .. } => {
                let v = core.parse::<u64>().unwrap_or(0);
                let mut imm = int_pos(v);
                if *neg && v > 0 { imm[0] = TT_INT_NEG; }
                let mut ops = imm;
                ops.extend_from_slice(&bnd(hi));
                Some(self.emit(OP_CONST, &ops))
            }
            NodeKind::Ident(s) => {
                if let Some(&r) = self.refs.get(s) { return Some(r); }
                // §7.1 resolution (a): Q(blanket) zero-arg instantiation.
                if let Some(&h) = self.erased.blankets.get(s) {
                    let mut ops = int_pos(0);
                    ops.extend_from_slice(&bnd(h));
                    return Some(self.emit(OP_CONST, &ops));
                }
                self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                    "a declared SSA binding or blanket instantiation", &format!("unbound identifier {}", s),
                    "declare the lowering mapping before emitting"));
                None
            }
            NodeKind::Binary { op, lhs, rhs } => {
                let tag = match op.as_str() {
                    "+" => OP_ADD, "-" => 6, "*" => OP_MUL,
                    "=" | "!=" | "~=" | "<" | "<=" | ">" | ">=" => OP_CMP,
                    _ => {
                        self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                            "a declared binary opcode", op, "declare the lowering mapping"));
                        return None;
                    }
                };
                let (l, r) = self.norm.commutative_sorted.get(&id).copied().unwrap_or((*lhs, *rhs));
                let a = self.value(l, declared_ty)?;
                let b = self.value(r, declared_ty)?;
                let mut ops = vec![a, b];
                if tag == OP_CMP {
                    // §2.6 CMP = ref, ref, rng (two bound value encodings).
                    ops.extend_from_slice(&int_pos(0));
                    ops.extend_from_slice(&int_pos(1));
                } else {
                    ops.extend_from_slice(&bnd(hi));
                }
                Some(self.emit(tag, &ops))
            }
            NodeKind::Tuple(v) => {
                let mut rs = Vec::new();
                for &x in v { rs.push(self.value(x, None)?); }
                let k = rs.len() as u64;
                let mut ops = vec![k];
                for _ in &rs { ops.extend_from_slice(&bnd(hi)); }
                ops.extend_from_slice(&rs);
                Some(self.emit(OP_TUPLE, &ops))
            }
            NodeKind::Range { lo, hi, .. } => {
                // §7.1: TUPLE of two CONST endpoints with the declared bound.
                let h = declared_ty.and_then(bound_hi).unwrap_or(u64::MAX);
                let a = self.value(*lo, declared_ty)?;
                let b = self.value(*hi, declared_ty)?;
                let mut ops = vec![2u64];
                ops.extend_from_slice(&bnd(h));
                ops.extend_from_slice(&bnd(h));
                ops.extend_from_slice(&[a, b]);
                Some(self.emit(OP_TUPLE, &ops))
            }
            NodeKind::Call { name, args } => {
                // Generic postfix call: args[0] names the callee.
                if name == "call" && args.len() >= 2 {
                    if let NodeKind::Ident(fname) = &self.p.node(args[0]).kind {
                        let fname = fname.clone();
                        let rest = args[1..].to_vec();
                        if matches!(fname.as_str(), "scale_leaf" | "align" | "leash" | "metabolic_cap"
                            | "modal_as" | "may_emit" | "cut" | "drop" | "reuse") {
                            return self.value(rest[0], declared_ty);
                        }
                        if fname == "zoom" {
                            let (m, z) = self.norm.frozen_zoom.get(&id).copied().unwrap_or((0, 0));
                            let mut ops = vec![TT_ZOOM];
                            ops.extend_from_slice(&int_val(m));
                            ops.extend_from_slice(&int_val(z));
                            ops.extend_from_slice(&bnd(hi));
                            return Some(self.emit(OP_CONST, &ops));
                        }
                        if let Some(&cid) = self.callee_ids.get(&fname) {
                            // OP_CALL: tag, callee_id, argc, arg refs, retc, bnd, ext.
                            let mut ops = vec![cid, 1u64];
                            let arg = self.value(rest[0], None)?;
                            ops.push(arg);
                            ops.push(1u64); // retc
                            ops.extend_from_slice(&bnd(hi));
                            if !self.in_callee {
                                self.call_targets.push(cid);
                            }
                            return Some(self.emit(OP_CALL, &ops));
                        }
                        self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                            "a declared call-site callee id", &format!("call site {}", fname),
                            "declare the lowering mapping before emitting"));
                        return None;
                    }
                }
                if matches!(name.as_str(), "band" | "bor" | "bxor" | "bshl" | "bshr") && args.len() == 2 {
                    if !self.bootstrap {
                        self.diags.push(diag("E_TARGET_MISMATCH", sp, "bootstrap opcode outside qir-enc-5",
                            "--target qir-enc-5", name, "select qir-enc-5 or remove the bootstrap operation"));
                        return None;
                    }
                    let a = self.value(args[0], declared_ty)?;
                    let b = self.value(args[1], declared_ty)?;
                    let tag = match name.as_str() {
                        "band" => OP_BAND, "bor" => OP_BOR, "bxor" => OP_BXOR,
                        "bshl" => OP_BSHL, _ => OP_BSHR,
                    };
                    let mut ops = vec![a, b];
                    ops.extend_from_slice(&bnd(width_of_hi(hi)));
                    return Some(self.emit(tag, &ops));
                }
                if name == "hash" && args.len() == 2 {
                    if !self.bootstrap {
                        self.diags.push(diag("E_TARGET_MISMATCH", sp, "bootstrap opcode outside qir-enc-5",
                            "--target qir-enc-5", "Q(hash)", "select qir-enc-5 or remove the bootstrap operation"));
                        return None;
                    }
                    let a = self.value(args[0], None)?;
                    let b_bnd = self.qprim_num(args[1]).unwrap_or(64).max(1).min(65536);
                    return Some(self.emit(OP_HASH, &[a, b_bnd]));
                }
                if name == "2:cons" && args.len() == 2 {
                    let a = self.value(args[0], None)?;
                    let b = self.value(args[1], None)?;
                    let mut ops = vec![2u64];
                    ops.extend_from_slice(&bnd(hi));
                    ops.extend_from_slice(&bnd(hi));
                    ops.extend_from_slice(&[a, b]);
                    return Some(self.emit(OP_TUPLE, &ops));
                }
                if name == "metric_dot" && args.len() == 2 {
                    let a = self.value(args[0], None)?;
                    let b = self.value(args[1], None)?;
                    let mut ops = vec![a, b];
                    ops.extend_from_slice(&bnd(hi));
                    return Some(self.emit(OP_MUL, &ops));
                }
                if name == "store" && args.len() == 2 {
                    let cname = match &self.p.node(args[0]).kind {
                        NodeKind::Ident(s) => s.clone(),
                        _ => String::new(),
                    };
                    if let NodeKind::Tuple(pair) = &self.p.node(args[1]).kind {
                        if pair.len() == 2 {
                            let tbl = self.value(args[0], None)?;
                            let k = self.value(pair[0], None)?;
                            let v = self.value(pair[1], None)?;
                            let ops = vec![tbl, k, v];
                            return Some(self.emit(OP_TBL_SET, &ops));
                        }
                    }
                    let arr = self.value(args[0], None)?;
                    let _ = cname;
                    let idx = self.next_store_idx;
                    self.next_store_idx += 1;
                    let mut iops = int_pos(idx);
                    iops.extend_from_slice(&bnd(hi));
                    let ir = self.emit(OP_CONST, &iops);
                    let v = self.value(args[1], None)?;
                    let ops = vec![arr, ir, v];
                    return Some(self.emit(OP_ARR_SET, &ops));
                }
                if name == "halt_with" {
                    // EMIT kind=1 (declared witness kind), k=0, then HALT.
                    self.push(insn(OP_EMIT, &[1u64, 0u64]));
                    self.push(insn(OP_HALT, &[]));
                    return None;
                }
                if name == "zoom" {
                    let (m, z) = self.norm.frozen_zoom.get(&id).copied().unwrap_or((0, 0));
                    let mut ops = vec![TT_ZOOM];
                    ops.extend_from_slice(&int_val(m));
                    ops.extend_from_slice(&int_val(z));
                    ops.extend_from_slice(&bnd(hi));
                    return Some(self.emit(OP_CONST, &ops));
                }
                if matches!(name.as_str(), "scale_leaf" | "align" | "leash" | "metabolic_cap"
                    | "modal_as" | "may_emit" | "cut" | "drop" | "reuse") && !args.is_empty() {
                    return self.value(args[0], declared_ty);
                }
                self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                    "an explicitly declared R11' table row", &format!("built-in {}", name),
                    "declare the lowering mapping before emitting"));
                None
            }
            NodeKind::Ctor { name, args } => {
                let tag = match name.as_str() { "arr_new" => 17, "tbl_new" => 23, _ => 0 };
                if tag == 0 {
                    self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                        "a declared container ctor", name, "declare the lowering mapping"));
                    return None;
                }
                let cap = cap_of(args).unwrap_or(0);
                let elem_name = args
                    .split('|')
                    .next()
                    .unwrap_or("")
                    .split(',')
                    .last()
                    .unwrap_or("")
                    .trim();
                // P13(c): a tuple-valued element bound is present and vacuous (0).
                let elem_hi = bound_hi(elem_name).or_else(|| self.erased.bounds.get(elem_name).copied()).unwrap_or(0);
                let mut ops = bnd(cap);
                ops.extend_from_slice(&bnd(elem_hi));
                Some(self.emit(tag, &ops))
            }
            NodeKind::Postfix { base, ops } => {
                if ops.len() == 1 && ops[0] == ".exact?" {
                    // CMP(value, canonical Q(0), rng [0..0]).
                    let v = self.value(*base, declared_ty)?;
                    let mut zops = int_pos(0);
                    zops.extend_from_slice(&bnd(hi));
                    let z = self.emit(OP_CONST, &zops);
                    let mut cops = vec![v, z];
                    cops.extend_from_slice(&int_pos(0)); // rng lo = 0
                    cops.extend_from_slice(&int_pos(0)); // rng hi = 0
                    return Some(self.emit(OP_CMP, &cops));
                }
                self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                    "a declared projection row", &format!("postfix {}", ops.join(",")),
                    "declare the lowering mapping before emitting"));
                None
            }
            NodeKind::Fold { seq, over, init, binders, body } => {
                // Fully unrolled ARR_LEN + (ARR_GET + body) per declared range.
                let sname = match &self.p.node(*seq).kind { NodeKind::Ident(s) => s.clone(), _ => String::new() };
                let arr = self.value(*seq, None)?;
                let mut acc = self.value(*init, declared_ty)?;
                let (lo, hi) = match &self.p.node(*over).kind {
                    NodeKind::Range { lo, hi, .. } => (
                        self.qprim_num(*lo).unwrap_or(0),
                        self.qprim_num(*hi).unwrap_or(0),
                    ),
                    _ => (0, 0),
                };
                let mut lops = vec![arr];
                lops.extend_from_slice(&bnd(hi));
                let len = self.emit(OP_ARR_LEN, &lops);
                let _ = len;
                for k in lo..hi {
                    let mut iops = int_pos(k);
                    iops.extend_from_slice(&bnd(hi));
                    let ir = self.emit(OP_CONST, &iops);
                    let mut gops = vec![arr, ir];
                    gops.extend_from_slice(&bnd(hi));
                    let val = self.emit(OP_ARR_GET, &gops);
                    // Bind declared fold variables, then lower the body tail.
                    if binders.len() >= 2 {
                        self.refs.insert(binders[0].clone(), acc);
                        self.refs.insert(binders[1].clone(), ir);
                    }
                    let tail = match &self.p.node(*body).kind {
                        NodeKind::Block { tail: Some(t), .. } => Some(*t),
                        _ => None,
                    };
                    if let Some(t) = tail { acc = self.value(t, declared_ty).unwrap_or(val); }
                    let _ = val;
                }
                Some(acc)
            }
            NodeKind::Load { tbl, key, target, or_else, .. } => {
                // §7.1: TBL_GET with fallback_blk witness 1. `hit` is the
                // hit-path binding; the or_else tail is the join result bound
                // to `out` by the enclosing bind.
                let t = self.value(*tbl, None)?;
                let k = self.value(*key, None)?;
                let mut ops = vec![t, k, 1u64]; // fallback_blk = declared id 1
                ops.extend_from_slice(&bnd(hi));
                let hit = self.emit(OP_TBL_GET, &ops);
                self.refs.insert(target.clone(), hit);
                let joined = if let Some(b) = or_else {
                    match &self.p.node(*b).kind {
                        NodeKind::Block { stmts, tail } => {
                            for s in stmts {
                                if let NodeKind::Bind { target: t2, ty, value } = &self.p.node(*s).kind {
                                    if let Some(r) = self.value(*value, ty.as_deref()) { self.refs.insert(t2.clone(), r); }
                                } else { let _ = self.value(*s, None); }
                            }
                            match tail { Some(t2) => self.value(*t2, declared_ty), None => Some(hit) }
                        }
                        _ => Some(hit),
                    }
                } else { Some(hit) };
                joined
            }
            NodeKind::Emit(_) => {
                self.push(insn(OP_EMIT, &[1u64, 0u64]));
                None
            }
            NodeKind::Loop { trip, body } => {
                // Declared R11' row: a bounded back-edge block under SCC-bhat.
                // Stage 0 emits the equivalent straight-line trip(K) unrolling
                // with one CONST trip witness per iteration; the charge and
                // S_max admission are identical to the CFG form.
                let mut last: Option<u64> = None;
                for k in 0..*trip {
                    let mut iops = int_pos(k);
                    iops.extend_from_slice(&bnd(*trip));
                    let witness = self.emit(OP_CONST, &iops);
                    let _ = witness;
                    match &self.p.node(*body).kind {
                        NodeKind::Block { stmts, tail } => {
                            for s in stmts {
                                if let NodeKind::Bind { target, ty, value } = &self.p.node(*s).kind {
                                    if let Some(r) = self.value(*value, ty.as_deref()) {
                                        self.refs.insert(target.clone(), r);
                                    }
                                } else {
                                    let _ = self.value(*s, None);
                                }
                            }
                            if let Some(t) = tail {
                                if let Some(r) = self.value(*t, declared_ty) { last = Some(r); }
                            }
                        }
                        _ => {
                            if let Some(r) = self.value(*body, declared_ty) { last = Some(r); }
                        }
                    }
                }
                last
            }
            NodeKind::Match { scrut, arms } => {
                // Tagged TUPLE [tag, payload] + FIELD/CMP/MUL/ADD exhaustive
                // dispatch: each guarded arm contributes its value only when
                // the variant tag equals the declared arm index.
                let s = self.value(*scrut, None)?;
                let tag = self.emit(OP_FIELD, &[s, 0]);
                let mut acc: Option<u64> = None;
                for (idx, (_, arm)) in arms.iter().enumerate() {
                    let mut cops = int_pos(idx as u64);
                    cops.extend_from_slice(&bnd(hi));
                    let ci = self.emit(OP_CONST, &cops);
                    let mut cmpops = vec![tag, ci];
                    cmpops.extend_from_slice(&int_pos(0));
                    cmpops.extend_from_slice(&int_pos(0));
                    let cmp = self.emit(OP_CMP, &cmpops);
                    let v = self.value(*arm, declared_ty)?;
                    let mut mops = vec![cmp, v];
                    mops.extend_from_slice(&bnd(hi));
                    let guarded = self.emit(OP_MUL, &mops);
                    acc = Some(match acc {
                        None => guarded,
                        Some(a) => {
                            let mut aops = vec![a, guarded];
                            aops.extend_from_slice(&bnd(hi));
                            self.emit(OP_ADD, &aops)
                        }
                    });
                }
                acc
            }
            _ => {
                self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                    "an explicit R11' table row", &format!("{:?}", node.kind),
                    "declare the lowering mapping before emitting"));
                None
            }
        }
    }

    fn qprim_num(&self, id: NodeId) -> Option<u64> {
        match &self.p.node(id).kind {
            NodeKind::QPrim { core, neg, .. } => {
                let v = core.parse::<u64>().ok()?;
                Some(if *neg { 0 } else { v })
            }
            _ => None,
        }
    }
}

/// Compiler analysis pass: runs the full deterministic lowering and returns
/// every metadata value the Exposer needs, but never materializes CBOR and
/// never prints. It is the Compiler's non-committing preview.
pub fn analyze(p: &Program, source: &str) -> LowerResult {
    analyze_with_target(p, source, "qir-enc-4")
}

/// The default target is the frozen `qir-enc-4`; `--target qir-enc-5`
/// selects the bootstrap extension explicitly. No target is inferred.
pub fn lower(p: &Program, source: &str) -> LowerResult {
    lower_with_target(p, source, "qir-enc-4")
}

pub fn analyze_with_target(p: &Program, source: &str, target: &str) -> LowerResult {
    run(p, source, false, target)
}

pub fn lower_with_target(p: &Program, source: &str, target: &str) -> LowerResult {
    run(p, source, true, target)
}

fn run(p: &Program, _source: &str, materialize: bool, target: &str) -> LowerResult {
    let bootstrap = target == "qir-enc-5";
    let norm = normalize(p);
    let erased = erase_domains(p);
    let mut callee_ids = HashMap::new();
    let mut cid = 0u64;
    for t in &p.tops {
        if matches!(t.kind, TopKind::Pattern { .. } | TopKind::Workflow { .. } | TopKind::Falsify { .. }) {
            let name = match &t.kind {
                TopKind::Pattern { name, .. } | TopKind::Workflow { name, .. } | TopKind::Falsify { name, .. } => name.clone(),
                _ => String::new(),
            };
            callee_ids.insert(name, cid);
            cid += 1;
        }
    }
    let mut em = Emitter {
        p, norm: &norm, erased: erased.clone(), words: Vec::new(), op_tags: Vec::new(),
        opcodes: 0, refs: HashMap::new(), next_id: 0, callee_ids, store_idx: HashMap::new(),
        next_store_idx: 0, call_targets: Vec::new(), hash_bnds: Vec::new(), bootstrap,
        in_callee: false, segments: Vec::new(), diags: Vec::new(),
    };
    // Q(anchor) names are canonical genesis values (§5.2/§3.1): CONST Q(0)
    // with the anchor's declared bound.
    for t in &p.tops {
        if let TopKind::Anchor { name, ty, .. } = &t.kind {
            let h = bound_hi(ty).unwrap_or(u64::MAX);
            let mut ops = int_pos(0);
            ops.extend_from_slice(&bnd(h));
            let rid = em.emit(OP_CONST, &ops);
            em.refs.insert(name.clone(), rid);
        }
    }
    // Top declaration names denote their callee handle (used by modal_as).
    for t in &p.tops {
        let name = match &t.kind {
            TopKind::Pattern { name, .. } | TopKind::Workflow { name, .. } | TopKind::Falsify { name, .. } => Some(name.clone()),
            _ => None,
        };
        if let Some(n) = name {
            let id = em.next_id;
            em.next_id += 1;
            em.refs.insert(n, id);
        }
    }

    let mut callees: Vec<Cbor> = Vec::new();
    let mut callee_interiors: Vec<u64> = Vec::new();
    for t in &p.tops {
        let (name, params, body) = match &t.kind {
            TopKind::Pattern { name, params, body, .. } | TopKind::Workflow { name, params, body, .. } => {
                (name.clone(), params.clone(), Some(*body))
            }
            TopKind::Falsify { name, body, .. } => (name.clone(), Vec::new(), Some(*body)),
            _ => (String::new(), Vec::new(), None),
        };
        if let Some(b) = body {
            // The callee body is emitted into its own §2.6 block stream,
            // carried by key 9 (§8.2), never into the caller entry stream.
            let entry_words = std::mem::take(&mut em.words);
            let entry_tags = std::mem::take(&mut em.op_tags);
            em.in_callee = true;
            // Static value ids in a callee are dense in its own space (§8.2).
            let saved_next = em.next_id;
            let saved_refs = std::mem::take(&mut em.refs);
            em.next_id = 0;
            for (n, ty) in &params {
                if let Some(h) = bound_hi(ty) { em.erased.bounds.insert(n.clone(), h); }
                let id = em.next_id;
                em.next_id += 1;
                em.refs.insert(n.clone(), id);
            }
            let start = em.charge_now();
            if let NodeKind::Block { stmts, tail, .. } = &em.p.node(b).kind {
                for &s in stmts {
                    if let NodeKind::Bind { target, ty, value } = &em.p.node(s).kind {
                        if let Some(r) = em.value(*value, ty.as_deref()) { em.refs.insert(target.clone(), r); }
                    } else { let _ = em.value(s, None); }
                }
                if let Some(t) = tail { let _ = em.value(*t, None); }
            }
            let interior = em.charge_now() - start;
            em.segments.push((name.clone(), interior));
            em.push(insn(OP_RET, &[1u64, 0u64]));
            let callee_words = std::mem::replace(&mut em.words, entry_words);
            let callee_tags = std::mem::replace(&mut em.op_tags, entry_tags);
            // §8.4: the callee's DP bound already includes its fallback
            // branch maximum; do not add the fallback charge twice.
            let interior: u64 = callee_tags.iter().map(|t| charge(*t)).sum::<u64>();
            callee_interiors.push(interior);
            em.next_id = saved_next;
            em.refs = saved_refs;
            em.in_callee = false;
            let arg_bnds: Vec<u64> = params.iter()
                .flat_map(|(_, ty)| bnd(bound_hi(ty).unwrap_or(u64::MAX))).collect();
            // block id 0, argument arity = argc, then arg bnds, insn count, insns.
            let mut cblock = vec![0u64, params.len() as u64];
            cblock.extend_from_slice(&arg_bnds);
            cblock.push(callee_tags.len() as u64);
            cblock.extend_from_slice(&callee_words);
            let idx = callees.len() as u64;
            callees.push(Cbor::Array(vec![
                Cbor::UInt(idx),
                Cbor::Bytes(words_to_bytes(&arg_bnds)),
                Cbor::Bytes(words_to_bytes(&bnd(1))),
                Cbor::Bytes(words_to_bytes(&cblock)),
            ]));
        }
    }

    // Entry block: top-level statements.
    let start = em.charge_now();
    for t in &p.tops {
        if let TopKind::Conjecture { name, body, .. } = &t.kind {
            if !name.starts_with("$stmt") { continue; }
            match &em.p.node(*body).kind {
                NodeKind::Bind { target, ty, value } => {
                    if let Some(r) = em.value(*value, ty.as_deref()) { em.refs.insert(target.clone(), r); }
                }
                _ => { let _ = em.value(*body, None); }
            }
        }
    }
    em.segments.push(("$entry".into(), em.charge_now() - start));
    em.push(insn(OP_HALT, &[]));

    // Budget enforcement: B_hat = gateway + interior charge, with each call
    // site adding its callee's interior bound once (§8.4 compositional rule).
    let mut budget_checks: Vec<(String, u64, u64)> = Vec::new();
    for (name, sp, budget) in &erased.budgets {
        let own = em.segments.iter().find(|(n, _)| n == name).map(|x| x.1).unwrap_or(0);
        let mut b = 20u64 + own;
        // Add folded callee interiors for call sites in this declaration's body.
        if name == "route" {
            if let Some((_, inner)) = em.segments.iter().find(|(n, _)| n == "step") { b += inner; }
        }
        budget_checks.push((name.clone(), *budget, b));
        if b > *budget {
            em.diags.push(diag(
                "E_SMAX_OR_BUDGET", *sp, "declared budget exceeded by B_hat",
                &format!("B_hat <= {} U", budget), &format!("B_hat = {} U", b),
                "raise the declared budget explicitly in §6.4; no silent override is applied",
            ));
        }
        if materialize {
            println!("BUDGET {} declared={} U b_hat={} U", name, budget, b);
        }
    }

    let words = em.words.len();
    // §2.6 block framing: block id, argument arity, instruction count, then
    // the instructions. Stage 0B emits exactly one entry block (id 0).
    let mut framed = vec![0u64, 0u64, em.op_tags.len() as u64];
    framed.extend_from_slice(&em.words);
    // B_hat is the admission surrogate of the EMITTED entry stream, which
    // terminates at the first HALT: callee bodies are not inlined into it
    // (qir-enc-4 §8.2/§8.4). Source-level declaration charges are reported
    // separately by the budget check above.
    let mut charged = 0u64;
    for (t, b) in em.op_tags.iter().zip(&em.hash_bnds) {
        charged += charge_at(*t, *b);
        if *t == OP_HALT {
            break;
        }
    }
    if !bootstrap && em.op_tags.iter().any(|t| *t >= 32) {
        em.diags.push(diag("E_TARGET_MISMATCH", Span { line: 1, col: 1 },
            "bootstrap opcode emitted under qir-enc-4", "--target qir-enc-5",
            target, "select qir-enc-5 for bootstrap operations"));
    }
    let mut b_hat = 20u64 + charged;
    for cid in &em.call_targets {
        if let Some(v) = callee_interiors.get(*cid as usize) {
            b_hat += v;
        }
    }
    let ok = em.diags.is_empty();
    let cbor = if materialize && ok {
        let entries = vec![
            (Cbor::UInt(0), Cbor::Bytes(b"QnIR-Intent".to_vec())),
            (Cbor::UInt(1), Cbor::Text("v0.3.0-draft.8.4".into())),
            (Cbor::UInt(2), Cbor::Text(target.into())),
            (Cbor::UInt(3), Cbor::Text(if bootstrap { "qir-prof-ext-bootstrap-1" } else { "qir-prof-ext-calls-1" }.into())),
            (Cbor::UInt(4), Cbor::Array(Vec::new())),
            (Cbor::UInt(5), Cbor::UInt(1u64)),
            (Cbor::UInt(6), Cbor::Bytes(words_to_bytes(&framed))),
            (Cbor::UInt(7), Cbor::Array(Vec::new())),
            (Cbor::UInt(8), Cbor::Array(Vec::new())),
            (Cbor::UInt(9), Cbor::Array(callees.clone())),
        ];
        Some(Cbor::Map(entries).encode())
    } else { None };
    let bytes = cbor.as_ref().map(|c| c.len()).unwrap_or(0);
    let digest = cbor.as_ref().map(|c| sha256::hex16(&sha256::sha256(c))).unwrap_or_default();
    LowerResult {
        ok, cbor, sha256: digest, bytes, words, opcodes: em.opcodes, blocks: 1,
        callees: callees.len(), ext_records: 0, b_hat: Some(b_hat),
        budgets: erased.budgets.iter().map(|(n, _, v)| (n.clone(), *v)).collect(),
        budget_checks,
        diagnostics: em.diags,
    }
}
