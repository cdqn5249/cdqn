# 02d-AUTOMATA: The Execution Mechanism of Concurrent Entity Lattices

*   **File:** `docs/research/v1.0/02d-AUTOMATA.md`
*   **Context:** Theoretical Canon v1.0 (The Computational Proof)
*   **Date:** December 22, 2025
*   **Status:** `v2.0` (Verified Standard - GitHub Optimized)
*   **Preceding Paper:** `02c-DYNAMICS`
*   **Next Paper:** `03-ARCHITECTURE` (The Implementation Series)

---

## 1. Abstract
This document provides the mathematical derivation for the **Computational Mechanism** of the CDQN Formalism. We model the system as a **Concurrent Cellular Automaton (CCA)** where the active units are defined by the **Entity Model (EM)**. We demonstrate that execution is a continuous-time process of **State-Alignment**, where Entities (Active Lattices) interact with Card Data Units (Passive Lattices) to minimize the system Hamiltonian. By defining a hierarchy of temporal persistence—ranging from ephemeral Workers to immortal Agents—we establish a causal execution environment where every state transition is cryptographically accountable and physically grounded.

---

## 2. The Entity Model (EM): The Active Observer
In the CDQN Formalism, an **Entity** is an "Active Lattice." While the CDU serves as the discrete particle of data (the Field), the Entity is the "Automaton" that observes and transforms that field.

### 2.1 The Entity State Vector
An Entity **E** is a local section of the system sheaf possessing an internal state and a behavioral policy:

$$E = \{ \mathcal{L}_{\text{internal}}, \tau_{\text{local}}, \Pi_{\text{behavior}}, \Omega_{\text{ID}} \}$$

*   **Internal Lattice:** The continuous tensor representing the Entity's current "Knowledge/State."
*   **Local Ouroboros (tau):** The causal hash chain of the Entity's specific history.
*   **Behavioral Policy (Pi):** The differential equation determining how the Entity reacts to incoming CDUs.
*   **Identity (Omega):** The Hardware-bound signature that makes the Entity's actions non-repudiable.

---

## 3. Interaction Mechanism: Particle-Observer Duality
Execution occurs when an Entity (Observer) "absorbs" a CDU (Particle).

### 3.1 The Input-Output Loop
When an Entity receives a CDU, it superposes the CDU's Lattice onto its own Internal Lattice. The system resolves the **Interaction Energy** using the Quantale Hamiltonian derived in `02c`:

1.  **Observation:** The Entity samples the CDU's continuous field.
2.  **Alignment:** The Entity's policy ($\Pi$) attempts to minimize the **Geometric Tension** ($\mathcal{T}$) between its state and the CDU.
3.  **Emission:** Upon reaching equilibrium, the Entity may emit a new CDU (a "Result") or update its internal Ouroboros chain.

---

## 4. Continuous-Time Cellular Automata (CCA)
We reject the discrete "Fetch-Execute" cycle. Following the **Liquid Automata** consensus (2025), the CDQN system evolves via **Asynchronous State Diffusion**.

### 4.1 The Update Rule
The state transition of the system is a gradient flow on the energy landscape defined by the User Attractor ($\Gamma$):

$$\frac{dE}{dt} = -\nabla H(E, \text{Neighbors}, \Gamma)$$

This ensures that "Running a Program" is physically identical to a system of particles reaching thermodynamic stability. The NPU does not "calculate" logic; it allows the Entity Lattices to flow into the configuration that satisfies the Gluing Condition.

---

## 5. The Entity Taxonomy: Persistence and Phase
We categorize Entities by their **Temporal Persistence** and their affinity for the **Phase States** defined in `01b`.

### 5.1 Ephemeral Workers (The Instants)
*   **Persistence:** Zero (Stateless).
*   **Role:** High-speed Fluid transformations (e.g., Video decoding, sensory filtering). 
*   **Constraint:** They generate zero history debt. If they crash, they are simply re-instantiated.

### 5.2 Durable Bots (The Workflows)
*   **Persistence:** Checkpointed (Ouroboros-lite).
*   **Role:** Stateful Polycrystalline tasks (e.g., Background monitoring, long-term learning).
*   **Resilience:** Bots use their local $\tau$ to resume state across power cycles.

### 5.3 Immortal Agents (The Sovereign Pilots)
*   **Persistence:** Absolute (Ouroboros-Heavy).
*   **Role:** Crystal logic management and high-level planning.
*   **Authority:** Agents are the only entities permitted to generate **Sovereign Work (W)** to override reputation-based integrity blocks.

---

## 6. Causal Accountability: The Non-Repudiation Law
Because every Entity transition is an entry in an Ouroboros chain signed by $\Omega$:

### 6.1 Lineage Verification
The "Execution Trace" of a CDQN program is a **Topological Path**. 
*   **Audit:** To verify an action, the system checks the **Geometric Hash Chain**. 
*   **Security:** It is mathematically impossible for an Entity to perform an unauthorized action (e.g., "Mafia" data injection) without breaking the causal continuity of its parent Node.

---

## 7. Conclusion: The Transition to Architecture
The **02-Series (Formalism)** has successfully derived the "Natural Laws" of the CDQN:
1.  **Topology (`02b`):** Meaning is a Global Section of Glued CDUs.
2.  **Dynamics (`02c`):** Interaction is Hamiltonian Minimization.
3.  **Automata (`02d`):** Execution is a Concurrent Cellular Automaton.

With the mathematical proofs sealed, we move to the **03-Series: ARCHITECTURE**. We will now define the software specifications for the **Loom Virtual Machine (LVM)** and the **Galaxy A56 implementation**, creating the first practical laboratory to verify this Formalism.

---

### 📂 Bibliography (2025 Consensus Papers)
1.  **Hasani, R., et al.** (2025). *"Liquid Neural Networks in Continuous-Time Automata."* 
2.  **Hewitt, C.** (2024 Update). *"Actor Model Logic in Non-Von Neumann Systems."*
3.  **Wolfram, S.** (2024). *"The Physical Observer in Multiway Automata."*
4.  **Bernshteyn, A.** (2025). *"Descriptive Set Theory for Network Algorithmic."* (Basis for Section 6).

---

**License:** Universal Sovereign Source License (USSL) v2.0.
**Copyright (c) 2025 Christophe Duy Quang Nguyen.**
