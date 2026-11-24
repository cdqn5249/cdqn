//! R12' dual-channel diagnostics (docs/qnlang.md §6.3).
//! Channel A is the agent tuple; Channel B is the qir-verbalizer-1 rendering.

use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Clone, Debug)]
pub struct Diag {
    pub code: String,
    pub span: Span,
    pub term_id: String,
    pub expected: String,
    pub actual: String,
    pub remedy: String,
    pub receipt_ref: String,
    pub short: String,
}

impl Diag {
    pub fn new(
        code: impl Into<String>,
        span: Span,
        short: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
        remedy: impl Into<String>,
    ) -> Self {
        let code = code.into();
        let term_id = format!("qnlangc/{}", code);
        let receipt_ref = format!("receipt:qnlangc/{}/{}", code, span);
        Diag {
            code,
            span,
            term_id,
            expected: expected.into(),
            actual: actual.into(),
            remedy: remedy.into(),
            receipt_ref,
            short: short.into(),
        }
    }

    /// Channel A — the machine tuple `(code, span, term_id, expected, actual, remedy, receipt_ref)`.
    pub fn channel_a(&self) -> String {
        format!(
            "({}, {}, {}, {}, {}, {}, {})",
            self.code, self.span, self.term_id, self.expected, self.actual, self.remedy, self.receipt_ref
        )
    }

    /// Channel B — the qir-verbalizer-1 natural-language line.
    pub fn channel_b(&self, file: &str) -> String {
        format!(
            "qir-verbalizer-1: {} at {}:{}:{} — {}; expected {}, got {}; remedy {}",
            self.code, file, self.span.line, self.span.col, self.short,
            self.expected, self.actual, self.remedy
        )
    }

    pub fn as_json(&self, file: &str) -> String {
        format!(
            "{{\"code\":{},\"span\":{{\"line\":{},\"col\":{}}},\"term_id\":{},\"expected\":{},\"actual\":{},\"remedy\":{},\"receipt_ref\":{},\"channel_a\":{},\"channel_b\":{}}}",
            json_str(&self.code),
            self.span.line,
            self.span.col,
            json_str(&self.term_id),
            json_str(&self.expected),
            json_str(&self.actual),
            json_str(&self.remedy),
            json_str(&self.receipt_ref),
            json_str(&self.channel_a()),
            json_str(&self.channel_b(file)),
        )
    }
}

pub fn json_str(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
