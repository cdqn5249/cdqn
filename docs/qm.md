---
layout: default
title: Qm — Quang Mathematics (Constructive Numeric and Relational Substrate)
description: Canonical specification of the constructive numeric, relational, and algebraic domain substrate of the Qn universe under SIMEMP constraints.
version: 1.2.8
updated: 2026-10-17
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qm.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qm.html
terms_used:
  - qm
  - qn
  - q0
  - q1
  - compute-unit-u
  - zoom-z
  - remainder-r
  - dimension-d
  - terminal-exactness
  - q-anchor
  - simemp
  - dependencies-determinism
  - structural-indirection
  - metric-exhaustion
  - no-implicit-rule
  - computational-consistency
  - dcc-profile
  - receipt
  - universal-envelope
  - metric-envelope
  - cdqn
  - causal-arrow
  - complexity-degree
  - local-first
  - ssl
  - paternity-reference
  - qexpr
  - abstraction-layer
  - layer-1
  - payload
  - licensed-work
  - simemp-gateway
  - qnlang
  - qnir
  - q-reuse
  - q-bypass
  - q-even
  - q-odd
---

# Qm — Quang Mathematics: Constructive Numeric and Relational Substrate

| Field | Specification |
|---|---|
| **Document Title** | Qm — Quang Mathematics: Constructive Numeric and Relational Substrate |
| **Version** | 1.2.7 |
| **Last Updated** | 2026-10-17 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category D (Local-First Base Domain) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing $$\mathrm{Qm}$$. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) (v1.1.1) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation (v1.4.1) | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and [Optimization Morphisms]({{ '/glossary.html' | relative_url }}#q-reuse) (v1.2.2) | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) (v1.1.2) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/q2_q9.md` | Single-digit secondary DCC anchors and parity partitions (v1.1.1) | [q2_q9.html]({{ '/q2_q9.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations (v1.2.1) | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qm_calculus.md` | Discrete Exterior Calculus, continued fractions, and invariant dynamics (v1.1.0) | [qm_calculus.html]({{ '/qm_calculus.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG container (v1.1.0) | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `docs/qs.md` | Constructive relational intent and [Intent Contracts]({{ '/glossary.html' | relative_url }}#intent-contract) (v1.0.0) | [qs.html]({{ '/qs.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Explicit Demarcation

In strict adherence to the {% include term.html id="no-implicit-rule" %} ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.2) and the constitutional constraints of the {% include term.html id="simemp" %} framework, this specification explicitly defines the boundaries of {% include term.html id="qm" text="Qm (Quang Mathematics)" %} to prevent implicit assumptions, hidden conventions, or non-deterministic state drift.

In the {% include term.html id="qn" %} computational universe, every mathematical proposition and state transition in $$\mathrm{Qm}$$ is governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

Operational validity is verified empirically via {% include term.html id="computational-consistency" %}. No unmeasured entity may cross an abstraction boundary or {% include term.html id="simemp-gateway" %}.

```
                         EXPLICIT DOMAIN BOUNDARY OF Qm
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ WHAT Qm IS                           │ WHAT Qm IS NOT                       │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ Constructive discrete arithmetic     │ Continuous floating-point arithmetic │
 │ Base-independent scale lattices (z)  │ Platform-dependent radix conventions │
 │ Conserved residual tracking (r)      │ Silent truncation / unmetered loss   │
 │ Geometric orientation involution (σ) │ Detached ontological sign entities   │
 │ Total-by-budget morphisms (Receipts) │ Infinite non-terminating expansions  │
 │ Height-bounded Diophantine relations │ Unbounded Hilbert-10 void search     │
 │ Bounded folds & constrained roots    │ Continuous unmetered exponents       │
 │ Positional order decidability        │ Arbitrary heuristic tie-breaking     │
 │ Concrete numeric leaf evaluation     │ Symbolic AST container (Qexpr)       │
 └──────────────────────────────────────┴──────────────────────────────────────┘
```

### 1.1. Explicit Inclusions: What Qm Is
- **The Constructive Numeric Leaf:** The canonical, discrete representation of fully evaluated quantities along dimensional axes encapsulated within the {% include term.html id="universal-envelope" %}.
- **The $\mathcal{R}$-Algebra:** Explicit arithmetic morphisms ($$+ $$, $$- $$, $$\times$$, $$\div$$, powers, and roots) preserving exact Diophantine invariants and emitting signed receipts.
- **Projective Scale Transitions:** Exact conservation of residual information ($$r_z$$) across zoom scales without numeric drift.
- **{% include term.html id="local-first" %} Execution:** 100% intra-node computation mediated by local cdqn memory-bus data movement, requiring zero distributed consensus.
- **Total-by-Budget Morphisms:** Discrete operations bounded by finite resource consumption via {% include term.html id="metric-exhaustion" %}.

### 1.2. Explicit Exclusions: What Qm Is Not
- **Not an IEEE 754 Floating-Point System:** Under **Axiom 10**, approximations, mantissa round-offs, subnormal representations, and $$+0.0 / -0.0$$ distinctions are prohibited.
- **Not a Non-Constructive Continuum:** $$\mathrm{Qm}$$ does not admit actual infinities ($$\aleph_0, \aleph_1$$), uncomputable real numbers (non-constructive Dedekind cuts), or undecidable Cauchy sequences.
- **Not an Unbounded Diophantine Search Oracle:** While arithmetic operations are defined through Diophantine constraints, $$\mathrm{Qm}$$ explicitly rejects unmetered infinite search spaces ($$\mathbb{Z}^n, \mathbb{Q}^n$$). Classical undecidability (Hilbert's Tenth Problem, DPRM 1970) is neutralized: all evaluations are constrained to finite logarithmic height bounds ($$h(x) \le H_{\mathrm{max}}$$) declared within the {% include term.html id="metric-envelope" %}.
- **Not an Authoring Language:** High-level source grammar belongs to {% include term.html id="qnlang" %}.
- **Not an Intermediate Execution Backend:** Hardware abstraction and register scheduling belong to {% include term.html id="qnir" %}.
- **Not an Unbounded Symbolic Container:** Unevaluated expression trees belong to {% include term.html id="qexpr" %}; $$\mathrm{Qm}$$ governs their deterministic numerical collapse.

---

## 2. Abstraction Layer Mapping and Complexity Stratification

$$\mathrm{Qm}$$ operates across the hierarchy as a vertical domain projection across {% include term.html id="abstraction-layer" text="abstraction layers" %} ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §11), rather than a static horizontal tier.

```
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ Layer 3: Symbolic Composition (Qexpr Trees, Bounded Expressions)           │
 │          Qm Degree 2: Multi-axial geometry (d_k), Polynomial Forms          │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ Layer 2: Governed Morphisms & Algebraic Interfaces                         │
 │          Qm Degree 1: Monoids, Involutions (I), Diophantine Constraints     │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ Layer 1: Node Genesis Layer                                                 │
 │          Qm Degree 0: Origin Q(0), Unit Q(1), Basis Alignment d1           │
 ├═════════════════════════════════════════════════════════════════════════════┤
 │ Layer 0: Physical Substrate (Entropy Harvesting, Landauer Work W ≥ kT ln 2) │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 2.1. Layer-by-Layer Responsibilities
- **{% include term.html id="layer-1" %}:** Hosts genesis primitives. Origin {% include term.html id="q0" text="Q(0)" %} establishes the spatial origin; unit {% include term.html id="q1" text="Q(1)" %} establishes the baseline metric quantum; the transition $$Q(0) \to Q(1)$$ establishes the initial dimensional axis {% include term.html id="dimension-d" text="d1" %} and the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}.
- **Layer 2:** Hosts elementary arithmetic morphisms ($$+ $$, $$- $$, $$\times$$, $$\div$$, exponents, roots) constrained by explicit {% include term.html id="dcc-profile" text="DCC Profiles" %}, indexed sequentially along the monotonic {% include term.html id="causal-arrow" %}.
- **Layer 3:** Ingests symbolic {% include term.html id="qexpr" %} trees and collapses them into canonical $$\mathrm{Qm}$$ leaf tuples within finite metric budgets.

### 2.2. Complexity Degree Stratification
- **Degree 0 (Foundational Primitives):** $$Q(0)$$, $$Q(1)$$.
- **Degree 1 (Elementary Arithmetic):** Positional tuples along axis $$d_1$$, Diophantine quotient-remainder partitions, bounded powers. *(Normative scope of this specification).*
- **Degree 2 (Multi-Axial & Symbolic):** Orthogonal coordinate spaces ($$d_k$$), Clifford geometric algebras ($$\mathcal{C}\ell_{p,q}$$), and Qexpr CAE-DAG reduction ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) and [`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }})).
- **Degree 3 (Discrete Calculus & Analysis):** Discrete exterior calculus, finite-difference operators, and projective continued fractions ([`docs/qm_calculus.md`]({{ '/qm_calculus.html' | relative_url }})).

---

## 3. Canonical Numeric Representation

Every fully evaluated numeric artifact in $$\mathrm{Qm}$$ is an immutable, discrete {% include term.html id="qn" text="Qn Numeric Entity" %} represented as a discrete leaf tuple:

$$\text{Qn Numeric Entity} \equiv \langle \sigma, \, q_z, \, z, \, r_z, \, d \rangle$$

```
                   CANONICAL Qm NUMERIC STRUCTURE
 ┌─────────────────────────────────────────────────────────────────┐
 │  σ    : Relational Orientation (⊖, ⊙, ⊕)                       │
 │  q_z  : Discrete Metric Magnitude Quotient (q_z ∈ ℕ)           │
 │  z    : Scale / Lattice Zoom Level (z ∈ ℕ)                     │
 │  r_z  : Conserved Residual Generator (0 ≤ |r_z| < δ_z(d))       │
 │  d    : Dimensional Coordinate Axis Parameter (d_k)            │
 └─────────────────────────────────────────────────────────────────┘
```

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
The discrete quotient $$q_z \in \mathbb{N}$$ represents the unsigned metric distance along axis $$d$$ at zoom level $$z$$. In physical execution substrates, metric resources (storage footprint, clock cycles, Landauer dissipation) are strictly non-negative. Absolute magnitude $$q_z$$ is ontologically primary; signed quantities are composite pairs $$\langle \sigma, q_z \rangle$$.

### 3.3. Base-Independent Scale Lattice ({% include term.html id="zoom-z" text="Zoom z" %})
Conforming to **Axiom 9**, scale is decoupled from arbitrary positional radices (binary, decimal, sexagesimal). A declared zoom level $$z \in \mathbb{N}$$ parameterizes a discrete rational subdivision quantum $$\delta_z(d) \in \mathbb{Q}^+$$ of the dimensional unit:

$$\delta_z(d) = \frac{Q(1)_d}{\kappa(z)}$$

where $$\kappa: \mathbb{N} \to \mathbb{N}$$ is a strictly monotonic resolution lattice function.

### 3.4. Conserved Residual ({% include term.html id="remainder-r" text="Remainder r" %})
The residual $$r_z$$ is the exact difference between the unscaled value and its discrete quotient:

$$0 \le \lvert r_z \rvert \lt \delta_z(d)$$

The remainder is never discarded silently. It acts as the exact, causal input state for subsequent resolution expansions.

### 3.5. Dimensional Orthogonality ({% include term.html id="dimension-d" text="Dimension d" %})
The axis parameter $$d_k$$ ensures that metric units remain orthogonal. Remainders along dimension $$d_i$$ cannot cancel, add to, or combine with remainders along dimension $$d_j$$ ($$i \neq j$$).

---

## 4. The Projective Scale Invariant and Terminal Exactness

The triplet $$\langle z, r_z, d \rangle$$ constitutes a projective resolution system over discrete lattices.

```
                           THE TELESCOPIC ZOOM CASCADE
  Zoom z:       X_d = ( q_z · δ_z(d) ) + r_z
                                          │
                       ┌──────────────────┘ (r_z acts as total argument)
                       ▼
  Zoom z+1:     r_z = ( q_{z+1} · δ_{z+1}(d) ) + r_{z+1}
                                                   │
                                ┌──────────────────┘
                                ▼
  Zoom z+2:     r_{z+1} = ( q_{z+2} · δ_{z+2}(d) ) + Q(0)  ◄── [TERMINAL EXACTNESS]
                                                       │
                                                       ▼
                                            Computation Terminates (ΔH = 0)
```

### 4.1. Scale Expansion Equation
When higher precision is demanded by a downstream workflow ($$z \to z + 1$$), the input state to the resolution transition is identically $$r_z$$:

$$r_z = \Big( q_{z+1} \cdot \delta_{z+1}(d) \Big) + r_{z+1}$$

Across arbitrary zoom cascades, total metric information is strictly conserved:

$$\mathcal{I}_{\mathrm{total}}(X_d) = \sum_{k=0}^{z} \Big( q_k \cdot \delta_k(d) \Big) + r_z$$

Zero precision drift occurs. Truncation error is zero at every intermediate scale.

### 4.2. {% include term.html id="terminal-exactness" text="Terminal Exactness" %} Invariant
If a state evaluation yields a remainder equal to the local causal origin:

$$r_z = Q(0)$$

the representation is algebraically exact.

1. **Information Invariance:** For all subsequent scales $$k \gt 0$$, $$q_{z+k} \equiv 0$$ and $$r_{z+k} \equiv Q(0)$$.
2. **Deterministic Early Halting:** Evaluating $$z+1$$ when $$r_z = Q(0)$$ yields zero entropy reduction ($$\Delta I = 0$$). Under the **Metric Invariant** ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §3.1), spending compute budget on idempotent iterations is forbidden.
3. **Receipt Emission:** The {% include term.html id="simemp-gateway" %} intercepts this state and emits an immutable terminal {% include term.html id="receipt" %}:
   - **Terminal Receipt:** `RECEIPT_TERMINAL_EXACTNESS`  
     `⟨ Status: SUCCESS_EXACT, Zoom: z, Remainder: Q(0), ConsumedWork: 0 ⟩`

---

## 5. Elementary Morphisms (R-Algebra)

Arithmetic in $$\mathrm{Qm}$$ operates over discrete tuples rather than continuous fields.

### 5.1. Addition and Subtraction
Addition is exact vector concatenation along axis $$d$$. Subtraction is addition composed with geometric reflection:

$$A - B \equiv A + \mathcal{I}_d(B)$$

Given $$A = \langle \sigma_A, q_A, z, r_A, d \rangle$$ and $$B = \langle \sigma_B, q_B, z, r_B, d \rangle$$:

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

The evaluation is constrained by three invariant conditions: the divisor must be non-zero ($$B \neq Q(0)$$), the remainder is strictly bounded ($$0 \le \lvert R \rvert \lt \lvert B \rvert$$), and the orientation sign satisfies parity composition ($$\sigma_C = \sigma_A \otimes \sigma_B$$).

#### Height-Bounded Diophantine Decidability

Classical Diophantine equations over unmetered spaces ($$\mathbb{Z}, \mathbb{Q}$$) inherit algorithmic undecidability under Hilbert's Tenth Problem. In $$\mathrm{Qm}$$, this undecidability void is closed structurally across three physical constraints:

**Logarithmic Height Boundedness:** In arithmetic geometry, the logarithmic height measures physical bit complexity, bounded by $$h(u/v) = \mathrm{max}(\mathrm{log}\lvert u \rvert, \, \mathrm{log}\lvert v \rvert) \le H_{\mathrm{max}}$$. Under the {% include term.html id="metric-envelope" %}, every Diophantine evaluation operates within an explicit height ceiling $$H_{\mathrm{max}}$$ and work budget $$W_{\mathrm{budget}}$$.

**Finite State Space (Northcott's Theorem):** By Northcott's theorem, the set of rational coordinates of bounded height is strictly finite:

$$\mathrm{Card}\big(\{ x \in \mathbb{Q} \colon h(x) \le H_{\mathrm{max}} \}\big) \lt \infty$$

Therefore, candidate solutions in $$\mathrm{Qm}$$ inhabit a finite discrete lattice rather than an unmetered void.

**Deterministic Decidability:** A Diophantine evaluation is decidable by construction: it either terminates with an exact quotient-remainder pair, reaches Terminal Exactness ($$R = Q(0)$$), or halts unconditionally upon budget exhaustion via **Metric Exhaustion** (`Axiom 6`), emitting `RECEIPT_BUDGET_EXHAUSTED`.

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
For integer exponents $$B \in \mathbb{N}$$, exponentiation is an iterative multiplicative fold over the positive monoid $$(\mathcal{Q}^+, \times, Q(1))$$:

$$A^B = \prod_{j=1}^B A$$

Evaluated at Layer 3 under `OP_FOLD`. The memory footprint expands linearly with the exponent: $$h(A^B) = B \cdot h(A) \le H_{\max}$$. If the resulting bit size exceeds the metric envelope ceiling, compilation halts with `RECEIPT_EXPRESSION_TOO_COMPLEX`.

#### II. Negative Exponents as Rational Pairs
Negative powers do not invoke floating-point arithmetic; they map to exact rational inversion pairs under `OP_RATIO`:

$$A^{-B} \equiv \mathrm{OP\_RATIO}\big( Q(1), \, A^B \big) = \frac{Q(1)}{A^B}$$

Projected onto scale lattices at zoom $$z$$ with exact remainder conservation ($$r_z$$). If the base is negative ($$\ominus A$$), the resulting sign is determined by the parity of exponent $$B$$ via secondary anchor $$Q(2)$$ (`DCC_ANCHOR_Q2_PARITY_v1`):

$$\sigma_{\mathrm{result}} = \begin{cases} 
\oplus & \mathrm{if} \; B \in \mathcal{Q}_{\mathrm{even}} \\ 
\ominus & \mathrm{if} \; B \in \mathcal{Q}_{\mathrm{odd}} 
\end{cases}$$

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

#### V. The Chiral Optimization Duality
Powers and roots form a **Chiral Morphism Pair** along the causal execution arrow:
- Forward powers collapse geometric grades ($$I_2^2 \to -Q(1)$$) and compute in feed-forward polynomial steps. Small integer powers disfavor memory lookups ($$\Delta W_{\mathrm{compute}} \le \Delta W_{\mathrm{lookup}}$$), executing directly on ALUs.
- Inverse roots expand geometric grades ($$\sqrt{\ominus Q(1)} \to I_2$$) and require bounded Diophantine lattice searches ($$\Delta W_{\mathrm{compute}} \gg \Delta W_{\mathrm{lookup}}$$). Roots cross the thermodynamic breakeven threshold immediately, prioritizing sub-graph reuse via {% include term.html id="q-reuse" %}.

### 5.6. Discrete Factorial and Combinatorial Morphisms

The factorial operator is formalized in $$\mathrm{Qm}$$ as an iterative discrete monoidal fold over the positive integer monoid $$(\mathcal{Q}^+, \times, Q(1))$$ under Node Class 4 (`OP_FOLD`):

$$n! = \prod_{j=1}^n j$$

#### I. Boundary Identifications and $\mathcal{O}(1)$ Bypasses
The factorial morphism exhibits three canonical identity states in the range $$Q(0) \dots Q(2)$$:
- **Empty Product Identity ($$Q(0)!$$):** In monoid theory, an empty product evaluates to the multiplicative identity $$Q(1)$$. Handled in $$\mathcal{O}(1)$$ via {% include term.html id="q-bypass" %}, emitting `RECEIPT_BYPASS_IDENTITY` at zero metric cost ($$\Delta W = 0$$).
- **Unit Invariant ($$Q(1)!$$):** Evaluates identically to $$Q(1)$$. Bypassed in $$\mathcal{O}(1)$$ via $$Q(\text{bypass})$$ ($$\Delta W = 0$$).
- **Fixed-Point Invariant ($$Q(2)!$$):** $$2$$ is the unique positive integer satisfying $$n! = n$$. Recognized as an algebraic fixed-point identity in `OP_REWRITE`, bypassing execution in $$\mathcal{O}(1)$$ ($$\Delta W = 0$$).

#### II. Thermodynamic Optimization Boundary
Under the **Metric Existential Invariant**, optimization morphisms adhere to strict thermodynamic breakeven bounds:
- **Prohibition of $$Q(\text{reuse})$$ for $$n \le 2$$:** For $$Q(0)$$, $$Q(1)$$, and $$Q(2)$$, computational cost is zero (direct bypass). Initiating a 256-bit cryptographic hash lookup across the Memory Wall consumes dynamic bus power ($$\mathcal{O}(C V^2 f)$$) that exceeds execution cost. $$Q(\text{reuse})$$ is strictly forbidden for $$n \le 2$$.
- **Admission of $$Q(\text{reuse})$$ for $$n \ge 3$$:** For $$k \ge 3$$, factorial computation satisfies $$Q(k)! = Q(k) \times Q(k-1)!$$. Because evaluating $$(k-1)!$$ requires cumulative multiplications, the runtime retrieves the existing hash commitment of $$(k-1)!$$ via $$Q(\text{reuse})$$, executing a single multiplication and avoiding redundant re-computation across the Memory Wall.

#### III. Non-Existence of Negative Factorials
The factorial recurrence relation is given by:

$$(n - 1)! = \frac{n!}{n}$$

Extending this relation to negative integers requires dividing by local genesis origin $$Q(0)$$:

$$(-1)! = \frac{0!}{0} = \frac{Q(1)}{Q(0)}$$

$$\forall k \in \mathcal{Q}^+, \quad (-k)! = \frac{Q(1)}{Q(0) \times \prod_{j=1}^{k-1} (-j)}$$

Because division by origin $$Q(0)$$ is an absolute constitutional prohibition in $$\mathrm{Qm}$$ (§5.3), and actual infinite poles are non-realizable under **Axiom 2**, the factorial of a negative integer cannot exist. Any attempt to compute $$(\ominus k)!$$ is intercepted deterministically at the SIMEMP Gateway, emitting:

$$\texttt{RECEIPT\_DIVISION\_BY\_ZERO\_REJECTED}$$

---

## 6. Relational Decidability and Positional Ordering

Ordering on dimension $$d$$ is positional, evaluated over bounded constructive intervals:

$$I(A) = [q_A \cdot \delta_z + r_A, \, q_A \cdot \delta_z + r_A + \epsilon_z]$$

$$\mathrm{Rel}(A, B) = \begin{cases} 
A \lt B & \mathrm{if} \; \sup I(A) \lt \inf I(B) \\
A \gt B & \mathrm{if} \; \inf I(A) \gt \sup I(B) \\
A \equiv_z B & \mathrm{if} \; I(A) \equiv I(B) \;\mathrm{and}\; r_A = r_B \\
\mathrm{Indeterminate} & \mathrm{if} \; I(A) \cap I(B) \neq \emptyset \;\mathrm{and}\; r_A \neq r_B
\end{cases}$$

When intervals overlap such that inequality cannot be constructively decided at zoom $$z$$, $$\mathrm{Qm}$$ forbids heuristic tie-breaking. It emits the deterministic receipt `ORDERING_INDETERMINATE_AT_ZOOM`. To resolve ordering, the caller must allocate additional compute budget to deepen zoom resolution ($$z \to z + \Delta z$$).

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
- **$$\mathrm{Qm}$$ (Evaluation Engine & Leaf):** Defines the concrete algebraic reduction rules that collapse Qexpr trees into discrete $$\langle \sigma, q_z, z, r_z, d \rangle$$ tuples at compilation time ({% include term.html id="qnlang" %} $\to$ {% include term.html id="qnir" %}).

### 7.2. Derivability of Real-World Mathematics
All computable real-world mathematics derives systematically from this foundation:
1. **Multi-dimensional Vector Spaces:** Constructed by parameterizing orthogonal sets of axes $$\{d_1, d_2, \dots, d_n\}$$, giving rise to Clifford geometric algebras ($$\mathcal{C}\ell_{p,q}$$) without non-deterministic trigonometric float approximations ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})).
2. **Calculus Without Infinities:** Replaces continuous infinitesimals ($$\epsilon \to 0$$) with discrete differences over lattice quanta $$\Delta X / \delta_z(d)$$, yielding exact discrete exterior calculus ([`docs/qm_calculus.md`]({{ '/qm_calculus.html' | relative_url }})).
3. **Transcendental Constants ($$\pi, e$$):** Represented as symbolic Qexpr continued fraction operators ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.3). Evaluated at zoom $$z$$, they emit an exact rational quotient $$q_z$$ and a conserved residual $$r_z$$ satisfying exact Diophantine bounds.

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

1. **Anchored Interfaces:** Critical operations (the division constraint, sign reflection, canonical unit $$\delta_z$$) expose abstract DCC contracts under {% include term.html id="structural-indirection" %}.
2. **Downstream Isolation:** Workflows in higher layers depend upon the abstract contract, not the internal execution engine.
3. **Agility Proofs:** Upgrading an arithmetic solver or patching an algorithmic vulnerability emits a signed migration receipt without invalidating the causal lineage of historical calculations.

---

## 9. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %} governing the {% include term.html id="licensed-work" %}:

1. **{% include term.html id="paternity-reference" %}:** All derivative implementations, mathematical solvers, or compiled runtime specification files derived from $$\mathrm{Qm}$$ must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $$\mathrm{Qm}$$ arithmetic methods against the Author or community.
3. **Passive Infrastructure Safe Harbor:** $$\mathrm{Qm}$$ constitutes neutral computational infrastructure. It exercises no editorial inspection over, and makes no proprietary claim upon, sovereign user content or {% include term.html id="payload" text="Payloads" %}.
