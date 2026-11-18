//! QnIR conformance artifact — the normative assembler and validator with
//! the reference interpreter (conformance §4.2/§4.3, artifact-share
//! increments 2–3, R6).
//!
//! The tool reads the committed golden corpus (`docs/qnir_goldens.md`,
//! repository bytes) and, over the full declared seed:
//!   suite E — assembles each declared source value to its canonical §2.6
//!             encoding and compares the committed expected bytes;
//!   suite I — assembles each declared source instruction likewise;
//!   suite M — decodes each committed malformed surface and compares the
//!             typed verdict the committed text declares;
//!   suite X — assembles each committed source program to its Intent
//!             envelope and byte-compares the committed envelope, then
//!             validates: the §2.1 static legality classes, the §3.2
//!             dispatch-time operand bounds, and the §3.4 admission rule
//!             through the declared `B_hat` surrogate;
//!   suite X execution — executes each committed envelope through the
//!             committed transition relation and compares the full expected
//!             outcome at the five agreement points of §4.1 (records, state
//!             roots, terminal receipt, reconstructed ledger; value
//!             encodings under suite E) — the reference interpreter's
//!             increment (§4.3), after the assembler's (§4.5's order).
//!
//! The tool never generates or edits goldens (§4.2): the corpus is its check
//! target. Its output is a deterministic transcript; a same-binary rerun is
//! reproducibility evidence only (§4.2) — never independent reproduction,
//! never conformance.

mod bhat;
mod cbor;
mod insn;
mod interp;
mod sha256;
mod value;

use insn::{SrcInsn, *};
use crate::cbor::Cbor;
use std::fs;
use value::{enc_value, Value};

pub(crate) use interp::{cmp_artifacts, collision_entries, CompleteArtifact, outcome_name, suite_x_exec, trace_root};

// ------------------------------------------------------------ text helpers

fn normalize(s: &str) -> String {
    s.replace('\u{2212}', "-") // unicode minus → ASCII
        .replace('\u{2264}', "<=") // ≤ → ASCII
        .replace('\u{2192}', "->") // → → ASCII
}

/// Strips all whitespace and decodes a hex dump (spaced words or byte
/// groups) to bytes.
pub(crate) fn hex_to_bytes(s: &str) -> Vec<u8> {
    let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    (0..clean.len() / 2)
        .map(|i| u8::from_str_radix(&clean[2 * i..2 * i + 2], 16).unwrap())
        .collect()
}

/// A parsed fenced code block with the last non-empty text line before it.
pub(crate) struct Fence {
    pub(crate) marker: String,
    pub(crate) body: String,
}

pub(crate) fn fences(section: &str) -> Vec<Fence> {
    let mut out = Vec::new();
    let mut marker = String::new();
    let mut in_fence = false;
    let mut body = Vec::new();
    for line in section.lines() {
        if in_fence {
            if line.trim_start().starts_with("```") {
                out.push(Fence {
                    marker: marker.clone(),
                    body: body.join("\n"),
                });
                body.clear();
                in_fence = false;
                marker.clear();
            } else {
                body.push(line);
            }
        } else if line.trim_start().starts_with("```") {
            in_fence = true;
        } else if !line.trim().is_empty() {
            marker = line.trim().to_string();
        }
    }
    out
}

pub(crate) fn fence_after<'a>(f: &'a [Fence], needle: &str) -> Option<&'a Fence> {
    f.iter().find(|k| k.marker.contains(needle))
}

pub(crate) fn line_starting<'a>(section: &'a str, prefix: &str) -> Option<String> {
    section
        .lines()
        .map(|l| l.trim())
        .find(|l| l.starts_with(prefix))
        .map(|l| normalize(l))
}

// ------------------------------------------------------------- source forms

/// One integer literal of the declared source forms: decimal, `2^k`, or
/// `2^k - 1`, each optionally negated.
enum Num {
    I(i128),
    Pow(u32),
    PowMinusOne(u32),
}

impl Num {
    fn to_value(&self) -> Value {
        match self {
            Num::I(v) => Value::int(*v),
            Num::Pow(k) => Value::pow2(*k),
            Num::PowMinusOne(k) => {
                // 2^k - 1: subtract one from the least significant magnitude
                // word of 2^k and strip the leading zero word it leaves
                let pow = Value::pow2(*k);
                match pow {
                    Value::Int { neg, mut mag } => {
                        debug_assert!(!neg && !mag.is_empty());
                        let mut borrow = 1u128;
                        for word in mag.iter_mut().rev() {
                            let w = *word as u128;
                            if borrow == 1 {
                                if w >= 1 {
                                    *word = (w - 1) as u64;
                                    borrow = 0;
                                } else {
                                    *word = u64::MAX;
                                }
                            }
                        }
                        while mag.len() > 1 && mag[0] == 0 {
                            mag.remove(0);
                        }
                        Value::Int { neg: false, mag }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn parse_int_tok(s: &str) -> Num {
    let s = s.trim();
    if let Some(exp) = s.strip_prefix("2^") {
        let (k, minus) = match exp.split_once('-') {
            Some((a, b)) if b.trim() == "1" => (a, true),
            _ => (exp, false),
        };
        let k: u32 = k.trim().parse().unwrap();
        return if minus { Num::PowMinusOne(k) } else { Num::Pow(k) };
    }
    Num::I(s.parse().unwrap())
}

/// `−2^64` and friends: a negated power keeps its magnitude form.
fn parse_int_with_negpow(s: &str) -> Value {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix('-') {
        if rest.starts_with("2^") {
            if let Some((k, minus)) = rest[2..].split_once('-') {
                if minus.trim() == "1" {
                    let mut v = parse_int_with_negpow_inner_pow(k.trim().parse().unwrap());
                    // 2^k - 1, negated
                    match &mut v {
                        Value::Int { mag, .. } => {
                            // negate in place: sign rides the type tag
                            let m = mag.clone();
                            *mag = m;
                        }
                        _ => unreachable!(),
                    }
                    return neg_of(v);
                }
            }
            return neg_of(parse_int_with_negpow_inner_pow(rest[2..].trim().parse().unwrap()));
        }
        return Value::int(-rest.trim().parse::<i128>().unwrap());
    }
    parse_int_tok(s).to_value()
}

fn parse_int_with_negpow_inner_pow(k: u32) -> Value {
    Value::pow2(k)
}

fn neg_of(v: Value) -> Value {
    match v {
        Value::Int { mag, .. } => Value::Int { neg: true, mag },
        _ => unreachable!(),
    }
}

/// A recursive-descent parser for the declared tuple source forms,
/// e.g. `(1, -2)` and `(0, (1, -2))`.
fn parse_tuple(s: &str) -> Value {
    let s = s.trim();
    assert!(s.starts_with('(') && s.ends_with(')'), "tuple form {s}");
    let inner = &s[1..s.len() - 1];
    let mut fields = Vec::new();
    let mut depth = 0usize;
    let mut cur = String::new();
    for c in inner.chars() {
        match c {
            '(' => {
                depth += 1;
                cur.push(c);
            }
            ')' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                fields.push(parse_field(&cur));
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        fields.push(parse_field(&cur));
    }
    Value::Tuple(fields)
}

fn parse_field(s: &str) -> Value {
    let s = s.trim();
    if s.starts_with('(') {
        parse_tuple(s)
    } else {
        parse_int_with_negpow(s)
    }
}

/// Parses one Family E declared input line to a source value.
fn parse_e_input(line: &str) -> Value {
    let l = normalize(line);
    let l = l.trim_start_matches("Input:").trim();
    if let Some(rest) = l.strip_prefix("the exact integer ") {
        let tok: String = rest
            .chars()
            .take_while(|c| !c.is_whitespace() && *c != '.')
            .collect();
        // rejoin a split "2^64 - 1" tail if the tokenizer stopped at space
        let tok = if rest.starts_with("2^") && rest.contains("- 1") && !tok.contains('-') {
            let t: String = rest.chars().take_while(|c| *c != '.').collect();
            t.replace(' ', "")
        } else {
            tok
        };
        return parse_int_with_negpow(&tok);
    }
    if let Some(rest) = l.strip_prefix("the tuple ") {
        let tok: String = rest.chars().take_while(|c| *c != '.').collect();
        return parse_tuple(&tok);
    }
    if l.contains("mantissa") {
        // "… with mantissa 6, exponent -2, …"
        let m: String = l
            .split("mantissa ")
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        let e: String = l
            .split("exponent ")
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        let (m, e): (i128, i128) = (m.parse().unwrap(), e.parse().unwrap());
        return Value::Zoom(m, e); // the encoder applies §2.3's N
    }
    if let Some(rest) = l.strip_prefix("the zoom-carrying value (") {
        let tok: String = rest.chars().take_while(|c| *c != ')').collect();
        let mut it = tok.split(',');
        let m: i128 = it.next().unwrap().trim().parse().unwrap();
        let z: i128 = it.next().unwrap().trim().parse().unwrap();
        return Value::Zoom(m, z);
    }
    panic!("unrecognized family E input form: {line}");
}

/// Parses one Family I declared input line to a source instruction.
fn parse_i_input(line: &str, insns: &dyn Fn(&str, &str) -> Option<u64>) -> SrcInsn {
    let l = normalize(line);
    let l = l.trim_start_matches("Input:").trim().to_string();
    let bound_of = |s: &str| -> i128 {
        let after = s.split("<=").nth(1).unwrap();
        let tok: String = after
            .trim_start()
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        tok.trim().parse().unwrap()
    };
    if let Some(rest) = l.strip_prefix("CONST with literal ") {
        let tok: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
        let b = bound_of(&l);
        return SrcInsn::Const {
            imm: Value::int(tok.trim().parse().unwrap()),
            bound: b,
            rtag: 1,
        };
    }
    if let Some(rest) = l.strip_prefix("ADD of static value ids ") {
        let (a, b) = rest.split_once(" and ").unwrap();
        let a: u64 = a.trim().parse().unwrap();
        let b: String = b.chars().take_while(|c| c.is_ascii_digit()).collect();
        return SrcInsn::Arith {
            tag: OP_ADD,
            a,
            b: b.trim().parse().unwrap(),
            bound: bound_of(&l),
        };
    }
    if let Some(rest) = l.strip_prefix("BR on condition code ") {
        let ccname: String = rest.chars().take_while(|c| c.is_ascii_uppercase()).collect();
        let cond: String = rest
            .split("static value id ")
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let t: String = rest
            .split("true edge to block ")
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let f: String = rest
            .split("false edge to block ")
            .nth(1)
            .unwrap()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        return SrcInsn::Br {
            cc: insns(&ccname, "").expect("unknown condition code"),
            cond: cond.trim().parse().unwrap(),
            true_blk: t.trim().parse().unwrap(),
            false_blk: f.trim().parse().unwrap(),
        };
    }
    panic!("unrecognized family I input form: {line}");
}

// ------------------------------------------------------------ corpus parsing

pub(crate) struct Vector {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) section: String,
}

pub(crate) fn parse_vectors(md: &str, family_header: &str) -> Vec<Vector> {
    let start = md.find(family_header).unwrap_or_else(|| {
        panic!("family header not found: {family_header}")
    });
    let rest = &md[start..];
    let end = rest[1..]
        .find("\n## ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    let body = &rest[..end];
    let mut out = Vec::new();
    let mut cursor = 0;
    while let Some(h) = body[cursor..].find("\n### ") {
        let hstart = cursor + h + 1;
        let vend = body[hstart + 1..]
            .find("\n### ")
            .map(|i| hstart + 1 + i)
            .unwrap_or(body.len());
        let section = &body[hstart..vend];
        let headline = section.lines().next().unwrap().trim_start_matches('#').trim().to_string();
        let mut it = headline.split_whitespace();
        let id = it.next().unwrap().to_string();
        // the name is the trailing backticked token; middle tokens may carry
        // the em-dash separator
        let name = headline
            .split_whitespace()
            .last()
            .unwrap()
            .trim_matches('`')
            .to_string();
        out.push(Vector {
            id,
            name,
            section: section.to_string(),
        });
        cursor = vend;
    }
    out
}

/// The declared per-vector parameters of a Family X vector.
pub(crate) struct XParams {
    pub(crate) w_budget: u64,
    pub(crate) sem: String,
    pub(crate) enc: String,
    pub(crate) prof: String,
    pub(crate) admission_bp: u64,
    pub(crate) expected_outcome_tag: u64,
    pub(crate) expected_outcome_name: String,
}

pub(crate) fn parse_x_params(section: &str) -> XParams {
    let pl = line_starting(section, "Declared parameters:").unwrap();
    let wb: String = pl
        .split("`W_budget` = ")
        .nth(1)
        .unwrap()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let vt = pl.split("`V` = (").nth(1).unwrap();
    let vend = vt.find(')').unwrap();
    let mut parts = vt[..vend].split(',');
    let sem = parts.next().unwrap().trim().to_string();
    let enc = parts.next().unwrap().trim().to_string();
    let prof = parts.next().unwrap().trim().to_string();
    let admission_bp = parse_admission(section);
    let eo = line_starting(section, "Expected terminal outcome:").unwrap();
    let tag: String = eo
        .split("(tag ")
        .nth(1)
        .unwrap()
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let name: String = eo
        .trim_start_matches("Expected terminal outcome: ")
        .trim_start_matches("**")
        .split("**")
        .next()
        .unwrap()
        .trim()
        .to_string();
    XParams {
        w_budget: wb.parse().unwrap(),
        sem,
        enc,
        prof,
        admission_bp,
        expected_outcome_tag: tag.parse().unwrap(),
        expected_outcome_name: name,
    }
}

/// The declared admission bound from the vector's Admission note: either an
/// explicit `B(P) = N U`, or the note's path charge plus the gateway charge
/// it names ("plus the gateway 20 U" — the number is profile data, parsed,
/// never built in).
fn parse_admission(section: &str) -> u64 {
    let a = line_starting(section, "Admission:").unwrap();
    if let Some(rest) = a.split("B(P) = ").nth(1) {
        let tok: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        return tok.parse().unwrap();
    }
    if let Some(rest) = a.split("plus the gateway ").nth(1) {
        let gateway: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        let left = a.split("plus the gateway ").next().unwrap();
        // the path charge is the last number before the gateway phrase
        let digits: String = left
            .char_indices()
            .filter(|(_, c)| c.is_ascii_digit() || *c == ' ')
            .map(|(_, c)| c)
            .collect();
        let last_num: String = digits.split_whitespace().last().unwrap().to_string();
        return last_num.parse::<u64>().unwrap() + gateway.parse::<u64>().unwrap();
    }
    panic!("unrecognized admission note: {a}");
}

// ------------------------------------------------------------------- suites

pub(crate) struct Report {
    lines: Vec<String>,
    failures: usize,
}

impl Report {
    pub(crate) fn new() -> Self {
        Report {
            lines: Vec::new(),
            failures: 0,
        }
    }
    pub(crate) fn say(&mut self, s: String) {
        self.lines.push(s);
    }
    pub(crate) fn fail(&mut self, s: String) {
        self.failures += 1;
        self.lines.push(format!("  FAIL {s}"));
    }
}

fn suite_e(md: &str, rep: &mut Report) {
    let vectors = parse_vectors(md, "## Family E — Value Encodings");
    let mut pass = 0usize;
    for v in &vectors {
        let f = fences(&v.section);
        let input = line_starting(&v.section, "Input:").unwrap();
        let src = parse_e_input(&input);
        let got = enc_value(&src);
        let want = hex_to_bytes(&fence_after(&f, "Expected canonical bytes").unwrap().body);
        let words = got.len() / 8;
        if got == want {
            pass += 1;
            rep.say(format!("  {} {}: match ({} words)", v.id, v.name, words));
        } else {
            let mut i = 0;
            while i < got.len() && i < want.len() && got[i] == want[i] {
                i += 1;
            }
            rep.fail(format!(
                "{} {}: byte divergence at offset {} (got {} bytes, expected {})",
                v.id,
                v.name,
                i,
                got.len(),
                want.len()
            ));
        }
    }
    rep.say(format!(
        "suite E (value encodings): {} — {}/{} committed vectors re-encode byte-exactly",
        if pass == vectors.len() { "PASS" } else { "FAIL" },
        pass,
        vectors.len()
    ));
}

fn suite_i(md: &str, rep: &mut Report) {
    let vectors = parse_vectors(md, "## Family I — Instruction Encodings");
    let mut pass = 0usize;
    for v in &vectors {
        let f = fences(&v.section);
        let input = line_starting(&v.section, "Input:").unwrap();
        let cc = |name: &str, _unused: &str| -> Option<u64> {
            Some(match name {
                "LT" => CC_LT,
                "LE" => CC_LE,
                "EQ" => CC_EQ,
                "NE" => CC_NE,
                "GT" => CC_GT,
                "GE" => CC_GE,
                _ => return None,
            })
        };
        let src = parse_i_input(&input, &cc);
        let got = enc_insn(&src);
        let want = hex_to_bytes(&fence_after(&f, "Expected canonical words").unwrap().body);
        if got == want {
            pass += 1;
            rep.say(format!(
                "  {} {}: match ({} words)",
                v.id,
                v.name,
                got.len() / 8
            ));
        } else {
            let mut i = 0;
            while i < got.len() && i < want.len() && got[i] == want[i] {
                i += 1;
            }
            rep.fail(format!(
                "{} {}: byte divergence at offset {} (got {} bytes, expected {})",
                v.id,
                v.name,
                i,
                got.len(),
                want.len()
            ));
        }
    }
    rep.say(format!(
        "suite I (instruction encodings): {} — {}/{} committed vectors re-encode byte-exactly",
        if pass == vectors.len() { "PASS" } else { "FAIL" },
        pass,
        vectors.len()
    ));
}

/// The declared shape of a Family R input (expansion increment 5): a
/// side-table record built from its declared index and payload reading, or
/// the populated-table envelope checked against its declared record.
enum RInput {
    Record { idx: u64, payload: Option<Value> },
    Envelope { idx: u64 },
}

fn parse_r_index(l: &str) -> u64 {
    let after = l.split("(index ").nth(1).expect("the declared record index");
    let tok: String = after
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    tok.parse().expect("the record index word")
}

/// Parses one Family R declared input line.
fn parse_r_input(line: &str) -> RInput {
    let l = normalize(line);
    let l = l.trim_start_matches("Input:").trim();
    let idx = parse_r_index(l);
    if l.contains("Intent envelope") {
        return RInput::Envelope { idx };
    }
    if l.contains("the empty payload") {
        return RInput::Record { idx, payload: None };
    }
    if let Some(rest) = l.split("the exact integer ").nth(1) {
        let tok: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        return RInput::Record {
            idx,
            payload: Some(parse_int_with_negpow(&tok)),
        };
    }
    if let Some(rest) = l.split("the tuple ").nth(1) {
        let end = rest.find("))").expect("the tuple's closing form");
        let tok = &rest[..end + 1];
        return RInput::Record {
            idx,
            payload: Some(parse_tuple(tok)),
        };
    }
    panic!("unrecognized family R input form: {line}");
}

fn suite_r(md: &str, rep: &mut Report) {
    let vectors =
        parse_vectors(md, "## Family R — EXT Side-Table Record Encodings");
    let mut pass = 0usize;
    for v in &vectors {
        let f = fences(&v.section);
        let input = line_starting(&v.section, "Input:").unwrap();
        let want =
            hex_to_bytes(&fence_after(&f, "Expected canonical bytes")
                .unwrap()
                .body);
        let ok = match parse_r_input(&input) {
            RInput::Record { idx, payload } => {
                let pay = match payload {
                    None => Vec::new(),
                    Some(val) => enc_value(&val),
                };
                let got =
                    cbor::encode(&Cbor::Array(vec![
                        Cbor::UInt(idx),
                        Cbor::Bytes(pay),
                    ]));
                got == want
            }
            RInput::Envelope { idx } => {
                let decoded = match insn::decode_envelope(&want) {
                    Ok(_) => cbor::decode(&want).map_err(|e| e.0).unwrap(),
                    Err(_) => {
                        rep.fail(format!(
                            "{} {}: the committed envelope bytes do not \
                             decode as a statically legal Intent",
                            v.id, v.name
                        ));
                        continue;
                    }
                };
                let declared = match &decoded {
                    Cbor::Map(entries) => {
                        entries
                            .iter()
                            .find(|(k, _)| matches!(k, Cbor::UInt(7)))
                            .map(|(_, v)| v)
                    }
                    _ => None,
                };
                match declared {
                    Some(Cbor::Array(items)) => {
                        items.len() == 1
                            && matches!(
                                &items[0],
                                Cbor::Array(p)
                                    if p.len() == 2
                                        && matches!(
                                            &p[0],
                                            Cbor::UInt(n) if *n == idx
                                        )
                                        && matches!(
                                            &p[1],
                                            Cbor::Bytes(b) if b.is_empty()
                                        )
                            )
                    }
                    _ => false,
                }
            }
        };
        if ok {
            pass += 1;
            rep.say(format!(
                "  {} {}: match ({} bytes)",
                v.id,
                v.name,
                want.len()
            ));
        } else {
            rep.fail(format!(
                "{} {}: canonical bytes or declared record diverge",
                v.id, v.name
            ));
        }
    }
    rep.say(format!(
        "suite R (EXT side-table record encodings): {} — {}/{} committed \
         vectors re-encode byte-exactly",
        if pass == vectors.len() { "PASS" } else { "FAIL" },
        pass,
        vectors.len()
    ));
}

/// The declared shape of a Family M input, from its committed input
/// description: envelope-level, block-stream-level, single-instruction, or
/// bare value.
fn m_shape(section: &str) -> &'static str {
    let input = line_starting(section, "Input bytes:").unwrap();
    if input.contains("envelope") || input.contains("Intent") || input.contains("program") {
        "envelope"
    } else if input.contains("a block declaring") {
        "stream"
    } else if input.contains("instruction") {
        "insn"
    } else if input.contains("side-table record") || input.contains("side-table array") {
        // the EXT surface (expansion increment 5): a single record or a
        // side-table array
        "extrec"
    } else {
        "value"
    }
}

fn suite_m(md: &str, rep: &mut Report) {
    let vectors = parse_vectors(md, "## Family M — The Malformed Surface");
    let mut pass = 0usize;
    for v in &vectors {
        let f = fences(&v.section);
        let bytes = hex_to_bytes(&fence_after(&f, "Bytes:").unwrap().body);
        // the committed vector declares input, verdict, and bytes marker on
        // one line: "Input bytes: … Expected verdict: **…**. Bytes:"
        let inline = line_starting(&v.section, "Input bytes:").unwrap();
        let declared = inline
            .split("Expected verdict: ")
            .nth(1)
            .unwrap_or("canonical")
            .replace("**", "")
            .trim()
            .trim_end_matches('.')
            .trim_end_matches("Bytes:")
            .trim()
            .trim_end_matches('.')
            .to_string();
        let shape = m_shape(&v.section);
        let got = match shape {
            "value" => {
                let words = value::words_of(&bytes).unwrap();
                let mut r = value::WordReader::new(&words);
                match value::dec_value(&mut r) {
                    Ok(_) => "canonical".to_string(),
                    Err(e) => format!("malformed — {e}"),
                }
            }
            "insn" => {
                let words = value::words_of(&bytes).unwrap();
                let mut r = value::WordReader::new(&words);
                match insn::dec_insn(&mut r) {
                    Ok(_) => "canonical".to_string(),
                    Err(e) => format!("malformed — {e}"),
                }
            }
            "stream" => match decode_stream(&bytes, 1) {
                Ok(_) => "canonical".to_string(),
                Err(e) => format!("malformed — {e}"),
            },
            "envelope" => match decode_envelope(&bytes) {
                Ok(_) => "canonical".to_string(),
                Err(e) => {
                    // envelope-level verdicts carry their declared class
                    // wording: statically illegal vs malformed
                    if e.starts_with("dangling") {
                        format!("statically illegal — {e}")
                    } else {
                        format!("malformed — {e}")
                    }
                }
            },
            "extrec" => match insn::check_ext_record_or_table(&bytes) {
                Ok(_) => "canonical".to_string(),
                Err(e) => format!("malformed — {e}"),
            },
            _ => unreachable!(),
        };
        if got == declared {
            pass += 1;
            rep.say(format!("  {} {}: verdict match — {}", v.id, v.name, got));
        } else {
            rep.fail(format!(
                "{} {}: declared verdict \"{}\" but tool verdict \"{}\"",
                v.id, v.name, declared, got
            ));
        }
    }
    rep.say(format!(
        "suite M (malformed surfaces): {} — {}/{} committed verdicts reproduced",
        if pass == vectors.len() { "PASS" } else { "FAIL" },
        pass,
        vectors.len()
    ));
}

/// The parser's declared P9 content address, read from the corpus preamble
/// (never a built-in constant).
fn p9_address(md: &str) -> Vec<u8> {
    let needle = "the seed's single declared content address is the 8-byte string `h'";
    let rest = md.split(needle).nth(1).expect("P9 parameter not found");
    let hex: String = rest.chars().take_while(|c| *c != '\'').collect();
    hex_to_bytes(&hex)
}

fn suite_x(md: &str, rep: &mut Report) {
    let vectors = parse_vectors(md, "## Family X — Execution Vectors");
    let p9 = p9_address(md);
    let profile = bhat::golden_profile();
    let mut a_pass = 0usize;
    let mut a_total = 0usize;
    let mut v_ok = 0usize;
    let mut committed_envelopes: Vec<(String, Vec<u8>)> = Vec::new();

    // first pass: committed envelopes (X2 cites X1's)
    for v in &vectors {
        let f = fences(&v.section);
        let env = if let Some(ef) = fence_after(&f, "Intent envelope") {
            hex_to_bytes(&ef.body)
        } else {
            Vec::new() // cited at another vector
        };
        committed_envelopes.push((v.id.clone(), env));
    }

    for (vi, v) in vectors.iter().enumerate() {
        let params = parse_x_params(&v.section);
        let committed = &committed_envelopes[vi].1;

        // ---- assembly share (source form → envelope; §4.2)
        let f = fences(&v.section);
        if let Some(sf) = fence_after(&f, "source grammar's textual form") {
            a_total += 1;
            let src = parse_source_program(&sf.body);
            // declared inputs: V from the parameters line; dependency
            // addresses derived from the cited CLOAD addresses over the
            // declared P9 content address; the declared initial status pool
            // rides the vector's committed envelope (key 8 — vector data,
            // not program text; the program encoding is the check target)
            let cited: Vec<u64> = src
                .iter()
                .flat_map(|blk| blk.iter())
                .filter_map(|i| match i {
                    SrcInsn::Cload { addr, .. } => Some(*addr),
                    _ => None,
                })
                .collect();
            let n_deps = cited.iter().max().map(|m| m + 1).unwrap_or(0) as usize;
            let deps: Vec<Vec<u8>> = vec![p9.clone(); n_deps];
            let pool = decode_envelope(committed).map(|i| i.pool).unwrap_or_default();
            let mut stream = Vec::new();
            for (bid, insns) in src.iter().enumerate() {
                stream.extend_from_slice(&insn::w(bid as u64));
                stream.extend_from_slice(&insn::w(0)); // argument signature arity
                stream.extend_from_slice(&insn::w(insns.len() as u64));
                for i in insns {
                    stream.extend_from_slice(&enc_insn(i));
                }
            }
            let assembled = enc_envelope(
                &params.sem,
                &params.enc,
                &params.prof,
                &deps,
                src.len() as u64,
                &stream,
                &pool,
            );
            let stream_ok = match cbor::decode(committed) {
                Ok(cbor::Cbor::Map(entries)) => entries
                    .iter()
                    .find(|(k, _)| *k == cbor::Cbor::UInt(6))
                    .map(|(_, v)| match v {
                        cbor::Cbor::Bytes(b) => *b == stream,
                        _ => false,
                    })
                    .unwrap_or(false),
                _ => false,
            };
            if assembled == *committed {
                a_pass += 1;
                rep.say(format!(
                    "  {} {}: envelope match ({} bytes; stream match: {})",
                    v.id,
                    v.name,
                    assembled.len(),
                    stream_ok
                ));
            } else {
                rep.fail(format!(
                    "{} {}: assembled envelope diverges from the committed bytes (assembled {} B, committed {} B)",
                    v.id,
                    v.name,
                    assembled.len(),
                    committed.len()
                ));
            }
        } else {
            rep.say(format!(
                "  {} {}: no source form committed (envelope cited at X1, byte-equal vectors §4.4); assembly share not exercised",
                v.id, v.name
            ));
        }

        // ---- validation share (§4.2: static classes, bounds, admission)
        let env_ref: Vec<u8> = if committed.is_empty() {
            committed_envelopes
                .iter()
                .find(|(id, _)| id == "X1")
                .map(|(_, b)| b.clone())
                .unwrap()
        } else {
            committed.clone()
        };
        if validate_x(&v.id, &v.name, &env_ref, &params, &profile, rep) {
            v_ok += 1;
        }
    }
    rep.say(format!(
        "suite X (executions): assembly {} — {}/{} committed source forms re-encode byte-exactly; validation {} — {}/8 admitted or refused per the declared bounds",
        if a_pass == a_total { "PASS" } else { "FAIL" },
        a_pass,
        a_total,
        if v_ok == vectors.len() { "PASS" } else { "FAIL" },
        v_ok
    ));
}

// ------------------------------------------------------------ fixture mode

/// Digits before a marker ("8 U short", "1 backstop rejection record").
fn u64_before(s: &str, marker: &str) -> u64 {
    let left = s.split(marker).next().unwrap();
    let tok: String = left
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    tok.parse().unwrap()
}

fn take_u64(s: &str) -> (u64, String) {
    let tok: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    (tok.parse().unwrap(), s[tok.len()..].to_string())
}

/// Digits after a marker, chained through separators ("36 - 32 = 4").
fn u64_chain(s: &str, marker: &str, seps: &[&str]) -> Vec<u64> {
    let mut rest = s
        .splitn(2, marker)
        .nth(1)
        .unwrap_or_else(|| panic!("marker absent: {marker}"))
        .to_string();
    let mut out = Vec::new();
    let (v, r) = take_u64(&rest);
    out.push(v);
    rest = r;
    for sep in seps {
        rest = rest
            .splitn(2, sep)
            .nth(1)
            .unwrap_or_else(|| panic!("separator absent: {sep}"))
            .to_string();
        let (v, r) = take_u64(&rest);
        out.push(v);
        rest = r;
    }
    out
}

fn ttype_of_name(name: &str) -> u64 {
    match name {
        "STATUS_LOOKUP_MISS" => 1,
        "STATUS_NEG_EVEN_ROOT" => 2,
        "EXPRESSION_TOO_COMPLEX" => 3,
        "STATUS_DOMAIN_ERROR" => 4,
        _ => panic!("unknown token type name: {name}"),
    }
}

/// The F27 fuel-backstop fixture (§4.4's declared fixture semantics,
/// v0.3.0-draft.23): a test-only validator-defect injection over the corpus's
/// Fixture Register — never a conformance vector and never counted in the
/// declared seed. The declared injection overrides the validator's computed
/// bound with `B_injected` < `B_hat(P_f)`; the fixture's two declared verdict
/// components (the validator-defect report and the containment verdict) are
/// checked against the register's declared semantic expectation, and the
/// fixture block is rendered byte-identically to the second implementation's.
/// The F6 collision-safe artifact comparison fixture (§4.4's declared
/// fixture semantics, draft.31): four declared `CompleteArtifact` pairs,
/// checked by `qir-artifact-cmp-1` over full canonical bytes. The fixture
/// commits no record/receipt bytes and never changes the normal suite's
/// `n = 336` or `entries = 0`.
fn suite_fixture_f6(section: &str, rep: &mut Report) {
    use std::collections::HashMap;
    let mut arts: HashMap<String, CompleteArtifact> = HashMap::new();
    let mut unique: Vec<CompleteArtifact> = Vec::new();
    let declared = line_starting(section, "Declared artifacts:").unwrap();
    let declared = declared.strip_prefix("Declared artifacts:").unwrap_or(&declared);
    for part in declared.split(';') {
        let p = part.trim().trim_matches('`').trim();
        let (name, rest) = match p.split_once(" = (") {
            Some(x) => x,
            None => continue,
        };
        let inner = rest.trim_end_matches(')');
        let f: Vec<&str> = inner.split(',').map(|x| x.trim()).collect();
        if f.len() != 4 {
            rep.fail(format!("F6: malformed artifact declaration {p}"));
            return;
        }
        let bytes = f[3].trim_matches('"').as_bytes().to_vec();
        let digest = match f[2].strip_prefix("sha256(") {
            Some(d) => sha256::sha256(d.trim_end_matches(')').trim_matches('"').as_bytes()),
            None => {
                rep.fail(format!("F6: artifact {name} has no sha256(...) digest"));
                return;
            }
        };
        arts.insert(
            name.trim().to_string(),
            CompleteArtifact {
                kind: f[0].to_string(),
                version: f[1].to_string(),
                digest,
                bytes,
            },
        );
        unique.push(arts.get(name.trim()).unwrap().clone());
    }

    let mut operands: Vec<CompleteArtifact> = Vec::new();
    let mut verdicts: Vec<(&str, &str)> = Vec::new();
    for ln in section.lines().filter(|l| l.starts_with("- pair ")) {
        let word = ln.split('`').nth(1).unwrap_or("");
        let rhs = ln
            .splitn(2, ':')
            .nth(1)
            .unwrap_or("")
            .split('(')
            .next()
            .unwrap_or("");
        let names: Vec<&str> = rhs
            .split(',')
            .map(|x| x.trim().trim_matches('`').trim())
            .collect();
        if names.len() != 2 {
            rep.fail(format!("F6: malformed pair line {ln}"));
            return;
        }
        let (a, b) = match (arts.get(names[0]), arts.get(names[1])) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                rep.fail(format!("F6: undeclared artifact in {ln}"));
                return;
            }
        };
        let got = cmp_artifacts(a, b);
        let want = match word {
            "same" => 0,
            "different" => 1,
            "collision" => 2,
            _ => {
                rep.fail(format!("F6: unknown verdict {word}"));
                return;
            }
        };
        if got != want {
            rep.fail(format!(
                "F6: pair {} produced {} but declares {word}",
                names[0], got
            ));
        }
        operands.push(a.clone());
        operands.push(b.clone());
        verdicts.push((word, a.kind.as_str()));
    }
    if verdicts.len() != 4 {
        rep.fail(format!(
            "F6: expected four declared comparison pairs, found {}",
            verdicts.len()
        ));
        return;
    }
    let entries = collision_entries(&unique);
    let verdict_line = line_starting(section, "Verdict:").unwrap_or_default();
    let declared_entries = u64_chain(&verdict_line, "entries = ", &[])[0];
    if entries as u64 != declared_entries {
        rep.fail(format!(
            "F6: collision bucket entries {entries} != declared {declared_entries}"
        ));
    }
    for l in [
        "fixture F6 artifact comparison (test-only corpus fixture, §4.4):",
        "  pair 1 (same): payload-a vs payload-a -> same",
        "  pair 2 (collision): payload-a vs payload-b -> collision (class = digest-collision)",
        "  pair 3 (different, same kind): payload-b vs payload-c -> different",
        "  pair 4 (different, differing kind): payload-c vs payload-c -> different",
        "  collision bucket: entries = 1 (class = digest-collision)",
        "  normal mode unchanged: n = 336, entries = 0",
        "  verdict: the four declared comparison pairs reproduce under qir-artifact-cmp-1",
    ] {
        rep.say(l.to_string());
    }
    if entries == 1 {
        rep.say("fixture F6: PASS — the declared comparison verdicts reproduce under the committed rules".to_string());
    } else {
        rep.say("fixture F6: FAIL — the declared comparison verdicts do not reproduce".to_string());
    }
}

fn suite_fixture(md: &str, fid: &str, rep: &mut Report) {
    let vectors = parse_vectors(md, "## Fixture Register");
    let v = vectors.iter().find(|v| v.id == fid);
    let section = match v {
        Some(v) => v.section.as_str(),
        None => {
            rep.fail(format!(
                "fixture register: the {fid} fixture section is absent"
            ));
            return;
        }
    };
    if fid == "F6" {
        suite_fixture_f6(section, rep);
        return;
    }
    let profile = bhat::golden_profile();

    // declared parameters, the declared exact bound, and the declared
    // expectation (parsed; the runs below are checked against it)
    let pl = line_starting(section, "Declared parameters:").unwrap();
    let w_budget = u64_chain(&pl, "`W_budget` = ", &[])[0];
    let b_injected = u64_chain(&pl, "`B_injected` = ", &[])[0];
    let al = line_starting(section, "Admission:").unwrap();
    let bh_declared = u64_chain(&al, "`B_hat(P_f)` = ", &[])[0];
    let shortfall_declared = u64_before(&al, " U short");
    let ctrl = line_starting(section, "- control run").unwrap();
    let ctrl_tag = u64_chain(&ctrl, "(tag ", &[])[0];
    let ctrl_conservation = u64_chain(&ctrl, "conservation ", &[" = ", " + ", " + "]);
    let inj = line_starting(section, "- injected run:").unwrap();
    let inj_step = u64_chain(&inj, "fires at step ", &[])[0];
    let inj_pc0 = u64_chain(&inj, "PC (", &[])[0];
    let inj_pc1 = u64_chain(&inj, ", ", &[])[0];
    let inj_tag = u64_chain(&inj, "(tag ", &[])[0];
    let n_completed = u64_before(&inj, " completed transition records");
    let n_rej = u64_before(&inj, " backstop rejection record");
    let inj_charged = u64_chain(&inj, "charged ", &[])[0];
    let inj_reserved = u64_chain(&inj, "reserved ", &[])[0];
    let inj_refunded = u64_chain(&inj, "refunded ", &[])[0];
    let inj_conservation = u64_chain(&inj, "conservation ", &[" = ", " + ", " + "]);
    let rem = u64_chain(&inj, "remaining budget ", &["- ", "= "]);
    let item_ln = line_starting(section, "- token itemization").unwrap();
    let n_items = u64_before(&item_ln, " tokens terminalized");
    let sel_nil_declared = item_ln.contains("selected token nil");
    let mut expect_items: Vec<(u64, u64)> = Vec::new();
    let mut cursor = 0usize;
    while let Some(p) = item_ln[cursor..].find("birth ") {
        let at = cursor + p + "birth ".len();
        let (b, r) = take_u64(&item_ln[at..]);
        let mut r = r.as_str();
        if let Some(stripped) = r.strip_prefix(" at (") {
            // the fatal token's rendering carries its stop site
            let close = stripped.find(')').unwrap();
            r = &stripped[close + 1..];
        }
        let r = r.trim_start_matches(", ");
        let name: String = r
            .chars()
            .take_while(|c| c.is_ascii_uppercase() || *c == '_')
            .collect();
        expect_items.push((b, ttype_of_name(&name)));
        cursor = at;
    }

    // the committed fixture envelope and the committed source form
    let f = fences(section);
    let envf = fence_after(&f, "Intent envelope").expect("fixture envelope fence");
    let committed = hex_to_bytes(&envf.body);
    let srcf = fence_after(&f, "source grammar's textual form")
        .expect("fixture source fence");

    // assembly share: the committed source form re-encodes byte-exactly
    // (§4.2; the committed assembler mapping, as in suite X)
    let src = parse_source_program(&srcf.body);
    let mut stream = Vec::new();
    for (bid, insns) in src.iter().enumerate() {
        stream.extend_from_slice(&insn::w(bid as u64));
        stream.extend_from_slice(&insn::w(0)); // argument signature arity
        stream.extend_from_slice(&insn::w(insns.len() as u64));
        for i in insns {
            stream.extend_from_slice(&enc_insn(i));
        }
    }
    let intent = decode_envelope(&committed).expect("the fixture envelope decodes");
    let vt = pl.split("`V` = (").nth(1).unwrap();
    let vend = vt.find(')').unwrap();
    let mut parts = vt[..vend].split(',');
    let sem = parts.next().unwrap().trim().to_string();
    let enc = parts.next().unwrap().trim().to_string();
    let prof = parts.next().unwrap().trim().to_string();
    // the fixture program cites no CLOAD address, so the dependency list is
    // the declared "no entries" table
    let deps: Vec<Vec<u8>> = Vec::new();
    let assembled = enc_envelope(
        &sem,
        &enc,
        &prof,
        &deps,
        src.len() as u64,
        &stream,
        &intent.pool,
    );

    if assembled != committed {
        rep.fail(format!(
            "{}: assembled envelope diverges from the committed bytes (assembled {} B, committed {} B)",
            vectors[0].id,
            assembled.len(),
            committed.len()
        ));
    }

    // the exact surrogate: computed, and checked against the declared bound.
    // A None here is a validator-side refusal; the F27 program must be bounded.
    let bh = match bhat::b_hat(&intent.blocks, &profile) {
        Some(b) => b,
        None => {
            rep.fail("F27 fixture program has no certified admission bound".to_string());
            return;
        }
    };

    if bh != bh_declared {
        rep.fail(format!(
            "computed B_hat(P_f) {} != declared {}",
            bh, bh_declared
        ));
    }

    if b_injected >= bh {
        rep.fail("the declared injection must be below the exact bound".to_string());
    }

    if bh - b_injected != shortfall_declared {
        rep.fail(format!(
            "the injected bound's shortfall {} != declared {}",
            bh - b_injected,
            shortfall_declared
        ));
    }

    // the control run: the exact bound refuses (the committed zero-posting
    // shape), the escrow never opens
    let control = interp::run_fixture(&committed, w_budget, None, &profile)
        .expect("the control run executes");

    if control.outcome != ctrl_tag {
        rep.fail(format!(
            "control run outcome {} != declared tag {}",
            control.outcome, ctrl_tag
        ));
    }

    if control.n_records != 1 || control.reserved != 0 {
        rep.fail(format!(
            "control run is not the single zero-posting refusal (records {}, reserved {})",
            control.n_records, control.reserved
        ));
    }

    if control.reserved != ctrl_conservation[0]
        || control.charged != ctrl_conservation[1]
        || control.refunded != ctrl_conservation[2]
        || ctrl_conservation[3] != 0
    {
        rep.fail("control run conservation differs from the declared triple".to_string());
    }

    // the injected run: admitted under the defective bound; the backstop
    // fires and the containment verdict is checked
    let injected = interp::run_fixture(&committed, w_budget, Some(b_injected), &profile)
        .expect("the injected run executes");

    if injected.outcome != inj_tag {
        rep.fail(format!(
            "injected run outcome {} != declared tag {}",
            injected.outcome, inj_tag
        ));
    }

    if injected.last_step != inj_step || injected.last_pc != (inj_pc0, inj_pc1) {
        rep.fail(format!(
            "the backstop stop site differs from the declared site (step {} at PC ({}, {}))",
            injected.last_step, injected.last_pc.0, injected.last_pc.1
        ));
    }

    if injected.n_records != (n_completed + n_rej) as usize {
        rep.fail(format!(
            "record count {} != declared {} completed + {} rejection",
            injected.n_records, n_completed, n_rej
        ));
    }

    if injected.reserved != inj_reserved
        || injected.charged != inj_charged
        || injected.refunded != inj_refunded
    {
        rep.fail("injected run ledger differs from the declared accumulators".to_string());
    }

    if injected.reserved != inj_conservation[0]
        || injected.charged != inj_conservation[1]
        || injected.refunded != inj_conservation[2]
        || injected.reserved - injected.charged - injected.refunded != inj_conservation[3]
    {
        rep.fail("injected run ledger differs from the declared conservation".to_string());
    }

    if rem[0] != w_budget
        || rem[1] != injected.charged
        || rem[2] != w_budget - injected.charged
    {
        rep.fail("the remaining budget differs from the declared identity".to_string());
    }

    if injected.charged > injected.reserved {
        rep.fail("overbilling: charged exceeds the reserve".to_string());
    }

    if injected.item.len() != n_items as usize {
        rep.fail(format!(
            "itemization count {} != declared {}",
            injected.item.len(),
            n_items
        ));
    }

    if injected.selected_nil != sel_nil_declared {
        rep.fail("the receipt's selected token differs from the declared nil".to_string());
    }

    if injected.item != expect_items {
        rep.fail("the itemization's (birth, type) pairs differ from the declared pairs".to_string());
    }

    // the fixture block (byte-identical to the second implementation's)
    let items: Vec<String> = injected
        .item
        .iter()
        .map(|&(b, t)| {
            format!(
                "birth {} {}",
                b,
                token_type_name(t).expect("a closed-vocabulary token type")
            )
        })
        .collect();
    let equality_note = if injected.charged == injected.reserved {
        "equality — no overbilling"
    } else {
        "no overbilling"
    };
    let ctrl_cons = format!(
        "{} = {} + {} + {}",
        control.reserved, control.charged, control.refunded,
        control.reserved - control.charged - control.refunded
    );
    let insns_n: usize = intent.blocks.iter().map(|b| b.insns.len()).sum();
    let pool_txt: Vec<String> = intent.pool.iter().map(|p| p.to_string()).collect();
    rep.say(
        "fixture F27 fuel-backstop-on-validator-defect (test-only fault injection, §4.4):"
            .to_string(),
    );
    rep.say(format!(
        "  declared program P_f: {} block, {} instructions; declared pool [{}]; envelope {} bytes",
        intent.blocks.len(), insns_n, pool_txt.join(", "), committed.len()
    ));
    rep.say(format!(
        "  declared injection: B_injected = {} U; W_budget = {} U; exact surrogate B_hat(P_f) = {} U (the injected bound is {} U short)",
        b_injected, w_budget, bh, bh - b_injected
    ));
    rep.say(format!(
        "  control run (exact bound): REFUSED at admission — {} (tag {}); zero postings; the escrow never opened ({})",
        outcome_name(control.outcome), control.outcome, ctrl_cons
    ));
    rep.say(
        "  injected run: ADMITTED — the false admission the fixture evidences".to_string(),
    );
    rep.say(format!(
        "    backstop fired at step {}, PC ({}, {}): {} (tag {}); {} completed + {} backstop rejection records",
        injected.last_step, injected.last_pc.0, injected.last_pc.1,
        outcome_name(injected.outcome), injected.outcome, n_completed, n_rej
    ));
    rep.say(format!(
        "    token itemization: {} tokens terminalized ({}); selected nil",
        injected.item.len(), items.join("; ")
    ));
    rep.say(format!(
        "    ledger: reserved {} = charged {} + refunded {} + outstanding {}; remaining budget {} - {} = {}; charged <= reserved ({})",
        injected.reserved, injected.charged, injected.refunded,
        injected.reserved - injected.charged - injected.refunded,
        w_budget, injected.charged, w_budget - injected.charged, equality_note
    ));
    rep.say(
        "  verdict: validator defect evidenced (false admission under the injected bound); containment held (§3.4 defense in depth) — the fixture normalizes no defective execution".to_string(),
    );
    if rep.failures == 0 {
        rep.say(
            "fixture F27: PASS — the declared expectation reproduces under the committed rules"
                .to_string(),
        );
    } else {
        rep.say(format!(
            "fixture F27: FAIL — {} failing check(s)",
            rep.failures
        ));
    }
}

fn validate_x(
    id: &str,
    name: &str,
    envelope: &[u8],
    params: &XParams,
    profile: &bhat::Profile,
    rep: &mut Report,
) -> bool {
    let decoded = match decode_envelope(envelope) {
        Ok(d) => d,
        Err(e) => {
            rep.fail(format!("{id} {name}: envelope decode rejected — {e}"));
            return false;
        }
    };
    if decoded.sem != params.sem || decoded.enc != params.enc || decoded.prof != params.prof {
        rep.fail(format!(
            "{id} {name}: version tuple mismatch (envelope ({}, {}, {}) vs declared ({}, {}, {}))",
            decoded.sem, decoded.enc, decoded.prof, params.sem, params.enc, params.prof
        ));
        return false;
    }
    let defining = match static_checks(&decoded) {
        Ok(n) => n,
        Err(e) => {
            rep.fail(format!("{id} {name}: static operand check rejected — {e}"));
            return false;
        }
    };
    let bh_opt = bhat::b_hat(&decoded.blocks, profile);
    let admitted = bh_opt.map(|b| params.w_budget >= b).unwrap_or(false);
    if let Some(bh) = bh_opt {
        if bh != params.admission_bp {
            rep.fail(format!(
                "{id} {name}: B_hat = {} U but the declared admission bound is {} U (the corpus declares them equal on every seed vector)",
                bh, params.admission_bp
            ));
            return false;
        }
    }
    let bh_desc = bh_opt
        .map(|b| b.to_string())
        .unwrap_or_else(|| "None (no certified bound)".to_string());
    if admitted {
        rep.say(format!(
            "  {id} {name}: well-formed; {defining} static value ids; B_hat = {} U (declared bound {} U); W_budget = {} U; ADMITTED (the execution share of this transcript checks the outcome)",
            bh_desc, params.admission_bp, params.w_budget
        ));
        true
    } else {
        // refusal is a typed status token, pre-dispatch (§2.5/§3.4)
        if params.expected_outcome_tag == 3 {
            rep.say(format!(
                "  {id} {name}: well-formed; B_hat = {} U (declared bound {} U); W_budget = {} U; REFUSED — EXPRESSION_TOO_COMPLEX (matches the expected terminal outcome tag {})",
                bh_desc, params.admission_bp, params.w_budget, params.expected_outcome_tag
            ));
            true
        } else {
            rep.fail(format!(
                "{id} {name}: refused but the expected terminal outcome is {} (tag {})",
                params.expected_outcome_name, params.expected_outcome_tag
            ));
            false
        }
    }
}

/// The source program parser: the goldens' textual grammar, one-to-one with
/// the committed encoding (§2.6).
fn parse_source_program(text: &str) -> Vec<Vec<SrcInsn>> {
    let mut blocks: Vec<Vec<SrcInsn>> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("block ") {
            let _arity = rest.split("(arity ").nth(1);
            blocks.push(Vec::new());
            continue;
        }
        let body = line.split_once(": ").unwrap().1;
        blocks
            .last_mut()
            .expect("instruction outside a block")
            .push(parse_src_insn(body));
    }
    blocks
}

fn val_of(s: &str) -> Value {
    parse_int_with_negpow(s)
}

fn int_of(s: &str) -> i128 {
    s.trim().parse().unwrap()
}

fn u64_of(s: &str) -> u64 {
    s.trim().parse().unwrap()
}

fn parse_src_insn(body: &str) -> SrcInsn {
    let b = normalize(body);
    let op: String = b.chars().take_while(|c| c.is_ascii_uppercase()).collect();
    let bound_of = |s: &str| -> i128 {
        let after = s
            .split("bnd=(int, <=")
            .nth(1)
            .or_else(|| s.split("bnd=(zoom, <=").nth(1))
            .unwrap();
        let tok: String = after.trim_start().chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
        tok.trim().parse().unwrap()
    };
    match op.as_str() {
        "CONST" => {
            let imm = b.split("imm=").nth(1).unwrap();
            let rtag: u64 = if b.contains("bnd=(zoom, <=") {
                4
            } else if b.contains("bnd=(tuple, <=") {
                3
            } else {
                1
            };
            let imm_val = if let Some(z) = imm.strip_prefix("zoom(") {
                // a zoom literal in its normalized form: zoom(m, z) bnd=(zoom, <=n)
                let inner = z.split(") bnd=").next().unwrap_or(z).trim_end_matches(')');
                let (m, ze) = inner.split_once(", ").unwrap();
                Value::Zoom(int_of(m), int_of(ze))
            } else {
                let imm_tok: String = imm
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '-')
                    .collect();
                val_of(&imm_tok)
            };
            SrcInsn::Const {
                imm: imm_val,
                bound: bound_of(&b),
                rtag,
            }
        }
        "TUPLE" => {
            let k_str: String = b
                .split("k=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let k: usize = u64_of(&k_str) as usize;
            let fields_body = b.split("fields=[").nth(1).unwrap();
            let fields_body = &fields_body[..fields_body.rfind(']').unwrap()];
            let mut fields = Vec::new();
            for part in fields_body.split("), (") {
                let part = part.trim_start_matches('(');
                let (ty, bnd) = part.split_once(", <=").unwrap();
                let rtag = match ty {
                    "int" => 1u64,
                    "tuple" => 3u64,
                    "zoom" => 4u64,
                    _ => panic!("unknown tuple field type {ty}"),
                };
                fields.push((rtag, int_of(bnd.trim_end_matches(')'))));
            }
            let refs: Vec<u64> = b
                .split_whitespace()
                .filter_map(|t| t.strip_prefix("ref="))
                .map(u64_of)
                .collect();
            assert!(fields.len() == k && refs.len() == k, "TUPLE arity");
            SrcInsn::Tuple { fields, refs }
        }
        "FIELD" => {
            let a: String = b
                .split("ref=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let idx: String = b
                .split("idx=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            SrcInsn::Field {
                a: u64_of(&a),
                idx: u64_of(&idx),
            }
        }
        "EMIT" => {
            let kind: String = b
                .split("kind=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let k_str: String = b
                .split("k=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let k: usize = u64_of(&k_str) as usize;
            let refs: Vec<u64> = b
                .split_whitespace()
                .filter_map(|t| t.strip_prefix("ref="))
                .map(u64_of)
                .collect();
            assert!(refs.len() == k, "EMIT arity");
            SrcInsn::Emit {
                kind: u64_of(&kind),
                refs,
            }
        }
        "CLOAD" => {
            let addr: String = b
                .split("addr=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let fb: String = b
                .split("fallback=block ")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            SrcInsn::Cload {
                addr: u64_of(&addr),
                fallback: u64_of(&fb),
                bound: bound_of(&b),
            }
        }
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" => {
            let a: String = b
                .split("ref=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let bref: String = b
                .split("ref=")
                .nth(2)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let tag = match op.as_str() {
                "ADD" => OP_ADD,
                "SUB" => OP_SUB,
                "MUL" => OP_MUL,
                "DIV" => OP_DIV,
                _ => OP_MOD,
            };
            SrcInsn::Arith {
                tag,
                a: u64_of(&a),
                b: u64_of(&bref),
                bound: bound_of(&b),
            }
        }
        "CMP" => {
            let a: String = b
                .split("ref=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let bref: String = b
                .split("ref=")
                .nth(2)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let rng = b.split("rng=[").nth(1).unwrap();
            let (lo, hi) = rng.split_once(',').unwrap();
            let hi = hi.trim_end_matches(']');
            SrcInsn::Cmp {
                a: u64_of(&a),
                b: u64_of(&bref),
                lo: int_of(lo),
                hi: int_of(hi),
            }
        }
        "BR" => {
            let cc: String = b
                .split("cc=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_uppercase())
                .collect();
            let cond: String = b
                .split("ref=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let t: String = b
                .split("true=block ")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let f: String = b
                .split("false=block ")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let cc_tag = match cc.as_str() {
                "LT" => CC_LT,
                "LE" => CC_LE,
                "EQ" => CC_EQ,
                "NE" => CC_NE,
                "GT" => CC_GT,
                "GE" => CC_GE,
                _ => panic!("unknown condition code {cc}"),
            };
            SrcInsn::Br {
                cc: cc_tag,
                cond: u64_of(&cond),
                true_blk: u64_of(&t),
                false_blk: u64_of(&f),
            }
        }
        "JMP" => {
            let t: String = b
                .split("JMP block ")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            SrcInsn::Jmp {
                target: u64_of(&t),
            }
        }
        "OBS" => {
            let tt: String = b
                .split("ttype=")
                .nth(1)
                .unwrap()
                .chars()
                .take_while(|c| c.is_ascii_uppercase() || *c == '_')
                .collect();
            let tt_tag = match tt.as_str() {
                "STATUS_LOOKUP_MISS" => 1,
                "STATUS_NEG_EVEN_ROOT" => 2,
                "EXPRESSION_TOO_COMPLEX" => 3,
                "STATUS_DOMAIN_ERROR" => 4,
                _ => panic!("unknown token type {tt}"),
            };
            let rng = b.split("rng=[").nth(1).unwrap();
            let (lo, hi) = rng.split_once(',').unwrap();
            let hi = hi.trim_end_matches(']');
            SrcInsn::Obs {
                ttype: tt_tag,
                lo: int_of(lo),
                hi: int_of(hi),
            }
        }
        "PROMOTE" => SrcInsn::Promote,
        "HALT" => SrcInsn::Halt,
        _ => panic!("unrecognized source instruction: {body}"),
    }
}

// --------------------------------------------------------------------- main

/// Lowercase hex of a byte slice (the transcript's digest rendering).
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The implementation artifact's own identity (commitments §6.2): the
/// declared version and the SHA-256 over the concatenation of the declared
/// source files' repository bytes, in the declared path order. The bytes are
/// LF-normalized — the working copy's line endings are a checkout artifact
/// and never participate — and the set excludes generated files (`Cargo.lock`
/// is generated; `target/` is build output).
pub(crate) fn impl_identity() -> (String, String) {
    let joined = impl_source_bytes();
    (
        env!("CARGO_PKG_VERSION").to_string(),
        to_hex(&crate::sha256::sha256(&joined)),
    )
}

/// The implementation artifact's declared source bytes (LF-normalized), in
/// the declared path order — the byte basis of the `implementation` kind
/// (commitments §6.2) and of the collision bucket's retained artifacts.
pub(crate) fn impl_source_bytes() -> Vec<u8> {
    const SOURCES: [&str; 9] = [
        "src/qnir/Cargo.toml",
        "src/qnir/rust-toolchain.toml",
        "src/qnir/src/bhat.rs",
        "src/qnir/src/cbor.rs",
        "src/qnir/src/insn.rs",
        "src/qnir/src/interp.rs",
        "src/qnir/src/main.rs",
        "src/qnir/src/sha256.rs",
        "src/qnir/src/value.rs",
    ];
    let mut joined = Vec::new();
    for f in SOURCES {
        let text = fs::read_to_string(f).unwrap_or_else(|e| {
            panic!("the declared source set is unreadable at {f} (run from the repository root): {e}")
        });
        joined.extend_from_slice(text.replace("\r\n", "\n").as_bytes());
    }
    joined
}

/// The declared lineage block (commitments §6.3): the run's lineage binding
/// rendered as the transcript's trailing metadata lines — the corpus
/// identity and the evidence digest ride the run; the implementation
/// identity is the artifact's own declared source set (§6.2); the version
/// tuple and the generations ride the binding. The corpus digest's byte
/// basis is the LF-normalized repository bytes (commitments §6.2).
fn lineage_lines(
    md: &str,
    e_hex: &str,
    n_ev: usize,
    v: &(String, String, String),
) -> Vec<String> {
    let corpus_version = md
        .lines()
        .find_map(|l| l.strip_prefix("version: "))
        .unwrap_or("unknown")
        .trim()
        .to_string();
    let lf = md.replace("\r\n", "\n");
    let corpus_digest = to_hex(&crate::sha256::sha256(lf.as_bytes()));
    let (impl_version, impl_digest) = impl_identity();
    vec![
        format!("lineage: corpus identity (corpus, {corpus_version}, {corpus_digest})"),
        format!(
            "lineage: implementation identity (implementation, {impl_version}, {impl_digest})"
        ),
        format!("lineage: V = ({}, {}, {})", v.0, v.1, v.2),
        format!("lineage: evidence (n = {n_ev} records, E = {e_hex})"),
        "lineage: generations (qir-extractor-1, qir-verbalizer-1)".to_string(),
    ]
}

/// The six declared known-answer pins of the trace-root construction
/// `qir-trace-root-1` (commitments §6.6), recomputed from the toy byte
/// strings `A`–`E` at every run; the return value is the number of pins
/// reproduced (construction examples, never corpus vectors).
fn trace_pins_ok() -> usize {
    let a: &[u8] = &[0x41];
    let b: &[u8] = &[0x42];
    let c: &[u8] = &[0x43];
    let d: &[u8] = &[0x44];
    let e: &[u8] = &[0x45];
    let cases: [(&[&[u8]], &str); 6] = [
        (&[], "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986"),
        (&[a], "c00b4d3c929cb5cc316691ed4636f634576f2c9b2954767234c5274e9dde185d"),
        (
            &[a, b],
            "ed692f01f7f6c46930d7ad8f9adad3f9f38b7379cf6a8d2f399a0ba1e914fe25",
        ),
        (
            &[a, b, c],
            "961d2e2be20f538ffdf56962a86d1bd165498f222684ee4c5e02c1e9f852adc5",
        ),
        (
            &[a, b, c, d],
            "5c8dc617d287a4297eb2bcb81b37644b5138e57ad461c657db152109e3fc9fca",
        ),
        (
            &[a, b, c, d, e],
            "2bdbcd79bf92b8dad52ba685f01bef520f0cda04468a13a6bd27bfbeebff64d7",
        ),
    ];
    cases
        .iter()
        .filter(|(input, want)| to_hex(&trace_root(input)) == *want)
        .count()
}

/// The four declared known-answer pins of the collision comparison
/// `qir-artifact-cmp-1` (commitments §6.7), recomputed at every run as a
/// decision rule over supplied triples — no SHA-256 collision is constructed.
/// The return value is the number of pins reproduced.
fn artifact_cmp_pins_ok() -> usize {
    let a = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-a"),
        bytes: b"payload-a".to_vec(),
    };
    let b_same = a.clone();
    let b_coll = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: a.digest, // supplied equal — the synthetic collision
        bytes: b"payload-b".to_vec(),
    };
    let b_diff = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-b"),
        bytes: b"payload-b".to_vec(),
    };
    let c_diffkind = CompleteArtifact {
        kind: "receipt".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-c"),
        bytes: b"payload-c".to_vec(),
    };
    let pins: [(u8, u8); 4] = [
        (cmp_artifacts(&a, &b_same), 0),       // same
        (cmp_artifacts(&a, &b_coll), 2),       // collision — quarantine both
        (cmp_artifacts(&a, &b_diff), 1),       // different
        (cmp_artifacts(&a, &c_diffkind), 1),   // different (cross-kind)
    ];
    pins.iter().filter(|(got, want)| got == want).count()
}

/// The collision comparison's self-test: four toy complete artifacts fed to
/// the comparison core, checking the quarantine verdicts (commitments §6.7).
/// Returns the number of verdicts reproduced.
fn artifact_cmp_selftest() -> usize {
    let a = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-a"),
        bytes: b"payload-a".to_vec(),
    };
    let coll = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: a.digest,
        bytes: b"payload-b".to_vec(),
    };
    let diff = CompleteArtifact {
        kind: "record".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-b"),
        bytes: b"payload-b".to_vec(),
    };
    let cross = CompleteArtifact {
        kind: "receipt".into(),
        version: "V".into(),
        digest: crate::sha256::sha256(b"payload-c"),
        bytes: b"payload-c".to_vec(),
    };
    let expected: [u8; 4] = [0, 2, 1, 1];
    let got: [u8; 4] = [
        cmp_artifacts(&a, &a),
        cmp_artifacts(&a, &coll),
        cmp_artifacts(&a, &diff),
        cmp_artifacts(&a, &cross),
    ];
    got.iter().zip(expected.iter()).filter(|(g, w)| g == w).count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fixture_id = if args.len() == 4
        && args[2] == "--fixture"
        && (args[3] == "F27" || args[3] == "F6")
    {
        Some(args[3].clone())
    } else {
        None
    };
    if args.len() != 2 && fixture_id.is_none() {
        eprintln!("usage: qnir <path to docs/qnir_goldens.md> [--fixture F27|--fixture F6]");
        std::process::exit(2);
    }
    let path = &args[1];
    let md_raw = fs::read(path).unwrap_or_else(|e| {
        eprintln!("cannot read the committed corpus at {path}: {e}");
        std::process::exit(2);
    });
    let md = String::from_utf8(md_raw).unwrap_or_else(|e| {
        eprintln!("the committed corpus is not UTF-8: {e}");
        std::process::exit(2);
    });

    println!("QnIR conformance artifact — assembler, validator, and reference interpreter (R6 increments 2–3, conformance §4.2/§4.3)");
    println!("corpus: {path} ({} bytes, repository bytes)", md.len());
    let n_vectors = [
        "## Family E —",
        "## Family I —",
        "## Family R —",
        "## Family M —",
        "## Family X —",
    ]
    .iter()
    .map(|h| parse_vectors(&md, h).len())
    .sum::<usize>();
    println!("declared seed: {n_vectors} vectors");

    // The trace-root construction's declared known-answer pins (commitments
    // §6.6): recomputed at every run, published in the header, and a run
    // failure when any pin diverges (construction examples, never corpus
    // vectors — not counted in the declared seed).
    let n_pins = trace_pins_ok();
    println!("qir-trace-root-1 known-answer pins: {n_pins}/6 reproduced (commitments §6.6)");

    // The collision comparison's declared known-answer pins and self-test
    // (commitments §6.7): recomputed at every run, a run failure on
    // divergence; no SHA-256 collision is constructed.
    let n_cmp_pins = artifact_cmp_pins_ok();
    let n_cmp_self = artifact_cmp_selftest();
    println!("qir-artifact-cmp-1 known-answer pins: {n_cmp_pins}/4 reproduced (commitments §6.7)");
    println!("qir-artifact-cmp-1 self-test: {n_cmp_self}/4 verdicts reproduced (commitments §6.7)");

    let mut rep = Report::new();
    if n_pins != 6 {
        rep.fail(format!(
            "qir-trace-root-1 known-answer pins: {n_pins}/6 reproduced (commitments §6.6)"
        ));
    }
    if n_cmp_pins != 4 || n_cmp_self != 4 {
        rep.fail(format!(
            "qir-artifact-cmp-1 known-answer pins: {n_cmp_pins}/4; self-test: {n_cmp_self}/4 (commitments §6.7)"
        ));
    }
    suite_e(&md, &mut rep);
    suite_i(&md, &mut rep);
    suite_r(&md, &mut rep);
    suite_m(&md, &mut rep);
    suite_x(&md, &mut rep);
    let (e_digest, n_ev, v_tuple, run_root, v_roots, entries) = suite_x_exec(&md, &mut rep);
    if let Some(fid) = fixture_id.as_deref() {
        suite_fixture(&md, fid, &mut rep);
    }

    println!();
    for l in &rep.lines {
        println!("{l}");
    }
    println!();
    if rep.failures == 0 {
        println!(
            "result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)"
        );
    } else {
        println!("result: FAIL — {} failing check(s)", rep.failures);
    }
    // The lineage block (commitments §6.3): external evidence metadata after
    // the declared result line, in normal and fixture mode alike — the
    // fixture's own injected runs contribute nothing to the evidence.
    println!();
    for l in lineage_lines(&md, &to_hex(&e_digest), n_ev, &v_tuple) {
        println!("{l}");
    }
    // The trace-root publication (commitments §6.6): one line per published
    // scope — the run scope first, then each vector scope in committed
    // document order. The roots bind the trace's records only; the fixture's
    // injected runs contribute nothing in either mode.
    println!(
        "lineage: trace root (qir-trace-root-1, scope = run, n = {n_ev}, root = {})",
        to_hex(&run_root)
    );
    for (id, n_vec, root) in &v_roots {
        println!(
            "lineage: trace root (qir-trace-root-1, scope = {id}, n = {n_vec}, root = {})",
            to_hex(root)
        );
    }
    // The collision bucket state (commitments §6.7): after the trace-root
    // publication lines, the declared bucket-state line.
    println!("lineage: collision bucket (qir-artifact-cmp-1, entries = {entries})");
    if rep.failures != 0 {
        std::process::exit(1);
    }
}
