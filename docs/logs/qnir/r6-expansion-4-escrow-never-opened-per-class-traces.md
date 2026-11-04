---
layout: default
title: R6 Expansion Increment 4 — the Per-Class Ledger Trace Publications and the Budget-Boundary Per-Combination Completion (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (76 vectors), with the registered escrow-opening defect class
nav_exclude: true
---

# R6 Expansion Increment 4 — the Per-Class Ledger Trace Publications and the Budget-Boundary Per-Combination Completion (Run Log)

Artifact wave: v0.3.0-draft.17 (R6 expansion increment 4) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (committed at 0.3.0-draft.17; corpus sha-256 prefix `f94d26ae9e3f0827`, 1,034,150 bytes — re-issued at v0.3.0-draft.18 from the pre-version-bump digest `d486156008580aba` per the declared metadata repair registered in the program of record's wave record v0.3.0-draft.18 and in the increment-5 run log; the byte count is unchanged, the version-line edit is same-length, and the vector bytes the runs checked are identical), carrying the seed's 34 independently reviewed vectors and the 41 expansion vectors of increments 1–3 byte-unchanged plus the 1 hand-derived expansion vector X50 and the Per-Class Ledger Trace Register (the Suite Register's fifth bullet — the per-class ledger trace publications beyond the seed's summaries and the per-combination expansion of the budget-boundary family, over the committed rendering P12 with no new declared parameter). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Four steps, in order:

1. the **anchored derivation** — the derivation instrument of record re-derived every committed execution vector byte-exact from its declared source form and declared parameters (all 49, seed and increments 1–3 alike) before any expansion byte was derived — the golden-first order applied to expansions — and, after the instrument repair registered below, re-anchored over the full 49-vector execution family (`ANCHORING: all 49 committed execution vectors re-derived byte-exact from the declared source forms and parameters`), then again over all 50 after the vector was committed;
2. the **first runs against the corrected bytes** — both implementations reproduced the same defect at the same point on X50 (details below): the §4.1 adjudication registered one shared defect class, and both implementations were repaired in the same wave;
3. the **conformance runs of record** — both artifacts against the corrected corpus (transcripts below); no divergence was registered: both implementations reproduced every committed expected outcome on their first recorded runs against the corrected bytes, so §4.1's adjudication rule was never triggered against the committed corpus a second time;
4. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, byte-identical, reproducibility evidence only (§4.2).

## The registered defect class — the reserve's opening placement

X50 is the corpus's first committed vector whose first attempted transition rejects: the declared table value (10) exceeds the `CLOAD`'s declared bound (9) before any transition completes, so the committed settlement is the escrow-never-opened shape — §3.3 opens the execution's reserve once, by the execution's first *completed* transition (§3.4: "on a pass, the transition completes normally and opens the escrow"), so on a rejection at the first attempted transition the gateway never posts, the reserve never opens, and no refund posts anywhere (Work(E) = 0 U). No committed vector exercised this point before: every committed vector's first attempted transition completes, so the opening's placement was unobservable in them all.

- **The derivation instrument.** The instrument opened the reserve at the first *attempted* transition, pre-dispatch, so the escrow-never-opened shape first derived with the reserve taken and refunded in full at the rejection record (32 U reserved, 32 U refunded, record field 10 carrying 32) — a settlement the committed text does not declare. Adjudicated against the committed rules (§3.3's first-completed-transition opening — fixed by the committed text, by every committed vector's settlement trace, and by expansion increment 2's scope disposition, which recorded the reading when it reserved this very shape to Suite Register bullet 5; §3.4's completion-coupled opening; X2's committed zero-posting settlement), the instrument was repaired before any committed byte of this increment existed (the reserve opening moved to the first transition's completion, the fuel-backstop guard unchanged in its pre-dispatch firing condition), the anchor was re-run over all 49 committed execution vectors byte-exact — the repair is unobservable whenever the first attempted transition completes — and the vector was re-derived to the zero-posting settlement. The corpus of record carries only the corrected derivation.
- **Both implementations.** The first runs against the corrected bytes found the same defect in both, at the same point — the implementation-side share of the same class: each admitted X50 (the transcript's admission line: `well-formed; 2 static value ids; B_hat = 32 U (declared bound 32 U); W_budget = 100 U; ADMITTED`), opened the reserve pre-dispatch, and settled the rejection with the whole outstanding escrow refunding into record field 10 (work_refunded 32 U) and the mutated accumulators carried in the post-state root (field 8), against the committed bytes' zero-posting settlement (work_refunded 0, accumulators unchanged). Registered per §4.1's adjudication rule with the disagreeing points named; the committed text governs and the derived bytes stand. Both implementations were repaired in the same wave — the reference interpreter's `src/qnir/src/interp.rs` dispatch loop and the second implementation's `src/qnir-py/qnir_ref.py` run loop, the reserve opening moved to the first completed transition — and the repair is seed-safe: X1–X49 reproduce unchanged, verified against the corpus before the corrected runs (the first-run transcripts show 49/50 with every non-X50 vector matching).

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.17, toolchain pinned in the crate (channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64, Linux; release profile; no network access at run time
- second implementation: CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

## How to reproduce

```
src/qnir/target/release/qnir docs/qnir_goldens.md
python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The artifact share of this wave

The increment's artifact share is the defect repair and the version bump: no execution rule changed except the reserve's opening placement (both implementations, per the adjudication above), and no harness surface changed — the vector rides the committed vector format, the declared rendering P12, and the five agreement points exactly as increments 2–3's vectors do. The assembler's and validator's shares are unchanged (X50's source form re-encodes byte-exactly, its admission is computed by the committed `B_hat` surrogate and passes at the declared budget).

## The conformance runs (both artifacts, first recorded runs, corrected corpus)

Verdict: all 76 committed vectors reproduce at the five agreement points of §4.1 — suite E 11/11 and suite I 3/3 canonical bytes (agreement point 1), suite M 12/12 committed verdicts exact (agreement point 2), all committed envelopes decoded with their source-grammar listings regenerated from the decoded bytes, and 50/50 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound.

Execution share of the transcripts (the increment's vector; every other vector's line is in the full transcript):

```
  X50 exec-escrow-never-opened-rejection: envelope match (353 bytes; stream match: true)
  X50 exec-escrow-never-opened-rejection: well-formed; 2 static value ids; B_hat = 32 U (declared bound 32 U); W_budget = 100 U; ADMITTED (the execution share of this transcript checks the outcome)
suite X (executions): assembly PASS — 49/49 committed source forms re-encode byte-exactly; validation PASS — 50/8 admitted or refused per the declared bounds
  X50 exec-escrow-never-opened-rejection: executed — 1/1 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 0 U; conservation 0 = 0 + 0 + 0
suite X execution (reference interpreter): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
```

```
  X50 exec-escrow-never-opened-rejection: executed — 1/1 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 0 U; conservation 0 = 0 + 0 + 0
suite X execution (second implementation): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
```

Suite verdicts of the runs of record:

```
suite E (value encodings): PASS — 11/11 committed vectors re-encode byte-exactly
suite I (instruction encodings): PASS — 3/3 committed vectors re-encode byte-exactly
suite M (malformed surfaces): PASS — 12/12 committed verdicts reproduced
suite X (executions): assembly PASS — 49/49 committed source forms re-encode byte-exactly; validation PASS — 50/8 admitted or refused per the declared bounds
suite X execution (reference interpreter): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
suite X execution (second implementation): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

The first runs against the corrected bytes (before the repairs; the adjudication's evidence):

```
  FAIL X50 exec-escrow-never-opened-rejection: record 1 field 8 diverges from the committed bytes (post_state_root false)
suite X execution (reference interpreter): FAIL — 49/50 committed expected outcomes reproduced at the five agreement points of §4.1
result: FAIL — 1 failing check(s)
```

```
  X50 exec-escrow-never-opened-rejection: FAILED — record 1 differs (agreement point 3)
suite X execution (second implementation): FAIL — 49/50 committed expected outcomes reproduced at the five agreement points of §4.1
```

The reruns (the same binary, the same source, twice each): byte-identical transcripts — reproducibility evidence only, never independent reproduction and never conformance (§4.2).
