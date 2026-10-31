#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""qnir_ref.py — the QnIR v0.3 working draft's second implementation
(R6 artifact-share increment 4, v0.3.0-draft.13).

Independence declaration (conformance §4.1/§4.3): implemented from the
committed text of docs/qnir.md §1, docs/qnir_semantics.md §2,
docs/qnir_accounting.md §3, docs/qnir_conformance.md §4, and the committed
golden corpus docs/qnir_goldens.md with its declared artifact-wave parameters
(P1–P11) and its declared profile.  This implementation consults neither the
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
           13: "OBS", 14: "PROMOTE", 15: "EMIT", 16: "HALT"}
OP_CODE = {v: k for k, v in OPCODES.items()}
CLASS_OF = {"CONST": "q_const", "CLOAD": "q_load", "TUPLE": "q_tuple",
            "FIELD": "q_field", "ADD": "q_add", "SUB": "q_sub", "MUL": "q_mul",
            "DIV": "q_div", "MOD": "q_mod", "CMP": "q_cmp", "BR": "q_br",
            "JMP": "q_jmp", "OBS": "q_obs", "PROMOTE": "q_promote",
            "EMIT": "q_emit", "HALT": "q_halt"}
TOKEN_TYPES = {1: "STATUS_LOOKUP_MISS", 2: "STATUS_NEG_EVEN_ROOT",
               3: "EXPRESSION_TOO_COMPLEX", 4: "STATUS_DOMAIN_ERROR"}
CC_NAMES = {0: "LT", 1: "LE", 2: "EQ", 3: "NE", 4: "GT", 5: "GE"}
DEFINING = {"CONST", "CLOAD", "TUPLE", "FIELD", "ADD", "SUB", "MUL", "DIV",
            "MOD", "CMP", "OBS"}
ARITH = ("ADD", "SUB", "MUL", "DIV", "MOD")
S_OPCODE = "malformed — unknown opcode tag"
S_CC = "malformed — unknown condition code"
S_TTYPE = "malformed — unknown token type tag"
S_PARTIAL = "malformed — trailing partial instruction"
S_DANGLE = "statically illegal — dangling side-table reference"


def decode_instruction(ws, i, end):
    """Decode one §2.6 instruction: opcode tag word, declared operand fields
    in declared order, one uniform trailing ext word."""
    if i + 1 > end:
        raise ValueError("truncated opcode")
    tag = ws[i]
    i += 1
    name = OPCODES.get(tag)
    if name is None:
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
    elif name in ("PROMOTE", "HALT"):
        pass
    else:
        # TUPLE, FIELD, EMIT: operand shapes are expansion obligations of the
        # seed; no committed vector exercises them.
        raise ValueError(f"opcode {name} operand grammar is an expansion obligation")
    ins["ext"] = word()
    return ins, i


def decode_stream(stream, n_blocks):
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
                ins, i = decode_instruction(ws, i, end)
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


def decode_envelope(bs):
    """Strict decode of one Intent envelope: canonical CBOR map, closed keys
    0–8, token type tags in the closed §2.5 vocabulary, the committed stream,
    and static legality of every ext bit against the EXT side table (§2.1)."""
    m = cbor_decode(bs)
    if not isinstance(m, dict):
        raise ValueError("envelope is not a map")
    for k in m:
        if not isinstance(k, int) or not 0 <= k <= 8:
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
    blocks = decode_stream(m[6], m[5])
    side_len = len(m[7])
    for b in blocks:
        for ins in b["instrs"]:
            for bit in range(64):
                if (ins["ext"] >> bit) & 1 and bit >= side_len:
                    raise Malformed(S_DANGLE)
    return {"sem": m[1], "enc": m[2], "prof": m[3], "deps": m[4],
            "n_blocks": m[5], "stream": m[6], "side": m[7], "pool": pool_decl,
            "blocks": blocks}


def listing_of(blocks):
    """The source grammar's textual form: the sixteen mnemonics with the same
    operands in the same order (§2.6), regenerated from the decoded bytes."""
    lines = []
    for b in blocks:
        lines.append(f"block {b['id']} (arity {b['arity']}):")
        for k, ins in enumerate(b["instrs"]):
            op = ins["op"]
            if op == "CONST":
                t = f"CONST imm={ins['imm']} bnd=(int, <={ins['rbnd']})"
            elif op == "CLOAD":
                t = (f"CLOAD addr={ins['addr']} fallback=block {ins['fallback']} "
                     f"bnd=(int, <={ins['rbnd']})")
            elif op in ARITH:
                t = f"{op} ref={ins['a']} ref={ins['b']} bnd=(int, <={ins['rbnd']})"
            elif op == "CMP":
                t = f"CMP ref={ins['a']} ref={ins['b']} rng=[{ins['rng'][0]}, {ins['rng'][1]}]"
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


def b_hat(blocks, gateway, charge_of, trip_ceiling):
    """The trip-ceiling unrolling dynamic program of §3.4: state space
    (block, instruction index, remaining trip budget per back-edge site);
    value = the maximum summed site-stable charge from a state to an exit;
    B_hat = gateway + the value at the entry state."""
    edges = block_graph(blocks)
    dom = dominators(len(blocks), edges)
    back = sorted({(s, d) for (s, d) in edges if d in dom[s]})
    bidx = {e: i for i, e in enumerate(back)}
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
            if op == "HALT":
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
    return gateway + v if v is not None else gateway


# ================================================================== machine
# Executes the committed transition relation of §2.3 under §2.5's lifecycle
# algebra and §3's accounting, with the corpus preamble's artifact-wave
# parameters P1–P11 and the declared profile.
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


class Machine:
    def __init__(self, envelope_bytes, D, w_budget, profile):
        env = decode_envelope(envelope_bytes)
        self.blocks = env["blocks"]
        self.deps = env["deps"]
        self.words = words_of(env["stream"])
        self.versions = [env["sem"], env["enc"], env["prof"]]
        self.D = D
        self.W = w_budget
        self.profile = profile
        self.BH = b_hat(self.blocks, profile["gateway"],
                        lambda ins: profile["charges"][CLASS_OF[ins["op"]]],
                        profile["trip_ceiling"])
        self.prog_root = hashlib.sha256(envelope_bytes).digest()
        self.pool = [{"birth": 0, "block": 0, "index": 0, "serial": k,
                      "type": t, "tier": TIER_S, "state": ST_OUT}
                     for k, t in enumerate(env["pool"])]
        self.acc = {"charged": 0, "reserved": 0, "refunded": 0}
        self.clock = 0
        self.step = 0
        self.halt = [0, 0]
        self.receipts = []          # P7: emission ordinals; no witness in the seed
        self.store = {}             # (static id, instance id) -> value
        self.current = {}           # static id -> current instance id
        self.next_iid = 0
        self.next_serial = len(env["pool"])
        self.escrow_open = False
        self.records = []
        self.boundaries = []
        self.per_class = {}
        self.traversals = {}        # (block, index, slot) -> traversal count
        self.pending_miss = None    # (token, fallback block)
        edges = block_graph(self.blocks)
        dom = dominators(len(self.blocks), edges)
        self.back_edges = sorted({(s, d) for (s, d) in edges if d in dom[s]})

    # ------------------------------------------------ configuration + records
    def config_bytes(self, pc):
        """The canonical configuration encoding (P2): the ten components of
        §2.2's table in that order, every component through deterministic
        CBOR, values carried as byte strings of their §2.6 encodings."""
        store_c = [[s, i, enc_value(v)] for (s, i), v in
                   sorted(self.store.items(), key=lambda kv: kv[0])]
        pool_c = [[t["birth"], t["block"], t["index"], t["serial"], t["type"],
                   t["tier"], t["state"]]
                  for t in sorted(self.pool, key=tid)]
        comps = [list(pc), [], store_c, pool_c,
                 [self.acc["charged"], self.acc["reserved"], self.acc["refunded"]],
                 self.clock, self.step, [self.halt[0], self.halt[1]],
                 list(self.receipts), list(self.versions)]
        return cbor_encode(comps)

    def emit(self, pc, decoded, charged, refunded, events, io_ids, pre, post):
        """One transition record: the fourteen fields of §2.6's F5 list, in
        order, through deterministic CBOR."""
        rec = [[1, pre], [2, self.prog_root], [3, self.step], [4, list(pc)],
               [5, decoded], [6, list(io_ids)], [7, pre], [8, post],
               [9, charged], [10, refunded], [11, self.clock], [12, events],
               [13, list(self.receipts)], [14, list(self.versions)]]
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
        if self.W < self.BH:
            # admission refusal (§3.4): the single rejection transition,
            # zero postings; the escrow never opens
            pre = self.config_bytes((0, 0))
            return self.rejection("EXPRESSION_TOO_COMPLEX", (0, 0),
                                  self.decoded_at(0, 0), pre)
        pc = (0, 0)
        pre = self.config_bytes(pc)
        while True:
            blk, idx = pc
            ins = self.blocks[blk]["instrs"][idx]
            decoded = self.decoded_at(blk, idx)
            c = self.profile["charges"][CLASS_OF[ins["op"]]]
            events = []
            io_ids = []
            gateway = 0
            if not self.escrow_open:
                # admission passed (§3.4); the escrow opens with this
                # transition and the reserve takes B(P) in full (§3.3)
                self.escrow_open = True
                self.acc["reserved"] += self.BH
                gateway = self.profile["gateway"]
            elif self.outstanding() < c:
                # fuel backstop (§3.4): defense in depth after admission
                return self.rejection("EXPRESSION_TOO_COMPLEX", pc, decoded, pre)
            try:
                transfer = self.dispatch(ins, pc, events, io_ids)
            except Rejection as r:
                return self.rejection(r.token_type, pc, decoded, pre)
            if transfer is None:
                transfer = ("fall", None)
            post_charge = gateway + c
            self.acc["charged"] += post_charge
            cls = CLASS_OF[ins["op"]]
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
                post = self.config_bytes(pc)
                self.emit(pc, decoded, post_charge, refund, events, io_ids,
                          pre, post)
                self.boundaries.append((self.step, dict(self.acc)))
                return self.pack(outcome)
            if self.pending_miss and self.pending_miss[1] == blk and last:
                self.pending_miss[0]["state"] = ST_CONSUMED
                events.append([2, tid(self.pending_miss[0])])
                self.pending_miss = None
            post = self.config_bytes(pc)
            self.emit(pc, decoded, post_charge, 0, events, io_ids, pre, post)
            self.boundaries.append((self.step, dict(self.acc)))
            pre = post
            if transfer[0] == "miss":
                self.pending_miss = (transfer[1], transfer[2])
                pc = (transfer[2], 0)
            elif transfer[0] in ("br", "jmp"):
                pc = (transfer[1], 0)
            else:
                pc = (blk, idx + 1)

    def dispatch(self, ins, pc, events, io_ids):
        op = ins["op"]
        return getattr(self, "do_" + op)(ins, pc, events, io_ids)

    # -------------------------------------------------------- the rules (§2.3)
    def do_CONST(self, ins, pc, events, io_ids):
        if ins["imm"] > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], ins["imm"])

    def do_CLOAD(self, ins, pc, events, io_ids):
        addr = self.deps[ins["addr"]]
        io_ids.append(addr)
        if addr in self.D:
            v = decode_value(words_of(self.D[addr]), 0, len(words_of(self.D[addr])))[0]
            if v > ins["rbnd"]:
                raise Rejection("EXPRESSION_TOO_COMPLEX")
            self.mint(ins["sid"], v)
            return None
        tok = self.birth(TOKEN_TYPES_REV["STATUS_LOOKUP_MISS"], pc)
        tok["tier"] = TIER_O
        events.append([1, tid(tok)])
        # the miss edge leaves the block here (§2.3 CLOAD row)
        return ("miss", tok, ins["fallback"])

    def do_ADD(self, ins, pc, events, io_ids):
        r = self.read(ins["a"]) + self.read(ins["b"])
        if r > ins["rbnd"]:
            raise Rejection("EXPRESSION_TOO_COMPLEX")
        self.mint(ins["sid"], r)

    do_SUB = do_ADD
    do_MUL = do_ADD

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
        a, b = self.read(ins["a"]), self.read(ins["b"])
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
        raise ValueError("TUPLE is an expansion obligation of the seed")

    do_FIELD = do_TUPLE
    do_EMIT = do_TUPLE

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
        settlement posts into this record (§3.3, P11)."""
        self.step += 1
        self.clock += 1
        tok = self.birth(TOKEN_TYPES_REV[ttype], pc, birth_step=self.step)
        tok["state"] = ST_TERM
        outcome = (OUTCOME["EXPRESSION_TOO_COMPLEX"]
                   if ttype == "EXPRESSION_TOO_COMPLEX" else OUTCOME["DOMAIN_ERROR"])
        self.halt = [1, outcome]
        refund = 0
        if self.escrow_open:
            refund = self.outstanding()
            if refund:
                self.acc["refunded"] += refund
        post = self.config_bytes(pc)
        self.emit(pc, decoded, 0, refund, [[1, tid(tok)]], [], pre, post)
        self.boundaries.append((self.step, dict(self.acc)))
        return self.pack(outcome)

    def build_receipt(self, outcome):
        """The terminal receipt (§2.5/§2.6): deterministic CBOR map, closed
        keys 0–4; the outcome-determining token selected for NO_SOLUTION and
        DOMAIN_ERROR; nil otherwise (the RFC 8949 nil, 0xf6)."""
        item = [[t["birth"], t["block"], t["index"], t["serial"], t["type"],
                 t["tier"], t["state"]] for t in sorted(self.pool, key=tid)]
        selected = None
        if outcome in (OUTCOME["NO_SOLUTION"], OUTCOME["DOMAIN_ERROR"]):
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
                "B_hat": self.BH, "versions": list(self.versions)}


TOKEN_TYPES_REV = {v: k for k, v in TOKEN_TYPES.items()}


# ============================================================ corpus parsing
# The committed corpus is consumed exactly as committed: a markdown document
# whose fences and field tables carry the committed expected bytes and
# outcomes (§4.4).  Nothing below writes to it.

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
           "M12": "envelope"}


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
        decode_envelope(bs)
        return "canonical"
    raise ValueError(kind)


def parse_profile(c):
    """The declared profile of the corpus preamble (qir-prof-golden-1)."""
    body = c.between("## The Declared Profile", "## Family E")
    text = "\n".join(body)
    g = int(re.search(r"`g = (\d+)` U", text).group(1))
    q = int(re.search(r"q_const … q_halt = (\d+)` U", text).group(1))
    trip = int(re.search(r"trip ceiling (\d+) per back-edge site", text).group(1))
    depth = int(re.search(r"depth ceiling (\d+)", text).group(1))
    charges = {("q_load" if op == "CLOAD" else f"q_{op.lower()}"): q
               for op in OPCODES.values()}
    return {"gateway": g, "charges": charges, "trip_ceiling": trip,
            "depth_ceiling": depth}


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
    for vid in order:
        body = by_id[vid]
        params = next(ln for ln in body if ln.startswith("Declared parameters:"))
        wb = int(re.search(r"`W_budget` = (\d+) U", params).group(1))
        if "no entries" not in params:
            raise ValueError("a populated dependency store is an expansion obligation")
        D = {}
        v_line = re.search(r"`V` = \(([^)]*)\)", params).group(1)
        V = [s.strip() for s in v_line.split(",")]
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
            if res["B_hat"] != b_committed:
                raise AssertionError(
                    f"B_hat {res['B_hat']} != committed declared bound {b_committed}")
            if len(res["records"]) != len(committed_records):
                raise AssertionError(
                    f"record count {len(res['records'])} != {len(committed_records)}")
            for k, (mine, want) in enumerate(zip(res["records"],
                                                 committed_records), 1):
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
    return len(order), n_env, n_listings, n_pass, x_lines, fails


def main():
    if len(sys.argv) != 2:
        print("usage: qnir_ref.py <path to docs/qnir_goldens.md>", file=sys.stderr)
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

    n, fails_m = run_family_m(corpus)
    ok &= not fails_m
    out.append(f"family M (malformed surfaces): "
               f"{'PASS' if not fails_m else 'FAIL'} — {n - len(fails_m)}/{n} "
               f"committed verdicts reproduced (agreement point 2)")

    n_x, n_env, n_list, n_pass, x_lines, fails_x = run_x_family(corpus, profile)
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
    fails = fails + fails_i + fails_m + fails_x
    for f in fails:
        out.append(f"  FAIL — {f}")
    out.append(f"result: {'PASS' if ok else 'FAIL'} — "
               f"every committed vector reproduces under the committed rules; "
               f"the second implementation never generates or edits goldens (§4.2)")
    print("\n".join(out))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
