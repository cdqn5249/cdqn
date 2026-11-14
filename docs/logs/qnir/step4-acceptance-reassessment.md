---
layout: default
title: Step-4 Acceptance Reassessment (v0.3.0-draft.24)
description: The reassessment of Repair Steps 2-4 against the committed evidence — Step 4's acceptance condition met by repository bytes, with the Step-5 deferences recorded
nav_exclude: true
---

# Step-4 Acceptance Reassessment (v0.3.0-draft.24)

Acceptance reassessment of Repair Steps 2–4 of the v0.3 repair sequence ([`docs/AiReviews/AiReviewQnIRv0.4.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.4.md) §3, as corrected at v0.1.1) against the committed evidence, under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.24. **Step 4 is ACCEPTED**: the acceptance condition of conformance §4.5 is met by repository bytes, with one Step-5 deference (the collision-bucket comparison, F24's deferral stands). Steps 2 and 3 are accepted by the closures of F25, F26, and F27. No vector, specification rule, or implementation is changed by this wave.

## The BOC selection

The increment is the review's own §5 order item 5 — "Update the corpus register and program of record with explicit PASS/PARTIAL/NOT APPLICABLE dispositions and evidence" — and the sole remaining queued item (AiReview v0.1.6's step table's row 5; v0.1.7's restatement). Under the standing rubric-mode key `LexKey(c) = (ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)))` (π = Efficiency, Modularity, Portability, v0.3.0-draft.7's standing permutation): `ImplicitDeps` = 0 — the reassessment consumes already-committed evidence (the 84 committed vectors' runs, the F20 traceability register, the F27 fixture) and applies the acceptance condition already declared in §4.5; `ΔW_worst` = one review record, one wave record, and status paragraphs (no rule, vector, or artifact byte); `M_coupling` = none added; `P_coverage` = closes the Step-4 disposition and frees Step 5, which the review's §5 orders to begin only after this acceptance.

## The acceptance condition

Conformance §4.5 closes the acceptance on five deliverables, each with its discharge site declared: the golden-first differential protocol with its independence-evidence requirement (§4.1); the normative assembler and validator requirements, including the error direction of the `B(P)` surrogate (§4.2); the reference interpreter and second implementation requirements, including the state-root artifact requirement of §2.6 (§4.3); the vector suite — format, the five adversarial families, and the per-class ledger traces (§4.4); and the re-encoding of all examples against the frozen core only (§4.2, §4.4). The acceptance test: "The artifact share is discharged when each named artifact exists as a repository increment that passes the golden-first differential protocol over the full suite (§4.1) and carries its declared version bindings and independence evidence."

## Deliverable dispositions (explicit PASS/PARTIAL/NOT APPLICABLE)

| Deliverable (§4.5) | Disposition | Evidence and citations |
|---|---|---|
| Golden-first differential protocol with independence evidence (§4.1) | **PASS** | The corpus was committed before any implementation (v0.3.0-draft.10; M4 bytes repaired at v0.3.0-draft.11 under §4.1's adjudication); both artifacts are implemented only after the corpus, checked against it, and never generate it; the reference artifact and the second implementation are independent standard-library implementations (Rust / CPython, `QO-33`/`QO-35`), and each passes every agreement point on its own run — mutual agreement is never the criterion |
| Normative assembler and validator, including the `B(P)` surrogate's error direction (§4.2) | **PASS** | Suite E 11/11 and suite I 3/3: every declared source value and instruction assembles to its committed canonical bytes; suite M 15/15: every committed malformed surface yields its declared typed verdict; suite X's assembly share 51/51: every committed source program re-encodes to its committed envelope and every source listing regenerates; suite X's admission lines 51/51: every declared bound matches the computed `B_hat`, and refusal/admission cells (X1, X2, X7) exercise the rule end to end — with `B_hat(P) = B(P)` exact since F26 (v0.3.0-draft.22) |
| Reference interpreter and second implementation, including the state-root artifact requirement (§4.3) | **PASS** | Suite X execution 51/51 at all five agreement points in both artifacts: (1) canonical encoding bytes, (2) committed verdicts, (3) the complete ordered record sequence with fields 1–14 and byte-exact state roots, (4) terminal receipt bytes, (5) the reconstructed ledger and terminal outcome; the state-root algorithm and width ride the declared encoding version of the corpus (§2.6) |
| Vector suite — format, the five adversarial families, the per-class ledger traces (§4.4) | **PASS**, one Step-5 deference | The 84-vector suite's family dispositions are below; the traceability register (§4.4, F20 discharge, v0.3.0-draft.21) maps every declared target to exactly one disposition class with repository-byte citations; the Per-Class Ledger Trace Register (v0.3.0-draft.17) carries one trace per declared charge class, each citing a committed vector's ledger block |
| Re-encoding of all examples against the frozen core only (§4.2, §4.4) | **PASS** | Every committed example is re-encoded from its declared source form to its committed canonical bytes by both artifacts (suite E/I/X assembly shares); no extension surface appears in any encoding, and the closed inventory of §1.2 bounds the corpus |

## The adversarial families, explicit dispositions

| Family (register row, §4.4) | Disposition | Evidence |
|---|---|---|
| Loops — bounded paths completing under the trip ceiling | **PASS** | X15 and the back-edge-carrying cells of X9–X38; each admission line declares the back-edge at the declared ceiling 1 and its maximum bounded path completes |
| Loops — at and over the trip ceiling (BR/JMP site) | **PASS** | X7, whose second traversal is rejected under §2.3's BR/JMP rejections with the §3.3 terminal settlement |
| Loops — over the trip ceiling (`CLOAD` fallback site) | **PASS** | X51 (F25's regression vector); `B_hat(P) = 32` counts the fallback back-edge exactly once |
| Loops — nesting at and over depth ceilings | **NOT APPLICABLE** | The depth ceiling is retired at v0.3.0-draft.22 (F26) — the obligation's target no longer exists; trip ceilings alone certify finiteness |
| Calls — the extension boundary's closure | **NOT APPLICABLE** (closed by declaration) | Calls are Step-6 extension work (§1.3's deferral register); the closed instruction table and total source grammar make a call construct's bytes an unknown opcode, whose verdict is fixed by M1 |
| Collisions — full-canonical-encoding equality, in-core surface | **PASS** | X45: byte-equal canonical tuple values under distinct names, the declared full-canonical-encoding equality, no overwrite |
| Collisions — the F6 collision-safe artifact comparison (header-equal, payload-different) | **DEFERRED (Step 5)** | The bucket's resolution is Step 5's (§1.4's F6 row, F24's deferral stands); the register fixes the test obligation, not the answer — the one row this reassessment does not close |
| Unresolved statuses — success-after-recovery and the declared residue rule | **PASS** | X6 and X16 |
| Unresolved statuses — unresolved debt at every exit; multiple outstanding tokens | **PASS** | X3; the itemization cells of X9–X38; X8 (earliest-birth precedence, F15) |
| Unresolved statuses — the fatal-terminal vectors of F7 and the family-9 row | **PASS** | The F12 combination closure X9–X38 (30 vectors), the 6 declared-unreachable cells, and the 18 fuel-backstop cells discharged by the F27 fixture |
| Encoding vectors — canonical and malformed | **PASS** | E1–E9, I1–I3, R1–R4, M1–M15 |
| Budget boundaries — fuel arithmetic, settlement shapes, the backstop's declared role | **PASS** | Every execution vector's ledger block (X1, X2, X5, X7, X41, X50, X51), and the F27 fuel-backstop fixture (v0.3.0-draft.23) discharging the last half of the row |
| Conservation vectors — the escrow settlement identities | **PASS** | X1's worked settlement and the refund-release identity lines across the suite |
| Per-class ledger traces — one trace per declared rule class | **PASS** | The Per-Class Ledger Trace Register (v0.3.0-draft.17), each citing a committed vector's ledger block |

## The revised milestone assessment

The AiReview v0.1.1 table is superseded as follows (the original review body and the v0.1.1 note stay unchanged; this record is the disposition):

| QnIR repair step | v0.1.1 status | Reassessed status | Basis |
|---|---|---|---|
| Step 1 — Minimal core | Retain pass, subject to existing promotion conditions | **Retain pass** (unchanged) | The frozen core of §1 is untouched by every closure wave; the promotion conditions are unchanged |
| Step 2 — Operational semantics | Partial; reopen for F25 and F26 | **ACCEPTED** | F25 discharged at v0.3.0-draft.19 (the `CLOAD` fallback edge as a counted trip-ceiling edge, both artifacts enforcing the check before the miss token is born); F26 discharged at v0.3.0-draft.22 (the depth ceiling retired; the surrogate exact, `B_hat(P) = B(P)`); the transition relation, terminalization, and encodings are unchanged otherwise and fully exercised by the suite |
| Step 3 — Work accounting and admission | Reopen; bounded execution is not yet demonstrated | **ACCEPTED** | Admission is exact (the §3.4 surrogate is `B(P)` itself, F26) with the refusal, exact, and slack cells committed (X1, X2, X7); the conservation identities ride every execution vector and the worked settlement (X1); the backstop's containment is demonstrated by the F27 fixture at v0.3.0-draft.23 — an admitted Intent under an injected under-reserve is stopped, contained, and settled with `charged ≤ reserved` and the closed conservation identity, while the control run shows the exact refusal; no double billing anywhere (`charged ≤ reserved` on every vector) |
| Step 4 — Conformance oracle and corpus | Partial; F20 traceability and F25–F27 remain outstanding | **ACCEPTED** | This record: the five deliverables above are discharged by repository bytes; F20's register maps every target; F25–F27 are closed; the suite passes 84/84 in both artifacts plus the F27 fixture; the one remaining row is the Step-5 collision-bucket comparison |
| Step 5 — Strengthen commitments | Deferred | **Deferred** (next) | The review's §5 orders Step 5 to begin only after Step-4 acceptance, "beginning with the minimum lineage and identity semantics needed by the commitment design"; that is the next BOC increment |
| Step 6 — Extensions with evidence | Deferred | **Deferred** | Unchanged |

## What acceptance does not claim

Acceptance of Steps 2–4 is not promotion. The promotion criterion is unchanged and unmet: Step 5's commitments — full-transition trace roots, collision-safe artifact comparison, normative artifact lineage, and versioned verbalizer/extractor identities — do not exist yet, and the draft remains a Working Draft with QnIR **[PENDING]** in the portal milestone. `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, and no claim is styled a theorem. The collision bucket's header-equal/payload-different comparison remains a Step-5 obligation, and the call construct remains Step-6 extension work; neither is quietly absorbed by this acceptance.

## Runs of record

The wave changes documentation only — no corpus byte, no artifact byte, no specification rule. The runs of record are therefore the v0.3.0-draft.23 runs, repeated unchanged at this record's commit state: both artifacts pass **84/84** (suite E 11/11, I 3/3, R 4/4, M 15/15, X 51/51 at §4.1's five agreement points), and both pass the `--fixture F27` mode with byte-identical fixture blocks. Transcript digests: reference `6135b1c60b7d69b0`, second implementation `27ce440c2ee5e620` (normal mode); `ba5d4e2c3decd018` and `9fc02e0a9843a596` (fixture mode); the fixture block `ed67e6cbd242c69c`. The corpus digest of record is `fbefaddb17903c18` over 1,066,409 LF/repository bytes, unchanged from v0.3.0-draft.23.

## The next BOC selection

With Step 4 accepted, the review's §5 order makes the next increment Step 5, "beginning with the minimum lineage and identity semantics needed by the commitment design" — the declaration of the full-transition trace-root format, the collision-safe artifact comparison, the normative lineage binding, and the versioned verbalizer/extractor identities, each as a committed specification increment over the accepted oracle. No implementation of Step 5 is started by this record, and no vector is added, deleted, or edited.
