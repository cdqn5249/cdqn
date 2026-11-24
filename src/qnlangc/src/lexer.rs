//! Lexer for the Rung-0 source surface (docs/qnlang.md §4). No comments and no
//! implicit tokens are admitted: an unrecognised character is a real error.

use crate::diag::Span;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok {
    Ident(String),
    Num(String),
    Arrow,       // ->
    BangBracket, // ![
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LT,
    GT,
    Comma,
    Semi,
    Colon,
    Eq,
    Ne,
    HashNe,
    Le,
    Ge,
    And,
    Or,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Question,
    Tilde,
    At,
    Range,     // ..
    RangeExcl, // ..<
    Bang,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tok: Tok,
    pub span: Span,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct LexError {
    pub span: Span,
    pub message: String,
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic()
}
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut col = 1usize;

    while i < chars.len() {
        let c = chars[i];
        if c == '\n' {
            line += 1;
            col = 1;
            i += 1;
            continue;
        }
        if c == '\r' || c == ' ' || c == '\t' {
            col += 1;
            i += 1;
            continue;
        }
        let start = Span { line, col };
        let peek = |n: usize| chars.get(i + n).copied();

        if is_letter(c) {
            let s = i;
            while i < chars.len() && (is_letter(chars[i]) || is_digit(chars[i]) || chars[i] == '_') {
                i += 1;
                col += 1;
            }
            let text: String = chars[s..i].iter().collect();
            out.push(Token { tok: Tok::Ident(text.clone()), span: start, text });
            continue;
        }
        if is_digit(c) {
            let s = i;
            while i < chars.len() && is_digit(chars[i]) {
                i += 1;
                col += 1;
            }
            let text: String = chars[s..i].iter().collect();
            out.push(Token { tok: Tok::Num(text.clone()), span: start, text });
            continue;
        }

        let (tok, len): (Tok, usize) = match (c, peek(1), peek(2)) {
            ('.', Some('.'), Some('<')) => (Tok::RangeExcl, 3),
            ('.', Some('.'), _) => (Tok::Range, 2),
            ('-', Some('>'), _) => (Tok::Arrow, 2),
            ('!', Some('['), _) => (Tok::BangBracket, 2),
            ('!', Some('='), _) => (Tok::Ne, 2),
            ('~', Some('='), _) => (Tok::HashNe, 2),
            ('<', Some('='), _) => (Tok::Le, 2),
            ('>', Some('='), _) => (Tok::Ge, 2),
            ('&', Some('&'), _) => (Tok::And, 2),
            ('|', Some('|'), _) => (Tok::Or, 2),
            ('{', _, _) => (Tok::LBrace, 1),
            ('}', _, _) => (Tok::RBrace, 1),
            ('(', _, _) => (Tok::LParen, 1),
            (')', _, _) => (Tok::RParen, 1),
            ('[', _, _) => (Tok::LBracket, 1),
            (']', _, _) => (Tok::RBracket, 1),
            ('<', _, _) => (Tok::LT, 1),
            ('>', _, _) => (Tok::GT, 1),
            (',', _, _) => (Tok::Comma, 1),
            (';', _, _) => (Tok::Semi, 1),
            (':', _, _) => (Tok::Colon, 1),
            ('=', _, _) => (Tok::Eq, 1),
            ('+', _, _) => (Tok::Plus, 1),
            ('-', _, _) => (Tok::Minus, 1),
            ('*', _, _) => (Tok::Star, 1),
            ('/', _, _) => (Tok::Slash, 1),
            ('.', _, _) => (Tok::Dot, 1),
            ('?', _, _) => (Tok::Question, 1),
            ('~', _, _) => (Tok::Tilde, 1),
            ('@', _, _) => (Tok::At, 1),
            ('!', _, _) => (Tok::Bang, 1),
            _ => {
                return Err(LexError {
                    span: start,
                    message: format!("unadmitted source character {:?}", c),
                })
            }
        };
        let text: String = chars[i..i + len].iter().collect();
        out.push(Token { tok, span: start, text });
        i += len;
        col += len;
    }
    Ok(out)
}
