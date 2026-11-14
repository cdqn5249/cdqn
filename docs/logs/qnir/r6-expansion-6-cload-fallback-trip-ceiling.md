---
layout: default
title: R6 Expansion Increment 6 — the `CLOAD` Fallback Trip-Ceiling Regression Vector (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (84 vectors); one admission-DP defect class registered and repaired in-wave
nav_exclude: true
---

# R6 Expansion Increment 6 — the `CLOAD` Fallback Trip-Ceiling Regression Vector (Run Log)

Artifact wave: v0.3.0-draft.20 (R6 expansion increment 6) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (committed at 0.3.0-draft.20; corpus sha-256 prefix `32028d2c76681893`, 1,059,386 repository bytes), carrying the seed's 34 independently reviewed vectors and the 48 expansion vectors of increments 1–5 byte-unchanged plus the 1 hand-derived execution vector of this increment — X51, the `CLOAD` fallback trip-ceiling regression vector (the Suite Register bullet registered by Finding F25's resolution at v0.3.0-draft.19). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Four steps, in order:

1. the **derivation of record** — unlike increments 1–5, no third derivation instrument was used: X51 was derived by hand from the amended normative text (§2.3's amended `CLOAD` row and derived-count paragraph, §2.5's amended Tier O rule, §3.3/§3.4) after the four-artifact order and expansion increments 1–5, before either artifact executed the vector — the golden-first order applied to expansions (the corpus's increment-6 section declares this provenance);
2. the **defect run** — the first recorded Rust execution of the vector, on the admission DP as committed at v0.3.0-draft.19, did not reproduce the vector: the `B_hat` DP recursed without bound on the self-looping fallback (stack overflow, `STATUS_STACK_OVERFLOW`). The defect class and repair are recorded below;
3. the **conformance runs of record** — both artifacts against the expanded corpus after the repair (transcripts below); no divergence was registered on the first recorded post-repair runs of either artifact: both implementations reproduced every committed expected outcome, so §4.1's adjudication rule was never triggered;
4. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, byte-identical, reproducibility evidence only (§4.2).

## The defect class registered in this wave — the Rust `B_hat` miss-edge traversal

Finding F25's ceiling check (the interpreter-side enforcement) was committed at v0.3.0-draft.19 in both artifacts and was correct. Verifying X51 exposed a second, distinct defect in the Rust artifact's admission surrogate (`src/qnir/src/bhat.rs`): the DP's two `CLOAD`-miss call sites recursed into the fallback target **without traversing the miss edge** — without calling the same `step_edge` treatment every other edge traversal uses — and `val` carried no pre-recursion cycle guard, so on a self-looping fallback edge the DP admitted the graph and then recursed without bound (observed as a stack overflow, exit `0xc00000fd`) instead of returning the declared bound. The Python artifact's `b_hat` already applied the correct treatment (`edge()` decrements the site's remaining budget; the memo pre-inserts the cycle guard), which is why the two artifacts could not both reproduce the vector before the repair.

The repair routes both miss-edge call sites (`val`'s `Term::Cload` arm and `in_block_continue`'s mid-block arm) through `step_edge`, and restores the Python artifact's cycle guard as a pre-insert in `val`. No committed vector's `B_hat` changed: over the 83 vectors committed before X51 the two artifacts' bounds already agreed, and after the repair all 84 agree — the repair is seed-safe and the 51 committed bounds match their declared bounds exactly (agreement point of the admission share, §3.4/§4.1).

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.20, toolchain pinned in the crate (`rust-toolchain.toml`, channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64-pc-windows-msvc, MSVC linker (Visual Studio Build Tools 2022, MSVC 14.44.35207), Windows; release profile; no network access at run time
- second implementation: CPython 3.12.10, AMD64, Windows; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

Both toolchains were freshly executed for this wave (the first runs of either artifact on a Windows host in this program's records are the defect run and the runs of record above).

## How to reproduce

```
src/qnir/target/release/qnir.exe docs/qnir_goldens.md
python src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The artifact share of this wave

- the **admission-DP repair** — `src/qnir/src/bhat.rs`: both `CLOAD` miss-edge call sites traverse the edge through `step_edge` (the site's remaining trip budget is consumed when the edge is a declared back edge; the traversal is refused when the budget is exhausted) and `val` pre-inserts the cycle guard (a state re-entered while undecided lies on a closed loop, which is not a ceiling-bounded path) — the Python artifact's already-committed treatment;
- the **version bumps** — the crate at 0.3.0-draft.20 (`Cargo.toml`, `Cargo.lock`); the corpus, the conformance companion, the semantics companion, and the program of record at their incremented versions (the corpus and the program of record at 0.3.0-draft.20, the conformance companion at 0.3.0-draft.20, the semantics companion at 0.3.0-draft.8.2 for the F25 amendment it already carries).

## The conformance runs (both artifacts, first recorded post-repair runs, expanded corpus)

Verdict: all 84 committed vectors reproduce — suite E 11/11, suite I 3/3, and suite R 4/4 canonical bytes (agreement point 1), suite M 15/15 committed verdicts exact (agreement point 2), all 47 committed envelope byte blocks decoded with their source-grammar listings regenerated from the decoded bytes, and 51/51 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound.

This increment's vector (every other vector's line is in the full transcript):

```
  X51 exec-cload-fallback-trip-ceiling-self-loop: envelope match (184 bytes; stream match: true)
  X51 exec-cload-fallback-trip-ceiling-self-loop: well-formed; 1 static value ids; B_hat = 32 U (declared bound 32 U); W_budget = 100 U; ADMITTED (the execution share of this transcript checks the outcome)
  X51 exec-cload-fallback-trip-ceiling-self-loop: executed — 2/2 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 32 = 24 + 8 + 0
suite X execution (reference interpreter): PASS — 51/51 committed expected outcomes reproduced at the five agreement points of §4.1
```

```
  X51 exec-cload-fallback-trip-ceiling-self-loop: executed — 2/2 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 24 U; conservation 32 = 24 + 8 + 0
suite X execution (second implementation): PASS — 51/51 committed expected outcomes reproduced at the five agreement points of §4.1
```

Suite verdicts of the runs of record:

```
suite E (value encodings): PASS — 11/11 committed vectors re-encode byte-exactly
suite I (instruction encodings): PASS — 3/3 committed vectors re-encode byte-exactly
suite R (EXT side-table record encodings): PASS — 4/4 committed vectors re-encode byte-exactly
suite M (malformed surfaces): PASS — 15/15 committed verdicts reproduced
suite X (executions): assembly PASS — 50/50 committed source forms re-encode byte-exactly; validation PASS — 51/8 admitted or refused per the declared bounds
suite X execution (reference interpreter): PASS — 51/51 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
family E (value encodings): PASS — 11/11 canonical bytes reproduced (agreement point 1)
family I (instruction encodings): PASS — 3/3 canonical bytes reproduced (agreement point 1)
family R (EXT side-table record encodings): PASS — 4/4 canonical bytes reproduced (agreement point 1)
family M (malformed surfaces): PASS — 15/15 committed verdicts reproduced (agreement point 2)
program envelopes: PASS — 47/47 committed envelope byte blocks decode; 50/50 committed source listings regenerate (agreement point 1)
suite X execution (second implementation): PASS — 51/51 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

Transcript digests (16-hex prefixes) of the runs of record: reference interpreter `d4b9156d712b8d2a`, second implementation `30c5aacf90f1e2d0`.

The reruns (the same binary, the same source, twice each): byte-identical transcripts — reproducibility evidence only, never independent reproduction and never conformance (§4.2).

## Scope boundaries

No vector is deleted or edited; the 83 vectors committed before this increment are byte-unchanged. No execution rule changed in either artifact. F26 (the depth ceiling) and F27 (the fuel-backstop fixture) remain separate registered obligations. Step 5 is not started.
