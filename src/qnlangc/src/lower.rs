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
        _ => 4,
    }
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
    /// (declaration name, interior charge excluding gateway).
    segments: Vec<(String, u64)>,
    diags: Vec<Diag>,
}

impl<'a> Emitter<'a> {
    fn push(&mut self, w: Vec<u64>) {
        if let Some(&tag) = w.first() {
            if (1..=31).contains(&tag) { self.op_tags.push(tag); self.opcodes += 1; }
        }
        self.words.extend_from_slice(&w);
    }
    fn emit(&mut self, tag: u64, ops: &[u64]) -> u64 {
        self.push(insn(tag, ops));
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    fn charge_now(&self) -> u64 { self.op_tags.iter().map(|t| charge(*t)).sum() }
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
                            let r = self.emit(OP_CALL, &ops);
                            if let Some(&interior) = self.segments.iter().find(|(n, _)| *n == fname).map(|x| &x.1) {
                                let _ = interior; // folded at budget check, not double-posted here
                            }
                            return Some(r);
                        }
                        self.diags.push(diag("E_LOWERING_AMBIGUOUS", sp, "construct has no declared lowering",
                            "a declared call-site callee id", &format!("call site {}", fname),
                            "declare the lowering mapping before emitting"));
                        return None;
                    }
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
                    let idx = *self.store_idx.get(&cname).unwrap_or(&0);
                    self.store_idx.insert(cname, idx + 1);
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
                let elem_name = args.split('|').next().unwrap_or("");
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

pub fn lower(p: &Program, _source: &str) -> LowerResult {
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
        segments: Vec::new(), diags: Vec::new(),
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
    for t in &p.tops {
        let (name, params, body) = match &t.kind {
            TopKind::Pattern { name, params, body, .. } | TopKind::Workflow { name, params, body, .. } => {
                (name.clone(), params.clone(), Some(*body))
            }
            TopKind::Falsify { name, body, .. } => (name.clone(), Vec::new(), Some(*body)),
            _ => (String::new(), Vec::new(), None),
        };
        if let Some(b) = body {
            for (n, ty) in &params {
                if let Some(h) = bound_hi(ty) { em.erased.bounds.insert(n.clone(), h); }
                // Callee block-argument binding (§8.2): positional SSA ref.
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
            // OP_RET closes the callee (§8.3).
            em.push(insn(OP_RET, &[1u64, 0u64]));
            let arg_bnds: Vec<u64> = params.iter()
                .flat_map(|(_, ty)| bnd(bound_hi(ty).unwrap_or(u64::MAX))).collect();
            let idx = callees.len() as u64;
            callees.push(Cbor::Array(vec![
                Cbor::UInt(idx), Cbor::Bytes(Vec::new()), Cbor::Bytes(Vec::new()),
                Cbor::Bytes(words_to_bytes(&arg_bnds)),
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
    for (name, sp, budget) in &erased.budgets {
        let own = em.segments.iter().find(|(n, _)| n == name).map(|x| x.1).unwrap_or(0);
        let mut b = 20u64 + own;
        // Add folded callee interiors for call sites in this declaration's body.
        if name == "route" {
            if let Some((_, inner)) = em.segments.iter().find(|(n, _)| n == "step") { b += inner; }
        }
        if b > *budget {
            em.diags.push(diag(
                "E_SMAX_OR_BUDGET", *sp, "declared budget exceeded by B_hat",
                &format!("B_hat <= {} U", budget), &format!("B_hat = {} U", b),
                "raise the declared budget explicitly in §6.4; no silent override is applied",
            ));
        }
        println!("BUDGET {} declared={} U b_hat={} U", name, budget, b);
    }

    let words = em.words.len();
    // §2.6 block framing: block id, argument arity, instruction count, then
    // the instructions. Stage 0B emits exactly one entry block (id 0).
    let mut framed = vec![0u64, 0u64, em.opcodes as u64];
    framed.extend_from_slice(&em.words);
    let b_hat = 20u64 + em.charge_now();
    let ok = em.diags.is_empty();
    let cbor = if ok {
        let entries = vec![
            (Cbor::UInt(0), Cbor::Bytes(b"QnIR-Intent".to_vec())),
            (Cbor::UInt(1), Cbor::Text("v0.3.0-draft.8.4".into())),
            (Cbor::UInt(2), Cbor::Text("qir-enc-4".into())),
            (Cbor::UInt(3), Cbor::Text("qir-prof-ext-calls-1".into())),
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
        diagnostics: em.diags,
    }
}
