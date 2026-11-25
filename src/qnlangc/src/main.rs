//! Stage 0 `qnlangc` — strict §4 parser, CAE-AST, 14 gates, R12' diagnostics
//! and deterministic three-step qir-enc-4 lowering.
//!
//! The Stage 0 pipeline is the explicit three-part interlock
//! `Checker` -> `Exposer` -> `Compiler`:
//!   * `Checker`  — parse + G1–G14 + `E_NAMESPACE_COLLISION`;
//!   * `Exposer`  — `qir-verbalizer-1` full-program read-back (Obj 15);
//!   * `Compiler` — R11' lowering, committed only after certification.
//!
//! Usage: qnlangc --check <file> [--name-map] [--gate-list]
//!        qnlangc --expose <file>
//!        qnlangc --emit-qir <file> [-o <out.cbor>] [--certify-readback <sha256>]

use qnlangc::diag::{self, json_str, Diag, Span};
use qnlangc::{exposer, gates, lower, parser};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 && args[0] == "--gate-list" {
        for (n, g) in gates::gate_list() {
            println!("G{}  {}", n, g);
        }
        return ExitCode::SUCCESS;
    }
    if let Some(i) = args.iter().position(|a| a == "--expose") {
        let file = match args.get(i + 1) {
            Some(f) => f.clone(),
            None => {
                eprintln!("usage: qnlangc --expose <file>");
                return ExitCode::from(2);
            }
        };
        return expose_program(&file);
    }
    if let Some(i) = args.iter().position(|a| a == "--emit-qir") {
        let file = match args.get(i + 1) {
            Some(f) => f.clone(),
            None => {
                eprintln!("usage: qnlangc --emit-qir <file> [-o <out.cbor>] [--certify-readback <sha256>]");
                return ExitCode::from(2);
            }
        };
        let out = args
            .iter()
            .position(|a| a == "-o")
            .and_then(|j| args.get(j + 1))
            .cloned();
        let certify = match args.iter().position(|a| a == "--certify-readback") {
            Some(j) => match args.get(j + 1) {
                Some(h) => Some(h.clone()),
                None => {
                    eprintln!("--certify-readback requires a sha-256 digest");
                    return ExitCode::from(2);
                }
            },
            None => None,
        };
        return emit_qir(&file, out.as_deref(), certify.as_deref());
    }
    let file = match args.iter().position(|a| a == "--check").and_then(|i| args.get(i + 1)) {
        Some(f) => f.clone(),
        None => {
            eprintln!("usage: qnlangc --check <file> [--name-map] [--gate-list]");
            return ExitCode::from(2);
        }
    };
    let src = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", file, e);
            return ExitCode::from(2);
        }
    };
    let prog = match parser::parse_source(&src) {
        Ok(p) => p,
        Err(e) => {
            emit(&file, &[e], false, args.iter().any(|a| a == "--name-map"), None);
            return ExitCode::from(1);
        }
    };
    let diags = gates::check(&prog, &src);
    let name_map = if args.iter().any(|a| a == "--name-map") {
        Some(&prog.name_map)
    } else {
        None
    };
    let ok = diags.is_empty();
    emit(&file, &diags, ok, args.iter().any(|a| a == "--name-map"), name_map);
    if ok { ExitCode::SUCCESS } else { ExitCode::from(1) }
}

fn emit(file: &str, diags: &[diag::Diag], ok: bool, with_map: bool, map: Option<&std::collections::HashMap<String, Vec<String>>>) {
    let mut json = format!(
        "{{\"file\":{},\"ok\":{},\"diagnostics\":[",
        json_str(file),
        if ok { "true" } else { "false" }
    );
    json.push_str(
        &diags
            .iter()
            .map(|d| d.as_json(file))
            .collect::<Vec<_>>()
            .join(","),
    );
    json.push_str("]");
    if with_map {
        json.push_str(",\"name_map\":{");
        if let Some(m) = map {
            let mut pairs: Vec<_> = m.iter().collect();
            pairs.sort_by_key(|(k, _)| (*k).clone());
            json.push_str(
                &pairs
                    .iter()
                    .map(|(k, v)| format!("{}:{}", json_str(k), json_str(&v.join("|"))))
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        json.push('}');
    }
    json.push('}');
    println!("{}", json);
    for d in diags {
        println!("CHANNEL_A {}", d.channel_a());
        println!("CHANNEL_B {}", d.channel_b(file));
    }
    if ok {
        println!("PASS: {} — admissible under G1–G14 and E_NAMESPACE_COLLISION", file);
    }
}

fn expose_program(file: &str) -> ExitCode {
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", file, e);
            return ExitCode::from(2);
        }
    };
    let prog = match parser::parse_source(&src) {
        Ok(p) => p,
        Err(e) => {
            println!("CHANNEL_A {}", e.channel_a());
            println!("CHANNEL_B {}", e.channel_b(file));
            return ExitCode::from(1);
        }
    };
    // Checker: the Exposer only runs on a gate-clean AST.
    let gate_diags = gates::check(&prog, &src);
    if !gate_diags.is_empty() {
        for d in &gate_diags {
            println!("CHANNEL_A {}", d.channel_a());
            println!("CHANNEL_B {}", d.channel_b(file));
        }
        return ExitCode::from(1);
    }
    // Exposer: Compiler analysis supplies the declared-budget evidence only.
    let analysis = lower::analyze(&prog, &src);
    let exp = exposer::expose(&prog, &analysis, file);
    print!("{}", exp.body);
    println!("readback_lines: {}", exp.lines);
    println!("readback_bytes: {}", exp.bytes);
    println!("readback_sha256: {}", exp.sha256_tagged);
    println!("EPISTEMIC MODAL_HYPOTHESIS (uncertified; co-signature pending)");
    if analysis.diagnostics.is_empty() {
        ExitCode::SUCCESS
    } else {
        for d in &analysis.diagnostics {
            println!("CHANNEL_A {}", d.channel_a());
            println!("CHANNEL_B {}", d.channel_b(file));
        }
        ExitCode::from(1)
    }
}

fn emit_qir(file: &str, out: Option<&str>, certify: Option<&str>) -> ExitCode {
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {}: {}", file, e);
            return ExitCode::from(2);
        }
    };
    // ---- 1. Checker ----------------------------------------------------
    let prog = match parser::parse_source(&src) {
        Ok(p) => p,
        Err(e) => {
            println!("CHANNEL_A {}", e.channel_a());
            println!("CHANNEL_B {}", e.channel_b(file));
            return ExitCode::from(1);
        }
    };
    let gate_diags = gates::check(&prog, &src);
    if !gate_diags.is_empty() {
        for d in &gate_diags {
            println!("CHANNEL_A {}", d.channel_a());
            println!("CHANNEL_B {}", d.channel_b(file));
        }
        return ExitCode::from(1);
    }
    // ---- 2. Exposer (before the Compiler commits any byte) -------------
    let analysis = lower::analyze(&prog, &src);
    let exp = exposer::expose(&prog, &analysis, file);
    println!(
        "EXPOSE lines={} bytes={} sha256={}",
        exp.lines, exp.bytes, exp.sha256_tagged
    );
    // ---- 3. Obj 15 read-back certification interlock -------------------
    let mut certified = false;
    if let Some(expected) = certify {
        if !exp.matches(expected) {
            let d = Diag::new(
                "E_CERTIFICATION_MISMATCH",
                Span::default(),
                "read-back certification does not match the Exposer digest",
                &exp.sha256_tagged,
                expected,
                "re-run --expose and certify the exact readback_sha256; no CBOR is written",
            );
            println!("CHANNEL_A {}", d.channel_a());
            println!("CHANNEL_B {}", d.channel_b(file));
            println!("EPISTEMIC MODAL_HYPOTHESIS (uncertified; emission refused)");
            return ExitCode::from(1);
        }
        certified = true;
    }
    // ---- 4. Compiler commit --------------------------------------------
    let r = lower::lower(&prog, &src);
    debug_assert_eq!(r.b_hat, analysis.b_hat, "analysis/commit B_hat drift");
    println!(
        "LOWER {} ok={} bytes={} sha256={} words={} opcodes={} blocks={} callees={} ext={} b_hat={:?} budgets={:?}",
        file, r.ok, r.bytes, r.sha256, r.words, r.opcodes, r.blocks, r.callees, r.ext_records,
        r.b_hat, r.budgets
    );
    for d in &r.diagnostics {
        println!("CHANNEL_A {}", d.channel_a());
        println!("CHANNEL_B {}", d.channel_b(file));
    }
    if !r.ok {
        return ExitCode::from(1);
    }
    println!(
        "EPISTEMIC {}",
        if certified {
            "READBACK_CERTIFIED"
        } else {
            "MODAL_HYPOTHESIS (uncertified dev emit)"
        }
    );
    if let Some(path) = out {
        if let Some(bytes) = &r.cbor {
            if let Err(e) = std::fs::write(path, bytes) {
                eprintln!("cannot write {}: {}", path, e);
                return ExitCode::from(2);
            }
            println!("WROTE {} ({} bytes)", path, bytes.len());
        }
    }
    ExitCode::SUCCESS
}
