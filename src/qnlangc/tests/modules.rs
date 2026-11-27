//! Multi-file `Q(use)` content-addressed module linking tests.

use qnlangc::modules;
use std::path::{Path, PathBuf};

const PURE_LIB: &str = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\nQ(pattern) square(x : u64) -> u64 ![may_emit(receipt)] { x * x -> r : u64; -> r }\n";

fn sha256_hex(data: &[u8]) -> String {
    qnlangc::sha256::hex(&qnlangc::sha256::sha256(data))
}

fn tmp_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("qnlangc-modules-{}-{}", std::process::id(), tag));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn main_using(digest: &str) -> String {
    format!(
        "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(use) lib_math : \"lib.qnl\" @ sha-256:{};\nQ(1) -> a : u64;\nlib_math::square(a) -> res : u64;\nemit receipt;\n",
        digest
    )
}

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name)
}

#[test]
fn modular_worked_example_links_key9_callee() {
    let p = fixture("main_mod.qnl");
    let prog = modules::load_program(&p).unwrap();
    assert!(prog.tops.iter().any(|t| matches!(&t.kind,
        qnlangc::ast::TopKind::Pattern { name, .. } if name == "lib_math::square")));
    assert!(qnlangc::gates::check(&prog, &std::fs::read_to_string(&p).unwrap()).is_empty());
    let src = std::fs::read_to_string(&p).unwrap();
    let r = qnlangc::lower::lower_with_target(&prog, &src, "qir-enc-5");
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    assert_eq!(r.callees, 1, "key-9 callee must be deduplicated across callers");
}

#[test]
fn digest_mismatch_is_loud() {
    let dir = tmp_dir("digest");
    let lib = dir.join("lib.qnl");
    std::fs::write(&lib, PURE_LIB).unwrap();
    let main = dir.join("main.qnl");
    std::fs::write(&main, main_using(&"0".repeat(64))).unwrap();
    let err = modules::load_program(&main).unwrap_err();
    assert_eq!(err.code, "E_DIGEST_MISMATCH");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cyclic_import_is_rejected() {
    let dir = tmp_dir("cycle");
    let a = dir.join("a.qnl");
    let b = dir.join("b.qnl");
    let head = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\n";
    // B is written first with a placeholder; A then pins sha-256(B).
    std::fs::write(&b, format!("{}Q(use) a : \"a.qnl\" @ sha-256:{};\n", head, "0".repeat(64))).unwrap();
    let sha_b = sha256_hex(&std::fs::read(&b).unwrap());
    std::fs::write(&a, format!("{}Q(use) b : \"b.qnl\" @ sha-256:{};\n", head, sha_b)).unwrap();
    let err = modules::load_program(&a).unwrap_err();
    assert_eq!(err.code, "E_CYCLIC_IMPORT");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn impure_library_import_is_rejected() {
    let dir = tmp_dir("impure");
    let lib = dir.join("lib.qnl");
    let impure = format!("{}emit receipt;\n", PURE_LIB);
    std::fs::write(&lib, impure).unwrap();
    let sha = sha256_hex(&std::fs::read(&lib).unwrap());
    let main = dir.join("main.qnl");
    std::fs::write(&main, main_using(&sha)).unwrap();
    let err = modules::load_program(&main).unwrap_err();
    assert_eq!(err.code, "E_IMPURE_LIBRARY_IMPORT");
    let _ = std::fs::remove_dir_all(&dir);
}
