# memCDU - memories of Context Data Unit
### Doc 3 Version 1.0.0

## Introduction: The Story of an AI That Remembers

Imagine an AI research assistant named Archie. In its first version, Archie was brilliant but flawed. It could access vast libraries of information, but it had no true memory. It would answer a question perfectly one moment, and then forget the context in the next. It would sometimes cite a scientific paper from 2010 as the latest finding, unaware of a newer, contradictory paper from 2023. Worse, when pressed on a complex topic, it would occasionally "hallucinate"—confidently stating a plausible but incorrect fact. Archie was a powerful tool, but it wasn't a reliable partner.

The problem wasn't Archie's intelligence; it was its memory. It lived in a perpetual present, with no stable foundation of what it knew, where that knowledge came from, or how reliable it was.

**`memCDU` is the architecture that gives Archie a mind.**

It is a new kind of memory, designed from the ground up for AI. It provides a foundation where every piece of knowledge is a verifiable, self-aware artifact. With `memCDU`, Archie can now build its own private, sovereign "universe of meaning" based on the data it processes. It can distinguish between hard facts, creative fiction, and known falsehoods. It can track how knowledge evolves over time and automatically resolve contradictions. It can learn new skills from experience and know, with mathematical certainty, which skills are the most effective.

This document describes the architecture of that mind. It is a blueprint for transforming brilliant but unreliable AI into consistent, auditable, and truly intelligent partners.

---
## 1. What is memCDU? A New Foundation for AI

**`memCDU`** is a complete memory architecture designed to provide AI systems with a trustworthy, adaptive, and sovereign long-term memory. It is not a database; it is a **verifiable knowledge substrate**—a foundational layer on which to build reliable AI.

### Why It Matters: Key Capabilities

*   **It Solves Hallucination:** By creating a base of validated, high-integrity knowledge, `memCDU` acts as an AI's external "source of truth." It allows an AI to verify information *before* speaking, transforming it from a creative storyteller into a reliable expert.
*   **It Provides Full Auditability:** Every piece of knowledge in `memCDU` has a cryptographic "chain of custody" (Provenance) that answers *where* it came from and *how* it was validated. This makes AI decisions fully auditable, a critical requirement for regulated industries like finance and healthcare.
*   **It Enables Sovereign, Private AI:** The architecture is designed to run as a private, self-governing instance on a user's device or server. This allows for deep personalization and learning without ever compromising user privacy by sending data to the cloud.
*   **It Creates a Mind That Learns and Evolves:** `memCDU` is not a static repository. It is a dynamic system that can discover new, fundamental concepts within its own knowledge, resolve contradictions, and learn which skills are most effective over time. It is a memory designed for growth.

---
## 2. Glossary of Core Concepts

*   **CDU (Context Data Unit):** The fundamental atom of knowledge. A secure, self-contained data package that holds not just the content, but all its metadata (provenance, confidence, vector, etc.).
*   **Content-Addressing:** The principle of identifying data by a unique hash of its content. This guarantees that a CDU's ID is a perfect, tamper-proof fingerprint.
*   **Provenance:** The verifiable, cryptographically signed history of a CDU, showing its origin and every transformation it has undergone.
*   **`cdqnPSH` (Prime Semantics Hypersphere):** The advanced architecture of Layer 3. It is a "Geometric Field Theory of Meaning" where all knowledge exists on the surface of a multi-dimensional sphere. A CDU's nature is an *emergent property* of its geometric position.
*   **Prime Ideal:** A fundamental concept (like "Verifiable Fact" or "Imaginative Fiction") that has a specific, canonical vector location on the hypersphere, acting as a "center of gravity" for meaning.
*   **Scope Label:** A simple, low-cost **Ingestion-Time Intent Tag** (e.g., `factual`, `fiction`). It represents the *claimed identity* of a CDU at the moment of its creation, which is then subject to verification by the deeper `cdqnPSH` geometric analysis.
*   **Polarity (`Z` Axis):** An independent axis representing the semantic "charge" of a CDU, from negative (contradictory, deconstructive) through neutral (objective) to positive (associative, constructive).
*   **HLC (Hybrid Logical Clock):** A sophisticated timestamping mechanism that captures both real-world time and the causal relationship between events.
*   **WASI Component:** A secure, sandboxed WebAssembly module. In this architecture, all high-level intelligence is encapsulated in a suite of distinct WASI components.
*   **Host Environment:** The core `memCDU` runtime that loads WASI components and provides them with essential services (like memory access and LLM inference).
*   **Trust List:** A cryptographically signed list of WASI component IDs that are authorized by the node's owner to make changes to its long-term memory. This forms the basis of the "Sovereign Trust" model for local consensus.

---
## 3. The `memCDU` Architecture: A Layered Approach

`memCDU` is composed of three core layers, supported by an ecosystem of intelligent agentic components.

### 3.1 Layer 1: The Content Manager - The Secure Gateway
This layer is the universal, secure "front door" for all information. Its primary directive is security: **all content is treated as inert data, never as executable code.** It handles the complexities of diverse data formats, performs security screening, and prepares all content for the core system through a process of canonicalization.

### 3.2 Layer 2: The memCDU Core - The Universal Data Container
This is the heart of the system, defining the structure of the **Context Data Unit (CDU)**. The CDU is a rich, standardized container that packages every piece of knowledge with the intelligence needed to understand it. This is where the principles of content-addressing, immutability, and rich metadata are enforced.

### 3.3 Layer 3: `cdqnPSH` - The Geometric Field of Meaning
This is the system's "brain." It is a dynamic, self-governing universe of meaning where a CDU's nature is an emergent property of its position in a geometric space.
*   **The Space is a Hypersphere:** All CDU `semantic vectors` are mathematically normalized, placing them on the surface of a multi-dimensional sphere.
*   **Prime Ideals Define the "Continents":** Fundamental concepts like "Fact" and "Fiction" are not brittle tags but "capital cities" of conceptual continents on the sphere. A CDU's nature is determined by its proximity to these ideals.
*   **Falsehood is Geometric:** A falsehood is not a label but a state. A direct contradiction is the **antipode** (the polar opposite point on the sphere) of a known fact. An inaccuracy is a CDU located in a region of the sphere that the node's local consensus has mapped as counter-factual.

### 3.4 The Agentic Framework: A Component-Based Mind
The intelligence of the `cdqnPSH` layer is implemented as a suite of discrete, secure WASI components, ensuring the system is modular and not a monolith. The Host Environment provides core services, and these components consume them to perform specialized tasks.

**Key Components:**
*   **`deepconf-validator`:** A "Quality Assurance" engine that rigorously validates new information using factual and semantic consistency checks before it is admitted to long-term memory.
*   **`knowledge-distiller`:** A maintenance agent that promotes knowledge up the memory hierarchy by summarizing `mid-term` session memories into `long-term` insights.
*   **`temporal-resolver`:** A maintenance agent that scans for and resolves time-based knowledge conflicts, ensuring the memory base remains coherent and up-to-date.
*   **`ingestion-handler`:** The "front door" for all data, responsible for vectorization, polarity scoring, and assembling the final CDU for publication.
*   **`factuality-engine`:** A real-time microservice for on-demand calculation of the mathematical `S_fact` (Factuality Score).
*   **`semantic-topographer`:** A background agent that discovers new candidate concepts (Prime Ideals) by analyzing the geometric structure of the knowledge base.
*   **`consensus-engine`:** Manages the local, sovereign voting process for new Prime Ideal proposals based on the node's Trust List.
*   **`anchor-manager`:** The secure governor that ratifies and manages the official registry of Prime Ideals.

---
## 4. Key Workflows in Action

### 4.1 The Journey to Truth: From a Question to a Validated Fact
1.  **Query:** An agent asks a novel question. The system first performs a fast query against `memCDU`. If a high-confidence, pre-validated answer exists, it is returned instantly.
2.  **Candidate Generation:** If no answer is found, the Host invokes the **`deepconf-validator`** component. The component calls a host-provided `llm-inference` service to generate multiple candidate answers.
3.  **Validation:** The **`deepconf-validator`** fans out requests in parallel: it queries the `memCDU` API for factual evidence (using `factual` CDUs as a ground truth) and calls a host `nli-service` for semantic consistency checks between candidates.
4.  **Selection & Publication:** The component selects the highest-scoring answer that passes a configured threshold. It assembles a new, high-quality `factual` CDU, complete with a `ValidationReceipt` in its metadata and a provenance trail linking back to the evidence used. It then publishes this new fact to `memCDU`. The next time this question is asked, the answer is retrieved instantly from memory.

### 4.2 The Journey to Understanding: Conceptual Discovery
1.  **Discovery:** On a schedule, the `semantic-topographer` agent runs an offline clustering analysis on the hypersphere. It discovers a new, coherent cluster of CDUs and calculates its **Prime Candidacy Score (`S_prime`)**.
2.  **Proposal:** It submits this cluster as a "New Prime Proposal" CDU.
3.  **Consensus:** The `consensus-engine` is triggered. It fetches the node's **Trust List** and invokes a `vote-on-proposal` function on each trusted component.
4.  **Ratification:** If consensus is achieved, the `anchor-manager` is authorized. It assigns the next available prime number to the new concept and adds the new Prime Ideal (name, prime, canonical vector) to its registry. A new "capital city" is founded, permanently altering the node's map of meaning.

### 4.3 The Loop of Learning: Procedural Self-Improvement
1.  **Execution:** An agent retrieves a `procedural` CDU and executes the skill.
2.  **Feedback:** The outcome (success or failure) is observed.
3.  **Update:** The agent publishes a *new version* of the CDU with updated performance metrics (like `success-rate` and `usage-count`). This enriches the memory base with empirical data, allowing the agent to make better choices in the future.

---
## 5. Real-World Problems Solved

*   **AI Hallucination:** `memCDU` acts as a verifiable long-term memory. An agent can cross-reference new information against its curated knowledge base, using the `factuality-engine` to compute a `S_fact` score before trusting the information. This is critical for applications like **Medical AI assistants** where accuracy is paramount.
*   **Auditability & Compliance:** The immutable history and cryptographic provenance provide a perfect audit trail. For any AI decision, the system can produce a verifiable report of the exact knowledge used. This is essential for **Financial AI advisors** and other regulated applications.
*   **Privacy & Personalization:** The sovereign, local-first architecture allows a **Personal AI assistant** to build a deep, nuanced understanding of a user's context, files, and preferences entirely on their own device, providing powerful personalization with absolute privacy.
*   **Long-Term AI Coherence:** The combination of temporal resolution (`temporal-resolver`), memory consolidation (`knowledge-distiller`), and hierarchical memory prevents "AI dementia," ensuring that long-running agents, like an **AI Research Partner**, remain coherent and effective over years-long projects.

---
## 6. The Mathematical Foundation

### 6.1 The Factuality Score (`S_fact`)
This formula determines the degree of real-world factuality for a CDU.
Let:
*   `prox(V_cdu, V_fact_prime)` be the cosine similarity (from -1 to 1) between the CDU's vector and the "Verifiable Fact" Prime Ideal's vector.
*   `neut(P_cdu)` be the neutrality score, calculated as `1 - abs(P_cdu)`, where `P_cdu` is the CDU's polarity (-1 to 1).

**`S_fact(cdu) = prox(V_cdu, V_fact_prime) * neut(P_cdu)`**

### 6.2 The Prime Candidacy Score (`S_prime`)
This formula provides an objective measure for evaluating if a new concept is fundamental enough to become a new Prime Ideal.
Let:
*   `cohesion(C)` be the internal self-consistency of the candidate cluster `C`.
*   `cohesion(Prime_max_cohesion)` be the cohesion of the most self-consistent existing Prime Ideal.
*   `separation(C, Prime_min_separation)` be the proximity of `C` to its nearest neighboring Prime Ideal.

**`S_prime(C) = (cohesion(C) / cohesion(Prime_max_cohesion)) * (1 - separation(C, Prime_min_separation))`**

---
## 7. Technical Reference & Data Structures

### 7.1 The CDU Data Structures (WIT)
```wit
package cdqn:memcdu-types@2.1.0

// The lifecycle stage of a piece of knowledge
enum memory-layer {
  short-term,
  mid-term,
  long-term,
}

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
  semantic: option<semantic-metadata>,
  episodic: option<episodic-metadata>,
  procedural: option<procedural-metadata>,
}

// A sophisticated timestamp for causal ordering
record hlc-record {
  wall-seconds: u64,
  wall-nanos: u32,
  logical: u64
}

record confidence-metrics {
  overall: f32,
  group-confidences: list<f32>,
}

// A classification of the CDU's intended nature at ingestion
variant scope-label {
  factual, semi-factual, semi-fiction, fiction, false
}

// The full list of supported content formats
variant content-type {
    text-plain, text-html, text-csv, text-css, text-javascript,
    application-json, application-xml, application-yaml,
    math-utf8, math-latex, math-expr-struct,
    image-png, image-jpeg, image-gif, image-svg, image-webp,
    audio-mp3, audio-wav, audio-ogg,
    video-mp4, video-webm,
    application-pdf, application-rtf, application-doc, application-docx,
    application-zip, application-tar,
    application-x-python, application-x-java, application-x-csrc, application-x-rust,
}

// The core CDU structure, corrected and finalized.
record cdu {
  id: string,
  content-type: content-type,
  content-data: list<u8>,
  vector: list<f32>,
  // The 'scope' label serves as an ingestion-time hint or
  // the claimed identity of the CDU. It is subject to
  // verification by the cdqnPSH layer's geometric analysis.
  scope: option<scope-label>,
  hlc: hlc-record,
  confidence-metrics: confidence-metrics,
  provenance: list<string>,
  metadata: metadata-record,
}
```

---
## 8. API Specifications: Host and Components

### 8.1 Core `memCDU` Host API (WIT)
This is the "microkernel" API provided by the Host Environment to all components.
```wit
package cdqn:memcdu

world memory-system {
  export memcdu-api: cdqn:memcdu-api@2.0.0
}

interface memcdu-api {
  use.cdqn:memcdu-types@2.1.0.{cdu, cdu-params, query-params}
  
  // All functions are async and non-blocking
  
  // Publishes a fully formed CDU to the memory store
  async publish: func(new-cdu: cdu-params) -> expected<string, string>

  // Retrieves a CDU by its unique, content-based hash ID
  async get: func(id: string) -> expected<cdu, string>

  // Performs a semantic search for CDUs closest to the query vector
  async query: func(params: query-params) -> expected<list<cdu>, string>
}
```

### 8.2 WASI Component APIs (WIT)
These are the interfaces for the key intelligent components.

#### `deepconf-validator`
```wit
package cdqn:deepconf-validator

world validator {
  import cdqn:memcdu-api@2.0.0
  import cdqn:llm-inference@1.0.0
  import cdqn:nli-service@1.0.0
  export deepconf-engine: cdqn:deepconf-engine@2.1.0
}

interface llm-inference { /* ... */ }
interface nli-service { /* ... */ }

interface deepconf-engine {
  record Query { text: string, vector: list<f32> }
  record ValidationReceipt {
    llm-raw-score: f32,
    factual-consistency-score: f32,
    semantic-consistency-score: f32,
    supporting-evidence-ids: list<string>,
  }
  record ValidatedAnswer {
    final-answer-text: string,
    final-confidence: f32,
    new-cdu-id: string,
    receipt: ValidationReceipt,
  }
  async validate-and-answer: func(q: Query, min-threshold: f32) -> expected<ValidatedAnswer, string>
}
```

#### `ingestion-handler`
```wit
package cdqn:ingestion

world ingestion-pipeline {
  import cdqn:memcdu-api@2.0.0
  import cdqn:embedding-service@1.0.0
  import cdqn:polarity-service@1.0.0
  export ingestion-handler: cdqn:ingestion-handler@1.0.0
}

interface embedding-service { /* ... */ }
interface polarity-service { /* ... */ }

interface ingestion-handler {
  record RawContent { content-bytes: list<u8>, claimed-scope: option<scope-label> }
  // Returns the ID of the newly created CDU.
  async process-and-publish: func(content: RawContent) -> expected<string, string>
}
```

#### `factuality-engine`
```wit
package cdqn:factuality

world fact-checker {
  import cdqn:memcdu-api@2.0.0
  import cdqn:anchor-manager-query@1.0.0 // A read-only interface to get Prime vectors
  export factuality-engine: cdqn:factuality-engine@1.0.0
}

interface anchor-manager-query { /* ... */ }

interface factuality-engine {
  use.cdqn:memcdu-types@2.1.0.{cdu}
  // Implements the S_fact formula
  calculate-factuality: func(target-cdu: cdu) -> f32
}
```

#### `knowledge-distiller`
```wit
package cdqn:distiller

world knowledge-distiller {
  import cdqn:memcdu-api@2.0.0
  import cdqn:llm-inference@1.0.0 // For summarization
  export distiller-engine: cdqn:distiller-engine@1.0.0
}

interface distiller-engine {
  // Takes a list of mid-term CDU IDs from a session.
  // Returns the ID of the new long-term summary CDU.
  async distill-session: func(session-cdu-ids: list<string>) -> expected<string, string>
}
```

---
## 9. Conclusion

`memCDU` represents a complete blueprint for a new generation of AI memory. By integrating a secure, content-addressable data layer with a dynamic and geometrically grounded model of meaning, it provides a solution for building AI systems that are not only powerful but also auditable, private, and capable of genuine learning and conceptual growth.

This architecture, with its clear separation between a core memory host and a suite of specialized intelligent components, moves beyond simply storing data; it creates a framework for cultivating knowledge. It is a self-governing, self-organizing, and self-improving foundation for the trustworthy AI of the future.
