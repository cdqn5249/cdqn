//! Stage 0 `Exposer` tests — deterministic `qir-verbalizer-1` read-back and
//! the `Obj 15` certification interlock (Checker -> Exposer -> Compiler).

use qnlangc::{exposer, gates, lower, parser};
use std::path::PathBuf;
use std::process::Command;

const FIXTURES: [&str; 7] = [
    "minimal",
    "qm",
    "qphy",
    "qbio",
    "qchem",
    "qs_qlog",
    "call_worked",
];

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(format!("{}.qnl", name))
}

fn fixture_src(name: &str) -> String {
    std::fs::read_to_string(fixture_path(name)).unwrap()
}

fn temp_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!(
        "qnlangc-expose-{}-{}",
        std::process::id(),
        tag
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn exposition_is_deterministic_and_bounded_for_all_seven() {
    let mut hashes = Vec::new();
    for name in FIXTURES {
        let src = fixture_src(name);
        let p = parser::parse_source(&src).unwrap();
        assert!(
            gates::check(&p, &src).is_empty(),
            "{} must be Checker-clean",
            name
        );
        let a = lower::analyze(&p, &src);

        // Two independent Exposer passes over two parser instances must agree.
        let p2 = parser::parse_source(&src).unwrap();
        let a2 = lower::analyze(&p2, &src);
        let e1 = exposer::expose(&p, &a, &format!("fixtures/{}.qnl", name));
        let e2 = exposer::expose(&p2, &a2, &format!("fixtures/{}.qnl", name));

        assert_eq!(e1.body, e2.body, "{} body drift", name);
        assert_eq!(e1.sha256, e2.sha256, "{} digest drift", name);
        assert_eq!(e1.lines, e1.body.lines().count());
        assert_eq!(e1.bytes, e1.body.as_bytes().len());
        assert_eq!(e1.sha256.len(), 64);
        assert_eq!(e1.sha256_tagged, format!("sha-256:{}", e1.sha256));
        assert!(e1.matches(&e1.sha256));
        assert!(e1.matches(&e1.sha256_tagged));

        // The body is path-independent and never self-referential.
        let abs = exposer::expose(&p, &a, &fixture_path(name).to_string_lossy());
        assert_eq!(abs.sha256, e1.sha256, "{} path-dependent digest", name);
        assert!(!e1.body.contains("readback_sha256"));

        // Required exposition surfaces.
        assert!(e1.body.contains("qir-verbalizer-1"));
        assert!(e1.body.contains("H(T | Contract) = 0"));
        assert!(e1.body.contains("MODAL_HYPOTHESIS"));
        assert!(e1.body.contains("Q(dcc)"));
        assert!(e1.body.contains("Q(kappa)"));
        assert!(e1.body.contains("Q(domain)"));
        assert!(e1.body.contains("Q(mode)"));
        assert!(e1.body.contains("[declarations]"));
        assert!(e1.body.contains("[containers]"));
        assert!(e1.body.contains("[control]"));
        assert!(e1.body.contains("[budget]"));
        assert!(e1.body.contains("emitted entry B_hat:"));

        println!(
            "{} lines={} bytes={} sha-256:{}",
            name, e1.lines, e1.bytes, e1.sha256
        );
        hashes.push(e1.sha256);
    }
    hashes.sort();
    hashes.dedup();
    assert_eq!(hashes.len(), FIXTURES.len(), "read-back digests must be distinct");
}

#[test]
fn wrong_certification_refuses_emission_with_zero_bytes() {
    let dir = temp_dir("wrong");
    let out = dir.join("refused.cbor");
    let bin = env!("CARGO_BIN_EXE_qnlangc");
    let fixture = fixture_path("call_worked");
    let wrong = "0".repeat(64);

    let output = Command::new(bin)
        .arg("--emit-qir")
        .arg(&fixture)
        .arg("-o")
        .arg(&out)
        .arg("--certify-readback")
        .arg(&wrong)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("exit={} stdout=\n{}", output.status, stdout);

    assert!(!output.status.success(), "mismatched certification must fail");
    assert!(stdout.contains("E_CERTIFICATION_MISMATCH"));
    assert!(stdout.contains("qir-verbalizer-1"));
    assert!(stdout.contains("emission refused"));
    assert!(!out.exists(), "no CBOR may be written on refusal");
    assert!(!stdout.contains("LOWER "), "Compiler must not commit");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn exact_certification_emits_identical_cbor() {
    let dir = temp_dir("exact");
    let uncert = dir.join("uncertified.cbor");
    let cert = dir.join("certified.cbor");
    let bin = env!("CARGO_BIN_EXE_qnlangc");
    let fixture = fixture_path("call_worked");

    // The exact read-back digest, computed through the same Exposer surface.
    let src = fixture_src("call_worked");
    let p = parser::parse_source(&src).unwrap();
    let a = lower::analyze(&p, &src);
    let exp = exposer::expose(&p, &a, "call_worked.qnl");
    let committed = lower::lower(&p, &src).cbor.unwrap();

    let plain = Command::new(bin)
        .arg("--emit-qir")
        .arg(&fixture)
        .arg("-o")
        .arg(&uncert)
        .output()
        .unwrap();
    let plain_out = String::from_utf8_lossy(&plain.stdout);
    assert!(plain.status.success());
    assert!(plain_out.contains("MODAL_HYPOTHESIS (uncertified dev emit)"));
    assert!(plain_out.contains(&format!("sha256={}", exp.sha256_tagged)));

    let certified = Command::new(bin)
        .arg("--emit-qir")
        .arg(&fixture)
        .arg("-o")
        .arg(&cert)
        .arg("--certify-readback")
        .arg(&exp.sha256_tagged)
        .output()
        .unwrap();
    let cert_out = String::from_utf8_lossy(&certified.stdout);
    println!("certified exit={} stdout=\n{}", certified.status, cert_out);
    assert!(certified.status.success());
    assert!(cert_out.contains("READBACK_CERTIFIED"));

    let b1 = std::fs::read(&uncert).unwrap();
    let b2 = std::fs::read(&cert).unwrap();
    assert_eq!(b1, b2, "certification must not perturb qir-enc-4 bytes");
    assert_eq!(b1, committed, "CLI bytes must equal in-process lowering");
    assert_eq!(b1.len(), 856);
    let _ = std::fs::remove_dir_all(&dir);
}
