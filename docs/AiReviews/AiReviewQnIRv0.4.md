---
layout: default
title: QnIR v0.3 — Independent Follow-up Review
description: Independent audit of the v0.3.0-draft.18 specification, corpus, implementations, and artifact lineage
version: 0.1.0
updated: 2026-11-07
author: "Christophe Duy Quang Nguyen via GPT-6 (OpenAI)"
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
