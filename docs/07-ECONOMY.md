# 07-ECONOMY: The Quantale Physics

*   **File:** `docs/07-ECONOMY.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 2.0 (Artifact-Driven Minting)
*   **Date:** November 30, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **The Thermodynamic Laws of the Sovereign Mesh.**
> *Defining the interaction between Local Fuel (`cdqnE`) and Global Value (`cdqnStar`) via Cryptographic Proofs.*

---

## 1. Philosophy: Thermodynamics of Information

In CDQN, "Economics" is not about finance; it is about **Physics**.
We reject the premise of infinite growth. We adopt the premise of **Homeostasis**.

*   **Entropy ($\Delta S$):** The measure of disorder (Chaos/Kites).
*   **Work ($W$):** The energy required to organize Chaos into Order (Ships).
*   **Value ($V$):** The utility provided to the network.

The system is designed to prevent "Gold Farming" (Fake Value) by enforcing strict thermodynamic costs.

---

## 2. The Quantale Algebra (Resource Physics)

We use **Quantales** to model resource consumption strictly.
Unlike standard integers, Quantale resources cannot be copied, only consumed or transferred.

### 2.1 The Equation
$$State_{New} = State_{Old} \otimes Resource$$
*   If `Resource < Required_Cost`, the operation is **Mathematically Impossible**. The transition does not happen.
*   This prevents "Debt" or "Overdraft" at the physics level. You cannot run a CPU cycle you cannot pay for.

---

## 3. The Local Fuel: `cdqnE` (Energy)

`cdqnE` is the internal battery of the Node. It represents **Computational Potential**.

### 3.1 Properties
*   **Scope:** Local Only (Non-Transferable).
*   **Sink:** **Runtime Execution**. Every Pulse of a Worker/Agent burns `cdqnE`.

### 3.2 Source-Weighted Minting (Entropy Reduction)
Fuel is generated when a Worker organizes data. To prevent "Entropy Farming" (creating fake chaos to fix it), we enforce strict source rules.

1.  **Local Source (Difficulty 0):**
    *   *Action:* Organizing user-generated data.
    *   *Reward:* **Zero.** Maintenance is its own reward.
2.  **External Source (Difficulty 1):**
    *   *Action:* Harvesting and organizing data from the Void (Web).
    *   *Proof:* The Kite must bear a cryptographic signature from the **Airlock Module**.
    *   *Reward:* **Mint `cdqnE`.** You are extracting order from the chaos of the world.

### 3.3 The Homeostatic Quota
*   **The Cap:** You cannot stockpile more than `1.5x` your 7-day average consumption.
*   **The Cutoff:** If `Stored_E > Max`, minting efficiency drops to zero.

---

## 4. The Global Currency: `cdqnStar` (Value)

`cdqnStar` is the external credit of the Node. It represents **Network Utility**.

### 4.1 Properties
*   **Scope:** Global (Transferable).
*   **Sink:** Purchasing Decks/Services from the Mesh.

### 4.2 Proof of Trade (Minting)
`cdqnStar` is minted *only* via the **Atomic Barter Protocol** (see `docs/05-NETWORK.md`).
1.  **The Event:** A successful trade of a valid Deck.
2.  **The Artifact:** A **`Cdu::Certificate`** signed by both parties, referencing the Deck's `Manifest`.
3.  **The Mint:** The `Treasurer` accepts the Certificate and mints `cdqnStar`.

### 4.3 The Fatigue Model (Anti-Mafia)
To prevent "Wash Trading" (Node A selling garbage to Node B repeatedly), we enforce **Interaction Fatigue**.
*   **The Curve:** Minting efficiency decays with frequency per Peer.
    *   Trade #1: 100% Value.
    *   Trade #4+: 0% Value.
*   **Recovery:** Efficiency recovers linearly over 24 hours.
*   **Result:** A Mafia requires millions of unique Node IDs (hardware) to farm value.

---

## 5. The Thermodynamic Tax (The Link)

This is the central security mechanism. **"It costs Energy to make Money."**

*   **The Rule:** Executing the **Tether-Shake** (Handshake, Encryption, Transfer, Audit) burns **`cdqnE`** (Real Electricity).
*   **The Profit Equation:**
    $$Profit = Value(Star_{Earned}) - Cost(E_{Burned})$$
*   **The Trap:** If you Wash Trade, you hit the **Fatigue Limit**. `Star` drops to 0, but `E_Burned` remains constant. The attack becomes economically negative.

---

## 6. The Economic Entities

### 6.1 `Agent::Treasurer` (System Root)
*   **Role:** The Sovereign Accountant.
*   **Task:** Verifies `Cdu::Certificate` and `EntropyReceipt`, credits the Origin, and sets the Homeostatic Quota.

### 6.2 `Bot::Bursar` (Module Level)
*   **Role:** The Payroll Manager.
*   **Task:** Requests `cdqnE` grants from the Treasurer and distributes to local Agents/Workers.

### 6.3 `Agent::Trader` (Network Level)
*   **Role:** The Merchant.
*   **Task:** Negotiates prices, tracks Peer Fatigue, and manages the `Cdu::Certificate` signing process.

---

> *"Energy is burned locally. Value is recognized globally."*
