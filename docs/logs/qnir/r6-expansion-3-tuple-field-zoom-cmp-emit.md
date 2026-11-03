---
layout: default
title: R6 Expansion Increment 3 — the TUPLE/FIELD/zoom-CMP/EMIT-witness Completion (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (75 vectors)
nav_exclude: true
---

# R6 Expansion Increment 3 — the TUPLE/FIELD/zoom-CMP/EMIT-witness Completion (Run Log)

Artifact wave: v0.3.0-draft.16 (R6 expansion increment 3) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (committed at 0.3.0-draft.16; corpus sha-256 prefix `024e0f9c1f63c428`, 1,019,366 bytes), carrying the seed's 34 independently reviewed vectors and the 34 expansion vectors of increments 1–2 byte-unchanged plus the 7 hand-derived expansion vectors X43–X49 (the Suite Register's third bullet — the `TUPLE`/`FIELD` value and execution vectors, the zoom `CMP` execution vectors under N, and the `EMIT` witness vectors, over the declared parameter P13). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Four steps, in order:

1. the **anchored derivation** — the derivation instrument of record re-derived every committed execution vector byte-exact from its declared source form and declared parameters (all 42, seed and increments 1–2 alike) before any expansion byte was derived — the golden-first order applied to expansions — and, after the instrument repair registered below, re-anchored over the full 49-vector execution family (`ANCHORING: all 49 committed execution vectors re-derived byte-exact from the declared source forms and parameters`);
2. the **first runs against the derived bytes** — two defects were registered and repaired before any committed byte existed for this wave (details below): the derivation instrument's envelope emitter defaulted the declared dependency list to the P9 address on the seven new no-`CLOAD` vectors, and the reference artifact's source-form parser mis-trimmed a zoom literal's bound suffix and aborted on the corpus's zoom-literal listings;
3. the **conformance runs of record** — both artifacts against the corrected corpus (transcripts below); no divergence was registered: both implementations reproduced every committed expected outcome on the first run against the corrected bytes, so §4.1's adjudication rule was never triggered against the committed corpus and no execution rule was repaired in this wave;
4. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, reproducibility evidence only (§4.2).

## The two registered pre-publication defects

Both were found and repaired in the same wave, before the corpus's draft.16 bytes were committed anywhere; the corpus of record carries only the corrected derivation.

- **The instrument's dependency-list default.** The derivation instrument's envelope emitter carried a default that populated the declared dependency list (envelope key 4) with the corpus's declared P9 content address on every vector it derived; no vector of this increment executes a `CLOAD`, so the committed rules require the empty list — key 4 is the address table that `CLOAD`'s `addr` operand indexes (§2.6), the seed's own committed convention is the empty list for programs that cite no address (X1, X5) and exactly the cited addresses otherwise (X6, X39–X42), and the increment's vectors declare `D` = no entries. The defect was exposed by the reference artifact's independent re-encoding check: the assembler derives the declared dependency list from the program's cited `CLOAD` addresses and its envelopes came out 9 bytes short of the derived bytes on all seven vectors (the one-entry list's exact cost). Adjudicated against the committed rules and the seed's convention, the instrument was repaired (`deps=()` for this increment's vectors), the seven vectors re-derived from their declared source forms and parameters, and the anchor re-run over all 49 execution vectors byte-exact. Neither implementation was involved in the adjudication; no vector byte of the committed corpus was ever the defective form.
- **The reference artifact's zoom-literal source parse.** The assembler's source-form parser cut a zoom literal's trailing text with a suffix trim that does not fire on the committed listing form (`imm=zoom(m, z) bnd=(zoom, <=n)` — the string ends with the bound's own `)`, not the literal's), so the exponent parse received the bound suffix and aborted on X46–X48's listings. Repaired in the same wave: the literal is cut at the bound suffix before the pair split. The repair touches the source-form parser only — no execution rule, no committed byte, no golden.

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.16, toolchain pinned in the crate (channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64, Linux; release profile; no network access at run time
- second implementation: CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

## How to reproduce

```
src/qnir/target/release/qnir docs/qnir_goldens.md
python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The artifact share of this wave

Unlike increments 1–2, whose artifact shares changed no execution rule, this increment's artifact share implements the committed execution surfaces no committed vector exercised before: the `TUPLE`, `FIELD`, and `EMIT` rows of §2.3's closed inventory and the zoom-carrying operand surface of the `CMP` row, in both artifacts — bound to exactly the committed rows, the corpus adding no execution rule of its own:

- the reference interpreter's decode gains the three operand shapes (`TUPLE` = cnt, k field declarations, k component references; `FIELD` = ref, idx; `EMIT` = the kind immediate, cnt, k refs) and their dispatch arms; the `CONST` result-type accept set gains the zoom literal; the record's field 13 carries the ordinals the transition emitted (P13(d)); the canonical configuration's receipt-order queue component carries the witness entries; and the `CMP` arm reads its operands as values, applies P13(b)'s declared-range check to each zoom-carrying operand, and decides by the zoom-boundary comparison under N with the exact-integer shift;
- the second implementation gains the same operand shapes, dispatch arms, field-13 and queue semantics, and the `CMP` zoom surface, from the committed text under §4.1's independence declaration;
- both artifacts' `CONST` and `TUPLE` arms apply P13(c)'s bound reading (a zoom result's INT_POS upper bound constrains the normalized exponent; a tuple declaration's bound word is present and vacuous).

## The conformance runs (both artifacts, first run, expanded corpus)

Verdict: all 75 committed vectors reproduce at the five agreement points of §4.1 — suite E 11/11 and suite I 3/3 canonical bytes (agreement point 1), suite M 12/12 committed verdicts exact (agreement point 2), all committed envelopes decoded with their source-grammar listings regenerated from the decoded bytes, and 49/49 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound. Execution share of the transcripts (the increment's seven vectors; every other vector's line is in the full transcript):

```
  X43 exec-tuple-field-extract: envelope match (424 bytes; stream match: true)
  X44 exec-tuple-component-bound-rejection: envelope match (287 bytes; stream match: true)
  X45 exec-tuple-nested-field-extract: envelope match (592 bytes; stream match: true)
  X46 exec-cmp-zoom-boundary-equal: envelope match (384 bytes; stream match: true)
  X47 exec-cmp-zoom-order-exact-shift: envelope match (472 bytes; stream match: true)
  X48 exec-cmp-zoom-range-rejection: envelope match (392 bytes; stream match: true)
  X49 exec-emit-witness-queue: envelope match (440 bytes; stream match: true)
  X43 exec-tuple-field-extract: executed — 6/6 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 44 U; conservation 44 = 44 + 0 + 0
  X44 exec-tuple-component-bound-rejection: executed — 2/2 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 32 = 24 + 8 + 0
  X45 exec-tuple-nested-field-extract: executed — 8/8 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 52 U; conservation 52 = 52 + 0 + 0
  X46 exec-cmp-zoom-boundary-equal: executed — 4/4 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 36 U; conservation 36 = 36 + 0 + 0
  X47 exec-cmp-zoom-order-exact-shift: executed — 5/5 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 40 U; conservation 40 = 40 + 0 + 0
  X48 exec-cmp-zoom-range-rejection: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 28 U; conservation 36 = 28 + 8 + 0
  X49 exec-emit-witness-queue: executed — 6/6 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 44 U; conservation 44 = 44 + 0 + 0
suite X execution (reference interpreter): PASS — 49/49 committed expected outcomes reproduced at the five agreement points of §4.1

result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
  X43 exec-tuple-field-extract: executed — 6/6 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 44 U; conservation 44 = 44 + 0 + 0
  X44 exec-tuple-component-bound-rejection: executed — 2/2 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 32 = 24 + 8 + 0
  X45 exec-tuple-nested-field-extract: executed — 8/8 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 52 U; conservation 52 = 52 + 0 + 0
  X46 exec-cmp-zoom-boundary-equal: executed — 4/4 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 36 U; conservation 36 = 36 + 0 + 0
  X47 exec-cmp-zoom-order-exact-shift: executed — 5/5 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 40 U; conservation 40 = 40 + 0 + 0
  X48 exec-cmp-zoom-range-rejection: executed — 3/3 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 28 U; conservation 36 = 28 + 8 + 0
  X49 exec-emit-witness-queue: executed — 6/6 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 44 U; conservation 44 = 44 + 0 + 0
suite X execution (second implementation): PASS — 49/49 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

Per-vector execution facts (each against the same committed expected outcomes both implementations matched): X43's `TUPLE` binds the canonical two-component tuple value to a fresh instance and both `FIELD` extractions mint the components' canonical values under their own static ids — three value-defining transitions, three fresh instances, the store carrying the tuple value and both extracted instances at the exit; X44's second component (9) exceeds its declared field bound (4) — the typed rejection fires before dispatch, no binding and no store effect, the open escrow's residue of 8 U refunding at the rejection record (the X5/X41 precedent of P11, the `CONST` lead transition having completed); X45's nested construction builds the inner tuple over components 3 and 5, then the outer tuple over 7 and the inner — the outer's second field declared TUPLE, its bound word present and vacuous under P13(c) — and the chained `FIELD` extractions mint the extracted inner tuple and then its component 3, two store entries carrying byte-equal canonical values under distinct names (the §2.2 name discipline); X46's two zoom literals denote the same normalized pair (1, 0) — the corpus's first zoom-carrying executions, the comparison binding 0 under N's equality rule; X47's (3, −1) against (1, 0) decides by the exact shift (3 against 2, result 1) and its symmetric (1, 0) against (3, −1) swaps the pair and negates (result −1), the comparison never rounding; X48's first operand's normalized exponent (5) lies outside the declared range [−4, 4] of P13(a) — the pre-dispatch typed rejection, no binding and no result instance, the open escrow's residue of 8 U refunding at the rejection record, the zoom literal itself having been admitted at its own declared bound 8 under P13(c); X49's two `EMIT` transitions append the witness records to the receipt-order queue — ordinal 0 (kind 1, the tuple's and the first constant's current-instance encodings, position [0, 3]) and ordinal 1 (kind 2, no cited values, position [0, 4]) — the queue riding the post-state roots from the first emission onward and record field 13 citing each transition's own ordinals, the terminal receipt unchanged (closed keys 0–4, tokens only). Every terminal settlement's refund closes the escrow (`reserve_outstanding` = 0).

## Independence and scope declarations

- The expansion vectors are derived from the committed text of §2.1–§2.6 and §3 plus the corpus's declared artifact-wave parameters (P1–P13) and its declared profile, through the derivation instrument of record; the instrument consults neither the reference interpreter's code nor its outputs, and neither implementation's code was consulted during the derivation. The goldens were fixed before the artifacts ran — the golden-first order applied to expansions.
- Neither implementation's execution rules were repaired against the committed corpus: the conformance runs of record reproduced every committed expected outcome on their first run, so no adjudication was registered against the committed bytes. The two registered defects above were repaired before the corpus's draft.16 bytes existed: the instrument's envelope emitter (adjudicated against the committed rules and the seed's convention) and the reference artifact's source-form parser (a parse defect, no execution rule).
- The declared inputs of families E, I, and M are unchanged; every execution input (the envelope bytes, `W_budget`, `D`, the version tuple `V`) is parsed from the committed vector text.
- The depth-ceiling and fuel-backstop fixture, the per-class ledger trace publications, and the `EXT`-payload reading types remain the registered expansion obligations they were (Suite Register bullets 4–6).

## Reproducibility

Step 4 is the same binary (reference interpreter) and the same source (second implementation) on the same corpus; each transcript is byte-identical to its run-of-record counterpart (transcript digests, 16-hex prefixes — reference interpreter `e0654d19c97ff374`, second implementation `bcd6b7beeba59c52`). This is reproducibility evidence only (§4.2); conformance rides §4.1's protocol — the committed golden results were derived from the normative rules before either artifact executed against them, and the artifacts' agreement with them is the claim.
