# The CDQN Stack: Chained & Distributed Quang Numbers

> **A Deductive Research Architecture for Structural Complexity, Zero-Drift Arithmetic, and Substrate-Neutral Post-Quantum Systems**

[![License: SSL v1.0](https://img.shields.io/badge/License-Scaling_Source_SSL_v1.0-amber.svg)](LICENSE.md)
[![Documentation: Live Portal](https://img.shields.io/badge/Docs-GitHub_Pages-blue.svg)](https://cdqn5249.github.io/cdqn/)
[![Status: Specification Complete](https://img.shields.io/badge/Status-Specification_Complete-emerald.svg)](#repository-architecture)

---

## Author & Project Metadata

* **Author:** Christophe Duy Quang Nguyen
* **Origin Location:** Bao Loc, Vietnam
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Documentation Portal:** [https://cdqn5249.github.io/cdqn/](https://cdqn5249.github.io/cdqn/)
* **License:** [Scaling Source License (SSL v1.0)](LICENSE.md)

---

## 1. Executive Summary & Epistemology

The **CDQN Stack** introduces a fundamental paradigm shift in computer systems architecture: redefining abstraction from classical **complexity concealment** (opaque black-box structures that induce non-deterministic latency and security vulnerabilities) to **complexity structuring** (*"Quang"* / Luminescent Clarity).

A **Quang Number ($Q_n$)** is a **Capability-Bounded Algebraic Type** instantiated as a canonical 3-element tuple:

$$Q_n = \left\langle S, \; V(\text{Qexpr}), \; M \right\rangle$$

* **Security Anchor ($S = \langle I, \Sigma \rangle$):** Cryptographic identity and intrinsic signed provenance. Unauthenticated payloads evaluate to the Empty Set ($Q_n = \emptyset$).
* **Exact Value Anchor ($V(\text{Qexpr})$):** Homoiconic memory payload holding exact, un-truncated mathematical or semantic expressions (`Qexpr`) without precision loss.
* **Metric Anchor ($M = \langle \text{DomainType}, \mathbf{z}, D, \mathbf{C}, \mathbf{DCC}, \mathbf{P} \rangle$):** Two-Way Zoom window $\mathbf{z}$, universal dimension $D$, chaining topology hook $\mathbf{C}$, self-describing DCC profile, and sparse metric pivots $\mathbf{P}$.

---

## 2. Official Documentation Suite (GitHub Pages)

The CDQN repository includes a complete, interactive, pure HTML5 documentation suite hosted on GitHub Pages:

| Specification Document | Scope & Contents | Direct Link |
| :--- | :--- | :--- |
| **Documentation Portal** | Official project portal hub and architectural pillars overview. | [docs/index.html](https://cdqn5249.github.io/cdqn/index.html) |
| **The SMEMP Manifesto** | Axiomatic laws, non-negotiable Invariants ($S \times M$), Agiles ($E, M_{\text{od}}, P$), and BoC policy engine. | [docs/smemp.html](https://cdqn5249.github.io/cdqn/smemp.html) |
| **The Formal Qn Definition** | Canonical type specification, Trinity Tuple, $Q(\text{reuse})$, Soft Halting, and $Q(\text{bypass})$. | [docs/qn-def.html](https://cdqn5249.github.io/cdqn/qn-def.html) |
| **The Formal SQS Definition** | Substrate-neutral Self-Quang Systems, $Q_{\text{tree}}$ vs. $Q_{\text{graph}}$, and zero-day feedback loops. | [docs/sqs-def.html](https://cdqn5249.github.io/cdqn/sqs-def.html) |
| **Qn Primitives ABI** | Bit-level hardware ABI for universal 64-bit bit-atoms ($Q_{n,64}$), 32-bit register pairs (`r0:r1`), and status flags. | [docs/qn-primitives.html](https://cdqn5249.github.io/cdqn/qn-primitives.html) |
| **Qn Base & Cryptography** | Base-10 seeds ($Q_0 \dots Q_9$), $Q_{\text{zero}}$ vs. $Q_{\text{origin}}$, $Q(\text{harvest})$ Genesis Tile $T_0$, and Dual-Ring PQC Conjecture. | [docs/qn-base.html](https://cdqn5249.github.io/cdqn/qn-base.html) |

---

## 3. The CDQN Master Architecture Matrix

```
+---------------------------------------------------------------------------------------------------+
|                                  THE UNIFIED CDQN ARCHITECTURE MATRIX                             |
+---------------------+----------------------------------+------------------------------------------+
| Architectural Layer | Core Technical Mechanism         | SMEMP & BoC Performance Guarantee        |
+---------------------+----------------------------------+------------------------------------------+
| Level 0:            | Universal Qn,64 Bit-Atom         | 1-cycle execution; 2-register r0:r1 pair |
| Register ABI        | <S(16b), M(16b), V(32b)>         | on 32-bit CPUs; 0% cache padding waste.   |
+---------------------+----------------------------------+------------------------------------------+
| Level 1:            | Base-10 Seeds (Q0..Q9),          | Q(harvest) extracts silicon noise e_harv |
| Digit & Seed Base   | Q_zero vs. Q_origin Distinction  | to seed Aperiodic Genesis Patch T0.      |
+---------------------+----------------------------------+------------------------------------------+
| Level 2:            | Dual-Ring PQC                    | Ring 1: Local physical Ring-LWE.         |
| Security Engine     | Q(crypto) & Qnet(crypto)         | Ring 2: Multi-node NP-Hard noise mesh.   |
+---------------------+----------------------------------+------------------------------------------+
| Level 3:            | Qexpr Token Engine               | Eliminates static files. 99.9% bandwidth |
| Network Data Flow   | & Local Compute Rendering        | reduction via polynomial logic streams.  |
+---------------------+----------------------------------+------------------------------------------+
```

---

## 4. Key Technological Innovations

### A. Universal 64-Bit Physical Bit-Atom ($Q_{n,64}$)
At the silicon register floor, $Q_{n,64}$ is the single universal physical primitive:
* **Bits 0–15:** Security Anchor ($S$) — Cryptographic origin hash and signature proof $\Sigma$.
* **Bits 16–31:** Metric Anchor ($M$) — 2-bit status field, 2-bit domain type, 6-bit $z_{\text{macro}}$, and 6-bit $z_{\text{micro}}$.
* **Bits 32–63:** Value Payload ($V$) — 16:16 $[N, D]$ canonical rational pair or 32-bit `Qexpr` arena offset.
* **Hardware Mappings:** On 32-bit CPUs (ARM Cortex-M, ESP32, RV32I), maps natively to a 2-register pair (`r0:r1`). On wider SIMD registers, scales as multi-lane vector arrays ($2 \times Q_{n,64}$, $4 \times Q_{n,64}$, $8 \times Q_{n,64}$). Exactly 8 $Q_{n,64}$ words fit inside a physical 64-byte L1 cache line with 0% padding waste.

### B. Hardware Entropy Harvesting ($Q(\text{harvest})$) & Genesis Tile Zero ($T_0$)
Upon device onboarding, $Q(\text{harvest})$ extracts physical silicon jitter and thermal noise ($e_{\text{harvest}}$) to bind 4 fundamental target families: $Q(\text{origin})$, $\{Q_0 \dots Q_9\}$, $Q(\text{ops})$, and $Q(\text{logics})$. These atoms populate **Genesis Tile Zero ($T_0$)** in the Aperiodic Einstein Tile memory arena, from which all dynamic application memory inflates self-similarly.

### C. The Dual-Ring Post-Quantum Cryptography Hardness Conjecture
* **Ring 1 (Local Node $Q(\text{crypto})$):** Executes local Ring-LWE ($A \cdot s_i + e_{\text{harvest}, i} = b_i \pmod q$) using real physical hardware noise in 1 clock cycle.
* **Ring 2 (Network Mesh $Q_{\text{net}}(\text{crypto})$):** High-capacity nodes aggregate public commitment polynomials ($B_{\text{net}} = \sum b_i$) without revealing private keys ($s_i$). We conjecture that breaking $Q_{\text{net}}(\text{crypto})$ over $k$ physically isolated hardware noise sources is an NP-Hard problem ($2^{\Omega(k)}$), while verification takes $O(1)$ constant time.

### D. The `Qexpr` Token Engine & Death of Static File Downloads
In the CDQN universe, static file downloads (`.pdf`, `.mp4`, `.exe`) do not exist. Network nodes transmit lightweight 2KB **Quang Expression (`Qexpr`) Logic Tokens**. Receiving nodes evaluate incoming `Qexpr` tokens locally using their private $T_0$ seeds and render visual/semantic outputs adapted to their local Two-Way Zoom window $\mathbf{z}$ and hardware capabilities.

### E. Space-Time Memory Dilation
Respects the physical Memory Wall constraint. Uses Two-Way Zoom ($\mathbf{z}$) for spatial bandwidth compression and transforms the 250-cycle DRAM fetch delay into an active execution window for local $Q(\text{reuse})$ residual folding, cryptographic pre-checks, and $Q(\text{bypass})$ identity short-circuiting.

---

## 5. Licensing & Paternity Terms

This project is licensed under the **Scaling Source License (SSL) — Version 1.0**.

```
                       SCALING SOURCE LICENSE (SSL v1.0)
 ┌───────────────────────────────────────────────────────────────────────────┐
 │ - 100% FREE for researchers, academics, students, open-source projects,  │
 │   and small-medium enterprises (SMEs) under the Scaling Threshold.       │
 │ - Requires explicit Paternity Reference back to the parent repository:   │
 │   https://github.com/cdqn5249/cdqn                                        │
 │ - Anti-Patent Retaliation: Any patent assertion against the author or     │
 │   project automatically revokes all license rights.                       │
 │ - Mandatory Commercial Negotiation required ONLY when crossing the       │
 │   Scaling Threshold ($1M+ Gross Revenue or 10,000+ active nodes).        │
 └───────────────────────────────────────────────────────────────────────────┘
```

For full legal terms, see [LICENSE.md](LICENSE.md).

---

## 6. Paternity & Attribution Reference

Any derivative work, fork, runtime implementation (`QnIR`), or child specification referencing this project MUST preserve the following paternity attribution:

> "Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL v1.0). Parent Repository: https://github.com/cdqn5249/cdqn"

---

*Authored by Christophe Duy Quang Nguyen &bull; Bao Loc, Vietnam &bull; July 2026*
