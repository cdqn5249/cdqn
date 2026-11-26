//! Stage 0 / bootstrap-profile (qir-enc-5) tests: bitwise, governed hash,
//! bounded loops, variant/match dispatch, and explicit target separation.

use qnlangc::{gates, lower, parser};

fn fx(name: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/fixtures/{}.ql", env!("CARGO_MANIFEST_DIR"), name)).unwrap()
}

#[test]
fn boot_ops_lowers_to_qir_enc5() {
    let src = fx("boot_ops");
    let p = parser::parse_source(&src).unwrap();
    assert!(gates::check(&p, &src).is_empty());
    let r = lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    // gateway 20 + 5 bitwise*4 + Q_HASH 290 + emit/halt/const charges.
    assert_eq!(r.b_hat, Some(358));
    assert_eq!(r.cbor.as_ref().unwrap().len(), 860);
    let bytes = r.cbor.unwrap();
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("qir-enc-5"));
    assert!(text.contains("qir-prof-ext-bootstrap-1"));
}

#[test]
fn boot_loop_lowers_variant_match_and_trip() {
    let src = fx("boot_loop");
    let p = parser::parse_source(&src).unwrap();
    assert!(gates::check(&p, &src).is_empty());
    let r = lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    assert_eq!(r.b_hat, Some(222));
    assert!(r.opcodes > 40);
    let bytes = r.cbor.unwrap();
    let text = String::from_utf8_lossy(&bytes);
    assert!(text.contains("qir-enc-5"));
}

#[test]
fn bootstrap_ops_are_rejected_under_qir_enc4() {
    let src = fx("boot_ops");
    let p = parser::parse_source(&src).unwrap();
    let r = lower::lower_with_target(&p, &src, "qir-enc-4");
    assert!(!r.ok);
    assert!(r.cbor.is_none());
    assert!(r.diagnostics.iter().any(|d| d.code == "E_TARGET_MISMATCH"));
    let bytes = r.cbor;
    assert!(bytes.is_none());
}

#[test]
fn qir_enc4_existing_program_stays_byte_stable() {
    let src = fx("minimal");
    let p = parser::parse_source(&src).unwrap();
    let old = lower::lower_with_target(&p, &src, "qir-enc-4");
    let default = lower::lower(&p, &src);
    assert!(old.ok && default.ok);
    assert_eq!(old.cbor, default.cbor);
    assert_eq!(old.sha256, default.sha256);
    let fresh = lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(fresh.ok);
    assert_eq!(fresh.b_hat, old.b_hat);
}

#[test]
fn bootstrap_gate_rejections_are_exact() {
    let head = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(1) -> a : u64;\n";
    let bad_shift = format!("{}Q(bshl)(a, Q(64)) -> s : u64;\n", head);
    let p = parser::parse_source(&bad_shift).unwrap();
    let ds = gates::check(&p, &bad_shift);
    assert!(ds.iter().any(|d| d.code == "E_SMAX_OR_BUDGET"));

    let bad_bnd = format!("{}Q(hash)(a, Q(65537)) -> h : Q(tuple)<u64,u64,u64,u64>;\n", head);
    let p = parser::parse_source(&bad_bnd).unwrap();
    let ds = gates::check(&p, &bad_bnd);
    assert!(ds.iter().any(|d| d.code == "E_BOUND_OVERFLOW"));
}

#[test]
fn multi_loop_scc_admission_widens_global_product() {
    // Twelve independent trip(1) loops: the global Cartesian product is
    // 2^12 = 4096 per block (above S_max once block/loop witnesses are
    // counted), while the SCC-additive sum is 12*2 = 24.
    const K: usize = 12;
    let mut src = String::from(
        "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\n",
    );
    for i in 0..K {
        src.push_str(&format!(
            "Q(loop) @[trip(1)] {{ Q(1) -> x{} : u64; -> x{} }} -> y{} : u64;\n",
            i, i, i
        ));
    }
    src.push_str("emit receipt;\n");
    let p = parser::parse_source(&src).unwrap();
    assert!(gates::check(&p, &src).is_empty());
    let r = lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    let global_product = 2u128.pow(K as u32);
    let scc_sum = K as u128 * 2;
    assert!(global_product >= 4096);
    assert!(scc_sum <= 4096);
    assert!(r.b_hat.unwrap() < 4096 * 4);
}
