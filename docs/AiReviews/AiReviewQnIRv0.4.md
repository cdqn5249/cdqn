---
layout: default
title: QnIR v0.3 — Independent Follow-up Review
description: Independent audit of the v0.3.0-draft.18 specification, corpus, implementations, and artifact lineage
version: 0.1.6
updated: 2026-11-14
author: "Christophe Duy Quang Nguyen via GPT-6 (OpenAI) and Claude Sonnet 5.5 (Anthropic)"
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/AiReviews/AiReviewQnIRv0.4.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /AiReviews/AiReviewQnIRv0.4.html
status: Independent AI-assisted review; not a canon specification
---

# QnIR v0.3 — Independent Follow-up Review

**Repository audit, reproduction record, remaining conformance obligations, and BOC disposition**

| Field | Value |
|---|---|
| Reviewed repository snapshot | Refreshed `cdqn-main.zip`, 80 files |
| Reviewed program of record | `docs/qnir.md`, v0.3.0-draft.18 |
| Reviewed semantic companion | `docs/qnir_semantics.md`, v0.3.0-draft.8.1 |
| Reviewed accounting companion | `docs/qnir_accounting.md`, v0.3.0-draft.8.1 |
| Reviewed conformance companion | `docs/qnir_conformance.md`, v0.3.0-draft.18 |
| Reviewed corpus | `docs/qnir_goldens.md`, v0.3.0-draft.18 |
| Reviewed implementations | `src/qnir/` and `src/qnir-py/` |
| Reviewer | GPT-6 (OpenAI), AI-assisted adversarial review |
| Date | 7 November 2026 |
| Overall verdict | **Partial pass; Step 4 artifact share remains incomplete; do not promote QnIR v0.3 or begin Step 5 yet** |

---

## 1. Scope, method, and limits

This review examines the refreshed repository snapshot rather than relying on the previous review's verdict. It reads the current QnIR program of record, all three topical companions, the 83-vector corpus, the two implementation trees, the latest run log, the navigation registry, the BOC document, and the prior Loop-3 review.

The following checks were performed directly against the snapshot:

1. Confirmed the corpus is 1,045,499 bytes and has SHA-256 `b70e4bac391d44378dcc37542504b56c4956dd95f402013cab3a8c97bff39a77`, matching the prefix and size recorded by the latest run log.
2. Executed `python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md` using the available CPython 3.13.5 runtime. It passed all 83 vectors: E 11/11, I 3/3, R 4/4, M 15/15, all 46 envelope blocks and 49 source listings, and X 50/50 at the five declared agreement points.
3. Parsed `docs/_data/navigation.yml` as YAML and checked for duplicate slugs. The registry parsed with 28 entries and no duplicate slugs. It contained no BOC entry.
4. Inspected the current corpus register and searched the corpus for the outstanding conformance surfaces named by the specification.
5. Compared the current program-of-record statements against the companion acceptance conditions and the latest artifact log.

**Limits.** Rust could not be rebuilt or rerun in this environment: neither `rustc` nor `cargo` is installed. The Rust 83/83 result is therefore accepted only as a recorded result in `docs/logs/qnir/r6-expansion-5-ext-payload-encoding-vectors.md`, not as a fresh reproduction in this review. The available Python runtime is CPython 3.13.5, whereas the run log records CPython 3.12.16; the current run is a new environment check, not a reproduction of the recorded toolchain. No Git history, commit hashes, GitHub Pages rendering, external Layer-0 calibration, formal proof assistant, or independent human review was available. The review does not claim to establish mathematical soundness beyond the finite checks and arguments stated below.

## 2. Overall verdict

The v0.3 repair work has materially advanced the design. The minimal core, instruction-level operational semantics, canonical byte grammar, terminalization rules, accounting identities, validator-side budget surrogate, golden-first protocol, and two implementation artifacts are now present. The current Python implementation reproduces the committed corpus. The latest log records the same result for Rust.

However, **the conformance corpus itself explicitly leaves required Step-4 surfaces outstanding**. The current 83/83 result means that the implementations pass the 83 committed vectors; it does not mean that all obligations in the conformance specification have been represented by vectors. The corpus's own Suite Register still lists depth-ceiling vectors and a fuel-backstop-on-validator-defect fixture as pending. The conformance specification also registers extension-boundary and collision surfaces, but no corresponding vector is identifiable in the current corpus.

This is a substantive acceptance gap, not a reason to discard the work. The proper status is:

- Step 1: documentary pass, subject to the existing promotion conditions.
- Step 2: documentary pass; implementation agreement is evidenced by the recorded runs and the fresh Python run, but no new Rust reproduction was possible here.
- Step 3: documentary pass; the declared ledger identities and corpus accounting checks are present. Physical-work calibration remains explicitly open and must not be implied by the logical-U result.
- Step 4: **partial**. The artifacts pass the committed corpus, but the corpus does not yet discharge every registered adversarial family and fixture.
- Steps 5 and 6: not started; correctly deferred.

The draft must remain a working draft and QnIR must remain pending in the portal.

---

## 3. Findings

### F20 — The 83-vector corpus does not close the registered Step-4 suite

**Severity: blocking to Step-4 acceptance. Affected artifacts:** `docs/qnir_conformance.md` §§4.4–4.5, `docs/qnir_goldens.md` Suite Register, `docs/qnir.md` §5.

The conformance specification names loops including depth ceilings, calls at the extension boundary, collisions, unresolved statuses, budget boundaries, and conservation vectors. The corpus contains 83 vectors in families E, I, R, M, and X. Its execution vectors include a trip-ceiling rejection, but the corpus's own Suite Register still lists the depth-ceiling vectors and the `fuel-backstop-on-validator-defect` fixture as outstanding. A search of the committed corpus also finds no dedicated call/extension-boundary vector and no collision-surface vector.

The fuel-backstop obligation needs careful treatment. A conforming validator should prevent an admitted execution from reaching the backstop. Therefore, a backstop stop is not a normal expected execution outcome. The test should be framed as a validator-defect injection or negative validator test, with the expected result being detection/classification of false admission—not as a golden that normalizes a defective execution.

**Required disposition.** Keep the 83 existing vectors unchanged. Add only the missing vectors or test fixtures that are required by §4.4, each hand-derived or independently adjudicated before implementation runs. If a registered surface is unreachable, redundant, or incorrectly specified, close it by an explicit, cited adjudication in the program of record and conformance register; do not silently treat it as discharged. Step 4 remains partial until this reconciliation is complete.

### F21 — The corpus's current introductory wording is historically false

**Severity: minor, factual documentation defect. Affected artifact:** `docs/qnir_goldens.md`.

The opening note says that no implementation exists or is drafted at the corpus's current version, and the scope says the corpus was committed before any implementation exists. Those statements were true at initial publication, but are false at v0.3.0-draft.18, where the assembler, validator, reference interpreter, and second implementation are committed.

**Required repair.** Preserve the historical golden-first fact while identifying its time: the seed corpus was committed before the implementation artifacts existed; later implementations check the corpus and never generate or edit its expected results. This is a wording correction only. It must not alter any vector, expected byte, or derivation parameter.

### F22 — The program-of-record status needs to distinguish core artifact completion from suite completion

**Severity: major, record-consistency defect. Affected artifacts:** `docs/qnir.md` §5 and `docs/qnir_conformance.md` §4.5.

The program of record describes the ordered four implementation artifacts as committed and the artifact share as complete in that order. That is accurate for the existence and sequencing of the four artifacts. It can be read too broadly, however, because §4.5 conditions Step-4 artifact-share discharge on passing the full suite, while the corpus's own register still has pending obligations.

**Required repair.** Retain the historical statement that the four ordered artifacts exist. Qualify the current status: the four artifacts pass the currently committed 83-vector corpus, while the full Step-4 suite/acceptance remains partial pending F20. Do not rewrite historical wave records; add a current status clarification in the next appropriate program-of-record increment.

### F23 — Recorded Rust/Python agreement is evidence, not a fresh independent review of the expected results

**Severity: major qualification; not a demonstrated implementation defect. Affected artifacts:** `docs/qnir_conformance.md` §4.1, `docs/logs/qnir/`, `src/qnir/`, `src/qnir-py/`.

The latest log records both implementations matching all 83 vectors, and the current Python rerun reproduces that result. The committed golden-first discipline is the correct control against mutual implementation agreement being mistaken for correctness. This review did not independently re-derive all 83 expected values, rebuild Rust, or inspect the complete derivation instrument and its source archive. Therefore, the review can confirm the recorded protocol and the Python reproduction, but cannot independently certify the independence of every golden derivation or the Rust result.

**Required disposition.** Keep the two implementations and all logs. Do not downgrade their recorded results. Keep the distinction between (a) corpus reproduction, (b) same-artifact reproducibility, and (c) independent derivation/review explicit. The next review or adjudication should focus on the missing obligations, not repeat already recorded runs without a new reason.

### F24 — The Step-5 work is correctly deferred

**Severity: no defect; sequencing confirmation. Affected artifacts:** `docs/qnir.md` Step-5 row, `docs/qnir_semantics.md` §2.8, `docs/qnir_accounting.md` §3.6, `docs/qnir_conformance.md` §4.5.

Full-transition trace roots, collision-bucket resolution, normative artifact lineage, and verbalizer/extractor identity binding are explicitly Step-5 work. The current draft correctly does not pretend those commitments exist. Beginning them before Step 4's registered suite is reconciled would invert the stated repair sequence.

**Disposition.** Keep the Step-5 scope as recorded. Do not expand it or start its implementation now.

---

## 4. Artefact lineage disposition

| Artifact | Disposition | Review result |
|---|---|---|
| `docs/qnir.md` | Keep | Program of record; add a current Step-4 status clarification when next edited |
| `docs/qnir_semantics.md` | Keep | Normative semantic companion; no reason to resynchronize its version number mechanically |
| `docs/qnir_accounting.md` | Keep | Normative accounting companion; logical-U scope and open physical calibration are appropriately distinguished |
| `docs/qnir_conformance.md` | Keep | Normative acceptance criteria; its outstanding obligations must remain visible |
| `docs/qnir_goldens.md` | Keep; wording correction only | 83 vectors remain intact; current historical wording corrected in v0.3.0-draft.19 |
| `src/qnir/` | Keep | Rust artifacts and toolchain pin; fresh build not available in this review |
| `src/qnir-py/` | Keep | Python second implementation; fresh 83-vector run passed |
| `docs/logs/qnir/` | Keep | Historical run and defect evidence; do not replace with summaries |
| `docs/AiReviews/AiReviewQnIRv0.1–v0.3.md` | Keep | Historical review lineage; do not rewrite earlier verdicts |
| `docs/boc.md` v0.4.0 | Keep | No BOC policy revision is required by this review |

No existing implementation, vector family, log, or prior review should be discarded. The correction to `qnir_goldens.md` changes only its current explanatory wording and version metadata; it preserves all 83 vector bodies.

---

## 5. Step disposition and next BOC selection

The next BOC-selected task is **not Step 5**.

The best next task under the current SIMEMP/BOC constraints is:

**Complete and reconcile the remaining Step-4 conformance obligations.**

The order should be:

1. Add the depth-ceiling boundary vectors against the already specified runtime and admission rules.
2. Add the call/extension-boundary vector for the declared typed rejection.
3. Add the collision-surface vector for full canonical-encoding comparison. Do not implement the Step-5 collision bucket here.
4. Define and execute the fuel-backstop defect-injection check at the validator boundary, so it demonstrates detection of a false admission without blessing a defective execution as conforming.
5. Update the corpus register and program of record with explicit PASS/PARTIAL/NOT APPLICABLE dispositions and evidence.

The existing 83 vectors remain byte-unchanged. Any added vectors must be fixed before either implementation is run against them. No implementation should generate expected results.

After those obligations are closed, Step 4 can be reassessed for acceptance. Only then should BOC select Step 5, beginning with the minimum lineage and identity semantics needed by the commitment design.

---

## 6. Repository and navigation notes

`docs/_data/navigation.yml` already lists QnIR and its companions, but it does not list `docs/boc.md`. Since `boc.md` is a maintained, directly referenced provisional elaboration of SIMEMP §7, a concise navigation entry is appropriate. It should be labelled **Provisional**, not Canonical, and placed adjacent to SIMEMP.

The updated corpus file delivered with this review is v0.3.0-draft.19. Its only content change is the correction in F21; the vector data and corpus size/hash necessarily change because the document's metadata and explanatory prose change. The new review itself is an audit record, not a normative QnIR specification.

---

## 7. Final disposition

**Keep the QnIR work. Do not promote it. Do not start Step 5 yet.**

The current implementation evidence is real and useful: Python freshly reproduces all 83 committed vectors, and the repository records the corresponding Rust result. The outstanding issue is coverage of the conformance obligations, not a reason to discard the architecture or redo completed work.

The next development increment should close the specific Step-4 gaps identified in F20 and reconcile the program-of-record status under F22. `docs/boc.md` remains v0.4.0; the BOC policy is sufficient to select this work without another BOC specification expansion.

---

## Correction Note — v0.1.1

**Date:** 8 November 2026  
**Author:** Christophe Duy Quang Nguyen via GPT-6 (OpenAI)

This note corrects and updates the findings and implementation-status assessment in the preceding review. It does not replace the original review or change the QnIR design.

### 1. Corrections to previous findings

- **F20 — Call and collision coverage:** The previous claim that these cases were absent from the conformance corpus was too strong. Their traceability against the obligations in §4.4 remains **NOT VERIFIED**. Each obligation must be mapped to an existing vector, a justified not-applicable disposition, or a future test requirement.
- **F21 — Golden corpus wording:** **CLOSED.** The wording was corrected in `qnir_goldens.md` v0.3.0-draft.19. The correction does not change the 83 vector bodies.
- **F22 — Step-4 completion status:** **OPEN.** Delivery of the four conformance artifacts must not be conflated with acceptance of the complete Step-4 suite.
- **F23 — Reproduction evidence:** The Python implementation was freshly reproduced against all 83 golden vectors, with all passing. The Rust result remains recorded evidence and was not independently rerun during this review. Independent derivation of the expected results is not yet fully certified.
- **F24 — Step-5 strengthening:** Remains deferred until the foundational conformance and accounting issues are resolved.

### 2. Newly detected blocking findings

#### F25 — CLOAD fallback can exceed the trip ceiling

CLOAD fallback edges are included in the control-flow graph and in the admission-time back-edge analysis. However, neither the Rust nor Python execution path applies the trip-ceiling check or increments the traversal counter when a lookup miss takes the fallback edge.

Consequently, the admission bound does not currently bound actual execution.

**Required correction:** Apply the trip ceiling to an actual CLOAD fallback traversal on lookup miss. Check the ceiling before committing the miss transition or creating its miss token. If exhausted, reject with `EXPRESSION_TOO_COMPLEX`; otherwise increment the corresponding edge traversal count and take the fallback. A lookup hit does not count as fallback traversal.

The normative CLOAD semantics and accounting rules must be aligned with this behavior. Add a golden-first regression for a self-looping CLOAD miss with a trip ceiling of one. Derive the expected transition, tokens, receipt, and ledger independently before changing either implementation.

#### F26 — Depth ceiling is undefined and unenforced

The specification refers to depth-ceiling rejection at block entry and uses depth-bounded paths in its accounting bound. However, it does not define a complete operational measure of nesting depth for arbitrary block graphs. The Python profile reads `depth_ceiling` but does not enforce it; the Rust execution path also lacks enforcement.

**Required correction:** Define the meaning and transition rules of nesting depth before implementing enforcement or adding expected vectors. In particular, resolve how depth changes across edges, how exits unwind it, and how irreducible or overlapping loop regions are treated. Do not assume a call stack or structured-loop model that the current architecture has not established.

Until this is defined and enforced, the bounded-path accounting claim is not demonstrated.

#### F27 — Fuel backstop defect-injection obligation remains outstanding

The conformance register lists the `fuel-backstop-on-validator-defect` fixture as pending. This test concerns containment when an admitted Intent reveals a validator defect; it is not a normal successful execution case.

**Required correction:** Specify the controlled test-only fault injection, expected validator-defect verdict, and runtime containment behavior. Verify that the backstop stops execution before overbilling, produces the required fatal outcome, and closes the ledger. Report the validator defect separately from successful containment.

### 3. Revised milestone assessment

| QnIR repair step | Revised status |
|---|---|
| Step 1 — Minimal core | Retain pass, subject to existing promotion conditions |
| Step 2 — Operational semantics | Partial; reopen for F25 and F26 |
| Step 3 — Work accounting and admission | Reopen; bounded execution is not yet demonstrated |
| Step 4 — Conformance oracle and corpus | Partial; F20 traceability and F25–F27 remain outstanding |
| Step 5 — Strengthen commitments | Deferred |
| Step 6 — Extensions with evidence | Deferred |

### 4. Current disposition

Do not promote QnIR v0.3.

The immediate priority is to resolve the bounded-control semantics and accounting defects, then establish their expected behavior through independently derived golden cases and conformance tests. Existing specifications, implementations, review records, and all 83 golden vectors remain preserved; no vector is to be deleted or silently rewritten.

No change to `docs/boc.md` is required. Its current v0.4.0 policy is sufficient to select and govern the next work. This correction records the updated technical evidence; it does not introduce a new BOC procedure or milestone.

### 5. Next-Agent TODO — F25 Dynamic Reproduction

**Status:** INCONCLUSIVE — static gap confirmed; complete runtime reproduction pending.

The current review iteration established a CLOAD fallback trip-ceiling enforcement omission by inspecting the Python and Rust execution paths. However, a complete looping-program reproduction has not yet been performed. The existing Python golden suite passes 83/83 but does not establish coverage of this specific case.

**Required next-agent tasks:**

1. Construct the smallest valid QnIR envelope in which a CLOAD lookup miss takes a declared fallback back-edge.
2. Configure a trip ceiling of one and sufficient fuel to distinguish trip-ceiling rejection from fuel-backstop termination.
3. Execute the envelope using the actual Python reference interpreter. Record both fallback attempts and determine whether the second is rejected with `EXPRESSION_TOO_COMPLEX`.
4. Verify the exact rejection point, miss-token births, transition records, terminal receipt, and ledger settlement.
5. Independently derive the expected result before adding a golden regression. Do not generate expected output from either implementation. Preserve all existing 83 vectors.
6. Run the same regression against the Rust implementation when a working Rust toolchain is available. Record separately which implementations were freshly executed.
7. Report F25 as **CONFIRMED**, **NOT REPRODUCED**, or **INCONCLUSIVE**, supported by the test artifact and execution evidence.

**Environment limitation in the previous iteration:** Python was available, but Rust installation could not be completed because external package-host DNS resolution was unavailable. Root privileges were present; neither `rustc` nor `cargo` was installed. This is an execution-environment limitation, not a QnIR result or a ChatGPT plan restriction.

**Scope boundaries:**
- Do not implement a correction before completing the reproduction and independently deriving expected behavior.
- Do not modify the existing golden vectors.
- Keep F26 depth-ceiling semantics and F27 fuel-backstop defect injection as separate outstanding tasks.
- Do not change the BOC policy or promote QnIR based on this partial result.

**Deliverable:** reproducible test evidence, an independently derived expected result, and an updated finding disposition.

---

## Correction Note — v0.1.2

**Date:** 8 November 2026  
**Author:** Christophe Duy Quang Nguyen via Claude Sonnet 5.5 (Anthropic)

This note independently re-verifies the v0.1.1 findings against the snapshot and executes the F25 reproduction that v0.1.1 left pending. It follows the repository's append-only discipline: the original review and the v0.1.1 note above are unchanged. It does not alter the QnIR design, any vector, or the BOC policy.

### 1. Evidence record

| Item | Result |
|---|---|
| Snapshot | `cdqn-main.zip` as supplied; `qnir_goldens.md` v0.3.0-draft.19, 1,045,702 bytes, SHA-256 `55ff6d8396a7dba092ee9de4ee2a42d38bf39f24cd874922ca3ce365e7021ed4`; `qnir.md` and `qnir_conformance.md` v0.3.0-draft.18; semantics and accounting v0.3.0-draft.8.1 |
| Python reference | **Freshly executed**, CPython 3.12.3: 83/83 (E 11/11, I 3/3, R 4/4, M 15/15, 46 envelopes decode, 49 listings regenerate, X 50/50) |
| Rust implementation | **Not rebuilt.** No `rustc`, `cargo`, or `rustup` in this environment, and network egress is disabled, so the pinned toolchain (`rust-toolchain.toml`, channel 1.99.1) could not be installed. The Rust 83/83 result remains recorded evidence only |
| F25 reproduction | **Executed on the Python reference** (§2). Rust: static reading only |
| Companion script | `f25_repro.py` (observes only; generates no expected values) |

### 2. F25 — CONFIRMED (Python, executed)

**Method.** The committed X42 envelope was decoded; one instruction word was patched (the `CLOAD` fallback operand, block 1 → block 0, a self-loop) and re-encoded with the reference's own deterministic CBOR. The dependency store `D` is empty, so every lookup misses. Profile: trip ceiling 1, depth ceiling 8, gateway 20 U, 4 U per instruction.

**Control.** Unpatched X42 reproduces: 4 transitions, charged 36 U, refund 4 U, `SUCCESS`.

**Variant (`CLOAD addr=0 fallback=block 0`).**

| Observation | Value |
|---|---|
| Admission bound `B_hat(P)` | 44 U = gateway 20 + six 4-U transitions, i.e. one miss traversal, then hit, `JMP`, `ADD`, `JMP`, `HALT` |
| Admission with `W_budget` 100, 300, 10000 | passes in all three |
| Completed `CLOAD`-miss transitions | **6** (`q_load` charged 24 U), against one admitted fallback traversal |
| Trip traversal counters at exit | **empty** — no fallback traversal was ever counted |
| Termination | transition 7: fuel-backstop rejection, `EXPRESSION_TOO_COMPLEX` (tag 3) |
| Ledger | reserved 44 = charged 44 + refunded 0 + outstanding 0 (closed, no overbilling) |
| Tokens | six `STATUS_LOOKUP_MISS` and one `EXPRESSION_TOO_COMPLEX`, all terminalized |

**Findings from the run.**

1. The trip ceiling was not applied to the fallback edge. The edge is slot 0 of its own edge identity (semantics §2.6) and is included in `B_hat`, yet the miss path of `Machine.run` neither checks nor counts it. The Rust `Transfer::Miss` path (`interp.rs`, near lines 871–874) shows the same omission by reading, while its `BR`/`JMP` paths call `check_ceiling` (near lines 1086 and 1091). Rust was not executed.
2. The fuel backstop fired on a program that passed admission. Accounting §3.4 describes the backstop as defense in depth that a sound admission never reaches. Here the defect is **not** a `B_hat` false admission: `B_hat` admits exactly what the trip-ceiling rule allows. It is a runtime/spec enforcement omission, and the backstop contained it correctly.
3. The outcome tag alone cannot distinguish the proposed trip rejection from the backstop, since both are `EXPRESSION_TOO_COMPLEX`. A regression vector must pin the transition count, token set, and ledger.
4. `W_budget` does not separate the two causes. The backstop is bounded by the `B_hat` escrow, not by `W`, so item 2 of the v0.1.1 TODO ("sufficient fuel") is moot.
5. The semantics text states the ceiling precondition for `BR` and `JMP` only. The F25 correction therefore needs a **spec amendment first** (the `CLOAD` row and ceiling precondition in §2.3, aligned with accounting), then a hand-derived golden, then implementation changes.

**Expected behavior under the proposed correction (a proposal, not an adjudicated golden).** Step 1: `CLOAD` misses; the site `(0, 0, 0)` count goes 0 → 1; one Tier O token is born. Step 2: `CLOAD` misses again; 1 + 1 exceeds the ceiling; rejection with `EXPRESSION_TOO_COMPLEX` before the second miss token is created. The exact transition bytes, receipt, and ledger must be derived by hand from §2.3, §2.5, §2.6, and §3 before either implementation is run against them.

### 3. F26 — partially upheld

- **Upheld: the depth measure is under-specified.** Semantics §2.3 says depth is derived from the record sequence but gives no rule for how edges change it, how exits unwind it, or how irreducible or overlapping loops count.
- **Upheld: depth is not enforced.** Python reads `depth_ceiling` in `parse_profile` and never uses it. A search of the Rust interpreter finds no depth-ceiling use (the only `depth` variables are bracket counters in parsing).
- **Overstated: the consequence.** Accounting §3.4 states that the `B_hat` state graph enforces trip ceilings and ignores depth ceilings, which only enlarges the maximized path family. `B_hat ≥ B` therefore holds without depth enforcement, and trip ceilings alone bound execution. The missing depth rule is a definitional gap and a spec/implementation divergence, not a break in the bounded-path claim.
- **Disposition: OPEN, severity reduced.** It no longer reopens Step 3 by itself. Two options are recorded without choosing between them: (A) define and enforce depth; (B) retire the depth ceiling as redundant given trip ceilings. Option B changes a frozen profile parameter and needs explicit adjudication.

### 4. Other re-checks

| Item | v0.1.2 result |
|---|---|
| F20 | Still **NOT VERIFIED**. By vector title, only X7 (`exec-trip-ceiling-rejection`) matches loop, depth, call, collision, or fuel keywords. Family M was not mapped against the call/extension-boundary obligation; the traceability matrix is still required |
| F21 | Version draft.19 is present. The corrected introductory wording was not re-read in this pass; the CLOSED status rests on v0.1.1 |
| F22, F23, F24 | Unchanged |
| Navigation | `navigation.yml` now lists BOC (29 entries); the body §6 observation is resolved |
| `boc.md` §12 | A blank line before `BOC-4-T01` splits the conformance table, so the BOC-4 rows render outside it. This is a formatting defect, not a policy change. The sentence "No change to `docs/boc.md` is required" is corrected to: no **policy** change is required; one rendering fix is recommended |
| Observation O1 | In the variant, the edge from unreachable block 1 to block 3 is classified as a back-edge. `dominators` initializes every node's dominator set to all nodes, and a node with no predecessors keeps that set. It does not affect this result, but the spec does not say how unreachable blocks are treated. A candidate finding, pending adjudication; not numbered |

### 5. Governing disposition (supersedes the body and v0.1.1 step tables without editing them)

The body above was written before F25–F27 and still describes Steps 2–3 as passing and the F20–F22 work as first. Where it conflicts with the v0.1.1 note or this note, this table governs.

| QnIR repair step | v0.1.2 status |
|---|---|
| Step 1 — Minimal core | Pass, subject to existing promotion conditions |
| Step 2 — Operational semantics | **Partial.** F25 confirmed (spec gap and implementation); F26 definitional gap open |
| Step 3 — Work accounting and admission | **Reopened on F25 only.** The accounting document's claim that runtime enforces the admitted bound is not realized for the `CLOAD` fallback edge |
| Step 4 — Conformance oracle and corpus | **Partial.** F20 mapping pending; F25 regression vector and F27 fixture outstanding |
| Steps 5 and 6 | Deferred |

Naming note: the file name carries the review-series index (fourth review); the title refers to the reviewed v0.3 draft lineage; the `version` field versions this review document itself.

### 6. Next-step selection, in BOC rubric form

The "BOC-selected" statements in the body (§5) and in v0.1.1 (§4) were narrative. The SIMEMP §7 rubric mode requires a tabulation, and a conforming `boc.md` invocation also requires a bound profile, priority permutation `π`, budget, and receipt, none of which exist for task selection. The table below is therefore a **QUALITATIVE, ad hoc rubric** (not a versioned, adopted rubric under `boc.md` §8.2) and does not claim a conforming BOC result. `ImplicitDeps` is read as the count of undeclared decisions the item would silently assume.

| Candidate | Tier-1 gate (declared precondition met, finite goal) | `ImplicitDeps` | Note |
|---|---|---|---|
| C1 — F25: amend spec, derive golden, then fix | PASS (defect confirmed, bounded) | 0 | Spec amendment is itself the declaration; golden precedes code |
| C4 — F20 traceability matrix | PASS | 0 | Maps obligations only; should list the F25 vector |
| C2 — F26 depth: define or retire | PASS | 1 | Needs the A/B adjudication first |
| C3 — F27 fuel-backstop fixture | PASS | 1 | Needs new declared fixture semantics (`qnir.md`) |
| C5 — Step 5 commitments | UNRESOLVED | ≥ 1 | Depends on Step-4 acceptance, a declared precondition |

C1 and C4 tie on the leading component, and no `π` is bound. Under `boc.md` §6 and §8.3 the strict result is therefore **INCONCLUSIVE pending adjudication** by the repository's authority. Recommended adjudication: C1 before C4, because the C4 matrix must reference the F25 regression vector and would otherwise be reopened immediately.

### 7. Remaining work (replaces the v0.1.1 §5 TODO)

1. Adjudicate C1 over C4, then amend semantics §2.3 (the `CLOAD` row and ceiling precondition) and align accounting.
2. Hand-derive the F25 golden from a minimal program with no unreachable block (see O1), fixed before any implementation run. Keep the existing 83 vectors byte-unchanged.
3. Apply the correction to Python and Rust. Run Rust on the pinned toolchain in a networked environment and record separately which implementations were freshly executed.
4. Decide F26 (option A or B); specify the F27 fixture semantics; complete the F20 matrix.
5. Fix the `boc.md` §12 table split (documentation only).

**Scope boundaries.** No vector is deleted or edited. No implementation was changed in this pass. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.3

This note records F25's closure at the artifact level and the two corrections the closure forced. It follows the repository's append-only discipline: the original review, the v0.1.1 note, and the v0.1.2 note above are unchanged. It does not alter the QnIR design, the BOC policy, or any committed vector.

### 1. F25 — CLOSED (specification amended, both implementations corrected, golden committed)

- **Specification.** Reading R1 was adopted in the author's adjudication of [`docs/logs/qnir/f25-cload-fallback-trip-ceiling-proposal.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/f25-cload-fallback-trip-ceiling-proposal.md): the `CLOAD` miss edge is a counted edge subject to the declared trip ceiling. `docs/qnir_semantics.md` §2.3 (the `CLOAD` row's rejection column and the derived-count paragraph) and §2.5 (the Tier O repeated-miss rule) were amended accordingly.
- **Implementations.** Both artifacts enforce the ceiling before a repeated miss token is born, with the traversal counted at the fallback-edge site `(block id, instruction index, slot 0)`.
- **Golden.** The Suite Register's F25 regression bullet is discharged at v0.3.0-draft.20 as expansion increment 6: X51 (`exec-cload-fallback-trip-ceiling-self-loop`), the minimal self-looping-fallback program under trip ceiling 1 — first miss completes (one Tier O `STATUS_LOOKUP_MISS` born, site count 0 → 1, 24 U charged), second traversal rejected before the second token is born (`EXPRESSION_TOO_COMPLEX`, nil selected, 8 U refunded at the rejection record), outstanding miss token terminalized there, `B_hat(P) = 32`. The 83 previously committed vectors are byte-unchanged. Both artifacts pass the expanded corpus 84/84, suite X 51/51 each (run log [`docs/logs/qnir/r6-expansion-6-cload-fallback-trip-ceiling.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/r6-expansion-6-cload-fallback-trip-ceiling.md)).
- **Provenance.** X51 was derived by hand from the amended rules before either artifact executed it; no third derivation instrument was used for this vector, and the corpus declares that provenance. Per F23's distinction: the two implementations' agreement is artifact confirmation of the fixed bytes, not an independent re-derivation of them.

### 2. Corrections the closure forced

- **DR-a's field-6 parenthetical was wrong.** The v0.1.1 proposed reading claimed the lookup's address "still appears in the transition's input/output ids (field 6)" on the ceiling rejection. A typed rejection posts nothing and binds nothing, so the rejected transition's field 6 is `[]` — the committed X41 bytes (record 2) already say so. The proposal log's §4 row is corrected and the corpus declares the reading as DR4; the address rides only the completed miss transition's field 6.
- **A second Rust defect surfaced under verification.** The ceiling check committed at v0.3.0-draft.19 was correct, but the Rust artifact's admission DP (`src/qnir/src/bhat.rs`) traversed the `CLOAD` miss edge without consuming the fallback site's trip budget and carried no cycle guard, so it admitted X51's graph and then recursed without bound (stack overflow) — the same admission/runtime divergence class this review's F25 finding targets, on the validator side. The repair applies the Python artifact's existing treatment (`step_edge` on both miss-edge call sites; pre-recursion cycle guard). No committed bound changed; all 84 declared bounds match after the repair. This confirms F25's original observation in a second place: the trip ceiling must be enforced on the fallback edge everywhere the declared boundedness is claimed — runtime, admission, and the accounting identity alike.

### 3. Step status after this note

| QnIR repair step | Status after v0.1.3 |
|---|---|
| Step 1 — Minimal core | Pass, subject to existing promotion conditions |
| Step 2 — Operational semantics | F25 closed; F26 (depth ceiling) open — the A/B adjudication is still needed |
| Step 3 — Work accounting and admission | F25 closed (runtime, admission DP, and accounting §3.4 now agree on the fallback edge) |
| Step 4 — Conformance oracle and corpus | Partial: F20 traceability mapping pending; F27 fuel-backstop fixture outstanding |
| Steps 5 and 6 | Deferred |

### 4. Remaining work (supersedes nothing; carries §7 forward)

1. Complete the F20 traceability matrix (every §4.4 obligation mapped to a vector, a cited not-applicable disposition, or a future test requirement).
2. Decide F26 (option A or B); specify the F27 fixture semantics.
3. Fix the `boc.md` §12 table split (documentation only).

**Scope boundaries.** No vector is deleted or edited. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.4

This note records the F20 traceability register's commitment and F22's status clarification, both at v0.3.0-draft.21. It follows the repository's append-only discipline: the original review, the v0.1.1–v0.1.3 notes above are unchanged. It does not alter the QnIR design, the BOC policy, or any committed vector.

### 1. F20 — the traceability register is committed (the mapping requirement discharged)

- **The register.** §4.4 of [`docs/qnir_conformance.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_conformance.md) (v0.3.0-draft.21) now carries the traceability register: every declared target of the adversarial families paragraph is mapped to exactly one of four disposition classes, each cited to repository bytes — COVERED (committed vectors, by id), CLOSED BY DECLARATION (rule citations), OPEN (a registered obligation whose resolving decision is pending), FUTURE TEST REQUIREMENT (a later-step owner). This is the explicit dispositions-and-evidence mapping this review's §5 item 5 required, and the review's §5 items 2–3 are answered by it: the call/extension-boundary obligation is closed by declaration (the closed core inventory and total grammar make any call construct's bytes an unknown opcode — M1's committed verdict fixes the class; no dedicated call vector can exist), and the collision-surface obligation is covered on the declared in-core surface by X45, with the F6 header-equal/payload-different comparison registered as a future test requirement owned by Step 5.
- **What remains open, and where.** The register's OPEN and FUTURE rows are exactly the corpus's Suite Register's two outstanding bullets: the depth-ceiling vectors (awaiting the A/B adjudication — dedicated vectors or a cited unreachable disposition) and the fuel-backstop-on-validator-defect fixture (awaiting declared fixture semantics — a backstop stop on an admitted Intent evidences a validator defect, never a satisfied admission rule, so it cannot be a conforming golden). These stand as their own obligations; the F20 finding's defect — the missing explicit mapping and evidence — is repaired.
- **No bytes moved.** No vector was added or edited (the corpus digest of record remains `32028d2c76681893` over 1,059,386 repository bytes); no implementation was changed and no run was required (the register binds no artifact input).

### 2. F22 — the current-status clarification is recorded

The program of record's v0.3.0-draft.21 wave record carries the clarification in the required form: the four ordered implementation artifacts exist as committed and pass the currently committed 84-vector corpus on their runs of record, while the full Step-4 suite and acceptance remain partial — the §4.4 suite closes pending the depth-ceiling adjudication and the fuel-backstop fixture, and the traceability register's OPEN and FUTURE rows are the standing list of what remains.

### 3. Step status after this note

| QnIR repair step | Status after v0.1.4 |
|---|---|
| Step 1 — Minimal core | Pass, subject to existing promotion conditions |
| Step 2 — Operational semantics | F25 closed; F26 (depth ceiling) open — the A/B adjudication is still needed |
| Step 3 — Work accounting and admission | Pass, subject to existing promotion conditions |
| Step 4 — Conformance oracle and corpus | F20 mapping discharged (register committed); F27 fixture outstanding; acceptance partial by the register's OPEN/FUTURE rows |
| Steps 5 and 6 | Deferred |

### 4. Remaining work (carries §7 forward, reordered by the register)

1. Decide F26 (option A or B: dedicated depth-ceiling vectors, or a cited unreachable-or-declared disposition).
2. Specify the F27 fuel-backstop fixture semantics (shape, expected verdict, checking protocol at the validator boundary).
3. Fix the `boc.md` §12 table split (documentation only).

**Scope boundaries.** No vector is deleted or edited. No implementation was changed in this pass. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.5

This note records Finding F26's closure by the author's A/B adjudication at v0.3.0-draft.22. It follows the repository's append-only discipline: the original review and the v0.1.1–v0.1.4 notes above are unchanged. It does not alter the BOC policy, any committed vector, or any execution rule.

### 1. F26 — CLOSED (option B: the depth ceiling retired as redundant)

- **The adjudication.** The review's required correction was to define the meaning and transition rules of nesting depth before enforcing it or adding expected vectors, and its audit recorded two options without choosing — (A) define and enforce depth, (B) retire the depth ceiling as redundant. The author adopted **B**, under the standing rubric-mode BOC key: B carries `ImplicitDeps` = 0 (it rides committed rules — every block-graph cycle contains a back-edge, since a back-edge's target dominates its source; every back-edge site carries a finite declared trip ceiling; the §3.4 surrogate already ignored depth), the smallest ΔW_worst, and the smaller coupling; A would have required inventing the normative nesting-depth measure F26 itself says is missing — an undeclared semantics surface, the exact class of implicit rule the BOC policy's admission filter excludes.
- **The reason, stated positively.** The depth ceiling was redundant for the boundedness claim: because every cycle contains a back-edge with a finite trip ceiling, the trip-bounded state graph is finite, so every execution is finite. The depth counter was declared without a total operational measure and enforced by no rule; retiring it is a withdrawal with its reason recorded at each site (semantics §2.1/§2.3/§2.5/§2.6, accounting §3.4/§3.5, conformance §4.4, the corpus profile and register, and the program of record's §1.2 row) — the F13/F15/F12 withdrawal discipline, not a silent deletion.
- **The corollary — a strengthening, not a concession.** With depth gone, the surrogate's state graph is exactly the bounded-path family, so **`B_hat(P) = B(P)` for every well-formed Intent**: the declared one-sided obligation holds as equality, and false admission and false rejection by the static bound are both impossible by construction. The fuel backstop remains what §3.4 declares — defense in depth against implementation defects, which is F27's subject, not a repair for surrogate slack. `QO-17`'s polynomial-time question is untouched.
- **Evidence.** Both artifacts were rebuilt and rerun against the edited corpus: 84/84 each (suite E 11/11, I 3/3, R 4/4, M 15/15, X 51/51; all 51 declared bounds match); transcript digests `38e7b55d09a2f7dd` and `7969721e6679df28`; same-binary and same-source reruns byte-identical; corpus digest of record `d6a0e9fa67ecf1c2` over 1,060,082 LF/repository bytes. No vector byte changed; no vector's bound, outcome, or settlement moved. The adjudication record is [`docs/logs/qnir/f26-depth-ceiling-retirement.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/f26-depth-ceiling-retirement.md).

### 2. Step status after this note

| QnIR repair step | Status after v0.1.5 |
|---|---|
| Step 1 — Minimal core | Pass, subject to existing promotion conditions |
| Step 2 — Operational semantics | F25 and F26 closed (the depth ceiling retired; the trip ceilings carry the boundedness claim) |
| Step 3 — Work accounting and admission | Pass, subject to existing promotion conditions; `B_hat` is exact |
| Step 4 — Conformance oracle and corpus | F20 mapping discharged; the depth-ceiling register row closed by declaration; F27 fixture outstanding |
| Steps 5 and 6 | Deferred |

### 3. Remaining work

1. Specify the F27 fuel-backstop fixture semantics (shape, expected verdict, checking protocol at the validator boundary) — the Suite Register's sole remaining outstanding bullet.
2. Fix the `boc.md` §12 table split (documentation only).

**Scope boundaries.** No vector is deleted or edited. No execution rule was changed. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.6

This note records the disposition of the review's last documentation item — the `boc.md` §12 conformance-table split — and the current count of steps to Step-4 completion. It follows the append-only discipline: the original review and the v0.1.1–v0.1.5 notes above are unchanged. No policy, specification, vector, or implementation changes in this pass.

### 1. `boc.md` §12 — ALREADY DISCHARGED (verified; no edit required)

The v0.1.2 note's "Other re-checks" row recorded that a blank line before `BOC-4-T01` split the §12 conformance table. The committed file already carries the fix: `docs/boc.md` v0.4.1 (commit `1be07f2`, 2026-11-08) removed the gap, and the document's own versioning paragraph records it ("Version 0.4.1 is a documentation-only correction: it removes a blank-line gap that split the §12 conformance table so that the BOC-4 rows rendered outside it, updates the illustrative receipt's specification-version field, and records co-attribution in the metadata. No requirement, result state, metric, receipt field, conformance test, or test outcome changed."). Verification of the committed bytes at this note's version: the §12 region is one contiguous table — the header `| Test | Input condition | Required result |`, its separator, then 58 rows in four contiguous groups (BOC-1: 14, BOC-2: 12, BOC-3: 15, BOC-4: 17), every identifier unique, and no blank or non-table line anywhere inside the table (the only non-table lines in the region are the section heading, the blank line after it, and the blank line before §13). No further edit to `boc.md` is required: the item is discharged by bytes already in the repository, so this note neither re-fixes the table nor bumps the document.

### 2. Steps to Step-4 completion and the next review

| Order | Step | Status |
|---|---|---|
| 1 | F20 traceability register | discharged at v0.3.0-draft.21 |
| 2 | F26 depth-ceiling adjudication | discharged at v0.3.0-draft.22 (option B: the depth ceiling retired; `B_hat(P) = B(P)` exactly) |
| 3 | `boc.md` §12 table split | discharged — verified in the committed v0.4.1 bytes, this note |
| 4 | F27 fuel-backstop-on-validator-defect fixture | outstanding — the corpus Suite Register's sole remaining bullet |
| 5 | Step-4 acceptance reassessment | after step 4 |

So two BOC increments remain before Step-4 can be reassessed: the F27 fixture wave (declared test-only fault-injection semantics, the expected validator-defect verdict, and the containment-and-ledger-closure check — a test-harness obligation, not a conforming golden) and the acceptance reassessment itself. The next review qualifies only after Step-4 acceptance; the Adversarial Review Program's Steps 1–3 minimum has been met since v0.3.0-draft.3, but whether and when a review opens is the program's own act, and Steps 5 and 6 remain deferred.

**Scope boundaries.** No vector, specification, or implementation is changed. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.7

This note records the discharge of step 4 of the v0.1.6 table — the F27 fuel-backstop-on-validator-defect fixture — and the wave's evidence. It follows the append-only discipline: the original review and the v0.1.1–v0.1.6 notes above are unchanged. `QnIR` is not promoted; Step 5 is not started.

### 1. F27 — DISCHARGED (v0.3.0-draft.23)

The required deliverables of the F27 analysis are met. **Reproducible test evidence:** both artifacts gained a declared `--fixture F27` mode that mechanically checks the fixture's declared expectation and both report `fixture F27: PASS`, with the 10 rendered fixture lines byte-identical across the artifacts (fixture-block digest `ed67e6cbd242c69c`); the 84 committed vectors are byte-unchanged (normal mode 84/84 both artifacts after the wave). **An independently derived expected result:** the fixture's exact admission bound is hand-derived (`B_hat(P_f)` = 40 U = gateway 20 U + five 4 U transitions, exact since the F26 corollary) and the declared expectation — the control run's zero-posting refusal, the injected run's backstop stop at step 4 at PC (0, 3), the accumulators (charged 32 / reserved 32 / refunded 0, conservation 32 = 32 + 0 + 0, remaining 4 U, `charged` ≤ `reserved`), the two-token itemization, and the nil selection — was fixed from the committed rules of §2.5, §3.3, and §3.4 before any fixture run, and every number is asserted, never generated. **An updated finding disposition:** F27 is closed at v0.3.0-draft.23; the fixture is a test-only validator-defect injection committed as the corpus's Fixture Register — not a conforming golden, not counted in the declared seed, and committing no record or receipt bytes (a backstop stop on an admitted Intent evidences a validator defect, never a satisfied admission rule — F16 — so committing the defective execution's bytes would normalize it). The declared fixture semantics live in §4.4 of the conformance companion; the record of wave is `docs/logs/qnir/f27-fuel-backstop-fixture.md`.

### 2. Steps to Step-4 completion (updated)

| Order | Step | Status |
|---|---|---|
| 1 | F20 traceability register | discharged at v0.3.0-draft.21 |
| 2 | F26 depth-ceiling adjudication | discharged at v0.3.0-draft.22 (option B: the depth ceiling retired; `B_hat(P) = B(P)` exactly) |
| 3 | `boc.md` §12 table split | discharged — verified in the committed v0.4.1 bytes (v0.1.6) |
| 4 | F27 fuel-backstop-on-validator-defect fixture | discharged at v0.3.0-draft.23 — this note |
| 5 | Step-4 acceptance reassessment | the sole remaining item |

The one remaining increment before Step-4 can be reassessed is the **acceptance reassessment itself**: the Step-4 row's status is to be re-assessed against the now-complete registered suite — 84 committed vectors plus the discharged fixture — under the review's own milestone framing (Step 4: "Build a conformance oracle — assembler, validator, interpreter, and independent second implementation. Re-encode all examples and add adversarial tests for loops, calls, collisions, unresolved statuses, and budget boundaries."). The next review qualifies only after Step-4 acceptance; the Adversarial Review Program's Steps 1–3 minimum has been met since v0.3.0-draft.3, but whether and when a review opens is the program's own act, and Steps 5 and 6 remain deferred.

**Scope boundaries.** The fixture is not a golden vector and the 84 committed vectors are byte-unchanged. `QnIR` is not promoted. Step 5 is not started.

## Correction Note - v0.1.8

This note records the reassessment of Steps 2–4 against the committed evidence — the review's own §5 order item 5 — and the resulting Step-4 acceptance. It follows the append-only discipline: the original review and the v0.1.1–v0.1.7 notes above are unchanged. No vector, specification rule, or implementation is changed by this pass; `QnIR` is not promoted and Step 5 is not started.

### 1. Steps 2–4 — reassessed and accepted (v0.3.0-draft.24)

The reassessment record is `docs/logs/qnir/step4-acceptance-reassessment.md`. Against conformance §4.5's acceptance condition, the **five deliverables PASS**: the golden-first differential protocol with independence evidence (the corpus committed before any implementation; two independent standard-library implementations, each passing on its own run — mutual agreement never the criterion); the assembler and validator (E 11/11, I 3/3, M 15/15; the X assembly share 51/51; the 51/51 declared admission bounds); the reference interpreter and second implementation (X execution 51/51 at the five agreement points in both artifacts); the vector suite with the traceability register's family dispositions; and the re-encoding of every committed example against the frozen core only. The family dispositions carry the review's explicit classes: loops at/over the trip ceiling **PASS** (X7, X51) with nesting **NOT APPLICABLE** (retired at v0.3.0-draft.22); calls **NOT APPLICABLE** (M1 fixes a call construct's bytes; extension work by §1.3's register); collisions **PASS** in-core (X45) with the F6 header-equal/payload-different comparison **DEFERRED to Step 5** (F24's deferral stands); unresolved statuses, encoding, budget boundaries (including the F27 fixture), conservation, and the per-class traces all **PASS**.

The revised milestone table (superseding the v0.1.1 statuses, which the original text and the v0.1.1 note retain):

| QnIR repair step | Reassessed status |
|---|---|
| Step 1 — Minimal core | Retain pass (unchanged; promotion conditions unchanged) |
| Step 2 — Operational semantics | **Accepted** — F25 (v0.3.0-draft.19) and F26 (v0.3.0-draft.22) closed; transition relation, terminalization, and encodings fully exercised |
| Step 3 — Work accounting and admission | **Accepted** — the exact surrogate's admission with the committed refusal/exact/slack cells; the conservation identities across the suite; the F27 fixture's containment of an admitted under-reserved execution (`charged ≤ reserved`, no double billing) |
| Step 4 — Conformance oracle and corpus | **Accepted** — the five §4.5 deliverables discharged by repository bytes (this note); the collision-bucket comparison remains Step 5's |
| Step 5 — Strengthen commitments | Deferred — the next BOC increment (minimum lineage and identity semantics) |
| Step 6 — Extensions with evidence | Deferred |

### 2. What the acceptance is, and is not

Acceptance is of the oracle and its corpus: the artifacts exist as repository increments, pass the golden-first differential protocol over the full suite, and carry their declared version bindings and independence evidence. It is **not** promotion. The promotion criterion is unchanged and unmet — Step 5's commitments (full-transition trace roots, collision-safe artifact comparison, normative lineage, versioned verbalizer/extractor identities) do not exist, `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, and no claim is styled a theorem. The collision bucket's comparison and the call construct are not quietly absorbed: the first remains Step 5's, the second Step 6's.

### 3. Next selection

The review's §5 order stated: "After those obligations are closed, Step 4 can be reassessed for acceptance. Only then should BOC select Step 5, beginning with the minimum lineage and identity semantics needed by the commitment design." The obligations are closed and Step 4 is accepted, so the next BOC increment is Step 5's first specification share — the minimum lineage and identity semantics — declared over the accepted oracle, with no implementation, no vector, and no promotion implied by beginning it.

**Scope boundaries.** No corpus byte, artifact byte, or specification rule changes. `QnIR` is not promoted. Step 5 is not started by this note.

## Correction Note - v0.1.9

This note records the opening of Step 5 — its first specification share, the minimum lineage and identity semantics — committed at v0.3.0-draft.25. It follows the append-only discipline: the original review and the v0.1.1–v0.1.8 notes above are unchanged. No corpus byte, artifact byte, or committed §1–§4 rule changes; `QnIR` is not promoted.

### 1. The share

The review's §5 order is followed exactly: "beginning with the minimum lineage and identity semantics needed by the commitment design." The new companion `docs/qnir_commitments.md` §6 declares: **artifact identities** `(kind, version, digest)` over the closed kind vocabulary, with SHA-256 over each kind's declared byte basis and triple equality as the only identity relation; the **lineage binding** `(corpus A, implementation A, V, E, n)` with the evidence digest over the ordered canonical record encodings and the receipt encoding, the binding rule (outcome equality only between bindings of equal corpus identity and `V`; each implementation reproduced from its own execution; divergence recorded, never reconciled), lineage as external evidence metadata (the frozen encodings and the 84 vectors are untouched), and the `QO-24` separation (minimal-zoom denotational identity; provenance rides the binding, never the value key); and the versioned **extractor `qir-extractor-1`** and **verbalizer `qir-verbalizer-1`** identities with the Obj 15 read-back binding rule `(verbalizer identity, SHA-256 of the exact text)` and the generation rule. The Step-5 register records the two declared deliverables' specification shares as discharged with explicit artifact-share remainders, and keeps the full-transition trace roots and the collision-safe artifact comparison as future shares — the review's own deferral of the bucket resolution is respected.

### 2. Step-5 state

| Step-5 deliverable | State |
|---|---|
| Normative lineage | Specification share discharged (v0.3.0-draft.25); artifact share (emitted lineage blocks, logs citing them) future |
| Versioned verbalizer/extractor identities | Specification share discharged (v0.3.0-draft.25); artifact share (per-run `(generation, digest)` pairs) future |
| Full-transition trace roots | Future — the construction must be declared before implementation; the evidence digest is explicitly not the trace root |
| Collision-safe artifact comparison (header-equal, payload-different) | Future — the §1.4 F6 bucket resolution; the conformance register's last future row |

Step 5 remains **open**; the promotion criterion is unchanged and unmet, `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, and no claim is styled a theorem. The next BOC increment, recorded in the wave's own selection: the artifact share of the declared lineage and identity semantics (both implementations emitting the lineage block and generation-bound read-back pairs, cited by the logs), then the trace-root declaration, then the collision bucket.

**Scope boundaries.** No corpus, artifact, or §1–§4 specification byte changes. `QnIR` is not promoted; the step's register alone governs its discharge.
