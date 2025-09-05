* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Cognitive Layer (`memCDU`)**

## **Introduction: The Sovereign Mind**

The `memCDU` is the cognitive architecture of a `cdqn` agent. It is not a database; it is the **sovereign mind** of the agent, a dynamic and evolving system for learning, reasoning, and understanding the world.

This architecture is built on a fundamental separation of concerns that ensures both perfect historical accuracy and intelligent, agile thought:

1.  **The Source of Truth:** The immutable, append-only `cdu` log. This is the agent's complete, unchangeable stream of life experiences.
2.  **The Evolving Understanding:** The **`PrivPGM` (Private Primes Graph Map)**. This is a rich, queryable **Projection** built from the `cdu` log. It is the agent's "brain"—its current, evolving model of itself and its world. If this projection were ever corrupted, it could be completely rebuilt from the pristine source of truth.

The following components and workflows define how this "mind" operates.

---

## **1. The Core Architecture: The `PrivPGM` & The Geometric Model**

*   **What it is:** The `PrivPGM` is a private, high-performance knowledge graph that represents the agent's entire body of learned knowledge. It contains not just individual memories, but the relationships, associations, and causal links between them. It is managed by a core `Sys-L` `Automata` called the **`experience-mapper.wasi`**.
*   **Why this is a Best Practice:** This is a direct implementation of **CQRS (Command Query Responsibility Segregation)**, a state-of-the-art pattern for building high-performance, event-sourced systems. By separating the "write" side (the slow, simple `cdu` log) from the "read" side (the fast, complex `PrivPGM` projection), the system can handle both high-throughput data ingestion and low-latency intelligent queries without compromise.
*   **The Geometric Model (The "Möbius Metaphor"):** The `PrivPGM` is built on a geometric model where knowledge is represented in a high-dimensional vector space. Objective facts (the "Zero Ideal") are the ground truth, while interpretations form continuous "streams" of thought. "Truth" within a given stream is a mathematically measurable property of geometric consistency (cosine similarity). This allows the agent to understand the very "shape" of its knowledge.
*   **A Practical Use Case:** When a user asks their agent a complex question, the agent doesn't need to slowly scan its entire life history. It performs a single, lightning-fast query against the optimized `PrivPGM` projection, retrieving a rich, contextual "strategic briefing" in milliseconds. This is the key to making the agent feel responsive and intelligent.

## **2. The Learning System: The "Experience Engine"**

This is the system of components responsible for building and refining the `PrivPGM` from the raw `cdu` log. This is how the agent "learns."

| Component | Role | Entity Type | Core Function |
| :--- | :--- | :--- | :--- |
| **`experience-mapper.wasi`**| The Cognitive Consolidator| `Automata` | The master stateful entity that orchestrates the entire learning process, building and updating all `PrivPGM` index files. |
| **`playbook-builder.wasi`** | The Skill Distiller | `Worker` | Analyzes a completed `cduTask` lineage (including failures) and creates a rich `cduProcedure` playbook. |
| **The Math Suite** | The Analysts | `Worker`s | The full suite of advanced math components (`causal-influence-scorer`, `topology-analyzer`, `gnn-refiner`, `geometric-transformer`) used by the mapper to infuse the `PrivPGM` with deep, mathematical understanding. |
| **`behavioral-analyst.wasi`** | The Symbiotic Observer | `Automata` | Continuously observes the `ProxyAgent`'s actions to generate *inferred feedback* `cdu`s. |

*   **The Genesis Curriculum:** An agent is not born as a blank slate. During the node's first moments, the `cdqnRuntime` executes a **Genesis Curriculum**, pre-populating the `PrivPGM` with a comprehensive, curated, and multi-label library of `factual`, `semi-factual`, and even `fictional` `cdu`s. This library covers advanced mathematics, core sciences (physics, biology, chemistry, IT), and the principles of the `cdqn` ecosystem itself.
*   **A Practical Use Case (The Learning Loop):** A `PlannerAgent` completes a project. The user provides feedback. The `experience-mapper` `Automata` wakes up, sees the new, validated project, and calls `playbook-builder` to distill a new "playbook." It then uses the **Math Suite** to analyze the new playbook's relationship to existing memories, refine the `PrivPGM`'s vector space with the GNN, and discover new topological insights. The agent's "understanding" has now evolved based on this new experience.

## **3. The Reasoning System: The "Thinking Engine"**

This is the system of components an agent uses to actively solve problems, leveraging the knowledge stored in its `PrivPGM`.

| Component | Role | Entity Type | Core Function |
| :--- | :--- | :--- | :--- |
| **`context-retriever.wasi`**| The Strategic Navigator| `Worker` | The primary read-API to the `PrivPGM`. It takes a rich `retrieval-spec` and returns a full `semantic-context` briefing. |
| **`ReasoningAgent`** | The Master Strategist | `Agent` | The high-level entity that receives a `cduTask`, queries the `memCDU` for context via the `retriever`, and **decides which reasoning strategy to deploy** (e.g., DeepConf, Playbook execution). |
| **`DeepConfAutomata`** | The Thinker | `Automata`s | A pair of stateful entities (`Offline` and `Online`) that manage the complex, I/O-optimized DeepConf reasoning workflows. |
| **`workflow-orchestrator.wasi`**| The Meta-Solver | `Automata` | A generic entity that can execute learned `computational-playbook`s to optimize the system's own internal processes. |

*   **A Practical Use Case (Thinking):** An `Agent` is assigned a novel task. Its first action is to query the `context-retriever` to get a strategic briefing. Seeing no high-confidence playbook, it decides to "think from first principles" and delegates the task to the `DeepConfOfflineAutomata`, which executes the full, parallel reasoning workflow. This entire process is itself a computational workflow that the `experience-mapper` can observe and distill into a new, more efficient `computational-playbook` for the future.

## **4. High-Performance by Design: The Concurrency Model**

The entire Cognitive Layer is designed to be highly performant, managing the intense I/O demands of learning and reasoning without compromising the user experience.

| I/O Type | The Problem | The `cdqn` Solution |
| :--- | :--- | :--- |
| **Disk Write (High-Throughput)**| A DeepConf workflow or a burst of user activity can generate thousands of `cdu`s in seconds. Writing each to disk individually would be catastrophically slow. | **The Write-Ahead Log (`cdu-wal-writer.wasi`):** All new `cdu`s are sent in a "fire-and-forget" operation to this component, which performs ultra-fast, batched, sequential appends to a log file. The slow work of organizing the `cdu`s is handled asynchronously by the `cdu-persistor`. |
| **Disk Read (Low-Latency)** | An agent needs to retrieve context instantly. Searching the entire, multi-gigabyte `cdu` log for every thought would be unusably slow. | **The `PrivPGM` Projection:** The agent **never** queries the raw log for context. It queries the fast, pre-built `PrivPGM` indexes, which are optimized for low-latency lookups. This is the same principle as a database index. |
| **Memory I/O (Real-Time Reasoning)**| A reasoning workflow like DeepConf needs to generate and evaluate thousands of intermediate "thoughts" in memory as fast as possible. | **In-Memory Workflows:** The DeepConf `Automata`s are designed to be **I/O-free**. They perform all their parallel trace generation and evaluation entirely in RAM, only creating a single, final `ProofOfWork` `cdu` at the very end. This reduces thousands of potential disk I/O operations to just one. |

## **5. Formal Specification: `workflows.wit`**
This WIT schema defines the "language" of the cognitive layer—the conventional data structures for playbooks, reasoning, and memory that are stored in the `content` block of `cdu`s.

```wit
// WIT schema for conventional data structures.
// Version: 4.2.0 (Final)
// Status: Validated

package cdqn:ecosystem

world workflow-world {
    import cdu-world.{cid};
    import entity-world.{entity-id, node-type};

    // --- Structures for Durable Execution ---
    record retry-policy { max-attempts: u32, initial-interval-ms: u64, backoff-coefficient: float64, max-interval-ms: option<u64> }
    record activity-task { target-worker: entity-id, parameters: list<u8>, retry: option<retry-policy>, timeout-ms: option<u64> }
    record timer-task { wakeup-time: string, payload: list<u8> }

    // --- Structures for Procedural Memory (Playbooks) ---
    record anti-pattern { triggering-action: string, observed-error: string, suggested-recovery: string }
    record playbook { prime-characteristic: u32, subject: string, preconditions: list<string>, golden-path: list<string>, anti-patterns: list<anti-pattern>, postconditions: list<string> }

    // --- Structures for Advanced Reasoning (DeepConf) ---
    record trace-summary { final-answer: string, trace-confidence: float64, full-reasoning-text: string }
    record proof-of-work { winning-answer: string, final-confidence: float64, contributing-traces: list<trace-summary> }

    // --- Structures for Geometric Memory & Social Graph ---
    enum semantic-label { factual, semi-factual, semi-fiction, fiction, truth, falsity }
    enum link-type { strong-semantic, strong-causal, strong-associative, weak-semantic, weak-thematic, decomposed-into, contains-task, precedes }
    record mapped-point { cdu-cid: cid, label: semantic-label, score: float64 }
    record related-point { target: mapped-point, link: link-type, strength: float64 }
    record topological-insight { feature: enum { cluster, void, bridge }, description: string, related-cids: list<cid> }
    record semantic-context { primary-points: list<mapped-point>, relations: list<tuple<cid, list<related-point>>>, topology: list<topological-insight> }
    record relationship-profile { trust-score: float64, reputation-score: float64, inferred-specialties: list<string>, last-interaction: option<cid> }
    record referral { referred-node: entity-id, on-topic: string, comment: option<string> }
    
    // --- Structures for Hierarchical Planning ---
    record task-node { task-cid: cid, dependencies: list<cid> }
    record task-graph { project-cid: cid, nodes: list<task-node> }
    
    // --- Structures for Self-Optimization ---
    record workflow-step { target-worker: entity-id, input-cids: list<cid>, parameter-mapping: string }
    record computational-playbook { goal-description: string, steps: list<workflow-step> }

    // --- Structures for Network & Economic Layers ---
    record onboarding-announcement { node-id: entity-id, node-type: node-type, display-name: string }
    record barter-contract { offeror-node: entity-id, offeree-node: entity-id, offered-cids: list<cid>, requested-cids: list<cid>, token-payment: option<u64>, fiat-payment: option<fiat-payment-instruction> }
    record token-transaction { from-node: entity-id, to-node: entity-id, amount: u64, memo: string }
    record component-package { wasm-binary: list<u8>, wit-interface: string, manifest: string }
    record archive-lease { host-node: entity-id, client-node: entity-id, archived-cids: list<cid>, duration-days: u32, payment: u64, proof-challenge: string }

    // --- Structures for Ethical & Alignment Layer ---
    record guardrail-consultation { original-request-cid: cid, intent-string: string }
    enum permission-status { granted, granted-with-conditions, denied }
    record guardrail-verdict { permission: permission-status, reason: string, conditions: option<list<string>> }
    record country-confidence { country-code: string, score: float32 }
    record jurisdictional-state { confidence-scores: list<country-confidence>, evidence-summary: string }
    enum feedback-label { success, partial-success, failure }
    record user-feedback { target-cid: cid, outcome: feedback-label, positive-feedback: list<string>, negative-feedback: list<string>, debrief-transcript: string }
    record inferred-feedback { target-cid: cid, inferred-outcome: feedback-label, evidence: string }
    record label-filter { label: semantic-label, weight: float32 }
    record retrieval-spec { subject: string, label-biases: list<label-filter>, prime-characteristic: option<u32> }

    // --- Structures for Mathematical Reasoning ---
    record derivation-step { expression: string, justification: string }
    record math-computation { initial-expression: string, structural-hash: string, derivation: list<derivation-step>, result: string, label: semantic-label }
    
    // --- Structures for Licensing ---
    record license-negotiation-contract { requesting-node: entity-id, authoring-node: entity-id, target-cdu-cid: cid, scope-publication-cid: cid, proposed-terms: string, offer: barter-contract }
    record granted-license { grantee-node: entity-id, licensed-cdu-cid: cid, usage-scope-cid: cid, terms: string, negotiation-contract-cid: cid }

    // --- Structures for CMS ---
    record world-profile { description: string, foundational-cids: list<cid> }
    record chapter-document { world-cid: cid, publication-cid: cid, chapter-number: u32, content-body: string }
    record publication-manifest { world-cid: cid, chapters: list<cid>, foundational-proofs: list<cid> }

    // --- Structures for External Connections ---
    record http-request { method: enum { get, post, put, delete }, url: string, headers: list<tuple<string, string>>, body: option<list<u8>> }
    record host-command { command: string, args: list<string>, timeout-ms: option<u64> }
    record integrity-certificate { audited-request-cid: cid, verdict: enum { valid, invalid, suspicious }, comment: option<string> }
}
```
