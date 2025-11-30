# 01-DATA: The Origami Protocol

*   **File:** `docs/01-DATA-v2.0.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 2.0 (Consolidated Master)
*   **Date:** November 29, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **The Unified Data Model.**
> *Mathematical specification for the Card Data Unit (CDU): Integrating Topology, Physics, and System Artifacts.*

---

## 1. The Archipelago Topology (Legacy Core)

In CDQN, Meaning is Geographic. We separate the **Immutable Map** from the **Mutable Territory**.

### 1.1 The Rocks (Prime Elements)
*   **Definition:** Prime Numbers ($P_2, P_3, \dots$) are the **Immutable Anchors** of the Lattice.
*   **Sovereignty:** There are **NO** reserved or hardcoded Primes.
    *   $P_{Local}$: You define the meaning (e.g., $P_7 = \text{Freedom}$).
    *   $P_{Global}$: The Network defines the meaning (e.g., $P_{19} = \text{Protocol}$).
*   **Translation:** Chronosa acts as the interpreter, mapping Local Rocks to Global Rocks using **Input Slots** in Decks.

### 1.2 The Spaces (Ocean & Sky)
*   **The Ocean (CWM - Chronosa World Model):** The domain of **Known Logic**.
    *   Concepts here are **Ships**. They are tightly anchored to Rocks via High-Tension Tethers.
*   **The Sky (CVM - Chronosa Void Model):** The domain of **Ambiguity**.
    *   Concepts here are **Kites**. They float. They are raw AI output or scraped data waiting for context strings to pull them down.

---

## 2. The Semantic Structure (The Recipe)

Meaning is a Vector, not a String.

### 2.1 The Tether Tuple
A Tether is the atomic bond between a Concept and a Prime.
$$Tether = (Target, \alpha, \phi, \tau)$$

*   **$Target$ (The Anchor):** The Prime ID (Hash).
*   **$\alpha$ (Amplitude):** The **Intensity** (Importance).
*   **$\phi$ (Phase):** The **Orientation** (Context/Channel).
*   **$\tau$ (Tension):** The **Certainty** (0.0 to 1.0).
    *   $\tau = 1.0$: A Ship (Fact).
    *   $\tau < 0.5$: A Kite (Guess).

---

## 3. The Unified Card Model (OrigamiCDU)

Every object in the system—Truth, Bot, Wallet, or Log—is a **CDU**.

### 3.1 The Master Struct
The CDU is a container for **Logic**, **Physics**, and **Art**.

*   **Identity:** The Prime Hash (CAS).
*   **Lineage:** The Owner ID (Module/Agent).
*   **Body (The Subtype):** The specific data payload (Ship, Blueprint, Ledger, etc.).
*   **Glyph (The Face):** Zagier’s Modular Form coefficients. Used to render the data visually or sonically.
*   **Shadow (The Ghost):** A superposition vector used by the Agent to simulate future states before collapsing them to reality.

---

## 4. The Subtype Catalog (The Expansion)

We classify CDUs into four functional domains.

### Domain A: Content (Epistemology)
*   **`Cdu::Ship`:** Anchored, immutable logic. Safe to execute.
*   **`Cdu::Kite`:** Floating, unverified data. Chemically inert (Write-Only/Read-Only).

### Domain B: Structure (The OS)
*   **`Cdu::Origin`:** The Root of the Node. Contains Master Keys and System Config.
*   **`Cdu::Registry`:** In-memory index mapping `Phase -> EntityID`. Used by the Orchestrator to route signals to sleeping entities without disk I/O.
*   **`Cdu::Ledger`:** Append-only log of `cdqnE` spending and Guardian Violations.

### Domain C: Entity (The Engine)
*   **`Cdu::Blueprint`:** The DNA. Static code (WASM/Rust dylib hash). Strict Immutability.
*   **`Cdu::Snapshot`:** The Body. Serialized RAM state (Variables, Context) used for Hibernation. Mutable.

### Domain D: Network (The Dark Forest)
*   **`Cdu::Manifest` (The Seal):**
    *   **Role:** Graph Integrity.
    *   **Content:** Vertex/Edge counts and Degree Sequence. Used to verify a Deck is topologically closed (no missing nodes) via **Kelly's Lemma**.
*   **`Cdu::Certificate` (The Receipt):**
    *   **Role:** Proof of Trade.
    *   **Content:** Signatures from both parties proving a valid atomic swap occurred. Used to mint `cdqnStar`.

---

## 5. Rust Implementation Strategy

```rust
use serde::{Deserialize, Serialize};

/// The specific nature of the Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CduType {
    // Content
    Ship,
    Kite { source: String, signature: Option<Vec<u8>> },
    
    // Structure
    Origin,
    Registry(Vec<crate::tether::Tether>),
    Ledger(Vec<LogEntry>),
    
    // Entity
    Blueprint(Vec<u8>), // Code Hash
    Snapshot { memory: Vec<u8>, energy: u64 }, // RAM Dump
    
    // Network
    Manifest { vertex_count: u64, edge_count: u64, degree_hash: [u8; 32] },
    Certificate { asset_hash: [u8; 32], signatures: Vec<[u8; 64]> },
}

/// The Universal Atom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrigamiCDU {
    /// Content Address (Hash)
    pub id: crate::prime::Prime,
    
    /// The Lineage (Who owns this?)
    pub owner: crate::prime::Prime,
    
    /// The Semantic Vector (Relationships)
    pub tethers: Vec<crate::tether::Tether>,
    
    /// The Functional Payload
    pub kind: CduType,
    
    /// Visuals: Zagier Modular Form Coefficients
    pub glyph: Option<Vec<f64>>,
    
    /// Physics: Quantum Superposition State (Future Simulation)
    pub shadow: Option<Vec<f64>>,
}
```

---

> *"The Rocks are eternal. The Meanings are sovereign. The Deck is complete."*
