---
layout: default
title: QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1)
description: Hand-derived conformance vectors — expected bytes and outcomes fixed from the normative rules of the QnIR draft before any code
version: 0.3.0-draft.11
updated: 2026-10-31
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_goldens.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_goldens.html
status: Working Draft under the Loop-2 Repair Sequence and the Loop-3 Repair Program (R1–R6) — Not Promoted to the Canon
terms_used:
  - boc-policy
  - qnir
  - no-implicit-rule
  - compute-unit-u
  - receipt
---

# QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1)

| Field | Specification |
|---|---|
| **Document Title** | QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1) |
| **Version** | 0.3.0-draft.11 |
| **Last Updated** | 2026-10-31 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Working Draft under the Loop-2 Repair Sequence and the Loop-3 Repair Program (R1–R6) — Not Promoted to the Canon |

*Christophe Duy Quang Nguyen note:* This file is the committed golden corpus of the QnIR v0.3 working draft — the first artifact increment under the declared order of conformance §4.5 (F18): the goldens precede all code, and no implementation exists or is drafted at this version. Every expected byte and outcome below is derived from the normative rules of §1–§3 and §4 and cites the rule that produces it. Nothing herein redefines, amends, or overrides any canonical specification.

## Scope and Derivation Discipline

This corpus is the seed suite of §4.4: a small suite derived from the normative rules of §1–§3 and §4, reviewed independently of both implementations, and committed before any implementation exists — the golden-first order (F17/F18), discharged here under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7 as artifact-share increment 1 of the {% include term.html id="qnir" text="QnIR" %} draft. Under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline, every vector cites the committed rule that produces each expected value; where the committed text leaves a rendering choice open, the choice is declared in the parameter block below and binds the vectors exactly as a version component of `V` (§2.1) — a change to any of them is a new encoding version. Charges are measured in the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}; the terminal {% include term.html id="receipt" text="receipt" %} of every execution vector is a committed artifact of §2.5/§2.6.

**The corpus binds 34 vectors: 11 value encodings (family E), 3 instruction encodings (family I), 12 malformed surfaces (family M), 8 executions (family X).** Each vector is committed data, citable by repository bytes; equal vectors are byte-equal vectors (§4.4).

## Declared Artifact-Wave Parameters

Declared here under §4.3's own delegation ("the hash's own algorithm and output width are declared with the encoding version at the artifact wave"); they ride the encoding version `qir-enc-1` of every vector's `V` and bind the vectors — no canon byte changes:

- **P1 — state-root function:** SHA-256, output width 32 bytes, over the canonical configuration encoding of P2.
- **P2 — canonical configuration encoding:** the ten components of §2.2's table in that table's order — program position `[block, index]`; block-argument environment (values in argument order); dynamic value store in `(static value id, instance id)` order, each entry `[static id, instance id, value bytes]`; status token pool in the full token-identifier order of §2.6, each entry `[birth step, block, index, serial, type, tier, state]`; work accumulators `[charged, reserved, refunded]`; clock; step index; halt state `[running flag (0 running, 1 halted), outcome tag (0 when running)]`; receipt-order queue in emission order; version bindings `[semantics, encoding, profile]`. Every component is encoded through deterministic CBOR, and a value inside CBOR is carried as the byte string of its §2.6 word encoding (§2.6: "every component through the value encoding above").
- **P3 — program root:** SHA-256 of the Intent envelope's canonical CBOR bytes (record field 2; §2.6 F5 field list).
- **P4 — first record's prev root:** the root of the declared initial configuration (§2.1: the initial configuration is declared, not computed).
- **P5 — step indices:** 1-based counts of completed transitions; a record's step index and clock field both carry the post-transition count (§2.2: one tick per transition; the fields stay distinct). Birth sites carry the birthing transition's own step index; the declared initial pool births at step 0 (§2.5).
- **P6 — token tier words:** Tier O = 1, Tier S = 2 (§2.5's two tiers); state words are §2.6's own receipt vocabulary: outstanding 1, consumed 2, terminalized 3.
- **P7 — receipt identifiers:** the emission ordinal, dense from 0 (§2.6 canonical maps); the receipt-order queue and record field 13 carry ordinals. No seed vector emits a witness, so both stay empty — witness payloads are an expansion obligation.
- **P8 — halt-state encoding:** `[running flag, outcome tag]` per P2's component 8, outcome tags per §2.6's closed terminal vocabulary (SUCCESS 1 … DOMAIN_ERROR 4).
- **P9 — dependency addresses:** the seed's single declared content address is the 8-byte string `h'0000000000000001'` (a declared vector parameter; §2.1 cites `D` by content address).
- **P10 — static value ids:** dense from 0 over the Intent's value-defining instructions in block-then-index scan order (§2.6: "static value ids are dense words over the Intent"); a `ref` operand cites these ids; an operand reads the current instance of its static id (§2.4: redefinition mints, never overwrites).
- **P11 — rejection records:** a typed rejection emits its own record with `work_charged = 0` (§3.1: rejection transitions post nothing), the terminal settlement's refund in `work_refunded` where the escrow is open (§3.3: settled by the execution's final transition, whatever its posting status), and the fatal birth traced in the status field (§2.5).

## The Declared Profile (`qir-prof-golden-1`)

One profile for every execution vector; the coefficients are profile parameters riding `V` (§3.2/§3.6 — no numeric charge table exists in the draft itself, and the corpus does not create one):

- gateway charge `g = 20` U, posted once by the first completed transition (§3.1);
- every rule charge class `q_const … q_halt = 4` U, site-stable, positive, inclusive of the base dispatch quantity (§3.1/§3.2);
- trip ceiling 1 per back-edge site; depth ceiling 8 (§2.3 derived counters; §3.4);
- `W_budget` is a per-vector declared parameter (X1–X8 below);
- in every seed vector `B_hat(P) = B(P)` — the surrogate DP is exact on these graphs, so the reserve opens at B(P) and no false admission or false rejection arises (§3.4).

## Family E — Value Encodings

Each vector fixes the expected canonical bytes of one §2.6 value; the four §2.6 worked encodings are committed here as named vectors.

### E1 enc-zero

Cites: §2.6 value grammar; §2.6 worked encoding (canonical zero = INT_POS with count 0).

Input: the exact integer 0. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000000
```

### E2 enc-neg-one

Cites: §2.6 value grammar; §2.6 worked encoding (sign rides the type tag, magnitude words carry the absolute value).

Input: the exact integer −1. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000002 0000000000000001 0000000000000001
```

### E3 enc-int-2^64

Cites: §2.6 value grammar; §2.6 worked encoding (multiword magnitude, minimal big-endian).

Input: the exact integer 2^64. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000002 0000000000000001 0000000000000000
```

### E4 enc-tuple-1-minus-2

Cites: §2.6 value grammar; §2.6 worked encoding (field count word, then k complete field encodings in declared order).

Input: the tuple (1, −2). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000003 0000000000000002 0000000000000001 0000000000000001
0000000000000001 0000000000000002 0000000000000001 0000000000000002
```

### E5 enc-int-255

Cites: §2.6 value grammar (one magnitude word).

Input: the exact integer 255. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000001 00000000000000ff
```

### E6 enc-int-max-u64

Cites: §2.6 value grammar (one full magnitude word, no leading zero).

Input: the exact integer 2^64 − 1. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000001 ffffffffffffffff
```

### E7 enc-int-2^128

Cites: §2.6 value grammar (three magnitude words, minimal big-endian, no leading zero word).

Input: the exact integer 2^128. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000003 0000000000000001 0000000000000000
0000000000000000
```

### E8 enc-neg-2^64

Cites: §2.6 value grammar (INT_NEG carries the absolute magnitude).

Input: the exact integer −2^64. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000002 0000000000000002 0000000000000001 0000000000000000
```

### E9 enc-tuple-nested

Cites: §2.6 value grammar (nested tuples: each field a complete value encoding).

Input: the tuple (0, (1, −2)). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000003 0000000000000002 0000000000000001 0000000000000000
0000000000000003 0000000000000002 0000000000000001 0000000000000001
0000000000000001 0000000000000002 0000000000000001 0000000000000002
```

### E10 enc-zoom-normalized

Cites: §2.6 value grammar (ZOOM: mantissa then exponent, each an INT_POS/INT_NEG encoding, in §2.3's normalized form); §2.3 normalizer N.

Input: the zoom-carrying value with mantissa 6, exponent −2, in the normalized form of §2.3's N (→ (3, −1)). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000004 0000000000000001 0000000000000001 0000000000000003
0000000000000002 0000000000000001 0000000000000001
```

### E11 enc-zoom-canonical-zero

Cites: §2.3 normalizer N (N(0, z) = (0, 0)); §2.6 value grammar.

Input: the zoom-carrying value (0, 5): N(0, z) = (0, 0), the canonical zero of §2.6. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000004 0000000000000001 0000000000000000 0000000000000001
0000000000000000
```

## Family I — Instruction Encodings

Each vector fixes the expected canonical word sequence of one instruction: opcode tag word, declared operand fields in declared order, one trailing ext word (§2.6 instruction grammar).

### I1 enc-insn-const-zero

Cites: §2.6 instruction grammar (CONST = imm, bnd; trailing ext word).

Input: CONST with literal 0, result bound (int, ≤ 0), no extension. Expected verdict: canonical. Expected canonical words:

```
0000000000000001 0000000000000001 0000000000000000 0000000000000001
0000000000000001 0000000000000000 0000000000000000
```

### I2 enc-insn-add

Cites: §2.6 instruction grammar (ADD = ref, ref, bnd).

Input: ADD of static value ids 0 and 1, result bound (int, ≤ 2), no extension. Expected verdict: canonical. Expected canonical words:

```
0000000000000005 0000000000000000 0000000000000001 0000000000000001
0000000000000001 0000000000000001 0000000000000002 0000000000000000
```

### I3 enc-insn-branch

Cites: §2.6 instruction grammar (BR = cc, ref, edge true, edge false; edge = blk, arity cnt, refs; condition codes LT 0 … GE 5).

Input: BR on condition code NE over static value id 0, true edge to block 1, false edge to block 2, both edges arity 0, no extension. Expected verdict: canonical. Expected canonical words:

```
000000000000000b 0000000000000003 0000000000000000 0000000000000001
0000000000000000 0000000000000002 0000000000000000 0000000000000000
```

## Family M — The Malformed Surface

Each vector fixes the expected verdict for one declared malformed or statically illegal class (§2.6: unknown-field rejection is total over every structure of the grammar).

### M1 insn-unknown-opcode

Cites: §2.6 closed tag assignments (the word 0 is no opcode; an opcode tag outside 1–16 is malformed).

Input bytes: an instruction whose opcode tag word is 17. Expected verdict: **malformed — unknown opcode tag**. Bytes:

```
0000000000000011 0000000000000000
```

### M2 value-negative-tagged-zero

Cites: §2.6 sign convention (a negative-tagged zero is malformed).

Input bytes: an INT_NEG encoding whose magnitude is 0. Expected verdict: **malformed — negative-tagged zero**. Bytes:

```
0000000000000002 0000000000000001 0000000000000000
```

### M3 value-leading-zero-word

Cites: §2.6 value grammar (minimal big-endian, no leading zero word).

Input bytes: an INT_POS encoding with magnitude words [0, 5]. Expected verdict: **malformed — leading zero magnitude word (non-canonical)**. Bytes:

```
0000000000000001 0000000000000002 0000000000000000 0000000000000005
```

### M4 value-non-normalized-zoom

Cites: §2.6 value grammar (a non-normalized zoom encoding is malformed); §2.3 N(6, −2) = (3, −1). The bytes are re-issued at v0.3.0-draft.11: the draft.10 form omitted the two component count words and did not parse as the pair it declares (the draft.11 wave record of docs/qnir.md names the disagreeing point and its §4.1 adjudication against §2.6).

Input bytes: a ZOOM encoding of the pair (6, −2) without normalization. Expected verdict: **malformed — non-normalized zoom encoding**. Bytes:

```
0000000000000004 0000000000000001 0000000000000001 0000000000000006
0000000000000002 0000000000000001 0000000000000002
```

### M5 value-count-exceeds-artifact

Cites: §2.6 structural bounds (a count must not exceed the remaining length of its enclosing artifact).

Input bytes: an INT_POS encoding declaring 3 magnitude words with only one word remaining in its artifact. Expected verdict: **malformed — count exceeds the enclosing artifact's remaining length**. Bytes:

```
0000000000000001 0000000000000003 00000000000000ff
```

### M6 value-unknown-type-tag

Cites: §2.6 closed tag assignments (no other type tag exists).

Input bytes: a value whose type-tag word is 5. Expected verdict: **malformed — unknown type tag**. Bytes:

```
0000000000000005 0000000000000000
```

### M7 insn-unknown-condition-code

Cites: §2.6 closed tag assignments (condition codes LT 0 … GE 5); §2.6 unknown-field rejection is total.

Input bytes: a BR instruction whose condition-code word is 6. Expected verdict: **malformed — unknown condition code**. Bytes:

```
000000000000000b 0000000000000006 0000000000000000 0000000000000001
0000000000000000 0000000000000002 0000000000000000 0000000000000000
```

### M8 envelope-unknown-key

Cites: §2.6 envelope grammar (an envelope carrying a key outside 0–8 is malformed).

Input bytes: an envelope map carrying key 9. Expected verdict: **malformed — unknown envelope key**. Bytes:

```
a10901
```

### M9 envelope-dangling-ext

Cites: §2.6 words-and-values (an unknown flag combination or a dangling side-table reference makes the Intent statically illegal, §2.1).

Input bytes: a one-block program whose CONST carries ext bit 0 (ext word 1) while the Intent's EXT side table is empty. Expected verdict: **statically illegal — dangling side-table reference**. Bytes:

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800501
06585000 00000000 00000000 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000107 800880
```

### M10 stream-trailing-partial

Cites: §2.6 instruction grammar (a trailing partial instruction is malformed).

Input bytes: a block declaring instruction count 2 whose stream carries one complete CONST and no further words. Expected verdict: **malformed — trailing partial instruction**. Bytes:

```
0000000000000000 0000000000000000 0000000000000002 0000000000000001
0000000000000001 0000000000000000 0000000000000001 0000000000000001
0000000000000000 0000000000000000
```

### M11 envelope-duplicate-key

Cites: §2.6 canonical maps (a duplicate key in any encoded artifact is malformed).

Input bytes: an envelope map carrying key 5 twice. Expected verdict: **malformed — duplicate envelope key**. Bytes:

```
a2050205 03
```

### M12 envelope-unknown-token-type

Cites: §2.6 closed tag assignments (token type tags are the closed vocabulary of §2.5); §2.6 unknown-field rejection is total.

Input bytes: an Intent whose declared initial status pool contains token type tag 7. Expected verdict: **malformed — unknown token type tag**. Bytes:

```
a9004b516e49522d 496e74656e74016e 76302e332e302d64 726166742e350269
7169722d656e632d 3103717169722d70 726f662d676f6c64 656e2d3104800501
0658280000000000 0000000000000000 0000000000000000 0000010000000000
0000100000000000 0000000780088107
```

## Family X — Execution Vectors

Each vector commits a complete expected outcome under §4.4's format: the encoded Intent, the declared version tuple `V` with the profile's charge parameters, the complete ordered transition-record sequence, the terminal receipt with its token itemization, the reconstructed ledger, and the expected terminal outcome. A vector passes only when both implementations independently match it at the five agreement points of §4.1.

### X1 — `exec-conservation-100-60-40`

Cites: §3.3 conservation identities (F13, v0.3.0-draft.7) — the finding's own worked settlement, realized; §3.4 admission; §4.4 conservation family.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=2 bnd=(int, <=2)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: CMP ref=0 ref=2 rng=[-1, 1]
  4: BR cc=GT ref=3 true=block 1 false=block 2
block 1 (arity 0):
  0: ADD ref=0 ref=1 bnd=(int, <=3)
  1: ADD ref=0 ref=1 bnd=(int, <=3)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: ADD ref=0 ref=1 bnd=(int, <=3)
  4: ADD ref=0 ref=1 bnd=(int, <=3)
  5: ADD ref=0 ref=1 bnd=(int, <=3)
  6: ADD ref=0 ref=1 bnd=(int, <=3)
  7: ADD ref=0 ref=1 bnd=(int, <=3)
  8: ADD ref=0 ref=1 bnd=(int, <=3)
  9: ADD ref=0 ref=1 bnd=(int, <=3)
  10: ADD ref=0 ref=1 bnd=(int, <=3)
  11: ADD ref=0 ref=1 bnd=(int, <=3)
  12: ADD ref=0 ref=1 bnd=(int, <=3)
  13: ADD ref=0 ref=1 bnd=(int, <=3)
  14: HALT
block 2 (arity 0):
  0: ADD ref=0 ref=1 bnd=(int, <=3)
  1: ADD ref=0 ref=1 bnd=(int, <=3)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: ADD ref=0 ref=1 bnd=(int, <=3)
  4: HALT
```

Intent envelope (canonical CBOR, 1680 bytes; instruction stream entry a byte string of 201 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800503
06590648 00000000 00000000 00000000
00000000 00000000 00000005 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000002 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000002 00000000 00000000 00000000
00000005 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000003 00000000 00000000 00000000
0000000a 00000000 00000000 00000000
00000002 00000000 00000002 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000000 00000000
0000000b 00000000 00000004 00000000
00000003 00000000 00000001 00000000
00000000 00000000 00000002 00000000
00000000 00000000 00000000 00000000
00000001 00000000 00000000 00000000
0000000f 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000005 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000003 00000000
00000000 00000000 00000010 00000000
00000000 00000000 00000002 00000000
00000000 00000000 00000005 00000000
00000005 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000003 00000000 00000000 00000000
00000005 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000003 00000000 00000000 00000000
00000005 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000003 00000000 00000000 00000000
00000005 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000003 00000000 00000000 00000000
00000010 00000000 00000000 07800880
```

Admission: no back-edges; the maximum bounded path is block 0 (5 transitions, 20 U) plus block 1 (15 transitions, 60 U) = 80 U, plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (10 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 64000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 64000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000002 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 64000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 8  post_state_root   8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     8083181c 18640002 02820000 80836e76
                     302e332e 302d6472 6166742e 35697169
                     722d656e 632d3171 7169722d 70726f66
                     2d676f6c 64656e2d 31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     8083181c 18640002 02820000 80836e76
                     302e332e 302d6472 6166742e 35697169
                     722d656e 632d3171 7169722d 70726f66
                     2d676f6c 64656e2d 31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     8083181c 18640002 02820000 80836e76
                     302e332e 302d6472 6166742e 35697169
                     722d656e 632d3171 7169722d 70726f66
                     2d676f6c 64656e2d 31
 8  post_state_root   8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03808318
                     20186400 03038200 0080836e 76302e33
                     2e302d64 72616674 2e356971 69722d65
                     6e632d31 71716972 2d70726f 662d676f
                     6c64656e 2d31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 4 — completed
 1  prev_root         8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03808318
                     20186400 03038200 0080836e 76302e33
                     2e302d64 72616674 2e356971 69722d65
                     6e632d31 71716972 2d70726f 662d676f
                     6c64656e 2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000002
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03808318
                     20186400 03038200 0080836e 76302e33
                     2e302d64 72616674 2e356971 69722d65
                     6e632d31 71716972 2d70726f 662d676f
                     6c64656e 2d31
 8  post_state_root   8a820003 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18241864
                     00040482 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 5 — completed
 1  prev_root         8a820003 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18241864
                     00040482 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      000000000000000b 0000000000000004 0000000000000003
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820003 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18241864
                     00040482 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 8  post_state_root   8a820004 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18281864
                     00050582 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 6 — completed
 1  prev_root         8a820004 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18281864
                     00050582 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        6
 4  pc                [2, 0]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820004 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018083 18281864
                     00050582 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 8  post_state_root   8a820200 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000380 83182c18 64000606
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 7 — completed
 1  prev_root         8a820200 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000380 83182c18 64000606
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        7
 4  pc                [2, 1]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820200 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000380 83182c18 64000606
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 8  post_state_root   8a820201 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 80831830 18640007 07820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 8 — completed
 1  prev_root         8a820201 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 80831830 18640007 07820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        8
 4  pc                [2, 2]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820201 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 80831830 18640007 07820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820202 80878300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03808318 34186400 08088200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 9 — completed
 1  prev_root         8a820202 80878300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03808318 34186400 08088200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        9
 4  pc                [2, 3]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820202 80878300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03808318 34186400 08088200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 8  post_state_root   8a820203 80888300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03831507 58180000 00000000 00010000
                     00000000 00010000 00000000 00038083
                     18381864 00090982 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 10 — completed
 1  prev_root         8a820203 80888300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03831507 58180000 00000000 00010000
                     00000000 00010000 00000000 00038083
                     18381864 00090982 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        10
 4  pc                [2, 4]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820203 80888300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03831507 58180000 00000000 00010000
                     00000000 00010000 00000000 00038083
                     18381864 00090982 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 8  post_state_root   8a820204 80888300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015818 00000000 00000001
                     00000000 00000001 00000000 00000002
                     83020258 18000000 00000000 01000000
                     00000000 01000000 00000000 03830303
                     58180000 00000000 00020000 00000000
                     00010000 00000000 00018312 04581800
                     00000000 00000100 00000000 00000100
                     00000000 00000383 13055818 00000000
                     00000001 00000000 00000001 00000000
                     00000003 83140658 18000000 00000000
                     01000000 00000000 01000000 00000000
                     03831507 58180000 00000000 00010000
                     00000000 00010000 00000000 00038083
                     183c1864 18280a0a 82010180 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     40
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization []
  3  selected_token   true   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 6e76302e 332e302d
  64726166 742e3569 7169722d 656e632d
  31717169 722d7072 6f662d67 6f6c6465
  6e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 60 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_add        20 U (5 postings)
    q_const       8 U (2 postings)
    q_cmp         4 U (1 posting)
    q_br          4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  100 /    0
    step  2:   28 /  100 /    0
    step  3:   32 /  100 /    0
    step  4:   36 /  100 /    0
    step  5:   40 /  100 /    0
    step  6:   44 /  100 /    0
    step  7:   48 /  100 /    0
    step  8:   52 /  100 /    0
    step  9:   56 /  100 /    0
    step 10:   60 /  100 /   40
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 100 = 60 + 40 + 0
    remaining_budget = W_budget - charged               : 100 - 60 = 40
    refund_final = B(P) - charged                       : 100 - 60 = 40
    reserve_outstanding after terminal settlement       : 0
```

### X2 — `exec-admission-refusal`

Cites: §3.4 admission rule and refusal (the refusal execution is the single rejection transition, the ledger closes at zero postings).

Declared parameters: `W_budget` = 60 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent: byte-identical to X1's committed envelope (equal vectors are byte-equal vectors, §4.4; the envelope is citable at X1).

Admission: identical program bytes to X1 — the same B(P) = 100 U; the declared budget 60 U is below it.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (1 record, complete ordered sequence; field values canonical per §2.6):

```
record 1 — rejection (posts nothing)
 1  prev_root         8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      9e3d1565 106b5041 9322bee5 96c834eb
                     90c06677 e8793337 8b7f3f6a 898c7fdd
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80808187 01000000 03020383
                     00000001 01820103 80836e76 302e332e
                     302d6472 6166742e 35697169 722d656e
                     632d3171 7169722d 70726f66 2d676f6c
                     64656e2d 31
 9  work_charged      0
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   true   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87010000 00030203 03f60483
  6e76302e 332e302d 64726166 742e3569
  7169722d 656e632d 31717169 722d7072
  6f662d67 6f6c6465 6e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 0 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
  settlement trace (charged / reserved / refunded after each transition):
    step  1:    0 /    0 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 0 = 0 + 0 + 0
    remaining_budget = W_budget - charged               : 60 - 0 = 60
    refund_final                                        : the escrow never opened (admission refusal) — no terminal refund
    reserve_outstanding after terminal settlement       : 0
```

### X3 — `exec-halt-no-solution`

Cites: §2.5 total terminalization function T, case (2) with an outstanding STATUS_NEG_EVEN_ROOT; §2.5 declared initial status pool (the conformance birth path); §4.4 QO-22 test list — unresolved debt at every exit.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: HALT
```

Intent envelope (canonical CBOR, 232 bytes; instruction stream entry a byte string of 20 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800501
0658a000 00000000 00000000 00000000
00000000 00000000 00000300 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000000 00000000 00000d00 00000000
00000200 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00001000 00000000
00000007 80088102
```

Admission: no back-edges; the single path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808187 00000000 02020183
                     00000000 00820000 80836e76 302e332e
                     302d6472 6166742e 35697169 722d656e
                     632d3171 7169722d 70726f66 2d676f6c
                     64656e2d 31
 2  program_root      0322b211 ca5719b6 6d6e43be 029bd306
                     428cf09b 7d1f73a6 3f1eab25 28507011
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808187 00000000 02020183
                     00000000 00820000 80836e76 302e332e
                     302d6472 6166742e 35697169 722d656e
                     632d3171 7169722d 70726f66 2d676f6c
                     64656e2d 31
 8  post_state_root   8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181820 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181820 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 2  program_root      0322b211 ca5719b6 6d6e43be 029bd306
                     428cf09b 7d1f73a6 3f1eab25 28507011
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181820 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 8  post_state_root   8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 20000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 20000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 2  program_root      0322b211 ca5719b6 6d6e43be 029bd306
                     428cf09b 7d1f73a6 3f1eab25 28507011
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 20000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 8  post_state_root   8a820002 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020203
                     83182018 20000303 82010280 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[3, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020281 87000000 00020203 03850000
  00000204 836e7630 2e332e30 2d647261
  66742e35 69716972 2d656e63 2d317171
  69722d70 726f662d 676f6c64 656e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
    q_obs         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X4 — `exec-promote-success`

Cites: §2.5 lifecycle verbs (consumption by the declared PROMOTE instruction); §2.3 PROMOTE row; §4.4 QO-22 test list — the success-after-promotion path.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: PROMOTE
  3: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  4: HALT
```

Intent envelope (canonical CBOR, 312 bytes; instruction stream entry a byte string of 30 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800501
0658f000 00000000 00000000 00000000
00000000 00000000 00000500 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000000 00000000 00000d00 00000000
00000200 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000e00 00000000
00000000 00000000 00000d00 00000000
00000200 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00001000 00000000
00000007 80088102
```

Admission: no back-edges; the single path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808187 00000000 02020183
                     00000000 00820000 80836e76 302e332e
                     302d6472 6166742e 35697169 722d656e
                     632d3171 7169722d 70726f66 2d676f6c
                     64656e2d 31
 2  program_root      a3897504 f1f81ab3 9af95334 86cf5632
                     b806d0b2 41a39cea 07e8ca77 b3f804f8
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808187 00000000 02020183
                     00000000 00820000 80836e76 302e332e
                     302d6472 6166742e 35697169 722d656e
                     632d3171 7169722d 70726f66 2d676f6c
                     64656e2d 31
 8  post_state_root   8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181828 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181828 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 2  program_root      a3897504 f1f81ab3 9af95334 86cf5632
                     b806d0b2 41a39cea 07e8ca77 b3f804f8
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00500000 00000000
                     00010000 00000000 00008187 00000000
                     02020183 18181828 00010182 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 8  post_state_root   8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 28000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 28000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 2  program_root      a3897504 f1f81ab3 9af95334 86cf5632
                     b806d0b2 41a39cea 07e8ca77 b3f804f8
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020201
                     83181c18 28000202 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 8  post_state_root   8a820002 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020202
                     83182018 28000303 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 4 — completed
 1  prev_root         8a820002 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020202
                     83182018 28000303 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 2  program_root      a3897504 f1f81ab3 9af95334 86cf5632
                     b806d0b2 41a39cea 07e8ca77 b3f804f8
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820002 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000181 87000000 00020202
                     83182018 28000303 82000080 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 8  post_state_root   8a820003 80838300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000183 02025000 00000000
                     00000100 00000000 00000081 87000000
                     00020202 83182418 28000404 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 5 — completed
 1  prev_root         8a820003 80838300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000183 02025000 00000000
                     00000100 00000000 00000081 87000000
                     00020202 83182418 28000404 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 2  program_root      a3897504 f1f81ab3 9af95334 86cf5632
                     b806d0b2 41a39cea 07e8ca77 b3f804f8
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820003 80838300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000183 02025000 00000000
                     00000100 00000000 00000081 87000000
                     00020202 83182418 28000404 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 8  post_state_root   8a820004 80838300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000183 02025000 00000000
                     00000100 00000000 00000081 87000000
                     00020202 83182818 28000505 82010180
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
  3  selected_token   true   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010281 87000000 00020202 03f60483
  6e76302e 332e302d 64726166 742e3569
  7169722d 656e632d 31717169 722d7072
  6f662d67 6f6c6465 6e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_obs         8 U (2 postings)
    q_const       4 U (1 posting)
    q_promote     4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   40 /    0
    step  2:   28 /   40 /    0
    step  3:   32 /   40 /    0
    step  4:   36 /   40 /    0
    step  5:   40 /   40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X5 — `exec-domain-error`

Cites: §2.3 DIV row (zero divisor: the STATUS_DOMAIN_ERROR birth, terminalizing); §2.5 T case (1) — a terminalizing birth exits at its own transition; §3.1 rejection transitions post nothing.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=7)
  1: CONST imm=0 bnd=(int, <=0)
  2: DIV ref=0 ref=1 bnd=(int, <=7)
  3: HALT
```

Intent envelope (canonical CBOR, 303 bytes; instruction stream entry a byte string of 29 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800501
0658e800 00000000 00000000 00000000
00000000 00000000 00000400 00000000
00000100 00000000 00000100 00000000
00000100 00000000 00000700 00000000
00000100 00000000 00000100 00000000
00000100 00000000 00000700 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000000 00000000
00000800 00000000 00000000 00000000
00000100 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000700 00000000 00000000 00000000
00001000 00000000 00000007 800880
```

Admission: no back-edges; the single path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      8aec9612 c0e6a09b 40f009c6 fa3f20a6
                     a1c96e56 99ff1d61 e4b90c26 7e78192f
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000007 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000780 83181818 24000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000780 83181818 24000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 2  program_root      8aec9612 c0e6a09b 40f009c6 fa3f20a6
                     a1c96e56 99ff1d61 e4b90c26 7e78192f
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000780 83181818 24000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 8  post_state_root   8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000783 01015000 00000000 00000100
                     00000000 00000080 83181c18 24000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000783 01015000 00000000 00000100
                     00000000 00000080 83181c18 24000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 2  program_root      8aec9612 c0e6a09b 40f009c6 fa3f20a6
                     a1c96e56 99ff1d61 e4b90c26 7e78192f
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000008 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000783 01015000 00000000 00000100
                     00000000 00000080 83181c18 24000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 8  post_state_root   8a820002 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000783 01015000 00000000 00000100
                     00000000 00000081 87030002 00040203
                     83181c18 24080303 82010480 836e7630
                     2e332e30 2d647261 66742e35 69716972
                     2d656e63 2d317171 69722d70 726f662d
                     676f6c64 656e2d31
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 0, 2, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [3, 0, 2, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [3, 0, 2, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040281 87030002 00040203 03850300
  02000404 836e7630 2e332e30 2d647261
  66742e35 69716972 2d656e63 2d317171
  69722d70 726f662d 676f6c64 656e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   36 /    0
    step  2:   28 /   36 /    0
    step  3:   28 /   36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X6 — `exec-lookup-miss-fallback-success`

Cites: §2.3 CLOAD row (miss: births STATUS_LOOKUP_MISS, routes to the declared fallback edge); §2.5 tier rule (the token is consumed only when that fallback entry completes successfully — F12, v0.3.0-draft.5); §4.4 QO-22 test list — success after recovery.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 353 bytes; instruction stream entry a byte string of 34 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04814800
00000000 00000105 03065901 10000000
00000000 00000000 00000000 00000000
00000000 02000000 00000000 02000000
00000000 00000000 00000000 01000000
00000000 01000000 00000000 01000000
00000000 01000000 00000000 09000000
00000000 00000000 00000000 0c000000
00000000 02000000 00000000 00000000
00000000 00000000 00000000 01000000
00000000 00000000 00000000 02000000
00000000 01000000 00000000 01000000
00000000 01000000 00000000 09000000
00000000 01000000 00000000 01000000
00000000 01000000 00000000 09000000
00000000 00000000 00000000 10000000
00000000 00000000 00000000 02000000
00000000 00000000 00000000 01000000
00000000 10000000 00000000 00078008
80
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      a40114b5 468188bc b9baecb8 6d0b4603
                     f5d62626 93aafda1 bf202b93 a16823ae
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80808187 01000000 01010183
                     18181820 00010182 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80808187 01000000 01010183
                     18181820 00010182 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 2  program_root      a40114b5 468188bc b9baecb8 6d0b4603
                     f5d62626 93aafda1 bf202b93 a16823ae
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808187 01000000 01010183
                     18181820 00010182 00008083 6e76302e
                     332e302d 64726166 742e3569 7169722d
                     656e632d 31717169 722d7072 6f662d67
                     6f6c6465 6e2d31
 8  post_state_root   8a820100 80818301 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000981 87010000 00010101 83181c18
                     20000202 82000080 836e7630 2e332e30
                     2d647261 66742e35 69716972 2d656e63
                     2d317171 69722d70 726f662d 676f6c64
                     656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820100 80818301 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000981 87010000 00010101 83181c18
                     20000202 82000080 836e7630 2e332e30
                     2d647261 66742e35 69716972 2d656e63
                     2d317171 69722d70 726f662d 676f6c64
                     656e2d31
 2  program_root      a40114b5 468188bc b9baecb8 6d0b4603
                     f5d62626 93aafda1 bf202b93 a16823ae
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820100 80818301 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000981 87010000 00010101 83181c18
                     20000202 82000080 836e7630 2e332e30
                     2d647261 66742e35 69716972 2d656e63
                     2d317171 69722d70 726f662d 676f6c64
                     656e2d31
 8  post_state_root   8a820101 80818301 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000981 87010000 00010102 83182018
                     20000303 82010180 836e7630 2e332e30
                     2d647261 66742e35 69716972 2d656e63
                     2d317171 69722d70 726f662d 676f6c64
                     656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   true   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010281 87010000 00010102 03f60483
  6e76302e 332e302d 64726166 742e3569
  7169722d 656e632d 31717169 722d7072
  6f662d67 6f6c6465 6e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X7 — `exec-trip-ceiling-rejection`

Cites: §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge); §2.3 derived counters (read from the record sequence); §3.3 terminal settlement at a rejection; §4.4 loops family.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=0 ref=1 rng=[-1, 1]
  3: BR cc=NE ref=2 true=block 1 false=block 2
block 1 (arity 0):
  0: JMP block 0
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 464 bytes; instruction stream entry a byte string of 49 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800503
06590188 00000000 00000000 00000000
00000000 00000000 00000004 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000000 00000000 00000001 00000000
00000001 00000000 00000000 00000000
00000001 00000000 00000001 00000000
00000000 00000000 00000000 00000000
0000000a 00000000 00000000 00000000
00000001 00000000 00000002 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000001 00000000 00000000 00000000
0000000b 00000000 00000003 00000000
00000002 00000000 00000001 00000000
00000000 00000000 00000002 00000000
00000000 00000000 00000000 00000000
00000001 00000000 00000000 00000000
00000001 00000000 0000000c 00000000
00000000 00000000 00000000 00000000
00000000 00000000 00000002 00000000
00000000 00000000 00000001 00000000
00000010 00000000 00000000 07800880
```

Admission: one back-edge (block 1 to block 0, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the true edge, one back-edge traversal, block 0 again, the false edge, and block 2 — 10 transitions (40 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (10 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808083 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 3c000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 3c000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000180 83181818 3c000101 82000080
                     836e7630 2e332e30 2d647261 66742e35
                     69716972 2d656e63 2d317171 69722d70
                     726f662d 676f6c64 656e2d31
 8  post_state_root   8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000080 83181c18 3c000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000080 83181c18 3c000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000080 83181c18 3c000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 8  post_state_root   8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831820 183c0003 03820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 4 — completed
 1  prev_root         8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831820 183c0003 03820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000002
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820002 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831820 183c0003 03820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820003 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831824 183c0004 04820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 5 — completed
 1  prev_root         8a820003 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831824 183c0004 04820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        5
 4  pc                [1, 0]
 5  decoded_word      000000000000000c 0000000000000000 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820003 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831824 183c0004 04820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820100 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831828 183c0005 05820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 6 — completed
 1  prev_root         8a820100 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831828 183c0005 05820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        6
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820100 80838300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 01015000 00000000 00000100
                     00000000 00000083 02025818 00000000
                     00000001 00000000 00000001 00000000
                     00000001 80831828 183c0005 05820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83020258 18000000 00000000
                     01000000 00000000 01000000 00000000
                     01808318 2c183c00 06068200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 7 — completed
 1  prev_root         8a820000 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83020258 18000000 00000000
                     01000000 00000000 01000000 00000000
                     01808318 2c183c00 06068200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        7
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80848300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83020258 18000000 00000000
                     01000000 00000000 01000000 00000000
                     01808318 2c183c00 06068200 0080836e
                     76302e33 2e302d64 72616674 2e356971
                     69722d65 6e632d31 71716972 2d70726f
                     662d676f 6c64656e 2d31
 8  post_state_root   8a820001 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01808318 30183c00 07078200
                     0080836e 76302e33 2e302d64 72616674
                     2e356971 69722d65 6e632d31 71716972
                     2d70726f 662d676f 6c64656e 2d31
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 8 — completed
 1  prev_root         8a820001 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01808318 30183c00 07078200
                     0080836e 76302e33 2e302d64 72616674
                     2e356971 69722d65 6e632d31 71716972
                     2d70726f 662d676f 6c64656e 2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        8
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80858300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01808318 30183c00 07078200
                     0080836e 76302e33 2e302d64 72616674
                     2e356971 69722d65 6e632d31 71716972
                     2d70726f 662d676f 6c64656e 2d31
 8  post_state_root   8a820002 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1834183c 00080882 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 9 — completed
 1  prev_root         8a820002 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1834183c 00080882 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        9
 4  pc                [0, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000002
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820002 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1834183c 00080882 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 8  post_state_root   8a820003 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1838183c 00090982 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 10 — rejection (posts nothing)
 1  prev_root         8a820003 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1838183c 00090982 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 2  program_root      80bea2a0 973509f7 a448aa32 339509d5
                     0597545a ec023bd8 4857f205 53b1c21b
 3  step_index        10
 4  pc                [1, 0]
 5  decoded_word      000000000000000c 0000000000000000 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820003 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018083 1838183c 00090982 00008083
                     6e76302e 332e302d 64726166 742e3569
                     7169722d 656e632d 31717169 722d7072
                     6f662d67 6f6c6465 6e2d31
 8  post_state_root   8a820100 80868300 00581800 00000000
                     00000100 00000000 00000100 00000000
                     00000183 00035818 00000000 00000001
                     00000000 00000001 00000000 00000001
                     83010150 00000000 00000001 00000000
                     00000000 83010450 00000000 00000001
                     00000000 00000000 83020258 18000000
                     00000000 01000000 00000000 01000000
                     00000000 01830205 58180000 00000000
                     00010000 00000000 00010000 00000000
                     00018187 0a010000 03020383 1838183c
                     040a0a82 01038083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 9  work_charged      0
10  work_refunded     4
11  clock             10
12  status            [[1, [10, 1, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [10, 1, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   true   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 870a0100 00030203 03f60483
  6e76302e 332e302d 64726166 742e3569
  7169722d 656e632d 31717169 722d7072
  6f662d67 6f6c6465 6e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 56 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      16 U (4 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_jmp         4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   60 /    0
    step  2:   28 /   60 /    0
    step  3:   32 /   60 /    0
    step  4:   36 /   60 /    0
    step  5:   40 /   60 /    0
    step  6:   44 /   60 /    0
    step  7:   48 /   60 /    0
    step  8:   52 /   60 /    0
    step  9:   56 /   60 /    0
    step 10:   56 /   60 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 60 = 56 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 56 = 44
    refund_final = B(P) - charged                       : 60 - 56 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X8 — `exec-multi-token-precedence`

Cites: §2.5 T case (2) — the earliest-birth outstanding Tier S token decides (ordered lexicographically by the full token identifier, F15); §4.4 QO-22 test list — multiple outstanding tokens.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: HALT
```

Intent envelope (canonical CBOR, 233 bytes; instruction stream entry a byte string of 20 words):

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 31037171 69722d70
726f662d 676f6c64 656e2d31 04800501
0658a000 00000000 00000000 00000000
00000000 00000000 00000300 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000000 00000000 00000d00 00000000
00000200 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00001000 00000000
00000007 80088202 03
```

Admission: no back-edges; the single path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         8a820000 80808287 00000000 02020187
                     00000001 03020183 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 2  program_root      37895055 9904f4ef cdeb543b d8641f5d
                     3f3b8192 b5f353c8 500f8427 60672bf0
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80808287 00000000 02020187
                     00000001 03020183 00000000 00820000
                     80836e76 302e332e 302d6472 6166742e
                     35697169 722d656e 632d3171 7169722d
                     70726f66 2d676f6c 64656e2d 31
 8  post_state_root   8a820000 80818300 00500000 00000000
                     00010000 00000000 00008287 00000000
                     02020187 00000001 03020183 18181820
                     00010182 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 2 — completed
 1  prev_root         8a820000 80818300 00500000 00000000
                     00010000 00000000 00008287 00000000
                     02020187 00000001 03020183 18181820
                     00010182 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 2  program_root      37895055 9904f4ef cdeb543b d8641f5d
                     3f3b8192 b5f353c8 500f8427 60672bf0
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820000 80818300 00500000 00000000
                     00010000 00000000 00008287 00000000
                     02020187 00000001 03020183 18181820
                     00010182 00008083 6e76302e 332e302d
                     64726166 742e3569 7169722d 656e632d
                     31717169 722d7072 6f662d67 6f6c6465
                     6e2d31
 8  post_state_root   8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000182 87000000 00020201
                     87000000 01030201 83181c18 20000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

```
record 3 — completed
 1  prev_root         8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000182 87000000 00020201
                     87000000 01030201 83181c18 20000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 2  program_root      37895055 9904f4ef cdeb543b d8641f5d
                     3f3b8192 b5f353c8 500f8427 60672bf0
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a820001 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000182 87000000 00020201
                     87000000 01030201 83181c18 20000202
                     82000080 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 8  post_state_root   8a820002 80828300 00500000 00000000
                     00010000 00000000 00008301 01581800
                     00000000 00000100 00000000 00000100
                     00000000 00000182 87000000 00020203
                     87000000 01030203 83182018 20000303
                     82010280 836e7630 2e332e30 2d647261
                     66742e35 69716972 2d656e63 2d317171
                     69722d70 726f662d 676f6c64 656e2d31
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[3, [0, 0, 0, 0, 2]], [3, [0, 0, 0, 1, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [0, 0, 0, 1, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.5", "qir-enc-1", "qir-prof-golden-1"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020282 87000000 00020203 87000000
  01030203 03850000 00000204 836e7630
  2e332e30 2d647261 66742e35 69716972
  2d656e63 2d317171 69722d70 726f662d
  676f6c64 656e2d31
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
    q_obs         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

## Suite Register

This seed is increment 1 of the golden corpus (§4.5: "a small suite"). The
following declared obligations of §4.4 are registered as expansion
increments of the same stage, hand-derived under this file's discipline and
committed before the artifacts that need them:

- the remaining `QO-22`/F12 terminalization combinations (every combination of {outstanding Tier S token class} × {promoted, not promoted} × {fallback success, fallback repeated miss, fallback typed rejection} × {exit event}) — the seed fixes one vector per listed surface and the combinations complete per the §4.4 closure;
- `CLOAD` hit vectors (D entries present, bound value checked against its declared bound), and a multi-block fallback completion vector fixing the consumption site beyond a HALT-terminated fallback block;
- `TUPLE`/`FIELD` value and execution vectors, zoom `CMP` execution vectors (boundary comparison under N), and `EMIT` witness vectors (receipt ordinals and the witness payload);
- depth-ceiling vectors and the fuel-backstop-on-validator-defect vector (a backstop stop on an admitted Intent evidences a validator defect, never a satisfied admission rule — F16);
- per-class ledger trace publications beyond the seed's summaries, and the per-combination expansion of the budget-boundary family;
- encoding vectors for `EXT` record payloads whose reading types the corpus declares with its encoding version.

The independently reviewed goldens of this increment are retained unchanged
by every later expansion (§4.5); the assembler and validator are implemented
only after this corpus, are checked against it, and never generate it.

## License and Invariant Lineage

This corpus is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and inherits the invariant lineage of the QnIR v0.3 working draft: the frozen core of §1, the semantics of §2, the accounting of §3, and the conformance specification of §4 bind every expected value above, and no vector redefines, amends, or overrides any canonical specification. The derivation records of this increment are held by the maintainer alongside the draft's offline archive; every expected byte is recomputable from the committed rules plus the declared parameters of this file.
