# The `cdqn` Ecosystem — The Foundational Layer

**Version:** 1.1.0
**Date:** September 9, 2025  
**Author:** Christophe Duy Quang Nguyen  
**Vibe Coding Engine:** Qwen3-Max-Preview, Alibaba

---

## Introduction: What `cdqn` Is

The `cdqn` (Context Datas Query Nodes) ecosystem is a platform for building smart immutable systems.

It is built on Rust — the most performant and secure systems programming language available — to ensure maximum speed, memory safety, and zero-cost abstractions at the lowest level.

It is programmed through `cdqnLang` — a domain-specific language designed to be beginner-friendly, architecturally explicit, and free of hidden control flow. Every operation, every data flow, and every dependency is declared visibly and verifiably.

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
  system, config, log, chat, task, project, contract, math, component, license,
  world, chapter, publication,
  security-audit,
  computational-workflow,
  component-interface,
  knowledge-graph-update,
  gossip-message,
  reputation-event,
  alignment-decision,
  system-telemetry,
  external-event,
  policy-rule
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

See the Doc 01 for all laws.

---

## 4. The Security Intelligence Engine

**Definition:**  
The Security Intelligence Engine is the cognitive immune system of the `cdqn` node. It consists of two core, self-evolving workflows: the `SandboxedComponentTester` for vetting executable logic, and the Universal `cdu` Validator Framework for validating all other types of `cdu`.

These workflows ensure that *every* piece of data and code entering the system is scrutinized, tested, and learned from, turning security into an adaptive, intelligent, and collaborative process.

### 4.1. The `SandboxedComponentTester`

**Role:** A mandatory `Sys-L Automata` responsible for vetting new components in a secure, isolated environment before they are allowed to run in the user’s sovereign space.

**Workflow:**
1.  Spawns the component as a temporary, restricted process with null/fake capabilities.
2.  Feeds it test `cduTask`s and monitors behavior, resource usage, and syscalls.
3.  Emits a `component-test-report` `cdu`.
4.  Collaborates with the `experience-mapper` to evolve new `security_playbook`s for novel threats.

### 4.2. The Universal `cdu` Validator Framework

**Role:** A modular framework of specialized `Validator` `Automata` that validate *all* incoming `cdu`s (not just components) based on their type and content.

**Core Validators:**
- `StructuralValidator`: Schema, signature, and CID integrity.
- `SemanticValidator`: Logical consistency and domain-specific rules.
- `EconomicValidator`: Contract terms, licenses, and financial instructions.
- `ReputationValidator`: Cross-referencing with network reputation.
- `ContentValidator`: Future validator for media and binary content.

**Workflow:**
1.  The `cdu-validator-router` receives a new `cdu` and routes it to the appropriate validator(s).
2.  Each validator performs its checks and emits a `cdu-validation-report`.
3.  If a novel threat is discovered, a new `threat_signature` is created and a `security_playbook` is evolved.
4.  The `ProxyAgent` presents the report(s) to the user for final approval.

### Formal Specification: Security Intelligence Schema

```cdqnlang
// --- Security-Specific Data Structures ---

schema threat_signature {
  pattern_name: string,
  syscall_sequence: list<string>, // e.g., ["open", "connect", "execve"]
  resource_abuse_profile: test_resource_metrics,
  behavioral_flags: list<string>, // e.g., ["rapid_file_creation", "obfuscated_code"]
  confidence_score: float64,
  first_observed: hlc_id,
  last_observed: hlc_id
}

schema security_playbook {
  goal: string, // e.g., "Detect Data Exfiltration"
  threat_signature: cid, // Reference to the threat this playbook counters
  test_procedure: computational_playbook, // The testing steps to detect it
  mitigation_strategy: list<string>, // e.g., "Block network, quarantine component"
  effectiveness_score: float64, // Learned from real-world outcomes
  version: u32
}

schema component-test-report {
  component_cid: cid,          // The CID of the component being tested.
  tester_node_id: entity_id,   // The ID of the node that ran the test.
  test_timestamp: hlc_id,      // When the test was run.
  resource_usage: test_resource_metrics,
  behavior_log: list<string>,  // Key events observed during the test.
  security_violations: list<syscall_violation>, // Any forbidden actions.
  discovered_threats: list<cid>, // References to new or known threat_signature cdu's
  network_reputation: optional<reputation_snapshot>,
  applied_playbooks: list<cid>, // Which security_playbook's were used in this test
  final_verdict: test_verdict,
  tester_signature: bytes      // Signature by the tester's key.
}

schema cdu-validation-report {
  target_cdu_cid: cid,         // The CID of the cdu being validated.
  validator_id: entity_id,     // The ID of the validator that ran the check.
  validation_timestamp: hlc_id,
  checks_performed: list<string>, // e.g., ["schema", "signature", "semantic", "reputation"]
  validation_results: list<validation_result>,
  discovered_threats: list<cid>, // References to threat_signature cdu's (if any)
  network_reputation: optional<reputation_snapshot>,
  final_verdict: validation_verdict,
  validator_signature: bytes
}

schema validation_result {
  check_name: string,
  passed: bool,
  details: optional<string>, // e.g., "Schema mismatch on field 'amount'"
  severity: threat_severity
}

schema syscall_violation {
  syscall_name: string,
  attempted_path: optional<string>,
  timestamp: hlc_id,
  severity: threat_severity
}

schema test_resource_metrics {
  max_memory_mb: u64,
  total_cpu_ms: u64,
  disk_io_bytes: u64,
}

schema reputation_snapshot {
  global_score: float64,
  recent_complaints: u32,
  recent_praises: u32,
}

enum threat_severity {
  low,
  medium,
  high,
  critical
}

enum test_verdict {
  passed,
  failed_resource_abuse,
  failed_security_violation,
  failed_incorrect_output,
  failed_timeout,
  failed_unknown_threat
}

enum validation_verdict {
  passed,
  failed_schema_violation,
  failed_signature_invalid,
  failed_semantic_inconsistency,
  failed_reputation_risk,
  failed_unknown_threat
}
```

---

## 5. The Component Interface: The Contract for Security and Modularity

**Definition:**  
The Component Interface (`.cdqnif`) is a formal, declarative contract that every `cdu` component must provide. It defines the component’s operational scope — the messages it can send and receive, the system capabilities it requires, and the resource limits it must adhere to. This interface is used by the `cdqnRuntime` to enforce security, modularity, and sovereignty.

### Key Properties & Rationale

- *What it is:* A machine-readable schema that travels with the component binary. It is the single source of truth for what the component is allowed to do. The `cdqnRuntime` uses this schema to set up the component’s sandbox, granting *only* the declared capabilities and setting *only* the declared resource limits.  
- *Why this is a Best Practice:* This is the architectural enforcement of the Principle of Least Privilege. It prevents a component from ever accessing resources or performing actions it didn’t explicitly declare, making the system fundamentally more secure and easier to audit. It also enables true modularity, as components can be understood and composed based on their declared interfaces.  
- *A Practical Use Case:* A user acquires a new `math-worker` component. Its `.cdqnif` declares that it only needs to receive `cduTask`s and emit `cduResult`s, with no file system or network access. The `cdqnRuntime` spawns the component with *only* those permissions. Even if the component is compromised, it cannot exfiltrate data or access sensitive files, because it was never granted the capability.

### Formal Specification: The Component Interface Schema

This schema defines the structure of every `.cdqnif` file in the ecosystem.

```cdqnlang
// --- Component Interface Schema ---
// This is the formal contract for a cdu component.
// It is used by the cdqnRuntime to enforce security and modularity.

schema component_interface {
  // The entity form this component will run as.
  form: entity_form,

  // The cdu types this component can RECEIVE as input.
  accepts: list<message_spec>,

  // The cdu types this component can EMIT as output.
  emits: list<message_spec>,

  // The system capabilities this component requires.
  requires_capabilities: list<capability_spec>,

  // Optional: Resource limits (enforced by the runtime via cgroups/ulimit).
  resource_limits: optional<resource_limits>
}

schema message_spec {
  // The type of cdu (e.g., "task", "result", "log").
  cdu_type: cdu_type,
  // An optional subject filter (e.g., "Calculate:Derivative").
  subject: optional<string>
}

schema capability_spec {
  // The type of capability (e.g., "file-read", "network-send", "memory-alloc").
  capability: string,
  // An optional target or scope for the capability (e.g., a specific file path or network address).
  scope: optional<string>
}

schema resource_limits {
  max_memory_mb: optional<u64>,
  max_cpu_percent: optional<u32>,
  max_disk_io_bytes: optional<u64>,
  max_network_bandwidth_kbps: optional<u64>
}
```

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

- **`SandboxedComponentTester`**  
  A `Sys-L Automata` responsible for vetting new components in a secure, isolated environment. It generates `component-test-report` `cdu`s and collaborates with the `experience-mapper` to evolve `security_playbook`s.

- **`component`**  
  A reusable piece of logic, packaged as a native, supervised process. It is acquired, tested by the `SandboxedComponentTester`, and spawned by the `cdqnRuntime`. It communicates with other components via secure, async message channels.

- **Security Intelligence Engine**  
  The cognitive immune system of the node, comprising the `SandboxedComponentTester` and the Universal `cdu` Validator Framework. It ensures all incoming data and code is validated, tested, and learned from.

- **Universal `cdu` Validator Framework**  
  A modular system of specialized `Validator` `Automata` that validate all types of `cdu`s (not just components) for structural, semantic, economic, and reputational integrity.

- **`security-audit`**  
  A dedicated `cdu` type for all security verdicts, including `component-test-report` and `cdu-validation-report`. Ensures security events are first-class, queryable, and auditable citizens of the ecosystem.

- **`computational-workflow`**  
  A dedicated `cdu` type for executable, multi-step plans used by the `workflow-orchestrator`. Replaces the generic `procedure` type for clarity and performance.

- **`component-interface`**  
  A dedicated `cdu` type that contains the formal, immutable contract (`.cdqnif`) for a component. It defines the component’s I/O, capabilities, and resource limits.

- **`knowledge-graph-update`**  
  A `cdu` type for auditable deltas that describe how the `PrivPGM` has changed. Enables efficient synchronization and versioning of the agent’s knowledge graph.

- **`gossip-message`**  
  A `cdu` type for protocol-level control messages in the `cdqNetwork`. Separates gossip traffic from application data for performance and security.

- **`reputation-event`**  
  A `cdu` type for verifiable events that update a node’s reputation score. Creates a clean, auditable ledger of reputation changes.

- **`alignment-decision`**  
  A `cdu` type for auditable records of guardrail decisions. Allows users and auditors to trace why a request was allowed, denied, or modified.

- **`system-telemetry`**  
  A `cdu` type for performance and resource metrics. Enables the system to learn and optimize its own workflows based on real-world data.

- **`external-event`**  
  A `cdu` type for the results of interactions with the outside world (e.g., HTTP responses, host command output). Makes external I/O explicit and auditable.

- **`policy-rule`**  
  A `cdu` type for guardrails, blacklists, and system-wide policies. Defines what is allowed or forbidden in the system, with full provenance and versioning.

- **`.cdqnif`**  
  The formal, declarative interface file for a `cdu` component. Defines its message types, required capabilities, and resource limits. Used by the `cdqnRuntime` to enforce security and modularity.
