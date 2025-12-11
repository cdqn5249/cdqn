# 03a-METAL: The Virtual Substrate

*   **File:** `docs/research/03a-METAL.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 1 Specification (Continuum Memory, Hardware Abstraction, & Signal Processing)
*   **Date:** December 11, 2025
*   **Status:** `v3.6` (The Sovereign Substrate)

> **The Physics of the Machine.**
> *From the high-level architecture of the Sovereign Loom (Paper 03), we descend to the metal. This paper defines the LVM not merely as software, but as a rigid **Virtual Instruction Set Architecture (vISA)**. It guarantees that the laws of Digital Physics (Papers 02a-02c) are enforced consistently, utilizing a **Genesis Seed** to generate unique geometry and a **Continuum Memory System** to scale infinitely.*

---

## 1. Abstract: The Nested Learning Module

The Loom Virtual Machine (LVM) is designed to be the **"JVM of the Post-AI Era."** Current operating systems treat "Memory" (RAM) and "Storage" (Disk) as separate physical domains. Following the **Nested Learning** paradigm, the LVM rejects this dichotomy.

We define the machine as a **Continuum Memory System (CMS)**. The **SovereignVector** (10,240 bits) is the atomic unit. The LVM abstracts the underlying silicon to enforce **"Compute Once, Trust Anywhere"** physics.

---

## 2. The Physical Atom: `SovereignVector`

The LVM enforces a uniform geometric standard derived from a sovereign source.

### 2.1 The Genesis Origin
*   **The Seed:** Every vector space is initialized by a **`GenesisSeed`** (Paper 02a).
*   **The Stream:** Vectors are not random; they are slices of a deterministic **ChaCha20 stream** keyed by the Seed.
*   **Implication:** This guarantees that the "Empty Space" of User A is mathematically distinct from User B.

### 2.2 The Virtual Specification
*   **Physical Alignment:** 10,240 bits ($160 \times 64$-bit words).
*   **Rationale:** LCM of 64, 128, 256, 512-bit vector lanes.

---

## 3. The Continuum Memory System (CMS)

We replace the "RAM vs Disk" model with a **Frequency-Based Hierarchy**.

### Level 1: The Fovea (High Frequency $\omega_{high}$)
*   **Physics:** **Liquid Phase**.
*   **Hardware:** L1/L2 Cache & NPU.
*   **Function:** **In-Context Learning.** Immediate reasoning.

### Level 2: The Context (Mid Frequency $\omega_{mid}$)
*   **Physics:** **Gas Phase**.
*   **Hardware:** System RAM (Encrypted).
*   **Blind Indexing:** Uses unencrypted **Perceptual Hashes** for search, while data remains encrypted blobs.

### Level 3: The Deep Lattice (Low Frequency $\omega_{low}$)
*   **Physics:** **Crystal Phase**.
*   **Hardware:** NVMe SSD / Disk-Resident Graph.
*   **Function:** **Long-Term Knowledge.**
    *   **Data Structure:** Vectors + **Thermal Metadata**.
    *   **Thermal Storage:** We store the **Temperature ($T$)** (Stability) and **Mass ($m$)** (Weight) of every vector. This allows the system to prioritize retrieval based on "Truth" (Low Temp) vs "Noise" (High Temp).
    *   **Scalability:** Uses Disk-Resident Indexing (e.g., DiskANN).

---

## 4. The Quad-State Physics (The Instruction Set)

The LVM vISA defines four classes of operations.

| State | Matter | Physics | LVM Opcode | Function |
| :--- | :--- | :--- | :--- | :--- |
| **1** | **Crystal** | **Solid** | `LVM_ACC` | **Consolidate.** (Accumulate Mass). |
| **2** | **Liquid** | **Fluid** | `LVM_BIND` | **React.** (XOR / Binding). |
| **3** | **Gas** | **Diffusion** | `LVM_MOV` | **Contextualize.** (Shift). |
| **4** | **Plasma** | **Energy** | `LVM_MASK` | **Isolate.** (Error Masking). |

---

## 5. The Runtime Topology: From App to Sovereign

### Tier 1: Library Mode (The Guest)
*   **Target:** Android (APK), Windows, Linux.
*   **Implementation:** `libcdqn` (Rust).
*   **Constraint:** Subject to OS scheduling. Uses **Async Entropy Workers** to pay the "Security Tax" during I/O waits.

### Tier 2: Sovereign Mode (The Host)
*   **Target:** Bare Metal, Robotics.
*   **Implementation:** `cdqnOS` (Unikernel).
*   **Physics:** Real-Time Consistency.

---

## 6. The Tri-Guard Security Protocol

1.  **Spatial Guard:** Virtual Arena (Stack Safety).
2.  **Logical Guard:** Provenance Tags.
3.  **Spectral Guard:** **Adaptive Thermodynamics**.
    *   *Green Mode:* Zeroed RAM (Battery Saver).
    *   *Dark Mode:* ChaCha20 Noise (Sovereignty).

---

## 7. The Execution Engine (The Bridge)

*   **The Scheduler:** Translates `cdqnLang` decks into LVM Bytecode.
*   **The Hysteresis Loop:** Dynamically routes batches to CPU or GPU based on the "PCIe Cliff" benchmark.

---

### 📂 Bibliography & References
1.  **Behrouz, A. et al.** (2025). *"Nested Learning."* Google Research.
2.  **Martín-Olalla, J.M.** (2025). *"Thermal stability."* (Thermodynamic basis of memory).
3.  **Jayaram, S. et al.** (2019). *"DiskANN."*
4.  **Bernstein, D. J.** *"ChaCha."*

---

**License:** Universal Sovereign Source License (USSL) v2.0.
