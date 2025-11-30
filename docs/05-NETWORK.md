# 05-NETWORK: The Sovereign Mesh

*   **File:** `docs/05-NETWORK.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 2.0 (Holographic Integrity & Atomic Barter)
*   **Date:** November 30, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **Protocols for Connectivity, Integrity, and Trade in the Dark Forest.**

---

## 1. Topology: Private Bays & Public Seas

The network is not a flat namespace. It is strictly divided to ensure sovereignty while allowing trade.

### 1.1 Private Space (The Node)
*   **Content:** User's `OriginCdu`, Private Decks, and internal Logic.
*   **Rules:** Total Sovereignty. You define the Primes ($P_7 = \text{My Cat}$). You define the Truth.
*   **Visibility:** **Invisible.** It exists only on your hardware.

### 1.2 Shared Space (The Lattice)
*   **Content:** `axons-cdu` (Shared Logic), Public Worlds (`FinanceWorld`), and the **Global Void**.
*   **Rules:** Defined by **Global Consensus**.
*   **Islands:** Corporate or Group zones exist here as gated territories.

---

## 2. The Translation Protocol (Solving Babel)

To prevent Semantic Drift between nodes with different private definitions, Chronosa acts as the **Interpreter**.

### 2.1 The Calibration Query
Before sending a message to the Shared Space:
1.  **Query:** Chronosa checks the **Global Consensus Instance**: "What is the current public anchor for 'Value'?"
2.  **Translation:** She maps your Private Concept (e.g., anchored to $P_{7}$) to the Public Concept (e.g., anchored to $P_{19}$).
3.  **Transmission:** The message is sent using **Public Coordinates**.

---

## 3. The Structural Defense (Holographic Integrity)

In a P2P network, you cannot trust the sender. You must verify the **Structure** of the data.

### 3.1 The Airlock Protocol
All incoming data is received into a **Restricted Module (Airlock)**. It has no read access to the Node's core.

### 3.2 The Holographic Manifest Check (Kelly's Lemma)
Every Deck transfer must include a **`Cdu::Manifest`**. The Receiver's `Bot::Auditor` performs a mathematical census before accepting the data.
1.  **Counting:** The Auditor counts the actual Vertices ($N$) and Tethers ($E$) in the received bundle.
2.  **Reconstruction:** It verifies the **Degree Sequence** against the Manifest.
3.  **Closure:** It ensures the Graph is **Closed** (no Tethers pointing to missing nodes).
4.  **Verdict:** If the math fails, the Deck is rejected as "Corrupted" or "Tampered."

---

## 4. The Atomic Barter Protocol (Trade)

We replace "Trust" with "Cryptographic Proof."

### 4.1 The Handshake (Tether-Shake)
**Scenario:** Alice sends a Deck to Bob.
1.  **Proposal:** Alice sends `Signal::Offer` with the Deck's Manifest.
2.  **Review:** Bob's Diplomat checks the Manifest against his **Fatigue Limit** (Have we traded too much with Alice?).
3.  **Transfer:** Alice sends the encrypted Deck to Bob's Airlock.
4.  **Audit:** Bob's Auditor runs the **Manifest Check** (Section 3.2).

### 4.2 The Certification (Minting)
If the Audit passes:
1.  **Signing:** Bob signs a partial **`Cdu::Certificate`**: *"I certify I received valid Deck [Hash]."*
2.  **Exchange:** Bob sends the Certificate to Alice.
3.  **Finalization:** Alice signs it. The Certificate is now a valid **Proof of Trade**.

---

## 5. The Economic Interface

*(For full physics, see `docs/07-ECONOMY.md`)*

### 5.1 Local Fuel (`cdqnE`)
*   **Source:** **Entropy Reduction.** Organizing Chaos (Kites) into Order (Ships).
*   **Constraint:** Only **External Entropy** (verified by Airlock signature) generates fuel. You cannot mint fuel by shuffling your own files.

### 5.2 Global Value (`cdqnStar`)
*   **Source:** **Trade Only.**
*   **Mechanism:** The `Treasurer` accepts valid **`Cdu::Certificates`** to mint `cdqnStar`.
*   **Constraint:** **Interaction Fatigue.** Repeated trades with the same Peer yield diminishing returns, preventing "Gold Farming" (Wash Trading).

---

> *"Trust is not required. Mathematics is sufficient."*
