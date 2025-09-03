# memCDU - The memory Context Data Unit
### Doc 3 Version 1.0.0

## Introduction: The Story of an AI That Remembers

Imagine an AI research assistant named Archie. In its first version, Archie was brilliant but flawed. It could access vast libraries of information, but it had no true memory. It would answer a question perfectly one moment, and then forget the context in the next. It would sometimes cite a scientific paper from 2010 as the latest finding, unaware of a newer, contradictory paper from 2023. Worse, when pressed on a complex topic, it would occasionally "hallucinate"—confidently stating a plausible but incorrect fact. Archie was a powerful tool, but it wasn't a reliable partner.

The problem wasn't Archie's intelligence; it was its memory. It lived in a perpetual present, with no stable foundation of what it knew, where that knowledge came from, or how reliable it was.

**`memCDU` is the architecture that gives Archie a mind.**

It is a new kind of memory, designed from the ground up for AI. It provides a foundation where every piece of knowledge is a verifiable, self-aware artifact. `memCDU` is the private, sovereign "universe of meaning" that an intelligent agent builds based on the data it processes. It allows the agent to distinguish between hard facts, creative fiction, and known falsehoods; to track how knowledge evolves over time; and to learn new skills from experience.

This document describes the architecture of that mind. It is a blueprint for transforming brilliant but unreliable AI into consistent, auditable, and truly intelligent partners.

---
## 1. What is memCDU? A New Foundation for AI

**`memCDU`** is a complete memory architecture designed to provide AI systems with a trustworthy, adaptive, and sovereign long-term memory. It is not a database; it is a **verifiable knowledge substrate**—a foundational layer on which to build reliable AI.

### Why It Matters: Key Capabilities

*   **It Solves Hallucination:** By creating a base of validated, high-integrity knowledge, `memCDU` acts as an AI's external "source of truth." It allows an AI to verify information *before* speaking.
*   **It Provides Full Auditability:** Every piece of knowledge has a cryptographic "chain of custody" (Provenance) that answers *where* it came from and *how* it was validated, making AI decisions fully auditable.
*   **It Enables Sovereign, Private AI:** The architecture is designed to run as a private, self-governing instance on a user's device or server, allowing for deep personalization without ever compromising user privacy.
*   **It is the Foundation for a Mind That Learns:** `memCDU` is not a static repository. It is a dynamic system that can discover new concepts, resolve contradictions, and serve as the persistence layer for the `cde` actor framework, enabling true agentic behavior and learning.

---
## 2. Glossary of Core Concepts

*   **CDU (Context Data Unit):** The universal, content-addressed "atom" of information. A secure, self-contained data package holding content and rich metadata.
*   **CDE (Context Data Entity):** A persistent, addressable "actor" managed by the runtime. The "locus of behavior." The state and definition of all `cde`s are themselves stored as CDUs.
*   **`cdqnPSH` (Prime Semantics Hypersphere):** The advanced architecture of Layer 3. A "Geometric Field Theory of Meaning" where a `cdu`'s nature is an emergent property of its position on a multi-dimensional sphere.
*   **Prime Ideal:** A fundamental concept (e.g., "Verifiable Fact") with a specific, canonical vector location on the hypersphere, acting as a "center of gravity" for meaning.
*   **Scope Label:** A simple, **Ingestion-Time Intent Tag** (e.g., `factual`, `fiction`) representing the *claimed identity* of a CDU.
*   **Polarity (`Z` Axis):** An independent axis representing the semantic "charge" of a CDU (negative, neutral, positive).
*   **HLC (Hybrid Logical Clock):** A sophisticated timestamping mechanism that captures both real-world time and the causal relationship between events. It is the foundation for ordering and identity.
*   **Genesis CDU:** The very first CDU created in a new `memCDU` instance, acting as the "birth certificate" of the sovereign node and the root of its causal history.

---
## 3. The `memCDU` Architecture: A Layered Approach

`memCDU` is composed of three core layers that work together to store, manage, and understand knowledge.

### 3.1 Layer 1: The Content Manager - The Secure Gateway
This layer is the universal, secure "front door" for all information. Its primary directive is security: **all content is treated as inert data, never as executable code.** It handles the complexities of diverse data formats and prepares them for the core system through a process of canonicalization.

### 3.2 Layer 2: The memCDU Core - The Universal Data Container
This is the heart of the system, defining the structure of the **Context Data Unit (CDU)**. The CDU is a rich, standardized container that packages every piece of knowledge with the intelligence needed to understand it. This is where the principles of content-addressing, immutability, and rich metadata are enforced.

### 3.3 Layer 3: `cdqnPSH` - The Geometric Field of Meaning
This is the system's "brain." It is a dynamic, self-governing universe of meaning where a CDU's nature is an emergent property of its position in a geometric space.
*   **The Space is a Hypersphere:** All CDU `semantic vectors` are mathematically normalized, placing them on the surface of a multi-dimensional sphere.
*   **Prime Ideals Define the "Continents":** Fundamental concepts like "Fact" and "Fiction" are not brittle tags but "capital cities" of conceptual continents on the sphere. A CDU's nature is determined by its proximity to these ideals.
*   **Falsehood is Geometric:** A falsehood is not a label but a state. A direct contradiction is the **antipode** (the polar opposite point on the sphere) of a known fact. An inaccuracy is a CDU located in a region of the sphere that the node's local consensus has mapped as counter-factual.

---
## 4. `memCDU` as the Foundation for the Actor Model

`memCDU` is not just a passive knowledge base; it is the **persistence and state management layer for the entire `cde` (Context Data Entity) actor framework.**

### 4.1 The Principle of `cdu`-Native Persistence
Every piece of data that must survive a restart of the `cdqnRuntime` is stored as a CDU. This makes the entire state of the node auditable and immutable. This includes:
*   **The `cde` Registry:** The definition and current status of every `Node`, `Agent`, `Automata`, and `Worker` are stored as individual `cde` descriptor CDUs.
*   **Actor State:** The state of a stateful `Automata` or `Agent` is a **State CDU**. State transitions are recorded as a new, versioned `State CDU` with a `provenance` link to the previous state.
*   **Durable Worker Queues:** The message queues for `Worker` entities are persisted as `Queue State CDU`s, and each task in the queue is its own `Task CDU`. This provides guaranteed, at-least-once execution semantics.

### 4.2 Workflow: The Role of the `Genesis CDU`
The `memCDU` instance of a new node is not created empty. It is bootstrapped through a formal onboarding process.
1.  **Hardware Assessment & User Choice:** The `cdqnRuntime` assesses the host hardware and the user chooses their preferred default language model.
2.  **Creation:** This configuration, along with the node's new master identity and its very first HLC timestamp, is recorded in the **`Genesis CDU`**.
3.  **Seeding:** The runtime then populates the `memCDU` with a **"Genesis Package"** of essential CDUs, including the WIT contracts and compiled components for the core services and the node's foundational entities (`Node`, `User`, `ProxyAgent`).
4.  **Causal Root:** The `Genesis CDU`'s HLC acts as the **"Epoch 0"** for the entire node. Every subsequent CDU and CDE created has an HLC that is causally descended from this single root, creating a provable, unbroken chain of history for the entire system.

---
## 5. The Mathematical Foundation

The `cdqnPSH` layer is governed by computable mathematical formulas.

### 5.1 The Factuality Score (`S_fact`)
Determines the degree of real-world factuality for a CDU.
**`S_fact(cdu) = prox(V_cdu, V_fact_prime) * (1 - abs(P_cdu))`**

### 5.2 The Prime Candidacy Score (`S_prime`)
Provides an objective measure for evaluating if a new concept is fundamental enough to become a new Prime Ideal.
**`S_prime(C) = (cohesion(C) / cohesion(Prime_max_cohesion)) * (1 - separation(C, Prime_min_separation))`**

---
## 6. Technical Reference & Data Structures

### 6.1 The CDU Data Structures (WIT)
```wit
package cdqn:memcdu-types@2.1.0

// The lifecycle stage of a piece of knowledge
enum memory-layer { short-term, mid-term, long-term }

// Hooks for an RL agent to track procedure performance
record procedural-metadata {
  expected-utility: option<f32>,
  usage-count: option<u64>,
  success-rate: option<f32>,
  pre-conditions: option<list<string>>,
}

// Rich metadata attached to every CDU
record metadata-record {
  data-rate: f32,
  mime: option<string>,
  layer: memory-layer,
  // A separate scalar for semantic polarity, calculated at ingestion.
  polarity: f32,
  license: option<string>, // SPDX or custom license identifier
  semantic: option<semantic-metadata>,
  episodic: option<episodic-metadata>,
  procedural: option<procedural-metadata>,
}

// A sophisticated timestamp for causal ordering
record hlc-record { /* ... */ }
record confidence-metrics { /* ... */ }
variant scope-label { factual, semi-factual, semi-fiction, fiction, false }
variant content-type { /* ... full list of types ... */ }

// The core CDU structure, corrected and finalized.
record cdu {
  id: string, // HLC-based ID for many system CDUs, content-hash for others
  content-type: content-type,
  content-data: list<u8>,
  vector: list<f32>,
  scope: option<scope-label>,
  hlc: hlc-record,
  confidence-metrics: confidence-metrics,
  provenance: list<string>,
  metadata: metadata-record,
}
```

### 6.2 The Core `memCDU` API (WIT)
```wit
package cdqn:memcdu

world memory-system {
  export memcdu-api: cdqn:memcdu-api@2.0.0
}

interface memcdu-api {
  use.cdqn:memcdu-types@2.1.0.{cdu, cdu-params, query-params}
  
  async publish: func(new-cdu: cdu-params) -> expected<string, string>
  async get: func(id: string) -> expected<cdu, string>
  async query: func(params: query-params) -> expected<list<cdu>, string>
}
```

---
## 7. Conclusion

`memCDU v2.5.0` is the foundational pillar of the `cdqn` ecosystem. It is a complete architecture for a new generation of AI memory, designed not just for storing data, but for cultivating knowledge. By integrating a secure, content-addressable data layer with a dynamic and geometrically grounded model of meaning, and by serving as the immutable persistence layer for the `cde` actor model, it provides a solution for building AI systems that are auditable, private, and capable of genuine learning and conceptual growth.
