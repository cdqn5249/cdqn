# The `cdqn` Ecosystem — The Foundational Layer

* **Version:** 1.1.0
* **Date:** September 9, 2025  
* **Author:** Christophe Duy Quang Nguyen  
* **Vibe Coding Engine:** Qwen3-Max-Preview, Alibaba

---

## Introduction: What `cdqn` Is

The `cdqn` (Context Datas Query Nodes) ecosystem is a platform for building smart immutable systems.

- It is built on **Rust** — the most performant and secure systems programming language available — to ensure maximum speed, memory safety, and zero-cost abstractions at the lowest level.
- It is programmed through **`cdqnLang`** — a domain-specific language designed to be beginner-friendly, architecturally explicit, and free of hidden control flow. Every operation, every data flow, and every dependency is declared visibly and verifiably.

At its core, the ecosystem operates on three foundational elements:

- The **`cdu`** — an immutable, content-addressed unit of data that serves as the universal atom for all knowledge, state, and action.
- The **`Entity`** — a sovereign, managed process that performs logic. Each `Entity` runs as a supervised native binary, communicating only through secure, asynchronous channels.
- The **`Manifesto`** — a set of seven non-negotiable architectural laws that govern how the system behaves, ensuring sovereignty, explicitness, and non-blocking operation at every level.

Together, these elements form a runtime environment where intelligence is built from verifiable, auditable events — not mutable state. Where components are isolated, supervised, and incapable of operating outside their sandbox. Where the entire system is observable, traceable, and reconstructable from first principles.

This is not a framework. This is a standard.

---

## 1. The `cdu`: The Context Datas Unit

**Definition:**  
A `cdu` is the atomic, immutable, and verifiable unit of data in the ecosystem. It is a self-contained digital container that represents a single, specific event. All state in the `cdqn` ecosystem is the result of replaying a causally-ordered history of these `cdu`s.

### Key Properties & Rationale

- **Immutable and Permanent**  
  - *What it is:* Once a `cdu` is created, it can never be altered or deleted. "Editing" a document simply means creating a new `cdu` that is causally linked to the old one.  
  - *Why this is a Best Practice:* This principle, known as Event Sourcing, is the architectural foundation for the world's most demanding systems, including high-performance financial ledgers and mission-critical databases. By treating history as an unchangeable log of events, it provides a perfect, auditable trail, which is the ultimate guarantee of data integrity.  
  - *A Practical Use Case:* For a team of scientists collaborating on a research project, this is non-negotiable. Every experimental result, every hypothesis, and every line of analysis is a permanent `cdu`. This creates a fully reproducible and verifiable research history, eliminating any ambiguity about how a conclusion was reached.

- **Content-Addressed (`cid`)**  
  - *What it is:* A `cdu` is identified not by its name or location, but by a unique fingerprint (a cryptographic hash) of its own content and metadata. This fingerprint is its `cid` (Content Identifier).  
  - *Why this is a Best Practice:* This is the core principle of globally successful systems like Git (the world's version control standard) and IPFS (the InterPlanetary File System). It makes the data itself the proof of its own integrity. If even a single bit of a `cdu` were to change, its `cid` would change completely, making tampering mathematically impossible to hide.  
  - *A Practical Use Case:* When a legal firm uses the `cdqn` ecosystem to manage a contract, the `cid` of the final, signed document is a perfect, non-repudiable fingerprint. Both parties can independently verify that their copy is identical and unaltered, simply by re-calculating the hash. This removes the need for trusted third-party escrow services.

- **Verifiable Provenance & Causality (`creator`, `hlc-id`)**  
  - *What it is:* Every `cdu` is cryptographically signed by its creator and contains a Hybrid Logical Clock (`hlc-id`), which gives it a strict, unambiguous position in the causal history of the system, even across a distributed network.  
  - *Why this is a Best Practice:* In an era of sophisticated AI-generated content, verifiable provenance is the only reliable defense against misinformation. Digital signatures provide non-repudiation, while HLCs solve the complex problem of event ordering in distributed systems.  
  - *A Practical Use Case:* An artist creates a new piece of digital art and shares it as a `cdu`. The `creator` and `hlc-id` fields act as a permanent, verifiable "birth certificate." Months later, if a dispute over ownership arises, the artist can present this original `cdu`. Its early timestamp and their cryptographic signature provide undeniable proof of their authorship.

### Formal Specification: The `cdu` Schema

This schema defines the structure of every `cdu` in the ecosystem. It is the single source of truth for data.

```cdqnlang
schema cdu {
  // The unique, immutable fingerprint of this cdu.
  cid: string,

  // The raw binary content of the event (e.g., a JSON payload, text, or binary data).
  content: bytes,

  // All metadata about this cdu.
  meta intrinsic_metadata,

  // A cryptographic signature of the `cid`, proving its origin and integrity.
  provenance_signature: bytes
}

schema intrinsic_metadata {
  // The license under which this cdu is shared.
  license: license_type,

  // The type of event this cdu represents.
  cdu_type: cdu_type,

  // A unique, causal identifier for this specific cdu (Hybrid Logical Clock).
  id: hlc_id,

  // The ID of the causal lineage this cdu belongs to.
  lineage_id: hlc_id,

  // Links to the IDs of predecessor cdu's, forming a causal chain.
  causal_links: list<hlc_id>,

  // A human-readable subject/title for the cdu.
  subject: string,

  // Tags for categorization and search.
  tags: list<string>,

  // Information about the entity that created this cdu.
  creator: creator_info,

  // The MIME type of the `content` (e.g., "application/json", "text/plain").
  content_type: mime_type,

  // Optional: Provenance for data sourced from an external system.
  source: optional<source_provenance>,

  // Optional: Metadata for AI-generated content.
  generation: optional<generation_info>
}

schema creator_info {
  // The unique ID of the Entity that created this cdu.
  id: entity_id,

  // The form of the Entity (worker, automata, agent, node).
  form: entity_form,

  // The execution context (sovereign-system, sovereign-user, etc.).
  context: execution_context,

  // The ID of the cdqnRuntime that spawned this creator. Enforces strong binding.
  supervisor_id: entity_id
}

schema source_provenance {
  // The ID of the remote node where this data originated.
  node_id: entity_id,

  // The cid of the original cdu on the remote node.
  cid: cid,

  // The HLC ID of the original cdu.
  hlc_id: hlc_id,

  // The creator info from the remote system.
  creator: creator_info
}

schema generation_info {
  // The cid of the prompt that generated this content.
  prompt_cid: cid,

  // A confidence score for the generated content.
  score: float64,

  // The metric used to calculate the score (token, group, trace).
  metric: confidence_metric,

  // Optional: The cid of the AI model used for generation.
  model_cid: optional<cid>
}

// --- Core Type Definitions ---
type hlc_id = string
type mime_type = string
type cid = string
type entity_id = string

// --- Enumerations ---
enum cdu_type {
  system, config, log, chat, task, project, contract, procedure, math, component, license,
  world, chapter, publication
}

enum license_type {
  standard(standard_license),
  custom(cid) // Reference to a custom license cdu.
}

enum standard_license {
  badaas_v1,
  mit,
  apache_2_0,
  gpl_3_0,
  cc_by_sa_4_0
}

enum confidence_metric {
  token,
  group,
  trace
}
```

---

## 2. The `Entity`: The Sovereign Logic Primitive

**Definition:**  
An `Entity` is a sovereign, managed process. It is a native, standalone binary spawned and supervised by the `cdqnRuntime`. It encapsulates a specific form of logic (`worker`, `automata`, `agent`, `node`) and communicates with other `Entity`s exclusively through the runtime-managed, asynchronous message-passing system. Its lifecycle and access to system resources are strictly controlled by the `cdqnRuntime`, ensuring it cannot operate outside its sandbox.

### Key Properties & Rationale

- *What it is:* The ecosystem is a "society of components," not a single, monolithic application. Each `Entity` has a specific form (`worker`, `automata`, `agent`, `node`) and runs as a distinct, supervised OS process. Communication between entities is direct and decentralized, facilitated by capability-based channels provided by the `cdqnRuntime`.  
- *Why this is a Best Practice:* This architecture is a fusion of the Actor Model (used to build resilient, highly concurrent systems) and modern Capability-Based Security. This makes the system robust, scalable, and adaptable. A failure in one component is isolated by the OS process boundary and cannot crash the entire node. The `cdqnRuntime`'s supervision enforces the "Node is Sovereign" law by acting as the gatekeeper for all system resources.  
- *A Practical Use Case:* A developer wants to add a new capability to their agent, such as the ability to analyze a new type of data file. They do not need to modify the agent itself. They simply write a new, self-contained `worker` component. Once this component is compiled into a native binary and acquired, the `cdqnRuntime` can spawn it as a new process. The `Agent` can then send it `cduTask`s via a secure, direct channel. This allows for rapid, secure, and decentralized innovation.

### Formal Specification: The `Entity` Schema

This schema defines the identity and properties of every actor in the system.

```cdqnlang
// --- Core Entity Definitions ---
type entity_id = string

enum entity_form {
  worker,     // Stateless logic unit
  automata,   // Stateful, event-driven logic unit
  agent,      // Stateful, planning & reasoning unit
  node        // The sovereign host entity (the cdqnRuntime itself)
}

enum execution_context {
  sovereign_system,  // Trusted kernel-level process, spawned by the cdqnRuntime
  sovereign_user,    // User-level, sandboxed process, spawned by the cdqnRuntime
  remote_system,     // Untrusted foreign system process
  remote_user        // Untrusted foreign user process
}

// --- Node-Specific Definitions ---
enum node_type {
  home_node,
  private_node,
  firm_node,
  public_node
}

// A node's public-facing profile.
schema node_profile {
  node_id: entity_id,
  node_type: node_type,
  display_name: string,
  pub_pgm_cid: cid // Reference to its public knowledge graph.
}
```

---

## 3. The `cdqn` Manifesto: The Unbreakable Laws

**Definition:**  
The Manifesto is a set of foundational, non-negotiable architectural laws that are enforced by the native `cdqnRuntime` binary. These laws ensure the long-term integrity, security, and sovereignty of the entire ecosystem.

### Key Laws & Rationale (Updated for New Architecture)

- **The Node is Sovereign:**  
  - *What it is:* Each user's `cdqn` node is their own sovereign territory. The `cdqnRuntime` is the only process with direct access to the host's file system and network. No outside entity can write data to it, change its state, or access its private memory without its explicit, verifiable consent.  
  - *Why it's a Best Practice:* This is a direct response to the failings of the centralized web, where user data is a commodity. By making the user the absolute owner of their data and digital identity, we build a foundation of trust and privacy. The `cdqnRuntime` enforces this by acting as a capability broker, providing child processes only with the specific, limited resources they need.  
  - *A Practical Use Case:* A user receives a chat message proposal. Their `ProxyAgent` (a supervised process) uses the sender's reputation score to decide whether to even show the message to the user. This Sovereign Ingestion Workflow acts as an automatic, intelligent spam filter, giving the user ultimate control over their digital space.

- **Absolutely Explicit:**  
  - *What it is:* There is no "hidden magic" in the system. Every significant action is represented by an explicit, auditable `cdu`. Furthermore, the operational scope of every `Entity` process is explicitly defined in its manifest file, which declares the capabilities it requires and the message types it can send and receive.  
  - *Why it's a Best Practice:* This principle is core to creating transparent, debuggable, and auditable systems. The manifest file makes the component's dependencies and permissions explicit before it is even spawned, preventing unexpected behavior.  
  - *A Practical Use Case:* An `Agent` makes a decision that leads to a failure. A developer can use the `cdqn-cli`'s `lineage` command to trace the exact, step-by-step history of `cdu`s that led to that decision. The agent's "thought process" is not a black box but a clear, readable log of events, making it possible to perform a root cause analysis. The manifest file for the agent's process also reveals exactly what resources it was permitted to access.

See the full manifesto in Doc 01.
---

## Glossary of `cdqn` Terms

- **`cdqn`**  
  **C**ontext **D**atas **Q**uery **N**odes. The name of the entire ecosystem and platform for building smart immutable systems.

- **`cdu`**  
  **C**ontext **D**atas **U**nit. The atomic, immutable, and verifiable unit of data. Every event, state change, or piece of knowledge in the system is a `cdu`.

- **`cid`**  
  **C**ontent **ID**entifier. A unique, cryptographic hash (fingerprint) of a `cdu`'s content and metadata. It is the primary, immutable identifier for any piece of data.

- **`Entity`**  
  A sovereign, managed process that performs logic. It can be a `worker`, `automata`, `agent`, or the `node` itself. It is the "actor" in the system.

- **`cdqnRuntime`**  
  The native, sovereign kernel. It is the only process with direct access to the host system (filesystem, network). It spawns, supervises, and manages all other `Entity` processes, enforcing the Manifesto.

- **`cdqnLang`**  
  The universal, beginner-friendly language for defining `Entity` logic and querying the system. It is designed to be explicit, with no hidden state or side effects.

- **`memCDU`**  
  The sovereign memory architecture. It consists of the immutable `cdu` log (source of truth) and the `PrivPGM` (a fast, queryable projection of that log).

- **`PrivPGM`**  
  **Priv**ate **P**rimes **G**raph **M**ap. The agent's private, evolving knowledge graph built from its `cdu` log. It is the "brain" used for fast, low-latency queries.

- **`cdqNetwork`**  
  The decentralized, peer-to-peer network protocol that allows sovereign `cdqn` nodes to discover each other, communicate, and collaborate.

- **`ProxyAgent`**  
  The user's personal representative and guardian in the system. It is the only `Entity` through which a user interacts with the `cdqn` ecosystem, ensuring security and alignment.

- **`hlc-id`**  
  **H**ybrid **L**ogical **C**lock **ID**. A timestamp that provides a strict, causal ordering for events in a distributed system, even when clocks are not perfectly synchronized.

- **Manifesto**  
  The seven non-negotiable architectural laws that govern the entire `cdqn` ecosystem, ensuring its core principles of sovereignty, security, and performance.
