//! Multi-file `Q(use)` content-addressed module linking tests.

use qnlangc::modules;
use std::path::{Path, PathBuf};

const PURE_LIB: &str = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\nQ(pattern) square(x : u64) -> u64 ![may_emit(receipt)] { x * x -> r : u64; -> r }\n";

fn sha256_hex(data: &[u8]) -> String {
    qnlangc::sha256::hex(&qnlangc::sha256::sha256(data))
}

static TMP_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn tmp_dir(tag: &str) -> PathBuf {
    // strictly isolated per test invocation: unique tag + atomic monotonic
    // counter, so concurrent tests never share a directory.
    let n = TMP_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let d = std::env::temp_dir().join(format!(
        "qnlangc-modules-{}-{}-{}-{}",
        std::process::id(),
        tag,
        n,
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|x| x.as_nanos()).unwrap_or(0)
    ));
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


// T34: multi-pattern pure library; pattern B calls sibling A inside the same
// imported .qnl (intra-library cross-call). Q(use) pins sha-256 of raw bytes.
// Must pass Checker (G1-G14 + E_NAMESPACE_COLLISION), Exposer, R11' lowering,
// and resolve lias::A to a key-9 callee id with no E_LOWERING_AMBIGUOUS.
#[test]
fn intra_library_cross_call_resolves_and_lowers() {
    let dir = tmp_dir("intra");
    let head = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\n";
    let lib_src = format!(
        "{}Q(pattern) A(v : u64) -> u64 ![may_emit(receipt)] {{ v + Q(1) -> r : u64; -> r }}\nQ(pattern) B(v : u64) -> u64 ![may_emit(receipt)] {{ A(v) -> t : u64; t + Q(1) -> r : u64; -> r }}\n",
        head
    );
    let lib = dir.join("lib.qnl");
    std::fs::write(&lib, &lib_src).unwrap();
    let sha = sha256_hex(&std::fs::read(&lib).unwrap());
    let main_src = format!(
        "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(use) x : \"lib.qnl\" @ sha-256:{};\nQ(0) -> a : u64;\nx::B(a) -> res : u64;\nemit receipt;\n",
        sha
    );
    let main = dir.join("main.qnl");
    std::fs::write(&main, &main_src).unwrap();

    let prog = modules::load_program(&main).unwrap();
    let src = std::fs::read_to_string(&main).unwrap();

    // Checker: G1-G14 + E_NAMESPACE_COLLISION clean.
    let diags = qnlangc::gates::check(&prog, &src);
    assert!(diags.is_empty(), "checker diagnostics: {:?}", diags.iter().map(|d| &d.code).collect::<Vec<_>>());

    // Multi-pattern pure library imported; intra-library sibling qualified.
    assert!(prog.tops.iter().any(|t| matches!(&t.kind, qnlangc::ast::TopKind::Pattern { name, .. } if name == "x::A")));
    assert!(prog.tops.iter().any(|t| matches!(&t.kind, qnlangc::ast::TopKind::Pattern { name, .. } if name == "x::B")));

    // R11' lowering (qir-enc-5); no E_LOWERING_AMBIGUOUS; key-9 callee for sibling A + B.
    let r = qnlangc::lower::lower_with_target(&prog, &src, "qir-enc-5");
    assert!(r.ok, "diagnostics: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    assert!(!r.diagnostics.iter().any(|d| d.code == "E_LOWERING_AMBIGUOUS"));
    assert_eq!(r.callees, 2, "key-9: exactly x::A and x::B callees");

    // Exposer over Checker + R11'.
    let exp = qnlangc::exposer::expose(&prog, &r, &src);
    assert!(!exp.sha256_tagged.is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}


#[test]
fn call_cycle_and_self_call_are_rejected_statically() {
    let dir = tmp_dir("cycle");
    let head = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\n";
    let lib_src = format!("{}Q(pattern) A(v : u64) -> u64 ![may_emit(receipt)] {{ A(v) -> r : u64; -> r }}\n", head);
    let lib = dir.join("lib.qnl");
    std::fs::write(&lib, &lib_src).unwrap();
    let sha = sha256_hex(&std::fs::read(&lib).unwrap());
    let main = dir.join("main.qnl");
    std::fs::write(&main, format!("Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(use) x : \"lib.qnl\" @ sha-256:{};\nQ(0) -> a : u64;\nx::A(a) -> r : u64;\nemit receipt;\n", sha)).unwrap();
    let prog = modules::load_program(&main).unwrap();
    let r = qnlangc::lower::lower_with_target(&prog, &std::fs::read_to_string(&main).unwrap(), "qir-enc-5");
    assert!(!r.ok, "self-call must be rejected");
    assert!(r.diagnostics.iter().any(|d| d.code == "E_LOWERING_AMBIGUOUS"), "{:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
    let _ = std::fs::remove_dir_all(&dir);
}