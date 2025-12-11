# 03c-MATHS: The Geometric Substrate API

*   **File:** `docs/research/03c-MATHS.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 2 Specification (Geometric Primitives & The Sovereign Uniqueness API)
*   **Date:** December 11, 2025
*   **Status:** `v1.1 - Validated Specification`

> **The Abstraction of Form.**
> *This document specifies the Application Programming Interface (API) for Layer 2 of the Loom Virtual Machine. It serves as the bridge between the physical "Metal" (Layer 1) and the logical "Physics" (Layer 3). Following the confrontation with the Digital Physics canon, this API is strictly "Resource-Aware." It enforces the laws of Thermodynamics (Fovea Limits) and Linear Logic (Write-to-Buffer) at the function signature level.*

---

## 1. The API Mandate

The functions defined herein are the "Iron Contract" of the LVM kernel. They are designed to be:
1.  **Thermodynamically Bounded:** No operation may exceed the physical capacity of the Fovea.
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

The geometric context is derived from the Sovereign Seed.

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

## 4. The Geometric Instruction Set (The Physics Engine)

These functions implement the **Quantale Economy**. To adhere to the "No-Cloning" theorem and prevent hidden resource exhaustion, all generative functions use a **Write-to-Buffer** pattern.

### 4.1 Binding (Multiplication)
```rust
/// Binds two vectors using XOR.
/// @param a, b: The input reagents.
/// @param out: The buffer for the product.
/// @return Plasma::Stable or error state.
fn lvm_bind(a: &SovereignVector, b: &SovereignVector, out: &mut SovereignVector) -> Plasma;
```

### 4.2 Permutation (Motion)
```rust
/// Encodes order via cyclic shift.
/// @param v: The vector to move.
/// @param shift: The degree of rotation.
/// @param out: The buffer for the new position.
fn lvm_permute(v: &SovereignVector, shift: i16, out: &mut SovereignVector) -> Plasma;
```

### 4.3 Bundling (Set Formation) & The Fovea Limit
**The Thermodynamic Law:** The "Bundle" operation is a superposition of inputs. It effectively happens in the system's "Fovea" (L1/L2 Cache). Therefore, the input batch size is physically limited by the `LVM_FOVEA_CAPACITY`.

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

This function measures the information content (Rank) of a set. Like bundling, it is constrained by the Fovea.

```rust
/// Measures the Matroid Rank (Information Content) of a set.
/// @param vectors: The set to measure.
/// @param count: MUST be <= LVM_FOVEA_CAPACITY.
/// @return rank: The calculated rank (via out pointer or tuple).
fn lvm_rank(vectors: &[SovereignVector], count: usize) -> Result<u16, Plasma>;
```

---

## 6. Implementation Notes for `libcdqn`

*   **Zero-Copy:** The API is designed so that `out` buffers can be reused, minimizing memory traffic.
*   **Panic-Free:** The kernel must never panic. All deviations return `Plasma`.
*   **Aliasing:** The implementation must handle pointer aliasing (e.g., if `a` and `out` are the same address) safely, or explicitly mark it `unsafe`.
