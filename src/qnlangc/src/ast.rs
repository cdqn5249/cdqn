//! Immutable CAE-AST: hash-consed, content-addressed expression DAG with a
//! De-Bruijn binder-depth attribute, plus the optional QO-24 NameMap sidecar
//! (docs/qnlang.md §2.1–§2.2).

use crate::diag::Span;
use std::collections::HashMap;

pub type NodeId = usize;

#[derive(Clone, Debug)]
pub enum NodeKind {
    QPrim { neg: bool, core: String, unit: Option<String> },
    Ident(String),
    Tuple(Vec<NodeId>),
    Call { name: String, args: Vec<NodeId> },
    Ctor { name: String, args: String },
    Split { scrut: NodeId, t: NodeId, f: NodeId },
    Match { scrut: NodeId, arms: Vec<(String, NodeId)> },
    Fold { seq: NodeId, over: NodeId, init: NodeId, binders: Vec<String>, body: NodeId },
    Loop { trip: u64, body: NodeId },
    Load { tbl: NodeId, key: NodeId, trip: u64, target: String, or_else: Option<NodeId> },
    Binary { op: String, lhs: NodeId, rhs: NodeId },
    Range { lo: NodeId, hi: NodeId, excl: bool },
    Bind { target: String, ty: Option<String>, value: NodeId },
    Postfix { base: NodeId, ops: Vec<String> },
    Block { stmts: Vec<NodeId>, tail: Option<NodeId> },
    Emit(String),
}

#[derive(Clone, Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
    /// De-Bruijn index of the enclosing binder chain (never a mutable cell).
    pub depth: usize,
    pub digest: String,
}

#[derive(Clone, Debug)]
pub struct Header {
    pub kind: HeaderKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum HeaderKind {
    Dcc(String),
    Kappa(String),
    Domain(Vec<String>),
    Mode(String),
}

#[derive(Clone, Debug)]
pub struct TopDecl {
    pub kind: TopKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum TopKind {
    Type { name: String, ty: String },
    Anchor { name: String, core: String, ty: String },
    Metric { name: String, args: String },
    Blanket { name: String, params: Vec<(String, String)> },
    Variant { name: String, arms: Vec<String> },
    Pattern { name: String, params: Vec<(String, String)>, ret: String, contract: String, body: NodeId },
    Workflow { name: String, params: Vec<(String, String)>, ret: String, contract: String, body: NodeId },
    Conjecture { name: String, contract: String, body: NodeId },
    Falsify { name: String, contract: String, body: NodeId },
    Experiment { name: String, for_name: String, contract: String, cases: usize },
}

#[derive(Clone, Debug)]
pub struct Program {
    pub headers: Vec<Header>,
    pub tops: Vec<TopDecl>,
    pub arena: Vec<Node>,
    /// Optional deterministic sidecar: digest -> human-facing names.
    pub name_map: HashMap<String, Vec<String>>,
    pub use_name_map: bool,
}

impl Program {
    pub fn node(&self, id: NodeId) -> &Node {
        &self.arena[id]
    }
}

/// FNV-1a over the canonical bytes; a stable stand-in for the profile-bound
/// commitment used by the lowering companions. It is never consulted by
/// typing, equality or evaluation (§2.2).
pub fn digest(s: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{:016x}", h)
}

pub struct Arena {
    pub nodes: Vec<Node>,
    cons: HashMap<String, NodeId>,
    pub name_map: HashMap<String, Vec<String>>,
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

impl Arena {
    pub fn new() -> Self {
        Arena { nodes: Vec::new(), cons: HashMap::new(), name_map: HashMap::new() }
    }

    fn canonical(kind: &NodeKind) -> String {
        match kind {
            NodeKind::QPrim { neg, core, unit } => format!("QPrim({},{},{})", neg, core, unit.clone().unwrap_or_default()),
            NodeKind::Ident(s) => format!("Ident({})", s),
            NodeKind::Tuple(v) => format!("Tuple({:?})", v),
            NodeKind::Call { name, args } => format!("Call({},{:?})", name, args),
            NodeKind::Ctor { name, args } => format!("Ctor({},{})", name, args),
            NodeKind::Split { scrut, t, f } => format!("Split({},{},{})", scrut, t, f),
            NodeKind::Match { scrut, arms } => format!("Match({},{:?})", scrut, arms),
            NodeKind::Fold { seq, over, init, binders, body } => format!("Fold({},{},{},{:?},{})", seq, over, init, binders, body),
            NodeKind::Loop { trip, body } => format!("Loop({},{})", trip, body),
            NodeKind::Load { tbl, key, trip, target, or_else } => format!("Load({},{},{},{},{:?})", tbl, key, trip, target, or_else),
            NodeKind::Binary { op, lhs, rhs } => format!("Binary({},{},{})", op, lhs, rhs),
            NodeKind::Range { lo, hi, excl } => format!("Range({},{},{})", lo, hi, excl),
            NodeKind::Bind { target, ty, value } => format!("Bind({},{:?},{})", target, ty, value),
            NodeKind::Postfix { base, ops } => format!("Postfix({},{:?})", base, ops),
            NodeKind::Block { stmts, tail } => format!("Block({:?},{:?})", stmts, tail),
            NodeKind::Emit(s) => format!("Emit({})", s),
        }
    }

    pub fn node(&self, id: NodeId) -> &Node {
        &self.nodes[id]
    }

    pub fn intern(&mut self, kind: NodeKind, span: Span, depth: usize) -> NodeId {
        let key = Self::canonical(&kind);
        if let Some(&id) = self.cons.get(&key) {
            return id;
        }
        let d = digest(&key);
        let id = self.nodes.len();
        self.nodes.push(Node { kind, span, depth, digest: d.clone() });
        self.cons.insert(key, id);
        id
    }

    /// A sidecar annotation: it never changes a commitment (§2.2).
    pub fn annotate_name(&mut self, node: NodeId, name: &str) {
        let d = self.nodes[node].digest.clone();
        let v = self.name_map.entry(d).or_default();
        if !v.iter().any(|n| n == name) {
            v.push(name.to_string());
        }
    }
}
