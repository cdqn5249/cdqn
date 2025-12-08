# 03a-METAL: The Vector Substrate

*   **File:** `docs/research/03a-METAL.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 1 Research (Hardware Physics & Memory Topology)
*   **Date:** December 6, 2025
*   **Status:** `Research Draft v1.3` (Security & Stack Hardening)

> **The Physics of the Machine.**
> *From the high-level architecture of the Sovereign Loom (Paper 03), we descend to the metal. This paper defines the atomic unit of the system. At Layer 1, we do not speak of Geometry or Meaning. We speak only of Alignment, Latency, and Entropy.*

---

## 1. Abstract

The Loom Virtual Machine (LVM) requires a computational substrate that is **Deterministic** (Zero-Jitter), **Fast** (L1 Cache Resident), and **Opaque** (Post-Quantum Obfuscated). Standard high-level memory models (Heap allocation, Garbage Collection) violate these constraints, creating the "Memory Wall" that makes verifying truth prohibitively expensive.

This paper defines the methodology for **Layer 1 (Metal)**. We propose a fixed-width **10,240-bit Sovereign Vector** as the fundamental data type. This layer provides the physical storage and raw bitwise operations required to support **Tensor Logic** and **Lattice-Based Cryptography**. We further define a **Zero-Dependency** ethos, with a singular exception for cryptographic primitives (`chacha20`), and mandate an **Iterative Virtual Stack** to prevent memory overflows on embedded hardware.

---

## 2. The Physical Atom: `SovereignVector`

We define the vector not by its content, but by its impact on the CPU Cache.

### 2.1 The Dimensions
*   **Logical Size:** 10,000 bits (The "Canvas" for the Lattice).
*   **Physical Size:** 10,240 bits ($160 \times 64$-bit words).
*   **Rationale:** 10,240 is evenly divisible by 128, 256, and 512. This ensures universal alignment across:
    *   **ARM NEON** (128-bit) – *Standard Consumer Mobile*.
    *   **AVX2** (256-bit) – *Standard Consumer Desktop*.
    *   **AVX-512 / SVE** (512-bit) – *High-Performance Workstation*.

### 2.2 Memory Alignment (The L1 Constraint)
*   **The Problem:** Misaligned data causes "Cache Line Splitting," doubling memory access time and introducing jitter.
*   **The Constraint:** All Vectors must be **64-byte Aligned**.
*   **Implementation Strategy:** We utilize the Rust `#[repr(align(64))]` directive.
*   **Outcome:** A Vector fits exactly into ~20 Cache Lines (assuming 64-byte lines). Loading a Vector is a deterministic burst operation, immune to False Sharing.

---

## 3. The Instruction Set (The Mechanism)

Layer 1 provides the raw capabilities. It does not enforce rules; it executes physics.

### 3.1 Primitive Operations (SIMD)
We restrict the engine to operations that map 1:1 to CPU instructions.
1.  **`XOR` / `AND` / `OR`:** The fundamental interactions. ($< 1$ cycle throughput).
2.  **`POPCNT` (Population Count):** Calculating the "Weight" of a vector. (Essential for Layer 2 Matroid Rank).
3.  **`ROTR` / `ROTL` (Rotate):** Cyclic shifts for context changes. (Essential for Layer 5 Fluidity).

### 3.2 The NPU Hypothesis (Tensor Acceleration)
*   **Observation:** Modern consumer SoCs (Apple Silicon, Snapdragon) prioritize **Neural Processing Units (NPUs)**.
*   **Hybrid Strategy:**
    *   **The Latency Path (CPU):** Single-Card operations (Logic, Permissions) execute on the CPU for sub-microsecond response.
    *   **The Throughput Path (NPU):** "Deck" operations (Batch Binding, Murmuration Statistical Analysis) are offloaded to the NPU via optional plugins.
    *   **Hysteresis Control:** The Kernel must only offload if Batch Size > 1,000 Vectors to satisfy the bus transfer latency cost.

---

## 4. Security as a Physical Property

At Layer 1, security is defined by **Indistinguishability**. To an observer, a vector must look like static.

### 4.1 The Security Exception (`chacha20`)
While the LVM follows a "Zero Dependency" ethos, **Cryptography** is the exception. "Rolling your own crypto" is a fatal error.
*   **The Primitive:** We utilize the **ChaCha20** stream cipher (via a `no_std` compatible crate).
*   **The Application:** We treat the `SovereignVector` memory as a mutable buffer. ChaCha20 fills or permutes this buffer.
*   **The Result:** High-speed, SIMD-optimized, cryptographically secure randomness (CSPRNG).

### 4.2 Lattice Obfuscation (LWE)
*   **Mechanism:** Unused or "Cold" vectors are initialized with ChaCha20 noise.
*   **Outcome:** Without the Basis Vectors (held in Layer 2), the data on disk creates a **Learning With Errors (LWE)** problem. It is mathematically hard to distinguish the "Signal" from the "Noise," providing Post-Quantum resistance and plausible deniability.

---

## 5. Memory Topology: The Virtual Stack

Standard Recursion is dangerous on bare metal (Stack Overflow risk).

### 5.1 The Iterative Mandate
*   **Risk:** A `SovereignVector` is 1.25 KB. A recursion depth of ~1,000 frames blows the standard OS Stack (2MB).
*   **The Solution:** **The Arena.**
    *   The LVM pre-allocates a large contiguous block of memory (The Arena) at startup.
    *   It manages a **Virtual Stack Pointer** manually.
    *   Logic execution is strictly **Iterative** (Loops), never Recursive.
*   **Benefit:** Total immunity to `SIGSEGV` (Stack Overflow) and predictable memory footprint.

---

## 6. Methodology for Validation

We propose the following benchmarks to validate Layer 1.

1.  **The "Metal" Benchmark:** Throughput > 1 Billion Ops/s on single-core ARM M3/Ryzen.
2.  **The "Jitter" Benchmark:** Execution variance $\sigma < 10$ nanoseconds (proving L1 residency).
3.  **The "Entropy" Benchmark:** Dieharder/NIST statistical tests on vector dumps to prove obfuscation.

---

## 📖 Glossary of Terms

*   **Arena:** A pre-allocated block of contiguous memory used to manage the Virtual Stack, preventing heap fragmentation and stack overflows.
*   **ChaCha20:** A high-speed, secure stream cipher used as the CSPRNG for the LVM.
*   **L1 Cache:** The fastest memory on a CPU. `SovereignVectors` are sized to fit here.
*   **LWE (Learning With Errors):** A lattice-based cryptographic problem that secures data by burying it in noise.
*   **SIMD (Single Instruction, Multiple Data):** CPU instructions (AVX, NEON) that process 512 bits at once.
*   **Virtual Stack:** A manual stack managed by the LVM to allow deep logic execution without crashing the OS stack.

---

### 📂 Bibliography & References
1.  **Bernstein, D. J.** *"ChaCha, a variant of Salsa20."* (The crypto primitive).
2.  **Regev, O.** (2009). *"On Lattices, Learning with Errors, Random Linear Codes, and Cryptography."*
3.  **Intel Corp.** (2024). *"Intel 64 and IA-32 Architectures Optimization Reference Manual."*
4.  **Apple Inc.** *"Accelerate Framework & BNNS (Basic Neural Network Subroutines)."* (NPU Integration).

---

**Transition:** With the physical substrate defined, we move to **03b-MATHS**, where we define the Geometric Laws (Matroids & Quantales) that govern these vectors.

**License:** This document is part of the **CDQN Source Complex** and is governed by the **Universal Sovereign Source License (USSL) v2.0**.
