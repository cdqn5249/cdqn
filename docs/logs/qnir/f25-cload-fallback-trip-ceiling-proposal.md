---
layout: default
title: F25 — CLOAD Fallback Edge and the Trip Ceiling — Draft Amendment Proposal
description: Non-normative proposal for the author's adjudication; draft wording to close finding F25 before any golden is derived or implementation changed
version: 0.1.0
updated: 2026-11-08
author: "Christophe Duy Quang Nguyen via Claude Sonnet 5.5 (Anthropic)"
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/logs/qnir/f25-cload-fallback-trip-ceiling-proposal.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /logs/qnir/f25-cload-fallback-trip-ceiling-proposal.html
status: Proposal for adjudication; not a canon specification; no vector, implementation, or normative companion changed
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
| DR-a | Where does the rejection occur relative to the lookup? A miss cannot be known before the lookup runs | After the lookup resolves to a miss, before the miss token is born. The address still appears in the transition's input/output ids (field 6), as it does for the existing hit-bound rejection |
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
