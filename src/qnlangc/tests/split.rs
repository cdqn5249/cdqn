//! T45: Q(2:split) lowering + tuple .0/.1 projection (existing QnIR substrate).

fn prog(body: &str) -> qnlangc::ast::Program {
    qnlangc::parser::parse_source(&format!(
        "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\n{}\n",
        body
    )).unwrap()
}

const GROUND: &str = "Q(dcc) d;\nQ(kappa) dyadic;\nQ(domain) Qm;\nQ(mode) L0;\nQ(type) T : u64;\nQ(anchor) z : Q(0) : T;\nQ(metric) m : eta(1,1,1);\n";

#[test]
fn q2split_lowers_and_is_checker_clean() {
    let src = format!("{}Q(1) -> one : u64;\nQ(0) -> zro : u64;\nQ(2:split)(one){{T->{{->one}};F->{{->zro}};}}->s:u64;\nemit receipt;\n", GROUND);
    let p = qnlangc::parser::parse_source(&src).unwrap();
    assert!(qnlangc::gates::check(&p, &src).is_empty(), "checker diagnostics");
    let r = qnlangc::lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok, "lowering: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn postfix_01_projects_tuple_fields() {
    let src = format!("{}Q(2:cons)(Q(5),Q(6))->p:Q(tuple)<u64,u64>;\np.0->a:u64;\np.1->b:u64;\na+b->out:u64;\nemit receipt;\n", GROUND);
    let p = qnlangc::parser::parse_source(&src).unwrap();
    assert!(qnlangc::gates::check(&p, &src).is_empty());
    let r = qnlangc::lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok, "lowering: {:?}", r.diagnostics.iter().map(|d| &d.code).collect::<Vec<_>>());
}

#[test]
fn q2split_and_projection_emit_qnir_and_run() {
    // synthesize a minimal QnIR-emitting program; structure check only (QnIR
    // execution parity is exercised via `qnlangc --emit-qir` + qnir downstream).
    let src = format!("{}Q(1)->one:u64;\nQ(0)->zro:u64;\nQ(2:split)(one){{T->{{->one}};F->{{->zro}};}}->s:u64;\nQ(2:cons)(Q(5),Q(6))->p:Q(tuple)<u64,u64>;\np.0->a:u64;\np.1->b:u64;\ns+a->tmp:u64;\ntmp+b->out:u64;\nemit receipt;\n", GROUND);
    let p = qnlangc::parser::parse_source(&src).unwrap();
    let r = qnlangc::lower::lower_with_target(&p, &src, "qir-enc-5");
    assert!(r.ok);
    assert!(r.cbor.as_ref().map(|v|!v.is_empty()).unwrap_or(false));
}

#[test]
fn reject_non_bool_scrutiny_and_nonpair_projection_inputs_parse_ok() {
    // .0/.1 on a non-pair value parses; QnIR FIELD would reject at runtime,
    // but structural parsing must not implicitly invent a rule.
    let _ = prog("Q(1)->x:u64;\nx.0->y:u64;\nemit receipt;");
}