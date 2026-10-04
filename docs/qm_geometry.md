---
layout: default
title: Qm Geometry — Multi-Axial Frames and Geometric Algebra
description: Constructive multi-dimensional spatial representation, Clifford geometric algebras, float-free rotations, non-Euclidean manifolds, and deferred symbolic collapse under SIMEMP constraints.
version: 1.1.1
updated: 2026-10-04
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qm_geometry.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qm_geometry.html
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
  - qm
  - compute-unit-u
  - q-reuse
  - q-bypass
  - qexpr
---

# Qm Geometry — Multi-Axial Frames and Geometric Algebra

| Field | Specification |
|---|---|
| **Document Title** | Qm Geometry — Multi-Axial Frames and Geometric Algebra |
| **Version** | 1.1.1 |
| **Last Updated** | 2026-10-04 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category B/D (Complexity Degree 2) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing this specification. If a technical conflict arises, `simemp.md` governs; if a structural conflict arises, `abstractionLayers.md` governs; if a legal conflict arises, `LICENSE.md` governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [Complexity Degree Stratification]({{ '/glossary.html' | relative_url }}#complexity-degree) | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/q0_q1.md` | Local genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) along axis d1 | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and projective remainder calculus | [qm.html]({{ '/qm.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Dimensional Generalization

The foundational operational conjecture of the Qn framework (`docs/simemp.md` §1) asserts that a discrete number system can abstract any computable physical phenomenon under SIMEMP constraints. In `docs/qm.md`, scalar arithmetic was established along the 1D genesis axis $d_1$. However, physical reality and distributed network topographies are inherently multi-dimensional.

Classical geometry relies on continuous coordinate spaces ($\mathbb{R}^n$) and irrational trigonometric functions ($\sin, \cos$), introducing platform-dependent IEEE 754 rounding approximations that violate **Axiom 10**.

$\mathrm{Qm}$ Geometry generalizes the 1D leaf state to $n$-dimensional space via **Clifford Geometric Algebra ($\mathcal{C}\ell_{p,q}$)**. Space, direction, area, volume, and rotation are constructed as **exact, discrete, rational multivector blades** without continuous limits or transcendental float approximations.

Every geometric operation in $\mathrm{Qm}$ satisfies {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

---

## 2. Orthogonal Coordinate Frames and Metric Signatures

In accordance with the {% include term.html id="no-implicit-rule" %}, multi-dimensional space cannot be assumed *a priori*. It is generated inductively from genesis origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) and unit [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1).

```
                    INDUCTIVE AXIAL EMERGENCE
  Q(0) ──► Q(1) along d1 ──► Orthogonal Transition ──► d2 ──► ... ──► dn
 (Origin)  (Genesis Unit)     Gram-Schmidt Functor    (Planar)       (n-Space)
```

### 2.1. The Orthogonal Basis Set

A spatial frame of dimension $n$ is defined by a set of $n$ mutually orthogonal dimensional axes:

$$\mathcal{D}_{\mathrm{frame}} = \{ d_1, \, d_2, \, \dots, \, d_n \}$$

Each coordinate axis $d_k$ is assigned an explicit basis vector instantiated as a canonical $\mathrm{Qm}$ leaf artifact:

$$\vec{e}_k \equiv \langle \oplus, \, 1, \, z=0, \, r=Q(0), \, d_k \rangle$$

### 2.2. Metric Signature ($\eta$)

The quadratic form of the space is governed by an explicit discrete diagonal metric signature $\eta = (p, q, s)$ where $p + q + s = n$:

$$\vec{e}_i \cdot \vec{e}_j = \begin{cases} 
+Q(1) & \text{if } i = j \le p \quad (\text{Space-like}) \\
-Q(1) & \text{if } p \lt i = j \le p + q \quad (\text{Time-like}) \\
Q(0) & \text{if } p + q \lt i = j \le n \quad (\text{Degenerate / Null}) \\
Q(0) & \text{if } i \neq j \quad (\text{Orthogonality})
\end{cases}$$

- **Euclidean $n$-Space:** $\mathcal{C}\ell_{n, 0}$ where $\eta = (+, +, \dots, +)$.
- **Space-Time Algebra (Minkowski):** $\mathcal{C}\ell_{1, 3}$ or $\mathcal{C}\ell_{3, 1}$, providing direct spatial foundations for $\mathrm{Qphy}$.

### 2.3. Non-Euclidean Topologies and Manifold Embeddings

$\mathrm{Qm}$ Geometry abstracts non-Euclidean spaces without continuous Riemannian metrics or differential singularities:

```
                  NON-EUCLIDEAN CONSTRUCTIVE EMBEDDINGS
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ SPHERICAL GEOMETRY (S^n)             │ HYPERBOLIC GEOMETRY (H^n)            │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ Metric Signature:                    │ Metric Signature:                    │
 │   Cl(n+1, 0) Euclidean Basis        │   Cl(n, 1) Minkowski Basis           │
 │ Quadratic Constraint:                │ Quadratic Constraint (Weierstrass):  │
 │   X · X = +Q(1)                      │   X · X = -Q(1),   X_0 > 0           │
 │ Geodesic Motion:                     │ Geodesic Motion:                     │
 │   Planar Bivector Rotors             │   Hyperbolic Boost Rotors            │
 └──────────────────────────────────────┴──────────────────────────────────────┘
```

1. **Spherical Geometry ($\mathbb{S}^n$):** Modeled as the locus of vectors satisfying the Diophantine quadratic constraint $X \cdot X = +Q(1)$ in Euclidean space $\mathcal{C}\ell_{n+1, 0}$. Great-circle geodesics and spherical triangles are evaluated purely via rational bivector rotors without trigonometric projections.
2. **Hyperbolic Geometry ($\mathbb{H}^n$):** Modeled via the Weierstrass hyperboloid within Minkowski spacetime $\mathcal{C}\ell_{n, 1}$ under constraint $X \cdot X = -Q(1)$. Non-Euclidean parallel transport and spatial expansion are governed by hyperbolic boost rotors $(\vec{e}_i \wedge \vec{e}_0)^2 = +Q(1)$.
3. **Conformal Geometric Algebra (CGA):** By introducing two discrete null bases ($e_\infty$ for spatial infinity and $e_0$ for local origin), conformal and projective transformations reduce to linear rotor reflections ($X^2 = 0$).
4. **Discrete Curved Manifolds:** General curved spaces are represented as simplicial complexes (discrete Regge calculus). Curvature is measured not by continuous Ricci tensors, but by **discrete deficit angles** around codimension-2 hinges, evaluated as exact closed bivector loops in $\mathrm{Qm}$.

---

## 3. The Clifford Geometric Product Without Approximations

In $\mathrm{Qm}$ Geometry, the multiplication of two vectors $\vec{u}$ and $\vec{v}$ is governed by the **Clifford Geometric Product**:

$$\vec{u} \vec{v} = \vec{u} \cdot \vec{v} + \vec{u} \wedge \vec{v}$$

This product decomposes the interaction into two distinct grades: a symmetric scalar inner product and an anti-symmetric bivector outer product.

```
                    THE CLIFFORD GEOMETRIC PRODUCT
 ┌─────────────────────────────────────────────────────────────────┐
 │               u v  =  (u · v)   +   (u ∧ v)                     │
 ├────────────────────────────────┬────────────────────────────────┤
 │ Grade 0: Inner Product         │ Grade 2: Outer / Wedge Product │
 │ Symmetric Metric Contraction   │ Anti-Symmetric Oriented Area   │
 │   u · v = 1/2 (uv + vu)        │   u ∧ v = 1/2 (uv - vu)        │
 │   Value ∈ Qm Scalar Leaf       │   Value ∈ Qm Bivector Leaf     │
 └────────────────────────────────┴────────────────────────────────┘
```

### 3.1. The Discrete Inner Product

Let vectors $\vec{u}$ and $\vec{v}$ be expressed along the orthogonal basis:

$$\vec{u} = \sum_{k=1}^n u_k \vec{e}_k, \quad \vec{v} = \sum_{k=1}^n v_k \vec{e}_k$$

where each component is a discrete tuple evaluated at zoom $z$. The inner product contracts to an exact scalar:

$$\vec{u} \cdot \vec{v} = \sum_{k=1}^n \eta_{kk} (u_k \times v_k)$$

The operation executes entirely via the discrete $\mathrm{Qm}$ $\mathcal{R}$-algebra (`docs/qm.md` §4), returning an exact scalar leaf with a conserved remainder $r_z$.

### 3.2. The Discrete Wedge Product

The outer product spans an oriented planar surface area:

$$\vec{u} \wedge \vec{v} = \sum_{1 \le i \lt j \le n} (u_i v_j - u_j v_i) (\vec{e}_i \wedge \vec{e}_j)$$

Under the **No-Implicit Rule**, anti-commutativity and nilpotency are exact:

$$\vec{e}_i \wedge \vec{e}_j = - (\vec{e}_j \wedge \vec{e}_i)$$

$$\vec{e}_i \wedge \vec{e}_i = Q(0)$$

Collinear vectors span zero area without precision loss.

---

## 4. Graded Multivectors and Blade Decomposition

A general geometric artifact $M \in \mathcal{C}\ell_{p, q}$ is a graded multivector spanning $2^n$ basis elements:

$$M = \sum_{k=0}^n \langle M \rangle_k = \langle M \rangle_0 + \langle M \rangle_1 + \langle M \rangle_2 + \dots + \langle M \rangle_n$$

```
                      GRADE STRUCTURE IN Qm GEOMETRY
 ┌───────┬──────────────┬────────────────────────┬─────────────────────────┐
 │ Grade │ Entity       │ Geometric Object       │ Basis Canonical Form    │
 ├───────┼──────────────┼────────────────────────┼─────────────────────────┤
 │   0   │ Scalar       │ Magnitude / Distance   │ 1 (Unit scalar Q(1))    │
 │   1   │ Vector       │ Directed Segment       │ e_i                     │
 │   2   │ Bivector     │ Oriented Planar Patch  │ e_i ∧ e_j               │
 │   3   │ Trivector    │ Oriented Volume Cell   │ e_i ∧ e_j ∧ e_k         │
 │   n   │ Pseudoscalar │ Maximum Volume Element │ I = e_1 ∧ e_2 ∧ ... ∧ e_n│
 └───────┴──────────────┴────────────────────────┴─────────────────────────┘
```

### 4.1. Canonical Multivector State Tuple

Every multivector artifact in $\mathrm{Qm}$ carries an explicit, finite representation:

$$M \equiv \Big\langle \sigma_M, \, \{ \alpha_B \}_{B \in \mathcal{B}}, \, z, \, r_M, \, \mathcal{D}_{\mathrm{frame}} \Big\rangle$$

where:
- $\mathcal{B} = \bigcup_{k=0}^n \binom{\mathcal{D}_{\mathrm{frame}}}{k}$ is the complete canonical basis blade set ($|\mathcal{B}| = 2^n$).
- Each blade coefficient $\alpha_B$ is a discrete scalar magnitude $q_B \in \mathbb{N}$ at zoom $z$.
- $r_M$ is the vector of conserved residual remainders, satisfying $0 \le |r_B| \lt \delta_z(d)$ for each blade component.

### 4.2. Pseudoscalar Duality

The unit pseudoscalar $I = \vec{e}_1 \wedge \vec{e}_2 \wedge \dots \wedge \vec{e}_n$ provides discrete Hodge dual mapping without matrix inversion:

$$M^* = M I^{-1}$$

mapping grade-$k$ blades to grade-$(n-k)$ orthogonal complements.

---

## 5. Float-Free Rotations: Rational Rotors and Double Reflections

In conventional graphics and physics simulations, rotations are parameterized via Euler angles or trigonometric unit quaternions:

$$R = \cos(\theta/2) - I \sin(\theta/2)$$

Because $\cos(\theta)$ and $\sin(\theta)$ are transcendental irrationals for almost all rational angles, continuous engines round them to 32-bit or 64-bit IEEE 754 floats. This introduces metric drift: after $10^6$ rotations, $|R|^2 \neq 1$, forcing artificial normalization cycles.

$\mathrm{Qm}$ Geometry solves this via **Cartan-Dieudonné Double Reflections** and **Rational Cayley-Klein Rotors**.

```
                     CARTAN-DIEUDONNÉ ROTATION MECHANISM
             Vector v ──► Reflect across a ──► Reflect across b ──► v'
                          v_mid = - a v a⁻¹     v' = - b v_mid b⁻¹
                          Result: v' = R v R†  where R = b a
```

### 5.1. The Rotor Morphism

A rotation of vector $\vec{v}$ in a plane is executed by two successive reflections across non-parallel unit vectors $\vec{a}$ and $\vec{b}$:

$$R = \vec{b} \vec{a} = \vec{b} \cdot \vec{a} + \vec{b} \wedge \vec{a}$$

$$\vec{v}' = R \vec{v} R^{\dagger}$$

where $R^{\dagger} = \vec{a} \vec{b}$ is the reverse rotor.

### 5.2. Rational Cayley Parameterization (Float-Free Rotors)

To rotate by an exact rational angle without evaluating trigonometric series, $\mathrm{Qm}$ parameterizes rotors via rational bivectors $B \in \langle M \rangle_2$:

$$R = \frac{Q(1) - B}{Q(1) + B}$$

For any planar rotation in plane $\vec{e}_1 \wedge \vec{e}_2$, the rotor coefficients are generated by Diophantine Pythagorean triplets $(m, n) \in \mathbb{N}^2$:

$$R = \frac{(m^2 - n^2) + 2mn (\vec{e}_1 \wedge \vec{e}_2)}{m^2 + n^2}$$

The norm of the rotor is identically unity by construction:

$$R R^{\dagger} = \frac{(m^2 - n^2)^2 + (2mn)^2}{(m^2 + n^2)^2} = \frac{(m^2 + n^2)^2}{(m^2 + n^2)^2} \equiv Q(1)$$

**Metric drift is identically zero.** The rotor can be applied $10^9$ consecutive times without ever departing from the unit manifold.

### 5.3. Projective Cascade for Arbitrary Target Angles

When an arbitrary target angle $\theta$ is externally supplied, $\mathrm{Qm}$ does not invoke floating-point libraries. It evaluates the continued fraction expansion of $\tan(\theta/4)$, generating a sequence of rational rotors $R_z$ at zoom level $z$:

$$\theta_z = \theta_{\mathrm{discrete}} + r_{\theta}$$

where $r_{\theta}$ is preserved in the remainder envelope. If $r_{\theta} = Q(0)$, the rotation is algebraically exact, emitting `RECEIPT_TERMINAL_EXACTNESS`.

---

## 6. Discrete Metric Norms and Diophantine Distance Solvers

Distance between two spatial points $A$ and $B$ along frame $\mathcal{D}_{\mathrm{frame}}$ is evaluated as the norm of the displacement vector $\vec{v} = B - A$:

$$D^2 = \vec{v} \cdot \vec{v} = \sum_{k=1}^n \eta_{kk} \Big( (q_{B,k} - q_{A,k}) \cdot \delta_z \Big)^2$$

### 6.1. The Diophantine Square-Root Constraint

In $\mathrm{Qm}$, distance evaluation resolves through an exact Diophantine square constraint:

$$D^2 = (q_D \cdot \delta_z)^2 + r_D$$

subject to integer quotient and bounded remainder criteria:

$$q_D \in \mathbb{N}, \quad 0 \le |r_D| \lt (2 q_D \cdot \delta_z^2 + \delta_z^2), \quad \sigma = \oplus$$

If $r_D = Q(0)$, the distance is an exact Pythagorean integer or rational, terminating with zero residual. If $r_D \neq Q(0)$, $r_D$ is preserved as the exact argument for subsequent scale expansion ($z \to z + 1$).

---

## 7. Deferred Symbolic Normalization and Bounded Collapse Engine

A governing distinction between $\mathrm{Qm}$ Geometry and classical numerical engines is the **rejection of eager evaluation** (`docs/abstractionLayers.md` §6.3).

In classical systems, every intermediate geometric operation eagerly rounds to machine floats, accumulating precision drift and repeatedly dissipating Landauer heat ($W \ge k_B T \ln 2$). $\mathrm{Qm}$ Geometry executes geometric workflows through a **Three-Stage Lifecycle**:

```
                    THE THREE-STAGE GEOMETRIC WORKFLOW
 ┌────────────────────────────────────────────────────────────────────────┐
 │ STAGE 1: SYMBOLIC COMPOSITION (Layer 3 — Qexpr AST)                    │
 │ Multivector products, rotor compositions, and reflections are authored │
 │ as unevaluated expression trees. No numerical rounding is executed.    │
 ├────────────────────────────────────────────────────────────────────────┤
 │ STAGE 2: ALGEBRAIC CANONICALIZATION (Factoring & Term Rewriting)       │
 │ • Contract Clifford basis metrics: e_i · e_i = ±Q(1)                   │
 │ • Annihilate collinear wedge products: e_i ∧ e_i = Q(0)                │
 │ • Composite rotor chains: R_net = R_k ... R_2 · R_1                    │
 │ • Factor like-blades and cancel inverse reflections: R R† = Q(1)       │
 │ Zero Landauer Dissipation: W_algebraic = 0 (Isomorphic & Reversible)   │
 ├────────────────────────────────────────────────────────────────────────┤
 │ STAGE 3: TERMINAL BOUNDED COLLAPSE (Layer 1/2 — Leaf Generation)       │
 │ Executed ONLY when an explicit numerical leaf is demanded at Zoom z.   │
 │ Quantizes into discrete tuple ⟨σ, q_z, z, r_z, D⟩ and Remainder Receipt│
 │ Single Dissipative Event: W_total = 1 · W_collapse << k · W_eager      │
 └────────────────────────────────────────────────────────────────────────┘
```

### 7.1. Thermodynamic Landauer Minimization Proof

Let a geometric sequence consist of $k$ consecutive rotor transformations:
1. **Eager Evaluation:** Dissipates $W_{\mathrm{eager}} \ge k \cdot (k_B T \ln 2)$ and compounds $k$ unmeasured remainder vectors.
2. **Deferred Collapse:** Reversible symbolic reduction preserves information without entropy production ($W_{\mathrm{Stage\,2}} = 0$). Numerical collapse occurs once at the boundary:

$$W_{\mathrm{deferred}} = 1 \cdot W_{\mathrm{collapse}} \ll k \cdot W_{\mathrm{collapse}}$$

### 7.2. Coupling to Optimization Morphisms

- **{% include term.html id="q-bypass" %}:** If Stage 2 proves that an expression reduces to an identity transformation ($R R^{\dagger} = Q(1)$) or null area ($\vec{e}_i \wedge \vec{e}_i = Q(0)$), the sub-tree is bypassed entirely at zero metric cost.
- **{% include term.html id="q-reuse" %}:** If a factored bivector patch or multivector rotor is shared across multiple geometric branches, `Q(reuse)` references its existing content commitment hash, preventing redundant allocation across the Memory Wall.

---

## 8. SIMEMP Governance and Metric Exhaustion

Operating in multi-dimensional space consumes physical computational resources governed by the **Metric Invariant** (`docs/simemp.md` §3.1).

### 8.1. Complexity Scaling
- Vector addition: $\mathcal{O}(n)$ discrete operations.
- Geometric product: $\mathcal{O}(2^n)$ blade multiplications.
- Rotor sandwich transformation ($R v R^{\dagger}$): $\mathcal{O}(n \cdot 2^n)$ operations.

### 8.2. Dimensional Ceilings and Budget Halts

Dimensional growth is bounded by the declared Metric Envelope:

$$n \le \mathrm{Dim}_{\mathrm{max}}$$

Typically $n \le 4$ for physical simulations in $\mathrm{Qphy}$; $n \le 16$ for high-dimensional semantic spaces in $\mathrm{Qs}$.

- If a composition of wedge products attempts to construct a grade exceeding $n$, the result collapses identically to zero ($Q(0)$) by the nilpotency of outer products, terminating in $\mathcal{O}(1)$ work.
- If an algebraic transformation exceeds the declared metric budget, execution halts deterministically under **Axiom 6**, emitting the receipt:

$$\mathrm{Receipt}(\texttt{DIMENSIONAL\_BUDGET\_EXHAUSTED}) \equiv \langle \text{Status: } \texttt{BUDGET\_EXHAUSTED}, \, \text{Frame: } n, \, \text{ConsumedWork: } W \rangle$$

---

## 9. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %}:

1. **{% include term.html id="paternity-reference" %}:** All multi-dimensional solvers, Clifford algebra engines, rotor transformations, or compiled geometric kernels derived from this document must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $\mathrm{Qm}$ geometric algorithms or float-free rotor methods against the Author or community.
3. **Statutory Safe Harbor:** $\mathrm{Qm}$ Geometry operates as passive, neutral mathematical infrastructure. It exercises no editorial inspection over, and makes no claim upon, external graphical models, physical simulations, or sovereign {% include term.html id="payload" text="Payloads" %}.
