---
layout: default
title: Qexpr — Content-Addressed Expression DAG (Layer 3 Symbolic Container)
description: Canonical specification of the Layer 3 Content-Addressed Expression DAG (CAE-DAG), hash-consing deduplication, symbolic irrationals, six canonical node classes, and hardware-aware collapse under SIMEMP constraints.
version: 1.0.0
updated: 2026-10-07
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qexpr.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qexpr.html
terms_used:
  - qexpr
  - qn
  - q0
  - q1
  - compute-unit-u
  - simemp
  - dependencies-determinism
  - no-implicit-rule
  - structural-indirection
  - metric-exhaustion
  - computational-consistency
  - dcc-profile
  - universal-envelope
  - metric-envelope
  - security-envelope
  - receipt
  - causal-arrow
  - complexity-degree
  - local-first
  - zoom-z
  - remainder-r
  - dimension-d
  - terminal-exactness
  - q-anchor
  - q-reuse
  - q-bypass
  - qm
  - layer-0
  - layer-1
  - payload
  - licensed-work
  - paternity-reference
  - ssl
  - existential-invariant
  - operational-agility
  - boc-policy
---

# Qexpr — Content-Addressed Expression DAG: Layer 3 Symbolic Container

| Field | Specification |
|---|---|
| **Document Title** | Qexpr — Content-Addressed Expression DAG: Layer 3 Symbolic Container |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-10-07 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Specification — Layer 3 Structural Container (Complexity Degree 2) |

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
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Constitutional Placement

Within the master abstraction hierarchy ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) v1.3.0 §2.6, §5), **`Qexpr` is situated strictly at Layer 3 (Complexity Degree 2)**.

```
 +-------------------------------------------------------------------------------+
 | Layer 4+: Domain Lattices (Qm Mathematics, Qs Semantics, Qphy Physics)        |
 +---------------------------------------^---------------------------------------+
                                         │ Domain Projections
 +---------------------------------------+---------------------------------------+
 | LAYER 3: SYMBOLIC COMPOSITION LAYER (Qexpr / CAE-DAG)                         |
 | • Structural Container: Content-Addressed Expression DAG                      |
 | • Role: Wires Layer 2 atomic morphisms into bounded, evaluatable graphs       |
 | • Mechanics: Hash-consing, algebraic rewrite, compile-time collapse budget    |
 +---------------------------------------^---------------------------------------+
                                         │ Gateway Constraint & Budget Metering
 +---------------------------------------+---------------------------------------+
 | Layer 2: Governed Morphism Layer (Atomic Operations: +, -, ×, ÷, Rotors)     |
 +---------------------------------------^---------------------------------------+
                                         │ Gateway Onboarding
 +---------------------------------------+---------------------------------------+
 | Layer 1: Node Genesis Layer (Static Existence: Q(0), Q(1), Q(2)...Q(9), Axis d1)|
 +-------------------------------------------------------------------------------+
```

### 1.1. The Role of Layer 3
* **Layer 1** defines **what exists** (static ontological primitives: local origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0), unit [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1), single digits $Q(2)\dots Q(9)$ along axis $d_1$).
* **Layer 2** defines **atomic actions** (isolated morphisms: addition, subtraction, multiplication, Diophantine division, Clifford wedge products, spatial rotors, $Q(\mathrm{reuse})$, $Q(\mathrm{bypass})$).
* **Layer 3 (`Qexpr`)** defines **compositional structure**: how multiple Layer 2 atomic actions are wired into directed acyclic networks *before* they are evaluated or collapsed into physical memory.

### 1.2. Acknowledgment of Universal Fallibility
A core design tenet of the Qn architecture is the explicit recognition that **both human authors and artificial agents are fallible entities**. Neither is an absolute arbitrator:
* Humans are prone to specification errors, unmeasured assumptions, and logical oversights.
* Artificial agents and compilers are prone to algorithmic hallucinations, non-terminating expansions, and parameter misalignments.

Under [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}), safety cannot rely on presumed actor infallibility. `Qexpr` serves as the **mechanical error-containment envelope**: it enforces strict syntactic cycle freedom, explicit graph depth bounds, and finite execution budgets, ensuring that an error by any actor halts deterministically at the structural boundary without inducing systemic divergence.

---

## 2. The Content-Addressed Expression DAG (CAE-DAG) Architecture

Under the **Best of Choices (BOC) Policy** ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §7), `Qexpr` explicitly rejects naive pointer-heap trees in favor of a **Content-Addressed Expression Directed Acyclic Graph (CAE-DAG)**.

```
    CLASSICAL POINTER-AST                         CONTENT-ADDRESSED EXPRESSION DAG (CAE-DAG)
    (Ephemeral, Redundant, Lossy)                 (Canonical, Deduplicated, Dissipation-Free)
          [ Operation * ]                                        [ Operation * ]
          /             \                                        /             \
    [ Sub-term A ]  [ Sub-term A ]                        ┌────►[ Sub-term A ]◄────┐
      (Address 0x1)   (Address 0x2)                       │     (Hash H_A)         │
         /     \         /     \                          │        /     \         │
       [x]     [y]     [x]     [y]                        └──────[x]     [y]───────┘
   • 2x Memory bit allocations                             • Exact 1x Memory bit allocation
   • RAM pointer drift across reboots                     • Platform-invariant cryptographic identity
   • Dissipates 2·W Landauer heat                         • Direct coupling to Q(reuse) (W = 0)
```

### 2.1. Canonical Node Tuple Structure
Every symbolic node $\alpha$ within a `Qexpr` DAG is an immutable, content-addressed tuple:

$$\alpha = \Big\langle \mathrm{OpCode}, \, \mathcal{H}(\mathrm{Left}), \, \mathcal{H}(\mathrm{Right}), \, z, \, r, \, d, \, \mathrm{DCC}_{\mathrm{ref}} \Big\rangle$$

* **$\mathrm{OpCode}$:** Discrete identifier of the operation or terminal leaf generator.
* **$\mathcal{H}(\mathrm{Child})$:** Cryptographic content commitment hash of child dependencies. For leaf nodes, this references underlying Layer 1 primitives or input constants.
* **$z, r, d$:** Target scale quantum {% include term.html id="zoom-z" text="Zoom z" %}, conserved residual {% include term.html id="remainder-r" text="Remainder r" %}, and coordinate frame {% include term.html id="dimension-d" text="Dimension d" %}.
* **$\mathrm{DCC}_{\mathrm{ref}}$:** Abstract capability contract governing transformation ceilings and execution rights.

### 2.2. Structural Hash-Consing and $\mathcal{O}(1)$ Equivalence
Node identity is strictly mathematical:

$$\mathcal{H}(\alpha) = \mathrm{Hash}\Big(\mathrm{OpCode} \,\|\, \mathcal{H}(\mathrm{Left}) \,\|\, \mathcal{H}(\mathrm{Right}) \,\|\, z \,\|\, r \,\|\, d \,\|\, \mathrm{DCC}_{\mathrm{ref}}\Big)$$

1. **Deduplication:** Any sub-expression with identical semantics and dependencies yields the identical commitment hash. The memory allocator maps it to the exact same physical node.
2. **$\mathcal{O}(1)$ Structural Equality:** Verifying whether two sub-expressions $A$ and $B$ are identical requires zero recursive graph traversal:

$$A \equiv B \iff \mathcal{H}(A) == \mathcal{H}(B)$$

Testing algebraic equality collapses to a single machine-word integer comparison.

### 2.3. Thermodynamic Work Minimization ($W = 0$)
In physical Layer 0 substrates, allocating and deallocating memory across physical buses dissipates electrical capacitance ($\mathcal{O}(C V^2 f)$) and Landauer work ($W \ge k_B T \ln 2$).
* By enforcing hash-consing, repeated sub-terms in multi-dimensional Clifford geometric products ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})) are allocated **exactly once**.
* Reversible symbolic normalization within the CAE-DAG incurs **zero Landauer dissipation** ($W = 0$), directly operationalizing [`Q(reuse)`]({{ '/glossary.html' | relative_url }}#q-reuse) across the Memory Wall.

### 2.4. Acyclicity by Construction (Axiom 5)
In classical graphs, circular references create non-halting loops and stack crashes. In a Content-Addressed DAG, **a circular dependency is mathematically impossible**. Because $\mathcal{H}(\alpha)$ requires the child hash $\mathcal{H}(\beta)$ as an input, $\beta$ cannot declare $\alpha$ as a dependency without breaking the pre-image resistance of cryptographic hash functions. Acyclicity is guaranteed by construction, satisfying **Axiom 5** ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }})).

---

## 3. Elimination of Rounding and the Ontological Status of Irrationals

In classical floating-point systems (IEEE 754), rounding is an unmetered, lossy artifact of fixed register widths. Rounding silently discards trailing bits, generating non-deterministic drift and violating **Axiom 10**.

```
          CLASSICAL IEEE 754 vs. Qn CONSTRUCTIVE CONSERVATION
 Classical Lossy Rounding:
   1 / 3  ──►  0.33333333...  ──►  [ Truncated to 53 bits ]  ──►  Silent Entropy Loss
                                                                   (Drift accumulates)

 Qn Constructive Remainder Conservation (qm.md §4):
   1 / 3  ──►  Qexpr(OP_RATIO: 1/3)  ──►  Exact Symbolic State (Drift = 0)
                      │
                      ▼ Boundary Demand at Zoom z
               ⟨ ⊕, q_z, z, r_z, d ⟩ + Remainder Envelope
               • q_z : Discrete Integer Quotient
               • r_z : Conserved Residual (Total argument for scale z+1)
               • Zero bits discarded. Zero drift possible.
```

### 3.1. Prohibition of Rounding Drift
In the Qn universe, **implicit rounding is outlawed by Axiom 9 and Axiom 10**:
1. Within Layer 3, expressions are held in unevaluated, exact symbolic form. Drift is identically zero.
2. When projected onto physical hardware boundaries, quantities are partitioned into an integer quotient $q_z$ and an **exact conserved remainder** $r_z$ ([`docs/qm.md`]({{ '/qm.html' | relative_url }}) §4):

$$X = \big( q_z \cdot \delta_z(d) \big) + r_z, \quad \text{where } 0 \le |r_z| < \delta_z(d)$$

The remainder $r_z$ is never discarded; it is sealed within the remainder envelope as the exact causal input state for subsequent precision expansions.

### 3.2. Ontological Status of Irrationals and Transcendentals
Under **Axiom 2**, actual infinities cannot exist as storable or executable machine states. Consequently:
* An irrational number **cannot be an infinite decimal string** ($3.14159\dots$), as storing infinite digits requires infinite energy.
* **Transcendental and irrational quantities exist permanently as `Qexpr` symbolic generators.**

```
       TRANSCENDENTAL & IRRATIONAL ONTOLOGY IN THE Qn STACK
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ LAYER 3: SYMBOLIC GENERATOR (Permanent Invariant State)                     │
 │ • Minimal Metric Root √2 : Qexpr(OP_ROOT: D² - Q(2) = Q(0))                 │
 │ • Golden Ratio φ         : Qexpr(OP_ROOT: φ² - φ - Q(1) = Q(0))             │
 │ • Circular Ratio π       : Qexpr(OP_CF: [3; 7, 15, 1, 292, ...])            │
 │ • Base e                 : Qexpr(OP_CF: [2; 1, 2, 1, 1, 4, ...])            │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │               │ Boundary Collapse Demand (Target Zoom z, Budget W)          │
 │               ▼                                                             │
 │ LAYER 1 / 2: DISCRETE NUMERIC LEAF (On-Demand Physical Projection)          │
 │ • Quotient   : q_z ∈ ℕ (Exact discrete integer units at scale z)            │
 │ • Remainder  : r_z (Exact residual boundary; r_z < δ_z(d))                  │
 │ • Terminal   : If r_z = Q(0), early halt via Terminal Exactness (ΔH = 0)    │
 └─────────────────────────────────────────────────────────────────────────────┘
```

1. **Algebraic Irrationals ($\sqrt{2}, \phi$):** Stored as exact polynomial constraint equations ($D^2 - Q(2) = Q(0)$). They are manipulated symbolically via radical identities with zero rounding drift.
2. **Transcendental Constants ($\pi, \tau, e, \ln$):** Stored as deterministic continued fraction recurrence generators. Evaluating $\pi$ to zoom level $z$ computes an exact rational quotient and a conserved residual without floating-point error.

---

## 4. The Six Canonical Node Classes

To achieve computational completeness across all abstraction layers, `Qexpr` defines **Six Canonical Node Classes**:

```
                       THE 6 CANONICAL Qexpr NODE CLASSES
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. OP_RATIO     : Symbolic rational pairs (p / q) preserving exact ratios.  │
 │ 2. OP_ROOT      : Constrained Diophantine polynomials for algebraic roots.  │
 │ 3. OP_CF        : Projective continued fraction generators for transcendentals.│
 │ 4. OP_FOLD      : Bounded monoidal accumulation (∑, ∏) over discrete frames.│
 │ 5. OP_FRAME_MAP : Clifford geometric products, rotors, and axis permutations.│
 │ 6. OP_REWRITE   : Hash-consed equality saturation and identity bypass rules.│
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 4.1. Node Class 1: Exact Rational Pairs (`OP_RATIO`)
* **Role:** Represents unevaluated rational fractions $\frac{p}{q}$ ($q \neq Q(0)$) without premature projection onto decimal or binary scale lattices.
* **Evaluation:** Multiplicative and additive morphisms execute via exact Diophantine cross-multiplication:

$$\frac{p_1}{q_1} \times \frac{p_2}{q_2} = \frac{p_1 \times p_2}{q_1 \times q_2}, \quad \frac{p_1}{q_1} + \frac{p_2}{q_2} = \frac{(p_1 \times q_2) + (p_2 \times q_1)}{q_1 \times q_2}$$

### 4.2. Node Class 2: Constrained Algebraic Roots (`OP_ROOT`)
* **Role:** Represents algebraic radical roots as exact polynomial Diophantine equations ($P(X) = Q(0)$).
* **Evaluation:** Algebraic roots (such as $\sqrt{2}$ via `DCC_ANCHOR_SQRT2_v1`) are held symbolically. Multiplication by identical roots ($D \times D$) resolves immediately to integer constants ($Q(2)$) via $Q(\mathrm{bypass})$ without invoking root-extraction algorithms.

### 4.3. Node Class 3: Projective Recurrence Generators (`OP_CF`)
* **Role:** Generates transcendental numbers ($\pi, e, \tau$) through deterministic continued fraction expansions:

$$X = a_0 + \cfrac{1}{a_1 + \cfrac{1}{a_2 + \cfrac{1}{\ddots}}}$$

* **Evaluation:** Partial convergents $\frac{p_k}{q_k}$ are generated iteratively. The truncation error is strictly bounded by $0 \le |r_k| < \frac{1}{q_k q_{k+1}}$, and the residual is preserved in the remainder envelope.

### 4.4. Node Class 4: Bounded Monoidal Folds (`OP_FOLD`)
* **Role:** Governs accumulation ($\sum$, $\prod$, vector concatenation) over arrays, streams, or multi-axial frames.
* **Evaluation:** Requires an explicit step ceiling ($k \le k_{\max}$) and declared compute budget. Unbounded loops or infinite series are syntactically invalid under **Axiom 2** and **Axiom 6**.

### 4.5. Node Class 5: Frame Transformations and Permutations (`OP_FRAME_MAP`)
* **Role:** Governs multi-axial Clifford geometric products, Cayley rotor conjugations ($R \vec{v} R^\dagger$), and coordinate permutations ($S_n$ group).
* **Evaluation:** Rotors maintain unitary norm ($\|R\|^2 \equiv Q(1)$) by construction, eliminating trigonometric normalization cycles ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5).

### 4.6. Node Class 6: Canonical Equality Saturation (`OP_REWRITE`)
* **Role:** Executes reversible term rewriting prior to numerical collapse:
  * Identity bypass: $A + Q(0) \to A$ (emits [`Q(bypass)`]({{ '/glossary.html' | relative_url }}#q-bypass)).
  * Rotor annihilation: $R R^\dagger \to Q(1)$.
  * Nilpotency: $\vec{e}_i \wedge \vec{e}_i \to Q(0)$.
* **Evaluation:** Rewrites sub-graphs into their canonical minimal form at zero Landauer work cost ($W = 0$).

---

## 5. Hardware Capability-Aware Collapse Engine (HCA-Collapse)

While `Qexpr` remains an abstract, substrate-agnostic DAG at Layer 3, its physical execution on physical SoCs is governed by the **Hardware Capability-Aware Collapse Engine (HCA-Collapse Engine)**.

```
                               THE COMPILATION PIPELINE
 LAYER 3: INVARIANT ROAD (Qexpr CAE-DAG)
   Abstract, substrate-agnostic mathematical graph (Exact Diophantine Relations)
                               │
                               ▼ Hardware Capability-Aware Collapse Engine
 ══════════════════════════════╪══════════════════════════════════════════════════════
 HARDWARE-SPECIFIC LOWERING    │ (Reads Local DCC Hardware Profile: NPU / DSP / ASIC)
 ┌─────────────────────────────┴─────────────────────────────┐
 │ OPTION A: NPU / Systolic Array                            │ OPTION B: Integer ALU / RISC-V
 │ • Rewrite to integer Matrix-Multiply-Accumulate (MAC)     │ • Rewrite to CORDIC / Shift-and-Add
 │ • Constant-time, parallel execution over INT16/INT32      │ • Exact Diophantine reciprocal table
 └───────────────────────────────────────────────────────────┘
                               │
                               ▼
 LAYER 0 / 1: PHYSICAL SOC EXECUTION
   Zero floating-point drift · Constant-time execution · Bounded Landauer dissipation
```

### 5.1. Target-Specific Lowering Profiles
The compiler queries the node's local **DCC Hardware Capability Profile** (established during Layer 0 onboarding):
1. **NPU / Systolic Arrays:** Lowers multi-axial Clifford geometric products ($\vec{u} \cdot \vec{v} = \sum \eta_{kk} u_k v_k$) directly into **integer Matrix-Multiply-Accumulate (MAC)** instructions (INT16/INT32), completing contractions in constant time.
2. **Cryptographic Co-Processors:** Offloads hash-consing deduplication and node commitment hashing to hardware SHA-3 or BLAKE3 accelerators.
3. **Integer RISC-V / Microcontrollers:** Replaces division by multiplication using Barrett integer reciprocal tables with exact remainder correction loops.

### 5.2. Side-Channel Immunity via Constant-Time Compute
Variable-time operations leak secret cryptographic keys and algorithmic state via power analysis and cache timing. Lowering `Qexpr` nodes into fixed-cycle SoC instruction sequences ensures that execution duration depends strictly on declared metric budgets, providing hardware-level side-channel immunity.

### 5.3. Invariance Mandate
SoC-specific lowering is permissible if, and only if, the transformation is an **algebraic isomorphism**. Lowering passes that introduce floating-point approximations, mantissa truncations, or non-deterministic rounding are intercepted by the gateway and rejected.

---

## 6. Bounded Execution and Fallibility Circuit Breakers

To guarantee that fallible human directives or unaligned agentic processes cannot compromise the execution environment, `Qexpr` enforces three structural circuit breakers:

```
                      Qexpr FALLIBILITY CIRCUIT BREAKERS
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. STATIC GRAPH DEPTH CEILING                                               │
 │    Depth(DAG) ≤ Depth_max (Metered at ingestion; rejects runaway recursion) │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. NODE COUNT BUDGET ALLOCATION                                             │
 │    Count(Nodes) ≤ Node_max (Prevents memory exhaustion across Memory Wall)  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. METRIC COLLAPSE BUDGET CEILING                                           │
 │    Burn(U) ≤ Budget_Q1 (Guarantees deterministic halting via Axiom 6)        │
 └─────────────────────────────────────────────────────────────────────────────┘
```

1. **Static Graph Depth Ceilings:** If an expression tree exceeds the declared depth ceiling ($D > D_{\max}$), execution terminates immediately with receipt `RECEIPT_EXPRESSION_TOO_COMPLEX`.
2. **Node Allocation Bounds:** If hash-consing attempts to instantiate node counts exceeding the local Metric Envelope, compilation aborts with `RECEIPT_BUDGET_EXHAUSTED`.
3. **Totality by Budget (Axiom 6):** Every collapse operation carries an explicit compute budget in units of $Q(1)$. Non-terminating expansions are impossible; when the budget is consumed, the engine emits a dissipative receipt and halts.

---

## 7. License and Invariant Lineage

This specification strictly conforms to the **[Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)**:

### 7.1. Paternity Reference
All derivative implementations, symbolic rewrite engines, compilers, and intermediate representation parsers derived from `Qexpr` must preserve the canonical attribution:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 7.2. Open Core Invariants
1. **Anti-Patent Defense:** Commercial or derivative licenses terminate automatically upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free licensing is guaranteed for non-commercial, academic, and sub-threshold usage.

### 7.3. Passive Infrastructure Safe Harbor
The `Qexpr` symbolic container acts as passive, neutral technical infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, models, or [`Payloads`]({{ '/glossary.html' | relative_url }}#payload) processed within its expression graphs.
