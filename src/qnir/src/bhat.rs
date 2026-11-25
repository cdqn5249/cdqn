//! The declared trip-ceiling unrolling dynamic program `B_hat(P)` (§3.4, F16)
//! — the validator-side admission surrogate of §4.2.
//!
//! State space: (block id, remaining trip budget per back-edge site);
//! `B_hat = g + max over state-graph paths from the entry state to an exit
//! block of the summed site-stable charges`. The profile's charge
//! coefficients, the gateway charge, and the trip ceiling ride `V` as
//! declared profile parameters (§3.2/§3.6) — never built-in constants.

use crate::insn::{Block, OP_BR, OP_CLOAD, OP_HALT, OP_JMP};
use std::collections::HashMap;

pub struct Profile {
    pub gateway: u64,
    /// One coefficient per declared charge class, keyed by opcode tag.
    pub charge: [u64; 17],
    pub trip_ceiling: u32,
    /// Declared pre-DP structural state-cardinality ceiling (`S_max`).
    pub s_max: u64,
}

/// The declared profile of the corpus (`qir-prof-golden-2`): gateway 20 U,
/// every rule charge class 4 U (site-stable, inclusive of the base dispatch
/// quantity), trip ceiling 1 per back-edge site (the depth ceiling of earlier
/// versions is retired — F26, v0.3.0-draft.22), and the pre-DP structural
/// state-cardinality ceiling `S_max = 4096` (Wave 1A, draft.31).
pub fn golden_profile() -> Profile {
    let mut charge = [0u64; 17];
    for tag in 1..=16u64 {
        charge[tag as usize] = 4;
    }
    Profile {
        gateway: 20,
        charge,
        trip_ceiling: 1,
        s_max: 4096,
    }
}

/// Static block-graph edges: every edge of every terminator (BR: both edges;
/// JMP: its edge; CLOAD as a block's non-final instruction contributes its
/// fallback edge too; HALT: none) — §2.6's back-edge declaration reads this
/// graph.
fn block_graph(blocks: &[Block]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for b in blocks {
        for ins in &b.insns {
            match ins.tag {
                OP_BR => {
                    edges.push((b.id as usize, ins.targets[0] as usize));
                    edges.push((b.id as usize, ins.targets[1] as usize));
                }
                OP_JMP => edges.push((b.id as usize, ins.targets[0] as usize)),
                OP_CLOAD => edges.push((b.id as usize, ins.targets[0] as usize)),
                _ => {}
            }
        }
    }
    edges
}

/// Iterative dominators over the block graph; entry = block 0.
fn dominators(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
    // dom[b][c] == true  <=>  c dominates b
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
    dom
}

/// A back-edge is declared by the graph itself: its target dominates its
/// source (§2.6).
fn back_edges(n: usize, edges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let dom = dominators(n, edges);
    let mut bes: Vec<(usize, usize)> =
        edges.iter().cloned().filter(|(s, d)| dom[*s][*d]).collect();
    bes.sort();
    bes.dedup();
    bes
}

#[derive(Clone, Debug)]
enum Term {
    Halt,
    Jmp { target: usize },
    Branch { true_b: usize, false_b: usize },
    Cload { at: usize, fallback: usize },
}

/// qir-prof-ext-calls-1 site-stable charges for extension tags (§7.5/§8.4);
/// core tags keep their profile charge. No default is inferred for an
/// unassigned tag — it inherits the declared core class.
pub fn charge_of(profile: &Profile, tag: u64) -> u64 {
    if (tag as usize) < profile.charge.len() {
        return profile.charge[tag as usize];
    }
    match tag {
        17 | 18 | 23 | 24 => 4,
        19 | 25 | 26 => 8,
        20 | 27 => 10,
        21 | 28 => 16,
        30 | 31 => 6,
        32..=36 => 4,          // §9.5: fixed-width bitwise
        37 => hash_charge(64), // the declared default when no bound is carried
        _ => 4,
    }
}

/// §9.5: the governed `Q_HASH` charge is a function of the declared byte
/// bound, never of the runtime payload length (F1/F14).
pub fn hash_charge(b_bnd: u64) -> u64 {
    160 + 130 * (b_bnd.saturating_add(63) / 64)
}

/// Charge of one decoded instruction, using its declared `b_bnd` for tag 37.
pub fn charge_ins(profile: &Profile, ins: &crate::insn::Insn) -> u64 {
    if ins.tag == crate::insn::OP_HASH {
        hash_charge(ins.addr.unwrap_or(64))
    } else {
        charge_of(profile, ins.tag)
    }
}

/// Walk a block's instructions: the sum of charges up to the terminator,
/// and the terminator's descriptor (§3.4's unrolling model).
fn block_charge_and_flow(blocks: &[Block], profile: &Profile, bid: usize) -> (u64, Term) {
    let b = &blocks[bid];
    let mut total = 0u64;
    for (k, ins) in b.insns.iter().enumerate() {
        total += charge_ins(profile, ins);
        match ins.tag {
            OP_BR => {
                return (
                    total,
                    Term::Branch {
                        true_b: ins.targets[0] as usize,
                        false_b: ins.targets[1] as usize,
                    },
                )
            }
            OP_JMP => {
                return (total, Term::Jmp { target: ins.targets[0] as usize })
            }
            OP_CLOAD => {
                // CLOAD mid-block: the miss edge leaves the block; the hit
                // path continues.  Modeled as a branch point at this index.
                return (
                    total,
                    Term::Cload {
                        at: k,
                        fallback: ins.targets[0] as usize,
                    },
                )
            }
            OP_HALT | 31 => return (total, Term::Halt),
            _ => {}
        }
    }
    unreachable!("block without terminator")
}

struct Ctx<'a> {
    blocks: &'a [Block],
    profile: &'a Profile,
    be_index: HashMap<(usize, usize), usize>,
    memo: HashMap<(usize, Vec<u32>), Option<u64>>,
}

/// An edge traversal either exceeds the declared trip ceiling (Refused —
/// the traversal is not a transition of the unrolled state graph) or
/// decrements the site's remaining budget (Go).
enum Edge {
    Refused,
    Go(Vec<u32>),
}

fn step_edge(ctx: &Ctx, src: usize, tgt: usize, rem: &[u32]) -> Edge {
    match ctx.be_index.get(&(src, tgt)) {
        None => Edge::Go(rem.to_vec()),
        Some(&i) => {
            if rem[i] == 0 {
                Edge::Refused
            } else {
                let mut nr = rem.to_vec();
                nr[i] -= 1;
                Edge::Go(nr)
            }
        }
    }
}

fn join(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.max(y)),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

/// Max charge from state (bid, rem) to an exit (HALT completion); None when
/// every continuation exceeds a declared ceiling.
fn val(ctx: &mut Ctx, bid: usize, rem: &[u32]) -> Option<u64> {
    let key = (bid, rem.to_vec());
    if let Some(v) = ctx.memo.get(&key) {
        return *v;
    }
    // Cycle guard (mirrors the Python reference's pre-insert): a state
    // re-entered while its own value is still undecided lies on a closed loop,
    // and a closed loop is not a ceiling-bounded path.
    ctx.memo.insert(key.clone(), None);
    let (total, term) = block_charge_and_flow(ctx.blocks, ctx.profile, bid);
    let v = match term {
        Term::Halt => Some(total),
        Term::Jmp { target } => match step_edge(ctx, bid, target, rem) {
            Edge::Refused => None,
            Edge::Go(nr) => val(ctx, target, &nr).map(|v| total + v),
        },
        Term::Branch { true_b, false_b } => {
            let a = match step_edge(ctx, bid, true_b, rem) {
                Edge::Refused => None,
                Edge::Go(nr) => val(ctx, true_b, &nr),
            };
            let b = match step_edge(ctx, bid, false_b, rem) {
                Edge::Refused => None,
                Edge::Go(nr) => val(ctx, false_b, &nr),
            };
            join(a, b).map(|v| v + total)
        }
        Term::Cload { at, fallback } => {
            // miss edge to the fallback target (the traversal decrements the
            // site's remaining trip budget when it is a declared back edge);
            // hit path continues in-block.
            let rest = in_block_continue(ctx, bid, at + 1, rem);
            let miss = match step_edge(ctx, bid, fallback, rem) {
                Edge::Refused => None,
                Edge::Go(nr) => val(ctx, fallback, &nr),
            };
            join(rest, miss).map(|v| v + total)
        }
    };
    ctx.memo.insert(key, v);
    v
}

/// Charge of executing a block's instructions from `from_idx` to the
/// terminator (the hit path of a mid-block CLOAD).
fn in_block_continue(ctx: &mut Ctx, bid: usize, from_idx: usize, rem: &[u32]) -> Option<u64> {
    let b = &ctx.blocks[bid];
    let mut total = 0u64;
    for k in from_idx..b.insns.len() {
        let ins = &b.insns[k];
        total += charge_ins(ctx.profile, ins);
        match ins.tag {
            OP_HALT => return Some(total),
            OP_JMP => {
                let tgt = ins.targets[0] as usize;
                return match step_edge(ctx, bid, tgt, rem) {
                    Edge::Refused => None,
                    Edge::Go(nr) => val(ctx, tgt, &nr).map(|v| total + v),
                };
            }
            OP_BR => {
                let t = ins.targets[0] as usize;
                let f = ins.targets[1] as usize;
                let a = match step_edge(ctx, bid, t, rem) {
                    Edge::Refused => None,
                    Edge::Go(nr) => val(ctx, t, &nr),
                };
                let b2 = match step_edge(ctx, bid, f, rem) {
                    Edge::Refused => None,
                    Edge::Go(nr) => val(ctx, f, &nr),
                };
                return join(a, b2).map(|v| v + total);
            }
            OP_CLOAD => {
                let rest = in_block_continue(ctx, bid, k + 1, rem);
                let tgt = ins.targets[0] as usize;
                let miss = match step_edge(ctx, bid, tgt, rem) {
                    Edge::Refused => None,
                    Edge::Go(nr) => val(ctx, tgt, &nr),
                };
                return join(rest, miss).map(|v| v + total);
            }
            OP_HALT | 31 => return Some(total),
            _ => {}
        }
    }
    unreachable!("block without terminator")
}

/// The `B_hat` dynamic program (§3.4, draft.31). Returns `None` when the
/// structural state cardinality cannot be bounded by the declared `S_max`,
/// or when the state graph has no ceiling-bounded path to an exit (including
/// irreducible cycles). `None` is an admission refusal at the gate — it is
/// never flattened to the gateway charge alone.
pub fn b_hat(blocks: &[Block], profile: &Profile) -> Option<u64> {
    let n = blocks.len();

    // Pre-dominator structural gate: an empty program, or a block count above
    // the declared S_max, is refused before `dominators()` / `back_edges()` is
    // entered. This bounds the O(|B|^3) dominator work by S_max and keeps a
    // > 4096-block Intent from costing any dominator computation at all.
    if n == 0 || n as u128 > profile.s_max as u128 {
        return None;
    }

    let edges = block_graph(blocks);
    let bes = back_edges(n, &edges);

    // Mandatory pre-DP structural gate (§3.4): compute
    // |B| * (trip_ceiling + 1)^|back-edges| in overflow-checked u128 before
    // any state table is allocated. Overflow or a product above S_max refuses
    // the Intent immediately; no partial/truncated DP is permitted.
    let trip_states = (profile.trip_ceiling as u128) + 1;
    let dp_bound = trip_states
        .checked_pow(bes.len() as u32)
        .and_then(|p| (n as u128).checked_mul(p));
    match dp_bound {
        Some(bound) if bound <= profile.s_max as u128 => {}
        _ => return None,
    }

    let be_index: HashMap<(usize, usize), usize> = bes
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let entry_rem: Vec<u32> = vec![profile.trip_ceiling; bes.len()];
    let mut ctx = Ctx {
        blocks,
        profile,
        be_index,
        memo: HashMap::new(),
    };
    val(&mut ctx, 0, &entry_rem).map(|v| profile.gateway + v)
}

// ---------------------------------------------------------------------------
// §9.4: SCC-additive admission surrogate (qir-enc-5). Tarjan condensation,
// exit-aware per-SCC trip-ceiling DP, condensation-DAG max-path aggregation.
// The value is identical to the global-product surrogate on every globally
// admitted graph; only the structural state count is widened.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Exit {
    Halt,
    To(usize),
}
type Exits = HashMap<Exit, u64>;

#[derive(Clone, Debug)]
enum SccTerm {
    Halt,
    Jmp(usize),
    Branch(usize, usize),
    Cload(usize, usize),
}

fn scc_flow(blocks: &[Block], profile: &Profile, bid: usize) -> (u64, SccTerm) {
    let b = &blocks[bid];
    let mut total = 0u64;
    for (k, i) in b.insns.iter().enumerate() {
        total += charge_ins(profile, i);
        match i.tag {
            crate::insn::OP_BR => {
                return (total, SccTerm::Branch(i.targets[0] as usize, i.targets[1] as usize))
            }
            crate::insn::OP_JMP => return (total, SccTerm::Jmp(i.targets[0] as usize)),
            crate::insn::OP_CLOAD => return (total, SccTerm::Cload(k, i.targets[0] as usize)),
            crate::insn::OP_HALT | 31 => return (total, SccTerm::Halt),
            _ => {}
        }
    }
    unreachable!("block without terminator")
}

/// Tarjan SCCs, deterministic by ascending block id, renumbered so that
/// component order is topological by first member. O(|V|+|E|).
pub fn tarjan(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    for &(s, d) in edges {
        if s < n && d < n {
            adj[s].push(d);
        }
    }
    for a in &mut adj {
        a.sort_unstable();
        a.dedup();
    }
    let mut index = 0usize;
    let mut ids = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut on = vec![false; n];
    let mut stack = Vec::new();
    let mut comp = vec![usize::MAX; n];
    let mut ncomp = 0usize;
    fn strong(
        v: usize,
        adj: &[Vec<usize>],
        index: &mut usize,
        ids: &mut [usize],
        low: &mut [usize],
        on: &mut [bool],
        stack: &mut Vec<usize>,
        comp: &mut [usize],
        ncomp: &mut usize,
    ) {
        ids[v] = *index;
        low[v] = *index;
        *index += 1;
        stack.push(v);
        on[v] = true;
        for &w in &adj[v] {
            if ids[w] == usize::MAX {
                strong(w, adj, index, ids, low, on, stack, comp, ncomp);
                low[v] = low[v].min(low[w]);
            } else if on[w] {
                low[v] = low[v].min(ids[w]);
            }
        }
        if low[v] == ids[v] {
            loop {
                let w = stack.pop().unwrap();
                on[w] = false;
                comp[w] = *ncomp;
                if w == v {
                    break;
                }
            }
            *ncomp += 1;
        }
    }
    for v in 0..n {
        if ids[v] == usize::MAX {
            strong(v, &adj, &mut index, &mut ids, &mut low, &mut on, &mut stack, &mut comp, &mut ncomp);
        }
    }
    let mut min_member = vec![usize::MAX; ncomp];
    for (v, &c) in comp.iter().enumerate() {
        min_member[c] = min_member[c].min(v);
    }
    let mut order: Vec<usize> = (0..ncomp).collect();
    order.sort_by_key(|&c| min_member[c]);
    let mut remap = vec![0usize; ncomp];
    for (new, &old) in order.iter().enumerate() {
        remap[old] = new;
    }
    comp.into_iter().map(|c| remap[c]).collect()
}

fn join_exits(a: Option<Exits>, b: Option<Exits>) -> Option<Exits> {
    match (a, b) {
        (Some(mut x), Some(y)) => {
            for (k, v) in y {
                x.entry(k).and_modify(|e| *e = (*e).max(v)).or_insert(v);
            }
            Some(x)
        }
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn add_prefix(m: Exits, p: u64) -> Exits {
    m.into_iter().map(|(k, v)| (k, v + p)).collect()
}

fn singleton(k: Exit, v: u64) -> Exits {
    let mut m = HashMap::new();
    m.insert(k, v);
    m
}

struct Local<'a> {
    blocks: &'a [Block],
    profile: &'a Profile,
    comp: &'a [usize],
    target: usize,
    be: HashMap<(usize, usize), usize>,
    memo: HashMap<(usize, Vec<u32>), Option<Exits>>,
}

impl<'a> Local<'a> {
    fn in_scc(&self, bid: usize) -> bool {
        self.comp[bid] == self.target
    }
    fn step(&self, src: usize, dst: usize, rem: &[u32]) -> Option<Vec<u32>> {
        match self.be.get(&(src, dst)) {
            None => Some(rem.to_vec()),
            Some(&i) => {
                if rem[i] == 0 {
                    None
                } else {
                    let mut r = rem.to_vec();
                    r[i] -= 1;
                    Some(r)
                }
            }
        }
    }
    fn val(&mut self, bid: usize, rem: &[u32]) -> Option<Exits> {
        let key = (bid, rem.to_vec());
        if let Some(v) = self.memo.get(&key) {
            return v.clone();
        }
        self.memo.insert(key.clone(), None);
        let (total, term) = scc_flow(self.blocks, self.profile, bid);
        let out = match term {
            SccTerm::Halt => Some(singleton(Exit::Halt, total)),
            SccTerm::Jmp(t) => {
                if self.in_scc(t) {
                    self.step(bid, t, rem).and_then(|r| self.val(t, &r)).map(|m| add_prefix(m, total))
                } else {
                    Some(singleton(Exit::To(self.comp[t]), total))
                }
            }
            SccTerm::Branch(t, f) => {
                let a = if self.in_scc(t) {
                    self.step(bid, t, rem).and_then(|r| self.val(t, &r)).map(|m| add_prefix(m, total))
                } else {
                    Some(singleton(Exit::To(self.comp[t]), total))
                };
                let b = if self.in_scc(f) {
                    self.step(bid, f, rem).and_then(|r| self.val(f, &r)).map(|m| add_prefix(m, total))
                } else {
                    Some(singleton(Exit::To(self.comp[f]), total))
                };
                join_exits(a, b)
            }
            SccTerm::Cload(k, f) => {
                let rest = self.cont(bid, k + 1, rem).map(|m| add_prefix(m, total));
                let miss = if self.in_scc(f) {
                    self.step(bid, f, rem).and_then(|r| self.val(f, &r)).map(|m| add_prefix(m, total))
                } else {
                    Some(singleton(Exit::To(self.comp[f]), total))
                };
                join_exits(rest, miss)
            }
        };
        self.memo.insert(key, out.clone());
        out
    }
    fn cont(&mut self, bid: usize, from: usize, rem: &[u32]) -> Option<Exits> {
        let mut total = 0u64;
        for k in from..self.blocks[bid].insns.len() {
            let i = &self.blocks[bid].insns[k];
            total += charge_ins(self.profile, i);
            match i.tag {
                crate::insn::OP_HALT => return Some(singleton(Exit::Halt, total)),
                crate::insn::OP_JMP => {
                    let t = i.targets[0] as usize;
                    return if self.in_scc(t) {
                        self.step(bid, t, rem).and_then(|r| self.val(t, &r)).map(|m| add_prefix(m, total))
                    } else {
                        Some(singleton(Exit::To(self.comp[t]), total))
                    };
                }
                crate::insn::OP_BR => {
                    let t = i.targets[0] as usize;
                    let f = i.targets[1] as usize;
                    let a = if self.in_scc(t) {
                        self.step(bid, t, rem).and_then(|r| self.val(t, &r)).map(|m| add_prefix(m, total))
                    } else {
                        Some(singleton(Exit::To(self.comp[t]), total))
                    };
                    let b = if self.in_scc(f) {
                        self.step(bid, f, rem).and_then(|r| self.val(f, &r)).map(|m| add_prefix(m, total))
                    } else {
                        Some(singleton(Exit::To(self.comp[f]), total))
                    };
                    return join_exits(a, b);
                }
                crate::insn::OP_CLOAD => {
                    let rest = self.cont(bid, k + 1, rem).map(|m| add_prefix(m, total));
                    let t = i.targets[0] as usize;
                    let miss = if self.in_scc(t) {
                        self.step(bid, t, rem).and_then(|r| self.val(t, &r)).map(|m| add_prefix(m, total))
                    } else {
                        Some(singleton(Exit::To(self.comp[t]), total))
                    };
                    return join_exits(rest, miss);
                }
                crate::insn::OP_HALT | 31 => return Some(singleton(Exit::Halt, total)),
                _ => {}
            }
        }
        unreachable!("block without terminator")
    }
}

/// §9.4 structural state count: Σ_i |SCC_i|·(trip+1)^|BE_i|.
pub fn scc_dp_states(blocks: &[Block], profile: &Profile) -> u128 {
    let n = blocks.len();
    if n == 0 {
        return 0;
    }
    let edges = block_graph(blocks);
    let bes = back_edges(n, &edges);
    let comp = tarjan(n, &edges);
    let nc = comp.iter().copied().max().unwrap_or(0) + 1;
    (0..nc)
        .map(|c| {
            let members = comp.iter().filter(|&&x| x == c).count() as u128;
            let k = bes.iter().filter(|(s, d)| comp[*s] == c && comp[*d] == c).count() as u32;
            ((profile.trip_ceiling as u128) + 1)
                .checked_pow(k)
                .map(|x| members * x)
                .unwrap_or(u128::MAX)
        })
        .sum()
}

/// The SCC-additive admission bound of §9.4; `None` is a refusal.
pub fn bhat_scc(blocks: &[Block], profile: &Profile) -> Option<u64> {
    let n = blocks.len();
    if n == 0 || n as u128 > profile.s_max as u128 {
        return None;
    }
    let edges = block_graph(blocks);
    let bes = back_edges(n, &edges);
    if scc_dp_states(blocks, profile) > profile.s_max as u128 {
        return None;
    }
    let comp = tarjan(n, &edges);
    let nc = comp.iter().copied().max().unwrap_or(0) + 1;
    let local_be: Vec<Vec<(usize, usize)>> = (0..nc)
        .map(|c| bes.iter().copied().filter(|(s, d)| comp[*s] == c && comp[*d] == c).collect())
        .collect();
    let mut cross_in: Vec<Vec<usize>> = vec![Vec::new(); nc];
    for &(s, d) in &edges {
        if comp[s] != comp[d] {
            cross_in[comp[d]].push(s);
        }
    }
    let mut local: Vec<Option<Exits>> = vec![None; nc];
    for c in 0..nc {
        let entries: Vec<usize> = if comp[0] == c {
            vec![0]
        } else {
            let mut v: Vec<usize> = (0..n).filter(|&b| comp[b] == c && cross_in[c].contains(&b)).collect();
            if v.is_empty() {
                v.push((0..n).find(|&b| comp[b] == c).unwrap());
            }
            v
        };
        let be_index: HashMap<(usize, usize), usize> =
            local_be[c].iter().cloned().enumerate().map(|(i, e)| (e, i)).collect();
        let mut ctx = Local {
            blocks,
            profile,
            comp: &comp,
            target: c,
            be: be_index,
            memo: HashMap::new(),
        };
        let rem = vec![profile.trip_ceiling; local_be[c].len()];
        let mut best: Option<Exits> = None;
        for b in entries {
            best = join_exits(best, ctx.val(b, &rem));
        }
        local[c] = best;
    }
    // Condensation is a DAG; min-member renumbering is topological.
    let mut best = vec![None::<u64>; nc];
    for c in (0..nc).rev() {
        let Some(m) = &local[c] else { continue };
        let mut value = None::<u64>;
        for (exit, charge) in m {
            let tail = match exit {
                Exit::Halt => Some(0u64),
                Exit::To(d) => best[*d],
            };
            if let Some(t) = tail {
                value = Some(value.map_or(charge + t, |v: u64| v.max(charge + t)));
            }
        }
        best[c] = value;
    }
    best[comp[0]].map(|v| profile.gateway + v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insn::Insn;

    fn jmp_block(id: u64, target: u64) -> Block {
        Block {
            id,
            insns: vec![Insn {
                tag: OP_JMP,
                ext: 0,
                refs: vec![],
                targets: vec![target],
                addr: None,
            }],
        }
    }

    fn halt_block(id: u64) -> Block {
        Block {
            id,
            insns: vec![Insn {
                tag: OP_HALT,
                ext: 0,
                refs: vec![],
                targets: vec![],
                addr: None,
            }],
        }
    }

    #[test]
    fn infinite_jmp_zero_loop_is_refused_not_charged_gateway() {
        // A self-loop under trip ceiling 1 has no ceiling-bounded exit path:
        // val(...) is None and b_hat MUST refuse (never return gateway = 20).
        let blocks = vec![jmp_block(0, 0)];
        let profile = golden_profile();
        assert_eq!(b_hat(&blocks, &profile), None);
        assert_eq!(b_hat(&blocks, &profile), None);
    }

    #[test]
    fn thirteen_back_edges_exceed_s_max_and_refuse_before_dp() {
        // |B| * (trip_ceiling + 1)^back_edges = 14 * 2^13 = 114688 > 4096.
        // The pre-DP gate returns None without allocating the state table.
        let mut blocks: Vec<Block> = (0..13).map(|id| jmp_block(id, id)).collect();
        blocks.push(halt_block(13));
        let profile = golden_profile();
        assert_eq!(b_hat(&blocks, &profile), None);
    }

    #[test]
    fn block_count_above_s_max_refuses_before_dominators() {
        // |B| = 4097 > S_max = 4096: the pre-dominator guard returns None
        // without entering the O(|B|^3) dominator computation. An empty
        // program is refused for the same structural reason.
        let blocks: Vec<Block> = (0..=4096).map(halt_block).collect();
        assert_eq!(blocks.len(), 4097);
        let profile = golden_profile();
        assert_eq!(b_hat(&blocks, &profile), None);
        assert_eq!(b_hat(&[], &profile), None);
    }

    #[test]
    fn simple_halt_still_yields_exact_bound() {
        // gateway 20 + one HALT charge 4 = 24 U on a terminating program.
        let blocks = vec![halt_block(0)];
        let profile = golden_profile();
        assert_eq!(b_hat(&blocks, &profile), Some(24));
    }
}
