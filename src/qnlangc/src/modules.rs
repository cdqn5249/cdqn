//! Multi-file `Module` / `Library` linking for `Q(use)` content-addressed
//! imports. Strictly stdlib; no external crates. A `Q(use)` names a pure
//! `Library` `.qnl` file with an exact `@ sha-256:<hex64>` pin.

use crate::ast::*;
use crate::diag::{Diag, Span};
use crate::parser;
use std::path::{Path, PathBuf};

fn diag(code: &str, short: &str, expected: &str, actual: &str, remedy: &str) -> Diag {
    Diag::new(code, Span::default(), short, expected, actual, remedy)
}

fn diag_span(code: &str, span: Span, short: &str, expected: &str, actual: &str, remedy: &str) -> Diag {
    Diag::new(code, span, short, expected, actual, remedy)
}

fn full_sha256_hex(data: &[u8]) -> String {
    crate::sha256::hex(&crate::sha256::sha256(data))
}

/// Load a root module and recursively resolve `Q(use)` imports.
pub fn load_program(root: &Path) -> Result<Program, Diag> {
    let mut stack = Vec::new();
    load(root, &mut stack)
}

fn load(path: &Path, stack: &mut Vec<PathBuf>) -> Result<Program, Diag> {
    let raw = std::fs::read(path).map_err(|_| {
        diag("E_DIGEST_MISMATCH", "cannot read module", "readable .qnl file", &path.display().to_string(), "check the Q(use) path")
    })?;
    let text = String::from_utf8(raw.clone()).map_err(|_| {
        diag("E_DIGEST_MISMATCH", "module is not UTF-8", "UTF-8 source", &path.display().to_string(), "save the module as UTF-8")
    })?;
    let mut prog = parser::parse_source(&text)?;
    let canon = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if stack.iter().any(|p| *p == canon) {
        return Err(diag("E_CYCLIC_IMPORT", "cyclic import detected", "acyclic module DAG", &path.display().to_string(), "remove the import cycle"));
    }
    stack.push(canon);

    let uses: Vec<(String, String, String, Span)> = prog
        .headers
        .iter()
        .filter_map(|h| match &h.kind {
            HeaderKind::Use { alias, path, digest } => Some((alias.clone(), path.clone(), digest.clone(), h.span)),
            _ => None,
        })
        .collect();
    let root_domains = domains_of(&prog);
    let mut imported: Vec<TopDecl> = Vec::new();
    for (alias, relpath, digest, span) in uses {
        let import_path = path.parent().unwrap_or_else(|| Path::new(".")).join(&relpath);
        // Static acyclicity over imports is checked before any digest check.
        let imp_canon = import_path.canonicalize().unwrap_or_else(|_| import_path.clone());
        if stack.iter().any(|p| *p == imp_canon) {
            return Err(diag_span(
                "E_CYCLIC_IMPORT", span, "cyclic import detected",
                "acyclic module DAG", &import_path.display().to_string(), "remove the import cycle",
            ));
        }
        let imp_raw = std::fs::read(&import_path).map_err(|_| {
            diag_span("E_DIGEST_MISMATCH", span, "cannot read imported module", "readable imported .qnl", &import_path.display().to_string(), "check the Q(use) path")
        })?;
        let actual = full_sha256_hex(&imp_raw);
        if actual.to_lowercase() != digest.to_lowercase() {
            return Err(diag_span(
                "E_DIGEST_MISMATCH", span, "imported module digest mismatch",
                &format!("sha-256:{}", digest.to_lowercase()),
                &format!("sha-256:{}", actual),
                "pin the exact imported file digest; no silent substitution is allowed",
            ));
        }
        let lib = load(&import_path, stack)?;
        if !is_pure_library(&lib) {
            return Err(diag_span(
                "E_IMPURE_LIBRARY_IMPORT", span, "imported module is not a pure Library",
                "no top-level statements / side effects; zero top-level emit",
                &import_path.display().to_string(),
                "import only pure Libraries; move top-level statements out",
            ));
        }
        if !domain_ok(&domains_of(&lib), &root_domains) {
            return Err(diag_span(
                "E_MODAL_OR_DOMAIN_LEAK", span, "imported module domain not a sub-domain",
                &format!("subset of {:?}", root_domains),
                &format!("{:?}", domains_of(&lib)),
                "match the importer's Q(domain) or import a compatible module",
            ));
        }
        // Run the Checker on every imported module before linking.
        let lib_src = String::from_utf8(imp_raw.clone()).unwrap_or_default();
        let gate_diags = crate::gates::check(&lib, &lib_src);
        if !gate_diags.is_empty() {
            return Err(gate_diags.into_iter().next().unwrap());
        }
        for t in &lib.tops {
            imported.push(rename_top(t, &alias));
        }
    }
    stack.pop();
    prog.tops.extend(imported);
    Ok(prog)
}

fn domains_of(p: &Program) -> Vec<String> {
    let mut out = Vec::new();
    for h in &p.headers {
        if let HeaderKind::Domain(v) = &h.kind {
            out.extend(v.clone());
        }
    }
    out
}

fn domain_ok(imported: &[String], root: &[String]) -> bool {
    if root.is_empty() || imported.is_empty() {
        return true;
    }
    imported.iter().all(|d| root.iter().any(|r| r == d))
}

fn is_pure_library(p: &Program) -> bool {
    !p.tops.iter().any(|t| match &t.kind {
        TopKind::Conjecture { name, .. } if name.starts_with("$stmt") => true,
        _ => false,
    })
}

fn rename_top(t: &TopDecl, alias: &str) -> TopDecl {
    let mut t = t.clone();
    let qualify = |n: &str| format!("{}::{}", alias, n);
    match &mut t.kind {
        TopKind::Pattern { name, .. } => *name = qualify(name),
        TopKind::Workflow { name, .. } => *name = qualify(name),
        TopKind::Falsify { name, .. } => *name = qualify(name),
        _ => {}
    }
    t
}
