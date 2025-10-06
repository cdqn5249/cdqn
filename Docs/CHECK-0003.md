# Checkpoint 0003: The Mathematical Learning Engine (CIN v1)

- **Date:** October 6, 2025
- **Author:** Christophe Duy Quang Nguyen
- **Vibe coding engine:** Gemini 2.5 Pro, Google
- **Status:** Mathematical Foundation & Intelligent Assimilation Implemented

---

## 1. Project Goal (Recap from CHECK-0002)

The `cdqn` project aims to create a **Sovereign AI Companion**, named **Chronosa**. This agent is founded on the principles of Verifiable Experience, Transparent Reasoning, and user ownership ("Your AI, your data, your identity"). The core objective is to design Chronosa to learn from its mistakes and successes, becoming a truly intelligent and autonomous partner.

## 2. Progress from CHECK-0002

This phase represents the most significant evolution of Chronosa's intelligence to date. We moved beyond a simple, rule-based reasoning system to a fully autonomous, self-correcting, and self-improving learning architecture. The work in this phase can be divided into two major thrusts:

1.  **The Foundational Learning Architecture:** We implemented the core vocabulary and mechanisms for learning, including `Constraint`s (learning from failure) and `Theorem`s (learning from success), and the `RefinementEngine` to discover them autonomously.
2.  **The Mathematical Engine (CIN v1):** We upgraded the system's understanding of knowledge from discrete, logical concepts to nuanced, mathematical ones. This involved representing concepts as vectors in a multi-dimensional space and implementing the first version of the Similarity Engine.

The system is no longer just a concurrent reasoner; it is a nascent mathematical intelligence capable of nuanced, context-aware learning.

## 3. Core Architectural Evolution: The CIN v1 Design

The architecture has been significantly upgraded to support true learning and mathematical reasoning.

-   **Vector-Based Knowledge Representation:** The most fundamental change. The `representation` field of a `PrimeElement` was upgraded from a scalar `f64` to a vector `Vec<f64>`. This allows concepts to be positioned as points in a high-dimensional "meaning-space," which is the prerequisite for all advanced mathematical reasoning.

-   **The Similarity Engine:** We have implemented the first version of this crucial component. It is currently realized as a `calculate_euclidean_distance` function. This allows Chronosa to measure the semantic "closeness" of two concepts, moving beyond simple string matching to a more nuanced, mathematical understanding of context.

-   **Intelligent Assimilation:** The `RefinementEngine` has been upgraded to use the Similarity Engine. Before creating a new `Constraint` or `Theorem`, it now checks for existing, semantically similar knowledge. If a near-duplicate is found (within a defined `epsilon`), the new discovery is discarded. This is the core mechanism that prevents log bloat and allows Chronosa to build an abstract, efficient knowledge graph over time.

-   **The Strategy Pattern Refactoring:** The monolithic `ReasoningProjector` was successfully refactored into a clean, modular pipeline, dramatically improving maintainability and clarity. The new components are:
    -   **`KnowledgeBase`:** A struct that creates a snapshot of all knowledge for a single reasoning cycle.
    -   **`ReasoningStrategy` Trait:** A common interface for all pieces of the reasoning process.
    -   **Concrete Strategies:** `TheoremStrategy`, `ConstraintStrategy`, and `AxiomEvaluationStrategy` are now independent, testable components.
    -   **The Orchestrator:** The `ReasoningProjector` is now a simple, clean orchestrator that executes these strategies in a defined order.

## 4. Schemas and Workflows

### 4.1. The Modular Reasoning Workflow

The reasoning process is a pipeline that can be terminated early for efficiency.

*   **1. Trigger:** An Input CDU is received.
*   **2. Preparation:** A `KnowledgeBase` is created, snapshotting all current knowledge from the state.
*   **3. Strategy 1: Theorem Execution:**
    *   The `TheoremStrategy` is executed.
    *   **If a conclusive `Theorem` is found whose premises are met:**
        *   A command is generated directly from the `Theorem`'s conclusion.
        *   The pipeline terminates early, and the command is returned.
*   **4. Strategy 2: Constraint Filtering:**
    *   **If no conclusive theorem was found:**
        *   The `ConstraintStrategy` is executed.
        *   It uses the `Similarity Engine` to compare the input context to the contexts of known `Constraint`s.
        *   Candidate axioms that are inhibited by a relevant constraint are filtered out.
*   **5. Strategy 3: Axiom Evaluation:**
    *   The `AxiomEvaluationStrategy` is executed on the final list of allowed axioms.
    *   Commands are generated from any axioms whose conditions are met.
*   **6. Conclusion:** The final list of generated commands is returned.

### 4.2. The Autonomous Learning Workflow

The `RefinementEngine` runs in the background, creating a continuous, asynchronous learning loop.

*   **Main Engine Loop:**
    *   Continuously processes incoming CDUs (observations, commands, feedback, etc.).
    *   All processed CDUs are written to the persistent, immutable CDU Log.
*   **Background Refinement Loop:**
    *   Periodically wakes up to perform an analysis cycle.
    *   Reads the entire CDU Log to build an up-to-date `KnowledgeBase`.
    *   **Identifies patterns:**
        *   **Failure Patterns:** Scans for events like a command that is causally followed by a `feedback.reputation.negative` CDU.
        *   **Success Patterns:** Scans for events like a command that is causally followed by a `feedback.reputation.positive` CDU.
    *   **If a new, non-redundant pattern is found:**
        *   The engine uses the `Similarity Engine` to check if the discovered knowledge is genuinely novel or just a minor variation of an existing `Constraint` or `Theorem`.
        *   If it is novel, a new `Constraint` (for failure) or `Theorem` (for success) CDU is created.
    *   The new knowledge CDU is sent back to the Main Engine's input channel.
    *   The Main Engine processes and persists the new knowledge, making it available for all future reasoning cycles.

## 5. Detailed Implementation Changes

-   **`src/cdu.rs`:** Expanded the `CduPayload` enum to include `Theorem` and `Constraint` structs.
-   **`src/reasoning/prime_element.rs`:** Upgraded `representation` from `f64` to `Vec<f64>`.
-   **`src/reasoning/mod.rs`:** Declared the new `knowledge_base` and `strategy` modules.
-   **`src/reasoning/knowledge_base.rs` (New):** Encapsulates knowledge extraction.
-   **`src/reasoning/strategy.rs` (New):** Implements the Strategy pattern and the `calculate_euclidean_distance` function.
-   **`src/reasoning/reasoning_projector.rs` (Refactored):** Rewritten as a simple orchestrator.
-   **`src/refinement.rs` (New & Upgraded):** Created to house the `RefinementEngine` and later upgraded with intelligent assimilation logic.
-   **`src/lib.rs`, `src/engine.rs`, `src/main.rs`:** All updated to integrate the new components and demonstrate the full learning and assimilation cycle.

## 6. Successes

-   **CIN v1 Achieved:** The core goal was met. The system now has a functional, mathematical foundation for Causal Interpolation Networks.
-   **Full Learning Loop Validated:** The final demo in `main.rs` provides a concrete, end-to-end proof that Chronosa can learn from both success and failure, and can do so efficiently by avoiding redundancy.
-   **Successful Foundational Refactoring:** We successfully upgraded the most fundamental data structure (`PrimeElement`) and refactored the entire reasoning engine without destabilizing the system, as proven by the green test suite.
-   **Improved Modularity and Maintainability:** The Strategy pattern has made the reasoning engine significantly cleaner and easier to extend in the future.

## 7. Failures and Resolutions

-   **Failure: Compiler Errors (Ownership, Mismatched Types, Unused Imports):** The development process was a continuous cycle of making a change and then methodically fixing the resulting cascade of compiler errors. This was particularly acute during the `f64` -> `Vec<f64>` refactoring.
    -   **Resolution:** Each error was treated as a guide from the compiler. We systematically worked through each file, correcting types and fixing logic until the entire project compiled cleanly.
-   **Failure: Test Failure due to Logic Bug:** The refactoring introduced a subtle bug in the `ConstraintStrategy`'s context-parsing logic, which was only caught by a failing test.
    -   **Resolution:** The test failure pinpointed the exact problem, which was quickly fixed. This validated the critical importance of a comprehensive and specific test suite.
-   **Failure: Persistent Formatting Errors:** The Vibe Coding Engine repeatedly produced code that failed `cargo fmt` checks.
    -   **Resolution:** Each failure was manually corrected, highlighting the need for strict adherence to project-wide quality standards.

## 8. Current State of the System

As of this checkpoint, the `cdqn` system is a functional CIN v1 engine.
-   **What it can do:**
    -   Represent concepts as vectors in a multi-dimensional space.
    -   Calculate the semantic distance between concepts.
    -   Use this distance to apply learned `Constraint`s in a nuanced, context-aware manner.
    -   Autonomously discover `Constraint`s and `Theorem`s from its experience.
    -   Intelligently assimilate new knowledge, avoiding the creation of redundant information.
-   **The system is stable, test-covered, and ready for the next major architectural challenge.**

## 9. Next Steps (Phase 5 - Causal Tensor Decomposition)

The CIN v1 foundation is stable. The next phase will focus on solving the problem of **scalability** by implementing the second major pillar of the advanced design: **Causal Tensor Decomposition (CTD)**.

-   **Task: Design and Implement `*.causal.mode.*` CDUs.**
    -   We must formalize the schema for these crucial components. A "mode" CDU will represent a decomposed vector of a high-level concept (e.g., a user's intent, the state of a particular world).

-   **Task: Implement the Decomposer Engine.**
    -   Create a new engine component or strategy that can take a high-level input (like a user request CDU) and generate the corresponding set of mode CDUs. This will likely involve algorithms inspired by linear algebra (e.g., Tucker or PARAFAC decomposition).

-   **Task: Refactor the `TheoremStrategy` to use CTD.**
    -   Upgrade the theorem lookup from its current direct pattern match (`premises_set.is_subset(...)`) to a CTD-based relevance search. The engine will decompose the current state into its modes and use those modes to mathematically reconstruct the most relevant `Theorem` from its knowledge base. This will allow Chronosa to find and apply relevant wisdom even when the situation is not an exact match to its past experiences, unlocking a powerful new level of generalization.

## 10. Glossary

-   **CDU (Causal Data Unit):** The atomic, immutable, and verifiable unit of data for the entire system. Every fact, action, result, and piece of learned knowledge is a CDU.
-   **Chronosa:** The name of the Sovereign AI Companion being built by the `cdqn` project.
-   **World:** A mathematical space (a set of real numbers) that provides a grounded context for concepts. All knowledge exists within a World (e.g., `uWorld`, `EnWorld`).
-   **Prime Element:** A node in the knowledge graph representing an irreducible concept within a World. Its `representation` is a vector (`Vec<f64>`) that positions it within that World's meaning-space.
-   **Semi-Axiom:** A rule or causal link that is valid only within a specific World.
-   **Axiom:** A Semi-Axiom that has been proven valid across multiple Worlds, acting as a bridge for cross-world reasoning.
-   **Theorem:** A cognitive shortcut representing a proven, successful reasoning path, discovered by the `RefinementEngine`. It allows the system to become more efficient.
-   **Constraint:** An emergent guardrail. A piece of learned knowledge, discovered by the `RefinementEngine`, that inhibits a specific reasoning path in a specific context to prevent repeating mistakes.
-   **Refinement Engine:** The autonomous background component that analyzes the historical log to discover new `Theorem`s and `Constraint`s, enabling the system to learn.
-   **Reasoning Projector:** The main "brain" component that orchestrates the reasoning pipeline by executing a series of `ReasoningStrategy` modules.
-   **Strategy Pattern:** The software design pattern used to refactor the `ReasoningProjector` into a modular pipeline of interchangeable components.
-   **KnowledgeBase:** An in-memory snapshot of all knowledge (Prime Elements, Theorems, etc.) extracted from the state for a single, atomic reasoning cycle.
-   **Similarity Engine:** The component responsible for calculating the semantic distance between concepts, currently using Euclidean distance on their vector representations. This is the core of CIN.
-   **CIN (Causal Interpolation Networks):** The architectural goal for Chronosa's reasoning, where intelligence emerges from interpolating the strength and relevance of causal links in a mathematical, context-aware way.
-   **CTD (Causal Tensor Decomposition):** The architectural goal for achieving scalable and generalized knowledge retrieval by decomposing high-dimensional problems (e.g., `Intent x State x Axiom`) into lower-dimensional modes.
