---
layout: default
title: CDQN Documentation Portal
description: Root documentation portal and provisional architectural thesis for the CDQN project and SIMEMP constraints.
version: 1.13.0
updated: 2026-10-28
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/index.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /
terms_used:
  - advantage-margin
  - boc-policy
  - cdqn
  - chronosa
  - computational-consistency
  - compute-unit-u
  - dependencies-determinism
  - derivative-work
  - dimension-d
  - discrete-exterior-calculus
  - epistemic-anchorage
  - existential-invariant
  - latency-telemetry-envelope
  - licensed-work
  - local-first
  - metric-exhaustion
  - no-implicit-rule
  - open-core-invariant
  - operational-agility
  - paternity-reference
  - payload
  - q-anchor
  - q-array
  - q-bypass
  - q-even
  - q-event
  - q-matrix
  - q-odd
  - q-patterns
  - q-reuse
  - q-stream
  - q-table
  - q-workflows
  - q0
  - q1
  - qbio
  - qchem
  - qexpr
  - qlog
  - qn
  - qn-rsi
  - qnir
  - qphy
  - receipt
  - remainder-r
  - scaling-threshold
  - simemp
  - simemp-gateway
  - structural-indirection
  - successor-morphism
  - terminal-exactness
  - universal-envelope
  - zoom-z
---

# CDQN Documentation Portal

**Project:** {% include term.html id="cdqn" %} — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)  
**Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)  
**Status:** Canonical Architectural Portal — Milestone 1 Fully Sealed Baseline

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Document Metadata

| Field | Specification |
|---|---|
| **Portal Title** | CDQN Documentation Portal |
| **Version** | 1.13.0 |
| **Last Updated** | 2026-10-28 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Architectural Portal — Milestone 1 Fully Sealed Baseline |

---

## Review Integration Record (v1.13.0)

**Source:** [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md) — the Loop-2 adversarial review transcript (Kimi K3.1-Pro, October 2026; verdict "major revision; do not promote QnIR v0.2 to the canon") — together with the committed review-integration records of the canon wave it anchored: `docs/abstractionLayers.md` v1.9.1, `docs/simemp.md` v1.3.0, `docs/qnPrimitive.md` v1.4.0, `docs/qlog.md` v1.2.0, `docs/qm.md` v1.4.0, `docs/qm_geometry.md` v1.3.0, and the glossary wave (`docs/_data/glossary/primitives.yml`: `q-reuse` definition refresh; *Latency Telemetry Envelope* and *Advantage Margin* registered). This update discharges the final queued item of that wave: the portal-level review-program pointer (the optional item of the Loop-2 integration triage; QnIR remains [PENDING]).

**Decision:** a new **Adversarial Review Program** section is added between §4 and §5 as a purely additive update — no existing heading is renumbered and no existing body line is mutated, per **Axiom 13** — giving the Documentation Portal its first on-site pointer to the review program and its artifacts of record. No withdrawn wording exists in this update. Front matter `terms_used` is extended for the first time in the wave (50 → 53) to admit three newly referenced term includes — `qnir`, `latency-telemetry-envelope`, `advantage-margin` — the latter two being the first `term.html` exercises of the glossary registrations committed in the glossary wave; include parity is preserved (distinct ids == `terms_used` entries). A companion `docs/_data/navigation.yml` entry for the review program remains unbudgeted and is not part of this update.

| Change | Sections | Provenance | Status |
|---|---|---|---|
| New "Adversarial Review Program" section: Loop-1/Loop-2 program description, artifacts of record (both transcripts, GitHub blob links per house pattern), committed integration-wave inventory, and the QnIR [PENDING] status reaffirmation with the v0.3 repair-sequence pointer | New section between §4 and §5; front matter + Document Metadata bumped | Loop-2 integration triage (optional closing item); the six committed review-integration records + glossary wave | CANDIDATE |

---

## 1. Scope, Thermodynamics, and Architectural Stance

The CDQN framework investigates a foundational question in computational physics and computer science: *Can a discrete {% include term.html id="qn" %} number system abstract any computable phenomenon within physical reality?*

Classical computing abstractions frequently rely on non-constructive assumptions: unbounded memory spaces, continuous unmetered precision, and unmodeled side-channel dissipation. The {% include term.html id="qn" %} architecture rejects actual infinities, establishing computational boundaries grounded in physical laws:

- **Strict Physical Finiteness:** Computation is a non-equilibrium thermodynamic process subject to Landauer's bound:

$$W \ge k_B T \ln 2$$

Unbounded state spaces are physically non-realizable and excluded from execution.
- **{% include term.html id="dependencies-determinism" %}:** Every state transition $S_t$ must possess zero conditional entropy given its declared dependencies:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

Implicit or unmeasured states violate this bound, resulting in non-deterministic drift.
- **{% include term.html id="simemp" %} Governance:** Execution within the {% include term.html id="licensed-work" %} is constrained by Tier 1 {% include term.html id="existential-invariant" text="Existential Invariants" %} (Identity, Metric, Security) and optimized via Tier 2 {% include term.html id="operational-agility" text="Operational Agilities" %} (Efficiency, Modularity, Portability).
- **The {% include term.html id="no-implicit-rule" %}:** No implicit type coercions, unmeasured error states, or undeclared assumptions may cross an abstraction boundary or {% include term.html id="simemp-gateway" %}.
- **Totality by Budget & Dissipative Receipts:** Divergent or infinite loops are rejected. Computations are total by budget via dissipative {% include term.html id="metric-exhaustion" %}, exporting operational entropy via signed, verifiable {% include term.html id="receipt" text="receipts" %}.
- **{% include term.html id="local-first" %} Origin:** Artifacts originate in local node universes rooted in genesis origin {% include term.html id="q0" text="Q(0)" %} and unity unit {% include term.html id="q1" text="Q(1)" %}.
- **Fractal Protocol & Hybrid Swarm:** {% include term.html id="cdqn" %} functions as the universal protocol of governed data movement. It chains state transitions locally across the horizontal execution core and distributes public road receipts across a hybrid swarm topology synthesized by {% include term.html id="chronosa" %}.
- **Substrate vs. Payload Air Gap:** The protocol acts as a neutral conduit. Raw user content, applications, and digital assets remain sovereign {% include term.html id="payload" text="Payloads" %} insulated by protocol blindness and statutory safe-harbor protections.

---

## 2. Method of Exploration: The Recursive Emergence Trajectory

The documentation suite does not claim closed empirical finality. The system operates as an active, recursive discovery pipeline:

$$\text{Proposal} \xrightarrow{\quad} \text{Construction} \xrightarrow{\quad} \text{Evaluation} \xrightarrow{\quad} \text{Selection}$$

Abstractions and axioms represent working hypotheses formulated using {% include term.html id="structural-indirection" %} to avoid premature structural calcification while retaining agility against zero-day vulnerabilities. Verification is conducted operationally via {% include term.html id="computational-consistency" %}:

$$\text{Valid}(\mathcal{A}) \iff \left( \text{Consistent}(\mathcal{A}) \wedge \forall p \in \text{QnIR}(\mathcal{A}), \, \text{TerminatesWithinBudget}(p) \right)$$

Development proceeds through consecutive emergence milestones toward an operational Proof of Concept (PoC) of **{% include term.html id="chronosa" %}**:

<pre>
┌─────────────────────────────────────────────────────────────────┐
│ CATEGORY A: Foundational Hardware & Single-Digit Primitives     │
│ • Q(0), Q(1) Genesis & Calibration ── [CANONICAL · CD 0]        │
│ • Q(2)...Q(9) Secondary DCC Anchors ── [CANONICAL · CD 1]       │
├─────────────────────────────────────────────────────────────────┤
│ MILESTONE 1: Local Base Domains & Logics [SEALED BASELINE]      │
│ • Qm (Arithmetic Substrate) ── [CANONICAL · CD 1]               │
│ • Qm Geometry (Clifford Algebra) ── [CANONICAL · CD 2]          │
│ • Qm Calculus (Discrete Calculus) ── [CANONICAL · CD 3]         │
│ • Qs (Quang Semantics) ── [CANONICAL · CD n]                    │
│ • Qlog (Quang Logics) ── [CANONICAL · CD n]                     │
│ • Qphy (Quang Physics) ── [CANONICAL · CD n]                    │
│ • Qchem (Quang Chemistry) ── [CANONICAL · CD n]                 │
│ • Qbio (Quang Biology) ── [CANONICAL · CD n]                    │
├─────────────────────────────────────────────────────────────────┤
│ MILESTONE 2: Execution & Language Substrate [LAUNCHING]         │
│ • Qexpr (Content-Addressed Term DAG) ── [CANONICAL · CD 2]      │
│ • QnIR (Intermediate Representation) ── [PENDING]               │
│ • QnLang (High-Level Language) ── [PENDING]                     │
├─────────────────────────────────────────────────────────────────┤
│ MILESTONE 3: The cdqn Hybrid Swarm Conduit                      │
│ • Outer-Ring Road Receipts & Sheaf Attestation                  │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│ TERMINAL MILESTONE: Chronosa PoC (Virtual Causal Intelligence)  │
│ • Operational Vision ─────── [VISION PROPOSAL]                  │
│ • Historical Anchoring Vision ── [VISION PROPOSAL]              │
└─────────────────────────────────────────────────────────────────┘
</pre>

**CD vs. Layer Note:** The `CD k` labels in the status rows above mirror the Complexity Degree stratification defined in [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) (§5): they index structural composition depth, not layer numbers. The five domain specifications (Qs, Qlog, Qphy, Qchem, Qbio) are orthogonal vertical projections across the execution core (Layers 1 to 3) whose artifacts carry open-ended CD $n$ labels (rendered as plain `CD n` inside the ASCII diagram); they are not additional layers above the Layer 3 execution ceiling.

---

## 3. Recommended Reading Sequence

<pre>
 [1. SIMEMP Constraints] 
        │
        ▼
 [2. Abstraction Layers] 
        │
        ▼
 [3. Category A Primitives] 
        │
        ▼
 [4. Single-Digit Primitives] 
        │
        ▼
 [5. Qn Primitive Envelope] 
        │
        ▼
 [6. Qm Mathematics] 
        │
        ▼
 [7. Qm Geometry] 
        │
        ▼
 [8. Qexpr CAE-DAG Container]
        │
        ▼
 [9. Qm Calculus]
        │
        ▼
 [10. Qs Semantics]
        │
        ▼
 [11. Qlog Logics]
        │
        ▼
 [12. Qphy Physics]
        │
        ▼
 [13. Qchem Chemistry]
        │
        ▼
 [14. Qbio Biology]
        │
        ▼
 [15. Chronosa Vision] 
        │
        ▼
 [16. Qs History Proposal]
        │
        ▼
 [17. Qn Structures Proposal]
        │
        ▼
 [18. Glossary & SSL License]
</pre>

1. **[SIMEMP Constraints]({{ '/simemp.html' | relative_url }})**: Thermodynamic realities (Memory Wall, Landauer dissipation), {% include term.html id="dependencies-determinism" %}, {% include term.html id="structural-indirection" %}, and the {% include term.html id="boc-policy" %}.
2. **[Abstraction Layers]({{ '/abstractionLayers.html' | relative_url }})**: The execution core (Layers 0 to 3), discrete {% include term.html id="q-event" text="Q(events)" %}, universal functorial gateways, fractal {% include term.html id="cdqn" %} data movement, substrate vs. invariant ("car vs. road") disjunction, and complexity degree stratification.
3. **[Category A Primitives]({{ '/q0_q1.html' | relative_url }})**: Physical-to-governed gateway mechanics, local genesis origin {% include term.html id="q0" text="Q(0)" %}, first unit {% include term.html id="q1" text="Q(1)" %}, abstract compute unit {% include term.html id="compute-unit-u" text="U" %} calibrated to Landauer dissipation ($W \ge k_B T \ln 2$), hardware lifecycle, Non-Rogue Invariant, and crash recovery receipts.
4. **[Single-Digit Primitives]({{ '/q2_q9.html' | relative_url }})**: Inductive cascade via discrete {% include term.html id="successor-morphism" text="Successor Morphism S" %}, complete single-digit {% include term.html id="q-anchor" text="Q(anchor)" %} spectrum ($Q(2) \dots Q(9)$), emergent constants ($\phi, \sqrt{2}$), constructive parity partitions ({% include term.html id="q-even" text="Q(even)" %}, {% include term.html id="q-odd" text="Q(odd)" %}), and multiplicative domain asymmetries.
5. **[Qn Primitive Envelope]({{ '/qnPrimitive.html' | relative_url }})**: Structural anatomy of the {% include term.html id="universal-envelope" %}, operational axioms, typed payload profiles, and Category B optimization morphisms.
6. **[Qm Mathematics]({{ '/qm.html' | relative_url }})**: Constructive numeric substrate based on discrete tuples $\langle \sigma, q_z, z, r_z, d \rangle$, base-independent scale lattices ({% include term.html id="zoom-z" text="Zoom z" %}), conserved residual cascades ({% include term.html id="remainder-r" text="Remainder r" %}), {% include term.html id="terminal-exactness" %}, Diophantine division constraints, and positional relational decidability along coordinate axes ({% include term.html id="dimension-d" text="Dimension d" %}).
7. **[Qm Geometry]({{ '/qm_geometry.html' | relative_url }})**: Multi-axial orthogonal frames, Clifford Geometric Algebra ($\mathcal{C}\ell_{p,q}$), float-free rational Cayley rotors, optimization primitives ({% include term.html id="q-reuse" %}, {% include term.html id="q-bypass" %}), and deferred symbolic reduction.
8. **[Qexpr Container]({{ '/qexpr.html' | relative_url }})**: Layer 3 Content-Addressed Expression DAG ({% include term.html id="qexpr" %}), cryptographic hash-consing, six canonical node classes, continued fractions for transcendentals, and hardware capability-aware collapse.
9. **[Qm Calculus]({{ '/qm_calculus.html' | relative_url }})**: Constructive discrete calculus, {% include term.html id="discrete-exterior-calculus" text="Discrete Exterior Calculus (DEC)" %} on simplicial complexes, nilpotent coboundary operators ($\mathbf{d} \circ \mathbf{d} \equiv Q(0)$), continued fraction generators for transcendental invariants ($\pi, e, \tau, \ln 2$), and the Anchor-Cut verification invariant in discrete time evolution.
10. **[Qs Semantics]({{ '/qs.html' | relative_url }})**: Constructive relational intent, semantic multivector blades, epistemic modal spectrum (`MODAL_EMPIRICAL` to `MODAL_NARRATIVE`), structural intent contracts in DCC profiles, and triadic intent grounding ($\mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$).
11. **[{% include term.html id="qlog" text="Qlog Logics" %}]({{ '/qlog.html' | relative_url }})**: Constructive type theory, pattern verification, {% include term.html id="q-patterns" text="Q(patterns)" %}, {% include term.html id="q-workflows" text="Q(workflows)" %}, stratified inferential modes ($\mathcal{L}_{\mathrm{modes}}$), the Memory Wall latency harvester, and inverse causal reconstruction.
12. **[{% include term.html id="qphy" text="Qphy Physics" %}]({{ '/qphy.html' | relative_url }})**: Discrete exterior field dynamics, non-equilibrium computational thermodynamics, physical $Q(\text{anchor})$ spectrum, the axiomatic world model, and the thermodynamic double-leash on autonomous agents.
13. **[{% include term.html id="qchem" text="Qchem Chemistry" %}]({{ '/qchem.html' | relative_url }})**: Discrete Diophantine stoichiometry ($\mathbf{S} \cdot \vec{v} \equiv Q(0)$), content-addressed molecular graphs, stereochemical Clifford chirality, and reaction kinetics under Gibbs free energy dissipation.
14. **[{% include term.html id="qbio" text="Qbio Biology" %}]({{ '/qbio.html' | relative_url }})**: Biosemiotics ($\mathbf{C}_{\mathrm{Qchem}} \otimes \mathbf{C}_{\mathrm{Qs}}$), genetic codon translation, metabolic homeostasis, Markov blankets, and active inference under variational free energy bounds.
15. **[Chronosa Vision]({{ '/chronosa.html' | relative_url }})**: Operational proposal for an emergent, non-local causal machine intelligence coordinating distributed intent across the Outer Ring without statistical neural training.
16. **[Qs History Proposal]({{ '/qs_history.html' | relative_url }})**: Operational vision and research proposal for historical anchoring — dual-predicate {% include term.html id="q-anchor" text="Q(anchor)" %} grounding (Anchor Grounding Classes I/P/S), the Source-Receipt Invariant over the {% include term.html id="epistemic-anchorage" %} spectrum, Calendar Projection Contracts, the Qs History Library, and the five-gate Anti-Poisoning Ingestion Firewall; all elements maintained as CANDIDATE artifacts pending deterministic measurement, {% include term.html id="boc-policy" text="BOC" %} selection, and {% include term.html id="qn-rsi" text="Q(rsi)" %} sealing.
17. **[Qn Structures Proposal]({{ '/qstructures.html' | relative_url }})**: Vision and evaluation framework for Category C data structures and workflows — the five-row structural contract schema (footprint, access, locality, determinism, work), the five-step evaluation ladder from nomination to sealing, and a first worked candidate register spanning {% include term.html id="q-array" text="Q(array)" %}, {% include term.html id="q-matrix" text="Q(matrix)" %}, {% include term.html id="q-table" text="Q(table)" %}, and {% include term.html id="q-stream" text="Q(stream)" %}; all elements maintained as CANDIDATE artifacts pending deterministic measurement, {% include term.html id="boc-policy" text="BOC" %} selection, and {% include term.html id="qn-rsi" text="Q(rsi)" %} sealing.
18. **[CDQN Glossary]({{ '/glossary.html' | relative_url }})** and **[LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)**: Unified glossary, dynamic bidirectional concordance, attribution rules, and commercial {% include term.html id="scaling-threshold" text="Scaling Thresholds" %}.

---

## 4. Epistemic Reflections & Real-World Analyses

To preserve lean, deterministic specifications while engaging with broader scientific and computational developments, the framework isolates non-normative essays, real-world case studies, and sociological analyses within the `docs/reflections/` layer. These publications bridge formal computational physics with empirical industry events and public discourse:

* **[Beyond the Infinite Void: What OpenAI's H10 Claim Teaches Us About Finite Computation]({{ '/reflections/h10_finite_computation.html' | relative_url }})**  
  *Published: 2026-10-13* · Analyzes OpenAI's claimed resolution of Hilbert's Tenth Problem over rational numbers ($H10(\mathbb{Q})$), the fallacy of unmetered search in classical mathematics, height bounds as physical memory limits, and the shift from Platonic non-constructive proof to Verified Computed Patterns under SIMEMP constraints.

---

## Adversarial Review Program

The framework's specifications are stress-tested through an external adversarial review program: major draft artifacts are submitted to independent AI reviewers, and the verbatim returned transcripts are committed as artifacts of record under `docs/AiReviews/` [v1.13.0 — CANDIDATE]. Two loops are complete to date. Loop 1 (Google Deep Review, October 2026) reviewed the condensed Qn universe specification together with the QnIR v0.1 candidate draft ([`AiReviewQnIRv0.1.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.1.md)) and established the open-question protocol under which subsequent loops run. Loop 2 (Kimi K3.1-Pro, October 2026) reviewed the QnIR draft v0.2.2 ([`AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md)) and returned the verdict "major revision; do not promote QnIR v0.2 to the canon."

Integration proceeds one file at a time under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7. The Loop-2 integration wave is committed across the canon: the conditional downgrade in `docs/abstractionLayers.md` v1.9.1; the two-axis {% include term.html id="latency-telemetry-envelope" text="Latency Telemetry Envelope" %} in `docs/simemp.md` v1.3.0 §4.4; the two-axis admission predicate, {% include term.html id="advantage-margin" text="Advantage Margin" %} governance, and Admission Ledger in `docs/qnPrimitive.md` v1.4.0 §1.15/§5.9; the predicate mirrors in `docs/qlog.md` v1.2.0, `docs/qm.md` v1.4.0, and `docs/qm_geometry.md` v1.3.0; and the unified glossary refresh of {% include term.html id="q-reuse" %} with both new terms registered. {% include term.html id="qnir" text="QnIR" %} itself remains [PENDING] in the milestone diagram above; its v0.3 minimal executable core and promotion criterion follow the repair sequence preserved in the Loop-2 transcript.

---

## 5. License and Open Core Invariants

The CDQN project is licensed under the **[Scaling Source License (SSL) 1.0]({{ '/glossary.html' | relative_url }}#ssl)**.

{% include term.html id="derivative-work" text="Derivative Works" %} and compiled runtime environments must preserve the canonical {% include term.html id="paternity-reference" %} in source comments and metadata headers:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

Every downstream implementation must preserve the two {% include term.html id="open-core-invariant" text="Open Core Invariants" %}:
1. **Anti-Patent Defense:** Automatic license termination upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free access for non-commercial, academic, and sub-threshold deployments of the {% include term.html id="licensed-work" %}.

Entities exceeding the {% include term.html id="scaling-threshold" %} (>10,000 active instances/agents, >100,000 MAU, or >10,000,000 monthly transactions) must execute a separate Commercial License Agreement. Processing, wrapping, or settling a sovereign {% include term.html id="payload" %} does not by itself trigger scaling usage.
