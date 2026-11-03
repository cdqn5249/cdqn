---
layout: default
title: R6 Expansion Increment 2 — the CLOAD-Hit and Multi-Block Fallback Completion (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (68 vectors)
nav_exclude: true
---

# R6 Expansion Increment 2 — the CLOAD-Hit and Multi-Block Fallback Completion (Run Log)

Artifact wave: v0.3.0-draft.15 (R6 expansion increment 2) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (committed at 0.3.0-draft.15; corpus sha-256 prefix `9b5195561590b1f6`), carrying the seed's 34 independently reviewed vectors and the 30 expansion vectors of increment 1 byte-unchanged plus the 4 hand-derived expansion vectors X39–X42 (the Suite Register's second bullet — the `CLOAD`-hit surface over a populated dependency store and the multi-block fallback completion). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Three steps, in order:

1. the **anchored derivation** — the derivation instrument of record re-derived every committed execution vector byte-exact from its declared source form and declared parameters (all 42, including the four new vectors' committed bytes) before either artifact executed against the expanded corpus — the golden-first order applied to expansions;
2. the **first run of both artifacts against the expanded corpus** — the conformance run of record (transcripts below); no divergence was registered: both implementations reproduced every committed expected outcome on the first run, so §4.1's adjudication rule was never triggered and no repair was made in this wave;
3. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, reproducibility evidence only (§4.2).

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.15, toolchain pinned in the crate (channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64, Linux; release profile; no network access at run time
- second implementation: CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

## How to reproduce

```
src/qnir/target/release/qnir docs/qnir_goldens.md
python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The artifact share of this wave

No execution rule changed in either artifact: the hit path with its pre-dispatch bound check, the store binding, and the multi-block fallback routing are committed §2.3/§2.4/§2.5 rules both implementations already executed. The wave's artifact share is the declared dependency table's harness plumbing (the corpus's declared rendering parameter P12): the reference interpreter's `run_vector` entry gains the declared dependency table (the `declared_dep_values` parse of the vector's Declared parameters line, each declared value resolved against the envelope's dependency list and encoded per §2.6), and the second implementation's corpus runner parses the same clause (the increment-4 guard "a populated dependency store is an expansion obligation" is discharged by exactly that expansion). The declared table is vector input (§2.1's `D`), never envelope content: X39's and X40's Intent envelopes are byte-equal to X6's, the three vectors differing in the declared `D` and hence in the entire expected outcome.

## The conformance runs (both artifacts, first run, expanded corpus)

Verdict: all 68 committed vectors reproduce at the five agreement points of §4.1 — suite E 11/11 and suite I 3/3 canonical bytes (agreement point 1), suite M 12/12 committed verdicts exact (agreement point 2), all committed envelopes decoded with their source-grammar listings regenerated from the decoded bytes, and 42/42 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound. Execution share of the transcripts (the increment's four vectors; every other vector's line is in the full transcript):

```
  X39 exec-cload-hit-success: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X40 exec-cload-hit-boundary: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X41 exec-cload-hit-bound-rejection: executed — 2/2 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 36 = 24 + 12 + 0
  X42 exec-cload-miss-multiblock-completion: executed — 4/4 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 36 U; conservation 40 = 36 + 4 + 0
suite X execution (reference interpreter): PASS — 42/42 committed expected outcomes reproduced at the five agreement points of §4.1

result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
  X39 exec-cload-hit-success: executed — 3/3 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X40 exec-cload-hit-boundary: executed — 3/3 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X41 exec-cload-hit-bound-rejection: executed — 2/2 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 36 = 24 + 12 + 0
  X42 exec-cload-miss-multiblock-completion: executed — 4/4 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 36 U; conservation 40 = 36 + 4 + 0
suite X execution (second implementation): PASS — 42/42 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

Per-vector execution facts (each against the same committed expected outcomes both implementations matched): X39's `CLOAD` hit binds the declared constant-table value 7 to a fresh instance of static value id 0 — the corpus's first populated-store execution, the store component of the canonical configuration carrying the entry from the hit transition onward, with no status token birthed; X40's declared value 9 equals its declared bound and is admitted, fixing the strict reading of the §2.3 rejection clause (a value exceeds its bound only when it is greater than it); X41's declared value 10 exceeds its bound 9 — the typed rejection fires before dispatch, minting nothing and touching no store entry, the fatal `EXPRESSION_TOO_COMPLEX` birth terminalizing at its own transition with the nil-selected receipt of the X10 shape (the receipt's selected-token key rides `NO_SOLUTION`/`DOMAIN_ERROR` exits only), and the open escrow's residue of 12 U refunding at the rejection record (the `CONST` lead transition having completed — the X5 precedent of P11); X42's multi-block fallback completion consumes the miss token at the fallback block's own final instruction (the `JMP` of record 3, pc [1, 1]) while the `SUCCESS` outcome is declared two blocks later at record 4's `HALT` — the declared reading DR3's consumption site, beyond the seed's HALT-terminated fallback-block shape. Every terminal settlement's refund closes the escrow (`reserve_outstanding` = 0).

## Independence and scope declarations

- The expansion vectors are derived from the committed text of §2.1–§2.6 and §3 plus the corpus's declared artifact-wave parameters (P1–P12) and its declared profile, through the derivation instrument of record; the instrument consults neither the reference interpreter's code nor its outputs, and neither implementation's code was consulted during the derivation. The goldens were fixed before the artifacts ran — the golden-first order applied to expansions.
- Neither implementation was repaired in this wave: the first run reproduced every committed expected outcome, so no adjudication was registered.
- The declared inputs of families E, I, and M are unchanged; every execution input (the envelope bytes, `W_budget`, `D`, the version tuple `V`) is parsed from the committed vector text; the declared dependency table is resolved against the envelope's dependency list and encoded per §2.6 by each artifact's own harness.
- The increment implements no new opcode execution surface in either artifact: `TUPLE`, `FIELD`, `EMIT`, witness receipts, the depth-ceiling and fuel-backstop fixture, the per-class ledger trace publications, and the `EXT`-payload reading types remain the registered expansion obligations they were at v0.3.0-draft.14 (Suite Register bullets 3–6).

## Reproducibility

Step 3 is the same binary (reference interpreter) and the same source (second implementation) on the same corpus; each transcript is byte-identical to its run-2 counterpart (transcript digests, 16-hex prefixes — reference interpreter `f0267230bb2da0bf`, second implementation `3b3597894f039bfb`). This is reproducibility evidence only (§4.2); conformance rides §4.1's protocol — the committed golden results were derived from the normative rules before either artifact executed against them, and the artifacts' agreement with them is the claim.
