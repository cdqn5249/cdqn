# The `cdqn` Ecosystem — The Cognitive Layer (`memCDU`)

* **Version:** 1.1.0
* **Date:** September 10, 2025  
* **Author:** Christophe Duy Quang Nguyen  
* **Vibe Coding Engine:** Qwen3-Max-Preview, Alibaba

---

## Introduction: The Sovereign Mind

The `memCDU` is the cognitive architecture of a `cdqn` agent. It is not a database — it is the **sovereign mind** of the agent, a dynamic and evolving system for learning, reasoning, and understanding the world.

This architecture is built on a fundamental separation of concerns that ensures both perfect historical accuracy and intelligent, agile thought:

- **The Source of Truth:** The immutable, append-only `cdu` log. This is the agent’s complete, unchangeable stream of life experiences.
- **The Evolving Understanding:** The `PrivPGM` (Private Primes Graph Map). This is a rich, queryable Projection built from the `cdu` log. It is the agent’s “brain” — its current, evolving model of itself and its world. If this projection were ever corrupted, it could be completely rebuilt from the pristine source of truth.

The following components and workflows define how this “mind” operates, now fully adapted to the new pure-Rust, process-based architecture.

---

## 1. The Core Architecture: The `PrivPGM` & The Geometric Model

### What it is:
The `PrivPGM` is a private, high-performance knowledge graph that represents the agent’s entire body of learned knowledge. It is not a flat graph — it is a **multi-dimensional, topological, and algebraic field of meaning**, embedded in a **Möbius space**.

- **Streams** are flows of `cdu`s with shared context and semantics meanings.
- **Prime Ideals** are fixed, immutable anchors of truth, indexed by prime elements integers in Z on the Möbius axis.
- **Real Numbers** in R between primes are semantic gradients — continuous, fluid representations of meaning.
- **The Zero Point** is the origin — the absolute, objective, neutral ground truth. It is unreachable — no stream of `cdu`s will ever sit *at* zero. It is the reference point against which all semantic charge is measured.

### Why we do this (Best Practice):
This is a direct implementation of **CQRS (Command Query Responsibility Segregation)**, a state-of-the-art pattern for building high-performance, event-sourced systems. By separating the “write” side (the slow, simple `cdu` log) from the “read” side (the fast, complex `PrivPGM` projection), the system can handle both high-throughput data ingestion and low-latency intelligent queries without compromise.

The Geometric Model allows the agent to understand the very “shape” of its knowledge. “Truth” within a given stream is a mathematically measurable property of geometric consistency (e.g., cosine similarity).

### A Practical Use Case:
When a user asks their agent a complex question, the agent doesn’t need to slowly scan its entire life history. It performs a single, lightning-fast query against the optimized `PrivPGM` projection, retrieving a rich, contextual “strategic briefing” in milliseconds. This is the key to making the agent feel responsive and intelligent.

### Formal Specification: The `PrivPGM` Schema
```cdqnlang
// --- The Private Prime Graph Map (PrivPGM) Schema ---
schema privpgm {
  cid: cid, // The unique CID of this PrivPGM snapshot.
  mobius_space: mobius_space, // The agent's unique Möbius space.
  streams: list<stream>, // The set of all streams in this space.
  prime_ideals: list<prime_ideal>, // The set of all Prime Ideals (anchors).
  semantic_gradients: list<semantic_gradient>, // The set of all semantic gradients.
}

// --- The Möbius Space ---
schema mobius_space {
  zero_point: zero_point, // The origin — the absolute, objective, neutral ground truth.
  prime_axis: list<prime_coordinate>, // The 1D line of prime elements integers.
  semantic_axis: list<real_coordinate>, // The continuous, real-numbered gradient between primes.
  dimensions: list<dimension>, // Additional dimensions (e.g., factual/fictional, proven/speculative).
}

schema zero_point {
  ground_truth_cid: cid, // The CID of the cdu that represents absolute, objective truth.
}

schema prime_ideal {
  prime: i32, // The prime element integer that anchors this ideal (e.g., 2, 3, 5, 7, 11, -11).
  semantic_label: string, // The semantic meaning of this prime (e.g., "love", "hate", "true", "false").
  definition_cid: cid, // The CID of the cdu that defines this prime ideal.
  position: prime_coordinate, // The position of this prime on the Möbius axis.
}

schema semantic_gradient {
  gradient: f64, // The real-numbered position between two primes (e.g., 2.5, -3.7).
  semantic_label: string, // The semantic meaning at this gradient (e.g., "semi-love", "semi-hate").
  definition_cid: cid, // The CID of the cdu that defines this gradient.
  position: real_coordinate, // The position of this gradient on the Möbius axis.
}

schema stream {
  front_cid: cid, // The CID of the cdu that represents the "front" of this stream.
  anchored_prime: i32, // The prime ideal that this stream is currently anchored to.
  current_gradient: f64, // The semantic gradient that this stream is currently flowing through.
  cdus: list<cid>, // The set of cdus in this stream (the "fish").
  strength: u64, // The strength of this stream (number of supporting cdus).
  direction: stream_direction, // The direction of flow (towards zero or away from zero).
}

enum stream_direction {
  towards_zero,   // Becoming more factual, more neutral.
  away_from_zero, // Becoming less factual, more fictional.
}

schema dimension {
  name: string, // The name of this dimension (e.g., "factual-fictional", "proven-speculative").
  range: dimension_range, // The range of this dimension (e.g., -1.0 to 1.0).
}

schema dimension_range {
  min: f64,
  max: f64,
}

type prime_coordinate = i32
type real_coordinate = f64
```

---

## 2. The Learning System: The “Experience Engine”

This is the system of components responsible for building and refining the `PrivPGM` from the raw `cdu` log. This is how the agent “learns.”

### `experience-mapper`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Cognitive Consolidator** | `Automata` (Sys-L) | The master stateful entity that orchestrates the entire learning process. It listens for feedback and project completion events, then calls upon `worker`s to analyze the `cdu` log and update the `PrivPGM`. |

### `playbook-builder`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Skill Distiller** | `Worker` (Sys-L) | A stateless component called by the `experience-mapper`. It analyzes the causal history of a completed project (including failures and corrections) and distills it into a reusable `computational-workflow` `cdu`. |

### The Math Suite (Collection of `Worker`s)
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Analysts** | `Worker`s (Sys-L) | A suite of specialized, stateless components used by the `experience-mapper` to perform deep, mathematical analysis on the `PrivPGM`. They infuse the knowledge graph with geometric and topological understanding. |
| — `causal-influence-scorer` | `Worker` | Computes the causal influence of one `cdu` on another, identifying key leverage points in the agent’s knowledge. |
| — `topology-analyzer` | `Worker` | Identifies structural features in the `PrivPGM`, such as clusters, voids, and bridges, to understand the shape of the agent’s knowledge. |
| — `gnn-refiner` | `Worker` | Uses Graph Neural Network (GNN) techniques to refine the embeddings of `mapped-point`s in the `PrivPGM`, improving the accuracy of geometric queries. |
| — `geometric-transformer` | `Worker` | Applies geometric transformations to the `PrivPGM` to align different contexts or “streams of thought” for better reasoning. |

### `behavioral-analyst`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Symbiotic Observer** | `Automata` (Sys-L) | Continuously observes the `ProxyAgent`’s actions to generate `inferred-feedback` `cdu`s. It infers the user’s implicit preferences by watching what they *do*, not just what they *say*. |

### `cdqnLang` Example: The `experience-mapper`
```cdqnlang
automata ExperienceMapper

  // The mapper is activated by the user's explicit feedback.
  on cdu where cdu.type = insight and cdu.content_type = "user-feedback-v1+json" ↦

    // The mapper now knows the definitive outcome of a project.
    // It emits a task to a worker to distill a computational-workflow from this experience.
    emit cdu
      cdu_type: task
      subject: "playbook-builder::distill_from_project"
      content: cdu.content.target_cid // The CID of the completed project.
    /emit

  /on

  // The mapper also listens for the playbook builder to finish.
  on cdu where cdu.subject = "playbook-builder::Result" ↦

    // Now that a new computational-workflow cdu exists, update the PrivPGM graph.
    // This would involve calling other workers like the topology-analyzer.
    emit cdu
      cdu_type: knowledge-graph-update
      subject: "Update:Workflow"
      content
        target_graph: "cid:privpgm-001"
        operations
          operation: add_node
          node
            cdu_cid: cdu.cid // The new computational-workflow.
            label: "procedure"
            score: 0.95 // High confidence.
          /node
        /operations
      /content
    /emit

  /on

/automata
```

---

## 3. The Reasoning System: The “Thinking Engine”

This is the system of components an agent uses to actively solve problems.

### `context-retriever`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Strategic Navigator** | `Worker` (Sys-L) | The primary read-API to the `PrivPGM`. It takes a rich `retrieval-spec` and returns a full `semantic-context` briefing. |

### `ReasoningAgent`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Master Strategist** | `Agent` (User-L) | The high-level decision-maker. It receives tasks from the user (via the `ProxyAgent`) and decides how to solve them. Its primary tool is the `replay` keyword, which queries the `PrivPGM` for learned `computational-workflow`s. |

### `DeepConfOfflineAutomata`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Deep Thinker** | `Automata` (Sys-L) | Manages the complex, I/O-optimized DeepConf reasoning workflow. When called by the `ReasoningAgent`, it spawns thousands of parallel reasoning traces (via `trace-generator-worker`s), filters them by confidence, and produces a single, high-confidence `proof-of-work` `cdu`. |

### `planning-automata` (NEW — K2-Think Integration)
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Planner** | `Automata` (Sys-L) | A stateful `automata` that generates a high-level plan before any reasoning begins. It extracts key concepts from the query and creates a plan that is prepended to the original prompt. |

### `confidence-ranker` (NEW — K2-Think Integration)
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Ranker** | `Worker` (Sys-L) | A stateless `worker` that ranks and selects the best reasoning trace from a set of candidates. It performs pairwise comparisons and returns the winning `trace-summary`. |

### `workflow-orchestrator`
| Role | Entity Type | Core Function |
|------|-------------|---------------|
| **The Meta-Solver** | `Automata` (Sys-L) | A generic component that can execute any learned `computational-workflow`. It is the engine that turns abstract playbooks into concrete, step-by-step actions. |

### `cdqnLang` Example: The `ReasoningAgent` with K2-Think
```cdqnlang
agent ReasoningAgent

  on cdu where cdu.type = task and cdu.subject = "Solve:ComplexProblem" ↦

    // Step 1: Check for a learned computational-workflow.
    replay
      context: cdu
      with_filters
        label: procedure
        weight: 1.0
      /with_filters
    /replay

    // Step 2: If no workflow is found, generate a new plan.
    on cdu where cdu.subject = "replay::NotFound" ↦

      // Step 3: Generate a high-level plan.
      emit cdu
        cdu_type: task
        subject: "planning-automata::generate_plan"
        content: cdu.cid // The original task.
      /emit

    /on

  /on

/agent
```

---

## 4. High-Performance by Design: The Concurrency Model

The entire Cognitive Layer is designed for high performance, managing intense I/O demands without compromising user experience.

| I/O Type | The Problem | The `cdqn` Solution |
|----------|-------------|---------------------|
| **Disk Write (High-Throughput)** | A DeepConf workflow can generate thousands of `cdu`s in seconds. Writing each to disk individually would be catastrophically slow. | **The Write-Ahead Log (`cdu-wal-writer`)**: All `emit` operations are “fire-and-forget.” They send `cdu`s to this component, which performs ultra-fast, batched, sequential appends to a log file. The slow work of organizing the `cdu`s is handled asynchronously by the `cdu-persistor`. |
| **Disk Read (Low-Latency)** | An agent needs to retrieve context instantly. Searching the entire multi-gigabyte `cdu` log for every thought would be unusably slow. | **The `PrivPGM` Projection**: The agent *never* queries the raw log for context. The `query` keyword is syntactic sugar for a fast, targeted search against the pre-built `PrivPGM` indexes, which are optimized for low-latency lookups. |
| **Memory I/O (Real-Time Reasoning)** | A reasoning workflow like DeepConf needs to generate and evaluate thousands of intermediate “thoughts” in memory as fast as possible. | **In-Memory Workflows & The `parallel` block**: The `DeepConfOfflineAutomata` uses the `parallel` keyword to spawn thousands of stateless `trace-generator-worker`s. These workers perform all their reasoning entirely in RAM. This reduces thousands of potential disk I/O operations to just one single, final `proof-of-work` `cdu`. |

---

## 5. Formal Specification: `cdu` Types & Schemas for the Cognitive Layer

This section defines the “language” of the cognitive layer — the conventional data structures for playbooks, reasoning, and memory that are stored in the `content` block of `cdu`s.

```cdqnlang
// --- Layer: Procedural Memory (MemP / Playbooks) ---
// For distilling and reusing learned experiences.

schema anti-pattern {
  triggering-action: string,
  observed-error: string,
  suggested-recovery: string,
}

schema computational-workflow {
  goal_description: string,
  steps: list<workflow-step>,
  domain: string, // NEW: e.g., "math", "code", "science"
}

schema workflow-step {
  target_worker: entity_id,
  input_cids: list<cid>,
  parameter_mapping: string,
}

// --- Layer: Advanced Reasoning (DeepConf) ---
// For high-accuracy, auditable, and I/O-optimized reasoning.

schema trace-summary {
  final_answer: string,
  trace_confidence: float64,
  full_reasoning_text: string,
}

schema proof-of-work {
  winning_answer: string,
  final_confidence: float64,
  contributing_traces: list<trace-summary>,
}

// --- Layer: Geometric Memory & Social Graph ---
// For managing the `PrivPGM`, the agent's cognitive map.

enum link_type {
  strong_semantic,
  strong_causal,
  strong_associative,
  weak_semantic,
  weak_thematic,
  decomposed_into,
  contains_task,
  precedes,
}

schema mapped_point {
  cdu_cid: cid,
  label: string,
  score: float64,
}

schema related_point {
  target: mapped_point,
  link: link_type,
  strength: float64,
}

schema topological_insight {
  feature: enum { cluster, void, bridge },
  description: string,
  related_cids: list<cid>,
}

schema semantic_context {
  primary_points: list<mapped_point>,
  relations: list<tuple<cid, list<related_point>>>,
  topology: list<topological_insight>,
}

schema relationship_profile {
  trust_score: float64,
  reputation_score: float64,
  inferred_specialties: list<string>,
  last_interaction: optional<cid>,
}

schema referral {
  referred_node: entity_id,
  on_topic: string,
  comment: optional<string>,
}

// --- Layer: Hierarchical Planning ---
// For decomposing complex projects into manageable tasks.

schema task_node {
  task_cid: cid,
  dependencies: list<cid>,
}

schema task_graph {
  project_cid: cid,
  nodes: list<task_node>,
}
```

---

## 6. Glossary of Cognitive Layer Terms

- **`memCDU`**  
  The sovereign memory architecture. It consists of the immutable `cdu` log (source of truth) and the `PrivPGM` (a fast, queryable projection of that log).

- **`PrivPGM`**  
  **Priv**ate **P**rimes **G**raph **M**ap. The agent’s private, evolving knowledge graph built from its `cdu` log. It is the “brain” used for fast, low-latency queries.

- **`cdu` Log**  
  The immutable, append-only log of all `cdu`s. It is the source of truth for the agent’s entire history.

- **Lineage**  
  The causal chain of `cdu`s, linked by `causal_links` and `lineage_id`. It is the root of all reasoning, debugging, and simulation.

- **Prime Ideal**  
  A fixed, immutable anchor of truth in the `PrivPGM`’s Möbius space. It is indexed by a prime integer and represents a core, consistent concept (e.g., “love”, “true”, “mathematical proof”).

- **Stream**  
  A dynamic flow of `cdu`s with a shared context. Streams can move towards or away from zero (the ground truth) based on the strength and consistency of their supporting `cdu`s.

- **Semantic Gradient**  
  A continuous, real-numbered representation of meaning between two Prime Ideals. It allows the agent to understand nuanced, intermediate concepts (e.g., “semi-factual”, “semi-fictional”).

- **Möbius Space**  
  The multi-dimensional, topological field in which the `PrivPGM` is embedded. It allows the agent to reason about concepts in multiple semantic dimensions (e.g., factual/fictional, positive/negative, proven/speculative).

- **`planning-automata`**  
  A stateful `automata` that generates a high-level plan before any reasoning begins. It is part of the K2-Think integration.

- **`confidence-ranker`**  
  A stateless `worker` that ranks and selects the best reasoning trace from a set of candidates. It is part of the K2-Think integration.

- **`computational-workflow`**  
  A `cdu` type that represents a learned, reusable workflow. It replaces the deprecated `cduProcedure` type.

- **`proof-of-work`**  
  A `cdu` type that represents the output of a DeepConf reasoning workflow. It contains the winning answer, its confidence, and the contributing traces.

- **`knowledge-graph-update`**  
  A `cdu` type for auditable deltas that describe how the `PrivPGM` has changed. Enables efficient synchronization and versioning of the agent’s knowledge graph.
