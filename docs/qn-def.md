[⬡ Back to Documentation Portal](index.html)

# The Formal Definition of a Quang Number (Qn)

[Next: Base-10 Seeds & Cryptography →](qn-base.md)

## Metadata

| Field | Value |
| :--- | :--- |
| **Version** | `1.0` |
| **Timestamp** | `2026-08-06` |
| **Author** | Christophe Duy Quang Nguyen |
| **Location** | Bao Loc, Vietnam |
| **Repository** | [`cdqn5249/cdqn`](https://github.com/cdqn5249/cdqn) |
| **AI Engine** | Qwen — Qwen Studio, Alibaba Group |
| **License** | [Scaling Source License (SSL v1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

## ⬡ Document Profile

- **id:** `qn-def`
- **version:** `1.0`
- **type:** `spec`
- **layer:** `middle`
- **depends:** `simemp-manifesto` · `qn-base`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

This specification formally defines the **Quang Number (Qn)** — a capability-bounded algebraic type that abstracts arbitrary computing states, operations, natural-language semantics, and system intents. It defines the canonical Trinity Tuple `⟨S, V(Qexpr), M⟩`, the computable distance function `d`, exact zero-drift arithmetic, memory persistence via `Q(reuse)`, the twin domains `Qm`/`Qs`, sublinear search, and the total `QnIR` execution engine. It adopts the cautious derive-then-verify epistemology of the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## 1. Epistemological Foundation

> **Governed by:** `[I]`

A Quang Number (Qn) is a **capability-bounded algebraic type** designed to abstract arbitrary computing states, operations, natural-language semantics, and system intents. It shifts abstraction from **concealing complexity** (opaque black boxes) to **structuring complexity** (luminescent, verifiable clarity — "Quang").

A Qn is a **computational object**, not a Platonic ideal: it exists only inside a machine that computes, is bound to **finitism**, and must always halt. Every Qn instance exists strictly within the boundary constraints set by the SIMEMP Manifesto: the Existence Triad `[I] × [S] × [M]` (invariants), the agile vectors `[E] · [Mod] · [P]`, and the Best-of-Choices (BoC) policy engine.

---

## 2. The Canonical Trinity State Model

> **Governed by:** `[I]` · `[S]` · `[M]`

A Quang Number is formally instantiated as a 3-element canonical tuple:

> **`Qn = ⟨ S , V(Qexpr) , M ⟩`**

### 2.1 The Security Anchor `[S]` & The Null Law (∅)

> **Governed by:** `[S]`

- **Structure:** `⟨ I , Σ ⟩`
- **Identity (I):** cryptographic provenance, public key, or namespace, rooted in a physically identified entity (`Q(harvest)`).
- **Signature (Σ):** integrity proof over `I ‖ V ‖ M`.
- **Enforced Termination:** `[S]` additionally carries a guarantee of bounded termination; a non-terminating payload is a security fault.
- **Null Law (∅):** `Verify(Σ) == FALSE ∨ M == Undefined ⇒ Qn = ∅`.

A payload that fails cryptographic verification, lacks a valid metric anchor, or cannot provably halt evaluates to the Empty Set (∅) and cannot be instantiated.

### 2.2 The Exact Value Anchor `[V]` & Quang Expressions (Qexpr)

> **Governed by:** `[M]`

`V(Qexpr)` is a **homoiconic payload buffer** storing exact, un-truncated mathematical or semantic states. It holds both the active view (`V_z`) and persistent residuals (`R = ⟨R_macro, R_micro⟩`) natively inside a single contiguous memory block, with zero precision loss.

### 2.3 The Metric Anchor `[M]`

> **Governed by:** `[M]`

> **`M = ⟨ DomainType , z , D , C , DCC , P ⟩`**

- **DomainType ∈ {Maths (Qm), Semantics (Qs), Hybrid}:** governs operational interpretation.
- **z = ⟨z_macro, z_micro⟩:** Two-Way Zoom Window defining scale ceiling and resolution floor.
- **D (Dimensionality):** universal dimension parameter (`+D` vector expansion, `D ∈ ℚ+` fractal scaling, `−D` constraint mask). Per the **Dimensional Regime Law** (manifesto §IX.3): algebraic/integer `D ∈ {1,2,4,8}` (Hurwitz normed division algebras; complex numbers are `D = 2`, with no imaginary primitive), and fractal/fractional `D ∈ ℚ+` (aperiodic self-similar structures).
- **C (Chaining Hook):** topological link specifying algebraic operators (`+`, `×`) for Qm or semantic predicates for Qs.
- **DCC Profile = ⟨D_dep, C_con, K_cap⟩:** self-describing profile of Dependencies, Constraints, and Capabilities.
- **P (Sparse Pivots):** sparse vantage-point vector enabling O(1) Triangle Inequality Pruning for sublinear O(log N) search.

### 2.4 The Computable Distance Function `d`

> **Governed by:** `[M]`

To satisfy the finite-metric-space requirement, `d` is defined as a bounded, deterministic, computable metric over discrete finite sets. *To be verified by QnLang.*

- **Topological distance (search/routing):** `d_T(Qa, Qb) = max_k | P_a[k] − P_b[k] |` (Chebyshev over the sparse pivot vector), enabling O(1) Triangle Inequality Pruning.
- **Value distance (proximity):** `d_V(Qa, Qb) = | V_a ⊖ V_b |`, computed as an exact canonical projected ratio over the finite ring, evaluated at zoom `z` with Soft Halt.
- **Anchors:** `d(Q0, Q1) = 1` (unit gauge) and the **Isometric Mirror Anchor Law** `d(Qorigin, +Q) ≡ d(Qorigin, −Q)`.

Both components are evaluable in finite time; the operative metric is `d_T` for routing and `d_V` for value proximity.

### 2.5 The S×M Coupling Rationale

> **Governed by:** `[S]` × `[M]`, grounded by `[I]`

Enforced termination (`[S]`) is only guaranteeable inside a bounded finite metric space (`[M]`); halting requires a boundary, `[M]` supplies it, `[S]` enforces it, and `[I]` grounds both in a physical entity. The invariants are coupled, not independent.

---

## 3. Arithmetic & Silicon ISA Reduction

> **Governed by:** `[M]` · `[E]`

Arithmetic is reduced to 2 Primary Operators and 2 Unary Inverses, aligned with abstract algebra and bare-metal register logic:

> **Primary Operators: `{ + , · }` | Unary Inverses: `{ Negate(−x) , Reciprocal(x⁻¹) }`**

- **Elimination of IEEE-754:** IEEE-754 notation is completely eliminated to prevent non-deterministic rounding drift. Numbers are represented as **Canonical Projected Ratios `[N, D]`** or Posit lattices.
- **Lazy Canonicalization:** intermediate micro-transformations remain unreduced in memory to maximize ALU speed; reduction is deferred until crossing an `S × M` boundary or projecting to the Base10 visual ceiling.
- **Type Unification:** classical set fragmentation (`ℕ ⊂ ℤ ⊂ ℚ`) is unified under a single Qn state structure, governed purely by denominator and sign flags inside Metric Anchor `M`.

---

## 4. Memory Persistence (Q(reuse)), Space-Time Dilation & Soft Halting

> **Governed by:** `[E]` · `[M]`

Qn replaces paper-and-pencil legacy with native memory persistence (`Q(reuse)`):

- **Q(reuse) — retrieval + residuals:** `Q(reuse)` is the retrieval of past computed states, reused **only when `Cost(retrieval) < Cost(recompute)`**, and it persists exact residuals `R` so precision is never lost.
- **HARD HALT (R == 0):** operation resolves with exact precision; returns active view `V_z`; zero residual memory required.
- **SOFT HALT (R ≠ 0):** returns active view `V_z`; persists exact residual `R = ⟨R_macro, R_micro⟩` in `Q(reuse)` SRAM for O(1) depth expansion.
- **Space-Time Memory Dilation:** respects the physical Memory Wall. Spatial dilation (zoom `z`) compresses bandwidth across the memory bus; temporal dilation exploits the 250-cycle DRAM fetch delay as an active window for `Q(reuse)` residual folding, cryptographic pre-checks, and `Q(bypass)` identity evaluations.

---

## 5. Multi-Domain Abstraction (Qm & Qs) via Qexpr & DCC

> **Governed by:** `[Mod]` · `[M]`

Every Qn carries a self-describing DCC Profile inside its Metric Anchor `M`: `DCC = ⟨ D_dep, C_con, K_cap ⟩`. This enables native operation across two twin abstraction layers:

- **Qm (Quang Maths):** deterministic algebraic, positional, geometric, and tensor evaluations. Continuous functions and series are modeled as **unbounded-composability Qexpr generator chains** evaluated to bounded zoom `z` with Soft Halt — never as completed infinities.
- **Qs (Quang Semantics):** natural language, context, intent, knowledge graphs, and logic trajectories. Intent is modeled as a **Qexpr** — a multivariate polynomial with unknowns resolved via `SUBST` and `ELIM`.
- **Qs ⇒ Qm Symbiosis (Mind/Matter/Joint):** Qs intent chains delegate exact calculations to Qm chains via Chaining Hook `C`, returning zero-drift results. This is the computational instance of the **Mind/Matter/Joint** law (manifesto §VIII): intent (Mind) becomes verified result (Matter) only through the lawful Joint transition (`SUBST`/`ELIM`/`EVAL`) — the structural defense against AI hallucination.

---

## 6. Spatial Mapping, Sparse Anchors & Sublinear Search

> **Governed by:** `[E]` · `[M]`

- **Aperiodic Einstein Tile Memory Mapping:** memory is partitioned using aperiodic monotiles (Einstein tiles / quasicrystals), eliminating periodic hash collisions, preventing fragmentation, and providing fractal self-similar scaling across zoom `z`.
- **Sparse Q(anchors) Landmarking:** Qn primitives carry zero anchor overhead; number-theoretic landmarks (`Q(primes)`, `Q(twinPrimes)`, `Q(collatz)`, `Q(goldbach)`) exist strictly as sparse region fences (1 anchor per 10,000 items, <0.01% overhead).
- **Sublinear O(log N) Search:** routers use the Pivot Vector `P ∈ M` and sparse fences for Triangle Inequality Pruning: `LowerBound = Max | P_q[k] − P_i[k] | > Radius r ⇒ Instant Prune in O(1)`.

---

## 7. Execution Engine, Q(bypass) & QnIR Opcodes

> **Governed by:** `[E]` · `[S]`

The `QnLang` compiler and `QnIR` runtime enforce `Q(bypass)` to short-circuit proven identities in **O(1)** ALU steps (`x+0`, `x·1`, `x·0`), and weigh `Cost(Lookup in Q(reuse))` versus `Cost(ALU Compute)` via the BoC cost engine.

**BoC Decision Policy:** every operation resolves `argmin over { BYPASS, REUSE, COMPUTE } of Cost(op)`, subject to `[S]×[M]` holding, source→destination freshness (stale ⇒ ∅), and Total Functionality.

`QnIR` operates on Qexpr through 4 fundamental opcodes:

| Opcode | Mathematical Function | Compiler / Runtime Action |
| :--- | :--- | :--- |
| `EVAL(Qexpr, z)` | Evaluation & Projection | Projects Qexpr through zoom `z`; triggers Soft Halt. |
| `SUBST(Qexpr, x, V)` | Variable Substitution | Binds context value `V` to unknown `x`. |
| `ELIM(Qexpr, C)` | Polynomial Elimination | Eliminates unknowns using constraints `C`, over finite bounded sets with a provable step bound. |
| `FOLD(Qexpr, R)` | Accumulator Folding | Folds exact residual terms into persistent `Q(reuse)` memory. |

**Total Functionality (Turing Incompleteness):** `QnIR` is total; every instruction carries a derivable upper bound on time and space prior to execution. *We derive this by construction; QnLang shall enforce and verify it.*

---

## 8. Abstraction Boundaries & Register Projection

> **Governed by:** `[M]`

The Qn definition is grounded between two boundaries — a bidirectional projection ladder:

- **CEILING (Semantic Interface / Base10):** human & AI-agent presentation; observable state; visual/document projection; natural-language intent.
- **MIDDLE (The Qn / CDQN Universe):** governed by the Existence Triad; verified by QnLang & QnIR; executing fractal Qexpr chains.
- **FLOOR (Silicon Ground Truth / Base2):** packed 64-bit registers (`Qn,64`), `r0:r1` register-pair mapping, SIMD multi-lane execution, physical voltage, Space-Time Dilation. **WHERE ABSTRACTION ENDS.**

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Documentation portal & navigation map → [`index.md`](index.md)
- Digit seeds, Q(harvest), Dual-Ring PQC → [`qn-base.md`](qn-base.md)
- Qn,64 ABI & Status codes → [`qn-primitives.md`](qn-primitives.md)
- SQS topologies & zero-trust → [`sqs-def.md`](sqs-def.md)
- Canonical term definitions → [`glossary.md`](glossary.md)
