---
layout: default
title: Qm — Quang Mathematics (Constructive Numeric and Relational Substrate)
description: Canonical specification of the constructive numeric, relational, and algebraic domain substrate of the Qn universe under SIMEMP constraints.
version: 1.1.0
updated: 2026-10-02
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qm.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qm.html
terms_used:
  - qn
  - q0
  - q1
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
---

# Qm — Quang Mathematics: Constructive Numeric and Relational Substrate

| Field | Specification |
|---|---|
| **Document Title** | Qm — Quang Mathematics: Constructive Numeric and Relational Substrate |
| **Version** | 1.1.0 |
| **Last Updated** | 2026-10-02 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category D (Local-First Base Domain) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing $\mathrm{Qm}$. If a technical conflict arises, `simemp.md` governs; if a structural conflict arises, `abstractionLayers.md` governs; if a legal conflict arises, `LICENSE.md` governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and [Lifecycle States]({{ '/glossary.html' | relative_url }}#lifecycle-state) | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Explicit Demarcation

In strict adherence to the {% include term.html id="no-implicit-rule" %} (`docs/abstractionLayers.md` §3.2), this specification explicitly defines the boundaries of $\mathrm{Qm}$ (Quang Mathematics) to prevent implicit assumptions, hidden conventions, or non-deterministic state drift.

Every mathematical proposition and state transition in the {% include term.html id="qn" %} domain of Qm is governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

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
 │ Positional order decidability        │ Arbitrary heuristic tie-breaking     │
 │ Concrete numeric leaf evaluation     │ Symbolic AST container (Qexpr)       │
 └──────────────────────────────────────┴──────────────────────────────────────┘
```

### 1.1. Explicit Inclusions: What $\mathrm{Qm}$ Is
- **The Constructive Numeric Leaf:** The canonical, discrete representation of fully evaluated quantities along dimensional axes.
- **The $\mathcal{R}$-Algebra:** Explicit arithmetic morphisms ($+, -, \times, \div$) preserving exact Diophantine invariants and emitting signed receipts.
- **Projective Scale Transitions:** Exact conservation of residual information ($r_z$) across zoom scales without numeric drift.
- **Local-First Execution:** 100% intra-node computation mediated by local {% include term.html id="cdqn" %} memory-bus data movement, requiring zero distributed consensus.

### 1.2. Explicit Exclusions: What $\mathrm{Qm}$ Is Not
- **Not an IEEE 754 Floating-Point System:** Under **Axiom 10**, approximations, mantissa round-offs, subnormal representations, and $+0.0/-0.0$ distinctions are prohibited.
- **Not a Non-Constructive Continuum:** $\mathrm{Qm}$ does not admit actual infinities ($\aleph_0, \aleph_1$), uncomputable real numbers (non-constructive Dedekind cuts), or undecidable Cauchy sequences.
- **Not an Authoring Language:** High-level source grammar belongs to {% include term.html id="qnlang" %}.
- **Not an Intermediate Execution Backend:** Hardware abstraction and register scheduling belong to {% include term.html id="qnir" %}.
- **Not an Unbounded Symbolic Container:** Unevaluated expression trees belong to {% include term.html id="qexpr" %}; $\mathrm{Qm}$ governs their deterministic numerical collapse.

---

## 2. Abstraction Layer Mapping and Complexity Stratification

$\mathrm{Qm}$ operates across the {% include term.html id="abstraction-layer" text="abstraction-layer" %} hierarchy as a **vertical domain projection** (`docs/abstractionLayers.md` §11), rather than a static horizontal tier.

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
- **{% include term.html id="layer-1" %}:** Hosts genesis primitives. Origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) establishes the spatial origin; unit [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1) establishes the baseline metric quantum; the transition $Q(0) \to Q(1)$ establishes the first dimensional axis $d_1$.
- **Layer 2:** Hosts elementary arithmetic morphisms ($+, -, \times, \div$) constrained by explicit {% include term.html id="dcc-profile" text="DCC Profiles" %}.
- **Layer 3:** Ingests symbolic {% include term.html id="qexpr" %} trees and collapses them into canonical $\mathrm{Qm}$ leaf tuples within finite metric budgets.

### 2.2. Complexity Degree Stratification
- **Degree 0 (Foundational Primitives):** $Q(0)$, $Q(1)$.
- **Degree 1 (Elementary Arithmetic):** Positional tuples along axis $d_1$, Diophantine quotient-remainder partitions. *(Normative scope of this v1.1.0 specification).*
- **Degree 2 (Multi-Axial & Symbolic):** Orthogonal coordinate spaces ($d_k$), Clifford geometric algebras ($\mathcal{C}\ell_{p,q}$), and $\text{Qexpr}$ AST reduction. *(Deferred to `qm_geometry.md` and `qexpr.md`).*
- **Degree 3 (Discrete Calculus & Analysis):** Discrete exterior calculus, finite-difference operators, and projective continued fractions. *(Deferred to `qm_calculus.md`).*

---

## 3. Canonical Numeric Representation

Every fully evaluated numeric artifact in $\mathrm{Qm}$ is an immutable, discrete leaf tuple:

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

### 3.1. Orientation as Geometric Involution ($\sigma$)
Sign is not an independent arithmetic substance. It is an **oriented displacement** relative to local origin $Q(0)$ along axis $d$:
- The base transition from $Q(0)$ to $Q(1)$ along axis $d_1$ defines canonical positive alignment ($\oplus$).
- Inversion is governed by the 1D reflection functor $\mathcal{I}_d$, satisfying the involution property:

$$\mathcal{I}_d: \Sigma \to \Sigma, \quad \mathcal{I}_d^2 = \mathrm{id}, \quad \text{where } \Sigma = \{ \ominus, \, \odot, \, \oplus \}$$

- **$\odot$ (Origin):** Coincident with local $Q(0)$ (zero metric displacement).
- **$\oplus$ (Collinear):** Parallel to the basis transition $Q(0) \to Q(1)$.
- **$\ominus$ (Anti-parallel):** Opposing the basis transition.

Under **Axiom 10**, signed zeros ($+0.0$, $-0.0$) are prohibited. If $q_z = 0$ and $r_z = Q(0)$, $\sigma$ collapses identically to $\odot$.

### 3.2. Ontological Primacy of Magnitude ($q_z$)
The discrete quotient $q_z \in \mathbb{N}$ represents the unsigned metric distance along axis $d$ at zoom level $z$. In physical execution substrates, metric resources (storage footprint, clock cycles, Landauer dissipation) are strictly non-negative. Absolute magnitude $q_z$ is ontologically primary; signed quantities are composite pairs $\langle \sigma, q_z \rangle$.

### 3.3. Base-Independent Scale Lattice ({% include term.html id="zoom-z" text="Zoom z" %})
Conforming to **Axiom 9**, scale is decoupled from arbitrary positional radixes (binary, decimal, sexagesimal). A declared zoom level $z \in \mathbb{N}$ parameterizes a discrete rational subdivision quantum $\delta_z(d) \in \mathbb{Q}^+$ of the dimensional unit:

$$\delta_z(d) = \frac{Q(1)_d}{\kappa(z)}$$

where $\kappa: \mathbb{N} \to \mathbb{N}$ is a strictly monotonic resolution lattice function.

### 3.4. Conserved Residual ({% include term.html id="remainder-r" text="Remainder r" %})
The residual $r_z$ is the exact difference between the unscaled value and its discrete quotient:

$$0 \le |r_z| \lt \delta_z(d)$$

The remainder is never discarded silently. It acts as the exact, causal input state for subsequent resolution expansions.

### 3.5. Dimensional Orthogonality ({% include term.html id="dimension-d" text="Dimension d" %})
The axis parameter $d_k$ ensures that metric units remain orthogonal. Remainders along dimension $d_i$ cannot cancel, add to, or combine with remainders along dimension $d_j$ ($i \neq j$).

---

## 4. The Projective Scale Invariant and Terminal Exactness

The triplet $\langle z, r_z, d \rangle$ constitutes a projective resolution system over discrete lattices.

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
When higher precision is demanded by a downstream workflow ($z \to z + 1$), the input state to the resolution transition is identically $r_z$:

$$r_z = \Big( q_{z+1} \cdot \delta_{z+1}(d) \Big) + r_{z+1}$$

Across arbitrary zoom cascades, total metric information is strictly conserved:

$$\mathcal{I}_{\mathrm{total}}(X_d) = \sum_{k=0}^{z} \Big( q_k \cdot \delta_k(d) \Big) + r_z$$

Zero precision drift occurs. Truncation error is zero at every intermediate scale.

### 4.2. {% include term.html id="terminal-exactness" text="Terminal Exactness" %} Invariant
If a state evaluation yields a remainder equal to the local causal origin:

$$r_z = Q(0)$$

the representation is algebraically exact. 

1. **Information Invariance:** For all subsequent scales $k > 0$, $q_{z+k} \equiv 0$ and $r_{z+k} \equiv Q(0)$.
2. **Deterministic Early Halting:** Evaluating $z+1$ when $r_z = Q(0)$ yields zero entropy reduction ($\Delta I = 0$). Under the **Metric Invariant** (`docs/simemp.md` §3.1), spending compute budget on idempotent iterations is forbidden.
3. **Receipt Emission:** The gateway intercepts this state and emits an immutable terminal receipt:
   - **Terminal Receipt:** `RECEIPT_TERMINAL_EXACTNESS`  
     `⟨ Status: SUCCESS_EXACT, Zoom: z, Remainder: Q(0), ConsumedWork: 0 ⟩`

---

## 5. Elementary Morphisms ($\mathcal{R}$-Algebra)

Arithmetic in $\mathrm{Qm}$ operates over discrete tuples rather than continuous fields.

### 5.1. Addition and Subtraction
Addition is exact vector concatenation along axis $d$. Subtraction is addition composed with geometric reflection:

$$A - B \equiv A + \mathcal{I}_d(B)$$

Given $A = \langle \sigma_A, q_A, z, r_A, d \rangle$ and $B = \langle \sigma_B, q_B, z, r_B, d \rangle$:

Step 1: Absolute values and remainders are combined over the common quantum $\delta_z(d)$:

$$\mu_{\mathrm{exact}} = (\sigma_A q_A \cdot \delta_z + \sigma_A r_A) + (\sigma_B q_B \cdot \delta_z + \sigma_B r_B)$$

Step 2: The result is partitioned into discrete quotient $q_C$, residual remainder $r_C$, and orientation $\sigma_C$:

$$q_C = \left\lfloor \frac{|\mu_{\mathrm{exact}}|}{\delta_z} \right\rfloor$$

$$r_C = |\mu_{\mathrm{exact}}| - (q_C \cdot \delta_z)$$

$$\sigma_C = \operatorname{sgn}(\mu_{\mathrm{exact}})$$

### 5.2. Multiplication
Multiplication scales magnitude and composes orientation parity:

$$\sigma_C = \sigma_A \otimes \sigma_B$$

$$\oplus \otimes \oplus = \oplus, \quad \ominus \otimes \ominus = \oplus, \quad \ominus \otimes \oplus = \ominus, \quad \odot \otimes \sigma = \odot$$

The unscaled magnitude is calculated constructively:

$$\mu_{\mathrm{exact}} = (q_A \cdot \delta_z + r_A) \times (q_B \cdot \delta_z + r_B)$$

The resulting unscaled quantity collapses to $\langle \sigma_C, q_C, z, r_C, d \rangle$ via Diophantine resolution.

### 5.3. Division as a Constrained Diophantine Equation
Division is not an unconstrained primitive. It is a bounded constraint relation:

$$\text{Given } A, B \implies \text{determine } C, R \quad \text{such that } A = (B \times C) + R$$

subject to:
1. $B \neq Q(0)$ (Enforced by DCC profile constraint).
2. $0 \le |R| \lt |B|$.
3. $\sigma_C = \sigma_A \otimes \sigma_B$.

#### Terminal Rejection:
Attempted division where the divisor satisfies $B = Q(0)$ is halted by the gateway, terminating with an immutable receipt:
- **Terminal Rejection Receipt:** `RECEIPT_DIVISION_BY_ZERO_REJECTED`  
  `⟨ Status: REJECTED, Code: ERR_DIV_ZERO ⟩`

### 5.4. Remainder Depth and Dissipative Truncation
When recursive compositions cause the symbolic tree of $r_C$ to exceed the metric envelope ceiling $\mathrm{Depth}_{\max}$:
1. Silent truncation is prohibited under the {% include term.html id="no-implicit-rule" %}.
2. Execution halts or emits an explicit dissipative receipt:
   - **Dissipative Truncation Receipt:** `RECEIPT_REMAINDER_TRUNCATION`  
     `⟨ Status: DISSIPATIVE_SINK, LostResidual: r_C, ExportedEntropy: ΔS ⟩`

---

## 6. Relational Decidability and Positional Ordering

Ordering on dimension $d$ is positional, evaluated over bounded constructive intervals:

$$I(A) = [q_A \cdot \delta_z + r_A, \, q_A \cdot \delta_z + r_A + \epsilon_z]$$

$$\operatorname{Rel}(A, B) = \begin{cases} 
A \lt B & \text{if } \sup I(A) \lt \inf I(B) \\
A \gt B & \text{if } \inf I(A) \gt \sup I(B) \\
A \equiv_z B & \text{if } I(A) \equiv I(B) \text{ and } r_A = r_B \\
\text{Indeterminate} & \text{if } I(A) \cap I(B) \neq \emptyset \text{ and } r_A \neq r_B
\end{cases}$$

When intervals overlap such that inequality cannot be constructively decided at zoom $z$, $\mathrm{Qm}$ forbids heuristic tie-breaking. It emits the deterministic receipt `ORDERING_INDETERMINATE_AT_ZOOM`. To resolve ordering, the caller must allocate additional compute budget to deepen zoom resolution ($z \to z + \Delta z$).

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

### 7.1. The Boundary Between $\text{Qexpr}$ and $\mathrm{Qm}$
- **$\text{Qexpr}$ (Symbolic AST):** Encapsulates unevaluated algebraic expressions, rational functions, and multi-step operator trees. It carries the symbolic structure before execution.
- **$\mathrm{Qm}$ (Evaluation Engine & Leaf):** Defines the concrete algebraic reduction rules that collapse $\text{Qexpr}$ trees into discrete $\langle \sigma, q_z, z, r_z, d \rangle$ tuples at compilation time ({% include term.html id="qnlang" %} $\to$ {% include term.html id="qnir" %}).

### 7.2. Derivability of Real-World Mathematics
All computable real-world mathematics derives systematically from this foundation:
1. **Multi-dimensional Vector Spaces:** Constructed by parameterizing orthogonal sets of axes $\{d_1, d_2, \dots, d_n\}$, giving rise to Clifford geometric algebras ($\mathcal{C}\ell_{p,q}$) without non-deterministic trigonometric float approximations.
2. **Calculus Without Infinities:** Replaces continuous infinitesimals ($\epsilon \to 0$) with discrete differences over lattice quanta $\Delta X / \delta_z(d)$, yielding exact discrete exterior calculus.
3. **Transcendental Constants ($\pi, e$):** Represented as symbolic $\text{Qexpr}$ continued fraction operators. Evaluated at zoom $z$, they emit an exact rational quotient $q_z$ and a conserved residual $r_z$ satisfying exact Diophantine bounds.

---

## 8. Structural Indirection: `Q(anchor)` in $\mathrm{Qm}$

To prevent cascading structural collapse across dependent mathematical workflows, high-centrality mathematical primitives are formalized as {% include term.html id="q-anchor" text="Q(anchor)" %} entities (`docs/_data/glossary.yml`):

$$\chi(\alpha) = \frac{|\mathcal{C}^+(\alpha)|}{|\mathcal{V}|} \ge \Theta_{\chi}$$

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

1. **Anchored Interfaces:** Critical operations (the division constraint, sign reflection, canonical unit $\delta_z$) expose abstract DCC contracts.
2. **Downstream Isolation:** Workflows in higher layers depend upon the abstract contract, not the internal execution engine.
3. **Agility Proofs:** Upgrading an arithmetic solver or patching an algorithmic vulnerability emits a signed migration receipt without invalidating the causal lineage of historical calculations.

---

## 9. License and Invariant Lineage

This domain specification enforces the legal and technical invariants of the {% include term.html id="ssl" %}:

1. **{% include term.html id="paternity-reference" %}:** All derivative implementations, mathematical solvers, or compiled runtime specification files derived from $\mathrm{Qm}$ must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $\mathrm{Qm}$ arithmetic methods against the Author or community.
3. **Passive Infrastructure Safe Harbor:** $\mathrm{Qm}$ constitutes neutral computational infrastructure. It exercises no editorial inspection over, and makes no proprietary claim upon, external user content or Payloads.
