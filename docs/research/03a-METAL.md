# 03a-METAL: The Vector Substrate

*   **File:** `docs/research/03a-METAL.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 1 Research (Hardware Physics, Memory Topology, & Signal Processing)
*   **Date:** December 9, 2025
*   **Status:** `Release Candidate v2.1` (Definitive Physics Model)

> **The Physics of the Machine.**
> *From the high-level architecture of the Sovereign Loom (Paper 03), we descend to the metal. This paper defines the atomic unit of the system. At Layer 1, we do not speak of Geometry or Meaning. We speak only of Alignment, Latency, Entropy, and the physical laws that govern the flow of information.*

---

## 1. Abstract

The Loom Virtual Machine (LVM) requires a computational substrate that is **Deterministic** (Zero-Jitter), **Fast** (L1 Cache Resident), and **Opaque** (Post-Quantum Obfuscated). Standard high-level memory models (Heap allocation, Garbage Collection) violate these constraints, creating the "Memory Wall" that makes verifying truth prohibitively expensive.

This paper defines the methodology for **Layer 1 (Metal)**. We propose a fixed-width **10,240-bit Sovereign Vector** as the fundamental data type. We establish a **Quad-State Physics** model (Crystal, Liquid, Gas, Plasma) to handle logic, search, locality, and attention. To secure this substrate, we implement a **Tri-Guard Protocol** (Spatial, Logical, Spectral) that validates data provenance at the silicon level, ensuring the system is immune to both noise flooding and geometric deepfakes.

---

## 2. The Physical Atom: `SovereignVector`

We define the vector not by its content, but by its impact on the CPU Cache and SIMD registers.

### 2.1 The Dimensions
*   **Logical Size:** 10,000 bits (The "Canvas" for the Lattice).
*   **Physical Size:** 10,240 bits ($160 \times 64$-bit words).
*   **Rationale:** 10,240 is the Lowest Common Multiple (LCM-adjacent) for hardware alignment. It divides cleanly by:
    *   **64-bit:** Standard Scalar.
    *   **128-bit:** ARM NEON (Mobile).
    *   **256-bit:** AVX2 / RISC-V (Desktop).
    *   **512-bit:** AVX-512 / SVE (Server).

### 2.2 Memory Alignment & Prefetching
*   **Constraint:** All Vectors must be **64-byte Aligned** to prevent Cache Line Splitting.
*   **Mechanism:** We utilize explicit software prefetching. The Metal Layer anticipates vector usage 50 cycles in advance, hiding the latency of the RAM bus and ensuring the CPU always feeds on L1 Cache.

---

## 3. The Quad-State Physics (The Engine)

The LVM does not just "process" data; it transitions data between four states of matter to optimize for specific tasks.

| State | Matter | Physics | Metal Instruction | Function | Good For |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **1** | **Crystal** | **Solid** | `AND` / `OR` | **Accumulate** | **Deep Logic / Truth.** Building immutable history via Tropical Causality. |
| **2** | **Liquid** | **Fluid** | `XOR` / `POPCNT` | **Compare** | **Search / Similarity.** Implementing **VSA Binding** (XOR) and **Bundling** (Majority). |
| **3** | **Gas** | **Diffusion** | `SHIFT` / `ROT` | **Move** | **Locality.** Cellular automata and local context shifting. |
| **4** | **Plasma** | **Energy** | `MASK` / `PDEP` | **Focus** | **Attention / Reflex.** Surgical isolation of errors without data loss. |

### 3.1 The Plasma Reflex (Masking)
Plasma is the system's nervous system.
*   **Event:** A bit corruption or logic violation is detected at Index $N$.
*   **Reflex:** The system generates a **Plasma Mask** (0 at Index $N$).
*   **Effect:** Subsequent operations apply this mask. The corrupted bit is electrically "insulated." It exists as **Scar Tissue** but does not propagate errors.
*   **Shedding:** At the end of the Epoch (Renormalization), the mask is used as a negative filter. The Scar Tissue is left behind in the dead shell; the new generation is born clean.

---

## 4. Memory Topology: The Fortress

We reject the flat memory model. We implement a **Zonal Topology** to enforce security via physics.

### 4.1 The Arena (Virtual Stack)
*   **Problem:** Recursion blows the OS Stack.
*   **Solution:** The LVM pre-allocates a contiguous **Arena**. Logic execution is strictly **Iterative**. We manage a Virtual Stack Pointer manually, guaranteeing immunity to Stack Overflow exploits.

### 4.2 The Ring Buffer (Crystalline WAL)
*   **Problem:** Disk I/O is too slow for the CPU.
*   **Solution:** A circular buffer in RAM that stores **Deltas** (XOR differences).
    *   **Async Mode:** CPU writes to Ring, continues immediately. (Liquid/Gas operations).
    *   **Sync Mode:** CPU stalls until Ring flushes to Disk. (Crystal/Value operations).

### 4.3 Zonal Segregation
*   **Zone 0 (Kernel):** Trusted internal logic.
*   **Zone 1 (I/O):** Untrusted network/disk buffers.
*   **Zone 2 (User):** `cdqnLang` script execution.

---

## 5. The Tri-Guard Security Protocol

We implement "Defense in Depth" by verifying the source of every vector using three independent physics checks.

| Guard Layer | Mechanism | The Check | When it Happens |
| :--- | :--- | :--- | :--- |
| **1. Spatial** | **Zonal Memory** | *"Is this data in the correct RAM bank?"* | **On Allocation.** Prevents external data from masquerading as Kernel data. |
| **2. Logical** | **Provenance Header** | *"Does the bitwise tag match the Zone?"* | **On Access.** A 4-bit tag in the header confirms origin (CPU/NPU/Net). |
| **3. Spectral** | **Entanglement** | *"Does the geometry match the Key?"* | **On Commit.** A `POPCNT(XOR)` check against the User's Geometric Key. |

### 5.1 LWE Obfuscation
*   **Primitive:** **ChaCha20** (The only external dependency).
*   **Application:** Unused memory is filled with deterministic noise.
*   **Result:** To an observer (OS/Attacker), the Local Lattice is mathematically indistinguishable from Gaussian Noise (Learning With Errors problem).

---

## 6. The Amber Protocol (Diagnostics)

Layer 1 cannot "reason" about errors. It can only preserve the crime scene.

*   **Trigger:** Physical Violation (Collision, Saturation, Zonal Mismatch).
*   **Action:**
    1.  **Freeze:** Copy State + Input + Mask to an **Amber Capsule**.
    2.  **Alert:** Wake the **Layer 4 Entity** (Supervisor).
    3.  **Throttle:** If Error Rate > Threshold, stop logging and enter **Panic Mode** (Drop Packets) to prevent DoS.
*   **Purpose:** Allows the upper layers to perform semantic autopsy without crashing the physics engine.

---

## 7. Hardware Acceleration: The Hybrid Model

We acknowledge the ubiquity of NPUs in consumer devices (Apple M-Series, Snapdragon).

*   **The CPU (Latency Engine):** Handles Crystal/Gas operations. Optimized for branching and bitwise logic.
*   **The NPU (Throughput Engine):** Handles Liquid operations (Murmuration/Search).
*   **The Watchdog:** A hysteresis timer ensures we only offload to NPU if the batch size justifies the bus transfer cost.

---

## 📖 Glossary of Terms

*   **Amber Capsule:** A snapshot of the LVM state at the moment of a crash, used for forensics.
*   **Arena:** A pre-allocated memory block for the Virtual Stack.
*   **Entanglement:** A geometric check that verifies a vector belongs to the local user's lattice.
*   **Plasma Mask:** A bitmask used to isolate corrupted bits (Scar Tissue) without stopping computation.
*   **Quad-State:** The four modes of vector operation: Crystal (Logic), Liquid (Search), Gas (Move), Plasma (Focus).
*   **SovereignVector:** The 10,240-bit atomic unit of the LVM.
*   **Tri-Guard:** The three-layer security check (Zone, Header, Geometry) for data provenance.
*   **VSA (Vector Symbolic Architectures):** A method of representing symbols and logic using high-dimensional vectors (Liquid State).

---

### 📂 Bibliography & References
1.  **Domingos, P.** (2025). *"Tensor Logic: The Language of AI."* (arXiv:2510.12269).
2.  **Zhang, P. et al.** (2020). *"Tropical Tensor Network for Ground States."* (PRL).
3.  **Kanerva, P.** (2009). *"Hyperdimensional Computing: An Introduction."* (VSA Foundation).
4.  **Gayler, R. W.** (2004). *"Vector Symbolic Architectures."* (Binding/Bundling Logic).
5.  **Bernstein, D. J.** *"ChaCha, a variant of Salsa20."*
6.  **Regev, O.** (2009). *"On Lattices, Learning with Errors, Random Linear Codes, and Cryptography."*

---

**Transition:** With the physical substrate (Layer 1) secured and defined, we move to **03b-MATHS**, where we define the **Cascading Logic** that runs upon this metal.

**License:** This document is part of the **CDQN Source Complex** and is governed by the **Universal Sovereign Source License (USSL) v2.0**.
