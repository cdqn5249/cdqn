---
layout: default
title: Qm — Quang Mathematics (Constructive Numeric and Relational Substrate)
description: Canonical specification of the constructive numeric, relational, and algebraic substrate
version: 1.4.0
updated: 2026-10-30
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qm.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qm.html
terms_used:
  - abstraction-layer
  - axiom-6
  - causal-arrow
  - computational-consistency
  - compute-unit-u
  - dcc-profile
  - dependencies-determinism
  - dimension-d
  - layer-1
  - licensed-work
  - local-first
  - metric-envelope
  - metric-exhaustion
  - no-implicit-rule
  - paternity-reference
  - payload
  - q-anchor
  - q-bypass
  - q-reuse
  - q0
  - q1
  - qexpr
  - qm
  - qn
  - qnir
  - qnlang
  - receipt
  - remainder-r
  - simemp
  - simemp-gateway
  - ssl
  - structural-indirection
  - terminal-exactness
  - universal-envelope
  - zoom-z
---

# Qm — Quang Mathematics: Constructive Numeric and Relational Substrate

| Field | Specification |
|---|---|
| **Document Title** | Qm — Quang Mathematics: Constructive Numeric and Relational Substrate |
| **Version** | 1.4.0 |
| **Last Updated** | 2026-10-27 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category D (Local-First Base Domain) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing $$\mathrm{Qm}$$. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs. References are deliberately version-unpinned: each target document's front matter carries its own authoritative version, so this specification never duplicates drift-prone patch state.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and [Optimization Morphisms]({{ '/glossary.html' | relative_url }}#q-reuse) | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |

---

## Review Integration Record (v1.3.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.1.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.1.md) — transcript of the external adversarial review (Google Deep Review, October 2026) of the condensed Qn universe specification and QnIR v0.1 candidate draft. This update integrates the numeric-substrate findings of that review into the canon. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place. New constructions and semantic refinements introduced below are tagged **[v1.3.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| $$\kappa(z)$$ bound per-artifact in the DCC profile; three canonical schedules declared; undeclared-$$\kappa$$ rejection | §3.3 | QO-4 / Segment 2 Point 1 (fatal) | CANDIDATE |
| Residual typing: collapsed integer vs. commitment handle to a Layer-3 generator | §3.4 | QO-3 / Segment 2 Point 2 | CANDIDATE |
| **Multi-digit natural construction** — the Radix-Anchor Morphism (cons || Q(9) gap closed) | §3.6 (new) | Objection 14 / QO-4 (fatal) | CANDIDATE |
| Incremental digits vs. consolidated quotient; zoom-alignment rule | §4.1 | Segment 2 Point 2 | CANDIDATE |
| Canonical zoom normalization before hashing (one commitment per value) | §4.3 (new) | Segment 4 Point 7 | CANDIDATE |
| Euclidean division orientation ($$\sigma_R \equiv \oplus$$ fixed); quotient-sign guard | §5.3 | Segment 2 Point 4 | CANDIDATE |
| Linear division vs. DPRM scope separation; "Decidability" renamed "Totality"; `HEIGHT_BOX_EXHAUSTED` reason code; `NO_SOLUTION` reserved | §5.3 | Segment 2 Point 4 / Objection 4 | editorial + semantic |
| Height log-origin guard $$h(u/v)$$ | §5.3 | Minor correction | editorial |
| Even-root boundary restriction ($$n \ge Q(4)$$ → spinor promotion, Degree 2) | §5.5.IV | Segment 2 Point 4 | CANDIDATE |
| Canonical negative rationals via GCD-reduced `OP_RATIO` pairs | §5.7 (new) | QO-4 part 3 | CANDIDATE |
| Sign-aware constructive enclosure intervals; undefined $$\epsilon_z$$ retired | §6 | Segment 2 Point 4 | CANDIDATE |
| "Neutralized" phrasing aligned to termination-not-decidability | §1.2 | Objection 4 (cosmetic) | editorial |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline, to be integrated in subsequent versioned updates): `docs/q2_q9.md` §7.1 monoid scope correction ("single-digit prime anchor basis", not generator of the full positive cone) and §7.2 Collatz `MODAL_HYPOTHESIS` tagging; `docs/qexpr.md` §4.6 identity-table rule count (16 rules, including the first-root rule); glossary term additions for the new §3.6/§5.7 constructs.

---

## Review Integration Record (v1.4.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md) — verbatim transcript and artifact of record of the Loop-2 adversarial review by Kimi K3.1-Pro (October 27th, 2026) of the QnIR v0.2 candidate draft v0.2.2 — as integrated upstream by the two committed anchors this file mirrors: the constitutional two-axis Layer-0 Hardware Telemetry Envelope of `docs/simemp.md` v1.3.0 and the operational two-axis admission predicate of `docs/qnPrimitive.md` v1.4.0 (§1.15 term provenance, §5.9 margin governance and Admission Ledger). The `docs/qnPrimitive.md` v1.4.0 record flags this file's mirror obligation — the §5.6 factorial rule re-derived as a corollary with a machine-derived threshold `k*` derived from the tier table, and the inverse-root breakeven wording re-issued over the two-axis form — discharged here; §5.5.V (the "§V" of the upstream flags) carries the same re-issue because its chiral-duality bullets quote the withdrawn single-axis inequality. Qm makes no new policy adjudication of its own: the margin range `[1, 64]` and the recommended preset (`κ_dcc = 2`, `κ_t = 4`) are inherited unchanged from `docs/qnPrimitive.md` §5.9, and `k*` is a derived per-profile quantity, not a new canon constant. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place; the withdrawn v1.3.0-era wording is preserved verbatim in this record. New constructions introduced below are tagged **[v1.4.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

**Withdrawn wording (v1.3.0-era, preserved verbatim):** the §5.5.V clauses "Small integer powers disfavor memory lookups (ΔW_compute ≤ ΔW_lookup), executing directly on ALUs" and "require bounded Diophantine lattice searches (ΔW_compute ≫ ΔW_lookup). Roots cross the thermodynamic breakeven threshold immediately, prioritizing sub-graph reuse via Q(reuse)"; the §5.6.II prohibition justification "Initiating a 256-bit cryptographic hash lookup across the Memory Wall consumes dynamic bus power (O(C V² f)) that exceeds execution cost. Q(reuse) is strictly forbidden for n ≤ 2"; and the §5.6.II admission bullet "Admission of Q(reuse) for n ≥ 3: For k ≥ 3, factorial computation satisfies Q(k)! = Q(k) × Q(k-1)!. Because evaluating (k-1)! requires cumulative multiplications, the runtime retrieves the existing hash commitment of (k-1)! via Q(reuse), executing a single multiplication and avoiding redundant re-computation across the Memory Wall."

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Chiral duality re-issue**: forward powers fail the two-axis margins or tie, resolving to Direct Compute (the declared fallback) rather than merely "disfavoring" lookups; inverse roots remain strong work-axis candidates under their declared cost model, but admission is decided solely by the canonical two-axis predicate — a work-axis advantage alone is inadmissible. The automatic-admission claim "roots cross the thermodynamic breakeven threshold immediately" is withdrawn: the bare inequality `ΔW_compute ≫ ΔW_lookup` carries no latency-axis verdict, which is exactly the QO-17 defect | §5.5.V | `QO-17` (via `docs/qnPrimitive.md` v1.4.0); consequentiality constraint (`docs/simemp.md` §4.4) | CANDIDATE |
| **Factorial admission re-derived as a fold-recurrence corollary**: §5.6.II now cites the canonical predicate; the blanket "admitted for n ≥ 3" is withdrawn as an implicit machine-uniformity assumption and replaced by a **machine-derived threshold** `k*` — both declared bounds of the direct path grow monotonically in `k` while the tier bounds, gateway slacks, and advantage margins are fixed declared constants, so the predicate's verdict on the recurrence is a threshold rule, derived deterministically at DCC declaration time (declared cost model + tier table + margins) and never runtime-sampled; a profile whose declared model yields `k* = 3` reproduces the historical behavior as a derived quantity, not a canon-fixed rule; the `n ≤ 2` prohibition is retained as the degenerate corollary (bypass precedence fires first per §5.6.I; the predicate rejects independently by the floor rule) | §5.6.II | Author field report (single-constant insufficiency across machines — the same defect class as the withdrawn blanket rule); F1 declared-bound discipline | CANDIDATE |
| Editorial: the §5.6.I fixed-point identity resolves in `OP_BYPASS` (canonical contraction suite); the former umbrella name `OP_REWRITE` is a deprecated alias (`docs/qnPrimitive.md` §1.16, v1.3.0) | §5.6.I | `docs/qnPrimitive.md` v1.3.0 OP_BYPASS/OP_SATURATE split | EDITORIAL |
| Editorial: the touched admission statements (§5.5.V bullet tails, §5.6.II) are re-encoded to engine-independent plain text per the `docs/abstractionLayers.md` v1.8.1 convention (Unicode operators; code spans for subscripted tokens); the remaining LaTeX spans of this file remain queued for file-wide re-encoding at its next versioned update | §5.5.V, §5.6.II | Rendering Fix Record (`docs/abstractionLayers.md` v1.8.1); `docs/qnPrimitive.md` v1.4.0 EDITORIAL precedent | EDITORIAL |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline): `docs/qm_geometry.md` — §7.2 predicate wording mirror (next queue item); `docs/qexpr.md` — §2.3 breakeven admission bound mirror and the same zero-cost scoping for its "zero Landauer dissipation (W = 0)" normalization sentence; `docs/qstructures.md` §6 work-row precedent and `docs/qphy.md` physical-impedance grounding inherit the two-axis form at their next versioned updates; glossary — `q-reuse` definition refresh and registration of the *Latency Telemetry Envelope* and *Advantage Margin* terms at the queued glossary update. No `term.html` includes are used in this update, under the standing no-dead-entries discipline of `docs/qs_history.md` §8; front matter `terms_used` unchanged.

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

| Schedule | Definition | Domain rationale |
|---|---|---|
| **Dyadic** | $$\kappa_2(z) = Q(2)^z$$ | Zoom shifts lower to exact hardware bit-shifts; default schedule for computation (`DCC_ANCHOR_Q2_PARITY_v1`) |
| **Decimal** | $$\kappa_{10}(z) = (Q(2) \times Q(5))^z$$ | Human financial and decimal contracts under the $$Q(9)$$ radix boundary (`DCC_ANCHOR_Q9_RADIX_v1`) |
| **Factorial** | $$\kappa_{\mathrm{fact}}(z) = (z+1)!$$ | Finite-zoom absorption of arbitrary small rational denominators (denominators up to $$z+1$$ terminate exactly) |

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

  **[v1.3.0 — CANDIDATE, boundary restriction per Review Integration Segment 2 Point 4]** The 1D→2D bivector promotion is sound **for the square root only** ($$n = Q(2)$$) in Euclidean signature $$\mathcal{C}\ell_{2,0}$$, where $$(\vec{e}_1 \wedge \vec{e}_2)^2 = -Q(1)$$. It does **not** extend to higher even roots $$n \ge Q(4)$$: any pure bivector blade $$\beta \,(\vec{e}_1 \wedge \vec{e}_2)$$ satisfies $$\big(\beta \,(\vec{e}_1 \wedge \vec{e}_2)\big)^4 = +\,\beta^4\, Q(1) \ge Q(0)$$ — a non-negative scalar, never a negative one. Fourth and higher even roots of negative scalars are **mixed-grade rotors (spinors)** in the even subalgebra (e.g. $$\sqrt[4]{\ominus Q(1)} = \big(Q(1) \pm \vec{e}_1 \wedge \vec{e}_2\big) \times \mathrm{r}_{\sqrt{2}}$$, coupled to `DCC_ANCHOR_SQRT2_v1`), which exist only inside frames whose declared signature $$\eta$$ provides adjoining axes with $$\eta_{i,i}\,\eta_{j,j} = \oplus$$ (Lorentzian $$\mathcal{C}\ell_{1,1}$$ spacetime bivectors square to $$+Q(1)$$ and cannot serve). Such promotion crosses into Qm Degree 2 ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})). At Degree 1, an even root $$n \ge Q(4)$$ of a negative scalar is intercepted deterministically: if the calling DCC profile declares a supporting frame, the gateway routes the metered promotion to Layer 3; otherwise it emits `RECEIPT_DIMENSIONAL_BUDGET_EXHAUSTED` with reason code `FRAME_SIGNATURE_INSUFFICIENT`. No heuristic fallback exists.

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
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $$\mathrm{Qm}$$ arithmetic methods against the Author or community.
3. **Passive Infrastructure Safe Harbor:** $$\mathrm{Qm}$$ constitutes neutral computational infrastructure. It exercises no editorial inspection over, and makes no proprietary claim upon, sovereign user content or {% include term.html id="payload" text="Payloads" %}.
