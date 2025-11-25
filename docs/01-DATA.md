# 01-DATA: The Origami Protocol

* **File:** `docs/01-DATA.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.2 (Sparse Factor Update)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **Mathematical Specification for the Card Data Unit (CDU).**
> *Defining the Dual Addressing Model, Sparse Semantic Factors, and Omni-Modal Glyphs.*

---

## 1. The Hybrid Addressing Model

To achieve "Tiny AI" performance on consumer hardware while maintaining mathematical rigor, we utilize a split addressing system:

1.  **Physical Address (Storage):** **Cryptographic Hash (Blake3).**
    *   Data is stored in Content-Addressable Memory (mmap). Access is $O(1)$.
    *   *Role:* The "Pointer."
2.  **Logical Identity (Reasoning):** **Prime Elements.**
    *   Concepts are defined by Number Theory relations.
    *   *Role:* The "Type System."

---

## 2. The Semantic Architecture: Prime Elements ($P$)

We treat meaning as a **Composite Material** made of fundamental atoms (Primes).

### 2.1 Sparse Factor Representation
To avoid Integer Overflow from multiplying large primes, we do not store the resulting product. We store the **Pre-Factored Vector**.

*   **The Structure:**
    A Logical ID is not a single number. It is a **Tensor of Tuples**:
    $$ID = [ (P_1, \alpha_1, \phi_1), (P_2, \alpha_2, \phi_2), \dots ]$$

    Where for each Prime Element $P$:
    *   **$P$ (Identity):** The fundamental concept (e.g., $P_{Time}=2$).
    *   **$\alpha$ (Amplitude/Exponent):** The intensity or weight of the concept.
    *   **$\phi$ (Phase/Spin):** The orientation (e.g., Positive/Love vs Negative/Hate).

*   **Example: "Urgent Risk"**
    *   $P_{Risk} = 7$, Amplitude = 10 (High).
    *   $P_{Time} = 2$, Amplitude = 5 (Urgent), Phase = -90° (Approaching).
    *   **Storage:** `[(7, 10, 0), (2, 5, -90)]`.

### 2.2 Semantic Arithmetic
This structure allows the Physics Engine to perform logic using simple vector math:
*   **Composition:** Merging two cards = Union of Vectors.
*   **Comparison:** Vector Dot Product = Semantic Resonance.
*   **Validation:** Prime divisibility checks = Type Safety.

---

## 3. The Visual Glyph: Zagier’s Modular Forms

The CDU acts as a **Holographic Seed** capable of generating Omni-Modal content (Text, Image, Audio) from a single compressed signature.

### 3.1 The Generating Function
Inspired by Don Zagier’s work on **Modular Forms**, we treat data as a continuous function on a Lattice, rather than a static bitmap.

*   **Implementation:** **Wavelet Coefficients.**
    *   We store the **Spectral Signature** of the data (the "Sheet Music") rather than the raw pixels (the "Recording").
    *   This allows for resolution independence. The same Glyph can be "played back" (rendered) at low resolution for icons or high resolution for analysis.

### 3.2 Projections (The Universal Player)
The **World (CWM)** acts as the decoder for the Wavelet:
*   **Visual Projection:** Maps coefficients to Spatial $(X,Y)$ frequencies $\to$ Image.
*   **Audio Projection:** Maps coefficients to Temporal $(T,Hz)$ frequencies $\to$ Sound.
*   **Logic Projection:** Maps coefficients to Logic Gates $\to$ Code.

---

## 4. Topology: Recursive Folding (Origami)

The CDU is a **Fractal Manifold**. It manages complexity through "Folding" (Abstraction).

### 4.1 The Fold State
*   **Unfolded (Hot):** The Vector of Factors is loaded in RAM. Full semantic resolution.
*   **Folded (Cold):** The Vector is hashed into a single **Singularity**. Details are hidden but verifiable via the Hash.

### 4.2 Mock Forms (The Shadow)
To handle "Noisy" or "Incomplete" data (Real World inputs), we utilize the concept of **Mock Modular Forms**.
*   **The Structure:** `MockCDU = { Ideal_Vector + Shadow_Vector }`.
*   **The Shadow:** Stores the "Error" or "Noise" that deviates from the ideal concept.
*   **Usage:** Chronosa uses the Shadow to calculate **Nuance** (Stochastic Resonance) without corrupting the purity of the Ideal Prime definitions.

---

## 5. Rust Implementation Strategy

```rust
/// The Atomic Unit of Meaning
struct PrimeFactor {
    prime: u64,      // The Concept ID
    amplitude: f32,  // The Intensity (Exponent)
    phase: f32,      // The Spin/Nuance
}

/// The "Card" Identity
struct SemanticTensor {
    factors: Vec<PrimeFactor>, // Sparse representation
    hash: [u8; 32],            // Physical Address (Blake3)
}

/// The Omni-Modal Content
struct Glyph {
    coefficients: Vec<Complex<f64>>, // Wavelet/Modular Form data
    mock_shadow: Option<Vec<f64>>,   // Noise/Error data
}
```

---

> *"We do not calculate the Universe. We factor it."*
