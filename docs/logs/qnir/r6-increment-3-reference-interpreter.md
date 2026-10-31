---
layout: default
title: R6 Increment 3 — The Reference Interpreter (Run Log)
description: Run transcripts of the QnIR reference interpreter over the committed golden corpus
nav_exclude: true
---

# R6 Increment 3 — The Reference Interpreter (Run Log)

Artifact: `src/qnir/` (v0.3.0-draft.12, R6 increment 3) — the reference interpreter of conformance §4.3, run as the artifact's execution share (suite X execution) beside the increment-2 suites. Corpus: `docs/qnir_goldens.md`, the committed golden corpus (§4.4), consumed as repository bytes; the artifact never generates or edits it.

## What ran

Two runs of one release binary over the corpus as committed at v0.3.0-draft.11 (131,122 bytes). The two runs are the same binary executed twice — the §4.2 same-binary rerun, reproducibility evidence only.

## Toolchain and build

Pinned per the registered toolchain record (QO-33: compiler, target triple, flags, dependency lock status):

- rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27)
- target x86_64-unknown-linux-gnu, release profile (`opt-level = 2`), `--offline`
- dependencies: none (standard library only; `Cargo.lock` present, empty of external packages)
- the crate pins the channel in `rust-toolchain.toml` (`1.99.1`)

Build command, from `src/qnir/`: `cargo build --release --offline`. The build is warning-clean.

## How to reproduce

```
cargo build --release --offline
./target/release/qnir <path to docs/qnir_goldens.md>
```

The tool parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The execution checks (both runs)

Verdict: 8/8 execution vectors reproduce their complete committed expected outcome at the five agreement points of §4.1 — the full fourteen-field record sequence in order, the state roots byte-exact (fields 1/7/8, the canonical configuration encodings of P2, and field 2, the P3 program root), the terminal receipt's canonical bytes, and the reconstructed ledger with every §3.3 conservation identity. Value encodings ride suite E (11/11) and the P2 components. Transcript, execution share:

```
  X1 exec-conservation-100-60-40: executed — 10/10 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 60 U; conservation 100 = 60 + 40 + 0
  X2 exec-admission-refusal: executed — 1/1 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 0 U; conservation 0 = 0 + 0 + 0
  X3 exec-halt-no-solution: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (NO_SOLUTION (tag 2)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X4 exec-promote-success: executed — 5/5 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 40 U; conservation 40 = 40 + 0 + 0
  X5 exec-domain-error: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (DOMAIN_ERROR (tag 4)); ledger holds: Work(E) = 28 U; conservation 36 = 28 + 8 + 0
  X6 exec-lookup-miss-fallback-success: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (SUCCESS (tag 1)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
  X7 exec-trip-ceiling-rejection: executed — 10/10 records match (fields 1–14, state roots byte-exact); receipt bytes match (EXPRESSION_TOO_COMPLEX (tag 3)); ledger holds: Work(E) = 56 U; conservation 60 = 56 + 4 + 0
  X8 exec-multi-token-precedence: executed — 3/3 records match (fields 1–14, state roots byte-exact); receipt bytes match (NO_SOLUTION (tag 2)); ledger holds: Work(E) = 32 U; conservation 32 = 32 + 0 + 0
suite X execution (reference interpreter): PASS — 8/8 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

The increment-2 share above it in the same transcript: 34/34 (suites E 11/11, I 3/3, M 12/12, X assembly 7/7 and admission 8/8). Per-vector execution facts: X1 realizes the F13 settlement exactly (10 records, `Work(E)` 60 U, refund 40 U); X2's refusal posts nothing (one rejection record, zero ledger, the escrow never opened); X3 terminalizes the declared token to NO_SOLUTION with the token selected in the receipt; X4's PROMOTE consumes and succeeds; X5's zero divisor births the domain error and the terminal settlement refunds 8 U into the rejection record; X6's lookup miss births the Tier O token, the fallback block completes, the token is consumed and success follows; X7's trip ceiling fires at the second traversal of the loop's back-edge (the rejection record at step 10, refund 4 U); X8's earliest-birth token decides NO_SOLUTION with both declared tokens terminalized.

## The finding and the declared repair (committed in this wave)

The receipt decode surfaced one defect in the increment-2 artifact: the canonical-CBOR decoder's simple-value table (`src/qnir/src/cbor.rs`) was off by one against RFC 8949 — simple value 20 decoded as null and 22 as true, where the RFC assigns false, true, null. The terminal receipt's nil selected-token key is the corpus's only committed simple value, so every other suite was unaffected. The repair is committed in this wave: the table re-issued with the RFC 8949 mapping and the finding noted in the source. The corpus is untouched: the receipt tables' selected-token annotation reads "true" where the committed bytes carry nil (0xf6) — the derivation decoder's same off-by-one rendered Python's `True`; the canonical bytes govern per §4.4, and the annotation is recorded here as an observation, not a byte defect.

## Reproducibility

Run 2 is the same binary on the same corpus; its transcript is byte-identical to run 1's (transcript digest, 12-hex prefix — `34067af6e53e`). This is reproducibility evidence only (§4.2); conformance rides §4.1's protocol — the committed golden results were derived before any code, and this artifact's agreement with them is the claim.

## Scope declarations

- The interpreter executes the committed transition relation of §2.3 under §2.5's lifecycle algebra and §3's accounting, with the corpus preamble's artifact-wave parameters (P1–P11) and the declared profile `qir-prof-golden-1`; it implements no surface beyond the seed's (TUPLE, FIELD, EMIT and witness receipts are expansion obligations).
- The dependency store `D` is the declared "no entries" table for every seed vector; a populated store is an expansion obligation.
- The §2.3 static-binding postulate's full dominance analysis remains an expansion obligation (as declared in the increment-2 log); the interpreter's admission reuses the validator's `B_hat` computation.
- The seed's expansion obligations (the suite register of `docs/qnir_goldens.md`) are untouched by this increment.
