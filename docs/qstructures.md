---
layout: default
title: Qn Structures — Vision and Evaluation Framework for Category C Data Structures and Workflows
description: Structural proposal for the Category C layer — a five-row contract schema and an evaluation ladder for Qn data structures and workflows, with a worked candidate register grounded in SIMEMP constraints and BOC selection. All artifacts herein are propositions pending deterministic measurement, BOC selection, and Q(rsi) sealing.
version: 0.1.0
updated: 2026-10-23
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qstructures.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qstructures.html
status: Vision and Structural Proposal — Not a Finalized Specification
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
  - q-bypass
  - q-datastruc
  - q-patterns
  - q-reuse
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
| **Version** | 0.1.0 |
| **Last Updated** | 2026-10-23 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Vision and Structural Proposal — Not a Finalized Specification |

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

The Qn universe already names operational morphisms and structure families in a consistent `Q(...)` pattern: {% include term.html id="q-reuse" %}, {% include term.html id="q-bypass" %}, {% include term.html id="qn-rsi" %}, and the registered trio {% include term.html id="q-datastruc" %}, {% include term.html id="q-patterns" %}, {% include term.html id="q-workflows" %}. It is proposed that the structure vocabulary extend in the same pattern with the four names used in Section 4 — `Q(array)`, `Q(matrix)`, `Q(table)`, `Q(stream)` — under one consistency rule: **a `Q(...)` name denotes a contract surface (the five rows of Section 2), never a runtime shape**. This rule is what keeps the nomenclature honest across heterogeneous hardware: the same `Q(array)` contract is satisfiable by different lowerings on different nodes, exactly as the compute unit evaluates identically across all nodes while local entropy parameterizes identity.

Deliberately, this document **registers nothing**. Glossary entries are data-only commits under the candidate pipeline, and the four proposed names remain plain text herein — they receive no include references and no concordance — until a separate registration commit follows the pipeline, as was done for the consultation vocabulary on 2026-10-22. The deferral is queued in Section 8.

---

## 6. Breakeven Rules and the Work Row

The work row of Section 2 has one committed precedent whose shape every admitted structure should follow: {% include term.html id="q-reuse" %} is admissible if and only if the predicted computational work of direct re-evaluation exceeds lookup and retrieval dissipation plus a declared overhead term ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15). That inequality is a *declared breakeven rule* — a contract row, not a runtime heuristic — and it is measured in units of the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}.

Two properties make U the right unit for comparable work rows across heterogeneous nodes. First, platform invariance: while local entropy parameterizes cryptographic identity, the metric magnitude of U evaluates identically across all nodes ([`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §4.2), so a work row declared on one node is interpretable on every node. Second, the thermodynamic leash: unbounded recursive execution is physically prohibited, every workflow operates within a finite, non-renewable work budget, and metric depletion halts execution deterministically under {% include term.html id="metric-exhaustion" text="metric exhaustion" %} — so no structure's work row can be rendered moot by unmetered iteration.

It is proposed, therefore, that every admitted structure declare in its work row: (i) the work class per access morphism, (ii) the versioning amplification — the cost emitted by row 1's growth-as-versioning discipline — and (iii) the breakeven inequality under which the structure is preferred over its recorded alternatives. A structure without a breakeven rule is not wrong; it is simply not yet comparable, and the {% include term.html id="boc-policy" text="BOC" %} policy requires comparability among recorded alternatives before selection.

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
| Q(reuse) breakeven inequality | Grounded | [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15; [`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §7.2 |
| Memory Wall locality accounting | Grounded | [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.1 |
| Candidate pipeline (measure, select, seal) | Grounded | [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 |
| **Five-row structural contract schema** | **New** | Composes the SIMEMP disciplines per structure |
| **Evaluation ladder (nomination to sealing)** | **New** | Instantiates the §6 pipeline for structures |
| **Proposed Q(array), Q(matrix), Q(table), Q(stream) names** | **New** | Plain-text vocabulary; registration deferred |
| **Recoverable-contract analysis (chain, ring)** | **New** | First worked pass of the ladder |

---

## 8. Open Questions and Integration Queue

All items below remain in CANDIDATE status. Sealing of any item occurs exclusively through the candidate pipeline of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 — deterministic measurement, {% include term.html id="boc-policy" text="BOC" %} selection, {% include term.html id="qn-rsi" text="Q(rsi)" %} sealing — at implementation time or under a dedicated optimization loop.

1. **Vocabulary registration.** The four proposed names of Section 5 await a data-only glossary commit; this document deliberately registers nothing itself, and until registration the names remain plain text with no concordance entries.
2. **Navigation and index ripple.** One navigation entry and one portal index entry are proposed for a follow-up commit, honoring the one-change-per-commit discipline; no canonical file is modified by this document.
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
