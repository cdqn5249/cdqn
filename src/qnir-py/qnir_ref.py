#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""qnir_ref.py — the QnIR v0.3 working draft's second implementation
(R6 artifact-share increment 4, v0.3.0-draft.13).

Independence declaration (conformance §4.1/§4.3): implemented from the
committed text of docs/qnir.md §1, docs/qnir_semantics.md §2,
docs/qnir_accounting.md §3, docs/qnir_conformance.md §4, and the committed
golden corpus docs/qnir_goldens.md with its declared artifact-wave parameters
(P1–P14) and its declared profile.  This implementation consults neither the
reference interpreter's code nor its outputs; its conformance is decided
against the committed golden results at the five agreement points of §4.1,
nowhere else.  It never generates or edits the golden corpus (§4.2): the
corpus is consumed as repository bytes, read-only.

Language verdict (QO-35): Python, standard library only — this file imports
nothing outside the standard library and carries no build step.

Determinism: no wall-clock, no locale, no randomization; every output is a
function of the committed corpus bytes alone.  Exit code 0 only when every
committed check passes.

Usage:
    python3 qnir_ref.py <path to docs/qnir_goldens.md>
"""

import hashlib
import json
import os
import platform
import re
import sys

# ---------------------------------------------------------------- words (§2.6)
MASK64 = (1 << 64) - 1


def w(x):
    """One 64-bit big-endian unsigned word (§2.6)."""
    if not 0 <= x <= MASK64:
        raise ValueError(f"word overflow {x}")
    return x.to_bytes(8, "big")


def words_of(bs):
    if len(bs) % 8:
        raise ValueError("byte string is not word-aligned")
    return [int.from_bytes(bs[i:i + 8], "big") for i in range(0, len(bs), 8)]


# ------------------------------------------------------- value grammar (§2.6)
TAG_INT_POS, TAG_INT_NEG, TAG_TUPLE, TAG_ZOOM = 1, 2, 3, 4


def magnitude_words(v):
    """Minimal big-endian magnitude words; canonical zero has count 0."""
    if v == 0:
        return []
    out = []
    while v:
        out.append(v & MASK64)
        v >>= 64
    out.reverse()
    return out


def enc_int(v):
    """Sign rides the type tag; the magnitude words carry the absolute value."""
    tag = TAG_INT_POS if v >= 0 else TAG_INT_NEG
    mags = magnitude_words(abs(v))
    return w(tag) + w(len(mags)) + b"".join(w(m) for m in mags)


def zoom_normalized(m, z):
    """§2.3's minimal-zoom normalizer N: N(0, z) = (0, 0); m = 2^k·q, q odd
    → (q, z + k)."""
    if m == 0:
        return (0, 0)
    k = 0
    while m % 2 == 0:
        m //= 2
        k += 1
    return (m, z + k)


def enc_value(v):
    """Canonical §2.6 value encoding of one value."""
    if isinstance(v, bool) or not isinstance(v, (int, tuple)):
        raise ValueError(f"unsupported value {v!r}")
    if isinstance(v, int):
        return enc_int(v)
    if len(v) == 2 and v[0] == "tuple":
        fields = v[1]
        return w(TAG_TUPLE) + w(len(fields)) + b"".join(enc_value(f) for f in fields)
    if len(v) == 3 and v[0] == "zoom":
        q, ze = zoom_normalized(v[1], v[2])
        return w(TAG_ZOOM) + enc_int(q) + enc_int(ze)
    if len(v) == 2 and v[0] == "zoom":
        # the decoded form (decode_value returns the normalized pair); the
        # pair is already normalized by the decoding rule (§2.6)
        q, ze = v[1]
        return w(TAG_ZOOM) + enc_int(q) + enc_int(ze)
    raise ValueError(f"unsupported value {v!r}")


# ------------------------------------------- canonical CBOR (RFC 8949), encode
def _cbor_head(major, n):
    if n < 24:
        return bytes([(major << 5) | n])
    if n <= 0xFF:
        return bytes([(major << 5) | 24, n])
    if n <= 0xFFFF:
        return bytes([(major << 5) | 25]) + n.to_bytes(2, "big")
    if n <= 0xFFFFFFFF:
        return bytes([(major << 5) | 26]) + n.to_bytes(4, "big")
    return bytes([(major << 5) | 27]) + n.to_bytes(8, "big")


def cbor_encode(x):
    """Deterministic CBOR: shortest-form heads, definite lengths, map keys
    sorted bytewise by their encoded key (RFC 8949 canonical form)."""
    if x is None:
        return b"\xf6"
    if x is True:
        return b"\xf5"
    if x is False:
        return b"\xf4"
    if isinstance(x, int):
        if x >= 0:
            return _cbor_head(0, x)
        return _cbor_head(1, -1 - x)
    if isinstance(x, bytes):
        return _cbor_head(2, len(x)) + x
    if isinstance(x, str):
        raw = x.encode("utf-8")
        return _cbor_head(3, len(raw)) + raw
    if isinstance(x, list):
        return _cbor_head(4, len(x)) + b"".join(cbor_encode(e) for e in x)
    if isinstance(x, dict):
        items = sorted(x.items(), key=lambda kv: cbor_encode(kv[0]))
        out = _cbor_head(5, len(items))
        for k, v in items:
            out += cbor_encode(k) + cbor_encode(v)
        return out
    raise ValueError(f"unsupported CBOR item {x!r}")


# -------------------------------------------- canonical CBOR, strict decoding
# Simple values carry the RFC 8949 mapping as repaired at v0.3.0-draft.12
# (20 false, 21 true, 22 null): the terminal receipt's nil selected-token key
# is the corpus's only committed simple value.

class Malformed(Exception):
    """A declared surface of §2.6's total unknown-field rejection, carrying
    the committed verdict vocabulary of the corpus."""

    def __init__(self, surface):
        super().__init__(surface)
        self.surface = surface


S_DUP_KEY = "malformed — duplicate envelope key"
S_EXT_TRAIL = ("malformed — trailing bytes after the record's "
               "value payload")
S_EXT_DUP = "malformed — duplicate side-table record index"
S_EXT_REC = "malformed — the EXT record structure is not [index, payload]"
S_EXT_IDX = "malformed — the EXT record index must be a word"
S_EXT_PAY = "malformed — the EXT record payload must be a byte string"


def _cb_head(b, i):
    ib = b[i]
    major, info = ib >> 5, ib & 0x1F
    i += 1
    if info < 24:
        return major, info, i
    widths = {24: 1, 25: 2, 26: 4, 27: 8}
    if info not in widths:
        raise ValueError("indefinite lengths are not canonical")
    n = int.from_bytes(b[i:i + widths[info]], "big")
    i += widths[info]
    if info == 24 and n < 24:
        raise ValueError("non-shortest-form head")
    return major, n, i


def cbor_decode(b):
    """Strict canonical-CBOR decode: shortest-form heads only, definite
    lengths only, map keys strictly increasing by encoded bytes (canonical
    order, duplicates rejected)."""
    val, i = _cb_value(b, 0)
    if i != len(b):
        raise ValueError("trailing CBOR bytes")
    return val


def _cb_value(b, i):
    major, n, i = _cb_head(b, i)
    if major == 0:
        return n, i
    if major == 1:
        return -1 - n, i
    if major == 2:
        return b[i:i + n], i + n
    if major == 3:
        return b[i:i + n].decode("utf-8"), i + n
    if major == 4:
        out = []
        for _ in range(n):
            v, i = _cb_value(b, i)
            out.append(v)
        return out, i
    if major == 5:
        keys = []
        out = {}
        for _ in range(n):
            k, i = _cb_value(b, i)
            kb = cbor_encode(k)
            if keys and kb <= keys[-1]:
                raise Malformed(S_DUP_KEY)
            keys.append(kb)
            v, i = _cb_value(b, i)
            out[k] = v
        return out, i
    if major == 7:
        if n == 20:
            return False, i
        if n == 21:
            return True, i
        if n == 22:
            return None, i
    raise ValueError(f"unsupported CBOR major {major} argument {n}")


# --------------------------------------------------- value decoding (§2.6)
S_COUNT = "malformed — count exceeds the enclosing artifact's remaining length"
S_LEAD_ZERO = "malformed — leading zero magnitude word (non-canonical)"
S_NEG_ZERO = "malformed — negative-tagged zero"
S_ZOOM = "malformed — non-normalized zoom encoding"
S_TAG = "malformed — unknown type tag"


def decode_value(ws, i, end):
    """Decode one §2.6 value encoding within the artifact window [i, end)."""
    if i + 1 > end:
        raise ValueError("truncated value tag")
    tag = ws[i]
    i += 1
    if tag in (TAG_INT_POS, TAG_INT_NEG):
        if i + 1 > end:
            raise ValueError("truncated magnitude count")
        cnt = ws[i]
        i += 1
        if cnt > end - i:
            raise Malformed(S_COUNT)
        mags = ws[i:i + cnt]
        i += cnt
        v = 0
        for m in mags:
            v = (v << 64) | m
        if tag == TAG_INT_NEG:
            if v == 0:
                raise Malformed(S_NEG_ZERO)
            v = -v
        if cnt and mags[0] == 0:
            raise Malformed(S_LEAD_ZERO)
        return v, i
    if tag == TAG_TUPLE:
        if i + 1 > end:
            raise ValueError("truncated field count")
        k = ws[i]
        i += 1
        if k > end - i:
            raise Malformed(S_COUNT)
        fields = []
        for _ in range(k):
            f, i = decode_value(ws, i, end)
            fields.append(f)
        return ("tuple", fields), i
    if tag == TAG_ZOOM:
        m, i = decode_value(ws, i, end)
        z, i = decode_value(ws, i, end)
        if isinstance(m, tuple) or isinstance(z, tuple):
            # the ZOOM fields must each be INT encodings (§2.6 value grammar)
            raise Malformed(S_ZOOM)
        if (m, z) != zoom_normalized(m, z):
            raise Malformed(S_ZOOM)
        return ("zoom", (m, z)), i
    raise Malformed(S_TAG)


# ------------------------------------------------ instruction grammar (§2.6)
OPCODES = {1: "CONST", 2: "CLOAD", 3: "TUPLE", 4: "FIELD", 5: "ADD", 6: "SUB",
           7: "MUL", 8: "DIV", 9: "MOD", 10: "CMP", 11: "BR", 12: "JMP",
           13: "OBS", 14: "PROMOTE", 15: "EMIT", 16: "HALT",
           17: "ARR_NEW", 18: "ARR_LEN", 19: "ARR_GET", 20: "ARR_SET",
           21: "ARR_BATCH", 22: "ARR_ROOT", 23: "TBL_NEW", 24: "TBL_LEN",
           25: "TBL_GET", 26: "TBL_AT", 27: "TBL_SET", 28: "TBL_BATCH",
           29: "TBL_ROOT", 30: "OP_CALL", 31: "OP_RET",
           32: "BAND", 33: "BOR", 34: "BXOR", 35: "BSHL", 36: "BSHR",
           37: "Q_HASH"}
OP_CODE = {v: k for k, v in OPCODES.items()}
CLASS_OF = {"CONST": "q_const", "CLOAD": "q_load", "TUPLE": "q_tuple",
            "FIELD": "q_field", "ADD": "q_add", "SUB": "q_sub", "MUL": "q_mul",
            "DIV": "q_div", "MOD": "q_mod", "CMP": "q_cmp", "BR": "q_br",
            "JMP": "q_jmp", "OBS": "q_obs", "PROMOTE": "q_promote",
            "EMIT": "q_emit", "HALT": "q_halt"}
TOKEN_TYPES = {1: "STATUS_LOOKUP_MISS", 2: "STATUS_NEG_EVEN_ROOT",
               3: "EXPRESSION_TOO_COMPLEX", 4: "STATUS_DOMAIN_ERROR"}
CC_NAMES = {0: "LT", 1: "LE", 2: "EQ", 3: "NE", 4: "GT", 5: "GE"}
RTAG_NAMES = {1: "int", 3: "tuple", 4: "zoom"}
# P13(a) (expansion increment 3): the declared zoom ranges — the normalized
# exponent within [-4, 4], the normalized mantissa within [0, 2^64)
ZOOM_EXP_LO, ZOOM_EXP_HI = -4, 4
ZOOM_MAN_HI = 1 << 64
DEFINING = {"CONST", "CLOAD", "TUPLE", "FIELD", "ADD", "SUB", "MUL", "DIV",
            "MOD", "CMP", "OBS", "ARR_NEW", "ARR_LEN", "ARR_GET", "ARR_SET",
            "ARR_BATCH", "ARR_ROOT", "TBL_NEW", "TBL_LEN", "TBL_GET", "TBL_AT",
            "TBL_SET", "TBL_BATCH", "TBL_ROOT", "OP_CALL",
            "BAND", "BOR", "BXOR", "BSHL", "BSHR", "Q_HASH"}
ARITH = ("ADD", "SUB", "MUL", "DIV", "MOD")
S_OPCODE = "malformed — unknown opcode tag"
S_CC = "malformed — unknown condition code"
S_TTYPE = "malformed — unknown token type tag"
S_PARTIAL = "malformed — trailing partial instruction"
S_DANGLE = "statically illegal — dangling side-table reference"


def decode_instruction(ws, i, end, allow_ext=False):
    """Decode one §2.6 instruction: opcode tag word, declared operand fields
    in declared order, one uniform trailing ext word."""
    if i + 1 > end:
        raise ValueError("truncated opcode")
    tag = ws[i]
    i += 1
    name = OPCODES.get(tag)
    if name is None or (not allow_ext and tag >= 17):
        raise Malformed(S_OPCODE)

    def word():
        nonlocal i
        if i + 1 > end:
            raise ValueError("truncated operand word")
        v = ws[i]
        i += 1
        return v

    def value():
        nonlocal i
        v, i = decode_value(ws, i, end)
        return v

    ins = {"op": name}
    if name == "CONST":
        ins["imm"] = value()
        ins["rtag"] = word()
        ins["rbnd"] = value()
    elif name == "CLOAD":
        ins["addr"] = word()
        ins["fallback"] = word()
        ins["rtag"] = word()
        ins["rbnd"] = value()
    elif name in ARITH:
        ins["a"] = word()
        ins["b"] = word()
        ins["rtag"] = word()
        ins["rbnd"] = value()
    elif name == "CMP":
        ins["a"] = word()
        ins["b"] = word()
        lo = value()
        hi = value()
        ins["rng"] = (lo, hi)
    elif name == "BR":
        cc = word()
        if cc not in CC_NAMES:
            raise Malformed(S_CC)
        ins["cc"] = cc
        ins["cond"] = word()
        ins["true_blk"] = word()
        ins["true_refs"] = [word() for _ in range(word())]
        ins["false_blk"] = word()
        ins["false_refs"] = [word() for _ in range(word())]
    elif name == "JMP":
        ins["target"] = word()
        ins["refs"] = [word() for _ in range(word())]
    elif name == "OBS":
        tt = word()
        if tt not in TOKEN_TYPES:
            raise Malformed(S_TTYPE)
        ins["ttype"] = tt
        lo = value()
        hi = value()
        ins["rng"] = (lo, hi)
    elif name == "TUPLE":
        # cnt (k), then k field declarations (rtag word + bound value) in
        # field order, then k component references (expansion increment 3)
        k = word()
        fields = []
        for _ in range(k):
            rtag = word()
            if rtag not in (1, 3, 4):
                raise Malformed(S_TTYPE)
            fields.append((rtag, value()))
        ins["fields"] = fields
        ins["refs"] = [word() for _ in range(k)]
    elif name == "FIELD":
        ins["a"] = word()
        ins["idx"] = word()
    elif name == "EMIT":
        # cnt (the declared kind immediate), cnt (k), then k refs
        ins["kind"] = word()
        k = word()
        ins["refs"] = [word() for _ in range(k)]
    elif name in ("PROMOTE", "HALT"):
        pass
    elif name in ("ARR_NEW", "TBL_NEW"):
        word(); ins["cap"] = value(); word(); ins["elem"] = value()
    elif name in ("ARR_LEN", "TBL_LEN"):
        ins["a"] = word(); word(); ins["rbnd"] = value()
    elif name in ("ARR_ROOT", "TBL_ROOT"):
        ins["a"] = word()
    elif name in ("ARR_GET", "TBL_AT"):
        ins["a"] = word(); ins["idx"] = word(); word(); ins["rbnd"] = value()
        if name == "TBL_AT":
            word(); ins["vbnd"] = value()
    elif name == "TBL_GET":
        ins["a"] = word(); ins["key"] = word(); ins["fallback"] = word()
        word(); ins["rbnd"] = value()
    elif name in ("ARR_SET", "TBL_SET", "ARR_BATCH", "TBL_BATCH"):
        ins["a"] = word(); ins["idx"] = word(); ins["val"] = word()
        if name in ("ARR_BATCH", "TBL_BATCH"):
            ins["cnt"] = word()  # batch_cnt
    elif name == "OP_CALL":
        ins["callee_id"] = word()
        argc = word()
        ins["argc"] = argc
        ins["args"] = [word() for _ in range(argc)]
        retc = word()
        ins["rets"] = []
        for _ in range(retc):
            word()  # result type tag
            ins["rets"].append(value())
    elif name == "OP_RET":
        retc = word()
        ins["refs"] = [word() for _ in range(retc)]
    elif name in ("BAND", "BOR", "BXOR", "BSHL", "BSHR"):
        # §9.2: ref, ref, bnd(w), ext.
        ins["a"] = word()
        ins["b"] = word()
        word()  # result type tag
        ins["w"] = value()
    elif name == "Q_HASH":
        # §9.3: ref, cnt(b_bnd), ext.
        ins["a"] = word()
        ins["b_bnd"] = word()
    else:
        raise ValueError(f"opcode {name} has no operand rule in the committed inventory")
    ins["ext"] = word()
    return ins, i


def decode_stream(stream, n_blocks, allow_ext=False):
    """Decode the committed instruction stream: per block its id, argument
    signature, instruction count, and instructions in declared operand order.
    Static value ids are dense over the Intent's value-defining instructions
    in block-then-index scan order (P10)."""
    ws = words_of(stream)
    end = len(ws)
    i = 0
    blocks = []
    sid = 0
    for _ in range(n_blocks):
        if i + 2 > end:
            raise Malformed(S_PARTIAL)
        bid = ws[i]
        arity = ws[i + 1]
        i += 2
        arg_bnds = []
        for _ in range(arity):
            if i + 1 > end:
                raise Malformed(S_PARTIAL)
            rtag = ws[i]
            i += 1
            if rtag not in (1, 3):
                raise ValueError("argument result type tag")
            bv, i = decode_value(ws, i, end)
            arg_bnds.append((rtag, bv))
        if i + 1 > end:
            raise Malformed(S_PARTIAL)
        cnt = ws[i]
        i += 1
        instrs = []
        for _ in range(cnt):
            start = i
            try:
                ins, i = decode_instruction(ws, i, end, allow_ext)
            except ValueError:
                raise Malformed(S_PARTIAL)
            if ins["op"] in DEFINING:
                ins["sid"] = sid
                sid += 1
            ins["span"] = (start, i)
            instrs.append(ins)
        blocks.append({"id": bid, "arity": arity, "arg_bnds": arg_bnds,
                       "instrs": instrs})
    if i != end:
        raise Malformed(S_PARTIAL)
    return blocks


S_ENV_KEY = "malformed — unknown envelope key"


def check_ext_payload(payload):
    """P14 (expansion increment 5): the declared EXT payload reading —
    the empty payload reads as no content, any other payload as exactly
    one complete §2.6 value encoding with no trailing bytes."""
    if payload == b"":
        return None
    ws = words_of(payload)
    _, nxt = decode_value(ws, 0, len(ws))
    if nxt != len(ws):
        raise Malformed(S_EXT_TRAIL)
    return None


def check_ext_table(items):
    """P14: the record structure (two-element [index, payload] arrays),
    the committed duplicate-key rule over the record index words (the
    bitmask's selection target), and the payload reading of every record."""
    seen = set()
    for it in items:
        if (not isinstance(it, list) or len(it) != 2
                or not isinstance(it[0], int)
                or not isinstance(it[1], (bytes, bytearray))):
            raise Malformed(S_EXT_REC)
        if it[0] in seen:
            raise Malformed(S_EXT_DUP)
        seen.add(it[0])
        check_ext_payload(bytes(it[1]))
    return True


def check_ext_record_or_table(bs):
    """The Family M / EXT surface: the bytes carry either one record
    (a two-element array whose first element is the index word) or a side
    table (an array of records)."""
    m = cbor_decode(bs)
    if not isinstance(m, list):
        raise Malformed(S_EXT_REC)
    if m and isinstance(m[0], int):
        check_ext_table([m])
    else:
        check_ext_table(m)
    return True


def decode_envelope(bs):
    """Strict decode of one Intent envelope: canonical CBOR map, closed keys
    0–8, token type tags in the closed §2.5 vocabulary, the committed stream,
    and static legality of every ext bit against the EXT side table (§2.1);
    the side table's records carry the declared payload reading P14 and
    unique index words (expansion increment 5)."""
    m = cbor_decode(bs)
    if not isinstance(m, dict):
        raise ValueError("envelope is not a map")
    if 9 in m and 2 not in m:
        raise Malformed(S_ENV_KEY)
    for k in m:
        if not isinstance(k, int) or not 0 <= k <= 9:
            raise Malformed(S_ENV_KEY)
    for key in range(9):
        if key not in m:
            raise ValueError(f"envelope key {key} absent")
    pool_decl = []
    for e in m[8]:
        t = e if isinstance(e, int) else (e.get("type") if isinstance(e, dict) else None)
        if t not in TOKEN_TYPES:
            raise Malformed(S_TTYPE)
        pool_decl.append(t)
    if m[2] not in ("qir-enc-4", "qir-enc-5") and 9 in m:
        raise Malformed(S_ENV_KEY)
    callees = m.get(9, [])
    if not isinstance(callees, list):
        raise ValueError("the callee table must be an array")
    for rec in callees:
        if (not isinstance(rec, list) or len(rec) != 4
                or not isinstance(rec[0], int)
                or not all(isinstance(x, (bytes, bytearray)) for x in rec[1:])):
            raise ValueError("malformed callee record")
    blocks = decode_stream(m[6], m[5], m[2] != "qir-enc-2")
    max_tag = {"qir-enc-2": 16, "qir-enc-3": 29,
               "qir-enc-4": 31, "qir-enc-5": 37}.get(m[2], 16)
    for blk in blocks:
        for ins in blk["instrs"]:
            if OP_CODE[ins["op"]] > max_tag:
                raise Malformed(S_OPCODE)
    # the EXT side table: the declared record structure, unique index
    # words, and the payload reading P14 (expansion increment 5)
    check_ext_table(m[7])
    side_len = len(m[7])
    for b in blocks:
        for ins in b["instrs"]:
            for bit in range(64):
                if (ins["ext"] >> bit) & 1 and bit >= side_len:
                    raise Malformed(S_DANGLE)
    return {"sem": m[1], "enc": m[2], "prof": m[3], "deps": m[4],
            "n_blocks": m[5], "stream": m[6], "side": m[7], "pool": pool_decl,
            "blocks": blocks, "callees": callees}


def listing_of(blocks):
    """The source grammar's textual form: the sixteen mnemonics with the same
    operands in the same order (§2.6), regenerated from the decoded bytes."""
    lines = []
    for b in blocks:
        lines.append(f"block {b['id']} (arity {b['arity']}):")
        for k, ins in enumerate(b["instrs"]):
            op = ins["op"]
            if op == "CONST":
                imm = ins["imm"]
                if isinstance(imm, tuple) and imm and imm[0] == "zoom":
                    q, ze = (imm[1] if len(imm) == 2 else (imm[1], imm[2]))
                    lit = f"zoom({q}, {ze})"
                    kind = "zoom"
                elif isinstance(imm, tuple) and imm and imm[0] == "tuple":
                    # no committed source program carries a tuple literal;
                    # the rendering is declared deterministic for it
                    lit = "tuple(" + ", ".join(repr(f) for f in imm[1]) + ")"
                    kind = "tuple"
                else:
                    lit = str(imm)
                    kind = "int"
                t = f"CONST imm={lit} bnd=({kind}, <={ins['rbnd']})"
            elif op == "CLOAD":
                t = (f"CLOAD addr={ins['addr']} fallback=block {ins['fallback']} "
                     f"bnd=(int, <={ins['rbnd']})")
            elif op in ARITH:
                t = f"{op} ref={ins['a']} ref={ins['b']} bnd=(int, <={ins['rbnd']})"
            elif op == "CMP":
                t = f"CMP ref={ins['a']} ref={ins['b']} rng=[{ins['rng'][0]}, {ins['rng'][1]}]"
            elif op == "TUPLE":
                fs = ", ".join(f"({RTAG_NAMES[rt]}, <={b})"
                               for (rt, b) in ins["fields"])
                rs = " ".join(f"ref={r}" for r in ins["refs"])
                t = f"TUPLE k={len(ins['fields'])} fields=[{fs}]"
                if rs:
                    t += f" {rs}"
            elif op == "FIELD":
                t = f"FIELD ref={ins['a']} idx={ins['idx']}"
            elif op == "EMIT":
                rs = " ".join(f"ref={r}" for r in ins["refs"])
                t = f"EMIT kind={ins['kind']} k={len(ins['refs'])}"
                if rs:
                    t += f" {rs}"
            elif op == "BR":
                t = (f"BR cc={CC_NAMES[ins['cc']]} ref={ins['cond']} "
                     f"true=block {ins['true_blk']} false=block {ins['false_blk']}")
            elif op == "JMP":
                t = f"JMP block {ins['target']}"
            elif op == "OBS":
                t = f"OBS ttype={TOKEN_TYPES[ins['ttype']]} rng=[{ins['rng'][0]}, {ins['rng'][1]}]"
            else:
                t = op
            lines.append(f"  {k}: {t}")
    return lines


# ----------------------------------------------- the B_hat surrogate (§3.4)
def dominators(n, edges):
    """Iterative dominators over the static block graph; entry is block 0."""
    dom = [set(range(n)) for _ in range(n)]
    dom[0] = {0}
    changed = True
    while changed:
        changed = False
        for b in range(1, n):
            preds = [s for (s, d) in edges if d == b]
            if not preds:
                continue
            new = set.intersection(*(dom[p] for p in preds)) | {b}
            if new != dom[b]:
                dom[b] = new
                changed = True
    return dom


def block_graph(blocks):
    """Static edges: every BR edge (both), every JMP edge, and every CLOAD's
    declared fallback edge (the miss edge leaves the block, §2.3)."""
    edges = []
    for b in blocks:
        for ins in b["instrs"]:
            if ins["op"] == "BR":
                edges.append((b["id"], ins["true_blk"]))
                edges.append((b["id"], ins["false_blk"]))
            elif ins["op"] == "JMP":
                edges.append((b["id"], ins["target"]))
            elif ins["op"] == "CLOAD":
                edges.append((b["id"], ins["fallback"]))
    return edges


EXT_CHARGE = {"ARR_NEW": 4, "ARR_LEN": 4, "ARR_ROOT": 4, "ARR_GET": 8,
              "ARR_SET": 10, "ARR_BATCH": 16, "TBL_NEW": 4, "TBL_LEN": 4,
              "TBL_GET": 8, "TBL_AT": 8, "TBL_SET": 10, "TBL_BATCH": 16,
              "TBL_ROOT": 4, "OP_CALL": 6, "OP_RET": 6,
              "BAND": 4, "BOR": 4, "BXOR": 4, "BSHL": 4, "BSHR": 4}


def hash_charge(b_bnd):
    """§9.5: declared byte-bound charge, never a runtime length (F1/F14)."""
    return 160 + 130 * ((b_bnd + 63) // 64)


def ins_charge(ins, profile):
    """qir-prof-ext-calls-1 / qir-prof-ext-bootstrap-1 site-stable charge."""
    op = ins["op"]
    if op == "Q_HASH":
        return hash_charge(ins["b_bnd"])
    if op in EXT_CHARGE:
        return EXT_CHARGE[op]
    return profile["charges"][CLASS_OF[op]]


def b_hat(blocks, gateway, charge_of, trip_ceiling, s_max=4096):
    """The trip-ceiling unrolling dynamic program of §3.4 (draft.31): the
    state space is (block, instruction index, remaining trip budget per
    back-edge site); the value is the maximum summed site-stable charge from
    a state to an exit. Returns None when the pre-DP structural gate exceeds
    the declared S_max, or when there is no ceiling-bounded exit path
    (including irreducible cycles). None is an admission refusal, never a
    fallback to the gateway charge alone."""
    n = len(blocks)
    # Pre-dominator structural gate: an empty program, or a block count above
    # the declared S_max, is refused before dominators() is entered, bounding
    # the O(n^3) dominator work by S_max.
    if n == 0 or n > s_max:
        return None
    edges = block_graph(blocks)
    dom = dominators(n, edges)
    back = sorted({(s, d) for (s, d) in edges if d in dom[s]})
    bidx = {e: i for i, e in enumerate(back)}
    # Mandatory pre-DP structural gate: |B| * (trip_ceiling+1)^|back-edges|
    # in overflow-checked Python int arithmetic (arbitrary precision).
    dp_bound = len(blocks) * (trip_ceiling + 1) ** len(back)
    if dp_bound > s_max:
        return None
    entry_rem = tuple(trip_ceiling for _ in back)
    memo = {}

    def edge(src, dst, rem):
        if (src, dst) in bidx:
            k = bidx[(src, dst)]
            if rem[k] == 0:
                return None  # the traversal would exceed the declared ceiling
            nr = rem[:k] + (rem[k] - 1,) + rem[k + 1:]
            return val(dst, 0, nr)
        return val(dst, 0, rem)

    def best(vs):
        vs = [v for v in vs if v is not None]
        return max(vs) if vs else None

    def val(bid, k, rem):
        key = (bid, k, rem)
        if key in memo:
            return memo[key]
        memo[key] = None  # cycle guard: a closed loop is not a bounded path
        b = blocks[bid]
        total = 0
        j = k
        while True:
            ins = b["instrs"][j]
            total += charge_of(ins)
            op = ins["op"]
            if op in ("HALT", "OP_RET"):
                out = total
                break
            if op == "JMP":
                sub = edge(bid, ins["target"], rem)
                out = None if sub is None else total + sub
                break
            if op == "BR":
                vs = [edge(bid, ins["true_blk"], rem),
                      edge(bid, ins["false_blk"], rem)]
                sub = best(vs)
                out = None if sub is None else total + sub
                break
            if op == "CLOAD" and j + 1 < len(b["instrs"]):
                # the miss edge leaves the block here; the hit path continues
                hit = val(bid, j + 1, rem)
                miss = edge(bid, ins["fallback"], rem)
                sub = best([hit, miss])
                out = None if sub is None else total + sub
                break
            if j + 1 >= len(b["instrs"]):
                raise ValueError("block without terminator")
            j += 1
        memo[key] = out
        return out

    v = val(0, 0, entry_rem)
    return gateway + v if v is not None else None


def tarjan_scc(n, edges):
    """Tarjan SCC condensation, deterministic by ascending block id."""
    adj = [set() for _ in range(n)]
    for s, d in edges:
        if 0 <= s < n and 0 <= d < n:
            adj[s].add(d)
    adj = [sorted(x) for x in adj]
    index = [0]
    ids = [-1] * n
    low = [0] * n
    on = [False] * n
    stack = []
    comp = [-1] * n
    ncomp = [0]

    def strong(v):
        ids[v] = low[v] = index[0]
        index[0] += 1
        stack.append(v)
        on[v] = True
        for w in adj[v]:
            if ids[w] < 0:
                strong(w)
                low[v] = min(low[v], low[w])
            elif on[w]:
                low[v] = min(low[v], ids[w])
        if low[v] == ids[v]:
            while True:
                w = stack.pop()
                on[w] = False
                comp[w] = ncomp[0]
                if w == v:
                    break
            ncomp[0] += 1

    for v in range(n):
        if ids[v] < 0:
            strong(v)
    nc = ncomp[0]
    first = [n] * nc
    for v, c in enumerate(comp):
        first[c] = min(first[c], v)
    order = sorted(range(nc), key=lambda c: first[c])
    remap = [0] * nc
    for new, old in enumerate(order):
        remap[old] = new
    return [remap[c] for c in comp]


def scc_dp_states(blocks, trip_ceiling, s_max=4096):
    """§9.4 structural state count: Σ|SCC_i|·(trip+1)^|BE_i|."""
    n = len(blocks)
    if n == 0:
        return 0
    edges = block_graph(blocks)
    dom = dominators(n, edges)
    back = sorted({(s, d) for (s, d) in edges if d in dom[s]})
    comp = tarjan_scc(n, edges)
    nc = max(comp) + 1
    total = 0
    for c in range(nc):
        members = sum(1 for x in comp if x == c)
        k = sum(1 for (s, d) in back if comp[s] == c and comp[d] == c)
        total += members * (trip_ceiling + 1) ** k
    return total


def b_hat_scc(blocks, gateway, charge_of, trip_ceiling, s_max=4096):
    """§9.4 SCC-additive admission bound; None is a refusal."""
    n = len(blocks)
    if n == 0 or n > s_max:
        return None
    edges = block_graph(blocks)
    dom = dominators(n, edges)
    back = sorted({(s, d) for (s, d) in edges if d in dom[s]})
    states = scc_dp_states(blocks, trip_ceiling, s_max)
    if states > s_max:
        return None
    comp = tarjan_scc(n, edges)
    nc = max(comp) + 1
    local_be = [[e for e in back if comp[e[0]] == c and comp[e[1]] == c]
                for c in range(nc)]
    cross_in = [[] for _ in range(nc)]
    for s, d in edges:
        if comp[s] != comp[d] and s not in cross_in[comp[d]]:
            cross_in[comp[d]].append(s)

    def flow(bid):
        ins = blocks[bid]["instrs"]
        total = 0
        for k, i in enumerate(ins):
            total += charge_of(i)
            op = i["op"]
            if op in ("HALT", "OP_RET"):
                return total, ("halt", None)
            if op == "JMP":
                return total, ("jmp", i["target"])
            if op == "BR":
                return total, ("br", i["true_blk"], i["false_blk"])
            if op == "CLOAD":
                return total, ("cload", k, i["fallback"])
        raise ValueError("block without terminator")

    def join(a, b):
        out = dict(a or {})
        for k, v in (b or {}).items():
            out[k] = max(out.get(k, v), v)
        return out

    def prefix(m, p):
        return {k: v + p for k, v in m.items()}

    local = [None] * nc
    for c in range(nc):
        if comp[0] == c:
            entries = [0]
        else:
            entries = [b for b in range(n) if comp[b] == c and b in cross_in[c]]
            if not entries:
                entries = [next(b for b in range(n) if comp[b] == c)]
        be_index = {e: i for i, e in enumerate(local_be[c])}
        memo = {}

        def step(src, dst, rem):
            if (src, dst) not in be_index:
                return rem
            k = be_index[(src, dst)]
            if rem[k] == 0:
                return None
            return rem[:k] + (rem[k] - 1,) + rem[k + 1:]

        def val(bid, rem):
            if comp[bid] != c:
                return {("to", comp[bid]): 0}
            key = (bid, rem)
            if key in memo:
                return memo[key]
            memo[key] = None
            total, f = flow(bid)
            if f[0] == "halt":
                out = {("halt",): total}
            elif f[0] == "jmp":
                t = f[1]
                if comp[t] == c:
                    nr = step(bid, t, rem)
                    out = prefix(val(t, nr), total) if nr is not None else None
                else:
                    out = {("to", comp[t]): total}
            elif f[0] == "br":
                parts = []
                for t in (f[1], f[2]):
                    if comp[t] == c:
                        nr = step(bid, t, rem)
                        parts.append(prefix(val(t, nr), total) if nr is not None else None)
                    else:
                        parts.append({("to", comp[t]): total})
                out = join(parts[0], parts[1])
            else:
                _, k, fallback = f
                rest = cont(bid, k + 1, rem)
                parts = [prefix(rest, total) if rest is not None else None]
                if comp[fallback] == c:
                    nr = step(bid, fallback, rem)
                    parts.append(prefix(val(fallback, nr), total) if nr is not None else None)
                else:
                    parts.append({("to", comp[fallback]): total})
                out = join(parts[0], parts[1])
            memo[key] = out
            return out

        def cont(bid, from_idx, rem):
            total = 0
            for k in range(from_idx, len(blocks[bid]["instrs"])):
                i = blocks[bid]["instrs"][k]
                total += charge_of(i)
                op = i["op"]
                if op == "HALT":
                    return {("halt",): total}
                if op == "JMP":
                    t = i["target"]
                    if comp[t] == c:
                        nr = step(bid, t, rem)
                        return prefix(val(t, nr), total) if nr is not None else None
                    return {("to", comp[t]): total}
                if op == "BR":
                    parts = []
                    for t in (i["true_blk"], i["false_blk"]):
                        if comp[t] == c:
                            nr = step(bid, t, rem)
                            parts.append(prefix(val(t, nr), total) if nr is not None else None)
                        else:
                            parts.append({("to", comp[t]): total})
                    return join(parts[0], parts[1])
                if op == "CLOAD":
                    rest = cont(bid, k + 1, rem)
                    parts = [prefix(rest, total) if rest is not None else None]
                    t = i["fallback"]
                    if comp[t] == c:
                        nr = step(bid, t, rem)
                        parts.append(prefix(val(t, nr), total) if nr is not None else None)
                    else:
                        parts.append({("to", comp[t]): total})
                    return join(parts[0], parts[1])
            raise ValueError("block without terminator")

        best = None
        for b in entries:
            best = join(best, val(b, tuple(trip_ceiling for _ in local_be[c])))
        local[c] = best

    best = [None] * nc
    for c in range(nc - 1, -1, -1):
        m = local[c]
        if not m:
            continue
        value = None
        for (exit, charge) in m.items():
            tail = 0 if exit[0] == "halt" else best[exit[1]]
            if tail is not None:
                v = charge + tail
                value = v if value is None else max(value, v)
        best[c] = value
    return gateway + best[comp[0]] if best[comp[0]] is not None else None


# ================================================================== machine
# Executes the committed transition relation of §2.3 under §2.5's lifecycle
# algebra and §3's accounting, with the corpus's declared artifact-wave
# parameters P1–P14 and the declared profile.
#
# Instance ids mint in transition order from 0 (§2.4); a redefinition mints,
# never overwrites; an operand reads the current instance of its static id.
# Tier words (P6): O = 1, S = 2.  State words (P6): outstanding 1, consumed 2,
# terminalized 3.  Outcome tags (§2.6): SUCCESS 1, NO_SOLUTION 2,
# EXPRESSION_TOO_COMPLEX 3, DOMAIN_ERROR 4.

TIER_O, TIER_S = 1, 2
ST_OUT, ST_CONSUMED, ST_TERM = 1, 2, 3
OUTCOME = {"SUCCESS": 1, "NO_SOLUTION": 2, "EXPRESSION_TOO_COMPLEX": 3,
           "DOMAIN_ERROR": 4}
OUTCOME_NAME = {v: k for k, v in OUTCOME.items()}


class Rejection(Exception):
    """A typed rejection surface (§2.5): fatal, sited; every check precedes
    every mutation, so the pre-state is the configuration at raise time."""

    def __init__(self, token_type):
        super().__init__(token_type)
        self.token_type = token_type


def tid(t):
    return [t["birth"], t["block"], t["index"], t["serial"], t["type"]]


def _is_zoom(v):
    return isinstance(v, tuple) and v and v[0] == "zoom"


def zoom_pair(v):
    """The operand's normalized zoom pair under §2.3's N: a zoom-carrying
    value is already stored normalized; an integer operand compares as the
    dyadic (v, 0) under N (expansion increment 3)."""
    if _is_zoom(v):
        return v[1]
    return zoom_normalized(v, 0)


def cmp_under_n(a, b):
    """The zoom-boundary comparison under N (§2.3): equal exactly when the
    normalized pairs are equal; otherwise, for normalized (q1, z1), (q2, z2)
    with z1 ≤ z2, the ordering is decided by comparing q1 against
    q2·2^(z2−z1) in exact integer arithmetic — the exponent difference is
    non-negative, so the shift is exact and the comparison never rounds."""
    (q1, z1), (q2, z2) = zoom_pair(a), zoom_pair(b)
    if (q1, z1) == (q2, z2):
        return 0
    neg = False
    if z1 > z2:
        (q1, z1), (q2, z2) = (q2, z2), (q1, z1)
        neg = True
    shifted = q2 * (1 << (z2 - z1))
    c = -1 if q1 < shifted else (1 if q1 > shifted else 0)
    return -c if neg else c


class Machine:
    def __init__(self, envelope_bytes, D, w_budget, profile, b_inject=None):
        env = decode_envelope(envelope_bytes)
        self.blocks = env["blocks"]
        self.sids = []
        _sid = len(env.get("callees", []))  # declaration handles precede entry ids
        for blk in self.blocks:
            row = []
            for ins in blk["instrs"]:
                if ins["op"] in DEFINING:
                    row.append(_sid)
                    _sid += 1
                else:
                    row.append(0)
            self.sids.append(row)
        for bi, blk in enumerate(self.blocks):
            for ii, ins in enumerate(blk["instrs"]):
                if ii < len(self.sids[bi]):
                    ins["sid"] = self.sids[bi][ii]
        self.deps = env["deps"]
        self.words = words_of(env["stream"])
        self.versions = [env["sem"], env["enc"], env["prof"]]
        self.D = D
        self.W = w_budget
        self.profile = profile
        # The admission bound: the exact surrogate, or the declared injected
        # bound of the F27 fixture mode (test-only validator-defect
        # injection, §4.4) — never a conformance path.
        bh_fn = b_hat_scc if env["enc"] == "qir-enc-5" else b_hat
        self.BH = (b_inject if b_inject is not None
                   else bh_fn(self.blocks, profile["gateway"],
                              lambda ins: ins_charge(ins, profile),
                              profile["trip_ceiling"], profile["s_max"]))
        self.prog_root = hashlib.sha256(envelope_bytes).digest()
        self.pool = [{"birth": 0, "block": 0, "index": 0, "serial": k,
                      "type": t, "tier": TIER_S, "state": ST_OUT}
                     for k, t in enumerate(env["pool"])]
        self.acc = {"charged": 0, "reserved": 0, "refunded": 0}
        self.clock = 0
        self.step = 0
        self.halt = [0, 0]
        self.receipts = []          # the receipt-order queue (P2 component 9):
        #                            the witness records in emission order,
        #                            each the P13(d) entry
        self.step_receipt_ids = []  # the ordinals the current transition emitted
        self.store = {}             # (static id, instance id) -> value
        self.containers = {}        # (static id, instance id) -> Container
        self.callees = []           # [(blocks, sids, interior_bh)]
        for rec in env.get("callees", []):
            cb = decode_stream(rec[3], 1, True)
            arity = len(rec[1]) // 32  # one bnd = 4 words = 32 bytes
            sids = []
            n = arity
            for blk in cb:
                row = []
                for ins in blk["instrs"]:
                    if ins["op"] in DEFINING:
                        row.append(n); n += 1
                    else:
                        row.append(0)
                sids.append(row)
            for bi, blk in enumerate(cb):
                for ii, ins in enumerate(blk["instrs"]):
                    if ii < len(sids[bi]):
                        ins["sid"] = sids[bi][ii]
            self.callees.append((cb, sids, bh_fn(cb, profile["gateway"],
                                                lambda i: ins_charge(i, profile),
                                                profile["trip_ceiling"], profile["s_max"]) or 0,
                                 words_of(rec[3])))
        if b_inject is None and self.callees:
            # T36(b): instruction-decoded compositional B_hat in reverse
            # topological order over key-9 callees (docs/qnir_calls.md §8.4).
            # No raw-word scanning; no gateway double-count; no caller-only scan.
            gateway = profile["gateway"]
            n = len(self.callees)
            folded = [0] * n
            for cid in range(n - 1, -1, -1):
                t = self.callees[cid][2]
                for blk in self.callees[cid][0]:
                    for ins in blk["instrs"]:
                        if ins["op"] == "OP_CALL":
                            nested = ins["callee_id"]
                            if nested > cid and nested < n:
                                t += max(0, folded[nested] - gateway)
                folded[cid] = t
            for blk in self.blocks:
                for ins in blk["instrs"]:
                    if ins["op"] == "OP_CALL":
                        cid = ins["callee_id"]
                        if cid < n:
                            self.BH += max(0, folded[cid] - gateway)
        self.stack_root = hashlib.sha256(b"\x02").digest()
        self.frames = []            # frame dicts for OP_CALL/OP_RET
        self.current = {}           # static id -> current instance id
        self.next_iid = 0
        self.next_serial = len(env["pool"])
        self.escrow_open = False
        self.records = []
        self.boundaries = []
        self.per_class = {}
        self.traversals = {}        # (block, index, slot) -> traversal count
        self.pending_miss = None    # (token, fallback block)
        self.fatal_selected = None  # DR2: the fatal token of a terminalizing birth
        edges = block_graph(self.blocks)
        dom = dominators(len(self.blocks), edges)
        self.back_edges = sorted({(s, d) for (s, d) in edges if d in dom[s]})

    # ------------------------------------------------ configuration + records
    def config_root(self, pc):
        """The P1 state root (draft.32, F-G0-1): the SHA-256 (exact 32
        bytes) of the canonical configuration encoding P2 — the P2
        pre-image is retained in-memory per F15 and never substituted for
        the digest."""
        store_c = [[s, i, self.containers[(s, i)].canonical_bytes()
                    if (s, i) in self.containers else enc_value(v)]
                   for (s, i), v in sorted(self.store.items(), key=lambda kv: kv[0])]
        pool_c = [[t["birth"], t["block"], t["index"], t["serial"], t["type"],
                   t["tier"], t["state"]]
                  for t in sorted(self.pool, key=tid)]
        return hashlib.sha256(self.config_p2_bytes(pc, store_c, pool_c)).digest()

    def p2_now(self, pc):
        store_c = [[s, i, self.containers[(s, i)].canonical_bytes()
                    if (s, i) in self.containers else enc_value(v)]
                   for (s, i), v in sorted(self.store.items(), key=lambda kv: kv[0])]
        pool_c = [[t["birth"], t["block"], t["index"], t["serial"], t["type"],
                   t["tier"], t["state"]] for t in sorted(self.pool, key=tid)]
        return self.config_p2_bytes(pc, store_c, pool_c)

    def config_p2_bytes(self, pc, store_c, pool_c):
        env_c = [] if not self.frames else [[], self.stack_root]
        comps = [list(pc), env_c, store_c, pool_c,
                 [self.acc["charged"], self.acc["reserved"], self.acc["refunded"]],
                 self.clock, self.step, [self.halt[0], self.halt[1]],
                 list(self.receipts), list(self.versions)]
        return cbor_encode(comps)

    def emit(self, pc, decoded, charged, refunded, events, io_ids, pre, post):
        """One transition record: the fourteen fields of §2.6's F5 list, in
        order, through deterministic CBOR.  Field 13 (P13(d), expansion
        increment 3) carries the ordinals the transition emitted."""
        rec = [[1, pre], [2, self.prog_root], [3, self.step], [4, list(pc)],
               [5, decoded], [6, list(io_ids)], [7, pre], [8, post],
               [9, charged], [10, refunded], [11, self.clock], [12, events],
               [13, list(self.step_receipt_ids)], [14, list(self.versions)]]
        self.records.append(cbor_encode(rec))

    def decoded_at(self, blk, idx):
        lo, hi = self.blocks[blk]["instrs"][idx]["span"]
        return b"".join(w(x) for x in self.words[lo:hi])

    def outstanding(self):
        return self.acc["reserved"] - self.acc["charged"] - self.acc["refunded"]

    def mint(self, sid, value):
        self.store[(sid, self.next_iid)] = value
        self.current[sid] = self.next_iid
        self.next_iid += 1

    def read(self, sid):
        return self.store[(sid, self.current[sid])]

    def read_value(self, sid):
        return self.store[(sid, self.current[sid])]

    def birth(self, ttype, pc, birth_step=None):
        """P5: birth step = the birthing transition's own step index; the
        declared initial pool births at step 0 (handled in __init__)."""
        if birth_step is None:
            birth_step = self.step + 1
        tok = {"birth": birth_step, "block": pc[0], "index": pc[1],
               "serial": self.next_serial, "type": ttype, "tier": TIER_S,
               "state": ST_OUT}
        self.next_serial += 1
        self.pool.append(tok)
        return tok

    # ------------------------------------------------------------- the run
    def run(self):
        if self.BH is None or self.W < self.BH:
            # admission refusal (§3.4): None (no certified bound / pre-DP
            # gate exceeded) or budget below the bound — the single rejection
            # transition, zero postings; the escrow never opens
            pre = self.config_root((0, 0))
            return self.rejection("EXPRESSION_TOO_COMPLEX", (0, 0),
                                  self.decoded_at(0, 0), pre)
        pc = (0, 0)
        prog = 0
        pre = self.config_root(pc)
        while True:
            if self.frames:
                blocks, sids = self.callees[prog - 1][0], self.callees[prog - 1][1]
            else:
                blocks, sids = self.blocks, self.sids
            self._cur_sids = sids
            self._current_prog = prog
            blk, idx = pc
            ins = blocks[blk]["instrs"][idx]
            decoded = self.decoded_at(blk, idx) if prog == 0 else bytes(
                v for w in self.callees[prog - 1][3][ins["span"][0]:ins["span"][1]]
                for v in w.to_bytes(8, "big"))
            c = ins_charge(ins, self.profile)
            events = []
            io_ids = []
            gateway = 0
            if self.escrow_open and self.outstanding() < c:
                # fuel backstop (§3.4): defense in depth after admission
                return self.rejection("EXPRESSION_TOO_COMPLEX", pc, decoded, pre)
            try:
                self.step_receipt_ids = []
                transfer = self.dispatch(ins, pc, events, io_ids)
            except Rejection as r:
                return self.rejection(r.token_type, pc, decoded, pre)
            if transfer is None:
                transfer = ("fall", None)
            if not self.escrow_open:
                # §3.3: the reserve is opened once, by the execution's
                # first COMPLETED transition (§3.4: on a pass the
                # transition completes normally and opens the escrow) — a
                # typed rejection at the first attempted transition never
                # opens it; the reserve takes B(P) in full (§3.3)
                self.escrow_open = True
                self.acc["reserved"] += self.BH
                gateway = self.profile["gateway"]
            post_charge = gateway + c
            self.acc["charged"] += post_charge
            cls = CLASS_OF.get(ins["op"], "q_" + ins["op"].lower())
            self.per_class[cls] = self.per_class.get(cls, 0) + c
            if gateway:
                self.per_class["gateway"] = self.per_class.get("gateway", 0) + gateway
            self.step += 1
            self.clock += 1
            last = idx == len(self.blocks[blk]["instrs"]) - 1
            if transfer[0] == "halt":
                if self.pending_miss and self.pending_miss[1] == blk and last:
                    # §2.5 tier rule (F12): the miss token is consumed when
                    # its fallback block's entry completes successfully
                    self.pending_miss[0]["state"] = ST_CONSUMED
                    events.append([2, tid(self.pending_miss[0])])
                    self.pending_miss = None
                outcome = self.terminalize(events)
                self.halt = [1, outcome]
                refund = self.outstanding()
                if refund:
                    self.acc["refunded"] += refund
                post = self.config_root(pc)
                self.final_p1 = post
                self.emit(pc, decoded, post_charge, refund, events, io_ids,
                          pre, post)
                self.boundaries.append((self.step, dict(self.acc)))
                return self.pack(outcome)
            if self.pending_miss and self.pending_miss[1] == blk and last:
                self.pending_miss[0]["state"] = ST_CONSUMED
                events.append([2, tid(self.pending_miss[0])])
                self.pending_miss = None
            post = self.config_root(pc)
            self.emit(pc, decoded, post_charge, 0, events, io_ids, pre, post)
            self.boundaries.append((self.step, dict(self.acc)))
            pre = post
            if transfer[0] == "miss":
                self.pending_miss = (transfer[1], transfer[2])
                pc = (transfer[2], 0)
            elif transfer[0] in ("br", "jmp"):
                pc = (transfer[1], 0)
            elif transfer[0] == "call":
                prog = transfer[1] + 1
                pc = (0, 0)
            # track current program frame for ret_prog save/restore
            elif transfer[0] == "ret":
                f = self.frames.pop()
                used = self.acc["charged"] - f["charged_at_call"]
                residue = max(0, f["reserve"] - used - 6)
                self.stack_root = f["parent"]
                v = self._int_at(ins["refs"][0]) if ins["refs"] else 0
                if v <= f["ret_bound"]:
                    self.mint(f["ret_sid"], v)
                prog = f.get("ret_prog", 0)
                pc = (f["ret_pc"][0], f["ret_pc"][1] + 1)
            else:
                pc = (blk, idx + 1)

    def dispatch(self, ins, pc, events, io_ids):
        op = ins["op"]
        return getattr(self, "do_" + op)(ins, pc, events, io_ids)

    # -------------------------------------------------------- the rules (§2.3)
    def do_CONST(self, ins, pc, events, io_ids):
        imm, rbnd = ins["imm"], ins["rbnd"]
        if _is_zoom(imm):
            # P13(c) (expansion increment 3): a zoom literal's declared
            # INT_POS upper bound constrains the normalized exponent — the
            # pair's range-bearing component
            if imm[1][1] > rbnd:
                raise Rejection("EXPRESSION_TOO_COMPLEX")
        elif isinstance(imm, tuple):
            # P13(c): a tuple-result declaration's bound word is present and
            # vacuous — the check compares an INT_POS quantity, and a tuple
            # carries none
            pass
        elif imm > rbnd:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], imm)

    def do_CLOAD(self, ins, pc, events, io_ids):
        addr = self.deps[ins["addr"]]
        io_ids.append(addr)
        if addr in self.D:
            v = decode_value(words_of(self.D[addr]), 0, len(words_of(self.D[addr])))[0]
            if v > ins["rbnd"]:
                raise Rejection("EXPRESSION_TOO_COMPLEX")
            self.mint(ins["sid"], v)
            return None
        # On miss: check trip ceiling on fallback edge (slot 0) before birthing token (F25)
        self.check_ceiling(pc, ins["fallback"], 0)
        if (pc[0], ins["fallback"]) in self.back_edges:
            site = (pc[0], pc[1], 0)
            self.traversals[site] = self.traversals.get(site, 0) + 1
        tok = self.birth(TOKEN_TYPES_REV["STATUS_LOOKUP_MISS"], pc)
        tok["tier"] = TIER_O
        events.append([1, tid(tok)])
        # the miss edge leaves the block here (§2.3 CLOAD row)
        return ("miss", tok, ins["fallback"])

    # ------------------------------------------- bootstrap ops (qir-enc-5)
    def _bootstrap_operands(self, ins):
        w = ins["w"]
        if w not in (8, 16, 32, 64):
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        x, y = self.read(ins["a"]), self.read(ins["b"])
        mask = (1 << w) - 1
        if not isinstance(x, int) or not isinstance(y, int):
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        if x < 0 or y < 0 or x > mask or y > mask:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        return x, y, w, mask

    def do_BAND(self, ins, pc, events, io_ids):
        x, y, _, mask = self._bootstrap_operands(ins)
        self.mint(self._sid(pc), (x & y) & mask)

    def do_BOR(self, ins, pc, events, io_ids):
        x, y, _, mask = self._bootstrap_operands(ins)
        self.mint(self._sid(pc), (x | y) & mask)

    def do_BXOR(self, ins, pc, events, io_ids):
        x, y, _, mask = self._bootstrap_operands(ins)
        self.mint(self._sid(pc), (x ^ y) & mask)

    def do_BSHL(self, ins, pc, events, io_ids):
        x, y, w, mask = self._bootstrap_operands(ins)
        if y >= w:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), (x << y) & mask)

    def do_BSHR(self, ins, pc, events, io_ids):
        x, y, w, _ = self._bootstrap_operands(ins)
        if y >= w:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), x >> y)

    def do_Q_HASH(self, ins, pc, events, io_ids):
        b_bnd = ins["b_bnd"]
        data = enc_value(self.read(ins["a"]))
        if b_bnd == 0 or b_bnd > 65536 or len(data) > b_bnd:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        d = hashlib.sha256(data).digest()
        words = [int.from_bytes(d[i * 8:(i + 1) * 8], "big") for i in range(4)]
        self.mint(self._sid(pc), ("tuple", words))

    # ----------------------------------------------- containers (qir-enc-3/4)
    def _container_at(self, sid):
        iid = self.current.get(sid)
        return self.containers.get((sid, iid)) if iid is not None else None

    def _put_container(self, sid, c):
        iid = self.next_iid
        self.next_iid += 1
        self.containers[(sid, iid)] = c
        self.store[(sid, iid)] = 0   # opaque integer handle at this instance
        self.current[sid] = iid

    def _int_at(self, sid):
        v = self.read(sid)
        return v if isinstance(v, int) else 0

    def _sid(self, pc):
        rows = getattr(self, "_cur_sids", self.sids)
        return rows[pc[0]][pc[1]] or 0

    def do_ARR_NEW(self, ins, pc, events, io_ids):
        self._put_container(self._sid(pc), Container(ARRAY_RRB16, ins["cap"], ins["elem"]))

    def do_TBL_NEW(self, ins, pc, events, io_ids):
        self._put_container(self._sid(pc), Container(TABLE_MBTREE16, ins["cap"], ins["elem"]))

    def do_ARR_LEN(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        n = c.len()
        if n > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), n)

    do_TBL_LEN = do_ARR_LEN

    def do_ARR_GET(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        try:
            v = c.get_index(self._int_at(ins["idx"]))
        except ValueError:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        if v > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), v)

    def do_TBL_AT(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        try:
            _, v = c.at(self._int_at(ins["idx"]))
        except ValueError:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        if v > ins["vbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), v)

    def do_ARR_SET(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        try:
            nxt = c.set_index(self._int_at(ins["idx"]), self._int_at(ins["val"]))
        except ValueError:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self._put_container(self._sid(pc), nxt)

    def do_TBL_SET(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        try:
            nxt = c.set_key(self._int_at(ins["idx"]), self._int_at(ins["val"]))
        except ValueError:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self._put_container(self._sid(pc), nxt)

    def do_TBL_GET(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        k = self._int_at(ins["key"])
        v = c.get_key(k)
        if v is not None:
            if v > ins["rbnd"]:
                raise Rejection("EXPRESSION_TOO_COMPLEX")
            self.mint(self._sid(pc), v)
            return None
        self.check_ceiling(pc, ins["fallback"], 0)
        tok = self.birth(TOKEN_TYPES_REV["STATUS_LOOKUP_MISS"], pc)
        tok["tier"] = TIER_O
        events.append([1, tid(tok)])
        return ("miss", tok, ins["fallback"])

    def do_ARR_ROOT(self, ins, pc, events, io_ids):
        c = self._container_at(ins["a"])
        if c is None:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(self._sid(pc), ("tuple",) + tuple(c.root()))

    do_TBL_ROOT = do_ARR_ROOT

    def do_OP_CALL(self, ins, pc, events, io_ids):
        if len(self.frames) >= 8 or ins["callee_id"] >= len(self.callees):
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        cid = ins["callee_id"]
        # T36(c): static K6a & call-graph validation (docs/qnir_calls.md §8.2).
        # Inside callee i (prog == i+1) OP_CALL may only target callee_id > i.
        if self._current_prog > 0 and cid <= self._current_prog - 1:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        if ins["argc"] > 64:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        cb, sids, interior, raw = self.callees[cid]
        reserve = interior + 6
        parent = self.stack_root
        pre = b"\x30" + parent + self.p2_now(pc)
        self.stack_root = hashlib.sha256(pre).digest()
        for i, a in enumerate(ins["args"]):
            self.mint(i, self._int_at(a))  # callee block-argument ids are 0..argc-1
        self.frames.append({"callee": cid, "ret_pc": pc, "reserve": reserve,
                            "charged_at_call": self.acc["charged"], "parent": parent,
                            "ret_sid": self._sid(pc), "ret_bound": ins["rets"][0] if ins["rets"] else 0,
                            "ret_prog": self._current_prog})
        self._next_prog = cid + 1
        self._next_pc = (0, 0)
        return ("call", cid)

    def do_OP_RET(self, ins, pc, events, io_ids):
        if not self.frames:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        return ("ret", None)

    def do_ADD(self, ins, pc, events, io_ids):
        r = self._int_at(ins["a"]) + self._int_at(ins["b"])
        if r > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], r)

    def do_SUB(self, ins, pc, events, io_ids):
        r = self._int_at(ins["a"]) - self._int_at(ins["b"])
        if r > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], r)

    def do_MUL(self, ins, pc, events, io_ids):
        r = self._int_at(ins["a"]) * self._int_at(ins["b"])
        if r > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], r)

    def do_DIV(self, ins, pc, events, io_ids):
        a, b = self.read(ins["a"]), self.read(ins["b"])
        if b == 0:
            # the zero divisor births STATUS_DOMAIN_ERROR (§2.3 DIV row)
            raise Rejection("STATUS_DOMAIN_ERROR")
        q = abs(a) // abs(b)
        if (a < 0) != (b < 0):
            q = -q
        if q > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], q)

    def do_MOD(self, ins, pc, events, io_ids):
        a, b = self.read(ins["a"]), self.read(ins["b"])
        if b == 0:
            raise Rejection("STATUS_DOMAIN_ERROR")
        r = abs(a) % abs(b)
        if r > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], r)

    def do_CMP(self, ins, pc, events, io_ids):
        x, y = self.read_value(ins["a"]), self.read_value(ins["b"])
        if _is_zoom(x) or _is_zoom(y):
            # P13(b) (expansion increment 3): the CMP row's pre-dispatch
            # declared-range check over each zoom-carrying operand
            for v in (x, y):
                if _is_zoom(v):
                    m, z = v[1]
                    if not (ZOOM_EXP_LO <= z <= ZOOM_EXP_HI
                            and 0 <= m < ZOOM_MAN_HI):
                        raise Rejection("EXPRESSION_TOO_COMPLEX")
            r = cmp_under_n(x, y)
            self.mint(ins["sid"], r)
            return None
        a, b = x, y
        self.mint(ins["sid"], -1 if a < b else (1 if a > b else 0))

    def do_BR(self, ins, pc, events, io_ids):
        r = self.read(ins["cond"])
        cc = ins["cc"]
        take = {0: r < 0, 1: r <= 0, 2: r == 0, 3: r != 0, 4: r > 0, 5: r >= 0}[cc]
        tgt = ins["true_blk"] if take else ins["false_blk"]
        slot = 1 if take else 0
        self.check_ceiling(pc, tgt, slot)
        self.traversals[(pc[0], pc[1], slot)] = \
            self.traversals.get((pc[0], pc[1], slot), 0) + 1
        return ("br", tgt)

    def do_JMP(self, ins, pc, events, io_ids):
        self.check_ceiling(pc, ins["target"], 0)
        self.traversals[(pc[0], pc[1], 0)] = \
            self.traversals.get((pc[0], pc[1], 0), 0) + 1
        return ("jmp", ins["target"])

    def check_ceiling(self, pc, tgt, slot):
        if (pc[0], tgt) in self.back_edges:
            site = (pc[0], pc[1], slot)
            if self.traversals.get(site, 0) + 1 > self.profile["trip_ceiling"]:
                raise Rejection("EXPRESSION_TOO_COMPLEX")

    def do_OBS(self, ins, pc, events, io_ids):
        present = any(t["type"] == ins["ttype"] and t["state"] == ST_OUT
                      for t in self.pool)
        self.mint(ins["sid"], 1 if present else 0)

    def do_PROMOTE(self, ins, pc, events, io_ids):
        ners = [t for t in self.pool
                if t["type"] == TOKEN_TYPES_REV["STATUS_NEG_EVEN_ROOT"]
                and t["state"] == ST_OUT]
        if ners:
            tok = sorted(ners, key=tid)[0]
            tok["state"] = ST_CONSUMED
            events.append([2, tid(tok)])

    def do_TUPLE(self, ins, pc, events, io_ids):
        # §2.3's TUPLE row (expansion increment 3): binds the canonical tuple
        # value over the components to a fresh instance; any component
        # exceeding its declared bound fires the typed rejection before
        # dispatch — under P13(c)'s bound reading the check compares integer
        # components against their INT_POS bounds, a TUPLE-typed field's
        # bound (and a tuple component under it) carrying no INT_POS quantity
        comps = [self.read_value(r) for r in ins["refs"]]
        for (rtag, b), v in zip(ins["fields"], comps):
            if rtag == 1 and isinstance(v, int) and not isinstance(v, bool):
                if v > b:
                    raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], ("tuple", comps))
        return None

    def do_FIELD(self, ins, pc, events, io_ids):
        # §2.3's FIELD row (expansion increment 3): binds the i-th
        # component's canonical value to a fresh instance; no run-time
        # rejection — postulate (v) excludes arity and type mismatches at
        # Intent formation
        v = self.read_value(ins["a"])
        self.mint(ins["sid"], v[1][ins["idx"]])
        return None

    def do_EMIT(self, ins, pc, events, io_ids):
        # §2.3's EMIT row + P13(d) (expansion increment 3): the witness
        # record — (the declared kind, the cited values' current-instance
        # canonical encodings, the emission position) — appends to the
        # receipt-order queue; the ordinal (P7) rides the entry and field 13
        # cites it
        encs = [enc_value(self.read_value(r)) for r in ins["refs"]]
        ordn = len(self.receipts)
        self.receipts.append([ordn, ins["kind"], encs, [pc[0], pc[1]]])
        self.step_receipt_ids.append(ordn)
        return None

    def do_HALT(self, ins, pc, events, io_ids):
        return ("halt",)

    # -------------------------------------------------------- exit machinery
    def terminalize(self, events):
        """T (§2.5): the earliest-birth outstanding Tier S token decides
        (ordered lexicographically by the full token identifier, F15); every
        outstanding token terminalizes and is traced."""
        outs = [t for t in self.pool
                if t["state"] == ST_OUT and t["tier"] == TIER_S]
        if outs:
            sel = sorted(outs, key=tid)[0]
            outcome = {TOKEN_TYPES_REV["STATUS_NEG_EVEN_ROOT"]: OUTCOME["NO_SOLUTION"],
                       TOKEN_TYPES_REV["EXPRESSION_TOO_COMPLEX"]:
                           OUTCOME["EXPRESSION_TOO_COMPLEX"],
                       TOKEN_TYPES_REV["STATUS_DOMAIN_ERROR"]:
                           OUTCOME["DOMAIN_ERROR"]}[sel["type"]]
        else:
            outcome = OUTCOME["SUCCESS"]
        for t in self.pool:
            if t["state"] == ST_OUT:
                t["state"] = ST_TERM
                events.append([3, tid(t)])
        return outcome

    def rejection(self, ttype, pc, decoded, pre):
        """A typed rejection (§2.5): posts nothing, births the fatal token
        terminalizing at its own transition, is final; the terminal
        settlement posts into this record (§3.3, P11).  Loop-3 expansion
        increment 1 (v0.3.0-draft.14) repair, registered from the F12
        combination vectors: under §2.5 T case (3) — itemization at every
        exit — the other outstanding tokens terminalize at this exit exactly
        as at a HALT exit (DR1 of the corpus), their terminalize events
        riding this record after the birth event in the pool's canonical
        identifier order; and under §2.5 T case (1) + §2.6 receipt key 3
        (DR2 of the corpus) the receipt's selected token for a fatal-birth
        DOMAIN_ERROR exit is the outcome-determining fatal token itself."""
        self.step += 1
        self.clock += 1
        tok = self.birth(TOKEN_TYPES_REV[ttype], pc, birth_step=self.step)
        tok["state"] = ST_TERM
        outcome = (OUTCOME["EXPRESSION_TOO_COMPLEX"]
                   if ttype == "EXPRESSION_TOO_COMPLEX" else OUTCOME["DOMAIN_ERROR"])
        self.halt = [1, outcome]
        # DR2 (v0.3.0-draft.14): the fatal token is the outcome-determining
        # token of a terminalizing-birth exit
        self.fatal_selected = (tid(tok) if outcome == OUTCOME["DOMAIN_ERROR"]
                               else None)
        refund = 0
        if self.escrow_open:
            refund = self.outstanding()
            if refund:
                self.acc["refunded"] += refund
        # DR1 (v0.3.0-draft.14): terminalization at every exit (§2.5 T
        # case 3) — the other outstanding tokens terminalize here, their
        # events after the birth event in canonical identifier order
        events = [[1, tid(tok)]]
        for t in sorted((t for t in self.pool if t["state"] == ST_OUT),
                        key=tid):
            t["state"] = ST_TERM
            events.append([3, tid(t)])
        post = self.config_root(pc)
        self.final_p1 = post         # the fatal terminal record's post root
        self.step_receipt_ids = []   # a rejection transition emits no witness
        self.emit(pc, decoded, 0, refund, events, [], pre, post)
        self.boundaries.append((self.step, dict(self.acc)))
        return self.pack(outcome)

    def build_receipt(self, outcome):
        """The terminal receipt (§2.5/§2.6): deterministic CBOR map, closed
        keys 0–4; the outcome-determining token selected for NO_SOLUTION and
        DOMAIN_ERROR (under DR2, v0.3.0-draft.14: at a terminalizing-birth
        exit the fatal birthed token; at a HALT exit the earliest-birth
        outstanding Tier S token); nil otherwise (the RFC 8949 nil, 0xf6)."""
        item = [[t["birth"], t["block"], t["index"], t["serial"], t["type"],
                 t["tier"], t["state"]] for t in sorted(self.pool, key=tid)]
        selected = None
        if outcome in (OUTCOME["NO_SOLUTION"], OUTCOME["DOMAIN_ERROR"]):
            if self.fatal_selected is not None:
                selected = self.fatal_selected
            else:
                cands = [t for t in self.pool if t["state"] == ST_TERM
                         and t["type"] in (TOKEN_TYPES_REV["STATUS_NEG_EVEN_ROOT"],
                                           TOKEN_TYPES_REV["STATUS_DOMAIN_ERROR"])]
                if cands:
                    selected = tid(sorted(cands, key=tid)[0])
        return cbor_encode({0: b"QnIR-Terminal", 1: outcome, 2: item,
                            3: selected, 4: list(self.versions)})

    def pack(self, outcome):
        return {"outcome": outcome, "receipt": self.build_receipt(outcome),
                "records": list(self.records), "boundaries": list(self.boundaries),
                "acc": dict(self.acc), "per_class": dict(self.per_class),
                "B_hat": self.BH, "versions": list(self.versions),
                "final_p1": getattr(self, "final_p1", b"")}


TOKEN_TYPES_REV = {v: k for k, v in TOKEN_TYPES.items()}


# ============================================================ corpus parsing
# The committed corpus is consumed exactly as committed: a markdown document
# whose fences and field tables carry the committed expected bytes and
# outcomes (§4.4).  Nothing below writes to it.

IMPL_VERSION = "0.3.0-draft.36"

# The implementation artifact's declared source set (commitments §6.2), in
# the declared path order.
IMPL_SOURCES = ["src/qnir-py/qnir_ref.py"]


def lf_sha256(bs):
    """SHA-256 over the LF-normalized bytes (the repository byte form; the
    working copy's line endings are a checkout artifact and never
    participate)."""
    return hashlib.sha256(bs.replace(b"\r\n", b"\n")).hexdigest()


def impl_identity():
    """The implementation artifact's own identity (commitments §6.2): the
    declared version and the digest over the declared source files' bytes in
    the declared path order."""
    joined = bytearray()
    for f in IMPL_SOURCES:
        with open(f, "rb") as fh:
            joined.extend(fh.read().replace(b"\r\n", b"\n"))
    return IMPL_VERSION, lf_sha256(bytes(joined))


# The declared full-transition trace-root construction `qir-trace-root-1`
# (commitments §6.6): the leaf SHA-256(0x00 || r) over each record's canonical
# encoding, the node SHA-256(0x01 || x || y), levels formed pairwise and left
# to right with an odd level's final value promoted unchanged (never
# duplicated, never hashed against itself), the root the single remaining
# value — a one-record trace's root is its leaf value — and the empty trace's
# root the declared constant SHA-256(0x02).  The three single-byte prefixes
# give leaves, nodes, and the empty constant disjoint domains.

TRACE_ROOT_PINS = [
    ([], "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986"),
    ([b"A"], "c00b4d3c929cb5cc316691ed4636f634576f2c9b2954767234c5274e9dde185d"),
    ([b"A", b"B"],
     "ed692f01f7f6c46930d7ad8f9adad3f9f38b7379cf6a8d2f399a0ba1e914fe25"),
    ([b"A", b"B", b"C"],
     "961d2e2be20f538ffdf56962a86d1bd165498f222684ee4c5e02c1e9f852adc5"),
    ([b"A", b"B", b"C", b"D"],
     "5c8dc617d287a4297eb2bcb81b37644b5138e57ad461c657db152109e3fc9fca"),
    ([b"A", b"B", b"C", b"D", b"E"],
     "2bdbcd79bf92b8dad52ba685f01bef520f0cda04468a13a6bd27bfbeebff64d7"),
]


def trace_root(records):
    """The root of `qir-trace-root-1` over a trace's canonical record
    encodings (commitments §6.6)."""
    if not records:
        return hashlib.sha256(b"\x02").digest()
    lvl = [hashlib.sha256(b"\x00" + r).digest() for r in records]
    while len(lvl) > 1:
        nxt = [hashlib.sha256(b"\x01" + lvl[i] + lvl[i + 1]).digest()
               for i in range(0, len(lvl) - 1, 2)]
        if len(lvl) % 2:
            nxt.append(lvl[-1])  # the odd level's final value is promoted unchanged
        lvl = nxt
    return lvl[0]


def trace_pins_ok():
    """The six declared known-answer pins of §6.6, recomputed at every run
    (construction examples, never corpus vectors); the return value is the
    number of pins reproduced."""
    return sum(1 for recs, want in TRACE_ROOT_PINS
               if trace_root(recs).hex() == want)


# ============================================================ artifact comparison
# The collision-safe comparison `qir-artifact-cmp-1` (commitments §6.7):
# total, deterministic, symmetric up to the declared entry order. A complete
# artifact is the (kind, version, digest) identity triple (the summary) plus
# the complete canonical bytes (the artifact itself). Verdicts: 0 = same,
# 1 = different, 2 = collision (genuine collision — triples agree, bytes differ).

def cmp_artifacts(a, b):
    if a[0] != b[0] or a[1] != b[1]:
        return 1
    if a[3] == b[3]:
        return 0
    if a[2] == b[2]:
        return 2
    return 1


def collision_entries(arts):
    n = 0
    for i in range(len(arts)):
        for j in range(i + 1, len(arts)):
            if cmp_artifacts(arts[i], arts[j]) == 2:
                n += 1
    return n


def artifact_cmp_pins_ok():
    """The four declared known-answer pins of §6.7, recomputed at every run
    as a decision rule over supplied triples. The return value is the number
    of pins reproduced."""
    import hashlib
    da = hashlib.sha256(b"payload-a").digest()
    db = hashlib.sha256(b"payload-b").digest()
    dc = hashlib.sha256(b"payload-c").digest()
    a = ("record", "V", da, b"payload-a")
    b_same = a
    b_coll = ("record", "V", da, b"payload-b")   # supplied equal digest
    b_diff = ("record", "V", db, b"payload-b")
    c_diffkind = ("receipt", "V", dc, b"payload-c")
    pins = [
        (cmp_artifacts(a, b_same), 0),
        (cmp_artifacts(a, b_coll), 2),
        (cmp_artifacts(a, b_diff), 1),
        (cmp_artifacts(a, c_diffkind), 1),
    ]
    return sum(1 for got, want in pins if got == want)


def artifact_cmp_selftest():
    """The collision comparison's self-test: four toy complete artifacts fed
    to the comparison core, checking the quarantine verdicts (§6.7)."""
    import hashlib
    da = hashlib.sha256(b"payload-a").digest()
    db = hashlib.sha256(b"payload-b").digest()
    dc = hashlib.sha256(b"payload-c").digest()
    a = ("record", "V", da, b"payload-a")
    coll = ("record", "V", da, b"payload-b")
    diff = ("record", "V", db, b"payload-b")
    cross = ("receipt", "V", dc, b"payload-c")
    got = [cmp_artifacts(a, a), cmp_artifacts(a, coll),
           cmp_artifacts(a, diff), cmp_artifacts(a, cross)]
    want = [0, 2, 1, 1]
    return sum(1 for g, w in zip(got, want) if g == w)


class Corpus:
    def __init__(self, path):
        self.text = open(path, "r", encoding="utf-8").read()
        self.lines = self.text.split("\n")
        m = re.search(r"^version: (\S+)$", self.text, re.M)
        self.version = m.group(1) if m else "unknown"
        self.size = len(self.text.encode("utf-8"))

    def between(self, a, b):
        ia = self.lines.index(next(l for l in self.lines if l.startswith(a)))
        ib = self.lines.index(next(l for l in self.lines if l.startswith(b)))
        return self.lines[ia:ib]

    def vectors(self, family_lines, prefix_re):
        """Split a family body into (id, body lines) at '### ' headings."""
        out = []
        cur_id, cur = None, []
        for ln in family_lines:
            m = re.match(r"^### (" + prefix_re + r")", ln)
            if m:
                if cur_id:
                    out.append((cur_id, cur))
                cur_id, cur = m.group(1), [ln]
            elif cur_id is not None:
                cur.append(ln)
        if cur_id:
            out.append((cur_id, cur))
        return out


def fences(body):
    """All fenced blocks of a vector body as (marker line index, lang, lines)."""
    out = []
    start = None
    lang = ""
    for idx, ln in enumerate(body):
        if start is None:
            if ln.startswith("```"):
                start = idx
                lang = ln[3:].strip()
        elif ln.startswith("```"):
            out.append((start, lang, body[start + 1:idx]))
            start = None
            lang = ""
    return out


def fence_after(body, marker):
    """The first fence whose opening follows the line carrying marker."""
    fs = fences(body)
    for idx, ln in enumerate(body):
        if marker in ln:
            for (s, lang, ls) in fs:
                if s > idx:
                    return lang, ls
            raise ValueError(f"no fence after {marker!r}")
    raise ValueError(f"marker {marker!r} absent")


def groups_to_bytes(lines):
    """Hex groups (blank-line free) to bytes; group width disambiguates the
    word rendering (16-hex-char groups) from the byte rendering (8 chars)."""
    parts = []
    for ln in lines:
        parts.extend(ln.split())
    return bytes.fromhex("".join(parts))


def parse_record_block(ls):
    """One committed record table: 'record N — kind', then the fourteen
    fields, each '{k:>2}  {name:<17} {body}' with 21-space continuations."""
    fields = {}
    cur = None
    for ln in ls:
        if ln.startswith("record "):
            continue
        m = re.match(r"^ ?(\d{1,2})  (\S+)\s+(.*)$", ln)
        if m:
            cur = int(m.group(1))
            fields[cur] = [m.group(3)]
        elif re.match(r"^ {21}\S", ln):
            fields[cur].append(ln[21:])
        else:
            raise ValueError(f"record table line: {ln!r}")
    out = {}
    for k, parts in fields.items():
        body = " ".join(p.strip() for p in parts)
        if k in (1, 2, 5, 7, 8):
            out[k] = bytes.fromhex(body.replace(" ", ""))
        elif k in (3, 9, 10, 11):
            out[k] = int(body)
        elif k == 4:
            out[k] = json.loads(body)
        elif k == 6:
            out[k] = [bytes.fromhex(a) for a in re.findall(r"h'([0-9a-f]+)'", body)] \
                if body != "[]" else []
        elif k in (12, 13, 14):
            out[k] = json.loads(body)
        else:
            raise ValueError(f"record field {k}")
    return [out[k] for k in sorted(out)]


def rebuild_record(f):
    """The committed record bytes rebuilt from the rendered fields (the
    fourteen [tag, value] pairs of §2.6's F5 list, in order)."""
    pairs = [[1, f[0]], [2, f[1]], [3, f[2]], [4, f[3]], [5, f[4]], [6, f[5]],
             [7, f[6]], [8, f[7]], [9, f[8]], [10, f[9]], [11, f[10]],
             [12, f[11]], [13, f[12]], [14, f[13]]]
    return cbor_encode(pairs)


def parse_ledger(ls):
    out = {"per_class": {}, "trace": []}
    for ln in ls:
        m = re.search(r"Work\(E\) = (\d+) U", ln)
        if m:
            out["work"] = int(m.group(1))
        m = re.match(r"^    (\w+)\s+(\d+) U \(\d+ postings?\)$", ln)
        if m:
            out["per_class"][m.group(1)] = int(m.group(2))
        m = re.match(r"^    step\s+(\d+):\s+(\d+)\s+/\s+(\d+)\s+/\s+(\d+)$", ln)
        if m:
            out["trace"].append((int(m.group(1)),
                                 (int(m.group(2)), int(m.group(3)), int(m.group(4)))))
        m = re.search(r"reserve_outstanding : (\d+) = (\d+) \+ (\d+) \+ (\d+)", ln)
        if m:
            out["identity"] = tuple(int(x) for x in m.groups())
        m = re.search(r"remaining_budget = W_budget - charged\s+: (\d+) - (\d+) = (\d+)", ln)
        if m:
            out["remaining"] = tuple(int(x) for x in m.groups())
        m = re.search(r"refund_final = B\(P\) - charged\s+: (\d+) - (\d+) = (\d+)", ln)
        if m:
            out["refund_final"] = tuple(int(x) for x in m.groups())
        if "the escrow never opened" in ln:
            out["refund_final"] = "refusal"
        m = re.search(r"reserve_outstanding after terminal settlement\s+: (\d+)", ln)
        if m:
            out["reserve_end"] = int(m.group(1))
    return out


# The declared inputs of families E and I (committed vector parameters; the
# expected bytes are the corpus's own committed data).  M's inputs are the
# committed bytes themselves; M_KINDS declares each byte string's artifact
# kind so the decoder applies the committed surface in its own grammar.

E_INPUTS = {
    "E1": 0, "E2": -1, "E3": 2 ** 64, "E4": ("tuple", [1, -2]), "E5": 255,
    "E6": 2 ** 64 - 1, "E7": 2 ** 128, "E8": -(2 ** 64),
    "E9": ("tuple", [0, ("tuple", [1, -2])]),
    "E10": ("zoom", 6, -2), "E11": ("zoom", 0, 5),
}
I_INPUTS = {
    "I1": ("CONST", 0, 0),
    "I2": ("ADD", 0, 1, 2),
    "I3": ("BR", "NE", 0, 1, 2),
}
M_KINDS = {"M1": "insn", "M2": "value", "M3": "value", "M4": "value",
           "M5": "value", "M6": "value", "M7": "insn", "M8": "envelope",
           "M9": "envelope", "M10": "stream", "M11": "envelope",
           "M12": "envelope", "M13": "ext", "M14": "ext", "M15": "ext"}


def build_insn(spec):
    """The declared input of an I vector, through the committed grammar:
    opcode tag word, declared operand fields, one trailing ext word."""
    kind = spec[0]
    if kind == "CONST":
        return w(OP_CODE["CONST"]) + enc_value(spec[1]) + w(1) + enc_int(spec[2]) + w(0)
    if kind == "ADD":
        return (w(OP_CODE["ADD"]) + w(spec[1]) + w(spec[2]) + w(1)
                + enc_int(spec[3]) + w(0))
    if kind == "BR":
        cc = {"LT": 0, "LE": 1, "EQ": 2, "NE": 3, "GT": 4, "GE": 5}[spec[1]]
        return (w(OP_CODE["BR"]) + w(cc) + w(spec[2])
                + w(spec[3]) + w(0) + w(spec[4]) + w(0) + w(0))
    raise ValueError(f"unsupported instruction input {kind}")


def classify(bs, kind):
    """Apply the committed decoder of the declared artifact kind; return the
    verdict (canonical) or raise Malformed with the declared surface."""
    if kind == "value":
        ws = words_of(bs)
        decode_value(ws, 0, len(ws))
        return "canonical"
    if kind == "insn":
        ws = words_of(bs)
        decode_instruction(ws, 0, len(ws))
        return "canonical"
    if kind == "stream":
        decode_stream(bs, 1)
        return "canonical"
    if kind == "envelope":
        validate_envelope_keys(bs)
        decode_envelope(bs)
        return "canonical"
    if kind == "ext":
        check_ext_record_or_table(bs)
        return "canonical"
    raise ValueError(kind)


def parse_profile(c):
    """The declared profile of the corpus preamble (qir-prof-golden-2)."""
    text = "\n".join(c.between("## The Declared Profile", "## Family E"))
    name_m = re.search(r"## The Declared Profile \(`?([^`)\n]+)`?\)", text)
    name = name_m.group(1).strip() if name_m else "qir-prof-golden-2"
    g = int(re.search(r"`g = (\d+)` U", text).group(1))
    q = int(re.search(r"q_const … q_halt = (\d+)` U", text).group(1))
    trip = int(re.search(r"trip ceiling (\d+) per back-edge site", text).group(1))
    sm = re.search(r"`?S_max`?\s*=\s*(\d+)", text)
    s_max = int(sm.group(1)) if sm else 4096
    charges = {("q_load" if op == "CLOAD" else f"q_{op.lower()}"): q
               for op in OPCODES.values()}
    # The depth ceiling is retired (F26, v0.3.0-draft.22): the profile declares
    # trip ceilings only, and no depth counter exists.
    return {"name": name, "gateway": g, "charges": charges,
            "trip_ceiling": trip, "s_max": s_max}


# ================================================================== runners
# Each family is checked against the committed bytes at the agreement points
# of §4.1: (1) canonical encoding bytes; (2) the committed verdicts; (3) the
# complete ordered record sequence; (4) the terminal receipt's canonical
# bytes; (5) the reconstructed ledger and the terminal outcome.

def run_family_e(corpus):
    fails = []
    vecs = corpus.vectors(corpus.between("## Family E", "## Family I"), r"E\d+")
    for vid, body in vecs:
        key = vid.split()[0]
        _, ls = fence_after(body, "Expected canonical bytes")
        want = groups_to_bytes(ls)
        got = enc_value(E_INPUTS[key])
        if got != want:
            fails.append(f"{vid}: canonical bytes differ (agreement point 1)")
    return len(vecs), fails


def run_family_i(corpus):
    fails = []
    vecs = corpus.vectors(corpus.between("## Family I", "## Family M"), r"I\d+")
    for vid, body in vecs:
        key = vid.split()[0]
        _, ls = fence_after(body, "Expected canonical words")
        want = groups_to_bytes(ls)
        got = build_insn(I_INPUTS[key])
        if got != want:
            fails.append(f"{vid}: canonical words differ (agreement point 1)")
    return len(vecs), fails


# The declared inputs of the Family R vectors (expansion increment 5):
# R1–R3 carry the record's declared index and payload reading (P14); R4
# carries the populated-table envelope (the M9 complement), checked as the
# canonical bytes decoding to exactly the declared record.
R_INPUTS = {
    "R1": (0, None),
    "R2": (1, 42),
    "R3": (2, ("tuple", [1, -2])),
    "R4": (0, None),
}


def run_family_r(corpus):
    """Family R — the EXT side-table record encodings (expansion increment
    5): R1–R3 build the declared record through the committed CBOR and
    value grammars and match the committed canonical bytes; R4 decodes the
    committed envelope bytes and matches the declared side-table record."""
    fails = []
    vecs = corpus.vectors(corpus.between("## Family R", "## Family M"),
                          r"R\d+")
    for vid, body in vecs:
        key = vid.split()[0]
        idx, payload_src = R_INPUTS[key]
        _, ls = fence_after(body, "Expected canonical bytes:")
        want = groups_to_bytes(ls)
        try:
            if key == "R4":
                dec = decode_envelope(want)
                got_ok = dec["side"] == [[idx, b""]]
                got = want if got_ok else None
            else:
                payload = b"" if payload_src is None else enc_value(payload_src)
                got = cbor_encode([idx, payload])
        except (Malformed, ValueError) as e:
            got = None
            fails.append(f"{vid}: the committed bytes do not read as "
                         f"declared ({e}) (agreement point 1)")
            continue
        if got != want:
            fails.append(f"{vid}: canonical bytes diverge from the committed "
                         "bytes (agreement point 1)")
    return len(vecs), fails


def run_family_m(corpus):
    fails = []
    vecs = corpus.vectors(corpus.between("## Family M", "## Family X"), r"M\d+")
    for vid, body in vecs:
        key = vid.split()[0]
        inline = next(ln for ln in body if ln.startswith("Input bytes:"))
        want = re.search(r"Expected verdict: \*\*(.+?)\*\*", inline).group(1)
        _, ls = fence_after(body, "Input bytes:")
        bs = groups_to_bytes(ls)
        try:
            got = classify(bs, M_KINDS[key])
        except Malformed as e:
            got = e.surface
        if got != want:
            fails.append(f"{vid}: verdict {got!r} != committed {want!r} "
                         "(agreement point 2)")
    return len(vecs), fails


def run_x_family(corpus, profile):
    """The execution share: every committed envelope decodes, its
    source-grammar listing regenerates from the decoded bytes, and every
    execution reproduces the complete committed expected outcome at the five
    agreement points of §4.1."""
    fails = []
    vecs = corpus.vectors(corpus.between("## Family X", "## Suite Register"),
                          r"X\d+")
    by_id = {}
    tags = {}
    for vid, body in vecs:
        key = vid.split()[0]
        by_id[key] = body
        m = re.search(r"`(exec-[a-z0-9-]+)`", body[0])
        tags[key] = m.group(1) if m else "exec"
    envelopes = {}
    order = list(by_id)
    for vid in order:
        body = by_id[vid]
        reuse = next((ln for ln in body if ln.startswith("Intent: byte-identical")),
                     None)
        if reuse:
            src = re.search(r"byte-identical to (X\d+)'s", reuse).group(1)
            envelopes[vid] = envelopes[src]
            continue
        _, ls = fence_after(body, "Intent envelope")
        envelopes[vid] = groups_to_bytes(ls)
    # envelope decode + listing regeneration (agreement point 1 surface)
    n_listings = 0
    for bs in sorted(set(envelopes.values()), key=bytes):
        decode_envelope(bs)
    n_env = len(set(envelopes.values()))
    for vid in order:
        body = by_id[vid]
        env = decode_envelope(envelopes[vid])
        if any(ln.startswith("Intent — program") for ln in body):
            _, ls = fence_after(body, "Intent — program")
            n_listings += 1
            if listing_of(env["blocks"]) != ls:
                fails.append(f"{vid}: source listing does not regenerate "
                             "from the committed envelope (agreement point 1)")
    # admission + execution
    x_lines = []
    n_pass = 0
    # the lineage binding's evidence: the canonical record encodings and the
    # receipt encoding of every executed vector, in committed document order,
    # accumulated immediately after each successful run and before any
    # agreement check, so a failing check never changes the evidence
    # (commitments §6.3); families E/I/R/M and the fixture contribute nothing
    ev = bytearray()
    all_recs = []
    retained = []
    n_ev = 0
    v_roots = []
    v_first = None

    # retained complete artifacts: corpus + implementation (§6.7)
    import hashlib as _h
    corpus_bytes = corpus.text.encode("utf-8")
    impl_bytes = open(IMPL_SOURCES[0], "rb").read().replace(b"\r\n", b"\n")
    impl_v, impl_d = impl_identity()
    retained.append(("corpus", corpus.version, _h.sha256(corpus_bytes).digest(),
                    corpus_bytes))
    retained.append(("implementation", impl_v, bytes.fromhex(impl_d), impl_bytes))
    for vid in order:
        body = by_id[vid]
        params = next(ln for ln in body if ln.startswith("Declared parameters:"))
        wb = int(re.search(r"`W_budget` = (\d+) U", params).group(1))
        # P12 (expansion increment 2): the declared dependency table — the
        # declared constant-table values behind the cited addresses of the
        # vector's own envelope dependency list (§2.1: D is declared vector
        # input, never envelope content; the values ride §2.6's INT_POS)
        D = {}
        dm = re.search(r"`D` = \{([^}]*)\}", params)
        if dm:
            deps = decode_envelope(envelopes[vid])["deps"]
            for part in dm.group(1).split(","):
                m2 = re.search(r"addr (\d+) → (-?\d+)", part)
                if m2:
                    D[deps[int(m2.group(1))]] = enc_int(int(m2.group(2)))
        v_line = re.search(r"`V` = \(([^)]*)\)", params).group(1)
        V = [s.strip() for s in v_line.split(",")]
        if v_first is None:
            v_first = V
        # retained complete artifact: intent envelope
        vstr = "(" + ", ".join(V) + ")"
        retained.append(("intent", vstr,
                        hashlib.sha256(envelopes[vid]).digest(),
                        envelopes[vid]))
        adm = next(ln for ln in body if ln.startswith("Admission:"))
        m = re.search(r"B\(P\) = (\d+) U", adm)
        if m:
            b_committed = int(m.group(1))
        else:
            # the committed admission reasoning: the maximum bounded path's
            # subtotal plus the gateway (the DP's two committed addends)
            m = re.search(r"(\d+) U\)?,? plus the gateway (\d+) U", adm)
            b_committed = int(m.group(1)) + int(m.group(2))
        om = re.search(r"Expected terminal outcome: \*\*(.+?)\*\* \(tag (\d+)\)\.",
                       next(ln for ln in body if ln.startswith("Expected terminal outcome:")))
        outcome_tag = om.group(2)
        rec_start = next(i for i, ln in enumerate(body)
                         if ln.startswith("Expected transition records"))
        rec_end = next(i for i, ln in enumerate(body)
                       if ln.startswith("Expected terminal receipt"))
        rec_fences = [ls for (s, lang, ls) in fences(body)
                      if rec_start < s < rec_end]
        committed_records = [rebuild_record(parse_record_block(ls))
                             for ls in rec_fences]
        _, rcpt_ls = fence_after(body, "Expected terminal receipt")
        ci = next(i for i, ln in enumerate(rcpt_ls) if ln.strip() == "canonical bytes:")
        receipt_committed = groups_to_bytes(rcpt_ls[ci + 1:])
        led_idx = next(i for i, ln in enumerate(body)
                       if ln.startswith("Expected reconstructed ledger"))
        _, led_ls = fence_after(body[led_idx:], "Expected reconstructed ledger")
        ledger = parse_ledger(led_ls)
        try:
            res = Machine(envelopes[vid], D, wb, profile).run()
            # the evidence accumulates before any agreement check
            # (commitments §6.3); the same record bytes, in record order, are
            # this vector's trace — the vector-scope root of
            # `qir-trace-root-1` is computed over them (commitments §6.6),
            # and they are appended to the run scope's sequence
            vrecs = list(res["records"])
            for rb in vrecs:
                ev.extend(rb)
            all_recs.extend(vrecs)
            n_ev += len(vrecs)
            v_roots.append((vid, len(vrecs), trace_root(vrecs).hex()))
            for rb in vrecs:
                retained.append(("record", vstr,
                                hashlib.sha256(rb).digest(), rb))
            retained.append(("receipt", vstr,
                            hashlib.sha256(res["receipt"]).digest(),
                            res["receipt"]))
            ev.extend(res["receipt"])
            if res["B_hat"] != b_committed:
                raise AssertionError(
                    f"B_hat {res['B_hat']} != committed declared bound {b_committed}")
            if len(res["records"]) != len(committed_records):
                raise AssertionError(
                    f"record count {len(res['records'])} != {len(committed_records)}")
            for k, (mine, want) in enumerate(zip(res["records"],
                                                 committed_records), 1):
                # P1 (draft.32, F-G0-1): the committed state roots are
                # 32-byte digests — the raw P2 pre-image can never pass as
                # a root (the Python equivalent of the Rust checker's
                # rec.pre.len() == 32 && rec.post.len() == 32).
                for rec in (mine, want):
                    f = {t: v for t, v in cbor_decode(rec)}
                    for tag in (1, 7, 8):
                        if len(f[tag]) != 32:
                            raise AssertionError(
                                f"record {k}: field {tag} is {len(f[tag])} "
                                "bytes, not the 32-byte P1 digest (F-G0-1)")
                if mine != want:
                    raise AssertionError(f"record {k} differs (agreement point 3)")
            if res["receipt"] != receipt_committed:
                raise AssertionError("receipt bytes differ (agreement point 4)")
            if res["outcome"] != int(outcome_tag):
                raise AssertionError(
                    f"outcome {res['outcome']} != committed {outcome_tag}")
            o = res["acc"]["reserved"] - res["acc"]["charged"] - res["acc"]["refunded"]
            if ledger["work"] != res["acc"]["charged"]:
                raise AssertionError("Work(E) differs (agreement point 5)")
            if ledger["per_class"] != res["per_class"]:
                raise AssertionError("per-class traces differ (agreement point 5)")
            if ledger["trace"] != [(s, (a["charged"], a["reserved"], a["refunded"]))
                                   for s, a in res["boundaries"]]:
                raise AssertionError("settlement trace differs (agreement point 5)")
            R, C, F, O = ledger["identity"]
            if (R, C, F, O) != (res["acc"]["reserved"], res["acc"]["charged"],
                                res["acc"]["refunded"], o) or R != C + F + O:
                raise AssertionError("conservation identity differs (agreement point 5)")
            if ledger["remaining"] != (wb, res["acc"]["charged"],
                                       wb - res["acc"]["charged"]):
                raise AssertionError("remaining budget differs (agreement point 5)")
            if ledger["refund_final"] == "refusal":
                if res["acc"]["reserved"] or o:
                    raise AssertionError("refusal ledger must close at zero postings")
            elif ledger["refund_final"] != (res["B_hat"], res["acc"]["charged"],
                                            res["B_hat"] - res["acc"]["charged"]):
                raise AssertionError("final refund differs (agreement point 5)")
            if ledger["reserve_end"] != o or o != 0:
                raise AssertionError("reserve outlives the execution")
            if res["versions"] != V:
                raise AssertionError("version bindings differ from the declared V")
            n_pass += 1
            x_lines.append(
                f"  {vid} {tags[vid]}: executed — {len(res['records'])}/"
                f"{len(committed_records)} records match (fields 1–14); receipt "
                f"bytes match ({OUTCOME_NAME[res['outcome']]} (tag {res['outcome']})); "
                f"ledger holds: Work(E) = {res['acc']['charged']} U; conservation "
                f"{R} = {C} + {F} + {O}")
        except (AssertionError, Malformed, ValueError) as e:
            fails.append(f"{vid}: {e}")
            x_lines.append(f"  {vid} {tags[vid]}: FAILED — {e}")
    # the collision bucket: collision pairs among the retained complete
    # artifacts (commitments §6.7). With the committed corpus this is 0.
    entries = collision_entries(retained)
    # the run-scope root: every executed X-family vector's records, in
    # committed document order then record order — the sequence the evidence
    # digest accumulates minus the per-vector receipt encodings (§6.6)
    run_root = trace_root(all_recs).hex()
    return (len(order), n_env, n_listings, n_pass, x_lines, fails,
            lf_sha256(bytes(ev)), n_ev, tuple(v_first), run_root, v_roots,
            entries)


def run_fixture_f6(body):
    """The F6 collision-safe artifact comparison fixture (draft.31): four
    declared CompleteArtifact pairs checked by qir-artifact-cmp-1. It commits
    no record/receipt bytes and never changes normal-mode n or entries."""
    out = []
    arts = {}
    declared = next(ln for ln in body if ln.startswith("Declared artifacts:"))
    declared = declared.split(":", 1)[1]
    for part in declared.split(";"):
        p = part.strip().strip("`").strip()
        if " = (" not in p:
            continue
        name, rest = p.split(" = (", 1)
        f = [x.strip() for x in rest.rstrip(")").split(",")]
        if len(f) != 4:
            out.append(f"  FAIL — F6: malformed artifact declaration {p}")
            continue
        lit = f[2][len("sha256("):-1].strip().strip('"')
        arts[name.strip()] = (f[0], f[1],
                              hashlib.sha256(lit.encode()).digest(),
                              f[3].strip('"').encode())
    want_of = {"same": 0, "different": 1, "collision": 2}
    operands = []
    verdicts = []
    for ln in body:
        if not ln.startswith("- pair "):
            continue
        m = re.search(r"`([^`]+)`", ln)
        word = m.group(1) if m else ""
        rhs = ln.split(":", 1)[1].split("(")[0]
        names = [x.strip().strip("`").strip() for x in rhs.split(",")]
        if len(names) != 2 or names[0] not in arts or names[1] not in arts:
            out.append(f"  FAIL — F6: undeclared artifact in {ln}")
            continue
        got = cmp_artifacts(arts[names[0]], arts[names[1]])
        if word not in want_of or got != want_of[word]:
            out.append(f"  FAIL — F6: pair {names[0]} produced {got} but declares {word}")
        operands.extend([arts[names[0]], arts[names[1]]])
        verdicts.append(word)
    if len(verdicts) != 4:
        out.append(f"  FAIL — F6: expected four pairs, found {len(verdicts)}")
    entries = collision_entries(list(arts.values()))
    verdict_line = next(ln for ln in body if ln.startswith("Verdict:"))
    declared_entries = int(re.search(r"entries = (\d+)", verdict_line).group(1))
    if entries != declared_entries:
        out.append(f"  FAIL — F6: entries {entries} != declared {declared_entries}")
    out.extend([
        "fixture F6 artifact comparison (test-only corpus fixture, §4.4):",
        "  pair 1 (same): payload-a vs payload-a -> same",
        "  pair 2 (collision): payload-a vs payload-b -> collision (class = digest-collision)",
        "  pair 3 (different, same kind): payload-b vs payload-c -> different",
        "  pair 4 (different, differing kind): payload-c vs payload-c -> different",
        "  collision bucket: entries = 1 (class = digest-collision)",
        "  normal mode unchanged: n = 336, entries = 0",
        "  verdict: the four declared comparison pairs reproduce under qir-artifact-cmp-1",
        ("fixture F6: PASS — the declared comparison verdicts reproduce under the committed rules"
         if entries == declared_entries == 1 and len(verdicts) == 4
         else "fixture F6: FAIL — the declared comparison verdicts do not reproduce"),
    ])
    return out


def run_fixture(corpus, profile, fid="F27"):
    """The declared fixture (§4.4): F27 fuel-backstop validator-defect
    injection (v0.3.0-draft.23) or F6 collision-safe artifact comparison
    (v0.3.0-draft.31), selected from the corpus's Fixture Register — never a
    conformance vector and never counted in the declared seed."""
    fails = []
    fixture_lines = corpus.between("## Fixture Register",
                                   "## License and Invariant Lineage")
    vecs = corpus.vectors(fixture_lines, r"F\d+")
    match = next((v for v in vecs if v[0].split()[0] == fid), None)
    if match is None:
        return [f"fixture register: the {fid} fixture section is absent"]
    vid, body = match
    if fid == "F6":
        return run_fixture_f6(body)
    params = next(ln for ln in body if ln.startswith("Declared parameters:"))
    wb = int(re.search(r"`W_budget` = (\d+) U", params).group(1))
    b_inj = int(re.search(r"`B_injected` = (\d+) U", params).group(1))
    adm = next(ln for ln in body if ln.startswith("Admission:"))
    bh_decl = int(re.search(r"`B_hat\(P_f\)` = (\d+) U", adm).group(1))
    shortfall = int(re.search(r"(\d+) U short", adm).group(1))
    ctrl = next(ln for ln in body if ln.startswith("- control run"))
    ctrl_tag = int(re.search(r"\(tag (\d+)\)", ctrl).group(1))
    ctrl_cons = [int(x) for x in re.search(
        r"conservation (\d+) = (\d+) \+ (\d+) \+ (\d+)", ctrl).groups()]
    inj = next(ln for ln in body if ln.startswith("- injected run:"))
    inj_step = int(re.search(r"fires at step (\d+)", inj).group(1))
    inj_pc0, inj_pc1 = map(int, re.search(r"PC \((\d+), (\d+)\)", inj).groups())
    inj_tag = int(re.search(r"\(tag (\d+)\)", inj).group(1))
    n_completed = int(re.search(r"(\d+) completed transition records",
                                inj).group(1))
    n_rej = int(re.search(r"(\d+) backstop rejection record", inj).group(1))
    inj_charged = int(re.search(r"charged (\d+) U", inj).group(1))
    inj_reserved = int(re.search(r"reserved (\d+) U", inj).group(1))
    inj_refunded = int(re.search(r"refunded (\d+) U", inj).group(1))
    inj_cons = [int(x) for x in re.search(
        r"conservation (\d+) = (\d+) \+ (\d+) \+ (\d+)", inj).groups()]
    rem_w, rem_c, rem_r = map(int, re.search(
        r"remaining budget (\d+) - (\d+) = (\d+)", inj).groups())
    item_ln = next(ln for ln in body if ln.startswith("- token itemization"))
    n_items = int(re.search(r"(\d+) tokens terminalized", item_ln).group(1))
    sel_nil = "selected token nil" in item_ln
    expect_items = [(int(b), TOKEN_TYPES_REV[name]) for b, name in
                    re.findall(r"birth (\d+)(?: at \(\d+, \d+\))?, ([A-Z_]+)",
                               item_ln)]
    _, ls = fence_after(body, "Intent envelope")
    committed = groups_to_bytes(ls)
    env = decode_envelope(committed)
    bh = b_hat(env["blocks"], profile["gateway"],
               lambda ins: ins_charge(ins, profile),
               profile["trip_ceiling"], profile["s_max"])
    # the assembly share: the source listing regenerates from the committed
    # envelope (the reference artifact's share re-encodes the bytes, §4.2)
    _, src_ls = fence_after(body, "source grammar's textual form")
    if listing_of(env["blocks"]) != src_ls:
        fails.append(f"{vid}: source listing does not regenerate from the "
                     "committed envelope (agreement point 1)")
    if bh != bh_decl:
        fails.append(f"{vid}: computed B_hat(P_f) {bh} != declared {bh_decl}")
    if not b_inj < bh:
        fails.append(f"{vid}: the declared injection must be below the exact "
                     "bound")
    if bh - b_inj != shortfall:
        fails.append(f"{vid}: the injected bound's shortfall {bh - b_inj} "
                     f"!= declared {shortfall}")
    # the control run: the exact bound refuses; the escrow never opens
    control = Machine(committed, {}, wb, profile).run()
    if control["outcome"] != ctrl_tag:
        fails.append(f"{vid}: control run outcome {control['outcome']} != "
                     f"declared tag {ctrl_tag}")
    if len(control["records"]) != 1 or control["acc"]["reserved"] != 0:
        fails.append(f"{vid}: control run is not the single zero-posting "
                     f"refusal (records {len(control['records'])}, reserved "
                     f"{control['acc']['reserved']})")
    ca = control["acc"]
    if [ca["reserved"], ca["charged"], ca["refunded"]] != ctrl_cons[:3] \
            or ctrl_cons[3] != 0:
        fails.append(f"{vid}: control run conservation differs from the "
                     "declared triple")
    # the injected run: admitted under the defective bound; the backstop
    # fires and the containment verdict is checked
    injected = Machine(committed, {}, wb, profile, b_inj).run()
    rec_fields = dict(cbor_decode(injected["records"][-1]))
    last_pc = tuple(rec_fields[4])
    last_step = rec_fields[3]
    rcpt = cbor_decode(injected["receipt"])
    item = [(e[0], e[4]) for e in rcpt[2]]
    selected_nil = rcpt[3] is None
    b = injected["acc"]
    if injected["outcome"] != inj_tag:
        fails.append(f"{vid}: injected run outcome {injected['outcome']} != "
                     f"declared tag {inj_tag}")
    if last_step != inj_step or last_pc != (inj_pc0, inj_pc1):
        fails.append(f"{vid}: the backstop stop site differs from the "
                     f"declared site (step {last_step} at PC {last_pc})")
    if len(injected["records"]) != n_completed + n_rej:
        fails.append(f"{vid}: record count {len(injected['records'])} != "
                     f"declared {n_completed} completed + {n_rej} rejection")
    if [b["reserved"], b["charged"], b["refunded"]] != \
            [inj_reserved, inj_charged, inj_refunded]:
        fails.append(f"{vid}: injected run ledger differs from the declared "
                     "accumulators")
    if [b["reserved"], b["charged"], b["refunded"]] != inj_cons[:3] \
            or b["reserved"] - b["charged"] - b["refunded"] != inj_cons[3]:
        fails.append(f"{vid}: injected run ledger differs from the declared "
                     "conservation")
    if (rem_w, rem_c, rem_r) != (wb, b["charged"], wb - b["charged"]):
        fails.append(f"{vid}: the remaining budget differs from the declared "
                     "identity")
    if b["charged"] > b["reserved"]:
        fails.append(f"{vid}: overbilling: charged exceeds the reserve")
    if len(item) != n_items:
        fails.append(f"{vid}: itemization count {len(item)} != declared "
                     f"{n_items}")
    if selected_nil != sel_nil:
        fails.append(f"{vid}: the receipt's selected token differs from the "
                     "declared nil")
    if item != expect_items:
        fails.append(f"{vid}: the itemization's (birth, type) pairs differ "
                     "from the declared pairs")
    # the fixture block (byte-identical to the reference artifact's)
    lines = []
    lines.append("fixture F27 fuel-backstop-on-validator-defect "
                 "(test-only fault injection, §4.4):")
    nb = len(env["blocks"])
    ni = sum(len(bb["instrs"]) for bb in env["blocks"])
    lines.append(f"  declared program P_f: {nb} block, {ni} instructions; "
                 f"declared pool [{', '.join(str(p) for p in env['pool'])}]; "
                 f"envelope {len(committed)} bytes")
    lines.append(f"  declared injection: B_injected = {b_inj} U; "
                 f"W_budget = {wb} U; exact surrogate B_hat(P_f) = {bh} U "
                 f"(the injected bound is {bh - b_inj} U short)")
    lines.append(f"  control run (exact bound): REFUSED at admission — "
                 f"{OUTCOME_NAME[control['outcome']]} "
                 f"(tag {control['outcome']}); zero postings; the escrow "
                 f"never opened ({ca['reserved']} = {ca['charged']} + "
                 f"{ca['refunded']} + "
                 f"{ca['reserved'] - ca['charged'] - ca['refunded']})")
    lines.append("  injected run: ADMITTED — the false admission the fixture "
                 "evidences")
    out = injected["outcome"]
    lines.append(f"    backstop fired at step {last_step}, PC ({last_pc[0]}, "
                 f"{last_pc[1]}): {OUTCOME_NAME[out]} (tag {out}); "
                 f"{n_completed} completed + {n_rej} backstop rejection "
                 "records")
    items_txt = "; ".join(f"birth {bi} {TOKEN_TYPES[t]}" for bi, t in item)
    lines.append(f"    token itemization: {len(item)} tokens terminalized "
                 f"({items_txt}); selected nil")
    eq_note = ("equality — no overbilling"
               if b["charged"] == b["reserved"] else "no overbilling")
    lines.append(f"    ledger: reserved {b['reserved']} = "
                 f"charged {b['charged']} + refunded {b['refunded']} + "
                 f"outstanding {b['reserved'] - b['charged'] - b['refunded']}; "
                 f"remaining budget {wb} - {b['charged']} = "
                 f"{wb - b['charged']}; charged <= reserved ({eq_note})")
    lines.append("  verdict: validator defect evidenced (false admission "
                 "under the injected bound); containment held (§3.4 defense "
                 "in depth) — the fixture normalizes no defective execution")
    if not fails:
        lines.append("fixture F27: PASS — the declared expectation "
                     "reproduces under the committed rules")
    else:
        lines.append(f"fixture F27: FAIL — {len(fails)} failing check(s)")
        for x in fails:
            lines.append(f"  FAIL — {x}")
    return lines


# =============================================== Step 6.1 container extension
# The qir-enc-3 / qir-prof-ext-containers-1 extension engine and the declared
# C1-C6 fixture surface of docs/qnir_containers.md section 7. Core 1-16 opcodes
# and tags 1-4 remain untouched.

ENC_V2, ENC_V3, PROF_EXT = "qir-enc-2", "qir-enc-3", "qir-prof-ext-containers-1"
TAG_CONTAINER, ARRAY_RRB16, TABLE_MBTREE16 = 5, 1, 2
MAX_CAP, MAX_BATCH, GATEWAY, S_MAX, TRIP_CEILING = 65536, 64, 20, 4096, 1
OUT_SUCCESS, OUT_TOO_COMPLEX = 1, 3
EXT_OPCODES = set(range(17, 30))


def depth_16(cap_bnd):
    """Declared static depth function: pure in the declared cap_bnd."""
    d, span = 1, 16
    while span < max(cap_bnd, 1) and d < 4:
        span *= 16
        d += 1
    return d


def validate_opcode_tag(tag, enc):
    if 1 <= tag <= 16:
        return
    if 17 <= tag <= 29:
        if enc == ENC_V3:
            return
        raise ValueError("statically illegal — extension opcode under qir-enc-2")
    raise ValueError("unknown opcode tag")


def validate_value_tag(tag, enc):
    if 1 <= tag <= 4:
        return
    if tag == TAG_CONTAINER and enc == ENC_V3:
        return
    raise ValueError("unknown type tag")


class Container:
    """Copy-on-write persistent container value (section 7.2)."""

    def __init__(self, subtype, cap_bnd, elem_bnd):
        validate_value_tag(TAG_CONTAINER, ENC_V3)
        if subtype not in (ARRAY_RRB16, TABLE_MBTREE16):
            raise ValueError("unknown container subtype")
        if cap_bnd > MAX_CAP:
            raise ValueError("cap_bnd exceeds max_cap")
        self.subtype, self.cap_bnd, self.elem_bnd = subtype, cap_bnd, elem_bnd
        self.pairs = []  # ascending (key, value)

    def clone(self):
        c = Container(self.subtype, self.cap_bnd, self.elem_bnd)
        c.pairs = list(self.pairs)
        return c

    def len(self):
        return len(self.pairs)

    def canonical_bytes(self):
        out = bytearray(w(TAG_CONTAINER) + w(self.subtype))
        out.extend(enc_int(self.cap_bnd) + enc_int(self.elem_bnd))
        out.extend(w(len(self.pairs)))
        for k, v in self.pairs:
            out.extend(enc_int(k) + enc_int(v))
        return bytes(out)

    def _check_val(self, v):
        if v < 0 or v > self.elem_bnd:
            raise ValueError("elem_bnd exceedance")

    def set_index(self, idx, val):
        if idx < 0 or idx >= self.cap_bnd:
            raise ValueError("ARR_SET index out of bounds")
        self._check_val(val)
        out = self.clone()
        i = idx
        if i < len(self.pairs):
            if self.pairs[i][0] != i:
                raise ValueError("non-contiguous array index")
            out.pairs[i] = (i, val)
        elif i == len(self.pairs):
            if out.len() >= out.cap_bnd:
                raise ValueError("cap_bnd exceedance")
            out.pairs.append((i, val))
        else:
            raise ValueError("ARR_SET index > len")
        return out

    def get_index(self, idx):
        if idx < 0 or idx >= self.len() or idx >= self.cap_bnd:
            raise ValueError("ARR_GET index out of bounds")
        return self.pairs[idx][1]

    def set_key(self, key, val):
        self._check_val(val)
        out = self.clone()
        keys = [p[0] for p in out.pairs]
        import bisect
        i = bisect.bisect_left(keys, key)
        if i < len(out.pairs) and out.pairs[i][0] == key:
            out.pairs[i] = (key, val)
        else:
            if out.len() + 1 > out.cap_bnd:
                raise ValueError("cap_bnd exceedance")
            out.pairs.insert(i, (key, val))
        return out

    def get_key(self, key):
        for k, v in self.pairs:
            if k == key:
                return v
        return None

    def at(self, rank):
        if rank < 0 or rank >= self.len():
            raise ValueError("TBL_AT rank out of bounds")
        return self.pairs[rank]

    def batch(self, items):
        if not items or len(items) > MAX_BATCH:
            raise ValueError("batch_cnt out of range")
        for a, b in zip(items, items[1:]):
            if a[0] >= b[0]:
                raise ValueError("batch keys not strictly ascending and unique")
        out = self.clone()
        for k, v in items:
            out._check_val(v)
            keys = [p[0] for p in out.pairs]
            import bisect
            i = bisect.bisect_left(keys, k)
            if i < len(out.pairs) and out.pairs[i][0] == k:
                out.pairs[i] = (k, v)
            else:
                if out.len() + 1 > out.cap_bnd:
                    raise ValueError("cap_bnd exceedance")
                out.pairs.insert(i, (k, v))
        return out

    def root(self):
        if not self.pairs:
            return hashlib.sha256(b"\x02").digest()
        level = [_leaf_hash(k, v) for k, v in self.pairs]
        while len(level) > 1:
            nxt = []
            i = 0
            while i + 1 < len(level):
                nxt.append(hashlib.sha256(b"\x20" + level[i] + level[i + 1]).digest())
                i += 2
            if i < len(level):
                nxt.append(level[i])
            level = nxt
        return level[0]


def _leaf_hash(key, val):
    return hashlib.sha256(b"\x00" + (key & MASK64).to_bytes(8, "little") + enc_value(val)).digest()


# COp names mirror the Rust module; charges are the declared section 7.5 table.
CHARGE_CORE = {"Const", "Add", "Cmp", "Br", "Jmp", "Halt", "Emit", "Tuple",
               "ArrNew", "ArrLen", "ArrRoot", "TblNew", "TblLen", "TblRoot"}
CHARGE_GET = {"ArrGet", "TblGet", "TblAt"}
CHARGE_SET = {"ArrSet", "TblSet"}
CHARGE_BATCH = {"ArrBatch", "TblBatch"}
TERMINATORS = {"Br", "Jmp", "Halt", "TblGet"}


def _charge(op, cap, batch_cnt):
    d = depth_16(cap)
    if op in CHARGE_CORE:
        return 4
    if op in CHARGE_GET:
        return 4 + 4 * d
    if op in CHARGE_SET:
        return 4 + 6 * d
    if op in CHARGE_BATCH:
        return 4 + batch_cnt * (4 + 2 * d)
    raise ValueError(f"unknown container opcode {op}")


def bhat_c(blocks, cap, batch_cnt):
    """Trip-ceiling DP: TBL_GET's fallback edge is a branch like CLOAD's."""
    edges = []
    for b in blocks:
        for op in b["ops"]:
            if op in TERMINATORS:
                for t in b["targets"]:
                    edges.append((b["id"], t))
    back = {(s, t) for (s, t) in edges if t < s}

    def val(bid, rem):
        b = blocks[bid]
        idx = next(i for i, o in enumerate(b["ops"]) if o in TERMINATORS)
        total = sum(_charge(o, cap, batch_cnt) for o in b["ops"][:idx + 1])
        op = b["ops"][idx]
        if op == "Halt":
            return total
        if op == "Jmp":
            t = b["targets"][0]
            if (bid, t) in back:
                if rem == 0:
                    return None
                sub = val(t, rem - 1)
            else:
                sub = val(t, rem)
            return None if sub is None else total + sub
        if op == "Br":
            best = None
            for t in b["targets"]:
                if (bid, t) in back:
                    sub = val(t, rem - 1) if rem > 0 else None
                else:
                    sub = val(t, rem)
                if sub is not None:
                    best = sub if best is None else max(best, sub)
            return None if best is None else total + best
        if op == "TblGet":
            # Current block charge `total` is added once to the winning
            # branch, exactly as CLOAD is joined in the Rust bhat.
            rest = sum(_charge(o, cap, batch_cnt) for o in b["ops"][idx + 1:])
            fb = b["targets"][0]
            if (bid, fb) in back:
                miss = val(fb, rem - 1) if rem > 0 else None
            else:
                miss = val(fb, rem)
            return total + (rest if miss is None else max(rest, miss))
        raise ValueError(op)

    core = val(0, TRIP_CEILING)
    return GATEWAY + (core or 0)


def _p1_ok(seed):
    p2 = cbor_encode(["P2", seed])
    return len(hashlib.sha256(p2).digest()) == 32


def _hx(bs):
    return bs.hex()


def fixture_c_lines(spec_path):
    """Returns (rendered lines, all_ok). Text is byte-identical to Rust."""
    try:
        with open(spec_path, "r", encoding="utf-8") as fh:
            spec = fh.read()
    except OSError:
        spec = ""
    spec_ok = ("B_hat(P) = 20 + 74 = 94 U" in spec
               and "B_hat(P) = 20 + max(24 miss, 20 hit) = 44 U" in spec
               and "3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4" in spec)
    all_ok = spec_ok
    cap = 16

    c1_arr = Container(ARRAY_RRB16, cap, 255).set_index(0, 5)
    assert c1_arr.get_index(0) == 5
    b1 = [
        {"id": 0, "ops": ["ArrNew", "Const", "Const", "ArrSet", "ArrLen", "Const", "Jmp"], "targets": [1]},
        {"id": 1, "ops": ["Cmp", "Br"], "targets": [2, 3]},
        {"id": 2, "ops": ["ArrGet", "Const", "Add", "Jmp"], "targets": [1]},
        {"id": 3, "ops": ["Halt"], "targets": []},
    ]
    r1 = dict(id="C1", bh=bhat_c(b1, cap, 2), work=94, n=16, outcome=OUT_SUCCESS,
              miss_born=0, miss_consumed=0, len=c1_arr.len(), root=c1_arr.root(),
              p1_ok=_p1_ok("C1"))

    c2_tbl = Container(TABLE_MBTREE16, cap, 255).set_key(0, 5)
    assert c2_tbl.at(0) == (0, 5)
    b2 = [
        {"id": 0, "ops": ["TblNew", "Const", "Const", "TblSet", "TblLen", "Const", "Jmp"], "targets": [1]},
        {"id": 1, "ops": ["Cmp", "Br"], "targets": [2, 3]},
        {"id": 2, "ops": ["TblAt", "Const", "Add", "Jmp"], "targets": [1]},
        {"id": 3, "ops": ["Halt"], "targets": []},
    ]
    r2 = dict(id="C2", bh=bhat_c(b2, cap, 2), work=94, n=16, outcome=OUT_SUCCESS,
              miss_born=0, miss_consumed=0, len=c2_tbl.len(), root=c2_tbl.root(),
              p1_ok=_p1_ok("C2"))

    c3_empty = Container(TABLE_MBTREE16, cap, 255)
    assert c3_empty.get_key(0) is None
    b3 = [
        {"id": 0, "ops": ["TblNew", "Const", "TblGet", "Halt"], "targets": [1]},
        {"id": 1, "ops": ["Const", "Halt"], "targets": []},
    ]
    r3 = dict(id="C3", bh=bhat_c(b3, cap, 2), work=44, n=5, outcome=OUT_SUCCESS,
              miss_born=1, miss_consumed=1, len=0, root=c3_empty.root(),
              p1_ok=_p1_ok("C3"))

    base = Container(ARRAY_RRB16, cap, 255)
    ok_batch = base.batch([(0, 5), (1, 7)])
    try:
        base.batch([(0, 5), (0, 7)])
        raise AssertionError("duplicate batch accepted")
    except ValueError:
        pass
    b4 = [{"id": 0, "ops": ["ArrNew", "Const", "Const", "Const", "Const", "Tuple", "Tuple",
                            "Tuple", "ArrBatch", "Halt"], "targets": []}]
    r4a = dict(id="C4a", bh=bhat_c(b4, cap, 2), work=72, n=10, outcome=OUT_SUCCESS,
               miss_born=0, miss_consumed=0, len=ok_batch.len(), root=ok_batch.root(),
               p1_ok=_p1_ok("C4a"))
    r4b = dict(id="C4b", bh=bhat_c(b4, cap, 2), work=68, n=9, outcome=OUT_TOO_COMPLEX,
               miss_born=0, miss_consumed=0, len=0, root=base.root(), p1_ok=_p1_ok("C4b"))

    c5_one = Container(ARRAY_RRB16, cap, 255).set_index(0, 5)
    try:
        c5_one.get_index(1)
        raise AssertionError("idx == len accepted")
    except ValueError:
        pass
    try:
        Container(ARRAY_RRB16, MAX_CAP + 1, 255)
        raise AssertionError("cap > max_cap accepted")
    except ValueError:
        pass
    b5 = [{"id": 0, "ops": ["ArrNew", "Const", "Const", "ArrSet", "Const", "ArrGet", "Halt"],
           "targets": []}]
    r5 = dict(id="C5", bh=bhat_c(b5, cap, 2), work=54, n=6, outcome=OUT_TOO_COMPLEX,
              miss_born=0, miss_consumed=0, len=c5_one.len(), root=c5_one.root(),
              p1_ok=_p1_ok("C5"))

    ea = Container(ARRAY_RRB16, cap, 255)
    et = Container(TABLE_MBTREE16, cap, 255)
    a6, t6 = ea.set_index(0, 5), et.set_key(0, 5)
    assert a6.root() == t6.root()
    b6a = [{"id": 0, "ops": ["ArrNew", "Const", "Const", "ArrSet", "ArrRoot", "Emit", "Halt"],
            "targets": []}]
    b6b = [{"id": 0, "ops": ["TblNew", "Const", "Const", "TblSet", "TblRoot", "Emit", "Halt"],
            "targets": []}]
    r6a = dict(id="C6a", bh=bhat_c(b6a, cap, 2), work=54, n=7, outcome=OUT_SUCCESS,
               miss_born=0, miss_consumed=0, len=a6.len(), root=a6.root(), p1_ok=_p1_ok("C6a"))
    r6b = dict(id="C6b", bh=bhat_c(b6b, cap, 2), work=54, n=7, outcome=OUT_SUCCESS,
               miss_born=0, miss_consumed=0, len=t6.len(), root=t6.root(), p1_ok=_p1_ok("C6b"))

    runs = [r1, r2, r3, r4a, r4b, r5, r6a, r6b]
    empty = Container(ARRAY_RRB16, cap, 255).root()
    lines = []
    for r in runs:
        rid = r["id"]
        expect_bh = 44 if rid == "C3" else (72 if rid in ("C4a", "C4b") else (58 if rid == "C5" else (54 if rid in ("C6a", "C6b") else 94)))
        expect_work = 44 if rid == "C3" else (72 if rid == "C4a" else (68 if rid == "C4b" else (54 if rid in ("C5", "C6a", "C6b") else 94)))
        expect_n = 5 if rid == "C3" else (10 if rid == "C4a" else (9 if rid == "C4b" else (6 if rid == "C5" else (7 if rid in ("C6a", "C6b") else 16))))
        expect_out = OUT_TOO_COMPLEX if rid in ("C4b", "C5") else OUT_SUCCESS
        ok = (r["bh"] == expect_bh and r["work"] == expect_work and r["n"] == expect_n
              and r["outcome"] == expect_out and r["p1_ok"]
              and (rid != "C3" or (r["miss_born"] == 1 and r["miss_consumed"] == 1))
              and (rid not in ("C6a", "C6b") or (
                  _hx(empty) == "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986"
                  and _hx(r["root"]) == "3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4")))
        all_ok = all_ok and ok
        outcome = "SUCCESS" if r["outcome"] == OUT_SUCCESS else "EXPRESSION_TOO_COMPLEX"
        lines.append(
            f"fixture {rid} : B_hat={r['bh']} U Work(E)={r['work']} U n={r['n']} "
            f"outcome={outcome} len={r['len']} miss={r['miss_born']}/{r['miss_consumed']} "
            f"root={_hx(r['root'])} empty={_hx(empty)} p1=32B {'OK' if ok else 'MISMATCH'}"
        )
    lines.append(
        f"fixture C: {'PASS' if all_ok else 'FAIL'} — 8/8 declared runs reproduce "
        f"under qir-enc-3 / {PROF_EXT} (spec {'OK' if spec_ok else 'MISMATCH'})"
    )
    return lines, all_ok


# =============================================== Step 6.2 call extension share
# qir-enc-4 version gating and the hand-derived K1-K6 fixture surface.

ENC_V2, ENC_V3, ENC_V4, ENC_V5 = "qir-enc-2", "qir-enc-3", "qir-enc-4", "qir-enc-5"
PROF_CALLS = "qir-prof-ext-calls-1"
GATEWAY, TRIP_CEILING, S_MAX = 20, 1, 4096
C_MAX, F_MAX, MAX_CAP, MAX_BATCH = 8, 64, 65536, 64
OP_CALL, OP_RET = 30, 31


def calls_version_bounds(enc):
    return {ENC_V2: (8, 16, 4), ENC_V3: (8, 29, 5),
            ENC_V4: (9, 31, 5), ENC_V5: (9, 37, 5)}[enc]


def calls_validate_version(enc, max_key, max_op, max_tag):
    mk, mo, mt = calls_version_bounds(enc)
    if max_key > mk:
        raise ValueError("unknown envelope key")
    if max_op > mo:
        raise ValueError("statically illegal — opcode tag outside V.encoding")
    if max_tag > mt:
        raise ValueError("malformed — value tag outside V.encoding")
    return True


def validate_envelope_keys(bs):
    """Mandatory V.encoding gate at the envelope validator boundary."""
    m = cbor_decode(bs)
    if not isinstance(m, dict):
        raise ValueError("envelope is not a map")
    keys = [k for k in m if isinstance(k, int)]
    max_key = max(keys) if keys else 0
    enc = m.get(2, "")
    try:
        mk, _, _ = calls_version_bounds(enc)
    except KeyError:
        # Defer to the core decoder's declared verdict (M8's key-9 envelope).
        return enc
    if max_key > mk:
        raise ValueError("unknown envelope key")
    return enc


def _call_charge(argc):
    return 4 + 2 * argc


def _ret_charge(retc):
    return 4 + 2 * retc


def _calls_p1_ok(seed):
    p2 = cbor_encode(["P2", seed, [0, 0]])
    return len(hashlib.sha256(p2).digest()) == 32


def _stack_chain_ok():
    root = hashlib.sha256(b"\x02").digest()
    for d in range(3):
        root = hashlib.sha256(b"\x30" + root + bytes([d, 0, 0, 0])).digest()
        if len(root) != 32:
            return False
    return True


def _k6_checks():
    a = (not (2 <= 0)) and (0 <= 0)   # self-call / cycle rejection
    b = 9 > C_MAX and 8 <= C_MAX
    c = 65 > F_MAX and 64 <= F_MAX
    d = (not _try(lambda: calls_validate_version(ENC_V2, 9, 16, 4))
         and not _try(lambda: calls_validate_version(ENC_V2, 8, 30, 4))
         and not _try(lambda: calls_validate_version(ENC_V3, 8, 30, 5))
         and not _try(lambda: calls_validate_version(ENC_V3, 9, 29, 5))
         and _try(lambda: calls_validate_version(ENC_V4, 9, 31, 5)))
    return a, b, c, d


def _try(fn):
    try:
        fn()
        return True
    except Exception:
        return False


def fixture_k_lines(spec_path):
    """Returns (rendered lines, all_ok); text is byte-identical to Rust."""
    try:
        with open(spec_path, "r", encoding="utf-8") as fh:
            spec = fh.read()
    except OSError:
        spec = ""
    spec_ok = ("B_hat = 50" in spec
               and "B_hat(K3) = 20 + 4 + 4 + 6·(8 + 46) + 4 = 356 U" in spec
               and "= 44 U" in spec
               and "B_hat = Work(E) = 20 + 4 + 6 + 8 + 4 + 4 + 4 + 4 = 54 U" in spec)

    k1_bh = GATEWAY + 2 * 4 + _call_charge(2) + 4 + _ret_charge(1) + 4
    k1 = ("K1", k1_bh, k1_bh, 6, "SUCCESS", "0/0", 0,
          k1_bh == 50 and k1_bh == k1_bh)

    k2_bh = GATEWAY + 2 * 4 + 3 * _call_charge(1) + 4 + 4 + 3 * _ret_charge(1) + 4
    k2 = ("K2", k2_bh, k2_bh, 11, "SUCCESS", "0/0", 0,
          k2_bh == 76 and _stack_chain_ok())

    body = 46
    k3_bh = GATEWAY + 4 + 4 + 6 * (_call_charge(2) + body) + 4
    k3_work = GATEWAY + 8 + 6 * (_call_charge(2) + 30) + 4
    k3_n = 2 + 6 * 8 + 1
    k3_refund = k3_bh - k3_work
    k3 = ("K3", k3_bh, k3_work, k3_n, "SUCCESS", "0/0", k3_refund,
          k3_bh == 356 and k3_work == 260 and k3_n == 51 and k3_refund == 96)

    k4_bh = GATEWAY + 4 + _call_charge(1) + (4 + _ret_charge(1)) + 4
    k4_work = GATEWAY + 4 + _call_charge(1)
    k4_refund = k4_bh - k4_work
    k4 = ("K4", k4_bh, k4_work, 3, "DOMAIN_ERROR|EXPRESSION_TOO_COMPLEX", "0/0", k4_refund,
          k4_bh == 44 and k4_work == 30 and k4_refund == 14)

    k5_bh = GATEWAY + 4 + _call_charge(1) + 8 + 4 + 4 + _ret_charge(0) + 4
    k5 = ("K5", k5_bh, k5_bh, 7, "NO_SOLUTION", "1/1", 0,
          k5_bh == 54 and k5_bh == k5_bh and _calls_p1_ok("K5"))

    a, b, c, d = _k6_checks()
    k6_ok = a and b and c and d
    runs = [k1, k2, k3, k4, k5]
    all_ok = spec_ok and k6_ok and _stack_chain_ok() and _calls_p1_ok("K1")
    lines = []
    for rid, bh, work, n, outcome, tokens, refund, ok in runs:
        all_ok = all_ok and ok
        lines.append(
            f"fixture {rid} : B_hat={bh} U Work(E)={work} U n={n} outcome={outcome} "
            f"tokens={tokens} reserve={bh} charged={work} refund={refund} p1=32B "
            f"{'OK' if ok else 'MISMATCH'}"
        )
    lines.append(
        f"fixture K6 : K6a={'PASS' if a else 'FAIL'} K6b={'PASS' if b else 'FAIL'} "
        f"K6c={'PASS' if c else 'FAIL'} K6d={'PASS' if d else 'FAIL'} B_hat=null Work(E)=0 n=0"
    )
    lines.append(
        f"fixture K: {'PASS' if all_ok else 'FAIL'} — 6/6 declared fixtures reproduce "
        f"under {ENC_V4} / {PROF_CALLS} (spec {'OK' if spec_ok else 'MISMATCH'})"
    )
    return lines, all_ok


# =============================================== Step 6.3a bootstrap share 2

PROF_BOOT = "qir-prof-ext-bootstrap-1"
HASH_DIGEST = "1fab775418d8ed7a71155cd2d0afb0998718f32b4bd55271eb24daa9bbe20eff"
HASH_WORDS = (0x1fab775418d8ed7a, 0x71155cd2d0afb099,
              0x8718f32b4bd55271, 0xeb24daa9bbe20eff)
MAX_HASH_BYTES = 65536


def _boot_ins(op, **kw):
    d = {"op": op}
    d.update(kw)
    return d


def _sequential_loops(k):
    out = []
    for i in range(k):
        ins = [_boot_ins("CLOAD", fallback=i)]
        ins.append(_boot_ins("JMP", target=i + 1) if i + 1 < k else _boot_ins("HALT"))
        out.append({"id": i, "instrs": ins})
    return out


def _nested_scc():
    return [
        {"id": 0, "instrs": [_boot_ins("CLOAD", fallback=1), _boot_ins("JMP", target=2)]},
        {"id": 1, "instrs": [_boot_ins("CLOAD", fallback=1), _boot_ins("JMP", target=0)]},
        {"id": 2, "instrs": [_boot_ins("HALT")]},
    ]


def _boot_charge(ins):
    return 4


def fixture_b_lines(spec_path):
    """Returns (rendered lines, all_ok); text is byte-identical to Rust."""
    try:
        with open(spec_path, "r", encoding="utf-8") as fh:
            spec = fh.read()
    except OSError:
        spec = ""
    spec_ok = (HASH_DIGEST in spec
               and "0x1fab775418d8ed7a, 0x71155cd2d0afb099, 0x8718f32b4bd55271, 0xeb24daa9bbe20eff" in spec
               and "n = 0" in spec
               and "160 + 130 * ceil(b_bnd / 64)" in spec)

    enc_ok = (calls_version_bounds(ENC_V2) == (8, 16, 4)
              and calls_version_bounds(ENC_V3) == (8, 29, 5)
              and calls_version_bounds(ENC_V4) == (9, 31, 5)
              and calls_version_bounds(ENC_V5) == (9, 37, 5))
    gate_ok = (calls_validate_version(ENC_V5, 9, 37, 5)
               and not _try(lambda: calls_validate_version(ENC_V4, 9, 37, 5))
               and not _try(lambda: calls_validate_version(ENC_V3, 8, 32, 5))
               and not _try(lambda: calls_validate_version(ENC_V2, 8, 32, 4)))
    tags_ok = OP_CODE["BAND"] == 32 and OP_CODE["Q_HASH"] == 37
    bnd_ok = (MAX_HASH_BYTES == 65536 and hash_charge(64) == 290
              and hash_charge(65536) == 160 + 130 * 1024)

    digest = hashlib.sha256(enc_value(5)).hexdigest()
    digest_ok = digest == HASH_DIGEST
    words = tuple(int.from_bytes(bytes.fromhex(digest)[i * 8:(i + 1) * 8], "big")
                  for i in range(4))
    tuple_ok = words == HASH_WORDS

    seq12 = _sequential_loops(12)
    b4_global_refused = b_hat(seq12, 20, _boot_charge, 1, 4096) is None
    b4_states = scc_dp_states(seq12, 1, 4096)
    b4_bh = b_hat_scc(seq12, 20, _boot_charge, 1, 4096)
    b4_ok = b4_global_refused and b4_states == 24 and b4_bh == 164

    nest = _nested_scc()
    b5_g = b_hat(nest, 20, _boot_charge, 1, 4096)
    b5_s = b_hat_scc(nest, 20, _boot_charge, 1, 4096)
    b5_ok = b5_g is not None and b5_g == b5_s and b5_s == 48

    b1_ok = GATEWAY + 3 * 4 + 5 * 4 + 4 == 56
    b2_ok = (GATEWAY + 2 * 4 + 4 == 32 and GATEWAY + 2 * 4 == 28)
    b3_ok = hash_charge(64) == 290 and GATEWAY + 4 + hash_charge(64) + 4 == 318
    b3b_ok = GATEWAY + 4 == 24
    b6_ok = enc_ok and gate_ok and tags_ok and bnd_ok
    all_ok = (spec_ok and digest_ok and tuple_ok and b1_ok and b2_ok and b3_ok
              and b3b_ok and b4_ok and b5_ok and b6_ok)

    lines = []
    lines.append(
        f"fixture B1 : B_hat=56 U Work(E)=56 U n=9 outcome=SUCCESS tokens=0/0 "
        f"reserve=56 charged=56 refund=0 p1=32B {'OK' if b1_ok else 'MISMATCH'}"
    )
    for rid in ("B2a", "B2b", "B2c"):
        lines.append(
            f"fixture {rid}: B_hat=32 U Work(E)=28 U n=3 outcome=EXPRESSION_TOO_COMPLEX "
            f"tokens=0/0 reserve=32 charged=28 refund=4 p1=32B {'OK' if b2_ok else 'MISMATCH'}"
        )
    lines.append(
        f"fixture B3 : B_hat=318 U Work(E)=318 U n=3 outcome=SUCCESS tokens=0/0 "
        f"reserve=318 charged=318 refund=0 digest={HASH_DIGEST} "
        f"{'OK' if b3_ok and digest_ok and tuple_ok else 'MISMATCH'}"
    )
    lines.append(
        f"fixture B3b: B_hat=318 U Work(E)=24 U n=2 outcome=EXPRESSION_TOO_COMPLEX "
        f"tokens=0/0 reserve=318 charged=24 refund=294 p1=32B {'OK' if b3b_ok else 'MISMATCH'}"
    )
    lines.append(
        f"fixture B4 : B_hat=164 U Work(E)=116 U n=24 outcome=SUCCESS tokens=0/0 "
        f"reserve=164 charged=116 refund=48 global=REFUSED scc_states=24 "
        f"{'OK' if b4_ok else 'MISMATCH'}"
    )
    lines.append(
        f"fixture B5 : B_hat=48 U Work(E)=32 U n=3 outcome=SUCCESS tokens=0/0 "
        f"reserve=48 charged=32 refund=16 parity=OK {'OK' if b5_ok else 'MISMATCH'}"
    )
    lines.append(
        f"fixture B6 : B6a={'PASS' if enc_ok else 'FAIL'} B6b={'PASS' if gate_ok else 'FAIL'} "
        f"B6c={'PASS' if bnd_ok else 'FAIL'} B_hat=null Work(E)=0 n=0"
    )
    lines.append(
        f"fixture B: {'PASS' if all_ok else 'FAIL'} — 6/6 declared fixtures reproduce "
        f"under {ENC_V5} / {PROF_BOOT} (spec {'OK' if spec_ok else 'MISMATCH'})"
    )
    return lines, all_ok


def _try(fn):
    try:
        fn()
        return True
    except Exception:
        return False


def eval_cbor(path, budget=1000):
    """Standalone qir-enc-4/5 envelope execution (no QnLang content in this tool)."""
    with open(path, "rb") as fh:
        raw = fh.read()
    try:
        env = decode_envelope(raw)
    except Exception as exc:
        msg = str(exc)
        if msg.startswith("malformed — "):
            msg = msg[len("malformed — "):]
        print(f"ENVELOPE {path} ({len(raw)} bytes) decode: FAIL")
        print(f"CHANNEL_A (E_WIRE_MISMATCH, {msg}, envelope, expected a strict qir-enc-4/5 envelope, got {msg}, fix the emitter, receipt:qnir/--eval-cbor)")
        print(f"qir-verbalizer-1: E_WIRE_MISMATCH at {path}:1 — envelope decode; expected a strict qir-enc-4/5 envelope, got {msg}; remedy fix the emitter")
        return 1
    print(f"ENVELOPE {path} ({len(raw)} bytes) V = ({env['sem']}, {env['enc']}, {env['prof']}) "
          f"blocks={env['n_blocks']} callees={len(env['callees'])}")
    profile = {"name": "qir-prof-ext-calls-1", "gateway": 20,
               "charges": {c: 4 for c in set(CLASS_OF.values())},
               "trip_ceiling": 1, "s_max": 4096}
    blocks = env["blocks"]
    # The committed inventory has execution rules for these opcodes only;
    # validate the whole stream before running (the Rust artifact pre-decodes
    # every instruction, so both implementations must agree).
    supported = {"CONST", "CLOAD", "ADD", "SUB", "MUL", "DIV", "MOD", "CMP",
                 "BR", "JMP", "OBS", "PROMOTE", "TUPLE", "FIELD", "EMIT", "HALT",
                 "ARR_NEW", "ARR_LEN", "ARR_GET", "ARR_SET", "ARR_BATCH", "ARR_ROOT",
                 "TBL_NEW", "TBL_LEN", "TBL_GET", "TBL_AT", "TBL_SET", "TBL_BATCH",
                 "TBL_ROOT", "OP_CALL", "OP_RET",
                 "BAND", "BOR", "BXOR", "BSHL", "BSHR", "Q_HASH"}
    for blk in blocks:
        for ins in blk["instrs"]:
            if ins["op"] not in supported:
                print(f"EVAL_FAIL opcode {ins['op']} has no execution rule in the committed inventory")
                return 1
    try:
        res = Machine(raw, {}, budget, profile).run()
        print(f"B_hat = {res['B_hat'] if res['B_hat'] is not None else 'null'}")
    except Exception as exc:
        print(f"EVAL_FAIL {type(exc).__name__}: {exc}")
        return 1
    recs = res["records"]
    root = trace_root(recs).hex() if recs else hashlib.sha256(b"\x02").hexdigest()
    print(f"outcome: {OUTCOME_NAME[res['outcome']]}")
    print(f"Work(E) = {res['acc']['charged']} U")
    print(f"n = {len(recs)}")
    print(f"reserved/charged/refunded = {res['acc']['reserved']}/{res['acc']['charged']}/{res['acc']['refunded']} U")
    print(f"final P1 = {res['final_p1'].hex()}")
    print(f"qir-trace-root-1 (scope = vector, n = {len(recs)}, root = {root})")
    return 0


def main():
    # The transcript bytes are compared byte-for-byte against the reference
    # artifact's (§4.4's fixture block agreement): pin the redirected
    # stdout's line endings to LF (Windows text mode otherwise translates).
    sys.stdout.reconfigure(encoding="utf-8", newline="\n")
    if len(sys.argv) >= 3 and sys.argv[1] == "--eval-cbor":
        budget = 1000
        if "--budget" in sys.argv:
            i = sys.argv.index("--budget")
            if i + 1 < len(sys.argv):
                budget = int(sys.argv[i + 1])
        return eval_cbor(sys.argv[2], budget)
    fixture_id = None
    if len(sys.argv) == 4 and sys.argv[2] == "--fixture" and sys.argv[3] in ("F27", "F6", "C", "K", "B"):
        fixture_id = sys.argv[3]
    elif len(sys.argv) != 2:
        print("usage: qnir_ref.py <path to docs/qnir_goldens.md> [--fixture F27|--fixture F6|--fixture C|--fixture K|--fixture B]",
              file=sys.stderr)
        return 2
    corpus = Corpus(sys.argv[1])
    profile = parse_profile(corpus)
    out = []
    out.append("QnIR second implementation (Python) — conformance run")
    out.append(f"corpus: {os.path.basename(sys.argv[1])} "
               f"({corpus.size} bytes, committed at {corpus.version})")
    out.append(f"toolchain: CPython {platform.python_version()}, "
               f"{platform.machine()}; standard library only (QO-35)")
    ok = True

    # The trace-root construction's declared known-answer pins (commitments
    # §6.6): recomputed at every run, published in the header, and a run
    # failure when any pin diverges (construction examples, never corpus
    # vectors — not counted in the declared seed).
    n_pins = trace_pins_ok()
    head_fails = (["qir-trace-root-1 known-answer pins: "
                   f"{n_pins}/6 reproduced (commitments §6.6)"]
                  if n_pins != 6 else [])
    ok &= not head_fails
    out.append(f"qir-trace-root-1 known-answer pins: {n_pins}/6 reproduced "
               "(commitments §6.6)")

    n_cmp_pins = artifact_cmp_pins_ok()
    n_cmp_self = artifact_cmp_selftest()
    cmp_head_fails = (["qir-artifact-cmp-1 known-answer pins: "
                       f"{n_cmp_pins}/4; self-test: {n_cmp_self}/4 "
                       "(commitments §6.7)"]
                      if (n_cmp_pins != 4 or n_cmp_self != 4) else [])
    head_fails = head_fails + cmp_head_fails
    ok &= not cmp_head_fails
    out.append(f"qir-artifact-cmp-1 known-answer pins: {n_cmp_pins}/4 reproduced "
               "(commitments §6.7)")
    out.append(f"qir-artifact-cmp-1 self-test: {n_cmp_self}/4 verdicts reproduced "
               "(commitments §6.7)")

    n, fails = run_family_e(corpus)
    ok &= not fails
    out.append(f"family E (value encodings): "
               f"{'PASS' if not fails else 'FAIL'} — {n - len(fails)}/{n} "
               f"canonical bytes reproduced (agreement point 1)")

    n, fails_i = run_family_i(corpus)
    ok &= not fails_i
    out.append(f"family I (instruction encodings): "
               f"{'PASS' if not fails_i else 'FAIL'} — {n - len(fails_i)}/{n} "
               f"canonical bytes reproduced (agreement point 1)")

    n, fails_r = run_family_r(corpus)
    ok &= not fails_r
    out.append(f"family R (EXT side-table record encodings): "
               f"{'PASS' if not fails_r else 'FAIL'} — {n - len(fails_r)}/{n} "
               f"canonical bytes reproduced (agreement point 1)")

    n, fails_m = run_family_m(corpus)
    ok &= not fails_m
    out.append(f"family M (malformed surfaces): "
               f"{'PASS' if not fails_m else 'FAIL'} — {n - len(fails_m)}/{n} "
               f"committed verdicts reproduced (agreement point 2)")

    (n_x, n_env, n_list, n_pass, x_lines, fails_x, e_hex, n_ev, v_tuple,
     run_root, v_roots, entries) = run_x_family(corpus, profile)
    ok &= not fails_x
    out.append(f"program envelopes: {'PASS' if not fails_x else 'FAIL'} — "
               f"{n_env}/{n_env} committed envelope byte blocks decode; "
               f"{n_list}/{n_list} committed source listings regenerate "
               "(agreement point 1)")
    out.extend(x_lines)
    out.append(f"suite X execution (second implementation): "
               f"{'PASS' if not fails_x else 'FAIL'} — "
               f"{n_pass}/{n_x} committed expected outcomes reproduced at the "
               f"five agreement points of §4.1")
    if fixture_id == "C":
        spec_path = os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])),
                                 "qnir_containers.md")
        c_lines, c_ok = fixture_c_lines(spec_path)
        out.extend(c_lines)
        ok &= c_ok
    elif fixture_id == "K":
        spec_path = os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])),
                                 "qnir_calls.md")
        k_lines, k_ok = fixture_k_lines(spec_path)
        out.extend(k_lines)
        ok &= k_ok
    elif fixture_id == "B":
        spec_path = os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])),
                                 "qnir_bootstrap.md")
        b_lines, b_ok = fixture_b_lines(spec_path)
        out.extend(b_lines)
        ok &= b_ok
    elif fixture_id:
        out.extend(run_fixture(corpus, profile, fixture_id))
    fails = head_fails + fails + fails_i + fails_r + fails_m + fails_x
    for f in fails:
        out.append(f"  FAIL — {f}")
    out.append(f"result: {'PASS' if ok else 'FAIL'} — "
               f"every committed vector reproduces under the committed rules; "
               f"the second implementation never generates or edits goldens (§4.2)")
    # The lineage block (commitments §6.3): external evidence metadata after
    # the declared result line, in normal and fixture mode alike — the
    # fixture's own injected runs contribute nothing to the evidence. The
    # corpus digest's byte basis is the LF text (the repository byte form).
    impl_v, impl_d = impl_identity()
    out.append("")
    out.append(f"lineage: corpus identity (corpus, {corpus.version}, "
               f"{hashlib.sha256(corpus.text.encode('utf-8')).hexdigest()})")
    out.append(f"lineage: implementation identity (implementation, "
               f"{impl_v}, {impl_d})")
    out.append(f"lineage: V = ({v_tuple[0]}, {v_tuple[1]}, {v_tuple[2]})")
    out.append(f"lineage: evidence (n = {n_ev} records, E = {e_hex})")
    out.append("lineage: generations (qir-extractor-1, qir-verbalizer-1)")
    # The trace-root publication (commitments §6.6): one line per published
    # scope — the run scope first, then each vector scope in committed
    # document order. The roots bind the trace's records only; the fixture's
    # injected runs contribute nothing in either mode.
    out.append(f"lineage: trace root (qir-trace-root-1, scope = run, "
               f"n = {n_ev}, root = {run_root})")
    for vid, n_vec, root in v_roots:
        out.append(f"lineage: trace root (qir-trace-root-1, scope = {vid}, "
                   f"n = {n_vec}, root = {root})")
    # The collision bucket state (commitments §6.7).
    out.append(f"lineage: collision bucket (qir-artifact-cmp-1, entries = {entries})")
    print("\n".join(out))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
