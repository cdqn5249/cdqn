//! The standing namespace invariant plus all fourteen semantic gates from
//! docs/qnlang.md §6.2, operator hygiene (§3.1), built-in signatures (§5.2)
//! and affine container linearity (§2.3).

use crate::ast::*;
use crate::diag::{Diag, Span};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct Facts {
    /// (name, span, binder scope). De Bruijn binders are scope-local (§2.1).
    names: Vec<(String, Span, usize)>,
    current_scope: usize,
    next_scope: usize,
    types: HashMap<String, String>,
    containers: Vec<(String, Span, bool)>, // name, span, consumed
    qprim_bounds: Vec<(String, Option<String>, Option<String>, Span)>, // core, target ty, unit, span
    calls: Vec<(String, Vec<NodeId>, Span)>,
    loads: Vec<(bool, Span)>,
    folds: Vec<(String, bool, Span)>,
    loops: Vec<(u64, Span)>,
    zoom_args: Vec<(Vec<NodeId>, Span)>,
    indices: Vec<(String, Span)>,
    domains: Vec<String>,
    headers: HashSet<String>,
    anchors: usize,
    may_emit: usize,
    halt_withs: Vec<Span>,
    budgets: Vec<u64>,
    contracts: Vec<String>,
}

fn num(s: &str) -> Option<i64> {
    s.trim().parse::<i64>().ok()
}

fn qm_bounds(ty: &str) -> Option<(i64, i64)> {
    let t = ty.trim();
    if let Some(rest) = t.strip_prefix("Qm[") {
        if let Some(inner) = rest.strip_suffix(']') {
            let mut it = inner.split("..");
            let lo = num(it.next()?)?;
            let hi = num(it.next()?)?;
            return Some((lo, hi));
        }
    }
    None
}

fn width_of(ty: &str) -> Option<u32> {
    match ty.trim() {
        "u8" => Some(8),
        "u16" => Some(16),
        "u32" => Some(32),
        "u64" => Some(64),
        _ => None,
    }
}

fn unit_of(ty: &str) -> Option<String> {
    let marker = "unit(";
    let i = ty.find(marker)?;
    let rest = &ty[i + marker.len()..];
    let j = rest.find(')')?;
    Some(rest[..j].to_string())
}

fn push_name(f: &mut Facts, n: &str, sp: Span) {
    f.names.push((n.to_string(), sp, f.current_scope));
}

fn new_scope(f: &mut Facts) -> usize {
    let s = f.next_scope;
    f.next_scope += 1;
    s
}

fn walk(f: &mut Facts, p: &Program, id: NodeId) {
    let node = p.node(id);
    let sp = node.span;
    match &node.kind {
        NodeKind::QPrim { core, neg, unit } => {
            f.qprim_bounds.push((
                if *neg { format!("-{}", core) } else { core.clone() },
                None,
                unit.clone(),
                sp,
            ));
        }
        NodeKind::Ident(s) => {
            if matches!(s.as_str(), "wallclock" | "now" | "clock" | "elapsed" | "time") {
                f.calls.push((format!("$wallclock:{}", s), Vec::new(), sp));
            }
            if f.domains.iter().any(|d| d == "Qchem")
                && (s == "Qs" || s == "Qlog" || s.starts_with("Qs") || s == "agent" || s == "Q(agent)")
            {
                f.calls.push(("$domain_leak".into(), Vec::new(), sp));
            }
        }
        NodeKind::Tuple(v) | NodeKind::Block { stmts: v, tail: None } => {
            for x in v {
                walk(f, p, *x);
            }
        }
        NodeKind::Block { stmts, tail } => {
            for &x in stmts {
                walk(f, p, x);
            }
            if let Some(x) = tail {
                walk(f, p, *x);
            }
        }
        NodeKind::Call { name, args } => {
            f.calls.push((name.clone(), args.clone(), sp));
            if name == "zoom" {
                f.zoom_args.push((args.clone(), sp));
            }
            for &a in args {
                walk(f, p, a);
            }
        }
        NodeKind::Ctor { .. } => {}
        NodeKind::Split { scrut, t, f: fb } => {
            walk(f, p, *scrut);
            walk(f, p, *t);
            walk(f, p, *fb);
        }
        NodeKind::Match { scrut, arms } => {
            walk(f, p, *scrut);
            for (_, e) in arms {
                walk(f, p, *e);
            }
        }
        NodeKind::Fold { seq, init, body, binders, .. } => {
            let seq_name = match &p.node(*seq).kind {
                NodeKind::Ident(s) => s.clone(),
                _ => String::new(),
            };
            let cap = p
                .arena
                .iter()
                .filter_map(|n| match &n.kind {
                    NodeKind::Bind { target, ty: Some(ty), .. } if *target == seq_name => Some(ty.contains("cap(")),
                    _ => None,
                })
                .next()
                .unwrap_or(false);
            f.folds.push((seq_name, cap, sp));
            for b in binders {
                push_name(f, b, sp);
            }
            walk(f, p, *seq);
            walk(f, p, *init);
            walk(f, p, *body);
        }
        NodeKind::Loop { body, .. } => walk(f, p, *body),
        NodeKind::Load { tbl, key, target, or_else, .. } => {
            f.loads.push((or_else.is_some(), sp));
            push_name(f, target, sp);
            walk(f, p, *tbl);
            walk(f, p, *key);
            if let Some(b) = or_else {
                walk(f, p, *b);
            }
        }
        NodeKind::Binary { lhs, rhs, .. } => {
            walk(f, p, *lhs);
            walk(f, p, *rhs);
        }
        NodeKind::Range { lo, hi, .. } => {
            walk(f, p, *lo);
            walk(f, p, *hi);
        }
        NodeKind::Bind { target, ty, value } => {
            push_name(f, target, sp);
            if let Some(t) = ty {
                f.types.insert(target.clone(), t.clone());
                if t.contains("Q(array)") || t.contains("Q(table)") {
                    f.containers.push((target.clone(), sp, false));
                }
                if let Some(NodeKind::QPrim { core, neg, unit }) = Option::Some(&p.node(*value).kind) {
                    let c = if *neg { format!("-{}", core) } else { core.clone() };
                    f.qprim_bounds.push((c, Some(t.clone()), unit.clone(), sp));
                }
            }
            walk(f, p, *value);
        }
        NodeKind::Postfix { base, ops } => {
            let nm = match &p.node(*base).kind {
                NodeKind::Ident(s) => s.clone(),
                _ => String::new(),
            };
            for o in ops {
                if o.starts_with('.') {
                    f.indices.push((nm.clone(), sp));
                }
            }
            walk(f, p, *base);
        }
        NodeKind::Emit(_) => {}
    }
}

fn contract_items(s: &str) -> Vec<String> {
    s.split(',').map(|x| x.trim().to_string()).collect()
}

fn collect(p: &Program) -> Facts {
    let mut f = Facts::default();
    for h in &p.headers {
        match &h.kind {
            HeaderKind::Dcc(n) => {
                f.headers.insert("dcc".into());
                push_name(&mut f, n, h.span);
            }
            HeaderKind::Kappa(n) => {
                f.headers.insert("kappa".into());
                push_name(&mut f, n, h.span);
            }
            HeaderKind::Domain(v) => {
                f.headers.insert("domain".into());
                f.domains.extend(v.clone());
            }
            HeaderKind::Mode(n) => {
                f.headers.insert("mode".into());
                push_name(&mut f, n, h.span);
            }
        }
    }
    for t in &p.tops {
        let is_top_stmt = matches!(&t.kind, TopKind::Conjecture { name, .. } if name.starts_with("$stmt"));
        // The declaration's own name is minted in program scope; its binders
        // and body live in their own De Bruijn scope (§2.1). Top-level
        // statements share program scope.
        f.current_scope = 0;
        match &t.kind {
            TopKind::Type { name, ty } => {
                push_name(&mut f, name, t.span);
                f.types.insert(name.clone(), ty.clone());
                if let Some(u) = unit_of(ty) {
                    f.types.insert(format!("@unit:{}", name), u);
                }
            }
            TopKind::Anchor { name, core, ty } => {
                f.anchors += 1;
                push_name(&mut f, name, t.span);
                f.types.insert(name.clone(), ty.clone());
                f.qprim_bounds.push((core.clone(), Some(ty.clone()), unit_of(ty), t.span));
            }
            TopKind::Metric { name, .. } => push_name(&mut f, name, t.span),
            TopKind::Blanket { name, params } => {
                push_name(&mut f, name, t.span);
                for (n, ty) in params {
                    push_name(&mut f, n, t.span);
                    f.types.insert(n.clone(), ty.clone());
                }
            }
            TopKind::Variant { name, arms } => {
                push_name(&mut f, name, t.span);
                for a in arms {
                    let n = a.split('(').next().unwrap_or(a).trim();
                    push_name(&mut f, n, t.span);
                }
            }
            TopKind::Pattern { name, params, ret, contract, body }
            | TopKind::Workflow { name, params, ret, contract, body } => {
                push_name(&mut f, name, t.span);
                f.current_scope = new_scope(&mut f);
                for (n, ty) in params {
                    push_name(&mut f, n, t.span);
                    f.types.insert(n.clone(), ty.clone());
                }
                if !ret.is_empty() {
                    f.types.insert(name.clone(), ret.clone());
                }
                scan_contract(&mut f, contract);
                walk(&mut f, p, *body);
            }
            TopKind::Conjecture { name, contract, body }
            | TopKind::Falsify { name, contract, body } => {
                push_name(&mut f, name, t.span);
                f.current_scope = if is_top_stmt { 0 } else { new_scope(&mut f) };
                scan_contract(&mut f, contract);
                walk(&mut f, p, *body);
            }
            TopKind::Experiment { name, for_name, contract, .. } => {
                push_name(&mut f, name, t.span);
                f.current_scope = new_scope(&mut f);
                push_name(&mut f, for_name, t.span);
                scan_contract(&mut f, contract);
            }
        }
    }
    f
}

fn scan_contract(f: &mut Facts, contract: &str) {
    f.contracts.push(contract.to_string());
    for item in contract_items(contract) {
        if item.starts_with("may_emit(") {
            f.may_emit += 1;
        }
        if let Some(rest) = item.strip_prefix("budget(") {
            let n = rest.trim_end_matches(')').split_whitespace().next().unwrap_or("0");
            if let Ok(v) = n.parse::<u64>() {
                f.budgets.push(v);
            }
        }
    }
}

const GATES: [&str; 14] = [
    "E_NO_NAKED_NUM",
    "E_BOUND_OVERFLOW",
    "E_POSSIBLE_DIV_ZERO",
    "E_UNPROMOTED_ROOT",
    "E_UNPROVED_INDEX/E_UNHANDLED_MISS",
    "E_UNBOUNDED_CONTROL",
    "E_UNCONSERVED_REMAINDER/E_UNALIGNED_ZOOM",
    "E_MODAL_OR_DOMAIN_LEAK",
    "E_UNIT_MISMATCH",
    "E_GROUNDING_VIOLATION",
    "E_LOCAL_GENESIS_HARDCODE",
    "E_IMPLICIT_WALLCLOCK",
    "E_UNDECLARED_TERMINAL_STATUS",
    "E_SMAX_OR_BUDGET",
];

pub fn gate_list() -> Vec<(usize, &'static str)> {
    GATES.iter().enumerate().map(|(i, g)| (i + 1, *g)).collect()
}

pub fn check(p: &Program, source: &str) -> Vec<Diag> {
    let f = collect(p);
    let mut d = Vec::new();

    // Standing invariant: SSA names are never rebound.
    let mut seen: HashMap<(usize, String), Span> = HashMap::new();
    for (n, sp, scope) in &f.names {
        if n.starts_with("$stmt") {
            continue;
        }
        let key = (*scope, n.clone());
        if let Some(first) = seen.get(&key) {
            d.push(Diag::new(
                "E_NAMESPACE_COLLISION",
                *sp,
                "SSA name minted twice in the same binder scope",
                &format!("fresh name (first mint at {})", first),
                n,
                "mint a fresh name; SSA names are never rebound within a scope",
            ));
        } else {
            seen.insert(key, *sp);
        }
    }

    // G1 is enforced at the parser's `primary`: a Num token there is a naked
    // literal. Valid Q(k) anchors carry a numeric core but are not G1 errors.

    // G2 — declared bounds and type declarations.
    let declared: HashSet<&str> = f.types.keys().map(|s| s.as_str()).collect();
    for (core, ty, _unit, sp) in &f.qprim_bounds {
        if let Some(t) = ty {
            if let Some((lo, hi)) = qm_bounds(t) {
                if let Some(v) = num(core) {
                    if !(lo..=hi).contains(&v) {
                        d.push(Diag::new(
                            "E_BOUND_OVERFLOW",
                            *sp,
                            "value outside declared qtype bound",
                            &format!("{} in [{}..{}]", core, lo, hi),
                            core,
                            "widen the declared bound explicitly or use an in-range anchor",
                        ));
                    }
                }
            } else if !declared.contains(t.as_str()) && !is_builtin_qtype(t) {
                d.push(Diag::new(
                    "E_BOUND_OVERFLOW",
                    *sp,
                    "undeclared bound/type",
                    "a Q(type) name or a §4 qtype",
                    t,
                    "declare the type with Q(type) before use",
                ));
            }
        }
    }

    // G3 — division without a nonzero proof.
    for (name, _, sp) in &f.calls {
        if name == "div_euc" || name == "mod_euc" || name == "div" {
            if !f.contracts.iter().any(|c| c.contains("nonzero")) {
                d.push(Diag::new(
                    "E_POSSIBLE_DIV_ZERO",
                    *sp,
                    "quotient without nonzero proof",
                    "an explicit nonzero proof contract",
                    name,
                    "supply a nonzero proof before the quotient",
                ));
            }
        }
    }

    // G4 — negative even root without PROMOTE.
    for (name, args, sp) in &f.calls {
        if name == "root" || name == "sqrt" {
            if !source.contains("PROMOTE") {
                d.push(Diag::new(
                    "E_UNPROMOTED_ROOT",
                    *sp,
                    "negative even root at exit",
                    "PROMOTE before exit",
                    name,
                    "promote the negative even root explicitly",
                ));
            }
            let _ = args;
        }
    }

    // G5 — unproved index / unhandled load miss.
    for (has_else, sp) in &f.loads {
        if !has_else {
            d.push(Diag::new("E_UNHANDLED_MISS", *sp, "Q(load) without or_else", "or_else block", "absent", "declare an or_else fallback"));
        }
    }
    for (nm, sp) in &f.indices {
        if f.types.get(nm).map(|t| t.contains("Q(array)") || t.contains("Q(table)")).unwrap_or(false)
            && !f.contracts.iter().any(|c| c.contains("proof"))
        {
            d.push(Diag::new("E_UNPROVED_INDEX", *sp, "indexed access without proof", "an explicit proof contract", nm, "prove the index before access"));
        }
    }

    // G6 — bounded control.
    for (seq, cap, sp) in &f.folds {
        if !cap {
            d.push(Diag::new("E_UNBOUNDED_CONTROL", *sp, "fold over container without cap(N)", "a Q(array)/Q(table) with cap(N)", seq, "declare cap(N) on the folded container"));
        }
    }
    for (trip, sp) in &f.loops {
        if *trip == 0 {
            d.push(Diag::new("E_UNBOUNDED_CONTROL", *sp, "zero trip ceiling", "trip(K) with K > 0", "0", "declare a positive trip ceiling"));
        }
    }

    // G7 — remainder/zoom conservation.
    for (args, sp) in &f.zoom_args {
        if args.len() == 2 {
            let z = &args[1];
            let ok = matches!(&p_node_kind(p, *z), NodeKind::Ident(s) if s == "z_min")
                || matches!(&p_node_kind(p, *z), NodeKind::QPrim { core, .. } if core == "0");
            if !ok {
                d.push(Diag::new("E_UNALIGNED_ZOOM", *sp, "zoom selector is not z_min", "z_min", format!("{:?}", p_node_kind(p, *z)), "zoom only at the declared minimal zoom"));
            }
        }
    }

    // G8 — modal/domain leakage.
    for (name, _, sp) in &f.calls {
        if name == "$domain_leak" {
            d.push(Diag::new("E_MODAL_OR_DOMAIN_LEAK", *sp, "Qs identifier inside abiotic Qchem example", "Qchem/Qm identifiers only", "Qs name", "remove the Qs identifier from the Qchem domain"));
        }
    }
    if let Some(i) = f.domains.iter().position(|x| x == "Qs" || x == "Qlog") {
        let _ = i;
        if f.may_emit == 0 && !source.contains("may_emit(") {
            d.push(Diag::new("E_GROUNDING_VIOLATION", Span { line: 1, col: 1 }, "Qs/Qlog domain without may_emit", "may_emit(...) authorization", "absent", "authorize emission explicitly"));
        }
    }

    // G9 — unit mismatch.
    for node in &p.arena {
        if let NodeKind::Binary { op, lhs, rhs } = &node.kind {
            if op == "+" || op == "-" {
                let ln = p_node_name(p, *lhs);
                let rn = p_node_name(p, *rhs);
                if let (Some(lu), Some(ru)) = (f.types.get(&format!("@unit:{}", ln)), f.types.get(&format!("@unit:{}", rn))) {
                    if lu != ru {
                        d.push(Diag::new("E_UNIT_MISMATCH", node.span, "mixed Qphy units", lu, ru, "align units before the operation"));
                    }
                }
            }
        }
    }

    // G10 — grounding declarations.
    for h in ["dcc", "kappa", "domain", "mode"] {
        if !f.headers.contains(h) {
            d.push(Diag::new("E_GROUNDING_VIOLATION", Span { line: 1, col: 1 }, "missing grounding header", h, "absent", "declare the header explicitly"));
        }
    }

    // G11 — local genesis.
    if f.anchors == 0 && source.contains("Q(0)") {
        d.push(Diag::new("E_LOCAL_GENESIS_HARDCODE", Span { line: 1, col: 1 }, "genesis used without Q(anchor)", "Q(anchor) declaration", "absent", "declare a local genesis anchor"));
    }

    // G12 — implicit wallclock.
    for (name, _, sp) in &f.calls {
        if name.starts_with("$wallclock:") {
            d.push(Diag::new("E_IMPLICIT_WALLCLOCK", *sp, "host time reference", "an explicit declared budget", name, "remove host time and declare work units"));
        }
    }

    // G13 — terminal status authorization.
    let authorized = f.may_emit > 0 || source.contains("may_emit(");
    for sp in &f.halt_withs {
        if !authorized {
            d.push(Diag::new("E_UNDECLARED_TERMINAL_STATUS", *sp, "Q(halt_with) without may_emit", "may_emit(...) authorization", "absent", "authorize the terminal receipt"));
        }
    }
    // halt_with calls were recorded as call name "halt_with".
    for (name, _, sp) in &f.calls {
        if name == "halt_with" && !authorized {
            d.push(Diag::new("E_UNDECLARED_TERMINAL_STATUS", *sp, "terminal exit without may_emit", "may_emit(...) authorization", "absent", "authorize the terminal receipt"));
        }
    }

    // G14 — structural/budget ceiling.
    for b in &f.budgets {
        if *b > 4096 {
            d.push(Diag::new("E_SMAX_OR_BUDGET", Span { line: 1, col: 1 }, "budget above S_max", "budget <= 4096 U", format!("{} U", b), "lower the declared budget"));
        }
    }
    for (trip, sp) in &f.loops {
        if *trip > 4096 {
            d.push(Diag::new("E_SMAX_OR_BUDGET", *sp, "trip ceiling above S_max", "trip(K) with K <= 4096", trip.to_string(), "lower the declared trip ceiling"));
        }
    }

    // §5.2 built-in signature arities.
    for (name, args, sp) in &f.calls {
        let expected = match name.as_str() {
            "metric_dot" | "store" | "2:cons" | "3:cmp" | "4:zoom_cmp" => Some(2),
            "reuse" | "drop" | "halt_with" | "cut" | "2:chirality" => Some(1),
            "band" | "bor" | "bxor" | "bshl" | "bshr" | "hash" => Some(2),
            _ => None,
        };
        if let Some(n) = expected {
            if args.len() != n {
                d.push(Diag::new("E_BUILTIN_ARITY", *sp, "built-in signature arity mismatch", format!("{} argument(s)", n), format!("{}", args.len()), "match the §5.2 signature"));
            }
        }
    }

    // §6.2 bootstrap-profile refinements (v0.1.0-draft.4): static byte bound
    // for Q(hash) and shift-width / declared-width compatibility.
    for (name, args, sp) in &f.calls {
        if name == "hash" && args.len() == 2 {
            let b = match &p.node(args[1]).kind {
                NodeKind::QPrim { core, neg: false, .. } => core.parse::<u64>().ok(),
                _ => None,
            };
            match b {
                Some(v) if (1..=65536).contains(&v) => {}
                other => d.push(Diag::new(
                    "E_BOUND_OVERFLOW", *sp, "invalid Q(hash) byte bound",
                    "1 <= b_bnd <= 65536", other.map(|v| v.to_string()).unwrap_or_else(|| "non-static".into()),
                    "declare a static INT_POS byte bound Q(k)",
                )),
            }
        }
        if matches!(name.as_str(), "bshl" | "bshr") && args.len() == 2 {
            let ty = match &p.node(args[0]).kind {
                NodeKind::Ident(s) => f.types.get(s).cloned(),
                _ => None,
            };
            if let (Some(w), NodeKind::QPrim { core, neg: false, .. }) =
                (ty.as_deref().and_then(width_of), &p.node(args[1]).kind)
            {
                if let Ok(s) = core.parse::<u64>() {
                    if s >= w as u64 {
                        d.push(Diag::new(
                            "E_SMAX_OR_BUDGET", *sp, "shift amount reaches declared width",
                            format!("s < w = {}", w), s.to_string(), "declare a shift below the operand width",
                        ));
                    }
                }
            }
        }
    }

    // §2.3 affine container linearity: Q(store)/Q(batch)/Q(drop) consume,
    // Q(fold)/Q(load) borrow only; every governed container is consumed once.
    let mut consumed: HashSet<String> = HashSet::new();
    for (name, args, sp) in &f.calls {
        if matches!(name.as_str(), "store" | "batch" | "drop") {
            let target = args.first().map(|a| p_node_name(p, *a)).unwrap_or_default();
            if target.is_empty() {
                continue;
            }
            if !consumed.insert(target.clone()) {
                d.push(Diag::new(
                    "E_AFFINE_DOUBLE_CONSUME",
                    *sp,
                    "governed container consumed twice",
                    "exactly one Q(store)/Q(batch)/Q(drop)",
                    format!("second consume of {}", target),
                    "borrow with Q(fold)/Q(load) instead, or mint a fresh container",
                ));
            }
        }
    }
    for (n, sp, _) in &f.containers {
        if !consumed.contains(n) {
            d.push(Diag::new(
                "E_AFFINE_MISSING_DROP",
                *sp,
                "governed container never consumed",
                "Q(drop) terminal transition",
                format!("{} outstanding", n),
                "consume the container exactly once with Q(drop)",
            ));
        }
    }

    // §3.1 operator hygiene beyond the parser: bare '/' and '==='.
    let mut in_unit = 0i32;
    for (i, line) in source.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '(' && line[..j].ends_with("unit") {
                in_unit += 1;
            }
            if ch == ')' && in_unit > 0 {
                in_unit -= 1;
            }
            if ch == '/' && in_unit == 0 {
                d.push(Diag::new("E_OP_HYGIENE", Span { line: i + 1, col: j + 1 }, "bare '/' is not value division", "a declared built-in", "/", "remove the bare slash"));
            }
        }
        if line.contains("===") || line.contains('·') {
            d.push(Diag::new("E_OP_HYGIENE", Span { line: i + 1, col: 1 }, "unadmitted operator token", "= or ~=", "=== | ·", "use the declared exact operator"));
        }
    }

    d
}

fn is_builtin_qtype(t: &str) -> bool {
    matches!(t, "u8" | "u16" | "u32" | "u64" | "bool")
        || t.starts_with("Qm[")
        || t.starts_with("Qd[")
        || t.starts_with("rat[")
        || t.starts_with("Qleaf<")
        || t.starts_with("Q(tuple)<")
        || t.starts_with("Q(array)<")
        || t.starts_with("Q(table)<")
        || t.starts_with("Q(blade)<")
        || t.starts_with("Qphy<")
        || t.starts_with("sim<")
}

fn p_node_kind<'a>(p: &'a Program, id: NodeId) -> &'a NodeKind {
    &p.arena[id].kind
}
fn p_node_name(p: &Program, id: NodeId) -> String {
    match &p.arena[id].kind {
        NodeKind::Ident(s) => s.clone(),
        _ => String::new(),
    }
}
