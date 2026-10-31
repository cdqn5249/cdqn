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
}

/// The declared profile of the corpus (`qir-prof-golden-1`): gateway 20 U,
/// every rule charge class 4 U (site-stable, inclusive of the base dispatch
/// quantity), trip ceiling 1 per back-edge site, depth ceiling 8.
pub fn golden_profile() -> Profile {
    let mut charge = [0u64; 17];
    for tag in 1..=16u64 {
        charge[tag as usize] = 4;
    }
    Profile {
        gateway: 20,
        charge,
        trip_ceiling: 1,
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

/// Walk a block's instructions: the sum of charges up to the terminator,
/// and the terminator's descriptor (§3.4's unrolling model).
fn block_charge_and_flow(blocks: &[Block], profile: &Profile, bid: usize) -> (u64, Term) {
    let b = &blocks[bid];
    let mut total = 0u64;
    for (k, ins) in b.insns.iter().enumerate() {
        total += profile.charge[ins.tag as usize];
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
            OP_HALT => return (total, Term::Halt),
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
            // miss edge to the fallback target; hit path continues in-block
            let rest = in_block_continue(ctx, bid, at + 1, rem);
            let miss = val(ctx, fallback, rem);
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
        total += ctx.profile.charge[ins.tag as usize];
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
                let miss = val(ctx, ins.targets[0] as usize, rem);
                return join(rest, miss).map(|v| v + total);
            }
            _ => {}
        }
    }
    unreachable!("block without terminator")
}

/// The `B_hat` dynamic program (§3.4). Returns the surrogate bound in U
/// (gateway included). A graph with no ceiling-bounded path to an exit
/// yields the gateway alone, matching the declared surrogate's behavior.
pub fn b_hat(blocks: &[Block], profile: &Profile) -> u64 {
    let n = blocks.len();
    let edges = block_graph(blocks);
    let bes = back_edges(n, &edges);
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
    match val(&mut ctx, 0, &entry_rem) {
        Some(v) => profile.gateway + v,
        None => profile.gateway,
    }
}
