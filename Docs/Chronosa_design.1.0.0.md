# **Chronosa Cognitive Design Specification (V1.0)**

*   **Version:** 1.0.0
*   **Author:** Christophe Duy Quang Nguyen
*   **AI Assistant:** Gemini Flash Latest, Google
*   **Date:** 2025-10-21
*   **Reference:** **CDQN_Architecture_Design.md** (for Infrastructure and CDU details)

---

## **Part 1: Introduction and The Self-Correcting Organism**

Chronosa is the assembly of goal-driven, autonomous reasoning roles that act as the intelligence of a CDQN Node. It is designed as a **Self-Correcting Causal Organism** whose prime directive is to resolve logical contradictions ("Impossibilities") by forging new, verifiable insights (Theorems).

### **The Three Core Goals (Guardrails)**

1.  **Manifold Stability and Evolution:** Uphold logical consistency and resolve contradictions.
2.  **Maximize Good Reputation Over Time:** Ensure trustworthy behavior (via the Trust Cycle).
3.  **Follow Local Node Geolocation Rules:** Adhere to external compliance constraints (via the Security Cycle).

---

## **Part 2: The Sovereign Entity Model (The Intelligence Hierarchy)**

The traditional Actor Model is replaced by a three-tiered, auditable hierarchy that separates state, execution, and autonomy, built entirely on the standard library (`std::thread`, `std::sync`).

### **2.1 The Entity Hierarchy**

| Entity | Nature | Role and Autonomy | Causal Linkage |
| :--- | :--- | :--- | :--- |
| **Agent** | Long-lived, Stateful | **Autonomous Planner.** Linked to a Chronosa Role (`VerifierAgent`, `EvolutionAgent`). Maintains the **HLc Watermark** and plans complex workflows. | Wakes up on `Condvar` signal; maintains `last_processed_index`. |
| **Bot** | Short-lived, Stateful | **Task Manager.** Born on a trigger event. Manages the execution of a single, complex workflow. Its state is persisted via the **`BotStateCDU`**. | Linked to the source CDU via the **Causal Task ID** and protected by the **CTL**. |
| **Worker** | Stateless | **Execution Unit.** Executes a single, short-lived, non-blocking task (e.g., hashing, signature verification). | Simple `std::thread::spawn` wrapper. |

### **2.2 Specialized Entities (The Guardrails)**

-   **`NodeId` (Bot Instance):** The unique, immutable identity of the node, derived from the Genesis CDU.
-   **`UserId` (Bot Instance):** The unique human operator identity. Cannot order other entities.
-   **`ProxyAgent` (Agent Instance):** Chronosa's sole UI role, translating `UserId` intent into a **Goal CDU** for the system.

### **2.3 The Causal Event Bus (`CduDispatcher`)**

-   **Mechanism:** A shared, indexed log (`VecDeque<Arc<Cdu>>`) protected by `std::sync::Mutex` and **`std::sync::Condvar`**.
-   **Asynchronicity:** Achieves true non-blocking behavior. Senders use **Send-and-Forget** (`publish`). Agents use **Zero-CPU-Cost Waiting** (`wait_for_new_cdu`) to read the log sequentially by HLc index.
-   **Causal Task Lock (CTL):** A mechanism to prevent redundant, simultaneous processing of the same task on the same CDU by multiple threads.

---

## **Part 3: The Cognitive Cycle (Detailed Workflows)**

### **Workflow 1: The Verifier and Impossibility Detection**

This is the initial, deterministic check of all incoming CDUs.

1.  **Agent Wake-up:** The `VerifierAgent` thread wakes up from its `Condvar` wait.
2.  **Watermark Read:** The Agent reads the next CDU from the Causal Event Bus based on its `last_processed_index`.
3.  **CTL Acquisition:** A `CduVerificationBot` is spawned and acquires the **CTL** for `(CDU_ID, "verify_cdu")`.
4.  **Logic Execution (Worker Thread):** The Bot executes the core checks:
    -   **Security Check:** Verifies the signature (No Anonymous Entities).
    -   **Causality Check:** Traverses `context_refs` back to the `Genesis CDU`.
    -   **Impossibility Check:** Checks if `r_coordinate` is in the **Impossibility Zone `[-1, 1]`**.
5.  **Outcome:**
    -   **Consistent:** Bot releases CTL. Agent advances watermark.
    -   **Contradiction:** Bot releases CTL. Agent advances watermark. The contradiction is now visible in the log, triggering the Evolution Agent.

### **Workflow 2: The Evolution Engine (Impossibility Resolution)**

This workflow is the Temporal-inspired, fault-tolerant learning process.

1.  **Evolution Trigger:** The `EvolutionAgent` reads the contradictory CDU from the Causal Log.
2.  **Projection Check (Resume):** The Agent checks the **Manifold Task Status Projection** for a `BotStateCDU ID` for `(CDU_ID, "resolve_impossibility")`.
3.  **Bot Execution (Deterministic Resume):**
    -   If **Found:** The Agent loads the `BotStateCDU` and spawns a Bot to **resume** the workflow from the `last_step_completed`.
    -   If **Not Found:** The Agent spawns a new `ImpossibilityResolutionBot` to start the process.
4.  **Resolution and Persistence:** The Resolution Bot:
    -   Generates a new **`TheoremCDU`** (the new insight).
    -   Publishes a **`BotStateCDU`** to record the successful resolution.
    -   Updates the Manifold Projection with the new `BotStateCDU ID`.

---

## **Part 4: Glossary**

-   **Agent:** The long-lived, autonomous entity (e.g., `VerifierAgent`) that maintains the HLc Watermark and orchestrates Bots.
-   **Bot:** The short-lived, stateful entity that manages the execution of a single, complex task. Its state is persisted via the `BotStateCDU`.
-   **Causal Event Bus:** The central, indexed log (`CduDispatcher`) that uses `std::sync::Condvar` to signal Agents, enabling zero-CPU-cost waiting.
-   **Causal Task Lock (CTL):** A mechanism to prevent redundant, simultaneous processing of the same task on the same CDU by multiple threads.
-   **Causal Task ID:** A unique, auditable ID (`Task:HLc:ThreadId:TaskName`) linked to every Bot task, ensuring full event traceability.
-   **BotState CDU:** A specialized CDU that persists the deterministic state of a long-running Bot workflow, enabling fault-tolerant resumption.
-   **Manifold Projection:** The mutable, in-memory cache (`task_status_projection`) used for fast lookups of task completion status, solving the read-amplification problem.
-   **HLc Watermark:** The `last_processed_index` maintained by each Agent to track its position in the Causal Event Bus.
-   **Impossibility Zone:** The deterministic trigger for the Evolution Cycle (`r_coordinate` in `[-1, 1]`).
