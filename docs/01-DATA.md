# 01-DATA: The Origami Protocol

* **File:** `docs/01-DATA.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.5 (Unified Pipeline Update)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **Mathematical Specification for the Card Data Unit (CDU).**
> *Defining the pipeline from Prime Anchors to Holographic Glyphs.*

---

## 1. The Archipelago Topology

We reject the rigid "Dictionary Model" of standard databases. In CDQN, Meaning is not static; it is **Geographic**. We separate the **Immutable Map** from the **Mutable Territory**.

### 1.1 The Rocks (Prime Elements)
*   **Definition:** Prime Numbers ($P_2, P_3, \dots$) are the **Immutable Anchors** of the Lattice.
*   **Function:** They act as "Geodetic Markers." Rock $P_7$ has no inherent human meaning (it is not "Red"). It is a **Coordinate** that never moves.

### 1.2 The Origamis (Semantic Concepts)
*   **Definition:** Meanings (e.g., "Time", "Risk") are **Fluid Structures** (Origamis).
*   **Tethering:** An Origami "anchors" itself to specific Rocks via a **Tethering Vector**.
*   **Agility:** This tether is dynamic. As the user's understanding evolves, a Concept can re-anchor to different Primes without breaking the underlying Lattice History.

---

## 2. The Semantic Structure: Sparse Factors

To avoid Integer Overflow and capture Nuance, we do not store a single product number. We store the **Recipe** (The Vector of Tethers).

### 2.1 The Semantic Tuple
A Tether is the atomic bond between a Concept and a Prime.
$$Tether = (P, \alpha, \phi)$$

*   **$P$ (The Anchor):** The Prime Element ID.
*   **$\alpha$ (Amplitude):** The **Intensity**. This acts as the **Coefficient** for the generating function.
*   **$\phi$ (Phase):** The **Orientation** (Spin).
    *   $0^\circ$: Positive/Active.
    *   $180^\circ$: Negative/Passive.
    *   $90^\circ$: Orthogonal/Nuanced.

### 2.2 The Sparse Tensor
A Card ID is the collection of all its Tethers.
$$ID = [ (P_1, \alpha_1, \phi_1), (P_2, \alpha_2, \phi_2), \dots ]$$

*   **Storage:** This Vector is hashed (Blake3) to create the physical filename.
*   **Arithmetic:** Merging two cards is **Vector Addition**.
    *   If Card A has $(P_7, 10, 0)$ and Card B has $(P_7, 5, 0)$, the result is $(P_7, 15, 0)$.

---

## 3. The Visual Glyph: Zagier’s Modular Forms

The CDU acts as a **Holographic Seed**. The Semantic Tensor (Section 2) *is* the code that generates the Media (Section 3).

### 3.1 The Generating Function
We treat data as a continuous function anchored to the Lattice. We use **Wavelets** as the implementation of Zagier's Modular Forms.

*   **The Input:** The Semantic Tensor (The Tethers).
*   **The Process:**
    $$f(z) = \sum \alpha_n q^{P_n} e^{i\phi_n}$$
    *   The **Amplitudes ($\alpha$)** drive the Wavelet coefficients.
    *   The **Phases ($\phi$)** drive the interference pattern.
*   **The Output:** A **Spectral Map** (The Glyph).

### 3.2 Projections (Runtime Only)
The CDU stores the **Invariant Glyph**. The **World (CWM)** determines how that Glyph is *perceived* at runtime.

*   **Visual Projection:** Maps the Spectral Map to $X,Y$ pixels.
*   **Audio Projection:** Maps the Spectral Map to $Time,Freq$ audio.
*   **Note:** The file on disk does **not** change when the World changes. Only the "Shadow" cast by the Glyph changes.

---

## 4. Topology: Recursive Folding

The CDU is a **Fractal Manifold**.

### 4.1 The Fold State
*   **Unfolded (Hot):** The Semantic Tensor is loaded in RAM. The Generating Function is active.
*   **Folded (Cold):** The Tensor is hashed. The data is "sedimented" to disk.

### 4.2 Mock Forms (The Shadow)
To handle "Noisy" or "Incomplete" data:
*   **The Shadow:** We store the "Ideal Tensor" plus a "Noise Vector."
*   **Usage:** Chronosa uses the Noise Vector to calculate **Stochastic Resonance** (Creativity). It is the "Grit" that gives the digital asset texture.

---

## 5. Rust Implementation Strategy

```rust
/// The Atomic Connection
#[derive(Serialize, Deserialize)]
struct Tether {
    prime_anchor: u64, // P
    amplitude: f32,    // Alpha (Coefficient)
    phase: f32,        // Phi (Spin)
}

/// The Card Data Unit (Immutable on Disk)
#[derive(Serialize, Deserialize)]
struct OrigamiCDU {
    // The Definition (The DNA)
    // This is hashed to generate the Content Address
    semantic_tensor: Vec<Tether>, 
    
    // The Representation (The Body)
    // Generated from the Tensor via Wavelet Transform
    glyph_cache: Option<Vec<u8>>, 
    
    // The Deviation (The Shadow)
    // Stores noise/error for Mock Forms
    shadow_vector: Option<Vec<f32>>,
}
```

---

> *"The Tethers are the DNA. The Glyph is the Body. The World is the Environment."*
