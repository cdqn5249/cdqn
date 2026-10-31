//! The §2.6 instruction grammar, block stream, and Intent envelope — the
//! decoder (validator side) and the assembler (source form → canonical
//! encoding, §4.2).
//!
//! Closed tag assignments: opcodes 1–16, condition codes LT 0 … GE 5, token
//! types 1–4 (§2.5's closed vocabulary). Envelope keys are the closed 0–8.

use crate::cbor::{self, Cbor};
pub use crate::value::w;
use crate::value::{
    dec_value, enc_value, skip_value, Value, WordReader,
};

pub const OP_CONST: u64 = 1;
pub const OP_CLOAD: u64 = 2;
pub const OP_TUPLE: u64 = 3;
pub const OP_FIELD: u64 = 4;
pub const OP_ADD: u64 = 5;
pub const OP_SUB: u64 = 6;
pub const OP_MUL: u64 = 7;
pub const OP_DIV: u64 = 8;
pub const OP_MOD: u64 = 9;
pub const OP_CMP: u64 = 10;
pub const OP_BR: u64 = 11;
pub const OP_JMP: u64 = 12;
pub const OP_OBS: u64 = 13;
pub const OP_PROMOTE: u64 = 14;
pub const OP_EMIT: u64 = 15;
pub const OP_HALT: u64 = 16;

pub fn op_name(tag: u64) -> Option<&'static str> {
    Some(match tag {
        OP_CONST => "CONST",
        OP_CLOAD => "CLOAD",
        OP_TUPLE => "TUPLE",
        OP_FIELD => "FIELD",
        OP_ADD => "ADD",
        OP_SUB => "SUB",
        OP_MUL => "MUL",
        OP_DIV => "DIV",
        OP_MOD => "MOD",
        OP_CMP => "CMP",
        OP_BR => "BR",
        OP_JMP => "JMP",
        OP_OBS => "OBS",
        OP_PROMOTE => "PROMOTE",
        OP_EMIT => "EMIT",
        OP_HALT => "HALT",
        _ => return None,
    })
}

pub const CC_LT: u64 = 0;
pub const CC_LE: u64 = 1;
pub const CC_EQ: u64 = 2;
pub const CC_NE: u64 = 3;
pub const CC_GT: u64 = 4;
pub const CC_GE: u64 = 5;

pub fn cc_name(tag: u64) -> Option<&'static str> {
    Some(match tag {
        CC_LT => "LT",
        CC_LE => "LE",
        CC_EQ => "EQ",
        CC_NE => "NE",
        CC_GT => "GT",
        CC_GE => "GE",
        _ => return None,
    })
}

pub fn token_type_name(tag: u64) -> Option<&'static str> {
    Some(match tag {
        1 => "STATUS_LOOKUP_MISS",
        2 => "STATUS_NEG_EVEN_ROOT",
        3 => "EXPRESSION_TOO_COMPLEX",
        4 => "STATUS_DOMAIN_ERROR",
        _ => return None,
    })
}

/// Instructions not exercised by the seed corpus (§4.5 registers their
/// vectors as expansion increments); the assembler types them, it does not
/// guess their operand layouts.
pub const UNEXERCISED: [u64; 3] = [OP_TUPLE, OP_FIELD, OP_EMIT];

#[derive(Clone, Debug)]
pub struct Insn {
    pub tag: u64,
    pub ext: u64,
    /// Operand summary for the static checks (refs, targets, addr index).
    pub refs: Vec<u64>,
    pub targets: Vec<u64>,
    pub addr: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub id: u64,
    pub insns: Vec<Insn>,
}

#[derive(Clone, Debug)]
pub struct Intent {
    pub sem: String,
    pub enc: String,
    pub prof: String,
    pub deps: Vec<Vec<u8>>,
    pub blocks: Vec<Block>,
    /// Declared initial status pool (token type tags, declared order).
    pub pool: Vec<u64>,
    pub n_blocks: u64,
}

fn err(verdict: &str) -> String {
    verdict.to_string()
}

/// Decodes one instruction starting at the reader position; every declared
/// malformed surface of the instruction grammar is typed here.
pub fn dec_insn(r: &mut WordReader) -> Result<Insn, String> {
    let tag = r.next("instruction")?;
    let name = op_name(tag)
        .ok_or_else(|| err("unknown opcode tag"))?;
    if UNEXERCISED.contains(&tag) {
        return Err(err(&format!(
            "opcode {} is not exercised by the seed corpus",
            name
        )));
    }
    let mut refs = Vec::new();
    let mut targets = Vec::new();
    let mut addr = None;
    match tag {
        OP_CONST => {
            dec_value(r)?; // imm
            let rtag = r.next("instruction")?;
            if rtag != 1 && rtag != 3 {
                return Err(err("unknown result type tag"));
            }
            dec_value(r)?; // bound
        }
        OP_CLOAD => {
            let a = r.next("instruction")?;
            let fb = r.next("instruction")?;
            addr = Some(a);
            targets.push(fb);
            let rtag = r.next("instruction")?;
            if rtag != 1 && rtag != 3 {
                return Err(err("unknown result type tag"));
            }
            dec_value(r)?;
        }
        OP_ADD | OP_SUB | OP_MUL | OP_DIV | OP_MOD => {
            refs.push(r.next("instruction")?);
            refs.push(r.next("instruction")?);
            let rtag = r.next("instruction")?;
            if rtag != 1 && rtag != 3 {
                return Err(err("unknown result type tag"));
            }
            dec_value(r)?;
        }
        OP_CMP => {
            refs.push(r.next("instruction")?);
            refs.push(r.next("instruction")?);
            dec_value(r)?; // range lo
            dec_value(r)?; // range hi
        }
        OP_BR => {
            let c = r.next("instruction")?;
            if cc_name(c).is_none() {
                return Err(err("unknown condition code"));
            }
            refs.push(r.next("instruction")?);
            let (t, tr) = dec_edge(r)?;
            targets.push(t);
            let _ = tr;
            let (f, fr) = dec_edge(r)?;
            targets.push(f);
            let _ = fr;
        }
        OP_JMP => {
            let (t, _tr) = dec_edge(r)?;
            targets.push(t);
        }
        OP_OBS => {
            let t = r.next("instruction")?;
            if token_type_name(t).is_none() {
                return Err(err("unknown token type tag"));
            }
            dec_value(r)?;
            dec_value(r)?;
        }
        OP_PROMOTE | OP_HALT => {}
        _ => unreachable!(),
    }
    let ext = r.next("instruction")?;
    Ok(Insn {
        tag,
        ext,
        refs,
        targets,
        addr,
    })
}

/// edge = blk word, arity cnt word, then exactly arity ref words (§2.6).
fn dec_edge(r: &mut WordReader) -> Result<(u64, Vec<u64>), String> {
    let blk = r.next("instruction")?;
    let arity = r.take_count("edge")?;
    let mut refs = Vec::with_capacity(arity);
    for _ in 0..arity {
        refs.push(r.next("instruction")?);
    }
    Ok((blk, refs))
}

/// Decodes the instruction stream: per block its id, argument signature,
/// instruction count, and instructions in declared operand order (§2.6).
pub fn decode_stream(stream: &[u8], n_blocks: u64) -> Result<Vec<Block>, String> {
    let words = crate::value::words_of(stream)?;
    let mut r = WordReader::new(&words);
    let mut blocks = Vec::new();
    for _ in 0..n_blocks {
        let id = r.next("instruction")?;
        let arity = r.take_count("block argument signature")?;
        let mut arg_bnds = Vec::with_capacity(arity);
        for _ in 0..arity {
            let t = r.next("instruction")?;
            if t != 1 && t != 3 {
                return Err(err("unknown result type tag"));
            }
            skip_value(&mut r)?;
            arg_bnds.push(t);
        }
        let cnt = r.take_count("instruction count")?;
        let mut insns = Vec::with_capacity(cnt);
        for _ in 0..cnt {
            insns.push(dec_insn(&mut r)?);
        }
        blocks.push(Block {
            id,
            insns,
        });
    }
    if r.i != words.len() {
        return Err(err("trailing partial instruction"));
    }
    Ok(blocks)
}

// ------------------------------------------------------------- the assembler

/// A parsed source instruction (the goldens' textual grammar).
#[derive(Clone, Debug)]
pub enum SrcInsn {
    Const {
        imm: Value,
        bound: i128,
    },
    Cload {
        addr: u64,
        fallback: u64,
        bound: i128,
    },
    Arith {
        tag: u64,
        a: u64,
        b: u64,
        bound: i128,
    },
    Cmp {
        a: u64,
        b: u64,
        lo: i128,
        hi: i128,
    },
    Br {
        cc: u64,
        cond: u64,
        true_blk: u64,
        false_blk: u64,
    },
    Jmp {
        target: u64,
    },
    Obs {
        ttype: u64,
        lo: i128,
        hi: i128,
    },
    Promote,
    Halt,
}

fn enc_bound(bound: i128) -> Vec<u8> {
    // result declaration: the type-tag word (int = 1) followed by one bound
    // value encoding, an INT_POS upper bound (§2.6)
    let mut v = w(1);
    v.extend_from_slice(&enc_value(&Value::int(bound)));
    v
}

/// Encodes one source instruction: opcode tag word, declared operand fields
/// in declared order, one uniform trailing ext word (§2.6).
pub fn enc_insn(s: &SrcInsn) -> Vec<u8> {
    let (tag, mut body) = match s {
        SrcInsn::Const { imm, bound } => {
            let mut b = enc_value(imm);
            b.extend_from_slice(&enc_bound(*bound));
            (OP_CONST, b)
        }
        SrcInsn::Cload {
            addr,
            fallback,
            bound,
        } => {
            let mut b = w(*addr);
            b.extend_from_slice(&w(*fallback));
            b.extend_from_slice(&enc_bound(*bound));
            (OP_CLOAD, b)
        }
        SrcInsn::Arith { tag, a, b, bound } => {
            let mut bb = w(*a);
            bb.extend_from_slice(&w(*b));
            bb.extend_from_slice(&enc_bound(*bound));
            (*tag, bb)
        }
        SrcInsn::Cmp { a, b, lo, hi } => {
            let mut bb = w(*a);
            bb.extend_from_slice(&w(*b));
            bb.extend_from_slice(&enc_value(&Value::int(*lo)));
            bb.extend_from_slice(&enc_value(&Value::int(*hi)));
            (OP_CMP, bb)
        }
        SrcInsn::Br {
            cc,
            cond,
            true_blk,
            false_blk,
        } => {
            let mut bb = w(*cc);
            bb.extend_from_slice(&w(*cond));
            bb.extend_from_slice(&w(*true_blk));
            bb.extend_from_slice(&w(0)); // true edge arity 0
            bb.extend_from_slice(&w(*false_blk));
            bb.extend_from_slice(&w(0)); // false edge arity 0
            (OP_BR, bb)
        }
        SrcInsn::Jmp { target } => {
            let mut bb = w(*target);
            bb.extend_from_slice(&w(0)); // edge arity 0
            (OP_JMP, bb)
        }
        SrcInsn::Obs { ttype, lo, hi } => {
            let mut bb = w(*ttype);
            bb.extend_from_slice(&enc_value(&Value::int(*lo)));
            bb.extend_from_slice(&enc_value(&Value::int(*hi)));
            (OP_OBS, bb)
        }
        SrcInsn::Promote => (OP_PROMOTE, Vec::new()),
        SrcInsn::Halt => (OP_HALT, Vec::new()),
    };
    let mut out = w(tag);
    out.append(&mut body);
    out.extend_from_slice(&w(0)); // trailing ext word: no extension
    out
}

/// The Intent envelope: deterministic CBOR map, closed keys 0–8 (§2.6).
/// `deps` carries the declared content addresses (§2.1's D declarations);
/// `pool` carries the declared initial status pool's token type tags.
pub fn enc_envelope(
    sem: &str,
    enc_v: &str,
    prof: &str,
    deps: &[Vec<u8>],
    n_blocks: u64,
    stream: &[u8],
    pool: &[u64],
) -> Vec<u8> {
    let entries = vec![
        (Cbor::UInt(0), Cbor::Bytes(b"QnIR-Intent".to_vec())),
        (Cbor::UInt(1), Cbor::Text(sem.to_string())),
        (Cbor::UInt(2), Cbor::Text(enc_v.to_string())),
        (Cbor::UInt(3), Cbor::Text(prof.to_string())),
        (
            Cbor::UInt(4),
            Cbor::Array(deps.iter().map(|d| Cbor::Bytes(d.clone())).collect()),
        ),
        (Cbor::UInt(5), Cbor::UInt(n_blocks)),
        (Cbor::UInt(6), Cbor::Bytes(stream.to_vec())),
        (Cbor::UInt(7), Cbor::Array(Vec::new())), // EXT side table (empty)
        (
            Cbor::UInt(8),
            Cbor::Array(pool.iter().map(|t| Cbor::UInt(*t)).collect()),
        ),
    ];
    cbor::encode(&Cbor::Map(entries))
}

// ---------------------------------------------------- envelope decode + checks

/// Decodes an Intent envelope with every declared static surface typed:
/// unknown envelope key (M8), duplicate envelope key (M11, in cbor::decode),
/// missing profile reference (§2.1), unknown token type tag (M12), unknown
/// opcode tag (M1), unknown condition code (M7), trailing partial
/// instruction (M10), structural bounds (M5), and the dangling side-table
/// reference (M9, statically illegal per §2.1).
pub fn decode_envelope(bytes: &[u8]) -> Result<Intent, String> {
    let decoded = cbor::decode(bytes).map_err(|e| e.0)?;
    let entries = match decoded {
        Cbor::Map(entries) => entries,
        _ => return Err(err("the Intent envelope is not a map")),
    };
    let mut fields: [Option<Cbor>; 9] = Default::default();
    for (k, v) in entries {
        let ki = match k {
            Cbor::UInt(n) if n <= 8 => n as usize,
            _ => return Err(err("unknown envelope key")),
        };
        if fields[ki].is_some() {
            return Err(err("duplicate envelope key"));
        }
        fields[ki] = Some(v);
    }
    let domain = match fields[0].take() {
        Some(Cbor::Bytes(b)) if b == b"QnIR-Intent" => b,
        _ => return Err(err("unknown domain tag")),
    };
    let _ = domain;
    let sem = take_text(&mut fields[1], "semantics version")?;
    let enc_v = take_text(&mut fields[2], "encoding version")?;
    // a profile reference present is a static legality rule (§2.1)
    let prof = match fields[3].take() {
        Some(Cbor::Text(s)) => s,
        _ => return Err(err("missing profile reference")),
    };
    let deps = match fields[4].take() {
        Some(Cbor::Array(items)) => {
            let mut out = Vec::new();
            for it in items {
                match it {
                    Cbor::Bytes(b) => out.push(b),
                    _ => return Err(err("dependency entries must be byte strings")),
                }
            }
            out
        }
        _ => return Err(err("the dependency list must be an array")),
    };
    let n_blocks = match fields[5].take() {
        Some(Cbor::UInt(n)) => n,
        _ => return Err(err("the block count must be a word")),
    };
    let stream = match fields[6].take() {
        Some(Cbor::Bytes(b)) => b,
        _ => return Err(err("the instruction stream must be a byte string")),
    };
    let ext_table = match fields[7].take() {
        Some(Cbor::Array(items)) => items,
        _ => return Err(err("the EXT side table must be an array")),
    };
    let pool = match fields[8].take() {
        Some(Cbor::Array(items)) => {
            let mut out = Vec::new();
            for it in items {
                match it {
                    Cbor::UInt(t) => {
                        // token type tags are the closed vocabulary of §2.5
                        if token_type_name(t).is_none() {
                            return Err(err("unknown token type tag"));
                        }
                        out.push(t);
                    }
                    _ => return Err(err("pool entries must be token type tag words")),
                }
            }
            out
        }
        _ => return Err(err("the declared initial status pool must be an array")),
    };
    let blocks = decode_stream(&stream, n_blocks)?;

    // every flag combination known and every EXT side-table reference
    // resolving (§2.1): an ext bit set beyond the side table is a dangling
    // reference — statically illegal (M9)
    for b in &blocks {
        for i in &b.insns {
            for bit in 0..64 {
                if i.ext & (1u64 << bit) != 0 && bit as usize >= ext_table.len() {
                    return Err(err("dangling side-table reference"));
                }
            }
        }
    }
    Ok(Intent {
        sem,
        enc: enc_v,
        prof,
        deps,
        blocks,
        pool,
        n_blocks,
    })
}

fn take_text(f: &mut Option<Cbor>, what: &str) -> Result<String, String> {
    match f.take() {
        Some(Cbor::Text(s)) => Ok(s),
        _ => Err(err(&format!("the {} must be text", what))),
    }
}

/// The validator's static operand checks (§2.1–§2.3 static legality, §3.2
/// dispatch-time bounds): reference ids dense over the Intent's
/// value-defining instructions (P10), cited addresses present in the
/// declared dependency list, and control targets inside the block graph.
pub fn static_checks(intent: &Intent) -> Result<usize, String> {
    // static value ids are dense words over the Intent's value-defining
    // instructions in block-then-index scan order (§2.6, parameter P10)
    let defining: [u64; 11] = [
        OP_CONST, OP_CLOAD, OP_TUPLE, OP_FIELD, OP_ADD, OP_SUB, OP_MUL, OP_DIV, OP_MOD,
        OP_CMP, OP_OBS,
    ];
    let mut defining_count: usize = 0;
    for b in &intent.blocks {
        for i in &b.insns {
            if defining.contains(&i.tag) {
                defining_count += 1;
            }
        }
    }
    for b in &intent.blocks {
        for i in &b.insns {
            for ref_id in &i.refs {
                if *ref_id as usize >= defining_count {
                    return Err(format!(
                        "reference {} exceeds the declared static value ids",
                        ref_id
                    ));
                }
            }
            for t in &i.targets {
                if *t >= intent.n_blocks {
                    return Err(format!("target block {} is not declared", t));
                }
            }
            if let Some(a) = i.addr {
                if a as usize >= intent.deps.len() {
                    return Err(format!(
                        "address {} is not in the declared dependency list",
                        a
                    ));
                }
            }
        }
    }
    Ok(defining_count)
}
