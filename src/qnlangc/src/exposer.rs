//! `Exposer` — the deterministic `qir-verbalizer-1` full-program exposition
//! and bounded read-back surface (docs/qnlang.md §6.3 Channel B;
//! docs/qnir_commitments.md §6.4, Obj 15).
//!
//! The Exposer consumes a `Checker`-verified `CAE-AST` — the interlock marker
//! is `H(T | Contract) = 0` — and renders it into a stable, bounded,
//! human-readable read-back. It is pure by construction:
//!
//! * no clock, no RNG, no environment, no absolute paths;
//! * no `HashMap` iteration — only `Program` order and source order;
//! * the digest covers the exposition body only, so the footer that carries
//!   the digest is never self-referential.
//!
//! `Obj 15` binds a read-back as the pair `(verbalizer identity, SHA-256 of
//! the exact text bytes)`. `GENERATION` is that identity's generation string;
//! any change to the rendered bytes requires a new generation.

use crate::ast::*;
use crate::lower::LowerResult;
use crate::sha256;
use std::fmt::Write as _;

/// The declared verbalizer identity (docs/qnir_commitments.md §6.4).
pub const VERBALIZER: &str = "qir-verbalizer-1";
/// The exposition generation. Changes to any committed byte must bump this.
pub const GENERATION: &str = "qnlang-expose-1";

/// A bounded read-back: the body text plus the `Obj 15` commitment over it.
#[derive(Clone, Debug)]
pub struct Exposition {
    /// The exact text hashed by `sha256` (UTF-8, LF, no trailing footer).
    pub body: String,
    /// Number of lines in `body`.
    pub lines: usize,
    /// Number of UTF-8 bytes in `body`.
    pub bytes: usize,
    /// Full lowercase hex SHA-256 of `body`.
    pub sha256: String,
    /// The same digest with its algorithm tag, `sha-256:<hex>`.
    pub sha256_tagged: String,
}

impl Exposition {
    /// `Obj 15` confirmation: does `expected` name this exact read-back?
    /// Accepts bare hex or the tagged `sha-256:<hex>` form, case-insensitively.
    pub fn matches(&self, expected: &str) -> bool {
        let e = expected.trim().to_ascii_lowercase();
        let e = e.strip_prefix("sha-256:").unwrap_or(&e);
        e == self.sha256
    }
}

/// Render a `Checker`-verified program into its bounded read-back.
///
/// `file` is the caller-supplied label only; it is never resolved or read.
pub fn expose(p: &Program, r: &LowerResult, file: &str) -> Exposition {
    let mut s = String::new();

    let _ = writeln!(s, "{} exposition ({})", VERBALIZER, GENERATION);
    let _ = writeln!(s, "file: {}", file_label(file));
    let _ = writeln!(s, "checker: G1-G14 + E_NAMESPACE_COLLISION = 0");
    let _ = writeln!(s, "interlock: H(T | Contract) = 0");
    let _ = writeln!(s, "epistemic: MODAL_HYPOTHESIS (uncertified; co-signature pending)");

    // ---- Header ---------------------------------------------------------
    let _ = writeln!(s, "[header]");
    for h in &p.headers {
        match &h.kind {
            HeaderKind::Dcc(v) => {
                let _ = writeln!(s, "  Q(dcc) {}", v);
            }
            HeaderKind::Kappa(v) => {
                let _ = writeln!(s, "  Q(kappa) {}", v);
            }
            HeaderKind::Domain(v) => {
                let _ = writeln!(s, "  Q(domain) {}", render_domain(v));
            }
            HeaderKind::Mode(v) => {
                let _ = writeln!(s, "  Q(mode) {}", v);
            }
        }
    }

    // ---- Declarations (source order) -----------------------------------
    let _ = writeln!(s, "[declarations]");
    let mut any_decl = false;
    for t in &p.tops {
        match &t.kind {
            TopKind::Type { name, ty } => {
                any_decl = true;
                let _ = writeln!(s, "  type {} = {}{}", name, ty, ty_annot(ty));
            }
            TopKind::Anchor { name, core, ty } => {
                any_decl = true;
                let _ = writeln!(s, "  anchor {} = {} : {}{}", name, render_core(core), ty, ty_annot(ty));
            }
            TopKind::Metric { name, args } => {
                any_decl = true;
                let _ = writeln!(s, "  metric {} = eta({})", name, args);
            }
            TopKind::Blanket { name, params } => {
                any_decl = true;
                let fields = params
                    .iter()
                    .map(|(n, ty)| format!("{} : {}", n, ty))
                    .collect::<Vec<_>>()
                    .join("; ");
                let _ = writeln!(s, "  blanket {} {{ {} }}", name, fields);
            }
            TopKind::Variant { name, arms } => {
                any_decl = true;
                let _ = writeln!(s, "  variant {} {{ {} }}", name, arms.join(", "));
            }
            _ => {}
        }
    }
    if !any_decl {
        let _ = writeln!(s, "  (none)");
    }

    // ---- Affine containers ---------------------------------------------
    // Usage is program-global: a binding in one entry statement may be
    // consumed or dropped by a later one, so the scan is not per-body.
    let mut dropped: Vec<String> = Vec::new();
    let mut used: Vec<String> = Vec::new();
    for t in &p.tops {
        let Some(body) = top_body(&t.kind) else { continue };
        for id in collect_nodes(p, body) {
            match &p.node(id).kind {
                NodeKind::Call { name, args } => {
                    if name == "drop" {
                        if let Some(NodeIdRef::Ident(n)) = args.first().map(|a| node_ref(p, *a)) {
                            dropped.push(n);
                        }
                    }
                    for &a in args {
                        if let NodeIdRef::Ident(n) = node_ref(p, a) {
                            used.push(n);
                        }
                    }
                }
                NodeKind::Fold { seq, .. } => {
                    if let NodeIdRef::Ident(n) = node_ref(p, *seq) {
                        used.push(n);
                    }
                }
                NodeKind::Load { tbl, .. } => {
                    if let NodeIdRef::Ident(n) = node_ref(p, *tbl) {
                        used.push(n);
                    }
                }
                _ => {}
            }
        }
    }
    let _ = writeln!(s, "[containers]");
    let mut any_container = false;
    for t in &p.tops {
        let Some(body) = top_body(&t.kind) else { continue };
        for id in collect_nodes(p, body) {
            if let NodeKind::Bind { target, ty, .. } = &p.node(id).kind {
                let Some(ty) = ty.as_deref() else { continue };
                if let Some(cap) = tag_arg(ty, "cap") {
                    any_container = true;
                    let status = if dropped.iter().any(|d| d == target) {
                        "dropped"
                    } else if used.iter().any(|u| u == target) {
                        "consumed"
                    } else {
                        "live"
                    };
                    let _ = writeln!(
                        s,
                        "  affine {} : {} cap={} status={}",
                        target, ty, cap, status
                    );
                }
            }
        }
    }
    if !any_container {
        let _ = writeln!(s, "  (none)");
    }

    // ---- Control and call structure ------------------------------------
    let _ = writeln!(s, "[control]");
    let mut any_control = false;
    for t in &p.tops {
        match &t.kind {
            TopKind::Pattern { name, params, ret, contract, body } => {
                any_control = true;
                let _ = writeln!(
                    s,
                    "  pattern {}({}) -> {} {} {}",
                    name,
                    render_params(params),
                    ret,
                    render_contract(contract),
                    render_body_shape(p, *body)
                );
                render_structure(p, *body, &mut s);
            }
            TopKind::Workflow { name, params, ret, contract, body } => {
                any_control = true;
                let _ = writeln!(
                    s,
                    "  workflow {}({}) -> {} {} {}",
                    name,
                    render_params(params),
                    ret,
                    render_contract(contract),
                    render_body_shape(p, *body)
                );
                render_structure(p, *body, &mut s);
            }
            TopKind::Falsify { name, contract, body } => {
                any_control = true;
                let _ = writeln!(
                    s,
                    "  falsify {} {} {}",
                    name,
                    render_contract(contract),
                    render_body_shape(p, *body)
                );
                render_structure(p, *body, &mut s);
            }
            TopKind::Conjecture { name, contract, body } => {
                any_control = true;
                let label = match name.strip_prefix("$stmt") {
                    Some(n) => format!("#{}", n),
                    None => name.clone(),
                };
                let _ = writeln!(
                    s,
                    "  entry {} {} {}",
                    label,
                    render_contract(contract),
                    render_body_shape(p, *body)
                );
                render_structure(p, *body, &mut s);
            }
            TopKind::Experiment { name, for_name, contract, cases } => {
                any_control = true;
                let _ = writeln!(
                    s,
                    "  experiment {} for {} {} cases={}",
                    name,
                    for_name,
                    render_contract(contract),
                    cases
                );
            }
            _ => {}
        }
    }
    if !any_control {
        let _ = writeln!(s, "  (none)");
    }

    // ---- Declared budgets vs B_hat -------------------------------------
    let _ = writeln!(s, "[budget]");
    if r.budget_checks.is_empty() {
        let _ = writeln!(s, "  declared (none)");
    } else {
        for (name, declared, checked) in &r.budget_checks {
            let within = if checked <= declared { "yes" } else { "no" };
            let _ = writeln!(
                s,
                "  declared {}: {} U; source-level b_hat {} U; within={}",
                name, declared, checked, within
            );
        }
    }
    let _ = writeln!(
        s,
        "  emitted entry B_hat: {} U (admission surrogate incl. folded callees)",
        r.b_hat.map(|v| v.to_string()).unwrap_or_else(|| "null".into())
    );

    let bytes = s.as_bytes().len();
    let lines = s.lines().count();
    let hex = sha256::hex(&sha256::sha256(s.as_bytes()));
    Exposition {
        body: s,
        lines,
        bytes,
        sha256_tagged: format!("sha-256:{}", hex),
        sha256: hex,
    }
}

/// A tiny borrow-free view used only to read an identifier out of a node.
enum NodeIdRef {
    Ident(String),
    Other,
}

fn node_ref(p: &Program, id: NodeId) -> NodeIdRef {
    match &p.node(id).kind {
        NodeKind::Ident(s) => NodeIdRef::Ident(s.clone()),
        _ => NodeIdRef::Other,
    }
}

/// The body node of any top declaration that carries one.
fn top_body(kind: &TopKind) -> Option<NodeId> {
    match kind {
        TopKind::Pattern { body, .. }
        | TopKind::Workflow { body, .. }
        | TopKind::Falsify { body, .. }
        | TopKind::Conjecture { body, .. } => Some(*body),
        _ => None,
    }
}

fn render_domain(v: &[String]) -> String {
    if v.len() == 1 {
        v[0].clone()
    } else {
        format!("({})", v.join(", "))
    }
}

/// A path-independent label: the exposition must be reproducible whether the
/// caller names the source relatively or absolutely.
fn file_label(file: &str) -> &str {
    file.rsplit(['\\', '/']).next().unwrap_or(file)
}

fn render_params(params: &[(String, String)]) -> String {
    params
        .iter()
        .map(|(n, ty)| format!("{} : {}", n, ty))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_contract(contract: &str) -> String {
    if contract.is_empty() {
        "![]".to_string()
    } else {
        format!("![{}]", contract)
    }
}

/// The parser stores an anchor's core without its `Q(...)` wrapper, e.g. `0`
/// or `0<unit(J)>`; restore the canonical source form.
fn render_core(core: &str) -> String {
    match core.find("<unit(") {
        Some(i) => format!("Q({}){}", &core[..i], &core[i..]),
        None => format!("Q({})", core),
    }
}

fn render_body_shape(p: &Program, body: NodeId) -> String {
    match &p.node(body).kind {
        NodeKind::Block { stmts, tail } => format!(
            "body=block(stmts={}, tail={})",
            stmts.len(),
            if tail.is_some() { "yes" } else { "no" }
        ),
        NodeKind::Bind { target, ty, .. } => format!(
            "body=bind({}{})",
            target,
            ty.as_ref().map(|t| format!(" : {}", t)).unwrap_or_default()
        ),
        _ => "body=expr".to_string(),
    }
}

/// Collect every node reachable from `root` in first-visit source order.
fn collect_nodes(p: &Program, root: NodeId) -> Vec<NodeId> {
    fn go(p: &Program, id: NodeId, seen: &mut [bool], out: &mut Vec<NodeId>) {
        if id >= seen.len() || seen[id] {
            return;
        }
        seen[id] = true;
        out.push(id);
        match &p.node(id).kind {
            NodeKind::Tuple(xs) => {
                for &x in xs {
                    go(p, x, seen, out);
                }
            }
            NodeKind::Call { args, .. } => {
                for &x in args {
                    go(p, x, seen, out);
                }
            }
            NodeKind::Split { scrut, t, f } => {
                go(p, *scrut, seen, out);
                go(p, *t, seen, out);
                go(p, *f, seen, out);
            }
            NodeKind::Match { scrut, arms } => {
                go(p, *scrut, seen, out);
                for (_, a) in arms {
                    go(p, *a, seen, out);
                }
            }
            NodeKind::Fold { seq, over, init, body, .. } => {
                go(p, *seq, seen, out);
                go(p, *over, seen, out);
                go(p, *init, seen, out);
                go(p, *body, seen, out);
            }
            NodeKind::Loop { body, .. } => go(p, *body, seen, out),
            NodeKind::Load { tbl, key, or_else, .. } => {
                go(p, *tbl, seen, out);
                go(p, *key, seen, out);
                if let Some(o) = or_else {
                    go(p, *o, seen, out);
                }
            }
            NodeKind::Binary { lhs, rhs, .. } => {
                go(p, *lhs, seen, out);
                go(p, *rhs, seen, out);
            }
            NodeKind::Range { lo, hi, .. } => {
                go(p, *lo, seen, out);
                go(p, *hi, seen, out);
            }
            NodeKind::Bind { value, .. } => go(p, *value, seen, out),
            NodeKind::Postfix { base, .. } => go(p, *base, seen, out),
            NodeKind::Block { stmts, tail } => {
                for &x in stmts {
                    go(p, x, seen, out);
                }
                if let Some(t) = tail {
                    go(p, *t, seen, out);
                }
            }
            _ => {}
        }
    }
    let mut seen = vec![false; p.arena.len()];
    let mut out = Vec::new();
    go(p, root, &mut seen, &mut out);
    out
}

fn render_structure(p: &Program, body: NodeId, s: &mut String) {
    for id in collect_nodes(p, body) {
        match &p.node(id).kind {
            NodeKind::Fold { seq, over, init, binders, .. } => {
                let _ = writeln!(
                    s,
                    "    fold seq={} over={} init({}) binders=({}) trip={}",
                    render(p, *seq),
                    render(p, *over),
                    render(p, *init),
                    binders.join(", "),
                    range_trip(p, *over)
                );
            }
            NodeKind::Load { tbl, key, trip, target, or_else } => {
                let _ = writeln!(
                    s,
                    "    load tbl={} key={} trip={} target={} or_else={}",
                    render(p, *tbl),
                    render(p, *key),
                    trip,
                    target,
                    if or_else.is_some() { "yes" } else { "no" }
                );
            }
            NodeKind::Loop { trip, .. } => {
                let _ = writeln!(s, "    loop trip={}", trip);
            }
            NodeKind::Call { name, args } => {
                if name == "call" {
                    let callee = args.first().map(|a| render(p, *a)).unwrap_or_default();
                    let _ = writeln!(
                        s,
                        "    call {} argc={}",
                        callee,
                        args.len().saturating_sub(1)
                    );
                } else {
                    let _ = writeln!(s, "    call {} argc={}", name, args.len());
                }
            }
            _ => {}
        }
    }
}

fn range_trip(p: &Program, id: NodeId) -> String {
    match &p.node(id).kind {
        NodeKind::Range { lo, hi, excl } => match (qprim_num(p, *lo), qprim_num(p, *hi)) {
            (Some(a), Some(b)) => {
                let span = b.saturating_sub(a);
                let trip = if *excl { span } else { span.saturating_add(1) };
                trip.to_string()
            }
            _ => "range".to_string(),
        },
        _ => "range".to_string(),
    }
}

fn qprim_num(p: &Program, id: NodeId) -> Option<u64> {
    match &p.node(id).kind {
        NodeKind::QPrim { neg, core, .. } if !*neg => core.parse::<u64>().ok(),
        _ => None,
    }
}

fn render(p: &Program, id: NodeId) -> String {
    match &p.node(id).kind {
        NodeKind::QPrim { neg, core, unit } => {
            let mut r = if *neg { format!("-Q({})", core) } else { format!("Q({})", core) };
            if let Some(u) = unit {
                let _ = write!(r, "<unit({})>", u);
            }
            r
        }
        NodeKind::Ident(s) => s.clone(),
        NodeKind::Tuple(xs) => format!(
            "({})",
            xs.iter().map(|x| render(p, *x)).collect::<Vec<_>>().join(", ")
        ),
        NodeKind::Call { name, args } => format!(
            "{}({})",
            name,
            args.iter().map(|x| render(p, *x)).collect::<Vec<_>>().join(", ")
        ),
        NodeKind::Range { lo, hi, excl } => format!(
            "[{}..{}{}]",
            render(p, *lo),
            if *excl { "<" } else { "" },
            render(p, *hi)
        ),
        NodeKind::Postfix { base, ops } => format!("{}.{}", render(p, *base), ops.join(".")),
        NodeKind::Emit(n) => format!("emit {}", n),
        NodeKind::Ctor { name, args } => format!("{}<{}>", name, args),
        NodeKind::Binary { op, lhs, rhs } => {
            format!("({} {} {})", render(p, *lhs), op, render(p, *rhs))
        }
        _ => "<expr>".to_string(),
    }
}

/// `bound=[..]` / `unit=unit(..)` / `cap=..` annotations for a declared type.
fn ty_annot(ty: &str) -> String {
    let mut out = String::new();
    if let Some(r) = range_of(ty) {
        let _ = write!(out, " bound=[{}]", r);
    } else {
        out.push_str(" bound=declared");
    }
    if let Some(u) = tag_arg(ty, "unit") {
        let _ = write!(out, " unit=unit({})", u);
    } else {
        out.push_str(" unit=none");
    }
    if let Some(c) = tag_arg(ty, "cap") {
        let _ = write!(out, " cap={}", c);
    }
    out
}

fn range_of(ty: &str) -> Option<&str> {
    let i = ty.find('[')?;
    let j = ty[i + 1..].find(']')?;
    Some(&ty[i + 1..i + 1 + j])
}

fn tag_arg<'a>(ty: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("{}(", key);
    let i = ty.find(&pat)? + pat.len();
    let j = ty[i..].find(')')?;
    Some(&ty[i..i + j])
}
