//! Recursive-descent parser for the reconciled §4 EBNF (docs/qnlang.md
//! v0.1.0-draft.2): `program := { header_decl } { top_decl | stmt } ;`.
//! Braced declarations end in `}` with an optional `;`; `Q(falsify)` is a
//! top_decl per §5.2. There is no harness-only parsing mode.

use crate::ast::*;
use crate::diag::{Diag, Span};
use crate::lexer::{Tok, Token};

const BUILTINS_UNARY: &[&str] = &["reuse", "drop", "halt_with", "cut", "2:chirality"];
const BUILTINS_BINARY: &[&str] = &["metric_dot", "store", "2:cons", "3:cmp", "4:zoom_cmp"];
const STRUCTURED: &[&str] = &["2:split", "match", "fold", "loop", "load"];
const CTORS: &[&str] = &["arr_new", "tbl_new"];

pub struct Parser {
    toks: Vec<Token>,
    eof: Token,
    pos: usize,
    arena: Arena,
    depth: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        let eof = Token {
            tok: Tok::Ident(String::new()),
            span: Span { line: 0, col: 0 },
            text: String::new(),
        };
        Parser { toks, eof, pos: 0, arena: Arena::new(), depth: 0 }
    }

    fn span(&self) -> Span {
        self.toks.get(self.pos).map(|t| t.span).unwrap_or(Span { line: 0, col: 0 })
    }
    fn peek(&self) -> &Tok {
        &self.toks.get(self.pos).unwrap_or(&self.eof).tok
    }
    fn peek_n(&self, n: usize) -> Tok {
        self.toks.get(self.pos + n).map(|t| t.tok.clone()).unwrap_or(Tok::Ident(String::new()))
    }
    fn text(&self) -> String {
        self.toks.get(self.pos).map(|t| t.text.clone()).unwrap_or_default()
    }
    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        self.pos += 1;
        t
    }
    fn err(&self, code: &str, short: &str, expected: &str, remedy: &str) -> Diag {
        let actual = if self.pos < self.toks.len() { self.text() } else { "<eof>".to_string() };
        Diag::new(code, self.span(), short, expected, &actual, remedy)
    }
    fn expect(&mut self, t: Tok, expected: &str) -> Result<Token, Diag> {
        if *self.peek() == t {
            Ok(self.bump())
        } else {
            if self.peek() == &Tok::Eq && matches!(t, Tok::Colon | Tok::Semi) {
                return Err(self.err(
                    "E_OP_HYGIENE",
                    "`=` is exact equality only, never a declaration/assignment",
                    expected,
                    "use `:` to declare a type and `->` to bind a value",
                ));
            }
            Err(self.err("E_GRAMMAR", expected, expected, "supply the declared token"))
        }
    }
    fn expect_ident(&mut self) -> Result<String, Diag> {
        match self.peek().clone() {
            Tok::Ident(s) => {
                self.bump();
                Ok(s)
            }
            _ => Err(self.err("E_GRAMMAR", "identifier required", "ident", "name the binding explicitly")),
        }
    }
    fn expect_num(&mut self) -> Result<u64, Diag> {
        match self.peek().clone() {
            Tok::Num(s) => {
                self.bump();
                s.parse::<u64>().map_err(|_| self.err("E_GRAMMAR", "uint required", "uint", "use a declared uint"))
            }
            _ => Err(self.err("E_GRAMMAR", "uint required", "uint", "use a declared uint")),
        }
    }
    fn is_ident(&self, s: &str) -> bool {
        matches!(self.peek(), Tok::Ident(x) if x == s)
    }

    fn is_header_start(&self) -> bool {
        if !self.is_ident("Q") || self.peek_n(1) != Tok::LParen {
            return false;
        }
        matches!(self.peek_n(2), Tok::Ident(ref x) if matches!(x.as_str(), "dcc" | "kappa" | "domain" | "mode"))
    }
    fn is_top_start(&self) -> bool {
        if !self.is_ident("Q") || self.peek_n(1) != Tok::LParen {
            return false;
        }
        matches!(
            self.peek_n(2),
            Tok::Ident(ref x)
                if matches!(
                    x.as_str(),
                    "type" | "anchor" | "metric" | "blanket" | "variant" | "pattern"
                        | "workflow" | "conjecture" | "experiment" | "falsify"
                )
        )
    }

    pub fn parse_program(mut self) -> Result<Program, Diag> {
        let mut headers = Vec::new();
        while self.is_header_start() {
            headers.push(self.parse_header()?);
        }
        let mut tops = Vec::new();
        while self.pos < self.toks.len() {
            if self.is_top_start() {
                tops.push(self.parse_top()?);
            } else {
                // Reconciled §4: top_decl | stmt at program level.
                let st = self.parse_stmt()?;
                tops.push(TopDecl {
                    kind: TopKind::Conjecture { name: format!("$stmt{}", tops.len()), contract: String::new(), body: st },
                    span: self.span(),
                });
            }
        }
        Ok(Program {
            headers,
            tops,
            arena: self.arena.nodes,
            name_map: self.arena.name_map,
            use_name_map: false,
        })
    }

    fn parse_header(&mut self) -> Result<Header, Diag> {
        let span = self.span();
        self.expect(Tok::Ident("Q".into()), "\"Q\"")?;
        self.expect(Tok::LParen, "\"(\"")?;
        let name = self.expect_ident()?;
        self.expect(Tok::RParen, "\")\"")?;
        let kind = match name.as_str() {
            "dcc" => {
                let v = self.expect_ident()?;
                self.expect(Tok::Semi, "\";\"")?;
                HeaderKind::Dcc(v)
            }
            "kappa" => {
                let v = self.expect_ident()?;
                if !matches!(v.as_str(), "dyadic" | "dec" | "fact") {
                    return Err(self.err("E_GRAMMAR", "unadmitted kappa schedule", "dyadic | dec | fact", "declare a listed schedule"));
                }
                self.expect(Tok::Semi, "\";\"")?;
                HeaderKind::Kappa(v)
            }
            "domain" => {
                let v = if self.peek() == &Tok::LParen {
                    self.bump();
                    let mut xs = vec![self.expect_ident()?];
                    while self.peek() == &Tok::Comma {
                        self.bump();
                        xs.push(self.expect_ident()?);
                    }
                    self.expect(Tok::RParen, "\")\"")?;
                    xs
                } else {
                    vec![self.expect_ident()?]
                };
                self.expect(Tok::Semi, "\";\"")?;
                HeaderKind::Domain(v)
            }
            "mode" => {
                let v = self.expect_ident()?;
                if !matches!(v.as_str(), "L0" | "L1" | "L2" | "L3") {
                    return Err(self.err("E_GRAMMAR", "unadmitted mode", "L0 | L1 | L2 | L3", "declare a listed mode"));
                }
                self.expect(Tok::Semi, "\";\"")?;
                HeaderKind::Mode(v)
            }
            _ => return Err(self.err("E_GRAMMAR", "unknown header", "header_decl", "use dcc/kappa/domain/mode")),
        };
        Ok(Header { kind, span })
    }

    fn parse_top(&mut self) -> Result<TopDecl, Diag> {
        let span = self.span();
        self.expect(Tok::Ident("Q".into()), "\"Q\"")?;
        self.expect(Tok::LParen, "\"(\"")?;
        let name = self.expect_ident()?;
        self.expect(Tok::RParen, "\")\"")?;
        let kind = match name.as_str() {
            "type" => {
                let n = self.expect_ident()?;
                self.expect(Tok::Colon, "\":\"")?;
                let ty = self.parse_qtype()?;
                self.expect(Tok::Semi, "\";\"")?;
                TopKind::Type { name: n, ty }
            }
            "anchor" => {
                let n = self.expect_ident()?;
                self.expect(Tok::Colon, "\":\"")?;
                let (neg, core, unit) = self.parse_qprim()?;
                let c = if neg { format!("-{}", core) } else { core };
                let c = match unit { Some(u) => format!("{}<unit({})>", c, u), None => c };
                self.expect(Tok::Colon, "\":\"")?;
                let ty = self.parse_qtype()?;
                self.expect(Tok::Semi, "\";\"")?;
                TopKind::Anchor { name: n, core: c, ty }
            }
            "metric" => {
                let n = self.expect_ident()?;
                self.expect(Tok::Colon, "\":\"")?;
                let args = self.parse_eta()?;
                self.expect(Tok::Semi, "\";\"")?;
                TopKind::Metric { name: n, args }
            }
            "blanket" => {
                let n = self.expect_ident()?;
                self.expect(Tok::LBrace, "\"{\"")?;
                let mut params = Vec::new();
                for phase in ["sense", "state", "act"] {
                    self.expect(Tok::Ident(phase.into()), phase)?;
                    self.expect(Tok::Colon, "\":\"")?;
                    params.extend(self.parse_param_list()?);
                    self.expect(Tok::Semi, "\";\"")?;
                }
                self.expect(Tok::RBrace, "\"}\"")?;
                // Reconciled §4: every braced declaration ends at `}` with an
                // optional `;` — the Qbio blanket omits it.
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                TopKind::Blanket { name: n, params }
            }
            "variant" => {
                let n = self.expect_ident()?;
                self.expect(Tok::LBrace, "\"{\"")?;
                let mut arms = vec![self.parse_variant_arm()?];
                while self.peek() == &Tok::Comma {
                    self.bump();
                    arms.push(self.parse_variant_arm()?);
                }
                self.expect(Tok::RBrace, "\"}\"")?;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                TopKind::Variant { name: n, arms }
            }
            "pattern" | "workflow" => {
                let n = self.expect_ident()?;
                let params = self.parse_opt_params()?;
                let ret = if self.peek() == &Tok::Arrow {
                    self.bump();
                    self.parse_qtype_no_contract()?
                } else {
                    String::new()
                };
                let contract = self.parse_contract_tag()?;
                self.depth += 1;
                let body = self.parse_block()?;
                self.depth -= 1;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                if name == "pattern" {
                    TopKind::Pattern { name: n, params, ret, contract, body }
                } else {
                    TopKind::Workflow { name: n, params, ret, contract, body }
                }
            }
            "conjecture" | "falsify" => {
                // §5.2: Q(falsify) f(params) contract_tag block, now a §4 top_decl.
                let n = self.expect_ident()?;
                let _ = self.parse_opt_params()?;
                let contract = self.parse_contract_tag()?;
                self.depth += 1;
                let body = self.parse_block()?;
                self.depth -= 1;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                if name == "conjecture" {
                    TopKind::Conjecture { name: n, contract, body }
                } else {
                    TopKind::Falsify { name: n, contract, body }
                }
            }
            "experiment" => {
                let n = self.expect_ident()?;
                self.expect(Tok::Ident("for".into()), "\"for\"")?;
                let f = self.expect_ident()?;
                let contract = self.parse_contract_tag()?;
                self.expect(Tok::LBrace, "\"{\"")?;
                let mut cases = 0usize;
                while self.is_ident("case") {
                    self.bump();
                    let _ = self.expect_ident()?;
                    self.expect(Tok::Colon, "\":\"")?;
                    let _ = self.parse_expr()?;
                    self.expect(Tok::Arrow, "\"->\"")?;
                    let _ = self.expect_ident()?;
                    self.expect(Tok::Semi, "\";\"")?;
                    cases += 1;
                }
                self.expect(Tok::RBrace, "\"}\"")?;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                TopKind::Experiment { name: n, for_name: f, contract, cases }
            }
            _ => return Err(self.err("E_GRAMMAR", "unknown top declaration", "top_decl", "use a §4 top_decl form")),
        };
        Ok(TopDecl { kind, span })
    }

    fn parse_eta(&mut self) -> Result<String, Diag> {
        self.expect(Tok::Ident("eta".into()), "\"eta\"")?;
        self.expect(Tok::LParen, "\"(\"")?;
        let mut xs = Vec::new();
        for i in 0..3 {
            xs.push(self.parse_sint()?);
            if i < 2 {
                self.expect(Tok::Comma, "\",\"")?;
            }
        }
        self.expect(Tok::RParen, "\")\"")?;
        Ok(xs.join(","))
    }
    fn parse_sint(&mut self) -> Result<String, Diag> {
        let neg = if self.peek() == &Tok::Minus {
            self.bump();
            true
        } else {
            false
        };
        let n = self.expect_num()?;
        Ok(if neg { format!("-{}", n) } else { n.to_string() })
    }

    fn parse_opt_params(&mut self) -> Result<Vec<(String, String)>, Diag> {
        self.expect(Tok::LParen, "\"(\"")?;
        let params = if self.peek() == &Tok::RParen {
            Vec::new()
        } else {
            self.parse_param_list()?
        };
        self.expect(Tok::RParen, "\")\"")?;
        Ok(params)
    }

    fn parse_param_list(&mut self) -> Result<Vec<(String, String)>, Diag> {
        let mut out = Vec::new();
        loop {
            let n = self.expect_ident()?;
            self.expect(Tok::Colon, "\":\"")?;
            let ty = self.parse_qtype()?;
            out.push((n, ty));
            if self.peek() != &Tok::Comma {
                break;
            }
            self.bump();
        }
        Ok(out)
    }

    fn parse_variant_arm(&mut self) -> Result<String, Diag> {
        let n = self.expect_ident()?;
        if self.peek() == &Tok::LParen {
            let params = self.parse_opt_params()?;
            Ok(format!("{}({})", n, params.into_iter().map(|(a, b)| format!("{}:{}", a, b)).collect::<Vec<_>>().join(",")))
        } else {
            Ok(n)
        }
    }

    fn parse_contract_tag(&mut self) -> Result<String, Diag> {
        self.expect(Tok::BangBracket, "\"![\"")?;
        let mut items = Vec::new();
        loop {
            items.push(self.parse_contract_item()?);
            if self.peek() != &Tok::Comma {
                break;
            }
            self.bump();
        }
        self.expect(Tok::RBracket, "\"]\"")?;
        Ok(items.join(","))
    }

    fn parse_contract_item(&mut self) -> Result<String, Diag> {
        let name = self.expect_ident()?;
        match name.as_str() {
            "Qs" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let a = self.expect_ident()?;
                self.expect(Tok::Comma, "\",\"")?;
                let b = self.expect_ident()?;
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("Qs({},{})", a, b))
            }
            "Qphy" | "Qbio" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let a = self.expect_ident()?;
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("{}({})", name, a))
            }
            "Qchem" | "Qlog" => Ok(name),
            "budget" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let n = self.expect_num()?;
                let _ = self.expect_ident()?; // U
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("budget({} U)", n))
            }
            "leash" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let a = self.expect_ident()?;
                self.expect(Tok::Colon, "\":\"")?;
                let n = self.expect_num()?;
                let b = self.expect_ident()?;
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("leash({}:{} {})", a, n, b))
            }
            "metabolic_cap" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let a = self.expect_ident()?;
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("metabolic_cap({})", a))
            }
            "may_emit" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let mut xs = vec![self.expect_ident()?];
                while self.peek() == &Tok::Comma {
                    self.bump();
                    xs.push(self.expect_ident()?);
                }
                self.expect(Tok::RParen, "\")\"")?;
                Ok(format!("may_emit({})", xs.join(",")))
            }
            _ => Err(self.err("E_GRAMMAR", "unknown contract_item", "contract_item", "use a §4 contract item")),
        }
    }

    fn parse_unit_expr(&mut self) -> Result<String, Diag> {
        let mut parts = vec![self.expect_ident()?];
        while self.peek() == &Tok::Slash {
            self.bump();
            parts.push(self.expect_ident()?);
        }
        Ok(parts.join("/"))
    }

    fn parse_qtype(&mut self) -> Result<String, Diag> {
        self.parse_qtype_inner(true)
    }

    /// The declaration form `Q(pattern) ... -> qtype contract_tag block`
    /// needs the qtype without its optional contract suffix, otherwise the
    /// declaration's own `![...]` is consumed twice.
    fn parse_qtype_no_contract(&mut self) -> Result<String, Diag> {
        self.parse_qtype_inner(false)
    }

    fn parse_qtype_inner(&mut self, allow_contract: bool) -> Result<String, Diag> {
        let mut out = String::new();
        if self.is_ident("Qm") && self.peek_n(1) == Tok::LBracket {
            self.bump();
            self.bump();
            let lo = self.parse_sint()?;
            self.expect(Tok::Range, "\"..\"")?;
            let hi = self.parse_sint()?;
            self.expect(Tok::RBracket, "\"]\"")?;
            out = format!("Qm[{}..{}]", lo, hi);
        } else if self.is_ident("Qd") && self.peek_n(1) == Tok::LBracket {
            self.bump();
            self.bump();
            let lo = self.parse_sint()?;
            self.expect(Tok::Range, "\"..\"")?;
            let hi = self.parse_sint()?;
            self.expect(Tok::Comma, "\",\"")?;
            let d = self.parse_sint()?;
            self.expect(Tok::RBracket, "\"]\"")?;
            out = format!("Qd[{}..{},{}]", lo, hi, d);
        } else if self.is_ident("rat") && self.peek_n(1) == Tok::LBracket {
            self.bump();
            self.bump();
            let a = self.parse_sint()?;
            self.expect(Tok::Comma, "\",\"")?;
            let b = self.parse_sint()?;
            self.expect(Tok::RBracket, "\"]\"")?;
            out = format!("rat[{},{}]", a, b);
        } else if self.is_ident("Qleaf") && self.peek_n(1) == Tok::LT {
            self.bump();
            self.bump();
            self.expect(Tok::Ident("kappa".into()), "\"kappa\"")?;
            self.expect(Tok::LParen, "\"(\"")?;
            let k = self.expect_ident()?;
            self.expect(Tok::RParen, "\")\"")?;
            self.expect(Tok::Comma, "\",\"")?;
            self.expect(Tok::Ident("z".into()), "\"z\"")?;
            self.expect(Tok::LBracket, "\"[\"")?;
            let zl = self.parse_sint()?;
            self.expect(Tok::Range, "\"..\"")?;
            let zh = self.parse_sint()?;
            self.expect(Tok::RBracket, "\"]\"")?;
            self.expect(Tok::Comma, "\",\"")?;
            self.expect(Tok::Ident("q".into()), "\"q\"")?;
            self.expect(Tok::LBracket, "\"[\"")?;
            let ql = self.parse_sint()?;
            self.expect(Tok::Range, "\"..\"")?;
            let qh = self.parse_sint()?;
            self.expect(Tok::RBracket, "\"]\"")?;
            self.expect(Tok::Comma, "\",\"")?;
            self.expect(Tok::Ident("axis".into()), "\"axis\"")?;
            self.expect(Tok::LParen, "\"(\"")?;
            let ax = self.expect_ident()?;
            self.expect(Tok::RParen, "\")\"")?;
            self.expect(Tok::GT, "\">\"")?;
            out = format!("Qleaf<kappa({}),z[{}..{}],q[{}..{}],axis({})>", k, zl, zh, ql, qh, ax);
        } else if self.peek() == &Tok::Ident("Q".into())
            && self.peek_n(1) == Tok::LParen
            && matches!(self.peek_n(2), Tok::Ident(ref x) if matches!(x.as_str(), "tuple" | "array" | "table" | "blade"))
        {
            let head = self.parse_q_head()?;
            self.expect(Tok::LT, "\"<\"")?;
            let mut xs = Vec::new();
            if head == "blade" {
                self.expect(Tok::Ident("axes".into()), "\"axes\"")?;
                self.expect(Tok::LParen, "\"(\"")?;
                xs.push(format!("axes({})", self.expect_num()?));
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::Comma, "\",\"")?;
            }
            xs.push(self.parse_qtype()?);
            while self.peek() == &Tok::Comma {
                self.bump();
                if head == "array" || head == "table" {
                    if self.is_ident("cap") {
                        self.bump();
                        self.expect(Tok::LParen, "\"(\"")?;
                        let n = self.expect_num()?;
                        self.expect(Tok::RParen, "\")\"")?;
                        xs.push(format!("cap({})", n));
                        break;
                    }
                }
                xs.push(self.parse_qtype()?);
            }
            self.expect(Tok::GT, "\">\"")?;
            out = format!("Q({})<{}>", head, xs.join(","));
        } else if self.peek() == &Tok::Ident("Q".into()) && self.peek_n(1) == Tok::LParen {
            let (neg, core, unit) = self.parse_qprim()?;
            out = if neg { format!("-{}", core) } else { core };
            if let Some(u) = unit {
                out.push_str(&format!("<unit({})>", u));
            }
        } else if self.is_ident("Qphy") && self.peek_n(1) == Tok::LT {
            self.bump();
            self.bump();
            self.expect(Tok::Ident("unit".into()), "\"unit\"")?;
            self.expect(Tok::LParen, "\"(\"")?;
            let u = self.parse_unit_expr()?;
            self.expect(Tok::RParen, "\")\"")?;
            self.expect(Tok::Comma, "\",\"")?;
            let t = self.parse_qtype()?;
            self.expect(Tok::GT, "\">\"")?;
            out = format!("Qphy<unit({}),{}>", u, t);
        } else if self.is_ident("sim") && self.peek_n(1) == Tok::LT {
            self.bump();
            self.bump();
            let a = self.expect_ident()?;
            self.expect(Tok::Comma, "\",\"")?;
            let b = self.expect_ident()?;
            self.expect(Tok::GT, "\">\"")?;
            out = format!("sim<{},{}>", a, b);
        } else {
            let name = self.expect_ident()?;
            if self.peek() == &Tok::LT {
                self.bump();
                let mut xs = Vec::new();
                if name == "Q(tuple)" || name == "Q(array)" || name == "Q(table)" || name == "Q(blade)" {
                    // These names arrive split as Q ( tuple ) from the lexer only
                    // via parse_q_head; a plain identifier form is not expected.
                    return Err(self.err("E_GRAMMAR", "container qtype must use Q(...) form", "qtype", "use Q(array)<...> etc."));
                }
                xs.push(self.parse_qtype()?);
                while self.peek() == &Tok::Comma {
                    self.bump();
                    xs.push(self.parse_qtype()?);
                }
                self.expect(Tok::GT, "\">\"")?;
                out = format!("{}<{}>", name, xs.join(","));
            } else {
                out = name;
            }
        }

        // qtype "![" contract_item ... "]" suffix
        if allow_contract && self.peek() == &Tok::BangBracket {
            out.push_str("![");
            out.push_str(&self.parse_contract_tag()?);
            out.push(']');
        }
        Ok(out)
    }

    /// Consumes `Q ( inner )` and returns the inner name (`0`, `2:cons`, `arr_new`...).
    fn parse_q_head(&mut self) -> Result<String, Diag> {
        self.expect(Tok::Ident("Q".into()), "\"Q\"")?;
        self.expect(Tok::LParen, "\"(\"")?;
        let name = match self.peek().clone() {
            Tok::Num(n) => {
                self.bump();
                if self.peek() == &Tok::Colon {
                    self.bump();
                    let t = self.expect_ident()?;
                    format!("{}:{}", n, t)
                } else {
                    n
                }
            }
            Tok::Ident(s) => {
                self.bump();
                s
            }
            _ => return Err(self.err("E_GRAMMAR", "Q(...) inner required", "uint | ident", "name the anchor or builtin")),
        };
        self.expect(Tok::RParen, "\")\"")?;
        Ok(name)
    }

    fn parse_qprim(&mut self) -> Result<(bool, String, Option<String>), Diag> {
        let neg = if self.peek() == &Tok::Minus {
            self.bump();
            true
        } else {
            false
        };
        let core = self.parse_q_head()?;
        let unit = if self.peek() == &Tok::LT {
            self.bump();
            self.expect(Tok::Ident("unit".into()), "\"unit\"")?;
            self.expect(Tok::LParen, "\"(\"")?;
            let u = self.parse_unit_expr()?;
            self.expect(Tok::RParen, "\")\"")?;
            self.expect(Tok::GT, "\">\"")?;
            Some(u)
        } else {
            None
        };
        Ok((neg, core, unit))
    }

    fn parse_primary(&mut self) -> Result<NodeId, Diag> {
        let span = self.span();
        match self.peek().clone() {
            Tok::Num(_) => Err(Diag::new(
                "E_NO_NAKED_NUM",
                span,
                "naked numeric literal",
                "Q(k) | -Q(k)",
                &self.text(),
                "wrap the value in a declared Q(k) anchor",
            )),
            Tok::Minus
                if self.peek_n(1) == Tok::Ident("Q".into()) && self.peek_n(2) == Tok::LParen =>
            {
                let (neg, core, unit) = self.parse_qprim()?;
                Ok(self.arena.intern(NodeKind::QPrim { neg, core, unit }, span, self.depth))
            }
            Tok::Ident(ref s) if s == "Q" => {
                // Look ahead at the Q(...) head without consuming it.
                let save = self.pos;
                let head = self.parse_q_head()?;
                self.pos = save;
                if CTORS.contains(&head.as_str()) {
                    self.parse_ctor(head)
                } else if BUILTINS_UNARY.contains(&head.as_str()) || BUILTINS_BINARY.contains(&head.as_str()) {
                    self.parse_builtin_call(head)
                } else if STRUCTURED.contains(&head.as_str()) {
                    self.parse_structured(head)
                } else {
                    let (neg, core, unit) = self.parse_qprim()?;
                    Ok(self.arena.intern(NodeKind::QPrim { neg, core, unit }, span, self.depth))
                }
            }
            Tok::Ident(ref s) if s == "emit" => {
                self.bump();
                let n = self.expect_ident()?;
                Ok(self.arena.intern(NodeKind::Emit(n), span, self.depth))
            }
            Tok::Ident(s) => {
                self.bump();
                Ok(self.arena.intern(NodeKind::Ident(s), span, self.depth))
            }
            Tok::LParen => {
                self.bump();
                let mut xs = Vec::new();
                if self.peek() != &Tok::RParen {
                    xs.push(self.parse_expr()?);
                    while self.peek() == &Tok::Comma {
                        self.bump();
                        xs.push(self.parse_expr()?);
                    }
                }
                self.expect(Tok::RParen, "\")\"")?;
                if xs.len() == 1 {
                    Ok(xs[0])
                } else {
                    Ok(self.arena.intern(NodeKind::Tuple(xs), span, self.depth))
                }
            }
            _ => Err(self.err("E_GRAMMAR", "expression required", "primary", "use Q(k), an ident, a tuple or a builtin")),
        }
    }

    fn parse_builtin_call(&mut self, name: String) -> Result<NodeId, Diag> {
        let span = self.span();
        self.parse_q_head()?;
        self.expect(Tok::LParen, "\"(\"")?;
        let mut args = Vec::new();
        if self.peek() != &Tok::RParen {
            args.push(self.parse_expr()?);
            while self.peek() == &Tok::Comma {
                self.bump();
                args.push(self.parse_expr()?);
            }
        }
        self.expect(Tok::RParen, "\")\"")?;
        Ok(self.arena.intern(NodeKind::Call { name, args }, span, self.depth))
    }

    fn parse_ctor(&mut self, name: String) -> Result<NodeId, Diag> {
        let span = self.span();
        self.parse_q_head()?;
        self.expect(Tok::LT, "\"<\"")?;
        let mut tys = vec![self.parse_qtype()?];
        if name == "tbl_new" {
            self.expect(Tok::Comma, "\",\"")?;
            tys.push(self.parse_qtype()?);
        }
        self.expect(Tok::Comma, "\",\"")?;
        self.expect(Tok::Ident("cap".into()), "\"cap\"")?;
        self.expect(Tok::LParen, "\"(\"")?;
        let cap = self.expect_num()?;
        self.expect(Tok::RParen, "\")\"")?;
        self.expect(Tok::GT, "\">\"")?;
        Ok(self.arena.intern(NodeKind::Ctor { name, args: format!("{}|cap({})", tys.join(","), cap) }, span, self.depth))
    }

    fn parse_structured(&mut self, name: String) -> Result<NodeId, Diag> {
        let span = self.span();
        self.parse_q_head()?;
        let node = match name.as_str() {
            "2:split" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let scrut = self.parse_expr()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::LBrace, "\"{\"")?;
                self.expect(Tok::Ident("T".into()), "\"T\"")?;
                self.expect(Tok::Arrow, "\"->\"")?;
                let t = self.parse_block()?;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                self.expect(Tok::Ident("F".into()), "\"F\"")?;
                self.expect(Tok::Arrow, "\"->\"")?;
                let f = self.parse_block()?;
                if self.peek() == &Tok::Semi {
                    self.bump();
                }
                self.expect(Tok::RBrace, "\"}\"")?;
                NodeKind::Split { scrut, t, f }
            }
            "match" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let scrut = self.parse_expr()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::LBrace, "\"{\"")?;
                let mut arms = Vec::new();
                while self.peek() != &Tok::RBrace {
                    let id = self.expect_ident()?;
                    self.expect(Tok::Arrow, "\"->\"")?;
                    let e = self.parse_expr()?;
                    self.expect(Tok::Semi, "\";\"")?;
                    arms.push((id, e));
                }
                self.expect(Tok::RBrace, "\"}\"")?;
                NodeKind::Match { scrut, arms }
            }
            "fold" => {
                let seq = self.parse_postfix()?;
                self.expect(Tok::Ident("over".into()), "\"over\"")?;
                let over = self.parse_range()?;
                self.expect(Tok::Ident("init".into()), "\"init\"")?;
                self.expect(Tok::LParen, "\"(\"")?;
                let init = self.parse_expr()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::Ident("as".into()), "\"as\"")?;
                self.expect(Tok::LParen, "\"(\"")?;
                let mut binders = vec![self.expect_ident()?];
                while self.peek() == &Tok::Comma {
                    self.bump();
                    binders.push(self.expect_ident()?);
                }
                self.expect(Tok::RParen, "\")\"")?;
                self.depth += 1;
                let body = self.parse_block()?;
                self.depth -= 1;
                NodeKind::Fold { seq, over, init, binders, body }
            }
            "loop" => {
                self.expect(Tok::Ident("trip".into()), "\"trip\"")?;
                self.expect(Tok::LParen, "\"(\"")?;
                let trip = self.expect_num()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.depth += 1;
                let body = self.parse_block()?;
                self.depth -= 1;
                NodeKind::Loop { trip, body }
            }
            "load" => {
                self.expect(Tok::LParen, "\"(\"")?;
                let tbl = self.parse_expr()?;
                self.expect(Tok::Comma, "\",\"")?;
                let key = self.parse_expr()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::At, "\"@\"")?;
                self.expect(Tok::LBracket, "\"[\"")?;
                self.expect(Tok::Ident("trip".into()), "\"trip\"")?;
                self.expect(Tok::LParen, "\"(\"")?;
                let trip = self.expect_num()?;
                self.expect(Tok::RParen, "\")\"")?;
                self.expect(Tok::RBracket, "\"]\"")?;
                self.expect(Tok::Arrow, "\"->\"")?;
                let target = self.parse_target_pat()?;
                let or_else = if self.is_ident("or_else") {
                    self.bump();
                    Some(self.parse_block()?)
                } else {
                    None
                };
                NodeKind::Load { tbl, key, trip, target, or_else }
            }
            _ => unreachable!(),
        };
        Ok(self.arena.intern(node, span, self.depth))
    }

    fn parse_target_pat(&mut self) -> Result<String, Diag> {
        if self.peek() == &Tok::LParen {
            self.bump();
            let mut xs = vec![self.expect_ident()?];
            while self.peek() == &Tok::Comma {
                self.bump();
                xs.push(self.expect_ident()?);
            }
            self.expect(Tok::RParen, "\")\"")?;
            Ok(format!("({})", xs.join(",")))
        } else {
            self.expect_ident()
        }
    }

    fn parse_block(&mut self) -> Result<NodeId, Diag> {
        let span = self.span();
        self.expect(Tok::LBrace, "\"{\"")?;
        let mut stmts = Vec::new();
        while self.peek() != &Tok::RBrace && self.peek() != &Tok::Arrow {
            if self.pos >= self.toks.len() {
                return Err(self.err("E_GRAMMAR", "unterminated block", "\"}\"", "close the block"));
            }
            stmts.push(self.parse_stmt()?);
        }
        let tail = if self.peek() == &Tok::Arrow {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(Tok::RBrace, "\"}\"")?;
        Ok(self.arena.intern(NodeKind::Block { stmts, tail }, span, self.depth))
    }

    fn parse_stmt(&mut self) -> Result<NodeId, Diag> {
        let e = self.parse_expr()?;
        if self.peek() == &Tok::Arrow {
            self.bump();
            let target = self.parse_target_pat()?;
            let ty = if self.peek() == &Tok::Colon {
                self.bump();
                Some(self.parse_qtype()?)
            } else {
                None
            };
            self.expect(Tok::Semi, "\";\"")?;
            let span = self.arena.node(e).span;
            let id = self.arena.intern(NodeKind::Bind { target, ty, value: e }, span, self.depth);
            return Ok(id);
        }
        if self.peek() == &Tok::Eq {
            return Err(self.err(
                "E_OP_HYGIENE",
                "`=` is exact equality only and never an assignment",
                "-> for a binding or `=` inside binary only",
                "replace `=` with `->` and declare the type with `:`",
            ));
        }
        self.expect(Tok::Semi, "\";\"")?;
        Ok(e)
    }

    fn parse_expr(&mut self) -> Result<NodeId, Diag> {
        if self.peek() == &Tok::LBracket {
            self.parse_range()
        } else {
            self.parse_binary()
        }
    }

    fn parse_range(&mut self) -> Result<NodeId, Diag> {
        let span = self.span();
        self.expect(Tok::LBracket, "\"[\"")?;
        let lo = self.parse_expr()?;
        let excl = if self.peek() == &Tok::RangeExcl {
            self.bump();
            true
        } else {
            self.expect(Tok::Range, "\".. | ..<\"")?;
            false
        };
        let hi = self.parse_expr()?;
        self.expect(Tok::RBracket, "\"]\"")?;
        Ok(self.arena.intern(NodeKind::Range { lo, hi, excl }, span, self.depth))
    }

    fn parse_binary(&mut self) -> Result<NodeId, Diag> {
        let lhs = self.parse_unary()?;
        let op = match self.peek().clone() {
            Tok::Plus => Some("+"),
            Tok::Minus => Some("-"),
            Tok::Star => Some("*"),
            Tok::Eq => Some("="),
            Tok::Ne => Some("!="),
            Tok::HashNe => Some("~="),
            Tok::LT => Some("<"),
            Tok::Le => Some("<="),
            Tok::GT => Some(">"),
            Tok::Ge => Some(">="),
            Tok::And => Some("&&"),
            Tok::Or => Some("||"),
            _ => None,
        };
        if let Some(op) = op {
            let span = self.arena.node(lhs).span;
            self.bump();
            let rhs = self.parse_unary()?;
            Ok(self.arena.intern(NodeKind::Binary { op: op.into(), lhs, rhs }, span, self.depth))
        } else {
            Ok(lhs)
        }
    }

    fn parse_unary(&mut self) -> Result<NodeId, Diag> {
        let span = self.span();
        let is_q = self.peek() == &Tok::Minus
            && self.peek_n(1) == Tok::Ident("Q".into())
            && self.peek_n(2) == Tok::LParen;
        if self.peek() == &Tok::Minus && !is_q {
            self.bump();
            let inner = self.parse_unary()?;
            Ok(self.arena.intern(NodeKind::Call { name: "unary_minus".into(), args: vec![inner] }, span, self.depth))
        } else if self.peek() == &Tok::Tilde {
            self.bump();
            let inner = self.parse_unary()?;
            Ok(self.arena.intern(NodeKind::Call { name: "tilde".into(), args: vec![inner] }, span, self.depth))
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<NodeId, Diag> {
        let mut base = self.parse_primary()?;
        loop {
            if self.peek() == &Tok::Dot {
                let span = self.arena.node(base).span;
                self.bump();
                let mut op = match self.peek().clone() {
                    Tok::Num(n) => {
                        self.bump();
                        format!(".{}", n)
                    }
                    Tok::Ident(s) => {
                        self.bump();
                        let mut v = format!(".{}", s);
                        if self.peek() == &Tok::Question {
                            self.bump();
                            v.push('?');
                        }
                        v
                    }
                    _ => return Err(self.err("E_GRAMMAR", "postfix field required", "uint | ident | ident?", "name the field")),
                };
                op.insert(0, ' ');
                base = self.arena.intern(NodeKind::Postfix { base, ops: vec![op.trim().into()] }, span, self.depth);
            } else if self.peek() == &Tok::LParen {
                let span = self.arena.node(base).span;
                self.bump();
                let mut args = Vec::new();
                if self.peek() != &Tok::RParen {
                    args.push(self.parse_expr()?);
                    while self.peek() == &Tok::Comma {
                        self.bump();
                        args.push(self.parse_expr()?);
                    }
                }
                self.expect(Tok::RParen, "\")\"")?;
                args.insert(0, base);
                base = self.arena.intern(NodeKind::Call { name: "call".into(), args }, span, self.depth);
            } else {
                break;
            }
        }
        Ok(base)
    }
}

/// The single strict `program` entry point used by the CLI, the worked-program
/// tests and the negative tests alike — no harness-only parsing mode remains.
pub fn parse_source(src: &str) -> Result<Program, Diag> {
    let toks = crate::lexer::lex(src).map_err(|e| {
        Diag::new("E_LEX", e.span, &e.message, "admitted source character", "unrecognised character", "remove it")
    })?;
    Parser::new(toks).parse_program()
}
