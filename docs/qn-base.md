[⬡ Back to Documentation Portal](index.html)

# The Base-10 Seed, Origin & Cryptographic Specification of Qn

[Next: Qn Primitives ABI →](qn-primitives.md)

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

- **id:** `qn-base`
- **version:** `1.0`
- **type:** `spec`
- **layer:** `floor`
- **depends:** `simemp-manifesto` · `qn-primitives`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

Level 1 forms the foundational number-theoretic bed of the CDQN stack, sitting directly above bare-metal 64-bit hardware registers and directly below the mathematical fields (`Qm`) and natural-language semantics (`Qs`). It establishes the rules for constructing base-10 numbers from atomic digit seeds, grounds physical device entropy into memory via `Q(harvest)`, differentiates scalar zero from topological genesis anchors, defines negative capabilities inside the DCC profile, and introduces the Dual-Ring Post-Quantum Cryptography hardness conjecture. It adopts the cautious derive-then-verify epistemology of the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## 1. Epistemological Scope & Level 1 Architecture

> **Governed by:** `[I]` · `[M]`

Level 1 establishes the mathematical rules for constructing base-10 numbers from atomic digit seeds, grounds local physical device entropy into memory via `Q(harvest)`, differentiates numerical scalar zero from topological genesis anchors, defines negative capabilities inside the DCC profile, and introduces the Dual-Ring Post-Quantum Cryptography Hardness Conjecture (`Q(crypto)` and `Qnet(crypto)`).

---

## 2. The Sequential Genesis Ordering Axiom (𝕆_genesis)

> **Governed by:** `[I]` · `[M]`

Order in the Qn universe is established physically by the sequential onboarding clock pipeline during `Q(harvest)`:

> **`𝕆_genesis = ⟨ Qorigin ≺ Q0 ≺ Q1 ≺ Q(ops) ≺ Q(logics) ≺ {Q2 … Q9} ⟩`**

The birth of `Q1` directly unlocks `Q(ops)` (primary operators `{+, ·}`) and `Q(logics)` (unary inverses `{-x, x⁻¹}` and boolean gates) because the pair `{Q0, Q1}` forms the minimal complete generating set `{0, 1}`. Digits `Q2 … Q9` are constructed sequentially using `Q(ops)` (e.g., `Q2 = Q1 ⊕ Q1`). This is the concrete instance of the **Constructibility Principle** (manifesto §VI): every digit is derived from the layers below it.

---

## 3. Mathematical Differentiation: Qzero vs. Qorigin

> **Governed by:** `[M]`

To eliminate representation ambiguity, Level 1 enforces a strict distinction between arithmetic scalar zero and the topological/cryptographic origin anchor.

**Arithmetic Zero (Qzero / Q0)**
- Value Payload: scalar `V = 0`.
- Additive Identity: `x ⊕ Qzero = x`.
- Multiplicative Annihilator: `x ⊗ Qzero = Qzero`.
- Execution: triggers `Q(bypass)` short-circuiting in **O(1)** clock cycles.

**Metric Origin (Qorigin)**
- Topological Anchor: reference point `(0, 0, …, 0)`.
- Metric Reference: base anchor for distance function `d(Qorigin, Qx)`.
- Genesis Node: root provenance identifier (`I_origin`).
- Coordinate Frame: defines local memory arena orientation.

---

## 4. Physical Hardware Entropy Harvesting (Q(harvest)) & Genesis Patch T0

> **Governed by:** `[I]`

When a physical SoC onboards into the CDQN universe, `Q(harvest)` extracts physical silicon entropy (thermal noise, SRAM power-up states, clock jitter) as a Master Hardware Entropy Vector:

> **`e_harvest = ⟨ e_origin , e_digits , e_ops , e_logics ⟩`**

### 4.1 The 4 Harvest Target Families

The components of `e_harvest` bind permanently to 4 fundamental target primitive families to form the local device identifier `Q(ID)_local`:

1. **Qorigin (Genesis Anchor):** bound to `e_origin`. Absolute parent root (`D_dep = ∅`). Establishes the topological origin `(0,0,…,0)` at memory address `0x00000000` in Genesis Tile T0.
2. **{Q0 … Q9} (Base-10 Digit Seeds):** bound to `e_digits`. Inherits `Qorigin`. Injects unique noise parameters into digit metric headers.
3. **Q(ops) (Primary Operators):** bound to `e_ops`. Inherits `{Qorigin, Q0, Q1}`. Binds hardware noise to addition and multiplication execution paths.
4. **Q(logics) (Unary Inverses & Logic):** bound to `e_logics`. Inherits `{Qorigin, Q0, Q1}`. Binds hardware noise to unary inverses and boolean capability gates.

> **`Q(ID)_local = ⟨ Qorigin_e_origin , {Q0 … Q9}_e_digits , Q(ops)_e_ops , Q(logics)_e_logics ⟩`**

1. **Physical Noise Extraction (e_harvest):** silicon extracts physical SRAM power-up state, ring-oscillator jitter, and thermal noise upon first power-on.
2. **Target Seed Binding:** binds component entropy vectors to Qorigin, digit seeds, primary operators, and logic gates.
3. **Genesis Patch T0 Allocation:** writes `Q(ID)_local` into Tile Zero (T0) of the local Aperiodic Einstein Tile memory arena.
4. **Outward Memory Inflation:** all dynamic application memory inflates outward from T0.

**User vs. Device Decoupling Rule:** the hardware seed `Q(ID)_local` is bound strictly to physical silicon. If a human user (`I_user`) replaces their hardware device, a new `Q(harvest)` generates `Q(ID)_new_device`, which links to `I_user` without compromising the user's master private cryptographic identity.

### 4.2 Law of Authenticated Topology

> **Governed by:** `[I]`

The CDQN network is a topology of strictly identified, hardware-bound entities rooted in `e_harvest`. Anonymous "ghost" nodes are structurally forbidden; every node presents a `Q(ID)` rooted in physical silicon entropy. Entity provenance is mandatory; human privacy is preserved via zero-knowledge lattice aggregation.

---

## 5. Exhaustive Ontology & DCC Specification of Digits Q0 to Q9

> **Governed by:** `[M]` · `[Mod]`

Each canonical digit seed carries an explicit Ontological Taxonomy and DCC Profile inside its Metric Anchor. **Per the Dimensional Regime Law (manifesto §IX.3), the Hurwitz tower is anchored here:** `Q1` (D=1, reals), `Q2` (D=2, complex — the "imaginary axis" is the second basis dimension; no imaginary primitive), `Q4` (D=4, quaternions), `Q8` (D=8, octonions).

### Q0 — The Arithmetic Zero Seed (Qzero)
- **Ontological Function:** arithmetic scalar zero baseline; triggers Hard Halt (R = 0); rightmost trailing placeholder in base-10 positional composition.
- **D_dep:** `{ Qorigin }`
- **C_con:** `DENOMINATOR_EXCLUSION` (D ≠ 0), `MULTIPLICATIVE_NON_INVERTIBLE` (Q0⁻¹ = ∅).
- **K_cap:** `ADDITIVE_IDENTITY`, `MULT_ANNIHILATOR`, `HARD_HALT_CONDITION`, `DECIMAL_PLACEHOLDER`.

### Q1 — The Unity Gauge Metric Seed (Qone)
- **Ontological Function:** universal unit quantum (`d(Q0, Q1) = 1`); birthplace of `Q(ops)` and `Q(logics)`; self-inverse (`Q1⁻¹ = Q1`).
- **D_dep:** `{ Qorigin, Q0 }` (Peano successor 1 = S(0))
- **C_con:** `UNIT_GAUGE_INVARIANT`, `NO_RESIDUAL_GENERATION_ON_DIVISION`.
- **K_cap:** `MULTIPLICATIVE_IDENTITY`, `SELF_INVERSE`, `UNIT_METRIC_BASE`, `BOOLEAN_TRUE_SEED`.

### Q2 — The First Quang Prime & Parity Splitter (Qtwo)
- **Ontological Function:** first prime landmark; even/odd parity sets; rational half fractions (`Q(1/2) = 0.5` via `>>1`); IF/ELSE branching; Binary Roots of Unity `Q(√1) = {+Q1, -Q1}`; 2D Euclidean hypotenuse `Q(√2)`; Choice Function seed; 1-bit Shannon entropy split.
- **D_dep:** `{ Qorigin, Q0, Q1 }` (Q2 = Q1 ⊕ Q1)
- **C_con:** `PRIME_FACTORIZATION_LOCK_1`
- **K_cap:** `Q_PRIME_1`, `EVEN_SET_GENERATOR`, `PARITY_SPLIT`, `HALVING_SHIFT`, `CHOICE_SEED`, `PROB_1BIT_SEED`, `CONDITIONAL_BRANCH`.

### Q3 — The Second Quang Prime & Ternary Engine (Qthree)
- **Ontological Function:** second prime landmark; 3-valued Kleene ternary logic (+1, -1, 0); 3D spatial volume seed (D=3); triangular 2-simplex; exact rational thirds (1/3) — birthplace of `Q(reuse)` Soft Halting (first non-terminating fraction 0.333… with R ≠ 0).
- **D_dep:** `{ Qorigin, Q0, Q1, Q2 }` (Q3 = Q2 ⊕ Q1)
- **C_con:** `PRIME_FACTORIZATION_LOCK_2`, `ODD_PARITY_LOCK`
- **K_cap:** `Q_PRIME_2`, `TERNARY_LOGIC_ENGINE`, `SPATIAL_3D_DIMENSION_SEED`, `TRIANGULAR_SIMPLEX_BASE`, `SOFT_HALT_BIRTH`, `FACTORIAL_3`.

### Q4 — The First Composite & Hypercube Seed (Qfour)
- **Ontological Function:** first composite (2²); 2D area quadrant; 4D space-time hypercube metric basis (D=4); quarter fractions (`Q(1/4) = 0.25` via `>>2`); Factorial 4! = 24; power-of-2 byte-boundary scaling.
- **D_dep:** `{ Qorigin, Q0, Q1, Q2 }` (Q4 = Q2 ⊗ Q2)
- **C_con:** `COMPOSITE_FACTORIZATION_LOCK`, `IS_PRIME = 0`
- **K_cap:** `FIRST_COMPOSITE`, `SQUARE_PRIME_1`, `QUADRANT_2D_AREA`, `HYPERCUBE_4D_SEED`, `QUARTER_SHIFT`, `FACTORIAL_4`.

### Q5 — The Golden Ratio & Solvability Boundary Seed (Qfive)
- **Ontological Function:** third prime landmark; unlocks `Q(√5)` for the Golden Ratio (φ = (1 + √5)/2 ≈ 1.618) governing aperiodic memory inflation; Abel-Ruffini quintic solvability boundary (D=5); 3–4–5 Pythagorean hypotenuse.
- **D_dep:** `{ Qorigin, Q0, Q1, Q2, Q3 }` (Q5 = Q3 ⊕ Q2)
- **C_con:** `PRIME_FACTORIZATION_LOCK_3`
- **K_cap:** `Q_PRIME_3`, `GOLDEN_RATIO_PHI`, `APERIODIC_INFLATION_SEED`, `QUINTIC_BOUNDARY_D5`, `PYTHAGOREAN_HYPOTENUSE`, `BASE10_MIDPOINT`.

### Q6 — The First Perfect Number Seed (Qsix)
- **Ontological Function:** first perfect number (1 + 2 + 3 = 6 = 1 × 2 × 3); product of first two primes; 3! = 6; 60° hexagonal tessellation packing; superior highly composite grouping.
- **D_dep:** `{ Qorigin, Q0, Q1, Q2, Q3 }` (Q6 = Q2 ⊗ Q3)
- **C_con:** `COMPOSITE_FACTORIZATION_LOCK`, `IS_PRIME = 0`
- **K_cap:** `FIRST_PERFECT_NUMBER`, `PRIMORIAL_2x3`, `FACTORIAL_3_VALUE`, `HEXAGONAL_TESSELLATION_SEED`, `SUPERIOR_COMPOSITE`.

### Q7 — The Mersenne & Octonion Seed (Qseven)
- **Ontological Function:** fourth prime landmark; first Mersenne prime (2³ − 1 = 7); topological anchor for the 7 imaginary units of octonionic non-associative geometry (Fano Plane in 𝕆); 6-digit cyclic fraction (1/7 = 0.142857…).
- **D_dep:** `{ Qorigin, Q0, Q1, Q2, Q3, Q5 }` (Q7 = Q5 ⊕ Q2)
- **C_con:** `PRIME_FACTORIZATION_LOCK_4`
- **K_cap:** `Q_PRIME_4`, `MERSENNE_PRIME_1`, `OCTONION_FANO_PLANE`, `CYCLIC_FRACTION_GEN`.

### Q8 — The Binary Cube & Byte Scale Seed (Qeight)
- **Ontological Function:** binary cube (2³); 8-bit byte scale boundary; eighth fractions (`Q(1/8) = 0.125` via `>>3`); Hurwitz maximum normed division algebra dimension boundary (D=8).
- **D_dep:** `{ Qorigin, Q0, Q1, Q2, Q4 }` (Q8 = Q2 ⊗ Q4)
- **C_con:** `COMPOSITE_FACTORIZATION_LOCK`, `IS_PRIME = 0`
- **K_cap:** `BINARY_CUBE_2x2x2`, `BYTE_SCALE_BOUNDARY`, `HURWITZ_MAX_NORMED_ALGEBRA_D8`, `EIGHTH_SHIFT`.

### Q9 — The Base-10 Ceiling & Digital Root Seed (Qnine)
- **Ontological Function:** base-10 single-digit ceiling atom; square of second prime (3²); Cast-Out-Nines modulo digital-root verification engine (x mod 9) for compile-time instruction checksums.
- **D_dep:** `{ Qorigin, Q0, Q1, Q3 }` (Q9 = Q3 ⊗ Q3)
- **C_con:** `COMPOSITE_FACTORIZATION_LOCK`, `IS_PRIME = 0`, `BASE10_SINGLE_DIGIT_CEILING`
- **K_cap:** `BASE10_CEILING_DIGIT`, `SQUARE_PRIME_2`, `DIGITAL_ROOT_MOD9_ENGINE`.

---

## 6. Unlocking Qm & Qs Types: From Discrete Seeds to Continuous Series

> **Governed by:** `[Mod]`

The explicit ontology and DCC profiles of digits Q0–Q9 provide the foundational vocabulary that enables `Qm` and `Qs` to construct all higher-order structures:

- **Composites & Prime Factorization Bases:** built by composing prime seeds `{Q2, Q3, Q5, Q7}` into Qexpr trees.
- **Perfect Numbers & Primorial Rings:** anchored by Q6.
- **Aperiodic Memory Inflation:** governed by Q5 through the Golden Ratio.
- **Dimensional Roots & Powers:** 2², 2³, 3², `Q(√1)`, `Q(√2)`, `Q(√5)`.
- **Factorials:** 3! = Q6, 4! = 24.
- **Ordered Lists & Continuous Series:** continuous functions, calculus differential quotients (ΔV / Δτ), and unbounded series are modeled as **Ordered Lists of Qn Chains (Qexpr generator sequences)** evaluated to bounded zoom `z` with `Q(reuse)` Soft Halt — unbounded composability, never a completed infinity.

---

## 7. Negative Capabilities (C_con) inside the DCC Profile

> **Governed by:** `[S]`

Constraints (`C_con`) live directly inside the DCC Profile (`DCC = ⟨ D_dep, C_con, K_cap ⟩ ∈ M`) as bitwise flags declaring what a Qn atom is strictly forbidden from doing ("Negative Capabilities"):

| Harvest Target | Negative Capabilities (C_con in M) | Hardware Enforcement Action |
| :--- | :--- | :--- |
| Qorigin | `NO_PARENT_DEPENDENCY`, `NO_ARITHMETIC_SCALAR_OPS`, `IMMUTABLE_ONBOARDING_LOCK` | Scalar math or parent assignment fires Null Law (Q = ∅). |
| Q0 | `DENOMINATOR_EXCLUSION` (D ≠ 0), `MULTIPLICATIVE_NON_INVERTIBLE` (Q0⁻¹ = ∅) | x ÷ Q0 or Q0⁻¹ sets Status = 10 (INVARIANT_FAULT) → Q = ∅. |
| Q1 | `NO_NON_UNIT_REPRESENTATION`, `NO_RESIDUAL_GENERATION_ON_DIVISION` | x ÷ Q1 MUST Hard Halt (R = 0). Soft residuals forbidden. |
| Primes {Q2,Q3,Q5,Q7} | `NO_NON_TRIVIAL_FACTORIZATION`, `NO_COMPOSITE_SUB_NODE` | Cannot be factored inside integer ring ℤ. |
| Q(ops) {+, ·} | `NO_SILENT_TRUNCATION_OVERFLOW`, `NO_ASSOCIATIVITY_VIOLATION` | Overflow MUST force Soft Halt R ≠ 0 into `Q(reuse)` SRAM. |
| Q(logics) {-x, x⁻¹} | `NO_ZERO_DIVISOR_RECIPROCAL`, `NO_NON_INVOLUTIVE_INVERSION` | (-(-x)) and ((x⁻¹)⁻¹) MUST equal x exactly or fire Q = ∅. |

---

## 8. Finite-Range Q(anchors) & The Strict Q(bypass) Boundary Law

> **Governed by:** `[S]` · `[E]`

Single-digit prime seeds initialize two fundamental landmark trees in Genesis Tile T0: `Q(primes)` and `Q(twinPrimes)`, defined strictly over finite bounded metric sets. Symmetric negative mirror anchors exist via Binary Roots of Unity `Q(√1) = {-Q1}`:

> **Isometric Mirror Anchor Law:** `d(Qorigin, +Q_anchor) ≡ d(Qorigin, -Q_anchor)`

**Strict Q(bypass) Law:** short-circuiting in **O(1)** cycles is STRICTLY RESTRICTED to proven algebraic identities (`x + 0 → x`, `x · 1 → x`, `x · 0 → 0`, `x ÷ 1 → x`). Unproven conjectures or heuristic paths MUST NEVER be used as `Q(bypass)` shortcuts.

---

## 9. The Dual-Ring Post-Quantum Cryptography Hardness Conjecture

> **Governed by:** `[S]`

CDQN enforces Post-Quantum Cryptography natively through a 2-Ring architecture built on Physical Ring-LWE polynomial algebra over finite quotient rings `R_q = ℤ_q[X] / (Xⁿ + 1)`.

**The Dual-Ring Q(crypto) Post-Quantum Hardness Conjecture:** we conjecture that, under `Q(reuse)` pattern persistence (which records exact residuals, computed invariant checks, and dimensional metric patterns over finite sets `z`), breaking the multi-node `Qnet(crypto)` composite proof is computationally equivalent to worst-case multi-key Ring-LWE lattice reduction. We state two separate claims: **(a)** the underlying problem is NP-hard (conjecture); **(b)** the concrete attack cost scales exponentially as `2^Ω(k)`. *A formal lemmatic derivation will be established during the Qm formal phase and verified by QnLang.*

### 9.1 Ring 1: Local Node Q(crypto)
Operates 100% locally on device registers without network consensus. The local key uses real physical hardware noise `e_harvest` as the LWE error distribution:

> **Local Key Formula:** `A · s_i + e_harvest,i = b_i (mod q)`

### 9.2 Ring 2: CDQN Network Mesh Qnet(crypto)
High-capacity CDQN server nodes aggregate public commitment polynomials `b_i` from surrounding leaf nodes:

> **`B_net = ∑ b_i = A · (∑ s_i) + ∑ e_harvest,i (mod q)`**

- **Zero-Knowledge Privacy:** remote peers only see composite proof `B_net`; private key `s_i` and physical noise `e_harvest,i` remain 100% private inside local SRAM.
- **Exponential Attack Cost:** breaking `Qnet(crypto)` requires solving a Multi-Key Ring-LWE lattice problem across `k` physically isolated hardware noise sources, scaling as `2^Ω(k)`.
- **O(1) Verification:** any node verifies the composite network proof in O(1) constant time.

### 9.3 Causal Verification Asymmetry Law

> **Governed by:** `[S]`

Valid states are causally gated behind verification; deriving a valid state without provenance is strictly costlier than verifying/generating with it. Enforcement is **layer-dependent**: local and O(1) at the atom floor; remote O(1) at the network layer. Brute force remains structurally slower than key-holding.

### 9.4 BoC Polymorphic Security Scaling Matrix

| Hardware Tier | BoC Cryptographic Mode | Execution Characteristics |
| :--- | :--- | :--- |
| Tier 1: 32-Bit Microcontrollers | 16-Bit Truncated HMAC / Hash-Tag Mode | Executes in 1 clock cycle on register `r0`. Minimal energy. |
| Tier 2: Mobile SoCs / SIMD Edge | Local Physical Ring-LWE (Q(crypto)) | Executes natively on 128-bit SIMD registers using local `e_harvest`. |
| Tier 3: Servers & Clusters | Multi-Key Network Aggregation (Qnet(crypto)) | Trades surplus compute power to aggregate noise and protect the mesh. |

---

## 10. The Qexpr Token Engine & Death of Static File Downloads

> **Governed by:** `[E]` · `[P]`

In the CDQN universe, the concept of static file downloads (.pdf, .mp4, .exe) does not exist.

- **Tokenized Transmission:** network nodes transmit lightweight 2KB Qexpr Logic Tokens instead of megabytes of raw rendered bytes.
- **Local Compute Rendering:** receiving nodes evaluate incoming `Qexpr` tokens locally using their private Tile Zero (T0) seeds and render outputs adapted to their local active Two-Way Zoom window `z`.
- **BoC Adaptive Fallback:** if a weak edge display lacks compute capability to render a complex `Qexpr` token locally, BoC requests a pre-evaluated coarse view (`V_z_macro`) from an adjacent high-capacity SQS node via sublinear O(log N) routing. Pure legacy nodes (no CDQN capability) are served by a Ceiling-Boundary Renderer (Gateway) defined in [`sqs-def.md`](sqs-def.md).

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Documentation portal & navigation map → [`index.md`](index.md)
- Qn Trinity & Metric Anchor → [`qn-def.md`](qn-def.md)
- Qn,64 ABI & Status codes → [`qn-primitives.md`](qn-primitives.md)
- SQS topologies & zero-trust → [`sqs-def.md`](sqs-def.md)
- Canonical term definitions → [`glossary.md`](glossary.md)
