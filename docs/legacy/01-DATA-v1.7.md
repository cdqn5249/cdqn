# 01-DATA: The Origami Protocol

* **File:** `docs/01-DATA.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.7 (Sovereign Consistency Update)
* **Date:** November 26, 2025
* **Author:** Christophe Duy Quang Nguyen

> **Mathematical Specification for the Card Data Unit (CDU).**
> *Defining the Topology of Ships, Kites, and Sovereign Anchors.*

---

## 1. The Archipelago Topology

In CDQN, Meaning is Geographic. We separate the **Immutable Map** from the **Mutable Territory**.

### 1.1 The Rocks (Prime Elements)
*   **Definition:** Prime Numbers ($P_2, P_3, \dots$) are the **Immutable Anchors** of the Lattice.
*   **Sovereign Definition:** There are **NO** reserved or hardcoded Primes.
    *   The User is free to assign any meaning to any Prime in their Private Space.
    *   *Example:* User A defines $P_7 = \text{Money}$. User B defines $P_7 = \text{Health}$.
*   **Interoperability (The Translation Layer):** To prevent confusion, Chronosa acts as the **Interpreter**.
    *   Before connecting to the Public Lattice, Chronosa queries the **Global Consensus**.
    *   She dynamically translates Private Primes into Public Consensus Primes for the duration of the transaction.

### 1.2 The Spaces (Ocean & Sky)
*   **The Ocean (CWM - Chronosa World Model):** The domain of **Known Logic**.
    *   Here, concepts are **Origami Ships**. They are tightly anchored to Rocks via Tethers.
*   **The Sky (CVM - Chronosa Void Model):** The domain of **Ambiguity**.
    *   Here, concepts are **Kites**. They float. They are "Unknowns" waiting for context strings to pull them down.

---

## 2. The Semantic Structure: Sparse Factors

To avoid Integer Overflow and capture Nuance, we store the **Recipe** (The Vector of Tethers).

### 2.1 The Semantic Tuple
A Tether is the atomic bond between a Concept and a Prime.
$$Tether = (P, \alpha, \phi, \tau)$$

*   **$P$ (The Anchor):** The Prime Element ID.
*   **$\alpha$ (Amplitude):** The **Intensity** (Coefficient).
*   **$\phi$ (Phase):** The **Orientation** (Spin).
    *   $0^\circ$: Positive/Active.
    *   $180^\circ$: Negative/Passive.
*   **$\tau$ (Tension):** The **Certainty** (0.0 to 1.0).
    *   $\tau = 1.0$: An Anchor (Ship).
    *   $\tau < 0.5$: A String (Kite).

### 2.2 The Sparse Tensor
A Card ID is the collection of all its Tethers.
$$ID = [ (P_1, \alpha_1, \phi_1, \tau_1), (P_2, \alpha_2, \phi_2, \tau_2), \dots ]$$
*   **Storage:** This Vector is hashed (Blake3) to create the physical Content Address.

---

## 3. The Visual Glyph: Zagier’s Modular Forms

The CDU acts as a **Holographic Seed**. The Semantic Tensor generates the Media via **Modular Forms**.

### 3.1 The Generating Function
We treat data as a continuous Wavelet anchored to the Lattice.
*   **The Input:** The Semantic Tensor (The Tethers).
*   **The Process:** The Amplitudes ($\alpha$) drive the Wavelet coefficients.
*   **The Output:** A **Spectral Map** (The Glyph).

### 3.2 Projections (Runtime Only)
The **World (CWM)** determines how that Glyph is *perceived*.
*   **Visual Projection:** Maps Spectral Map to $X,Y$ pixels.
*   **Note:** When a Kite lands (becomes a Ship), its Glyph stabilizes. While floating, the Glyph may shimmer (Phase Uncertainty).

---

## 4. The State Machine: Ships vs. Kites

This is the defense against Hallucination.

### 4.1 The Ship (Anchored Knowledge)
*   **State:** **Locked.**
*   **Logic:** "I know this is True in this World."
*   **Usage:** Safe to use in strict Quantale operations.

### 4.2 The Kite (Suspended Judgment)
*   **State:** **Floating.**
*   **Logic:** "I see this data, but I don't know where it fits."
*   **Structure:** Held by weak "Strings" (Low Tension Tethers) to potential Rocks.
*   **Resolution:** As new feedback arrives, the Strings tighten. When Tension > Threshold, the Kite is pulled down from the Void and becomes a Ship.

---

## 5. Rust Implementation Strategy

```rust
/// The Atomic Connection
#[derive(Serialize, Deserialize)]
struct Tether {
    prime_anchor: u64, // P (Local Definition)
    amplitude: f32,    // Alpha
    phase: f32,        // Phi
    tension: f32,      // Tau
}

/// The Card State
enum TopologyState {
    Ship(Vec<Tether>), // Anchored in CWM
    Kite(Vec<Tether>), // Floating in CVM
}

/// The Card Data Unit
#[derive(Serialize, Deserialize)]
struct OrigamiCDU {
    // The Logic
    state: TopologyState,
    
    // The Representation
    glyph_cache: Option<Vec<u8>>, 
    
    // The Shadow
    shadow_vector: Option<Vec<f32>>,
}
```

---

> *"The Rocks are eternal. The Meanings are sovereign."*
