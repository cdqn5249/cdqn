# 01-DATA: The Origami Protocol

* **File:** `docs/01-DATA.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.0 (Initial Architecture)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **Mathematical Specification for the Card Data Unit (CDU).**

---

## 1. The Atomic Unit: Prime Elements ($P$)

In standard computing, data is identified by arbitrary Integers (UUIDs) or Strings. In CDQN, we use **Number Theory**. We treat Meaning as a fundamental property of arithmetic.

### 1.1 The Fundamental Theorem of Arithmetic
Every positive integer greater than 1 is either a prime itself or can be represented as the product of prime numbers in a unique way. This is the foundation of our semantic system.

**CDQN Application:**
*   **Atomic Concepts** are assigned **Prime Numbers**.
    *   $P_{Time} = 2$
    *   $P_{Space} = 3$
    *   $P_{Entity} = 5$
*   **Complex Concepts** are created by **Multiplication (Concatenation)**.
    *   Concept "TimeSpace": $2 \times 3 = 6$.
    *   Concept "Living Entity": $5 \times P_{Life}$.

### 1.2 Semantic Factorization
Because prime factorization is unique, any Card ID (Integer) can be instantly decomposed into its constituent concepts without needing a lookup table.
*   **Input:** Card ID `30`.
*   **Factorization:** $2 \times 3 \times 5$.
*   **Meaning:** This card relates to Time, Space, and Entity.

---

## 2. The Glyph: Zagier’s Modular Forms

How do we represent media (Sound, Image, Logic) inside a Prime Number system? We use **Generating Functions**, inspired by the work of Don Zagier on Modular Forms.

### 2.1 The Omni-Modal Token
A generic "Token" (like in LLMs) is static. A **CDQN Glyph** is a function.
We define the Glyph as a **Spectral Map** of coefficients ($a_n$) where $n$ corresponds to our Prime Elements.

$$f(z) = \sum_{n=0}^{\infty} a_n q^n$$

*   **The Coefficients ($a_n$):** These are the Prime Factors stored in the Lattice.
*   **The Projection ($q$):** This is determined by the **World** (CWM).

### 2.2 Projections (The Universal File Format)
The same mathematical object (The Glyph) renders differently depending on the Observer's Context.

*   **Projection A (Visual):** Map the coefficients to an $X,Y$ grid. The "Valleys" of energy become pixels.
*   **Projection B (Audio):** Map the coefficients to Frequency/Time. The "Peaks" become notes.
*   **Projection C (Logic):** Map the coefficients to `cdqnLang` instructions.

**Result:** We do not store `.jpg` or `.mp3`. We store the **Mathematical DNA** (The Sheet Music) that generates the file. This drastically reduces storage size ("Tiny AI") and allows for resolution independence.

---

## 3. Topology: Recursive Folding (Origami)

Data in CDQN is not flat; it is a **Manifold**. We handle complexity through **Folding**, a concept mathematically linked to Nested Learning.

### 3.1 The Fold State
*   **Unfolded (Hot):** The Glyph is fully expanded in RAM. High fidelity. All Prime factors are active and interacting via the Pulse.
*   **Folded (Cold):** The Glyph is compressed into a **Singularity** (A Hash or a single large Prime). Details are hidden but mathematically retrievable.

### 3.2 Mock Forms (Handling Uncertainty)
Real-world data is messy. Sometimes we have "Noise" or "Missing Data."
We utilize Zagier’s concept of **Mock Modular Forms**.
*   **The Shadow:** We store the "Ideal Form" (The Prime) plus a "Shadow" (The Noise Vector).
*   **Restoration:** When the system reads a Mock Form, it uses Lattice Symmetry to mathematically "guess" the missing data (In-painting/Error Correction) without using heavy Neural Networks.

---

## 4. Implementation Strategy (Rust)

This architecture will be implemented using the following Rust structures (Draft):

1.  **`struct Prime`:** A wrapper around `u64` (or `BigInt` for complex concepts) with cached factorization methods.
2.  **`struct Glyph`:** A vector of coefficients `Vec<Complex<f64>>` representing the Modular Form.
3.  **`trait Foldable`:**
    *   `fn fold(&self) -> Hash`
    *   `fn unfold(&self, world: World) -> Result<Media, Error>`

---

> *"Data is not a static record. It is a vibration."*
