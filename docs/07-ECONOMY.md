# 07-ECONOMY: The Quantale Physics

*   **File:** `docs/07-ECONOMY.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 1.0 (Dual Token & Fatigue Model)
*   **Date:** November 28, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **The Thermodynamic Laws of the Sovereign Mesh.**
> *Defining the interaction between Local Fuel (`cdqnE`) and Global Value (`cdqnStar`).*

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
*   **Source:** **Entropy Reduction**.
    *   Creating Order (Ship) from Chaos (Kite) releases Energy.
*   **Sink:** **Runtime Execution**.
    *   Every Pulse (Tick) of a Worker/Agent burns `cdqnE`.

### 3.2 The Homeostatic Quota (Anti-Hoarding)
To prevent a node from stockpiling infinite energy for a sudden attack, the system mimics biology.

*   **The Prediction:** The `SystemModule` calculates a 7-day Moving Average of energy consumption.
*   **The Cap:** `Max_Storage = Predicted_Need \times 1.5`.
*   **The Cutoff:** If `Stored_E > Max_Storage`, minting efficiency drops to zero. You cannot store fat you don't need.
*   **Anomaly Detection:** A sudden +500% spike in generation triggers a "Metabolic Lock," freezing the Runtime until Guardian authorization.

---

## 4. The Global Currency: `cdqnStar` (Value)

`cdqnStar` is the external credit of the Node. It represents **Network Utility**.

### 4.1 Properties
*   **Scope:** Global (Transferable).
*   **Source:** **Proof of Trade (Barter).**
    *   Minted *only* when a successful Atomic Swap (Data for Token) occurs between two unique Nodes.
*   **Sink:** **Purchasing Services.**
    *   Buying a "News Deck" or a "Stock Predictor" from another Ronin.

### 4.2 The Fatigue Model (Anti-Mafia)
To prevent "Wash Trading" (Node A selling garbage to Node B repeatedly), we enforce **Interaction Fatigue**.

*   **The Curve:** Minting efficiency decays with frequency per Peer.
    *   **Trade #1:** 100% Minting.
    *   **Trade #2:** 50% Minting.
    *   **Trade #3:** 10% Minting.
    *   **Trade #4+:** 0% Minting.
*   **Recovery:** Efficiency recovers linearly over 24 hours.
*   **Result:** A Mafia requires millions of unique IPs to farm value, making the attack economically unfeasible.

---

## 5. The Thermodynamic Tax (The Link)

This is the central security mechanism.
**"It costs Energy to make Money."**

*   **The Rule:** Executing a Trade Protocol (Encryption, Handshake, Transfer) burns **`cdqnE`** (Real Electricity).
*   **The Profit Equation:**
    $$Profit = Value(Star_{Earned}) - Cost(E_{Burned})$$
*   **The Trap:**
    *   If you Wash Trade (Fake), you hit the **Fatigue Limit**.
    *   `Star_Earned` drops to 0.
    *   `E_Burned` remains constant.
    *   **Result:** The attacker burns real electricity for zero gain.

---

## 6. The Economic Entities

### 6.1 `Agent::Treasurer` (System Root)
*   **Role:** The Central Bank.
*   **Task:** Manages the `cdqnE` Homeostatic Quota and credits the Origin.

### 6.2 `Bot::Bursar` (Module Level)
*   **Role:** The Payroll Manager.
*   **Task:** Requests `cdqnE` grants from the Treasurer and distributes to local Workers.

### 6.3 `Agent::Trader` (Network Level)
*   **Role:** The Merchant.
*   **Task:** Negotiates prices in `cdqnStar`, tracks Peer Fatigue, and executes Atomic Swaps.
