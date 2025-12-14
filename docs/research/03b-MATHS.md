# 03b-MATHS: The Geometric Substrate API

*   **File:** `docs/research/03b-MATHS.md`
*   **Context:** Layer 2 Specification (Geometric Primitives & The Sovereign Uniqueness API)
*   **Date:** December 14, 2025
*   **Status:** `v2.0` (The Discrete Physics Standard)

> **The Abstraction of Form.**
> *This document specifies the Application Programming Interface (API) for Layer 2 of the Loom Virtual Machine. It serves as the bridge between the physical "Metal" (Layer 1) and the dynamic "Physics" (Layer 3). It strictly implements the **MAP Algebra** defined in `02a-MATHS` to enable **Discrete Harmonic Diffusion**. It enforces the laws of Thermodynamics (Fovea Limits) and Linear Logic (Write-to-Buffer) at the function signature level.*

---

## 1. The API Mandate

The functions defined herein are the "Iron Contract" of the LVM kernel. They are designed to be:
1.  **Thermodynamically Bounded:** No operation may exceed the physical capacity of the Fovea (`L1 Cache`).
2.  **Resource Explicit:** All memory usage is caller-controlled (Linear Logic). No hidden allocations.
3.  **Physically Reactive:** Errors are returned as **Plasma**, a specific state of matter representing rejection.

---

## 2. Core Data Structures

### 2.1 The Atom: `SovereignVector`
The fundamental object is the 10,240-bit coordinate.
```rust
#[repr(C, align(64))]
pub struct SovereignVector {
    pub bits: [u64; 160],
}
```

### 2.2 The Immune Response: `Plasma`
Errors in the LVM are not abstract exceptions; they are physical states of rejection.
```rust
#[repr(u8)]
pub enum Plasma {
    Stable = 0,          // Success (Solid/Liquid/Gas state)
    FoveaOverflow = 1,   // Input exceeded physical capacity (Thermodynamic Violation)
    MatroidCollision = 2,// Geometric conflict (Rank Violation)
    EntropicDecay = 3,   // Stream integrity failure
}
```

---

## 3. The Genesis Function

The geometric context is derived from the Sovereign Seed (`02a`).

**Constraint:** The `EntropyStream` is **Single-Threaded**. Sharing a stream across threads violates the Sovereign Uniqueness axiom. Higher layers must instantiate distinct streams for distinct Agents.

```rust
/// Initializes the deterministic stream.
/// @param seed: A 256-bit high-entropy seed.
fn lvm_genesis(seed: [u8; 32]) -> EntropyStream;

/// Generates the next unique atom.
/// @param stream: Mutable reference to the unique stream.
/// @param out: Buffer to write the new vector (No-Allocation Policy).
fn lvm_next_atom(stream: &mut EntropyStream, out: &mut SovereignVector) -> Plasma;
```

---

## 4. The Geometric Instruction Set (The MAP Algebra)

These functions implement the **Discrete Physics** required by `02b-PHYSICS`. To adhere to the "No-Cloning" theorem, all generative functions use a **Write-to-Buffer** pattern.

### 4.1 Binding (Multiplication / Association)
*   **Theory:** `02a` Binding ($\otimes$).
*   **Mechanism:** Bitwise XOR.
*   **Properties:** Invertible, Commutative, Distributive.

```rust
/// Binds two vectors using XOR. Preserves orthogonality.
/// @param a, b: The input reagents.
/// @param out: The buffer for the product.
/// @return Plasma::Stable or error state.
fn lvm_bind(a: &SovereignVector, b: &SovereignVector, out: &mut SovereignVector) -> Plasma;
```

### 4.2 Permutation (Motion / Time)
*   **Theory:** `02a` Permutation ($\Pi$).
*   **Mechanism:** Cyclic Shift (ROR/ROL).
*   **Properties:** Non-Commutative.

```rust
/// Encodes order via cyclic shift.
/// @param v: The vector to move.
/// @param shift: The degree of rotation.
/// @param out: The buffer for the new position.
fn lvm_permute(v: &SovereignVector, shift: i16, out: &mut SovereignVector) -> Plasma;
```

### 4.3 Bundling (Superposition / Diffusion)
*   **Theory:** `02a` Bundling ($+$).
*   **Mechanism:** Component-wise Majority Rule.
*   **Role:** This function is the **Discrete Laplacian Operator**. It averages neighbors without using floats, allowing the system to find the "Geometric Center" of conflicting inputs.

**The Thermodynamic Law:** The "Bundle" operation effectively happens in the system's "Fovea" (L1/L2 Cache). Therefore, the input batch size is physically limited by `LVM_FOVEA_CAPACITY`.

```rust
pub const LVM_FOVEA_CAPACITY: usize = 256; // Defined by L1 Cache lines

/// Bundles a set of vectors via Majority Rule.
/// @param vectors: A slice of reagents.
/// @param count: Number of vectors. MUST be <= LVM_FOVEA_CAPACITY.
/// @param out: Buffer for the result.
/// @return Plasma::FoveaOverflow if count > Capacity.
fn lvm_bundle(vectors: &[SovereignVector], count: usize, out: &mut SovereignVector) -> Plasma;
```

---

## 5. The Matroid Oracle (Information Measurement)

This function measures the **Intrinsic Mass ($m_0$)** defined in `02a`. It serves as the "Scale" for the Physics Engine.

```rust
/// Measures the Matroid Rank (Information Content) of a set.
/// Used to calculate Inertia in Layer 3.
/// @param vectors: The set to measure.
/// @param count: MUST be <= LVM_FOVEA_CAPACITY.
/// @return rank: The calculated rank.
fn lvm_rank(vectors: &[SovereignVector], count: usize) -> Result<u16, Plasma>;
```

---

## 6. Implementation Notes for `libcdqn`

*   **Zero-Copy:** The API is designed so that `out` buffers can be reused, minimizing memory traffic and preventing GC pauses.
*   **Panic-Free:** The kernel must never panic. All deviations return `Plasma`.
*   **Constant Time:** `lvm_bind` and `lvm_permute` must execute in constant time to prevent side-channel attacks on vector content. `lvm_bundle` must be linear time relative to input size $N$, but constant relative to vector values.

---

### Appendix: Test Vector Specification

The correctness of this API will be validated by `docs/validation/03b_maths.check`.

**Example Test Case (`check/majority.test`):**
```
// Test Case: Majority Rule (Bundling).
// 1. GIVEN: 3 Vectors.
//    A: 111000...
//    B: 100111...
//    C: 010101...
// 2. WHEN: Bundle(A, B, C).
// 3. THEN: Result bit 0 is 1 (A=1, B=1, C=0 -> Majority 1).
```

**License:** Universal Sovereign Source License (USSL) v2.0.
