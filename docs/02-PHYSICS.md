# 02-PHYSICS: The Pulse Engine

* **File:** `docs/02-PHYSICS.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.2 (Quantum Update)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **The Laws of Motion, Time, and Quantum Interaction within the Lattice.**

---

## 1. Philosophy: The Virtual Physics

Traditional software operates on **Mechanical Time** (Clock Cycles) and **Fixed Wiring** (Bus/Pointers). CDQN operates on **Biological Time** (Pulse) and **Wave Propagation** (Echo).

We do not "execute code." We **simulate a physical state**. The Runtime acts as a Physics Engine that enforces conservation laws and causality upon the Data.

---

## 2. The Pulse (Active Time & Energy)

In CDQN, Time is the fundamental constant. Space is merely the delay in signal propagation.

### 2.1 The Resonators (Entities)
Only **Entities** (Workers, Bots, Agents) generate a Pulse. The Pulse determines the "Frame Rate" of their logic execution.
*   **High Pulse (Hot):** Active Agents processing critical tasks. Ticks frequently (e.g., ms). High Energy consumption.
*   **Low Pulse (Cold):** Archival Bots or dormant processes. Ticks rarely. Near-zero Energy consumption.

### 2.2 The Ratchet (The Arrow of Time)
Time only moves forward. This is enforced by the **Cryptographic Ratchet** in the `la-cdu`.
*   **Mechanism:** Every Pulse triggers a one-way Key Rotation.
*   **Causality:** You cannot "undo" a Pulse. Past states are immutable.
*   **Liveness:** An Entity with zero `cdqnStar` (Energy) stops Pulsing and freezes in time (Halts).

---

## 3. The Echo (Signal & Space)

Entities communicate by emitting signals into the Lattice. Communication is **Wireless** and **Non-Blocking**.

### 3.1 Vector Propagation
*   **Emission:** An Agent emits an **Echo** (a Query Vector or Intent) into a specific Lattice Sector.
*   **Propagation:** The Runtime calculates the "Time of Flight" (Distance) to nearby Cards.
*   **Resonance:**
    *   **Constructive Interference:** If a Card's **Prime Factors** match the Echo's intent, the signal is amplified. The Card "wakes up" and responds.
    *   **Destructive Interference:** If the Basis Vectors are orthogonal (unrelated), the signal fades into the noise floor.

### 3.2 The "Error" as Nuance
Leveraging **Lattice Cryptography (LWE)**, the "Noise" (Error Vector) carries the context.
*   **The Signal:** The Identity ($P$).
*   **The Noise:** The Phase/Spin ($\phi$).
*   *Result:* A clean signal means "Literal Interpretation." A noisy signal means "Contextual/Nuanced Interpretation."

---

## 4. Quantum Quantale Dynamics (Interaction)

The **Quantale** is not just a cost function; it is a **Probability Engine**.

### 4.1 The Wave Function (Superposition)
When an Agent proposes a complex action, the Physics Engine does not calculate a single result. It generates a **Superposition State ($\Psi$)**.
*   *Input:* `Action` + `Context`.
*   *State $\Psi$:* Contains multiple potential timelines (e.g., "Success Path", "Failure Path", "Creative Path").
*   *Isolation:* These paths exist as "Shadow Cards" and do not affect the immutable Log until collapsed.

### 4.2 The Observer (HMM)
The **Hidden Markov Model (HMM)** acts as the Observer.
*   It scans the Superposition.
*   It compares potential futures against the **Node’s Goal** and **Reputation History**.
*   **The Collapse:** The HMM selects the single path that maximizes utility and safety, making it Real.

---

## 5. Thermodynamics: Noise & Control

The system manages "Creativity" vs. "Safety" using a thermodynamic variable: **Theta ($\theta$)**.

### 5.1 The Temperature ($\theta$)
*   **Low $\theta$ (Freeze):** The system rejects Noise. Only strict, local neighbor connections are allowed. (Used for Finance/Security).
*   **High $\theta$ (Heat):** The system amplifies Noise. Distant, weak connections become viable. (Used for Brainstorming/Innovation).

### 5.2 The Local Rule (Bernshteyn’s Bridge)
Following the principles of **Descriptive Set Theory**, global order emerges from **Local Interaction Rules**.
*   **Rule:** If an interaction generates excessive **Entropy** (Noise violates $\theta$), the Physics Engine dampens the Pulse immediately.
*   **Result:** The network self-stabilizes. Bad logic "cools down" and dies; good logic "heats up" and creates structure.

---

> *"Entities provide the Pulse. Cards provide the Structure. The Engine enforces the Law."*
