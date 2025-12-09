# 03a-METAL: The Virtual Substrate

*   **File:** `docs/research/03a-METAL.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 1 Specification (The Hardware Abstraction Layer & Signal Processing)
*   **Date:** December 9, 2025
*   **Status:** `v3.3` (The Hypervisor Standard)

> **The Physics of the Machine.**
> *From the high-level architecture of the Sovereign Loom (Paper 03), we descend to the metal. This paper defines the LVM not merely as software, but as a rigid **Virtual Instruction Set Architecture (vISA)**. It guarantees that the laws of Digital Physics (Papers 02a-02c) are enforced consistently, maximizing the capabilities of Sovereign Hardware while maintaining compatibility with Consumer Devices via a "Hypervisor" strategy.*

---

## 1. Abstract: The Universal Constant

The Loom Virtual Machine (LVM) is designed to be the **"JVM of the Post-AI Era."** Just as the Java Virtual Machine abstracted the CPU to run "Write Once, Run Anywhere" logic, the LVM abstracts the **Entropy** of the underlying system to run **"Compute Once, Trust Anywhere"** physics.

This paper defines the **Layer 1 (Metal)** Specification. It establishes the **SovereignVector** (10,240 bits) as the atomic unit of computation and defines the **Hardware Abstraction Layer (HAL)** that maps high-level "Chemical Reactions" (Valency/Bonding) onto physical silicon. It adopts a "VMware Strategy"—starting as a guest library on consumer devices (`libcdqn`) and evolving into a bare-metal hypervisor (`cdqnOS`) for sovereign infrastructure.

---

## 2. The Physical Atom: `SovereignVector`

The LVM ignores the host's native word size (32/64-bit). It enforces a uniform geometric standard.

### 2.1 The Virtual Specification
*   **Logical Size:** 10,000 bits (The "Canvas" for the Lattice).
*   **Physical Alignment:** 10,240 bits ($160 \times 64$-bit words).
*   **Rationale:** This dimension is the **Least Common Multiple (LCM)** of modern hardware vector lanes. It aligns perfectly with:
    *   **Mobile:** ARM NEON (128-bit) $\times$ 80 cycles.
    *   **Desktop:** AVX2 (256-bit) $\times$ 40 cycles.
    *   **Server:** AVX-512 (512-bit) $\times$ 20 cycles.

### 2.2 The "L1 Streaming" Model
Contrary to standard databases that rely on RAM random access, the LVM operates as a **Streaming Engine**.
*   **The Prefetcher:** The LVM anticipates vector usage based on Chemical Valency (Paper 02c). It prefetches the specific "Active Vectors" (Liquid Phase) into the L1 Cache *before* the operation cycle.
*   **The Guarantee:** While the total dataset lives in RAM (L2/L3), the **Interaction Point** always occurs in L1, ensuring deterministic latency for the critical path.

---

## 3. The Quad-State Physics (The Instruction Set)

The LVM vISA defines four classes of operations, modeled as states of matter. The Execution Engine translates these into the host's native assembly.

| State | Matter | Physics | LVM Opcode | Host Translation (e.g., x86/ARM) |
| :--- | :--- | :--- | :--- | :--- |
| **1** | **Crystal** | **Solid** | `LVM_ACC` | `OR` / `APPEND` (Log updates) |
| **2** | **Liquid** | **Fluid** | `LVM_BIND` | `XOR` / `POPCNT` (VSA Search) |
| **3** | **Gas** | **Diffusion** | `LVM_MOV` | `SHL` / `ROL` (Context Shift) |
| **4** | **Plasma** | **Energy** | `LVM_MASK` | `ANDN` / `PDEP` (Error Isolation) |

### 3.1 The Plasma Reflex (Masking)
Plasma acts as the system's "Immune Response."
*   **Event:** A Matroid Exclusion failure (Collision) or Logic Violation.
*   **Reflex:** The LVM generates a **Plasma Mask** (Bitmask `0` at Index $N$).
*   **Result:** The corrupted bit is electrically insulated. Logic flows around the damage. This prevents the "Null Pointer Exception" crashes typical of legacy systems; the organism survives with "Scar Tissue."

---

## 4. The Runtime Topology: From App to Sovereign

To ensure universal adoption while maintaining sovereign capabilities, the LVM Specification defines two primary modes of operation.

### Tier 1: Library Mode (The Guest)
*   **Target:** Android (APK), Windows, Linux (User Space).
*   **Implementation:** `libcdqn` (Rust).
*   **Physics:** **Logical Consistency.**
*   **Constraint:** Subject to OS scheduling and W^X (Write XOR Execute) security policies on mobile.
*   **Solution:** Uses a **Bytecode Interpreter** or **Ahead-of-Time (AOT)** compilation to comply with mobile store rules while enforcing LVM logic.
*   **Use Case:** Education, Wallets, Gaming (Diablo-class Laptops).

### Tier 2: Sovereign Mode (The Host)
*   **Target:** Raspberry Pi, RISC-V Boards, Robotics, Industrial Servers.
*   **Implementation:** `cdqnOS` (Unikernel / Type-1 Hypervisor).
*   **Physics:** **Temporal Consistency (Zero-Jitter).**
*   **Constraint:** Requires dedicated hardware or virtualization.
*   **Solution:** The LVM *is* the Kernel. It manages the hardware directly, offering real-time guarantees.
*   **Use Case:** Critical Infrastructure, Drone Flight Controllers, Nuclear Safety.

---

## 5. The Tri-Guard Security Protocol

We implement "Defense in Depth" within the Hardware Abstraction Layer (HAL).

| Guard Layer | Mechanism | The Check | Behavior |
| :--- | :--- | :--- | :--- |
| **1. Spatial** | **Virtual Arena** | *"Is this stack safe?"* | **Stack Safety.** A pre-allocated Arena prevents recursion overflows. |
| **2. Logical** | **Provenance Tag** | *"Is this vector tainted?"* | **Taint Tracking.** Headers tag data origin (Network vs. Kernel). |
| **3. Spectral** | **LWE Noise** | *"Is this memory visible?"* | **Obfuscation.** (See 5.1). |

### 5.1 Adaptive Thermodynamics (The Dark Mode)
To balance Security with Usability (Battery Life), the LVM implements **Adaptive Obfuscation**.

*   **Green Mode (Default on Battery):** Unused RAM is zeroed. High efficiency.
*   **Dark Mode (Default on Plugged/Panic):** Unused RAM is filled with **ChaCha20** noise.
    *   **Effect:** To an external observer (Hardware Probe or Host OS), the LVM memory bank is mathematically indistinguishable from Gaussian Noise (Learning With Errors problem).
    *   **User Override:** A user in a hostile environment (e.g., protest) can force **Dark Mode** on battery, trading energy for maximum obfuscation.

---

## 6. The Execution Engine (The Bridge)

The LVM acts as a bridge between the **Chemical Logic** of the user and the **Binary Logic** of the chip.

*   **The Scheduler:** Translates `cdqnLang` interactions ($A \multimap B$) into Vector Operations.
*   **The Hysteresis Loop:** To prevent bus saturation on consumer devices, the engine batches operations.
    *   *Small Batch (< 1k vectors):* Processed on CPU (L1 Cache) for low latency.
    *   *Large Batch (> 1k vectors):* Offloaded to NPU/GPU (Vulkan/Compute) for high throughput.
*   **Result:** The same binary scales seamlessly from a smartphone (using the NPU for efficiency) to a workstation (using the GPU for massive simulation).

---

## 📖 Glossary

*   **Amber Capsule:** A diagnostic snapshot preserving the physics of a crash.
*   **Adaptive Thermodynamics:** The protocol for balancing LWE security with battery life.
*   **Library Mode:** LVM running as a guest application (Logical Consistency).
*   **Sovereign Mode:** LVM running as a Host/Unikernel (Real-Time Consistency).
*   **SovereignVector:** The 10,240-bit atomic unit of the LVM.
*   **Quad-State:** The instruction set mapped to states of matter (Crystal, Liquid, Gas, Plasma).
*   **vISA:** Virtual Instruction Set Architecture.

---

### 📂 Bibliography & References
1.  **Lindholm, T. et al.** (2014). *"The Java Virtual Machine Specification."* (The Virtual Machine model).
2.  **Armstrong, J.** (2007). *"Programming Erlang."* (The BEAM VM and fault tolerance).
3.  **Kanerva, P.** (2009). *"Hyperdimensional Computing."*
4.  **Bernstein, D. J.** *"ChaCha, a variant of Salsa20."*
5.  **Madhavapeddy, A. et al.** (2013). *"Unikernels: Library Operating Systems for the Cloud."* (Sovereign Mode foundation).

---

**Transition:** With the Virtual Substrate defined and the deployment modes clarified, we move to **03b-KERNEL**, where we implement the Rust code that powers this Specification.

**License:** Universal Sovereign Source License (USSL) v2.0.
