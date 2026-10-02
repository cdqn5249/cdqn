---
layout: default
title: Qm — Quang Mathematics (Constructive Numeric and Relational Substrate)
description: Formal specification of the constructive numeric, relational, and algebraic domain substrate of the Qn universe under SIMEMP constraints.
version: 1.0.0
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
---

# Qm — Quang Mathematics: Constructive Numeric and Relational Substrate

| Field | Specification |
|---|---|
| **Document Title** | Qm — Quang Mathematics: Constructive Numeric and Relational Substrate |
| **Version** | 1.0.0 |
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

## 1. Epistemic Stance and Foundational Scope

$\mathrm{Qm}$ (Quang Mathematics) constitutes the foundational constructive algebraic, numeric, and relational domain within the {% include term.html id="qn" %} stack. Projected directly from Layer 1 (`docs/abstractionLayers.md` §2.3, §11), $\mathrm{Qm}$ executes 100% locally via intra-node {% include term.html id="cdqn" %} data movement, requiring zero distributed consensus.

Classical numerical computing relies on continuous unmetered approximations (IEEE 754 floating-point arithmetic), non-constructive real infinities, and unmeasured truncation errors. $\mathrm{Qm}$ rejects actual infinities, unmetered precision, and silent bit-discarding.

Every state transition in $\mathrm{Qm}$ is governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

All arithmetic morphisms must be total by budget ({% include term.html id="metric-exhaustion" %}), base-independent in representation, and structurally verified via {% include term.html id="computational-consistency" %}.

---

## 2. Canonical Numeric Representation

Every numeric artifact evaluated in $\mathrm{Qm}$ is represented by the discrete canonical tuple:

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

### 2.1. Orientation as Geometric Involution ($\sigma$)

Sign is not an ungrounded bit or independent substance. It is an **oriented displacement** relative to local genesis origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) along dimensional axis $d$:
- The base transition from [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) to unit [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1) along axis $d_1$ defines canonical positive alignment ($\oplus$).
- Directional inversion is governed by the 1D geometric reflection functor $\mathcal{I}_d$, satisfying the involution property:

$$\mathcal{I}_d: \Sigma \to \Sigma, \quad \mathcal{I}_d^2 = \mathrm{id}, \quad \text{where } \Sigma = \{ \ominus, \, \odot, \, \oplus \}$$

- **$\odot$ (Origin):** Coincident with local $Q(0)$ (unoriented zero displacement).
- **$\oplus$ (Collinear):** Parallel to the basis transition $Q(0) \to Q(1)$.
- **$\ominus$ (Anti-parallel):** Opposing the basis transition.

Under **Axiom 10**, signed zeros ($+0.0$, $-0.0$) are prohibited. If $q_z = 0$ and $r_z = Q(0)$, $\sigma$ collapses identically to $\odot$.

### 2.2. Ontological Primacy of Magnitude ($q_z$)

The discrete quotient $q_z \in \mathbb{N}$ represents the unsigned metric distance along axis $d$ at zoom level $z$. In physical computational substrates, metric properties (memory, clock cycles, Landauer work) are strictly non-negative. Absolute magnitude $q_z$ is ontologically primary; the signed quantity is the composite pair $\langle \sigma, q_z \rangle$.

### 2.3. Base-Independent Scale Lattice ({% include term.html id="zoom-z" text="Zoom z" %})

Conforming to **Axiom 9**, scale is decoupled from specific numerical radixes (e.g., base 2 or base 10). A declared zoom level $z \in \mathbb{N}$ parameterizes a discrete rational subdivision quantum $\delta_z(d) \in \mathbb{Q}^+$ of the dimensional unit:

$$\delta_z(d) = \frac{Q(1)_d}{\kappa(z)}$$

where $\kappa: \mathbb{N} \to \mathbb{N}$ is a strictly monotonic resolution lattice function.

### 2.4. Conserved Residual ({% include term.html id="remainder-r" text="Remainder r" %})

The residual $r_z$ is the exact difference between the unscaled value and its discrete quotient:

$$0 \le |r_z| \lt \delta_z(d)$$

The remainder is never discarded silently. It acts as the exact, causal input state for subsequent resolution expansions.

### 2.5. Dimensional Orthogonality ({% include term.html id="dimension-d" text="Dimension d" %})

The axis parameter $d_k$ ensures that metric units remain orthogonal. Remainders along dimension $d_i$ cannot cancel, add to, or combine with remainders along dimension $d_j$ ($i \neq j$).

---

## 3. The Projective Scale Invariant & Terminal Exactness

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

### 3.1. Scale Expansion Equation

When higher precision is demanded by a downstream workflow ($z \to z + 1$), the input state to the resolution transition is identically $r_z$:

$$r_z = \Big( q_{z+1} \cdot \delta_{z+1}(d) \Big) + r_{z+1}$$

Across arbitrary zoom cascades, total metric information is strictly conserved:

$$\mathcal{I}_{\mathrm{total}}(X_d) = \sum_{k=0}^{z} \Big( q_k \cdot \delta_k(d) \Big) + r_z$$

Zero precision drift occurs. Truncation error is zero at every intermediate scale.

### 3.2. {% include term.html id="terminal-exactness" text="Terminal Exactness" %} Invariant

If a state evaluation yields a remainder equal to the local causal origin:

$$r_z = Q(0)$$

the representation is algebraically exact. 

1. **Information Invariance:** For all subsequent scales $k > 0$, $q_{z+k} \equiv 0$ and $r_{z+k} \equiv Q(0)$.
2. **Deterministic Early Halting:** Evaluating $z+1$ when $r_z = Q(0)$ yields zero entropy reduction ($\Delta I = 0$). Under the **Metric Invariant** (`docs/simemp.md` §3.1), spending compute budget on idempotent iterations is forbidden.
3. **Receipt Emission:** The gateway intercepts this state and emits an immutable terminal receipt:
   - **Terminal Receipt:** `RECEIPT_TERMINAL_EXACTNESS`  
     `⟨ Status: SUCCESS_EXACT, Zoom: z, Remainder: Q(0), ConsumedWork: 0 ⟩`

---

## 4. Elementary Morphisms ($\mathcal{R}$-Algebra)

Arithmetic in $\mathrm{Qm}$ operates over discrete tuples rather than continuous fields.

### 4.1. Addition and Subtraction

Addition is exact vector concatenation along axis $d$. Subtraction is addition composed with geometric reflection:

$$A - B \equiv A + \mathcal{I}_d(B)$$

Given $A = \langle \sigma_A, q_A, z, r_A, d \rangle$ and $B = \langle \sigma_B, q_B, z, r_B, d \rangle$:

1. Absolute values and remainders are combined over the common quantum $\delta_z(d)$:

$$\mu_{\mathrm{exact}} = (\sigma_A q_A \cdot \delta_z + \sigma_A r_A) + (\sigma_B q_B \cdot \delta_z + \sigma_B r_B)$$

2. The result is partitioned into quotient $q_C$, remainder $r_C$, and orientation $\sigma_C$:

$$q_C = \left\lfloor \frac{|\mu_{\mathrm{exact}}|}{\delta_z} \right\rfloor, \quad r_C = |\mu_{\mathrm{exact}}| - (q_C \cdot \delta_z)$$

$$\sigma_C = \operatorname{sgn}(\mu_{\mathrm{exact}})$$

### 4.2. Multiplication

Multiplication scales magnitude and composes orientation parity:

$$\sigma_C = \sigma_A \otimes \sigma_B$$

$$\oplus \otimes \oplus = \oplus, \quad \ominus \otimes \ominus = \oplus, \quad \ominus \otimes \oplus = \ominus, \quad \odot \otimes \sigma = \odot$$

The unscaled magnitude is calculated constructively:

$$\mu_{\mathrm{exact}} = (q_A \cdot \delta_z + r_A) \times (q_B \cdot \delta_z + r_B)$$

The resulting unscaled quantity collapses to $\langle \sigma_C, q_C, z, r_C, d \rangle$ via Diophantine resolution.

### 4.3. Division as a Constrained Diophantine Equation

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

### 4.4. Remainder Depth and Dissipative Truncation

When recursive compositions cause the symbolic tree of $r_C$ to exceed the metric envelope ceiling $\mathrm{Depth}_{\max}$:
1. Silent truncation is prohibited under the {% include term.html id="no-implicit-rule" %}.
2. Execution halts or emits an explicit dissipative receipt:
   - **Dissipative Truncation Receipt:** `RECEIPT_REMAINDER_TRUNCATION`  
     `⟨ Status: DISSIPATIVE_SINK, LostResidual: r_C, ExportedEntropy: ΔS ⟩`

---

## 5. Relational Decidability and Positional Ordering

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

## 6. Structural Indirection: `Q(anchor)` in $\mathrm{Qm}$

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

## 7. License and Invariant Lineage

This domain specification enforces the legal and technical invariants of the {% include term.html id="ssl" %}:

1. **{% include term.html id="paternity-reference" %}:** All derivative implementations, mathematical solvers, or compiled runtime specification files derived from $\mathrm{Qm}$ must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $\mathrm{Qm}$ arithmetic methods against the Author or community.
3. **Passive Infrastructure Safe Harbor:** $\mathrm{Qm}$ constitutes neutral computational infrastructure. It exercises no editorial inspection over, and makes no proprietary claim upon, external user content or Payloads.
