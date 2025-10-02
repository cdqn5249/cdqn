# Checkpoint 0001: Foundation and Architecture

- **Date:** October 2, 2025
- **Author:** Christophe Duy Quang Nguyen
- **Vibe coding engine:** Gemini 2.5 Pro, Google
- **Status:** Stable Baseline Established

---

## 1. Project Goal

The `cdqn` project aims to create a **Sovereign AI Companion**, named **Chronosa**. This agent is founded on the principles of Verifiable Experience, Transparent Reasoning, and user ownership ("Your AI, your data, your identity").

The core objective is to design Chronosa to learn from its mistakes, fully autonomous at helping you based on your intents and your datas.

## 2. Core Concepts

The entire architecture is built upon a single, unified data model:

-   **The CDU (Causal Data Unit):** The CDU is the fundamental, atomic unit of the system. It represents every piece of information: observations, commands, results, and internal thoughts. Its design (immutable payload, content-addressed name, Hybrid Logical Clock) makes it the bedrock of Chronosa's verifiable and causal memory.

## 3. Current Architecture & File Structure

The project has evolved into a robust, asynchronous, event-sourced system. The logic is cleanly separated into modular components, each with a distinct responsibility.

* **Project Homepage**: https://cdqn5249.github.io/cdqn/

-   **`/src/main.rs`**: The main application entry point. It is responsible for composing and launching the system components (Engine, Executor, Projector).
-   **`/src/lib.rs`**: The library crate root, which declares and exposes all the public modules of the `cdqn` framework.
-   **`/src/cdu.rs`**: Defines the `Cdu` and `CduMetadata` structs, the universal data model for the entire system.
-   **`/src/hlc.rs`**: Implements the `Hlc` (Hybrid Logical Clock) for causally ordered, distributed-friendly timestamping.
-   **`/src/state.rs`**: Defines the `ChronosaState` struct, the pure in-memory projection of the event log, and the `evolve` function for state reduction.
-   **`/src/engine.rs`**: Implements the core "CDQN Native Engine". This single-threaded component owns the state, processes incoming events, and uses the Projector to make decisions.
-   **`/src/projector.rs`**: Defines the `Projector` trait and the `RuleBasedProjector` implementation. This is the deterministic "brain" of the agent, containing the pure functional logic.
-   **`/src/executor.rs`**: Implements the `Executor`, which runs on a background thread. It listens for command CDUs and handles all non-deterministic, side-effectful tasks.
-   **`/src/codec.rs`**: Implements a custom, dependency-free binary `Encode`/`Decode` trait and logic for serializing CDUs.
-   **`/src/storage.rs`**: Manages all disk I/O. It handles the append-only Write-Ahead Log (`.cdqn` file) and the rehydration of events on startup.

## 4. Development Journey & Resolutions

The path to this stable baseline involved overcoming several critical technical and design challenges. Our rigorous, test-driven approach, enforced by the CI/CD pipeline, was instrumental in arriving at the current robust architecture.

### Challenges Encountered

-   **Bootstrapping a Professional CI/CD Pipeline:** The initial setup failed due to the repository not yet being a valid Rust project. This was resolved by correctly initializing the project with `Cargo.toml`.
-   **Optimizing CI/CD Performance:** Initial build times were over 3 minutes. We identified that compiling tools like `cargo-audit` from scratch on every run was the bottleneck. This was solved by adding caching to all CI jobs, dramatically reducing feedback time to under 90 seconds.
-   **Enforcing Code Quality:** The pipeline consistently caught numerous issues, including formatting errors, unused imports, and violations of Rust API conventions (e.g., missing `Default` or `is_empty` implementations), forcing a high standard of code quality.
-   **Mastering Rust's Ownership Model:** A major hurdle was a series of borrow checker errors (`E0499`, `E0505`) when designing the API. An initial "mutable" design and a subsequent "cloning" design were rejected as they violated our core principles.
-   **Ensuring Data Durability and Consistency:** You raised critical concerns about data loss during disorderly shutdowns, which led to a deep architectural review of our persistence strategy.

### Key Accomplishments & Architectural Decisions

-   **Event Sourcing Architecture:** We have successfully designed and implemented a professional-grade Event Sourcing model. This is the single most important architectural decision, providing a foundation for speed, security, and maintainability.
-   **Write-Ahead Log (WAL):** The single source of truth is an append-only log of CDU events (`.cdqn` file). This design ensures maximum write performance and data integrity. The design is resilient to disorderly shutdowns through a journaling-inspired commit process.
-   **CDQN Native Engine:** We built a custom, asynchronous, non-blocking engine from the ground up using standard Rust libraries (`std::thread`, `std::sync::mpsc`), avoiding heavy dependencies like `tokio`. This "Entity Model" consists of:
    -   A single-threaded **Engine** that manages the state.
    -   A background **Executor** thread for handling side effects.
-   **Pure Functional Core:** The agent's intelligence is now fully decoupled from its actions. The **Projector** is a pure function that deterministically produces command CDUs based on the current state and input, making the core logic transparent and easily testable.
-   **Extensible Logic:** The agent's "brain" has been formalized into a `RuleBasedProjector`, allowing new behaviors to be added by simply defining new rules, without altering the core engine.

## 5. Next Steps: Phase 2

The foundational architecture is complete, verified, and robust. The system can now reliably perceive, decide, act, and remember.

The next phase of the project will focus on the core of Chronosa's intelligence:

-   **Design how Chronosa will think:** We will move beyond the simple, stateless `RuleBasedProjector`. The next step is to design a more sophisticated reasoning engine. This will involve exploring how Chronosa can query its own historical log, identify patterns, form hypotheses, and make decisions based on the entirety of its verifiable experience.
-   **First Tests of Reasoning:** We will implement the first version of this reasoning engine and create a suite of tests to validate its ability to make simple, stateful, history-aware decisions.
