# 04-LANG: The Instruction Set (cdqnLang)

*   **File:** `docs/04-LANG.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 2.0 (Orchestration, Slots, & Adaptive Physics)
*   **Date:** November 30, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **The Logic of the Virtual Chip. Programming with Physics, not Memory.**

---

## 1. Philosophy: The Geometrization of Code

Traditional programming languages manage **Linear Memory** (Stack/Heap). **cdqnLang** manages **Topological Memory** (Lattice/Graph).

It is a **Domain Specific Language (DSL)** for orchestrating Entities, Data Flows, and Tethers. It is **NOT** a general-purpose scripting language.
*   **The Bricks (Rust/WASM):** Atomic logic (Math, Hashing, rendering) is written in Rust and compiled to WASM.
*   **The Blueprint (cdqnLang):** The wiring diagram that connects these bricks, defines their Energy cost, and maps their inputs.

---

## 2. The Layered Stack

We address the physical constraints of binary hardware by splitting concerns.

### Level 0: The Hybrid Addressing (Storage)
*   **Physical Layer:** **Content-Addressed Storage (CAS)**. Data is immutable.
*   **Logical Layer:** **Prime Elements**. Types are defined by factorization ($P_A \times P_B = P_{AB}$).

### Level 1: The Compiler (The Architect)
*   **Role:** Translation & Verification.
*   **Input:** `cdqnLang` source.
*   **Output:** `Cdu::Blueprint` (Binary).
*   **The Kelly Check:** The compiler verifies the **Topological Closure** of the deck using Kelly's Lemma. It ensures no Tethers point to non-existent nodes (Orphans).
*   **The Energy Check:** It calculates the **Worst-Case Energy Cost** (`cdqnE`) pre-runtime. Unbounded loops are rejected.

### Level 2: The Interface (Deck Building)
*   **Role:** Portability.
*   **Relative Addressing (Slots):** To share logic between Nodes with different private definitions, Decks use **Input Slots**.
    *   *Bad:* Hardcoding `P_7` (My definition of Money).
    *   *Good:* Declaring `SLOT: $Finance_Concept`. The User maps this to their own Prime upon installation.

---

## 3. Syntax Specification (Draft)

The syntax allows users to define **Tethers** (Topology), **Physics** (Time/Energy), and **Logic** (Execution).

### 3.1 Defining a Ship (Standard Logic)
A "Ship" is a fully defined, active logic unit.

```cdqn
BLUEPRINT #Analyze_Sentiment {
    // 1. Dependencies (Relative Addressing)
    // The User must map these slots to local Primes to install the Deck.
    SLOTS: [
        $Positive_Anchor,
        $Negative_Anchor
    ]

    // 2. Topology & Context
    TYPE: SHIP 
    WORLD: SocialWorld

    // 3. Identity: The Tether Vector
    // Syntax: target ^Amplitude /Phase
    TETHERS: [
        @Self            ^1.0,       // Internal Anchor
        $Positive_Anchor ^0.8 /0.0,  // Resonance (In Phase)
        $Negative_Anchor ^0.8 /PI    // Dissonance (Anti-Phase)
    ]

    // 4. Physics: Bounded Execution
    // "Adaptive" allows the worker to burn extra fuel for deep thought.
    PULSE: ADAPTIVE 
    COST: 5 cdqnE
    TIMEOUT: 500ms 

    // 5. Logic: The Binding
    // References the hash of the compiled Rust/WASM function.
    EFFECT: wasm("hash:a1b2c3d4...") 
}
```

### 3.2 Defining a Kite (Ambiguous Data)
A "Kite" is data retrieved from the Void (e.g., via the Harvester).

```cdqn
BLUEPRINT #Unknown_Signal {
    TYPE: KITE
    WORLD: Void // The CVM
    
    // Strings: Weak connections (Low Tension)
    STRINGS: [
        $Finance_Topic ~0.3, // 30% probability
        $Scam_Topic    ~0.4
    ]
    
    // Behavior: Passive
    // It does not execute logic; it waits for external validation (Tension increase).
    EFFECT: NONE
}
```

### 3.3 The Consensus Call (Networking)
How to talk to the Public Lattice without Semantic Drift.

```cdqn
FUNCTION #Send_Public_Message {
    INPUT: $Private_Card
    
    PROCESS: {
        // 1. Check Global Consensus via the Diplomat
        let public_map = QUERY_CONSENSUS(target: PublicLattice);
        
        // 2. Translate Private Primes to Public Primes
        let public_card = TRANSLATE($Private_Card, using: public_map);
        
        // 3. Emit the Echo (Signal)
        EMIT(public_card);
    }
}
```

---

## 4. The Compiler Strategy

1.  **Slot Verification:** Ensure all `SLOTS` are defined.
2.  **Static Analysis:** Validate Tether syntax and Phase alignment.
3.  **Graph Integrity:** Run **Kelly's Reconstruction Check** on the Module graph. Verify $\sum(Edges) = \text{Manifest Count}$.
4.  **Cost Bounding:** Verify `COST` does not exceed System Max.
5.  **Hashing:** Generate the `Cdu::Blueprint` hash.

---

> *"We restrict the language to liberate the logic."*
