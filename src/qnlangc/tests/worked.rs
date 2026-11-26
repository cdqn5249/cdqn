//! Real tests against the reconciled §4 EBNF: one strict parser, full gates.

use qnlangc::{diag::Diag, gates, parser};

fn fx(name: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/fixtures/{}.qnl", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}

/// Runs the exact CLI pipeline: strict parse -> full G1..G14 + standing
/// invariant + linearity + type checks. Prints both diagnostic channels.
fn check_strict(label: &str, src: &str) -> Vec<Diag> {
    println!("===== {} =====", label);
    let p = match parser::parse_source(src) {
        Ok(p) => p,
        Err(e) => {
            println!("CHANNEL_A {}", e.channel_a());
            println!("CHANNEL_B {}", e.channel_b(&format!("fixtures/{}.qnl", label)));
            return vec![e];
        }
    };
    let ds = gates::check(&p, src);
    if ds.is_empty() {
        println!("PASS: fixtures/{}.qnl — admissible under G1–G14 and E_NAMESPACE_COLLISION", label);
    }
    for d in &ds {
        println!("CHANNEL_A {}", d.channel_a());
        println!("CHANNEL_B {}", d.channel_b(&format!("fixtures/{}.qnl", label)));
    }
    ds
}

#[test]
fn worked_qm() {
    let ds = check_strict("qm", &fx("qm"));
    assert!(ds.is_empty(), "Qm diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn worked_qphy() {
    let ds = check_strict("qphy", &fx("qphy"));
    assert!(ds.is_empty(), "Qphy diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn worked_qchem() {
    let ds = check_strict("qchem", &fx("qchem"));
    assert!(ds.is_empty(), "Qchem diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn worked_qbio() {
    let ds = check_strict("qbio", &fx("qbio"));
    assert!(ds.is_empty(), "Qbio diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn worked_qs_qlog() {
    let ds = check_strict("qs_qlog", &fx("qs_qlog"));
    assert!(ds.is_empty(), "Qs+Qlog diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn q_falsify_is_now_a_top_decl() {
    let src = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qlog;\nQ(mode) L3;\nQ(falsify) bad(k : u64) ![Qlog, may_emit(receipt)] { Q(halt_with)(receipt); }\n";
    let ds = check_strict("falsify", src);
    assert!(ds.is_empty(), "Q(falsify) diagnostics: {:?}", ds.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn gate_list_is_exact() {
    let list = gates::gate_list();
    assert_eq!(list.len(), 14);
    println!("--- G1..G14 ---");
    for (n, g) in list {
        println!("G{} {}", n, g);
    }
}

// ---- negative tests, all through the same strict parser ----

const HEAD: &str = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : Qm[-255..255];\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\n";

#[test]
fn neg_namespace_collision() {
    let src = format!("{}Q(1) -> x : T;\nQ(2) -> x : T;\n", HEAD);
    let ds = check_strict("namespace_collision", &src);
    assert!(ds.iter().any(|d| d.code == "E_NAMESPACE_COLLISION"));
}

#[test]
fn neg_naked_numeric() {
    let src = format!("{}Q(1) -> x : T;\n42 -> y : T;\n", HEAD);
    let ds = check_strict("naked_numeric", &src);
    assert!(ds.iter().any(|d| d.code == "E_NO_NAKED_NUM"));
}

#[test]
fn neg_equals_outside_binary() {
    let src = format!("{}Q(type) T = Qm[-255..255];\n", HEAD);
    let ds = check_strict("equals_outside_binary", &src);
    assert!(ds.iter().any(|d| d.code == "E_OP_HYGIENE"));
}

#[test]
fn neg_qs_identifier_in_qchem() {
    let src = "Q(dcc) d;\nQ(kappa) fact;\nQ(domain) (Qchem, Qm);\nQ(mode) L1;\nQ(type) T : Qm[-255..255];\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\nQs -> agent : T;\n";
    let ds = check_strict("qs_in_qchem", src);
    assert!(ds.iter().any(|d| d.code == "E_MODAL_OR_DOMAIN_LEAK"));
}

#[test]
fn neg_affine_double_consume() {
    let src = format!(
        "{}Q(0) -> v : T;\nQ(arr_new)<T, cap(2)> -> arr : Q(array)<T, cap(2)>;\nQ(store)(arr, v) -> a1 : Q(array)<T, cap(2)>;\nQ(store)(arr, v) -> a2 : Q(array)<T, cap(2)>;\nQ(drop)(a1);\nQ(drop)(a2);\n",
        HEAD
    );
    let ds = check_strict("affine_double_consume", &src);
    assert!(ds.iter().any(|d| d.code == "E_AFFINE_DOUBLE_CONSUME"));
}

#[test]
fn neg_affine_missing_drop() {
    let src = format!("{}Q(arr_new)<T, cap(2)> -> arr : Q(array)<T, cap(2)>;\n", HEAD);
    let ds = check_strict("affine_missing_drop", &src);
    assert!(ds.iter().any(|d| d.code == "E_AFFINE_MISSING_DROP"));
}

#[test]
fn neg_bound_overflow() {
    let src = format!("{}Q(300) -> x : Qm[-255..255];\n", HEAD);
    let ds = check_strict("bound_overflow", &src);
    assert!(ds.iter().any(|d| d.code == "E_BOUND_OVERFLOW"));
}

#[test]
fn neg_undeclared_bound() {
    let src = format!("{}Q(1) -> x : Nope;\n", HEAD);
    let ds = check_strict("undeclared_bound", &src);
    assert!(ds.iter().any(|d| d.code == "E_BOUND_OVERFLOW"));
}
