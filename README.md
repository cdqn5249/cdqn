# Chained and Distributed Quang Numbers (CDQN)

At the heart of this repository lies a fundamental, interdisciplinary inquiry: 

**"Can numbers abstract anything?"** 

The cdqn project proposes a structural hypothesis to explore this question: that a complete, next-generation operating system stack can be modeled and executed purely via the mathematical primitives of the **Quang numbers ($Qn$)**. 

By theoretically redefining "numbers" from classical flat decimals to condensed topological sheaves over profinite spaces, we seek to investigate whether we can govern computation, memory, security, and human accountability through the category-theoretic laws of sheaves. 

This project is an active, open-ended research exploration. We have not "proven" a working system; we have generated a highly cohesive **Formal Design Hypothesis**. We are publishing these initial papers to the public commons to **ask for help from the global scientific, mathematical, and systems-engineering communities**. We invite you to audit our derivations, stress-test our logic, find our errors and vulnerabilities, and help us discover whether this topological model can be realized on physical, finite silicon registers.

---

## 1. The Proposed Full-Stack Hypothesis

The cdqn ecosystem is designed as a speculative, vertically integrated compilation pipeline:

$$Qn \text{ Primitives} \to QnIR \to QnLang \to cdqnK \to cdqnOS$$

The documentation of this full-stack architecture is organized sequentially across dedicated paper series located in the [docs/](docs/) directory:

### [Series 01: Quang Number Primitives (CDQN-01)](docs/)
This series formalizes the abstract mathematical definitions, valuations, metrics, and axiomatic contracts of the proposed number system.
*   **Mathematical Bedrock:** Condensed Abelian Groups ($\mathbf{CondAb}$), profinite truncation, and $p$-adic ultrametric valuation fields.
*   **Key Documents:** [docs/01.1.md](docs/01.1.md) through [docs/01.15.md](docs/01.15.md). Refer to [docs/01.12.md](docs/01.12.md) for the consolidated specifications contract.

### [Series 02: Quang Number Intermediate Representation (CDQN-02 - QnIR)](docs/)
This series specifies the target-independent, immutable virtual Instruction Set Architecture (vISA) and compiled dataflow mappings.
*   **SMC Bytecode & Targets:** Free strict Symmetric Monoidal Categories (SMC), Single-Writer Wires, table-driven branchless dispatch, $p$-adic tree-structured slab allocators, and GPU-accelerated SPIR-V compute shaders.
*   **Key Documents:** [docs/02.01.md](docs/02.01.md) through [docs/02.14.md](docs/02.14.md). Refer to [docs/02.14.md](docs/02.14.md) for the consolidated stable bytecode contract.

### [Series 03: The Sovereign High-Level Language (CDQN-03 - QnLang)](docs/)
This series specifies the high-level compiler syntax, substructural type-checkers, and pregroup grammatical parsing engines.
*   **Linguistic & Security Lattices:** Linear/affine types, Symmetric Monoidal Stack Invariants, row-based effect capability rows (`@`), Information-Flow Control (IFC) labels (`#`), and the non-biometric Human Accountability Anchor ($\mathcal{A}_{\text{human}}$).
*   **Key Documents:** [docs/03.01.md](docs/03.01.md) through [docs/03.06a.md](docs/03.06a.md).

### [Series 04: Quang Mathematics (CDQN-04 - Qm)](docs/)
This series specifies the constructive, computable mathematical behaviors, axioms, and continuous thermodynamic scaling models of the Quang numbers ($Qn$).
*   **Core Research Scope:** The optimization of bi-directional translation pathways between human-scale decimal math ($\mathbb{R}_{10}$) and register-scale binary math ($\mathbb{F}_2$), constrained strictly to the domain of Computable Quang Numbers ($Qn_{\text{comp}}$).
*   **Key Documents:** Commencing with `docs/04.01.md` (The Foundations of Quang Mathematics - Qm), which axiomatizes the Substrate, Abstraction, and AI Context Gaps (Knowledge and Action) and formulates the Thermodynamic Value-of-Information (VoI) Inequality to prevent physical microarchitectural thermal cycling fatigue under Dynamic Voltage and Frequency Scaling (DVFS).

### [Series 05: Quang Semantics (CDQN-05 - Qs)](docs/)
This series specifies the semantics of $Qn$ based on contexts and domains of applications.
*   **Key Documents:** Commencing with `CDQN-05.01` (to be drafted in subsequent co-design phases).

### cdqnK & cdqnOS (Future Series)
The low-level microkernel drivers, physical memory allocators, and hardware-software operating system interfaces are deferred and assigned to dedicated, separate paper series in subsequent developmental phases to preserve complete modularity and prevent premature microkernel technical debt.

---

## 2. The Collaborative Invitation: How to Contribute

This repository is designed under the **Variation-Evaluation-Selection (VES)** lifecycle. We treat every paper as a speculative "Variation" that requires adversarial auditing. We ask the community to help us evaluate these models:

1.  **Break Our Mathematics:** If you identify a category-theoretic inconsistency in our condensed sets ($\mathbf{CondAb}$), a homological gap in our spectral sequences ($E_r$), or an algebraic error in our $p$-adic valuations, please open an Issue or submit a Critique.
2.  **Audit Our Security:** If you find a side-channel vulnerability in our $O(1)$ constant-time register deallocation, a collusion exploit in our Direct Anonymous Attestation (DAA), or a logical loophole in our compile-time smart contract validation, help us document and correct it.
3.  **Refine Our Complexity:** Help us optimize our algorithmic complexity bounds for Fast GCD and cellular sheaf cohomology, or propose more efficient physical register mapping protocols.

Your feedback, corrections, and push-backs will be recorded in our VES Revision History tables, driving the collaborative evolution of the cdqn ecosystem.

---

## 3. Licensing and Covenant (USSL v1.0)

To protect the collaborative contributions of the community and prevent corporate or state enclosure of these open-core specifications, this repository is governed by the **Universal Sovereign Source License (USSL) v1.0**, authored by Christophe Duy Quang Nguyen.

### Core Invariants:
1.  **Sovereign Exemption:** Use of the Source Complex is completely free and unrestricted for Personal Use, Academic Research, Non-Profit Education, or Small-Scale Individual Creation.
2.  **The Iron Shield (Section 4):** If any licensee instigates patent, copyright, or trade-secret litigation against the Author(s), the Community, or any sovereign node, all rights and licenses granted to that licensee terminate immediately and retroactively. This work is the un-enclosable physical floor of its field.
3.  **Open Core Requirement (Section 5):** The fundamental logic and formalisms within the Source Complex must remain transparent and auditable in any derivative work.
4.  **Commercial Thresholds (Section 6):** Deployment by government entities, corporations with $>500$ employees, projects servicing $>10,000$ active users, or generating $> \$1,000,000$ USD in gross annual revenue requires entering a formal Commercial Partnership Agreement with the primary author.

*All inquiries regarding commercial deployment or partnerships should be directed to the primary author, Christophe Duy Quang Nguyen, under the jurisdiction of Vietnam / Open International Arbitration.*
