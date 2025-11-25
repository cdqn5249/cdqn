# 04-LANG: The Instruction Set (cdqnLang)

* **File:** `docs/04-LANG.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.2 (Bounded Execution Update)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **The Logic of the Virtual Chip. Programming with Physics, not Memory.**

---

## 1. Philosophy: The Geometrization of Code

Traditional programming languages (C, Python, Rust) manage **Linear Memory** (Stack/Heap). The programmer must worry about allocation, pointers, and garbage collection.

**cdqnLang** eliminates this burden. It is a **Domain Specific Language (DSL)** for orchestrating Agents, Logic, and Data Flows on a **Topological Memory** model.

*   **Immutability:** Data is never overwritten. It is **Folded** (Compressed) or **Unfolded** (Accessed).
*   **Bounded Execution:** It is a **Total Functional Language**. It forbids infinite loops and unbound recursion to guarantee system stability and predictable energy costs.

---

## 2. The Layered Stack

We address the physical constraints of binary hardware by splitting concerns into a fractal stack.

### Level 0: The Hybrid Addressing (Storage)
To achieve high performance on consumer hardware, we separate Storage from Logic.
*   **Physical Layer (Storage):** Uses **Cryptographic Hashes (Blake3)**.
    *   *Mechanism:* `Store(Data)` $\to$ `Hash` $\to$ `mmap` location. Uses Content-Addressed Storage (CAS) via Hash Maps (Blake3). Avoids sequential scanning by using Hash-based direct addressing (Pointer Swizzling).
    *   *Performance:* $O(1)$ lookup speed.
*   **Logical Layer (Identity):** Uses **Prime Elements**.
    *   *Mechanism:* Types and Concepts are defined by Prime Factorization.
    *   *Performance:* Semantic validation is done via integer math (divisibility checks), not string parsing.

### Level 1: The Kernel (Rust Bootstrap)
*   **Role:** The Mother Mold.
*   **Function:** The compiler is written in **Rust**. It translates `cdqnLang` syntax into the optimized ECS (Entity Component System) schedule used by the Runtime.

### Level 2: The Logic Layer (Bounded Quantales)
*   **Role:** The Safety Switch.
*   **Mechanism:** The Compiler calculates the **Worst-Case Energy Cost** (`cdqnStar`) of every function *before* it runs.
*   **The Guarantee:** If a script exceeds the Node's energy limit or contains a potential infinite loop, **It Will Not Compile.** This makes the system crash-proof by design.

### Level 3: The Interface (Deck Building)
*   **Role:** The Developer Experience (DX).
*   **Metaphor:**
    *   **Function** $\to$ **Card**.
    *   **Module** $\to$ **Deck**.
    *   **Scope** $\to$ **Table**.
*   **Safety:** The Prime Math ensures Type Safety. You cannot connect a "Fire Card" ($P_{Fire}$) to a "Water Slot" ($P_{Water}$) unless a valid Functor exists to bridge them.

---

## 3. Syntax Specification (Draft)

The syntax fuses Functional Programming patterns with Resource Physics.

### 3.1 Defining a Card (The Logic Object)

```cdqn
CARD #Analyze_Sentiment {
    // Identity: Logical Type defined by Primes
    PRIME: [Analysis * Text * Emotion]
    WORLD: SocialWorld

    // Physics: Bounded Execution Constraints
    // The compiler verifies this function cannot exceed these limits
    COST: 5 Star
    TIMEOUT: 500ms 

    // Logic: Deterministic Script
    EFFECT: |input| => {
        // ECHO is a Vector Search (Approximate Match)
        let resonance = ECHO(input, target: #Positive_Vibe);
        
        // Control Flow is strict (Match/Switch)
        match resonance {
            case > 0.8 -> {
                PULSE(High); // Signal excitement
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

### 3.2 The Learning Loop (Crystallization)
Used to turn "Vague AI Output" (Cloud) into "Strict Local Logic" (Rust).

```cdqn
// The Parasite Interface
LEARN from #Google_API {
    INPUT: "Extract date from: Meeting at 5pm tomorrow"
    
    // The logic returned by the Cloud is compiled into a Regex/Rule
    // Neural weights are discarded; only the Logic remains.
    CRYSTALLIZE into CARD #Extract_Date
}
```

---

## 4. The Compiler Strategy

We are building a **Just-In-Time (JIT) Logic Simulator**.

1.  **Static Analysis:** Check Prime compatibility (Type Safety).
2.  **Cost Bounding:** Verify the Energy Budget (Halting Safety).
3.  **Hashing:** Generate Content IDs for storage mapping.
4.  **Execution:** The Rust Runtime loads the manifest and executes the graph on the Virtual Chip.

---

> *"We restrict the language to liberate the logic."*
