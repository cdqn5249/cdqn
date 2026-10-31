---
layout: default
title: R6 Increment 4 — The Second Implementation (Run Log)
description: Run transcripts of the QnIR second implementation over the committed golden corpus
nav_exclude: true
---

# R6 Increment 4 — The Second Implementation (Run Log)

Artifact: `src/qnir-py/qnir_ref.py` (v0.3.0-draft.13, R6 increment 4) — the second implementation of conformance §4.3, standard-library Python per `QO-35`'s registered verdict. Corpus: `docs/qnir_goldens.md`, the committed golden corpus (§4.4), consumed as repository bytes; the artifact never generates or edits it.

## What ran

Two runs of one interpreter over the corpus as committed at v0.3.0-draft.11 (131,122 bytes). The two runs are the same interpreter and source executed twice on the same input — the same-source rerun, reproducibility evidence only.

## Toolchain

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`)
- no build step, no dependencies, no network access at run time
- the language verdict is `QO-35`'s registered adjudication (standard-library Python for the second implementation; wasm/wasi a later portability target)

## How to reproduce

```
python3 src/qnir-py/qnir_ref.py <path to docs/qnir_goldens.md>
```

The artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The conformance checks (both runs)

Verdict: all 34 committed vectors reproduce at the five agreement points of §4.1 — family E 11/11 and family I 3/3 canonical bytes (agreement point 1), family M 12/12 committed verdicts exact (agreement point 2), all 7 committed envelopes decoded with their source-grammar listings regenerated from the decoded bytes, and 8/8 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). `B_hat` is this implementation's own instantiation of §3.4's declared surrogate; it equals every committed declared bound. Execution share of the transcript:

```
  X1 exec-conservation-100-60-40: executed — 10/10 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 60 U; conservation 100 = 60 + 40 + 0
  X2 exec-admission-refusal: executed — 1/1 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 0 U; conservation 0 = 0 + 0 + 0
  X3 exec-halt-no-solution: executed — 3/3 records match (fields 1–14); receipt bytes match (NO_SOLUTION (tag 2)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X4 exec-promote-success: executed — 5/5 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 40 U; conservation 40 = 40 + 0 + 0
  X5 exec-domain-error: executed — 3/3 records match (fields 1–14); receipt bytes match (DOMAIN_ERROR (tag 4)); ledger holds: Work(E) = 28 U; conservation 36 = 28 + 8 + 0
  X6 exec-lookup-miss-fallback-success: executed — 3/3 records match (fields 1–14); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X7 exec-trip-ceiling-rejection: executed — 10/10 records match (fields 1–14); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 56 U; conservation 60 = 56 + 4 + 0
  X8 exec-multi-token-precedence: executed — 3/3 records match (fields 1–14); receipt bytes match (NO_SOLUTION (tag 2)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
suite X execution (second implementation): PASS — 8/8 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

Per-vector execution facts (each against the same committed expected outcomes the reference interpreter matched at v0.3.0-draft.12): X1 realizes the F13 settlement exactly (10 records, `Work(E)` 60 U, refund 40 U); X2's refusal posts nothing (one rejection record, zero ledger, the escrow never opens); X3 terminalizes the declared token to NO_SOLUTION with the token selected in the receipt; X4's PROMOTE consumes and succeeds; X5's zero divisor births the domain error and the terminal settlement refunds 8 U into the rejection record; X6's lookup miss births the Tier O token, the fallback block completes, the token is consumed and success follows; X7's trip ceiling fires at the second traversal of the loop's back-edge (the rejection record at step 10, refund 4 U); X8's earliest-birth token decides NO_SOLUTION with both declared tokens terminalized.

## Independence and scope declarations

- The implementation is written from the committed text of §1–§4 plus the corpus's declared artifact-wave parameters (P1–P11) and its declared profile; it consults neither the reference interpreter's code nor its outputs (§4.1/§4.3), and its conformance is decided against the committed golden results at the five agreement points, nowhere else.
- The canonical-CBOR decoder carries the RFC 8949 simple-value mapping as repaired at v0.3.0-draft.12 (20 false, 21 true, 22 null); the receipt tables' selected-token annotation ("true" where the committed bytes carry nil) remains the recorded derivation-decoder observation, the committed canonical bytes governing per §4.4.
- The declared inputs of families E and I are the corpus's committed vector parameters; family M's inputs are the committed bytes themselves, classified under the committed verdict vocabulary; every execution input (the envelope bytes, `W_budget`, `D`, the version tuple `V`) is parsed from the committed vector text.
- The implementation implements no surface beyond the seed's (TUPLE, FIELD, EMIT and witness receipts are expansion obligations); the dependency store `D` is the declared "no entries" table for every seed vector; a populated store is an expansion obligation.
- The seed's expansion obligations (the suite register of `docs/qnir_goldens.md`) are untouched by this increment.

## Reproducibility

Run 2 is the same interpreter and source on the same corpus; its transcript is byte-identical to run 1's (transcript digest, 12-hex prefix — `65835f90263f`). This is reproducibility evidence only (§4.2); conformance rides §4.1's protocol — the committed golden results were derived before any code, and this artifact's agreement with them is the claim.
