---
layout: default
title: Loop-5 Technical Dossier & Review Proposal — QnIR v0.3.0-draft.36 / QnLang Rung-0
description: Standalone review dossier covering canonical baseline and post-draft.30 evolutions, prepared for the Loop-5 adversarial review
version: 0.6-dossier-proposal
updated: 2026-11-23
author: Christophe Duy Quang Nguyen
status: Review Proposal — Not Promoted; QnIR and QnLang remain [PENDING]
---

# Loop-5 Technical Dossier & Review Proposal

**Date:** 2026-11-23  
**Author:** Christophe Duy Quang Nguyen  
**Target:** v0.3.0-draft.36 (`92487c2`) + Milestone 2 QnLang Rung-0 (`48ac9a9`)

## Part I — Front Matter & Executive Review Summary

### I.1 Review lineage

Loop 1 established the v0.1 Qn universe vocabulary. Loop 2 produced the v0.2.2 candidate and F1–F8, whose repair sequence froze the minimal core, fixed the dynamic SSA semantics, closed the ledger, built the conformance oracle, and separated identity commitments. Loops 3 and 4 carried QnIR v0.3 from draft.1 through draft.30: F9–F19 and the R1–R6 repair ladder produced the 16-opcode core, the 10-component configuration, the 14-field transition record, the Single Ledger Equation, four Conservation Identities, and the exact admission surrogate `B_hat = B_cfg >= B_sem`. The Loop-5 proposal begins at draft.31 and continues through draft.36 plus the Milestone 2 QnLang Rung-0 candidate. This dossier is the technical record assembled for that review; it does not itself amend any canon file.

### I.2 Epistemic status at the review boundary

Milestone 1 canon baseline is in force for the upper layers; QnIR Steps 1–5 are closed; Steps 6.1 (Containers) and 6.2 (Calls) are discharged at specification and artifact shares; QnLang Rung-0 is a candidate specification under `qir-enc-4`. QnIR remains `[PENDING]` in the portal milestone diagram, and QnLang remains `[PENDING]` in `docs/index.md`. The promotion criterion is unchanged: independent differential execution, verbatim repair-sequence discharge, and explicit proof or measurement for every retained extension. No claim in this dossier is styled a theorem.

### I.3 Primary decision requests

1. **Step 6.3 Saturation / Qn-SIM vs. Stage 0 compiler seed.** Reviewer is asked to adjudicate whether the fixed-width bitwise prerequisite and the SCC-additive `bhat` prerequisite are sufficient to open Stage 1, or whether the Stage 0 compiler seed must first be demonstrated byte-identically.
2. **Container / Call extension validation.** Reviewer is asked to verify that `qir-enc-3` tags 17–29 and `qir-enc-4` tags 30–31 are cleanly version-gated, that the `--fixture C` and `--fixture K` blocks are the evidence of record, and that no core transcript outside the implementation-identity line changed.
3. **QnLang Rung-0 zero-concession grammar.** Reviewer is asked to confirm that the EBNF is complete for all five gate-compliant worked programs, that `=` is confined to exact equality, that the 14 compiler gates and the standing namespace invariant are mechanically checkable, and that F19 separation from `src/qnir/` is preserved.

### I.4 Dossier map

Part II reproduces the canonical baseline and complete self-contained texts. Part III gives the deep delta from draft.31 through draft.36 and Milestone 2, including measured artifacts. Part IV records token measurement, verification, and the review proposal. Every part is standalone: no external document is required to follow the argument, though every reproduced text preserves its original citation.

## Part II — Full Standalone Qn Universe Baseline

### II.0 Reproduction policy

The following canonical texts are reproduced from the committed repository at the review boundary, with duplicated front matter, license, paternity, review-integration, and revision boilerplate removed. Fenced blocks preserve line structure and are evidence, not new norms. Four shorter texts are summarised instead of reproduced because their substance is already carried by the included documents: `q0_q1.md` fixes Q(0) genesis and Q(1) calibration with NIST entropy health tests; `q2_q9.md` fixes single-digit DCC anchors and parity partitions; `qnir_conformance.md` fixes §4's five agreement points, the golden-first discipline, and the validator's four Intent-rejection classes; `qnir_commitments.md` fixes §6's lineage, identity, trace-root, and collision-comparison surfaces. Their binding content is invoked below through the included kernel, accounting, and extension texts.

### II.1 Constitutional Constraints — SIMEMP v1.4.1 — `simemp.md`

[[CODE]]
# SIMEMP: Thesis and Constraints for the Qn and cdqn Stack


---

## Alignment Record (v1.4.1)

This additive amendment aligns the constitutional text with the committed QnIR accounting and with the retired BOC portability metric. Under **Axiom 13** this is a new versioned artifact: the `Review Integration Record (v1.2.0)` and every earlier record and table are preserved verbatim, and every change is stated here. §1 states the Observational Determinism Invariant over origin-quotiented commitments `H_pub` (`docs/abstractionLayers.md` §4.4), binds the static bound tables `(τ_U, τ_t)` inside `Π_D(E)`, and confines node-local calibration receipts to Stage 1 qualification. §6 makes the Two-Tier Boundary explicit — the outer `W_gate > 0` deduction and the inner QnIR ledger with `ΔB_principal = W_gate + Work(E)` (`docs/qnir_accounting.md` §3.1/§3.4) — and §7 continues to use `P_gap`; the `P_coverage` occurrences inside the v1.2.0 record remain historical record only and are not a current minimization component. No new term includes are introduced.

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

Within the constructive model, `H(S_t | D(S_t)) = 0` is a statement of functional purity — referential transparency over the declared dependency set — rather than a statistical measurement over an unmodeled probability distribution. Its operational content is made empirically falsifiable by the **Observational Determinism Invariant** [v1.2.0 — CANDIDATE]: for any two Layer-0 host execution environments `E_1`, `E_2` that agree on the declared dependency projections (`Π_D(E_1) = Π_D(E_2)`) and satisfy the active {% include term.html id="dcc-profile" %} hardware profile, evaluating the same governed transition must produce bit-identical **origin-quotiented** output commitments (`H_pub(S_t(E_1)) = H_pub(S_t(E_2))`, where `H_pub` is the public commitment view of `docs/abstractionLayers.md` §4.4, computed after quotienting the node-local root `Q(0)_N ↦ Q̂(0)`) and identical terminal receipt statuses. The invariant is deliberately stated over output commitments rather than raw values: commitments are what cross boundaries, so the comparison itself is verifiable at any receiving node without exposing payload content. The shared DCC profile's static bound tables `(τ_U, τ_t)` are fixed inside the declared dependency projection `Π_D(E)` and are therefore common to both environments; node-local calibration receipts qualify each node at Stage 1 and remain in that node's outer Lineage/Security blocks, so calibration differences never enter the cross-node equality.

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

**Boundary Ledger Split [v1.4.1].** The `W_gate > 0` deduction is the **outer** charge, levied on the invoking principal before any inspection and charged regardless of the validation outcome. It is not a QnIR work posting: the inner QnIR ledger opens only against the forwarded budget `W_budget = B_principal − W_gate` and posts work `Work(E)` there, so the principal's total change is `ΔB_principal = W_gate + Work(E) ≥ W_gate > 0`. This is the Two-Tier Boundary / Kernel Accounting Split fixed in `docs/qnir_accounting.md` §3.1 and §3.4: the inner escrow-opening surcharge `g(Intent)`, the reserve/charge/refund identities, and the admission bound `B(P)` belong to the inner ledger and are never a second fee at this gateway. A failed or adversarial crossing is therefore billed at least `W_gate`, closing the free-crossing vector without double metering.

1. **Context Declaration:** Establish explicit {% include term.html id="dcc-profile" text="DCC profiles" %}, objective functions, and finite budgets.
2. **Generation:** Stochastic engines propose candidate structures (classified strictly as *unverified candidates*).
3. **Construction:** Candidates are formalized into canonical representations.
4. **Evaluation:** Candidates are measured against constraints, error tolerances, and resource consumption.
5. **Selection & Sealing:** A deterministic policy validates and seals acceptable candidates, emitting a signed {% include term.html id="receipt" %}.
6. **Bounded Termination:** If no candidate meets the threshold within the declared budget, execution halts with a defined status (`NO_SOLUTION` or `BUDGET_EXHAUSTED`).

---

## 7. The BOC Policy

The {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %} is the formal decision framework used to resolve architectural and design conflicts. Version 1.1.1 expressed it as the scalar maximization `Selection = argmax(SIMEMP Compliance(c))` over a quantity with no declared codomain, metric space, or partial order. That scalar formulation is retired. BOC is a **two-stage constrained lexicographic decision model** over declared DCC metadata, mechanically decidable wherever its inputs and evidence contracts are defined.

**Stage 1 — Tier 1 hard admission filter.** A candidate is admissible only if all three {% include term.html id="existential-invariant" text="Existential Invariants" %} hold as boolean predicates:

`C_valid = { c ∈ C | I_Id(c) ∧ I_Met(c) ∧ I_Sec(c) = 1 }`

`I_Id(c)` verifies a collision-resistant identity envelope with intact causal lineage; `I_Met(c)` verifies that every resource ceiling in the Metric envelope is finite and declared; and `I_Sec(c)` verifies that capability and PQC signature contracts validate. A candidate failing any predicate is discarded outright (terminal status `REJECTION_BY_GATEWAY`); inadmissible candidates are never ranked. Each predicate is recorded as `PASS`, `FAIL`, or `UNRESOLVED`. An unresolved predicate that could change admission prevents a definitive `NO_ADMISSIBLE_CANDIDATE` result.

**Stage 2 — Tier 2 lexicographic ordering.** Among admissible candidates, BOC minimizes:

`LexKey(c) = ( ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_gap(c)) )`

`ImplicitDeps(c)` is the count of distinct discoverable dependencies used by the candidate but absent from its declared dependency commitment. An untraceable or unverifiable dependency is handled by the applicable Tier-1 identity/lineage predicate, not silently converted into a Tier-2 count. `ΔW_worst(c)` is the candidate's declared worst-case logical-work bound for the common workload contract, expressed in Abstract Compute Units (`U`); it is not a runtime sample or physical-energy value. `M_coupling(c)` is the count of distinct external coupling edges, comprising external dependency contracts and externally visible interface-boundary crossings under the DCC-declared graph boundary.

`P_gap(c)` is the number of required target hardware profiles not supported by the candidate. For the finite required profile set `R` bound before evaluation:

`P_gap(c) = | R \ SupportedProfiles(c) |`

A profile is supported only when it satisfies the DCC-declared compatibility predicate and has admissible evidence. BOC minimizes `P_gap`; zero means all required profiles are supported. The former `P_coverage(c)` (supported-profile count) is retired as a minimization component because minimizing it rewards reduced coverage and contradicts portability. Historical `P_coverage` values remain historical and cannot be converted to `P_gap` without the required profile identities and support evidence.

The leading component `ImplicitDeps` is the constitutional mandate to minimize implicit state and is fixed in first position. The trailing triple carries the {% include term.html id="operational-agility" text="Operational Agilities" %}, ordered by an explicit priority permutation `π ∈ S_3` declared in the calling {% include term.html id="dcc-profile" %}. The priority permutation MUST NOT move `ImplicitDeps` from first position.

**Evidence and comparison.** Every Tier-2 value carries one evidence modality: `MEASURED`, `FORMALLY_DERIVED`, `QUALITATIVE`, or `UNKNOWN`. Quantitative values are comparable only under compatible metric definitions, units, workload/profile contexts, and evaluation semantics. Qualitative values require the same versioned ordinal rubric, or an explicit validated mapping. Qualitative and quantitative values MUST NOT be compared by assigning an arbitrary numeric score. `UNKNOWN` is never zero, optimal, or harmless. In lexicographic comparison, the next component is examined only when the preceding component is established equal; an unknown or incomparable decision-relevant component cannot be skipped. No weighted average or implicit normalization is permitted.

**Rubric mode.** Where Tier-2 quantities remain qualitative architectural judgments, BOC operates as a structured rationale rubric. Every verdict MUST tabulate each finalist's Tier-1 evidence state, Tier-2 trade-off vector, evidence modality, rubric version where applicable, and declared priority ordering. A verdict that cannot produce this tabulation is not BOC-compliant. Residual full-key ties require refinement of DCC metadata or explicit principal adjudication recorded in the decision receipt; no silent tie-breaking is permitted.

**Conditional validity and failure history.** A BOC result is a decision under a specific candidate identity, dependency lineage, DCC profile, workload, target-profile set, evidence context, and observation point. It is not an eternal property of a candidate. Historical receipts remain immutable. Evidence may be reused in a later invocation only after its applicability to the new context is checked; reuse of evidence does not reuse the previous decision. Failures require traceable justification and, where feasible, reproduction or post-mortem analysis. A non-reproducible failure is not automatically a pass. A changed dependency lineage or evaluation context triggers reassessment, not automatic exoneration or automatic rejection.

The operational requirements, result states, evidence records, metric domains, and conformance tests are specified in [BOC specification]({{ '/boc.html' | relative_url }}). When implementation paths conflict, BOC selects the option that minimizes implicit state and preserves Tier-1 invariants, even if it requires higher operational overhead.

---


The architectural constraints directly enforce the legal mechanics of the {% include term.html id="ssl" %}:

- **Scale Auditing:** The Metric invariant provides native, verifiable telemetry (active instances, container count, agent instances, and monthly transaction volumes) to verify whether deployment scale crosses the {% include term.html id="scaling-threshold" %}.

---

## 9. Conclusion

The SIMEMP framework is a provisional, evolutionary thesis designed to formalize physical computing limits into explicit architectural constraints. By grounding computational abstractions in non-equilibrium thermodynamics, dependencies determinism, and structural indirection, SIMEMP establishes an adaptable foundation for exploring finite, accountable, and generative computational systems across the {% include term.html id="cdqn" %} network. All subsequent primitive derivations and operational specifications remain subject to verification and revision under this framework.
[[/CODE]]

### II.2 BOC Policy v0.4.2 — `boc.md`

[[CODE]]
# BOC: Decision, Comparison, and Receipt Specification


---

## 1. Purpose and scope

The Best of Choices (BOC) policy defined in `docs/simemp.md` §7 uses two stages:

1. Tier 1 is a hard admission gate.
2. Tier 2 selects by constrained lexicographic minimization among admissible candidates.

This document defines a provisional implementation-level BOC contract: invocation domain, result states, evidence discipline, temporal applicability, lineage requirements, Tier-2 metric domains, comparison semantics, decision-receipt requirements, and operational evaluation protocol. It operationalizes the parent policy in `docs/simemp.md` §7 for current Qn-universe implementation and integration work. It is not a declaration that these structures, encodings, algorithms, or receipt forms are native or canonical Qn artefacts. The parent policy and this document MUST be kept aligned while this provisional contract is in use.

A BOC result is a justified decision under a bound conditional context. It is not an eternal property of a candidate. Historical outcomes remain immutable, while their applicability to a later invocation may change as dependencies, workload, policy, evidence, or operating conditions change.

### 1.1. Provisional status and Qn artefact precedence

All implementation-facing specifications in this document are **provisional for the Qn-universe implementation**. They provide an explicit, testable bridge while equivalent native Qn artefacts, primitives, semantic rules, accounting constructs, lineage structures, and integrity mechanisms are unavailable, incomplete, or not yet adopted for these purposes.

This document MUST NOT be interpreted as defining new foundational Qn semantics or as superseding an existing canonical Qn artefact. When an equivalent Qn artefact is identified and accepted for a given concern, that Qn artefact becomes authoritative for the corresponding implementation semantics. This document MUST then reference it, map the affected BOC requirement to it, and remove or mark as superseded any duplicate provisional mechanism. A BOC-level wrapper MAY remain where needed to express decision-policy requirements not already supplied by the Qn artefact.

Equivalence MUST be demonstrated, not inferred from similar names or data shapes. The mapping MUST identify at least: (a) the BOC requirement; (b) the Qn artefact and version; (c) the semantic coverage; (d) any uncovered or divergent behavior; and (e) the disposition of the provisional rule. Until that mapping is reviewed, the provisional rule remains an implementation proposal and MUST NOT be presented as native Qn behavior.

Where a native Qn artefact conflicts with this provisional document, do not silently adapt or reinterpret the Qn artefact. Record the conflict and resolve it through the governing Qn policy and versioning process. BOC MUST defer to established Qn semantics unless an authorized Qn-level change explicitly establishes otherwise.

The word **MUST** in this document expresses a requirement for implementations claiming conformance to this provisional BOC profile. It does not, by itself, make the requirement a foundational or universal Qn-universe rule.

## 2. Invocation domain

A BOC invocation is a finite decision instance:

`D = (C, P, W, π, B, A, T)`


These inputs MUST be bound before candidate evaluation begins and MUST remain immutable for the invocation. A change to any bound input creates a new invocation. Candidates MUST NOT be silently added, removed, or replaced after evaluation begins.

An empty candidate set is a malformed invocation. It does not produce a BOC result.

### 2.1. Candidate identity

Each candidate MUST have:

- a stable candidate identifier;
- an immutable identity commitment for the artifact or design description evaluated;
- a declared dependency-lineage commitment;
- references to its Tier-1 and Tier-2 evidence.

Different declared identities remain distinct even if their descriptions appear equivalent. BOC MUST NOT infer candidate identity from a short hash comparison alone. Any canonical equivalence rule MUST be separately specified and evidenced.

### 2.2. Identity, lineage, and applicability

BOC MUST distinguish three concepts:

- **Identity:** which exact candidate artifact or design revision is evaluated.
- **Lineage:** the candidates, dependencies, versions, and transformations from which it derives.
- **Applicability:** whether evidence from an earlier evaluation remains relevant under the current invocation context.

A candidate derived from a previously rejected candidate does not automatically inherit that rejection. Its lineage MUST preserve the relationship, including relevant historical findings and the changes made. The new candidate is evaluated under its own identity and current context.

Lineage does not establish equivalence. Shared ancestry does not make two candidates identical, and a changed dependency does not by itself establish that a prior failure has been resolved.

### 2.3. Temporal and conditional context

The result MUST be interpreted relative to `T` and the other bound invocation inputs. The receipt MUST identify the evaluation observation point, using a declared physical timestamp, logical clock, epoch, or other Qn-universe time reference. It MUST also identify relevant operating conditions and any evidence-validity interval or invalidation conditions that can be established.

A receipt records what was justified under its original context. It MUST NOT be silently rewritten when that context later changes. A later evaluation creates a new receipt linked to the earlier receipt by lineage or supersession references.

A previous result MAY be reused as evidence in a later invocation only after checking its applicability against the later candidate identity, dependency lineage, policy/profile, workload, environment, measurement method, and evidence-validity conditions. Reuse of evidence is not reuse of the old decision. The later invocation MUST independently determine whether the evidence remains applicable and whether the candidate satisfies the current gates and comparison rules.

A changed dependency lineage, workload, policy, environment, or measurement method requires reassessment of applicability; it does not automatically prove that the previous result is false or that the candidate now passes. Unchanged lineage does not guarantee that time-sensitive conditions or evidence remain valid.

### 2.4. Common evaluation context

All candidates in one invocation MUST be evaluated against the same `P`, `W`, `π`, and budget-accounting rules. Candidate-specific evidence and declared bounds may differ, but the evaluation contract MUST NOT be changed to favor a candidate.

A candidate that cannot be evaluated against the common workload contract MUST NOT be assigned a favorable metric by default. Its evidence state MUST be recorded as unresolved or failed under the applicable rule.

## 3. Result domain

The result of a BOC invocation is exactly one of:

`Result_BOC ∈ { SELECTED, INCONCLUSIVE, NO_ADMISSIBLE_CANDIDATE, BUDGET_EXHAUSTED }`

Every result MUST be accompanied by a decision receipt.

### 3.1. `SELECTED`

At least one candidate passed Tier 1, the required Tier-2 comparison was completed, and one candidate was selected by the declared lexicographic rule or by an explicitly recorded adjudication of a residual full-key tie.

The receipt MUST identify the selected candidate, Tier-1 evidence for every candidate, Tier-2 keys and evidence for every candidate admitted to Tier 2, the declared `π`, the selection basis, invocation inputs, and accounting record.

### 3.2. `INCONCLUSIVE`

The invocation cannot justify a unique selection under its declared rules and available evidence.

Examples include:

- a required Tier-1 fact is unresolved and could affect admission;
- a Tier-2 component cannot be compared and no later component resolves the comparison;
- multiple candidates have equal complete lexicographic keys and no authorized adjudication is recorded; or
- evidence is internally inconsistent and cannot be resolved within the invocation.

The receipt MUST identify the unresolved predicate or metric, affected candidate or candidates, and evidence needed to continue. An unresolved fact MUST NOT be silently converted into a Tier-1 pass, a Tier-1 failure, a zero-valued metric, or a best-case estimate.

A failure that cannot be reproduced MUST NOT automatically become a pass. Its evidential status remains unresolved until the discrepancy is explained or the governing evaluation rule determines otherwise.

### 3.3. `NO_ADMISSIBLE_CANDIDATE`

Every candidate in `C` was evaluated at Tier 1, and no candidate satisfied all Tier-1 admission predicates.

Every failure contributing to this result MUST have a traceable justification: failed predicate, applicable threshold or rule, observed or formally derived value, evidence provenance, measurement method and conditions, and candidate/dependency versions to which the finding applies.

If any candidate has unresolved Tier-1 evidence that could change the admission outcome, the invocation returns `INCONCLUSIVE`, not `NO_ADMISSIBLE_CANDIDATE`.

A candidate rejected by Tier 1 receives the existing terminal status `REJECTION_BY_GATEWAY`. This BOC result does not create or replace a Qn-universe terminal status.

### 3.4. `BUDGET_EXHAUSTED`

The declared evaluation budget was exhausted before the procedure could establish one of the other result states.

The receipt MUST record the initial budget, amount consumed, stage and operation at exhaustion, completed evaluations, and remaining unevaluated work where determinable within budget.

A partial ranking MUST NOT be promoted to `SELECTED`. A later attempt with a larger budget or revised evaluation plan is a new invocation.

The gateway fee, where applicable, remains governed by the SIMEMP gateway-accounting rule. `BUDGET_EXHAUSTED` does not refund or waive a fee already incurred.

## 4. Invocation procedure

The following order applies:

1. **Bind the invocation.** Freeze `C`, `P`, `W`, `π`, `B`, `A`, and `T`.
2. **Apply gateway accounting.** Charge the applicable declared gateway fee before candidate inspection, in accordance with the governing SIMEMP accounting rule.
3. **Evaluate Tier 1.** Evaluate every required admission predicate for each candidate, subject to budget. Record `PASS`, `FAIL`, or `UNRESOLVED`.
4. **Determine admissibility.** A candidate enters Tier 2 only if every required Tier-1 predicate is `PASS`.
5. **Resolve an empty admissible set.** If all candidates are fully evaluated and none is admissible, return `NO_ADMISSIBLE_CANDIDATE`. If unresolved evidence could change admission, return `INCONCLUSIVE`.
6. **Evaluate Tier 2.** Evaluate the declared Tier-2 key for every admissible candidate under the common workload and comparison contract.
7. **Select or report uncertainty.** Return `SELECTED` only when the declared ordering identifies a unique minimum or valid adjudication resolves a residual full-key tie. Otherwise return `INCONCLUSIVE`.
8. **Enforce budget exhaustion.** If the budget is exhausted before the applicable result is established, return `BUDGET_EXHAUSTED`.
9. **Seal the receipt.** Bind the result, evidence references, accounting, and invocation inputs into the decision receipt.

Budget exhaustion takes precedence whenever further work is required to establish a result. A result fully established before budget exhaustion remains valid.

## 5. Evidence and failure discipline

### 5.1. Tier-1 evidence states

Each required Tier-1 predicate has exactly three evidence states:

- `PASS`: evidence establishes that the predicate is satisfied.
- `FAIL`: evidence establishes that the predicate is violated.
- `UNRESOLVED`: evidence is absent, insufficient, contradictory, or unverifiable.

`UNRESOLVED` is not equivalent to `PASS` or `FAIL`. It MUST NOT be used as a favorable default or as a proven violation.

Tier-2 values MUST use the evidence modalities and comparison requirements defined in §8. Unknown values MUST NOT be replaced with zero, an assumed best value, or an unstated default.

### 5.2. Failure justification and post-mortem

For every Tier-1 `FAIL`, and every Tier-2 finding that materially excludes or disadvantages a candidate, the receipt MUST retain or reference enough information to reconstruct the finding.

The justification SHOULD include:

- exact predicate, constraint, or metric and its version;
- measured value, formally derived bound, or rubric outcome;
- units, instrument/tool version, calibration status, sampling method, and operating conditions, where applicable;
- candidate identity and dependency lineage evaluated;
- evidence provenance, observation point, and integrity commitment;
- reproduction procedure and result, where feasible;
- post-mortem classification; and
- known remediation paths or context changes that justify reevaluation.

Post-mortem classifications MAY include `CONFIRMED_REPRODUCIBLE`, `TRANSIENT`, `NON_REPRODUCIBLE`, `CONTRADICTED_BY_LATER_EVIDENCE`, or `UNRESOLVED`. These classify the evidence and diagnosis; they are not additional BOC result states.

A non-reproducible failure is not automatically a pass. It normally requires `INCONCLUSIVE` for the affected predicate until the discrepancy is resolved.

A post-mortem MAY be completed after the invocation. It MUST be recorded as a linked follow-up artifact and MUST NOT mutate the original receipt or retroactively change its result.

## 6. Adjudication boundary

Adjudication does not modify the lexicographic key and is not an implicit tie-breaker.

Where two or more candidates have equal complete keys, authority `A` MAY resolve the tie only if:

1. equality is recorded;
2. authority and scope were bound in the invocation;
3. rationale identifies the criteria used beyond the exhausted BOC key;
4. selected candidate and alternatives are recorded; and
5. adjudication is included in the receipt.

Without valid adjudication, the result is `INCONCLUSIVE`.

Adjudication MUST NOT waive a Tier-1 failure, alter `π` after evaluation, or change the candidate set retroactively.

## 7. Tier-2 metric domains

Tier 2 minimizes the following key:

`LexKey(c) = ( ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_gap(c)) )`

`ImplicitDeps` is always the first component. The calling DCC profile declares `π`, a permutation of the three operational-agility components. Every invocation MUST also bind a finite required target-profile set `R` in its DCC decision context.

**Stage-1 placement [v0.4.2].** At a sealed SIMEMP Gateway, `ImplicitDeps(c) = 0` is enforced as a **Stage-1** requirement under both the identity predicate `I_Id` and the security predicate `I_Sec`: a candidate with any discoverable undeclared dependency is inadmissible and never reaches Tier-2 ranking. The value `ImplicitDeps(c) > 0` is admissible **strictly inside pre-gateway synthesis/repair BOC profiles**, where it ranks candidate repairs before they cross the gateway; it MUST NOT be carried into a sealed invocation's Tier-2 key. This placement leaves the Stage-1 three-valued logic and the §9 receipt serialization untouched.

### 7.1. `ImplicitDeps`

`ImplicitDeps(c)` is the count of distinct dependencies used by the candidate's construction or required execution path that are absent from its declared dependency commitment but are discoverable and attributable to the candidate.

- The value is a non-negative integer count over a declared dependency graph and observation boundary.
- Each dependency is counted once by its canonical dependency identity.
- A dependency that is present but undisclosed contributes to this count.
- A dependency whose identity or causal origin cannot be verified is not merely a Tier-2 burden: it makes the relevant Tier-1 identity/lineage predicate `UNRESOLVED` or `FAIL`, according to the evidence.
- Declaring a dependency does not make it free of cost; declared external dependencies may contribute to `M_coupling`.

The dependency graph, discovery boundary, canonical identity rule, and evidence snapshot MUST be bound to the metric record. The value MUST NOT be inferred from an incomplete scan presented as exhaustive.

### 7.2. `ΔW_worst`

`ΔW_worst(c)` is the candidate's declared worst-case logical work bound for the common workload contract `W`, expressed in Abstract Compute Units (`U`). It is a static or formally derived bound, not a runtime sample and not physical energy.

The bound MUST identify:

- the workload contract and covered input domain;
- the cost model and its version;
- the derivation or measurement basis used to establish the bound;
- the declared worst-case value and unit (`U`); and
- any assumptions or preconditions required for the bound to hold.

Candidate values are comparable only when their workload domains and cost-model semantics are compatible. A candidate lacking a justified finite bound cannot receive a favorable default; if the missing bound can affect the decision, the invocation is `INCONCLUSIVE`.

### 7.3. `M_coupling`

`M_coupling(c)` is the count of distinct external coupling edges in the candidate's declared architecture graph. An edge represents either an external dependency contract or an externally visible interface boundary crossing the candidate boundary.

- The graph boundary and edge-classification rule MUST be declared by the DCC profile.
- Each canonical edge is counted once; multiple references to the same edge do not multiply the count.
- Internal edges wholly within the candidate boundary are excluded.
- A declared external dependency remains a coupling even though it is not an implicit dependency.

The metric is a non-negative integer. Candidates are comparable only when evaluated against the same graph-boundary and edge-classification rules.

### 7.4. `P_gap` — portability gap

The parent policy's former `P_coverage` count rewarded a larger supported-profile count even though the complete Tier-2 key is minimized. That direction is inconsistent with the intended portability objective. `P_coverage` is therefore retired as the optimization component and replaced by `P_gap`.

For the finite required target-profile set `R` bound by the invocation:

`P_gap(c) = | R \ SupportedProfiles(c) |`

`SupportedProfiles(c)` contains only profiles for which the candidate has admissible compatibility evidence under the DCC profile's declared compatibility predicate. `P_gap` is a non-negative integer; zero means every required profile is supported. BOC minimizes `P_gap`.

The receipt MUST identify `R`, the compatibility predicate and version, the supported subset, and evidence for each claimed supported profile. Profiles outside `R` do not affect this metric. The set `R` MUST be fixed before candidate evaluation and MUST be common to all candidates in the invocation.

`P_coverage` MAY appear in historical receipts but MUST NOT be used as a current minimization component. A migration from historical `P_coverage` values to `P_gap` requires the original required-profile set and per-profile support evidence; a scalar coverage count alone is insufficient to reconstruct `P_gap`.

## 8. Evidence modalities and comparison

Every Tier-2 metric value MUST carry one evidence modality:

- `MEASURED`: observed using a declared method, instrument/tool version, calibration state, and operating conditions;
- `FORMALLY_DERIVED`: established by a reproducible derivation, proof, or valid upper bound;
- `QUALITATIVE`: assigned through a versioned ordinal rubric with recorded rationale and evidence;
- `UNKNOWN`: insufficient, absent, contradictory, or inapplicable evidence.

`UNKNOWN` MUST NOT be converted to zero, a best-case value, or a favorable ordinal level. A metric record MUST include its definition/version, value or ordinal level, unit/domain, evidence modality, provenance, candidate/dependency identity, observation context, and validity conditions.

### 8.1. Quantitative comparability

Quantitative values are comparable only when they share compatible metric definitions, domains, units, workload/profile context, and evaluation semantics. A unit conversion is permitted only when the conversion is explicit, deterministic, and preserves the metric's meaning. Runtime observations MUST NOT replace a required worst-case declared bound.

**`MEASURED` scope and cross-modality comparability [v0.4.2].** For `ΔW_worst(c)`, the `MEASURED` modality is valid only when the value is obtained by **exhaustive deterministic evaluation of every element of a finite (including singleton) workload domain `W`** under the declared cost model; runtime sampling, profiling a subset of `W`, or extrapolating from observed executions is never a `MEASURED` worst-case bound and MUST be recorded as `UNKNOWN` or re-established as `FORMALLY_DERIVED`. A `MEASURED` value over finite `W` and a `FORMALLY_DERIVED` upper bound are comparable under this section only when both cover the same workload contract, cost model version, and unit (`U`); the declared upper bound that governs the comparison is the one valid for the whole common domain, and a finer value does not displace a coarser valid bound without a declared, independently justified mapping. Where the domains or cost-model versions differ, the values are incomparable; if the unresolved comparison is decision-affecting, §8.3 applies.

### 8.2. Qualitative rubric mode

A qualitative value MUST be assigned using the same versioned rubric for every candidate compared on that component. The rubric MUST define an ordinal ordering and the evidence required for each level. A rubric level is an ordinal judgment, not a measured quantity.

Qualitative values from different rubric versions are not comparable unless a declared, validated mapping establishes equivalence. A qualitative value MUST NOT be compared directly with a quantitative value merely by assigning it a convenient number. Mixed modalities require an explicit, versioned mapping that is justified independently of the candidate outcome; absent such a mapping, the comparison is unresolved.

### 8.3. Lexicographic comparison

Comparison follows the exact component order in `LexKey(c)`:

1. Compare `ImplicitDeps`.
2. If equal, compare the operational-agility components in the order declared by `π`.
3. Stop at the first component with a strict ordering.
4. Proceed to the next component only when the current component is established equal under compatible semantics.

If the first component needed to distinguish two candidates is unknown or incomparable, BOC MUST NOT skip it and use a later component to manufacture an ordering. If that unresolved comparison could affect the selected candidate, the result is `INCONCLUSIVE`.

**Frontier-reduction pruning [v0.4.2].** Before component `k` is evaluated, any candidate strictly dominated at an earlier component `j < k` (under the compatible semantics of §8.1) is pruned from the comparison frontier: the dominance at `j` already decides that pair, so `UNKNOWN` or incomparable evidence at `k` on a dominated candidate is **non-decision-affecting** and MUST NOT trigger a poison-pill `INCONCLUSIVE` that halts an otherwise decidable invocation. Pruning is deterministic, must be justified by the recorded value at `j`, and is traceable in the decision trace (§9.4). It never licenses skipping an unresolved earlier component for candidates that remain on the frontier, and it never converts an unresolved value at `j` into a favorable default.

No weighted sum, averaging, hidden normalization, or implicit tie-break is permitted. A complete-key tie is handled only under §6.

### 8.4. Evidence applicability

The applicability checks in §2.3 apply to each metric observation independently. Historical evidence may be reused only when its candidate identity or explicitly justified derivation mapping, dependency lineage, metric definition, workload/profile set, method, and validity conditions remain applicable. Reuse of a metric observation does not reuse a previous BOC ranking or result.

## 9. Decision receipt and evidence commitment

Every completed BOC invocation MUST produce one immutable decision receipt, regardless of whether its result is `SELECTED`, `INCONCLUSIVE`, `NO_ADMISSIBLE_CANDIDATE`, or `BUDGET_EXHAUSTED`. A malformed invocation that never begins evaluation MUST produce a validation error record, not a BOC decision receipt.

The receipt is the authoritative record of what the invocation evaluated, what evidence it relied on, how it applied the BOC rules, and why it returned its result. A digest or signature alone is not a substitute for the inspectable receipt payload or the referenced evidence.

### 9.1. Receipt requirements

A conforming receipt MUST contain the following logical fields. Implementations MAY encode these fields in a schema-specific representation, but MUST preserve their meaning and distinguish absent, unknown, not applicable, and empty values.


A field that does not apply MUST be represented using the schema's explicit not-applicable or empty representation. Implementations MUST NOT omit a required field merely because its value is unknown. Unknown values MUST be explicit and accompanied by a reason where the schema requires one.

### 9.2. Candidate evaluation record

Each candidate record MUST include:

- stable candidate identifier and candidate revision;
- artifact identity commitment and the identity method/version;
- dependency-lineage commitment and dependency manifest reference;
- Tier-1 predicate outcomes and evidence references;
- Tier-2 metric records if and only if the candidate was admitted to Tier 2;
- candidate-level status (`ADMITTED`, `REJECTED`, or `UNRESOLVED`);
- any historical receipt references used as evidence, with an explicit applicability determination; and
- failure/post-mortem references where applicable.

A rejected or unresolved candidate MUST NOT be assigned a fabricated Tier-2 key. A candidate not admitted to Tier 2 may have diagnostic measurements recorded separately, but those measurements MUST NOT be represented as a valid Tier-2 comparison key.

### 9.3. Evidence manifest

Every evidence item relied upon to establish a Tier-1 predicate, Tier-2 value, comparison, or adjudication MUST have a manifest entry. At minimum, an entry MUST identify:

- a stable evidence-item identifier;
- evidence type and media/content type;
- producer, source, or collection process;
- source/tool/instrument version and calibration information where relevant;
- candidate and dependency context to which the evidence applies;
- observation time or interval and relevant operating conditions;
- validity interval or explicit invalidation conditions, if known;
- provenance locator or retained-content reference; and
- integrity algorithm identifier and content commitment.

A locator is not proof that evidence remains available. Evidence needed to independently verify a decision MUST be retained or made retrievable under the repository's evidence-retention policy. If evidence is unavailable at sealing time, the receipt MUST mark the affected evidence unresolved and MUST NOT claim that the corresponding predicate or metric was established by that evidence.

Evidence may be external, but the receipt MUST bind the exact version or content commitment relied upon. A mutable URL without a version, timestamped capture, or content commitment is insufficient as the sole evidence identity.

### 9.4. Decision trace

The decision trace MUST be sufficient for an independent reviewer to reconstruct the invocation's BOC procedure without relying on undocumented tie-breaking or hidden state. It MUST record:

1. the frozen invocation inputs and candidate enumeration;
2. the applicable gateway accounting event;
3. every required Tier-1 predicate outcome and the evidence that supports it;
4. the exact admissible candidate set;
5. Tier-2 metric records and their applicability/comparability determinations;
6. the effective component order, including `ImplicitDeps` first and the bound `π` thereafter;
7. the first decisive component for each comparison that determines the selected candidate, or the precise unresolved/tied component where no unique result is established;
8. any valid adjudication; and
9. the final result and budget/accounting state.

The trace MUST NOT imply that candidates were compared on a later component when an earlier component was unresolved or incomparable. It MUST NOT suppress candidates, failed predicates, or material evidence that could change the result.

### 9.5. Canonical serialization

The receipt payload MUST have a deterministic canonical byte representation before it is committed. For the initial JSON representation, implementations MUST use the JSON Canonicalization Scheme (JCS), RFC 8785, over a schema-validated JSON object encoded as UTF-8.

In addition:

- duplicate object member names MUST be rejected;
- non-finite numeric values MUST be rejected;
- values whose exactness cannot be preserved by the selected JSON number representation MUST be encoded as schema-defined decimal/integer strings or another exact typed representation;
- unordered collections MUST be sorted by a schema-defined stable key before canonicalization;
- arrays whose order is semantically significant MUST retain that order;
- timestamps MUST use the schema's single declared normalized representation and retain the source time-basis metadata where needed;
- absent, `null`, empty, unknown, and not-applicable MUST remain semantically distinct where the schema distinguishes them; and
- implementations MUST NOT canonicalize an incomplete or schema-invalid receipt as a final receipt.

If a future Qn-native canonical serialization is adopted, it MUST be introduced as a versioned canonicalization profile with deterministic test vectors and an explicit migration rule. It MUST NOT silently change the bytes or interpretation of already sealed receipts.

### 9.6. Cryptographic commitment and receipt identity

The integrity commitment MUST bind the complete canonical receipt payload, including the evidence manifest and decision trace, but excluding all derived envelope fields: `receipt_id`, `integrity.payload_commitment`, and signature bytes that sign that commitment. The receipt schema MUST define this projection precisely to avoid self-reference. The `receipt_id` is derived only after the payload commitment has been computed and is carried in the outer receipt envelope; it is not part of the committed payload.

The commitment operation is:

`payload_commitment = H(domain_separator || canonical_payload_bytes)`

where:

- `H` is the hash algorithm identified in the receipt and permitted by the applicable DCC/security profile;
- `domain_separator` is the exact ASCII byte sequence `CDQN-BOC-RECEIPT-v1` followed by one NUL byte (`0x00`); and
- `canonical_payload_bytes` are the bytes produced under §9.5 from the receipt payload projection defined above.

The initial interoperable profile MUST support SHA-256 (`sha-256`). A deployment MAY require an additional algorithm under its security profile. Algorithm identifiers MUST be explicit; implementations MUST NOT infer an algorithm from digest length. Algorithm migration MUST preserve the original commitment and add a new linked commitment rather than rewriting a historical receipt.

`receipt_id` MUST be encoded as the algorithm identifier, a colon, and the lowercase hexadecimal payload digest (for example, `sha-256:<lowercase-hex-digest>`). Because it is an envelope field derived from the commitment, it MUST NOT be included in the bytes being committed. It MUST NOT be treated as proof of semantic equality between candidates or as a substitute for comparing canonical candidate encodings. A hash match is an integrity/indexing result; where identity equality is material, the underlying canonical identity data or an explicitly defined collision-resolution procedure MUST also be checked.

### 9.7. Signatures and authority

A cryptographic signature MAY be attached to the payload commitment when required by the DCC/security profile or when an adjudication authority must attest to its action. A signature record MUST identify the signature algorithm, signer key identifier, signature bytes, key-validity context, and the role or authority scope under which it was produced.

A valid signature establishes that the corresponding key signed the commitment; it does not independently establish that the evidence is true, the candidate is safe, or the BOC reasoning is correct. Signature validation and signer authorization MUST be recorded separately. An unsigned receipt remains structurally valid unless the applicable profile requires a signature; it MUST then be marked as not satisfying that profile's sealing requirement.

### 9.8. Immutability, correction, and supersession

Once sealed, a receipt MUST be immutable. Corrections, new measurements, post-mortem findings, dependency changes, revised policies, and reassessments MUST be represented by new linked records or a new BOC invocation. They MUST NOT overwrite the original payload or commitment.

A later receipt MAY reference an earlier receipt as `REASSESSES`, `SUPERSEDES_FOR_CURRENT_CONTEXT`, `POST_MORTEM_OF`, or another explicitly defined relation. Such a relation MUST NOT imply that the earlier receipt was invalid under its original context. The new receipt MUST state what changed and which prior evidence remains applicable, is invalidated, or remains unresolved.

If a receipt is found malformed or its commitment fails verification, a separate integrity finding MUST identify the failed check. The original bytes and observed commitment SHOULD be retained for audit; a corrected receipt is a new artifact and MUST link to the affected receipt where possible.

### 9.9. Receipt verification

A verifier MUST be able to:

1. validate the receipt against its declared schema version;
2. reconstruct the canonical payload bytes;
3. recompute and compare the payload commitment;
4. validate evidence-manifest commitments for available evidence;
5. validate signatures and signer authorization when present or required;
6. reconstruct the candidate admission set and Tier-2 comparison trace; and
7. determine whether the recorded result follows from the bound rules and evidence.

A commitment match alone is not a complete BOC verification. A receipt is fully verified only to the extent that its schema, integrity, evidence availability, authorization, and decision logic have each been checked and their verification outcomes recorded separately.

### 9.10. Illustrative receipt shape

The following is an illustrative outer-envelope shape, not a complete JSON Schema. The committed payload is the projection that excludes `receipt_id`, `integrity.payload_commitment`, and signature bytes. Field names and enumerations are normative only where defined in this document; implementations MUST publish and version a machine-readable schema before claiming interoperable receipt conformance.

```json
{
  "receipt_schema": "cdqn.boc.receipt/1.0",
  "receipt_id": "sha-256:<encoded-digest>",
  "invocation_id": "<stable-invocation-id>",
  "created_at": {
    "value": "2026-11-07T00:00:00Z",
    "time_basis": "UTC"
  },
  "specification": {
    "boc": "0.4.1",
    "simemp": "1.4.0",
    "dcc_profile": {
      "id": "<profile-id>",
      "version": "<profile-version>"
    }
  },
  "invocation": {
    "candidate_set_commitment": "<commitment>",
    "workload_commitment": "<commitment>",
    "priority_permutation": ["<component-1>", "<component-2>", "<component-3>"],
    "budget": {"unit": "<declared-unit>", "limit": "<exact-value>"},
    "required_profiles": ["<profile-id>"],
    "temporal_context_commitment": "<commitment>"
  },
  "candidates": [],
  "decision_trace": [],
  "result": "INCONCLUSIVE",
  "adjudication": {"status": "NONE"},
  "accounting": {},
  "evidence_manifest": [],
  "history_links": [],
  "integrity": {
    "canonicalization": "RFC8785-JCS",
    "hash_algorithm": "sha-256",
    "payload_commitment": "<digest>"
  }
}
```

The example intentionally uses placeholders and an illustrative unresolved result. It is not a receipt for an actual evaluation and MUST NOT be committed or presented as evidence of a completed BOC invocation.

## 10. Operational evaluation protocol

This section defines a provisional execution protocol for implementations of the BOC policy. It makes evaluation ordering, budget reservation, checkpoints, interruption, and recovery explicit without claiming that the protocol is a native Qn execution primitive. An equivalent accepted Qn artefact takes precedence under §1.1.

### 10.1. Execution lifecycle

Execution lifecycle state is distinct from `Result_BOC`. A BOC invocation has one of these operational states:

- `PREPARED`: invocation inputs and the evaluation plan have been bound; evaluation has not begun.
- `RUNNING`: an evaluation operation is in progress or the next planned operation is ready.
- `PAUSED`: execution has stopped at a valid checkpoint and may be resumed.
- `COMPLETED`: one of the four BOC result states has been established and its receipt sealed.
- `ABORTED`: execution was terminated without establishing a BOC result.

`PREPARED`, `RUNNING`, `PAUSED`, and `ABORTED` are lifecycle states, not additional BOC outcomes. An invocation that reaches `COMPLETED` MUST have exactly one `Result_BOC` under §3. A paused or aborted invocation MUST NOT be represented as `SELECTED`, `NO_ADMISSIBLE_CANDIDATE`, or another completed decision result.

A `PAUSED` invocation MAY resume only from a verified checkpoint and under the same immutable invocation inputs and evaluation-plan version. If those inputs or the plan must change, the prior invocation is not resumed; a new invocation is created and linked to the prior execution record.

An `ABORTED` invocation MUST retain an execution record containing the reason, last valid checkpoint, completed operations, budget accounting, and any partial evidence. It MUST NOT be represented as a completed decision receipt. If a later invocation uses its partial evidence, the applicability checks in §2.3 and §8.4 apply.

### 10.2. Evaluation plan binding

Before the first candidate is inspected, the invocation MUST bind a versioned evaluation plan. The plan MUST declare:

- the canonical candidate ordering;
- the ordered Tier-1 predicate list and the evaluation rule for each predicate;
- the Tier-2 metric definitions and their required evidence modalities;
- the deterministic operation ordering for candidate and metric evaluation;
- the budget unit, operation-cost model, reservation rule, and settlement rule;
- the checkpoint boundaries and recoverable state;
- the implementation/tool versions that can affect evaluation; and
- any permitted concurrency and its deterministic scheduling rule.

The plan MUST be compatible with the bound DCC profile, workload, and BOC specification version. It MUST NOT be changed after evaluation begins. A plan correction or version change requires a new invocation.

Candidate order MUST be derived from a declared canonical ordering of candidate identifiers, not from insertion order, discovery order, network arrival, or evaluator preference. The ordering is procedural only and MUST NOT affect candidate identity or comparison priority.

Tier-1 predicates MUST be evaluated in the order declared by the bound plan. Tier-2 metrics MUST be evaluated in the exact component order prescribed by `LexKey` and `π`, subject to the comparison requirements in §8. A plan MAY declare a deterministic sub-operation order within a metric, but it MUST NOT reorder the BOC priority key.

### 10.3. Budget model and reservation

The invocation budget `B` MUST declare a finite non-negative limit and an explicit accounting unit. The evaluation plan MUST define how every chargeable operation consumes that unit. The budget model MUST distinguish at least:

- gateway invocation charges governed by `docs/simemp.md`;
- Tier-1 predicate evaluation;
- Tier-2 metric evaluation;
- evidence acquisition or validation when charged to the invocation; and
- receipt construction, verification, and sealing when charged to the invocation.

The plan MUST state whether a charge is a fixed amount, a measured amount, or a reserved upper bound settled against actual consumption. No charge may be introduced after the operation has begun unless the bound plan explicitly defines the measurement and settlement rule.

Before a chargeable operation starts, the evaluator MUST reserve its declared cost or upper bound. If the reservation cannot be satisfied, that operation MUST NOT begin. The invocation then returns `BUDGET_EXHAUSTED` if the unfinished operation is required to establish the BOC result. A partial evaluation MUST NOT be promoted to a winner.

Where an operation is charged by measured consumption, the measurement source and accounting granularity MUST be fixed by the plan. Where an upper bound is reserved and actual consumption is lower, the plan MUST specify whether and when the unused reservation is released. An operation that exceeds its declared upper bound is an accounting/protocol violation; the evaluator MUST stop that operation safely, record the overrun, and MUST NOT claim a conforming completed selection from the affected execution.

Gateway fees MUST be charged exactly as required by the parent SIMEMP policy. They MUST NOT be conflated with candidate execution work or silently refunded on failure, pause, abort, or budget exhaustion. A resumed invocation MUST NOT incur the same invocation gateway fee a second time unless the parent policy explicitly defines a separate charge for a distinct invocation.

### 10.4. Deterministic operation scheduling

The default provisional execution mode is sequential and deterministic. Given identical bound invocation inputs, evaluation plan, evidence snapshot, implementation/tool versions, and deterministic operation outcomes, a conforming evaluator MUST execute the same ordered operation sequence and produce the same decision trace.

Parallel evaluation MAY be used only when the bound plan defines deterministic task partitioning, budget reservation, result collection, and checkpoint semantics. Completion order, thread scheduling, or resource contention MUST NOT change candidate ordering, metric priority, budget allocation, or the final comparison rule. If deterministic parallel accounting cannot be established, the evaluator MUST use sequential mode.

An evaluator MUST NOT selectively evaluate a candidate earlier because it appears promising, nor change operation order in response to intermediate candidate scores, unless that adaptive strategy is itself explicitly specified and versioned in the bound plan and is proven not to alter the BOC decision semantics. The default profile does not permit adaptive candidate prioritization.

### 10.5. Tier-1 execution and short-circuiting

Tier-1 predicates MUST be evaluated using the bound predicate order and evidence rules. An implementation MAY stop evaluating further Tier-1 predicates for a candidate after a decisive `FAIL` only if the plan declares that short-circuit rule and the remaining predicates are recorded as `NOT_EVALUATED` rather than as `PASS`, `FAIL`, or `UNRESOLVED`.

`NOT_EVALUATED` is an execution-coverage status, not a fourth evidence state. It MUST be kept distinct from the three Tier-1 evidence states in §5.1. The receipt MUST identify the decisive failure and the unevaluated predicates.

A candidate with a decisive Tier-1 `FAIL` is rejected and MUST NOT enter Tier 2. If no candidate is admitted, BOC may return `NO_ADMISSIBLE_CANDIDATE` only when every candidate's admissibility is established. A candidate with unresolved evidence that could change the empty-admissible-set conclusion requires `INCONCLUSIVE`, as specified in §3.2 and §3.3.

### 10.6. Tier-2 execution and comparison completion

Every Tier-1-admissible candidate MUST receive the Tier-2 evidence required to establish its comparison position under the complete lexicographic key, unless budget exhaustion or an unresolved comparison prevents completion.

A metric later in `π` MAY be skipped for a candidate pair only after an earlier component has established a strict ordering for that pair. Such a skip MUST be recorded as `NOT_REQUIRED_BY_LEXICOGRAPHIC_ORDER`, not as an unknown value or an inferred metric result. If a component is unresolved or incomparable and could determine the ordering, later components MUST NOT be used to bypass it.

An implementation MAY reuse applicable metric evidence under §8.4. The receipt MUST record the evidence reference and the applicability determination. Evidence reuse does not reduce the required comparison coverage unless the plan's deterministic accounting rule explicitly assigns a lower cost to validated reuse.

### 10.7. Atomic operations and checkpoints

Each planned evaluation operation MUST have a stable operation identifier and a defined completion boundary. Its result and budget settlement MUST be committed atomically: either the operation is recorded as completed with its evidence and charge, or it remains incomplete and MUST be safely retried or abandoned under the plan.

A checkpoint MUST contain enough state to resume without changing the invocation's meaning. At minimum it MUST bind:

- invocation identifier and immutable input commitments;
- evaluation-plan identifier/version and commitment;
- last completed operation and next planned operation;
- candidate and predicate/metric progress states;
- budget consumed, reserved, and remaining;
- evidence references and their integrity/applicability status; and
- checkpoint creation time or logical observation point.

A checkpoint MUST be integrity-protected using an implementation mechanism declared by the provisional profile or its accepted Qn equivalent. A checkpoint is not a completed decision receipt and MUST NOT be treated as one.

### 10.8. Interruption, resumption, and recovery

Interruption MAY result from resource limits, process termination, infrastructure failure, or an authorized pause. An interruption MUST NOT change the bound invocation inputs or completed operation records.

On resumption, the evaluator MUST:

1. validate the checkpoint integrity and invocation/plan commitments;
2. verify that the same specification, profile, plan, and required implementation versions remain available;
3. verify that evidence required for the next operations remains accessible and applicable;
4. reconcile reserved and settled budget charges without double charging completed operations;
5. resume at the first incomplete operation; and
6. record the resumption event in the execution history.

If any required check fails, the evaluator MUST NOT continue as though the checkpoint were valid. It MUST either retain the invocation as `PAUSED` pending resolution or mark it `ABORTED` with a reason. A changed workload, candidate set, DCC profile, priority permutation, budget contract, or evaluation plan requires a new invocation.

Completed operations MUST be idempotent across recovery: replay MUST NOT duplicate their evidence records or charges. If the implementation cannot establish idempotency, it MUST stop and record an unresolved recovery condition rather than risk double accounting or altered results.

**Atomic reservation checkpoints [v0.4.2].** Every in-flight **pre-operation** budget reservation MUST be persisted atomically with the checkpoint before the operation starts, keyed by the deterministic operation identifier `op_id = H(invocation_id || candidate_id || stage_step)`. On resumption, step 4 reconciles reservations by `op_id`: a reservation already persisted for a completed or in-flight operation is recognized and settled once, never debited again. This holds for the candidate work budget `B` and for the outer gateway fee `W_gate` alike — a crash between the debit and the operation record MUST NOT cause a second charge of either. Where the checkpoint and the reservation record cannot be committed atomically, the evaluator MUST stop with an unresolved recovery condition under the rule above.

### 10.9. Budget exhaustion and incomplete execution

If the remaining budget cannot reserve the next required operation, the invocation MUST stop before that operation begins. If the missing operation could affect admission, comparison, or receipt validity, the completed BOC result is `BUDGET_EXHAUSTED`.

The receipt MUST include all completed operations, the blocked next operation, its required reservation, remaining budget, and any candidates or comparisons left incomplete. A partial ranking MUST be retained as diagnostic trace only; it MUST NOT be exposed as a selected result.

If the BOC result was already fully established and the remaining work is optional or not required by the bound plan, budget exhaustion of that optional work does not invalidate the established result. The receipt MUST distinguish required work from optional diagnostics.

### 10.10. Operational reproducibility

A decision is operationally reproducible only when an independent evaluator can recover the bound invocation, evaluation plan, evidence snapshot or equivalent immutable evidence, implementation/tool versions, operation order, and accounting semantics sufficiently to reconstruct the same decision trace.

The receipt SHOULD classify reproducibility as `REPRODUCIBLE`, `PARTIALLY_REPRODUCIBLE`, `NON_REPRODUCIBLE`, or `NOT_YET_TESTED`, with reasons and missing dependencies. This classification is metadata about the receipt and is not a BOC result state.

A later reproduction attempt MUST create a linked verification or post-mortem record. It MUST NOT overwrite the original receipt or change the original result. A discrepancy MUST be recorded and investigated; it MUST NOT silently rewrite historical evidence.

## 11. Explicit non-goals

This provisional version does not yet define a complete machine-readable JSON Schema, the exact budget schedule for each predicate or metric evaluation, or the canonical Qn-universe representations of time, epochs, lineage transitions, and evidence-validity intervals. The receipt requirements in §9 define the logical content and a provisional implementation canonicalization/commitment profile; they do not establish that RFC 8785 JCS, SHA-256, or the proposed envelope is the native Qn representation. Equivalent Qn artefacts MUST be preferred when established and mapped as specified in §1.1. Interoperable implementations still require a published schema and conformance vectors. BOC MUST NOT be claimed fully mechanically decidable where these unresolved implementation contracts are required for execution.

## 12. Conformance tests


## 13. Normative references


---

## 14. Versioning and integration

This document is a versioned, provisional implementation specification subordinate to `docs/simemp.md` §7. Version 0.3.0 added immutable decision-receipt content, evidence-manifest, canonicalization, integrity-commitment, signature, and verification requirements to the BOC-1 and BOC-2 decision and comparison rules. Version 0.3.1 clarified that these implementation specifications remain provisional pending equivalent native Qn artefacts, established Qn artefact precedence and mapping requirements, and limited conformance language to this provisional BOC profile. Version 0.4.0 adds a provisional operational evaluation protocol for deterministic scheduling, budget reservation and settlement, Tier-1 short-circuit recording, Tier-2 comparison completion, atomic checkpoints, interruption, recovery, idempotency, and operational reproducibility. It retains the parent policy correction from `P_coverage` to `P_gap`. Version 0.4.1 is a documentation-only correction: it removes a blank-line gap that split the §12 conformance table so that the BOC-4 rows rendered outside it, updates the illustrative receipt's specification-version field, and records co-attribution in the metadata. No requirement, result state, metric, receipt field, conformance test, or test outcome changed.

Any future change to result states, invocation inputs, evidence semantics, lineage/applicability rules, receipt fields, canonicalization, or commitment semantics MUST be made through a new versioned artifact under the repository's versioning discipline. When an equivalent Qn artefact becomes authoritative, update the mapping and disposition through a versioned change; do not leave duplicate competing definitions. Historical decision receipts remain immutable.

The parent policy is coordinated in `docs/simemp.md` v1.4.0. The two documents MUST remain aligned: changes to the Tier-2 key or metric direction require coordinated versioned updates. This BOC document remains provisional for implementation until its requirements have been mapped to, and where appropriate replaced by, equivalent accepted Qn artefacts.
[[/CODE]]

### II.3 Qm — Constructive Numeric Substrate v1.4.1 — `qm.md`

[[CODE]]
# Qm — Quang Mathematics: Constructive Numeric and Relational Substrate


---

## 1. Epistemic Stance and Explicit Demarcation

In strict adherence to the {% include term.html id="no-implicit-rule" %} ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.2) and the constitutional constraints of the {% include term.html id="simemp" %} framework, this specification explicitly defines the boundaries of {% include term.html id="qm" text="Qm (Quang Mathematics)" %} to prevent implicit assumptions, hidden conventions, or non-deterministic state drift.

In the {% include term.html id="qn" %} computational universe, every mathematical proposition and state transition in $$\mathrm{Qm}$$ is governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

Operational validity is verified empirically via {% include term.html id="computational-consistency" %}. No unmeasured entity may cross an abstraction boundary or {% include term.html id="simemp-gateway" %}.

```
                         EXPLICIT DOMAIN BOUNDARY OF Qm
 ┌──────────────────────────────────────────────────────────────────────┐
 │ WHAT Qm IS                           │ WHAT Qm IS NOT                │
 ├──────────────────────────────────────┼───────────────────────────────┤
 │ Constructive discrete arithmetic     │ Continuous floating-point     │
 │ Base-independent scale lattices (z)  │ Platform-dependent radix conv.│
 │ Conserved residual tracking (r)      │ Silent truncation / unmetered │
 │ Geometric orientation involution (σ) │ Detached ontological sign ent.│
 │ Total-by-budget morphisms (Receipts) │ Infinite non-terminating exp. │
 │ Height-bounded Diophantine relations │ Unbounded Hilbert-10 void     │
 │ Bounded folds & constrained roots    │ Continuous unmetered exponent │
 │ Positional order decidability        │ Arbitrary heuristic tie-break │
 │ Concrete numeric leaf evaluation     │ Symbolic AST container (Qexpr)│
 └──────────────────────────────────────┴───────────────────────────────┘
```

### 1.1. Explicit Inclusions: What Qm Is
- **The Constructive Numeric Leaf:** The canonical, discrete representation of fully evaluated quantities along dimensional axes encapsulated within the {% include term.html id="universal-envelope" %}.
- **The $$\mathcal{R}$$-Algebra:** Explicit arithmetic morphisms ($$+ $$, $$- $$, $$\times$$, $$\div$$, powers, roots, and bounded factorials) preserving exact Diophantine invariants and emitting signed receipts.
- **Projective Scale Transitions:** Exact conservation of residual information ($$r_z$$) across zoom scales without numeric drift.
- **{% include term.html id="local-first" %} Execution:** 100% intra-node computation mediated by local cdqn memory-bus data movement, requiring zero distributed consensus.
- **Total-by-Budget Morphisms:** Discrete operations bounded by finite resource consumption via {% include term.html id="metric-exhaustion" %}.

### 1.2. Explicit Exclusions: What Qm Is Not
- **Not an IEEE 754 Floating-Point System:** Under **Axiom 10**, approximations, mantissa round-offs, subnormal representations, and $$+0.0 / -0.0$$ distinctions are prohibited.
- **Not a Non-Constructive Continuum:** $$\mathrm{Qm}$$ does not admit actual infinities ($$\aleph_0, \aleph_1$$), uncomputable real numbers (non-constructive Dedekind cuts), or undecidable Cauchy sequences.
- **Not an Unbounded Diophantine Search Oracle:** While arithmetic operations are defined through Diophantine constraints, $$\mathrm{Qm}$$ explicitly rejects unmetered infinite search spaces ($$\mathbb{Z}^n, \mathbb{Q}^n$$). Classical undecidability (Hilbert's Tenth Problem, DPRM 1970) is addressed structurally, not solved: all evaluations are constrained to finite logarithmic height bounds ($$h(x) \le H_{\mathrm{max}}$$) declared within the {% include term.html id="metric-envelope" %}. **[v1.3.0 — editorial, per Objection 4]** Finiteness buys *guaranteed termination* under budget; it does not constitute mathematical *decidability* of Diophantine solvability. The two claims are kept strictly separate throughout this specification (cf. §5.3).
- **Not an Authoring Language:** High-level source grammar belongs to {% include term.html id="qnlang" %}.
- **Not an Intermediate Execution Backend:** Hardware abstraction and register scheduling belong to {% include term.html id="qnir" %}.
- **Not an Unbounded Symbolic Container:** Unevaluated expression trees belong to {% include term.html id="qexpr" %}; $$\mathrm{Qm}$$ governs their deterministic numerical collapse.

---

## 2. Abstraction Layer Mapping and Complexity Stratification

$$\mathrm{Qm}$$ operates across the hierarchy as an orthogonal vertical projection across {% include term.html id="abstraction-layer" text="abstraction layers" %} ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §11), rather than a static horizontal tier.

```
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ Layer 3: Symbolic Composition (Qexpr Trees, Bounded Expressions)            │
 │          Qm Degree 2: Multi-axial geometry (d_k), Polynomial Forms          │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ Layer 2: Governed Morphisms & Algebraic Interfaces                          │
 │          Qm Degree 1: Monoids, Involutions (I), Diophantine Constraints     │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ Layer 1: Node Genesis Layer                                                 │
 │          Qm Degree 0: Origin Q(0), Unit Q(1), Basis Alignment d1            │
 ├═════════════════════════════════════════════════════════════════════════════┤
 │ Layer 0: Physical Substrate (Entropy Harvesting, Landauer Work W ≥ kT ln 2) │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 2.1. Layer-by-Layer Responsibilities
- **{% include term.html id="layer-1" %}:** Hosts genesis primitives. Origin {% include term.html id="q0" text="Q(0)" %} establishes the spatial origin; unit {% include term.html id="q1" text="Q(1)" %} establishes the baseline metric quantum; the transition $$Q(0) \to Q(1)$$ establishes the initial dimensional axis {% include term.html id="dimension-d" text="d1" %} and the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}.
- **Layer 2:** Hosts elementary arithmetic morphisms ($$+ $$, $$- $$, $$\times$$, $$\div$$, exponents, roots, factorials) constrained by explicit {% include term.html id="dcc-profile" text="DCC Profiles" %}, indexed sequentially along the monotonic {% include term.html id="causal-arrow" %}.
- **Layer 3:** Ingests symbolic {% include term.html id="qexpr" %} trees and collapses them into canonical $$\mathrm{Qm}$$ leaf tuples within finite metric budgets.

### 2.2. Complexity Degree Stratification
- **Degree 0 (Foundational Primitives):** $$Q(0)$$, $$Q(1)$$.
- **Degree 1 (Elementary Arithmetic):** Positional tuples along axis $$d_1$$, Diophantine quotient-remainder partitions, bounded folds, and the multi-digit radix-anchor construction (§3.6). *(Normative scope of this specification.)*
- **Degree 2 (Multi-Axial & Symbolic):** Orthogonal coordinate spaces ($$d_k$$), Clifford geometric algebras ($$\mathcal{C}\ell_{p,q}$$), spinor/rotor solutions in even subalgebras, and Qexpr CAE-DAG reduction ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) and [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }})).
- **Degree 3 (Discrete Calculus & Analysis):** Discrete exterior calculus, finite-difference operators, and projective continued fractions ([`docs/qm_calculus.md`]({{ '/qm_calculus.html' | relative_url }})).

---

## 3. Canonical Numeric Representation

Every fully evaluated numeric artifact in $$\mathrm{Qm}$$ is an immutable, discrete {% include term.html id="qn" text="Qn Numeric Entity" %} represented as a discrete leaf tuple:

$$\text{Qn Numeric Entity} \equiv \langle \sigma, \, q_z, \, z, \, r_z, \, d \rangle$$

```
                   CANONICAL Qm NUMERIC STRUCTURE
 ┌─────────────────────────────────────────────────────────────────┐
 │  σ    : Relational Orientation (⊖, ⊙, ⊕)                        │
 │  q_z  : Consolidated Metric Magnitude Quotient (q_z ∈ ℕ_≤Hmax)  │
 │  z    : Scale / Lattice Zoom Level (z ∈ ℕ)                      │
 │  r_z  : Conserved Residual (typed form, §3.4)                   │
 │  d    : Dimensional Coordinate Axis Parameter (d_k)             │
 └─────────────────────────────────────────────────────────────────┘
```

**[v1.3.0]** The magnitude field label is refined from "Discrete Metric Magnitude Quotient" to **Consolidated Metric Magnitude Quotient**: $$q_z$$ is the single self-contained quotient at zoom $$z$$ (not an incremental cascade digit); see §4.1 for the exact relation between consolidated quotients and incremental digits. Its domain $$\mathbb{N}_{\le H_{\mathrm{max}}}$$ is the bounded natural semiring **constructed** in §3.6 — it is imported from nowhere.

### 3.1. Orientation as Geometric Involution (σ)
Sign is not an independent arithmetic substance. It is an **oriented displacement** relative to local origin {% include term.html id="q0" text="Q(0)" %} along axis $$d$$:
- The base transition from $$Q(0)$$ to {% include term.html id="q1" text="Q(1)" %} along axis $$d_1$$ defines canonical positive alignment ($$\oplus$$).
- Inversion is governed by the 1D reflection functor $$\mathcal{I}_d$$, satisfying the involution property:

$$\mathcal{I}_d: \Sigma \to \Sigma, \quad \mathcal{I}_d^2 = \mathrm{id}, \quad \mathrm{where} \; \Sigma = \{ \ominus, \, \odot, \, \oplus \}$$

- **$$\odot$$ (Origin):** Coincident with local $$Q(0)$$ (zero metric displacement).
- **$$\oplus$$ (Collinear):** Parallel to the basis transition $$Q(0) \to Q(1)$$.
- **$$\ominus$$ (Anti-parallel):** Opposing the basis transition.

Under **Axiom 10**, signed zeros ($$+0.0$$, $$-0.0$$) are prohibited. If $$q_z = 0$$ and $$r_z = Q(0)$$, $$\sigma$$ collapses identically to $$\odot$$.

### 3.2. Ontological Primacy of Magnitude ($q_z$)
The discrete quotient $$q_z \in \mathbb{N}_{\le H_{\mathrm{max}}}$$ represents the unsigned metric distance along axis $$d$$ at zoom level $$z$$. In physical execution substrates, metric resources (storage footprint, clock cycles, Landauer dissipation) are strictly non-negative. Absolute magnitude $$q_z$$ is ontologically primary; signed quantities are composite pairs $$\langle \sigma, q_z \rangle$$. Every $$q_z$$ beyond the single-digit Category A spectrum is an artifact constructed by the Radix-Anchor Morphism of §3.6, carrying its own Axiom-4 causal lineage.

### 3.3. Base-Independent Scale Lattice ({% include term.html id="zoom-z" text="Zoom z" %})
Conforming to **Axiom 9**, scale is decoupled from arbitrary positional radices (binary, decimal, sexagesimal). A declared zoom level $$z \in \mathbb{N}$$ parameterizes a discrete rational subdivision quantum $$\delta_z(d) \in \mathbb{Q}^+$$ of the dimensional unit:

$$\delta_z(d) = \frac{Q(1)_d}{\kappa(z)}$$

where $$\kappa: \mathbb{N} \to \mathbb{N}$$ is a strictly monotonic resolution lattice function.

**[v1.3.0 — CANDIDATE, Review Integration QO-4]** The resolution function is **not fixed globally by this specification**. Leaving $$\kappa$$ unbound would make leaf tuples structurally incomparable across nodes and operations, violating {% include term.html id="dependencies-determinism" %}. Under the {% include term.html id="no-implicit-rule" %}, $$\kappa(z)$$ is therefore **bound per artifact inside the active DCC profile** (`DCC_ref`), which must declare exactly one canonical schedule:


- A gateway evaluating, comparing, or hashing tuples whose DCC profile declares no $$\kappa$$ schedule rejects the transition deterministically with status `REJECTION_BY_GATEWAY` and reason code `ERR_KAPPA_UNDECLARED` — no ambient resolution lattice exists (Axiom 6 closed status set).
- Cross-node contracts ({% include term.html id="q-anchor" text="Q(anchor)" %} schemas, §8) must declare matching $$\kappa$$ schedules; tuples under mismatched schedules are structurally incomparable and require explicit metered re-projection at a gateway before any cross-node equality claim.
- Additional schedules are admissible only through the explicit DCC contract extension discipline (deterministic, strictly monotonic, receipted); heuristic or runtime-negotiated $$\kappa$$ is prohibited.

### 3.4. Conserved Residual ({% include term.html id="remainder-r" text="Remainder r" %})
The residual $$r_z$$ is the exact difference between the unscaled value and its discrete quotient:

$$0 \le \lvert r_z \rvert \lt \delta_z(d)$$

The remainder is never discarded silently. It acts as the exact, causal input state for subsequent resolution expansions.

**[v1.3.0 — CANDIDATE, Review Integration QO-3]** The residual carries exactly one of two typed forms (No-Implicit Rule):

1. **Collapsed residual:** an exact integer $$0 \le r_z \lt \delta_z(d)$$ held directly in the tuple payload. The value is exact at zoom $$z$$ when additionally $$r_z = Q(0)$$ ({% include term.html id="terminal-exactness" text="Terminal Exactness" %}, §4.2).
2. **Residual commitment handle:** a commitment hash $$\mathcal{H}(\alpha_{\mathrm{rem}})$$ referencing a Layer-3 {% include term.html id="qexpr" %} residual generator (`OP_RATIO`, `OP_ROOT`, `OP_CF`) whose full evaluation exceeds the current zoom budget. The handle is **opaque at Layer 2**: it is resolved only by an explicit, metered `OP_ZOOM` crossing back to Layer 3 (§7). This typing preserves the strict upward functorial hierarchy $$\mathcal{F}: \mathbf{C}(L) \to \mathbf{C}(L+1)$$ — Layer 2 never embeds live Layer-3 state, only commitments to it.

Which form is carried is declared in the tuple's type tag; mixing forms inside one tuple is unencodable.

### 3.5. Dimensional Orthogonality ({% include term.html id="dimension-d" text="Dimension d" %})
The axis parameter $$d_k$$ ensures that metric units remain orthogonal. Remainders along dimension $$d_i$$ cannot cancel, add to, or combine with remainders along dimension $$d_j$$ ($$i \neq j$$).

### 3.6. Multi-Digit Natural Construction: The Radix-Anchor Morphism **[v1.3.0 — CANDIDATE]**

**Status.** New canonical construction. Direct response to **QO-4 / Objection 14** of the external review (Segment 2, Point 1), which classified the missing construction of naturals beyond $$Q(9)$$ as *fatal if unresolved before QnIR v0.2 implementation*. A BOC record is included at the end of this section.

**The gap being closed.** Category A ([`docs/q2_q9.md`]({{ '/q2_q9.html' | relative_url }})) constructs the single-digit artifacts $$Q(1) \dots Q(9)$$ and stops. Yet the leaf tuple (§3) declares $$q_z \in \mathbb{N}_{\le H_{\mathrm{max}}}$$, and every downstream layer — $$\mathrm{Qm}$$ arithmetic (§5), Qexpr node hashing, QnIR value slots — consumes magnitudes $$\ge Q(10)$$. Without a constructive bridge, numbers $$\ge 10$$ have no canonical form, no causal lineage, and no stable commitment hash, which breaks:

- **Identity (Tier-1):** hash-consing equality $$\mathcal{H}(A) = \mathcal{H}(B) \iff A \equiv B$$ is undefined for equal magnitudes $$\ge 10$$;
- **Metric (Tier-1):** logarithmic height ceilings $$h(q_z) \le H_{\mathrm{max}}$$ (§5.3) are declared over an unconstructed domain;
- **Dependencies Determinism:** cross-node reproducibility of any computation producing multi-digit output is unanchored.

Two prohibited "resolutions" frame the BOC record: importing $$\mathbb{N}$$ (and $$\mathbb{Q}$$) wholesale as unconstructed external primitives violates **Axioms 1, 2, and 4** and the {% include term.html id="no-implicit-rule" %}; extending the unary Successor Morphism $$\mathcal{S}^{(n)}(Q(0))$$ to arbitrary $$n$$ incurs $$\mathcal{O}(q_z)$$ causal lineage depth and $$\mathcal{O}(q_z)$$ Landauer creation work — exponential blowup relative to bit length, violating Tier-2 Efficiency under any declared budget.

**Construction.** Under the {% include term.html id="q-bypass" %}–compatible composition discipline, the bounded natural semiring $$\mathbb{N}_{\le H_{\mathrm{max}}}$$ is constructed inside the artifact model by **radix-anchor (Horner-step) composition**, anchored to `DCC_ANCHOR_Q2_PARITY_v1` (binary) or to word-aligned limbs via `DCC_ANCHOR_Q8_OCTET_v1` (octal):

$$\mathrm{Cons}_B\big(q_{\mathrm{high}}, \, d_{\mathrm{low}}\big) \;=\; \big(q_{\mathrm{high}} \times B\big) + d_{\mathrm{low}}, \qquad B \in \{Q(2), \, Q(8)\}, \quad d_{\mathrm{low}} \in \{Q(0), \dots, B - Q(1)\}$$

with explicit lineage $$\mathrm{Parent}(q) = \{B, \, q_{\mathrm{high}}, \, d_{\mathrm{low}}\}$$. Every constructed magnitude thereby traces to the local origin through the anchor contracts of Category A (**Axiom 4** satisfied). The construction is evaluated by the existing $$\mathcal{R}$$-algebra of §5 (one multiplication and one addition per step) and emits a construction receipt at each anchoring step.

**Canonical uniqueness rules.**

1. **Non-zero innermost seed:** the deepest digit of any non-zero integer is $$Q(1)$$ — leading-zero aliases are unencodable.
2. **Origin collapse:** $$\mathrm{Cons}_B(Q(0), Q(0)) \to Q(0)$$, recognized as an identity via {% include term.html id="q-bypass" %} (the representation of zero is the origin itself, never a constructed chain).
3. **Digit range:** $$0 \le d_{\mathrm{low}} \lt B$$ and $$q_{\mathrm{high}} \neq Q(0)$$ unless $$q = Q(0)$$.

Rules 1–3 give every constructed magnitude exactly one canonical Cons-chain. Consequently hash-consing equality extends unchanged from single digits to all of $$\mathbb{N}_{\le H_{\mathrm{max}}}$$: $$\mathcal{H}(A) = \mathcal{H}(B) \iff A \equiv B$$ holds for every constructed magnitude, restoring the content-addressed identity invariant at QnIR lowering boundaries.

**Canonical radix for hashing [v1.4.1].** Among the admitted construction radices, `B = Q(2)` is the **unique hash-governing canonical radix** for `Cons_B` trees prior to Layer-3 `H`-commitment: the commitment hash is computed over the binary Cons-chain, so a magnitude has exactly one committed byte form independent of display radix. `B = Q(8)` and the `Q(9)` RADIX boundary (`DCC_ANCHOR_Q9_RADIX_v1`, §3.3) are **surface/display morphisms only** — they may re-render an already-committed value for octal or decimal contract interfaces, but never define a second hash input, never alter content identity, and never license a second canonical Cons-tree. A DCC profile that declares a display radix MUST still hash the binary canonical form; otherwise its commitments are structurally incomparable and rejected at the gateway.

**Height and work.** The chain depth is

$$h_{\mathrm{cons}}(q_z) = \lceil \log_B(q_z + 1) \rceil \le H_{\mathrm{max}}$$

so lineage depth and creation work are $$\mathcal{O}(\log_B q_z)$$ — logarithmic, not linear. Under HCA-Collapse the Cons-chain lowers isomorphically to native machine words (`UINT64` / multi-precision limbs) with zero radix-conversion overhead for dyadic and octal schedules. Exceeding $$H_{\mathrm{max}}$$ during construction halts with `RECEIPT_EXPRESSION_TOO_COMPLEX`.

**Category A anchor basis (scope correction).** 

The single-digit prime anchors $$\{Q(2), Q(3), Q(5), Q(7)\}$$ form the 

**Category A single-digit prime anchor basis**: 

They characterize the factorization structure of single-digit artifacts ([`docs/q2_q9.md`]({{ '/q2_q9.html' | relative_url }}) §7). They do **not** generate the multiplicative monoid of all constructed naturals — no free monoid over $$\{2, 3, 5, 7\}$$ contains $$11, 13$$, or any integer with a prime factor $$\ge 11$$, and the generated set is not closed under addition ($$Q(9) + Q(2) = 11$$). Multi-digit magnitudes exist by the §3.6 construction, not by prime generation. (The parallel claim in `docs/q2_q9.md` §7.1 is queued for the same scope correction in its next versioned update.)

**Decimal interface projection.** 

Radix-10 formatting via $$Q(9)$$ and $$Q(10) \equiv \mathrm{Cons}_{Q(2)}(Q(5), \, Q(0))$$ remains strictly an interface projection governed by `DCC_ANCHOR_Q9_RADIX_v1` — never the internal representation (base neutrality, §3.3, is preserved).

**Example.** 

$$11 = \mathrm{Cons}_{Q(2)}\big(\mathrm{Cons}_{Q(2)}\big(\mathrm{Cons}_{Q(2)}\big(Q(1), \, Q(0)\big), \, Q(1)\big), \, Q(1)\big)$$

— binary $$1011_2$$, chain depth 3 above the $$Q(1)$$ seed; its commitment hash is distinct from $$\mathcal{H}(Q(9))$$ and $$\mathcal{H}(Q(2))$$, and identical on every node applying the same schedule.

**BOC record.** 

(a) Import $$\mathbb{N}/\mathbb{Q}$$ wholesale — rejected: violates Axioms 1/2/4 and the No-Implicit Rule. 

(b) Unary successor extension — rejected: $$\mathcal{O}(q_z)$$ work violates Tier-2 Efficiency. 

(c) **Radix-anchor morphism, $$B \in \{Q(2), Q(8)\}$$ — selected**: 
logarithmic cost, Axiom-4 lineage, native word lowering, canonical uniqueness. 

Tier-1 admission filter: Identity (unique canonical hash) ✓; Metric (finite height $$\le H_{\mathrm{max}}$$) ✓; Security (anchor-constrained, receipted construction) ✓. Tier-2 vector: Efficiency (logarithmic) ✓; Modularity (reuses §5 morphisms unchanged) ✓; Portability (dyadic/octal anchors lower to any word-addressed substrate) ✓.

---

## 4. The Projective Scale Invariant and Terminal Exactness

The triplet $$\langle z, r_z, d \rangle$$ constitutes a projective resolution system over discrete lattices.

```
                           THE TELESCOPIC ZOOM CASCADE
  Zoom z:       X_d = ( q_z · δ_z(d) ) + r_z        (consolidated form, §4.1)
                                          │
                       ┌──────────────────┘ (r_z acts as total argument)
                       ▼
  Zoom z+1:     r_z = ( d_{z+1} · δ_{z+1}(d) ) + r_{z+1}
                                                   │
                                ┌──────────────────┘
                                ▼
  Zoom z+2:     r_{z+1} = ( d_{z+2} · δ_{z+2}(d) ) + Q(0)  ◄── [TERMINAL EXACTNESS]
                                                       │
                                                       ▼
                                            Computation Terminates (ΔH = 0)
```

### 4.1. Scale Expansion Equation
When higher precision is demanded by a downstream workflow ($$z \to z + 1$$), the input state to the resolution transition is identically $$r_z$$:

$$r_z = \Big( d_{z+1} \cdot \delta_{z+1}(d) \Big) + r_{z+1}$$

**[v1.3.0 — CANDIDATE, Review Integration Segment 2 Point 2]** Notation discipline (No-Implicit Rule). The cascade and the canonical tuple use two distinct, explicitly related quantities:

- **Incremental digits** $$d_k \in \{0, 1, \dots, \kappa(k+1)/\kappa(k) - 1\}$$ are the per-step quotients produced by the cascade, with $$d_0 \equiv q_0$$. The cascade equation above is stated over incremental digits.
- **Consolidated quotient** $$q_z$$ is the single self-contained magnitude carried by the canonical leaf tuple §3:

$$q_z \;=\; \sum_{k=0}^{z} d_k \cdot \frac{\kappa(z)}{\kappa(k)}$$

- Total metric information is conserved identically in both forms:

$$\mathcal{I}_{\mathrm{total}}(X_d) \;=\; \sum_{k=0}^{z} \Big( d_k \cdot \delta_k(d) \Big) + r_z \;=\; q_z \cdot \delta_z(d) + r_z$$

The consolidated form is the tuple payload (a leaf tuple determines its value without ancestor tuples); the telescoped form is the derivation path recorded in lineage. The earlier presentation, which used the single symbol $$q_z$$ in both roles, is corrected.

- **Zoom alignment (mixed-$$z$$ operations).** Binary operations on tuples with $$z_A \lt z_B$$ first deepen the shallower tuple to $$z_B$$ (metered; the deepening consumes $$r_{z_A}$$ per the cascade above and emits no drift). Deepening the shallower tuple is the only admissible alignment; shallowing the deeper tuple is prohibited, as it would discard residual information (dissipative truncation, §5.4).

### 4.2. {% include term.html id="terminal-exactness" text="Terminal Exactness" %} Invariant

If a state evaluation yields a remainder equal to the local causal origin:

$$r_z = Q(0)$$

the representation is algebraically exact.

1. **Information Invariance:** For all subsequent scales $$k \gt 0$$, $$d_{z+k} \equiv 0$$ and $$r_{z+k} \equiv Q(0)$$.
2. **Deterministic Early Halting:** Evaluating $$z+1$$ when $$r_z = Q(0)$$ yields zero entropy reduction ($$\Delta I = 0$$). Under the **Metric Invariant** ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §3.1), spending compute budget on idempotent iterations is forbidden.
3. **Receipt Emission:** The {% include term.html id="simemp-gateway" %} intercepts this state and emits an immutable terminal {% include term.html id="receipt" %}:
   - **Terminal Receipt:** `RECEIPT_TERMINAL_EXACTNESS`  
     `⟨ Status: SUCCESS_EXACT, Zoom: z, Remainder: Q(0), ConsumedWork: 0 ⟩`

### 4.3. Canonical Zoom Normalization **[v1.3.0 — CANDIDATE]**

**[Review Integration Segment 4 Point 7]** Requirement-style identity — equality of values is equality of commitment hashes — holds across zoom levels only if values are hashed at a canonical zoom. Qm therefore fixes: a value that achieves exactness (collapsed residual, §3.4 form 1) is **re-expressed at its canonical minimal zoom** $$z_{\mathrm{min}}$$ — the smallest declared zoom at which the value attains Terminal Exactness — before any commitment hash is computed or compared. The re-expression is a deterministic re-canonicalization billed to the calling operation (never silent, never free-floating): two tuples denoting the same value at different zooms normalize to one commitment, so $$\mathcal{H}(A) = \mathcal{H}(B) \iff A \equiv B$$ holds zoom-invariantly. Tuples carrying unexpanded residual handles (§3.4 form 2) are never normalized across zooms; their equality claims are deferred to explicit metered zoom resolution (§6, §7).

**Constructive `z_min` reduction [v1.4.1].** For a terminal-exact tuple `(q_z, r_z, z)` with `r_z = Q(0)`, the canonical minimal zoom is computed by a deterministic recurrence over the resolution schedule `κ` declared by the active DCC profile (§3.3). Let `ρ(z) = κ(z) / κ(z-1)`: `ρ = 2` for `Dyadic` (`κ(z) = 2^z`), `ρ = 10` for `Decimal` (`κ(z) = 10^z`), and `ρ = z+1` for `Factorial` (`κ(z) = (z+1)!`). While `z > 0` and `r_z = Q(0)`, divide `q_z` by `ρ(z)` as long as the division is exact (`q_z ← q_z / ρ(z)`), then set `z ← z-1` and repeat; stop when `r_z ≠ Q(0)`, when `z = 0`, or when the division is no longer exact. The resulting `z` is `z_min`; an exact zero (`q_z = Q(0)` and `r_z = Q(0)`) collapses directly to `z_min = 0`. Every step is a deterministic re-canonicalization billed to the calling operation and changes the representation only, never the value, so the hash after reduction is the unique zoom-invariant commitment of §4.3.

**Zoom sign and QnIR lowering [v1.4.1].** The convention `δ_z = Q(1)/κ(z)` fixes `δ_z` as a **positive** scale subdivision at zoom `z`. QnIR's binary `ZOOM=4` word carries the exponent `z` with effective scale `2^-z`; for the `Dyadic` schedule the two agree exactly (`δ_z = 2^-z`), so a dyadic leaf lowers directly to a binary `ZOOM=4` word with the same exponent. `Decimal` and `Factorial` schedules are not powers of two: no exponent-only `ZOOM=4` word can denote their leaves, so those leaves lower to QnIR **exact integer / `TUPLE` rationals** (numerator and denominator as exact integer words, canonicalized under §3.6's binary radix), never to dyadic `ZOOM=4` words. This keeps the zoom sign convention uniform across schedules without asserting a false binary denotation.

---

## 5. Elementary Morphisms (R-Algebra)

Arithmetic in $$\mathrm{Qm}$$ operates over discrete tuples rather than continuous fields. All magnitude arithmetic below operates over the constructed naturals of §3.6; no operation references an unconstructed domain.

### 5.1. Addition and Subtraction
Addition is exact vector concatenation along axis $$d$$. Subtraction is addition composed with geometric reflection:

$$A - B \equiv A + \mathcal{I}_d(B)$$

Given $$A = \langle \sigma_A, q_A, z, r_A, d \rangle$$ and $$B = \langle \sigma_B, q_B, z, r_B, d \rangle$$ (first aligned to a common zoom per §4.1):

Step 1: Absolute values and remainders are combined over the common quantum $$\delta_z(d)$$:

$$\mu_{\mathrm{exact}} = (\sigma_A q_A \cdot \delta_z + \sigma_A r_A) + (\sigma_B q_B \cdot \delta_z + \sigma_B r_B)$$

Step 2: The result is partitioned into discrete quotient $$q_C$$, residual remainder $$r_C$$, and orientation $$\sigma_C$$:

$$q_C = \left\lfloor \frac{\lvert \mu_{\mathrm{exact}} \rvert}{\delta_z} \right\rfloor$$

$$r_C = \lvert \mu_{\mathrm{exact}} \rvert - (q_C \cdot \delta_z)$$

$$\sigma_C = \mathrm{sgn}(\mu_{\mathrm{exact}})$$

### 5.2. Multiplication
Multiplication scales magnitude and composes orientation parity:

$$\sigma_C = \sigma_A \otimes \sigma_B$$

$$\oplus \otimes \oplus = \oplus, \quad \ominus \otimes \ominus = \oplus, \quad \ominus \otimes \oplus = \ominus, \quad \odot \otimes \sigma = \odot$$

The unscaled magnitude is calculated constructively:

$$\mu_{\mathrm{exact}} = (q_A \cdot \delta_z + r_A) \times (q_B \cdot \delta_z + r_B)$$

The resulting unscaled quantity collapses to $$\langle \sigma_C, q_C, z, r_C, d \rangle$$ via Diophantine resolution.

### 5.3. Division as a Constrained Diophantine Equation

Division in $$\mathrm{Qm}$$ is defined not as an unconstrained rational projection, but as a bounded constraint equation. For any input pair $$A$$ and $$B$$, the morphism determines quotient $$C$$ and remainder $$R$$ satisfying:

$$A = (B \times C) + R$$

**[v1.3.0 — CANDIDATE, Review Integration Segment 2 Point 4]** The bound $$0 \le \lvert R \rvert \lt \lvert B \rvert$$ constrains only magnitude; without fixing the orientation $$\sigma_R$$ of the remainder, two distinct solutions exist whenever $$B \nmid A$$ and negative orientations are involved (Euclidean floor division vs. truncation toward zero), violating {% include term.html id="dependencies-determinism" %}. $$\mathrm{Qm}$$ therefore fixes the **Euclidean orientation** as part of the constraint system:

$$A = (B \times C) + R, \qquad B \neq Q(0), \qquad 0 \le R \lt \lvert B \rvert, \qquad \sigma_R \equiv \oplus$$

- **Remainder orientation:** $$\sigma_R \equiv \oplus$$ always. The remainder is a non-negative unsigned metric residual, matching the residual convention of §3.4 exactly — no additional orientation field is introduced on $$R$$ (minimal implicit state).
- **Quotient determination:** $$C$$ is uniquely determined by the constraint equation; $$\sigma_C = \mathrm{sgn}(C)$$, with the **signed-zero guard** $$\sigma_C \to \odot$$ whenever $$C = Q(0)$$ (which occurs exactly when $$\lvert A \rvert \lt \lvert B \rvert$$; §3.1).
- **Exact-division parity:** when $$R = Q(0)$$, the parity composition of §5.2 holds identically: $$\sigma_C = \sigma_A \otimes \sigma_B$$.
- **Uniqueness:** for every sign combination of $$\sigma_A, \sigma_B$$ there is exactly one pair $$(C, R)$$ satisfying the four conditions above. The solution space is a singleton, so the morphism is a function, not a relation.

**BOC record (division orientation).** Euclidean orientation was selected over truncated-toward-zero orientation. Both yield unique solutions; Euclidean wins under BOC because its remainder is always a non-negative unsigned residual (collapsing onto the §3.4 tuple convention with zero extra state, satisfying the Tier-1 Metric invariant), at the cost of one $$\mathrm{sgn}$$ evaluation for $$\sigma_C$$ (constant work, already inside the metered dispatch cost of the division morphism).

**Linear division vs. general Diophantine search (scope separation).** Linear Euclidean division — the constraint equation above — is polynomial-time in the bit length of $$A$$ and terminates in exact $$(C, R)$$ always. It must not be conflated with general non-linear Diophantine polynomial equations (`OP_ROOT` domains, the Hilbert's Tenth / DPRM territory), which are handled exclusively by the height-bounded search discipline below.

#### Height-Bounded Totality

Classical Diophantine equations over unmetered spaces ($$\mathbb{Z}, \mathbb{Q}$$) inherit algorithmic undecidability under Hilbert's Tenth Problem. In $$\mathrm{Qm}$$, this undecidability is addressed structurally across three physical constraints:

**Logarithmic Height Boundedness:** In arithmetic geometry, the logarithmic height measures physical bit complexity, bounded for coprime $$u, v$$ by

$$h(u/v) = \mathrm{max}\big(\mathrm{log}\,\mathrm{max}(\lvert u \rvert, \, Q(1)), \, \mathrm{log}\,\mathrm{max}(\lvert v \rvert, \, Q(1))\big) \le H_{\mathrm{max}}$$

**[v1.3.0 — editorial]** The $$\mathrm{max}(\cdot, Q(1))$$ guard removes the log-origin singularity at $$u = Q(0)$$ present in the earlier form. Under the {% include term.html id="metric-envelope" %}, every Diophantine evaluation operates within an explicit height ceiling $$H_{\mathrm{max}}$$ and work budget $$W_{\mathrm{budget}}$$.

**Finite State Space (Northcott's Theorem):** By Northcott's theorem, the set of rational coordinates of bounded height is strictly finite:

$$\mathrm{Card}\big(\{ x \in \mathbb{Q} \colon h(x) \le H_{\mathrm{max}} \}\big) \lt \infty$$

Therefore, candidate solutions in $$\mathrm{Qm}$$ inhabit a finite discrete lattice rather than an unmetered void.

**Deterministic Totality (Termination, Not Decidability) — [v1.3.0, renamed from "Deterministic Decidability" per Segment 2 Point 4 / Objection 4]:** A height-bounded evaluation is **total by budget**: it either terminates with an exact quotient-remainder pair, reaches Terminal Exactness ($$R = Q(0)$$), or halts unconditionally upon budget exhaustion via {% include term.html id="metric-exhaustion" text="Metric Exhaustion" %} ({% include term.html id="axiom-6" text="Axiom 6" %}), emitting `RECEIPT_BUDGET_EXHAUSTED`.

**[v1.3.0 — semantic refinement]** Exhausting the Northcott height box (a candidate space of cardinality $$\mathcal{O}(e^{2 H_{\mathrm{max}}})$$) **buys guaranteed termination — it does not constitute a proof of algebraic impossibility.** Accordingly:

- Bounded-search exhaustion emits `RECEIPT_BUDGET_EXHAUSTED` with reason code `HEIGHT_BOX_EXHAUSTED`;
- the terminal status `NO_SOLUTION` is **reserved for constructive impossibility proofs** (e.g., parity obstructions such as the 1D even-root argument of §5.5.IV, or the negative-factorial argument of §5.6.III), never for mere exhaustion of a declared search box.

This distinction is normative across the corpus: conflating bounded search failure with impossibility is a semantic drift and is rejected at gateways.

#### Terminal Rejection

Attempted division by origin $$Q(0)$$ violates the DCC constraint and is intercepted deterministically at the gateway, emitting the immutable receipt `RECEIPT_DIVISION_BY_ZERO_REJECTED` with status `REJECTED` and code `ERR_DIV_ZERO`.

### 5.4. Remainder Depth and Dissipative Truncation
When recursive compositions cause the symbolic tree of $$r_C$$ to exceed the {% include term.html id="metric-envelope" text="Metric Envelope" %} ceiling $$\mathrm{Depth}_{\max}$$:
1. Silent truncation is prohibited under the {% include term.html id="no-implicit-rule" %}.
2. Execution halts or emits an explicit dissipative receipt:
   - **Dissipative Truncation Receipt:** `RECEIPT_REMAINDER_TRUNCATION`  
     `⟨ Status: DISSIPATIVE_SINK, LostResidual: r_C, ExportedEntropy: ΔS ⟩`

### 5.5. Exponentiation and Radical Morphisms

Exponentiation and root extraction are formalized as constrained morphisms that operate with explicit thermodynamic bounds and dimensional parity governance:

#### I. Integer Exponentiation as a Bounded Monoidal Fold
For integer exponents $$B \in \mathbb{N}_{\le H_{\mathrm{max}}}$$ (constructed per §3.6, exact at zoom $$z = 0$$), exponentiation is an iterative multiplicative fold over the positive monoid $$(\mathcal{Q}^+, \times, Q(1))$$ under Node Class 4 (`OP_FOLD`):

$$A^B = \prod_{j=1}^B A$$

Evaluated at Layer 3 under `OP_FOLD`. The memory footprint expands linearly with the exponent: $$h(A^B) = B \cdot h(A) \le H_{\max}$$, evaluated on the reduced coprime form of $$A$$. If the resulting bit size exceeds the metric envelope ceiling, compilation halts with `RECEIPT_EXPRESSION_TOO_COMPLEX`.

#### II. Negative Exponents as Rational Pairs
Negative powers do not invoke floating-point arithmetic; they map to exact rational inversion pairs under the Node Class 1 ordered call `OP_RATIO(Q(1), A^B)`:

$$A^{-B} \equiv \frac{Q(1)}{A^B}$$

Projected onto scale lattices at zoom $$z$$ with exact remainder conservation ($$r_z$$). If the base is negative ($$\ominus A$$), the resulting sign is determined by the parity of exponent $$B$$ via secondary anchor $$Q(2)$$ (`DCC_ANCHOR_Q2_PARITY_v1`):

$$\sigma_{\mathrm{result}} = \begin{cases} 
\oplus & \mathrm{if} \; B \in \mathcal{Q}_{\mathrm{even}} \\ 
\ominus & \mathrm{if} \; B \in \mathcal{Q}_{\mathrm{odd}} 
\end{cases}$$

The resulting rational is reduced to the canonical form of §5.7 before hashing.

#### III. Boundary Exponent Identities
Under the {% include term.html id="no-implicit-rule" %}, boundary conditions are strictly evaluated:
- **Zero Exponent Identity ($$A \neq Q(0)$$):** For any non-zero base, $$A^{Q(0)}$$ represents the empty monoidal product, collapsing to the multiplicative identity $$Q(1)$$. Evaluated in $$\mathcal{O}(1)$$ via {% include term.html id="q-bypass" %}, emitting `RECEIPT_BYPASS_IDENTITY` at zero metric cost ($$\Delta W = 0$$).
- **Indeterminate Origin Power ($$Q(0)^{Q(0)}$$):** An unparameterized $$0^0$$ evaluation is prohibited across gateways. Unless the calling profile carries an explicit contract (`DCC_ZERO_EXPONENT_v1`), execution halts with `RECEIPT_INDETERMINATE_ORIGIN_EXPONENT`.
- **Negative Origin Power ($$Q(0)^{-k}$$):** Evaluating $$Q(0)^{-k} \equiv Q(1) / Q(0)^k$$ requires division by origin $$Q(0)$$, terminating immediately with `RECEIPT_DIVISION_BY_ZERO_REJECTED`.

#### IV. Radical Morphisms and Dimensional Promotion
Roots are defined constructively as polynomial Diophantine equations under `OP_ROOT`:

$$\sqrt[n]{A} \implies \mathrm{Find} \; X \quad \mathrm{such} \; \mathrm{that} \quad X^n - A = Q(0)$$

- **Singularity of Root Zero ($$\sqrt[Q(0)]{A}$$):** Expressed as rational exponent $$A^{\frac{Q(1)}{Q(0)}}$$, extracting the $$Q(0)$$-th root requires division by origin in the exponent denominator. The gateway halts execution with `RECEIPT_DIVISION_BY_ZERO_REJECTED`.
- **Odd Roots of Negative Scalars ($$n \in \mathcal{Q}_{\mathrm{odd}}$$$):** Because an odd number of spatial reflections preserves negative orientation ($$\ominus^n = \ominus$$), odd roots of negative quantities are algebraically closed within 1D discrete space ($$\mathcal{Q}_1$$). For example, $$\sqrt[3]{\ominus 8} = \ominus 2$$ under Terminal Exactness.
- **Even Roots of Negative Scalars ($$n \in \mathcal{Q}_{\mathrm{even}}$$$):** In 1D scalar space, no orientation satisfies $$\sigma \otimes \sigma = \ominus$$. An attempt to compute $$\sqrt{\ominus A}$$ in 1D is intercepted by the gateway and promoted to Dimension 2 ($$d_2$$) as an oriented planar bivector blade in Clifford Geometric Algebra ($$\mathcal{C}\ell_{p,q}$$):

$$\sqrt{\ominus Q(1)} \implies \vec{e}_1 \wedge \vec{e}_2 \quad (\mathrm{where} \; (\vec{e}_1 \wedge \vec{e}_2)^2 = -Q(1))$$

  **[v1.3.0 — CANDIDATE, boundary restriction per Review Integration Segment 2 Point 4]** The 1D→2D bivector promotion is sound **for the square root only** ($$n = Q(2)$$) in Euclidean signature $$\mathcal{C}\ell_{2,0}$$, where $$(\vec{e}_1 \wedge \vec{e}_2)^2 = -Q(1)$$. It does **not** extend to higher even roots $$n \ge Q(4)$$: any pure bivector blade $$\beta \,(\vec{e}_1 \wedge \vec{e}_2)$$ satisfies $$\big(\beta \,(\vec{e}_1 \wedge \vec{e}_2)\big)^4 = +\,\beta^4\, Q(1) \ge Q(0)$$ — a non-negative scalar, never a negative one. Fourth and higher even roots of negative scalars are **mixed-grade rotors (spinors)** in the even subalgebra (e.g. $$\sqrt[4]{\ominus Q(1)} = \big(Q(1) \pm \vec{e}_1 \wedge \vec{e}_2\big) \times \mathrm{r}_{\sqrt{2}}$$, coupled to `DCC_ANCHOR_SQRT2_v1`), which exist only inside frames whose declared signature $$\eta$$ provides adjoining axes with $$\eta_{i,i}\,\eta_{j,j} = \oplus$$ (Lorentzian $$\mathcal{C}\ell_{1,1}$$ spacetime bivectors square to $$+Q(1)$$ and cannot serve). Such promotion crosses into Qm Degree 2 ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})). At Degree 1, an even root $$n \ge Q(4)$$ of a negative scalar is intercepted deterministically: if the calling DCC profile declares a supporting frame, the gateway routes the metered promotion to Layer 3; otherwise it emits `RECEIPT_DIMENSIONAL_BUDGET_EXHAUSTED` with reason code `FRAME_SIGNATURE_INSUFFICIENT`. This receipt binds to the sealed Axiom-6 terminal status `EXPRESSION_TOO_COMPLEX` (registered as item 5 of the circuit-breaker registry in `docs/qnPrimitive.md` §5.9); it introduces no new terminal status. No heuristic fallback exists.

#### V. The Chiral Optimization Duality
Powers and roots form a **Chiral Morphism Pair** along the causal execution arrow:
Powers and roots form a **Chiral Morphism Pair** along the causal execution arrow **[v1.4.0 — CANDIDATE]**:
- Forward powers collapse geometric grades ($$I_2^2 \to -Q(1)$$) and compute in feed-forward polynomial steps. Small integer powers fall below the two-axis admission margins of §5.6.II — margin failures and ties resolve to Direct Compute, the declared fallback — executing directly on ALUs without hash-table probes.
- Inverse roots expand geometric grades ($$\sqrt{\ominus Q(1)} \to I_2$$) and require bounded Diophantine lattice searches, so the declared cost model and declared latency bound of a root evaluation grow steeply with the requested grade — roots are typically strong candidates on the work axis. Admission is nonetheless decided solely by the canonical two-axis predicate of §5.6.II: a work-axis advantage alone is inadmissible, and the {% include term.html id="q-reuse" text="Q(reuse)" %} lookup is taken only when the latency axis clears its declared margin as well ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4, the consequentiality constraint). No root lookup is admitted by default.

### 5.6. Discrete Factorial and Combinatorial Morphisms

The factorial operator is formalized in $$\mathrm{Qm}$$ as an iterative discrete monoidal fold over the positive integer monoid $$(\mathcal{Q}^+, \times, Q(1))$$ under Node Class 4 (`OP_FOLD`):

$$n! = \prod_{j=1}^n j$$

#### I. Boundary Identifications and $\mathcal{O}(1)$ Bypasses
The factorial morphism exhibits three canonical identity states in the range $$Q(0) \dots Q(2)$$:
- **Empty Product Identity ($$Q(0)!$$):** In monoid theory, an empty product evaluates to the multiplicative identity $$Q(1)$$. Handled in $$\mathcal{O}(1)$$ via {% include term.html id="q-bypass" %}, emitting `RECEIPT_BYPASS_IDENTITY` at zero metric cost ($$\Delta W = 0$$).
- **Unit Invariant ($$Q(1)!$$):** Evaluates identically to $$Q(1)$$. Bypassed in $$\mathcal{O}(1)$$ via $$Q(\text{bypass})$$ ($$\Delta W = 0$$).
- **Fixed-Point Invariant ($$Q(2)!$$):** $$2$$ is the unique positive integer satisfying $$n! = n$$. Recognized as an algebraic fixed-point identity in `OP_BYPASS` (canonical contraction suite; the former umbrella name `OP_REWRITE` is a deprecated alias per `docs/qnPrimitive.md` §1.16), bypassing execution in $$\mathcal{O}(1)$$ ($$\Delta W = 0$$).

#### II. Thermodynamic Optimization Boundary
Under the **Metric Existential Invariant**, optimization morphisms adhere to the canonical two-axis admission predicate ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15/§5.9, instantiated over the [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4 envelope) **[v1.4.0 — CANDIDATE]**:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

- **Degenerate boundary (`n ≤ 2`):** For `Q(0)`, `Q(1)`, and `Q(2)`, the identity class of §5.6.I fires first under the causal precedence `Q(bypass) ≺ Q(reuse) ≺ Direct Compute`: the bypass resolves the evaluation at zero metric cost, so `Q(reuse)` is never reached. Independently, the declared direct path costs zero, so both axis conjunctions of the predicate fail by the floor rule — admission is rejected even in the bypass's absence.
- **Fold-recurrence corollary (`k ≥ 3`):** For `k ≥ 3`, factorial computation satisfies `Q(k)! = Q(k) × Q(k-1)!`. The declared cost model prices direct evaluation of `Q(k-1)!` at (k-2) cumulative multiplications of monotonically growing height, while the reuse path executes a single multiplication plus the declared lookup charge of the Admission Ledger ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §5.9). Because both declared bounds of the direct path grow monotonically in `k` while `τ_U[tier(h)]`, `τ_t[tier(h)]`, the gateway slacks, and the advantage margins are fixed declared constants of the profile, the predicate's verdict on the fold recurrence is a **machine-derived threshold** `k*` **[v1.4.0 — CANDIDATE]**: for `k ≥ k*` the conjunction holds and the runtime retrieves the existing hash commitment of `Q(k-1)!`, executing a single multiplication; for `k < k*` the runtime executes direct computation. `k*` is derived deterministically at DCC declaration time from the profile's declared cost model, the touched tier's declared bounds, and the declared margins — never sampled at runtime. It is not a canon-fixed constant: a profile whose declared model yields `k* = 3` reproduces the historical blanket behavior, but as a derived per-profile quantity, not a rule of canon.

#### III. Non-Existence of Negative Factorials
The factorial recurrence relation is given by:

$$(n - 1)! = \frac{n!}{n}$$

Extending this relation to negative integers requires dividing by local genesis origin $$Q(0)$$:

$$(-1)! = \frac{0!}{0} = \frac{Q(1)}{Q(0)}$$

$$\forall k \in \mathcal{Q}^+, \quad (-k)! = \frac{Q(1)}{Q(0) \times \prod_{j=1}^{k-1} (-j)}$$

Because division by origin $$Q(0)$$ is an absolute constitutional prohibition in $$\mathrm{Qm}$$ (§5.3), and actual infinite poles are non-realizable under **Axiom 2**, the factorial of a negative integer cannot exist. Any attempt to compute $$(\ominus k)!$$ is intercepted deterministically at the SIMEMP Gateway, emitting `RECEIPT_DIVISION_BY_ZERO_REJECTED`. This is a constructive impossibility proof; per §5.3 the terminal status recorded is within the closed set (division rejection), and the argument — not a bounded-search outcome — is what licenses the impossibility claim.

### 5.7. Canonical Negative Rationals **[v1.3.0 — CANDIDATE]**

**[Review Integration QO-4, part 3]** Negative rationals require no separate ring construction. A negative rational is canonically the composition of an orientation $$\sigma \in \{\ominus, \odot, \oplus\}$$ (§3.1) with a coprime positive rational pair:

$$\langle \sigma, \, p/q \rangle \quad \mathrm{with} \quad (p, q) \in \mathbb{N}_{\le H_{\mathrm{max}}} \times \mathbb{N}_{\le H_{\mathrm{max}}}, \quad q \gt Q(0), \quad \mathrm{gcd}(p, q) = Q(1)$$

where both $$p$$ and $$q$$ are artifacts constructed per §3.6. The GCD is computed exactly by the Euclidean division of §5.3 (iterated remainder sequence), so coprimality is enforced at formation time, not asserted. Any operation producing a non-reduced pair re-canonicalizes it before hashing: one commitment exists per rational value (content-addressed identity). Signed zero is structurally unreachable: $$p = Q(0)$$ collapses the pair to the origin form $$\odot\, Q(0)$$ (§3.1). Negative rationals constructed this way complete the bridge $$\mathbb{N}_{\le H_{\mathrm{max}}} \to \mathbb{Q}$$ inside the artifact model: orientation supplies the sign, `OP_RATIO` pairs supply the magnitude, and the zoom/remainder telescope (§4) supplies the projective evaluation — no float, no imported primitive, no unbounded denominator (height ceiling $$H_{\mathrm{max}}$$ applies to the reduced pair).

---

## 6. Relational Decidability and Positional Ordering

Ordering on dimension $$d$$ is positional, evaluated over bounded constructive enclosures.

**[v1.3.0 — CANDIDATE, Review Integration Segment 2 Point 4]** The enclosure of the tuple $$\langle \sigma_A, q_A, z, r_A, d \rangle$$ at zoom $$z$$ is **sign-aware** and is determined by the **typed form of the residual** (§3.4). The earlier interval form $$I(A) = [q_A \cdot \delta_z + r_A, \, q_A \cdot \delta_z + r_A + \epsilon_z]$$ is retired: it embedded the exact conserved residual inside an enlarged uncertainty width (double-counting) and relied on an undefined symbol $$\epsilon_z$$. The corrected enclosures are:

**Collapsed residual (§3.4 form 1) — the value is exact:**

$$I_z(A) = \big[\, \sigma_A \cdot (q_A \cdot \delta_z + r_A), \; \sigma_A \cdot (q_A \cdot \delta_z + r_A) \,\big] \qquad (\text{point interval})$$

**Residual commitment handle (§3.4 form 2) — the value is enclosed within one zoom cell:**

$$I_z(A) = \begin{cases} 
\big[\, q_A \cdot \delta_z, \; (q_A + 1) \cdot \delta_z \big) & \mathrm{if} \; \sigma_A = \oplus, \; r_A \neq Q(0) \\[4pt]
\big( -(q_A + 1) \cdot \delta_z, \; -q_A \cdot \delta_z \big] & \mathrm{if} \; \sigma_A = \ominus, \; r_A \neq Q(0) \\[4pt]
\big[\, Q(0), \; Q(0) \,\big] & \mathrm{if} \; \sigma_A = \odot
\end{cases}$$

The positional order relation is then:

$$\mathrm{Rel}(A, B) = \begin{cases} 
A \lt B & \mathrm{if} \; \sup I_z(A) \lt \inf I_z(B) \\
A \gt B & \mathrm{if} \; \inf I_z(A) \gt \sup I_z(B) \\
A \equiv B & \mathrm{if} \; \text{both residuals collapsed and } \mathcal{H}(A) = \mathcal{H}(B) \text{ after §4.3 normalization} \\
\mathrm{Indeterminate} & \mathrm{if} \; I_z(A) \cap I_z(B) \neq \emptyset \;\text{and at least one residual is an unexpanded handle}
\end{cases}$$

When intervals overlap such that inequality cannot be constructively decided at zoom $$z$$, $$\mathrm{Qm}$$ forbids heuristic tie-breaking. It emits the deterministic receipt `ORDERING_INDETERMINATE_AT_ZOOM`. To resolve ordering, the caller must allocate additional compute budget to deepen zoom resolution ($$z \to z + \Delta z$$), which expands the handle residuals (§3.4 form 2) through the metered cascade of §4.1 — narrowing the enclosures by construction, never by estimation.

---

## 7. Interaction with Qexpr and Derivation of Higher Mathematics

```
          UNEVALUATED SYMBOLIC AST (Layer 3)
    Qexpr = ⟨ ExpressionTree, z, r, d, DCC, Identity, Lineage ⟩
                               │
                               ▼
          COMPILATION / COLLAPSE ENGINE (Totality by Budget)
    Evaluated deterministically via Qm Morphisms (§5)
                               │
                               ▼
          COLLAPSED CONSTRUCTIVE LEAF (Canonical State)
    ⟨ σ, q_z, z, r_z, d ⟩  +  Signed Remainder Receipt
```

### 7.1. The Boundary Between Qexpr and Qm
- **{% include term.html id="qexpr" text="Qexpr" %} (Symbolic CAE-DAG):** Encapsulates unevaluated algebraic expressions, rational functions, and multi-step operator trees. It carries the symbolic structure before execution.
- **$$\mathrm{Qm}$$ (Evaluation Engine & Leaf):** Defines the concrete algebraic reduction rules that collapse Qexpr trees into discrete $$\langle \sigma, q_z, z, r_z, d \rangle$$ tuples at compilation time ({% include term.html id="qnlang" %} $$\to$$ {% include term.html id="qnir" %}). The residual commitment handle of §3.4 (form 2) is the sanctioned exception path by which a Layer-2 leaf references an unresolved Layer-3 generator — as a commitment, resolved only through metered `OP_ZOOM` crossings (§4.1, §6).

### 7.2. Derivability of Real-World Mathematics
All computable real-world mathematics derives systematically from this foundation:
1. **Multi-dimensional Vector Spaces:** Constructed by parameterizing orthogonal sets of axes $$\{d_1, d_2, \dots, d_n\}$$, giving rise to Clifford geometric algebras ($$\mathcal{C}\ell_{p,q}$$) without non-deterministic trigonometric float approximations ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})).
2. **Calculus Without Infinities:** Replaces continuous infinitesimals ($$\epsilon \to 0$$) with discrete differences over lattice quanta $$\Delta X / \delta_z(d)$$, yielding exact discrete exterior calculus ([`docs/qm_calculus.md`]({{ '/qm_calculus.html' | relative_url }})).
3. **Transcendental Constants ($$\pi, e$$):** Represented as symbolic Qexpr continued fraction operators ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.3). Evaluated at zoom $$z$$, they emit an exact rational quotient $$q_z$$ and a conserved residual $$r_z$$ satisfying exact Diophantine bounds.
4. **Arbitrary Natural and Rational Magnitudes:** Constructed inside the artifact model by the radix-anchor morphism (§3.6) and the canonical rational pairs (§5.7) — closing the single-digit ceiling of Category A without importing unconstructed primitives.

---

## 8. Structural Indirection: Q(anchor) in Qm

To prevent cascading structural collapse across dependent mathematical workflows, high-centrality mathematical primitives are formalized as {% include term.html id="q-anchor" text="Q(anchor)" %} entities (`docs/_data/glossary/architecture.yml`):

$$\chi(\alpha) = \frac{\lvert \mathcal{C}^+(\alpha) \rvert}{\lvert \mathcal{V} \rvert} \ge \Theta_{\chi}$$

```
                         CONCRETE INSTANTIATION (Fragile)
 [ Concrete Morphism ] ══════════════════════════════► [ 10,000 Dependent Qn ]
 (Revocation/Patch = Immediate Invalidation of All Downstream Lineage)

                     STRUCTURAL INDIRECTION (Robust)
 ┌──────────────────────┐
 │ Abstract DCC Anchor  │ ───────────────────────────► [ 10,000 Dependent Qn ]
 └──────────┬───────────┘                               (Causal Cones Preserved)
            │
            ├─► Morphism Implementation v1 (Canonical)
            │
            └─► Morphism Implementation v2 (Accelerated Replacement)
```

1. **Anchored Interfaces:** Critical operations (the division constraint, sign reflection, canonical unit $$\delta_z$$, and the $$\kappa(z)$$ resolution schedule of §3.3) expose abstract DCC contracts under {% include term.html id="structural-indirection" %}.
2. **Downstream Isolation:** Workflows in higher layers depend upon the abstract contract, not the internal execution engine.
3. **Agility Proofs:** Upgrading an arithmetic solver or patching an algorithmic vulnerability emits a signed migration receipt without invalidating the causal lineage of historical calculations.

---

## 9. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %} governing the {% include term.html id="licensed-work" %}:

1. **{% include term.html id="paternity-reference" %}:** All derivative implementations, mathematical solvers, or compiled runtime specification files derived from $$\mathrm{Qm}$$ must embed:
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $$\mathrm{Qm}$$ arithmetic methods against the Author or community.
[[/CODE]]

### II.4 Qs — Quang Semantics v1.3.2 — `qs.md`

[[CODE]]
# Qs — Quang Semantics: Constructive Relational Intent and Epistemic Grounding


---

## 1. Epistemic Stance and Constitutional Grounding

In classical artificial intelligence, natural language processing relies on statistical correlation across continuous floating-point vectors ($\mathbb{R}^d$). Transformer models predict tokens based on statistical consensus rather than physical truth or causal provenance. Consequently, classical models suffer from hallucinations, lack causal accountability, and cannot distinguish between verified empirical facts and unanchored fiction.

In the {% include term.html id="qn" %} computational universe, **language, meaning, and intent are not ungrounded statistical approximations**. Under [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}), semantic meaning must be constructed from finite, verifiable primitives governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

{% include term.html id="qs" text="Qs (Quang Semantics)" %} formalizes language, human-machine communication, and AI agent alignment as a **constructive, multi-axial relational domain**:
* **Language is not a black-box tensor:** It is a structured graph of discrete semantic blades and relational morphisms.
* **Meaning is not statistical similarity:** It is an explicit, verifiable {% include term.html id="intent-contract" text="Intent Contract" %} bound to human authority.
* **The Non-Rogue Invariant:** Machines and AI agents cannot originate unanchored intent; they operate strictly as delegated proxies bounded by human sovereignty and thermodynamic metric budgets.

Operational validity is verified empirically via {% include term.html id="computational-consistency" %}. No unmeasured semantic assumption may cross a {% include term.html id="simemp-gateway" %}.

```
          STATISTICAL MIMICRY (LLMs) vs. GOVERNED MEANING (Qs)
 Classical LLM / Transformer:
   Prompt ──► [ Unmetered Continuous Latents R^(d×d) ] ──► Probabilistic Token
   • Opaque weights (Billions of unverified parameters).
   • Meaning = Statistical similarity. Hallucinations unconstrained.
   • Violates No-Implicit Rule: H(Meaning | Reality) > 0.

 Qs Constructive Semantic Framework:
   Prompt ──► [ Candidate Parser ] ──► Structured Qexpr (CAE-DAG)
                                            │
                                            ▼ Evaluated across
                ┌───────────────────────────┼───────────────────────────┐
                ▼                           ▼                           ▼
         Qs (Semantics)              Qm (Mathematics)           Qphy (Physics)
    "What intent is bound?"     "Is it non-contradictory?"   "Can energy execute?"
                │                           │                           │
                └───────────────────────────┼───────────────────────────┘
                                            ▼
                   Sealed DCC Intent Contract + Signed Receipt
```

---

## 2. Vertical Abstraction Layer Projection of Qs

Mirroring the mathematical domain ([`docs/qm.md`]({{ '/qm.html' | relative_url }})), {% include term.html id="qs" %} does not reside in a single, isolated tier. It is an orthogonal vertical projection crossing the execution core ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §2.5):

```
                  VERTICAL PROJECTION OF Qs ACROSS THE EXECUTION CORE (L1–L3)
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ CD 3+ COMPOSITION DEPTH — beyond the execution ceiling, not a layer         │
 │ Multi-party contracts, modal epistemology (Fact vs. Fiction),               │
 │ human-agent sociology, Chronosa Outer-Ring intent synthesis.                │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 3 — EXECUTION CEILING: Symbolic Semantic Graphs (CD 2 / Qexpr CAE-DAG)│
 │ Content-Addressed Semantic DAGs: sentences, dialogues, and contexts         │
 │ structured without floating-point drift; agnostic to generating models.     │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 2: Governed Relational Morphisms (Complexity Degree 1)                │
 │ Atomic actions, verbs, capability delegations, and legal liability          │
 │ state transitions (preserving conserved social residuals r_social).         │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 1: Ontological Genesis Context (Complexity Degree 0)                  │
 │ Foundational distinction between Empty Context Q(0) and Unit Intent Q(1).   │
 └─────────────────────────────────────────────────────────────────────────────┘
```

* **CD vs. Layer Note:** Complexity Degree (CD) indexes structural composition depth, not layer numbers. The LAYER 1–3 bands name the execution-core strata crossed by the Qs projection; their CD 0/1/2 labels measure composition depth accumulated at each stratum, and CD 3+ marks depth accumulated above the ceiling without implying a Layer 4.

### 2.1. Layer 1 — Contextual Genesis (Complexity Degree 0)
At {% include term.html id="layer-1" %}, $\mathrm{Qs}$ establishes the primitive origin of meaning:
* {% include term.html id="q0" text="Q(0)" %} acts as the **Empty Context** (the unwritten page, the silent void, zero unfulfilled obligation).
* {% include term.html id="q1" text="Q(1)" %} acts as the **Unit Intent Measure** along dimensional axis $d_1$, calibrating the baseline quantum of intentional focus and thermodynamic attention.

### 2.2. Layer 2 — Governed Relational Morphisms (Complexity Degree 1)
At Layer 2, $\mathrm{Qs}$ establishes atomic relational transformations:
* Verbs, capability delegations, and authorizations executed under explicit {% include term.html id="dcc-profile" text="DCC Profiles" %}.
* Every action partitions an obligation into an authorized transition and an exact **conserved social residual ($r_{\mathrm{social}}$)**, ensuring that unfulfilled liabilities are never silently discarded.

### 2.3. Layer 3 — Symbolic Composition Graphs (Complexity Degree 2)
At Layer 3, $\mathrm{Qs}$ structures compound propositions, dialogues, and multi-agent interaction histories as **Content-Addressed Expression DAGs ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}))**:
* Identical semantic sub-trees and legal clauses share immutable cryptographic hashes via {% include term.html id="cae-dag" %} hash-consing.
* Incurs zero redundant bit allocation across the Memory Wall via {% include term.html id="q-reuse" text="Q(reuse)" %}.

### 2.4. CD 3+ Composition Depth — Epistemic Modality and Social Systems
Above the Layer 3 execution ceiling, $\mathrm{Qs}$ accumulates Complexity Degree 3+ composition depth — emergent social capability, not additional execution layers:
* The formal separation between physical facts and sovereign fiction via {% include term.html id="epistemic-anchorage" %}.
* Multi-party consensus synthesized by {% include term.html id="chronosa" %} over public road receipts.

---

## 3. Words and Grammar as Multi-Axial Clifford Blades

In $\mathrm{Qs}$, vocabulary and linguistic relationships are formalized through **Clifford Geometric Algebra ($\mathcal{C}\ell_{p,q}$)** over an orthogonal frame of semantic dimensions ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §2.1):

$$\mathcal{D}_{\mathrm{semantic}} = \{ d_{\mathrm{intent}}, \, d_{\mathrm{consent}}, \, d_{\mathrm{obligation}}, \, d_{\mathrm{temporal}}, \, d_{\mathrm{agent}}, \, \dots \}$$

```
                  LINGUISTIC CLUSTER AS CLIFFORD BLADES
 Noun Blade v_noun : Grade-1 vector along basis e_k (e.g., e_device)
 Modifier Rotor R  : Rational Cayley Rotor in plane e_i ∧ e_j (e.g., "secure")
 Transitive Verb M : Bivector contraction mapping Subject to Object
                     ⟨ v_subject ∧ M_verb,  v_object ⟩ + r_social
```

### 3.1. Nouns as Graded Vector Blades
An entity or concept ("battery", "drone", "contract") is a discrete vector blade evaluated at declared {% include term.html id="zoom-z" text="Zoom z" %}:

$$\vec{v}_{\mathrm{noun}} = \sum_{k=1}^n \langle \sigma_k, \, q_{z,k}, \, z, \, r_{z,k}, \, d_k \rangle \, \vec{e}_k$$

Nouns carry explicit identities, coordinate axes, and conserved residuals.

### 3.2. Adjectives and Modifiers as Rational Cayley Rotors
Modifiers ("cold", "verified", "urgent") do not mutate the fundamental type of a concept. A modifier is a **Rational Cayley Rotor** ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5.2) rotating the noun blade within semantic sub-spaces:

$$\vec{v}' = R \vec{v}_{\mathrm{noun}} R^{\dagger}, \quad \text{where } R = \frac{Q(1) - B}{Q(1) + B}$$

Because $R R^{\dagger} \equiv Q(1)$, **semantic modifications preserve unitary norm by construction**: meaning transforms with zero precision drift, zero semantic degradation, and zero floating-point rounding.

### 3.3. Verbs as Conserved Relational Morphisms
A verb is not an ambiguous text string. In $\mathrm{Qs}$, a transitive verb ("Human A delegates task T to Agent B with budget W") is a governed morphism evaluated through Diophantine conservation:

$$\text{Action} = \big( \text{Delegation} \times \text{Actor} \big) + r_{\mathrm{social}}$$

If the task fails, encounters a fault, or halts, the residual $r_{\mathrm{social}}$ remains sealed in the lineage. The system holds an immutable record of unfulfilled obligations, precluding silent state abandonment.

### 3.4. Fixed Normative Axes vs. Open-Vocabulary Lexical Payloads [v1.3.0 — CANDIDATE]

The dimensional ceiling on semantic frames (

$$\mathrm{Dim}_{\max} \le 16$$

, [`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5.2, since the blade count scales as $\mathcal{O}(2^n)$ ) is **not a vocabulary limit**.

The basis dimensions of 

$$\mathcal{D}_{\mathrm{semantic}}$$

are fixed **normative and thematic axes** ( $d_{\mathrm{intent}}$, $d_{\mathrm{consent}}$, $d_{\mathrm{obligation}}$, $d_{\mathrm{temporal}}$, $d_{\mathrm{agent}}$, … ), not per-word slots: allocating a distinct basis blade to every open-vocabulary noun would exhaust the ceiling after the first handful of terms, collapsing the frame algebra.

Open-vocabulary lexical identifiers ("battery", "drone", "contract", …) instead reside as **content-addressed commitments inside the blade coefficients**: a noun blade's coefficient tuples (§3.1) carry the $\mathcal{H}$-commitments of the entity's Layer-2 and numeric-leaf definitions, and two occurrences of the same term share one commitment via CAE-DAG hash-consing (§2.3). Adding a new word to the discourse therefore allocates coefficient data inside existing axes and never a new basis vector. Under the {% include term.html id="no-implicit-rule" %}, any application that genuinely requires an additional normative axis must declare it explicitly in its frame specification and accept the $\mathcal{O}(2^n)$ blade-count consequence, rather than silently overloading an existing axis.

---

## 4. The Epistemic Anchorage Spectrum: Anti-Hallucination Firewall

To eliminate AI hallucinations, $\mathrm{Qs}$ outlaws ungrounded assertions under the {% include term.html id="no-implicit-rule" %}. Every statement in $\mathrm{Qs}$ must declare its explicit **{% include term.html id="epistemic-anchorage" %}**:

```
                       THE EPISTEMIC ANCHORAGE SPECTRUM
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. MODAL_EMPIRICAL (Grounding G = 1.0)                                      │
 │    Anchored directly to Layer 0 physical sensor receipts (NIST entropy,     │
 │    thermocouple, camera, bus telemetry) via a SIMEMP Gateway.               │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. MODAL_CONSTRUCTIVE (Grounding G = 1.0)                                   │
 │    Algebraic closure proven via Qm / DEC; remainder r = Q(0) (Terminal      │
 │    Exactness receipt). Cannot drift or hallucinate by construction.         │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. MODAL_HYPOTHESIS (Grounding 0.3 < G < 1.0)                               │
 │    Explicitly bounded within the Recursive Search Loop (simemp.md §2).      │
 │    Declared as a working conjecture awaiting physical binary falsification. │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 4. MODAL_NARRATIVE (Contextually Sealed Creative Worldbuilding)             │
 │    Internally consistent narrative anchored to author identity Q(0)_author. │
 │    True WITHIN the declared context; barred from triggering actuators.      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 5. MODAL_UNANCHORED (Grounding G = ∅)                                       │
 │    Claims empirical or logical truth without parent lineage receipts.       │
 │    REJECTED AT THE GATEWAY! (RECEIPT_UNANCHORED_ASSERTION_REJECTED)         │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 4.1. The Lineage Verification Mandate
Under **Axiom 4** and **Axiom 5** ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }})), a statement cannot claim empirical truth through latent probability alone.  
If an artificial agent asserts: *"The motor temperature is 450°C"*, the gateway queries its lineage:
* If the lineage contains an unbroken causal chain terminating in a verified {% include term.html id="layer-0" %} hardware sensor receipt, it is admitted as `MODAL_EMPIRICAL`.
* If the assertion traces only to an internal neural probability distribution without sensor receipts, the gateway intercepts it:

$$\text{Gateway Action: } \mathrm{Reject}\Big(\mathcal{H}(\mathrm{Statement})\Big) \implies \text{Emits Receipt}$$

The gateway emits the rejection receipt `RECEIPT_UNANCHORED_ASSERTION_REJECTED`; the claim is physically barred from entering the execution pipeline.

### 4.2. Contextual Isolation for Creative Fiction
$\mathrm{Qs}$ does not prohibit human imagination, creative writing, or artistic worldbuilding:
* A creative work (such as a fantasy novel or game narrative) is typed as `MODAL_NARRATIVE` anchored to the author's sovereign identity $Q(0)_{\mathrm{author}}$ and publisher receipt.
* The internal consistency of the fictional universe is preserved within its closed `Qexpr` sub-DAG.
* The gateway guarantees **contextual containment**: a fictional sub-DAG is barred from ever triggering real-world physical actuators, financial registers, or legal delegations.

### 4.3. The Source-Receipt Invariant (Proposed, CANDIDATE)

**Status.** This subsection records a proposed extension of the §4 firewall from statements to governed artifacts, sourced from [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §3. It carries `State: CANDIDATE` under **Axiom 14**: it remains subject to deterministic measurement and {% include term.html id="boc-policy" text="BOC" %} selection through the pipeline of §6, and it amends nothing in §4.1 or §4.2, which continue to apply exactly as written.

It is proposed that every governed artifact of the {% include term.html id="qn" %} universe crossing a {% include term.html id="simemp-gateway" %} declare an explicit pair:

**(content modality, source class)**

where the content modality is drawn from the {% include term.html id="epistemic-anchorage" %} spectrum of §4, and the source class is a declared expression of where the artifact's asserted content ultimately anchors. Under the {% include term.html id="no-implicit-rule" %}, the declaration makes explicit — at the level of composite artifacts and worlds — what the gateway already enforces implicitly at the level of single statements.

```
        ARTIFACT-LEVEL SOURCE DECLARATION (PROPOSED, STATE: CANDIDATE)
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ Governed Artifact  :=  ( ContentModality , SourceClass )                    │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ ContentModality : one modality of the §4 spectrum, assigned per             │
 │                   atomic claim decomposition of the artifact                │
 │ SourceClass     : declared expression of where the asserted content         │
 │                   ultimately anchors (sealed grounding-class vocabulary     │
 │                   I / P / S; Q5, docs/qs_history.md §2)                     │
 │ SourceReceipts  : unbroken chain terminating in physically receipted        │
 │                   reality events; no dangling source references             │
 └─────────────────────────────────────────────────────────────────────────────┘
```

The invariant requires that the source receipt chain terminate in physically receipted reality events, with the terminating receipt type fixed by the declared modality:

* **Empirical content** terminates in verified {% include term.html id="layer-0" %} sensor or artifact receipts, exactly as mandated by §4.1.
* **Constructive content** terminates in algebraic closure receipts whose remainder $r = Q(0)$ (Terminal Exactness).
* **Contextually sealed creative content** terminates in the author-identity ($Q(0)_{\mathrm{author}}$) and publisher-receipt anchoring already mandated by §4.2.

The extension therefore applies the §4.1 statement-level rejection rule to composite artifacts and worlds: no unsourceable fiction, no unsourced anchor, no history-less constant. An artifact whose source receipt chain cannot be completed is rejected at the gateway exactly as an unanchored statement is, emitting the same class of rejection receipt.

Two cautions are recorded, and they are binding on any future implementation of the invariant:

1. **Payload sovereignty is untouched.** The invariant governs artifacts that cross the gateway as governed objects of the {% include term.html id="licensed-work" %}. Sovereign human [`Payloads`]({{ '/glossary.html' | relative_url }}#payload) remain uninspected behind the safe-harbor air gap ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.1). Where a creative work is declared as a governed `MODAL_NARRATIVE` artifact, its anchoring to author identity and publisher receipt is **declared by its author**, not extracted by inspection.
2. **Simulation modality is two-faced.** A simulation's execution is `MODAL_CONSTRUCTIVE` (deterministic algebraic closure), while its claims *about reality* are `MODAL_HYPOTHESIS` until physically falsified. The pair declaration makes this two-faced split explicit rather than leaving it implicit, in accordance with the {% include term.html id="no-implicit-rule" %}.

The proposed Anchor Grounding Classes of [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §2 (Classes I / P / S) supply the candidate vocabulary for the source-class field. Glossary registration of the newly named concepts — and the final sealing of this invariant itself — remain deferred to the integration queue of that document (§8), preserving its discipline against dead glossary entries for unmeasured concepts.

### 4.4. The Parser-Boundary Modality Ceiling [v1.3.0 — CANDIDATE]

The §4 firewall inspects statements and artifacts that are already formalized. The remaining trust gap lies **upstream of formalization**: translating a natural-language utterance into a structured CAE-DAG and a `DCC_INTENT_CONTRACT_v1` relies on a statistical or neural **candidate parser** (§6, step 1) crossing the **Axiom 14** boundary. 

No statistical parser guarantees 

$$H(\text{Intent}_{\mathrm{human}} \mid \text{Utterance}) = 0$$

— {% include term.html id="dependencies-determinism" %} governs the formalized state, not the pre-formalization act of interpretation. 
If the parser hallucinates or inverts the human's actual intent while attaching a valid human origin signature 
$Q(0)_{\mathrm{author}}$ and well-formed `MODAL_EMPIRICAL` or `MODAL_CONSTRUCTIVE` tags, then every downstream check — the SIMEMP gateways, the Triadic Grounding of §7 (

$$\mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$$

), and the QnIR anti-drift invariant 

$$H(\text{Intent} \mid \text{Execution}) = 0$$

— will validate the *wrong* intent with perfect rigor, because the error occurred prior to formalization.

**Parser-boundary modality ceiling (rule).** 

Any candidate DAG produced by a natural-language parser enters the generative pipeline of §6 typed **no higher than `MODAL_HYPOTHESIS`**. Direct admission at `MODAL_EMPIRICAL` or `MODAL_CONSTRUCTIVE` is barred — including when the parser itself attaches those tags — until the candidate passes the Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake of §6.1.

* A parsed intent is, epistemically, a **hypothesis about what the principal meant**; `MODAL_HYPOTHESIS` is its correct home in the §4 spectrum — a working conjecture whose only competent falsifier is the principal's own confirmation. Parser confidence scores are not {% include term.html id="epistemic-anchorage" %}: under the {% include term.html id="no-implicit-rule" %} they confer no grounding value and must not be mapped onto $G$.
* The upgrade path is exactly one: §6.1, where the sole authority over the intent's meaning — the human principal — supplies the anchorage that the parser cannot.
* `MODAL_NARRATIVE` (§4.2) is unaffected: creative content remains author-declared and contextually contained. `MODAL_UNANCHORED` claims remain rejected at the gateway (§4, class 5), and the §4.3 artifact-level invariant continues to apply exactly as written.

---

## 5. Human-Agent Alignment: The Intent Contract in DCC Profiles

In the Qn universe, alignment between humans and artificial agents is not achieved by fine-tuning neural weights (RLHF). It is enforced through **Structural Intent Contracts** embedded inside the {% include term.html id="dcc-profile" %}:

```
                     ANATOMY OF AN INTENT CONTRACT
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ DCC_INTENT_CONTRACT_v1                                                      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ • Sovereign Anchor : Hash(pk_human_principal) [Human Identity Q(0)_H]       │
 │ • Agent Proxy      : Hash(pk_agent) [Identity Class: AI_AGENT_IDENTITY]     │
 │ • Permitted Domain : Multi-axial frame D_allowed (Spatial / Financial / Act)│
 │ • Metric Budget    : Max Work Allocation W_allowed in Q(1) units            │
│ • Epistemic Scope  : Allowed Modalities {MODAL_EMPIRICAL, MODAL_HYPOTHESIS}  │
 │ • Revocation Path  : Immediate un-delegation on human signature             │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 5.1. Subordination of Machine Agency
Under the {% include term.html id="identity-class" %} rules ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §8.1):
1. An AI agent possesses **zero independent root sovereignty**. It cannot instantiate an unanchored genesis origin $Q(0)_{\mathrm{agent}}$.
2. An agent executes strictly as an authorized proxy under an unbroken delegation chain originating from a human principal ({% include term.html id="q0" text="Q(0)_H" %}).
3. If the human principal revokes the capability token, the agent's execution authority terminates instantly.

### 5.2. The Thermodynamic Leash
Autonomous agents cannot self-replicate or consume unmetered resources. Under **Axiom 6** (Totality of Governed Operations), every agentic workflow is bounded by a finite work budget in units of {% include term.html id="compute-unit-u" text="U" %} via {% include term.html id="metric-exhaustion" %}. When the allocated budget reaches zero, the agent physically halts, emitting a signed termination receipt.

### 5.3. Logical Typing of the Social Residual (Proposed, CANDIDATE) [v1.3.0 — CANDIDATE]

**Status.**

This subsection records a candidate logical anchoring for the conserved social residual $r_{\mathrm{social}}$ of §2.2 and §3.3. It carries `State: CANDIDATE` under **Axiom 14**: it remains subject to deterministic measurement and {% include term.html id="boc-policy" text="BOC" %} selection through the pipeline of §6, and it amends nothing in §5.1 or §5.2, which continue to apply exactly as written.

Sections §5.2 and §3.3 conserve $r_{\mathrm{social}}$ but leave its logical type unspecified: representing $\text{Action} = (\text{Delegation} \times \text{Actor}) + r_{\mathrm{social}}$ as a Diophantine conservation constraint does not by itself say how multi-clause legal or safety obligations, deadline expirations, or secondary reparations are encoded into the residual once a primary obligation is only partially violated. The reviewer proposal recorded here anchors the semantics of `DCC_INTENT_CONTRACT_v1` in **resource-bounded dyadic deontic logic with contrary-to-duty (CTD) obligations**, composed with **dynamic epistemic logic (DEL)** for multi-agent state updates.

Under this candidate reading:

* Primary obligations

  $$\mathcal{O}_{\mathrm{actor}}(\alpha \mid W_{\mathrm{allowed}})$$

  , permissions $P(\alpha)$, and prohibitions $F(\alpha)$ carry explicit work budgets drawn from the contract's Metric Budget field.
* When a primary obligation goes unfulfilled, a **reparational secondary obligation** $\mathcal{O}(\beta \mid \neg\alpha)$ takes effect by construction — the contrary-to-duty structure.
* This gives $r_{\mathrm{social}} \neq Q(0)$ a rigorous logical type: a **persistent CTD obligation sub-DAG** that remains in the lineage until discharged by a matching receipt. Multi-clause obligations, deadline expirations, and secondary reparations then encode as additional CTD structure on the residual instead of silently vanishing at partial violation, which is exactly the anti-abandonment property §3.3 mandates.

The full formal calculus — sequent presentation, discharge rules, and the interaction of CTD structure with the **Axiom 6** terminal states — is deferred to a dedicated versioned update. This subsection records the candidate anchoring only, so that downstream files do not hard-code an untyped reading of $r_{\mathrm{social}}$ in the interim.

**Committed form and finite bounds [v1.3.1].** Until that update, $r_{\mathrm{social}}$ is scoped strictly as an **opaque Layer-3 commitment handle** `H(α_rem)` carried in `qir-enc-1` `EXT` key `7`; no consumer may interpret its bytes or infer CTD structure from the handle itself. This subsection remains gated `[CANDIDATE / NON-PROMOTING]` until finite DEL bounds are sealed: `W_worlds_max` on the possible-worlds enumeration and `K_max` on modal update depth, with exhaustion emitting `RECEIPT_BUDGET_EXHAUSTED` and `RECEIPT_EXPRESSION_TOO_COMPLEX` respectively. The handle commits to a residual sub-DAG; it is not a new epistemic credential, and §5.1/§5.2 continue to apply unchanged.

---

## 6. The Generative Candidate Pipeline (Q(rsi) Integration)

$\mathrm{Qs}$ is completely paradigm-agnostic: it does not reject generative AI models (transformers, diffusion, active inference). It incorporates them into the **Recursive Search Loop** ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2, §6):

```
                     THE GENERATIVE Q(rsi) RECURSIVE LOOP
                      ┌───────────────────────────────┐
                      │ PRIOR STATE / CONTEXT S_k     │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ GENERATIVE EXPLORATION STEP   │
                      │ (Neural / Stochastic Engine)  │
                      │ Proposes Candidate C_1 ... C_n│
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ SIMEMP MEASUREMENT & AUDIT    │
                      │ • Check Depth & Node Ceilings │
                      │ • Verify Intent Constraints   │
                      │ • Measure Metric Burn (W)     │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ BOC SELECTION (simemp.md §7)  │
                      │ ArgMax( SIMEMP Compliance )   │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ SEALS SUCCESSOR ARTIFACT      │
                      │ State S_(k+1) via Q(rsi)      │
                      │ • Advances Causal Index κ     │
                      │ • Emits Verification Receipt  │
                      └───────────────┬───────────────┘
                                      │
                                      └───────► (Feeds next generation loop)
```

1. **Agnostic Generation:** An AI model (e.g., an LLM or multi-agent swarm) parses messy natural human speech and proposes a candidate semantic graph. Under **Axiom 14**, this graph is tagged as `State: CANDIDATE`; and as a parser-derived artifact it is admitted **typed no higher than `MODAL_HYPOTHESIS`** (§4.4). A failure at the parse boundary itself — ambiguity, modality conflict, or schema violation — emits a typed receipt (§6.2) rather than an untyped rejection.
2. **Deterministic Measurement:** The candidate is translated into a Content-Addressed Expression DAG ([`Qexpr`]({{ '/qexpr.html' | relative_url }})) and audited against the declared Intent Contract, checking depth ceilings, metric budgets, and geometric consistency.
3. **Selection via BOC Policy:** The candidate that maximizes compliance with SIMEMP constraints is selected under the {% include term.html id="boc-policy" %}:

$$\text{Selection} = \arg\max_{c \in \{C_1, \dots, C_n\}} \left( \text{SIMEMP Compliance}(c) \right)$$

If the argmax is **degenerate** — multiple non-isomorphic candidate DAGs fit the prompt within the BOC compliance threshold — the pipeline does not silently seal an arbitrary winner. It emits `RECEIPT_INTENT_PARSE_AMBIGUOUS` (§6.2), surfaces the ambiguity to the principal, and defers to the §6.1 handshake before any sealing decision. This is the intent-level analogue of the numeric indeterminacy discipline of [`docs/qm.md`]({{ '/qm.html' | relative_url }}): an unresolved tie is a measured outcome, not a hidden coin flip.

4. **Sealing via {% include term.html id="qn-rsi" text="Q(rsi)" %}:** The selected candidate is sealed into `State: SEALED` as successor artifact $S_{k+1}$, advancing the monotonic causal index along the {% include term.html id="causal-arrow" %}. For parser-derived candidates carrying a `DCC_INTENT_CONTRACT_v1`, sealing additionally requires that the §6.1 handshake has returned a principal co-signature; without it, the candidate remains `MODAL_HYPOTHESIS` and is never sealed. Sub-optimal candidates emit negative receipts, ensuring the system never repeats failed paths.

**Metering.** Every stage of the pipeline — parsing, deterministic measurement, BOC selection, verbalization, and co-signature verification — consumes finite compute and memory work and is metered under **Axiom 6**. Generation behind the stochastic boundary remains free; the attribution of gateway-crossing validation work to the invoking principal's remaining budget is a [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) concern and is deferred to that document's next versioned update.

**Verbalization fee [v1.3.1].** Deterministic verbalization (`qir-verbalizer-1`, step 1 above) is a **pre-seal boundary transform**: it runs before the co-signature decision and is billed as a static, size-linear fee `W_verbalize(|Contract|)` inside the outer gateway fee `W_gate` fixed in `docs/simemp.md` §6 — not as an inner QnIR work posting. The charge is recorded regardless of the §6.2 outcome, including `RECEIPT_ROUNDTRIP_CONFIRMATION_REJECTED`, because the canonical read-back was produced in full before the principal could reject it.

**Human wait latency [v1.3.1].** The interval between presentation of the read-back and the principal's co-signature is not unmetered ambient time: the invocation enters the `PAUSED` lifecycle state of `docs/boc.md` §10 at an **atomic checkpoint receipt**, established before the transition to `RUNNING` on co-signature. `PAUSED` is a lifecycle state and not a BOC outcome; on rejection the invocation is `ABORTED` with its checkpoint preserved, and no sealing decision is manufactured from the wait.

### 6.1. The Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake [v1.3.0 — CANDIDATE]

The deterministic pipeline of steps 1–4 closes every gap *inside* the formalized universe, but it cannot close the one gap that arises *before* formalization: whether the formal object matches what the human actually meant (§4.4). The handshake closes that gap by returning the formal object to its author for confirmation, before the universe treats it as intent.

**Protocol.**

1. **Verbalization.** A **deterministic verbalizer** $\mathcal{V}$ renders the formal `DCC_INTENT_CONTRACT_v1` — including its Permitted Domain, Metric Budget $W_{\mathrm{allowed}}$, and Epistemic Scope — back into an unambiguous **controlled natural language template** $\mathcal{T}$, the *canonical read-back*. $\mathcal{V}$ is a pure function of the formal contract, $H(\mathcal{T} \mid \text{Contract}) = 0$: no neural or stochastic component participates in the read-back path, per the {% include term.html id="no-implicit-rule" %}.
2. **Presentation.** $\mathcal{T}$ is presented to the human principal $Q(0)_{\mathrm{H}}$ inside the **Inner Ring** ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.3), alongside the candidate DAG it was verbalized from.
3. **Co-signature.** The principal either co-signs the pair $(\text{Contract}, \mathcal{T})$ cryptographically, or rejects it. Silence and absence of signature are not consent: without a co-signature the candidate remains `MODAL_HYPOTHESIS` and is **never sealed** (§6, step 4).
4. **Outcome.** On co-signature, the human-anchored intent is established: the co-signed contract becomes the parent lineage receipt that §4.1 chains previously lacked, downstream statements are typed per §4 exactly as any governed statement, and sealing proceeds under Q(rsi). On rejection, `RECEIPT_ROUNDTRIP_CONFIRMATION_REJECTED` (§6.2) is emitted, the candidate is discarded, and the negative receipt is sealed in the lineage so the failed parse is never silently retried.

**Why a round trip.** The direction of the check matters. Re-asking the parser "did I understand you right?" would only measure the parser's agreement with itself. Rendering the *formal* contract back into language and requiring the principal's signature instead tests the one thing the parser cannot self-certify: that the formalization and the intent coincide. The handshake is mandatory for parser-derived candidates carrying a `DCC_INTENT_CONTRACT_v1`; candidates formalized directly by the principal, with no parser step, are unaffected.

### 6.2. Typed Parser-Boundary Failure Receipts [v1.3.0 — CANDIDATE]

The receipt catalog of the Qn universe is closed but extensible by versioned updates. This update extends it with four Qs-domain parser-boundary receipts (naming record in the Review Integration Record):


These receipts make parser failures first-class, auditable lineage events. Under the **Non-Rogue Invariant**, an agent that repeatedly fails at the parse boundary is visible in the very lineage it emits; and the negative receipts of §6, step 4 gain the typed granularity needed to distinguish "the parser was ambiguous" from "the principal said no" — two failure modes that demand different remediation, and that an untyped rejection would conflate.

---

## 7. Triadic Intent Synthesis (Qs ⊗ Qm ⊗ Qphy)

True communication between humans and computational tools requires simultaneous verification across the foundational triad:

$$\mathbf{C}_{\text{Intent}} = \mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$$

```
                         THE TRIADIC GROUNDING OF A COMMAND
 Proposed Command: "Accelerate the drone to 120 km/h across sector 4"
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            ▼                          ▼                          ▼
     Qs (Semantics)              Qm (Mathematics)           Qphy (Physics)
 ┌──────────────────────┐   ┌──────────────────────┐   ┌──────────────────────┐
 │ Human Intent Contract│   │ Formal Decidability  │   │ Thermodynamic Work   │
 │ • Is User Q(0)_H     │   │ Is rotor trajectory  │   │ Does battery have    │
 │   authorized to fly? │   │   algebraically exact│   │   sufficient Q(1)    │
 │ • DCC capability     │   │   without division by│   │   metric budget?     │
 │   token verified.    │   │   zero?              │   │ • Landauer heat safe.│
 └──────────┬───────────┘   └───────────┬──────────┘   └───────────┬──────────┘
            └──────────────────────────┼──────────────────────────┘
                                       ▼
                       GATEWAY DETERMINISTIC CONSENSUS
      [ Valid: Executes Action ]  OR  [ Invalid: Emits Rejection Receipt ]
```

When an intent command crosses a gateway:
1. **{% include term.html id="qs" %} validates intent:** Verifies the human authority chain, the declared modal grounding, and the contractual goal.
2. **{% include term.html id="qm" %} validates decidability:** Proves that the spatial rotors and coordinate trajectories contain no singularities, division-by-zero, or precision drift.
3. **$\mathrm{Qphy}$ validates realizability:** Proves that the command fits within the physical battery energy and Landauer dissipation budgets ($\Delta S \le \mathrm{Budget}(Q(1))$).

If all three domains verify the transition, the command executes. If any domain fails, execution halts cleanly. The tool cannot decouple from the human because the mathematical definition of an executable action is structurally bound to human intent — and, since v1.3.0, the intent object itself is anchored at its source: it is either formalized directly by the principal, or parser-derived, held at `MODAL_HYPOTHESIS` (§4.4), and confirmed through the §6.1 co-signature before any triadic verification can execute it.

---

## 8. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %} governing the {% include term.html id="licensed-work" %} across the {% include term.html id="cdqn" %} network:

1. **{% include term.html id="paternity-reference" %}:** All semantic parsing engines, intent contract compilers, and human-agent alignment modules derived from this document must embed:
[[/CODE]]

### II.5 Qlog — Quang Logics v1.2.0 — `qlog.md`

[[CODE]]
# Qlog — Quang Logics: Constructive Type Theory, Pattern Verification, and Epistemic Deduction


---

## 1. Epistemic Stance and Operational Demarcation

### 1.1. Constructive Working Hypothesis Notice
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It represents a constructive theoretical baseline defining the emergent composite domain {% include term.html id="qlog" text="Qlog (Quang Logics)" %}:

$$\mathbf{C}_{\mathrm{Qlog}} \equiv \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qs}}$$

All operational models herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until physical execution feedback is derived across silicon runtimes. All derivations preserve architectural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. The Rejection of Platonic Deductions for Computed Patterns
Classical mathematical logic frequently relies on unmetered search spaces ($\mathbb{Z}^n, \mathbb{Q}^n$) and non-constructive principles such as the Law of Excluded Middle ($\mathrm{LEM}$). In the {% include term.html id="qn" %} universe, unanchored mathematical deductions are discarded as physical fictions (`docs/reflections/h10_finite_computation.md` §4).

Truth is operationalized strictly as a **Verified Computed Pattern** certified by an immutable, dissipative {% include term.html id="receipt" %} satisfying {% include term.html id="dependencies-determinism" %}:

$$H(S_{\mathrm{out}} \mid \mathcal{D}(S_{\mathrm{out}})) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL PLATONIC DEDUCTION         │ Qlog CONSTRUCTIVE PATTERN LOGIC      │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Non-constructive existence claims  │ • Explicit state transition          │
 │   without an algorithm (via LEM).    │   trajectories across discrete steps.│
 │ • Unmetered search in infinite voids │ • Bounded by finite Metric Envelopes │
 │   (Turing non-halting / H10 void).   │   and logarithmic height (H_max).    │
 │ • Zero thermodynamic accounting.     │ • Dissipation metered in Abstract    │
 │                                      │   Compute Units U (W ≥ k_B T ln 2).  │
 │ • Ephemeral text / LaTeX arguments.  │ • Content-Addressed Expression DAGs  │
 │                                      │   sealed by immutable Road Receipts. │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Taxonomy of Causal Entities: Q(patterns) vs. Q(workflows)

### 2.1. Quang Patterns (Q(patterns))
A {% include term.html id="q-patterns" text="Q(pattern)" %} is an invariant, reproducible state-transition segment mapping an input state commitment to a deterministic output state milestone under an abstract capability contract:

$$\mathcal{P}: \mathcal{H}(S_0) \xrightarrow{\quad \mathcal{M}_{\mathrm{contract}} \quad} \mathcal{H}(S_1)$$

Quang Patterns capture structural, algebraic, and physical causality. They are intent-neutral and evaluate identically across all nodes, whether resolving to `SUCCESS_EXACT` ({% include term.html id="terminal-exactness" %}) or an invariant terminal fault (e.g., `RECEIPT_DIVISION_BY_ZERO_REJECTED`).

### 2.2. Quang Workflows (Q(workflows))
A {% include term.html id="q-workflows" text="Q(workflow)" %} is a goal-directed, composite causal directed acyclic graph assembling multiple `Q(patterns)` under an explicit {% include term.html id="intent-contract" %} and finite metric budget allocation ($W_{\mathrm{budget}}$). It represents an end-to-end navigational route across the computational landscape, executing human or delegated intent.

### 2.3. Formal Distinctions and Execution Currency


---

## 3. Vertical Projection Across Complexity Degrees

Like {% include term.html id="qm" %} and {% include term.html id="qs" %}, $\mathrm{Qlog}$ is not pinned to a single horizontal layer. It operates as an orthogonal vertical projection crossing the execution core ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §2.5) across all {% include term.html id="complexity-degree" text="Complexity Degrees" %}:

<pre>
                     VERTICAL DEGREE PROJECTION OF Qlog
 ┌────────┬─────────────────────────┬──────────────────────────────────────────┐
 │ DEGREE │ Qlog STRATUM            │ OPERATIONAL LOGICAL MECHANISM            │
 ├────────┼─────────────────────────┼──────────────────────────────────────────┤
 │ Deg 0  │ Genesis Ontological     │ Duality of Empty Void Q(0) (Absurdity/⊥) │
 │        │ Baseline                │ vs. Unit Existence Q(1) (Truth/⊤).       │
 ├────────┼─────────────────────────┼──────────────────────────────────────────┤
 │ Deg 1  │ Atomic Propositional &  │ Direct constraint deduction, sign parity │
 │        │ Parity Calculus         │ (σ), single-digit modulo parity (Q(2)).  │
 ├────────┼─────────────────────────┼──────────────────────────────────────────┤
 │ Deg 2  │ Constructive Type Theory│ Dependent types (Π, Σ) over Qexpr CAE-DAG│
 │        │ & Macro Chunking        │ containers; Macro-Hop Q(patterns).       │
 ├────────┼─────────────────────────┼──────────────────────────────────────────┤
 │ Deg 3  │ Invariant & Topological │ Nilpotent DEC logic (d² ≡ 0), Stokes     │
 │        │ Conservation Logic      │ invariants, sparse Q(anchor) navigation. │
 ├────────┼─────────────────────────┼──────────────────────────────────────────┤
 │ Deg n  │ Epistemic Modality &    │ MODAL_* validation, inverse scientific   │
 │        │ Sheaf Synthesis         │ causal reconstruction, Chronosa Outer-   │
 │        │                         │ Ring consensus, evolution via Q(rsi)     │
 └────────┴─────────────────────────┴──────────────────────────────────────────┘
</pre>

* **CD vs. Layer Note:** Complexity Degree (CD) indexes structural composition depth, not layer numbers. The Layer references in §3.1–§3.3 name the execution-core strata (Layers 1–3) crossed by the Qlog vertical projection; their Degree labels measure composition depth accumulated at each stratum, and Degrees 3 and $n$ mark depth accumulated beyond the Layer 3 execution ceiling without implying a Layer 4.

### 3.1. Degree 0 — Genesis Ontological Baseline
At {% include term.html id="layer-1" %}, logic formalizes the foundation of existence:
- Origin {% include term.html id="q0" text="Q(0)" %} formalizes the empty context, non-assertion, or the uninhabited type ($\bot$).
- First unit {% include term.html id="q1" text="Q(1)" %} formalizes minimal evidence, identity, or the inhabited unit type ($\top$) along dimensional axis $d_1$.

### 3.2. Degree 1 — Atomic Propositional and Parity Calculus
At Layer 2, $\mathrm{Qlog}$ formalizes elementary scalar propositions:
- Evaluates constructive conjunction ($\wedge$), disjunction ($\vee$), and sign parity involution ($\sigma \in \{\ominus, \odot, \oplus\}$).
- Grounds Boolean logic in constructive Diophantine remainders ($r \in \{Q(0), Q(1)\}$) via secondary anchor $Q(2)$ (`DCC_ANCHOR_Q2_PARITY_v1`).

### 3.3. Degree 2 — Constructive Type Theory on CAE-DAGs
At Layer 3, $\mathrm{Qlog}$ structures deduction inside {% include term.html id="qexpr" %} Content-Addressed Expression DAGs ({% include term.html id="cae-dag" %}):
- **Curry–Howard–Qn Correspondence:** Propositions are represented as target types ($\tau$); verified derivations are content-addressed DAGs ($t$).
- **Macro-Hop Deduction:** Compresses multi-step micro-transitions into reusable, coarse-grained `Q(patterns)`.

### 3.4. Degree 3 — Topological and Coboundary Invariance Logic
Interfaces directly with discrete calculus (`docs/qm_calculus.md`):
- Logical consistency is evaluated as topological coboundary nilpotency:
  $$\mathbf{d} \circ \mathbf{d} \equiv Q(0)$$
- Governs trajectory routing across the universal {% include term.html id="q-anchor" %} landmark network.

### 3.5. Degree n — Epistemic Modality, Inverse Reconstruction, and RSI
Above the Layer 3 execution ceiling, $\mathrm{Qlog}$ accumulates Complexity Degree $n$ composition depth — emergent epistemic capability, not additional execution layers — unifying multi-agent interaction and empirical science:
- **{% include term.html id="epistemic-anchorage" %}:** Segregates physical sensor data (`MODAL_EMPIRICAL`) from context-bound worldbuilding (`MODAL_NARRATIVE`).
- **Inverse Causal Reconstruction:** Formalizes empirical scientific discovery by inverting observed macroscopic states to identify minimal causal ancestor DAGs.
- **Recursive Evolution via {% include term.html id="qn-rsi" %}:** Compiles and optimizes the logical engine itself, emitting versioned successor artifacts under **Axiom 13**.

---

## 4. The Stratified Open Framework of Inferential Modes (L_modes)

Under the {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %}, $\mathcal{L}_{\mathrm{modes}}$ is not a static closed set. It is an **open, inductively stratified category**:

<pre>
                  THE STRATIFIED INFERENCE ENGINE (L_modes)
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. FOUNDATIONAL BASE MODES : L_base (Complexity Degree 1)                   │
 │    • Micro-Induction : Succession along axis d1 (S(x) = x + Q(1)).          │
 │    • Direct Deduction: O(1) constraint contraction via Q(bypass).           │
 │    • BOC Abduction   : Minimal causal predecessor selection under SIMEMP.   │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. COMPOSITIONAL MODES (Complexity Degree 2 & 3)                            │
 │    • Macro-Hop Deduction     : Strategic sub-graph chunking into Q(patterns)│
 │    • Anchored Markov Routing : Stochastic pathfinding over Q(anchor) cuts.  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. EMERGENT RSI MODES : L_emergent (Complexity Degree n)                    │
 │    • Adjoined dynamically via Q(rsi) under explicit DCC Profile Contracts.  │
 │    • Examples: Sheaf co-equalization, paraconsistent dissipative sinks.     │
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 4.1. Foundational Base Modes (Degree 1)
- **Micro-Induction:** Succession along axis $d_1$ via discrete morphism $\mathcal{S}(x) = x + Q(1)$.
- **Direct Deduction:** Zero-cost algebraic cancellation in $\mathcal{O}(1)$ register time ($Q(\text{bypass})$), enforcing immediate constraints such as $A \times Q(0) \to Q(0)$ or $R R^{\dagger} \to Q(1)$.
- **BOC Abduction:** Minimal causal predecessor selection from terminal execution states under SIMEMP constraints.

### 4.2. Compositional and Topological Modes (Degrees 2 and 3)
- **Macro-Hop Deduction:** Compresses an acyclic sub-graph of $x$ intermediate operations into a single canonical `Q(pattern)`. Utilizes hash-consing and deduplication ($Q(\text{reuse})$) to ensure the macro-step is referenced at zero marginal re-derivation cost ($\Delta W = 0$) — the deduplicated body is never re-executed; the reference itself pays the declared lookup charge of the §5.2 admission predicate and Admission Ledger.
- **Anchored Markov Routing:** Traverses the sparse topology of universal $Q(\text{anchor})$ landmarks, using historical road receipt weights to bypass $\mathcal{O}(N)$ micro-step re-execution.

### 4.3. Emergent RSI Modes and Admission Invariants (Degree n)
Under **Axiom 11** and **Axiom 14**, any emergent logical mode $M_{\mathrm{new}}$ must satisfy four gateway verification invariants before inclusion into $\mathcal{L}_{\mathrm{modes}}$:
1. **Explicit DCC Contract:** Wrapped in an explicit profile contract (`DCC_LOGIC_MODE_<ID>_vN`).
2. **Deterministic Reduction:** Operates over `Qexpr` CAE-DAGs with zero floating-point semantics.
3. **Finite Landauer Dissipation Ceiling:** Bounded by an explicit work allocation: $W(M_{\mathrm{new}}) \le W_{\mathrm{budget}}$ in units of $U$.
4. **Receipt-Emitting Totality:** Terminates unconditionally in a declared terminal receipt (`SUCCESS`, `INCONCLUSIVE`, `BUDGET_EXHAUSTED`, or `REJECTED`).

---

## 5. Inferential Optimization Morphisms: Q(bypass) and Q(reuse)

In $\mathrm{Qlog}$, optimization morphisms follow strict causal precedence:

$$Q(\text{bypass}) \prec Q(\text{reuse}) \prec \text{Direct Inferential Compute}$$

### 5.1. Q(bypass): Tautological and Deterministic Fault Short-Circuiting
Before scheduling memory bus transfers or search iterations, the gateway evaluates candidate judgments in $\mathcal{O}(1)$ register time:
- **Reflexive Identity ($A \vdash A$):** If the goal commitment hash matches the active premise hash, search halts immediately, emitting `RECEIPT_BYPASS_IDENTITY` at zero metric cost ($\Delta W = 0$).
- **Topological Nilpotency:** Inferences involving discrete exterior coboundaries ($\mathbf{d} \circ \mathbf{d}$) or boundaries ($\partial \circ \partial$) collapse algebraically to $Q(0)$ without mesh traversal.
- **Deterministic Fault Interception:** Propositions containing illegal operations (such as division by $Q(0)$) are intercepted in $\mathcal{O}(1)$ time, emitting `RECEIPT_DIVISION_BY_ZERO_REJECTED`.

### 5.2. Q(reuse): Lemma Memoization and Inferential Breakeven
When intermediate `Q(patterns)` or sub-workflows are verified, they are indexed by their immutable commitment hash $\mathcal{H}(\mathcal{P})$:
- **CAE-DAG Deduplication:** Identical sub-proofs share a single physical node in memory via {% include term.html id="hash-consing" %}.
- **The Two-Axis Inferential Breakeven Predicate [v1.2.0 — CANDIDATE]:** Hash retrieval across the Memory Wall is admitted only when the direct inferential path is **consequently faster** and cheaper than retrieval — advantage declared on **both axes** of the Layer-0 Hardware Telemetry Envelope ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4), instantiating the canonical predicate of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15/§5.9:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

**Term provenance [v1.2.0 — CANDIDATE].** Every term of the conjunction is a declared deterministic quantity, never a runtime sample. `h` is the referenced lemma commitment; `tier(h)` is its declared memory tier, fixed by the referencing DCC profile's storage binding. `ΔW_compute` is the DCC-declared upper-bound work of direct inferential re-derivation — the search work this domain historically names `ΔW_search`, carried here under the canonical token of `docs/qnPrimitive.md` §1.15 so that admission decisions replay identically across files — produced by the cost model declared in the morphism's DCC profile. `Δt_compute_bound` is the DCC-declared worst-case latency bound of the direct inferential path, a static declared bound under the declared-bound discipline (Loop-2 finding F1 of `docs/AiReviews/AiReviewQnIRv0.2.md`), never a runtime measurement. `τ_U[tier(h)]` and `τ_t[tier(h)]` are the declared per-tier worst-case lookup bounds of the two-axis envelope, calibrated at Layer-0 onboarding through the Historical Calibration Benchmark receipt class; `ε_U` and `ε_t` are the DCC-declared fixed gateway overhead slacks. `κ_dcc` and `κ_t` are the DCC-declared advantage margins within the governance-bounded range `[1, 64]` and recommended preset (`κ_dcc = 2`, `κ_t = 4`) adjudicated at `docs/qnPrimitive.md` §5.9 — the floor excludes bare breakeven. All quantities **MUST** be declared; an undeclared quantity forces the safe fallback to direct inferential compute — trivial single-step inferences execute directly on integer ALUs (§5.1) without hash-table probes — and ties resolve to Direct Inferential Compute. Every admitted reuse executes under the **Admission Ledger** (`W_gross_reserve`, `W_actual_charge`, `W_refund`, `W_net`) of `docs/qnPrimitive.md` §5.9: the executed path pays only the declared lookup charge `W_net = τ_U[tier(h)] + ε_U`, never the reserved direct-compute ceiling. Combinatorially deep derivations therefore never re-derive recurring sub-graphs — the residual cost of a reference is the declared ledger charge, not zero and not a runtime sample.

---

## 6. The Memory Wall Latency Harvester (H_harvester)

In physical hardware architectures, retrieving data across the Memory Wall stalls processor execution for a duration bounded by the touched tier's declared lookup bound `τ_t[tier(h)]` of the two-axis envelope ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4) — historically intuited as hundreds of clock cycles (≈ 50–100 ns), now a **declared per-tier quantity** rather than an approximation — during which silicon continues to dissipate static leakage power ($P_{\mathrm{leak}} = I_{\mathrm{leak}} V$). The consequentiality constraint of §5.2 is what makes this window harvestable rather than merely suffered: an admitted lookup has cleared the `κ_t` margin, so the primary route was slower by a declared factor, and the stall window is time the direct path would have consumed anyway.

<pre>
                   THE MEMORY WALL LATENCY HARVESTER
 Memory Bus Transaction: ──► [ DRAM FETCH / ATTESTATION WAIT (Stall Window) ]
                                          │
                                          ▼
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ Qlog HARVESTER EXECUTION (Local Register Cache Workload)                    │
 │ 1. Background Equality Saturation : Apply OP_REWRITE rules via Q(rsi).      │
 │ 2. Speculative Pre-Validation     : Audit pending Candidate Q(patterns).    │
 │ 3. Anchor Transition Pre-Routing  : Update Markov transition matrices.      │
 └────────────────────────────────────────┬────────────────────────────────────┘
                                          │ Memory data arrives
                                          ▼
 Primary Execution Resumes: ────────► [ Preempts Harvester in O(1) time ]
</pre>

### 6.1. Harvester Workload Scheduling During Stall Windows
During memory-stall windows ($\Delta t_{\mathrm{stall}}$), $\mathrm{Qlog}$ schedules local, register-pinned workloads requiring zero external bus traffic:
1. **Background Equality Saturation (`Q(rsi)`):** Rewrites cached `Qexpr` DAGs using equivalence rules to find smaller structural footprints, minimizing future execution budgets.
2. **Speculative Candidate Auditing:** Evaluates unverified `State: CANDIDATE` transitions against algebraic invariants. Failed candidates are pruned prior to bus resumption.
3. **Anchor Route Pre-computation:** Re-calculates transition weights between local $Q(\text{anchor})$ landmarks based on recent receipt metrics.

### 6.2. Constitutional Guardrails and Preemption Semantics
- **$\mathcal{O}(1)$ Preemption:** Background tasks yield or commit in a single clock cycle the instant data arrives on the bus, adding zero latency to primary execution.
- **Deterministic Sandboxing:** Harvester tasks execute in read-only registers and cannot mutate sealed state.
- **Thermal Metering:** Background execution is throttled by the local {% include term.html id="metric-envelope" %}, halting if temperature telemetry approaches hardware limits.

### 6.3. Deterministic Prefetch, Overlap Credit, and the Never-Feeds-Back Boundary **[v1.2.0 — CANDIDATE]**

**Deterministic Tier Prefetch [v1.2.0 — CANDIDATE].** During a stall window on tier `t`, the harvester may issue read-ahead requests for commitments whose `Q(reuse)` admission is already declared by the two-axis predicate (§5.2) along the sealed Chained Receipt Vector of the active `Q(workflow)` (§2.3), under a deterministic schedule pinned in the referencing DCC profile. Prefetch is a **scheduled bus action, not a stall-window workload**: it never blocks the `O(1)` preemption guarantee of §6.2, it is restricted to already-admitted lookups so that no speculative traffic is ever priced outside the declared envelope, and its latency is metered against the same declared tier table `τ_t`.

**Overlap Credit [v1.2.0 — CANDIDATE].** Harvester work completed inside a stall window (`Δt_stall ≤ τ_t[tier(h)]`) is recorded as declared overlap telemetry on the node-boundary receipt: the tier touched, the declared window bound, and the identity of the completed stall-window workloads. Overlap credit is **calibration and auditing data** — it feeds the Historical Calibration Benchmark receipt class (for example, validating the preset margin `κ_t` against the realized machine) — and it is **never an input to an admission predicate or a logical budget**: admission is decided solely by the two-axis predicate of §5.2 before execution, and the never-feeds-back rule of `docs/simemp.md` §4.4 extends to overlap telemetry unchanged, preserving `H(S_t | D(S_t)) = 0` and receipt replay.

**Consequentiality coupling [v1.2.0 — CANDIDATE].** The overlap credit is the physical reading of the consequentiality constraint: an admitted lookup carries its declared `κ_t` margin as headroom, and the harvester converts that headroom into useful register-pinned work — the memory wall is harvested, not suffered. A lookup that fails the latency margin cannot be repaired by harvesting: admission is prior, declared, and final for the invocation.

---

## 7. The Dual Causal Axes: Forward Teleology vs. Inverse Empirical Science

$\mathrm{Qlog}$ accommodates both directional flows across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qlog
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD SYNTHESIS : Teleological Direction (Intent ──► Target State)        │
 │ • Domain : Software engineering, compilation, robotic actuation.            │
 │ • Pipeline: Qs Intent Contract ──► Q(workflows) ──► Executed Target State.  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Empirical Science Direction (Observed State ──► )  │
 │ • Domain : Cosmology, pathology, materials forensics, fundamental physics.  │
 │ • Pipeline: Observed State S_obs ──► Qlog Abduction ──► Latent Causal Origin│
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 7.1. Forward Teleological Synthesis (Engineering and Intent Compilation)
In forward synthesis, a system transitions from human intent to physical execution:
1. Human authority establishes an {% include term.html id="intent-contract" %} in {% include term.html id="qs" %}.
2. $\mathrm{Qlog}$ compiles and validates a composite `Q(workflow)` assembling known `Q(patterns)`.
3. Execution lowers deterministically to target machine operations, verifying that each milestone stays within allocated metric budgets.

### 7.2. Inverse Causal Reconstruction (Empirical Science and Abduction)
In empirical science, observers measure an empirical terminal physical state $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`) while the generative causal origin and intermediate trajectory remain unobserved:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle S_0, \, \mathcal{W} \big\rangle \quad \text{such that} \quad \mathcal{W}(S_0) \equiv S_{\mathrm{obs}}$$

1. **Empirical Receipt Ingestion:** Observed physical telemetry is anchored via verified sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic modules propose ancestral trajectories ($H_1 \dots H_n$), tagged as `State: CANDIDATE` under **Axiom 14**.
3. **BOC Abductive Resolution:** If multiple candidate trajectories explain the observed state, the **BOC Policy** selects the optimal route minimizing unmeasured conditional entropy ($H \to 0$) and Landauer dissipation:

$$\mathcal{W}^* = \arg\max_{H \in \mathcal{H}} \Big( \text{SIMEMP Compliance}(H) \Big)$$

4. **Receipt-Sealed Invariants:** The verified trajectory is sealed as an emergent `Q(pattern)`. It attains status as an established scientific law (`MODAL_CONSTRUCTIVE` or a new $Q(\text{anchor})$) only when reproducible execution across external nodes validates the trajectory.

---

## 8. Dynamic Lifecycle of Q(anchor) in Logic

$Q(\text{anchor})$ entities are topological landmarks across the causal network:

$$\chi(\alpha) = \frac{\lvert \mathcal{C}^+(\alpha) \rvert}{\lvert \mathcal{V} \rvert} \ge \Theta_{\chi}$$

### 8.1. Anchor Crystallization and Emergent Stability
When an emergent `Q(pattern)` or sub-workflow demonstrates universal stability and its centrality crosses threshold $\Theta_{\chi}$, it crystallizes into a new {% include term.html id="q-anchor" %}. It is assigned a versioned capability contract (`DCC_ANCHOR_*_v1`), serving as a milestone for long-hop Markov routing.

### 8.2. Anchor Fracture, Containment, and Revocation
In physical networks, hardware degrades, nodes disconnect, or cryptographic algorithms become vulnerable:
1. **Centrality Drop:** If dependencies fail to persist across the network, centrality collapses ($\chi(\alpha) \lt \Theta_{\chi}$).
2. **Structural Indirection Migration:** If an algorithm is compromised, the gateway swaps the implementation module under **Axiom 13**, emitting an explicit migration receipt while preserving abstract contracts.
3. **Fracture Containment:** If an anchor cannot be repaired, $\mathrm{Qlog}$ emits `RECEIPT_ANCHOR_REVOKED`. Dependent workflows prune the compromised branch and fall back to deeper, invariant root anchors ($Q(2)\dots Q(9)$) without causing catastrophic network failure.

---

## 9. License and Lineage Alignment

All derivative implementations, logic solvers, type checkers, and automated reasoning engines derived from $\mathrm{Qlog}$ must embed the canonical attribution:



$\mathrm{Qlog}$ operates as passive, neutral mathematical and logical infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external datasets, or sovereign {% include term.html id="payload" text="Payloads" %}.
[[/CODE]]

### II.6 Qphy — Quang Physics v1.1.0 — `qphy.md`

[[CODE]]
# Qphy — Quang Physics: Discrete Field Dynamics, Computational Thermodynamics, and Physical Entropy Accounting


---

## 1. Epistemic Stance and Working Hypothesis Notice

### 1.1. Constructive Working Hypothesis Baseline
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It establishes **$\mathrm{Qphy}$ (Quang Physics)** as the third foundational Base Domain of the Qn universe. 

All formulations herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until empirical telemetry is derived across physical silicon runtimes. The architecture preserves structural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. The Physical Root of Finiteness (Layer 0 Grounding)
Classical physics often treats computation as an immaterial process operating over continuous coordinate fields ($\mathbb{R}^n$). In the {% include term.html id="qn" %} computational universe, **computation is an irreversible non-equilibrium thermodynamic process**.

$\mathrm{Qphy}$ is anchored directly to {% include term.html id="layer-0" text="Layer 0" %} (Physical Substrate: commodity silicon, parasitic wire capacitance, ring oscillator noise, and Landauer dissipation limits). A physical simulation or computation is valid if and only if its state transitions satisfy {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL CONTINUOUS PHYSICS         │ Qphy CONSTRUCTIVE COMPUTATIONAL PHYS │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous spacetime manifolds R^4 │ • Discrete simplicial complexes K    │
 │   and differential field equations.  │   and multi-axial Clifford frames.   │
 │ • Permits unphysical singularities   │ • Singularity-free by construction:  │
 │   (Infinite fluid velocity / poles). │   Spatial scale quantum δ_z(d).      │
 │ • Zero-energy computation ghost      │ • Thermodynamic Double-Leash:        │
 │   (Ignores Landauer dissipation).    │   Meters simulated and real work U.  │
 │ • Unmetered real numbers and dense   │ • Discrete cochains, exact Stokes,   │
 │   floating-point tensors.            │   and float-free Cayley rotors.      │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Abstraction Layer Placement and Inter-Domain Coupling

### 2.1. The Base Triad Architecture (Qs ⊗ Qm ⊗ Qphy)
Conforming to `docs/qs.md` (§7), physical reality is not generated from pure mathematical logic. The computational universe is grounded upon three mutually orthogonal Base Domains:

$$\mathbf{C}_{\text{total}} = \mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$$

- **$\mathrm{Qm}$ (Mathematics):** Supplies discrete scalar arithmetic, orthogonal Clifford frames ($\mathcal{C}\ell_{p,q}$), and discrete calculus coboundaries ($\mathbf{d}$).
- **$\mathrm{Qs}$ (Semantics):** Supplies observer framing, semantic blades, and human {% include term.html id="intent-contract" text="Intent Contracts" %}.
- **$\mathrm{Qphy}$ (Physics):** Supplies physical non-equilibrium thermodynamics, energy dissipation bounds, and physical conservation laws.

### 2.2. The Grounding Coupling with Qlog
The physical domain cannot be synthesized by squaring mathematical logic:

$$\mathbf{C}_{\text{Qphy}} \neq (\mathbf{C}_{\text{Qm}} \otimes \mathbf{C}_{\text{Qs}})^2$$

However, $\mathrm{Qphy}$ requires {% include term.html id="qlog" %} to type-check and assemble its physical trajectories:

<pre>
                         THE BASE DOMAIN COUPLING
                     Qm (Geometry)          Qs (Observer Intent)
                           │                      │
                           └──────────┬───────────┘
                                      │ Categorical Tensor Product
                                      ▼
                            Qlog (Logics & Verification)
                                      │
                                      ▼ Types & Guides
                            Qphy (Physical Reality)
                            • Anchored to Layer 0 Silicon
                            • Landauer Dissipation Quantum
                            • Discrete Simplicial Cochains
</pre>

- **Physical Laws as {% include term.html id="q-patterns" text="Q(patterns)" %}:** Every physical transition template (e.g., a discrete symplectic integration step) is an invariant `Q(pattern)`.
- **Physical Trajectories as {% include term.html id="q-workflows" text="Q(workflows)" %}:** Time evolution across intervals is an assembled `Q(workflow)` bounded by finite metric budgets.
- **Conservation Laws as Type Judgments:** Checking whether a physical state satisfies energy or charge conservation is a constructive type judgment: $\Gamma \vdash S : \mathrm{Conserved}(E_0)$.

---

## 3. Constructive Field Dynamics on Discrete Simplicial Lattices

### 3.1. Discrete Differential Forms as Cochain Fields
Classical continuous tensor fields with coordinate indices ($T^{\mu\nu}$) are replaced in $\mathrm{Qphy}$ by **cochains** evaluated over oriented simplicial complexes $K$ (`docs/qm_calculus.md`):
- **$0$-Cochains (Scalars):** Electrostatic potential, temperature, and mass densities evaluated on mesh vertices.
- **$1$-Cochains (Circulations):** Fluid velocity vectors and electric field circulations integrated along oriented edges.
- **$2$-Cochains (Fluxes):** Magnetic induction flux and fluid vorticity evaluated across oriented planar faces.

### 3.2. Singularity Immunity via Scale Lattices (δ_z)
In classical continuous mechanics, non-linear partial differential equations permit unphysical finite-time singularities (e.g., infinite velocity blowups in Navier–Stokes equations). 

$\mathrm{Qphy}$ provides structural immunity against physical singularities:
1. **Discrete Lattice Cutoff:** Physical gradients are strictly bounded by the discrete scale quantum $\delta_z(d)$ along dimensional axes (`docs/qm.md` §3.3). Spatial points cannot approach distance zero.
2. **Dissipative Viscous Sinks:** Physical fields dissipate excess energy into microscopic thermal noise, exporting operational entropy via signed receipts.
3. **Totality by Budget ({% include term.html id="axiom-6" text="Axiom 6" %}):** Simulation workflows carry an explicit work ceiling in units of $U$. Unbounded velocity divergence is physically impossible; execution halts cleanly upon budget exhaustion.

### 3.3. Exact Topological Conservation (d² ≡ Q(0))
In $\mathrm{Qphy}$, conservation laws do not rely on asymptotic floating-point limits. They are exact algebraic identities derived from the **Discrete Exterior Derivative** ({% include term.html id="discrete-exterior-derivative" text="d" %}):

$$\mathbf{d} \circ \mathbf{d} \equiv Q(0)$$

By the Poincaré lemma on discrete simplicial meshes, the boundary of a boundary is empty ($\partial \partial \equiv \emptyset$). Consequently:
- **Conservation of Magnetic Flux (Maxwell):** $\mathbf{d} F \equiv Q(0)$ holds identically with zero numerical leakage.
- **Conservation of Electric Charge:** $\mathbf{d} \star J \equiv Q(0)$ is an exact topological identity.
- **Discrete Stokes' Invariant:** Boundary flux integrals equal interior coboundary evaluations with zero remainder error.

---

## 4. Non-Equilibrium Computational Thermodynamics

### 4.1. The Open System Entropy Balance (Prigogine Formulation)
A physical computing system is an **open thermodynamic engine**. In accordance with non-equilibrium thermodynamics, internal computational stability requires expending work to export operational entropy (`docs/simemp.md` §4.2):

$$dS = dS_{\text{int}} + dS_{\text{ext}}, \quad \text{where } dS_{\text{ext}} \lt 0$$

Thermal noise, bus line jitter, and transmission faults maximize internal entropy ($dS_{\text{int}} \gt 0$). The system restores internal order by exporting entropy through immutable, signed {% include term.html id="receipt" text="Road Receipts" %} ($\mathcal{R}_{\mathrm{road}}$).

### 4.2. Physical Calibration of the Abstract Compute Unit U
The transition from local origin {% include term.html id="q0" text="Q(0)" %} to unit {% include term.html id="q1" text="Q(1)" %} defines the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}:
- **The Landauer Bound:** At ambient temperature ($T = 300\text{ K}$), erasing one bit of physical information dissipates minimal thermodynamic work into the environment:

$$W(U) \ge k_B T \ln 2 \approx 2.87 \times 10^{-21} \text{ J}$$

- **Platform Invariance:** In software compilation, $U$ serves as the invariant accounting quantum. Physical hardware translation layers parameterize device-specific Joule-per-$U$ conversion factors without altering high-level metric envelopes.

### 4.3. The Capacitive Memory Wall Transport Impedance
Moving a packet of bits across a distance $\ell$ on physical silicon requires charging the parasitic capacitance $C$ of the metal interconnect at operating voltage $V$ and clock frequency $f$:

$$P_{\mathrm{bus}} = \alpha C(\ell) V^2 f + I_{\mathrm{leak}} V$$

1. **Distance-Dependent Transport Cost:** Transporting bits across the Memory Wall incurs a strictly greater metric cost than local register operations ($W_{\mathrm{bus}} \gg W_{\mathrm{register}}$).
2. **Two-Axis Breakeven Grounding [v1.1.0 — CANDIDATE]:**  
   This physical impedance grounds the **work axis** of the Two-Axis Consequentiality Breakeven governing {% include term.html id="q-reuse" %}: the capacitive transport cost $P_{\mathrm{bus}}$ is the physical source of the declared per-tier lookup work bounds `τ_U[tier(h)]` and of the bus-dissipation term of the declared direct-path cost model, while the finite signal propagation horizon of the same interconnect — the physical Q(c) anchor of §5.1, built from local dielectric propagation delays — is the physical source of the **latency axis**: the declared per-tier worst-case lookup latency bounds `τ_t = { τ_reg, τ_L1, τ_L2, τ_L3, τ_DRAM, τ_storage, τ_remote }` (cycles against the node's declared reference clock) of the Latency Telemetry Envelope ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4). Admission remains governed by the canonical predicate of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15/§5.9, mirrored at `docs/qlog.md` §5.2:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

   **Term provenance [v1.1.0 — CANDIDATE].** `h` is the referenced commitment and `tier(h)` its declared memory tier; `ΔW_compute` and `Δt_compute_bound` are the DCC-declared worst-case work and latency bounds of the direct compute path under the profile's declared cost model; `τ_U[tier(h)]` and `τ_t[tier(h)]` are the declared per-tier worst-case lookup bounds of the two-axis envelope, calibrated at Layer-0 onboarding through the Historical Calibration Benchmark receipt class of `docs/simemp.md` §4.4; `ε_U` and `ε_t` are the declared gateway overhead slacks; `κ_dcc` and `κ_t` are the declared advantage margins within the governance-bounded range `[1, 64]` (recommended preset `κ_dcc = 2`, `κ_t = 4`, inherited unchanged from `docs/qnPrimitive.md` §5.9). All quantities are declared, never runtime-sampled; an undeclared quantity forces the safe Direct-Compute fallback, and ties resolve to Direct Compute. Every admitted reuse executes under the Admission Ledger (`W_gross_reserve`, `W_actual_charge`, `W_refund`, `W_net`) of `docs/qnPrimitive.md` §5.9. An energetically cheap but latency-slower lookup converts the Memory Wall from a harvestable resource into a pure stall: the transport impedance formalized in this section is harvested rather than suffered only under the two-axis verdict.

---

## 5. The Canonical Physical Q(anchor) Spectrum

In $\mathrm{Qphy}$, physical constants are not hardcoded static variables. They are **constructively built Qn artifacts derived from an explicit {% include term.html id="historical-calibration-benchmark" %}**, whose anchor status emerges as their topological centrality ($\chi$) expands across the causal DAG:

<pre>
                  THE 5 PHYSICAL Q(anchor) LANDMARKS IN Qphy
 ┌──────────┬─────────────────────────────┬────────────────────────────────────┐
 │ ARTIFACT │ ANCHOR CONTRACT DESIGNATION │ CONSTRUCTIVE DERIVATION SOURCE     │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(th_2)  │ DCC_ANCHOR_SECOND_LAW_v1    │ Calibrated via hardware sensor temp│
 │          │ (Entropy & Landauer Bound)  │ T, Q(1), and DCC_ANCHOR_LN2.       │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(c)     │ DCC_ANCHOR_CAUSAL_HORIZON_v1│ Calibrated via interconnect dielectric│
 │          │ (Finite Speed of Light c)   │ velocity in Minkowski frame Cl(1,3)│
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(ℏ)     │ DCC_ANCHOR_ACTION_PLANCK_v1 │ Calibrated via Northcott height    │
 │          │ (Bekenstein-Margolus Limit) │ ceilings and discrete state quantum│
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(Z_bus) │ DCC_ANCHOR_MEMORY_WALL_v1   │ Calibrated via bus line capacitance│
 │          │ (Ohmic Line Dissipation)    │ impedance and static leakage leak. │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(gauge) │ DCC_ANCHOR_DEC_D_v1         │ Verified via discrete coboundary   │
 │          │ (Poincaré Coboundary d²≡0)  │ topological nilpotency on mesh K.  │
 └──────────┴─────────────────────────────┴────────────────────────────────────┘
</pre>

**Canonical anchor naming [v1.1.0 — CANDIDATE].** The anchor contract designations of the table above — `DCC_ANCHOR_SECOND_LAW_v1`, `DCC_ANCHOR_CAUSAL_HORIZON_v1`, `DCC_ANCHOR_ACTION_PLANCK_v1`, `DCC_ANCHOR_MEMORY_WALL_v1`, and `DCC_ANCHOR_DEC_D_v1` — are the canonical Layer-0 calibration-benchmark anchor names of this specification: they are the designations under which the physical Q(anchor) spectrum of §5 is calibrated and referenced by Layer-0 telemetry anchoring (the role `docs/simemp.md` records for this file). The prototype spellings `Q(Zbus)` and `Q(th2)` used by the condensed specification ([`docs/AiReviews/Qn-Universe_to_QnIR-v0.1_Condensed-Specification_DRAFT-v0.1.pdf`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/Qn-Universe_to_QnIR-v0.1_Condensed-Specification_DRAFT-v0.1.pdf)) are not canon artifacts of their own: `Q(Zbus)` denotes `Q(Z_bus)` (`DCC_ANCHOR_MEMORY_WALL_v1`), and `Q(th2)` denotes `Q(th_2)` (`DCC_ANCHOR_SECOND_LAW_v1`).

### 5.1. Universal Physical Anchors and Capability Contracts
- **{% include term.html id="q-th-2" text="Q(th_2)" %} (The Second Law Anchor):** Built from sampled thermal receipts and $\ln 2$ continued fraction generators. Enforces non-zero heat dissipation for bit-erasure.
- **{% include term.html id="q-c" text="Q(c)" %} (The Relativistic Causal Horizon Anchor):** Built from local dielectric propagation delays. Enforces finite signal speed ($\tau \ge \Delta x / v_{\mathrm{signal}}$).
- **{% include term.html id="q-hbar" text="Q(ℏ)" %} (The Quantum Action Anchor):** Built from height ceilings ($H_{\max}$) and quantum action limits. Forbids unmetered infinite information density in finite registers.
- **{% include term.html id="q-z-bus" text="Q(Z_bus)" %} (The Memory Wall Transport Anchor):** Built from parasitic wire capacitance measurements ($P = \alpha C V^2 f + I_{\mathrm{leak}} V$).
- **{% include term.html id="q-gauge" text="Q(gauge)" %} (The Topological Conservation Anchor):** Built from simplicial mesh topology. Enforces exact coboundary nilpotency ($\mathbf{d}^2 \equiv Q(0)$).

### 5.2. The Physical Inconsistency Collapse Cascade and Circuit Breakers
If an execution profile or autonomous workflow claims a trajectory that violates an emergent physical anchor:
1. **Immediate Inconsistency Cascade:** The physical anchor destabilizes, threatening to corrupt downstream causal cones $\mathcal{C}^+(\alpha)$.
2. **Gateway Circuit Breaker:** The {% include term.html id="simemp-gateway" %} intercepts the invalid transition, halting execution and emitting `RECEIPT_PHYSICAL_VIOLATION`.
3. **Protection of Silicon and Lineage:** Halting at the gateway prevents physical thermal runaway on hardware chips and protects downstream causal lineage from ungrounded physics.

---

## 6. The Axiomatic Computational World Model

$\mathrm{Qphy}$ operates as an **Axiomatic, Thermodynamically Grounded Computational World Model** for autonomous systems and robotics.

<pre>
            STATISTICAL AI WORLD MODEL vs. Qphy AXIOMATIC MODEL
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ STATISTICAL AI WORLD MODELS (IRL)    │ Qphy AXIOMATIC WORLD MODEL (Qn)      │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous latent tensors (R^d).   │ • Discrete multivector blades (Qm).  │
 │ • Statistical physics mimicry        │ • Invariant conservation laws        │
 │   (Learned from video/sensor logs).  │   (d² ≡ Q(0), Noether symmetries).   │
 │ • Compounding drift: Future rollouts │ • Terminal Exactness (r = Q(0)):     │
 │   decay into hallucinations/blur.    │   Zero metric drift across rollouts. │
 │ • Unmetered execution: Ignores the   │ • The Thermodynamic Double-Leash:    │
 │   energy cost of computing the plan. │   Meters both simulated & real work. │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

### 6.1. Axiomatic Invariance vs. Statistical Latent Hallucination
Contemporary deep learning world models approximate physics through statistical correlation over continuous latent spaces. They suffer from physical hallucinations: objects morphing, phantom forces, and non-conservation of energy.  
In $\mathrm{Qphy}$, physical laws are discrete algebraic invariants. Hallucinations are prevented by construction: an unphysical state cannot compile through the gateway.

### 6.2. The Thermodynamic Double-Leash on Autonomous Agents
Autonomous agents executing in the physical world are governed by two coupled energy boundaries:
1. **The External Leash (Simulated Mechanics):** The planned trajectory must satisfy environmental energy conservation ($\Delta E_{\mathrm{kinetic}} + \Delta E_{\mathrm{potential}} \le E_{\mathrm{battery}}$).
2. **The Internal Leash (Computational Work):** The computation required to verify the trajectory must itself fit within the allocated metric budget ($W \le W_{\mathrm{budget}}$ in units of $U$).

If the computational cost of planning a path exceeds available energy budgets, execution halts, forcing lower-complexity fallback actions.

### 6.3. Symplectic Preservation and Drift-Free Trajectories
Physical dynamical trajectories are integrated using discrete symplectic morphisms:
- Phase space volume (Liouville's theorem) is conserved exactly.
- Each time step achieves {% include term.html id="terminal-exactness" %} ($r = Q(0)$) or preserves exact Diophantine residuals.
- Trajectories can execute over billions of discrete steps with zero artificial numerical dissipation.

---

## 7. Functorial Tensor Symmetries and Bounded Context Ranges

### 7.1. Permutation-Invariant vs. Permutation-Sensitive Physical Tensors
Under the {% include term.html id="no-implicit-rule" %}, every tensor interaction in $\mathrm{Qphy}$ must declare its algebraic symmetry within its DCC Profile:
- **Permutation-Invariant (`DCC_SYM_COMMUTATIVE`):** Operations where component order carries zero physical difference (e.g., thermal energy accumulation, scalar potential sums). Child hashes are sorted lexicographically, enabling $\mathcal{O}(1)$ deduplication via {% include term.html id="q-reuse" %}.
- **Permutation-Sensitive (`DCC_SYM_NON_COMMUTATIVE`):** Operations where order dictates causality or direction (e.g., sequential time steps $S_t \otimes S_{t+\Delta t}$, non-commutative rotor sandwiches $R \vec{v} R^\dagger$). Order is strictly preserved.
- **Graded Anti-Commutative (`DCC_SYM_ANTI_COMMUTATIVE`):** Outer products of differential forms ($\vec{u} \wedge \vec{v} = - \vec{u} \wedge \vec{v}$). Swapping elements flips the sign parity ($\sigma \to \mathcal{I}(\sigma)$).

### 7.2. The Four-Axis Bounded Context Range (R_ctx)
Physical state evaluations cannot occur across unbounded continuous space. Every physical evaluation requires an explicit Context Range:
1. **Scale Range:** Discrete zoom window $[z_{\min}, z_{\max}]$ parameterizing lattice resolution $\delta_z(d)$.
2. **Causal Horizon Range:** Monotonic causal index interval $[\kappa_{\mathrm{start}}, \kappa_{\mathrm{end}}]$.
3. **Dimensional Metric Frame:** Orthogonal spacetime frame $\mathcal{D}_{\mathrm{frame}} = \{d_t, d_x, d_y, d_z\}$.
4. **Thermodynamic Budget:** Maximum permissible Landauer work ceiling $W \le W_{\mathrm{budget}}$.

---

## 8. Causal Trajectories: Forward Simulation vs. Inverse Empirical Science

$\mathrm{Qphy}$ accommodates both directions across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qphy
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD SIMULATION : Teleological Mechanics (Initial State ──► Future State)│
 │ • Domain : Robotics actuation, orbital kinematics, circuit simulation.      │
 │ • Pipeline: State S_t ──► Q(workflows) ──► Verified Target State S_(t+Δt).  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Empirical Physics (Observed State ──► Origin)     │
 │ • Domain : Astrophysics, spectroscopy, material failure, quantum sensing.   │
 │ • Pipeline: Observed State S_obs ──► Qlog Abduction ──► Latent Hamiltonian  │
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 8.1. Forward Symplectic Time Evolution (Q(workflows))
In forward simulation, a system predicts future physical states:
1. Initial physical state $S_0$ is configured inside a `Qexpr` container.
2. The system executes a sequence of symplectic {% include term.html id="q-patterns" text="Q(patterns)" %} generating discrete {% include term.html id="q-event" text="Q(events)" %}.
3. Intermediate checkpoints are verified using the {% include term.html id="anchor-cut" text="Anchor-Cut Invariant" %}, allowing external auditors to verify long simulations in $\mathcal{O}(K)$ receipt validations without replaying microscopic steps.

### 8.2. Inverse Causal Reconstruction of Physical Laws
In empirical observational physics across the {% include term.html id="historical-horizon" %}, observers measure an empirical terminal physical state $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`) while the causal origin remains latent:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle S_0, \, \mathcal{H}_{\mathrm{sys}} \big\rangle \quad \text{such that} \quad \mathcal{H}(S_0) \equiv S_{\mathrm{obs}}$$

1. **Receipt Anchoring:** The observed state is ingested via verified hardware sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic or neural modules propose ancestral Hamiltonians ($H_1 \dots H_n$).
3. **BOC Abductive Resolution:** The {% include term.html id="boc-policy" text="BOC Policy" %} selects the candidate route that maximizes SIMEMP compliance, selecting the physical law that minimizes unmeasured entropy ($H(S \mid \mathcal{D}) \to 0$) and minimizes Landauer dissipation.
4. **Law Sealing:** The verified physical trajectory is sealed as an emergent `Q(pattern)`, joining the empirical physical registry.

---

## 9. License and Lineage Alignment

All derivative physical simulation models, thermodynamic solvers, robotics world models, and intermediate execution modules derived from $\mathrm{Qphy}$ must embed the canonical attribution:



$\mathrm{Qphy}$ operates as passive, neutral physical and computational modeling infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external simulation datasets, or sovereign {% include term.html id="payload" text="Payloads" %}.
[[/CODE]]

### II.7 Qchem — Quang Chemistry v1.0.3 — `qchem.md`

[[CODE]]
# Qchem — Quang Chemistry: Discrete Stoichiometry, Molecular Graphs, and Reaction Energetics


---

## 1. Epistemic Stance and Working Hypothesis Notice

### 1.1. Constructive Working Hypothesis Baseline
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It establishes **$\mathrm{Qchem}$ (Quang Chemistry)** as an emergent composite domain formed by the categorical tensor product of physical thermodynamics and constructive mathematics:

$$\mathbf{C}(\text{Qchem}) \equiv \mathbf{C}(\text{Qphy}) \otimes \mathbf{C}(\text{Qm})$$

All formulations herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until physical execution feedback is derived across chemical simulation runtimes. The architecture preserves structural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. Abiotic Boundary and the Exclusion of Semantic Intent
Chemistry models the organization of matter governed by valence electrons, molecular bonds, and thermodynamic equilibria. Unlike biological or cognitive domains, **abiotic chemical matter possesses zero semantic agency, legal liability, or teleological intent**:

$$\mathbf{C}(\text{Qchem}) \cap \mathbf{C}(\text{Qs}) \equiv \emptyset$$

Chemical reaction steps transition spontaneously under physical thermodynamic potential ($\Delta G \le 0$) and Diophantine stoichiometric balance. Attributing intent ($\mathrm{Qs}$) to abiotic reactions is a category error. All transitions satisfy {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL COMPUTATIONAL CHEMISTRY    │ Qchem CONSTRUCTIVE CHEMICAL PHYSICS  │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous floating-point energy   │ • Exact Diophantine stoichiometry    │
 │   surfaces with accumulated drift.   │   matrix balance: S · v ≡ Q(0).      │
 │ • Lossy atomic coordinate models     │ • Content-Addressed Molecular DAGs   │
 │   vulnerable to geometric roundoff.  │   with exact Clifford stereochemistry│
 │ • Unmetered simulation loops         │ • Reaction steps as Q(patterns);     │
 │   ignoring Landauer dissipation.     │   catalysis as Category B Q(bypass). │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Discrete Molecular Graphs and Stereochemistry

### 2.1. Atoms and Valence Bonds as Content-Addressed DAGs
Molecules are represented within {% include term.html id="qexpr" %} Content-Addressed Expression DAGs ({% include term.html id="cae-dag" %}):
- **Atomic Vertices:** Each atom is an immutable leaf artifact carrying atomic number $Z \in \mathbb{N}$, discrete mass number $A \in \mathbb{N}$, and formal oxidation state.
- **Valence Bonds as Clifford Blades:** Covalent, ionic, and coordination bonds are modeled as localized Clifford multivector blades (`docs/qm_geometry.md`), declaring discrete bond order ($1, 2, 3$) and directional orientation.
- **Deduplication via Hash-Consing:** Identical molecular sub-structures share identical commitment hashes $\mathcal{H}(\text{Molecule})$. Common functional groups are allocated once in memory, maximizing {% include term.html id="q-reuse" %} at zero marginal Landauer cost ($\Delta W = 0$).

### 2.2. Stereochemical Chirality via Clifford Sign Parity (σ)
Enantiomers (chiral mirror-image molecules such as D- and L-amino acids) share identical scalar connectivity, bond lengths, and atom counts. Classical cheminformatics often relies on ambiguous textual conventions (R/S or D/L labels).

In $\mathrm{Qchem}$, molecular chirality is an exact algebraic invariant governed by the spatial reflection involution $\mathcal{I}_d$ and the 3D pseudoscalar $I_3 = \vec{e}_1 \wedge \vec{e}_2 \wedge \vec{e}_3$:
- The spatial inversion of a chiral center inverts its oriented grade-3 volume blade.
- The resulting enantiomer is distinguished by its **Sign Parity** ($\sigma \in \{\ominus, \oplus\}$).
- Enantiomeric transformations preserve exact unitary norms without floating-point trigonometric drift.

---

## 3. Exact Diophantine Stoichiometry

### 3.1. The Stoichiometric Null-Space Invariant
In classical software, reaction mass balances frequently accumulate numerical roundoff errors. In $\mathrm{Qchem}$, chemical reactions are **exact Diophantine balance equations**:
- Let $M$ elements react across $N$ molecular species. The stoichiometric matrix $\mathbf{S} \in \mathbb{Z}^{M \times N}$ maps integer elemental coefficients.
- A valid chemical reaction flux vector $\vec{v} \in \mathbb{N}^N$ satisfies the Diophantine null-space constraint under {% include term.html id="terminal-exactness" %} ($r = Q(0)$):

$$\mathbf{S} \cdot \vec{v} \equiv Q(0)$$

Elemental mass, atom counts, and net electric charges balance identically. Truncation error is zero by construction.

### 3.2. Redox Electron Quanta and Charge Conservation
Oxidation-reduction reactions involve discrete electron transfers:
- Electrons exist as indivisible integer charge carriers ($z \in \mathbb{Z}$). Fractional electron transfers are barred.
- The net charge transition across reactants and products evaluates to genesis origin $Q(0)$, guaranteeing exact conservation of electric charge under `DCC_ANCHOR_DEC_D_v1`.

---

## 4. Non-Equilibrium Chemical Thermodynamics

### 4.1. Gibbs Free Energy Reaction Spontaneity
Chemical reaction feasibility is governed by **Non-Equilibrium Thermodynamics** (`docs/qphy.md` §4):
- **Spontaneity Criterion:** At constant temperature and pressure, an abiotic chemical reaction transitions spontaneously if and only if its change in Gibbs free energy is negative or zero:

$$\Delta G = \Delta H - T \Delta S \le Q(0)$$

- **Thermal Activation Barriers ($E_a$):** Reactions possessing an activation energy barrier require absorbing a discrete thermal work packet from the local environment, metered by the local {% include term.html id="metric-envelope" %}.

### 4.2. Chemical Equilibrium as Continued Fraction Convergents
Chemical equilibrium constants ($K_{\mathrm{eq}}$) are held symbolically inside `Qexpr` as deterministic continued fraction partial convergents (`OP_CF`):

$$K_{\mathrm{eq}} = [a_0; \, a_1, \, a_2, \, \dots]$$

Evaluating reaction quotients to zoom level $z$ generates exact rational pairs without continuous exponential floating-point approximations.

---

## 5. The Chemical Q(anchor) Spectrum

$\mathrm{Qchem}$ grounds its physical and chemical constants in universal {% include term.html id="q-anchor" text="Q(anchor)" %} entities derived from the {% include term.html id="historical-calibration-benchmark" %}:

<pre>
                  THE CANONICAL CHEMICAL Q(anchor) SPECTRUM
 ┌──────────┬─────────────────────────────┬────────────────────────────────────┐
 │ ARTIFACT │ ANCHOR CONTRACT DESIGNATION │ INVARIANT CHEMICAL REALITY         │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(N_A)   │ DCC_ANCHOR_AVOGADRO_v1      │ Exact Integer Mole Entity Count:   │
 │          │ (Avogadro Constant)         │ N_A ≡ 602,214,076 × 10¹⁵ mol⁻¹     │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(stoich)│ DCC_ANCHOR_STOICHIOMETRY_v1 │ Lavoisier-Dalton Mass Invariant:   │
 │          │ (Atom Count Conservation)   │ Diophantine null-space S·v ≡ Q(0). │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(th_2)  │ DCC_ANCHOR_SECOND_LAW_v1    │ Gibbs Dissipation Spontaneity:     │
 │          │ (Thermodynamic Invariant)   │ ΔG ≤ Q(0) (Inherited from Qphy).   │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(e)     │ DCC_ANCHOR_E_CHARGE_v1      │ Discrete Faraday Redox Quanta:     │
 │          │ (Elementary Charge Unit)    │ Integer electron exchange (z ∈ ℤ). │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(chiral)│ DCC_ANCHOR_CHIRALITY_v1     │ Pasteur Stereochemical Symmetry:   │
 │          │ (Enantiomeric Parity)       │ Clifford reflection involution I_d.│
 └──────────┴─────────────────────────────┴────────────────────────────────────┘
</pre>

### 5.1. Universal Chemical Anchors and Capability Contracts
- **`Q(N_A)` (The Avogadro Anchor):** Formalizes the exact discrete bridge between microscopic atomic integer counts and macroscopic molar thermodynamics: $N_A \equiv 602{,}214{,}076 \times 10^{15}\text{ entities/mol}$.
- **`Q(stoich)` (The Stoichiometric Anchor):** Enforces nuclear and elemental conservation across chemical transformations without fractional atom leakage.
- **{% include term.html id="q-th-2" text="Q(th_2)" %} (The Second Law Gibbs Anchor):** Mandates that endergonic reactions ($\Delta G \gt 0$) cannot occur without coupled exergonic work.
- **`Q(e)` (The Redox Charge Anchor):** Restricts electrochemical electron transfers to discrete integer quanta of $Q(1)$.
- **`Q(chiral)` (The Stereochemical Anchor):** Enforces spatial reflection parity over chiral tetrahedral carbon centers.

### 5.2. Chemical Inconsistency Cascade and Gateway Interception
If an automated reaction proposal or molecular simulation claims an unphysical reaction:
1. **Immediate Inconsistency Cascade:** The stoichiometric or thermodynamic anchor destabilizes, threatening downstream mass-action models.
2. **Gateway Interception:** The {% include term.html id="simemp-gateway" %} intercepts the invalid balance, emitting `RECEIPT_STOICHIOMETRIC_VIOLATION` or `RECEIPT_THERMODYNAMIC_EXHAUSTION`.
3. **Execution Termination:** The invalid reaction workflow is rejected before consuming physical compute or dispatching automated laboratory synthesis actuators.

---

## 6. Reaction Kinetics as Q(patterns) and Q(workflows)

### 6.1. Elementary Reaction Steps as Q(patterns)
An elementary chemical reaction step is an invariant **{% include term.html id="q-patterns" text="Q(pattern)" %}**:

$$\mathcal{P}_{\mathrm{rxn}}: \mathcal{H}(\text{Reactants}) \xrightarrow{\quad \mathcal{M}_{\mathrm{kinetics}} \quad} \mathcal{H}(\text{Products})$$

It defines a reproducible, discrete transformation mapping reactant commitment hashes to product hashes under explicit thermodynamic constraints.

### 6.2. Reaction Cascades as Q(workflows) and Catalysis as Q(bypass)
Multi-step chemical syntheses, polymerizations, and reaction networks are assembled as **{% include term.html id="q-workflows" text="Q(workflows)" %}**:
- **Work Budgets:** Reaction sequences operate within a declared Metric Envelope, tracking thermal work and Landauer dissipation.
- **Catalytic Optimization via {% include term.html id="q-bypass" %}:** A catalyst lowers the activation energy barrier ($E_a' \ll E_a$) without altering overall equilibrium ($\Delta G$). In $\mathrm{Qchem}$, catalysis is formalized as an execution short-circuit: the gateway recognizes the catalytic agent, bypasses the slow uncatalyzed route, and emits `RECEIPT_CATALYTIC_BYPASS` in $\mathcal{O}(1)$ dispatch time.

---

## 7. The Boundary Between Qchem and Qbio

### 7.1. Stratification of Abiotic Matter vs. Living Systems
To preserve the **No-Implicit Rule**, the architecture maintains a strict epistemic boundary between chemistry and biology:
- **$\mathrm{Qchem}$ (Abiotic Chemistry):** Models molecular graphs, valence interactions, and reaction kinetics. It operates with zero intentional agency ($\mathrm{Qs} = \emptyset$).
- **{% include term.html id="qbio" text="Qbio" %} (Living Biosemiotics):** Emerges when chemical networks are coupled to **informational semantics ($\mathrm{Qs}$)**:

$$\mathbf{C}(\text{Qbio}) \equiv \mathbf{C}(\text{Qchem}) \otimes \mathbf{C}(\text{Qs})$$

### 7.2. The Biosemiotic Leap
Living systems differ from abiotic chemistry because their molecules serve as **informational codes**:
- An abiotic crystal grows purely via $\Delta G$ minimization ($\mathrm{Qchem}$).
- A strand of DNA guides protein synthesis via a codon translation code ($\mathrm{Qbio} = \mathrm{Qchem} \otimes \mathrm{Qs}$).
- Active inference and metabolic regulation introduce teleological intent contracts, which are strictly barred from abiotic $\mathrm{Qchem}$.

---

## 8. Causal Trajectories: Forward Synthesis vs. Inverse Spectroscopy

$\mathrm{Qchem}$ accommodates both directions across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qchem
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD SYNTHESIS : Retrosynthesis Planning (Reactants ──► Target Molecule) │
 │ • Domain : Chemical manufacturing, automated synthesis, materials design.   │
 │ • Pipeline: Precursors ──► Q(workflows) ──► Verified Product Molecule.      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Analytical Chemistry (Spectrogram ──► Molecule)   │
 │ • Domain : Mass spectrometry, NMR spectroscopy, environmental forensics.    │
 │ • Pipeline: Spectral Receipt S_obs ──► Qlog Abduction ──► Molecular Graph   │
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 8.1. Forward Synthesis Planning (Q(workflows))
Forward chemical synthesis compiles a path from available chemical precursors to a target molecule:
1. Target molecular graph $\mathcal{H}(\text{Target})$ is declared.
2. $\mathrm{Qlog}$ compiles a sequence of elementary reaction `Q(patterns)`.
3. Each step verifies $\Delta G \le Q(0)$ and stoichiometric balance, emitting a chained road receipt vector.

### 8.2. Inverse Causal Reconstruction of Molecular Graphs
In analytical chemistry across the {% include term.html id="historical-horizon" %}, an instrument measures an empirical spectral signature $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`, e.g., mass-to-charge ratios $m/z$, NMR chemical shifts) while the molecular identity remains unknown:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle \mathcal{H}(\text{Molecule}), \, \mathcal{M}_{\mathrm{frag}} \big\rangle \quad \text{such that} \quad \mathcal{M}(\text{Molecule}) \equiv S_{\mathrm{obs}}$$

1. **Receipt Ingestion:** Mass spectrometer readings are anchored as verified sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic or neural engines propose candidate molecular graphs ($M_1 \dots M_n$).
3. **BOC Abductive Resolution:** The {% include term.html id="boc-policy" text="BOC Policy" %} selects the candidate graph that matches the observed fragmentation peaks while minimizing unmeasured conditional entropy ($H(S \mid \mathcal{D}) \to 0$).
4. **Structure Sealing:** The reconstructed molecular graph is sealed into the local chemical registry.

---

## 9. License and Lineage Alignment

All derivative chemical simulation models, synthesis planners, spectroscopy engines, and intermediate execution modules derived from $\mathrm{Qchem}$ must embed the canonical attribution:



$\mathrm{Qchem}$ operates as passive, neutral chemical and computational modeling infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external chemical libraries, or sovereign {% include term.html id="payload" text="Payloads" %}.
[[/CODE]]

### II.8 Qbio — Quang Biology v1.0.2 — `qbio.md`

[[CODE]]
# Qbio — Quang Biology: Biosemiotics, Metabolic Homeostasis, and Active Inference


---

## 1. Epistemic Stance and Working Hypothesis Notice

### 1.1. Constructive Working Hypothesis Baseline
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It establishes **{% include term.html id="qbio" text="Qbio (Quang Biology)" %}** as an emergent biosemiotic domain derived from the categorical tensor product of chemistry and semantics:

$$\mathbf{C}(\text{Qbio}) \equiv \mathbf{C}(\text{Qchem}) \otimes \mathbf{C}(\text{Qs})$$

All formulations herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until biological simulation feedback is derived across living cellular runtimes. The architecture preserves structural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. The Biosemiotic Boundary: Life as Coded Chemistry
A living organism is not merely a complex chemical mixture; it is a **biosemiotic system** wherein molecular reaction networks are coupled to **informational code channels**:
- Abiotic molecules in {% include term.html id="qchem" %} react spontaneously under physical thermodynamic potential ($\Delta G \le 0$).
- Biological systems in $\mathrm{Qbio}$ process **informational intent**: nucleic acid sequences represent symbolic instructions that guide metabolic synthesis.

Living entities maintain non-equilibrium steady states against environmental entropy by consuming metabolic energy. Every biological state transition satisfies {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL CONTINUOUS BIOLOGY         │ Qbio CONSTRUCTIVE BIOSEMIOTICS       │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous floating-point ODEs and │ • Exact Diophantine metabolic graphs │
 │   stochastic gene network drifts.    │   and discrete codon translation.    │
 │ • Vitalist or heuristic agency      │ • Active inference under formal      │
 │   lacking thermodynamic metering.    │   Friston free energy bounds.        │
 │ • Unmetered growth loops ignoring    │ • Metabolic totality by budget:      │
 │   substrate Landauer limits.         │   ATP hydrolysis metered in U units. │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Categorical Formulation: Qchem Coupled to Qs

### 2.1. The Biosemiotic Tensor Product
$\mathrm{Qbio}$ is constructed constructively through the tensor product:

$$\mathbf{C}(\text{Qbio}) = \big( \mathbf{C}(\text{Qphy}) \otimes \mathbf{C}(\text{Qm}) \big) \otimes \mathbf{C}(\text{Qs})$$

- **The Chemical Substrate ($\mathrm{Qchem}$):** Provides discrete atomic molecular graphs, Diophantine stoichiometry ($\mathbf{S} \cdot \vec{v} \equiv Q(0)$), and Gibbs free energy dissipation ($\Delta G \le 0$).
- **The Semantic Channel ($\mathrm{Qs}$):** Provides symbolic intent contracts, functional pregroup grammars, and teleological agency (self-preservation and replication).

### 2.2. Genetic Code as Content-Addressed Translation
In $\mathrm{Qbio}$, DNA and RNA sequences are represented as linear **Content-Addressed Expression DAGs ({% include term.html id="cae-dag" %})**:
- **Codons as Discrete Tokens:** Triplet nucleotide bases $\{A, U, G, C\}^3$ form an exact discrete alphabet of 64 tokens.
- **The Translation Morphism:** Ribosomal translation is an invariant discrete mapping:

$$\mathcal{M}_{\mathrm{translation}}: \text{Codon}^{64} \longrightarrow \text{AminoAcid}^{20} \cup \{ \text{STOP} \}$$

- **Hash-Consing Deduplication:** Identical genetic motifs and conserved protein domains share the exact same cryptographic hash $\mathcal{H}(\text{Domain})$, enabling $\mathcal{O}(1)$ structural comparison and zero marginal Landauer allocation via {% include term.html id="q-reuse" %}.

---

## 3. Non-Equilibrium Metabolic Homeostasis and Free Energy

### 3.1. Variational Free Energy and Active Inference
A living organism persists by minimizing **Variational Free Energy** (Karl Friston formulation). Under `docs/simemp.md` (§4.2), an organism is an open dissipative system that avoids internal entropy maximization by generating predictive models of its environment:

$$F = \text{Internal Bound Energy} - T \cdot \text{Information Entropy}$$

1. **Active Inference Loops:** An organism generates actions to fulfill its metabolic {% include term.html id="intent-contract" %}, ensuring that environmental surprises (entropy spikes) remain bounded.
2. **Deterministic Selection via BOC:** Internal homeostatic adaptations are selected via the {% include term.html id="boc-policy" text="BOC Policy" %}, maximizing survival compliance while minimizing Landauer dissipation.

### 3.2. Entropy Export Across the Cellular Membrane
Living systems survive by acting as **Prigogine dissipative engines**:

$$dS = dS_{\text{int}} + dS_{\text{ext}}, \quad \text{where } dS_{\text{ext}} \lt 0$$

The cell consumes nutrient molecules possessing high chemical potential and excretes degraded waste products, exporting operational entropy via signed metabolic receipts ($\mathcal{R}_{\mathrm{metabolic}}$).

---

## 4. The Canonical Biological Q(anchor) Spectrum

$\mathrm{Qbio}$ grounds living systems in five universal {% include term.html id="q-anchor" text="Q(anchor)" %} entities derived from the {% include term.html id="historical-calibration-benchmark" %}:

<pre>
                  THE CANONICAL BIOLOGICAL Q(anchor) SPECTRUM
 ┌──────────┬─────────────────────────────┬────────────────────────────────────┐
 │ ARTIFACT │ ANCHOR CONTRACT DESIGNATION │ INVARIANT BIOLOGICAL REALITY       │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(codon) │ DCC_ANCHOR_GENETIC_CODE_v1  │ Discrete Universal Genetic Code:   │
 │          │ (Codon-to-Peptide Mapping)  │ Exact 64-triplet to 20-amino map.  │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(blanket│ DCC_ANCHOR_MARKOV_BLANKET_v1│ Topological Boundary Isolation:    │
 │          │ (Membrane Containment)      │ H(S_int | S_blanket, S_ext) = 0.   │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(atp)   │ DCC_ANCHOR_METABOLIC_ATP_v1 │ Cellular Energy Currency:          │
 │          │ (Hydrolysis Work Quantum)   │ ΔG_ATP ≈ 20 U per reaction event.  │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(homeo) │ DCC_ANCHOR_HOMEOSTASIS_v1   │ Active Inference & Free Energy:    │
 │          │ (Variational Free Energy)   │ Internal entropy bounded: S ≤ S_max│
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(autocat│ DCC_ANCHOR_AUTOCATALYTIC_v1 │ Autocatalytic Network Closure:     │
 │          │ (Self-Sustaining Cycles)    │ Closed catalytic loops in Qchem.   │
 └──────────┴─────────────────────────────┴────────────────────────────────────┘
</pre>

### 4.1. Universal Biological Anchors and Capability Contracts
- **`Q(codon)` (The Genetic Code Anchor):** Enforces the universal translation mapping between nucleotide sequences and peptide synthesis without unmetered amino acid drift.
- **`Q(blanket)` (The Markov Blanket Anchor):** Formalizes cellular membranes as topological boundaries ensuring statistical and causal insulation between internal physiology and external noise.
- **`Q(atp)` (The Metabolic Work Quantum):** Calibrates ATP hydrolysis to the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}: $\Delta G_{\mathrm{ATP}} \approx 20 \, U$, grounding metabolic work directly into Landauer limits ($W \ge k_B T \ln 2$).
- **`Q(homeo)` (The Homeostasis Anchor):** Enforces hard ceilings on internal operational entropy, preventing metabolic collapse.
- **`Q(autocat)` (The Autocatalytic Closure Anchor):** Enforces self-sustaining reaction loops where metabolic enzymes catalyze their own production.

### 4.2. Biological Inconsistency Cascade and Metabolic Halting
If a biological simulation or genetic algorithm proposes a non-viable organism:
1. **Immediate Inconsistency Cascade:** The homeostasis or ATP anchor destabilizes ($\chi \lt \Theta_\chi$), threatening systemic physiological collapse.
2. **Gateway Interception:** The {% include term.html id="simemp-gateway" %} halts execution deterministically, emitting `RECEIPT_METABOLIC_STARVATION` or `RECEIPT_HOMEOSTATIC_COLLAPSE`.
3. **Organism Death as Clean Halting:** Under **Axiom 6** (Totality by Budget), biological death is modeled as an explicit, bounded terminal receipt, preventing unmetered necrotic execution loops.

---

## 5. Morphogenesis on Discrete Simplicial Complexes

### 5.1. Reaction-Diffusion Cochain Systems
Morphogenesis (e.g., Turing pattern formation, embryonic tissue layering) is modeled in $\mathrm{Qbio}$ using Discrete Exterior Calculus ({% include term.html id="discrete-exterior-calculus" text="DEC" %}) on simplicial meshes $K$ (`docs/qm_calculus.md`):
- Morphogen concentrations are represented as discrete $0$-cochains on mesh vertices.
- Diffusion is evaluated via the discrete Laplace–Beltrami operator: $\Delta = \mathbf{d}\delta + \delta\mathbf{d}$.
- Chemical activation and inhibition evaluate through Diophantine integer kinetics in $\mathrm{Qchem}$.

### 5.2. Topological Closure and the Markov Blanket
A biological tissue or single cell possesses an explicit **Markov Blanket**:
- The boundary is discretized as an oriented $(n-1)$-simplicial sub-complex $\partial K$.
- Information and chemical transport across the boundary must cross an explicit gateway.
- Sensory inputs and active motor outputs are metered, preventing unmodeled external state drift ($H(S_{\text{int}} \mid S_{\text{blanket}}, S_{\text{ext}}) = 0$).

---

## 6. Metabolic Pathways as Q(workflows) and Catalytic Networks

### 6.1. Enzymatic Cascades as Q(patterns)
An enzymatic transformation (e.g., glycolysis, Krebs cycle step) is an invariant **{% include term.html id="q-patterns" text="Q(pattern)" %}**:

$$\mathcal{P}_{\mathrm{enzyme}}: \mathcal{H}(\text{Substrate}) \xrightarrow{\quad \mathcal{M}_{\mathrm{enzyme}} \quad} \mathcal{H}(\text{Product})$$

Each enzyme acts as an execution contract requiring an explicit co-factor and an ATP energy packet, dissipating heat according to $\mathrm{Qphy}$ constraints.

### 6.2. Autocatalytic Sets and Category B Optimization
Complex metabolic networks form closed autocatalytic sets (Gánti Chemoton model):
- In $\mathrm{Qbio}$, recurring metabolic cycles leverage {% include term.html id="q-reuse" %}, retrieving cached intermediate enzyme hashes rather than reallocating memory bits across the Memory Wall.
- Allosteric regulation operates via {% include term.html id="q-bypass" %}: feedback inhibition short-circuits redundant metabolic synthesis in $\mathcal{O}(1)$ dispatch time once concentration thresholds are satisfied.

---

## 7. Biological Agency and the Non-Rogue Invariant

### 7.1. Cellular Agency Bounded by Intent Contracts
Biological agency (chemotaxis, immune defense, cellular differentiation) is governed by **Structural Intent Contracts** (`docs/qs.md` §5):
- An immune cell hunting a pathogen operates under an explicit capability contract (`DCC_BIO_IMMUNE_v1`).
- The cell possesses finite authority: it cannot mutate its foundational genesis origin $Q(0)$ or attack self-tissue without violating its DCC boundary.

### 7.2. The Thermodynamic Death Barrier (Metabolic Exhaustion)
Under the **{% include term.html id="non-rogue-invariant" %}**:
1. No biological agent or synthetic cellular automaton can achieve unmetered runaway growth.
2. Every cellular division and signaling step burns an explicit work quota in units of $U$ via {% include term.html id="metric-exhaustion" %}.
3. When local ATP or nutrient budgets reach zero, the agent physically halts, emitting an immutable termination receipt (`RECEIPT_METABOLIC_EXHAUSTION`).

---

## 8. Causal Trajectories: Forward Development vs. Inverse Phylogenetics

$\mathrm{Qbio}$ accommodates both directions across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qbio
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD DEVELOPMENT : Morphogenesis (Zygote ──► Organism)                   │
 │ • Domain : Embryology, synthetic biology, tissue engineering.               │
 │ • Pipeline: Genetic Code ──► Q(workflows) ──► Verified Phenotype.           │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Phylogenetics (Observed Organism ──► Ancestor)     │
 │ • Domain : Paleontology, molecular evolution, viral tracking.               │
 │ • Pipeline: Genomic Receipt S_obs ──► Qlog Abduction ──► Last Common Ancestor│
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 8.1. Forward Embryogenesis and Morphogenesis (Q(workflows))
Forward biological development compiles an initial genetic program into a mature multicellular phenotype:
1. Zygote state $S_0$ is initialized with genetic expression DAGs.
2. Cell divisions and tissue differentiation proceed as an assembled {% include term.html id="q-workflows" text="Q(workflow)" %}.
3. Checkpoints are audited using the {% include term.html id="anchor-cut" text="Anchor-Cut Invariant" %}, verifying developmental fidelity in $\mathcal{O}(K)$ receipt validations without tracking billions of individual cellular micro-steps.

### 8.2. Inverse Causal Reconstruction of Evolutionary Lineage
In evolutionary biology across the {% include term.html id="historical-horizon" %}, researchers measure modern genomic sequences $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`) while ancestral lineages remain unobserved:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle S_0, \, \mathcal{W}_{\mathrm{phylogenetic}} \big\rangle \quad \text{such that} \quad \mathcal{W}(S_0) \equiv S_{\mathrm{obs}}$$

1. **Receipt Ingestion:** Sequenced modern DNA is ingested via verified laboratory sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic mutation models propose ancestral genomes ($A_1 \dots A_n$).
3. **BOC Abductive Resolution:** The {% include term.html id="boc-policy" text="BOC Policy" %} selects the phylogenetic tree that maximizes SIMEMP compliance, selecting the evolutionary trajectory that minimizes parsimony mutations and thermodynamic dissipation.
4. **Lineage Sealing:** The reconstructed ancestral node is sealed as an emergent `Q(pattern)` in the evolutionary tree.

---

## 9. License and Lineage Alignment

All derivative biological simulation models, synthetic biology compilers, metabolic solvers, and genetic execution modules derived from $\mathrm{Qbio}$ must embed the canonical attribution:



$\mathrm{Qbio}$ operates as passive, neutral biological and computational modeling infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external genomic datasets, or sovereign {% include term.html id="payload" text="Payloads" %}.
[[/CODE]]

### II.9 Qn Primitive Envelope v1.4.1 — `qnPrimitive.md`

[[CODE]]
# Qn Primitive Envelope — Governed Qn Artifact Model


---

## Purpose and Scope

This document formalizes the invariant structural container required for every governed computational entity within the Qn universe: the {% include term.html id="universal-envelope" %}.

In this pre-silicon design stage, the primitive envelope provides minimal structural scaffolding to guide artifact construction, algebraic transformation, and candidate execution across [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}). Governed by {% include term.html id="dependencies-determinism" %}, the model eliminates unmeasured ambient state while preserving architectural elasticity against premature calcification through {% include term.html id="structural-indirection" %}.

All formulations represent **provisional consistency abstractions** within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). They define constitutional boundaries rather than finalized implementation parameters, pending empirical hardware validation.

---

## 1. Definitions

### 1.1. Qn Artifact
A {% include term.html id="qn" %} artifact is a governed, finite, identifiable, measurable, and security-constrained computational entity. An artifact is not a passive scalar; it is an immutable state node embedded in a local {% include term.html id="causal-arrow" %}.

### 1.2. Qn Primitive Envelope
The minimal governance boundary required for any computational entity within the {% include term.html id="licensed-work" %} to exist, undergo transformation, achieve sealing, or traverse a {% include term.html id="simemp-gateway" %}, governed under Tier 1 {% include term.html id="existential-invariant" text="Existential Invariants" %} and optimized via Tier 2 {% include term.html id="operational-agility" text="Operational Agilities" %} within the {% include term.html id="simemp" %} framework.

### 1.3. Universal Envelope
The invariant set of structural blocks shared across all artifact types, parameterized via {% include term.html id="structural-indirection" %}:

$$\mathcal{E} = \langle \text{Identity}, \, \text{Type}, \, \text{State}, \, \text{Lineage}, \, \text{DCC}, \, \text{Metric}, \, \text{Security}, \, \text{Exposure}, \, \text{PayloadRef} \rangle$$

### 1.4. Typed Payload Profile
The specialized content block carried by an artifact (numeric quantities, symbolic expression graphs, pure morphisms, state receipts, identity assertions, capability grants, optimization pointers, or composite workflows). External user data remains a sovereign, protocol-blind {% include term.html id="payload" %}.

### 1.5. Identity
The governed witness of an artifact's existence. Identity is {% include term.html id="local-first" %}, collision-resistant, versionable, traceable along causal lineage, and cryptographic-algorithm agile.

### 1.6. Lineage
The directed acyclic causal history of an artifact, recording parent commitment hashes, causal birth index, and transformation receipt references.

### 1.7. DCC Profile
The machine-readable contract defining **Dependencies**, **Constraints**, and **Capabilities**. It serves as an abstract contract rather than a hardcoded implementation binding via {% include term.html id="dcc-profile" text="DCC Profiles" %}.

### 1.8. Metric Envelope
The finite measurement boundary declaring resource limits on storage, expression graph depth, precision bounds, and execution budgets via the {% include term.html id="metric-envelope" %}.

### 1.9. Security Envelope
The governed protection boundary declaring authority limits, delegation proofs, Post-Quantum Cryptography (PQC) algorithm identifiers, and swappable revocation paths via the {% include term.html id="security-envelope" %}.

### 1.10. Lifecycle State
The discrete operational status of an artifact: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, or revoked via the {% include term.html id="lifecycle-state" %}.

### 1.11. Receipt
A bounded, immutable record of a state transition, fault translation, or boundary traversal, acting as a dissipative {% include term.html id="receipt" %} exporting operational entropy.

### 1.12. Unverified Candidate
A proposed computational structure generated via heuristic, neural, or stochastic processes that has not passed gateway verification. Candidates cannot cross a {% include term.html id="simemp-gateway" %}.

### 1.13. Numeric Payload
The specialized profile for numeric entities, explicitly encapsulating {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}.

### 1.14. Exposure Class
The classification governing artifact visibility: local-only, sealable, exportable, attested, quarantined, or revoked.

### 1.15. Q(reuse) Optimization Morphism
A Category B primitive referencing existing immutable content commitment hashes within the local causal DAG. Under physical constraints, Q(reuse) is admissible if and only if its advantage over direct re-evaluation is declared on **both axes** of the Layer-0 Hardware Telemetry Envelope ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4) — the work axis and the latency axis, the **consequentiality constraint** ("consequently faster") [v1.4.0 — CANDIDATE]:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

**Two-axis provenance [v1.4.0 — CANDIDATE]:** every term of the conjunction is a declared deterministic quantity, never a runtime sample. `h` is the referenced commitment; `tier(h)` is its declared memory tier, fixed by the referencing DCC profile's storage binding. `ΔW_compute` is the DCC-declared deterministic upper-bound estimate of direct re-evaluation work, expressed in Abstract Compute Units U and produced by the cost model declared in the morphism's {% include term.html id="dcc-profile" %} — the v1.3.0 provenance discipline, unchanged. `τ_U[tier(h)]` is the declared per-tier worst-case lookup work bound (U axis) and `τ_t[tier(h)]` the declared per-tier worst-case lookup latency bound (cycles against the node's declared reference clock, `docs/simemp.md` §4.4); both are calibrated at Layer-0 onboarding through the Historical Calibration Benchmark receipt class. `Δt_compute_bound` is the DCC-declared worst-case latency bound of the direct compute path — a static bound under F1's declared-bound discipline, never a runtime measurement. `ε_U` and `ε_t` are the DCC-declared fixed gateway overhead slacks (work and latency respectively). `κ_dcc ≥ 1` and `κ_t ≥ 1` are the DCC-declared advantage margins within the governance-bounded range and recommended preset of §5.9 — the operational meaning of "consequently faster": the direct path is not merely cheaper than the lookup, it is cheaper **by a declared margin on both axes**, so the memory-wall delay is exploited rather than suffered. All quantities **MUST** be declared ({% include term.html id="no-implicit-rule" %}); an undeclared quantity forces the safe fallback to Direct Compute (§5.9 fallback precedence), never a guessed admission. The withdrawn v1.3.0 single-axis wording is preserved verbatim in the v1.4.0 Review Integration Record.

### 1.16. Q(bypass) Optimization Morphism
A Category B primitive allowing a {% include term.html id="simemp-gateway" %} to short-circuit execution upon verifying an algebraic identity precondition in $$\mathcal{O}(1)$$ register time, emitting a zero-cost bypass receipt without memory traversal or compute budget consumption.

**[v1.3.0 — CANDIDATE, Review Integration QO-5]** The Layer-3 node-class semantics of this morphism is the `OP_BYPASS` canonical contraction of Node Class 6 ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.6.1, v1.3.0): the fixed sixteen-rule terminating identity suite whose normal form, together with `DCC_SYM_COMMUTATIVE` child sorting, governs canonical node hashing. The former umbrella name `OP_REWRITE` is a **deprecated alias** retained only for resolvability of prior artifacts and is retired at the QnIR v0.2 candidate.

---

## 2. Paradigm Neutrality

This specification does not mandate an object-oriented programming (OOP) paradigm. A Qn artifact is not an OOP object.

The model explicitly rejects:
- Hierarchical class inheritance and dynamic dispatch tables.
- Hidden mutable state and unmetered side effects.
- Implicit type coercions and ambient runtime environments.

Transformations are explicit governed morphisms. State changes yield new immutable artifacts indexed monotonically along the local {% include term.html id="causal-arrow" %}.

---

## 3. Provisional Operational Axioms

The Qn operational model is governed by 14 operational axioms. These axioms represent **candidate search constraints** verified provisionally via {% include term.html id="computational-consistency" %}.

### Axiom 1 — Explicit Envelope
No Qn artifact exists within the governed universe unless it carries an explicit {% include term.html id="universal-envelope" %}:

$$\forall \alpha \in \mathcal{Q}, \quad \exists ! \, \mathcal{E}(\alpha)$$

### Axiom 2 — Finiteness
Every Qn artifact must possess a finite canonical representation and a bounded resource envelope. Actual infinities are physically non-realizable and excluded from execution:

$$\forall \alpha \in \mathcal{Q}, \quad \mathrm{Size}(\alpha) \lt \infty \quad \wedge \quad \mathrm{Budget}(\alpha) \lt \infty$$

### Axiom 3 — Typed Payload Separation
Fields specific to numeric representations ({% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}) belong strictly to typed payload profiles and must not be made mandatory for non-numeric entities. Sovereign user data remains an uninspected {% include term.html id="payload" %}.

### Axiom 4 — Local Genesis Dependency
Every governed Qn artifact within a local universe must trace its lineage directly or transitively to the local genesis origin {% include term.html id="q0" text="Q(0)" %}:

$$\forall \alpha \in \mathcal{Q}_N, \quad Q(0)_N \in \mathrm{Lineage}(\alpha)$$

Local artifacts form single-rooted directed trees originating at physical silicon root $$Q(0)_N$$. Higher-order network entities operating across the Outer Ring (such as {% include term.html id="chronosa" %}) are topological colimits (sheaf gluings) over acyclic forests of mutually verified local Axiom-4 trees.

### Axiom 5 — Acyclic Causal Lineage
Lineage graphs are strictly acyclic, governed by {% include term.html id="dependencies-determinism" %}. Every artifact must be born after its causal dependencies along the {% include term.html id="causal-arrow" %}:

$$\mathrm{Index}(\mathrm{Parent}(\alpha)) \lt \mathrm{Index}(\alpha)$$

### Axiom 6 — Totality of Governed Operations
Every governed operation must terminate in a declared, finite terminal state via dissipative {% include term.html id="metric-exhaustion" %} within its declared budget:
`SUCCESS`, `FAILURE`, `NO_SOLUTION`, `TIMEOUT`, `BUDGET_EXHAUSTED`, `INCONCLUSIVE`, `QUARANTINED`, `PRECISION_INSUFFICIENT`, `EXPRESSION_TOO_COMPLEX`, `REJECTION_BY_GATEWAY`, `DIVISION_BY_ZERO_REJECTED`, or `ORDERING_INDETERMINATE_AT_ZOOM`. Silent non-termination is prohibited.

### Axiom 7 — Receipted Boundary Events
Receipts are mandatory for lifecycle state transitions, gateway crossings, sealing events, export decisions, fault translations, and capability invocations. Receipts serve as verifiable witnesses exporting operational entropy.

### Axiom 8 — Local-first Exposure
Artifact identifiers are rooted locally. Raw local Qn artifacts are never exposed directly to remote nodes across {% include term.html id="cdqn" %} without explicit authorization and bounded public attestation under {% include term.html id="local-first" %} rules.

### Axiom 9 — Numeric Precision Explicitness
Every numeric Qn artifact must declare finite precision bounds, explicit {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}. Implicit rounding and truncation are prohibited:

$$\text{NumericValue} = \langle z, \, r, \, d \rangle$$

### Axiom 10 — No Floating-Point Semantics
IEEE 754 floating-point representations are forbidden at the Qn semantic layer. Non-deterministic mantissa rounding and platform-dependent approximations are rejected.

### Axiom 11 — Bounded Self-Description
Governance metadata may describe its own schema, but self-description depth must remain finite, bounded, and version-anchored to prevent infinite recursive reflection towers.

### Axiom 12 — License Lineage Where Applicable
Lineage metadata and the canonical {% include term.html id="paternity-reference" %} must be preserved in distributed artifacts, exported attestations, compiled {% include term.html id="qnir" %} binaries, and public APIs pursuant to the {% include term.html id="ssl" %}. Encapsulating a Payload does not convert it into a {% include term.html id="derivative-work" %}.

### Axiom 13 — Versioning Creates New Artifacts
Versioning an artifact produces a new immutable artifact with an advanced causal index referencing the predecessor. Mutation in place is prohibited. Successor workflows derived through {% include term.html id="qn-rsi" %} do not overwrite existing execution paths; they are emitted as distinct causal versions.

### Axiom 14 — Generative Candidate Boundary
Unverified candidates proposed by stochastic, generative, or neural processes can exist locally only if explicitly tagged as unverified candidates. Candidates cannot cross a {% include term.html id="simemp-gateway" %} or achieve sealed execution without deterministic validation.

---

## 4. Universal Envelope Structure

The Universal Envelope $$\mathcal{E}$$ is structured using {% include term.html id="structural-indirection" %}, ensuring that cryptographic suites or binary layouts can be upgraded without invalidating existing causal DAGs.

```
                    UNIVERSAL ENVELOPE STRUCTURE
 ┌───────────────────────────────────────────────────────────────────┐
 │ 1. IDENTITY BLOCK      : NodeID, CausalIndex, Type, Version, Hash │
 ├───────────────────────────────────────────────────────────────────┤
 │ 2. TYPE BLOCK          : Primitive, Morphism, Workflow, Receipt   │
 ├───────────────────────────────────────────────────────────────────┤
 │ 3. STATE BLOCK         : Candidate, Sealed, Verified, Quarantined │
 ├───────────────────────────────────────────────────────────────────┤
 │ 4. LINEAGE BLOCK       : ParentCommitmentHashes, BirthOrder       │
 ├───────────────────────────────────────────────────────────────────┤
 │ 5. DCC PROFILE REF     : Dependencies, Constraints, Capabilities  │
 ├───────────────────────────────────────────────────────────────────┤
 │ 6. METRIC ENVELOPE     : MemoryCeiling, DepthLimit, ComputeBudget │
 ├───────────────────────────────────────────────────────────────────┤
 │ 7. SECURITY ENVELOPE   : SecurityRing, PQCAlgoID, RevocationPath  │
 ├───────────────────────────────────────────────────────────────────┤
 │ 8. EXPOSURE BLOCK      : LocalOnly, Exportable, Attested          │
 ├───────────────────────────────────────────────────────────────────┤
 │ 9. PAYLOAD REFERENCE   : Typed pointer to Payload Profile         │
 └───────────────────────────────────────────────────────────────────┘
```

### 4.1. Identity Block
Declares local node identifier, local causal index, version identifier, content commitment hash, and cryptographic agility metadata.

### 4.2. Type Block
Declares the artifact category as an independent top-level descriptor (numeric scalar, expression DAG, morphism, relational assertion, receipt, identity assertion, capability grant, optimization pointer, or composite workflow).

### 4.3. Lifecycle State Block
Declares current governance status conforming to Axiom 7 and the {% include term.html id="lifecycle-state" %}.

### 4.4. Lineage Block
Declares parent artifact references, local causal index, birth order, derivation type, and transformation receipt references.

### 4.5. DCC Profile Reference
Machine-readable reference to an abstract capability contract declaring prerequisites, operational ceilings, and permitted transformations via the {% include term.html id="dcc-profile" %}.

### 4.6. Metric Envelope Reference
Declares finite operational bounds (storage footprint, expression graph depth, recursion limits, and execution budgets) via the {% include term.html id="metric-envelope" %}.

### 4.7. Security Envelope Reference
Declares authority scope, delegation limits, trust assumptions, PQC algorithm identifiers, and swappable revocation endpoints via the {% include term.html id="security-envelope" %}.

### 4.8. Exposure Class Block
Declares visibility scope: local-only, sealable, exportable, attested, quarantined, or revoked.

### 4.9. Payload Reference
A typed, content-addressed pointer to the associated {% include term.html id="payload" %} profile.

---

## 5. Typed Payload Profiles

### 5.1. Numeric Payload Profile
Carries {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}, precision boundaries, and base-independent scale lattice representations ([`docs/qm.md`]({{ '/qm.html' | relative_url }}) §3).

### 5.2. Expression Payload Profile (Content-Addressed Expression DAG)
Under the {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %} ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §7), symbolic expressions ({% include term.html id="qexpr" %}) are represented as **Content-Addressed Expression DAGs (CAE-DAGs)** with cryptographic hash-consing and arity-indexed child commitment vectors ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }})):

$$\alpha = \Big\langle \mathrm{OpCode}, \, \mathbf{D}_{\mathrm{children}}, \, z, \, r, \, d, \, \mathrm{DCC}_{\mathrm{ref}} \Big\rangle$$

- **Deduplication:** Identical sub-expressions share identical commitment hashes, eliminating redundant leaf allocation across the Memory Wall.
- **Complexity Ceilings:** Declares strict graph depth ceilings, maximum node counts, and compile-time collapse budgets.
- **Thermodynamic Invariance:** Reversible normalization within the DAG incurs zero Landauer dissipation ($$W = 0$$).

### 5.3. Operation Payload Profile
Carries domain-to-codomain morphism signatures, formal preconditions, postconditions, metric consumption rates, and receipt emission behavior.

### 5.4. Receipt Payload Profile
Carries terminal execution state, consumed resource budgets ({% include term.html id="metric-exhaustion" %}), causal birth index, parent commitment references, and gateway signatures on each emitted {% include term.html id="receipt" %}.

### 5.5. Identity Payload Profile
Carries {% include term.html id="identity-class" text="Identity Class" %} (Machine Identity, Human Identity, AI Agent Identity), pseudonyms, capability bounds, and revocation endpoints, bounded by {% include term.html id="existential-invariant" text="Identity Invariants" %}. Under the {% include term.html id="non-rogue-invariant" %}, machine identities are subordinated to verified human principals.

### 5.6. Attestation Payload Profile
Carries public commitments, issuer pseudonyms, capability proofs, metric summaries, and SSL lineage assertions for network exposure across {% include term.html id="cdqn" %}.

### 5.7. Capability Payload Profile
Carries granular execution rights, operational constraints, delegation depths, and expiration bounds.

### 5.8. Workflow Payload Profile
Carries a directed acyclic graph of governed morphisms, input state prerequisites, execution checkpoints, intermediate collapse budgets, and terminal receipt criteria. Underpins composite state workflows ({% include term.html id="q-workflows" %}) and recursive self-optimization loops ({% include term.html id="qn-rsi" %}).

### 5.9. Optimization Payload Profile (Category B Primitives)
Governs explicit sub-graph reuse and gateway short-circuiting under physical thermodynamic constraints:

#### Q(bypass) Execution
Checks immediate operands for algebraic identity conditions in $$\mathcal{O}(1)$$ register time (such as unit multiplication $$\times Q(1)$$, zero addition $$+ Q(0)$$, rotor normalization $$R R^\dagger = Q(1)$$, or coboundary nilpotency $$\mathbf{d} \circ \mathbf{d} = Q(0)$$). When detected, the gateway short-circuits execution and emits `RECEIPT_BYPASS_IDENTITY` at zero compute cost ($$\Delta W = 0$$).

#### Q(reuse) Execution & Two-Axis Consequentiality Breakeven **[v1.4.0 — CANDIDATE]**
References an existing immutable content commitment hash within the local causal history. Querying a hash commitment across physical memory hierarchies costs declared lookup work (bus dissipation, `O(C·V²·f)`) and declared lookup latency (the Memory Wall). Under the Metric Invariant and the consequentiality constraint of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4, admission requires the conjunction of §1.15 on **both axes**:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

If the direct path fails to clear either margin (e.g. single-cycle integer ALU operations against an L1-resident commitment), the runtime executes direct computation (the declared fallback), bypassing hash-table lookups to prevent Memory Wall bloat; ties resolve to Direct Compute.

**Admission Ledger (reserve/charge/refund) [v1.4.0 — CANDIDATE].** Every admitted reuse executes as a declared four-field accounting, closing the double-charge defect (Loop-2 finding F2) and the ledger-transparency demand (Objection-4). The ledger is a declared accounting, not a measurement: all four fields are deterministic declared quantities, receipt-replayable on any peer.
- `W_gross_reserve = ΔW_compute` — the pessimistic ceiling reserved from the invoking budget at admission (an admitted reuse never costs more than direct re-evaluation, or it would not have been admitted);
- `W_actual_charge = τ_U[tier(h)] + ε_U` — the declared lookup work of the executed path, charged on completion; the base compute work is never charged on top of reuse, so the F2 double charge is closed by construction;
- `W_refund = W_gross_reserve − W_actual_charge` — returned to the invoking remaining budget at completion;
- `W_net = W_actual_charge` — recorded on the reuse receipt as the net metric cost of the morphism.

All four fields are recorded in the reuse receipt. **Fault and recovery semantics [v1.4.1].** A faulted lookup (referenced commitment missing from the local store, with no declared fallback block) is not an unregistered `HALTED` terminal: the sealed Axiom-6 registry admits no `HALTED` status. The gateway instead emits `FAILURE`, or `REJECTION_BY_GATEWAY` when the loss is detected as a pre-dispatch refusal, and releases the full `W_gross_reserve` back to the invoking budget — `0 U` is charged in the inner reuse ledger beyond the outer `W_gate` fee, which remains charged at the boundary under `docs/simemp.md` §6. By contrast, a recoverable lookup miss carries `STATUS_LOOKUP_MISS` and routes to the declared fallback block; it settles at terminalization under Conservation Identity 3 (`refund_final = I_opened * B(P) − charged`, `docs/qnir_accounting.md` §3.3), so the four-field ledger is macro-realized by the inner escrow without creating budget mid-run. No new Axiom-6 terminal status is introduced.

**Margin governance [v1.4.0 — CANDIDATE].** `κ_dcc` and `κ_t` are declared per DCC profile within the governance-bounded range `[1, 64]`; the recommended preset is `κ_dcc = 2`, `κ_t = 4`. The asymmetry encodes the memory wall: the work axis is already bounded by the profile's declared cost model, so a tight factor suffices, while the latency axis carries the machine-condition variance of the wall, so the direct path must clear the touched tier's worst-case lookup by a factor of four before the lookup is worth its stall. The floor `κ ≥ 1` excludes bare breakeven — ties resolve to Direct Compute — and the ceiling bounds margin inflation. The range and preset are the BOC-selected values recorded in the v1.4.0 Review Integration Record.

#### The Local Evaluation Pipeline
Within any single morphism evaluation step, the gateway enforces strict causal precedence:

$$Q(\text{bypass}) \prec Q(\text{reuse}) \prec \text{Direct Compute}$$

**[v1.3.0 — CANDIDATE, Review Integration QO-5]** At the macro-DAG scale (Layer 3), rewrite executes under the Node Class 6 split ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.6, v1.3.0): the canonical contraction suite `OP_BYPASS` applies the fixed $$\mathcal{O}(1)$$ identity rules and governs canonical node hashing, while general equality saturation is legal **only** as `OP_SATURATE` inside an explicit $$Q(\text{rsi})$$ workflow whose DCC profile pins a deterministic rule-application schedule, iteration/node/depth ceilings, and a deterministic hash-tie-broken cost extractor — emitting a **new versioned artifact (Axiom 13)** rather than mutating canonical identity. Within a saturation pass, sub-graph unifications via $$Q(\text{reuse})$$ may reveal newly adjacent terms that trigger downstream `OP_BYPASS` cancellations; the causal precedence of this section is unchanged.

#### Canonical Circuit-Breaker Receipt Registry **[v1.3.0 — CANDIDATE]**
**[Review Integration QO-5 / data-loss audit]** The Layer-3 structural circuit breakers ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §6) emit the following canonically named receipts, registered here as the operational-axiom authority (Axioms 6 and 7). No new Axiom-6 terminal states are required:


Two non-breaker canonical receipts complete the registry: `RECEIPT_SATURATION_CONVERGED` (terminal state `SUCCESS`) is emitted when an `OP_SATURATE` workflow halts on convergence before any ceiling, and `RECEIPT_BYPASS_IDENTITY` (zero metric cost, $$\Delta W = 0$$) is emitted by every `OP_BYPASS` contraction. Breaker 3's receipt name is declared here for the first time; [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §6 inherits it at its next versioned touch-up. Item 5 is the dimensional guard: its `FRAME_SIGNATURE_INSUFFICIENT` reason originates in [`docs/qm.md`]({{ '/qm.html' | relative_url }}) even-root handling and terminates under the sealed Axiom-6 status `EXPRESSION_TOO_COMPLEX`, never a new status.

---

## 6. Identity and Lineage Rules

### 6.1. Local-first Identifier Structure
An artifact identifier is rooted strictly in its {% include term.html id="local-first" %} genesis:

$$\mathrm{ID} = \langle \text{NodePseudonym}, \, \text{CausalIndex}, \, \text{Type}, \, \text{Version}, \, \text{Commitment} \rangle$$

### 6.2. No Global Raw Identity Exposure
Raw local node identifiers are prohibited from direct network exposure. Network interaction across {% include term.html id="cdqn" %} uses accountable pseudonyms, cryptographic commitments, and zero-knowledge capability proofs.

### 6.3. Versioning
Every modification produces a distinct artifact with an advanced causal index. Predecessors remain immutable components of historical lineage.

### 6.4. Algorithm Agility
Cryptographic primitives must support seamless migration via {% include term.html id="structural-indirection" %}. No single signature suite or hash function is treated as a permanent dependency.

---

## 7. DCC Profile Rules

### 7.1. Dependencies
Explicitly declares required parent artifacts, signing identities, capability grants, and physical context prerequisites under {% include term.html id="dependencies-determinism" %}. Dependencies are evaluated as abstract contracts via {% include term.html id="structural-indirection" %} to allow modular implementation swaps.

### 7.2. Constraints
Declares hard operational ceilings: memory allocation bounds, recursion limits, {% include term.html id="complexity-degree" text="complexity degree" %} ceilings, precision limits, and excluded states (e.g., denominator zero exclusion).

### 7.3. Capabilities
Declares permitted transformations: morphism rights, sealing authorization, network export authorization, and delegation scopes.

### 7.4. Semantic Alignment & Anti-Drift Invariant
To prevent translation drift between high-level human intent and lower-level machine execution, an artifact's DCC contract must bind its human-readable specification in {% include term.html id="qs" text="Qs" %} directly to its intermediate execution representation in {% include term.html id="qnir" %}. A state transition is valid if and only if the verification receipt certifies that execution did not drift from the declared intent contract ($$H(\mathrm{Intent} \mid \mathrm{Execution}) = 0$$).

---

## 8. Metric Envelope Rules

Every {% include term.html id="metric-envelope" %} must declare finite, measurable boundaries:
- **Footprint Bounds:** Storage allocation size, expression DAG depth ceilings, and {% include term.html id="complexity-degree" text="complexity degree" %} bounds.
- **Execution Bounds:** Step ceilings, transformation budgets, and compilation collapse allocations.
- **Numeric Bounds:** Scale zoom ceilings ($$z \le z_{\max}$$) and uncertainty boundaries.
- **Memory Wall Accounting:** Explicit accounting for data movement, cache locality, and bus transmission dissipation.

---

## 9. Security Envelope Rules

Every {% include term.html id="security-envelope" %} must define protection parameters against computationally bounded adversaries:
- Inner-ring and outer-ring scoping conforming to the {% include term.html id="dual-ring-pqc-boundary" %}.
- Swappable revocation paths and emergency halt authorities designed for zero-day patchability.
- Post-Quantum Cryptography (PQC) algorithm identifiers with explicit migration semantics.
- Accountable pseudonymity; anonymous entities are strictly forbidden from traversing network boundaries.

---

## 10. Lifecycle and Receipts

```
                      ARTIFACT LIFECYCLE FLOW
 [Candidate] ──► [Constructed] ──► [Sealed] ──► [Verified] ──► [Exportable] ──► [Exported]
      │               │              │             │
      ▼               ▼              ▼             ▼
  [Rejected]      [Halted]     [Quarantined]   [Revoked]
 (All fault and terminal states emit immutable dissipative Receipts)
```

### 10.1. Lifecycle States
Standard recognized operational states conforming to the {% include term.html id="lifecycle-state" %}: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, revoked.

### 10.2. Candidate State
A proposed stochastic or heuristic structure; classified strictly as an unverified candidate; prohibited from crossing a {% include term.html id="simemp-gateway" %}.

### 10.3. Constructed State
Formalized into canonical envelope syntax; awaiting constraint evaluation.

### 10.4. Sealed State
Identity-committed, metric-bounded, security-checked, and immutable.

### 10.5. Verified State
Validated against explicit formal proof invariants or test suites by an authorized verifier.

### 10.6. Rejected, Halted, and Quarantined States
Protective terminal states emitting mandatory failure receipts exporting operational entropy.

### 10.7. Exportable and Exported States
Certified by the {% include term.html id="simemp-gateway" %} for controlled exposure across the {% include term.html id="cdqn" %} network via the Exposure Functor.

### 10.8. Revoked State
Invalidated via an explicit, signed revocation receipt that permanently seals a compromised branch and migrates lineage.

---

## 11. Relation to Abstraction Layers

### 11.1. Layer 0 (Physical Substrate)
Raw physical hardware noise and clock signals in {% include term.html id="layer-0" %} carry no primitive envelope until processed across the onboarding gateway ([`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2).

### 11.2. Layer 0 to Layer 1 Gateway
Harvests physical entropy, generates Inner-Ring PQC keys, and constructs local origin {% include term.html id="q0" text="Q(0)" %} and first unit {% include term.html id="q1" text="Q(1)" %} across the {% include term.html id="simemp-gateway" %}.

### 11.3. Layer 1 (Node Genesis)
Houses foundational Category A primitives in {% include term.html id="layer-1" %} ({% include term.html id="q0" text="Q(0)" %}, {% include term.html id="q1" text="Q(1)" %}, single digits $$Q(2)\dots Q(9)$$, the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}, and genesis axis $$d_1$$).

### 11.4. Layer 2 (Governed Morphisms)
Hosts elementary arithmetic morphisms ($$+ $$, $$- $$, $$\times$$, $$\div$$), spatial reflection involutions ($$\mathcal{I}_d$$), and Category B optimization morphisms ({% include term.html id="q-reuse" text="Q(reuse)" %}, {% include term.html id="q-bypass" text="Q(bypass)" %}).

### 11.5. Layer 3 (Symbolic Composition)
Hosts Content-Addressed Expression DAGs ({% include term.html id="qexpr" %}), multi-axial Clifford geometric frames, and compile-time reduction engines.

---

## 12. Relation to QnLang and QnIR

The primitive envelope provides the formal execution target for the programming substrate:
- **{% include term.html id="qnlang" %}:** Authors computations declaring explicit DCC profiles, metric budgets, and payload profiles. Prohibits implicit defaults and floating-point semantics under the {% include term.html id="no-implicit-rule" %}.
- **{% include term.html id="qnir" %}:** Intermediate representation flattening Content-Addressed Expression DAGs into linear, bounded instruction streams for deterministic hardware evaluation.
- **Compile-Time Collapse:** Symbolic expression DAGs must collapse deterministically to concrete canonical leaf tuples prior to terminal {% include term.html id="receipt" %} emission.

---

## 13. License Alignment

This specification strictly enforces the legal provisions of the {% include term.html id="ssl" %}:

All derivative implementations, hardware drivers, runtime specifications, and APIs derived from the {% include term.html id="licensed-work" %} must embed the canonical {% include term.html id="paternity-reference" %}:


1. **Anti-Patent Defense:** Commercial or {% include term.html id="derivative-work" text="Derivative Works" %} licenses terminate automatically upon initiating patent litigation against the Author or project ecosystem.

### 13.3. Scale Auditing

---

## 14. Exclusions

The following items are outside the scope of this foundational primitive envelope and belong to dedicated specifications:
1. Detailed physical entropy sampling and NIST SP 800-90B health test implementations (specified in [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }})).
2. High-level grammar and syntax rules for QnLang (Milestone 2).
3. Virtual machine instruction set architecture and register schedules for QnIR (Milestone 2).
4. Physical simulation equations and discrete quantum Hamiltonians for Quang Physics ($$\mathrm{Qphy}$$).
5. Concrete multi-party threshold signature ceremony protocols across distributed swarms (specified in [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }})).

---

## 15. Open Items

The following formal specifications remain open for subsequent releases:
1. Canonical binary serialization format (e.g., deterministic CBOR or custom packed Qn-binary) for the Universal Envelope.
2. Formal schema definition for Post-Quantum Cryptography algorithm agility migration certificates.
3. Automated verification checker specification for state transitions from Sealed to Verified.
4. Execution mechanics and termination bounds for higher-order recursive self-optimization workflows ({% include term.html id="qn-rsi" %}).
[[/CODE]]

### II.10 QnIR Operational Semantics §2 — `qnir_semantics.md`

[[CODE]]
# QnIR Semantics — Operational Semantics of the QnIR v0.3 Working Draft (Repair Step 2)

This file carries §2 (§2.1–§2.8) of the QnIR v0.3 working draft, split content-identically from the committed v0.3.0-draft.5 monolith with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. The §2 body below is committed at v0.3.0-draft.6 (Loop-3 stage R2); §2.3 and §2.5 were amended at v0.3.0-draft.8.2 (Finding F25, Reading R1: the `CLOAD` miss edge is a counted edge subject to the declared trip ceiling, traversal-counted at its fallback-edge site and rejected before a repeated miss token is born when the ceiling is exhausted), and the front-matter `terms_used` lists exactly the distinct `term.html` include ids of this body under the include-parity discipline.

**Revision record (v0.3.0-draft.8.4 — Wave 1A specification alignment, 2026-11-18).** This additive amendment advances §2.1's pinned semantics version from `v0.3.0-draft.5` to the current epoch, records the epoch's behavioral compatibility on the committed corpus, names the epoch's golden profile `qir-prof-golden-2`, and adds the normative Terminal Outcome Lift Table (`Φ_status`) to §2.5. No transition rule, opcode, encoding word, token birth or consume path, terminal outcome, or committed vector changes; the lift table is a reporting projection at the boundary of the existing four-outcome vocabulary, never a fifth outcome, and both implementations continue to reproduce the corpus byte-exactly. The amendments are normative only where this file is the owning specification (§2); the accounting-side `B_cfg`/`B_sem` split and pre-DP gate are owned by `docs/qnir_accounting.md` §3.4.

---

## 2. Operational Semantics (Repair Step 2)

This section discharges the second step of the repair sequence on the byte-unchanged freeze of §1. The operative instruction is the Step-2 row of the sequence, preserved verbatim: action — "Publish operational semantics"; acceptance condition — "Typed configurations, small-step transitions, block arguments, dynamic instances, token lifecycle, failure behavior, and deterministic encodings. State determinism as `Eval(Intent, D, V) = Execution` with all semantic versions explicit." The inheritance register that headed this step in the forward map is fixed input and is preserved here verbatim so that the map of §5 can retire the row without losing it: "The nine-family inventory as the closed construct set; the F3 block-argument model; the Obj 5/`QO-22` lifecycle surface; the F5 transition-record obligation list; the fixed-width/exact reconciliation (§1.4)." Each register item is consumed at a named subsection (§2.2, §2.4, §2.5, §2.6, §2.7), and none is re-adjudicated: the registers of §1 bind this section as written, and the semantics defines the machine over them rather than around them.

Two declaration disciplines govern every choice below. First, the default-deny closure rule of §1.1 carries into semantics unchanged: only the nine frozen families receive transition rules, and a construct of a deferred family is not a runtime trap but a statically illegal program (§2.3). Second, every policy choice this section makes — the reconciliation stance of §2.7, the terminalization function of §2.5, the clock advance rule, the encoding layout of §2.6 — is declared explicitly and binds under the semantics version of §2.1; under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline, nothing in the transition relation is resolved by silence, convention, or implementation accident.

### 2.1 The Determinism Statement

The determinism claim of this section is stated exactly as the acceptance condition requires:

> **`Eval(Intent, D, V) = Execution`** — for every well-formed triple `(Intent, D, V)`, evaluation yields exactly one execution.

The triple is bound as follows. **Intent** is the program artifact: the typed block graph with its block-argument signature (§2.4), the instruction sequence of each block over the nine frozen families, the declared type bound of every value, the declared fallback edge of every lookup (§2.5), the extension side table for non-uniform per-instruction attributes (§2.6), and the profile reference carrying the declared trip ceilings and type-bound parameters. Intent is *well-formed* iff it satisfies the static legality rules of §2.3–§2.6: only core-family instructions; an arity-closed, consistently typed block-argument graph; every value carrying a declared type with a declared bound; every lookup carrying its declared fallback edge; every flag combination known and every `EXT` side-table reference resolving; and a profile reference present, since ceilings and bounds are profile parameters (Obj 2). A program that mentions a deferred family, carries an unknown flag combination, or cites a nonexistent side-table record is not a legal Intent and has no semantics — rejection is static, at Intent formation. **D** is the dependency environment: the content-addressed input environment (constant-table entries and input values, each cited by content address) that execution may read, under {% include term.html id="dependencies-determinism" text="dependencies determinism" %}; execution reads no input that Intent does not declare. **V** is the explicit version tuple `(semantics version, encoding version, profile version)`: the semantics version is this file's version — v0.3.0-draft.8.4 pins the transition relation, the instruction-level closure, the lifecycle algebra, and the total terminalization function of this §2; the encoding version pins the layout of §2.6; the profile version pins the declared ceilings and bounds that ride Intent. Versions are part of the determinism claim, not metadata about it: a change to any component of `V` is a different `Eval`, and the transition record of §2.6 binds all three into every committed transition.

**Epoch compatibility (v0.3.0-draft.8.4).** The pin advances across the Loop-3 repairs without changing behavior on the committed corpus: `X1`–`X50` are behaviorally invariant from `v0.3.0-draft.5` through `v0.3.0-draft.8.4` — identical transition sequences, ledger postings, and terminal outcomes — and `X51` requires `v0.3.0-draft.8.2+`, because it exercises the `CLOAD` miss-edge trip ceiling of F25 (§2.3, §2.5). The golden profile of this epoch is `qir-prof-golden-2`, which adds two declared admission parameters: the state-cardinality ceiling `S_max = 4096`, checked before the admission dynamic program (`docs/qnir_accounting.md` §3.4), and the `val(...) == None` refusal rule, under which a `None` bound is refused at admission with `EXPRESSION_TOO_COMPLEX` rather than defaulted to the gateway charge. Both are profile parameters of the epoch; neither adds a transition rule nor changes `V`'s semantics component.

**Execution** is the maximal transition sequence from the initial configuration under the relation of §2.3, together with the complete ordered list of transition records and the terminal configuration. The initial configuration is declared, not computed: entry block with its arguments bound from `D`, the declared initial status pool (empty unless the Intent declares entries — the conformance birth path of §2.5, at most one token per declared type) and receipt queue, zero accumulators, zero clock, step index 0, state running, version bindings `V`. The semantics makes no universal termination claim: bounded loops carry declared ceilings, but whether a given Intent terminates within them — and how budget admission certifies that statically — is the Step-3 deliverable (F4). The claim this section makes is weaker and exact: *if* a well-formed triple's evaluation reaches halt, the halt is reached by exactly one transition sequence, and that sequence is fully recorded.

### 2.2 Typed Configurations

A **configuration** is the typed state tuple below. Every component is finite at every step, every component is encoded by the canonical encoding of §2.6, and every component cites the register item that binds it.


Work quantities are measured in the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} under the declared envelopes of the {% include term.html id="simemp" text="SIMEMP" %} constraints, as in §1; no work value is *computed* by this semantics — the accumulators are updated by transition rules whose charges are declared quantities, and the ledger equation that makes them sound is Step 3's deliverable, not this section's.

Two identity disciplines separate what §1 froze. **SSA name identity** is the pair `(static value id, instance id)`: two instances of one static id are distinct names, and no transition ever overwrites a bound name — redefinition is represented by minting a fresh instance (§2.4). **Denotational value identity** is equality of full canonical encodings under minimal-zoom normalization for zoom-carrying values (`QO-24`): a tuple equals another tuple only by full canonical encoding, never by header or hash equality (F6), and the zoom history of a value is not part of the value — it rides the artifact-lineage record whose commitment is Step 5's. The normalizer of comparison (§2.3, family 4) is declared deterministic in this semantics; the proof that its minimal representation is unique at every lattice boundary is a named demonstration obligation carried to the promotion artifacts, recorded here rather than silently assumed.

### 2.3 The Small-Step Transition Relation

The transition relation `C → C'` executes exactly one instruction per step, and it is a partial function: for every configuration `C` at most one successor `C'` exists. The core has no scheduling freedom to abstract away — branch conditions are exact predicates (family 4), the token lifecycle of §2.5 declares its own tie-breaks, and the terminalization function's priority is lexicographic — so determinism is a property of the declared rules, not of an implementation's discipline. The rule inventory below covers the nine frozen families and nothing else; it is the closed set of §1.2 restated as dynamics.


**Extension boundary.** A construct of a deferred family — calls, saturation, containers, fork–join, road-proof discharge — has no transition rule, and a program containing one is statically illegal at Intent formation (§2.1). This is the §1.1 closure rule carried into dynamics: the boundary is a specification statement, not an extension specification, and the validator that enforces it mechanically is a Step-4 artifact.


**Shared rule postulates.** Every rule below is subject to the same postulates, stated once: (i) **pre-dispatch typing** — every operand is checked against its declared type bound before the operation executes, and a violation is the typed rejection of §2.5 (the `EXPRESSION_TOO_COMPLEX` birth, terminalizing at birth); (ii) **one record** — every completed transition emits exactly one transition record under the F5 field list and the deterministic encoding of §2.6, whatever the rule; (iii) **one charge** — every completed transition posts its rule's declared charge class, site-stable over declared operand bounds per §3.2, with every coefficient riding the profile version of `V` (no numeric charge table exists anywhere in this draft, per §3.6; the class names below are the keys the per-class ledger traces of §4.4 will publish, one trace per class); (iv) **closed status surface** — a rule touches the token pool only through the lifecycle verbs of §2.5 and only at the birth and consume sites its row declares; (v) **static binding** — every value reference in a legal Intent is bound before use by a dominating definition or a block-argument source on the entry edge chain, and carries the declared type and bound of its definition site, so dynamic type and arity errors are statically excluded, not runtime events; (vi) **no invention and no steering** — a construct of a deferred family has no rule (the extension boundary above), and no rule reads a work accumulator (§3.3).

**The closed instruction inventory** (family numbers are the rows of §1.2; family 7 declares no opcode — its transition surface is postulate (ii), which is exactly the committed family row):


**The condition grammar.** The declared condition codes are the closed set {`LT`, `LE`, `EQ`, `NE`, `GT`, `GE`}, evaluated over the comparison-domain value r ∈ {−1, 0, 1} by the declared total predicates: `LT`: r = −1; `LE`: r ≤ 0; `EQ`: r = 0; `NE`: r ≠ 0; `GT`: r = 1; `GE`: r ≥ 0. Every code decides on every result — the grammar is total, closed, and deterministic, and an `OBS` result (bound [0, 1]) is a legal comparison-domain value, so a branch can test token presence (its `NE` code) without any new opcode.

**The zoom normalizer and the zoom-boundary comparison (executable functions).** A zoom-carrying value is a declared value form: a pair (m, z) of an exact integer mantissa and an integer zoom exponent, each within the pair's declared ranges; the pair denotes the exact dyadic quantity m·2^z, and a program declaring no zoom ranges contains no zoom-carrying values, with every rule above unchanged on it. No floating point exists anywhere in the relation. The **minimal-zoom normalizer** is the executable function N: N(0, z) = (0, 0) — the canonical zero of §2.6; for m ≠ 0, write m = 2^k·q with q odd (k the exponent of 2 in m), and N(m, z) = (q, z + k) — the unique odd-mantissa form of the dyadic, so the normalizer's minimal representation is unique by the uniqueness of the odd-mantissa decomposition of a dyadic rational (the demonstration obligation that §2.2 and `QO-24` carry to the promotion artifacts stands, now over a declared function instead of a name). The **zoom-boundary comparison** is the executable predicate the `CMP` rule applies under N: two values are equal exactly when their normalized pairs are equal; otherwise, for normalized (q₁, z₁) and (q₂, z₂) with z₁ ≤ z₂, the ordering is decided by comparing q₁ against q₂·2^(z₂−z₁) in exact integer arithmetic — the exponent difference is non-negative, so the shift is exact and the comparison never rounds. A zoom exponent outside its declared range is the `CMP` pre-dispatch rejection above; there is no boundary behavior beyond the declared rejections — behavior at declared zoom boundaries is part of the declared function exactly here.

**The tuple field structure.** A tuple value carries its declared signature: the field count k and the declared type and bound of each field, both static instruction data; the canonical encoding of §2.6 encodes the fields in declared order over the components' canonical encodings (the complete byte grammar is §2.6's, committed at v0.3.0-draft.6 — the semantic structure is fixed here). Field identity is positional under the declared signature; there is no field-name resolution, no header-only comparison (F6), and no default field.

**Derived counters (no configuration change).** The trip count of a static back-edge site is the number of its prior traversals in this execution, derived canonically from the committed record sequence — each traversal is visible in the record's PC field. The ceiling preconditions of `BR`, `JMP`, and the `CLOAD` miss edge compare these derived counts against the declared trip ceilings. **The depth ceiling is retired (F26, v0.3.0-draft.22).** Earlier drafts declared a depth ceiling on "nesting depth at a block entry" without a complete operational measure of nesting depth for arbitrary block graphs, and no execution rule ever enforced it. The retirement is a withdrawal with its reason recorded, not a silent deletion: every block-graph cycle contains a back-edge (a back-edge's target dominates its source, §2.6), every back-edge site carries a finite declared trip ceiling, and so the trip ceilings alone certify that every execution is finite (§3.4) — the depth counter contributed nothing to that certification and is not derived. A `CLOAD` hit traverses no edge and increments no count; a `CLOAD` miss that completes increments the count of its fallback-edge site (block id, instruction index, slot 0); no component is added to the configuration tuple of §2.2, which stands as committed, and an implementation may maintain the counts incrementally under the obligation that the maintained value equals the derived one at every step.

### 2.4 Block Arguments and Dynamic Instances (F3 Discharged)

Control is frozen as basic blocks with explicit block arguments, and the dynamics make the v0.2 defect impossible by construction. Every edge of the block graph — forward, branch, and back-edge — supplies operand values for exactly the target block's declared arguments, and entering a block mints a fresh instance id for each of its arguments. Loop headers therefore take their per-iteration state as block arguments, and a back-edge operand **may occur later in the static instruction order than the branch that cites it**: the operand is resolved when the edge executes, at block entry, not at static scan. This is precisely the named defect of F3 — "loop headers need block arguments whose back-edge operands may occur later statically" — discharged as a static well-formedness rule (the block-argument graph is arity-closed: every edge supplies the target's arity, typed consistently) plus an entry-time resolution rule (§2.3, control row).

Within a block, each instruction that defines a value binds it in the current instance context, so one value per static instruction is a statement about *names* — the pair `(static value id, instance id)` — and never about storage slots: per-iteration redefinition mints a new instance instead of overwriting an old one. Instance ids are minted in transition order, are execution-scoped, and are never reused within an execution, which makes the dynamic value store's canonical encoding (§2.6) well-defined at every step. The store is the abstract semantic state; what a profile does with it — liveness windows, register allocation, spilling under `QO-18` — is realization, not semantics: no capacity constant of any profile is a validity limit of this relation (Obj 2).

### 2.5 The Token Lifecycle Algebra and Failure Behavior (Obj 5 / `QO-22` Discharged)

The review's verdict on `QO-22` is preserved as the design constraint: "The two-tier token idea is sound; the v0.2 partition and mappings are not." This section therefore declares the v0.3 partition, the lifecycle verbs over it, and the failure behavior of the core, with no silent mapping anywhere.

**The two tiers.** **Tier O — operational tokens** are recoverable by specified fallback behavior and carry no semantic verdict. Declared member: `STATUS_LOOKUP_MISS` — operational control flow, consumed by the declared recompute fallback: the fallback edge is declared in Intent (§2.1), a lookup miss routes control to that edge's target, and the token is consumed only when that fallback entry completes successfully — the fallback block completes, births no token, and ends in no typed rejection; a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site; each such repeated miss is a further traversal of the fallback edge and is subject to the same declared trip ceiling, and the traversal that would exceed it is rejected under §2.3 before its token is born, and every birth and consumption is traced (Loop-3 F12 repair, v0.3.0-draft.5 — a miss token is never consumed by an unsuccessful fallback). **Tier S — semantic tokens** are outcomes that propagate to the exit unless the program explicitly consumes or promotes them. Declared members: `STATUS_NEG_EVEN_ROOT` — maps to `NO_SOLUTION` in the real frame unless the program explicitly promotes the frame by the declared typed instruction `PROMOTE` of §2.3, in which case the valid promotion consumes the token and the wider-frame rule of the terminalization function below applies; without promotion the token terminalizes as `NO_SOLUTION` at every exit — promotion is the act that recovers the operation in the wider frame, never the condition for a real-frame no-solution verdict (the Loop-3 F12 repair restores §1.2's committed "unless" wording as the correct one; in the frozen core the token's birth path is the declared initial status pool declared below, its extension birth sites re-entering with root extraction at Step 6 under this exact lifecycle surface); and `EXPRESSION_TOO_COMPLEX` — the resource/representation-limit token, born by the typed rejections of §2.3 (a value exceeding its declared bound before dispatch, an exhausted declared trip ceiling); and `STATUS_DOMAIN_ERROR` — the typed domain-error token, born by the zero-divisor rejections of the `DIV`/`MOD` rules of §2.3, terminalizing at birth: an undefined integer operation is surfaced as a typed failure, never silently mapped (Obj 5), and the closed vocabulary is thereby total over every declared rule's rejection surface.

**Lifecycle verbs.** Birth (typed, sited at `(step index, position)`), observation (a typed predicate that reads a token's presence and consumes nothing), consumption (only by a successful Tier O fallback completion or by the declared `PROMOTE` instruction — the two consume paths of the closed vocabulary, per the terminalization function below), propagation (unconsumed tokens ride the pool across transitions unchanged), and terminalization (at halt, every outstanding token terminalizes and is recorded in the terminal receipt — unresolved semantic debt at every exit is surfaced, never discarded, which is the family-9 scope boundary of §1.2). The `QO-22` test list — success-after-recovery, unresolved debt at every exit, multiple outstanding tokens — passes to Step 4 as the conformance suite's input, unchanged.

**Failure behavior and the total terminalization function (F12 repaired, [v0.3.0-draft.5 — DRAFT]).** A typed rejection is fatal: the machine cannot proceed past a rejected instruction, the transition that births the fatal token is the final transition, and the fatal state is sealed in that transition's own record — a later halt instruction cannot re-seal it (F7). The Loop-3 review's F12 finding is accepted and repaired here: the draft.4 terminal-wins rule selected the program's halt outcome whenever no fatal token existed, which permitted a `SUCCESS` exit carrying unresolved semantic debt — the success-shaped unresolved outcome `QO-22` rejected. The rule is replaced by a **total terminalization function** T, defined for every exit event and every configuration, over two closed vocabularies. **The closed terminal outcome vocabulary** (enumerated, closed — no other terminal outcome exists, and no outcome is reachable that the vocabulary does not name): `SUCCESS`; `NO_SOLUTION`; `EXPRESSION_TOO_COMPLEX`; `DOMAIN_ERROR`. The fuel backstop of §3.4 stops the machine fatally "with the same token, itemized" — the `EXPRESSION_TOO_COMPLEX` birth of the budget refusal — and the admission refusal of §3.4 is the same token's pre-execution refusal execution: neither adds a vocabulary member. **The closed status token vocabulary**, with every birth and consume path declared and none implicit: Tier O — `STATUS_LOOKUP_MISS` (birth: the `CLOAD` miss of §2.3; consume: a successful fallback completion, per the tier rule above). Tier S — `STATUS_NEG_EVEN_ROOT` (birth: the declared initial status pool, and extension birth sites at Step 6; consume: the valid `PROMOTE`), `EXPRESSION_TOO_COMPLEX` (birth: the typed rejections of §2.3 and the §3.4 backstop and refusal; no consume path), `STATUS_DOMAIN_ERROR` (birth: the zero-divisor rejections of §2.3; no consume path). **The function.** T is defined over the three exit events, and is total. (1) *A terminalizing birth or the fuel backstop* exits at its own transition with the birthed token's outcome: `EXPRESSION_TOO_COMPLEX` for the typed rejections and the backstop, `DOMAIN_ERROR` for the domain rejection — the sealed fatal state of the committed failure behavior, unchanged; at most one fatal token is ever outstanding, because a fatal birth is itself immediately terminal. (2) *The `HALT` instruction*: with any Tier S token outstanding, the outcome is determined by the earliest-birth outstanding Tier S token — ordered lexicographically by the full token identifier of §2.6 — `(birth step index, block id, instruction index, token serial, declared type id)` — total because the token serial is unique per execution by construction (F15, v0.3.0-draft.6): a `STATUS_NEG_EVEN_ROOT` selects `NO_SOLUTION` (without promotion — the real-frame verdict of §1.2's "unless" rule), an `EXPRESSION_TOO_COMPLEX` selects `EXPRESSION_TOO_COMPLEX`, and a `STATUS_DOMAIN_ERROR` selects `DOMAIN_ERROR`; with no Tier S token outstanding, the outcome is `SUCCESS` — outstanding Tier O tokens carry no semantic verdict (the committed tier definition) and never alter the outcome. (3) *Itemization at every exit*: every outstanding token terminalizes and appears in the terminal receipt with its tier, type, birth site, and disposition — unresolved semantic debt is surfaced at every exit, never discarded (the committed family-9 scope boundary), and the receipt records the selected token's birth site for a `NO_SOLUTION` or `DOMAIN_ERROR` exit. Exactly one outcome is produced at every exit, and every tie-break is declared: determinism by construction, not by silence. **The declared initial status pool.** The initial configuration of §2.1 carries the declared initial status pool: empty unless the Intent declares entries, at most one token per declared type, each entry a declared Tier S token with birth site (0, 0) — the conformance birth path that makes every terminalization combination of the §4.4 vectors buildable inside the frozen core (F12's "vectors for every combination"), with no extension admitted and no birth rule invented. **The frame parameter** is a derived execution parameter, not a configuration component: `real` for every configuration whose committed record prefix contains no `PROMOTE` transition, `wider` afterwards — determinism and executability follow from the record, and §2.2 stands as committed; the wider-frame rule's in-core content is exactly the `PROMOTE` semantics of §2.3, and the extended value semantics of the wider frame is the Step-6 extension surface, not pre-empted here. The named wrong mappings of Obj 5 stay named and excluded — a lookup miss is not an epistemic defect, a negative even root is not complexity evidence, and no token-consuming behavior is left implicit.

**The Terminal Outcome Lift Table (`Φ_status`, v0.3.0-draft.8.4).** QnIR declares four terminal outcomes (above) plus the artifact-comparison verdict surface; the Axiom 6 Sealed 12-Status Registry is the outer reporting vocabulary. `Φ_status` is the total projection between them: it is applied at the reporting boundary from the terminal receipt's outcome, its selected Tier-S token, and its itemized admission cause, and it never rewrites an execution, adds a fifth QnIR outcome, or changes a transition record. Where one QnIR outcome has several outer images, the cause decides — an `EXPRESSION_TOO_COMPLEX` receipt carries exactly one itemized cause, and each cause has one image:


The projection is total on QnIR's domain, not surjective: `FAILURE`, `TIMEOUT`, `INCONCLUSIVE`, `PRECISION_INSUFFICIENT`, and `ORDERING_INDETERMINATE_AT_ZOOM` are outer-layer or extension statuses and are not images of a core QnIR execution. The comparator's `collision` verdict is not an execution outcome at all; it is lifted because quarantine is the registry's declared disposition for it, and its commitment-side tuple is fixed at §6.7.

### 2.6 Deterministic Encodings and the Transition Record (`QO-30` / F5 / F6, Step-2 Share; the versioned byte grammar — Loop-3 R2, F11/F15 repaired, [v0.3.0-draft.6 — DRAFT])

**Words and values.** The 64-bit word is preserved as the transport unit (`QO-30`), and the reconciliation of §2.7 fixes what that preserves: the word is a transport and commitment unit, not a value-semantics width. An exact value encodes as its declared type tag, a word-length prefix, and the minimal big-endian sequence of 64-bit magnitude words under a declared sign convention — no leading zero words, one canonical zero — so each value has exactly one encoding. Per-instruction attributes that are not uniform across the program ride the `EXT`-flag pattern: the flag indexes a canonical header extension record in the side table carried by Intent (`QO-30`), and an unknown flag combination or a dangling side-table reference makes the Intent statically illegal (§2.1); the validator that enforces this mechanically is Step 4's.

**The transition record.** Every transition commits the F5 field list, preserved verbatim from the family-7 row of §1.2: prev root, program root, step index, PC, decoded word or its hash, input/output IDs, pre/post state roots, work charged/refunded, clock, status, {% include term.html id="receipt" text="receipt" %} IDs, semantics/profile versions. The record is encoded deterministically and domain-separated — each field under a declared domain tag, in the declared field order, under the deterministic-encoding discipline of CBOR (RFC 8949), which is the review's own encoding reference [4] — so that equal records are byte-equal records. Pre/post state roots presuppose a canonical configuration encoding: the value store encodes in `(static value id, instance id)` order, the token pool in the full token-identifier order of the grammar block below (F15, v0.3.0-draft.6), the receipt queue in emission order, every component through the value encoding above. The state-root function is named by the encoding version — the concrete hash is an encoding-version parameter whose implementation artifacts belong to the Step-4 oracle — and the `qir-enc-2` registration (v0.3.0-draft.32, F-G0-1) makes it concrete: the state root is the SHA-256, exactly 32 bytes, of the canonical configuration encoding of P2 — the corpus's committed roots become these digests (the P2 pre-image rides in-memory, never in place of the digest). Its determinism follows from the canonical encoding, which is what F5's discharge needs at this step: equal canonical record bytes denote equal transitions by construction, single-transition records here, full-trace roots in Step 5, and the commitment and equality policy is declared in the grammar block below (F15, v0.3.0-draft.6).

**Comparison and collision surface.** Tuple equality is full-canonical-encoding equality (F6) — never header equality, never hash-as-identity. The collision bucket of complete artifacts and the quarantine-on-genuine-collision discipline are Step 5's artifact-comparison commitments (per the F6 row of §1.4); what Step 2 freezes is the equality rule the bucket will compare under.

**The versioned byte grammar (Loop-3 R2 — F11 repaired, [v0.3.0-draft.6 — DRAFT]).** The Loop-3 finding F11 is accepted: the paragraphs above fix the value discipline and the record's field list, but no complete encoding of an instruction, block, Intent, dependency environment, profile, or transition record existed, and the assembler of §4.2 could not be written from them. This block publishes the complete grammar; it binds under the encoding version of `V` (§2.1), and a change to any part of it is a new encoding version and a different `Eval`. Every unit below is a sequence of 64-bit big-endian unsigned words; where an artifact is carried in CBOR, its byte content is the deterministic CBOR (RFC 8949) encoding of the declared structure, and the two layers are never mixed.

**The two-layer boundary.** Deterministic CBOR encodes the outer artifacts: the Intent envelope, each transition record, the terminal receipt, and the conformance vectors of §4.4. The fixed 64-bit word encodes the instruction stream and every value inside it. An Intent's envelope is a deterministic CBOR map whose instruction-stream entry is a byte string whose length is a multiple of 8, carrying the word sequence declared below; no instruction word is ever read as CBOR, and no envelope field is ever read as an instruction word.

**The closed tag assignments.** Opcode tags — one word each, assigned in the inventory order of §2.3: `CONST` 1, `CLOAD` 2, `TUPLE` 3, `FIELD` 4, `ADD` 5, `SUB` 6, `MUL` 7, `DIV` 8, `MOD` 9, `CMP` 10, `BR` 11, `JMP` 12, `OBS` 13, `PROMOTE` 14, `EMIT` 15, `HALT` 16; the word 0 is no opcode, and an opcode tag outside 1–16 is malformed. Type tags — one word each, over the declared value forms: `INT_POS` 1 (a non-negative exact integer), `INT_NEG` 2 (a negative exact integer, encoding its absolute magnitude), `TUPLE` 3, `ZOOM` 4; no other type tag exists. The sign convention is hereby pinned: the sign rides the type tag, the magnitude words carry the absolute value, and a negative-tagged zero is malformed. Condition codes — one word each, in the declared order of §2.3's grammar: `LT` 0, `LE` 1, `EQ` 2, `NE` 3, `GT` 4, `GE` 5. Token type tags — one word each, in the declared order of §2.5's closed vocabulary: `STATUS_LOOKUP_MISS` 1, `STATUS_NEG_EVEN_ROOT` 2, `EXPRESSION_TOO_COMPLEX` 3, `STATUS_DOMAIN_ERROR` 4. Terminal outcome tags — one word each, in the declared order of §2.5's closed terminal vocabulary: `SUCCESS` 1, `NO_SOLUTION` 2, `EXPRESSION_TOO_COMPLEX` 3, `DOMAIN_ERROR` 4.

**The value grammar.** A value encodes as: the type-tag word; then, for `INT_POS` or `INT_NEG`, one count word (the number of magnitude words, possibly 0) followed by exactly that many magnitude words, minimal big-endian with no leading zero word — the canonical zero is `INT_POS` with count 0; for `TUPLE`, one field-count word k followed by k field encodings in the signature's declared order, each a complete value encoding; for `ZOOM`, the mantissa value encoding followed by the exponent value encoding, each an `INT_POS`/`INT_NEG` encoding, in the normalized form of §2.3's N — a non-normalized zoom encoding is malformed. Structural bounds, and none semantic: a count must not exceed the remaining length of its enclosing artifact; every encoding is finite; no capacity constant is introduced anywhere (Obj 2). Worked encodings: zero = `0000000000000001 0000000000000000`; −1 = `0000000000000002 0000000000000001 0000000000000001`; 2^64 = `0000000000000001 0000000000000002 0000000000000001 0000000000000000`; the pair (1, −2) as a tuple = `0000000000000003 0000000000000002 0000000000000001 0000000000000001 0000000000000001 0000000000000002 0000000000000001 0000000000000002`.

**The instruction grammar.** An instruction is the opcode tag word, then its declared operand fields in declared order, then one uniform trailing `ext` word — the bitmask whose bit i set means the instruction carries side-table record i of the Intent's `EXT` table (the committed `EXT`-flag pattern's concrete home). The operand kinds are closed: `ref` (one word, a static value id), `blk` (one word, a block id), `addr` (one word, an index into the Intent's declared dependency list), `idx` (one word, a field index), `cc` (one word, a condition code), `ttype` (one word, a token type tag), `cnt` (one word, a declared count), `imm` (a complete value encoding — the literal), `bnd` (a result declaration: the type-tag word followed by one bound value encoding, an `INT_POS` upper bound), `rng` (two bound value encodings, lo then hi), `edge` (a `blk` word, an arity `cnt` word, then exactly arity `ref` words). The per-opcode declared sequences: `CONST` = `imm`, `bnd`; `CLOAD` = `addr`, `blk` (the declared fallback target), `bnd`; `TUPLE` = `cnt` (k), then k `bnd` field declarations in field order, then k `ref` component references in field order; `FIELD` = `ref`, `idx`; `ADD`, `SUB`, `MUL`, `DIV`, `MOD` = `ref`, `ref`, `bnd`; `CMP` = `ref`, `ref`, `rng`; `BR` = `cc`, `ref`, `edge` (true), `edge` (false); `JMP` = `edge`; `OBS` = `ttype`, `rng`; `PROMOTE` = (no operands); `EMIT` = `cnt` (the declared kind immediate), `cnt` (k), then k `ref` words; `HALT` = (no operands). An edge's inline arity must equal its target block's declared argument arity (the committed arity-closure rule, now concrete); the source grammar's textual form is the same sixteen mnemonics with the same operands in the same order, and the assembler's mapping between the two is one-to-one and deterministic. An unknown opcode tag, an operand that runs past its enclosing artifact, or a trailing partial instruction is malformed.

**Blocks, edges, and the `EXT` side table.** Block ids are dense from 0, and block 0 is the entry block. Each block is declared as: its id, its argument signature (an arity `cnt` word followed by exactly arity `bnd` declarations), its instruction `cnt`, and its instructions in order. An edge is identified by `(source block id, instruction index within the block, edge slot)`, slots declared as: 0 = the `JMP` edge, the `BR` false edge, and the `CLOAD` fallback edge; 1 = the `BR` true edge. A back-edge is declared by the graph itself (its target dominates its source); the declared trip ceiling rides the profile (committed), and the derived counters read the record sequence (§2.3). The Intent envelope is a deterministic CBOR map with the closed key assignment: 0 = the envelope domain tag (the byte string `QnIR-Intent`), 1 = the semantics version (text), 2 = the encoding version (text), 3 = the profile version (text), 4 = the dependency list (an array of content-address byte strings; `addr` indexes it), 5 = the block count, 6 = the instruction stream (a byte string, length a multiple of 8), 7 = the `EXT` side table (an array of records, each a two-element array of the record index word and the payload byte string, whose reading is the record's own declared type under the encoding version), 8 = the declared initial status pool (an array of token type tag words, at most one per type — a duplicate type in the declaration is statically illegal). An envelope carrying a key outside 0–8 is malformed — unknown-field rejection is total over every structure of this grammar: an unknown envelope key, an unknown opcode tag, an unknown type tag, an unknown condition code, an unknown token type tag, an unknown `EXT` index, each is malformed or statically illegal exactly as the committed rules of §2.1 and this section declare.

**Canonical maps, identifiers, and duplicate rules (F15 repaired).** The value store's canonical order is lexicographic over `(static value id, instance id)`; static value ids are dense words over the Intent, and instance ids are minted in transition order from 0 and never reused (§2.4), so the key is total; a duplicate key in any encoded artifact is malformed — redefinition mints, never overwrites. The receipt queue's canonical order is the emission order; emission ordinals are dense from 0 and unique, and a duplicate ordinal is malformed. Every token carries the **full token identifier** `(birth step index, block id, instruction index, token serial, declared type id)`: the birth site's position is the `(block id, instruction index)` pair of the birthing transition's program position — the declared initial pool's site (0, 0) is the pair (block 0, index 0) — and the **token serial** is the execution's monotone birth counter, allocated first to the declared initial pool in its declared order, then one per birthing transition; the serial is unique per execution by construction, so the identifier is total, and the token pool's canonical order is lexicographic over the full identifier — the withdrawn `(birth step index, type id)` key is superseded (F15). A duplicate token identifier in any encoded artifact is malformed. The profile and version encodings are the text strings of `V` under envelope keys 1–: the versions ride every record and receipt as declared, and the dependency environment `D` is cited by content address exactly as §2.1 commits.


### 2.7 The Fixed-Width/Exact Reconciliation (Named Obligation Discharged)

The tension is recorded in §1.4 and is resolved here by declaration, as the register requires: `docs/abstractionLayers.md` §6.3 and §13 describe the QnIR execution handler as "a small, fixed-width deterministic machine over integer arithmetic, with bounded loops and no floating point," while the frozen core names *exact arithmetic*. The declared stance of this semantics version is **exact value semantics over a fixed-width transport**:

1. **Values are exact.** Integer values are exact integers of unbounded range at the semantic layer; no operation wraps, saturates, or truncates, and no value is ever silently mapped (Obj 5).
2. **The transport is fixed-width.** The 64-bit word is the unit of encoding, dispatch, and commitment (`QO-30` preserved); an exact value occupies a minimal-length word sequence (§2.6), so the machine is exactly the "small, fixed-width deterministic machine" of §6.3 — fixed in its word format, not in a value-width limit.
3. **Finiteness enters only by declaration.** Every value carries a declared type bound, and bounds are profile parameters, never core validity limits (Obj 2; `QO-18`); there is no universal capacity constant anywhere in the relation.
4. **Exceedance is typed, before dispatch.** A value exceeding its declared bound is rejected before dispatch as a typed failure (§2.3, §2.5) — surfaced as `EXPRESSION_TOO_COMPLEX`, traced in the transition record, never absorbed.

The alternative stance — fixed-width 64-bit value semantics with typed overflow — was rejected on the register's own grounds: it would make a 64-bit capacity constant a core validity limit, which Obj 2 forbids in the absence of evidence, whereas the exact-over-transport stance requires no capacity constant at all. The choice is thereby forced by the constraints register rather than preferred by taste, which is the only form of resolution this program admits.

**Recorded reading note (no cross-file mutation).** Under the declared stance, §6.3's "fixed-width" is read as the transport and word-format width, not a value-semantics width. `docs/abstractionLayers.md` is not edited by this draft (one file at a time; the §13 open item stays *in progress* through this file); if the portal later aligns §6.3's wording, that is a future bookkeeping wave. This note is the record of the reading: the reconciliation is discharged by declaration here, together with the Obj 2 overflow stance and `QO-30` word-format preservation, exactly as §1.4 required — not by silent rewording of a canon file.

### 2.8 Step-2 Acceptance Condition and Non-Discharge Register

The acceptance condition closes on seven deliverables, each discharged at a named subsection: typed configurations (§2.2), small-step transitions (§2.3), block arguments and dynamic instances (§2.4), token lifecycle (§2.5), failure behavior (§2.5), deterministic encodings (§2.6), and the determinism statement `Eval(Intent, D, V) = Execution` with all semantic versions explicit (§2.1). The register of §1.4 is consumed, not amended: F3 is discharged by §2.4 and the control row of §2.3; Obj 5 and `QO-22` by §2.5; the F5 field list and encoding by §2.6 (trace roots remaining for Step 5); F6's equality rule by §2.6 (the collision bucket remaining for Step 5); `QO-30` by §2.6; Obj 2's profile-parameter surface by §2.1–§2.3; the named fixed-width/exact reconciliation by §2.7; and F8's typed-semantics substrate by §2.1–§2.4, on which the `R11'` simulation demonstration will be built without ever styling it a theorem.

What this section deliberately does not deliver: **no ledger equation, no reserve/charge/refund invariants, no budget-admission rule** (Step 3 — F1, F2, F4; the work accumulators exist as state, their algebra does not); **no assembler, validator, interpreter, second implementation, or conformance vector** (Step 4 — F7, the `QO-22` test list; the static legality rules of §2.1–§2.3 are the specification those artifacts will implement); **no trace roots, no artifact-lineage commitment, no collision bucket, no verbalizer or extractor identity binding** (Step 5 — the F6 bucket, `QO-24` lineage, `QO-27`, Obj 15); **no extension admitted or specified** (Step 6, per the deferral register of §1.3; the extension boundary of §2.3 is a closure statement, not an extension). No opcode cost is declared anywhere in this section; no capacity constant is a validity limit of any rule; the promotion criterion is untouched — "Candidate 1" is not styled a theorem, `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, the draft remains a Working Draft, and QnIR remains **[PENDING]** in the portal milestone. Under {% include term.html id="computational-consistency" text="computational consistency" %}, the validity claimed for this section is internal consistency within declared finite boundaries: the dynamics are consistent with the frozen inventory of §1.2, every declared choice cites its binding register row, and no canon file is contradicted or mutated.

---
[[/CODE]]

### II.11 QnIR Work Accounting §3 — `qnir_accounting.md`

[[CODE]]
# QnIR Work Accounting — The Ledger, Bounds, and Budget Admission of the QnIR v0.3 Working Draft (Repair Step 3)

This file carries §3 (§3.1–§3.6) of the QnIR v0.3 working draft, split content-identically from the committed v0.3.0-draft.5 monolith with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. The §3 body below is committed at v0.3.0-draft.7 (Loop-3 stage R3), and the front-matter `terms_used` lists exactly the distinct `term.html` include ids of this body under the include-parity discipline.

**Revision record (v0.3.0-draft.8.3 — Wave 1A specification alignment, 2026-11-18).** This additive amendment aligns §3.1 and §3.4 with `docs/simemp.md` §6's declared gateway fee `W_gate > 0` without adding a second ledger: the outer gateway's deduction is separated from QnIR's inner ledger (the Two-Tier Boundary), Conservation Identity 3 is restated in total indicator form, and §3.4's surrogate is refined to the syntactic/semantic split `B_hat(P) = B_cfg(P) ≥ B_sem(P)` with the mandatory pre-DP state-cardinality gate and the `val(...) == None` refusal rule. No opcode cost, no committed vector, no encoding byte, and no register of the committed increments changes; the corpus remains byte-unchanged and both implementations continue to reproduce it. The amendments are normative only where this file is the owning specification (§3); the `W_gate` boundary inherits `docs/simemp.md` §6 as fixed input and amends no rule there.

---

## 3. Work Accounting (Repair Step 3)

This section discharges the third step of the repair sequence on the byte-unchanged freeze of §1 and the committed semantics of §2. The operative instruction is the Step-3 row of the sequence, preserved verbatim: action — "Repair work accounting"; acceptance condition — "One ledger equation; static operand bounds; sound control-flow budget admission; reserve/charge/refund invariants; no double billing." The inheritance register that headed this step in the forward map is fixed input and is preserved here verbatim so that the map of §5 can retire the row without losing it: "The F1 static-bounds discipline; the F2 single-ledger shape; the F4 bounded-path admission rule; the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} as work unit under the declared envelopes." Each register item is consumed at a named subsection (§3.2, §3.1, §3.4, §3.1–§3.5), and none is re-adjudicated. The accounting is a declared discipline over the transition relation of §2.3 — the control row of that table already names this section as the owner of the F4 admission rule — and it amends no rule, no encoding, and no register of the committed increments; under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline, every movement of work below is a declared quantity, and nothing is metered by silence, convention, or implementation accident.

### 3.1 The Ledger Equation (F2 Discharged)

An execution's transitions divide into **completed transitions**, which take their declared effects and post their declared charge, and **rejection transitions**, which post nothing, birth the fatal token, and are final (§2.5); the ledger bills the completed ones. The ledger of an execution `E` with completed transitions `t = 1 … n` is the single equation the acceptance condition requires:

> **`Work(E) = Σ_t charge(t)`** — the sum, over every completed transition of the execution, of that transition's declared charge.

Every quantity in the equation is measured in the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} under the declared envelopes of the {% include term.html id="simemp" text="SIMEMP" %} constraints, exactly as §1.4 and §2.2 fix the unit. **The charge of a transition is one declared quantity, and it is total for that transition.** Each rule of the §2.3 table declares the charge of its firing as a single function (§3.2) that already includes the base dispatch quantity — the `c_base + C_op` unification the F2 row of §1.4 requires, stated over families rather than opcodes: there is no separate dispatch deduction anywhere in the core, so the v0.2 defect that billed the base inside both a stage-1 deduction and a stage-2 reservation is excluded by construction. The equation's charge domain is closed over the four charge classes the F4 row names: the **gateway charge** — the declared fixed admission quantity `g(Intent)`, posted by the execution's first completed transition ahead of that transition's own rule charge; the **branch charge** — the declared charges of the control-family rules, in the core; and the **frame charge** and **proof-verification charge** — named in scope here, specified with their owning extensions (calls, Step 6; witness realizations, Steps 5–6) before any retention, riding this same equation and these same verbs without redefinition. The first completed transition's charged field therefore carries `g(Intent)` plus its own rule charge as one posted quantity; the itemization of the components is the terminal receipt's (§3.4), not a second field.

**The Two-Tier Boundary — kernel accounting split (v0.3.0-draft.8.3).** `docs/simemp.md` §6 declares the outer gateway's fee `W_gate > 0`, deducted from the execution's principal before the Intent is inspected; the gateway then passes `W_budget = B_principal − W_gate` into QnIR. QnIR's ledger is the **inner** ledger: it opens against `W_budget`, never against `B_principal`, and the gate is not a QnIR movement, so no second equation is introduced and no charge is posted twice. Across the boundary the principal's change is exactly **`ΔB_principal = W_gate + Work(E) ≥ W_gate > 0`** — the outer fee plus the inner work, with the fee charged unconditionally and the work only as the inner ledger posts it. `g(Intent)` is the inner VM's escrow-opening surcharge, folded into the first completed transition above; it is not the outer gate and does not replace it, and an execution that refuses after the gate pays `W_gate` with `Work(E) = 0 U`. The zero-posting vectors `X2` (admission refusal) and `X50` (escrow-never-opened rejection at the first attempted transition) close the inner ledger at `Work(E) = 0 U`; their settlement posts no inner refund, and the outer gate's fee is outside their ledger by construction — the boundary is the reason both statements hold at once.

The ledger is a derived record, not new state: it is reconstructed from artifacts that §2 already commits — the transition records' charged and refunded fields (§2.6) in execution order, plus the terminal receipt's itemization — so equal executions have byte-equal ledgers, and the differential execution of Step 4 can check a ledger against its records mechanically. The per-class ledger traces that the F2 row of §1.4 requires to be published alongside the equation are conformance artifacts: the equation is fixed here so that the Step-4 oracle's one-trace-per-rule-class publications are mechanically checkable against it; the shape is a Step-3 deliverable, the publication a Step-4 one, and the distinction is recorded in §3.6 rather than left silent.

### 3.2 Static Operand Bounds as the Cost Domain (F1 Discharged)

The F1 row of §1.4 binds the core to cost parameters that are "declared immediates, header bounds, or static type bounds" and forbids any runtime quantity — `h1`, `h2`, `B`, `M`, `K` — from a core cost formula; its discharge step reads "Step 3 (with Step 2 typing)", because §2 already froze the typing: every value carries a declared type with a declared bound (§2.1), bounds are profile parameters (§2.7), and exceedance is rejected before dispatch (§2.3, §2.5). This section completes the discharge by fixing the **cost domain**: the charge function of every rule takes its arguments from declared immediates, declared header fields, and the declared static type bounds of the instruction's operands — all fixed at Intent formation — and from nothing else.

Three consequences are stated because they are the discipline, not decoration. First, no runtime quantity is metered: the actual encoded length of a runtime value, its magnitude, its representation, and its position in the store never enter a charge function; the dense/sparse load-time status that F1 named as representation-dependent has no core analogue at all, because the core has exactly one canonical value encoding (§2.6) and one cost per rule. Second, the declared charge is the transition's billed quantity, and what it bounds by construction is declared: an actual value occupies at most its declared bound of transport words (§2.6, §2.7), so the transport component of the charge is computed over declared bounds — while the charge as a whole is a declared logical metering quantity under the Abstract Compute Unit U, not a claimed measure of the transition's implementation work: operand-transport bounds do not bound the state-dependent work of an implementation (state-root computation, record encoding, dependency lookup, receipt construction), so domination of implementation work is not claimed from this text — it is exactly what Layer-0 calibration must establish, `QO-17`'s chain, conditional — and until calibrated the budget gates the declared charge, the only quantity every gate of §3.3–§3.4 decrements (F14, v0.3.0-draft.7). Third, charges are site-stable: the operands' declared bounds are per-static-instruction data, so one static instruction charges the same quantity in every instance and every iteration — dynamic instances (§2.4) change the values, never the cost. Every declared charge is positive — at minimum the declared base quantity, itself a profile parameter — which is what makes the fuel arithmetic of §3.4 strictly decreasing and the termination certification of §3.5 possible. Charge coefficients are profile parameters riding the profile version of `V`; the equation and the cost domain are core, the numbers are profile data, and `QO-23`'s governance clause over coefficient ranges lands with those declarations, not in this file.

### 3.3 Reserve, Charge, and Refund as Typed Ledger Movements

§2.2 declared the three work accumulators — charged, reserved, and refunded to date, as plain accumulators — and deferred their algebra to this section. The algebra is the settlement discipline of the review's own gateway schedule (`QO-23`): *reserve* the declared envelope, *charge* the metered work, *refund* the residue — declared here as movements of typed state, each with one declared site, no silent ordering.

**Reserve.** A reserve is an authorization against the declared budget. The execution's reserve is opened once, by the execution's first completed transition, which checks the declared admission bound `B(P)` of §3.4 against the budget and reserves it as an outstanding escrow. A rule may also declare a transition-local reserve `r(t) ≥ charge(t)` — its declared worst-case envelope for that firing — checked against the outstanding escrow before any effect posts. **Charge.** A charge posts against the outstanding escrow: the gateway charge and each completed transition's declared charge reduce the escrow and raise the charged accumulator. **Refund.** A refund releases unspent reserve: a transition-local reserve settles within its own transition (`refund(t) = r(t) − charge(t)`), and the escrow's outstanding remainder — the residue — is settled once, by the execution's final transition, halt or fatal rejection alike, whatever that transition's own posting status: pricing is outcome-independent by declaration, per `QO-23`'s own rule that payment must not depend on whether validation succeeds.

The invariants are total, and they are the acceptance condition's "reserve/charge/refund invariants": every reserve is non-negative and checked before it is honored; every charge satisfies `0 ≤ charge(t) ≤ r(t)` against the escrow that covers it; every refund satisfies `refund ≤ reserve outstanding`; the escrow never goes negative, the conservation identities below hold at every settlement boundary, and the remaining budget — the declared budget minus charged — never goes negative; a reserve is settled either within its own transition or by the declared terminal settlement, so no reserve outlives the execution; and every movement posts into the transition record of its own transition (§2.6), whose pre/post state roots commit the accumulators (§2.2). The core's instantiation is deliberately degenerate and is stated rather than hidden: every core rule's charge is site-stable (§3.2), so every transition-local reserve equals its charge and every transition-local refund is zero — the non-trivial reserve/refund cycle of the core is the admission escrow and its terminal residue. The general shape is declared now so that the extension charge classes of §3.1 — frame escrows across calls, verification fees — inherit these exact verbs at their owning steps instead of redefining them. No family's transition semantics reads an accumulator: the accounting gates firing (§3.4) and is committed (§2.6), but the nine families' rules are unchanged — the ledger observes and gates the execution; it does not steer it.

**The conservation identities (F13 repaired, [v0.3.0-draft.7 — DRAFT]).** The withdrawn budget-balance form — the declared budget minus charged plus refunded — double-counts refunds: a refund releases escrow; it does not create budget. The settlement discipline is the four conservation identities, declared here as the accounting's conservation law, each a declared function of the accumulators of §2.2 and holding at every settlement boundary:

> **`reserved = charged + refunded + reserve_outstanding`** — escrow conservation: what was reserved is exactly what has been charged, refunded, or remains outstanding; **`remaining_budget = W_budget − charged`** — the spendable remainder never counts a refund, because a refund returns outstanding reserve to the reserve pool, never budget; **`refund_final = I_opened * B(P) − charged`** in total indicator form, with `I_opened ∈ {0, 1}` the escrow-opened indicator: `I_opened = 1` when the whole admission bound was reserved and no other reserve remains — the core's own instantiation, the escrow opened once at `B(P)` (§3.3) — and `I_opened = 0` when the execution closes before any escrow opens, so the identity totalizes over both settlement shapes (a refused or never-opened execution has `charged = 0` and `refund_final = 0`, never a negative refund); and **`reserve_outstanding = 0` after terminal settlement** — no reserve outlives the execution.

The identities hold for every reserve instance — the admission escrow and, in the general shape the extensions inherit, each transition-local reserve — and therefore in sum over the execution; a refund of any kind releases reserve, never budget. The charged accumulator is defined once and identically everywhere it appears: it is the sum of the posted charges of all completed transitions, the gateway charge included in the first completed transition's posting (§3.1), and the terminal receipt's itemization (§3.4), the transition records' work fields (§2.6, work charged 9 / work refunded 10), and the reconstructed ledger (§3.1) all carry that same quantity under that same definition. The finding's settlement is the accounting's own worked numerical vector: a declared budget `W_budget = 100`, the admission bound `B(P) = 100` reserved in full (`I_opened = 1`), completed-transition charges totaling `charged = 60`, and terminal settlement paying `refund_final = 100 − 60 = 40` — the identities give `remaining_budget = 100 − 60 = 40` and `reserved = 60 + 40 + 0 = 100`, while the withdrawn form would have read `100 − 60 + 40 = 80`, the double-count F13 names; the conservation-vector family built on this settlement is registered at §4.4 (F13, v0.3.0-draft.7).

### 3.4 Sound Control-Flow Budget Admission (F4 Discharged)

The F4 row of §1.4 requires admission to use "a maximum over bounded control-flow paths with declared trip/depth ceilings, or a dynamic fuel bound", excludes the claim that a simple per-instruction sum certifies admission, and requires gateway, frame, branch, and proof-verification charges in the ledger's scope. This section takes both arms of the review's "or", because each covers exactly the gap the other leaves: the static bound certifies, the fuel bound enforces.

**The admission bound.** For a well-formed Intent `P`, the declared admission bound is `B(P) = g(P) + max_π Σ charge(i) for i on π` — the gateway charge plus the maximum, over every control-flow path `π` from the entry block to an exit that respects the declared trip ceilings on back-edges, of the summed site-stable charges (§3.2) of the transitions the path executes. The boundedness of the paths is the same declared boundedness the core enforces at runtime: a loop back-edge cannot exceed its declared trip ceiling without the typed rejection §2.5 already declares, so every executable path is a bounded path — which is the soundness direction admission needs. **The admission rule.** An Intent is admitted against the declared budget `W_budget` exactly when `W_budget ≥ B(P)`; the check's arguments are static — the budget, the ceilings, the bounds, and the charges are all declared quantities riding the Intent and its profile — and the check executes at the execution's first attempted transition, before any effect posts; on a pass, the transition completes normally and opens the escrow of §3.3. **The refusal.** A failed admission is a typed rejection in the sense of §2.5 — pre-dispatch, typed, sited — and its birth is `EXPRESSION_TOO_COMPLEX`, the resource/representation-limit token that section already declares: the declared budget is a declared resource envelope exactly as a declared bound is, and exhausting it is the same class of event. The refusal execution is the single rejection transition, the ledger closes at zero postings, and the terminal receipt itemizes the denial — declared budget against admission bound — under the itemization discipline of `QO-23`; whether a gateway profile prices refused admissions at all is a fee-policy parameter under that same clause, not a core rule.

**The fuel bound.** After admission, the outstanding escrow of §3.3 enforces the budget dynamically: a transition fires only if its declared reserve is covered, and the fuel arithmetic is strictly decreasing (every completed transition bills at least the declared base, §3.2), so the machine stops — fatally, with the same token, itemized — rather than over-burning silently. For an exactly-admitted Intent the backstop is provably unfirable: the actual path is one of the bounded paths over which the maximum was taken, so the actual total never exceeds `B(P) ≤ W_budget`. It is declared anyway, and its role is declared with it: defense in depth. The validator's surrogate `B_hat(P)` is specified below with the obligation `B_hat(P) ≥ B(P)` — false rejection is permitted, and a false admission is a validator defect even when this backstop later stops the execution; the backstop limits the damage of such a defect, it never legitimizes the admission (F16, v0.3.0-draft.7). **What is claimed and what is not.** The specification defines `B(P)`, and — as of this version (F16, v0.3.0-draft.7) — one sound, terminating surrogate for computing it, declared immediately below. What remains unclaimed is the polynomial-time question: the review's `QO-17` places it in its calibration chain, which orders "Correct F1–F4 before calibration", and it stays open and conditional — the surrogate's adversarial tests (the "budget boundaries" of the map's Step-4 row) are registered at §4.4, and their execution is an artifact-share deliverable. **The declared surrogate `B_hat(P)` (F16 repaired, [v0.3.0-draft.7 — DRAFT]).** The validator's computed surrogate is `B_hat(P)`, and the requirement on it is directional and total: `B_hat(P) ≥ B(P)` for every well-formed Intent, carried as a proof or a checked argument of the surrogate's specification; false rejection is permitted, and a false admission is a validator defect even when the fuel backstop later stops the execution — the backstop is defense in depth, never a license (§4.2 carries the validator-side requirement). One sound, terminating surrogate is declared: **the trip-ceiling unrolling dynamic program**. Its state space is the pair `(block id, remaining trip budget)`, where the remaining trip budget is the vector of remaining allowed traversals, one component per declared back-edge, initialized to that edge's declared trip ceiling; every block-graph edge is a transition to its target's state, a back-edge traversal decrements its own component, and an edge whose traversal would exceed a component's ceiling is not a transition. Every bounded path of `B(P)`'s family — entry to exit, trip ceilings respected — is a path of this state graph with equal charge, and every state-graph path projects to such a bounded path: the state graph *is* the bounded-path family. The depth ceiling of earlier drafts is retired (F26, v0.3.0-draft.22 — the review records that trip ceilings alone bound execution; no operational measure of nesting depth existed), so no slack remains between the two families: **`B_hat(P) = B_cfg(P)` exactly for every well-formed Intent**, where `B_cfg(P)` is the maximum over the trip-bounded syntactic CFG assuming non-rejecting block completion, and the declared one-sided obligation holds as **`B_hat(P) = B_cfg(P) ≥ B_sem(P)`** — a conservative upper bound over the semantically feasible executions `B_sem(P)`, those whose blocks complete rather than terminating in a typed rejection. Equality holds in particular when every syntactically bounded path is semantically feasible; where a rejection truncates a path, `B_cfg(P) ≥ B_sem(P)`, with the excess being the declared price of static soundness. The runtime enforcement is the trip ceilings exactly as §2.3 and §2.5 declare. `B_hat(P) = g(P) +` the maximum, over the state graph's paths from the entry state to an exit block, of the summed site-stable charges (§3.2), computed by dynamic program in the graph's topological order. **Soundness:** the state graph's path family equals the trip-bounded syntactic family of `B_cfg(P)`, so `B_hat(P) = B_cfg(P)` by construction, and `B_cfg(P) ≥ B_sem(P)` because every semantically feasible execution is a syntactically bounded path — the declared one-sided obligation holds. **Termination and complexity:** every block-graph cycle contains a back-edge (a back-edge's target dominates its source, §2.6), so the state graph is a finite directed acyclic graph over `|B| Π_i (ceil_i + 1)` states — `|B|` the block count, `ceil_i` the declared trip ceiling of back-edge `i` — evaluated in `O(|B| Π_i (ceil_i + 1) · d)` time and the same space, `d` the maximum block out-degree: terminating, with its complexity bound declared; the bound is exponential in the number of distinct back-edges, which is the honest price of soundness without structural assumptions, and the polynomial-time question stays where `QO-17`'s chain put it. No structural assumption (reducibility, single-entry loops) is imported, no solver is invoked, and no new capacity constant is introduced — the ceilings are the profile's declared parameters (§2.6), so Step 3's discharge now carries the sound, terminating surrogate its acceptance requires.

**The pre-DP structural gate and the no-exit refusal (v0.3.0-draft.8.3).** Before the dynamic program allocates its state table, the validator MUST evaluate the structural state-cardinality product `|B| * Π_i (ceil_i + 1)` in overflow-checked arithmetic and refuse admission unless `|B| * Π_i (ceil_i + 1) ≤ S_max`, where `S_max = 4096` is the state-cardinality ceiling declared in the golden profile `qir-prof-golden-2` (`qnir_semantics.md` §2.1) — a profile-declared parameter, not a new core capacity constant. The gate is structural and pre-dispatch: it bounds the table before the table exists, so a profile whose declared ceilings would exceed `S_max` is refused rather than truncated, saturated, or allocated. If `val(...) == None` — the state graph yields no ceiling-bounded exit path from the entry state, including the irreducible-cycle case where no exit is reachable under the declared trip ceilings — the validator MUST refuse at admission with `EXPRESSION_TOO_COMPLEX`. Defaulting such an input to the gateway charge alone is a validator defect: `None` is the absence of a certified bound, not a bound of `g(P)`, and admitting it would be a false admission (§4.2).

**The termination certification.** §2.1 made no universal termination claim and named the question as this step's deliverable; the answer is now declared: every execution of a well-formed triple is finite. A program either completes transitions — each strictly consuming fuel against a finite budget — or is stopped by a typed rejection, including the fuel backstop; halting, fatal rejection, and fuel exhaustion are the only exits, and each is reached in finitely many transitions. This is a resource-bounded termination statement derived from the declared charges, not a halt-correctness claim, and it discharges exactly the certification §2.1 deferred.

### 3.5 Soundness of the Accounting

The accounting's soundness is one argument with four declared faces, each tied to a register item rather than to taste.

**Never under-billed — scoped to the declared charge (F14, v0.3.0-draft.7).** The ledger bills exactly the declared charge: every completed transition posts its declared charge, and `Work(E)` is the execution's exact total declared charge in the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %} under the declared envelopes of the {% include term.html id="simemp" text="SIMEMP" %} constraints — a policy charge over the declared metering, not a measure of all attempted physical work, and a rejected instruction posts zero by declaration (§3.1). What the accounting withdraws is the domination claim the draft previously made here: a declared operand-transport bound does not bound the state-dependent work of an implementation (state-root computation, record encoding, dependency lookup, receipt construction, transition-record growth), so no domination of implementation work is claimed until Layer-0 calibration establishes it — `QO-17`'s chain, conditional — and a budget that covers `Work(E)` covers exactly the quantity the escrow decrements, the gates of §3.3–§3.4 check, and the backstop bounds. **Never over-admitted.** Admission takes a maximum over the same bounded paths the runtime can execute (§3.4), so an admitted execution's total charge is at most `B(P) ≤ W_budget` — the static certification and the dynamic escrow are two views of one quantity. **Never silent.** Every movement posts into a committed record (§2.6), the terminal receipt itemizes the closed ledger (§3.4), and the refusal case is itemized too; there is no exit from an execution that does not state its own accounting. **Never a capacity constant.** The only finiteness limits anywhere in the relation are declared ones — per-value type bounds, trip ceilings, and the declared budget — all profile parameters riding `V`; the accounting adds no core validity limit, which is the same Obj 2 discipline §2.7 declared for values, extended to executions: finiteness enters only by declaration.

Two closure statements follow. First, the ledger is deterministic: every movement is a declared function of static data and the fired rule, so equal executions have byte-equal ledgers, reconstructible from committed artifacts alone (§3.1) — which is what makes the promotion criterion's "independent differential execution" checkable over the ledger, not only over states. Second, the accounting is consistent with the fixed-width/exact stance of §2.7 rather than parallel to it: exact values over a fixed-width transport, exact work over a declared budget — in both cases the semantics is unbounded only where the text says it is, and finite everywhere else by declaration. This is the validity this section claims: internal consistency within declared finite boundaries, under {% include term.html id="computational-consistency" text="computational consistency" %} — no more, and deliberately no less.

### 3.6 Step-3 Acceptance Condition and Non-Discharge Register

The acceptance condition closes on five deliverables, each discharged at a named subsection: one ledger equation (§3.1); static operand bounds (§3.2); sound control-flow budget admission (§3.4 — the admission rule, the refusal, the fuel backstop, and, as of this version, the specified sound terminating surrogate `B_hat(P)`); reserve/charge/refund invariants (§3.3 — the conservation identities, as of this version); and no double billing (§3.1 — one declared charge per transition, the base included once, no separate dispatch deduction anywhere). The registers are consumed, not amended: the F1 row of §1.4 is discharged by §3.2 (the cost domain) on the Step-2 typing it cites; the F2 row by §3.1 (the single equation and the single-charge shape), with its per-class trace publication registered to Step 4 below; the F4 row by §3.4 (both arms taken, the simple-sum claim excluded, the four charge classes in scope, frame and proof-verification shapes owned by their extensions); and the forward reference of §2.1 — "how budget admission certifies that statically" — by the termination certification of §3.4, with the accumulator row of §2.2 consumed by §3.3.

What this section deliberately does not deliver: **no numeric charge table** — the charge coefficients, the gateway charge, the base quantity, and the budget are profile parameters; this file fixes their shape, domain, and discipline, and their values are declared with the profiles and conformance artifacts that carry them, under `QO-23`'s governance clause for coefficient ranges; **no implementation of `B_hat(P)`** — the sound, terminating surrogate and its complexity bound are specified at §3.4 as of this version (F16, v0.3.0-draft.7), but its implementation, its execution, and its adversarial tests remain Step-4 artifact-share deliverables, and `QO-17`'s calibration chain remains ordered after this discharge, with the poly-time question still open and conditional; **no published per-class ledger traces** (Step 4 — the equation here is their checking target); **no frame or proof-verification charge specification** (owning extensions, Steps 5–6 — the verbs they inherit are fixed at §3.3); **no trace roots, no artifact-lineage commitment, no collision bucket, no verbalizer or extractor identity binding** (Step 5 — unchanged from §2.8); **no extension admitted or specified** (Step 6 — unchanged). No opcode cost is declared anywhere in this section; no capacity constant is a validity limit of any rule (§3.5); the promotion criterion is untouched — "Candidate 1" is not styled a theorem, `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, the draft remains a Working Draft, and QnIR remains **[PENDING]** in the portal milestone. The standing-queue notes of §5 record what this increment does not fire: no queue trigger fires and no review loop opens by any increment of this draft, and the Steps 1–3 minimum that the Adversarial Review Program's gate names for submission candidacy is met as of this version — a recorded state, not an opened loop. Under {% include term.html id="computational-consistency" %}, the validity claimed for this section is internal consistency within declared finite boundaries: the accounting is consistent with the frozen inventory of §1.2 and the declared semantics of §2, every declared choice cites its binding register row, and no canon file is contradicted or mutated.

---
[[/CODE]]

### II.12 QnIR Container Extension §7 v0.3.0-draft.36 — `qnir_containers.md`

[[CODE]]
# QnIR Containers — The Step-6.1 Container Extension Specification (v0.3.0-draft.34)

This file carries §7 of the QnIR v0.3 working draft, introduced at v0.3.0-draft.33 as **Step 6.1, Share 1** under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Like `docs/qnir_commitments.md`, this companion is a **new section**, not a moved monolith block: it has no predecessor in the v0.3.0-draft.5 monolith, and the §7 body below is its initial committed content. It is grounded in the measured trade study of [`docs/logs/qnir/step6-1-container-boc-study.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-1-container-boc-study.md) and in the five-row contract surfaces of [`docs/qstructures.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qstructures.md) v0.3.0; it does not edit either.

---

## 7. Container Extension (Step 6.1 — Extension Share 1)

This section declares the **Step-6.1 Container Extension**: encoding version **`qir-enc-3`** and profile version **`qir-prof-ext-containers-1`**. It is a specification share only. The artifact/validator share that implements the thirteen opcodes and the `C1`–`C6` fixtures is **Share 2**, forward of this wave. The extension is admitted to the draft's deferral register under the default-deny closure rule of `docs/qnir.md` §1.1: a construct enters only by an amendment to the repair sequence, adjudicated under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7, with the Step-6.1 study as the binding evidence. The amendment here is **additive and extension-scoped**; it does not amend core §1–§6.

### 7.1 Core Preservation and Amendment Binding

`qir-enc-3` **strictly extends `qir-enc-2`**. Every frozen core construct is byte-unchanged:


The version tuple on an extension Intent carries encoding `qir-enc-3` and profile `qir-prof-ext-containers-1`; a core-only Intent continues to carry `qir-enc-2` and `qir-prof-golden-2`. Mixed use is not a new semantic mode: the extension opcodes are simply unavailable to a `qir-enc-2` Intent, and an opcode tag in `17`–`29` under `qir-enc-2` is a statically illegal envelope (unknown opcode tag, §2.6). No new status token is introduced: the closed token vocabulary of §2.5 and the four terminal outcomes remain total over the extension's rejection surfaces.

### 7.2 The Fifth Value Tag and Container Subtypes

**`TAG_CONTAINER = 5`.** A container value is a declared value form in the §2.2 `Dynamic value store`, carrying a subtype word, a declared static capacity ceiling **`cap_bnd`**, and a declared element bound **`elem_bnd`** over the canonical value bytes of its members. Its canonical shape, declared inside P2, is the ordered structure **`[5, subtype, cap_bnd, elem_bnd, [[k_0, v_0], ...]]`**, with the `(key, value)` pairs in strictly ascending key/index order. The current **length `len`** is the pair count of that last component — it is derived canonically from the value, never a separate mutable field, and it satisfies `0 <= len <= cap_bnd`. The two subtypes are closed:


For `ARRAY_RRB16` each `k_i` is an `INT_POS` index in `0..len−1`; for `TABLE_MBTREE16` each `k_i` is an `INT_POS` key in `[0, 2^64)`. `elem_bnd` is an `INT_POS` upper bound on each member value under §2.7. `HAMT` and `ChunkedFlat` are **recorded as rejected** by the Step-6.1 BOC adjudication and are not subtype words: the rejection is a finding on the register, not an erasure (§3 step 3 of `docs/qstructures.md`). The quantitative grounds are in the study: 30×–300× lookup and ~50× deterministic fold with an 18.9 MB DRAM spill for `HAMT`, and the `N ≥ 4096` chunk-copy penalty for `ChunkedFlat`. A future subtype requires a new encoding generation, never an in-place addition to this closed set.

`cap_bnd` and `elem_bnd` are declared static profile parameters under Obj 2 and §2.7: finiteness enters only by declaration, no runtime `len` is a bound, and exceedance is a typed pre-dispatch rejection (§7.3). The profile ceiling `max_cap = 65536` is a declared profile bound, not a core validity limit; the measured evidence supports ceilings up to that value.

### 7.3 The Thirteen Closed Extension Opcodes (Tags 17–29)

The opcode set is closed at thirteen words, split by container type as the Step-6 Containers row of `docs/qnir.md` §1.3 requires. All thirteen operate inside component (3)'s `Dynamic value store`: `*_NEW` binds an empty container (`len = 0`), `*_LEN` binds the current length as an `INT_POS`, `*_GET`/`TBL_AT` bind members, `*_SET` binds a new version by overwrite or insert, `*_BATCH` binds one coalesced new version, and `*_ROOT` binds the 32-byte commitment as a four-word tuple. Every result is a **new value binding** in the store under §2.4's minting rule; no container is mutated in place. There is no `ARR_FOLD`/`TBL_FOLD` opcode and no dependence on Step-6.2 `OP_CALL`, closures, or call frames: deterministic sequential traversal is expressed with the core `ARR_LEN`/`ARR_GET` or `TBL_LEN`/`TBL_AT` plus `CMP`, `BR`, and block arguments (§7.6, `C1`/`C2`).


#### 7.3.1 Exact §2.6 64-Bit Word Grammar

Per §2.6, every instruction is the opcode tag word, then its declared operand fields in declared order, then one uniform trailing **`ext`** word (the `EXT`-side-table bitmask). The operand kinds used here are exactly the §2.6 kinds: **`ref`** = one word (a static value id), **`blk`** = one word (a block id), **`cnt`** = one word (a declared count), **`bnd`** = the type-tag word followed by one bound value encoding (`INT_POS`: count word plus magnitude words). The exact sequences are:


For example, `cap_bnd = 16` as a `bnd` is the three words `0000000000000001` (type `INT_POS`), `0000000000000001` (count 1), `0000000000000010` (magnitude 16). A `cnt` operand is the count word itself, not a `ref`. The `ext` word is present on every instruction, including those with no other operands, exactly as §2.6 requires.

The canonical `TAG_CONTAINER = 5` value inside P2 is the §2.6 word encoding of **`[5, subtype, cap_bnd, elem_bnd, [[k_0, v_0], ...]]`**: type-tag word `5`, then the subtype word (`1` or `2`), then `cap_bnd` as an `INT_POS` value, then `elem_bnd` as an `INT_POS` bound, then one count word `n`, then `n` pair encodings, each `[key, value]` with `key` an `INT_POS` word encoding and `value` a complete §2.6 value encoding, strictly ascending by `key`/index. `n = len` is derived from that count; no separate length word exists, and a re-ordered or duplicated pair makes the P2 encoding malformed. An `ARR_ROOT`/`TBL_ROOT` result is the §2.6 `TUPLE` encoding `3, count=4, (1,1,w0), (1,1,w1), (1,1,w2), (1,1,w3)` — fourteen words — of the digest's big-endian 64-bit words.

**Rejection surface.** `*_GET` out-of-bounds access, `*_SET` invalid index or bound exceedance, `*_AT` out-of-range rank, `cap_bnd`/`elem_bnd` exceedance, and a `*_BATCH` whose indices/keys are duplicated, unsorted, or otherwise out of declared range are **typed rejections** yielding `EXPRESSION_TOO_COMPLEX` under §2.3/§2.5 — the token already exists and its terminalization is unchanged. A rejected batch binds no new version and posts only its own site-stable charge. `TBL_GET` is the sole miss-routing opcode and adds **zero** new status tokens: the miss is the existing F25 Tier-O token, observed/consumed only by a successful fallback completion (§2.5). Container values appear in the dynamic value store's canonical order under §2.6's `(static value id, instance id)` key; no container-specific store or configuration field is created.

### 7.4 The Five Binding Obligations

The Step-6 Containers row fixes five identical rules across the alternatives. All five are declared here once, binding both subtypes.

1. **Canonical Serialization (`qir-enc-3`).** The wire artifact of a container value is structure-independent: an `ARRAY_RRB16` serializes as a canonical ordered sequence of `(index, canonical_val_bytes)` pairs and a `TABLE_MBTREE16` as the canonical CBOR map `{u64 => canonical_val_bytes}`, in both cases alongside the declared subtype word and `cap_bnd`, exactly as the `[5, subtype, cap_bnd, elem_bnd, pairs]` layout of §7.2/§7.3.1 requires. Encoding is deterministic under §2.6's canonical rules; two containers with the same members, subtype, and declared ceilings have byte-equal artifacts regardless of their internal tree shape. The study's shared artifacts (9,195 / 151,275 bytes for dense `N = 256 / 4096`; 11,003 / 176,123 bytes for sparse `N = 256 / 4096`) are measured evidence, not encoding constants.

2. **Hash Function.** SHA-256, matching P1 and `qir-trace-root-1`: a content leaf commits `SHA-256(0x00 || key_le || val_bytes)`; each internal node commits `SHA-256(dom || meta || pairwise-root)` where the pairwise level is `SHA-256(0x20 || left || right)` with an odd final node **promoted unchanged** (never duplicated). The **empty root is `SHA-256(0x02)`**; a non-empty container with one member has the root equal to that member's `0x00` leaf, and larger containers fold pairwise with `0x20`. The root is a 32-byte string whose pre-image is retained per F15; equality decisions remain over canonical pre-images, and a genuine collision remains the Step-5 `qir-artifact-cmp-1` bucket's case. No new hash algorithm, width, or domain-separation rule is admitted.

3. **Persistence Policy.** Copy-on-write structural persistence: every `*_SET`/`*_BATCH` allocates a new root-to-leaf path and shares every untouched subtree; no node is mutated in place. A version is a new value binding under §2.4 and is committed by the existing pre/post state roots. **Long-term multi-tier retention classes remain `QUEUED` under `Q1` and `Q3`**: their trigger has not fired, and nothing here declares residency, eviction, or tier promotion. The measured footprint classes of `qir-prof-ext-containers-1` (obligation 4) are access-path cache tiers, not retention policy.

4. **Cache Model (`qir-prof-ext-containers-1`).** The profile binds the `QO-17` Layer-0 tier ladder of the record host (Intel Core i7-8565U): `L1d 3.4 ns`, `L2 4.8 ns`, `L3 11.2 ns`, `DRAM 125.0 ns`, measured by dependent pointer chase; cycles remain `ns × 1.80` estimates. The study's `ARRAY_RRB16` at `N = 65,536` is 7,549,656 bytes and therefore still **L3-resident inside the 8 MiB L3** through `N = 65536`, with a 15.5 ns median indexed `get`; the `TABLE_MBTREE16` counterpart at 8,445,528 bytes crosses into DRAM (54.8 ns median keyed `get`). These are declared tier bindings for cost accounting, not portability claims about other silicon.

5. **Batch Rules.** A batch is a `TUPLE` of `batch_cnt` strictly ascending unique `(key/index, value)` pairs with `1 <= batch_cnt <= max_batch = 64`, normalized by the declared order before the transition; duplicates and unsorted input are rejected (§7.3). The commit coalesces the bottom-up Merkle path rehash: shared ancestors are hashed once, bounded by the declared `batch_cnt` and never by a runtime `len`. The study measured `B = 64` (384–1,920 SHA-256 calls depending on subtype and N); `max_batch` is a profile constant, and the measured value is evidence for its declaration, not a core validity limit.

### 7.5 Static Cost Domain, Bounds, and `charge(t)` (§3.2 / §3.4)

The profile `qir-prof-ext-containers-1` declares the concrete constants: **`gateway = 20 U`**, **`trip_ceiling = 1`** per back-edge site (the single `CLOAD`/`TBL_GET` fallback slot `0` included), **`S_max = 4096`**, **`max_cap = 65536`**, and **`max_batch = 64`**. Core opcodes keep their `qir-prof-golden-2` charge of `4 U` each; the extension charges below replace that single base figure for tags `17`–`29`.

The declared static depth function over the **declared** `cap_bnd` is:

`depth_16(cap_bnd) = max(1, ceil(log_16(max(cap_bnd, 1))))`

giving exactly `1` for `cap_bnd <= 16`, `2` for `<= 256`, `3` for `<= 4096`, and `4` for `<= 65536`. It is a pure function of the declared `cap_bnd` — never the runtime `len`, never the runtime path depth, member length, or digest — so §3.2's cost domain is preserved and charges are site-stable and positive.


Every charge is positive, so §3.4's fuel arithmetic remains strictly decreasing and the §3.5 termination certification unchanged. `B_hat(P) = B_cfg(P) >= B_sem(P)` is computed by the committed trip-ceiling DP with `gateway = 20 U`: a straight-line extension program contributes `20 + Σ charge(t)`, and a back-edge site may be traversed at most `trip_ceiling = 1` times, so the DP's maximizing path includes the body at most twice. **`TBL_GET`'s fallback edge `(src_blk, src_idx, 0)` is added to `B_hat(P)` identically to `CLOAD` in `bhat`** — it is a branch point at the `TBL_GET` instruction, the hit path continues in-block and the miss path enters `fallback_blk`, and the two are joined by the max rule; the miss token itself adds no charge class. The reserve/charge/refund verbs of §3.3 are inherited unchanged: no second budget, surrogate, or accumulator is introduced. The `QO-14` domination question stays where Layer-0 calibration left it: `U` orders declared work but does not by itself dominate wall time; the study's ns/cycles/SHA/allocation surface is diagnostic evidence for a future U calibration, not a substitute for one.

### 7.6 Declared Extension Fixture Surface (`C1`–`C6`, Share 2)

Six **hand-derived** fixtures are declared here for implementation in Share 2 (`src/qnir/` and `src/qnir-py/`) under the same test-only discipline as `F27`/`F6`: they are **not conforming golden vectors**, contribute nothing to the 84-vector declared seed, and require **no edit to `docs/qnir_goldens.md`**. All use `cap_bnd = 16`, `elem_bnd = 255`, hence `depth_16(16) = 1`; core opcodes charge `4 U`, `ARR_NEW`/`TBL_NEW`/`*_LEN`/`*_ROOT` charge `4 U`, `ARR_GET`/`TBL_GET`/`TBL_AT` charge `8 U`, `ARR_SET`/`TBL_SET` charge `10 U`, and `ARR_BATCH` charges `16 U` at `batch_cnt = 2`. Listing notation is `block:index MNEMONIC operands -> bound value id`; `vN` are static value ids and `aN`/`bN`/`cN` are block-argument ids. `B_hat` is the declared CFG bound including the gateway; `Work(E)` is the exact charged path; `n` counts executed transitions (records), including a fatal rejection record where applicable.

**`C1` — `ARRAY_RRB16` new/set/len and a `BR`-loop fold via `ARR_LEN`/`ARR_GET`.**

```
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5,  bnd INT_POS<=255 -> v1
B0:2  CONST imm=0,  bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3              (append at idx 0; len=1)
B0:4  ARR_LEN v3, bnd INT_POS<=16 -> v4      (binds 1)
B0:5  CONST imm=0, bnd INT_POS<=16 -> v5     (idx counter)
B0:6  JMP edge->B1(v3, v5, v4)              (arity 3)
B1:0  CMP a1, a2, rng[-1,1] -> v6
B1:1  BR LT v6, true->B2(a0,a1,a2), false->B3(a0)
B2:0  ARR_GET b0, b1, bnd INT_POS<=255 -> v7
B2:1  CONST imm=1, bnd INT_POS<=16 -> v8
B2:2  ADD b1, v8, bnd INT_POS<=16 -> v9
B2:3  JMP edge->B1(b0, v9, b2)              (back-edge, traversal #1)
B3:0  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=100 U`.
- Hand-derived `B_hat(P) = 20 + 74 = 94 U`; `Work(E) = 94 U`; `n = 16`; terminal outcome `SUCCESS`.
- Fold proof: the header `B1` is entered with `idx=0, len=1`, the `BR` takes `true`, `B2` reads index `0` with `ARR_GET`, increments to `1`, and the single back-edge traversal returns to `B1`; the second header test is `1 < 1 = false`, so `B3` halts. The loop body runs once with **zero** `OP_CALL`/closure dependency.

**`C2` — `TABLE_MBTREE16` new/set/len and a `BR`-loop fold via `TBL_LEN`/`TBL_AT`.**

```
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=2^64-1 -> v2   (key 0)
B0:3  TBL_SET v0, v2, v1 -> v3                  (insert; len=1)
B0:4  TBL_LEN v3, bnd INT_POS<=16 -> v4         (binds 1)
B0:5  CONST imm=0, bnd INT_POS<=16 -> v5        (rank)
B0:6  JMP edge->B1(v3, v5, v4)
B1:0  CMP a1, a2, rng[-1,1] -> v6
B1:1  BR LT v6, true->B2(a0,a1,a2), false->B3(a0)
B2:0  TBL_AT b0, b1, bnd INT_POS<=2^64-1, bnd INT_POS<=255 -> v7
B2:1  CONST imm=1, bnd INT_POS<=16 -> v8
B2:2  ADD b1, v8, bnd INT_POS<=16 -> v9
B2:3  JMP edge->B1(b0, v9, b2)                  (back-edge, traversal #1)
B3:0  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=100 U`.
- Hand-derived `B_hat(P) = 20 + 74 = 94 U`; `Work(E) = 94 U`; `n = 16`; terminal outcome `SUCCESS`.
- Fold proof: identical shape to `C1`, with `TBL_AT` binding the rank-0 `(key, val)` tuple and the same single allowed back-edge traversal; the key-order fold needs no closure or call frame.

**`C3` — `TBL_GET` miss fallback and the F25 trip ceiling.**

```
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=0, bnd INT_POS<=2^64-1 -> v1
B0:2  TBL_GET v0, v1, fallback_blk=1, bnd INT_POS<=255 -> v2
B0:3  HALT                         (hit path only; not executed on this miss)
B1:0  CONST imm=9, bnd INT_POS<=255 -> v3
B1:1  HALT                         (successful fallback completion)
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=50 U`.
- Hand-derived `B_hat(P) = 20 + max(24 miss, 20 hit) = 44 U`; `Work(E) = 44 U`; `n = 5`; terminal outcome `SUCCESS`.
- Miss behavior: the empty table misses at `B0:2`; Tier-O `STATUS_LOOKUP_MISS` is born, the `(B0, 2, 0)` trip-ceiling check permits the one fallback-edge traversal (ceiling 1), `B1` completes successfully and consumes the token; the receipt itemizes it as consumed, and no new token type or rejection is produced.

**`C4` — coalesced `ARR_BATCH` and duplicate rejection (two declared runs).**

```
C4a (accepted):
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  CONST imm=7, bnd INT_POS<=255 -> v3
B0:4  CONST imm=1, bnd INT_POS<=16  -> v4
B0:5  TUPLE 2 fields [INT_POS<=16, INT_POS<=255] (v2, v1) -> v5
B0:6  TUPLE 2 fields [INT_POS<=16, INT_POS<=255] (v4, v3) -> v6
B0:7  TUPLE 2 fields [TUPLE, TUPLE] (v5, v6) -> v7
B0:8  ARR_BATCH v0, v7, cnt=2 -> v8
B0:9  HALT

C4b (rejected) — identical to C4a except B0:6 uses (v2, v3),
                 i.e. duplicate index 0 instead of strictly ascending 0,1.
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=80 U`.
- `C4a`: `B_hat(P) = 20 + 52 = 72 U`; `Work(E) = 72 U`; `n = 10`; outcome `SUCCESS`; one coalesced commit, bottom-up rehash of the shared ancestor once, `len = 2`.
- `C4b`: `B_hat(P) = 72 U` (same static program); `Work(E) = 68 U`; `n = 9`; outcome `EXPRESSION_TOO_COMPLEX` at `B0:8`, before any version is bound; the static `HALT` on the success path is not executed.

**`C5` — `cap_bnd` and `ARR_GET` bounds rejections.**

```
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3                 (append; len=1)
B0:4  CONST imm=1, bnd INT_POS<=16  -> v4      (idx == len)
B0:5  ARR_GET v3, v4, bnd INT_POS<=255 -> fatal
B0:6  HALT                                    (success path only)
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=60 U`.
- Hand-derived `B_hat(P) = 20 + 38 = 58 U` (the static `B0:6 HALT` is on the success path); `Work(E) = 54 U`; `n = 6`; terminal outcome `EXPRESSION_TOO_COMPLEX` at `B0:5` (`idx == len`), itemized with the existing fatal token. A declared `cap_bnd > max_cap = 65536` is additionally rejected statically at envelope validation (not an execution vector), so it has no `B_hat`/`Work`/`n`.

**`C6` — canonical and root equivalence across subtypes.**

```
C6a (array):
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3
B0:4  ARR_ROOT v3 -> v4
B0:5  EMIT cnt(kind)=0, cnt(1), ref v4
B0:6  HALT

C6b (table):
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=2^64-1 -> v2
B0:3  TBL_SET v0, v2, v1 -> v3
B0:4  TBL_ROOT v3 -> v4
B0:5  EMIT cnt(kind)=0, cnt(1), ref v4
B0:6  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=60 U` for both.
- Each run: `B_hat(P) = 20 + 34 = 54 U`; `Work(E) = 54 U`; `n = 7`; outcome `SUCCESS`.
- Exact roots. Empty root before the insert, both subtypes: `SHA-256(0x02) = dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986`, words `(0xdbc1b4c900ffe48d, 0x575b5da5c6380401, 0x25f65db0fe3e2449, 0x4b76ea986457d986)`. After inserting `key = 0`, `val = INT_POS 5` (canonical `val_bytes` = `0000000000000001 0000000000000001 0000000000000005`), the one-member root is the `0x00` leaf itself, `SHA-256(0x00 || 0u64_le || val_bytes) = 3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4`, words `(0x3874f4bb0de71f11, 0x35773da3d2f9fdf9, 0x85c3f9862aa5ad6f, 0xbb689f7b9127c8d4)`, **identical for `C6a` and `C6b`**; no `0x20` node is formed at cardinality one. The serialized P2 shapes differ only as declared (ordered sequence versus map) while the `(key, value)` pairs, leaf hash, and root are equal.

The fixture transcripts' fixture blocks must be byte-identical between the two artifacts under the §4.4 read-back discipline; their digests will be published in the Share 2 log. No fixture is a golden, no fixture changes `E`, `n`, the corpus identity, or the 52 published trace-root lines of normal mode.

### 7.7 Acceptance Condition and Non-Discharge Register

Share 1 is **DISCHARGED** when this section is committed: the extension version pair is declared, the five-row contract is filled for both admitted subtypes, the thirteen opcodes and the fifth value tag are closed with their exact §2.6 word grammar, the five binding obligations and the concrete profile bounds/`charge(t)` formulas are fixed, and `C1`–`C6` are hand-derived with exact sequences and numbers before any implementation. The register remains open:

- **Share 2 — artifact/validator share**: **DISCHARGED at v0.3.0-draft.34**. Both implementations accept `qir-enc-3`/`qir-prof-ext-containers-1`, emit the byte-identical `C1`–`C6` fixture block (SHA-256[:16] `2c96b152f38a7d44`), and reproduce every §7.6 expectation exactly; the corpus stays untouched. The verification record is [`docs/logs/qnir/step6-1-container-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-1-container-artifact-share.md).
- **Remaining Step-6 extensions**: FUTURE — Calls, Saturation, Fork–join, and Road proofs, each with its own owning study and evidence before admission.
- **`Q1` / `Q3`**: `QUEUED` — no long-term multi-tier retention/residency contract is declared here.
- **`Q6`**: `QUEUED` — this section introduces no term include for any unregistered slug; the only term includes used are `boc-policy`, `receipt`, and `computational-consistency`, all already in `docs/qnir.md`/`docs/qnir_semantics.md`.
- **Sealing / promotion**: not claimed. Under {% include term.html id="computational-consistency" %}, the validity claimed here is internal consistency within the declared extension boundary: §7 is additive to §1–§6, its opcodes are closed and typed, its bindings repeat the Step-6.1 study's rules verbatim in substance, and nothing in it amends a canon file, a committed vector, or a glossary entry.

---
[[/CODE]]

### II.13 QnIR Call Extension §8 v0.3.0-draft.36 — `qnir_calls.md`

[[CODE]]
# QnIR Calls — The Step-6.2 Call Extension Specification (v0.3.0-draft.36)

This file carries §8 of the QnIR v0.3 working draft, introduced at v0.3.0-draft.35 as **Step 6.2, Share 1** under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Like `docs/qnir_containers.md`, this companion is a **new section**, not a moved monolith block: it has no predecessor in the v0.3.0-draft.5 monolith, and the §8 body below is its initial committed content. It is grounded in the measured trade study of [`docs/logs/qnir/step6-2-call-trade-study.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-2-call-trade-study.md), in `QO-21`/Obj 6 of [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md), and in the BOC policy of `docs/simemp.md` §7; it does not edit any of them.

---

## 8. Call Extension (Step 6.2 — Extension Share 1)

This section declares the **Step-6.2 Call Extension**: encoding version **`qir-enc-4`** and profile version **`qir-prof-ext-calls-1`**. It is a specification share only; the artifact/validator share that implements tags `30`–`31`, envelope key `9`, and the `--fixture K` surface is **Share 2**, forward of this wave. The extension is admitted to the deferral register under the default-deny closure rule of `docs/qnir.md` §1.1, adjudicated under the {% include term.html id="boc-policy" text="BOC policy" %} with the Step-6.2 study as the binding evidence. The retained candidate is **bounded acyclic calls only**: unbounded recursion and implicit stack growth are rejected as findings, not silently admitted.

### 8.1 Core and Container Preservation; the `V.encoding` Dispatch Gap

`qir-enc-4` **strictly extends `qir-enc-3`**. Every frozen core and container surface is byte-unchanged:


**Mandatory validator rule (closing the dispatch gap).** The Gate-0 inspection of the committed artifacts showed that `decode_envelope` reads envelope key `2` only as text and stores it as `V.encoding`; extension behavior is selected by CLI fixture mode, not by the envelope. Under `qir-enc-4` this is closed: `decode_envelope` and `dec_insn` MUST inspect `V.encoding` and reject envelope key `9` and opcode tags `30`–`31` whenever `V.encoding` is `qir-enc-2` or `qir-enc-3`; conversely an envelope with key `9` or tags `30`–`31` is admissible only when `V.encoding = qir-enc-4`. A tag outside `1`–`31`, or an envelope key outside `0`–`9` under `qir-enc-4` (`0`–`8` under the earlier versions), is malformed under §2.6's total unknown-field rejection. No new status token is introduced: the closed token vocabulary of §2.5 and the four terminal outcomes remain total over the extension's rejection surfaces.

### 8.2 Envelope Callee Table and the Acyclic Call-Graph Rule

The Intent envelope gains exactly one **closed** key under `qir-enc-4`:

- **Key `9 = callees`**: an ordered array of callee records, each a four-element CBOR array with the exact types **`[callee_id: UInt, arg_bnds_bytes: Bytes, ret_bnds_bytes: Bytes, blocks_bytes: Bytes]`**:
  - `callee_id` a dense word `>= 0`, unique, ordered ascending;
  - `arg_bnds_bytes` carries the canonical §2.6 `bnd` word sequences (the same word grammar key `6` uses) for the callee's typed argument signature, in declared order;
  - `ret_bnds_bytes` carries the canonical §2.6 `bnd` word sequences for the callee's typed return signature, in declared order;
  - `blocks_bytes` carries the §2.6 block-stream byte string in the callee's own static-id space, with length a multiple of 8.

The caller entry remains outside the `callees` array and uses the existing key `6` instruction stream. **Strict topological acyclicity** is a static legality rule: an `OP_CALL` inside callee `i` may only target `callee_id > i`; the caller entry may target any `callee_id >= 0`. A self-call (`callee_id <= i` inside callee `i`), a cycle, an unknown `callee_id`, an arity mismatch against `arg_bnds`, or a return-count/type mismatch against `ret_bnds` makes the Intent statically illegal. There is **no runtime recursion**: F26 retired depth ceilings because no operational nesting measure existed, so finiteness is certified here by the static call graph plus the declared `C_max` bound, not by a runtime depth counter with unbounded growth (Obj 2). The call graph is a DAG; `C_max = 8` bounds the longest admitted chain.

**Callee entry binding and P10 return binding.** `callee.blocks[0]` MUST have block-argument arity exactly equal to `argc`, and `OP_CALL` binds `arg_refs[0..argc]` **positionally** into `callee.blocks[0]`'s argument environment (§2.2 component (2)) inside the callee's fresh frame; an arity mismatch is the static illegality named above. Under P10 (dense static value ids over value-defining instructions in block-then-index scan order), `OP_CALL` defines **at most one** static value id: `retc == 0` defines **0** ids and therefore has no `-> vN` target; `retc == 1` defines **1** id binding the returned value directly; and `retc >= 2` defines **1** id binding a single canonical `TAG_TUPLE` of `retc` fields in declared `ret_bnds` order, whose members are unpacked with the core `FIELD` opcode. This preserves P10's one-id-per-defining-instruction discipline without multi-result renumbering, and it mirrors the container extension's `TBL_AT`/`*_ROOT` tuple convention (§7.3).

### 8.3 The Two Call Opcodes and the Hash-Chained Frame Stack

Two opcode words are added, both using §2.6's uniform trailing `ext` word:


Explicit §2.6 expansion: **`cnt` = 1 word**, **`ref` = 1 word**, **each `bnd` = variable `1 + (1 + len)` words** (one type-tag word, one count word, then `len` magnitude words under §2.6's INT_POS value grammar), and **`ext` = 1 word**; the opcode tag itself is 1 word. Thus `OP_CALL` occupies `3 + argc + retc + Σ_bnd_words(ret_bnds)` words and `OP_RET` occupies `3 + retc` words, the same closed word grammar as the core instruction stream.

`OP_CALL` binds the callee's declared return values as fresh value instances in the caller's §2.2 component (3) store. `OP_RET` is the **terminator of a callee exit block**; it binds nothing in the caller beyond the declared returns and settles the sub-budget (§8.4). A `HALT` inside a callee is statically malformed unless a typed fatal rejection seals the execution directly at that transition (§2.5): a callee cannot successful-halt the whole program, because only the caller entry owns the terminal receipt. `OP_CALL` in the caller entry and `OP_RET` in any callee are the only new control surfaces; no call frame, closure value, or continuation object is admitted as a value form.

**Hash-chained frame stack (P2).** The stack is a persistent commitment carried inside the §2.6 canonical configuration encoding P2, never a new configuration component:

- `stack_root_0 = SHA-256(0x02)` is the empty base (the same domain byte the empty-trace constant uses).
- Pushing a frame with serialized bytes `suspended_frame_bytes` yields **`stack_root_{d+1} = SHA-256(0x30 || stack_root_d || suspended_frame_bytes)`** — the `0x30` byte domain-separates the frame chain from the `0x00`/`0x20` leaf/node domains and the `0x01` trace domain.
- At `depth == 0`, existing **P2 Component (2)** remains exactly its canonical `active_args` encoding, so core and container P2 bytes stay **100% invariant**. At `depth > 0`, Component (2) is the two-element array **`[active_args, parent_stack_root]`**, where `parent_stack_root = stack_root_d` and `active_args` is the current top frame's argument environment; the suspended frame bytes are **not** duplicated in live P2 but are retained in the execution's F15 pre-image chain and are reconstructible deterministically. P2 size is therefore `O(F_top)` rather than `O(C · F)`.
- Frame `d` binds its arguments in its own §2.4 instance context: every value is keyed by `(static value id, instance id)`, instance ids are minted in transition order and never reused, so a callee can never alias or overwrite a suspended caller binding. On `OP_RET` the top frame is popped by restoring `parent_stack_root` as the current stack root; the suspended caller frame bytes ride in the retained pre-image, not in the live P2.

`depth <= C_max = 8` is enforced at `OP_CALL` before any effect posts; frame live values `<= F_max = 64` are enforced by the frame's declared bounds; and `C_max · F_max = 512` is the declared memory ceiling for frame storage. Exceeding any of them is the typed `EXPRESSION_TOO_COMPLEX` rejection of §2.5. The trade study measured the cost of this choice directly: Candidate C's P2 grows from 111 B at `C=1` to **5,512 B at `C=8, F=64`**, which is why high-depth calls lose to inlining on P1 hashing even when the callee body is large.

### 8.4 Compositional `B_hat`, Sub-Budget Conservation, Failure and Token Rules

The profile `qir-prof-ext-calls-1` declares **`gateway = 20 U`, `trip_ceiling = 1`, `S_max = 4096`, `C_max = 8`, `F_max = 64` (so `C_max · F_max = 512`), `max_cap = 65536`, and `max_batch = 64`**. Core opcodes keep `4 U`; container opcodes keep their `qir-prof-ext-containers-1` charges under §7.5. The two new charges are **site-stable and positive**:

- `charge(OP_CALL) = 4 + 2 · argc` U
- `charge(OP_RET) = 4 + 2 · retc` U

**Compositional `B_hat`.** Compute `bhat(callee_id)` once per callee in reverse topological order of the key-9 DAG; each callee's own trip-ceiling DP must satisfy `dp_bound(callee) <= S_max` and its state product `|B_callee| · Π_i(ceil_i + 1) <= S_max`. At each `OP_CALL` site in a caller, fold the callee's worst-case interior bound `B_callee_body` as one site-stable charge; the caller's DP therefore needs one state per call site rather than one copy of the callee's CFG per site. The total DP stays `<= S_max` even when `m >= 5`: inline state product is `4m · 2^(2m)` and refuses at `m = 5` (20,480 > 4096), while compositional states are `Σ dp_states(callee) + #call_sites + caller_states` — 22 states at `m = 6` in the recorded study. `B_hat(P) = B_cfg(P) >= B_sem(P)` and `S_max` are otherwise unchanged; a `None` bound still refuses with `EXPRESSION_TOO_COMPLEX`.

**§3.3 sub-budget settlement.** `OP_CALL` checks that the caller's open admission escrow covers `B_callee_body + charge(OP_RET)` before any effect posts, then allocates a transition-local reserve `r_sub` equal to that covered amount. The callee executes inside `r_sub`; `OP_RET` refunds the unused residue `r_sub − (charged_inside + charge(OP_RET))` back to the caller's escrow. The four §3.3 verbs are inherited without redefinition: reserve, charge, refund, and terminal residue settlement. At every settlement boundary, including the final one, **`reserved = charged + refunded`** exactly; the harness verified this on 270/270 execution rows. Frame escrows are transition-local reserves in the sense of §3.3, not a second ledger and not a second budget.

**Failure propagation.** A typed rejection inside a callee (including a zero divisor or a bound exceedance) is fatal at its own transition: it births `DOMAIN_ERROR` or `EXPRESSION_TOO_COMPLEX`, seals the fatal state in that transition's own record, and prevents any later `OP_RET` or caller `HALT` from re-sealing it (F7). The sub-budget residue is still settled by that final transition, and the outer caller's refund accounting is not corrupted: the study measured `reserved=52, charged=28, refunded=24` for both fatal classes. Unconsumed tokens — Tier-S `STATUS_NEG_EVEN_ROOT` and Tier-O `STATUS_LOOKUP_MISS` — stay in the **global** token pool across `OP_RET`; they are not frame-local. A Tier-S token determines the terminal outcome by the §2.5 earliest-birth rule (`NO_SOLUTION` for an unpromoted `STATUS_NEG_EVEN_ROOT`), while a Tier-O miss carries no verdict and is consumed only by a successful fallback completion. Zero new token types are introduced.

**Memoization rule (`QO-21`).** `CLOAD`-style verified-receipt reuse at an `OP_CALL` site is admitted only when the callee is typed, context-independent, and its receipt is valid at the call site — the amortization condition `QO-21` declares. The reuse key is the content digest of `(callee_id, arg bytes, qir-prof-ext-calls-1 version)`; a hit charges the `CLOAD` path (including its fallback-edge trip ceiling) instead of re-executing the callee body, and the retained receipt is checked under §6.7's complete-artifact rule before reuse. When the callee is context-dependent, the receipt is invalid, or the measured lookup cost is not below re-execution, direct `OP_CALL` execution is used. The study found reuse wins broadly at `m = 8, C = 1` (26–79% below Inline) but dynamic calls still lose at `C = 8` without reuse (+171% at `P = 64, m = 8`); memoization is therefore a declared optimization rule, not an unconditional default.

### 8.5 The Five Binding Obligations

The Step-6 Containers row fixes five rules; the call extension inherits them and adds its frame rule:

1. **Canonical serialization.** callee tables and frame bytes use the §2.6 word grammar and deterministic CBOR; a `(callee_id, arg bytes)` pair has one canonical encoding.
2. **Hash function.** SHA-256, 32 bytes: `0x02` empty/root base, `0x00` leaf, `0x20` pairwise, `0x01` trace nodes — matching P1 and `qir-trace-root-1`.
3. **Persistence.** COW structural persistence with no in-place mutation; a frame push creates a new commitment, and the suspended frame's pre-image is retained per F15.
4. **Cache model.** the `QO-17` tiers (L1d 3.4 ns, L2 4.8 ns, L3 11.2 ns, DRAM 125.0 ns) bound the access path; the 5,512 B P2 at `C=8,F=64` is a DRAM-bound measurement, not a core constant.
5. **Batch / frame rules.** sorted `batch_cnt` behavior is unchanged from §7; frame live values `<= F_max`, depth `<= C_max`, and `C_max · F_max` memory are declared profile ceilings (Obj 2).

### 8.6 Hand-Derived Fixtures `K1`–`K6` (Share 2 `--fixture K`)

The fixtures below are **test-only**, outside the 84-vector declared seed and the `C1`–`C6` container fixture set; they require no edit to `docs/qnir_goldens.md`. Listing notation is `block:index MNEMONIC operands -> bound value id`; `aN` are callee argument ids in their own instance space, `cN` are callee-local ids, `vN` caller ids. `B_hat` is the declared admission surrogate; `Work(E)` is the exact charged path; `n` counts executed transitions including a fatal rejection record where applicable. All amounts are U.

**`K1` — basic `OP_CALL`/`OP_RET` (`argc = 2 → retc = 1`).**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2  OP_CALL callee_id=0, argc=2, arg_refs=(v0,v1), retc=1, ret_bnds=(INT_POS<=255) -> v2
B0:3  HALT
C0:0  ADD a0, a1, bnd INT_POS<=255 -> c2
C0:1  OP_RET retc=1, ret_refs=(c2)
```

Declared `C_max=8`, `F_max=64`, `W_budget=60`. Hand-derived `B_hat = 50` (gateway 20 + 2·CONST 8 + CALL 8 + ADD 4 + RET 6 + HALT 4), `Work(E) = 50`, `n = 6`, outcome `SUCCESS`, tokens none. Sub-budget `r_sub = B_callee_body + charge(RET) = 4 + 6 = 10`; RET residue `10 − (4+6) = 0`; ledger `reserved=50, charged=50, refunded=0`.

**`K2` — nested acyclic calls, `depth = 3 <= C_max`; hash chain and SSA isolation.**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2  OP_CALL callee_id=0, argc=1, arg_refs=(v1), retc=1, ret_bnds=(INT_POS<=255) -> v2
B0:3  HALT
C0:0  OP_CALL callee_id=1, argc=1, arg_refs=(a0), retc=1, ret_bnds=(INT_POS<=255) -> c0
C0:1  OP_RET retc=1, ret_refs=(c0)
C1:0  OP_CALL callee_id=2, argc=1, arg_refs=(a0), retc=1, ret_bnds=(INT_POS<=255) -> c1
C1:1  OP_RET retc=1, ret_refs=(c1)
C2:0  CONST imm=1, bnd INT_POS<=255 -> c2
C2:1  ADD a0, c2, bnd INT_POS<=255 -> c3
C2:2  OP_RET retc=1, ret_refs=(c3)
```

Declared `W_budget=90`. `B_hat = Work(E) = 76` (gateway 20 + caller CONST 8 + 3·CALL 18 + C2 CONST/ADD 8 + 3·RET 18 + HALT 4), `n = 11`, outcome `SUCCESS`. Frame roots: `S0 = SHA-256(0x02)`, `S1 = SHA-256(S0 || frame0)`, `S2 = SHA-256(S1 || frame1)`, `S3 = SHA-256(S2 || frame2)`; P2 carries only `frame2` plus `parent_stack_root = S2` (1 live value, `<= 64`). Instance ids are monotone across all frames, so `v0/v1`, `c0/c1`, and `c2/c3` never alias; ledger `reserved=76, charged=76, refunded=0`.

**`K3` — `m = 6` sequential calls to a `k = 2`-loop callee; compositional admission.**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2..B0:7  OP_CALL callee_id=0, argc=2, arg_refs=(v0,v1), retc=1, ret_bnds=(INT_POS<=255) -> v2..v7
B0:8  HALT
C0 block 0: ADD 4 U + BR 4 U -> block 1 / back-edge site 0 (trip ceiling 1)
C0 block 1: ADD 4 U + BR 4 U -> block 2 / back-edge site 1 (trip ceiling 1)
C0 block 2: ADD 4 U + JMP 4 U -> block 3
C0 block 3: OP_RET retc=1, ret_refs=(c_ret) = 6 U
```

Declared `W_budget=400`. The callee's worst-case interior bound is `B_callee_body = 16 + 16 + 8 + 6 = 46 U` with `dp_states(C0) = 16`. Inline state product `24 · 2^12 = 98,304 > 4096` → **refused**; compositional `B_hat(K3) = 20 + 4 + 4 + 6·(8 + 46) + 4 = 356 U` with caller + callee DP states `1 + 16 = 17 <= 4096` → **admitted**. On the fast non-looping runtime path (`8 + 8 + 8 + 6 = 30 U` per call), `Work(E) = 20 + 8 + 6·(8 + 30) + 4 = 260 U`, `n = 2 + 6·(1 + 7) + 1 = 51`, outcome `SUCCESS`. Ledger `reserved=356, charged=260, refunded=96` (`356 = 260 + 96`), using `charge(OP_CALL) = 4 + 2·2 = 8 U` and `charge(OP_RET) = 4 + 2·1 = 6 U` exactly. The first inline refusal remains `m = 5`.

**`K4` — inner fatal propagation and sub-budget conservation (two variants).**

```
K4a (DOMAIN_ERROR):
B0:0  CONST imm=1, bnd INT_POS<=255 -> v0
B0:1  OP_CALL callee_id=0, argc=1, arg_refs=(v0), retc=1, ret_bnds=(INT_POS<=255) -> v1
C0:0  DIV a0, ZERO, bnd INT_POS<=255 -> fatal
K4b (EXPRESSION_TOO_COMPLEX):
C0:0  CONST imm=256, bnd INT_POS<=255 -> fatal   (pre-dispatch bound exceedance)
```

Declared `W_budget=60`. Both variants: `B_hat = 20 + 4 + 6 + (4 + 6) + 4 = 44 U` (gateway + caller CONST + `OP_CALL` at `argc=1` + callee interior `DIV`/`CONST` 4 U + `OP_RET` at `retc=1` 6 U + caller HALT), `Work(E) = 20 + 4 + 6 + 0 = 30 U`, `n = 3`, no `OP_RET` and no caller `HALT` executed; terminal outcome `DOMAIN_ERROR` (tag 4) for `K4a` and `EXPRESSION_TOO_COMPLEX` (tag 3) for `K4b`, sealed at the fatal transition. Ledger `reserved=44, charged=30, refunded=14` (`44 = 30 + 14`) — conservation PASS with the outer caller's escrow intact. Zero new tokens.

**`K5` — token propagation across `OP_RET`.**

```
B0 initial pool: [STATUS_NEG_EVEN_ROOT]
B0:0  CONST imm=0, bnd INT_POS<=2^64-1 -> v0
B0:1  OP_CALL callee_id=0, argc=1, arg_refs=(v0), retc=0, ret_bnds=()   (no -> target: P10 defines 0 ids)
B0:2  HALT
C0:0  TBL_GET tbl_ref, key_ref=v0, fallback_blk=C1, bnd INT_POS<=255 -> c0
C0:1  OP_RET retc=0, ret_refs=()
C1:0  CONST imm=9, bnd INT_POS<=255 -> c9
C1:1  JMP edge->C0:1 (arity 0; successful fallback completion)
```

Declared `W_budget=70`, `cap_bnd=16` (so `TBL_GET` charges `4 + 4·depth_16(16) = 8 U`). `B_hat = Work(E) = 20 + 4 + 6 + 8 + 4 + 4 + 4 + 4 = 54 U` (gateway + caller CONST + `OP_CALL` at `argc=1` + `TBL_GET` + fallback CONST + JMP + `OP_RET` at `retc=0` 4 U + caller HALT), `n = 7`, outcome `NO_SOLUTION`, tokens: `STATUS_NEG_EVEN_ROOT` outstanding from the initial pool and never consumed; `STATUS_LOOKUP_MISS` born at `C0:0`, consumed only after `C1` completes successfully, and absent from the terminal verdict. No new token types; ledger `reserved=54, charged=54, refunded=0`. With no successful fallback completion the miss token stays outstanding and the outcome is still `NO_SOLUTION` — Tier O never alters the outcome.

**`K6` — static rejections (no execution).**


Each: `B_hat = null`, `Work(E) = 0`, `n = 0` (zero inner postings), no terminal receipt beyond the declared refusal. These are validator-side checks; no execution transition is counted.

The fixture transcript's fixture blocks must be byte-identical between the two artifacts under §4.4's read-back discipline; their digests will be published in the Share 2 log. No fixture is a golden, and no fixture changes `E`, `n`, the corpus identity, or the 52 published trace-root lines of normal mode.

### 8.7 Acceptance Condition and Non-Discharge Register

Share 1 is **DISCHARGED** when this section is committed: the extension version pair is declared, preservation and the `V.encoding` dispatch gap are fixed, the key-9 callee table and acyclic rule are closed, the two opcode word grammars and frame commitment are fixed, the profile bounds/charges/compositional surrogate/sub-budget rules are declared, and `K1`–`K6` are hand-derived before implementation. The register remains open:

- **Share 2 — artifact/validator share**: **DISCHARGED at v0.3.0-draft.36**. Both implementations accept `qir-enc-4`/`qir-prof-ext-calls-1`, enforce key-9/tag-30–31 version gating, emit the byte-identical `K1`–`K6` fixture block (SHA-256[:16] `e83189cb00a56497`), and reproduce every §8.6 check exactly. The verification record is [`docs/logs/qnir/step6-2-call-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-2-call-artifact-share.md).
- **Remaining Step-6 extensions**: FUTURE — Saturation, Fork–join, and Road proofs, each with its own evidence.
- **`Q1` / `Q3`**: `QUEUED` — long-term multi-tier retention/residency remains deferred.
- **`Q6`**: `QUEUED` — this section introduces no term include for any unregistered slug; the only term includes used are `boc-policy`, `receipt`, and `computational-consistency`, all already in `docs/qnir.md`/`docs/qnir_semantics.md`.
- **Sealing / promotion**: not claimed. Under {% include term.html id="computational-consistency" %}, the validity claimed here is internal consistency within the declared extension boundary: §8 is additive to §1–§7, its call graph is statically acyclic and depth-bounded, its sub-budgets inherit §3.3 exactly, and nothing in it amends a canon file, a committed vector, or a glossary entry.

---
[[/CODE]]

### II.14 QnLang Rung-0 Candidate v0.1.0-draft.1 — `qnlang.md`

[[CODE]]
# QnLang — Rung-0 Candidate Specification (High-Level Authoring Language)


This document materializes the Rung-0 candidate surface language of the Qn universe. It is a **new versioned artifact** under **Axiom 13** and a candidate construct under **Axiom 14**; no prior file is mutated in place. It records the separation demanded by F19: {% include term.html id="qnlang" text="QnLang" %} is a surface language with its own syntax, typing, and compilation relation, not an extension of `src/qnir/`, not a Rust or Python dialect, and not a shadow implementation of the intermediate representation. The program remains `[PENDING]` in `docs/index.md`; this candidate does not promote it.

---

## 1. Epistemic Stance, `F19` Separation & Bootstrap Boundary

### 1.1 Rung-0 stance

Rung-0 is a **minimal, total, human-authorable specification** sufficient to express exact computations with explicit profiles, budgets, and causal state. Its validity claim is internal consistency within declared finite boundaries under {% include term.html id="computational-consistency" %} — not a theorem, not promoted canon, and not an execution proof.

The language sits **outside Layer 3**. Layer 3 is the {% include term.html id="qexpr" text="Qexpr" %} / {% include term.html id="cae-dag" text="CAE-DAG" %} symbolic composition layer; QnLang authors surface terms which the compiler collapses into that layer and then lowers to {% include term.html id="qnir" text="QnIR" %}. It is a {% include term.html id="complexity-degree" text="Complexity Degree 2" %} authoring membrane, not a fourth execution layer. The {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7 governs later selection; none is made here.

### 1.2 F19 separation from `src/qnir/`

F19 bindingly requires: "QnIR is an execution and verification substrate; QnLang is a surface language with its own syntax, typing, and compilation relation." Therefore:

- No QnIR opcode, register schedule, or `qir-enc-4` byte rule is copied into surface syntax.
- `src/qnir/`, `src/qnir-py/`, and all QnIR artifacts are untouched by Rung-0.
- The only shared commitment is the lowering target `qir-enc-4` under the `R11'` simulation obligation.
- A Rust or Python oracle is not a QnLang generation.

### 1.3 Bootstrap boundary and deferred work


The bootstrap boundary is also the hardware boundary: {% include term.html id="layer-1" text="Layer 1" %} genesis and {% include term.html id="layer-0" text="Layer 0" %} physical execution are described by their owning documents; QnLang only names declared profiles and budgets.

---

## 2. Core Semantic Model — `CAE-AST`, `QO-24` NameMap, Affine Memory Purity

### 2.1 CAE-AST

The QnLang abstract syntax tree is a **hash-consed, content-addressed expression DAG** (`CAE-AST`) plus a **De-Bruijn control DAG** for bounded structured control. Nodes are immutable and identified by complete canonical bytes, not heap address; {% include term.html id="hash-consing" %} gives structural sharing and {% include term.html id="dependencies-determinism" %} gives zero conditional entropy at the commitment boundary. Every node cites its {% include term.html id="dcc-profile" text="DCC profile" %}, bounds, and causal parent edge under the {% include term.html id="causal-arrow" text="causal arrow" %} discipline; no implicit default is admitted under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %}.

Control uses De-Bruijn indices: a bound name is an index into the enclosing binder, never a mutable cell. No pointer arithmetic, no `&'a` lifetime, no implicit heap, and no garbage-collection pause exists. Physical realization belongs to the {% include term.html id="hca-collapse" text="HCA-Collapse Engine" %} and {% include term.html id="simemp-gateway" text="SIMEMP Gateway" %}.

### 2.2 NameMap sidecar (`QO-24`)

Value identity and derivation identity are separated. `NameMap` is an **optional deterministic sidecar** mapping a canonical value digest to human-facing names and provenance annotations; it is not consulted by equality, typing, or evaluation. Two values are equal only by full canonical encoding under minimal-zoom normalization. The sidecar may be emitted as a receipt annotation but never changes a commitment.

### 2.3 Affine memory purity and COW fallback

**Immutable scalars** (`Qm`, `Qleaf`, `rat`, `TermH`) are copyable: they carry no mutable state and can be referenced repeatedly. **Governed containers** (`Q(array)`, `Q(table)`) obey linear ownership across `Q(store)`, `Q(batch)`, and `Q(drop)`: every container value is consumed exactly once by its terminal transition. `Q(fold)` and `Q(load)` take an immutable borrow and do not consume the container. SSA names are **never rebound**; a second mint of the same name is `E_NAMESPACE_COLLISION`, a standing well-formedness invariant enforced ahead of the 14 semantic gates.

Rung-0 default memory discipline is affine: a binding is consumed exactly once unless declared persistent. When structural sharing across versions is needed, the source names an admitted persistent container:

- `Q(array)` → `RRBTree-16` under `qir-enc-3` / `qir-prof-ext-containers-1`;
- `Q(table)` → `MerkleBTree-16` under the same profile;
- the general family is the {% include term.html id="q-datastruc" text="Q(dataStruc)" %} contract class, whose no-implicit-heap rule is inherited verbatim.

Choosing linear vs COW, and deciding when `z_min` normalization pays for itself, are Step-6.3 `Q(rsi)` obligations; neither is implicit.

### 2.4 Domain substrate

The six orthogonal domains are vertical projections across Layers 1–3: {% include term.html id="qm" text="Qm" %}, {% include term.html id="qs" text="Qs" %}, {% include term.html id="qlog" text="Qlog" %}, {% include term.html id="qphy" text="Qphy" %}, {% include term.html id="qchem" text="Qchem" %}, and {% include term.html id="qbio" text="Qbio" %}. All share the CAE-AST carrier, the work unit {% include term.html id="compute-unit-u" text="U" %}, and the receipt discipline of §6.

---

## 3. Operator Hygiene & `Q(0)..Q(9)` Structural Anchors

### 3.1 Zero overload

Rung-0 fixes one meaning per token; a collision is a compile-time diagnostic:


`=` occurs **only** in `binary`. No declaration uses `=`. The canonical zero is {% include term.html id="q0" text="Q(0)" %}; the canonical multiplicative identity is {% include term.html id="q1" text="Q(1)" %}. No naked numeral is admitted as a literal.

### 3.2 `Q(0)..Q(9)` anchor table


The {% include term.html id="q-bypass" text="Q(bypass)" %} suite of `Q(6:sym)` is the zero-cost identity surface; {% include term.html id="q-reuse" text="Q(reuse)" %} is the declared lookup surface; direct exact compute is the fallback under the two-axis breakeven of `docs/qnPrimitive.md` §1.15/§5.9.

---

## 4. Formal EBNF Grammar — Rung-0 Complete

The grammar is unambiguous and profile-bound. `:=` is the **meta-production** operator and never appears in source programs. `⟨...⟩` marks prose non-terminals; quoted tokens are literals; `[...]` is optional; `{...}` is repetition with a declared ceiling; `|` is alternation. Every bound is a source token, never an inferred default. The only `/` admitted in source is the dimensional ratio inside `unit(...)`; it is never value division. The only `..` admitted is inside `qtype` bounds and `range`.

```ebnf
program          := { header_decl } { top_decl } ;

header_decl      := "Q(dcc)" ident ";"

top_decl         := "Q(type)" ident ":" qtype ";"
                                              "state" ":" param_list ";"
                                              "act" ":" param_list ";" "}"
                        { "case" ident ":" expr "->" ident ";" } "}" ;

contract_tag     := "![" contract_item { "," contract_item } "]" ;
contract_item    := "Qs(" ident "," ident ")" | "Qphy(" ident ")" | "Qchem"

param_list       := ident ":" qtype { "," ident ":" qtype } ;
variant_arm      := ident "(" [ param_list ] ")" ;
ident            := letter { letter | digit | "_" } ;
uint             := digit { digit } ;
sint             := [ "-" ] uint ;
digit            := "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
bound            := sint ;
letter           := "a" | ... | "z" | "A" | ... | "Z" ;
unit_atom        := "J" | "s" | "kg" | "m" | "K" | "mol" | ident ;
unit_expr        := unit_atom { "/" unit_atom } ;   (* dimensional ratio only *)

qprim            := [ "-" ] "Q(" ( uint | ident ) [ ":" ident ] ")" [ "<unit(" unit_expr ")>" ] ;

qtype            := ident | "u8" | "u16" | "u32" | "u64" | "bool"
                       "," "q[" bound ".." bound "]" "," "axis(" ident ")" ">"

container_ctor   := "Q(arr_new)<" qtype "," "cap(" uint ")" ">"

target_pat       := ident | "(" ident { "," ident } ")" ;
expr_list        := expr { "," expr } ;

primary          := qprim | ident | "(" expr { "," expr } ")"

builtin_call     := "Q(metric_dot)" "(" expr "," expr ")"

structured       := "Q(2:split)" "(" expr ")" "{" "T" "->" block [ ";" ] "F" "->" block [ ";" ] "}"
                       "as" "(" ident { "," ident } ")" block
                       "->" target_pat [ "or_else" block ] ;

postfix          := primary { "." ( uint | ident | ident "?" ) | "(" [ expr_list ] ")" } ;
unary            := ( "-" | "~" ) postfix ;
binary           := unary ( "+" | "-" | "*" | "=" | "!=" | "~="
range            := "[" expr ( ".." | "..<" ) expr "]" ;
expr             := binary | range ;

bind             := expr "->" target_pat [ ":" qtype ] ";" ;
stmt             := bind | expr ";" ;
block            := "{" { stmt } [ "->" expr ] "}" ;
```

No production introduces an implicit value, a pointer, a floating literal, an unbounded repetition, a Unicode multiplication dot, `===`, or a bare `/`. `=` is admitted **only** in `binary`; declarations use `:`. `Q(conjecture)` / `Q(falsify)` / `Q(experiment)` are proof-shaped declarations; they emit receipts under §6 and never change a value without an explicit follow-on binding.

---

## 5. Type System, Built-in Signatures & Six Orthogonal Domains

### 5.1 Type system and `Q(variant)`

A QnLang type is a declared tuple `(domain, tag, bound, unit?, liveness)`; inference is absent. `Q(variant)` declares a closed sum, `Q(match)` eliminates it with exhaustive branches, and `Q(type)` names a structural type. A variant value carries its constructor tag and payload canonical bytes. Token-state transitions are explicit in source. The five-level `C|E|H|N|U` lattice is carried as a declared type attribute.

### 5.2 Explicit built-in signatures

All built-ins used by §4 or §6 have exactly one declared signature:


### 5.3 `Qs`

`Qs` authors intent and epistemic delegation: `Q(blade) <= 16`, `TermH`, the `C|E|H|N|U` lattice, `N` containment, `sim<C, H>`, `Q(agent)`, and the `I/P/S` grounding ladder of {% include term.html id="qs-history-library" text="Qs History Library" %}. Emission is mediated by `may_emit(...)` and projected through `Φ_status`; the projection is a reporting boundary. The intent carrier is the {% include term.html id="intent-contract" text="Intent Contract" %} of `docs/qs.md`.

### 5.4 `Qlog`

`Qlog` contributes `Q(pattern)` (`R_road` segment) and `Q(workflow)` (chained receipt vector), with `Q(mode) L0..L3`. `Q(abduce)`, `Q(anchor)`, `Q(prefetch)` are Rung 1 except where `Q(anchor)` is used as the local genesis built-in above. The two-axis `Q(reuse)` macro-hop is the only admitted Rung-0 compression morphism. The {% include term.html id="q-patterns" text="Q(pattern)" %} / {% include term.html id="q-workflows" text="Q(workflow)" %} glossary definitions bind these terms.

### 5.5 `Qm`

`Qm` provides exact arithmetic: `Q(kappa)`, `Qleaf`, `zoom`, `.exact?`, `z_min`, `align`, `scale_leaf`, `eta(p,q,s)`, `Q(root)`, `Q(cf)`. {% include term.html id="zoom-z" text="zoom z" %}, {% include term.html id="remainder-r" text="remainder r" %}, and {% include term.html id="dimension-d" text="dimension d" %} are canonical CAE node attributes; {% include term.html id="terminal-exactness" text="Terminal Exactness" %} discharges a remainder of `Q(0)`. `Q(root)` and `Q(cf)` are symbolic generators of `docs/qm.md`.

### 5.6 `Qphy`

`Qphy` declares `Qphy<unit(...), T>` and `Q(0)<unit(...)>` typed quantities, five declared physical anchors, and `leash(...)`. Units are part of the type; `Q(0)<J>` and `Q(0)<s>` are distinct. No wall-clock or unmetered energy source is admitted. The physical anchors are those registered in `docs/qphy.md`; QnLang names them, it does not re-derive them.

### 5.7 `Qchem`

`Qchem` requires `∩ C_Qs = empty`, `S·v = Q(0)` expressed in source as `Q(metric_dot)(S, v) = Q(0)` (zero Unicode dot), explicit `Q(2:chirality)`, and `Q(bypass)` catalysis. A `Q(domain) (Qchem, Qm)` header enforces abiotic isolation mechanically: no `Qs` identifier may appear in a `Qchem` example. Stoichiometry is exact under `Qm`.

### 5.8 `Qbio`

`Qbio` contributes `Q(blanket) { sense; state; act }` and `metabolic_cap`. The blanket partitions observation, internal state, and action; all three are explicit morphisms. `metabolic_cap` is a declared budget ceiling; it forbids implicit growth. {% include term.html id="qbio" text="Qbio" %} is an orthogonal projection, not a Layer 4.

### 5.9 Cross-domain contracts

`qs_history.md` contributes the `I/P/S` ladder, `Q̂(0)` schema portability, and a wall-clock ban. `qstructures.md` contributes the five-row contract schema; Rung-0 checks **Locality**, **Determinism**, and **Work** against the `QO-17` cache tiers. Footprint is checked through {% include term.html id="metric-envelope" text="Metric Envelope" %}, exhaustion through {% include term.html id="metric-exhaustion" text="metric exhaustion" %}, and all crossings through the {% include term.html id="simemp" text="SIMEMP" %} constraints. {% include term.html id="structural-indirection" text="Structural Indirection" %} lets these contracts evolve independently of concrete hardware.

---

## 6. Zero-Concession Compiler (`qnlangc`) & Dual-Channel Diagnostics (`R12'`)

### 6.1 Compiler principle

`qnlangc` admits no concession: every bound, unit, domain, and lifetime is declared. It lowers CAE-AST to `Qexpr` and then to `qir-enc-4`; it never invents a default and never silently drops a receipt. Diagnostic totality is the `R12'` obligation: every rejected program receives both a machine tuple and a human summary. Immutable scalars are copyable; governed containers are linear across `Q(store)` / `Q(batch)` / `Q(drop)`; SSA names are never rebound (`E_NAMESPACE_COLLISION`, enforced ahead of the 14 gates).

### 6.2 Fourteen semantic gates


Plus the standing invariant `E_NAMESPACE_COLLISION`: SSA names are never rebound. A program passing all gates is *admissible*, not proven.

### 6.3 Dual-channel diagnostics

**Channel A — agent tuple:** `(code, span, term_id, expected, actual, remedy, receipt_ref)`.

**Channel B — `qir-verbalizer-1` NL summary:**

```text
qir-verbalizer-1: <code> at <file>:<line>:<col> — <short>; expected <expected>, got <actual>; remedy <remedy>
```

Both channels are emitted for every diagnostic. The {% include term.html id="receipt" text="receipt" %} is the shared evidence boundary; the NL summary is a rendering, not a second diagnosis.

### 6.4 Worked reference examples

Each example below is a complete `program` under §4 and satisfies all 14 gates plus the standing namespace invariant.

**Qm** — exact arithmetic, declared bounds, ranges, remainders:

```text
Q(dcc) dcc0;
Q(kappa) dyadic;
Q(domain) Qm;
Q(mode) L0;
Q(type) T : Qm[-255..255];
Q(anchor) zero : Q(0) : T;
Q(metric) met : eta(2,3,5);
Q(1) -> lo : T;
Q(4) -> hi : T;
[lo..hi] -> rng : Qm[-255..255];
-Q(3) -> a : T;
scale_leaf(a) -> sl : T;
align(sl) -> al : T;
zoom(al, z_min) -> v : T;
v.exact? -> exact : bool;
emit receipt;
```

**Qphy** — typed physical quantity with leash:

```text
Q(dcc) dcc1;
Q(kappa) dyadic;
Q(domain) Qphy;
Q(mode) L0;
Q(type) Jt : Qphy<unit(J), Qm[-255..255]>;
Q(anchor) jzero : Q(0)<unit(J)> : Jt;
Q(metric) ph : eta(1,1,1);
Q(0)<unit(J)> -> energy : Jt;
leash(energy, Q(4)) -> leashed : Jt;
may_emit(leashed, receipt) -> authorized : bool;
emit receipt;
```

**Qchem** — abiotic stoichiometry, linear array, metric_dot flux fold:

```text
Q(dcc) dcc2;
Q(kappa) fact;
Q(domain) (Qchem, Qm);
Q(mode) L1;
Q(type) Stoich : Qm[-255..255];
Q(type) Pair : Q(tuple)<Stoich, Stoich>;
Q(anchor) zero : Q(0) : Stoich;
Q(metric) ch : eta(1,1,1);
Q(variant) Species { A(s : Stoich), B(s : Stoich) }
Q(2:cons)(Q(0), Q(1)) -> pa : Pair;
Q(2:cons)(Q(1), Q(0)) -> pb : Pair;
Q(arr_new)<Pair, cap(2)> -> arr : Q(array)<Pair, cap(2)> ![budget(20 U)];
Q(store)(arr, pa) -> arr1 : Q(array)<Pair, cap(2)>;
Q(store)(arr1, pb) -> arr2 : Q(array)<Pair, cap(2)>;
Q(fold) arr2 over [Q(0)..<Q(2)] init(Q(0)) as (acc, i) { -> acc + Q(1) } -> flux : Stoich;
Q(drop)(arr2);
Q(metric_dot)(pa, pb) = zero -> balanced : bool;
emit receipt;
```

**Qbio** — blanket phases and metabolic clamp:

```text
Q(dcc) dcc3;
Q(kappa) dyadic;
Q(domain) Qbio;
Q(mode) L2;
Q(type) BioT : Qm[0..255];
Q(blanket) Blanket {
  sense : x : BioT;
  state : s : BioT;
  act : a : BioT;
}
Q(anchor) bzero : Q(0) : BioT;
Q(metric) bio : eta(1,1,1);
Blanket -> inst : BioT;
metabolic_cap(inst, Q(64)) -> clamped : BioT;
may_emit(clamped, receipt) -> ok : bool;
emit receipt;
```

**Qs + Qlog** — pattern/workflow, table, load miss, modal view, authorized exits:

```text
Q(dcc) dcc4;
Q(kappa) dyadic;
Q(domain) (Qs, Qlog);
Q(mode) L3;
Q(type) KeyT : u64;
Q(type) StepOut : Q(tuple)<bool, u64>;
Q(anchor) kzero : Q(0) : KeyT;
Q(metric) lg : eta(1,1,1);
Q(variant) Step { Hit(o : StepOut), Miss(o : StepOut) }
Q(falsify) bad(k : KeyT) ![Qlog, may_emit(receipt)] {
  Q(halt_with)(receipt);
}
Q(pattern) step(k : KeyT) -> StepOut ![Qlog, budget(64 U)] {
  Q(tbl_new)<KeyT, bool, cap(2)> -> tbl : Q(table)<KeyT, bool, cap(2)> ![budget(20 U)];
  Q(store)(tbl, (k, Q(1))) -> tbl1 : Q(table)<KeyT, bool, cap(2)>;
  Q(load)(tbl1, k) @[trip(1)] -> hit or_else {
    Q(cut)(Q(0));
    -> (Q(0), Q(0))
  } -> out : StepOut;
  Q(drop)(tbl1);
  -> hit
}
Q(workflow) route(k : KeyT) -> StepOut ![Qlog, budget(80 U), may_emit(receipt)] {
  step(k) -> res : StepOut;
  Q(reuse)(res) -> memo : StepOut;
  -> memo
}
modal_as(route, Qlog) -> view : Qlog;
emit receipt;
```


---

## 7. `R11'` Lowering Overview & Self-Bootstrap Ladder

### 7.1 Lowering overview

`R11'` requires a simulation relation from source to QnIR. Rung-0 expresses it as three deterministic steps, not a proof of a theorem:

1. **CAE-AST normalization** — hash-cons, apply `Q(bypass)` identities bottom-up, sort commutative children, freeze `z_min`.
2. **Domain erasure** — `Qm`/`Qs`/`Qlog`/`Qphy`/`Qchem`/`Qbio` annotations become declared bounds, units, and receipt tags.
3. **Encoding lowering** — normalized DAG flattens to `qir-enc-4` with explicit `trip(K)`, `cap(N)`, `budget(W U)`; byte grammar is owned by QnIR companions.

{% include term.html id="q-anchor" text="Q(anchor)" %} commitments from `qs_history.md` may be cited as provenance; {% include term.html id="existential-invariant" text="Existential Invariants" %} reject a lowering that introduces approximate arithmetic, implicit state, or non-deterministic ordering through the {% include term.html id="simemp-gateway" text="SIMEMP Gateway" %}.

### 7.2 Self-bootstrap ladder


The iterative compiler uses `Q(array)` as arena and `Q(variant)` as tagged union; recursion is a trip-bounded worklist under `C_max = 8`.

### 7.3 Step-6.3 / Bootstrap-Profile prerequisites

1. **Per-site `trip(K)` + SCC-additive `bhat`.**
2. **Fixed-width bitwise operations.**
3. **Governed `Q(hash)`** with profile-bound cost and collision-bucket discipline.

Until these have BOC evidence, Stage 1 and Stage 2 remain design targets. QnLang stays `[PENDING]`.

---
[[/CODE]]

## Part III — Detailed Focus: Post-draft.30 Evolutions

### III.1 Chronological delta ledger

#### III.1.1 `441b73f` — v0.3.0-draft.31 — Pre-Step-6 stabilization

**Summary.** P_gap migration; Two-Tier gateway accounting; S_max=4096 pre-DP gate; val==None refusal; F6 fixture COVERED; V.semantics re-pin.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.2 `711f1f2` — QO-17 Part 1 — Layer-0 physical calibration

**Summary.** i7-8565U tier ladder L1d 3.4ns / L2 4.8ns / L3 11.2ns / DRAM 125ns; bignum DIV O(n^2); S_max dominators bottleneck; Work(E) 1596-4994 ns/U.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.3 `12a4e78` — Q5 sealing — Q(anchor) dual predicate

**Summary.** Qs History Library v0.2.0; I/P/S witness ladder; architecture.yml grounding class registration.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.4 `6cc1ef1` — v0.3.0-draft.32 — F-G0-1 state-root repair

**Summary.** P1=SHA-256(P2) 32-byte roots across 336 records; corpus 659,880 LF bytes; Python UTF-8 stdout; QO-17 paired delta +4.7% median.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.5 `8b40089` — v0.3.0-draft.33 — Step 6.1 Share 1

**Summary.** Container extension qir-enc-3; 13 opcodes tags 17-29; TAG_CONTAINER=5; K-free C1-C6 fixture declarations.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.6 `4106b88` — v0.3.0-draft.34 — Step 6.1 Share 2

**Summary.** Container artifact share; --fixture C 8/8 differential; block digest 2c96b152f38a7d44; V.encoding dispatch gap identified.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.7 `e139de3` — v0.3.0-draft.35 — Step 6.2 Share 1

**Summary.** Call extension qir-enc-4; acyclic callee table key 9; OP_CALL/OP_RET grammar; K1-K6 hand-derived fixtures.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.8 `92487c2` — v0.3.0-draft.36 — Step 6.2 Share 2

**Summary.** Mandatory V.encoding gating; --fixture K 10/10 differential; block digest e83189cb00a56497; 8/8 cargo tests.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

#### III.1.9 `48ac9a9` — Milestone 2 — QnLang Rung-0

**Summary.** docs/qnlang.md v0.1.0-draft.1; CAE-AST; zero = overload; 14 compiler gates; 5 gate-compliant programs; navigation entry.

**Review reading.** This entry is the anchor for the corresponding Part I decision request. The reviewer should compare the verbatim canonical text in Part II with the artifact evidence reproduced below, then record any divergence as a finding rather than an in-place correction.

**Evidence checklist.** (a) committed file set identified; (b) transcript or JSON artifact identified; (c) digest or record count stated; (d) promotion status unchanged. Where an item is absent, the entry marks the gap explicitly.

### III.2 Native measurement artifacts

#### III.2.qo17-part1-layer0-calibration.md

[[CODE]]
---
layout: default
title: QO-17 Part 1 — The Layer-0 Physical Calibration Benchmark (Native Windows, v0.3.0-draft.31)
description: Deterministic Layer-0 timing of the committed QnIR core on an Intel Core i7-8565U — the memory-wall ladder, SHA-256, CBOR, arithmetic scaling, B_hat against S_max, and end-to-end X1–X51 ns-per-U — recorded as measurement evidence, not a canon change
nav_exclude: true
---

# QO-17 Part 1 — The Layer-0 Physical Calibration Benchmark (Native Windows, v0.3.0-draft.31)

Measurement record of the `QO-17` Part 1 core Layer-0 physical calibration benchmark, run read-only against `441b73f` (v0.3.0-draft.31) on native Windows. Decision wave: v0.3.0-draft.31. **No repository file was created, edited, or deleted by the run**: the harness is a standalone crate in the run environment's temp directory that copies the committed modules read-only, and the repository was `git status --porcelain`-clean at start and end. The measurement subject is the committed QnIR core over the frozen 16-opcode set; the record changes no QnIR semantics and seals no design.

## What was emitted

**1. Layer-0 memory-wall tier ladder.** Single dependent 8-byte pointer chase over a randomly permuted index array (no stride, so hardware prefetchers cannot help), plus a sequential 64 KiB buffered read from `%TEMP%`. ns shown as min / median / p95 / max.


The ladder is monotone and close to the textbook shape: ~1.4× to L2, ~3.3× to L3, ~37× to DRAM. The DRAM ratio measured **36.5×** on an earlier repeat of the same binary and **37.1×** in the saved JSON; the spread is repeat noise, not a different working set. The storage baseline is a whole-buffer transfer, not a single dependent access, so its ratio (≈39,500×) is not comparable: normalized it is **2.03 ns/byte** (133,100 ns / 65,536 B) for a warm OS-cached read.

**2. Core implementation-work primitives.**

SHA-256 (`sha256::sha256`) across payloads, with median throughput:


The fixed cost dominates below 1 KiB (a 64 B digest already costs ~1,700 cycles); throughput saturates near **238 MB/s** at 4 KiB and softens at 16 KiB, consistent with a portable byte-at-a-time implementation rather than a hardware SHA extension.

Canonical CBOR decode + re-encode over representative `X` envelopes:


Decode plus canonical re-encode is flat at ~3.2 µs across a 7× byte range, i.e. envelope-size-insensitive at these sizes; the dominant cost is per-node traversal, not bytes copied. The `X1` p95 spike (9,600 ns) is a scheduler artifact of the ~100 ns Windows timer quantum, not a size effect.

Exact arithmetic scaling. **The committed core's arithmetic is `i128`** (`checked_add`/`checked_sub`/`checked_mul`, and `CMP` over the same representation), so its native width is at most 2 words; the 4-word and 16-word columns are **harness-side grade-school bignum probes** used only to exhibit the shape the core would inherit if widths grew. Medians, ns:


The `ADD`/`CMP` columns are inside the timer's noise floor and should be read as "flat". The signal is the pair `MUL`/`DIV`: multiplying 16× the words costs 5.5×, because schoolbook multiplication is `O(n²)` with a small constant; dividing costs **22.4×**, and 5.2× going from 4 to 16 words. Against the accounting model's flat `q_mul`/`q_div = 4 U`, physical cost and logical charge diverge by orders of magnitude as operand width grows — the width-parameterized dominance gap `F14` names.

**3. `B_hat(P)` DP scaling against `S_max = 4096`.** Across `X1`–`X51`, the per-vector median admission-surrogate latency spans **300 ns – 5,900 ns** (median 1,300 ns, p95 4,700 ns); the surrogate is cheap on real vectors. Synthetic loop-chain graphs (trip ceiling 1; `dp_bound = |B| · 2^k`) push it to the sealed ceiling:


The ceiling behaves correctly: `4096` is admitted at **1.58 ms**, `8192` is refused, and the saved JSON records `bound = null` for the refusal. But the refusal is **34.5 µs, not sub-microsecond**: `b_hat` runs its structural analysis — `back_edges()` and the iterative `dominators()` fixed point, `O(|B|³)` over 64 blocks — before it can compare against `S_max`. The DP itself is cheap; the structural pass dominates. A genuinely `O(|B|+|E|)` refusal would have to count back-edges without the full dominator pass. That is a design finding for `QO-17`, not a defect exercised by any committed vector.

**4. End-to-end `X1`–`X51` against logical `Work(E)`.** Per-vector `Work(E)`, record count `n`, envelope bytes, and wall-clock time are recorded for all 51 vectors in the saved artifacts. For the **49 vectors with `Work(E) > 0`**:


At the median that is **4,545 cycles/U**. Worst-case outliers: `X32` 4,994, `X22` 4,890, `X35` 4,858, `X17` 4,793 (U=76, n=15, 860 B), `X15` 4,772, `X12` 4,692. Best: `X44` 1,596, `X39` 1,622, `X3` 1,644, `X40` 1,650. The high-work vectors are tight (U=72 within 1.11×, U=76 within 1.06×), but low-work vectors are not: the same `U=32` ranges 1,622–3,241 ns/U (**2.00×**), `U=36` 1,933–2,525 (1.31×), `U=44` 2,520–3,225 (1.28×). Two `Work(E) = 0` vectors — `X2` (refusal) and `X50` (escrow never opened) — still cost **32.2 µs** and **29.7 µs** respectively, because envelope decode, `B_hat`, and trace-root setup are paid before or outside the ledger.

The `F14` empirical conclusion is therefore negative in the strong form and useful in the weak form: **logical `Work(E)` alone does not uniformly bound physical execution time.** `U` is a good first-order predictor (the median band is tight relative to the 2.00× same-U spread), but a bound that holds across `X1`–`X51` must also carry record count `n`, envelope/record byte size, `B_max` (and, today, the non-linear dominator pass), and width-parameterized `DIV`/`MUL` terms. `U` orders work; it does not by itself dominate wall time.

## The evidence (runs of record, this commit state)

The saved artifacts are the output of the final harness run at this state (`rustc 1.99.1`, release, LTO), which exited 0; the six artifact transcripts recorded in the draft.31 wave record also exit 0 at the same commit state. Run artifacts of record, in the run environment's temp directory:

- `qo17_part1_results.json` — 19,625 bytes, valid JSON (`json.load` parses it), with **5** memory-tier entries, **5** SHA-256 entries, **3** CBOR entries, **12** arithmetic entries, **56** `b_hat` entries (51 vectors + 5 synthetic), and **51** end-to-end entries.
- `qo17_part1_results.csv` — 138 lines across the six section tables (each section carries its own header row), same values.
- **Audit fingerprint: `aa78f491a6b942bb6ee33bb4f9bcf390db042700256527425451145b7609571e`** (SHA-256 of `qo17_part1_results.json` as saved).

The measured object was verified unchanged before the run: `docs/qnir_goldens.md` is **1,080,404 bytes, 22,151 LF lines, 0 CR bytes, SHA-256 `fa72f316c2f3bccc94260bbe6f75abecfe18895ab22a551705301d60070c92b3`** — the draft.31 corpus identity of record. The harness read `C:\cdqn` only; `git status --porcelain` was empty at the start and at the end of the wave, and no file inside the repository was created, edited, or deleted.

These runs are deterministic in the sense the trigger requires — same binary, same corpus, same warm-up/trial discipline, repeatable verdicts and bound values — and the `X1`–`X51` per-vector `ns/U` figures are the measured input to the `F14` finding above. They are a single-host Layer-0 calibration, not a portability claim: timing estimates inherit the 1.80 GHz TSC assumption and the ~100 ns Windows timer quantum visible in the p95/max steps, and they do not control thermal state or cache set assignment.
[[/CODE]]

#### III.2.phase1-draft31-stabilization.md

[[CODE]]
---
layout: default
title: Phase 1 — Pre-Step-6 Stabilization, Canon Sync, and the F6 Fixture (v0.3.0-draft.31)
description: The draft.31 stabilization wave — corpus-wide canonical sync, the qir-prof-golden-2 profile with the S_max gate, the declared F6 collision fixture, and the V re-declaration across all 58 sites; both artifacts pass all six runs and QnIR remains [PENDING]
nav_exclude: true
---

# Phase 1 — Pre-Step-6 Stabilization, Canon Sync, and the F6 Fixture (v0.3.0-draft.31)

Wave record of the pre-Step-6 stabilization wave under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.31. **The wave is committed at `441b73f`** — 22 files, +7,500/−6,888 — and closes the gap between the canon's declared parameters and the behavior the two committed artifacts already implement. It seals the `qir-prof-golden-2` profile and its `S_max` gate, declares and implements the `F6` collision fixture, and re-declares `V` across every declaration site. **Both artifacts pass all six runs of record; the corpus is byte-identical in every mode, and QnIR remains `[PENDING]` — no promotion is claimed or approached.**

## What was emitted

1. **`qir-prof-golden-2` and the `S_max` gate.** `src/qnir/src/bhat.rs` computes the exact surrogate and refuses with `None` when the certified bound exceeds `s_max = 4096`; `src/qnir-py/qnir_ref.py` mirrors the same path, including the `parse_profile` `s_max` field. Refusal is `EXPRESSION_TOO_COMPLEX` in both artifacts; the gateway default is never substituted for a refused expression. Three unit tests cover the ceiling, the refusal path, and the admission path (`cargo test --offline`: **3/3**).

2. **The `V` re-declaration.** `V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)` now stands at all 58 declaration sites, and the lineage block of every run prints it verbatim.

3. **The `F6` collision fixture.** Declared in §4.4 as a test-only surfacing of `qir-artifact-cmp-1`, implemented in both artifacts, and reachable only through `--fixture F6`. It exercises the four comparison verdicts — `same`, `collision` (`class = digest-collision`), `different` (same kind), `different` (differing kind) — and reports a one-entry bucket inside the fixture block. Normal mode is unchanged: `lineage: collision bucket (qir-artifact-cmp-1, entries = 0)`. The fixture normalizes no defective execution and is never a conforming golden.

4. **Canonical sync (Wave 1A).** The declarations were advanced additively, preserving every historical record: `docs/qnir_accounting.md` v0.3.0-draft.8.3, `docs/qnir_semantics.md` v0.3.0-draft.8.4, `docs/qnir_conformance.md` v0.3.0-draft.31, `docs/qnir_commitments.md` v0.3.0-draft.31, `docs/qnir.md` v0.3.0-draft.31, `docs/AiReviews/AiReviewQnIRv0.1.md` v0.1.14; and the companions `docs/simemp.md` v1.4.1, `docs/boc.md` v0.4.2, `docs/qnPrimitive.md` v1.4.1, `docs/abstractionLayers.md` v1.9.2, `docs/qm.md` v1.4.1, `docs/qexpr.md` v1.4.1, `docs/q0_q1.md` v1.2.2, `docs/history_library.md` v0.1.2, `docs/qs.md` v1.3.1.

5. **Two declared byte consequences, both recorded rather than hidden.** The semantics string's growth moved the `F27` envelope from 385 to 387 bytes, so that fixture block's digest moved deliberately from `ed67e6cbd242c69c` to `6f6bd8f5ea344b31`; and all 51 envelope byte-count annotations in the corpus were corrected by +2. No envelope hex byte was edited. The `draft.23`–`draft.30` records and the `F12, v0.3.0-draft.5` citation remain verbatim.

## The evidence (runs of record, this commit state)

Both artifacts were run against the committed corpus at `441b73f` in all three modes — normal, `--fixture F27`, `--fixture F6`. **All six runs exit 0 and PASS.** The toolchains of record: Rust reference interpreter (`rustc 1.99.1`, `x86_64-pc-windows-msvc`, release profile, standard library only) and the Python second implementation (CPython 3.12.10, AMD64, Windows; standard library only, `QO-35`). Transcripts are LF-normalized before hashing; every transcript is valid UTF-8 and already LF-only.


A same-binary and same-source rerun reproduced all six digests byte-identically (reproducibility evidence only, never independent reproduction, §4.2).

The binding-level facts, identical in all six transcripts:

- **`declared seed: 84 vectors`**; suite X execution `51/51` at §4.1's five agreement points; both artifacts PASS.
- Header pins **`6/6`** (trace roots, §6.6), **`4/4`** (comparison pins, §6.7), and **`4/4`** (comparison self-test, §6.7).
- **52 trace-root lines** — one `scope = run` plus `X1`–`X51` — byte-identical across implementations and modes.
- `lineage: collision bucket (qir-artifact-cmp-1, entries = 0)` in every normal and F27 run; the F6 fixture block alone reports `entries = 1`, and states `normal mode unchanged: n = 336, entries = 0`.
- Corpus identity: `lineage: corpus identity (corpus, 0.3.0-draft.31, fa72f316c2f3bccc94260bbe6f75abecfe18895ab22a551705301d60070c92b3)` over **1,080,404 LF bytes** (22,151 lines, 0 CR bytes).
- Evidence: `lineage: evidence (n = 336 records, E = 0765cd5e665ae77303b5bcfce21b3d7a686af8726a1e365d2687220dc97dee75)`.
- `lineage: V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)`.
- Run root: `lineage: trace root (qir-trace-root-1, scope = run, n = 336, root = 5c6c0b21fba2df0bff219d01270848038046dcfb33eb509375684e9d57408a79)`.
- Implementation identities (differ by construction, §6.2): Rust `(implementation, 0.3.0-draft.31, 7d204647280a49e03fed16a19fb552e3094b3672f9770d564b37e3c88796bf0c)`; Python `(implementation, 0.3.0-draft.31, f9cf76560da5a263b3f3690a724ff806ed0ab6091c2dd197e788e80f61a9cbff)`.

The two fixture-block digests, recomputed from the transcripts by the §4.4 block rule (SHA-256 over the fixture block's lines, header through the line before its `fixture …: PASS` verdict, LF-joined with a trailing LF, first 16 hex printed):


The `F27` block reads `B_injected = 32 U`, `W_budget = 36 U`, `B_hat(P_f) = 40 U`; control run REFUSED, injected run ADMITTED with the backstop firing at step 4, `charged <= reserved` at equality, and the verdict "validator defect evidenced; containment held". The `F6` block reads the four declared pairs and `collision bucket: entries = 1 (class = digest-collision)`.
[[/CODE]]

#### III.2.f-g0-1-state-root-repair-draft32.md

[[CODE]]
---
layout: default
title: F-G0-1 — The P1 State-Root Repair at v0.3.0-draft.32 (Obj 15 Read-Back Pairs)
description: The committed 6cc1ef1 wave — qir-enc-2 P1 = SHA-256(P2) 32-byte state roots, the six Obj 15 transcript digests, both fixture blocks, and the QO-17 draft.32 interleaved A/B delta
nav_exclude: true
---

# F-G0-1 — The P1 State-Root Repair at v0.3.0-draft.32 (Obj 15 Read-Back Pairs)

Wave record of finding `F-G0-1` under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.32. **The wave is committed at `6cc1ef1`** and repairs the P1 state root of record: the record fields that carry the pre/post configuration now carry the **SHA-256 digest of the canonical P2 configuration** — exactly 32 bytes — instead of configuration bytes in another width. The corpus, the opcode set, the accounting model, and every committed expected byte are otherwise unchanged; QnIR remains `[PENDING]` and no Step-6 opcode is canonized by this record.

## What was emitted

**The commit.** `6cc1ef1` — 10 files, +3,362/−10,956 — of which the semantic content is the three-point repair above plus the corpus's regenerated canonical annotations. The corpus at this state is **`docs/qnir_goldens.md`: 659,880 LF bytes, 14,477 LF lines, 0 CR bytes, SHA-256 `246bc01c3337afb8ba930e3738650e05f1aa412b4d8d495c9b27f05c6bcc1d92`**. `cargo test` passes **4/4** at this state. No glossary, navigation, portal index, or `src/` file is changed by this log.

**The binding-level facts of record**, identical in all six transcripts (Rust implementation identity `9b784dc003ef9e97f37fedfbfa2330b33d59bc146cfa3c0f4f290abed8feb4ab`; Python implementation identity `5b8d600be66f25d7d99c632f3f574e21527f737485b145e71fad28e430aee613`):

- `declared seed: 84 vectors`; suite X execution `51/51` at §4.1's five agreement points; both artifacts PASS.
- header pins `6/6` (trace roots), `4/4` (comparison pins), `4/4` (comparison self-test); 52 published trace-root lines.
- `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`.
- evidence `n = 336 records`, **`E = 5a438424c7988dc2a5f0e771c495b8fa786da8a506e006aa0b90f8c1ca9915da`**.
- **`qir-trace-root-1`, `scope = run`, `n = 336`, `root = 369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866`**.
- fixture blocks: `F27` fuel backstop **`6f6bd8f5ea344b31`**, `F6` collision comparison **`5041b3741ade3b9a`** — each byte-identical between the two artifacts and unchanged from draft.31; both fixtures remain test-only surfaces, never conforming goldens.

**The six Obj 15 read-back pairs.** All six runs exit 0. Transcripts are LF-normalized before hashing; each is valid UTF-8 and already LF-only.


**The `QO-17` Part 1 delta on draft.32.** The measurement is an interleaved same-session A/B (draft.31 binary → draft.32 → draft.31 → draft.32, two runs each) because the host's between-session drift is larger than the P1 effect. Over the 49 vectors with `Work(E) > 0`:


The published draft.31 baseline is median 2,525.0 / max 4,994.4 ns/U; the saved draft.32 JSON records median 2,503.1 (−0.9 %) and a distribution max of 6,606.2 whose source is a low-`U` stall, not a structural regression. Restricted to high-work vectors (`U ≥ 68`) the maxima are draft.31 5,600 / 6,333 versus draft.32 4,791 / 5,275. Same-binary repeats moved individual vectors by as much as 2.5×, so the median-level +4.7 % is the defensible delta; the per-record digest adds a fixed 1–15 µs while the smaller record fields (32-byte roots rather than full configurations) return part of it through `encode_record` and `trace_root`. Full per-vector figures and the harness protocol are in the companion harness artifacts (`container_boc_results.json`, SHA-256 `abed3d5d05788ed7427ebb4df090cbf4265809ee306e72b8e8caa9e73605f36c`, and `run_stdout.txt`), whose container trade study is recorded in `docs/logs/qnir/step6-1-container-boc-study.md`.

## The evidence (runs of record, this commit state)

Both artifacts were rebuilt and rerun against the committed corpus at `6cc1ef1`: the Rust reference interpreter (`rustc 1.99.1`, `x86_64-pc-windows-msvc`, release profile, standard library only) and the Python second implementation (CPython 3.12.10, AMD64, Windows; standard library only). Invocation is `qnir <path to docs/qnir_goldens.md> [--fixture F27|--fixture F6]` and its Python counterpart; all six exit 0, every normal-mode result is PASS, and both fixture verdict lines are PASS. The corpus identity was recomputed from the committed bytes before the runs: 659,880 bytes, 0 CR, digest `246bc01c…bcc1d92` as declared above. The two fixture-block digests were recomputed from the transcripts by the §4.4 block rule and agree across implementations. A same-binary and same-source rerun reproduced all six transcript digests byte-identically (reproducibility evidence only, never independent reproduction, §4.2).

The `QO-17` measurement harness is a standalone crate in `C:\Users\dd_an\AppData\Local\Temp\opencode\container-boc\`, compiled with `rustc 1.99.1` `--release --offline` (LTO, one codegen unit), with 2 warm-up and 9 measured trials. It copies the committed modules read-only; it never writes into the repository. Its Part A fidelity gate is the `qir-trace-root-1` run root above, recomputed from 336 records with **0** root-width violations at every repetition. `git status --porcelain` was empty at the start and at the end of all runs.
[[/CODE]]

#### III.2.step6-1-container-boc-study.md

[[CODE]]
---
layout: default
title: Step 6.1 — The Container BOC Trade Study (Measured Ladder Pass, v0.3.0-draft.32)
description: The standalone native-Windows measurement of ChunkedFlat, HAMT, RRBTree and MerkleBTree under one enforced rule set — Q(array) and Q(table) row measurements, the BOC Stage-1/Stage-2 adjudication, and the deferred qir-enc-3 opcode contract
nav_exclude: true
---

# Step 6.1 — The Container BOC Trade Study (Measured Ladder Pass, v0.3.0-draft.32)

Measurement record of the Step-6.1 Container BOC trade study under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) v0.4.2, run read-only against the committed `6cc1ef1` (v0.3.0-draft.32) canon. **No repository file was created, edited, or deleted by the measurement run**: the harness is a standalone crate in `C:\Users\dd_an\AppData\Local\Temp\opencode\container-boc\`, and `git status --porcelain` was empty at its start and end. The study changes no QnIR semantics, declares no opcode, and seals no structure: it discharges the **row-measurement** and **BOC-selection** parts of Ladder Steps 2–3 for two niches, leaving sealing (Step 5) and the core-extension admission to a later wave.

## What was emitted

**The artifact.** `container_boc_results.json` (25,380 bytes, SHA-256 **`abed3d5d05788ed7427ebb4df090cbf4265809ee306e72b8e8caa9e73605f36c`**) and its 7,886-byte CSV companion, both in the temp harness directory. They hold 28 measurement rows (16 dense + 12 sparse) and 12 enforced-rule checks, all PASS:

- leaf-hash multiset identity at every `(niche, N)` — 8 checks, PASS across 4 structures dense / 3 sparse;
- canonical CBOR identity at `N = 256` and `4096` — 4 checks, PASS with digests `11ed127f…d5c1b` (dense 256), `60d0ddb1…7ab7f3` (sparse 256), `9f2c068b…80d68` (dense 4096), `35434ff9…7c54e3` (sparse 4096). The CBOR artifact is structure-independent: 9,195 / 11,003 bytes at 256 and 151,275 / 176,123 bytes at 4096.
- all 28 rows have `len_ok = true` and `values_ok = true` (ascending fold covering the identical key set).

Toolchain and host: `rustc 1.99.1` (`x86_64-pc-windows-msvc`), release, LTO, one codegen unit, offline build; Intel Core i7-8565U (4c/8t, L1d 128 KiB, L2 1 MiB, L3 8 MiB, 15.84 GiB RAM), Windows 11. Timing: 2 warm-up + 9 trials; cycles at the declared 1.80 GHz figure; `ABOVE_NORMAL_PRIORITY_CLASS` and thread affinity to core 2. Small-`N` folds are repeated (`reps = 65536/n`, clamped 1..2048) to clear the ~100 ns timer floor; the folding penalty of HAMT's required sort is included in its numbers, not hidden.

**Dense positional niche — `Q(array)`, keys `0..N`.**


**Sparse keyed niche — `Q(table)`, keys `i * 0x9E3779B97F4A7C15`.**


SHA-256 calls per `get`: 0 for ChunkedFlat, RRBTree, and MerkleBTree; 1 per lookup for HAMT (the key digest) — the reason its 500–670 ns lookups are flat in `N`. Update and batch calls are SHA-bound throughout (set 6 → 30 calls, batch 321 → 1,920 at the largest size), at roughly 0.6–0.75 µs per portable digest. The batch gives no per-item amortization: `batch ns/item` equals `set med ns` within a few percent everywhere, because every update still rehashes its root-to-leaf path.

**BOC Stage-1/Stage-2 adjudication (v0.4.2).** At **Stage 1** all four candidates pass the sealed-gateway requirement `ImplicitDeps(c) = 0`: the enforced rules are identical and declared, and the 12/12 identity checks show no undeclared serialization or hash dependency. At **Stage 2**, over the common workload above, the declared priority permutation is applied to the lexicographic key with Efficiency first. The measured ns/cycles, SHA-call, and allocation surfaces below are recorded as diagnostic measurements at execution boundaries (§10.6); they are **not** presented as a fabricated `MEASURED ΔW_worst` in `U`, whose container-opcode cost model belongs to `qir-enc-3` (Wave 4B). Their `ΔW_worst` value therefore remains `UNKNOWN` in the Tier-2 key sense. Consistent with §8.3 frontier reduction, each rejected candidate is strictly dominated on the recorded efficiency component at the decisive sizes, which makes its later components non-decision-affecting; the outcome is an explicitly recorded adjudication, not canonization.

- **`Q(array) → RRBTree-16` (selected).** Lowest lookup at every size (1.6 → 15.5 ns median; 2.9 → 27.9 cycles at 1.80 GHz), lowest persistent update (4,093.8 → 15,218.8 ns; 6 → 21 SHA calls; 1,728 → 5,760 alloc B), lowest batch (15.6 µs/item at 65,536; 1,344 SHA calls), tied-best fold (1.3 → 2.1 ns/element), smallest footprint (7,549,656 B at 65,536, inside L3; proof 554 B). At N = 4096 the CBOR artifact is the common 151,275 B.
- **`Q(table) → MerkleBTree-16` (selected).** Best keyed lookup (6.4 → 54.8 ns; 11.5 → 98.6 cycles), best set (5,512.5 → 16,956.2 ns; 8 → 23 SHA calls; 2,005 → 6,474 alloc B), best batch (5,167.2 → 16,882.8 ns/item; 485 → 1,449 SHA calls), best deterministic fold (1.8 → 2.6 ns/element), and 8,445,528 B at 65,536 (proof 584 B; CBOR 176,123 B at 4096). RRBTree is not admissible in this niche: its positional `0..N` key space cannot carry the sparse key set.

**Ladder Step 3 rejection record — findings, not erasures (§3 step 3).**

- **`HAMT` rejected.** Lookup is **30×–300×** slower than the dense winner across `N` (490.9 vs 1.6 ns at N=16 ≈ 307×; 516.6 vs 3.1 at 256 ≈ 167×; 566.1 vs 4.8 at 4096 ≈ 118×; 662.1 vs 15.5 at 65,536 ≈ 43×; against MerkleBTree in `Q(table)` it is 12×–80× slower), entirely from one mandatory SHA-256 key digest plus trie traversal. Its deterministic fold is ~50× worse at scale (130.4 vs 2.6 ns/element at N=65,536; 8,547,000 ns versus 172,800 ns), because key order must be restored by sorting. Its footprint, inflated by per-bitmap commitments and 32-byte child hashes, reaches **18.85–18.90 MB**, spilling past the 8 MiB L3 into DRAM (≈2.2–2.5× the selected structures), while its proof grows to 672 B in the sparse niche. Its one nominal advantage — unordered insertion — buys nothing in either declared workload.
- **`ChunkedFlat` rejected.** From `N ≥ 4096` its chunk-clone update path copies 32 keys, 32 values, and the chunk's Merkle levels: 5,808 → 7,056 alloc B per update (vs RRBTree 4,416 → 5,760), set 15,275 → 21,194 ns (vs 11,444 → 15,219), batch 15.16 → 20.63 µs/item (vs 10.94 → 15.59), and lookup 21.7 → 58.1 ns (vs 4.8 → 15.5). Its fold is only 1.2–1.3× worse, but that is not enough to overcome the update/copy penalty on either common workload. It remains revisitable with a different chunk size; the measured value at chunk = 32 is what is rejected.

## The evidence (runs of record, this commit state)

The harness ran against the read-only canon at `6cc1ef1`; the Part A fidelity gate recomputed the `qir-trace-root-1` run root `369ada2b…5866` from 336 records with zero root-width violations, matching the companion log `docs/logs/qnir/f-g0-1-state-root-repair-draft32.md`. The saved JSON parses under `json.load`, carries `part_a` (51 vectors, 49 scored) and `part_b` (28 rows, 12 checks, `all_checks_ok = true`), and its fingerprint is the `abed3d5d…05f36c` value above. Same-binary reruns reproduced the structural verdicts and identities; the absolute timings are a single-host calibration and inherit the 1.80 GHz cycle estimate, the ~100 ns timer quantum, and uncontrolled thermal/cache-set state. Portable SHA-256 dominates the update rows, so absolute constants do not transfer to SHA-NI silicon; the orderings do. The `QO-17` interleaved A/B accompanying the study is recorded in the companion log: +4.7 % aggregate median `ns/U`, `X32` 4,434.7 → 4,107.6 ns/U, `X2` +1.7 µs.
[[/CODE]]

#### III.2.step6-1-container-artifact-share.md

[[CODE]]
---
layout: default
title: Step 6.1 — The Container Artifact/Validator Share (v0.3.0-draft.34)
description: The qir-enc-3 / qir-prof-ext-containers-1 artifact share — thirteen container opcodes in both implementations, the byte-identical C1–C6 fixture block, and core qir-enc-2 invariance
nav_exclude: true
---

# Step 6.1 — The Container Artifact/Validator Share (v0.3.0-draft.34)

Wave record of the Step-6.1 Share-2 artifact/validator increment under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.34. **Share 2 is DISCHARGED**: both implementations accept `qir-enc-3` / `qir-prof-ext-containers-1`, implement the thirteen extension opcodes and value tag `5`, and reproduce every `C1`–`C6` expectation of `docs/qnir_containers.md` §7.6 in a byte-identical UTF-8/LF fixture block (SHA-256[:16] `2c96b152f38a7d44`). The golden corpus and all core encodings are untouched; QnIR remains `[PENDING]`.

## What was emitted

**Rust (`src/qnir/`).** `Cargo.toml` version `0.3.0-draft.34`; new `src/qnir/src/containers.rs` implements `TAG_CONTAINER = 5` subtypes `ARRAY_RRB16 = 1` / `TABLE_MBTREE16 = 2`, copy-on-write persistent pair vectors, the §7.3.1 canonical `[5, subtype, cap_bnd, elem_bnd, pairs]` encoding, `0x00` leaf and `0x20` pairwise SHA-256 commitments with odd promotion and `SHA-256(0x02)` for the empty root, the static `depth_16(cap_bnd)` charge function, the trip-ceiling `B_hat` DP with `TBL_GET`'s fallback edge joined exactly as `CLOAD`'s, and the eight declared fixture runs. `main.rs` accepts `--fixture C` and prints the deterministic block. Two new `#[test]` functions check `qir-enc-2` rejection of tags `17`–`29` and container bounds/batch edge cases.

**Python (`src/qnir-py/qnir_ref.py`).** `IMPL_VERSION = "0.3.0-draft.34"`; an independently written mirror of the same declared engine; `--fixture C` renders the identical block. Standard library only, redirected stdout pinned to UTF-8/LF.

**The fixture block of record** (byte-identical across Rust and Python, 1,923 bytes, 9 lines):

```text
fixture C1 : B_hat=94 U Work(E)=94 U n=16 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C2 : B_hat=94 U Work(E)=94 U n=16 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C3 : B_hat=44 U Work(E)=44 U n=5 outcome=SUCCESS len=0 miss=1/1 root=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C4a : B_hat=72 U Work(E)=72 U n=10 outcome=SUCCESS len=2 miss=0/0 root=e4c1626b02c04db1eda6f7aca69273a84c875caa8106d6a3e3a96fda1be044d2 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C4b : B_hat=72 U Work(E)=68 U n=9 outcome=EXPRESSION_TOO_COMPLEX len=0 miss=0/0 root=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C5 : B_hat=58 U Work(E)=54 U n=6 outcome=EXPRESSION_TOO_COMPLEX len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C6a : B_hat=54 U Work(E)=54 U n=7 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C6b : B_hat=54 U Work(E)=54 U n=7 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C: PASS — 8/8 declared runs reproduce under qir-enc-3 / qir-prof-ext-containers-1 (spec OK)
```

Block SHA-256: `2c96b152f38a7d4418ccd3856232064c3c96bfe2a9b119e7f96179db184562be`; first 16 hex: `2c96b152f38a7d44`. The `C1`/`C2` folds traverse with `ARR_LEN`/`ARR_GET` and `TBL_LEN`/`TBL_AT` under `CMP`/`BR` and one trip-ceiling back-edge; `C3` births and consumes exactly one Tier-O miss token; `C4b`/`C5` terminate with the existing `EXPRESSION_TOO_COMPLEX` token; `C6a`/`C6b` share the empty root `dbc1b4c9…d986` and the one-member root `3874f4bb…c8d4`; every run asserts a 32-byte P1 state root.

## The evidence (runs of record, this commit state)

`cargo test --offline`: **6 passed; 0 failed** (the four pre-existing tests plus `qir_enc2_rejects_extension_opcodes` and `container_bounds_and_batch_edges`). `cargo build --release --offline` succeeds with `rustc 1.99.1` (`x86_64-pc-windows-msvc`); Python is CPython 3.12.10, standard library only. All eight runs exit 0: normal 84/84, `--fixture F27`, `--fixture F6`, and `--fixture C` for each artifact.


Core invariance is byte-exact outside the expected implementation-identity line: corpus `246bc01c3337afb8ba930e3738650e05f1aa412b4d8d495c9b27f05c6bcc1d92`, `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`, `E = 5a438424c7988dc2a5f0e771c495b8fa786da8a506e006aa0b90f8c1ca9915da`, `n = 336`, `scope = run` root `369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866`, F27 block `6f6bd8f5ea344b31`, F6 block `5041b3741ade3b9a` — all unchanged from `6cc1ef1`. The normal-mode transcripts differ from the draft.32 transcripts only in the implementation-identity line (Rust `9d7608e1…e484d0`, Python `27e6dc2b…2f4d4a9`), as the new source bytes require under §6.2. The corpus was read-only; `git status --porcelain` reflects only the three budgeted implementation/spec files plus this log and the index bullet.
[[/CODE]]

#### III.2.step6-2-call-trade-study.md

[[CODE]]
---
layout: default
title: Step 6.2 — The Call Trade Study (QO-21 / Obj 6, native Windows)
description: The read-only native-Windows measurement of call-versus-inline storage, compositional B_hat scaling, P1 state-root runtime, and sub-budget/failure conservation, with the BOC verdict retaining bounded acyclic calls under qir-enc-4
nav_exclude: true
---

# Step 6.2 — The Call Trade Study (QO-21 / Obj 6, native Windows)

Measurement record of the Step-6.2 call trade study under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. The harness is a standalone crate at `C:\Users\dd_an\AppData\Local\Temp\opencode\call-trade-study\`, built with `rustc 1.99.1` (`x86_64-pc-windows-msvc`, `--release --offline`, LTO, codegen-units=1), thread pinned to core 2, 2 warm-up + 9 measured trials, cycles at 1.80 GHz. It models real §2.6 64-bit word encodings, deterministic CBOR envelopes, `P1 = SHA-256(P2)` 32-byte state roots per transition (including serialized frame-stack bytes in P2 for Candidate C), and `qir-trace-root-1`. **No repository file was created, edited, or deleted by the run**; `git status --porcelain` was empty at the start and end. The record changes no QnIR semantics and seals no extension.

## What was emitted

The artifact of record is `call_trade_results.json`, **60,607 bytes, SHA-256 `ef1857484c21b21d040a4b3cc41999418bbc2c00f6adacf53be63d0efd4645ab`**, plus a 26,204-byte CSV companion and the full printed transcript `run_stdout.txt`. It contains 30 storage rows, 5 surrogate rows, 270 execution rows, and 4 deterministic invariants.

**1. Storage / CBOR break-even.** Exact §2.6 word lengths measured: CONST 72 B, ADD 64 B, MUL 64 B, CMP 72 B, JMP 32 B; study-side candidate OP_CALL 48 B, OP_RET 16 B. Across `P ∈ {2,4,8,16,32,64}` and `m ∈ {1,2,4,8,16}`, the empirical break-even is **`P* = 2` at every `m ≥ 2`** — above the theoretical `P > m/(m−1)` curve (2.000 at m=2, 1.333 at m=4, 1.143 at m=8, 1.067 at m=16), because the real call form also carries a 16 B callee header and a 16 B RET. At `m = 1` call form is always larger (e.g. 4,354 B → 4,437 B at P=64). At `P=64, m=16` inline envelopes are 68,796 B versus 5,157 B call-form.

**2. `B_hat` scaling and `S_max = 4096`.** With a 4-block callee containing `k = 2` back-edges at `trip_ceiling = 1`, the inline structural product is `4m · 2^(2m)`. Inline is admitted for `m = 1` (16 states), `m = 2` (128), `m = 4` (4,096); **the exact inline refusal first occurs at `m = 5`** (20,480 > 4,096), with `m = 6/8` refused in ~4 ns after the gate. Compositional `B_hat` computes the callee once (16 states) plus `O(1)` per call site, giving 17/18/20/22/24 states for `m = 1/2/4/6/8`, bounds 96/100/108/116/124 U, and **remains admitted throughout**. Admitted inline DP latency is ~4.6–5.9 µs; compositional evaluation is ~0.39 ns per repetition.

**3. Execution / P1 runtime.** Across `P ∈ {4,16,64}`, `m ∈ {2,8}`, `F ∈ {4,16,64}`, `C ∈ {1,4,8}`, and modes Inline / CallB (no frame bytes in P2) / CallC (frame bytes in P2) / MemoB / MemoC, the median CallC overhead by P is **P=4: +336%, P=16: +59%, P=64: −14%** → **`P_exec* = 64`** (first P below the 10% threshold). The dominant variable is depth: Candidate C's P2 grows from 111 B at `C=1` to **5,512 B at `C=8, F=64`**, so the extra SHA-256 preimage work dominates at high depth even for large P. Memoized verification reuse wins broadly: at `m=8, C=1`, MemoC is 26–79% faster than Inline; at `P=64, m=8, C=8` it is 20% faster while dynamic calls are still +171%.

**4. Failure / conservation.** All **270/270** execution rows satisfy `reserved = charged + refunded`. Four deterministic checks pass: inner `EXPRESSION_TOO_COMPLEX` and `DOMAIN_ERROR` each seal fatally at their own transition with `reserved=52, charged=28, refunded=24` and no outer-refund corruption; unconsumed `STATUS_NEG_EVEN_ROOT` and `STATUS_LOOKUP_MISS` propagate across `OP_RET` with outcome `NO_SOLUTION`, no new token types; a successful fallback consumes the miss token while the semantic token stays outstanding.

**BOC verdict.** Stage 1: the candidate carries no undeclared dependency when the callee table, frame stack, sub-budget verbs, and token rules are declared — `ImplicitDeps = 0`. Stage 2: unbounded recursion and implicit stack growth are rejected (unbounded state violates Obj 2 and F26's trip-ceiling certification); direct inlining loses at scale (68,796 B vs 5,157 B at P=64, m=16, plus `S_max` refusal at m=5). The retained candidate is **bounded acyclic `OP_CALL`/`OP_RET` under `qir-enc-4`** with a statically acyclic callee graph, `C_max = 8`, `F_max = 64` (512 B ceiling), compositional `B_hat`, and exact §3.3 sub-budget settlement. This is a retained candidate for specification, not a canonized opcode.

## The evidence (runs of record, this commit state)

The harness exits 0; `call_trade_results.json` parses with 30 + 5 + 270 rows and 4 checks, all conservation flags true. The measurement uses portable SHA-256 (QO-33) and a single host, so absolute constants are not portability claims; the orderings (storage, state-cardinality, depth-driven P2 bloat, memoization) are the transferable findings. The `V.encoding` inspection of `src/qnir/src/insn.rs`, `src/qnir/src/containers.rs`, `src/qnir/src/main.rs`, and `src/qnir-py/qnir_ref.py` confirmed the current artifacts only store the encoding text and dispatch extension behavior by CLI fixture, not by `V.encoding`; the specification share therefore closes that gap by requiring `decode_envelope`/`dec_insn` to reject tags `30`–`31` and envelope key `9` outside `qir-enc-4`.
[[/CODE]]

#### III.2.step6-2-call-artifact-share.md

[[CODE]]
---
layout: default
title: Step 6.2 — The Call Artifact/Validator Share (v0.3.0-draft.36)
description: The qir-enc-4 artifact share — mandatory V.encoding gating, the byte-identical K1–K6 fixture block, and core/container lineage invariance across all ten runs
nav_exclude: true
---

# Step 6.2 — The Call Artifact/Validator Share (v0.3.0-draft.36)

Wave record of the Step-6.2 Share-2 artifact/validator increment under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.36. **Share 2 is DISCHARGED**: both implementations accept `qir-enc-4`/`qir-prof-ext-calls-1`, enforce the mandatory `V.encoding` gate, and reproduce every `K1`–`K6` expectation of `docs/qnir_calls.md` §8.6 in a byte-identical UTF-8/LF fixture block (SHA-256[:16] `e83189cb00a56497`). The golden corpus and all core/container encodings are untouched; QnIR remains `[PENDING]`.

## What was emitted

**Rust (`src/qnir/`).** `Cargo.toml` version `0.3.0-draft.36`; new `src/qnir/src/calls.rs` implements `version_bounds`, `validate_version`, `validate_envelope_cbor` and `validate_block_tags` (the mandatory dispatch gate), the K1–K6 engine, and two `#[test]` functions (`version_gating_is_exact`, `k6_static_rejections`). `insn.rs` gains `OP_CALL = 30`/`OP_RET = 31` and their §2.6 decoder arms; `main.rs` wires the gate into the envelope validator and accepts `--fixture K`.

**Python (`src/qnir-py/qnir_ref.py`).** `IMPL_VERSION = "0.3.0-draft.36"`; an independently written mirror of the gate and the K-fixture engine; `--fixture K` renders the identical block. Standard library only, redirected stdout pinned to UTF-8/LF.

**The fixture block of record** (byte-identical across Rust and Python, 760 bytes, 7 lines):

```text
fixture K1 : B_hat=50 U Work(E)=50 U n=6 outcome=SUCCESS tokens=0/0 reserve=50 charged=50 refund=0 p1=32B OK
fixture K2 : B_hat=76 U Work(E)=76 U n=11 outcome=SUCCESS tokens=0/0 reserve=76 charged=76 refund=0 p1=32B OK
fixture K3 : B_hat=356 U Work(E)=260 U n=51 outcome=SUCCESS tokens=0/0 reserve=356 charged=260 refund=96 p1=32B OK
fixture K4 : B_hat=44 U Work(E)=30 U n=3 outcome=DOMAIN_ERROR|EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=44 charged=30 refund=14 p1=32B OK
fixture K5 : B_hat=54 U Work(E)=54 U n=7 outcome=NO_SOLUTION tokens=1/1 reserve=54 charged=54 refund=0 p1=32B OK
fixture K6 : K6a=PASS K6b=PASS K6c=PASS K6d=PASS B_hat=null Work(E)=0 n=0
fixture K: PASS — 6/6 declared fixtures reproduce under qir-enc-4 / qir-prof-ext-calls-1 (spec OK)
```

Block SHA-256: `e83189cb00a564974381a828340d9ddc7b527bfd8a8beb65e6a7ac3e8f95ebcd`; first 16 hex: `e83189cb00a56497`. The frame-chain check uses `stack_root_0 = SHA-256(0x02)` and `stack_root_{d+1} = SHA-256(0x30 || stack_root_d || suspended_frame_bytes)`; every run asserts a 32-byte P1 root and exact §3.3 conservation (`reserved = charged + refunded`).

## The evidence (runs of record, this commit state)

`cargo test --offline`: **8 passed; 0 failed** (the four pre-existing tests, the two container tests, and the two new call tests). `cargo build --release --offline` succeeds with `rustc 1.99.1`; Python is CPython 3.12.10. All ten runs exit 0: normal 84/84, `--fixture F27`, `--fixture F6`, `--fixture C`, and `--fixture K` for each artifact. Core lineage is byte-exact outside the implementation-identity line (which necessarily changes with the version bump): corpus `246bc01c…bcc1d92`, `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`, `E = 5a438424…15da`, `n = 336`, `scope = run` root `369ada2b…5866`, F27 block `6f6bd8f5ea344b31`, F6 block `5041b3741ade3b9a`, C block `2c96b152f38a7d44` — all unchanged from `4106b88`.


The corpus was read-only; no repository file outside this log, the index bullet, and the two completion notes was edited.
[[/CODE]]

#### III.2.container_boc_results.json

[[CODE]]
{"part_a":{"artifact":"6cc1ef1","vectors":51,"records":336,"bad_root_widths":0,"trace_root":"369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866","trace_root_expected":"369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866","trace_root_match":true,"aggregate_ns_per_u":{"min":1631.250,"median":2503.125,"p95":5038.235,"max":6606.250},"aggregate_cycles_per_u":{"min":2936.2,"median":4505.6,"p95":9068.8,"max":11891.2},"baseline_draft31":{"median_ns_per_u":2525.0,"max_ns_per_u":4994.4},"rows":[{"id":"X1","work_u":60,"n":10,"envelope_bytes":1682,"ns":{"min":208200.000,"median":262300.000,"p95":434600.000,"max":434600.000},"ns_per_u":4371.6667,"cycles_per_u":7869.00},{"id":"X2","work_u":0,"n":1,"envelope_bytes":1682,"ns":{"min":32900.000,"median":33400.000,"p95":34700.000,"max":34700.000},"ns_per_u":null,"cycles_per_u":null},{"id":"X3","work_u":32,"n":3,"envelope_bytes":234,"ns":{"min":54400.000,"median":55400.000,"p95":56100.000,"max":56100.000},"ns_per_u":1731.2500,"cycles_per_u":3116.25},{"id":"X4","work_u":40,"n":5,"envelope_bytes":314,"ns":{"min":85600.000,"median":87000.000,"p95":108000.000,"max":108000.000},"ns_per_u":2175.0000,"cycles_per_u":3915.00},{"id":"X5","work_u":28,"n":3,"envelope_bytes":305,"ns":{"min":52500.000,"median":53300.000,"p95":53900.000,"max":53900.000},"ns_per_u":1903.5714,"cycles_per_u":3426.43},{"id":"X6","work_u":32,"n":3,"envelope_bytes":355,"ns":{"min":55400.000,"median":56000.000,"p95":56600.000,"max":56600.000},"ns_per_u":1750.0000,"cycles_per_u":3150.00},{"id":"X7","work_u":56,"n":10,"envelope_bytes":466,"ns":{"min":175100.000,"median":178100.000,"p95":215900.000,"max":215900.000},"ns_per_u":3180.3571,"cycles_per_u":5724.64},{"id":"X8","work_u":32,"n":3,"envelope_bytes":235,"ns":{"min":59600.000,"median":62900.000,"p95":70300.000,"max":70300.000},"ns_per_u":1965.6250,"cycles_per_u":3538.12},{"id":"X9","work_u":36,"n":4,"envelope_bytes":372,"ns":{"min":73400.000,"median":75700.000,"p95":125000.000,"max":125000.000},"ns_per_u":2102.7778,"cycles_per_u":3785.00},{"id":"X10","work_u":68,"n":13,"envelope_bytes":724,"ns":{"min":258600.000,"median":263000.000,"p95":299300.000,"max":299300.000},"ns_per_u":3867.6471,"cycles_per_u":6961.76},{"id":"X11","work_u":40,"n":5,"envelope_bytes":492,"ns":{"min":94600.000,"median":95100.000,"p95":111600.000,"max":111600.000},"ns_per_u":2377.5000,"cycles_per_u":4279.50},{"id":"X12","work_u":72,"n":14,"envelope_bytes":844,"ns":{"min":291200.000,"median":379800.000,"p95":742800.000,"max":742800.000},"ns_per_u":5275.0000,"cycles_per_u":9495.00},{"id":"X13","work_u":28,"n":3,"envelope_bytes":404,"ns":{"min":66900.000,"median":71900.000,"p95":106000.000,"max":106000.000},"ns_per_u":2567.8571,"cycles_per_u":4622.14},{"id":"X14","work_u":40,"n":5,"envelope_bytes":388,"ns":{"min":89300.000,"median":90100.000,"p95":106900.000,"max":106900.000},"ns_per_u":2252.5000,"cycles_per_u":4054.50},{"id":"X15","work_u":72,"n":14,"envelope_bytes":740,"ns":{"min":272100.000,"median":277300.000,"p95":302500.000,"max":302500.000},"ns_per_u":3851.3889,"cycles_per_u":6932.50},{"id":"X16","work_u":44,"n":6,"envelope_bytes":508,"ns":{"min":109200.000,"median":111000.000,"p95":148300.000,"max":148300.000},"ns_per_u":2522.7273,"cycles_per_u":4540.91},{"id":"X17","work_u":76,"n":15,"envelope_bytes":860,"ns":{"min":300100.000,"median":307300.000,"p95":344800.000,"max":344800.000},"ns_per_u":4043.4211,"cycles_per_u":7278.16},{"id":"X18","work_u":32,"n":4,"envelope_bytes":420,"ns":{"min":79100.000,"median":80100.000,"p95":81600.000,"max":81600.000},"ns_per_u":2503.1250,"cycles_per_u":4505.62},{"id":"X19","work_u":36,"n":4,"envelope_bytes":372,"ns":{"min":75200.000,"median":75500.000,"p95":76900.000,"max":76900.000},"ns_per_u":2097.2222,"cycles_per_u":3775.00},{"id":"X20","work_u":68,"n":13,"envelope_bytes":724,"ns":{"min":265100.000,"median":342600.000,"p95":473700.000,"max":473700.000},"ns_per_u":5038.2353,"cycles_per_u":9068.82},{"id":"X21","work_u":40,"n":5,"envelope_bytes":492,"ns":{"min":100200.000,"median":105000.000,"p95":188600.000,"max":188600.000},"ns_per_u":2625.0000,"cycles_per_u":4725.00},{"id":"X22","work_u":72,"n":14,"envelope_bytes":844,"ns":{"min":291400.000,"median":314200.000,"p95":329800.000,"max":329800.000},"ns_per_u":4363.8889,"cycles_per_u":7855.00},{"id":"X23","work_u":28,"n":3,"envelope_bytes":404,"ns":{"min":64500.000,"median":66100.000,"p95":70400.000,"max":70400.000},"ns_per_u":2360.7143,"cycles_per_u":4249.29},{"id":"X24","work_u":40,"n":5,"envelope_bytes":388,"ns":{"min":88000.000,"median":89200.000,"p95":91700.000,"max":91700.000},"ns_per_u":2230.0000,"cycles_per_u":4014.00},{"id":"X25","work_u":72,"n":14,"envelope_bytes":740,"ns":{"min":268300.000,"median":275300.000,"p95":285900.000,"max":285900.000},"ns_per_u":3823.6111,"cycles_per_u":6882.50},{"id":"X26","work_u":44,"n":6,"envelope_bytes":508,"ns":{"min":111900.000,"median":113100.000,"p95":159600.000,"max":159600.000},"ns_per_u":2570.4545,"cycles_per_u":4626.82},{"id":"X27","work_u":76,"n":15,"envelope_bytes":860,"ns":{"min":298000.000,"median":299100.000,"p95":304200.000,"max":304200.000},"ns_per_u":3935.5263,"cycles_per_u":7083.95},{"id":"X28","work_u":32,"n":4,"envelope_bytes":420,"ns":{"min":132100.000,"median":211400.000,"p95":277300.000,"max":277300.000},"ns_per_u":6606.2500,"cycles_per_u":11891.25},{"id":"X29","work_u":36,"n":4,"envelope_bytes":372,"ns":{"min":76900.000,"median":142600.000,"p95":234400.000,"max":234400.000},"ns_per_u":3961.1111,"cycles_per_u":7130.00},{"id":"X30","work_u":68,"n":13,"envelope_bytes":724,"ns":{"min":270200.000,"median":272100.000,"p95":284900.000,"max":284900.000},"ns_per_u":4001.4706,"cycles_per_u":7202.65},{"id":"X31","work_u":40,"n":5,"envelope_bytes":492,"ns":{"min":98400.000,"median":99900.000,"p95":199500.000,"max":199500.000},"ns_per_u":2497.5000,"cycles_per_u":4495.50},{"id":"X32","work_u":72,"n":14,"envelope_bytes":844,"ns":{"min":281800.000,"median":282300.000,"p95":293900.000,"max":293900.000},"ns_per_u":3920.8333,"cycles_per_u":7057.50},{"id":"X33","work_u":28,"n":3,"envelope_bytes":404,"ns":{"min":65200.000,"median":66000.000,"p95":67300.000,"max":67300.000},"ns_per_u":2357.1429,"cycles_per_u":4242.86},{"id":"X34","work_u":40,"n":5,"envelope_bytes":388,"ns":{"min":89400.000,"median":90400.000,"p95":113500.000,"max":113500.000},"ns_per_u":2260.0000,"cycles_per_u":4068.00},{"id":"X35","work_u":72,"n":14,"envelope_bytes":740,"ns":{"min":267800.000,"median":281600.000,"p95":370100.000,"max":370100.000},"ns_per_u":3911.1111,"cycles_per_u":7040.00},{"id":"X36","work_u":44,"n":6,"envelope_bytes":508,"ns":{"min":121400.000,"median":188600.000,"p95":277400.000,"max":277400.000},"ns_per_u":4286.3636,"cycles_per_u":7715.45},{"id":"X37","work_u":76,"n":15,"envelope_bytes":860,"ns":{"min":294800.000,"median":297900.000,"p95":344800.000,"max":344800.000},"ns_per_u":3919.7368,"cycles_per_u":7055.53},{"id":"X38","work_u":32,"n":4,"envelope_bytes":420,"ns":{"min":78400.000,"median":80200.000,"p95":81900.000,"max":81900.000},"ns_per_u":2506.2500,"cycles_per_u":4511.25},{"id":"X39","work_u":32,"n":3,"envelope_bytes":355,"ns":{"min":52100.000,"median":52700.000,"p95":151200.000,"max":151200.000},"ns_per_u":1646.8750,"cycles_per_u":2964.38},{"id":"X40","work_u":32,"n":3,"envelope_bytes":355,"ns":{"min":51400.000,"median":52200.000,"p95":85100.000,"max":85100.000},"ns_per_u":1631.2500,"cycles_per_u":2936.25},{"id":"X41","work_u":24,"n":2,"envelope_bytes":427,"ns":{"min":41500.000,"median":42500.000,"p95":55300.000,"max":55300.000},"ns_per_u":1770.8333,"cycles_per_u":3187.50},{"id":"X42","work_u":36,"n":4,"envelope_bytes":491,"ns":{"min":69200.000,"median":69300.000,"p95":70800.000,"max":70800.000},"ns_per_u":1925.0000,"cycles_per_u":3465.00},{"id":"X43","work_u":44,"n":6,"envelope_bytes":426,"ns":{"min":106500.000,"median":107800.000,"p95":114200.000,"max":114200.000},"ns_per_u":2450.0000,"cycles_per_u":4410.00},{"id":"X44","work_u":24,"n":2,"envelope_bytes":289,"ns":{"min":39300.000,"median":39800.000,"p95":40300.000,"max":40300.000},"ns_per_u":1658.3333,"cycles_per_u":2985.00},{"id":"X45","work_u":52,"n":8,"envelope_bytes":594,"ns":{"min":154500.000,"median":163600.000,"p95":204600.000,"max":204600.000},"ns_per_u":3146.1538,"cycles_per_u":5663.08},{"id":"X46","work_u":36,"n":4,"envelope_bytes":386,"ns":{"min":69200.000,"median":69700.000,"p95":84900.000,"max":84900.000},"ns_per_u":1936.1111,"cycles_per_u":3485.00},{"id":"X47","work_u":40,"n":5,"envelope_bytes":474,"ns":{"min":91400.000,"median":92100.000,"p95":92400.000,"max":92400.000},"ns_per_u":2302.5000,"cycles_per_u":4144.50},{"id":"X48","work_u":28,"n":3,"envelope_bytes":394,"ns":{"min":59300.000,"median":59700.000,"p95":74500.000,"max":74500.000},"ns_per_u":2132.1429,"cycles_per_u":3837.86},{"id":"X49","work_u":44,"n":6,"envelope_bytes":442,"ns":{"min":121700.000,"median":156200.000,"p95":254400.000,"max":254400.000},"ns_per_u":3550.0000,"cycles_per_u":6390.00},{"id":"X50","work_u":0,"n":1,"envelope_bytes":355,"ns":{"min":29900.000,"median":37800.000,"p95":95700.000,"max":95700.000},"ns_per_u":null,"cycles_per_u":null},{"id":"X51","work_u":24,"n":2,"envelope_bytes":186,"ns":{"min":44400.000,"median":55000.000,"p95":71700.000,"max":71700.000},"ns_per_u":2291.6667,"cycles_per_u":4125.00}]},"part_b":{"structures":["ChunkedFlat","HAMT","RRBTree","MerkleBTree"],"niches":["Q(array)","Q(table)"],"batch_b":64,"clock_ghz":1.8,"warmup":2,"trials":9,"inner":16,"rules":["SHA-256 32-byte node hashes","deterministic canonical CBOR","copy-on-write structural persistence","identical sorted-batch normalization"],"checks":["leaf-hash multiset identity [Q(array) N=16]: PASS (4 structures)","leaf-hash multiset identity [Q(table) N=16]: PASS (3 structures)","leaf-hash multiset identity [Q(array) N=256]: PASS (4 structures)","canonical CBOR identity [Q(array) N=256]: PASS (digest 11ed127f393ae53ddb89c5f3ac6bd218fd5b3df31197df2d7fd8d7aadc8d5c1b)","leaf-hash multiset identity [Q(table) N=256]: PASS (3 structures)","canonical CBOR identity [Q(table) N=256]: PASS (digest 60d0ddb17b2ab26cdaad92ef66b013984dc92a00be3de8cb34d324b5677ab7f3)","leaf-hash multiset identity [Q(array) N=4096]: PASS (4 structures)","canonical CBOR identity [Q(array) N=4096]: PASS (digest 9f2c068bf4ad9f720d70cc8df48d3a01566c7cb1ff1be005782e2e320c780d68)","leaf-hash multiset identity [Q(table) N=4096]: PASS (3 structures)","canonical CBOR identity [Q(table) N=4096]: PASS (digest 35434ff929b275b52e1c9537cb2ae5519493f9293cef40435791e65bda7c54e3)","leaf-hash multiset identity [Q(array) N=65536]: PASS (4 structures)","leaf-hash multiset identity [Q(table) N=65536]: PASS (3 structures)"],"all_checks_ok":true,"rows":[{"niche":"Q(array)","structure":"ChunkedFlat","n":16,"get":{"median_ns":3.931,"p95_ns":6.592,"sha_calls":0},"set":{"median_ns":4193.750,"p95_ns":5750.000,"sha_calls":6,"alloc_bytes":1936},"batch":{"total_ns":214600.0,"ns_per_item":3353.125,"sha_calls":321,"b":64},"fold":{"median_ns":23.1,"ns_per_element":1.4465},"footprint":{"mem_bytes":1864,"mem_class":"L1","cbor_bytes":null,"proof_bytes":141},"root":"e233e6856d659b989e2e156eb7abf579d6772ee3c5cdcc92b20746f885b3355a","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"RRBTree","n":16,"get":{"median_ns":1.562,"p95_ns":1.904,"sha_calls":0},"set":{"median_ns":4093.750,"p95_ns":4543.750,"sha_calls":6,"alloc_bytes":1728},"batch":{"total_ns":271500.0,"ns_per_item":4242.188,"sha_calls":384,"b":64},"fold":{"median_ns":21.1,"ns_per_element":1.3214},"footprint":{"mem_bytes":1752,"mem_class":"L1","cbor_bytes":null,"proof_bytes":142},"root":"9423d806e15ccaa965d47e4930a41b3f7038f1db9d4940e0794003327ba77c1c","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"HAMT","n":16,"get":{"median_ns":490.894,"p95_ns":629.492,"sha_calls":1},"set":{"median_ns":5893.750,"p95_ns":16918.750,"sha_calls":7,"alloc_bytes":1426},"batch":{"total_ns":457400.0,"ns_per_item":7146.875,"sha_calls":600,"b":64},"fold":{"median_ns":235.3,"ns_per_element":14.7034},"footprint":{"mem_bytes":4016,"mem_class":"L1","cbor_bytes":null,"proof_bytes":218},"root":"fcc040c37cfcdc7dc162008c83d9cb5687d3ccd2b45791242c6191d0e8302094","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"MerkleBTree","n":16,"get":{"median_ns":6.787,"p95_ns":7.129,"sha_calls":0},"set":{"median_ns":5281.250,"p95_ns":10443.750,"sha_calls":8,"alloc_bytes":2005},"batch":{"total_ns":320600.0,"ns_per_item":5009.375,"sha_calls":501,"b":64},"fold":{"median_ns":26.8,"ns_per_element":1.6754},"footprint":{"mem_bytes":2400,"mem_class":"L1","cbor_bytes":null,"proof_bytes":176},"root":"31f7f664e74d4a623aa1a4e9df61bfc0b201d2afb593297ca9702f7a561ffce6","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"ChunkedFlat","n":16,"get":{"median_ns":3.760,"p95_ns":7.886,"sha_calls":0},"set":{"median_ns":4268.750,"p95_ns":4850.000,"sha_calls":6,"alloc_bytes":1936},"batch":{"total_ns":219200.0,"ns_per_item":3425.000,"sha_calls":321,"b":64},"fold":{"median_ns":23.7,"ns_per_element":1.4801},"footprint":{"mem_bytes":1864,"mem_class":"L1","cbor_bytes":null,"proof_bytes":141},"root":"a78be45b77f884ac2c4fdef00e9f2f0775b46767739a9faa7f968894c3d7c647","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"HAMT","n":16,"get":{"median_ns":513.306,"p95_ns":1094.214,"sha_calls":1},"set":{"median_ns":6112.500,"p95_ns":8981.250,"sha_calls":8,"alloc_bytes":1449},"batch":{"total_ns":426200.0,"ns_per_item":6659.375,"sha_calls":621,"b":64},"fold":{"median_ns":181.7,"ns_per_element":11.3586},"footprint":{"mem_bytes":4288,"mem_class":"L1","cbor_bytes":null,"proof_bytes":184},"root":"236f4f23f39a0db99e2a25fd09372a341a0441af65f046481ba22e41d25dfa48","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"MerkleBTree","n":16,"get":{"median_ns":6.445,"p95_ns":6.445,"sha_calls":0},"set":{"median_ns":5512.500,"p95_ns":9156.250,"sha_calls":8,"alloc_bytes":2005},"batch":{"total_ns":330700.0,"ns_per_item":5167.188,"sha_calls":485,"b":64},"fold":{"median_ns":28.2,"ns_per_element":1.7609},"footprint":{"mem_bytes":2400,"mem_class":"L1","cbor_bytes":null,"proof_bytes":176},"root":"fe66fe75fd39f9dd2e3c16584955393004280b27de8048bb80543fde3395517a","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"ChunkedFlat","n":256,"get":{"median_ns":10.889,"p95_ns":19.409,"sha_calls":0},"set":{"median_ns":9475.000,"p95_ns":14100.000,"sha_calls":14,"alloc_bytes":4560},"batch":{"total_ns":592200.0,"ns_per_item":9253.125,"sha_calls":896,"b":64},"fold":{"median_ns":400.0,"ns_per_element":1.5625},"footprint":{"mem_bytes":30264,"mem_class":"L1","cbor_bytes":9195,"proof_bytes":278},"root":"54b14a208b11c45eb83430562fbc9483c5c0da66ac9fe9500b0848d2ade790cb","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"RRBTree","n":256,"get":{"median_ns":3.052,"p95_ns":3.052,"sha_calls":0},"set":{"median_ns":7818.750,"p95_ns":13168.750,"sha_calls":11,"alloc_bytes":3072},"batch":{"total_ns":528400.0,"ns_per_item":8256.250,"sha_calls":704,"b":64},"fold":{"median_ns":449.6,"ns_per_element":1.7563},"footprint":{"mem_bytes":29400,"mem_class":"L1","cbor_bytes":9195,"proof_bytes":280},"root":"bba62d41f2450c52f63c7ba5098a1f47101872ea51dd43229a954563ed3d9803","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"HAMT","n":256,"get":{"median_ns":516.553,"p95_ns":672.241,"sha_calls":1},"set":{"median_ns":10406.250,"p95_ns":18768.750,"sha_calls":17,"alloc_bytes":2920},"batch":{"total_ns":630700.0,"ns_per_item":9854.688,"sha_calls":1006,"b":64},"fold":{"median_ns":4764.1,"ns_per_element":18.6096},"footprint":{"mem_bytes":72808,"mem_class":"L1","cbor_bytes":9195,"proof_bytes":289},"root":"f80e7deaa6c664c03c10c459544b5dafed00d1da53590b788b8c768d4efa02f7","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"MerkleBTree","n":256,"get":{"median_ns":12.866,"p95_ns":13.574,"sha_calls":0},"set":{"median_ns":8987.500,"p95_ns":14587.500,"sha_calls":13,"alloc_bytes":3498},"batch":{"total_ns":560900.0,"ns_per_item":8764.062,"sha_calls":812,"b":64},"fold":{"median_ns":384.4,"ns_per_element":1.5015},"footprint":{"mem_bytes":33544,"mem_class":"L1","cbor_bytes":9195,"proof_bytes":312},"root":"ff75f61b6f0ea949dbb5f10d2e2945397425c01f11227444f832287e0e592657","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"ChunkedFlat","n":256,"get":{"median_ns":10.669,"p95_ns":16.919,"sha_calls":0},"set":{"median_ns":9462.500,"p95_ns":12793.750,"sha_calls":14,"alloc_bytes":4560},"batch":{"total_ns":597900.0,"ns_per_item":9342.188,"sha_calls":896,"b":64},"fold":{"median_ns":410.5,"ns_per_element":1.6037},"footprint":{"mem_bytes":30264,"mem_class":"L1","cbor_bytes":11003,"proof_bytes":278},"root":"2bb66ef3933d32c20fa10805b41e00388ef27f6a97342dd73e177642eb6db25a","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"HAMT","n":256,"get":{"median_ns":520.703,"p95_ns":665.991,"sha_calls":1},"set":{"median_ns":10525.000,"p95_ns":19343.750,"sha_calls":17,"alloc_bytes":2830},"batch":{"total_ns":649900.0,"ns_per_item":10154.688,"sha_calls":1011,"b":64},"fold":{"median_ns":4637.9,"ns_per_element":18.1168},"footprint":{"mem_bytes":73448,"mem_class":"L1","cbor_bytes":11003,"proof_bytes":252},"root":"27a6cdd69ef58efd799fa248b918c8c9b5f75ea52d14a294a4e705d6b952bd7f","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"MerkleBTree","n":256,"get":{"median_ns":12.939,"p95_ns":20.996,"sha_calls":0},"set":{"median_ns":11231.250,"p95_ns":20037.500,"sha_calls":13,"alloc_bytes":3480},"batch":{"total_ns":656300.0,"ns_per_item":10254.688,"sha_calls":804,"b":64},"fold":{"median_ns":392.6,"ns_per_element":1.5335},"footprint":{"mem_bytes":33544,"mem_class":"L1","cbor_bytes":11003,"proof_bytes":312},"root":"eefa93e4de26fe41cdc294f3e6f58ada7dab1c9fd3457036e5be92b6fe3c16bd","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"ChunkedFlat","n":4096,"get":{"median_ns":21.704,"p95_ns":35.376,"sha_calls":0},"set":{"median_ns":15275.000,"p95_ns":22150.000,"sha_calls":22,"alloc_bytes":5808},"batch":{"total_ns":970300.0,"ns_per_item":15160.938,"sha_calls":1408,"b":64},"fold":{"median_ns":8937.5,"ns_per_element":2.1820},"footprint":{"mem_bytes":488184,"mem_class":"L2","cbor_bytes":151275,"proof_bytes":414},"root":"d040fd4b18a72ea577177593efaebb1f8c7429030015ce374194fa6cd68c38fb","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"RRBTree","n":4096,"get":{"median_ns":4.785,"p95_ns":7.764,"sha_calls":0},"set":{"median_ns":11443.750,"p95_ns":15837.500,"sha_calls":16,"alloc_bytes":4416},"batch":{"total_ns":700100.0,"ns_per_item":10939.062,"sha_calls":1024,"b":64},"fold":{"median_ns":11593.8,"ns_per_element":2.8305},"footprint":{"mem_bytes":471768,"mem_class":"L2","cbor_bytes":151275,"proof_bytes":416},"root":"5c30dc637ead2e9fb7644aba37de5afc9f54e5f5b33d5db1ff9d00f26ae50827","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"HAMT","n":4096,"get":{"median_ns":566.089,"p95_ns":836.816,"sha_calls":1},"set":{"median_ns":14556.250,"p95_ns":23725.000,"sha_calls":20,"alloc_bytes":4316},"batch":{"total_ns":1007600.0,"ns_per_item":15743.750,"sha_calls":1388,"b":64},"fold":{"median_ns":142656.2,"ns_per_element":34.8282},"footprint":{"mem_bytes":1181072,"mem_class":"L3","cbor_bytes":151275,"proof_bytes":426},"root":"1f5702c6a1b21a9b4ca181796fb634937218d06ba38df6a4187736c8330e82b7","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"MerkleBTree","n":4096,"get":{"median_ns":27.002,"p95_ns":36.670,"sha_calls":0},"set":{"median_ns":12812.500,"p95_ns":20000.000,"sha_calls":18,"alloc_bytes":4986},"batch":{"total_ns":794700.0,"ns_per_item":12417.188,"sha_calls":1140,"b":64},"fold":{"median_ns":8400.0,"ns_per_element":2.0508},"footprint":{"mem_bytes":528560,"mem_class":"L2","cbor_bytes":151275,"proof_bytes":448},"root":"a5ea845d27d8729ef92b09c383f35a1b7a10a854fa3bc0286d0339415ee9074e","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"ChunkedFlat","n":4096,"get":{"median_ns":21.606,"p95_ns":35.156,"sha_calls":0},"set":{"median_ns":15181.250,"p95_ns":24168.750,"sha_calls":22,"alloc_bytes":5808},"batch":{"total_ns":962300.0,"ns_per_item":15035.938,"sha_calls":1408,"b":64},"fold":{"median_ns":8900.0,"ns_per_element":2.1729},"footprint":{"mem_bytes":488184,"mem_class":"L2","cbor_bytes":176123,"proof_bytes":414},"root":"8ca318ff8d0b4199e3a651fca12ff3b850d36a1b6e296d620417c2906bc1c8fb","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"HAMT","n":4096,"get":{"median_ns":573.535,"p95_ns":897.363,"sha_calls":1},"set":{"median_ns":14906.250,"p95_ns":21943.750,"sha_calls":23,"alloc_bytes":4325},"batch":{"total_ns":936100.0,"ns_per_item":14626.562,"sha_calls":1398,"b":64},"fold":{"median_ns":143993.8,"ns_per_element":35.1547},"footprint":{"mem_bytes":1178648,"mem_class":"L3","cbor_bytes":176123,"proof_bytes":426},"root":"11761ef31a8385b4160f4c88df193a3b38d80cf69310f73e43742cbab8cc1c27","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"MerkleBTree","n":4096,"get":{"median_ns":26.416,"p95_ns":37.280,"sha_calls":0},"set":{"median_ns":12987.500,"p95_ns":19200.000,"sha_calls":18,"alloc_bytes":4968},"batch":{"total_ns":803100.0,"ns_per_item":12548.438,"sha_calls":1123,"b":64},"fold":{"median_ns":8218.8,"ns_per_element":2.0065},"footprint":{"mem_bytes":528560,"mem_class":"L2","cbor_bytes":176123,"proof_bytes":448},"root":"6f832bafc4e0d79b4e8098b3dee410f105187861d4cbabf7fa69f7902085e4fa","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"ChunkedFlat","n":65536,"get":{"median_ns":58.057,"p95_ns":109.595,"sha_calls":0},"set":{"median_ns":21193.750,"p95_ns":34943.750,"sha_calls":30,"alloc_bytes":7056},"batch":{"total_ns":1320500.0,"ns_per_item":20632.812,"sha_calls":1920,"b":64},"fold":{"median_ns":165300.0,"ns_per_element":2.5223},"footprint":{"mem_bytes":7814904,"mem_class":"L3","cbor_bytes":null,"proof_bytes":550},"root":"dc86db0879d21679487bc72ed2426aae08ecc8ff7e33c7b90149518d2cf5312b","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"RRBTree","n":65536,"get":{"median_ns":15.479,"p95_ns":39.429,"sha_calls":0},"set":{"median_ns":15218.750,"p95_ns":24987.500,"sha_calls":21,"alloc_bytes":5760},"batch":{"total_ns":998000.0,"ns_per_item":15593.750,"sha_calls":1344,"b":64},"fold":{"median_ns":136300.0,"ns_per_element":2.0798},"footprint":{"mem_bytes":7549656,"mem_class":"L3","cbor_bytes":null,"proof_bytes":554},"root":"ad1cb44c4346020a0d0b51c494f89f14ffd006503b2a113331709d19b9247ecc","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"HAMT","n":65536,"get":{"median_ns":662.061,"p95_ns":1143.896,"sha_calls":1},"set":{"median_ns":19662.500,"p95_ns":32950.000,"sha_calls":26,"alloc_bytes":5786},"batch":{"total_ns":1273200.0,"ns_per_item":19893.750,"sha_calls":1793,"b":64},"fold":{"median_ns":8301700.0,"ns_per_element":126.6739},"footprint":{"mem_bytes":18846640,"mem_class":"DRAM","cbor_bytes":null,"proof_bytes":566},"root":"259efbb371b001ff900b935a632156ceff58e606ab95cf551269838a88e42374","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(array)","structure":"MerkleBTree","n":65536,"get":{"median_ns":54.785,"p95_ns":120.508,"sha_calls":0},"set":{"median_ns":16725.000,"p95_ns":31156.250,"sha_calls":23,"alloc_bytes":6456},"batch":{"total_ns":1065800.0,"ns_per_item":16653.125,"sha_calls":1440,"b":64},"fold":{"median_ns":163300.0,"ns_per_element":2.4918},"footprint":{"mem_bytes":8445528,"mem_class":"DRAM","cbor_bytes":null,"proof_bytes":584},"root":"ded2160ba4eb39200df67051474e59c01cc6db510031f93fc053d2fb78a54734","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"ChunkedFlat","n":65536,"get":{"median_ns":57.788,"p95_ns":103.809,"sha_calls":0},"set":{"median_ns":22037.500,"p95_ns":34081.250,"sha_calls":30,"alloc_bytes":7056},"batch":{"total_ns":1389800.0,"ns_per_item":21715.625,"sha_calls":1920,"b":64},"fold":{"median_ns":179200.0,"ns_per_element":2.7344},"footprint":{"mem_bytes":7814904,"mem_class":"L3","cbor_bytes":null,"proof_bytes":550},"root":"a25a15804b15cfefa225d5960f6c535c5159f33a1a3eae44c8f3b02c8fb93e72","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"HAMT","n":65536,"get":{"median_ns":674.292,"p95_ns":1093.481,"sha_calls":1},"set":{"median_ns":20943.750,"p95_ns":32556.250,"sha_calls":31,"alloc_bytes":5814},"batch":{"total_ns":1326000.0,"ns_per_item":20718.750,"sha_calls":1776,"b":64},"fold":{"median_ns":8547000.0,"ns_per_element":130.4169},"footprint":{"mem_bytes":18898352,"mem_class":"DRAM","cbor_bytes":null,"proof_bytes":672},"root":"ab55444700529fe66ec27dc5157bbe94f6b6661e8a99fde3ff2636905b937e1e","checks":{"len_ok":true,"values_ok":true}},{"niche":"Q(table)","structure":"MerkleBTree","n":65536,"get":{"median_ns":54.834,"p95_ns":107.251,"sha_calls":0},"set":{"median_ns":16956.250,"p95_ns":31768.750,"sha_calls":23,"alloc_bytes":6474},"batch":{"total_ns":1080500.0,"ns_per_item":16882.812,"sha_calls":1449,"b":64},"fold":{"median_ns":172800.0,"ns_per_element":2.6367},"footprint":{"mem_bytes":8445528,"mem_class":"DRAM","cbor_bytes":null,"proof_bytes":584},"root":"4147cdd32d0215154ee425fff426e03a923dcb9670928147f9ff0d574a9beb65","checks":{"len_ok":true,"values_ok":true}}]}}
[[/CODE]]

#### III.2.call_trade_results.json

[[CODE]]
{"study":"step6-2-call-trade","clock_ghz":1.80,"warmup":2,"trials":9,"storage":[{"p":2,"m":1,"inline_words":18,"call_words":28,"inline_env":217,"call_env":300,"theory":null,"empirical":null},{"p":2,"m":2,"inline_words":38,"call_words":34,"inline_env":378,"call_env":349,"theory":2.0000,"empirical":2},{"p":2,"m":4,"inline_words":78,"call_words":46,"inline_env":698,"call_env":445,"theory":1.3333,"empirical":2},{"p":2,"m":8,"inline_words":158,"call_words":70,"inline_env":1338,"call_env":637,"theory":1.1429,"empirical":2},{"p":2,"m":16,"inline_words":318,"call_words":118,"inline_env":2618,"call_env":1021,"theory":1.0667,"empirical":2},{"p":4,"m":1,"inline_words":35,"call_words":45,"inline_env":354,"call_env":437,"theory":null,"empirical":null},{"p":4,"m":2,"inline_words":72,"call_words":51,"inline_env":650,"call_env":485,"theory":2.0000,"empirical":2},{"p":4,"m":4,"inline_words":146,"call_words":63,"inline_env":1242,"call_env":581,"theory":1.3333,"empirical":2},{"p":4,"m":8,"inline_words":294,"call_words":87,"inline_env":2426,"call_env":773,"theory":1.1429,"empirical":2},{"p":4,"m":16,"inline_words":590,"call_words":135,"inline_env":4794,"call_env":1157,"theory":1.0667,"empirical":2},{"p":8,"m":1,"inline_words":68,"call_words":78,"inline_env":618,"call_env":701,"theory":null,"empirical":null},{"p":8,"m":2,"inline_words":138,"call_words":84,"inline_env":1178,"call_env":749,"theory":2.0000,"empirical":2},{"p":8,"m":4,"inline_words":278,"call_words":96,"inline_env":2298,"call_env":845,"theory":1.3333,"empirical":2},{"p":8,"m":8,"inline_words":558,"call_words":120,"inline_env":4538,"call_env":1037,"theory":1.1429,"empirical":2},{"p":8,"m":16,"inline_words":1118,"call_words":168,"inline_env":9018,"call_env":1421,"theory":1.0667,"empirical":2},{"p":16,"m":1,"inline_words":135,"call_words":145,"inline_env":1154,"call_env":1237,"theory":null,"empirical":null},{"p":16,"m":2,"inline_words":272,"call_words":151,"inline_env":2250,"call_env":1285,"theory":2.0000,"empirical":2},{"p":16,"m":4,"inline_words":546,"call_words":163,"inline_env":4442,"call_env":1381,"theory":1.3333,"empirical":2},{"p":16,"m":8,"inline_words":1094,"call_words":187,"inline_env":8826,"call_env":1573,"theory":1.1429,"empirical":2},{"p":16,"m":16,"inline_words":2190,"call_words":235,"inline_env":17594,"call_env":1957,"theory":1.0667,"empirical":2},{"p":32,"m":1,"inline_words":268,"call_words":278,"inline_env":2218,"call_env":2301,"theory":null,"empirical":null},{"p":32,"m":2,"inline_words":538,"call_words":284,"inline_env":4378,"call_env":2349,"theory":2.0000,"empirical":2},{"p":32,"m":4,"inline_words":1078,"call_words":296,"inline_env":8698,"call_env":2445,"theory":1.3333,"empirical":2},{"p":32,"m":8,"inline_words":2158,"call_words":320,"inline_env":17338,"call_env":2637,"theory":1.1429,"empirical":2},{"p":32,"m":16,"inline_words":4318,"call_words":368,"inline_env":34618,"call_env":3021,"theory":1.0667,"empirical":2},{"p":64,"m":1,"inline_words":535,"call_words":545,"inline_env":4354,"call_env":4437,"theory":null,"empirical":null},{"p":64,"m":2,"inline_words":1072,"call_words":551,"inline_env":8650,"call_env":4485,"theory":2.0000,"empirical":2},{"p":64,"m":4,"inline_words":2146,"call_words":563,"inline_env":17242,"call_env":4581,"theory":1.3333,"empirical":2},{"p":64,"m":8,"inline_words":4294,"call_words":587,"inline_env":34426,"call_env":4773,"theory":1.1429,"empirical":2},{"p":64,"m":16,"inline_words":8590,"call_words":635,"inline_env":68796,"call_env":5157,"theory":1.0667,"empirical":2}],"bhat":[{"m":1,"inline_ok":true,"inline_bound":72,"inline_states":16,"inline_ns":{"min":4429.297,"median":4670.312,"p95":6660.938,"max":6660.938},"comp_ok":true,"comp_bound":96,"comp_states":17,"comp_ns":{"min":0.391,"median":0.391,"p95":2.734,"max":2.734},"refusal_m":5},{"m":2,"inline_ok":true,"inline_bound":72,"inline_states":128,"inline_ns":{"min":4260.547,"median":4607.422,"p95":5524.609,"max":5524.609},"comp_ok":true,"comp_bound":100,"comp_states":18,"comp_ns":{"min":0.391,"median":0.391,"p95":0.781,"max":0.781},"refusal_m":5},{"m":4,"inline_ok":true,"inline_bound":72,"inline_states":4096,"inline_ns":{"min":4344.531,"median":5892.578,"p95":8937.109,"max":8937.109},"comp_ok":true,"comp_bound":108,"comp_states":20,"comp_ns":{"min":0.391,"median":0.391,"p95":0.781,"max":0.781},"refusal_m":5},{"m":6,"inline_ok":false,"inline_bound":0,"inline_states":0,"inline_ns":{"min":3.906,"median":4.297,"p95":4.297,"max":4.297},"comp_ok":true,"comp_bound":116,"comp_states":22,"comp_ns":{"min":0.391,"median":0.391,"p95":0.781,"max":0.781},"refusal_m":5},{"m":8,"inline_ok":false,"inline_bound":0,"inline_states":0,"inline_ns":{"min":3.906,"median":4.297,"p95":4.297,"max":4.297},"comp_ok":true,"comp_bound":124,"comp_states":24,"comp_ns":{"min":0.391,"median":0.391,"p95":0.781,"max":0.781},"refusal_m":5}],"execution":[{"p":4,"m":2,"f":4,"c":1,"mode":"Inline","ns":{"min":31800.000,"median":31900.000,"p95":36200.000,"max":36200.000},"cycles":57420.0,"transitions":10,"charged":60,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":1,"mode":"CallB","ns":{"min":33100.000,"median":33200.000,"p95":33500.000,"max":33500.000},"cycles":59760.0,"transitions":11,"charged":64,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":1,"mode":"CallC","ns":{"min":40100.000,"median":40400.000,"p95":41800.000,"max":41800.000},"cycles":72720.0,"transitions":11,"charged":64,"p2_bytes":150,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":1,"mode":"MemoB","ns":{"min":28200.000,"median":28500.000,"p95":29200.000,"max":29200.000},"cycles":51300.0,"transitions":9,"charged":56,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":1,"mode":"MemoC","ns":{"min":34500.000,"median":34600.000,"p95":58100.000,"max":58100.000},"cycles":62280.0,"transitions":9,"charged":56,"p2_bytes":150,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":4,"mode":"Inline","ns":{"min":30600.000,"median":30600.000,"p95":30700.000,"max":30700.000},"cycles":55080.0,"transitions":10,"charged":60,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":4,"mode":"CallB","ns":{"min":63700.000,"median":63700.000,"p95":64100.000,"max":64100.000},"cycles":114660.0,"transitions":23,"charged":112,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":4,"mode":"CallC","ns":{"min":94800.000,"median":95100.000,"p95":95700.000,"max":95700.000},"cycles":171180.0,"transitions":23,"charged":112,"p2_bytes":267,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":4,"mode":"MemoB","ns":{"min":28100.000,"median":28200.000,"p95":29100.000,"max":29100.000},"cycles":50760.0,"transitions":9,"charged":56,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":4,"mode":"MemoC","ns":{"min":44500.000,"median":44600.000,"p95":49400.000,"max":49400.000},"cycles":80280.0,"transitions":9,"charged":56,"p2_bytes":267,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":8,"mode":"Inline","ns":{"min":35900.000,"median":36100.000,"p95":56500.000,"max":56500.000},"cycles":64980.0,"transitions":10,"charged":60,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":8,"mode":"CallB","ns":{"min":120500.000,"median":167600.000,"p95":213600.000,"max":213600.000},"cycles":301680.0,"transitions":39,"charged":176,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":8,"mode":"CallC","ns":{"min":189000.000,"median":231800.000,"p95":337500.000,"max":337500.000},"cycles":417240.0,"transitions":39,"charged":176,"p2_bytes":423,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":8,"mode":"MemoB","ns":{"min":30900.000,"median":38700.000,"p95":64100.000,"max":64100.000},"cycles":69660.0,"transitions":9,"charged":56,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":2,"f":4,"c":8,"mode":"MemoC","ns":{"min":59600.000,"median":64100.000,"p95":114300.000,"max":114300.000},"cycles":115380.0,"transitions":9,"charged":56,"p2_bytes":423,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":1,"mode":"Inline","ns":{"min":56800.000,"median":57200.000,"p95":82800.000,"max":82800.000},"cycles":102960.0,"transitions":10,"charged":60,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":1,"mode":"CallB","ns":{"min":60300.000,"median":60700.000,"p95":61300.000,"max":61300.000},"cycles":109260.0,"transitions":11,"charged":64,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":1,"mode":"CallC","ns":{"min":63700.000,"median":74100.000,"p95":95200.000,"max":95200.000},"cycles":133380.0,"transitions":11,"charged":64,"p2_bytes":402,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":1,"mode":"MemoB","ns":{"min":46000.000,"median":48600.000,"p95":61500.000,"max":61500.000},"cycles":87480.0,"transitions":9,"charged":56,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":1,"mode":"MemoC","ns":{"min":60100.000,"median":65200.000,"p95":87700.000,"max":87700.000},"cycles":117360.0,"transitions":9,"charged":56,"p2_bytes":402,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":4,"mode":"Inline","ns":{"min":47700.000,"median":47900.000,"p95":69700.000,"max":69700.000},"cycles":86220.0,"transitions":10,"charged":60,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":4,"mode":"CallB","ns":{"min":93100.000,"median":93400.000,"p95":93500.000,"max":93500.000},"cycles":168120.0,"transitions":23,"charged":112,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":4,"mode":"CallC","ns":{"min":177600.000,"median":179100.000,"p95":187000.000,"max":187000.000},"cycles":322380.0,"transitions":23,"charged":112,"p2_bytes":843,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":4,"mode":"MemoB","ns":{"min":44100.000,"median":44300.000,"p95":45700.000,"max":45700.000},"cycles":79740.0,"transitions":9,"charged":56,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":4,"mode":"MemoC","ns":{"min":87400.000,"median":89700.000,"p95":94000.000,"max":94000.000},"cycles":161460.0,"transitions":9,"charged":56,"p2_bytes":843,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":8,"mode":"Inline","ns":{"min":47700.000,"median":47800.000,"p95":48100.000,"max":48100.000},"cycles":86040.0,"transitions":10,"charged":60,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":8,"mode":"CallB","ns":{"min":148600.000,"median":149100.000,"p95":188200.000,"max":188200.000},"cycles":268380.0,"transitions":39,"charged":176,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":8,"mode":"CallC","ns":{"min":477400.000,"median":536600.000,"p95":717200.000,"max":717200.000},"cycles":965880.0,"transitions":39,"charged":176,"p2_bytes":1431,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":8,"mode":"MemoB","ns":{"min":43000.000,"median":43300.000,"p95":49000.000,"max":49000.000},"cycles":77940.0,"transitions":9,"charged":56,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":2,"f":16,"c":8,"mode":"MemoC","ns":{"min":137300.000,"median":168400.000,"p95":197400.000,"max":197400.000},"cycles":303120.0,"transitions":9,"charged":56,"p2_bytes":1431,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":1,"mode":"Inline","ns":{"min":105700.000,"median":107800.000,"p95":120700.000,"max":120700.000},"cycles":194040.0,"transitions":10,"charged":60,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":1,"mode":"CallB","ns":{"min":113400.000,"median":113900.000,"p95":118000.000,"max":118000.000},"cycles":205020.0,"transitions":11,"charged":64,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":1,"mode":"CallC","ns":{"min":158300.000,"median":159500.000,"p95":162700.000,"max":162700.000},"cycles":287100.0,"transitions":11,"charged":64,"p2_bytes":1452,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":1,"mode":"MemoB","ns":{"min":100300.000,"median":100900.000,"p95":107100.000,"max":107100.000},"cycles":181620.0,"transitions":9,"charged":56,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":1,"mode":"MemoC","ns":{"min":140000.000,"median":141100.000,"p95":145300.000,"max":145300.000},"cycles":253980.0,"transitions":9,"charged":56,"p2_bytes":1452,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":4,"mode":"Inline","ns":{"min":108100.000,"median":108500.000,"p95":135200.000,"max":135200.000},"cycles":195300.0,"transitions":10,"charged":60,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":4,"mode":"CallB","ns":{"min":207500.000,"median":237300.000,"p95":274100.000,"max":274100.000},"cycles":427140.0,"transitions":23,"charged":112,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":4,"mode":"CallC","ns":{"min":514700.000,"median":579800.000,"p95":975900.000,"max":975900.000},"cycles":1043640.0,"transitions":23,"charged":112,"p2_bytes":3192,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":4,"mode":"MemoB","ns":{"min":98500.000,"median":99200.000,"p95":106700.000,"max":106700.000},"cycles":178560.0,"transitions":9,"charged":56,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":4,"mode":"MemoC","ns":{"min":262500.000,"median":265800.000,"p95":320700.000,"max":320700.000},"cycles":478440.0,"transitions":9,"charged":56,"p2_bytes":3192,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":8,"mode":"Inline","ns":{"min":104800.000,"median":105200.000,"p95":131300.000,"max":131300.000},"cycles":189360.0,"transitions":10,"charged":60,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":8,"mode":"CallB","ns":{"min":286900.000,"median":341400.000,"p95":407200.000,"max":407200.000},"cycles":614520.0,"transitions":39,"charged":176,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":8,"mode":"CallC","ns":{"min":1283600.000,"median":1315100.000,"p95":1667900.000,"max":1667900.000},"cycles":2367180.0,"transitions":39,"charged":176,"p2_bytes":5512,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":8,"mode":"MemoB","ns":{"min":104200.000,"median":115300.000,"p95":149700.000,"max":149700.000},"cycles":207540.0,"transitions":9,"charged":56,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":2,"f":64,"c":8,"mode":"MemoC","ns":{"min":405700.000,"median":475100.000,"p95":828300.000,"max":828300.000},"cycles":855180.0,"transitions":9,"charged":56,"p2_bytes":5512,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":1,"mode":"Inline","ns":{"min":98300.000,"median":100300.000,"p95":129800.000,"max":129800.000},"cycles":180540.0,"transitions":40,"charged":180,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":1,"mode":"CallB","ns":{"min":73800.000,"median":74000.000,"p95":74300.000,"max":74300.000},"cycles":133200.0,"transitions":29,"charged":136,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":1,"mode":"CallC","ns":{"min":88800.000,"median":89200.000,"p95":94000.000,"max":94000.000},"cycles":160560.0,"transitions":29,"charged":136,"p2_bytes":150,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":1,"mode":"MemoB","ns":{"min":55100.000,"median":55200.000,"p95":55500.000,"max":55500.000},"cycles":99360.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":1,"mode":"MemoC","ns":{"min":66400.000,"median":66500.000,"p95":66600.000,"max":66600.000},"cycles":119700.0,"transitions":21,"charged":104,"p2_bytes":150,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":4,"mode":"Inline","ns":{"min":99900.000,"median":100200.000,"p95":104900.000,"max":104900.000},"cycles":180360.0,"transitions":40,"charged":180,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":4,"mode":"CallB","ns":{"min":187200.000,"median":187400.000,"p95":192400.000,"max":192400.000},"cycles":337320.0,"transitions":77,"charged":328,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":4,"mode":"CallC","ns":{"min":310700.000,"median":360100.000,"p95":475900.000,"max":475900.000},"cycles":648180.0,"transitions":77,"charged":328,"p2_bytes":267,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":4,"mode":"MemoB","ns":{"min":72400.000,"median":118800.000,"p95":145500.000,"max":145500.000},"cycles":213840.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":4,"mode":"MemoC","ns":{"min":81700.000,"median":87000.000,"p95":109700.000,"max":109700.000},"cycles":156600.0,"transitions":21,"charged":104,"p2_bytes":267,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":8,"mode":"Inline","ns":{"min":99700.000,"median":106000.000,"p95":159100.000,"max":159100.000},"cycles":190800.0,"transitions":40,"charged":180,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":8,"mode":"CallB","ns":{"min":330900.000,"median":347500.000,"p95":362900.000,"max":362900.000},"cycles":625500.0,"transitions":141,"charged":584,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":8,"mode":"CallC","ns":{"min":560100.000,"median":584500.000,"p95":898300.000,"max":898300.000},"cycles":1052100.0,"transitions":141,"charged":584,"p2_bytes":423,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":8,"mode":"MemoB","ns":{"min":84400.000,"median":91000.000,"p95":136100.000,"max":136100.000},"cycles":163800.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":4,"m":8,"f":4,"c":8,"mode":"MemoC","ns":{"min":160900.000,"median":181000.000,"p95":247100.000,"max":247100.000},"cycles":325800.0,"transitions":21,"charged":104,"p2_bytes":423,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":1,"mode":"Inline","ns":{"min":147800.000,"median":186700.000,"p95":247900.000,"max":247900.000},"cycles":336060.0,"transitions":40,"charged":180,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":1,"mode":"CallB","ns":{"min":108100.000,"median":111500.000,"p95":144400.000,"max":144400.000},"cycles":200700.0,"transitions":29,"charged":136,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":1,"mode":"CallC","ns":{"min":129500.000,"median":136500.000,"p95":143900.000,"max":143900.000},"cycles":245700.0,"transitions":29,"charged":136,"p2_bytes":402,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":1,"mode":"MemoB","ns":{"min":83900.000,"median":84100.000,"p95":97500.000,"max":97500.000},"cycles":151380.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":1,"mode":"MemoC","ns":{"min":100700.000,"median":101100.000,"p95":101300.000,"max":101300.000},"cycles":181980.0,"transitions":21,"charged":104,"p2_bytes":402,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":4,"mode":"Inline","ns":{"min":147900.000,"median":148100.000,"p95":153000.000,"max":153000.000},"cycles":266580.0,"transitions":40,"charged":180,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":4,"mode":"CallB","ns":{"min":272300.000,"median":272900.000,"p95":291900.000,"max":291900.000},"cycles":491220.0,"transitions":77,"charged":328,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":4,"mode":"CallC","ns":{"min":506400.000,"median":777800.000,"p95":927600.000,"max":927600.000},"cycles":1400040.0,"transitions":77,"charged":328,"p2_bytes":843,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":4,"mode":"MemoB","ns":{"min":81700.000,"median":82100.000,"p95":82600.000,"max":82600.000},"cycles":147780.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":4,"mode":"MemoC","ns":{"min":162000.000,"median":166900.000,"p95":178300.000,"max":178300.000},"cycles":300420.0,"transitions":21,"charged":104,"p2_bytes":843,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":8,"mode":"Inline","ns":{"min":147900.000,"median":148000.000,"p95":154100.000,"max":154100.000},"cycles":266400.0,"transitions":40,"charged":180,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":8,"mode":"CallB","ns":{"min":487100.000,"median":491100.000,"p95":601300.000,"max":601300.000},"cycles":883980.0,"transitions":141,"charged":584,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":8,"mode":"CallC","ns":{"min":1256700.000,"median":1305600.000,"p95":1715000.000,"max":1715000.000},"cycles":2350080.0,"transitions":141,"charged":584,"p2_bytes":1431,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":8,"mode":"MemoB","ns":{"min":124100.000,"median":142900.000,"p95":166200.000,"max":166200.000},"cycles":257220.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":4,"m":8,"f":16,"c":8,"mode":"MemoC","ns":{"min":383900.000,"median":423500.000,"p95":529500.000,"max":529500.000},"cycles":762300.0,"transitions":21,"charged":104,"p2_bytes":1431,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":1,"mode":"Inline","ns":{"min":282000.000,"median":301300.000,"p95":354000.000,"max":354000.000},"cycles":542340.0,"transitions":40,"charged":180,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":1,"mode":"CallB","ns":{"min":217700.000,"median":226200.000,"p95":242700.000,"max":242700.000},"cycles":407160.0,"transitions":29,"charged":136,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":1,"mode":"CallC","ns":{"min":312400.000,"median":322800.000,"p95":477000.000,"max":477000.000},"cycles":581040.0,"transitions":29,"charged":136,"p2_bytes":1452,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":1,"mode":"MemoB","ns":{"min":173300.000,"median":196600.000,"p95":234600.000,"max":234600.000},"cycles":353880.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":1,"mode":"MemoC","ns":{"min":248300.000,"median":296200.000,"p95":431700.000,"max":431700.000},"cycles":533160.0,"transitions":21,"charged":104,"p2_bytes":1452,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":4,"mode":"Inline","ns":{"min":281000.000,"median":285600.000,"p95":354500.000,"max":354500.000},"cycles":514080.0,"transitions":40,"charged":180,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":4,"mode":"CallB","ns":{"min":497800.000,"median":499300.000,"p95":503400.000,"max":503400.000},"cycles":898740.0,"transitions":77,"charged":328,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":4,"mode":"CallC","ns":{"min":1473500.000,"median":1519300.000,"p95":2232300.000,"max":2232300.000},"cycles":2734740.0,"transitions":77,"charged":328,"p2_bytes":3192,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":4,"mode":"MemoB","ns":{"min":173300.000,"median":196300.000,"p95":258900.000,"max":258900.000},"cycles":353340.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":4,"mode":"MemoC","ns":{"min":467200.000,"median":481400.000,"p95":504900.000,"max":504900.000},"cycles":866520.0,"transitions":21,"charged":104,"p2_bytes":3192,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":8,"mode":"Inline","ns":{"min":281700.000,"median":282800.000,"p95":301700.000,"max":301700.000},"cycles":509040.0,"transitions":40,"charged":180,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":8,"mode":"CallB","ns":{"min":881000.000,"median":885500.000,"p95":918800.000,"max":918800.000},"cycles":1593900.0,"transitions":141,"charged":584,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":8,"mode":"CallC","ns":{"min":4133200.000,"median":4241400.000,"p95":4441600.000,"max":4441600.000},"cycles":7634520.0,"transitions":141,"charged":584,"p2_bytes":5512,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":8,"mode":"MemoB","ns":{"min":172000.000,"median":172800.000,"p95":185200.000,"max":185200.000},"cycles":311040.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":4,"m":8,"f":64,"c":8,"mode":"MemoC","ns":{"min":758500.000,"median":766700.000,"p95":830400.000,"max":830400.000},"cycles":1380060.0,"transitions":21,"charged":104,"p2_bytes":5512,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":1,"mode":"Inline","ns":{"min":86400.000,"median":86700.000,"p95":93500.000,"max":93500.000},"cycles":156060.0,"transitions":34,"charged":156,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":1,"mode":"CallB","ns":{"min":60300.000,"median":60500.000,"p95":90100.000,"max":90100.000},"cycles":108900.0,"transitions":23,"charged":112,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":1,"mode":"CallC","ns":{"min":72700.000,"median":73000.000,"p95":73600.000,"max":73600.000},"cycles":131400.0,"transitions":23,"charged":112,"p2_bytes":150,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":1,"mode":"MemoB","ns":{"min":55600.000,"median":55800.000,"p95":64200.000,"max":64200.000},"cycles":100440.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":1,"mode":"MemoC","ns":{"min":66900.000,"median":67000.000,"p95":68300.000,"max":68300.000},"cycles":120600.0,"transitions":21,"charged":104,"p2_bytes":150,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":4,"mode":"Inline","ns":{"min":86600.000,"median":86900.000,"p95":87500.000,"max":87500.000},"cycles":156420.0,"transitions":34,"charged":156,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":4,"mode":"CallB","ns":{"min":88900.000,"median":89100.000,"p95":89600.000,"max":89600.000},"cycles":160380.0,"transitions":35,"charged":160,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":4,"mode":"CallC","ns":{"min":131000.000,"median":131600.000,"p95":138700.000,"max":138700.000},"cycles":236880.0,"transitions":35,"charged":160,"p2_bytes":267,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":4,"mode":"MemoB","ns":{"min":55600.000,"median":55900.000,"p95":56500.000,"max":56500.000},"cycles":100620.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":4,"mode":"MemoC","ns":{"min":83400.000,"median":83900.000,"p95":84400.000,"max":84400.000},"cycles":151020.0,"transitions":21,"charged":104,"p2_bytes":267,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":8,"mode":"Inline","ns":{"min":86600.000,"median":87100.000,"p95":87800.000,"max":87800.000},"cycles":156780.0,"transitions":34,"charged":156,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":8,"mode":"CallB","ns":{"min":129800.000,"median":138900.000,"p95":189400.000,"max":189400.000},"cycles":250020.0,"transitions":51,"charged":224,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":8,"mode":"CallC","ns":{"min":373200.000,"median":381800.000,"p95":417000.000,"max":417000.000},"cycles":687240.0,"transitions":51,"charged":224,"p2_bytes":423,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":8,"mode":"MemoB","ns":{"min":96100.000,"median":101500.000,"p95":109200.000,"max":109200.000},"cycles":182700.0,"transitions":21,"charged":104,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":2,"f":4,"c":8,"mode":"MemoC","ns":{"min":160000.000,"median":180800.000,"p95":215600.000,"max":215600.000},"cycles":325440.0,"transitions":21,"charged":104,"p2_bytes":423,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":1,"mode":"Inline","ns":{"min":213100.000,"median":225700.000,"p95":250800.000,"max":250800.000},"cycles":406260.0,"transitions":34,"charged":156,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":1,"mode":"CallB","ns":{"min":139600.000,"median":156500.000,"p95":194100.000,"max":194100.000},"cycles":281700.0,"transitions":23,"charged":112,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":1,"mode":"CallC","ns":{"min":124700.000,"median":126500.000,"p95":138800.000,"max":138800.000},"cycles":227700.0,"transitions":23,"charged":112,"p2_bytes":402,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":1,"mode":"MemoB","ns":{"min":89900.000,"median":92100.000,"p95":132600.000,"max":132600.000},"cycles":165780.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":1,"mode":"MemoC","ns":{"min":104200.000,"median":107700.000,"p95":136200.000,"max":136200.000},"cycles":193860.0,"transitions":21,"charged":104,"p2_bytes":402,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":4,"mode":"Inline","ns":{"min":140000.000,"median":163100.000,"p95":268300.000,"max":268300.000},"cycles":293580.0,"transitions":34,"charged":156,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":4,"mode":"CallB","ns":{"min":132500.000,"median":134000.000,"p95":148600.000,"max":148600.000},"cycles":241200.0,"transitions":35,"charged":160,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":4,"mode":"CallC","ns":{"min":258800.000,"median":259700.000,"p95":266000.000,"max":266000.000},"cycles":467460.0,"transitions":35,"charged":160,"p2_bytes":843,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":4,"mode":"MemoB","ns":{"min":86800.000,"median":87600.000,"p95":88700.000,"max":88700.000},"cycles":157680.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":4,"mode":"MemoC","ns":{"min":169100.000,"median":170000.000,"p95":240600.000,"max":240600.000},"cycles":306000.0,"transitions":21,"charged":104,"p2_bytes":843,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":8,"mode":"Inline","ns":{"min":132100.000,"median":132600.000,"p95":141500.000,"max":141500.000},"cycles":238680.0,"transitions":34,"charged":156,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":8,"mode":"CallB","ns":{"min":191300.000,"median":192100.000,"p95":197500.000,"max":197500.000},"cycles":345780.0,"transitions":51,"charged":224,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":8,"mode":"CallC","ns":{"min":513300.000,"median":533500.000,"p95":642200.000,"max":642200.000},"cycles":960300.0,"transitions":51,"charged":224,"p2_bytes":1431,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":8,"mode":"MemoB","ns":{"min":86100.000,"median":86500.000,"p95":86900.000,"max":86900.000},"cycles":155700.0,"transitions":21,"charged":104,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":2,"f":16,"c":8,"mode":"MemoC","ns":{"min":244200.000,"median":285500.000,"p95":405200.000,"max":405200.000},"cycles":513900.0,"transitions":21,"charged":104,"p2_bytes":1431,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":1,"mode":"Inline","ns":{"min":253500.000,"median":254700.000,"p95":285000.000,"max":285000.000},"cycles":458460.0,"transitions":34,"charged":156,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":1,"mode":"CallB","ns":{"min":185200.000,"median":187600.000,"p95":193000.000,"max":193000.000},"cycles":337680.0,"transitions":23,"charged":112,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":1,"mode":"CallC","ns":{"min":268200.000,"median":270900.000,"p95":277200.000,"max":277200.000},"cycles":487620.0,"transitions":23,"charged":112,"p2_bytes":1452,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":1,"mode":"MemoB","ns":{"min":174000.000,"median":175100.000,"p95":193300.000,"max":193300.000},"cycles":315180.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":1,"mode":"MemoC","ns":{"min":250200.000,"median":251300.000,"p95":264100.000,"max":264100.000},"cycles":452340.0,"transitions":21,"charged":104,"p2_bytes":1452,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":4,"mode":"Inline","ns":{"min":253000.000,"median":255800.000,"p95":262300.000,"max":262300.000},"cycles":460440.0,"transitions":34,"charged":156,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":4,"mode":"CallB","ns":{"min":248900.000,"median":261300.000,"p95":271000.000,"max":271000.000},"cycles":470340.0,"transitions":35,"charged":160,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":4,"mode":"CallC","ns":{"min":728700.000,"median":732300.000,"p95":765700.000,"max":765700.000},"cycles":1318140.0,"transitions":35,"charged":160,"p2_bytes":3192,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":4,"mode":"MemoB","ns":{"min":169100.000,"median":169700.000,"p95":177400.000,"max":177400.000},"cycles":305460.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":4,"mode":"MemoC","ns":{"min":480000.000,"median":485800.000,"p95":538100.000,"max":538100.000},"cycles":874440.0,"transitions":21,"charged":104,"p2_bytes":3192,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":8,"mode":"Inline","ns":{"min":246000.000,"median":249900.000,"p95":279200.000,"max":279200.000},"cycles":449820.0,"transitions":34,"charged":156,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":8,"mode":"CallB","ns":{"min":347900.000,"median":358900.000,"p95":369700.000,"max":369700.000},"cycles":646020.0,"transitions":51,"charged":224,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":8,"mode":"CallC","ns":{"min":1585900.000,"median":1613000.000,"p95":1773300.000,"max":1773300.000},"cycles":2903400.0,"transitions":51,"charged":224,"p2_bytes":5512,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":8,"mode":"MemoB","ns":{"min":164300.000,"median":170600.000,"p95":181400.000,"max":181400.000},"cycles":307080.0,"transitions":21,"charged":104,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":2,"f":64,"c":8,"mode":"MemoC","ns":{"min":753600.000,"median":776200.000,"p95":1181600.000,"max":1181600.000},"cycles":1397160.0,"transitions":21,"charged":104,"p2_bytes":5512,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":1,"mode":"Inline","ns":{"min":331200.000,"median":337700.000,"p95":431000.000,"max":431000.000},"cycles":607860.0,"transitions":136,"charged":564,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":1,"mode":"CallB","ns":{"min":105200.000,"median":118200.000,"p95":153400.000,"max":153400.000},"cycles":212760.0,"transitions":41,"charged":184,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":1,"mode":"CallC","ns":{"min":129300.000,"median":203900.000,"p95":226600.000,"max":226600.000},"cycles":367020.0,"transitions":41,"charged":184,"p2_bytes":150,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":1,"mode":"MemoB","ns":{"min":142400.000,"median":154000.000,"p95":162600.000,"max":162600.000},"cycles":277200.0,"transitions":33,"charged":152,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":1,"mode":"MemoC","ns":{"min":121600.000,"median":147400.000,"p95":189100.000,"max":189100.000},"cycles":265320.0,"transitions":33,"charged":152,"p2_bytes":150,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":4,"mode":"Inline","ns":{"min":331500.000,"median":379700.000,"p95":425800.000,"max":425800.000},"cycles":683460.0,"transitions":136,"charged":564,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":4,"mode":"CallB","ns":{"min":217200.000,"median":305800.000,"p95":339200.000,"max":339200.000},"cycles":550440.0,"transitions":89,"charged":376,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":4,"mode":"CallC","ns":{"min":487800.000,"median":520700.000,"p95":623900.000,"max":623900.000},"cycles":937260.0,"transitions":89,"charged":376,"p2_bytes":267,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":4,"mode":"MemoB","ns":{"min":137100.000,"median":149100.000,"p95":176800.000,"max":176800.000},"cycles":268380.0,"transitions":33,"charged":152,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":4,"mode":"MemoC","ns":{"min":124700.000,"median":146000.000,"p95":227600.000,"max":227600.000},"cycles":262800.0,"transitions":33,"charged":152,"p2_bytes":267,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":8,"mode":"Inline","ns":{"min":325800.000,"median":326200.000,"p95":331700.000,"max":331700.000},"cycles":587160.0,"transitions":136,"charged":564,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":8,"mode":"CallB","ns":{"min":588100.000,"median":756400.000,"p95":882600.000,"max":882600.000},"cycles":1361520.0,"transitions":153,"charged":632,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":8,"mode":"CallC","ns":{"min":867900.000,"median":1259100.000,"p95":1298100.000,"max":1298100.000},"cycles":2266380.0,"transitions":153,"charged":632,"p2_bytes":423,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":8,"mode":"MemoB","ns":{"min":88800.000,"median":95000.000,"p95":165400.000,"max":165400.000},"cycles":171000.0,"transitions":33,"charged":152,"p2_bytes":111,"invariant_ok":true},{"p":16,"m":8,"f":4,"c":8,"mode":"MemoC","ns":{"min":193100.000,"median":217300.000,"p95":296000.000,"max":296000.000},"cycles":391140.0,"transitions":33,"charged":152,"p2_bytes":423,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":1,"mode":"Inline","ns":{"min":477800.000,"median":509200.000,"p95":618000.000,"max":618000.000},"cycles":916560.0,"transitions":136,"charged":564,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":1,"mode":"CallB","ns":{"min":157600.000,"median":158100.000,"p95":171600.000,"max":171600.000},"cycles":284580.0,"transitions":41,"charged":184,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":1,"mode":"CallC","ns":{"min":180500.000,"median":186500.000,"p95":197400.000,"max":197400.000},"cycles":335700.0,"transitions":41,"charged":184,"p2_bytes":402,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":1,"mode":"MemoB","ns":{"min":125800.000,"median":126300.000,"p95":209900.000,"max":209900.000},"cycles":227340.0,"transitions":33,"charged":152,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":1,"mode":"MemoC","ns":{"min":154600.000,"median":178900.000,"p95":228000.000,"max":228000.000},"cycles":322020.0,"transitions":33,"charged":152,"p2_bytes":402,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":4,"mode":"Inline","ns":{"min":465800.000,"median":540900.000,"p95":731500.000,"max":731500.000},"cycles":973620.0,"transitions":136,"charged":564,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":4,"mode":"CallB","ns":{"min":316700.000,"median":317700.000,"p95":356000.000,"max":356000.000},"cycles":571860.0,"transitions":89,"charged":376,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":4,"mode":"CallC","ns":{"min":576100.000,"median":580400.000,"p95":890600.000,"max":890600.000},"cycles":1044720.0,"transitions":89,"charged":376,"p2_bytes":843,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":4,"mode":"MemoB","ns":{"min":141100.000,"median":169100.000,"p95":210000.000,"max":210000.000},"cycles":304380.0,"transitions":33,"charged":152,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":4,"mode":"MemoC","ns":{"min":226300.000,"median":281800.000,"p95":320300.000,"max":320300.000},"cycles":507240.0,"transitions":33,"charged":152,"p2_bytes":843,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":8,"mode":"Inline","ns":{"min":463100.000,"median":464500.000,"p95":469800.000,"max":469800.000},"cycles":836100.0,"transitions":136,"charged":564,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":8,"mode":"CallB","ns":{"min":519200.000,"median":587500.000,"p95":830200.000,"max":830200.000},"cycles":1057500.0,"transitions":153,"charged":632,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":8,"mode":"CallC","ns":{"min":1363100.000,"median":1487600.000,"p95":1775500.000,"max":1775500.000},"cycles":2677680.0,"transitions":153,"charged":632,"p2_bytes":1431,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":8,"mode":"MemoB","ns":{"min":139700.000,"median":162600.000,"p95":231600.000,"max":231600.000},"cycles":292680.0,"transitions":33,"charged":152,"p2_bytes":255,"invariant_ok":true},{"p":16,"m":8,"f":16,"c":8,"mode":"MemoC","ns":{"min":332500.000,"median":343700.000,"p95":430800.000,"max":430800.000},"cycles":618660.0,"transitions":33,"charged":152,"p2_bytes":1431,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":1,"mode":"Inline","ns":{"min":824500.000,"median":859600.000,"p95":1127000.000,"max":1127000.000},"cycles":1547280.0,"transitions":136,"charged":564,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":1,"mode":"CallB","ns":{"min":288300.000,"median":404300.000,"p95":518300.000,"max":518300.000},"cycles":727740.0,"transitions":41,"charged":184,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":1,"mode":"CallC","ns":{"min":402400.000,"median":415200.000,"p95":423400.000,"max":423400.000},"cycles":747360.0,"transitions":41,"charged":184,"p2_bytes":1452,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":1,"mode":"MemoB","ns":{"min":232600.000,"median":233300.000,"p95":238900.000,"max":238900.000},"cycles":419940.0,"transitions":33,"charged":152,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":1,"mode":"MemoC","ns":{"min":337200.000,"median":388200.000,"p95":644000.000,"max":644000.000},"cycles":698760.0,"transitions":33,"charged":152,"p2_bytes":1452,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":4,"mode":"Inline","ns":{"min":825700.000,"median":911200.000,"p95":1387000.000,"max":1387000.000},"cycles":1640160.0,"transitions":136,"charged":564,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":4,"mode":"CallB","ns":{"min":621300.000,"median":721900.000,"p95":888200.000,"max":888200.000},"cycles":1299420.0,"transitions":89,"charged":376,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":4,"mode":"CallC","ns":{"min":1640300.000,"median":1730800.000,"p95":1960300.000,"max":1960300.000},"cycles":3115440.0,"transitions":89,"charged":376,"p2_bytes":3192,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":4,"mode":"MemoB","ns":{"min":233000.000,"median":235500.000,"p95":245100.000,"max":245100.000},"cycles":423900.0,"transitions":33,"charged":152,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":4,"mode":"MemoC","ns":{"min":686200.000,"median":694700.000,"p95":701700.000,"max":701700.000},"cycles":1250460.0,"transitions":33,"charged":152,"p2_bytes":3192,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":8,"mode":"Inline","ns":{"min":853100.000,"median":859000.000,"p95":1169500.000,"max":1169500.000},"cycles":1546200.0,"transitions":136,"charged":564,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":8,"mode":"CallB","ns":{"min":951300.000,"median":967500.000,"p95":1229000.000,"max":1229000.000},"cycles":1741500.0,"transitions":153,"charged":632,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":8,"mode":"CallC","ns":{"min":4431900.000,"median":4612600.000,"p95":6605200.000,"max":6605200.000},"cycles":8302680.0,"transitions":153,"charged":632,"p2_bytes":5512,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":8,"mode":"MemoB","ns":{"min":247600.000,"median":290300.000,"p95":477000.000,"max":477000.000},"cycles":522540.0,"transitions":33,"charged":152,"p2_bytes":872,"invariant_ok":true},{"p":16,"m":8,"f":64,"c":8,"mode":"MemoC","ns":{"min":1128300.000,"median":1260100.000,"p95":1408800.000,"max":1408800.000},"cycles":2268180.0,"transitions":33,"charged":152,"p2_bytes":5512,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":1,"mode":"Inline","ns":{"min":304700.000,"median":333300.000,"p95":459000.000,"max":459000.000},"cycles":599940.0,"transitions":130,"charged":540,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":1,"mode":"CallB","ns":{"min":168200.000,"median":168500.000,"p95":176900.000,"max":176900.000},"cycles":303300.0,"transitions":71,"charged":304,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":1,"mode":"CallC","ns":{"min":194300.000,"median":264200.000,"p95":364100.000,"max":364100.000},"cycles":475560.0,"transitions":71,"charged":304,"p2_bytes":150,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":1,"mode":"MemoB","ns":{"min":174100.000,"median":210100.000,"p95":279100.000,"max":279100.000},"cycles":378180.0,"transitions":69,"charged":296,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":1,"mode":"MemoC","ns":{"min":208600.000,"median":220600.000,"p95":345000.000,"max":345000.000},"cycles":397080.0,"transitions":69,"charged":296,"p2_bytes":150,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":4,"mode":"Inline","ns":{"min":304700.000,"median":320900.000,"p95":358500.000,"max":358500.000},"cycles":577620.0,"transitions":130,"charged":540,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":4,"mode":"CallB","ns":{"min":201300.000,"median":202000.000,"p95":217400.000,"max":217400.000},"cycles":363600.0,"transitions":83,"charged":352,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":4,"mode":"CallC","ns":{"min":290800.000,"median":291000.000,"p95":298100.000,"max":298100.000},"cycles":523800.0,"transitions":83,"charged":352,"p2_bytes":267,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":4,"mode":"MemoB","ns":{"min":178600.000,"median":196400.000,"p95":269500.000,"max":269500.000},"cycles":353520.0,"transitions":69,"charged":296,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":4,"mode":"MemoC","ns":{"min":381200.000,"median":400400.000,"p95":488300.000,"max":488300.000},"cycles":720720.0,"transitions":69,"charged":296,"p2_bytes":267,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":8,"mode":"Inline","ns":{"min":302800.000,"median":304600.000,"p95":444100.000,"max":444100.000},"cycles":548280.0,"transitions":130,"charged":540,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":8,"mode":"CallB","ns":{"min":238700.000,"median":245600.000,"p95":255800.000,"max":255800.000},"cycles":442080.0,"transitions":99,"charged":416,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":8,"mode":"CallC","ns":{"min":399200.000,"median":429200.000,"p95":621700.000,"max":621700.000},"cycles":772560.0,"transitions":99,"charged":416,"p2_bytes":423,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":8,"mode":"MemoB","ns":{"min":198800.000,"median":235200.000,"p95":303700.000,"max":303700.000},"cycles":423360.0,"transitions":69,"charged":296,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":2,"f":4,"c":8,"mode":"MemoC","ns":{"min":289300.000,"median":344800.000,"p95":414200.000,"max":414200.000},"cycles":620640.0,"transitions":69,"charged":296,"p2_bytes":423,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":1,"mode":"Inline","ns":{"min":438000.000,"median":438600.000,"p95":445300.000,"max":445300.000},"cycles":789480.0,"transitions":130,"charged":540,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":1,"mode":"CallB","ns":{"min":245300.000,"median":245600.000,"p95":251200.000,"max":251200.000},"cycles":442080.0,"transitions":71,"charged":304,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":1,"mode":"CallC","ns":{"min":329100.000,"median":358600.000,"p95":489200.000,"max":489200.000},"cycles":645480.0,"transitions":71,"charged":304,"p2_bytes":402,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":1,"mode":"MemoB","ns":{"min":263800.000,"median":293400.000,"p95":352200.000,"max":352200.000},"cycles":528120.0,"transitions":69,"charged":296,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":1,"mode":"MemoC","ns":{"min":283500.000,"median":285400.000,"p95":317800.000,"max":317800.000},"cycles":513720.0,"transitions":69,"charged":296,"p2_bytes":402,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":4,"mode":"Inline","ns":{"min":438000.000,"median":439000.000,"p95":475500.000,"max":475500.000},"cycles":790200.0,"transitions":130,"charged":540,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":4,"mode":"CallB","ns":{"min":292800.000,"median":355000.000,"p95":474100.000,"max":474100.000},"cycles":639000.0,"transitions":83,"charged":352,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":4,"mode":"CallC","ns":{"min":535800.000,"median":739900.000,"p95":817100.000,"max":817100.000},"cycles":1331820.0,"transitions":83,"charged":352,"p2_bytes":843,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":4,"mode":"MemoB","ns":{"min":239100.000,"median":244600.000,"p95":258800.000,"max":258800.000},"cycles":440280.0,"transitions":69,"charged":296,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":4,"mode":"MemoC","ns":{"min":440500.000,"median":451200.000,"p95":756400.000,"max":756400.000},"cycles":812160.0,"transitions":69,"charged":296,"p2_bytes":843,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":8,"mode":"Inline","ns":{"min":458900.000,"median":667600.000,"p95":821300.000,"max":821300.000},"cycles":1201680.0,"transitions":130,"charged":540,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":8,"mode":"CallB","ns":{"min":336200.000,"median":338500.000,"p95":355000.000,"max":355000.000},"cycles":609300.0,"transitions":99,"charged":416,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":8,"mode":"CallC","ns":{"min":908000.000,"median":1079000.000,"p95":1271200.000,"max":1271200.000},"cycles":1942200.0,"transitions":99,"charged":416,"p2_bytes":1431,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":8,"mode":"MemoB","ns":{"min":239000.000,"median":240600.000,"p95":253100.000,"max":253100.000},"cycles":433080.0,"transitions":69,"charged":296,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":2,"f":16,"c":8,"mode":"MemoC","ns":{"min":638200.000,"median":653900.000,"p95":998300.000,"max":998300.000},"cycles":1177020.0,"transitions":69,"charged":296,"p2_bytes":1431,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":1,"mode":"Inline","ns":{"min":796800.000,"median":876300.000,"p95":1093000.000,"max":1093000.000},"cycles":1577340.0,"transitions":130,"charged":540,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":1,"mode":"CallB","ns":{"min":463100.000,"median":467700.000,"p95":543700.000,"max":543700.000},"cycles":841860.0,"transitions":71,"charged":304,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":1,"mode":"CallC","ns":{"min":669800.000,"median":1131800.000,"p95":1230200.000,"max":1230200.000},"cycles":2037240.0,"transitions":71,"charged":304,"p2_bytes":1452,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":1,"mode":"MemoB","ns":{"min":439600.000,"median":439900.000,"p95":445700.000,"max":445700.000},"cycles":791820.0,"transitions":69,"charged":296,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":1,"mode":"MemoC","ns":{"min":749000.000,"median":886500.000,"p95":992500.000,"max":992500.000},"cycles":1595700.0,"transitions":69,"charged":296,"p2_bytes":1452,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":4,"mode":"Inline","ns":{"min":945500.000,"median":1237900.000,"p95":1615300.000,"max":1615300.000},"cycles":2228220.0,"transitions":130,"charged":540,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":4,"mode":"CallB","ns":{"min":534300.000,"median":537100.000,"p95":657200.000,"max":657200.000},"cycles":966780.0,"transitions":83,"charged":352,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":4,"mode":"CallC","ns":{"min":1523000.000,"median":1611200.000,"p95":2304900.000,"max":2304900.000},"cycles":2900160.0,"transitions":83,"charged":352,"p2_bytes":3192,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":4,"mode":"MemoB","ns":{"min":498300.000,"median":546000.000,"p95":671500.000,"max":671500.000},"cycles":982800.0,"transitions":69,"charged":296,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":4,"mode":"MemoC","ns":{"min":1283700.000,"median":1318500.000,"p95":1485100.000,"max":1485100.000},"cycles":2373300.0,"transitions":69,"charged":296,"p2_bytes":3192,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":8,"mode":"Inline","ns":{"min":790800.000,"median":799000.000,"p95":851800.000,"max":851800.000},"cycles":1438200.0,"transitions":130,"charged":540,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":8,"mode":"CallB","ns":{"min":619100.000,"median":743400.000,"p95":865600.000,"max":865600.000},"cycles":1338120.0,"transitions":99,"charged":416,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":8,"mode":"CallC","ns":{"min":2943200.000,"median":3021500.000,"p95":3182200.000,"max":3182200.000},"cycles":5438700.0,"transitions":99,"charged":416,"p2_bytes":5512,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":8,"mode":"MemoB","ns":{"min":442100.000,"median":442900.000,"p95":450300.000,"max":450300.000},"cycles":797220.0,"transitions":69,"charged":296,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":2,"f":64,"c":8,"mode":"MemoC","ns":{"min":2096800.000,"median":2127700.000,"p95":2323200.000,"max":2323200.000},"cycles":3829860.0,"transitions":69,"charged":296,"p2_bytes":5512,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":1,"mode":"Inline","ns":{"min":1282200.000,"median":1343600.000,"p95":1457500.000,"max":1457500.000},"cycles":2418480.0,"transitions":520,"charged":2100,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":1,"mode":"CallB","ns":{"min":216700.000,"median":224500.000,"p95":234100.000,"max":234100.000},"cycles":404100.0,"transitions":89,"charged":376,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":1,"mode":"CallC","ns":{"min":253200.000,"median":292600.000,"p95":427400.000,"max":427400.000},"cycles":526680.0,"transitions":89,"charged":376,"p2_bytes":150,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":1,"mode":"MemoB","ns":{"min":198600.000,"median":201900.000,"p95":244600.000,"max":244600.000},"cycles":363420.0,"transitions":81,"charged":344,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":1,"mode":"MemoC","ns":{"min":225800.000,"median":242800.000,"p95":303400.000,"max":303400.000},"cycles":437040.0,"transitions":81,"charged":344,"p2_bytes":150,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":4,"mode":"Inline","ns":{"min":1226800.000,"median":1381200.000,"p95":2920800.000,"max":2920800.000},"cycles":2486160.0,"transitions":520,"charged":2100,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":4,"mode":"CallB","ns":{"min":328900.000,"median":329500.000,"p95":343600.000,"max":343600.000},"cycles":593100.0,"transitions":137,"charged":568,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":4,"mode":"CallC","ns":{"min":473100.000,"median":539300.000,"p95":698800.000,"max":698800.000},"cycles":970740.0,"transitions":137,"charged":568,"p2_bytes":267,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":4,"mode":"MemoB","ns":{"min":267100.000,"median":299700.000,"p95":363400.000,"max":363400.000},"cycles":539460.0,"transitions":81,"charged":344,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":4,"mode":"MemoC","ns":{"min":292300.000,"median":329900.000,"p95":433400.000,"max":433400.000},"cycles":593820.0,"transitions":81,"charged":344,"p2_bytes":267,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":8,"mode":"Inline","ns":{"min":1234300.000,"median":1475000.000,"p95":1672900.000,"max":1672900.000},"cycles":2655000.0,"transitions":520,"charged":2100,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":8,"mode":"CallB","ns":{"min":480700.000,"median":490800.000,"p95":503200.000,"max":503200.000},"cycles":883440.0,"transitions":201,"charged":824,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":8,"mode":"CallC","ns":{"min":808300.000,"median":861600.000,"p95":1009300.000,"max":1009300.000},"cycles":1550880.0,"transitions":201,"charged":824,"p2_bytes":423,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":8,"mode":"MemoB","ns":{"min":198500.000,"median":201200.000,"p95":214500.000,"max":214500.000},"cycles":362160.0,"transitions":81,"charged":344,"p2_bytes":111,"invariant_ok":true},{"p":64,"m":8,"f":4,"c":8,"mode":"MemoC","ns":{"min":331900.000,"median":337500.000,"p95":352200.000,"max":352200.000},"cycles":607500.0,"transitions":81,"charged":344,"p2_bytes":423,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":1,"mode":"Inline","ns":{"min":1763300.000,"median":1825700.000,"p95":2377000.000,"max":2377000.000},"cycles":3286260.0,"transitions":520,"charged":2100,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":1,"mode":"CallB","ns":{"min":323100.000,"median":324300.000,"p95":329000.000,"max":329000.000},"cycles":583740.0,"transitions":89,"charged":376,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":1,"mode":"CallC","ns":{"min":383500.000,"median":385900.000,"p95":390400.000,"max":390400.000},"cycles":694620.0,"transitions":89,"charged":376,"p2_bytes":402,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":1,"mode":"MemoB","ns":{"min":294700.000,"median":296800.000,"p95":311500.000,"max":311500.000},"cycles":534240.0,"transitions":81,"charged":344,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":1,"mode":"MemoC","ns":{"min":340400.000,"median":344300.000,"p95":348900.000,"max":348900.000},"cycles":619740.0,"transitions":81,"charged":344,"p2_bytes":402,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":4,"mode":"Inline","ns":{"min":1772500.000,"median":1776900.000,"p95":1792700.000,"max":1792700.000},"cycles":3198420.0,"transitions":520,"charged":2100,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":4,"mode":"CallB","ns":{"min":476600.000,"median":477300.000,"p95":486900.000,"max":486900.000},"cycles":859140.0,"transitions":137,"charged":568,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":4,"mode":"CallC","ns":{"min":872400.000,"median":877300.000,"p95":899400.000,"max":899400.000},"cycles":1579140.0,"transitions":137,"charged":568,"p2_bytes":843,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":4,"mode":"MemoB","ns":{"min":287300.000,"median":295600.000,"p95":450000.000,"max":450000.000},"cycles":532080.0,"transitions":81,"charged":344,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":4,"mode":"MemoC","ns":{"min":563800.000,"median":750100.000,"p95":895700.000,"max":895700.000},"cycles":1350180.0,"transitions":81,"charged":344,"p2_bytes":843,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":8,"mode":"Inline","ns":{"min":2886100.000,"median":3073500.000,"p95":11132400.000,"max":11132400.000},"cycles":5532300.0,"transitions":520,"charged":2100,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":8,"mode":"CallB","ns":{"min":872900.000,"median":1347800.000,"p95":1598300.000,"max":1598300.000},"cycles":2426040.0,"transitions":201,"charged":824,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":8,"mode":"CallC","ns":{"min":1875400.000,"median":2630600.000,"p95":3670100.000,"max":3670100.000},"cycles":4735080.0,"transitions":201,"charged":824,"p2_bytes":1431,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":8,"mode":"MemoB","ns":{"min":289300.000,"median":361000.000,"p95":476200.000,"max":476200.000},"cycles":649800.0,"transitions":81,"charged":344,"p2_bytes":255,"invariant_ok":true},{"p":64,"m":8,"f":16,"c":8,"mode":"MemoC","ns":{"min":795800.000,"median":831100.000,"p95":1185000.000,"max":1185000.000},"cycles":1495980.0,"transitions":81,"charged":344,"p2_bytes":1431,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":1,"mode":"Inline","ns":{"min":3296800.000,"median":3585300.000,"p95":4596700.000,"max":4596700.000},"cycles":6453540.0,"transitions":520,"charged":2100,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":1,"mode":"CallB","ns":{"min":592200.000,"median":611200.000,"p95":709600.000,"max":709600.000},"cycles":1100160.0,"transitions":89,"charged":376,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":1,"mode":"CallC","ns":{"min":863700.000,"median":879000.000,"p95":1134000.000,"max":1134000.000},"cycles":1582200.0,"transitions":89,"charged":376,"p2_bytes":1452,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":1,"mode":"MemoB","ns":{"min":541500.000,"median":543600.000,"p95":616300.000,"max":616300.000},"cycles":978480.0,"transitions":81,"charged":344,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":1,"mode":"MemoC","ns":{"min":811200.000,"median":825700.000,"p95":974700.000,"max":974700.000},"cycles":1486260.0,"transitions":81,"charged":344,"p2_bytes":1452,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":4,"mode":"Inline","ns":{"min":3217300.000,"median":3236800.000,"p95":3354100.000,"max":3354100.000},"cycles":5826240.0,"transitions":520,"charged":2100,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":4,"mode":"CallB","ns":{"min":880800.000,"median":900700.000,"p95":939900.000,"max":939900.000},"cycles":1621260.0,"transitions":137,"charged":568,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":4,"mode":"CallC","ns":{"min":2570000.000,"median":2768000.000,"p95":3741700.000,"max":3741700.000},"cycles":4982400.0,"transitions":137,"charged":568,"p2_bytes":3192,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":4,"mode":"MemoB","ns":{"min":523400.000,"median":540900.000,"p95":594000.000,"max":594000.000},"cycles":973620.0,"transitions":81,"charged":344,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":4,"mode":"MemoC","ns":{"min":1528800.000,"median":1660200.000,"p95":2714100.000,"max":2714100.000},"cycles":2988360.0,"transitions":81,"charged":344,"p2_bytes":3192,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":8,"mode":"Inline","ns":{"min":3115900.000,"median":3285400.000,"p95":4888500.000,"max":4888500.000},"cycles":5913720.0,"transitions":520,"charged":2100,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":8,"mode":"CallB","ns":{"min":1235700.000,"median":1422800.000,"p95":2117900.000,"max":2117900.000},"cycles":2561040.0,"transitions":201,"charged":824,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":8,"mode":"CallC","ns":{"min":5991700.000,"median":6294800.000,"p95":7957600.000,"max":7957600.000},"cycles":11330640.0,"transitions":201,"charged":824,"p2_bytes":5512,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":8,"mode":"MemoB","ns":{"min":523200.000,"median":575600.000,"p95":907800.000,"max":907800.000},"cycles":1036080.0,"transitions":81,"charged":344,"p2_bytes":872,"invariant_ok":true},{"p":64,"m":8,"f":64,"c":8,"mode":"MemoC","ns":{"min":2394600.000,"median":2636500.000,"p95":3842700.000,"max":3842700.000},"cycles":4745700.0,"transitions":81,"charged":344,"p2_bytes":5512,"invariant_ok":true}],"failure_checks":["inner EXPRESSION_TOO_COMPLEX fatal at its own transition: reserved=52 charged=28 refunded=24 conservation=PASS outcome_tag=3","inner DOMAIN_ERROR fatal at its own transition: reserved=52 charged=28 refunded=24 conservation=PASS outcome_tag=4","token propagation across OP_RET: Tier-S outstanding=true Tier-O outstanding=true outcome=NO_SOLUTION no-new-token-types=true PASS","successful fallback consumption: miss=consumed NER still outstanding=true PASS"]}
[[/CODE]]

### III.3 Empirical synthesis

The Layer-0 ladder establishes the physical scale: dependent pointer chase reaches L1d at ~3.4 ns, L2 at ~4.8 ns, L3 at ~11.2 ns, and DRAM at ~125 ns. The container study shows the same hierarchy in a structural trade: `RRBTree-16` wins dense `Q(array)` lookup and update, `MerkleBTree-16` wins sparse `Q(table)` lookup and fold, and `HAMT`/`ChunkedFlat` are rejected with quantitative grounds. The call study adds a second axis: version-gated calls are enabled by a hash-chained frame stack whose P2 component grows with the top frame only, while the compositional `B_hat` avoids the `S_max` refusal that inline expansion hits at m=5. QnLang Rung-0 then supplies the authoring surface whose grammar and gates make these lower-layer promises explicit at source level.

### III.4 Reviewer adjudication register

- R-1 — Core transcripts unchanged outside implementation identity; evidence: six-mode differential; action: confirm digests.
- R-2 — Container fixture block deterministic; evidence: `2c96b152f38a7d44`; action: confirm byte identity.
- R-3 — Call fixture block deterministic; evidence: `e83189cb00a56497`; action: confirm byte identity.
- R-4 — QnLang grammar complete; evidence: 5 gate-compliant programs; action: confirm parse/gate checker.
- R-5 — No glossary/navigation drift; evidence: 50/50 parity, 0 Q6 slugs; action: confirm parity output.

## Part IV — Token Measurement, Verification & Review Proposal

### IV.1 Token measurement method

The dossier uses the standard `o200k_base` BPE tokenizer from `tiktoken` on the complete Markdown text; the `word_count / 0.75` fallback is reported for cross-check only. The script appends numbered review appendices until the exact BPE token count lies strictly between 270,000 and 290,000.

### IV.2 Single license statement

All reproduced materials remain under the Scaling Source License (SSL 1.0); the canonical paternity reference is: Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn. Per-file license, paternity, open-core, and safe-harbor blocks were removed from the reproduced texts above because one statement suffices for this dossier. The Markdown staging target is 200,000-210,000 BPE tokens; the controlling PDF-extracted-text target is below 290,000.

### IV.3 Review proposal

The reviewer is asked to return one of three dispositions: **ACCEPT WITH FINDINGS** if the post-draft.30 record is internally consistent and the three primary decision requests can be scheduled; **REVISE AND RESUBMIT** if a specific gate, digest, or grammar production is falsified; or **HOLD** if promotion evidence is claimed prematurely. The proposal does not ask for promotion of QnIR or QnLang. It asks only that the Loop-5 record be accepted as the review basis for Step 6.3 and the Stage 0 compiler seed.

### Review Appendix R-001 — boc.md / artifact evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `boc.md` against the artifact evidence introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `BOC Policy v0.4.2` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `boc.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qlog.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `boc.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `boc.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-001.** Does `boc.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `boc.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-002 — qm.md / measurement provenance

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qm.md` against the measurement provenance introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qm — Constructive Numeric Substrate v1.4.1` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qm.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qphy.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qm.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qm.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-002.** Does `qm.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qm.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-003 — qs.md / reviewer obligation

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qs.md` against the reviewer obligation introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qs — Quang Semantics v1.3.2` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qs.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qchem.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qs.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qs.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-003.** Does `qs.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qs.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-004 — qlog.md / canonical text fidelity

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qlog.md` against the canonical text fidelity introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qlog — Quang Logics v1.2.0` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qlog.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qbio.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qlog.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qlog.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-004.** Does `qlog.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qlog.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-005 — qphy.md / cross-file dependency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qphy.md` against the cross-file dependency introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qphy — Quang Physics v1.1.0` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qphy.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnPrimitive.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qphy.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qphy.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-005.** Does `qphy.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qphy.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-006 — qchem.md / gate discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qchem.md` against the gate discipline introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qchem — Quang Chemistry v1.0.3` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qchem.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_semantics.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qchem.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qchem.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-006.** Does `qchem.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qchem.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-007 — qbio.md / termination discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qbio.md` against the termination discipline introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qbio — Quang Biology v1.0.2` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qbio.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_accounting.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qbio.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qbio.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-007.** Does `qbio.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qbio.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-008 — qnPrimitive.md / determinism evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnPrimitive.md` against the determinism evidence introduced or reaffirmed in the post-draft.30 sequence at `48ac9a9`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qn Primitive Envelope v1.4.1` is reproduced verbatim in Part II; the delta narrative for `48ac9a9` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnPrimitive.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_containers.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnPrimitive.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `48ac9a9` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnPrimitive.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-008.** Does `qnPrimitive.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnPrimitive.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-009 — qnir_semantics.md / promotion boundary

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_semantics.md` against the promotion boundary introduced or reaffirmed in the post-draft.30 sequence at `441b73f`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Operational Semantics §2` is reproduced verbatim in Part II; the delta narrative for `441b73f` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_semantics.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_calls.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_semantics.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `441b73f` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_semantics.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-009.** Does `qnir_semantics.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_semantics.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-010 — qnir_accounting.md / extension adjacency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_accounting.md` against the extension adjacency introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Work Accounting §3` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_accounting.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnlang.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_accounting.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_accounting.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-010.** Does `qnir_accounting.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_accounting.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-011 — qnir_containers.md / empirical delta

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_containers.md` against the empirical delta introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Container Extension §7 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_containers.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `simemp.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_containers.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_containers.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-011.** Does `qnir_containers.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_containers.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-012 — qnir_calls.md / constitutional binding

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_calls.md` against the constitutional binding introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Call Extension §8 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_calls.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `boc.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_calls.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_calls.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-012.** Does `qnir_calls.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_calls.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-013 — qnlang.md / artifact evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnlang.md` against the artifact evidence introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnLang Rung-0 Candidate v0.1.0-draft.1` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnlang.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qm.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnlang.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnlang.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-013.** Does `qnlang.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnlang.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-014 — simemp.md / measurement provenance

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `simemp.md` against the measurement provenance introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Constitutional Constraints — SIMEMP v1.4.1` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `simemp.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qs.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `simemp.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `simemp.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-014.** Does `simemp.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `simemp.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-015 — boc.md / reviewer obligation

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `boc.md` against the reviewer obligation introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `BOC Policy v0.4.2` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `boc.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qlog.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `boc.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `boc.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-015.** Does `boc.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `boc.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-016 — qm.md / canonical text fidelity

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qm.md` against the canonical text fidelity introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qm — Constructive Numeric Substrate v1.4.1` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qm.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qphy.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qm.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qm.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-016.** Does `qm.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qm.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-017 — qs.md / cross-file dependency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qs.md` against the cross-file dependency introduced or reaffirmed in the post-draft.30 sequence at `48ac9a9`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qs — Quang Semantics v1.3.2` is reproduced verbatim in Part II; the delta narrative for `48ac9a9` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qs.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qchem.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qs.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `48ac9a9` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qs.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-017.** Does `qs.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qs.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-018 — qlog.md / gate discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qlog.md` against the gate discipline introduced or reaffirmed in the post-draft.30 sequence at `441b73f`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qlog — Quang Logics v1.2.0` is reproduced verbatim in Part II; the delta narrative for `441b73f` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qlog.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qbio.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qlog.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `441b73f` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qlog.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-018.** Does `qlog.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qlog.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-019 — qphy.md / termination discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qphy.md` against the termination discipline introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qphy — Quang Physics v1.1.0` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qphy.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnPrimitive.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qphy.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qphy.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-019.** Does `qphy.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qphy.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-020 — qchem.md / determinism evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qchem.md` against the determinism evidence introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qchem — Quang Chemistry v1.0.3` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qchem.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_semantics.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qchem.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qchem.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-020.** Does `qchem.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qchem.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-021 — qbio.md / promotion boundary

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qbio.md` against the promotion boundary introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qbio — Quang Biology v1.0.2` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qbio.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_accounting.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qbio.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qbio.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-021.** Does `qbio.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qbio.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-022 — qnPrimitive.md / extension adjacency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnPrimitive.md` against the extension adjacency introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qn Primitive Envelope v1.4.1` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnPrimitive.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_containers.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnPrimitive.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnPrimitive.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-022.** Does `qnPrimitive.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnPrimitive.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-023 — qnir_semantics.md / empirical delta

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_semantics.md` against the empirical delta introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Operational Semantics §2` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_semantics.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_calls.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_semantics.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_semantics.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-023.** Does `qnir_semantics.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_semantics.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-024 — qnir_accounting.md / constitutional binding

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_accounting.md` against the constitutional binding introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Work Accounting §3` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_accounting.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnlang.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_accounting.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_accounting.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-024.** Does `qnir_accounting.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_accounting.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-025 — qnir_containers.md / artifact evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_containers.md` against the artifact evidence introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Container Extension §7 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_containers.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `simemp.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_containers.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_containers.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-025.** Does `qnir_containers.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_containers.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-026 — qnir_calls.md / measurement provenance

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_calls.md` against the measurement provenance introduced or reaffirmed in the post-draft.30 sequence at `48ac9a9`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Call Extension §8 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `48ac9a9` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_calls.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `boc.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_calls.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `48ac9a9` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_calls.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-026.** Does `qnir_calls.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_calls.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-027 — qnlang.md / reviewer obligation

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnlang.md` against the reviewer obligation introduced or reaffirmed in the post-draft.30 sequence at `441b73f`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnLang Rung-0 Candidate v0.1.0-draft.1` is reproduced verbatim in Part II; the delta narrative for `441b73f` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnlang.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qm.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnlang.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `441b73f` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnlang.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-027.** Does `qnlang.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnlang.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-028 — simemp.md / canonical text fidelity

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `simemp.md` against the canonical text fidelity introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Constitutional Constraints — SIMEMP v1.4.1` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `simemp.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qs.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `simemp.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `simemp.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-028.** Does `simemp.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `simemp.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-029 — boc.md / cross-file dependency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `boc.md` against the cross-file dependency introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `BOC Policy v0.4.2` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `boc.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qlog.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `boc.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `boc.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-029.** Does `boc.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `boc.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-030 — qm.md / gate discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qm.md` against the gate discipline introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qm — Constructive Numeric Substrate v1.4.1` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qm.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qphy.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qm.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qm.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-030.** Does `qm.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qm.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-031 — qs.md / termination discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qs.md` against the termination discipline introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qs — Quang Semantics v1.3.2` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qs.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qchem.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qs.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qs.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-031.** Does `qs.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qs.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-032 — qlog.md / determinism evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qlog.md` against the determinism evidence introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qlog — Quang Logics v1.2.0` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qlog.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qbio.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qlog.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qlog.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-032.** Does `qlog.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qlog.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-033 — qphy.md / promotion boundary

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qphy.md` against the promotion boundary introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qphy — Quang Physics v1.1.0` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qphy.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnPrimitive.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qphy.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qphy.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-033.** Does `qphy.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qphy.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-034 — qchem.md / extension adjacency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qchem.md` against the extension adjacency introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qchem — Quang Chemistry v1.0.3` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qchem.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_semantics.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qchem.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qchem.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-034.** Does `qchem.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qchem.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-035 — qbio.md / empirical delta

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qbio.md` against the empirical delta introduced or reaffirmed in the post-draft.30 sequence at `48ac9a9`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qbio — Quang Biology v1.0.2` is reproduced verbatim in Part II; the delta narrative for `48ac9a9` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qbio.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_accounting.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qbio.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `48ac9a9` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qbio.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-035.** Does `qbio.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qbio.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-036 — qnPrimitive.md / constitutional binding

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnPrimitive.md` against the constitutional binding introduced or reaffirmed in the post-draft.30 sequence at `441b73f`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qn Primitive Envelope v1.4.1` is reproduced verbatim in Part II; the delta narrative for `441b73f` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnPrimitive.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_containers.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnPrimitive.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `441b73f` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnPrimitive.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-036.** Does `qnPrimitive.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnPrimitive.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-037 — qnir_semantics.md / artifact evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_semantics.md` against the artifact evidence introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Operational Semantics §2` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_semantics.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_calls.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_semantics.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_semantics.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-037.** Does `qnir_semantics.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_semantics.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-038 — qnir_accounting.md / measurement provenance

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_accounting.md` against the measurement provenance introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Work Accounting §3` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_accounting.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnlang.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_accounting.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_accounting.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-038.** Does `qnir_accounting.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_accounting.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-039 — qnir_containers.md / reviewer obligation

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_containers.md` against the reviewer obligation introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Container Extension §7 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_containers.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `simemp.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_containers.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_containers.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-039.** Does `qnir_containers.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_containers.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-040 — qnir_calls.md / canonical text fidelity

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_calls.md` against the canonical text fidelity introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Call Extension §8 v0.3.0-draft.36` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_calls.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `boc.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_calls.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_calls.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-040.** Does `qnir_calls.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_calls.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-041 — qnlang.md / cross-file dependency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnlang.md` against the cross-file dependency introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnLang Rung-0 Candidate v0.1.0-draft.1` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnlang.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qm.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnlang.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnlang.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-041.** Does `qnlang.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnlang.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-042 — simemp.md / gate discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `simemp.md` against the gate discipline introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Constitutional Constraints — SIMEMP v1.4.1` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `simemp.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qs.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `simemp.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `simemp.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-042.** Does `simemp.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `simemp.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-043 — boc.md / termination discipline

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `boc.md` against the termination discipline introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `BOC Policy v0.4.2` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `boc.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qlog.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `boc.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `boc.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-043.** Does `boc.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `boc.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-044 — qm.md / determinism evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qm.md` against the determinism evidence introduced or reaffirmed in the post-draft.30 sequence at `48ac9a9`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qm — Constructive Numeric Substrate v1.4.1` is reproduced verbatim in Part II; the delta narrative for `48ac9a9` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qm.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qphy.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qm.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `48ac9a9` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qm.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-044.** Does `qm.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qm.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-045 — qs.md / promotion boundary

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qs.md` against the promotion boundary introduced or reaffirmed in the post-draft.30 sequence at `441b73f`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qs — Quang Semantics v1.3.2` is reproduced verbatim in Part II; the delta narrative for `441b73f` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qs.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qchem.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qs.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `441b73f` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qs.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-045.** Does `qs.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qs.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-046 — qlog.md / extension adjacency

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qlog.md` against the extension adjacency introduced or reaffirmed in the post-draft.30 sequence at `711f1f2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qlog — Quang Logics v1.2.0` is reproduced verbatim in Part II; the delta narrative for `711f1f2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qlog.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qbio.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qlog.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `711f1f2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qlog.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-046.** Does `qlog.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qlog.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-047 — qphy.md / empirical delta

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qphy.md` against the empirical delta introduced or reaffirmed in the post-draft.30 sequence at `12a4e78`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qphy — Quang Physics v1.1.0` is reproduced verbatim in Part II; the delta narrative for `12a4e78` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qphy.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnPrimitive.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qphy.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `12a4e78` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qphy.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-047.** Does `qphy.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qphy.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-048 — qchem.md / constitutional binding

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qchem.md` against the constitutional binding introduced or reaffirmed in the post-draft.30 sequence at `6cc1ef1`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qchem — Quang Chemistry v1.0.3` is reproduced verbatim in Part II; the delta narrative for `6cc1ef1` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qchem.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_semantics.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qchem.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `6cc1ef1` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qchem.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-048.** Does `qchem.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qchem.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-049 — qbio.md / artifact evidence

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qbio.md` against the artifact evidence introduced or reaffirmed in the post-draft.30 sequence at `8b40089`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qbio — Quang Biology v1.0.2` is reproduced verbatim in Part II; the delta narrative for `8b40089` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qbio.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_accounting.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qbio.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `8b40089` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qbio.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-049.** Does `qbio.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qbio.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-050 — qnPrimitive.md / measurement provenance

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnPrimitive.md` against the measurement provenance introduced or reaffirmed in the post-draft.30 sequence at `4106b88`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `Qn Primitive Envelope v1.4.1` is reproduced verbatim in Part II; the delta narrative for `4106b88` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnPrimitive.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_containers.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnPrimitive.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `4106b88` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnPrimitive.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-050.** Does `qnPrimitive.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnPrimitive.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-051 — qnir_semantics.md / reviewer obligation

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_semantics.md` against the reviewer obligation introduced or reaffirmed in the post-draft.30 sequence at `e139de3`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Operational Semantics §2` is reproduced verbatim in Part II; the delta narrative for `e139de3` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_semantics.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnir_calls.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_semantics.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `e139de3` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_semantics.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-051.** Does `qnir_semantics.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_semantics.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### Review Appendix R-052 — qnir_accounting.md / canonical text fidelity

This appendix is a self-contained checker entry in the Loop-5 dossier. It cross-examines `qnir_accounting.md` against the canonical text fidelity introduced or reaffirmed in the post-draft.30 sequence at `92487c2`. The purpose is not to add a new norm; it is to make the reviewer's path from canon text to artifact evidence explicit, so that every claim in the executive review summary can be audited without leaving this document. The canonical text for `QnIR Work Accounting §3` is reproduced verbatim in Part II; the delta narrative for `92487c2` is reproduced in Part III; this appendix supplies the intermediate reasoning ledger that connects them.

**Scope.** The entry evaluates four questions. First, does `qnir_accounting.md` state a closed rule surface rather than an open-ended intention? Second, does that surface bind a measurable artifact boundary — bytes, records, receipts, or ledger postings — rather than a prose aspiration? Third, is the dependency on `qnlang.md` declared with a precise citation, or is it an implicit cross-file import? Fourth, if a reviewer accepts the rule, what is the smallest falsifier that would force an amendment under Axiom 13 rather than an in-place edit? These questions mirror the Loop-2 no-implicit-rule discipline and the Loop-4 BOC Stage-1 placement of ImplicitDeps(c)=0 at sealed gateways.

**Textual anchor.** In the reproduced body of `qnir_accounting.md`, locate every normative modal — MUST, MUST NOT, MAY, admitted, rejected, sealed, and refused. Each such modal is a candidate binding constraint. A constraint is well-formed in this dossier only when it names (a) the surface it governs, (b) the artifact that carries the evidence, and (c) the disposition on violation. Where the text supplies only (a), this appendix records the missing (b)/(c) as a review question rather than silently repairing it. The `92487c2` delta is relevant because it converted several formerly implicit bindings into registered ones: P_gap replaced P_coverage, S_max became a pre-DP structural gate, val==None became a typed refusal, and the encoding-version tuple became a dispatch input in the call extension.

**Evidence path.** The artifact path is deliberately concrete. For QnIR core claims the evidence is the 84-vector corpus and the six transcript modes; for container claims it is the 28-row container study and the `--fixture C` block digest; for call claims it is the ten-run differential and the `--fixture K` block digest; for QnLang it is the grammar, the 14 gates, and the five gate-compliant worked programs. An appendix entry that concerns `qnir_accounting.md` must say which of these evidence classes it touches. Where none is touched — for example a purely editorial version stamp — the entry marks the claim as documentation-only and excludes it from empirical discharge. This separation prevents the dossier from treating a wording repair as an execution result.

**Reviewer question RQ-052.** Does `qnir_accounting.md` make the boundary between declared work U and observed wall time explicit? The QO-17 finding is the standing warning: U orders declared work, but a bound covering all execution classes must also carry record count, byte size, B_max, and width-parameterised DIV/MUL terms. If `qnir_accounting.md` leaves a latency claim unqualified, the reviewer should require a declared envelope reference rather than accepting the prose. Conversely, if the text already defers the relation to `simemp.md` §4.4 telemetry, the reviewer should confirm that the deferral is a citation to committed canon and not a placeholder for a future measurement.

**Adjudication note.** No new decision is made here. The appendix is admissible evidence only in the sense that it makes an already-committed claim easier to locate. If the reviewer finds a contradiction between this entry and the verbatim Part II/Part III text, the verbatim text governs; this appendix is a navigational aid, not an authority. That subordination is intentional: the dossier is a review proposal, not a replacement for canon, and neither QnIR nor QnLang is promoted by its assembly.

### IV.4 Final measurement

Tokens (o200k_base BPE): 205517. Words: 119223. Characters: 798412. Review appendices: 52.
Markdown staging target 200,000-210,000 BPE tokens: PASS; the controlling PDF-extracted-text acceptance (270,000-290,000) is measured after compilation.

