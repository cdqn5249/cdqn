---
layout: default
title: Qn Structures — Vision and Evaluation Framework for Category C Data Structures and Workflows
description: Category C proposal — contract schema, evaluation ladder, and candidate register for Qn data structures
version: 0.3.0
updated: 2026-11-21
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qstructures.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qstructures.html
status: Candidate Specification — Measured Ladder Pass (Step 6.1 Container BOC); QnIR Extension Contract Pending
terms_used:
  - boc-policy
  - cae-dag
  - causal-arrow
  - cdqn
  - computational-consistency
  - compute-unit-u
  - dcc-profile
  - dependencies-determinism
  - existential-invariant
  - historical-calibration-benchmark
  - layer-0
  - licensed-work
  - lifecycle-state
  - local-first
  - metric-envelope
  - metric-exhaustion
  - no-implicit-rule
  - open-core-invariant
  - paternity-reference
  - payload
  - q-array
  - q-bypass
  - q-datastruc
  - q-matrix
  - q-patterns
  - q-reuse
  - q-stream
  - q-table
  - q-workflows
  - qm
  - qn
  - qn-rsi
  - qnir
  - receipt
  - security-envelope
  - simemp
  - simemp-gateway
  - ssl
  - structural-indirection
  - terminal-exactness
  - universal-envelope
---

# Qn Structures — Vision and Evaluation Framework for Category C Data Structures and Workflows

| Field | Specification |
|---|---|
| **Document Title** | Qn Structures — Vision and Evaluation Framework for Category C Data Structures and Workflows |
| **Version** | 0.3.0 |
| **Last Updated** | 2026-11-21 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Candidate Specification — Measured Ladder Pass (Step 6.1 Container BOC); QnIR Extension Contract Pending |

---

## Review Integration Record (v0.2.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md) — verbatim transcript and artifact of record of the Loop-2 adversarial review by Kimi K3.1-Pro (October 27th, 2026) of the QnIR v0.2 candidate draft v0.2.2 — as integrated upstream by the two committed anchors this file's §6 work-row precedent mirrors: the constitutional two-axis Layer-0 Hardware Telemetry Envelope of `docs/simemp.md` v1.3.0 and the operational two-axis admission predicate of `docs/qnPrimitive.md` v1.4.0 (§1.15 term provenance, §5.9 margin governance and Admission Ledger). The cross-file dependency paragraphs of the `docs/qnPrimitive.md` v1.4.0, `docs/qm_geometry.md` v1.3.0, and `docs/qexpr.md` v1.4.0 records flag this file's mirror obligation — the §6 work-row precedent (which cites `docs/qnPrimitive.md` §1.15 as the committed breakeven shape) inherits the two-axis form at its next versioned update — discharged here. Qstructures makes no new policy adjudication of its own: the predicate tokens, the margin range `[1, 64]`, and the recommended preset (`κ_dcc = 2`, `κ_t = 4`) are inherited unchanged from `docs/qnPrimitive.md` §5.9, and the proposal register of this document is unchanged — nothing herein redefines, amends, or overrides any canonical specification. Under **Axiom 13** the update is a new versioned artifact; no prior content is mutated in place; the withdrawn v0.1.0-era wording is preserved verbatim in this record. Every construction herein remains a candidate artifact under **Axiom 14** (`docs/qs.md` §6), tagged **[v0.2.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

**Withdrawn wording (v0.1.0-era, preserved verbatim):**
1. Q(reuse) is admissible if and only if the predicted computational work of direct re-evaluation exceeds lookup and retrieval dissipation plus a declared overhead term (its live `term.html` include is rendered as plain "Q(reuse)" here to keep the record inert) — the work-axis-only "if and only if" form, composed of the two v1.3.0-era `docs/qnPrimitive.md` statements withdrawn there (the §1.15 provenance sentence and the §5.9 display inequality `Admit(Q(reuse)) ⟺ ΔW_compute > ΔW_lookup + ε_overhead`); the bare form carries no latency-axis verdict — the QO-17 defect;
2. That inequality is a *declared breakeven rule* — a contract row, not a runtime heuristic — and it is measured in units of the Abstract Compute Unit U. (its live `term.html` include is rendered as plain text here to keep the record inert) — re-issued below with the U measurement confined to the work axis;
3. the §6 work-row declaration items "(i) the work class per access morphism" and "(iii) the breakeven inequality under which the structure is preferred over its recorded alternatives" — re-issued below with the declared latency companion and the two-axis breakeven rule;
4. the §5 registration-deferral paragraph "Deliberately, this document **registers nothing**. Glossary entries are data-only commits under the candidate pipeline, and the four proposed names remain plain text herein — they receive no include references and no concordance — until a separate registration commit follows the pipeline, as was done for the consultation vocabulary on 2026-10-22. The deferral is queued in Section 8." — the registration commit has since followed (glossary, portal index, navigation registry), so the deferral state no longer holds;
5. the §8 item 1 "**Vocabulary registration.** The four proposed names of Section 5 await a data-only glossary commit; this document deliberately registers nothing itself, and until registration the names remain plain text with no concordance entries." — discharged;
6. the §8 item 2 "**Navigation and index ripple.** One navigation entry and one portal index entry are proposed for a follow-up commit, honoring the one-change-per-commit discipline; no canonical file is modified by this document." — discharged;
7. the §8 opening sentence "All items below remain in CANDIDATE status." — re-scoped to items 3–6.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Two-axis re-issue of the work-row precedent**: the §6 admission precedent is re-issued as the Two-Axis Consequentiality Breakeven — advantage declared on **both axes** of the `docs/simemp.md` §4.4 envelope ("consequently faster") over the canonical tokens of `docs/qnPrimitive.md` §1.15/§5.9 (`ΔW_compute`, `Δt_compute_bound`, tier-indexed `τ_U[tier(h)]`/`τ_t[tier(h)]`, gateway slacks `ε_U`/`ε_t`, margin range `[1, 64]` with preset `κ_dcc = 2`/`κ_t = 4` inherited unchanged), with compact term provenance, the safe-fallback discipline for undeclared quantities, and Admission Ledger execution (`docs/qnPrimitive.md` §5.9); the U measurement is confined to the work axis, the latency axis being declared in cycles against the node's declared reference clock; a new comparability paragraph records the heterogeneity rationale — no single declared constant can be smart across machines whose lookup timings differ, so cross-node comparability comes from the declared envelopes themselves rather than from assumed identical timings | §6 | `QO-17` (via `docs/qnPrimitive.md` v1.4.0); consequentiality constraint (`docs/simemp.md` §4.4); F1 declared-bound discipline | CANDIDATE |
| **Work-row declaration extension**: the §6 declaration triple is re-issued — item (i) gains the declared latency companion per access class, item (iii) becomes the two-axis breakeven rule (advantage declared on both axes of the envelope, never on the work axis alone), after the Q(reuse) precedent | §6 | author field report ("consequently faster", via `docs/qnPrimitive.md` v1.4.0); two-axis envelope (`docs/simemp.md` §4.4) | CANDIDATE |
| **Registration-state reconciliation**: §5 adopts the four registered names as include references (`q-array`, `q-matrix`, `q-table`, `q-stream`, already exercised at the portal index and in the navigation registry) and re-issues its deferral paragraph; §8 items 1–2 are marked discharged and the §8 preamble re-scoped — the data-only registration was performed by separate commits, not by this proposal, which registers nothing itself | §5, §8 | glossary registration commit (`primitives.yml`); committed portal index and navigation-registry state; candidate-pipeline registration discipline (§3 step 1) | CANDIDATE |
| Editorial: the §7 grounding-table rows are aligned — the Q(reuse) row is re-named to the two-axis consequentiality breakeven and gains §5.9 in its grounding; the structures-names row grounding is updated from "registration deferred" to the registered state — index alignment, no new claims | §7 | `docs/qnPrimitive.md` v1.4.0 record cross-file flag | EDITORIAL |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline): `docs/qphy.md` — the physical-impedance breakeven grounding inherits the two-axis form and the tier-table naming at its next versioned update (the last open content mirror of the Q(reuse) re-issue wave); glossary — the `q-table` definition's "breakeven inequality" wording inherits the two-axis form at the next glossary update (registered pre-wave, it cites the committed Q(reuse) analysis); the unbudgeted `docs/_data/navigation.yml` review-program entry remains optional; the substantive next horizon is the QnIR v0.3 minimal executable core per the Loop-2 repair sequence. Front matter `terms_used` is extended with the four registered slugs (`q-array`, `q-matrix`, `q-table`, `q-stream`) — this file's first extension — under the include-parity discipline; no new glossary registrations are proposed, and no other new `term.html` include ids are used (the re-issued §6 re-uses only the `q-reuse` and `compute-unit-u` includes that pre-date it) under the standing no-dead-entries discipline of `docs/qs_history.md` §8.

---

## Revision Record (v0.3.0)

Under **Axiom 13** this v0.3.0 artifact is a new versioned document: §1–§9 prior text, the Review Integration Record (v0.2.0), and its withdrawn-wording register are preserved verbatim — no prior content is mutated in place. The revision changes only the front-matter and body-table version/date/status fields and adds one new section at the end, `## Measured Ladder Pass — Step 6.1 Container BOC (v0.3.0 — CANDIDATE)`, recording deterministic row measurements for `Q(array)` and `Q(table)` against the committed `6cc1ef1` (v0.3.0-draft.32) study. Evidence of record: `docs/logs/qnir/step6-1-container-boc-study.md` and its coupled `docs/logs/qnir/f-g0-1-state-root-repair-draft32.md`. The status advances from "Not a Finalized Specification" to "Candidate Specification — Measured Ladder Pass (Step 6.1 Container BOC); QnIR Extension Contract Pending": Ladder Steps 2–3 are measured for two families, while sealing (Step 5), the `qir-enc-3` opcode contract, and final QnIR core extension admission (Wave 4B) remain pending. **No new term include is introduced** — the `terms_used` list is unchanged, the `term-include` delta is 0, and `Q6` remains `QUEUED`. No canon file, glossary, navigation registry, or `src/` path is touched by this revision.

---

## Purpose and Epistemic Stance

This document is proposed as the anchoring specification of **Category C — Data Structures and Workflows** of the Qn definition taxonomy ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §11). It originates in the October 2026 design consultation on substrate feasibility (recorded internally as the structures consultation), which observed that the validated {% include term.html id="cae-dag" text="CAE-DAG" %} answers how every artifact is *represented* while leaving open how structures are *specified, measured, and selected* as access contracts. It is deliberately positioned in the same epistemic register as [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) and [`docs/history_library.md`]({{ '/history_library.html' | relative_url }}): a proposal record, not a sealed specification.

Every design element below carries the explicit status of a **candidate artifact** under **Axiom 14** ([`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6): it remains subject to deterministic measurement against the declared {% include term.html id="simemp" %} constraints, to selection under the {% include term.html id="boc-policy" text="BOC" %} policy, and to eventual sealing through a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop at implementation time. Nothing in this document redefines, amends, or overrides any existing canonical specification, prescribes no runtime implementation, and assigns no storage layout.

One methodological commitment governs the whole document, and it is stated here because it outranks every particular claim that follows: **the steps that guide and explain the design are more important than the final selections**. Which structures are ultimately admitted is a function of implementation feedback and {% include term.html id="qn-rsi" text="Q(rsi)" %} optimization, not of authorial decree. Accordingly, this document invests primarily in the evaluation procedure (Sections 2 and 3) and offers the candidate register (Section 4) only as a first worked pass of that procedure — a record of reasoning that keeps alternatives revisitable, not a set of winners. In accordance with {% include term.html id="computational-consistency" %}, the validity claimed herein is internal consistency within declared finite boundaries.

---

## 1. Substrate and Interface: Why the Validated DAG Is Not Sufficient

The {% include term.html id="qn" %} universe commits every artifact to a single substrate: each is a governed {% include term.html id="universal-envelope" text="Universal Envelope" %} carrier participating in the local {% include term.html id="causal-arrow" text="causal DAG" %}, hash-addressed, acyclic by construction (a circular dependency would break the pre-image resistance of the commitment hash, [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2.4), and sealed in an immutable {% include term.html id="lifecycle-state" %}. The choice of this substrate was itself an exercise of the {% include term.html id="boc-policy" %}: `Qexpr` explicitly rejects naive pointer-heap trees in favor of the CAE-DAG ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2), recording the rejection and its reasons rather than silently defaulting.

The substrate, however, answers only the representation question. In established computer science, a data structure is not primarily a memory layout but a **documented access contract with explicit performance tradeoffs** — an array promises indexed access at contiguous-locality cost, a hash table promises keyed access at hashing-and-resize cost, and each is chosen by matching its contract to the workload. The canonical files already exercise such contracts ad hoc — bounded arrays appear as `OP_FOLD` child vectors, keyed lookup appears inside the {% include term.html id="q-reuse" text="Q(reuse)" %} breakeven analysis — but no committed text specifies the contract surface itself. The question "is the validated DAG enough?" therefore receives the answer: **necessary, not sufficient**. The DAG subsumes representation; the contract vocabulary for structures is the open gap this document proposes to close, under {% include term.html id="local-first" %} discipline: every structure proposed here executes one hundred percent locally, importing no network consensus into its access path.

---

## 2. The Structural Contract Schema

It is proposed that every Qn data structure — and every proposed extension of the structure vocabulary — be specified as a **five-row contract**, each row binding one declared discipline of the {% include term.html id="simemp" %} constraint system. The schema is the stable core of this proposal; particular structures are its outputs.

```
+----------------------------------------------------------------------------+
| THE FIVE-ROW STRUCTURAL CONTRACT SCHEMA (CANDIDATE)                        |
+----------------------------------------------------------------------------+
| 1. FOOTPRINT   : declared Metric Envelope ceilings; growth emits a new     |
|                  versioned artifact, never mutates in place                |
| 2. ACCESS      : enumerated permitted access morphisms; no implicit        |
|                  operation is admitted (No-Implicit Rule)                  |
| 3. LOCALITY    : Memory Wall movement class per access pattern, accounting |
|                  for physical data-movement dissipation (SIMEMP 4.1)       |
| 4. DETERMINISM : iteration and traversal order fixed by causal index;      |
|                  no runtime-adaptive reordering is permitted               |
| 5. WORK        : thermodynamic tradeoff row — work per access class in U,  |
|                  versioning amplification, and a declared breakeven rule   |
+----------------------------------------------------------------------------+
```

Each row derives from committed canon rather than from this document's preference. Row 1 instantiates the {% include term.html id="metric-envelope" text="Metric Envelope" %} footprint discipline of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §8 and the versioning rule that successor artifacts advance the causal index while mutation in place is prohibited. Row 2 instantiates the {% include term.html id="no-implicit-rule" text="No-Implicit Rule" %} at the structure level: the glossary definition of {% include term.html id="q-datastruc" text="Q(dataStruc)" %} already commits that "no implicit heap state is admitted", and this document generalizes that refusal from heaps to all implicit operations. Row 3 instantiates the {% include term.html id="simemp" %} Memory Wall accounting of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.1, which requires explicit accounting for memory bandwidth and locality rather than purely abstract asymptotic complexity. Row 4 instantiates {% include term.html id="dependencies-determinism" text="Dependencies-Determinism" %}: a structure whose iteration order could vary at runtime would inject an untraceable input into every workflow that consumes it. Row 5 anticipates Section 6: every admitted structure must declare its thermodynamic cost surface, in the same spirit in which the canon already declares breakeven inequalities for optimization morphisms.

A proposal that cannot yet fill a row is **not rejected** — it is recorded as incomplete, with the missing row named. This is deliberate caution: the schema is a measuring instrument, and an instrument must be able to report partial results without discarding the specimen.

---

## 3. The Evaluation Ladder: From Nomination to Sealing

The following five-step ladder is proposed as the standing procedure by which any structure — classical, novel, or machine-discovered — enters the Qn universe. The ladder is the deliverable of this document; the register of Section 4 is merely its first worked pass.

1. **Nomination.** A structure is proposed by naming its contract surface: the five rows of Section 2, filled or explicitly marked incomplete. At this stage the structure is CANDIDATE vocabulary in plain text; it receives no glossary entry and no include reference until registration (Section 8, item 1).
2. **Row measurement.** Each row is measured against the declared {% include term.html id="simemp" %} constraints. All threshold values are **declared values inside the owning {% include term.html id="dcc-profile" text="DCC contract" %}**, never runtime negotiations, in accordance with the {% include term.html id="no-implicit-rule" %}; this mirrors the tolerance-envelope discipline already proposed for the {% include term.html id="historical-calibration-benchmark" text="Historical Calibration Benchmark" %} wiring in [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2.7.
3. **BOC selection.** Where alternatives exist for the same access niche, the {% include term.html id="boc-policy" text="BOC" %} policy of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §7 selects among recorded alternatives, and rejections are recorded with their reasons — the precedent being the recorded pointer-heap rejection of [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2. A rejection is a finding, not an erasure.
4. **Implementation feedback.** Deterministic measurement at execution boundaries — {% include term.html id="receipt" text="receipts" %} emitted by actual {% include term.html id="qnir" text="QnIR" %} lowering — tests each row's declared values against observed behavior. A row whose declared values are falsified sends the structure back to step 1 with the finding recorded; it does not silently adjust thresholds.
5. **Sealing.** A structure whose rows survive measurement is sealed through a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop under strict semantic invariance, in the manner of the anti-drift discipline of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §7.4. A sealed selection is superseded only by a new versioned artifact along the local {% include term.html id="causal-arrow" %}, never by mutation in place.

Two cautions bound the ladder itself. First, the ladder adds no stage to any canonical state machine: it is a proposal discipline for this document's own scope, executable entirely within the candidate pipeline of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6. Second, the ladder is itself a legitimate candidate for optimization by a future {% include term.html id="qn-rsi" text="Q(rsi)" %} loop — including its own row definitions — under the same semantic-invariance requirement it applies to others.

---

## 4. The Candidate Register: A First Worked Pass

The register below applies Sections 2 and 3 to the classical structure menu. Its status is deliberately modest: these are **worked examples of the ladder's first pass**, recorded so that reasoning survives and alternatives remain revisitable — not selections. Each entry cites its grounding in committed canon; nothing herein redefines that canon.

**Canonical-admitted candidates.** Five structure families already have their contract surfaces exercised across the canon and are nominated for full row measurement:

- **Sequenced container** (proposed name `Q(array)`): grounded in the `OP_FOLD` node class, whose child vector encapsulates the full array of term commitments under an explicit step ceiling ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.4). Footprint is a declared length; access is indexed; locality is the contiguous class; determinism is index order; work is constant per indexed access under a declared budget. Unbounded series are already syntactically invalid under Axioms 2 and 6 in the committed text.
- **Geometric container** (proposed name `Q(matrix)`): grounded in `OP_FRAME_MAP` and the integer Matrix-Multiply-Accumulate lowering of [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §5.1, which completes contractions in constant time without floating-point drift. Exactness is preserved by construction; the {% include term.html id="q-bypass" text="Q(bypass)" %} morphism short-circuits identity preconditions in register time.
- **Keyed container** (proposed name `Q(table)`): grounded in hash-consing itself — the CAE-DAG is already a content-addressed keyed index — and in the committed breakeven analysis that weighs Q(reuse) against hash-table overhead ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §7.2). Capacity is declared; growth is re-versioning.
- **Record tuple**: grounded in the {% include term.html id="qm" text="Qm" %} base record and the canonical multivector state tuple ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §4.1) under {% include term.html id="terminal-exactness" text="Terminal Exactness" %}. The fixed-field record is the atomic structure family and requires the least novel specification.
- **Persistent tree (DAG-deduped)**: grounded in the single-rooted directed trees that local artifacts already form from the physical silicon root ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §3, Axiom 4). Hash-consing makes path-sharing between versions the expected case rather than the exception.

**Rejected by construction.** Two classical families cannot enter, and the register records why:

- **Pointer-heap tree**: rejected under the {% include term.html id="boc-policy" text="BOC" %} policy in committed text ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2) — implicit allocation violates the {% include term.html id="no-implicit-rule" %}, and pointer chasing dissipates across the Memory Wall per hop. The rejection stands unless a future nomination declares a complete, measurable contract surface that cures both defects.
- **Cyclic graph**: excluded mathematically, not by policy — a circular dependency would require breaking the pre-image resistance of the commitment hash ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2.4, Axiom 5). No contract surface can cure a construction impossibility.

**Recoverable as contracts.** Two classical families are absent from the canon and are nominated only as *open contract shapes*, pending implementation feedback:

- **Sequenced chain** (linked-list analog): as a pointer-chasing runtime shape it fails rows 1 and 3 outright; it may be recoverable only as a declared sequenced container whose traversal is index-ordered, or as a chain of {% include term.html id="receipt" text="receipts" %} in the manner of the chained Road Receipts by which {% include term.html id="q-workflows" text="Q(workflows)" %} achieve portability.
- **Bounded ring** (queue/stream analog): bounded streams are already exercised — entropy sampling collects a declared sample volume at [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2.1, and receipt chains accumulate append-only — but no container type is committed. A ring is nominated only with declared capacity and sequenced access; an implicitly resizable queue would violate rows 1 and 4.

The recoverable pair illustrates the document's method commitment: the ladder records *why* each family stalls, so a later nomination — informed by implementation feedback or by a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop — can re-enter at step 1 with the findings attached, rather than reconstructing the analysis from conversation history.

---

## 5. Proposed Nomenclature Extension

The Qn universe already names operational morphisms and structure families in a consistent `Q(...)` pattern: {% include term.html id="q-reuse" %}, {% include term.html id="q-bypass" %}, {% include term.html id="qn-rsi" %}, and the registered trio {% include term.html id="q-datastruc" %}, {% include term.html id="q-patterns" %}, {% include term.html id="q-workflows" %}. It is proposed that the structure vocabulary extend in the same pattern with the four names used in Section 4 — {% include term.html id="q-array" text="Q(array)" %}, {% include term.html id="q-matrix" text="Q(matrix)" %}, {% include term.html id="q-table" text="Q(table)" %}, {% include term.html id="q-stream" text="Q(stream)" %} — under one consistency rule: **a `Q(...)` name denotes a contract surface (the five rows of Section 2), never a runtime shape**. This rule is what keeps the nomenclature honest across heterogeneous hardware: the same {% include term.html id="q-array" text="Q(array)" %} contract is satisfiable by different lowerings on different nodes, exactly as the compute unit evaluates identically across all nodes while local entropy parameterizes identity.

Deliberately, this document itself still registers nothing: glossary entries are data-only commits under the candidate pipeline, made by a separate registration commit rather than by this proposal. That separate registration has now followed — the four names are registered in the glossary (`primitives.yml`) and exercised as include references at the portal index and in the navigation registry — so from this update the names carry include references and concordance entries herein. The deferral queued in Section 8, item 1, is discharged. [v0.2.0 — CANDIDATE]

---

## 6. Breakeven Rules and the Work Row

The work row of Section 2 has one committed precedent whose shape every admitted structure should follow: {% include term.html id="q-reuse" %} is admissible under the **Two-Axis Consequentiality Breakeven** of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15/§5.9 — its advantage over direct re-evaluation must be declared on **both axes** of the Layer-0 Hardware Telemetry Envelope of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4, the work axis and the latency axis, the consequentiality constraint that retrieval be *consequently faster* than direct compute rather than merely cheaper in work:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

Every token of that predicate is a declared deterministic quantity, never a runtime sample: `ΔW_compute` and `Δt_compute_bound` are the DCC-declared worst-case work and latency bounds of the direct re-evaluation path; `τ_U[tier(h)]` and `τ_t[tier(h)]` are the declared per-tier worst-case lookup bounds, calibrated at Layer-0 onboarding; `ε_U` and `ε_t` are the declared gateway overhead slacks; and `κ_dcc`, `κ_t` are the declared advantage margins within the governance-bounded range `[1, 64]` (recommended preset `κ_dcc = 2`, `κ_t = 4`, inherited unchanged from [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §5.9). An undeclared quantity forces the safe fallback to direct execution, and every admitted reuse executes under the Admission Ledger of `docs/qnPrimitive.md` §5.9. That predicate is a *declared breakeven rule* — a contract row, not a runtime heuristic — with its work axis measured in units of the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} and its latency axis in cycles against the node's declared reference clock. [v0.2.0 — CANDIDATE]

Two properties make U the right unit for comparable work rows across heterogeneous nodes. First, platform invariance: while local entropy parameterizes cryptographic identity, the metric magnitude of U evaluates identically across all nodes ([`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §4.2), so a work row declared on one node is interpretable on every node. Second, the thermodynamic leash: unbounded recursive execution is physically prohibited, every workflow operates within a finite, non-renewable work budget, and metric depletion halts execution deterministically under {% include term.html id="metric-exhaustion" text="metric exhaustion" %} — so no structure's work row can be rendered moot by unmetered iteration.

Platform invariance, however, belongs to the work axis alone, and that limitation is the substance of the two-axis re-issue: no single declared constant can be smart across heterogeneous hardware, because every machine reports different lookup timings and every lookup's cost varies with the memory tier it touches and with machine condition. The latency axis is therefore declared per tier — `τ_t[tier(h)]` in cycles against the node's declared reference clock, calibrated at Layer-0 onboarding — so that comparability across nodes comes from the declared envelopes themselves rather than from assumed identical timings, and retrieval is admitted only when it is consequently faster on both axes. A work row that declares advantage on the work axis alone is incomplete under this discipline: it reports how much work a structure saves, but not when retrieval is worth the Memory Wall at all. [v0.2.0 — CANDIDATE]

It is proposed, therefore, that every admitted structure declare in its work row: (i) the work class per access morphism, measured in units of the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}, with its declared latency companion per access class in cycles against the node's declared reference clock, (ii) the versioning amplification — the cost emitted by row 1's growth-as-versioning discipline — and (iii) the breakeven rule under which the structure is preferred over its recorded alternatives — declared on **both axes** of the Layer-0 Hardware Telemetry Envelope, after the {% include term.html id="q-reuse" %} precedent above, never on the work axis alone. A structure without a breakeven rule is not wrong; it is simply not yet comparable, and the {% include term.html id="boc-policy" text="BOC" %} policy requires comparability among recorded alternatives before selection. [v0.2.0 — CANDIDATE]

---

## 7. Relation to the DCC Hardware Profile and to Existing Canon

This document and the proposed **DCC Hardware Profile schema** are sibling propositions facing opposite directions across the same boundary. The hardware profile describes the *machine side*: the measured {% include term.html id="layer-0" text="Layer 0" %} capabilities — accelerator inventory, memory hierarchy, storage bounds, entropy sources — established during onboarding and referenced by the committed lowering discipline of [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §5.1. This document describes the *artifact side*: the contract surfaces that structures declare regardless of which {% include term.html id="cdqn" %} node executes them. They meet at lowering: the compiler queries the node's hardware profile and must lower each admitted structure in a way that honors its locality and work rows, while the {% include term.html id="simemp-gateway" text="SIMEMP Gateway" %} intercepts any lowering that is not an algebraic isomorphism, protecting {% include term.html id="existential-invariant" text="Existential Invariants" %} and preserving the constant-time properties required by the {% include term.html id="security-envelope" %} ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §5.2). The {% include term.html id="structural-indirection" text="Structural Indirection" %} discipline is what allows the two sides to evolve independently: abstract capability contracts are decoupled from concrete primitives, so structures and hardware can each be swapped without fracturing the other.

The following table records, for each element of this proposal, whether it is already grounded in canon or genuinely new to this document. No element herein redefines existing canon.

| Element | Status | Canon grounding |
|---|---|---|
| CAE-DAG as sole substrate; pointer-heap rejection | Grounded | [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2 (BOC-recorded) |
| Acyclicity by hash pre-image resistance | Grounded | [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §2.4 (Axiom 5) |
| Universal Envelope carriage and immutable lifecycle | Grounded | [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.3, §1.10 |
| Q(dataStruc), Q(patterns), Q(workflows) definitions | Grounded | Glossary (`primitives.yml`); [`docs/qlog.md`]({{ '/qlog.html' | relative_url }}) §2 |
| Workflow as DAG of patterns under Intent Contract | Grounded | [`docs/qlog.md`]({{ '/qlog.html' | relative_url }}) §2 |
| Q(reuse) two-axis consequentiality breakeven | Grounded | [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15, §5.9; [`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §7.2 |
| Memory Wall locality accounting | Grounded | [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.1 |
| Candidate pipeline (measure, select, seal) | Grounded | [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 |
| **Five-row structural contract schema** | **New** | Composes the SIMEMP disciplines per structure |
| **Evaluation ladder (nomination to sealing)** | **New** | Instantiates the §6 pipeline for structures |
| **Proposed Q(array), Q(matrix), Q(table), Q(stream) names** | **New** | Registered in the glossary (`primitives.yml`); include references adopted in this update |
| **Recoverable-contract analysis (chain, ring)** | **New** | First worked pass of the ladder |

---

## 8. Open Questions and Integration Queue

Items 1 and 2 record their discharge; items 3–6 remain in CANDIDATE status. Sealing of any item occurs exclusively through the candidate pipeline of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 — deterministic measurement, {% include term.html id="boc-policy" text="BOC" %} selection, {% include term.html id="qn-rsi" text="Q(rsi)" %} sealing — at implementation time or under a dedicated optimization loop.

1. **Vocabulary registration.** Discharged [v0.2.0 — CANDIDATE]: the four proposed names of Section 5 are registered in the glossary (`primitives.yml`) as data-only commits under the candidate pipeline, and this document now carries their include references and concordance entries; the registration itself was performed by a separate commit, not by this proposal.
2. **Navigation and index ripple.** Discharged [v0.2.0 — CANDIDATE]: the navigation registry entry and the portal index entry are present in the committed state, honoring the one-change-per-commit discipline; no canonical file is modified by this document.
3. **Row measurement of canonical candidates.** The five canonical-admitted families of Section 4 await deterministic measurement at {% include term.html id="qnir" %} lowering boundaries; their work rows are proposals until receipt-backed measurements exist.
4. **Recoverable pair.** The sequenced chain and bounded ring remain open contract shapes; the ladder records their stall reasons so that future nominations — including machine-proposed ones under a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop — can re-enter with findings attached.
5. **Final selections.** Which structures are ultimately admitted depends on implementation feedback and {% include term.html id="qn-rsi" text="Q(rsi)" %} optimization; this framework is proposed as the stable part, and selections are its outputs, revisable only by new versioned artifacts along the {% include term.html id="causal-arrow" %}.
6. **Sibling proposition.** The DCC Hardware Profile schema (Section 7) is proposed as a separate follow-up, anchoring the machine side at [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §7 or [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §3; its placement is an open authorial decision.

---

## 9. License Alignment

This proposal is part of the {% include term.html id="licensed-work" %} and aligns with the {% include term.html id="ssl" text="Scaling Source License (SSL 1.0)" %}:

- **Attribution:** Any implementation or research derivative referencing this document must preserve the canonical {% include term.html id="paternity-reference" %}.
- **Open-Core Research:** Exploratory research and academic derivations of the propositions herein remain royalty-free under Non-Scaling Open Access, governed by the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}.
- **Safe Harbor Integrity:** The structure vocabulary proposed herein is designed to operate as technical infrastructure of the licensed work. It exercises no editorial control over, and acquires no intellectual property in, sovereign human {% include term.html id="payload" text="Payloads" %}.

---

## Measured Ladder Pass — Step 6.1 Container BOC (v0.3.0 — CANDIDATE)

This section records the first receipt-backed filling of the five-row contract schema of §2 for two canonical-admitted candidates of §4. It is the measurement output of Ladder Steps 2–3, not a sealing: the evidence is the committed `6cc1ef1` (v0.3.0-draft.32) trade study in `docs/logs/qnir/step6-1-container-boc-study.md`, whose raw artifact is `container_boc_results.json` (SHA-256 `abed3d5d05788ed7427ebb4df090cbf4265809ee306e72b8e8caa9e73605f36c`, 28 rows, 12/12 enforced-rule checks PASS). Every candidate was measured under the same rules: SHA-256 32-byte leaf and node hashes (`SHA-256(0x00 ‖ key_le ‖ value)` for leaves, a domain-separated pairwise Merkle commitment per internal node), deterministic canonical CBOR as one ascending-key `{u64 => bstr(32)}` map, copy-on-write structural persistence with no mutation in place, and one stable ascending-key normalization of the same 64-update batch. Two warm-up and nine measured trials; cycles are `ns × 1.80` at the declared reference clock. The row figures below are *measurements in ns / cycles / SHA-256 calls / alloc B*; they are diagnostic evidence at execution boundaries and do not by themselves declare a U-denominated worst case or the two-axis breakeven of §6 — those remain contractual proposals for `qir-enc-3` (Wave 4B).

**`Q(array)` = `RRBTree-16`** (persistent 16-way radix vector, positional keys `0..N`; dense niche winner)

| §2 row | Declared contract surface | Measured evidence (N = 16 → 256 → 4096 → 65536) |
|---|---|---|
| 1. Footprint | Declared Metric Envelope ceiling; growth emits a new versioned artifact, never mutates in place | `mem_bytes` 1,752 → 29,400 → 471,768 → 7,549,656 B; canonical CBOR 9,195 B (256) / 151,275 B (4096), identical across all dense candidates; single-key proof 142 → 280 → 416 → 554 B; COW versioning 1,728 → 3,072 → 4,416 → 5,760 alloc B per update |
| 2. Access | Enumerated morphisms only — point `get`, persistent `set` + incremental root, sorted batch `B = 64`, sequential `fold` | `get` 1.6 → 3.1 → 4.8 → 15.5 ns (2.9 → 5.6 → 8.6 → 27.9 cycles; 0 SHA calls); `set` 4,093.8 → 7,818.8 → 11,443.8 → 15,218.8 ns (7,369 → 14,074 → 20,599 → 27,394 cycles; 6 → 11 → 16 → 21 SHA calls); batch 271,500 → 528,400 → 700,100 → 998,000 ns total, 4,242 → 8,256 → 10,939 → 15,594 ns/item (384 → 704 → 1,024 → 1,344 SHA calls); `fold` 21 → 450 → 11,594 → 136,300 ns (1.3 → 1.8 → 2.8 → 2.1 ns/element) |
| 3. Locality | SIMEMP Memory Wall movement class per access pattern, tier-indexed | 1,752/29,400 B in L1, 471,768 B in L2, 7,549,656 B in L3 (≈90 % of the i7-8565U's 8 MiB L3); no DRAM spill at any measured N; `get` rises from 1.6 ns (L1) to 15.5 ns (L3), matching the calibrated tier ladder (L1 3.4 ns → L3 11.2 ns → DRAM 125 ns in QO-17 Part 1) |
| 4. Determinism | Iteration/traversal order fixed by causal index; no runtime-adaptive reordering | `fold` emits keys strictly ascending `0..N` and is byte-stable across repetitions; canonical CBOR digests `11ed127f…d5c1b` (256) and `9f2c068b…80d68` (4096) are identical to the other dense candidates; all structural checks PASS |
| 5. Work | Work per access class in U, versioning amplification, declared breakeven rule | Versioning amplification 1,728 → 5,760 alloc B/update; 6 → 21 SHA calls/update; 384 → 1,344 SHA calls per sorted batch; physical costs above are in ns/cycles, with the U charge and two-axis breakeven rule declared-pending in `qir-enc-3`; no batch amortization was observed (ns/item ≈ set median) |

**`Q(table)` = `MerkleBTree-16`** (persistent sorted B-tree, keys `i * 0x9E3779B97F4A7C15`; sparse niche winner)

| §2 row | Declared contract surface | Measured evidence (N = 16 → 256 → 4096 → 65536) |
|---|---|---|
| 1. Footprint | Declared Metric Envelope ceiling; capacity is declared, growth is re-versioning | `mem_bytes` 2,400 → 33,544 → 528,560 → 8,445,528 B; canonical CBOR 11,003 B (256) / 176,123 B (4096), identical across all sparse candidates; single-key proof 176 → 312 → 448 → 584 B; COW versioning 2,005 → 3,480 → 4,968 → 6,474 alloc B per update |
| 2. Access | Enumerated morphisms only — keyed `get`, persistent `set` + incremental root, sorted batch `B = 64`, key-order `fold` | `get` 6.4 → 12.9 → 26.4 → 54.8 ns (11.5 → 23.2 → 47.5 → 98.6 cycles; 0 SHA calls); `set` 5,512.5 → 11,231.2 → 12,987.5 → 16,956.2 ns (9,923 → 20,216 → 23,378 → 30,521 cycles; 8 → 13 → 18 → 23 SHA calls); batch 330,700 → 656,300 → 803,100 → 1,080,500 ns total, 5,167 → 10,255 → 12,548 → 16,883 ns/item (485 → 804 → 1,123 → 1,449 SHA calls); `fold` 28 → 393 → 8,219 → 172,800 ns (1.8 → 1.5 → 2.0 → 2.6 ns/element) |
| 3. Locality | SIMEMP Memory Wall movement class per access pattern, tier-indexed | 2,400/33,544 B in L1, 528,560 B in L2, 8,445,528 B in DRAM at N = 65,536 (past the i7-8565U's 8 MiB L3); keyed `get` rises 6.4 → 54.8 ns across those tiers, while the three-order-of-magnitude spreading key space rules out a flat contiguous layout |
| 4. Determinism | Iteration/traversal order fixed by causal index; no runtime-adaptive reordering | `fold` emits ascending keys, independent of insertion history or tree rotations; canonical CBOR digests `60d0ddb1…7ab7f3` (256) and `35434ff9…7c54e3` (4096) are identical across all sparse candidates; all structural checks PASS |
| 5. Work | Work per access class in U, versioning amplification, declared breakeven rule | Versioning amplification 2,005 → 6,474 alloc B/update; 8 → 23 SHA calls/update; 485 → 1,449 SHA calls per sorted batch; U charge and two-axis breakeven declared-pending in `qir-enc-3`; comparison to `Q(reuse)` and to the rejected candidates is recorded below on the measured axes only |

**Formal Step-3 rejection record (findings, not erasures).** Under `boc.md` v0.4.2, all four candidates pass Stage 1 with `ImplicitDeps(c) = 0`; at Stage 2 the efficiency component decides the frontier and §8.3 frontier reduction makes the later components non-decision-affecting for dominated candidates. The `ΔW_worst(c)` U-bound remains `UNKNOWN` until the container opcode cost model is declared, so the outcome below is an explicitly recorded adjudication rather than a fabricated Tier-2 key or a sealing.

| Candidate | Niche(s) | Measured rejection ground | Disposition |
|---|---|---|---|
| `HAMT` | both | Lookup **30×–300×** slower than the dense winner (490.9 vs 1.6 ns at 16; 662.1 vs 15.5 ns at 65,536) and 12×–80× slower than the keyed winner, from one mandatory SHA-256 key digest per `get`; deterministic `fold` ~**50×** worse at scale (130.4 vs 2.6 ns/element; 8,547,000 vs 172,800 ns at 65,536) because key order must be restored by sorting; **18.85–18.90 MB** footprint spills past the 8 MiB L3 into DRAM (2.2–2.5× the winners), with proofs to 672 B | Rejected, recorded as a finding; revisitable only with a different commitment layout or a niche whose order is genuinely unordered |
| `ChunkedFlat` | both | From **N ≥ 4096** the 32-entry chunk clone copies keys, values and Merkle levels: 5,808 → 7,056 alloc B/update vs 4,416 → 5,760; set 15,275 → 21,194 ns vs 11,444 → 15,219; batch 15.16 → 20.63 µs/item vs 10.94 → 15.59; lookup 21.7 → 58.1 ns vs 4.8 → 15.5 | Rejected on the update/copy penalty; revisitable with a declared different chunk size |

**§8 item 3 discharge.** Item 3 ("Row measurement of canonical candidates") is **discharged for `Q(array)` and `Q(table)` only**: all five rows above are filled with deterministic, commit-state, receipt-adjacent measurements, and the Step-3 selections and rejections are recorded in the linking log. `Q(matrix)` and `Q(stream)` retain their §8 item 3 status — no row measurement for them is claimed here. The "record tuple" and "persistent tree" families of §4 were not part of this workload contract. Final QnIR core extension admission is **Wave 4B**, and the `qir-enc-3` container opcode specification plus the `Q1`/`Q3` residency contract are the next wave; no structure in this section is sealed, no glossary or canon file is changed, and no new term include is introduced. [v0.3.0 — CANDIDATE]
