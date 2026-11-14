---
layout: default
title: F25 — CLOAD Fallback Edge and the Trip Ceiling — Draft Amendment Proposal
description: Non-normative proposal for the author's adjudication; draft wording to close finding F25 before any golden is derived or implementation changed
version: 0.3.0
updated: 2026-11-14
author: "Christophe Duy Quang Nguyen via Gemini 4 Hydrogen and Antigravity"
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/logs/qnir/f25-cload-fallback-trip-ceiling-proposal.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /logs/qnir/f25-cload-fallback-trip-ceiling-proposal.html
status: Adjudicated and integrated; normative semantics and interpreters updated per Reading R1 at v0.3.0-draft.19; regression vector X51 derived and committed at v0.3.0-draft.20
---

# F25 — CLOAD Fallback Edge and the Trip Ceiling: Draft Amendment Proposal

| Field | Value |
|---|---|
| Basis | `docs/AiReviews/AiReviewQnIRv0.4.md` v0.1.2, §2 (executed reproduction on the Python reference) |
| Snapshot | `qnir_semantics.md` v0.3.0-draft.8.1; `qnir_accounting.md` v0.3.0-draft.8.1; `qnir.md` and `qnir_conformance.md` v0.3.0-draft.18; `qnir_goldens.md` v0.3.0-draft.19 |
| Decision needed from | The repository author (adjudication authority) |
| Not done here | No golden bytes derived, no implementation edited, no Rust run |

## 1. What is being proposed

A `CLOAD` miss traverses its declared fallback edge (edge slot 0 of its own edge identity, semantics §2.6). The reference interpreters do not apply the declared trip ceiling to that traversal, although admission (`B_hat`) counts the edge. On a self-looping fallback with trip ceiling 1, the Python reference completed six miss transitions and stopped only at the fuel backstop (review v0.1.2 §2). This proposal makes the ceiling apply to the miss edge in the specification, so the implementations and the admission bound agree.

## 2. Reading to adjudicate

| Reading | Statement | Consequence |
|---|---|---|
| **R1 (recommended)** | The `CLOAD` miss edge is a counted edge subject to the trip ceiling | Matches edge identity (slot 0), `B_hat`, and accounting §3.4 ("the same declared boundedness the core enforces at runtime"). Makes the fuel backstop unfirable on sound admission, as §3.4 states |
| R2 | The miss edge is not ceiling-counted | `B_hat` must then stop counting it, or fallback back-edges must be rejected at Intent formation. Otherwise unbounded fallback loops are bounded only by the backstop, which contradicts the backstop's declared role |

The rest of this proposal assumes R1.

## 3. Draft wording (old to new)

Locations are given by section and row, with snapshot line numbers for orientation only.

**A. `qnir_semantics.md` §2.3, `CLOAD` row, rejection column (line 94).** Current: "hit value exceeds its declared bound: typed rejection before dispatch". Proposed addition after it:

> ; a miss whose fallback edge is a back-edge with an exhausted declared trip ceiling: typed rejection (§2.5), before the miss token is born

**B. `qnir_semantics.md` §2.3, "Derived counters" paragraph.** Current: "The ceiling preconditions of `BR` and `JMP` compare these derived counts against the declared ceilings". Proposed:

> The ceiling preconditions of `BR`, `JMP`, and the `CLOAD` miss edge compare these derived counts against the declared ceilings. A `CLOAD` hit traverses no edge and increments no count; a `CLOAD` miss that completes increments the count of its fallback-edge site (block id, instruction index, slot 0).

**C. `qnir_semantics.md` §2.5, Tier O paragraph.** After "a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site", add:

> ; each such repeated miss is a further traversal of the fallback edge and is subject to the same declared trip ceiling, and the traversal that would exceed it is rejected under §2.3 before its token is born

**D. `qnir_accounting.md` §3.4.** No wording change is proposed: the sentence on bounded paths already holds under R1. After adoption, re-check that "paths that respect the declared trip ceilings on back-edges" is read to include the `CLOAD` miss edge, as `B_hat` already does.

**E. `qnir_goldens.md` Suite Register.** Add one registered obligation: a regression vector for a self-looping `CLOAD` miss under a trip ceiling of one (see §5).

## 4. Declared readings the author must fix

| Id | Question | Proposed reading |
|---|---|---|
| DR-a | Where does the rejection occur relative to the lookup? A miss cannot be known before the lookup runs | After the lookup resolves to a miss, before the miss token is born. The rejected transition's input/output ids (field 6) are the empty list: a rejection posts nothing and binds nothing, and the committed X41 bytes are decisive (corrected at the X51 derivation — see §9) |
| DR-b | Comparison form | Reject when prior traversals + 1 exceeds the ceiling, the same form `BR` and `JMP` use in the Python reference's `check_ceiling` |
| DR-c | Ledger and receipt consequences of the new rejection | Exactly those of the existing "typed rejection before dispatch" rule. No new charging rule is proposed. To be confirmed when the golden is derived |

## 5. Regression vector requirement (not derived here)

- A minimal program with **no unreachable block** (the X42-derived variant used for the reproduction keeps an unreachable block 1, whose edge the reference classifies as a back-edge; this is an unrelated observation O1 in the review and should not enter the golden).
- Empty `D`, trip ceiling 1, a budget that passes admission.
- Expected shape under R1: transition 1 is a completed miss (site count 0 to 1, one Tier O token born); transition 2 is a rejection with `EXPRESSION_TOO_COMPLEX` before a second token is born.
- The exact bytes, receipt, and ledger **must be derived independently from the amended rules before either implementation is run against them**. The reproduction script `f25_repro.py` observes behavior and must not be used to produce expected values. All 83 existing vectors stay byte-unchanged.

## 6. Handoff for an agent with Rust and network access

1. Obtain the author's adjudication of R1 or R2 and of DR-a to DR-c.
2. Apply §3 to the normative companions as a new draft increment (version bump, wave record in `qnir.md`).
3. Derive the §5 golden with the independent derivation instrument of record, then add it as a new corpus version.
4. Fix the Python and Rust interpreters; run both on the pinned toolchain (`rust-toolchain.toml`, channel 1.99.1) and record separately which were freshly executed.
5. Confirm that the Rust `Transfer::Miss` path reproduces the six-miss trace on the unamended code before the fix (static reading only so far), then confirm the fix.
6. Keep F26 (depth) and F27 (fuel-backstop fixture) as separate increments.

## 7. Out of scope

This proposal does not decide F26, does not specify the F27 fixture, does not start Step 5, and does not change `docs/boc.md` policy.

---

## 8. Adjudication Record (v0.2.0 — 2026-11-13)

- **Authority Verdict:** Reading **R1** is adopted as the canonical reading. The `CLOAD` miss edge is a counted edge subject to the declared trip ceiling.
- **Declared Readings:** DR-a, DR-b, and DR-c are adopted verbatim:
  - DR-a: Rejection occurs after lookup resolves to a miss, before the Tier O token is born. (The field-6 parenthetical of §4 is struck; see §9.)
  - DR-b: Comparison form is `prior_traversals + 1 > trip_ceiling`.
  - DR-c: Standard typed rejection before dispatch (escrow residue refunded; zero work charged).
- **Normative Diffs Applied:** `docs/qnir_semantics.md` §2.3 (`CLOAD` row, "Derived counters" paragraph) and §2.5 (Tier O repeated miss traversals).
- **Interpreter Implementations:** Updated in `src/qnir-py/qnir_ref.py` and `src/qnir/src/interp.rs`.
- **Program of Record:** Registered as Wave record v0.3.0-draft.19 in `docs/qnir.md`.

---

## 9. Derivation Addendum (v0.3.0 — 2026-11-14)

This addendum records the regression vector's derivation and the two corrections verification forced. It does not change Reading R1 or DR-b/DR-c.

- **X51 derived and committed.** The Suite Register vector is committed at v0.3.0-draft.20 in [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) as `exec-cload-fallback-trip-ceiling-self-loop`: `block 0` = `CLOAD addr=0 fallback=block 0` then `HALT`, no unreachable block, empty `D`, trip ceiling 1, `W_budget` = 100 U. Expected: two records — the first miss completes (one Tier O `STATUS_LOOKUP_MISS` born, site count 0 → 1, 24 U charged); the second traversal is rejected before a second token is born (`EXPRESSION_TOO_COMPLEX`, nil selected, 8 U refunded at the rejection record), with the outstanding miss token terminalized there. `B_hat(P) = 32`. The 83 vectors committed before it are byte-unchanged.
- **DR-a field 6 corrected.** The §4 parenthetical was wrong: a typed rejection posts nothing and binds nothing, so the rejected transition's field 6 is `[]` — the committed X41 bytes (record 2) already say so. DR4 in the corpus declares the correct reading; the address rides only the completed miss transition's field 6.
- **Rust `B_hat` defect registered and repaired.** The ceiling check of v0.3.0-draft.19 was correct in `interp.rs`, but the Rust admission DP (`src/qnir/src/bhat.rs`) traversed the `CLOAD` miss edge without consuming the fallback site's trip budget and carried no cycle guard, so it admitted X51's graph and then recursed without bound (stack overflow). The repair applies the Python artifact's existing treatment: both miss-edge call sites step through `step_edge` and `val` pre-inserts the cycle guard. Both artifacts then reproduce X51 byte-exactly, including the receipt and both state roots.
- **Derivation of record.** X51 was derived by hand from the amended rules before either artifact executed it; no third derivation instrument was used for this vector (increments 1–5's instrument is not re-used), and the two artifacts are independent checks of the fixed bytes, not their source. This respects the F23 distinction between corpus reproduction and independent derivation: the vector's provenance is the maintainer's derivation plus two implementations' agreement, and the corpus declares it.
- **Run evidence.** [`docs/logs/qnir/r6-expansion-6-cload-fallback-trip-ceiling.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/r6-expansion-6-cload-fallback-trip-ceiling.md) records both artifacts over the 84-vector corpus (suite X 51/51 each), the toolchains, and the corpus digest `32028d2c76681893` over 1,059,386 repository bytes.