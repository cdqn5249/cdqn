---
layout: default
title: Qm Calculus — Discrete Exterior Calculus, Continued Fractions, and Invariant Dynamics
description: Canonical specification of constructive discrete calculus, Discrete Exterior Calculus (DEC) on simplicial meshes, transcendental continued fractions, and aperiodic lineage trajectories under SIMEMP constraints.
version: 1.0.1
updated: 2026-10-10
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qm_calculus.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qm_calculus.html
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
  - security-envelope
  - cdqn
  - causal-arrow
  - complexity-degree
  - local-first
  - ssl
  - paternity-reference
  - q-reuse
  - q-bypass
  - qexpr
  - layer-0
  - layer-1
  - payload
  - licensed-work
  - existential-invariant
  - operational-agility
  - boc-policy
  - simemp-gateway
---

# Qm Calculus — Discrete Exterior Calculus, Continued Fractions, and Invariant Dynamics

| Field | Specification |
|---|---|
| **Document Title** | Qm Calculus — Discrete Exterior Calculus, Continued Fractions, and Invariant Dynamics |
| **Version** | 1.0.1 |
| **Last Updated** | 2026-10-10 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category D ({% include term.html id="complexity-degree" text="Complexity Degree 3" %}) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing this specification. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [Complexity Degree Stratification]({{ '/glossary.html' | relative_url }}#complexity-degree) | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and optimization primitives | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/q2_q9.md` | Single-digit secondary DCC anchors and single-digit spectrum | [q2_q9.html]({{ '/q2_q9.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG container | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance: Calculus Without Infinities

In classical mathematics, calculus relies on non-constructive, continuous foundations: the Cauchy–Weierstrass $\epsilon$-$\delta$ limit, actual infinities ($\mathbb{R}, \infty$), and vanishing infinitesimals ($dx \to 0$). While analytically elegant, continuous calculus creates physical paradoxes: it allows mathematical equations to predict unphysical singularities (such as infinite fluid velocities in Navier–Stokes or infinite gravitational densities in black holes) that never occur in real physical substrates.

In the {% include term.html id="qn" %} computational universe, **continuous limits are discarded as physically unrealizable approximations**. Under [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}), physical computation is strictly finite and bounded by two real horizons:
1. **The Upper Horizon ($Qn_{\max}$):** The finite memory capacity, register width, and operational energy ceiling declared in the local {% include term.html id="metric-envelope" %}.
2. **The Lower Horizon ($\delta_z(d)$ near {% include term.html id="q0" text="Q(0)" %}):** The physical resolution quantum at {% include term.html id="zoom-z" text="Zoom z" %}, below which state transitions dissipate into physical thermal noise bounded by Landauer's bound:

$$W \ge k_B T \ln 2$$

{% include term.html id="qm" %} Calculus replaces continuous infinitesimals with **Constructive Discrete Exterior Calculus (DEC)** and **Projective Continued Fraction Cascades**, satisfying {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

All operational derivations represent working hypotheses evaluated via {% include term.html id="computational-consistency" %}. No unmeasured limit may cross a {% include term.html id="simemp-gateway" %}.

```
                          THE BOUNDED DYNAMICAL SCOPE
      [ The Upper Horizon: Qn_max / Declared Metric Ceiling ]
                           ▲
                           │  CONSTRUCTIVE DYNAMICAL DOMAIN
                           │  • Simplicial Coboundary Operators (d ∘ d ≡ 0)
                           │  • Projective Continued Fractions ([a_0; a_1...])
                           │  • Exact Discrete Stokes Invariants
                           ▼
      [ The Lower Horizon: Q(0) / Physical Thermal Dissipation Limit ]
```

---

## 2. The Discrete Replacement of Continuous Limits

Classical continuous limits ($\lim_{\Delta x \to 0}$) are replaced by three constructive mechanisms that operate with zero rounding drift under the {% include term.html id="no-implicit-rule" %}:

### 2.1. Scale-Lattice Refinement (z-Refinement)
Space and time are not continuous voids; they are partitioned by the discrete resolution quantum along {% include term.html id="dimension-d" text="dimensional axes" %} ([`docs/qm.md`]({{ '/qm.html' | relative_url }}) §3.3):

$$\delta_z(d) = \frac{Q(1)_d}{\kappa(z)}$$

"Approaching a limit" does not mean shrinking an infinitesimal toward zero. It represents an explicit, metered **Lattice Refinement Step ($z \to z+1$)**:
- Refinement requires burning an explicit metric budget in units of the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}.
- When the declared metric budget expires, refinement terminates cleanly via {% include term.html id="metric-exhaustion" %}.

### 2.2. Conserved Residual Telescoping
Classical series convergence silently truncates unmeasured trailing terms. In $\mathrm{Qm}$ Calculus, convergence is an exact Diophantine cascade where the residual is strictly conserved:

$$r_z = \big( q_{z+1} \cdot \delta_{z+1}(d) \big) + r_{z+1}$$

Across arbitrary expansions, total metric information is preserved:

$$\mathcal{I}_{\mathrm{total}}(X_d) = \sum_{k=0}^{z} \big( q_k \cdot \delta_k(d) \big) + r_z$$

If $r_z = Q(0)$, the system achieves {% include term.html id="terminal-exactness" %}: the calculation reaches exact algebraic closure, triggering immediate early halting with zero residual entropy ($\Delta H = 0$).

---

## 3. Discrete Exterior Calculus (DEC) on Simplicial Complexes

To model fields, waves, fluids, and electrodynamics without floating-point PDEs, $\mathrm{Qm}$ Calculus adopts **Discrete Exterior Calculus (DEC)** on orthogonal multi-axial frames ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})).

```
                      DEC OPERATOR MAPPING ON SIMPLICES
 Simplicial Primal Mesh K:            Dual Circumcentric Mesh ⋆K:
   0-Simplex (Vertex v) ─────────────► Dual n-Cell (Voronoi cell ⋆v)
   1-Simplex (Edge e)   ─────────────► Dual (n-1)-Face (Flux facet ⋆e)
   2-Simplex (Face f)   ─────────────► Dual (n-2)-Edge (Circulation ⋆f)
            │                                     ▲
            ▼ Discrete Exterior Derivative d      │ Discrete Hodge Star ⋆
   (k+1)-Simplex Coboundary ──────────────────────┘
```

### 3.1. Discrete Differential Forms as Cochains
A physical or geometric field is not a continuous function; it is a **discrete cochain** evaluated on oriented $k$-simplices of a simplicial complex $K$:
* **$0$-Forms (Scalars):** Potential fields, temperatures, and scalar charges evaluated on vertices.
* **$1$-Forms (Circulations):** Electric fields ($E$) and fluid velocities ($u$) integrated along oriented edges.
* **$2$-Forms (Fluxes):** Magnetic flux ($B$) and vorticity ($\omega$) evaluated across oriented planar faces.

### 3.2. The Discrete Exterior Derivative (d / DCC_ANCHOR_DEC_D_v1)
The discrete derivative is the algebraic coboundary operator $\mathbf{d}$ mapping $k$-forms to $(k+1)$-forms:

$$\langle \mathbf{d}\omega, \, \sigma_{k+1} \rangle \equiv \langle \omega, \, \partial \sigma_{k+1} \rangle = \sum_{j=0}^{k+1} (-1)^j \langle \omega, \, [v_0, \dots, \hat{v}_j, \dots, v_{k+1}] \rangle$$

#### The Fundamental Topological Invariant:
Under the {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %}, $\mathbf{d}$ satisfies the exact nilpotent identity:

$$\mathbf{d} \circ \mathbf{d} \equiv Q(0)$$

Because the boundary of a boundary is identically zero ($\partial \partial \equiv \emptyset$), applying the exterior derivative twice vanishes identically. This invariant is anchored via `DCC_ANCHOR_DEC_D_v1` as a primary {% include term.html id="q-anchor" text="Q(anchor)" %}.

### 3.3. The Discrete Hodge Star Dual (DCC_ANCHOR_HODGE_v1)
The Hodge star operator $\star$ maps primal $k$-forms to dual $(n-k)$-forms across the circumcentric dual mesh:

$$\star \omega = \sum_{\sigma_k \in K} \frac{\lvert \star \sigma_k \rvert}{\lvert \sigma_k \rvert} \langle \omega, \, \sigma_k \rangle \, (\star \sigma_k)$$

Dual volume ratio quotients:

$$\frac{\lvert \star \sigma_k \rvert}{\lvert \sigma_k \rvert}$$

represent exact geometric ratios held symbolically inside the {% include term.html id="qexpr" %} container via `OP_RATIO`.

### 3.4. The Discrete Laplace–Beltrami Operator (Δ / DCC_ANCHOR_LAPLACE_v1)
The Laplacian governing diffusion, wave propagation, and electrostatic potential is constructed constructively:

$$\Delta \equiv \mathbf{d}\delta + \delta\mathbf{d}, \quad \text{where } \delta = (-1)^{n(k-1)+1} \star \mathbf{d} \star$$

### 3.5. Exact Discrete Stokes' Theorem
Stokes' Theorem is not an asymptotic limit; it is an exact Diophantine identity on discrete meshes:

$$\sum_{K} (\mathbf{d}\omega)(K) \equiv \sum_{\partial K} \omega(\partial K)$$

Truncation error is zero by construction. Conservation laws (mass, charge, vorticity) hold identically without numerical dispersion.

---

## 4. Dynamical Q(anchor) Spectrum: Transcendental Continued Fractions

Transcendental numbers cannot exist as static Layer 1 digits. In $\mathrm{Qm}$ Calculus, they are anchored as **Deterministic Projective Continued Fraction Generators** inhabiting the Layer 3 {% include term.html id="qexpr" %} CAE-DAG container ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §4.3).

```
                 THE DYNAMICAL Q(anchor) SPECTRUM IN Qm CALCULUS
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. TRANSCENDENTAL INVARIANT ANCHORS                                         │
 │    • DCC_ANCHOR_PI_v1   : Circular ratio π, wave periods, harmonic phase    │
 │    • DCC_ANCHOR_TAU_v1  : Full-turn metric constant τ = 2π                  │
 │    • DCC_ANCHOR_E_v1    : Natural base e, solution to df/dx = f             │
 │    • DCC_ANCHOR_LN2_v1  : Natural logarithm ln(2), Landauer dissipation    │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. DISCRETE EXTERIOR CALCULUS (DEC) OPERATOR ANCHORS                        │
 │    • DCC_ANCHOR_DEC_D_v1    : Discrete Exterior Derivative d (d² ≡ 0)       │
 │    • DCC_ANCHOR_HODGE_v1    : Discrete Hodge Star Dual ⋆ (Metric contraction│
 │    • DCC_ANCHOR_LAPLACE_v1  : Discrete Laplace–Beltrami Operator Δ = dδ + δd│
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. DYNAMICAL FIELD & WAVE ANCHORS                                           │
 │    • DCC_ANCHOR_WAVE_v1     : Discrete Wave D'Alembertian Operator ◻        │
 │    • DCC_ANCHOR_FOURIER_v1  : Rational Discrete Fourier Transform (DFT)     │
 │    • DCC_ANCHOR_STOKES_v1   : Exact Coboundary Conservation ⟨∂σ, ω⟩ ≡ ⟨σ, dω│
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 4.1. The Circular Ratio (π / DCC_ANCHOR_PI_v1)
Governs circular phase rotations, Fourier analysis, and wave periods:

$$\pi = [3; \, 7, \, 15, \, 1, \, 292, \, 1, \, 1, \, 1, \, 2, \, 1, \, 3, \, 1, \, 14, \, \dots]$$

Evaluating $\pi$ to zoom level $z$ generates partial convergents $\frac{p_k}{q_k}$ bounded by strict residual conservation:

$$0 \le |r_k| \lt \frac{1}{q_k q_{k+1}}$$

### 4.2. The Turn Constant (τ / DCC_ANCHOR_TAU_v1)
Defined as the full-turn angular period:

$$\tau = Q(2) \times \pi = [6; \, 3, \, 1, \, 1, \, 7, \, 2, \, 146, \, \dots]$$

Eliminates arbitrary factor-of-two fractions in spatial rotor transformations ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})).

### 4.3. The Natural Exponential Base (e / DCC_ANCHOR_E_v1)
Governs exponential damping, thermal relaxation, and solutions to discrete differential equations ($\mathbf{d}f = f$):

$$e = [2; \, 1, \, 2, \, 1, \, 1, \, 4, \, 1, \, 1, \, 6, \, 1, \, 1, \, 8, \, \dots, \, 1, \, 1, \, 2k, \, \dots]$$

### 4.4. The Thermodynamic Logarithm (ln 2 / DCC_ANCHOR_LN2_v1)
Directly calibrates the physical Landauer dissipation limit:

$$\ln(2) = [0; \, 1, \, 2, \, 3, \, 1, \, 6, \, 3, \, 1, \, 1, \, 2, \, 1, \, 1, \, 4, \, \dots]$$

Guarantees platform-invariant accounting when translating physical Layer 0 energy consumption into {% include term.html id="compute-unit-u" text="U" %} units.

---

## 5. Lineage, Aperiodicity, and the Anchor-Cut in Time Evolution

In continuous mechanics, time is treated as an unmeasured parameter ($t \in \mathbb{R}$) that allows algorithms to discard initial states. In $\mathrm{Qm}$ Calculus, time evolution is the **strictly monotonic progression of discrete causal state transitions along the {% include term.html id="causal-arrow" %}**.

```
                         THE ANCHOR-CUT THEOREM
 Full Micro-Step Lineage (O(N) Replay - Rejected by SIMEMP):
   Q(0) ──► s_1 ──► s_2 ──► s_3 ──► ... ──► s_999,999 ──► S_terminal
   (Verifying every micro-step violates Memory Wall & wastes gigabytes of RAM)

 The Anchor-Cut Verification (O(K) Verification - BOC):
   Q(0)_N ════════► Q(anchor)_1 ════════► Q(anchor)_2 ════════► S_terminal
   [Silicon Root]    [Receipt R_1]        [Receipt R_2]         [Current State]
   • Replay between anchors: ELIMINATED!
   • Verifier only checks: H(Q(0)) + Chain of Signed Anchor Receipts + H(S_terminal).
   • Complexity: O(K) where K ≪ N.
```

### 5.1. Formal Causal Lineage of a Dynamic State
Every field state $S_k$ generated during dynamical evolution possesses an explicit lineage tuple:

$$\mathcal{L}(S_k) = \Big\langle \mathbf{P}(S_k), \, \kappa(S_k), \, \tau_{\mathrm{type}}, \, \mathcal{W}_{\mathrm{cum}}(S_k), \, \Pi_{\mathrm{receipts}}(S_k) \Big\rangle$$

- **Parent Commitment Set ($\mathbf{P}$):** References the preceding field state $S_{k-1}$ and the applied operator contract $\mathcal{M}_{\mathbf{d}}$.
- **Monotonic Causal Index ($\kappa$):** Increments deterministically with each discrete timestep ($\Delta t$).
- **Cumulative Work ($\mathcal{W}_{\mathrm{cum}}$):** Tracks the total Landauer work expended across the simulation run in units of $Q(1)$.

### 5.2. Aperiodic Memory Trajectory
Under `DCC_ANCHOR_Q5_APERIODICITY_v1` ([`docs/q2_q9.md`]({{ '/q2_q9.html' | relative_url }})), dynamical simulations avoid circular recurrence locks. The trajectory through memory space is non-repeating:

$$\forall i \neq j, \quad \mathcal{H}(S_i) \neq \mathcal{H}(S_j)$$

This guarantees an irreversible internal arrow of time without consulting external clocks.

### 5.3. The Anchor-Cut Verification Invariant
To audit or verify a dynamical simulation consisting of $N$ discrete timesteps passing through $K$ intermediate $Q(\mathrm{anchor})$ checkpoints ($K \ll N$):
1. An auditor does not replay the $N$ micro-steps across volatile memory.
2. The auditor verifies the **Anchor-Cut Chain**: the local origin {% include term.html id="q0" text="Q(0)" %}, the signed anchor receipts $\mathcal{R}_{\mathrm{anchor}}$, and the terminal state $S_N$.
3. Verification complexity collapses from $\mathcal{O}(N)$ memory bus traversals to $\mathcal{O}(K)$ receipt validations. Micro-states between anchors are safely archived or dereferenced via {% include term.html id="q-reuse" text="Q(reuse)" %}.

---

## 6. Coupling to Optimization Morphisms: Q(bypass) and Q(reuse)

In classical scientific simulations, processors waste extensive energy re-evaluating terms that are algebraically zero. $\mathrm{Qm}$ Calculus couples DEC directly to Category B optimization morphisms:

```
                  Q(bypass) IN DISCRETE EXTERIOR CALCULUS
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. THE POINCARÉ COBOUNDARY BYPASS (d² ≡ 0)                                  │
 │    • Mathematical Invariant: d ∘ d ≡ Q(0) ("Boundary of a boundary is zero")│
 │    • Classical Engine      : Evaluates mesh derivatives twice (O(N) work).  │
 │    • Q(bypass) Execution   : Gateway intercepts d(dω), immediately emits     │
 │                              RECEIPT_BYPASS_IDENTITY at ΔW = 0 cost!        │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. CONSERVATIVE FIELD FLUX BYPASS (Stokes Collapse)                         │
 │    • If a differential form is closed (dω = Q(0)), any boundary integral    │
 │      ∫_(∂K) ω collapses algebraically to Q(0).                              │
 │    • Q(bypass) skips numerical mesh integration entirely.                   │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. PROJECTIVE SERIES EARLY HALTING (Terminal Exactness)                     │
 │    • In continued fraction expansions or Taylor series, if r_k = Q(0),       │
 │      Q(bypass) halts all remaining terms, saving 100% of remaining budget.  │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 6.1. Poincaré Nilpotent Bypass
When an expression graph attempts to evaluate the curl of a gradient or the divergence of a curl ($\mathbf{d}(\mathbf{d}\omega)$):
- The {% include term.html id="simemp-gateway" %} matches the identity proof hash of `DCC_ANCHOR_DEC_D_v1`.
- {% include term.html id="q-bypass" text="Q(bypass)" %} halts evaluation in $\mathcal{O}(1)$ time and emits `RECEIPT_BYPASS_IDENTITY`.
- Zero compute budget is burned ($\Delta W = 0$).

### 6.2. Closed Form Stokes Bypass
If a differential form is algebraically closed ($\mathbf{d}\omega \equiv Q(0)$), boundary circulation integrals collapse to zero without numerical mesh iteration:

$$\sum_{\partial K} \omega \equiv Q(0)$$

### 6.3. Simplicial Mesh Deduplication via Q(reuse)
During spatial evolution, static geometric mesh configurations (dual metric tensors, Hodge star ratio matrices) are referenced across time steps via their immutable content commitment hashes using {% include term.html id="q-reuse" text="Q(reuse)" %}, eliminating redundant bit allocation across the Memory Wall.

---

## 7. Thermodynamic Governance and Singularity Immunity

In classical continuous fluid mechanics, the Navier–Stokes equations allow solutions to develop unphysical mathematical singularities (finite-time velocity blowups $\|u\|_{L^\infty} \to \infty$).

$\mathrm{Qm}$ Calculus provides **structural immunity against physical singularities**:
1. **Lattice Cutoff:** Spatial velocity cannot diverge because spatial gradients are bounded by the discrete scale quantum $\delta_z(d)$. Space cannot be divided infinitely.
2. **Thermodynamic Viscous Dissipation:** Physical fluids dissipate energy into microscopic thermal noise. In $\mathrm{Qm}$ Calculus, viscosity is modeled as an explicit dissipative sink exporting operational entropy via signed {% include term.html id="receipt" text="receipts" %}.
3. **Totality by Budget ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) Axiom 6):** Time evolution is allocated a finite work budget in units of {% include term.html id="q1" text="Q(1)" %}. An unphysical blowup is impossible: before any field value can exceed physical bounds, the budget exhausts and the engine halts cleanly with `RECEIPT_BUDGET_EXHAUSTED`.

---

## 8. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %} governing the {% include term.html id="licensed-work" %} across the {% include term.html id="cdqn" %} network:

1. **{% include term.html id="paternity-reference" %}:** All discrete exterior calculus solvers, continued fraction libraries, and dynamical simulators derived from this specification must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $\mathrm{Qm}$ discrete calculus methods or DEC operator anchors against the Author or community under the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}.
3. **Statutory Safe Harbor:** $\mathrm{Qm}$ Calculus operates as passive, neutral mathematical infrastructure. It exercises no editorial inspection over, and makes no claim upon, external simulation models, physical datasets, or sovereign {% include term.html id="payload" text="Payloads" %}.
