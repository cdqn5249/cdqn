//! Stage 0 `qnlangc` — strict §4 parser, CAE-AST, 14 gates, R12' diagnostics
//! and deterministic three-step qir-enc-4 lowering.
//! Usage: qnlangc --check <file> [--name-map] [--gate-list]
//!        qnlangc --emit-qir <file> [-o <out.cbor>]

use qnlangc::diag;
use qnlangc::diag::json_str;
use qnlangc::{gates, lower, parser};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 1 && args[0] == "--gate-list" {
        for (n, g) in gates::gate_list() {
            println!("G{}  {}", n, g);
        }
        return ExitCode::SUCCESS;
    }
    if let Some(i) = args.iter().position(|a| a == "--emit-qir") {
        let file = match args.get(i + 1) {
            Some(f) => f.clone(),
            None => {
                eprintln!("usage: qnlangc --emit-qir <file> [-o <out.cbor>]");
                return ExitCode::from(2);
            }
        };
        let out = args
            .iter()
            .position(|a| a == "-o")
            .and_then(|j| args.get(j + 1))
            .cloned();
        return emit_qir(&file, out.as_deref());
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

fn emit_qir(file: &str, out: Option<&str>) -> ExitCode {
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
    let gate_diags = gates::check(&prog, &src);
    if !gate_diags.is_empty() {
        for d in &gate_diags {
            println!("CHANNEL_A {}", d.channel_a());
            println!("CHANNEL_B {}", d.channel_b(file));
        }
        return ExitCode::from(1);
    }
    let r = lower::lower(&prog, &src);
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
