# Checkpoint 0003: The Foundational Learning Architecture

- **Date:** October 6, 2025
- **Author:** Christophe Duy Quang Nguyen
- **Vibe coding engine:** Gemini 2.5 Pro, Google
- **Status:** Core Learning Loop & CTD v1 Prototype Implemented and Stabilized

---

## 1. Project Goal (Recap from CHECK-0002)

The `cdqn` project aims to create a **Sovereign AI Companion**, named **Chronosa**. This agent is founded on the principles of Verifiable Experience, Transparent Reasoning, and user ownership. The core objective is to design Chronosa to learn from its mistakes and successes, becoming a truly intelligent and autonomous partner.

## 2. Progress from CHECK-0002

This phase was a critical and challenging evolution of the system. We moved from a simple concurrent reasoning engine to a fully autonomous, self-correcting learning architecture. The work involved three major thrusts:

1.  **Implementing the Foundational Learning Loop:** We introduced the core vocabulary for learned knowledge (`Theorem`, `Constraint`) and the `RefinementEngine` to discover them autonomously.
2.  **Implementing the Mathematical Engine (CIN v1):** We upgraded the system's understanding of knowledge from discrete concepts to nuanced, mathematical ones by vectorizing `PrimeElement`s and implementing a `Similarity Engine`.
3.  **Implementing the CTD v1 Prototype:** We introduced the `CausalMode` vocabulary and a `DecompositionStrategy` to prove the viability of the Causal Tensor Decomposition architecture.

Most importantly, this phase involved a deep and difficult debugging process that uncovered and fixed fundamental architectural flaws, resulting in a vastly more robust, stable, and verifiable system.

## 3. Core Architectural Evolution: The Final Design

The architecture has been significantly upgraded and stabilized.

-   **Vector-Based Knowledge Representation (CIN v1):** The `representation` field of a `PrimeElement` is now a vector (`Vec<f64>`), allowing concepts to be positioned as points in a high-dimensional "meaning-space."

-   **The Similarity Engine:** A `calculate_euclidean_distance` function now allows Chronosa to measure the semantic "closeness" of two concepts, enabling nuanced, context-aware reasoning.

-   **Intelligent Assimilation:** The `RefinementEngine` now uses the Similarity Engine to check for semantically similar knowledge before creating new `Constraint`s or `Theorem`s, preventing log bloat and building a more abstract knowledge graph.

-   **The Strategy Pattern:** The monolithic `ReasoningProjector` has been successfully refactored into a clean, modular pipeline of discrete `ReasoningStrategy` components (`Decomposition`, `Theorem`, `Constraint`, `AxiomEvaluation`).

-   **Explicit Shutdown Signaling:** The entire concurrent architecture was refactored to use an explicit `EngineInput::Shutdown` signal, eliminating deadlocks and ensuring a graceful, predictable shutdown sequence.

-   **Robust Type Parsing:** The core `Cdu::extract_payload` function was rewritten to use robust subtype matching, eliminating a critical "type confusion" vulnerability that could lead to memory allocation failures.

## 4. Workflows

### 4.1. The Modular Reasoning Workflow

*   **1. Trigger:** An `EngineInput::Cdu` is received by the `Engine`.
*   **2. Task Spawning:** The `Engine` spawns a new, short-lived "Engine-Task" thread to process the CDU, adding its handle to a tracking list.
*   **3. Preparation:** Inside the task, a `KnowledgeBase` is created, snapshotting all current knowledge.
*   **4. Strategy Pipeline Execution:**
    *   A `ReasoningContext` is created.
    *   The `DecompositionStrategy` runs, potentially creating new `CausalMode` CDUs.
    *   The `TheoremStrategy` runs, checking for both mode-based (CTD) and premise-based shortcuts. If a conclusive theorem is found, the pipeline terminates early.
    *   The `ConstraintStrategy` runs, filtering candidate axioms using the Similarity Engine.
    *   The `AxiomEvaluationStrategy` runs on the final list of allowed axioms.
*   **5. State Evolution:** The task gathers all newly generated CDUs, persists them to the log, sends commands to the `Executor`, and evolves the shared state. The task then terminates.

### 4.2. The Graceful Shutdown Workflow

*   **1. Signal:** The `main` thread sends an `EngineInput::Shutdown` message.
*   **2. Engine Termination:** The `Engine`'s main loop receives the signal and breaks. It then waits for all "Engine-Task" handles in its tracking list to be `.join()`ed, ensuring all work is complete. The `Engine` thread then terminates.
*   **3. Cascading Shutdown:**
    *   As the `Engine` is dropped, its `command_sender` is dropped. This closes the command channel, causing the `Executor`'s loop to break and its thread to terminate.
    *   The `RefinementEngine`'s "heartbeat" `send` call eventually fails because the `Engine`'s receiver is gone. This causes its loop to break and its thread to terminate.
*   **4. Final Join:** The `main` thread, having successfully joined the `Engine`, can now successfully join the `Executor` and `RefinementEngine` handles, leading to a clean exit.

## 5. Detailed Implementation Changes

-   **`src/cdu.rs`:** Expanded `CduPayload` with `CausalMode`. Critically refactored `extract_payload` to use robust subtype matching, fixing a major memory allocation vulnerability.
-   **`src/payloads/*` (New Module):** All payload structs (`Theorem`, `Constraint`, `CausalMode`) were moved into this new, modular directory.
-   **`src/reasoning/prime_element.rs`:** Upgraded `representation` from `f64` to `Vec<f64>`.
-   **`src/reasoning/strategy.rs`:** Implemented the Strategy pattern, the `calculate_euclidean_distance` function, and the new `DecompositionStrategy`.
-   **`src/reasoning/reasoning_projector.rs`:** Refactored into a simple orchestrator for the strategy pipeline.
-   **`src/refinement.rs`:** Upgraded with intelligent assimilation logic and a robust "heartbeat" shutdown mechanism.
-   **`src/engine.rs`:** Introduced the `EngineInput` enum for explicit shutdown signaling. Implemented the "responsible parent" pattern, where the `Engine` now tracks and joins its child task threads before terminating.
-   **`src/executor.rs`:** Instrumented with verbose logging.
-   **`src/main.rs`:** Rewritten to test and demonstrate the final, stable, and deadlock-free architecture.

## 6. Successes

-   **Deadlock Resolution:** We successfully diagnosed and fixed a series of complex, interacting deadlocks, resulting in a robust and provably correct concurrent architecture.
-   **Critical Bug Fix:** We identified and fixed a catastrophic "type confusion" vulnerability in `cdu.rs` that could cause memory allocation failures.
-   **CIN v1 and CTD v1 Prototypes Achieved:** The core goals were met. The system now has a functional mathematical foundation for CIN and a working prototype of the CTD workflow.
-   **Full Learning Loop Validated:** The final demo proves that Chronosa can learn from success and failure, do so efficiently, and shut down cleanly.
-   **Successful Major Refactoring:** The entire data model and reasoning engine were successfully refactored for modularity and clarity while maintaining stability.

## 7. Failures and Resolutions

-   **Failure: Catastrophic Deadlocks:** The most critical failure of this phase. The initial design for concurrency and shutdown was fundamentally flawed, leading to multiple, hard-to-diagnose deadlocks. Initial fixes were incorrect "patches" that addressed symptoms rather than the root cause.
    -   **Resolution:** A systematic, logging-driven debugging process revealed the true architectural flaws: first, a thread self-deadlock on the `RwLock`, and finally, a flawed lifecycle in the `RefinementEngine`. The final solution involved implementing an explicit `Shutdown` signal and a robust "heartbeat" for the `RefinementEngine`, leading to a provably correct shutdown cascade.
-   **Failure: Infinite Feedback Loop:** A logical flaw in the `TheoremStrategy` (treating an empty premise set as always true) caused a runaway infinite loop that was not caught by unit tests.
    -   **Resolution:** The logic was corrected to require that standard theorems must have non-empty premises. This highlighted the need for more integration-style tests.
-   **Failure: Memory Allocation Crash:** A "type confusion" vulnerability in `cdu.rs` was discovered that could cause the system to attempt to allocate terabytes of memory and crash.
    -   **Resolution:** The `extract_payload` function was completely rewritten to use robust subtype matching instead of brittle `string.contains()` checks, permanently fixing the vulnerability.

## 8. Current State of the System

As of this checkpoint, the `cdqn` system is a stable, robust, and verifiable learning agent. It is feature-complete for this phase and serves as a solid foundation for further development. The system is no longer just a prototype; it is a resilient piece of software.

## 9. Next Steps (Phase 5 - Full CTD Implementation)

The CIN v1 foundation is stable. The next phase will focus on evolving the CTD prototype into a fully-fledged, powerful engine for generalization.

-   **Task: Implement a Real Decomposer Engine.**
    -   The current `DecompositionStrategy` is a placeholder. We will research and implement a real mathematical algorithm (e.g., inspired by PARAFAC, Tucker decomposition, or modern neural network techniques) to take a high-level CDU (like a user's natural language request) and generate a meaningful set of `CausalMode` vectors.

-   **Task: Implement the CTD-based Theorem Retriever.**
    -   We will create a new `ReasoningStrategy` or upgrade the `TheoremStrategy`. Its job will be to take the set of `CausalMode`s generated by the decomposer and use them to mathematically **reconstruct** the most relevant `Theorem` from the `KnowledgeBase`. This is the core of CTD—finding knowledge by relevance and similarity, not just by an exact match of premises. This will be the key to unlocking Chronosa's ability to generalize its knowledge to new, unseen situations.
