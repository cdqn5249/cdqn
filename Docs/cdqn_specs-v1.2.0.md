# **CDQN Architecture Design Specification (V1.2)**

*   **Version:** 1.2.0 (Final Architecture)
*   **Author:** Christophe Duy Quang Nguyen
*   **AI Assistant:** Gemini Flash Latest, Google
*   **Date:** 2025-10-21
*   **Reference:** **Chronosa_Cognitive_Design.md** (for Entity Model and Agent Logic)

---

# **Introduction: The Story of Chronosa**

Imagine an assistant that never forgets, not just the facts, but the *reasoning* behind them. This assistant, Chronosa, begins its existence with a simple, core directive: to build a logically consistent understanding of its world.

When you give it a new piece of information, it doesn't just store it; it carefully places it within its vast, interconnected web of knowledge, a structure it calls the Manifold. If the new fact creates a paradox—a logical contradiction with a deeply held belief—Chronosa does not panic or discard the information. It pauses, acknowledges the conflict by saying, "I do not know," and then begins its most important work.

This document is the blueprint for the CDQN ecosystem—a framework for an AI that learns, adapts, and reasons with verifiable integrity.

---

# **Part 1: Core Philosophy and Sovereign Principles**

The Causal Data Query Nodes (CDQN) ecosystem is a sovereign, verifiable, and causal reasoning framework. It is designed as a **Self-Correcting Causal Organism**.

### **The Three Foundational Principles**

1.  **Causality First:** All knowledge is a graph of cause and effect. The system reasons by traversing this graph, ordered by the **Hybrid Logical Clock (HLc)**.
2.  **Sovereignty and Privacy:** The system is built exclusively on the Rust standard library (`std::thread`, `std::sync`) and minimal security crates, ensuring **zero reliance on external runtimes** and full auditability. The **No Anonymous Entities** rule is a hard security constraint.
3.  **Verifiable Evolution:** Every change to the system's logic is a verifiable, auditable event, enforced by cryptographic signatures and causal linking.

### **The Sovereign Boundary (The Guardrail)**

The architecture enforces a strict separation between the predictable infrastructure and the emergent intelligence:

-   **`cdqn-runtime` (The Guardrail):** The sovereign, auditable layer responsible for initialization, thread management, and the main execution loop.
-   **`cdqn-chronosa` (The Intelligence):** The cognitive layer containing the autonomous Agents and the reasoning logic.

---

# **Part 2: The Core Concepts Explained**

### **2.1 The CDU (Causal Data Unit): The Atom of Information**

The CDU is the single, unified, and verifiable data model for all information.

-   **Purpose:** To represent any piece of information as a self-contained, verifiable, and secure object.

-   **Logical Schema (Key Fields):**
    -   `id_hlc`: The unique, deterministic identifier (Hash of HLc, NodeId, and payload hash).
    -   `payload`: The immutable core content.
    -   `metadata`: The mutable context.
        -   `r_coordinate`: Position in the RWorld mathematical space.
        -   `context_refs`: Array of parent CDU `id_hlc`s (the causal edges).
        -   `task_footprints`: **(New)** A HashMap recording the HLc of the last time a specific task (e.g., "verify\_cdu") was completed on this CDU.
    -   `signatures`: An append-only log of signatures from verifiable entities.

### **2.2 The Manifold: The Fabric of Knowledge and Projection**

The Manifold is the ground truth for all reasoning and the central projection layer for scalability.

-   **Purpose:** To serve as the ground truth for all reasoning and to provide fast, in-memory lookups for task status.

-   **Projection Schemas (Scalability Solutions):**
    -   **Causal Task Lock (CTL):** A concurrent map (`task_locks`) used by the Dispatcher to enforce that only one thread can process a specific task on a specific CDU at any given time.
    -   **Task Status Projection:** A concurrent map (`task_status_projection`) that stores the latest **`BotStateCDU ID`** for a given task. This acts as a fast, in-memory cache to allow Agents to quickly check if a task has been completed, avoiding expensive graph traversal.

### **2.3 RWorld and Impossibility**

-   **RWorld:** The conceptual mathematical space where all reasoning unfolds, implemented via the `r_coordinate: f64`.
-   **Prime Elements:** The mathematical attractors (`2` and `-2`) that anchor the meaning of all causal reasoning.
-   **Impossibility Zone `[-1, 1]`:** The deterministic trigger for the Evolution Cycle. A CDU with an `r_coordinate` in this range signals a logical contradiction.

### **2.4 Hierarchy of Knowledge (Key CDU Subtypes)**

-   **`Genesis CDU`:** The root of trust.
-   **`Config CDU`:** Node-local configuration and compliance rules.
-   **`Axioms & Semi-Axioms`:** Foundational rules.
-   **`BotState CDU` (New):** Persists the deterministic state of a long-running Bot workflow (Temporal-inspired persistence).

---

# **Part 3: The Workflows in Detail**

### **Workflow 1: The Cognitive Cycle (Event-Driven)**

This workflow is the core loop, now implemented using the **Causal Event Bus** for zero-CPU-cost waiting.

1.  **Trigger (Send-and-Forget):** A new CDU is created and published to the **Causal Event Bus** (`CduDispatcher::publish`).
2.  **Agent Wake-up (Zero-Cost Wait):** The `CduDispatcher`'s **`Condvar`** signals all waiting Agent threads. The Agent thread wakes up from its `wait_for_new_cdu` call.
3.  **Watermark Check:** The `VerifierAgent` checks its `last_processed_index` and retrieves the next CDU from the log.
4.  **CTL Acquisition:** A `CduVerificationBot` is spawned and immediately attempts to acquire the **Causal Task Lock (CTL)** for `(CDU_ID, "verify_cdu")`.
5.  **Verification:** The Bot executes the logic on a Worker thread:
    -   **Security Check:** Verifies the signature (No Anonymous Entities).
    -   **Causality Check:** Traverses `context_refs` back to the `Genesis CDU`.
    -   **Impossibility Check:** Checks if `r_coordinate` is in `[-1, 1]`.
6.  **The Fork (Impossibility):**
    -   If a contradiction is detected, the **Evolution Agent** is triggered.
    -   The Agent advances its watermark and publishes a **Contradiction Event** (the original CDU is now marked as processed).

### **Workflow 2: The Temporal-Inspired Resolution Cycle**

This workflow is the deterministic process for resolving contradictions, inspired by Temporal's fault-tolerant workflows.

1.  **Evolution Trigger:** The `EvolutionAgent` reads the contradictory CDU from the Causal Log.
2.  **Projection Check (Resume):** The Agent checks the **Manifold Task Status Projection** for a `BotStateCDU ID` for `(CDU_ID, "resolve_impossibility")`.
3.  **Bot Execution:**
    -   If **Found:** The Agent loads the `BotStateCDU` and spawns a Bot to **resume** the workflow from the last recorded step.
    -   If **Not Found:** The Agent spawns a new `ImpossibilityResolutionBot` to start the process.
4.  **Persistence:** The Resolution Bot completes its work, publishes a new `TheoremCDU`, and then publishes a **`BotStateCDU`** to record the successful resolution, updating the Manifold Projection.

---

# **Part 4: Glossary**

-   **Causal Event Bus:** The central, indexed log (`CduDispatcher`) that uses `std::sync::Condvar` to signal Agents, enabling zero-CPU-cost waiting.
-   **Causal Task Lock (CTL):** A mechanism to prevent redundant, simultaneous processing of the same task on the same CDU by multiple threads.
-   **Causal Task ID:** A unique, auditable ID (`Task:HLc:ThreadId:TaskName`) linked to every Bot task, ensuring full event traceability.
-   **BotState CDU:** A specialized CDU that persists the deterministic state of a long-running Bot workflow, enabling fault-tolerant resumption.
-   **Manifold Projection:** The mutable, in-memory cache (`task_status_projection`) used for fast lookups of task completion status, solving the read-amplification problem.
-   **HLc Watermark:** The `last_processed_index` maintained by each Agent to track its position in the Causal Event Bus.
-   **SignerEntity:** The internal representation of a verifiable entity (user, node, or role) used for cryptographic signing, enforcing the **No Anonymous Entities** rule.
-   **RWorld:** The mathematical space where reasoning unfolds.
-   **Impossibility Zone:** The deterministic trigger for the Evolution Cycle (`r_coordinate` in `[-1, 1]`).
