# 04-LANG: The Instruction Set (cdqnLang)

* **File:** `docs/04-LANG.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.0 (Initial Architecture)
* **Date:** November 25, 2025
* **Author:** Christophe Duy Quang Nguyen

> **The Logic of the Virtual Chip. Programming with Physics, not Memory.**

---

## 1. Philosophy: The Geometrization of Code

Traditional programming languages (C, Python, Rust) manage **Linear Memory**. The programmer must worry about stack, heap, pointers, and garbage collection.

**cdqnLang** eliminates this burden. It operates on **Topological Memory**.
*   **No Addresses:** You do not store data at `0xAF40`. You store data at `Prime(User) * Prime(ID)`.
*   **No Allocation:** You do not `malloc`. You **Fold** (Compress) or **Unfold** (Access).
*   **No Loops:** You do not write `while(true)`. You define a **Pulse** (Frequency).

It is a **Declarative Language** for the CDQN Virtual Chip. You define the *State* and the *Rules*, and the Physics Engine handles the execution.

---

## 2. The Layered Stack

`cdqnLang` is omnipresent, existing at every layer of the system to ensure coherence from the metal up to the user.

### Level 0: The Geometrization (LLVM IR)
*   **Role:** The Translator.
*   **Mechanism:** Maps binary hardware operations to Lattice Coordinates.
*   **The Trick:** It uses **Content-Addressable Memory (Hashing)**. Every variable is a Hash. If the value changes, the address changes (Immutability). This removes the class of bugs related to "Memory Corruption."

### Level 1: The Rust Bootstrap (The Kernel)
*   **Role:** The Mother Mold.
*   **Current State:** The compiler is written in Rust for performance and safety.
*   **Future State:** Once mature, `cdqnLang` will become self-hosting (written in itself).

### Level 2: The Logic Layer (Quantale Constraints)
*   **Role:** The Safety Switch.
*   **Mechanism:** The Compiler calculates the **Energy Cost** (`cdqnStar`) of every function *before* it runs.
*   **Benefit:** It is impossible to write code that crashes the node via resource exhaustion. If a script is too expensive for the hardware, the compiler rejects it with: `Error: Insufficient Pulse Energy.`

### Level 3: The Interface (Deck Building)
*   **Role:** The User Experience.
*   **Metaphor:**
    *   **Function** $\to$ **Card**.
    *   **Library** $\to$ **Deck**.
    *   **Program** $\to$ **Game State**.
*   **Usage:** The user combines Cards to create effects. The language ensures that only "Compatible Primes" (Valid Types) can be connected.

---

## 3. Syntax Specification (Draft)

The syntax fuses **Functional Programming** with **Number Theory**.

### 3.1 Defining a Card (The Object)
A Card is a container for Identity, Cost, and Logic.

```cdqn
CARD #Analyze_Sentiment {
    // Identity: Defined by Prime Elements
    PRIME: [Analysis * Text * Emotion]
    WORLD: SocialWorld

    // Physics: The Cost to run this logic
    COST: 5 Star
    PULSE: Instant // Runs once immediately

    // Logic: The deterministic script (No LLM guessing)
    EFFECT: |input| => {
        // 1. Vector Search (Echo)
        let resonance = ECHO(input, target: #Positive_Vibe);
        
        // 2. Quantale Check
        if resonance > 0.8 {
            return #Happy;
        } else {
            return #Neutral;
        }
    }
}
```

### 3.2 The Feedback Loop (The Training)
`cdqnLang` supports **"Crystallization"**—saving the result of a computation as a new hardcoded rule.

```cdqn
// Learning Mode
LEARN from #Google_API {
    INPUT: "Extract date from: Meeting at 5pm tomorrow"
    
    // The logic returned by the Cloud is saved as a local Regex
    CRYSTALLIZE into CARD #Extract_Date
}
```

---

## 4. The Compiler Strategy

We are building a **Just-In-Time (JIT) Logic Simulator**.

1.  **Input:** The User's Deck (`.cdqn` file).
2.  **Analysis:** The Compiler checks the **Topology** (Are the connections valid?) and the **Energy** (Can the hardware afford this?).
3.  **Compilation:** It generates a **QIR (Quantale Intermediate Representation)**—a highly compressed binary manifest.
4.  **Execution:** The Rust Runtime loads the QIR and "plays" the cards in the Lattice.

---

> *"We do not write code to control the machine. We write rules to structure the chaos."*
