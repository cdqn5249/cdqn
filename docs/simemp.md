---
layout: default
title: SIMEMP Constraints
description: Foundational physical, computational, and architectural constraints governing the Qn and cdqn stack under Dependencies Determinism.
version: 1.3.0
updated: 2026-10-28
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/simemp.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /simemp.html
terms_used:
  - simemp
  - qn
  - cdqn
  - qnlang
  - qnir
  - dcc-profile
  - boc-policy
  - existential-invariant
  - operational-agility
  - causal-arrow
  - scaling-threshold
  - computational-consistency
  - receipt
  - ssl
  - dependencies-determinism
  - structural-indirection
  - metric-exhaustion
  - compute-unit-u
  - no-implicit-rule
  - paternity-reference
  - open-core-invariant
  - simemp-gateway
  - payload
---

# SIMEMP: Thesis and Constraints for the Qn and cdqn Stack

| Field | Specification |
|---|---|
| **Document Title** | SIMEMP: Thesis and Constraints for the Qn and cdqn Stack |
| **Version** | 1.3.0 |
| **Last Updated** | 2026-10-28 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Constitutional Specification — Foundational Invariants |

---

## Review Integration Record (v1.2.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.1.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.1.md) — transcript of the external adversarial review (Google Deep Review, October 2026) of the condensed Qn universe specification and QnIR v0.1 candidate draft. This update integrates the constitutional findings of that review: the decidability of the two-tier hierarchy and of the BOC policy (`QO-1`, Segment 1 Point 3), the decoupling of the logical compute unit from physical thermodynamics (`QO-2`, Segment 1 Point 2), the observational-determinism operationalization of the conditional-entropy postulate (Segment 1 Point 1), and the gateway billing-attribution and migration-witnessing findings (Segment 1 Point 4 — double-flagged: the same point surfaced independently in the data-loss audit as the missing free-generation / metered-crossing statement). It also executes the site-wide rendering decision recorded in the Rendering Fix Record of `docs/abstractionLayers.md` v1.8.1: all mathematical notation in this file is re-encoded into engine-independent plain text, because the site's Jekyll pipeline ships no MathJax/KaTeX engine. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place. New constructions introduced below are tagged **[v1.2.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Two-stage constrained lexicographic BOC model**: Tier 1 as a hard boolean admission filter (`C_valid`, failure → `REJECTION_BY_GATEWAY`, inadmissible candidates never ranked); Tier 2 as lexicographic minimization of `LexKey(c) = ( ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)) )` with the priority permutation `π ∈ S_3` declared in the calling DCC profile; **rubric mode** requiring tabulated Tier-1 pass/fail plus Tier-2 trade-off vectors wherever quantities remain qualitative; the unparameterized scalar `argmax` formulation retired | §7 (rewritten) | QO-1 / Segment 1 Point 3 | CANDIDATE |
| **Observational Determinism Invariant**: bit-identical output commitments (`H(S_t(E_1)) = H(S_t(E_2))`) and identical terminal receipt statuses for any two Layer-0 environments agreeing on the declared dependency projections (`Π_D(E_1) = Π_D(E_2)`), making the incomplete-dependency defect `D' ⊂ D` empirically falsifiable via differential execution testing; the conditional-entropy postulate is additionally annotated as a functional-purity statement | §1 | Segment 1 Point 1 | CANDIDATE |
| **Abstract Compute Unit U decoupled from physical energy**: U redefined strictly as a platform-invariant logical work quantum (normalized so Q(0) → Q(1) costs exactly 1 U); the Landauer bound retained as a theoretical floor with numeric anchor (`2.87 × 10^-21 J` per bit at T = 300 K) and the `10^7`–`10^10` CMOS scale gap acknowledged; new **Layer-0 Hardware Telemetry Envelope** (`E_phys ≤ α_compute · U_op + β_bus · U_mem`, declared error bars) bound to U exclusively via node-local Historical Calibration Benchmark receipts | §4.4 (new), §4.1 | QO-2 / Segment 1 Point 2 | CANDIDATE |
| **Receipt entropy-export reframing**: a receipt compresses epistemic/audit state for an external verifier; physically it is irreversible work that increases Layer-0 internal dissipation — the Prigogine balance is an accounting analogy, not a thermodynamic identity | §4.2 | QO-2 / Segment 1 Point 2 (validity issue 3) | CANDIDATE |
| **Gateway billing attribution**: static DCC-declared verification fee `W_gate > 0` deducted from the invoking principal's remaining work budget before any candidate inspection, charged regardless of validation outcome — closing the free-riding metric-exhaustion (denial-of-service) vector at the boundary; explicit free-generation / metered-crossing statement added | §6 | Segment 1 Point 4 (a); data-loss audit (double flag) | CANDIDATE |
| **Gateway soundness scope**: gateways enforce structural/resource compliance (contract soundness, resource bounds, float prohibition), not informal-to-formal semantic intent alignment; semantic fidelity is governed exclusively by the `docs/qs.md` §6.1 round-trip co-signing handshake | §6 | Segment 1 Point 4 (b) | CANDIDATE |
| **Migration witnessing under structural indirection**: algorithm-tagged (multihash-style) self-describing digests; migration receipts record superseded/successor algorithm identifiers; historical commitments preserved by re-witnessing (re-digest and re-seal anchored to the original commitments) | §4.3 | Segment 1 Point 4 (c) | CANDIDATE |
| **Terminal-status registry acknowledgment**: the five constitutional lifecycle terminals cross-referenced to the sealed 12-status registry of `docs/abstractionLayers.md` §3.6 as the single source of truth | §5.2 | Data-loss audit of this file | CANDIDATE |
| Prior-art positioning against resource-metered bytecodes (eBPF, deterministic Wasm), content-addressed stores (Unison, egg/egglog), zkVM receipts (RISC Zero, Cairo), and exact arithmetic (Posits, iRRAM) deferred to the QnIR v0.2 candidate and the regenerated condensed specification | — | Segment 1 Point 5 | DEFERRED |
| Rendering: all 7 display-math blocks and 5 inline math spans re-encoded to engine-independent plain text (Unicode operators; code spans for every token containing subscripts, superscripts, or pipes), matching the `docs/abstractionLayers.md` v1.8.1 convention | file-wide | Site-wide rendering finding (Rendering Fix Record, `docs/abstractionLayers.md` v1.8.1) | EDITORIAL |
| Editorial box fix: §4.3 structural-indirection diagram, middle box row 2 (`Dilithium3` instantiation) was 1 column wider than its own borders — a pre-existing canon defect of v1.1.1, same class as the fixes recorded in `docs/qexpr.md` v1.3.0 and `docs/qs.md` v1.3.0; re-padded to the 31-column interior with row text preserved byte-for-byte | §4.3 | Pre-existing editorial defect | EDITORIAL |
| Normative References: `docs/qphy.md` added for Landauer floor enforcement and telemetry anchoring; front matter `terms_used` gains `compute-unit-u`; §3.2 cross-references the mechanical BOC procedure | Normative References, front matter, §3.2 | Editorial consequence of §4.4 and §7 | EDITORIAL |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline): QnIR v0.2 — binds `W_gate` into its budget accounting (the caller's remaining budget `W_rem`), adopts the two-stage BOC model in its design verdicts, inherits the 1 U normalization for its static cost table, and carries the deferred prior-art positioning; `docs/qphy.md` — names the Layer-0 calibration-benchmark anchors at its next versioned update (the condensed specification prototypes `Q(Zbus)` / `Q(th2)`, which are not yet canon); `docs/qs.md` — §6.1 remains the sole semantic-alignment path referenced by §6; `docs/index.md` — the SSL scaling-threshold table there quotes 3 of 5 thresholds (a touch-up may ride along a later index update); glossary — the `boc-policy` and `compute-unit-u` entries refresh at the queued glossary update.

## Review Integration Record (v1.3.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md) — verbatim transcript and artifact of record of the Loop-2 adversarial review by Kimi K3.1-Pro (October 27th, 2026) of the QnIR v0.2 candidate draft v0.2.2 — together with an author field report on the admission policy of the Tier-2 optimization morphisms: the single breakeven constant of the current `Q(reuse)` lookup predicate cannot be smart across heterogeneous hardware, because every machine reports different lookup timings and every lookup's cost varies with the memory tier it touches and with machine condition; reuse should therefore be admitted only when retrieval is **consequently faster** than direct compute, so that the timing of the memory-wall delay is fully exploited rather than suffered. This update integrates the constitutional half of that joint finding; the operational predicate and its margin policy land in `docs/qnPrimitive.md` at its next versioned update. It answers `QO-17` ("Time and energy require separate coefficients: a sequence that is worst in cycles need not be worst in joules") by declaring the latency twin of the §4.4 telemetry envelope, and it applies F1's declared-bound discipline ("Replace every runtime quantity by a declared immediate/header bound") to admission itself: runtime-sampled latency is inadmissible as a predicate input. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place. New constructions introduced below are tagged **[v1.3.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Latency Telemetry Envelope (twin axis of §4.4)**: the Layer-0 Hardware Telemetry Envelope is declared two-axis; the latency axis carries a DCC-declared per-tier worst-case lookup bound table `τ_t = { τ_reg, τ_L1, τ_L2, τ_L3, τ_DRAM, τ_storage, τ_remote }` (cycles against the node's declared reference clock), calibrated at Layer-0 onboarding through the same **Historical Calibration Benchmark receipt** class as the energy coefficients; runtime-sampled latency is never an admissible predicate input (Observational Determinism Invariant; `H(S_t \| D(S_t)) = 0` preserved by construction); the never-feeds-back rule of the energy axis extends to the latency axis | §4.4 (new) | `QO-17` (separate time/energy coefficients); F1 declared-bound discipline; author field report (machine heterogeneity + tier-dependent lookup timing) | CANDIDATE |
| **Consequentiality constraint on optimization morphisms**: a memory-locality optimization (the `Q(reuse)`/`Q(bypass)` harvesting classes) is admissible only when its advantage is declared on both axes — recorded compute-work cost of the direct path exceeding the lookup bound by a declared margin `κ_dcc ≥ 1` (U axis) AND declared compute latency bound of the direct path exceeding the tier lookup bound by a declared margin `κ_t ≥ 1` (cycles axis); an energetically cheap but latency-slower lookup converts the memory wall from a harvestable resource into a pure stall; the margin ranges and the recommended preset are adjudicated at the primitive definition under the BOC policy | §4.4 (new) | `QO-17`; author field report ("consequently faster" / memory-wall exploitation) | CANDIDATE |
| **DCC profile telemetry schema item**: every DCC profile declares a telemetry schema binding its energy coefficients (`α_compute`, `β_bus`) and its latency tier table `τ_t` to Historical Calibration Benchmark receipt anchors; profiles without realized calibration receipts declare statically chosen conservative bounds flagged **uncalibrated** under **Axiom 14**, and an uncalibrated profile may not claim lookup-advantage admission | §4.3 | `QO-17` (calibration validates, composition proves); consequence of rows 1–2 | CANDIDATE |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline): `docs/qnPrimitive.md` — the operational admission predicate over the two axes and the governance-bounded `κ_dcc`/`κ_t` ranges with recommended preset land in §1.15/§5.9 at its next versioned update (this file supplies only the constitutional envelope and schema; the commitment is already flagged in the v1.9.0 record of `docs/abstractionLayers.md`); until that definition lands, the single-axis predicate of `docs/qnPrimitive.md` §1.15/§5.9 remains the operative canon text and this file's consequentiality constraint binds it as a CANDIDATE obligation discharged there; `docs/qlog.md` — §5.2 predicate mirror and the §6 Memory Wall Latency Harvester coupling (declared stall-window workloads, deterministic prefetch, overlap credit against `τ_t` tiers) at its next versioned update; `docs/qm.md` — §5.6 factorial rule re-derived as a corollary with a machine-derived threshold `k*` at its next versioned update; `docs/qm_geometry.md` — §7.2 predicate wording mirror; glossary — registration of the *Latency Telemetry Envelope* and *Advantage Margin* terms at the queued glossary update (no term includes are used in this update, under the standing no-dead-entries discipline of `docs/qs_history.md` §8); front matter `terms_used` unchanged.

## Normative References

The following documents establish the structural, mathematical, and legal constraints derived from this constitutional framework.

| Document | Role | Target |
|---|---|---|
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and optimization primitives | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG specification | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `docs/qm_calculus.md` | Discrete Exterior Calculus, continued fractions, and invariant dynamics | [qm_calculus.html]({{ '/qm_calculus.html' | relative_url }}) |
| `docs/qphy.md` | Physical-layer projections; Landauer floor enforcement and Layer-0 telemetry anchoring | [qphy.html]({{ '/qphy.html' | relative_url }}) |
| `docs/qs.md` | Constructive relational intent, epistemic spectrum, and human-agent alignment | [qs.html]({{ '/qs.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. The Foundational Conjecture and Dependencies Determinism

The foundational premise of this architecture addresses the formal capability of discrete number systems: *Can a discrete {% include term.html id="qn" %} number system abstract any computable phenomenon?*

Classical mathematics admits non-constructive entities: actual infinities, unmetered memory access, and continuous spaces lacking thermodynamic boundaries. Conversely, physical computational engines are strictly finite, bounded by thermodynamics, discrete memory hierarchies, and finite signal transmission speeds.

We postulate the operational conjecture:

> A number system can abstract any computable phenomenon within a computational environment if, and only if, that abstraction is strictly governed by finite physical realities and constrained by the {% include term.html id="simemp" %} framework.

This conjecture is formally rooted in {% include term.html id="dependencies-determinism" %}. In an information-theoretic sense, the state transition of any computational entity `S_t` is fully determined if, and only if, its conditional entropy given its complete set of declared causal dependencies `D(S_t)` vanishes:

`H(S_t | D(S_t)) = 0`

If any dependency is implicit, hidden, or unmeasured (violating the {% include term.html id="no-implicit-rule" %}), or attempts to cross an unverified abstraction boundary or {% include term.html id="simemp-gateway" %}, the available dependency set is incomplete (`D' ⊂ D`), yielding non-zero conditional entropy:

`H(S_t | D') > 0`

This residual entropy manifests operationally as non-deterministic state drift, runtime instability, or unmodeled attack surfaces. The SIMEMP framework ensures that every state transition's causal entropy is fully accounted for by explicit, verifiable dependencies across the {% include term.html id="cdqn" %} stack.

Within the constructive model, `H(S_t | D(S_t)) = 0` is a statement of functional purity — referential transparency over the declared dependency set — rather than a statistical measurement over an unmodeled probability distribution. Its operational content is made empirically falsifiable by the **Observational Determinism Invariant** [v1.2.0 — CANDIDATE]: for any two Layer-0 host execution environments `E_1`, `E_2` that agree on the declared dependency projections (`Π_D(E_1) = Π_D(E_2)`) and satisfy the active {% include term.html id="dcc-profile" %} hardware profile, evaluating the same governed transition must produce bit-identical output commitments (`H(S_t(E_1)) = H(S_t(E_2))`) and identical terminal receipt statuses. The invariant is deliberately stated over output commitments rather than raw values: commitments are what cross boundaries, so the comparison itself is verifiable at any receiving node without exposing payload content.

Under this invariant, an incomplete dependency set `D' ⊂ D` is no longer merely a hidden-model assumption: it is an empirically detectable defect, exposed by differential execution testing of the same governed transition across heterogeneous environments. An implementation that silently depends on unmodeled Layer-0 physical state — microarchitectural timing, cache residency, runtime assumptions — diverges under differential testing at exactly the boundary where its declared projections fail to capture the executed environment, and is thereby discarded under the same discipline as any other Tier-1 violation.

---

## 2. Method of Verification: The Recursive Search Loop

The Qn architecture does not claim closed empirical finality. The system operates as an active, recursive research framework grounded in operational falsifiability and multi-tier constraint auditing.

### 2.1. Operational Verification and Computational Consistency

Because the system explores finite computational bounds ahead of physical hardware realization, all models are working hypotheses structured to prevent premature calcification. The development trajectory is governed by a cyclic search loop:

Proposal → Construction → Evaluation → Selection

Abstractions are verified empirically through {% include term.html id="computational-consistency" %}:

> If a proposed abstraction is internally consistent, and if computations authored in {% include term.html id="qnlang" %} and compiled to {% include term.html id="qnir" %} execute deterministically and terminate within declared finite resource envelopes, the abstraction is deemed provisionally valid within the Qn universe:

`Validity(A) ⟺ ( Consistent(A) ∧ ∀ p ∈ QnIR(A), ExecutesDeterministicallyWithinBounds(p) )`

Failure to compute within finite bounds, or divergence during execution, refutes the proposed abstraction and forces an iteration of the search loop.

### 2.2. Epistemic Demarcation: Documentary Baselines vs. Empirical Grounding

To prevent ambiguity across documentation indexes and automated repository ingestion, the architecture strictly distinguishes between **Documentary Maturity** and **Epistemic Modality**:

- **Documentary Maturity (Canonical Baseline):** Specifications designated as **Canonical** represent frozen structural contracts for the current iteration of the search loop. They provide an authoritative, immutable baseline that distinguishes validated architecture from exploratory vision proposals (`docs/chronosa.md`) or uncommitted roadmap items.
- **Epistemic Modality (Constructive Working Hypothesis):** Because the runtime has not yet been executed on physical silicon substrates (Layer 0), all canonical specifications maintain the formal epistemic status of **Constructive Working Hypotheses** pending empirical Layer 0 execution receipts.

---

## 3. The SIMEMP Constraint Architecture

The constraint architecture enforces a strict two-tier hierarchy across all abstraction boundaries.

```
                  ┌────────────────────────────────────────┐
                  │    Tier 1: Existential Invariants      │
                  │     (Identity, Metric, Security)       │
                  │          [Non-Negotiable]              │
                  └──────────────────┬─────────────────────┘
                                     │ Governs
                                     ▼
                  ┌────────────────────────────────────────┐
                  │    Tier 2: Operational Agilities       │
                  │   (Efficiency, Modularity, Portability)│
                  │          [Trade-off-able]              │
                  └────────────────────────────────────────┘
```

### 3.1. Tier 1: Existential Invariants

{% include term.html id="existential-invariant" text="Tier 1 Existential Invariants" %} represent non-negotiable requirements for any governed artifact to exist. If an abstraction violates any invariant, it is discarded:

1. **Identity:** The abstraction must be distinctly known, collision-resistant, versionable, and causally rooted.
2. **Metric:** The abstraction must carry an explicit, finite measurement envelope declaring memory, compute, precision, and uncertainty boundaries.
3. **Security:** The abstraction's state transitions, causal integrity, and execution limits must be verifiable against finite adversarial models.

### 3.2. Tier 2: Operational Agilities

{% include term.html id="operational-agility" text="Tier 2 Operational Agilities" %} represent optimization parameters. They naturally conflict with one another:

1. **Efficiency:** Algorithmic throughput and minimal resource dissipation.
2. **Modularity:** Strict isolation of concerns, boundary containment, and composability.
3. **Portability:** Substrate independence across execution targets and network architectures.

*The Golden Rule of Trade-offs:* An Operational Agility may always be sacrificed to preserve an Existential Invariant, but an Existential Invariant must never be compromised for an Agility.

The mechanical decision procedure that instantiates this hierarchy is the two-stage BOC model of §7 [v1.2.0 — CANDIDATE].

---

## 4. Environmental Realities, Thermodynamics, and the DCC Profile

Physical computation requires formalizing hardware boundaries and thermodynamic laws directly into computational metadata.

### 4.1. The Memory Wall, Semantic Gap, and Landauer's Principle

- **Landauer's Bound and Dissipation:** Any irreversible manipulation or erasure of information dissipates thermodynamic work:

`W ≥ k_B T ln 2`

At a reference ambient temperature of T = 300 K this floor evaluates to approximately `2.87 × 10^-21 J` per erased bit. It is a **theoretical scale anchor**, not a meter reading: commodity CMOS Layer-0 substrates dissipate on the order of `10^7` to `10^10` times this floor per logical operation (capacitive charging, leakage currents, interconnect overhead), regardless of the logical reversibility of the high-level symbolic operation. The constitutional consequence — the separation of the logical work quantum from the physical energy ledger — is drawn in §4.4 [v1.2.0 — CANDIDATE].

- **The Memory Wall:** Transporting data across physical memory hierarchies dissipates physical energy (`O(C·V²·f)`) and introduces transmission latency. The Metric invariant requires explicit accounting for memory bandwidth and locality rather than purely abstract asymptotic complexity.
- **The Semantic Gap:** Translation between high-level intent ({% include term.html id="qnlang" %}) and physical execution ({% include term.html id="qnir" %}) represents a primary vulnerability. The system mandates bounded, verifiable intermediate representations.

### 4.2. Open Systems, Entropy Export, and Dissipative Error Correction

A computational system is an **open thermodynamic system**. Left unconstrained, thermal noise, transmission faults, and physical substrate errors maximize internal entropy (`dS_internal > 0`).

To maintain internal structural order and deterministic reproducibility, the system operates as a **dissipative structure** (Prigogine formulation):

`dS = dS_internal + dS_exchange, where dS_exchange < 0`

Maintaining computational stability requires expending work—formally recognized as {% include term.html id="metric-exhaustion" %}—to verify state transitions and export entropy via bounded {% include term.html id="receipt" text="receipts" %}. Without reproducible, receipt-generating validation steps, a system cannot self-repair against entropy accumulation.

The entropy exported by a bounded receipt is **epistemic, not thermal** [v1.2.0 — CANDIDATE]: a receipt compresses audit state for an external verifier, reducing the verifier's information uncertainty about the transition it attests. The physical act of allocating, hashing, signing, and transmitting that receipt is itself irreversible Layer-0 work and increases internal dissipation (`dS_internal > 0`) on the emitting node. The Prigogine balance above is therefore an analogy of accounting structure — internal disorder must be compensated by bounded, verifiable export — not an identity of thermodynamic quantities; the physical energy ledger is carried separately by the Layer-0 Hardware Telemetry Envelope of §4.4.

### 4.3. The DCC Profile and Structural Indirection

To prevent rigid calcification and ensure defense against zero-day vulnerabilities, the system implements {% include term.html id="structural-indirection" %}:

```
                       ┌───────────────────────────────┐
                       │     Abstract DCC Contract     │
                       │ (Interface, Proof Requirement)│
                       └───────────────┬───────────────┘
                                       │ Implemented by
                                       ▼
                       ┌───────────────────────────────┐
                       │ Versioned Instantiation       │
                       │(e.g., Dilithium3, HashTree_v1)│
                       └───────────────┬───────────────┘
                                       │ Migrates upon fault
                                       ▼
                       ┌───────────────────────────────┐
                       │ Agile Replacement Module      │
                       │ (e.g., PQC_Patch_v2)          │
                       └───────────────────────────────┘
```

Every abstraction layer, morphism, and gateway exposes an explicit {% include term.html id="dcc-profile" %} (**Dependencies, Constraints, Capabilities**):

- **Dependencies:** Abstract capability contracts, cryptographic commitments, and environmental prerequisites. Root dependencies must be parameterized interfaces rather than hardcoded low-level implementations.
- **Constraints:** Hard ceilings on memory allocation, execution steps, recursion depth, and precision budgets.
- **Capabilities:** Permitted operations, transformation rights, and network export authorizations.

**Telemetry Schema [v1.3.0 — CANDIDATE].** Every DCC profile additionally declares a **telemetry schema** binding the node-local Layer-0 telemetry of §4.4: the per-unit energy coefficients (`α_compute`, `β_bus`) and the per-tier worst-case lookup bound table (`τ_t`), each anchored to a **Historical Calibration Benchmark receipt**. Until calibration receipts are physically realized on Layer 0 (§2.2), profiles declare statically chosen conservative bounds flagged **uncalibrated** under **Axiom 14**; an uncalibrated profile may not claim lookup-advantage admission under §4.4's consequentiality constraint, because a fabricated tier bound would manufacture the very advantage it claims to measure.

If a concrete cryptographic primitive or implementation module suffers a zero-day vulnerability, structural indirection allows the system to swap the implementation module and emit an explicit migration receipt without fracturing the abstract DCC contract or breaking causal lineage.

**Migration Witnessing [v1.2.0 — CANDIDATE].** Because node identity and causal lineage are content-addressed by cryptographic digests, swapping a compromised primitive invalidates historical hash bindings unless digests are self-describing. Every content-addressed digest therefore carries an explicit **algorithm tag** (a multihash-style prefix identifying its hash function), and every migration receipt records both the superseded and the successor algorithm identifiers. Historical commitments are preserved by **re-witnessing**: previously sealed artifacts are re-digested under the successor primitive and re-sealed with fresh signatures, each re-witness record anchoring back to the original commitment so that lineage verification remains possible across the full algorithm history. Bare un-tagged digests are not admissible across a {% include term.html id="simemp-gateway" %}.

### 4.4. The Abstract Compute Unit U and the Layer-0 Hardware Telemetry Envelope [v1.2.0 — CANDIDATE]

The {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} is defined strictly as a **platform-invariant logical work quantum** at Layers 1–3 and in QnIR: the unique unit in which totality guarantees, static instruction costs, and cross-node receipt verification are expressed, normalized by definition so that the minimal governed state transition Q(0) → Q(1) costs exactly 1 U. U is deliberately not a physical measurement. Joule costs vary continuously with silicon process node, voltage–frequency scaling, ambient temperature, and cache residency — quantities unknowable statically at the IR level — and folding them into U would break deterministic cross-node receipt verification, violating the substrate/invariant (car-vs-road) separation. One bit of logical erasure corresponds to the theoretical Landauer floor `W ≥ k_B T ln 2` (§4.1); that floor is a scale relation anchored in {% include term.html id="qphy" text="Qphy" %}, not a per-node meter reading.

Physical energy and thermal budgets (`E_battery`, `P_thermal`, in Joules and Watts) constitute a separate **Layer-0 Hardware Telemetry Envelope**, bound to U exclusively through the node's local **Historical Calibration Benchmark receipt** — a DCC-declared mapping from logical work to a hardware-specific worst-case energy envelope with stated error bars:

`E_phys ≤ α_compute · U_op + β_bus · U_mem`

where `α_compute` and `β_bus` are the node's declared worst-case per-unit energy coefficients for the compute and memory-traffic work classes, and `U_op`, `U_mem` are the logical work executed in those classes. The coefficients are Layer-0 telemetry metadata: they gate battery and thermal leash enforcement at the node boundary and never feed back into logical budgets, so no implicit physical channel can perturb deterministic state transitions and `H(S_t | D(S_t)) = 0` is preserved by construction. Until calibration receipts are physically realized on Layer 0 (§2.2), the envelope remains a CANDIDATE metadata contract under **Axiom 14**.

The Layer-0 Hardware Telemetry Envelope is a **two-axis** contract [v1.3.0 — CANDIDATE]. Its energy axis is the `α_compute`/`β_bus` envelope above. Its **latency axis** declares, per memory tier, a DCC-declared worst-case lookup bound: `τ_t = { τ_reg, τ_L1, τ_L2, τ_L3, τ_DRAM, τ_storage, τ_remote }`, expressed in cycles against the node's declared reference clock and calibrated at Layer-0 onboarding through the same **Historical Calibration Benchmark receipt** class as the energy coefficients. A worst-case declared bound — not a runtime sample — is the only admissible form: a runtime-measured lookup latency is a function of cache residency, bus contention, thermal and power state, and co-running workloads, quantities unknowable statically at the IR level and invisible to the declared dependency set; folding any of them into an admission predicate would make admission depend on unmodeled Layer-0 state, breaking the Observational Determinism Invariant (§1) and receipt replay. The never-feeds-back rule of the energy axis extends unchanged to the latency axis: the `τ_t` bounds gate resource admission and receipt auditing at the node boundary and never feed back into logical budgets, so `H(S_t | D(S_t)) = 0` is preserved by construction.

**Consequentiality constraint [v1.3.0 — CANDIDATE].** A Tier-2 optimization morphism whose declared benefit rests on memory reuse or locality — the `Q(reuse)` and `Q(bypass)` harvesting classes — is admissible only when its advantage is declared **on both axes**: the recorded compute-work cost of the direct path must exceed the lookup bound of the touched tier by a declared advantage margin (`κ_dcc ≥ 1`, U axis), and the declared compute latency bound of the direct path must exceed that tier's `τ_t` bound by a declared advantage margin (`κ_t ≥ 1`, cycles axis). The rationale is the memory wall itself (§4.1): a lookup that is energetically cheaper but latency-slower than direct compute converts the wall from a harvestable resource into a pure stall, so admission on the work axis alone is unsound. The margin ranges, their governance bounds, and the recommended preset are adjudicated at the primitive definition (`docs/qnPrimitive.md`) under the BOC policy (§7); pending that definition, this constraint binds the current single-axis predicate of `docs/qnPrimitive.md` §1.15/§5.9 as a CANDIDATE obligation discharged at its next versioned update.

---

## 5. Existential Invariants Deep Dive

### 5.1. Identity Constraints

- **Layered Verification:** Abstractions must generate verifiable witnesses at each layer they traverse.
- **Provenance and Lineage:** Artifacts maintain a traceable derivation history. Pursuant to the {% include term.html id="ssl" %}, all runtime specifications and derivative works must embed the required {% include term.html id="paternity-reference" %}.

### 5.2. Metric Constraints

- **Finite Resource Envelopes:** Unmeasurable abstractions are physically unrealizable. Every operation must declare an explicit budget for execution, storage, and communication.
- **Totality by Budget:** Every computational transition must be total by budget, resolving to an explicit {% include term.html id="receipt" %} containing a discrete terminal state (`SUCCESS`, `FAILURE`, `TIMEOUT`, `BUDGET_EXHAUSTED`, `INCONCLUSIVE`).
- **Terminal-Status Registry [v1.2.0 — CANDIDATE]:** The five statuses above are the constitutional core lifecycle terminals. The complete closed terminal-status registry of the Qn universe — adding `NO_SOLUTION`, `QUARANTINED`, `PRECISION_INSUFFICIENT`, `EXPRESSION_TOO_COMPLEX`, `REJECTION_BY_GATEWAY`, `DIVISION_BY_ZERO_REJECTED`, and `ORDERING_INDETERMINATE_AT_ZOOM` — is canonically sealed in [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.6, which remains the single source of truth for terminal-status vocabulary; this constitution references it without duplicating or amending it.

### 5.3. Security Constraints

- **Bounded Adversaries and PQC Agility:** Security models assume computationally bounded adversaries. The cryptographic substrate must maintain agility, remaining Post-Quantum Cryptography (PQC) capable without mutating underlying identity semantics.
- **Halt-by-Construction:** Computational subsystems must retain deterministic halt authority upon constraint exhaustion. Turing-unbounded execution is rejected within the governed core.
- **The Causal Arrow:** Distributed state ordering relies strictly on a persistent {% include term.html id="causal-arrow" %} (monotonic sequences and cryptographic hash chains) rather than external wall-clock synchronization.

---

## 6. Constraint on Generative and Probabilistic Workflows

Stochastic and generative artificial intelligence processes are integrated via an explicit epistemic boundary:

```
[ Context & DCC ] ──► [ Probabilistic Proposal ] ──► [ Deterministic Construction ]
                                                              │
[ Verified Receipt ] ◄── [ Finite Selection ] ◄── [ Metric Evaluation ]
```

**Boundary Billing and Soundness Scope [v1.2.0 — CANDIDATE].** Generation behind the stochastic boundary is free of governance metering; every crossing of a {% include term.html id="simemp-gateway" %} into governed state is deterministic, metered, and receipted. Parsing, measuring, and validating an unverified candidate — PQC signature checks, DCC contract checks, metric-envelope evaluation — themselves consume finite compute and memory work. Every gateway invocation therefore deducts a static, DCC-declared verification fee `W_gate > 0` from the **invoking principal's** remaining work budget **before** any candidate is inspected, charged regardless of validation outcome, so failed and adversarial candidate floods are billed to their invoker and cannot induce free-riding metric exhaustion at the boundary. Gateway checks enforce **structural and resource compliance** — formal contract soundness, resource bounds, and representation constraints (the float prohibition) — and not informal-to-formal **semantic intent alignment**: whether a formalized candidate matches the human principal's actual intent is governed exclusively by the Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake ([`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6.1), which gates the transition to `SEALED`.

1. **Context Declaration:** Establish explicit {% include term.html id="dcc-profile" text="DCC profiles" %}, objective functions, and finite budgets.
2. **Generation:** Stochastic engines propose candidate structures (classified strictly as *unverified candidates*).
3. **Construction:** Candidates are formalized into canonical representations.
4. **Evaluation:** Candidates are measured against constraints, error tolerances, and resource consumption.
5. **Selection & Sealing:** A deterministic policy validates and seals acceptable candidates, emitting a signed {% include term.html id="receipt" %}.
6. **Bounded Termination:** If no candidate meets the threshold within the declared budget, execution halts with a defined status (`NO_SOLUTION` or `BUDGET_EXHAUSTED`).

---

## 7. The BOC Policy

The {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %} is the formal decision framework used to resolve architectural and design conflicts. Version 1.1.1 expressed it as the scalar maximization `Selection = argmax(SIMEMP Compliance(c))` over a `SIMEMP Compliance(c)` quantity that carried no declared codomain, metric space, or partial order — a formulation that could not be evaluated mechanically and under which any design choice could be rationalized post hoc as BOC-compliant. That scalar formulation is retired. Under the external review (`QO-1`), BOC is specified as a **two-stage constrained lexicographic decision model** over declared DCC metadata, mechanically decidable wherever its inputs are declared [v1.2.0 — CANDIDATE]:

**Stage 1 — Tier 1 hard admission filter (binary).** A candidate is admissible only if all three {% include term.html id="existential-invariant" text="Existential Invariants" %} hold as boolean predicates:

`C_valid = { c ∈ C | I_Id(c) ∧ I_Met(c) ∧ I_Sec(c) = 1 }`

where `I_Id(c)` verifies a collision-resistant identity envelope with intact causal lineage, `I_Met(c)` verifies that every resource ceiling in the Metric envelope is finite and declared, and `I_Sec(c)` verifies that capability and PQC signature contracts validate. A candidate failing any predicate is discarded outright (terminal status `REJECTION_BY_GATEWAY`); inadmissible candidates are never ranked, because Tier-1 invariants are admission predicates, not quantities to be maximized.

**Stage 2 — Tier 2 lexicographic ordering over `C_valid`.** Among admissible candidates, BOC minimizes the lexicographic key:

`LexKey(c) = ( ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)) )`

The leading component is the constitutional mandate to **minimize implicit state**: `ImplicitDeps(c)` attests the candidate's residual implicit-state burden — dependencies present in its construction but absent from its declared dependency commitment — and its minimization is fixed in first position, never tradeable by any Tier-2 priority declaration. The trailing triple carries the {% include term.html id="operational-agility" text="Operational Agilities" %}, mutually ordered by an explicit priority permutation `π ∈ S_3` declared in the calling {% include term.html id="dcc-profile" %}: `ΔW_worst(c)` the worst-case declared work (Efficiency), `M_coupling(c)` the external dependency and interface boundary count (Modularity), and `P_coverage(c)` the supported target hardware profile count (Portability). Ties inside a component resolve to the next component.

**Rubric mode.** Where Tier-2 quantities remain qualitative architectural judgments during high-level specification design — before measurable DCC metadata exists — BOC operates explicitly as a **structured architectural rationale rubric**: every design verdict must tabulate each finalist's Tier-1 pass/fail status and its Tier-2 trade-off vector together with the declared priority ordering used, and a verdict that cannot produce this tabulation is not BOC-compliant. Residual ties at the full key are never broken implicitly ({% include term.html id="no-implicit-rule" %}): they force either refinement of the declared DCC metadata or explicit principal adjudication recorded in the design verdict.

When implementation paths conflict, the BOC policy selects the option that minimizes implicit state and guarantees invariant preservation, even if it requires higher operational overhead.

---

## 8. Alignment with the Scaling Source License

The architectural constraints directly enforce the legal mechanics of the {% include term.html id="ssl" %}:

- **Open Core Invariants:** Identity and security layers cryptographically preserve the {% include term.html id="open-core-invariant" text="Open Core Invariants" %} (anti-patent retaliation and non-scaling open-access clauses) across derivative lineage trees.
- **Scale Auditing:** The Metric invariant provides native, verifiable telemetry (active instances, container count, agent instances, and monthly transaction volumes) to verify whether deployment scale crosses the {% include term.html id="scaling-threshold" %}.
- **Safe Harbor Neutrality:** The technical substrate operates as a neutral conduit, preserving sovereign user content and applications as uninspected {% include term.html id="payload" text="Payloads" %}.

---

## 9. Conclusion

The SIMEMP framework is a provisional, evolutionary thesis designed to formalize physical computing limits into explicit architectural constraints. By grounding computational abstractions in non-equilibrium thermodynamics, dependencies determinism, and structural indirection, SIMEMP establishes an adaptable foundation for exploring finite, accountable, and generative computational systems across the {% include term.html id="cdqn" %} network. All subsequent primitive derivations and operational specifications remain subject to verification and revision under this framework.
