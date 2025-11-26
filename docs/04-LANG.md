# 04-LANG: The Instruction Set (cdqnLang)

* **File:** `docs/04-LANG.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.3 (Tether & Topology Update)
* **Date:** November 26, 2025
* **Author:** Christophe Duy Quang Nguyen

> **The Logic of the Virtual Chip. Programming with Physics, not Memory.**

---

## 1. Philosophy: The Geometrization of Code

Traditional programming languages manage **Linear Memory**. **cdqnLang** manages **Topological Memory**.

It is a **Domain Specific Language (DSL)** for orchestrating Agents, Logic, and Data Flows. It enforces **Total Functional Programming**: no infinite loops, no unbound recursion, and strict resource accounting via **Quantales**.

---

## 2. The Layered Stack

We address the physical constraints of binary hardware by splitting concerns.

### Level 0: The Hybrid Addressing (Storage)
*   **Physical Layer:** **Content-Addressed Storage (CAS)** via Blake3 Hashing. Data is immutable and deduplicated.
*   **Logical Layer:** **Prime Elements**. Types are defined by factorization, allowing for mathematical type safety ($P_A \times P_B = P_{AB}$).

### Level 1: The Kernel (Rust Bootstrap)
*   **Role:** The Compiler. Written in Rust to translate `cdqnLang` into the optimized ECS schedule.

### Level 2: The Logic Layer (Bounded Quantales)
*   **Role:** The Safety Switch.
*   **Mechanism:** The Compiler calculates the **Worst-Case Energy Cost** (`cdqnStar`) pre-runtime.
*   **Constraint:** Code that exceeds the Node's Pulse budget or threatens infinite execution **fails to compile**.

### Level 3: The Interface (Deck Building)
*   **Role:** The Developer Experience.
*   **Metaphor:** Functions are **Cards**. Modules are **Decks**.
*   **Safety:** Connections between cards are validated by **Morphisms**. You cannot connect incompatible Primes unless a Functor (Translation) exists.

---

## 3. Syntax Specification (Draft)

The syntax allows users to define **Tethers** (Nuance) and **Topology** (State).

### 3.1 Defining a Ship (Anchored Logic)
A "Ship" is a fully defined, active logic unit.

```cdqn
CARD #Analyze_Sentiment {
    // Topology: Anchored to specific Primes
    STATE: SHIP 
    WORLD: SocialWorld

    // Identity: The Tether Vector (Prime, Amplitude, Phase)
    // Syntax: @Prime ^Intensity /Phase
    TETHERS: [
        @Analysis ^1.0,       // Standard
        @Emotion  ^2.0,       // High Intensity
        @Text     ^1.0 /0.0   // Positive Phase
    ]

    // Physics: Bounded Execution
    COST: 5 Star
    TIMEOUT: 500ms 

    // Logic: Deterministic Script
    EFFECT: |input| => {
        // ECHO is a Vector Search (Wavelet Propagation)
        // Returns a Resonance score (0.0 to 1.0)
        let resonance = ECHO(input, target: #Positive_Vibe);
        
        match resonance {
            case > 0.8 -> {
                PULSE(High); 
                return #Happy;
            }
            case _ -> {
                PULSE(Low);
                return #Neutral;
            }
        }
    }
}
```

### 3.2 Defining a Kite (Draft Logic)
A "Kite" is ambiguous data deployed to the Void.

```cdqn
CARD #Unknown_Signal {
    STATE: KITE
    WORLD: Void // The CVM
    
    // Strings: Weak connections (Low Tension)
    STRINGS: [
        @Finance ~0.3, // 30% probability
        @Scam    ~0.4
    ]
    
    // Behavior: Wait for context
    EFFECT: |context| => {
        if context.confirms(@Finance) {
            // Pull down to Reality
            return ANCHOR(to: FinanceWorld);
        } else {
            return FLOAT;
        }
    }
}
```

### 3.3 The Network Handshake (Consensus)
How to talk to the Public Lattice without Semantic Drift.

```cdqn
FUNCTION #Send_Public_Message {
    INPUT: private_card
    
    PROCESS: {
        // 1. Check Global Consensus
        let map = QUERY_CONSENSUS(target: PublicLattice);
        
        // 2. Translate Private Primes to Public Primes
        let public_card = TRANSLATE(private_card, using: map);
        
        // 3. Emit the Echo
        EMIT(public_card);
    }
}
```

---

## 4. The Compiler Strategy

1.  **Static Analysis:** Validate Tethers and Phase alignments.
2.  **Cost Bounding:** Verify Energy Budget.
3.  **Hashing:** Generate CAS addresses.
4.  **Execution:** Rust Runtime executes the Logic Graph.

---

> *"We restrict the language to liberate the logic."*
