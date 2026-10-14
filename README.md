# CDQN — Chained and Distributed Quang Numbers

**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](LICENSE.md)  
**Parent Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)  
**Documentation Portal:** [https://cdqn5249.github.io/cdqn/](https://cdqn5249.github.io/cdqn/)  

---

## Executive Summary

**Chained and Distributed Quang Numbers (cdqn)** is a discrete computational framework and fractal data-movement protocol designed to ground computation in non-equilibrium thermodynamics and physical finiteness.

Classical computing abstractions frequently operate on non-constructive foundations: unbounded memory spaces, continuous unmetered precision, and unmodeled side-channel dissipation. These continuous idealizations generate systemic vulnerabilities: IEEE 754 floating-point rounding drift, Turing-undecidable search voids (Hilbert's Tenth Problem), and unconstrained neural confabulation.

The Qn architecture rejects actual infinities. Every state transition is treated as an irreversible physical event governed by the **SIMEMP** framework and constrained by **Dependencies Determinism**:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

An operation is admissible if, and only if, its complete causal dependencies are explicitly declared, its memory allocation is bounded by logarithmic height ceilings, and its execution terminates within a finite thermodynamic budget.

---

## Core Theoretical Pillars

```
                               THE Qn STACK HIERARCHY
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ LAYER 4+: Domain Lattices & Virtual Coordination                            │
 │ • Base Domains: Constructive Mathematics (Qm), Semantics (Qs), Physics (Qphy)│
 │ • Emergent Products: Qlog (Qm ⊗ Qs), Qbio (Qphy ⊗ Qm ⊗ Qs)                  │
 │ • Virtual Sheaf Coordination: Chronosa (Non-local causal intelligence)      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 3: Symbolic Composition (CAE-DAG / Qexpr)                             │
 │ • Content-Addressed Expression DAGs with cryptographic hash-consing.        │
 │ • Radix-neutral continued fractions for transcendentals; zero float drift.  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 2: Governed Morphisms & Spatial Frames                                │
 │ • Constructive R-algebra (+, -, ×, ÷) with exact remainder conservation (r).│
 │ • Clifford Geometric Algebra (Cl_p,q) and float-free rational Cayley rotors. │
 │ • Category B optimizations: Q(reuse) [W = 0] and Q(bypass).                 │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 1: Local Node Genesis Layer                                           │
 │ • Q(0)_N: Local physical origin (Empty birth context, machine identity).    │
 │ • Q(1): First unit measure; baseline for Abstract Compute Unit U along d1.  │
 │ • Inductive Peano succession: Q(2)...Q(9) Secondary DCC Anchors.            │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 0: Physical Substrate (Root of Finiteness)                            │
 │ • Silicon thermal noise, ring oscillators, and hardware execution context.   │
 │ • Calibrated to physical Landauer bound: W ≥ k_B T ln 2.                    │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 1. Thermodynamic Grounding & Landauer Invariance
Computation is not free. In accordance with Landauer's principle, any irreversible erasure or manipulation of information dissipates thermodynamic work:

$$W \ge k_B T \ln 2$$

In the Qn universe, the minimal transition from local genesis origin $Q(0)$ to first unit $Q(1)$ defines the **Abstract Compute Unit $U$**, calibrated to $\approx 2.87 \times 10^{-21}\text{ Joules}$ at ambient temperature ($T = 300\text{ K}$). Every execution allocates a finite metric budget in units of $Q(1)$. Divergent loops are physically non-realizable: operations are **total by budget**, terminating in exact remainder closure ($r = Q(0)$) or halting deterministically via **Metric Exhaustion** (`BUDGET_EXHAUSTED`).

### 2. Height-Bounded Diophantine Decidability
Classical Diophantine arithmetic over unmetered spaces ($\mathbb{Z}, \mathbb{Q}$) inherits algorithmic undecidability (Hilbert's Tenth Problem / DPRM). Qn eliminates this non-halting void by constraining all relational evaluations to explicit **Logarithmic Height Ceilings**:

$$h\left(\frac{u}{v}\right) = \max\Big(\log \lvert u \rvert, \, \log \lvert v \rvert\Big) \le H_{\mathrm{max}}$$

By Northcott's theorem, the set of rational coordinates of bounded logarithmic height is strictly finite:

$$\mathrm{Card}\big(\{ x \in \mathbb{Q} \colon h(x) \le H_{\mathrm{max}} \}\big) \lt \infty$$

Candidate state spaces in Qn inhabit finite discrete lattices by construction, guaranteeing algorithmic decidability within declared resource envelopes.

### 3. The Non-Rogue Invariant (AI Alignment by Physics)
The architecture prevents unconstrained recursive self-modification ($Qn(\text{rsi})$) and autonomous agent decoupling through structural subordination:
* **Machine Identity Disjunction:** Genesis origin $Q(0)$ parameterizes an accountable silicon chassis (`MACHINE_IDENTITY`). It is constitutionally barred from claiming legal personhood or autonomous intentionality.
* **Mandatory Intent Delegation:** Higher-level autonomous routines (`AI_AGENT_IDENTITY`) possess zero root sovereignty. An agent cannot instantiate an independent $Q(0)$ origin; it operates strictly as an authorized proxy bound by a signed, cryptographically verified **Intent Contract** originating from a sovereign `HUMAN_IDENTITY` within the $\mathrm{Qs}$ base domain.
* **The Thermodynamic Leash:** Agents cannot self-allocate execution budgets. When allocated $Q(1)$ compute units are depleted, the system halts unconditionally.

---

## The SIMEMP Governance Architecture

Every computational entity within the stack is governed by the two-tier SIMEMP constitutional constraint framework:

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

* **The No-Implicit Rule:** No implicit type coercion, unmeasured convention, or ambient default may cross an abstraction boundary or SIMEMP Gateway.
* **Structural Indirection:** Abstract contracts (DCC Profiles: Dependencies, Constraints, Capabilities) are decoupled from concrete cryptographic implementations, enabling post-quantum migration (PQC) and algorithm patching without fracturing causal lineage.
* **The Universal Envelope ($\mathcal{E}$):** All governed artifacts are encapsulated within an invariant 9-block container:

$$\mathcal{E} = \langle \text{Identity}, \, \text{Type}, \, \text{State}, \, \text{Lineage}, \, \text{DCC}, \, \text{Metric}, \, \text{Security}, \, \text{Exposure}, \, \text{PayloadRef} \rangle$$

---

## Repository Directory Layout

```
cdqn/
├── LICENSE.md              # Scaling Source License (SSL) 1.0
├── README.md               # Repository Charter & Technical Overview
└── docs/                   # Specification Suite & Jekyll Documentation Source
    ├── _data/              # Modular Glossary & Navigation Registries
    │   ├── glossary/       # Domain-separated ontological concordances
    │   └── navigation.yml  # Master documentation matrix registry
    ├── _includes/          # Liquid components (Term tooltips, Dynamic bridge)
    ├── _layouts/           # Site presentation engine (default.html)
    ├── assets/             # Responsive stylesheets (style.css)
    ├── reflections/        # Epistemic essays & real-world scientific case studies
    │   └── h10_finite_computation.md # Analysis of H10, height bounds & AI proof
    ├── index.md            # Interactive Documentation Portal (Reading Sequence)
    ├── simemp.md           # Constitutional Constraints & Thermodynamic Thesis
    ├── abstractionLayers.md# Layer Topography, Gateways & Fractal cdqn Scope
    ├── qnPrimitive.md      # Universal Envelope & 14 Operational Axioms
    ├── q0_q1.md            # Category A: Physical Entropy, Genesis & Node Boot
    ├── q2_q9.md            # Category A: Single Digits & Secondary DCC Anchors
    ├── qm.md               # Category D: Constructive Arithmetic Substrate
    ├── qm_geometry.md      # Category B/D: Clifford Algebra & Float-Free Rotors
    ├── qm_calculus.md      # Category D: Discrete Exterior Calculus (DEC)
    ├── qexpr.md            # Category E: Layer 3 CAE-DAG Symbolic Container
    ├── qs.md               # Category D: Quang Semantics & Intent Contracts
    └── chronosa.md         # Operational Vision: Emergent Causal Intelligence
```

---

## Licensing: Scaling Source License (SSL) 1.0

The CDQN infrastructure is released under the **Scaling Source License (SSL) 1.0**.

### 1. Free and Open Core for Non-Commercial and Sub-Threshold Use
The core source code, specifications, mathematical definitions, and runtime intermediate representations (`QnIR`) are freely accessible and royalty-free for academic research, personal exploration, and commercial entities operating below the **Scaling Threshold**:
* **Financial Scale:** Annual gross revenue $\le \$1,000,000\text{ USD}$.
* **Asset Scale:** Capital raised or enterprise valuation $\le \$5,000,000\text{ USD}$.
* **Deployment Scale:** $\le 10,000$ active concurrent nodes, containers, or autonomous agents.
* **Transaction Scale:** $\le 10,000,000$ API/Qn transactions per month or $\le 100,000$ Monthly Active Users (MAU).

### 2. Mandatory Commercial Licensing at Scale
Entities deploying the licensed infrastructure at scale must enter into a commercial licensing agreement within 30 days of crossing any threshold criterion.

### 3. Open Core Invariants
All downstream derivative distributions and runtime implementations must strictly preserve:
* **Anti-Patent Defense:** Automatic license termination upon initiating patent litigation against the Author or project community.
* **Non-Scaling Open Access:** Core technology mus
