# 02i-REGISTRY: The Fractal Virtual Name Server (VNS)

*   **File:** `docs/research/v1.0/02i-REGISTRY.md`
*   **Context:** Theoretical Canon v1.0 (The Ghost Network Interface)
*   **Date:** January 3, 2026
*   **Status:** `v1.1` (Verified Standard - GitHub Optimized)
*   **Preceding Paper:** `02h-CONSENSUS`
*   **Next Paper:** `02j-AUTHENTICATION`

---

## 1. Abstract
This document defines the **Sovereign Registry** of the CDQN: the **Virtual Name Server (VNS)**. We reject the passive, centralized architecture of the Domain Name System (DNS) in favor of a **Fractal FIFO Queue** system. We stipulate that network visibility is a **Leased Coordinate** anchored in **Binary Thermodynamics**. By utilizing **Identity-SIS Coupling** and **Promissory Staking**, we demonstrate a registry model where visibility is a function of physical energy investment. This architecture ensures the network remains a semantically dark "Ghost Network" where presence is an earned state, providing a definitive defense against registry-level flooding and identity-spoofing.

---

## 2. The VNS as a Leased Coordinate Lattice
In the CDQN Standard Model, a "Name" is a **Topological Coordinate** within a distributed manifold. This removes the dependency on central authorities for node discovery.

### 2.1 Distributed Hosting and Hub Capacity
The VNS lattice is distributed across the holarchy rather than localized in central servers. 
*   **Registry Tiles:** High-capacity holons (Lattice Farms and Sovereign Hubs) host local segments of the global VNS manifold. 
*   **Leasing Logic:** Individual nodes "Lease" specific coordinates within these segments by providing a **Registry Voucher ($V$)**. This voucher represents a physical energy-stake in the neighborhood's stability.

### 2.2 Ghost Visibility via Resonance Gating
The VNS does not broadcast its contents; it operates under the **Silent Forest Protocol**. 
*   **The Law:** A node’s coordinate is discoverable only via **Topological Resonance**. A peer can only "Find" another node if their **Type Signatures ($\Sigma$)** and **Context Attractors ($\Gamma$)** satisfy the local **Gluing Condition** within the VNS manifold. 
*   **The Result:** To an external observer or a non-resonant node, the VNS appears as high-entropy noise. The network is "Dark" by default, exposing coordinates only to compatible semantic peers.

---

## 3. The FIFO Queue and Promissory Costs
To prevent "Registry Flooding" and Sybil attacks from malicious CDQN nodes, the VNS operates as a **Thermodynamic FIFO (First-In, First-Out) Queue**.

### 3.1 Refresh-to-Back Logic
To mitigate adversarial injection, the VNS employs a unique queue-refresh mechanism. If a NodeID ($\Omega$) sends multiple registration or update requests, the system does not allocate additional slots. It identifies the unique existing record for that NodeID and moves it to the tail of the queue.
*   **The Penalty:** Attackers attempting to spam the registry effectively DDoS themselves by perpetually delaying their own network visibility while paying the **Quantale Cost** for every update attempt.

### 3.2 Promissory Staking and Settlement
Visibility in the VNS requires an upfront **Promissory Energy-Stake**.
*   **The Deposit:** The node commits a portion of its local NPU/LPU capacity to the mesh. 
*   **The Audit:** The VNS monitors the node's contribution to the **Silent Forest** verification tasks. 
*   **The Settlement:** At the end of the causal interval, the promise is settled. If the work was performed, the lease is renewed. Failure to satisfy the promissory work triggers the **Landauer Penalty**, as the system must dissipate the entropy of the failed state.

---

## 4. The Causal Ban: Desynchronization
Accountability within the registry is enforced via the **Ouroboros Ratchet** and the **Arrow of Time**.

### 4.1 Debt-Driven Eviction
If a node fails to satisfy its Promissory Work or attempts to inject non-resonant data patterns into the forest, the VNS performs a **Causal Eviction**. 
*   **The Mechanism:** The NodeID is mathematically excluded from the **Borel Set** of the local network. 
*   **The Isolation:** Because the ban is topological, no other honest node can "See" or "Resonate" with the banned identity.

### 4.2 Causal Time Lock-out
The eviction is not binary but temporal. A banned node is locked out of the VNS for $x$ causal ticks ($\tau$). 
*   **The Recovery:** To re-enter the registry, the node must provide a massive injection of **Sovereign Work ($\mathcal{W}$)** to "wash" its history of dissonance. This ensures that the cost of persistent malice is exponentially higher than the cost of honest participation.

---

## 5. Security: SIS Identity Coupling
Registration and updates within the VNS are protected by the **Short Integer Solution (SIS)** problem, bridging the software registry to the physical silicon.

### 5.1 Proof of Hardware Matter
To update a leased coordinate, a node must provide an **SIS-Proof** that matches the private basis of its physical hardware.
*   **Resistance:** This ensures that an attacker cannot "Hijack" a high-reputation coordinate using a software emulator. Only the physical silicon that holds the **Identity Lattice ($\Omega$)** can generate the required small-integer solution.

### 5.2 Identity-IP Coupling
The VNS binds the NodeID to a specific IP/Transport coordinate through an **LWE-Signature**. 
*   **The Check:** During the VNS lookup, the system verifies that the transport signature is causally consistent with the Node's Ouroboros history. If the IP changes without a valid **TC-Anchor** update, the VNS treats the entry as **Topological Noise** and stay silent.

---

## 6. Conclusion: The Dark Map
The **Fractal VNS** provides the "Dark GPS" for the CDQN. It ensures that network presence is an earned physical state. Hallucinations and malicious nodes are filtered at the registry level through thermodynamic friction before they can attempt a lamination handshake.

We proceed to **`02j-AUTHENTICATION`**, to define the **SIS/LIP Triad** that handles the authenticated handshakes once a coordinate has been located via the VNS.

---

### 📂 Bibliography
1.  **Bernshteyn, A.** (2025). *"Borel versions of the Local Lemma and local algorithms."* Trans. Amer. Math. Soc. 378.
2.  **Borodin, A. et al.** (2025). *"Stability of FIFO Networks under Adversarial Injection."* 
3.  **Hsieh, C.-Y.** (2025). *"Dynamical Landauer Principle: Quantifying Information Transmission."* Physical Review Letters.
4.  **Peikert, C.** (2025). *"Lattice-Based Cryptography: From Foundations to Standards."* 
5.  **Won, J. et al. (MIT CSAIL).** (Oct 2025). *"The Continuous Tensor Abstraction."*

---

**License:** Universal Sovereign Source License (USSL) v2.0.

**Copyright (c) 2025 Christophe Duy Quang Nguyen.**
