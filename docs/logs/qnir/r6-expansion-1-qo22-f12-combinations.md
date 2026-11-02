---
layout: default
title: R6 Expansion Increment 1 — the QO-22/F12 Combination Completion (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (64 vectors)
nav_exclude: true
---

# R6 Expansion Increment 1 — the QO-22/F12 Combination Completion (Run Log)

Artifact wave: v0.3.0-draft.14 (R6 expansion increment 1) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (848,884 bytes, committed at 0.3.0-draft.14; corpus sha-256 prefix `487ac7d02de05090`), carrying the seed's 34 independently reviewed vectors byte-unchanged plus the 30 hand-derived expansion vectors X9–X38 (the reachable cells of the §4.4 F12 combination closure). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Three runs per artifact, in order:

1. the **first run of both artifacts against the derived expansion vectors, before any repair** — this run registered the wave's one shared defect class under §4.1's adjudication (below);
2. the **repaired run of both artifacts against the expanded corpus** — the conformance run of record (transcripts below);
3. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, reproducibility evidence only (§4.2).

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.12, toolchain pinned in the crate (channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64, Linux; release profile; no network access at run time
- second implementation: CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

## How to reproduce

```
src/qnir/target/release/qnir docs/qnir_goldens.md
python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The registered adjudication (first run, before repair)

The expansion vectors were derived from the committed normative rules through the derivation instrument of record (a third codebase anchored byte-exact on the committed seed — all 38 execution vectors re-derive byte-exact from their declared source forms and declared parameters — and consulting neither implementation), and both artifacts then ran against the derived bytes. Both implementations disagreed with the derived goldens on the same two points, on exactly the vectors whose fatal exit leaves tokens outstanding — the seed's own committed bytes are silent on both points (X2/X5/X7 declare no outstanding token at their fatal exits), so this is the F12 closure doing its work:

- **DR1 — terminalization at every exit (§2.5 T case 3).** Both implementations left the outstanding tokens un-terminalized at a fatal exit: the itemization kept the state word 1 (outstanding) where §2.5 T case 3 declares every outstanding token terminalizes at every exit, and the final record carried the birth event only. 17 vectors diverged at the final record's post-state root (the pool's state words enter the canonical configuration encoding).
- **DR2 — the selected token (§2.5 T cases 1–2; §2.6 receipt key 3).** At a fatal-birth `DOMAIN_ERROR` exit, both implementations selected the earliest-birth terminated `STATUS_NEG_EVEN_ROOT`/`STATUS_DOMAIN_ERROR` token (the pool token) where the outcome-determining token is the fatal birthed token itself (X13, X18, X23, X28, X33, X38).

Per §4.1's adjudication rule the disagreements were adjudicated against the golden results and the specification, with the vector ids and disagreeing points named above, and both implementations were repaired in this same wave — `src/qnir/src/interp.rs` (`fatal()`, `build_receipt()`), `src/qnir-py/qnir_ref.py` (`rejection()`, `build_receipt()`). The repair is seed-safe: both artifacts were rerun against the unexpanded corpus before the expanded run and reproduce the seed's 34 vectors unchanged.

## The conformance runs (both artifacts, repaired, expanded corpus)

Verdict: all 64 committed vectors reproduce at the five agreement points of §4.1 — suite E 11/11 and suite I 3/3 canonical bytes (agreement point 1), suite M 12/12 committed verdicts exact (agreement point 2), all 38 committed envelopes decoded with their source-grammar listings regenerated from the decoded bytes, and 38/38 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound. Execution share of the transcripts (representative vectors; every expansion vector's line is in the full transcript):

```
  X9 exec-combo-ner-noprom-fbsuccess-halt: executed — 4/4 records match (fields 1–14); receipt bytes match (NO_SOLUTION (tag 2)); ledger holds: Work(E) = 36 U; conservation 36 = 36 + 0 + 0
  X10 exec-combo-ner-noprom-fbsuccess-tbirth: executed — 13/13 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 68 U; conservation 72 = 68 + 4 + 0
  X13 exec-combo-ner-noprom-fbreject-tbirth: executed — 3/3 records match (fields 1–14); receipt bytes match (DOMAIN_ERROR (tag 4)); ledger holds: Work(E) = 28 U; conservation 36 = 28 + 8 + 0
  X14 exec-combo-ner-prom-fbsuccess-halt: executed — 5/5 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 40 U; conservation 40 = 40 + 0 + 0
  X16 exec-combo-ner-prom-fbremiss-halt: executed — 6/6 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 44 U; conservation 44 = 44 + 0 + 0
  X38 exec-combo-dom-prom-fbreject-tbirth: executed — 4/4 records match (fields 1–14); receipt bytes match (DOMAIN_ERROR (tag 4)); ledger holds: Work(E) = 32 U; conservation 40 = 32 + 8 + 0
suite X execution (reference interpreter): PASS — 38/38 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
suite X execution (second implementation): PASS — 38/38 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

Per-vector execution facts (each against the same committed expected outcomes both implementations matched): X9's pool `STATUS_NEG_EVEN_ROOT` rides the `HALT` exit to `NO_SOLUTION` with the token selected and the miss token consumed by the fallback completion; X10's trip-ceiling rejection births the fatal `EXPRESSION_TOO_COMPLEX` after the fallback completed, the pool token terminalized-unresolved in the itemization (DR1); X13's fallback-typed rejection (the zero divisor) exits `DOMAIN_ERROR` with the fatal token selected (DR2) and the pool token terminalized; X14's promotion consumes the pool token before the fallback and exits `SUCCESS`; X16 realizes the §4.4 residue rule — `SUCCESS` with the residual Tier O miss token outstanding and itemized (the repeated miss leaves the earlier token outstanding, §2.5's F12 rule); X38's promoted `STATUS_DOMAIN_ERROR` pool token takes the `PROMOTE` row's no-token-effect branch and the zero-divisor exit selects the fatal token. The declared initial pool entries birth at step 0 with serials allocated in declared order (P5/P6/§2.6); every terminal settlement's refund closes the escrow (`reserve_outstanding` = 0).

## Independence and scope declarations

- The expansion vectors are derived from the committed text of §2.3/§2.5/§2.6 and §3 plus the corpus's declared artifact-wave parameters (P1–P11) and its declared profile, through the derivation instrument of record; the instrument consults neither the reference interpreter's code nor its outputs, and neither implementation's code was consulted during the derivation. The goldens were fixed before the artifacts ran — the golden-first order applied to expansions.
- Both implementations were repaired against the derived goldens under §4.1's adjudication; neither implementation generated or edited any golden.
- The declared inputs of families E, I, and M are unchanged from the seed increments; every execution input (the envelope bytes, `W_budget`, `D`, the version tuple `V`) is parsed from the committed vector text; the declared initial status pool is committed envelope data (envelope key 8).
- The expansion implements no new opcode execution surface in either artifact: `TUPLE`, `FIELD`, `EMIT`, witness receipts, and a populated dependency store `D` remain the registered expansion obligations they were at v0.3.0-draft.13.

## Reproducibility

Run 3 is the same binary (reference interpreter) and the same source (second implementation) on the same corpus; each transcript is byte-identical to its run-2 counterpart (transcript digests, 16-hex prefixes — reference interpreter `820920ee740d013c`, second implementation `f8bf639ee4e99bd4`). This is reproducibility evidence only (§4.2); conformance rides §4.1's protocol — the committed golden results were derived from the normative rules before either artifact executed against them, and the artifacts' agreement with them is the claim.
