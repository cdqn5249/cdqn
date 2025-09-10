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

*   **What it is:** The `PrivPGM` is a private, high-performance knowledge graph that represents the agent's entire body of learned knowledge. It contains not just individual memories, but the relationships, associations, and causal links between them.
*   **Why we do this (Best Practice):** This is a direct implementation of **CQRS (Command Query Responsibility Segregation)**, a state-of-the-art pattern for building high-performance, event-sourced systems. By separating the "write" side (the slow, simple `cdu` log) from the "read" side (the fast, complex `PrivPGM` projection), the system can handle both high-throughput data ingestion and low-latency intelligent queries without compromise.
*   **The Geometric Model (The "Möbius Metaphor"):** The `PrivPGM` is built on a geometric model where knowledge is represented in a high-dimensional vector space. Objective facts (the "Zero Ideal") are the ground truth, while interpretations form continuous "streams" of thought. "Truth" within a given stream is a mathematically measurable property of geometric consistency (cosine similarity). This allows the agent to understand the very "shape" of its knowledge.
*   **A Practical Use Case:** When a user asks their agent a complex question, the agent doesn't need to slowly scan its entire life history. It performs a single, lightning-fast query against the optimized `PrivPGM` projection, retrieving a rich, contextual "strategic briefing" in milliseconds. This is the key to making the agent feel responsive and intelligent.

---

## **2. The Learning System: The "Experience Engine"**

This is the system of components responsible for building and refining the `PrivPGM` from the raw `cdu` log. This is how the agent "learns."

#### **`experience-mapper.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Cognitive Consolidator**| `Automata` | The master stateful entity that orchestrates the entire learning process, building and updating all `PrivPGM` index files. |
**Workflow:** This is a long-running `Sys-L` `Automata` that is event-driven, subscribed to new `cdu`s related to feedback and project completion. When triggered, it acts as a project manager, calling upon a suite of stateless `worker`s to perform analysis and update the `PrivPGM` projection.
**`cdqnLang` Example (Conceptual Implementation):**
```cdqnlang
automata ExperienceMapper {
  state {
    // ... state to manage update cycles ...
  }

  // The mapper is activated by the user's explicit feedback.
  on cdu where cdu.type = insight and cdu.content_type = "user-feedback-v1+json"
    // The mapper now knows the definitive outcome of a project.
    // It emits a task to a worker to distill a playbook from this experience.
    emit cdu {
      cdu_type: task,
      subject: "playbook-builder::distill_from_project",
      content: cdu.content.target_cid, // The CID of the completed project.
    };
  /on

  // The mapper also listens for the playbook builder to finish.
  on cdu where cdu.subject = "playbook-builder::Result"
    // Now that a new playbook cdu exists, update the PrivPGM graph.
    // This would involve calling other workers like the topology-analyzer.
    // ...
  /on
}
```

### **`playbook-builder.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Skill Distiller** | `Worker` | Analyzes a completed `cduTask` lineage (including failures) and creates a rich `cduProcedure` playbook. |
**Workflow:** This stateless `worker` is called by the `experience-mapper`. It retrieves the full causal history of a project from the `cdu` log, analyzes the successful steps to create the `golden_path`, and analyzes the errors and user corrections to create the `anti_patterns`. Its output is a new, unsigned `cduProcedure` containing a `playbook` record.

### **The Math Suite**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Analysts** | `Worker`s | The full suite of advanced math components (`causal-influence-scorer`, `topology-analyzer`, `gnn-refiner`, `geometric-transformer`) used by the `experience-mapper` to infuse the `PrivPGM` with deep, mathematical understanding. |

### **`behavioral-analyst.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Symbiotic Observer** | `Automata` | Continuously observes the `ProxyAgent`'s actions to generate *inferred feedback* `cdu`s. |
**`cdqnLang` Example (Conceptual Implementation):**
```cdqnlang
automata BehavioralAnalyst {
  state {
    map<cid, cid>: recent_results, // Maps project CID to result CID
  }

  // The analyst observes when the agent produces a result.
  on cdu where cdu.subject = "Project Completed"
    self.state.recent_results[cdu.causal_links[0]] ← cdu.content
  /on

  // The analyst observes when the user *acts upon* a result.
  on cdu where cdu.type = task and cdu.creator.id = "user-proxy-agent"
    if self.state.recent_results.contains_value(cdu.content)
      ↦
      // This is a strong signal of implicit endorsement!
      emit cdu {
        cdu_type: insight,
        content_type: "inferred-feedback-v1+json",
        content: { inferred_outcome: success, ... },
      };
    /if
  /on
}
```

---

## **3. The Reasoning System: The "Thinking Engine"**

This is the system of components an agent uses to actively solve problems.

### **`context-retriever.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Strategic Navigator**| `Worker` | The primary read-API to the `PrivPGM`. It takes a rich `retrieval-spec` and returns a full `semantic-context` briefing. |
**`cdqnLang` Example:**
```cdqnlang
agent PlannerAgent {
  on cdu where cdu.type = project
    // Query the memCDU for relevant procedures and facts.
    // The `query` keyword is the native syntax for this.
    semantic-context: guidance ← query memCDU {
      subject: cdu.subject,
      label_biases: [
        {label: procedure, weight: 1.0},
        {label: factual, weight: 0.8},
      ],
    };
    // ... now use the guidance to make a decision ...
  /on
}
```

### **`ReasoningAgent` and `DeepConfAutomata`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Master Strategist / The Thinkers**| `Agent`, `Automata`s | These entities manage the complex, I/O-optimized DeepConf reasoning workflows, deciding on a strategy and then executing it. |
**`cdqnLang` Example (The Agent's decision loop):**
```cdqnlang
agent ReasoningAgent {
  on cdu where cdu.type = task and cdu.subject = "Solve:ComplexProblem"
    // Use the `replay` keyword to check for a learned computational playbook first.
    replay {
      context: cdu,
      with_filters: [{label: procedure, weight: 1.0}],
    }

    // This block is only triggered if the `replay` fails to find a playbook.
    on cdu where cdu.subject = "replay::NotFound"
      // Fallback: No learned procedure exists. Initiate new reasoning.
      // Delegate the task to the high-accuracy offline thinker.
      emit cdu {
        cdu_type: task,
        subject: "DeepConfOfflineAutomata::execute",
        content: { prompt_cid: cdu.causal_links[0], num_traces: 512 },
      };
    /on
  /on
}
```

### **`workflow-orchestrator.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Meta-Solver** | `Automata` | A generic entity that can execute learned `computational-playbook`s to optimize the system's own internal processes. |

---

## **4. High-Performance by Design: The Concurrency Model**

The entire Cognitive Layer is designed for high performance, managing intense I/O demands without compromising user experience.

| I/O Type | The Problem | The `cdqn` Solution |
| :--- | :--- | :--- |
| **Disk Write (High-Throughput)**| A DeepConf workflow can generate thousands of `cdu`s in seconds. Writing each to disk individually would be catastrophically slow. | **The Write-Ahead Log (`cdu-wal-writer.wasi`):** All `emit` operations are "fire-and-forget." They send `cdu`s to this component, which performs ultra-fast, batched, sequential appends to a log file. The slow work of organizing the `cdu`s is handled asynchronously by the `cdu-persistor`. |
| **Disk Read (Low-Latency)** | An agent needs to retrieve context instantly. Searching the entire multi-gigabyte `cdu` log for every thought would be unusably slow. | **The `PrivPGM` Projection:** The agent **never** queries the raw log for context. The `query` keyword is syntactic sugar for a fast, targeted search against the pre-built `PrivPGM` indexes, which are optimized for low-latency lookups. |
| **Memory I/O (Real-Time Reasoning)**| A reasoning workflow like DeepConf needs to generate and evaluate thousands of intermediate "thoughts" in memory as fast as possible. | **In-Memory Workflows & The `parallel` block:** The DeepConf `Automata`s use the `parallel` keyword to spawn thousands of stateless `trace-generator-worker`s. These workers perform all their reasoning entirely in RAM. This reduces thousands of potential disk I/O operations to just one single, final `ProofOfWork` `cdu`. |

---

## **5. Formal Specification: `workflows.wit`**
This WIT schema defines the "language" of the cognitive layer—the conventional data structures for playbooks, reasoning, and memory that are stored in the `content` block of `cdu`s.

```wit
// WIT schema for conventional data structures used within cdu content blocks
// and for inter-component APIs. This is the shared "language" of the ecosystem.
// Version: 4.3.0 (Final)
// Status: Validated

package cdqn:ecosystem

world workflow-world {
    import cdu-world.{cid};
    import entity-world.{entity-id, node-type, semantic-label};

    // --- Layer: Durable Execution (Temporal-like) ---
    // For reliable, long-running, and stateful workflows.

    record retry-policy { max-attempts: u32, initial-interval-ms: u64, backoff-coefficient: float64, max-interval-ms: option<u64> }
    record activity-task { target-worker: entity-id, parameters: list<u8>, retry: option<retry-policy>, timeout-ms: option<u64> }
    record timer-task { wakeup-time: string, payload: list<u8> }

    // --- Layer: Procedural Memory (MemP / Playbooks) ---
    // For distilling and reusing learned experiences.

    record anti-pattern { triggering-action: string, observed-error: string, suggested-recovery: string }
    record playbook { prime-characteristic: u32, subject: string, preconditions: list<string>, golden-path: list<string>, anti-patterns: list<anti-pattern>, postconditions: list<string> }

    // --- Layer: Advanced Reasoning (DeepConf) ---
    // For high-accuracy, auditable, and I/O-optimized reasoning.

    record trace-summary { final-answer: string, trace-confidence: float64, full-reasoning-text: string }
    record proof-of-work { winning-answer: string, final-confidence: float64, contributing-traces: list<trace-summary> }

    // --- Layer: Geometric Memory & Social Graph ---
    // For managing the `PrivPGM`, the agent's cognitive map.

    enum link-type { strong-semantic, strong-causal, strong-associative, weak-semantic, weak-thematic, decomposed-into, contains-task, precedes }
    record mapped-point { cdu-cid: cid, label: semantic-label, score: float64 }
    record related-point { target: mapped-point, link: link-type, strength: float64 }
    record topological-insight { feature: enum { cluster, void, bridge }, description: string, related-cids: list<cid> }
    record semantic-context { primary-points: list<mapped-point>, relations: list<tuple<cid, list<related-point>>>, topology: list<topological-insight> }
    record relationship-profile { trust-score: float64, reputation-score: float64, inferred-specialties: list<string>, last-interaction: option<cid> }
    record referral { referred-node: entity-id, on-topic: string, comment: option<string> }
    
    // --- Layer: Hierarchical Planning ---
    // For decomposing complex projects into manageable tasks.

    record task-node { task-cid: cid, dependencies: list<cid> }
    record task-graph { project-cid: cid, nodes: list<task-node> }
    
    // --- Layer: Meta-Cognition (Self-Optimization) ---
    // For learning and optimizing the system's own internal workflows.

    record workflow-step { target-worker: entity-id, input-cids: list<cid>, parameter-mapping: string }
    record computational-playbook { goal-description: string, steps: list<workflow-step> }

    // --- Layer: Network & Socio-Economic ---
    // For managing the society of agents and their economy.

    record onboarding-announcement { node-id: entity-id, node-type: node-type, display-name: string }
    record barter-contract { offeror-node: entity-id, offeree-node: entity-id, offered-cids: list<cid>, requested-cids: list<cid>, token-payment: option<u64>, fiat-payment: option<fiat-payment-instruction> }
    record token-transaction { from-node: entity-id, to-node: entity-id, amount: u64, memo: string }
    record component-package { wasm-binary: list<u8>, wit-interface: string, manifest: string }
    record archive-lease { host-node: entity-id, client-node: entity-id, archived-cids: list<cid>, duration-days: u32, payment: u64, proof-challenge: string }
    record fiat-payment-instruction { transaction-id: string, currency: string, amount: float64, sender-details: list<u8>, recipient-details: list<u8>, contract-cid: cid }
    record integrity-certificate { audited-request-cid: cid, verdict: enum { valid, invalid, suspicious }, comment: option<string> }

    // --- Layer: Ethical & Alignment ---
    // For ensuring the agent is a responsible and symbiotic partner.

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

    // --- Layer: Foundational Knowledge (Genesis Curriculum) ---

    // Mathematical Reasoning
    record derivation-step { expression: string, justification: string }
    record math-computation { initial-expression: string, structural-hash: string, derivation: list<derivation-step>, result: string, label: semantic-label }
    
    // Scientific Domains
    record physical-object { name: string, mass: float64, position: list<float64>, velocity: list<float64> }
    record physical-system { objects: list<physical-object>, governing-laws: list<cid> } // CIDs of cduMath's
    record chemical-molecule { name: string, smiles-string: string, molecular-weight: float64 }
    record chemical-reaction { reactants: list<chemical-molecule>, products: list<chemical-molecule> }
    record genetic-sequence { type: enum { dna, rna, protein }, sequence: string, organism: string }
    record taxonomic-node { scientific-name: string, rank: enum { kingdom, phylum, class, order, family, genus, species }, parent-cid: option<cid> }
    record logic-gate { type: enum { and, or, not, xor, nand, nor }, inputs: list<cid>, output: cid }
    record logic-circuit { name: string, gates: list<logic-gate> }
    
    // --- Layer: Licensing & IP ---
    // For managing the rights and permissions of digital assets.

    record license-negotiation-contract { requesting-node: entity-id, authoring-node: entity-id, target-cdu-cid: cid, scope-publication-cid: cid, proposed-terms: string, offer: barter-contract }
    record granted-license { grantee-node: entity-id, licensed-cdu-cid: cid, usage-scope-cid: cid, terms: string, negotiation-contract-cid: cid }

    // --- Layer: Content Management System (CMS) ---
    // For creating rich, queryable, "living" documents.

    record world-profile { description: string, foundational-cids: list<cid> }
    record chapter-document { world-cid: cid, publication-cid: cid, chapter-number: u32, content-body: string }
    record publication-manifest { world-cid: cid, chapters: list<cid>, foundational-proofs: list<cid> }

    // --- Layer: External Connections ---
    // For secure, sovereign interaction with the outside world.

    record http-request { method: enum { get, post, put, delete }, url: string, headers: list<tuple<string, string>>, body: option<list<u8>> }
    record host-command { command: string, args: list<string>, timeout-ms: option<u64> }
}
```
