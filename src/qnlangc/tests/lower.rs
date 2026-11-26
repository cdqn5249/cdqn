//! Stage 0B tests: the closed §7.1 lowering table makes all five §6.4
//! programs emit deterministic qir-enc-4 CBOR; budgets are enforced.

use qnlangc::{gates, lower, parser, sha256};

const MINIMAL: &str = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : Qm[-255..255];\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\nQ(1) -> x : T;\nQ(2) -> y : T;\nx + y -> w : T;\n";

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/fixtures/{}.qnl", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}

#[test]
fn emits_deterministic_cbor_on_declared_subset() {
    let p = parser::parse_source(MINIMAL).unwrap();
    let r = lower::lower(&p, MINIMAL);
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    let bytes = r.cbor.as_ref().unwrap();
    assert_eq!(&bytes[0..1], &[0xaa]); // definite-length map, keys 0..=9
    assert_eq!(bytes.len(), r.bytes);
    assert_eq!(r.sha256, sha256::hex16(&sha256::sha256(bytes)));
    let again = lower::lower(&parser::parse_source(MINIMAL).unwrap(), MINIMAL);
    assert_eq!(again.cbor.as_ref().unwrap(), bytes);
    assert_eq!(r.b_hat, Some(40));
}

#[test]
fn all_five_worked_programs_emit_cbor() {
    for name in ["qm", "qphy", "qchem", "qbio", "qs_qlog"] {
        let src = fixture(name);
        let p = parser::parse_source(&src).unwrap();
        assert!(gates::check(&p, &src).is_empty(), "{} gate diagnostics", name);
        let r = lower::lower(&p, &src);
        assert!(r.ok, "{} lowering diagnostics: {:?}", name, r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
        let bytes = r.cbor.as_ref().unwrap();
        assert_eq!(bytes.len(), r.bytes);
        assert_eq!(r.sha256.len(), 16);
        assert!(r.opcodes > 0 && r.blocks == 1);
        if name == "qs_qlog" {
            assert_eq!(r.budgets, vec![("step".to_string(), 64), ("route".to_string(), 80)]);
        }
        println!(
            "{} PASS bytes={} sha256={} words={} opcodes={} blocks={} callees={} b_hat={:?} budgets={:?}",
            name, r.bytes, r.sha256, r.words, r.opcodes, r.blocks, r.callees, r.b_hat, r.budgets
        );
    }
}

#[test]
fn declared_budget_exceedance_is_loud() {
    let src = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qlog;\nQ(mode) L3;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(pattern) tiny(k : T) -> T ![Qlog, budget(10 U)] {\n  Q(store)(Q(0), (k, Q(1))) -> v : T;\n  -> v\n}\n";
    let p = parser::parse_source(src).unwrap();
    let r = lower::lower(&p, src);
    assert!(r.diagnostics.iter().any(|d| d.code == "E_SMAX_OR_BUDGET"));
    assert!(!r.ok && r.cbor.is_none());
    println!("{}", r.diagnostics.iter().find(|d| d.code == "E_SMAX_OR_BUDGET").unwrap().channel_b("budget.qnl"));
}
