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
use std::fs;
use value::{enc_value, Value};

pub(crate) use interp::suite_x_exec;

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
    let bh = bhat::b_hat(&decoded.blocks, profile);
    let admitted = params.w_budget >= bh;
    if bh != params.admission_bp {
        rep.fail(format!(
            "{id} {name}: B_hat = {} U but the declared admission bound is {} U (the corpus declares them equal on every seed vector)",
            bh, params.admission_bp
        ));
        return false;
    }
    if admitted {
        rep.say(format!(
            "  {id} {name}: well-formed; {defining} static value ids; B_hat = {} U (declared bound {} U); W_budget = {} U; ADMITTED (the execution share of this transcript checks the outcome)",
            bh, params.admission_bp, params.w_budget
        ));
        true
    } else {
        // refusal is a typed status token, pre-dispatch (§2.5/§3.4)
        if params.expected_outcome_tag == 3 {
            rep.say(format!(
                "  {id} {name}: well-formed; B_hat = {} U (declared bound {} U); W_budget = {} U; REFUSED — EXPRESSION_TOO_COMPLEX (matches the expected terminal outcome tag {})",
                bh, params.admission_bp, params.w_budget, params.expected_outcome_tag
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
        let after = s.split("bnd=(int, <=").nth(1).unwrap();
        let tok: String = after.trim_start().chars().take_while(|c| c.is_ascii_digit() || *c == '-').collect();
        tok.trim().parse().unwrap()
    };
    match op.as_str() {
        "CONST" => {
            let imm = b.split("imm=").nth(1).unwrap();
            let imm_tok: String = imm
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '-')
                .collect();
            SrcInsn::Const {
                imm: val_of(&imm_tok),
                bound: bound_of(&b),
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: qnir <path to docs/qnir_goldens.md>");
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
    let n_vectors = ["## Family E —", "## Family I —", "## Family M —", "## Family X —"]
        .iter()
        .map(|h| parse_vectors(&md, h).len())
        .sum::<usize>();
    println!("declared seed: {n_vectors} vectors");

    let mut rep = Report::new();
    suite_e(&md, &mut rep);
    suite_i(&md, &mut rep);
    suite_m(&md, &mut rep);
    suite_x(&md, &mut rep);
    suite_x_exec(&md, &mut rep);

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
        std::process::exit(1);
    }
}
