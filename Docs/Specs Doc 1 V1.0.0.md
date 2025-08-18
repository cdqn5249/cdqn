# Specs Doc 1: The CDQN Ecosystem (V1.0.0)

**Version:** V1.0.0
**Date:** 2025-08-18T10:11:12Z
**Agent:** Gemini: Google (2025-08-18)
**Lead Author:** Christophe Duy Quang Nguyen
**Human Contributors:** ...

**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)

**Changelog:** Initial release of the CDQN Ecosystem Master Blueprint. This document consolidates and provides unabridged detail for all validated design decisions, serving as the single source of truth for the ecosystem's architecture and philosophy.

---

### Introduction: A Digital Partner

Imagine a digital environment that doesn't just store your data, but understands its context. Imagine an AI that doesn't just follow commands, but acts as a creative partner, helping you explore ideas. This is the vision of the cdqn ecosystem. It is not a tool you use, but a space you inhabit alongside a cohort of personalized agents that learn *with* you.

This ecosystem is designed for the "vibe coder"—the creator, the thinker, the strategist who wants to build powerful agentic systems without getting lost in the complexities of traditional code. It is a world where your intent is the language, where your data is your sovereign territory, and where every interaction is a learning experience that makes your digital partner more attuned to you. This document is the blueprint for that world.

---

### Section 1: Core Philosophy & Principles

The entire architecture is built upon these foundational, non-negotiable principles.

#### 1.1. Primary Goal
To establish the cdqn ecosystem as the reference for **vibe coding** in the Rust ecosystem. It is designed to be an **engine for inspiration**, valuing the generative potential and kinetic energy of ideas over their simple factual correctness.

#### 1.2. The Three Pillars
1.  **Empower the Vibe Coder:** Provide a high-abstraction, intent-driven environment (`cdqnLang`) where outcomes are prioritized over algorithms.
2.  **Build a Secure & Sovereign Digital Home:** Create a private, user-controlled "Personal Agent Fabric" founded on a "zero trust" security model.
3.  **Create a Resilient & Intelligent Agentic Environment:** Build a foundation that supports inspiration, hierarchical reasoning, critique, and continuous, personalized learning.

#### 1.3. Non-Negotiable Principles
*   **Data Sovereignty:** All nodes are responsible for their own data. The user has absolute ownership.
*   **Asynchronous-Only Communication:** The system is designed for a disconnected, real-world environment.
*   **Verifiable Identity:** There are no anonymous nodes, agents, or tools.
*   **Explicitness:** All language and protocols are unambiguous. There is no implicit or "magic" behavior.
*   **User Agency:** The user is always in control. The AI is a creative partner that presents choices, not a predictor that makes decisions for the user.

---

### Section 2: The Data Layer

#### 2.1. The Context Datas Unit (CDU)
The atomic, immutable data model for all information in the ecosystem.

| Category | Field Name | Detailed Purpose |
| :--- | :--- | :--- |
| **Identity & History**| `Unique ID` | The single, globally unique identifier for this CDU. |
| | `Version` | The semantic version (`Major.Minor.Patch`) of the payload content. |
| | `HLC` | The Hybrid Logical Clock timestamp, ensuring a verifiable, causal order of events. |
| | `Lineage`| An array of parent `Unique ID`s, forming a directed acyclic graph that provides a complete, auditable history. |
| **Payload Semantics**| `CDUType`| A mandatory string that defines the payload's schema and purpose (e.g., `cdqn.sys.ToolDefinition`). |
| | `FSSFF` | The mandatory veracity classification: **F**actual, **S**emi-**F**actual, **S**emi-**F**iction, **F**iction, or **F**alse. |
| | `Payload`| The immutable data content, serialized into a compact binary format. |
| **Agent Interaction** | `AgentSourceID`| The `AgentID` of the agent that created this CDU. |
| | `Target`| The intended recipient(s) (`AgentID`, `ToolID`, or `NodeID`). |
| | `Intent`| The purpose of the CDU (`Information`, `Task`, `Response`, `Goal`, `Critique`). |
| | `TaskStatus`| The execution state of a task (`Pending`, `Executing`, `Success`, `Failed`, `PendingReview`, `Quarantined`). |
| | `SharedContextID`| A common ID that groups all CDUs belonging to a single collaborative project or workflow. |
| **Kinetic Metadata** | `QoS` | An object containing metrics to calculate the "generative potential" of an idea. |
| | `QoS.read_count` | A counter of how many times the payload has been accessed. |
| | `QoS.descendant_count`| A counter of how many child CDUs have been created from this one. |
| | `QoS.last_access_hlc`| The HLC timestamp of the last interaction. |

#### 2.2. `cdqnDB`: The Veracity-Partitioned Database
*   **Architecture:** An embedded, disk-first **Log-Structured Merge-Tree (LSM Tree)** database.
*   **Memory Profile:** Prioritizes freeing up RAM for Agent inference by keeping its own memory footprint minimal via a small, configurable `Memtable` and delegating caching to the OS page cache.
*   **Logical Subgraphs:**
    *   **Veracity Subgraphs:** The database is logically partitioned based on the `FSSFF` field: `Factual`, `Semi-Factual`, `Fiction`, `Semi-Fiction`, and `False`.
    *   **Security Subgraphs:** A **Configuration Subgraph** for strictly-validated system definitions and a **Data Subgraph** for operational data.

---

### Section 3: The `cdqnLang` Language of Intent

#### 3.1. Syntax Philosophy
Visually intuitive, using indentation, explicit `end` tags, and symbolic UTF-8 operators. No brackets `{}`.

#### 3.2. Validated Constructs
*   **`goal` Block:** Top-level container for user intent.
*   **`tool` Definition Block:** Declarative syntax for defining new Tools.
*   **`variables` Block:** A mandatory "ingredients list" at the top of a scope. Multiple variables of the same type can be declared on one line.
    ```coffeescript
    variables
        let Number: a, b, c
        let String: message
    end variables
    ```
*   **`plan` Block:** The main block for the sequence of actions.
*   **`task` Declaration:** A single unit of work performed by a Tool.
*   **`On each ... end each` Loop:** For processing collections.
*   **`Repeat until ... end repeat` Loop:** For conditional logic with mandatory safeguards.

#### 3.3. Validated Operators
| Operation | Symbol | Example |
| :--- | :--- | :--- |
| **Assignment** | `:=` | `let x: Number := 10` |
| **Equality** | `=` | `if x = 10` |
| **Inequality** | `≠` | `if x ≠ 10` |
| **Flow/Assignment**| `→` | `let result → task "id"` |
| **Math** | `+`, `−`, `×`, `÷`, `²`, `³`, `ⁿ` | `x² + y²` |

---

### Section 4: The Actors (Agents & Tools)

#### 4.1. Agents: The "Thinkers"
Stateful, long-lived, cognitive entities powered by AI models.
*   **`ProxyAgent`:** The user's sole interface. Translates vibes into `cdqnLang` `goal`s.
*   **`OrchestrAgent`:** The "brain." Analyzes `goal`s, orchestrates the PIG+GCR protocol, and learns from `CDU Experience` records.
*   **`SecurAgent`:** The guardian. Manages Tool approval, the `HlcEcho` protocol, and security policies.
*   **`NetAgent`:** The conductor. Monitors network health and node reputation.
*   **The Council of Veracity (Mixture of Experts):**
    *   **`FactualAgent`:** Verifies objective truth.
    *   **`SemiFactualAgent`:** Interprets data and generates reasoned insights.
    *   **`SemiFictionAgent`:** Creates narratives grounded in factual constraints.
    *   **`FictionAgent`:** Generates unconstrained creative content.
    *   **`FalseAgent`:** Identifies and analyzes falsehoods for forensic purposes.

#### 4.2. Tools: The "Doers"
Stateless, ephemeral, deterministic functions. The `cdqn.sys.ToolDefinition` CDU is their complete manifest.

| Field | Detailed Purpose |
| :--- | :--- |
| `ToolID` | The unique, versioned identifier (e.g., `tools.web.http-get.v1.2`). |
| `GoalDescription`| A rich, natural language description for semantic discovery. |
| `InputSchema`, `OutputSchema` | The `CDUType`s defining the Tool's "API." |
| `SourceID`, `SourceSignature`| Links the Tool to a trusted `ToolSource` and guarantees its authenticity. |
| `RequiredPermissions` | A structured list of the exact, scoped capabilities the Tool needs (e.g., `{ capability: "network.http.get", scope: "api.weather.com", ... }`). |
| `FunctionBody` | The `cdqnLang` script containing the Tool's logic. |

---

### Section 5: The Runtime & Network Layer

#### 5.1. `cdqnRuntime`: The Portable Operating System
*   **Architecture:** A single, portable **WASM/WASI binary**.
*   **Stratified Profiles:** **`Runtime-Light`** (for phones) and **`Runtime-Full`** (for desktops/servers).
*   **Personal Swarm:** A user's `Runtime-Full` nodes can aggregate into a logical "virtual server" to which their `Runtime-Light` devices can securely delegate heavy tasks.

#### 5.2. `cdqNetwork` & `cdqnProt`: The Asynchronous Fabric
*   **Architecture:** A hybrid P2P network of **Personal Nodes** (forming the private "Personal Agent Fabric") and **Community Nodes** (stateless signaling/rendezvous servers).
*   **Protocol (`cdqnProt`):** An agent-aware gossip protocol that enables **Asynchronous Distributed Data Escrow**.
    1.  **Encrypt:** The sender's runtime encrypts the CDU payload with the recipient's public key.
    2.  **Shard:** The encrypted data is sharded into `N` pieces, and `M` recovery shards are created using erasure coding.
    3.  **Distribute:** All `N+M` meaningless shards are sent to different Community Nodes for temporary storage.
    4.  **Signal:** The sender sends a `ShardMap` CDU (containing shard locations) to the recipient's mailbox on a Community Node.
    5.  **Reassemble:** The recipient's runtime uses the `ShardMap` to gather a quorum of shards, reconstruct the encrypted data, and decrypt it locally.

---

### Section 6: Key Protocols & Workflows

#### 6.1. Hierarchical Reasoning (PIG + GCR)
The core protocol for AI-driven generation.
1.  **Preference-Informed Generation (PIG):** The `OrchestrAgent` consults past `CDU Experience` records to write more nuanced, personalized prompts for the other agents.
2.  **Generate-Critique-Refine (GCR):**
    *   **Generate:** A primary Veracity Agent creates a "first draft."
    *   **Critique:** Two other Veracity Agents review the draft from different perspectives.
    *   **Refine:** The original expert generates three distinct final versions based on the critiques.
3.  **User Choice:** The `ProxyAgent` **always** presents summaries of the three options to the user, who makes the final selection.

#### 6.2. The `CDU Experience` & The Learning Loop
*   The entire PIG+GCR process is captured in a `cdqn.sys.Experience` CDU.
*   These records allow the `OrchestrAgent` to refine its future prompts, personalizing the user experience over time.

#### 6.3. "Idea Genesis" Workflow
The protocol for understanding the influence of ideas.
1.  A `Tool` (`tools.analysis.idea-genesis`) calculates a CDU's **`Kinetic Score`** from its `QoS` metrics.
2.  It traces the CDU's `Lineage`, identifying "Cross-Veracity Jumps."
3.  This allows agents to reason about the *dynamics of ideas*, valuing the generative potential of all information.

#### 6.4. Secure Tool Use Platform
1.  **Discovery:** Agents find Tools via semantic search.
2.  **Approval:** The `SecurAgent` vets the Tool's signature and enforces the user's `ToolSource` whitelist.
3.  **Execution:** The `cdqnRuntime` creates a bespoke, least-privilege WASM sandbox for every execution.

#### 6.5. `HlcEcho` Protocol
A decentralized integrity check orchestrated by the `SecurAgent` to detect compromised nodes by comparing local and remote HLC timelines.

#### 6.6. Device Lifecycle Management
*   A user's trusted devices are managed via a `cdqn.sys.DeviceRoster` CDU.
*   Secure, user-driven workflows exist for adding, revoking access for, and decommissioning devices.

---

### Conclusion

The cdqn ecosystem is a holistic, vertically integrated architecture designed from first principles to support a new paradigm of human-computer interaction. By combining a sovereign data model, a secure and decentralized network, an intent-driven language, and a sophisticated hierarchy of intelligent agents, it provides a foundation for a new class of applications that are not just tools, but true creative and analytical partners.

---

### Glossary

*   **Agent:** A stateful, long-lived, cognitive entity powered by an AI model, responsible for thinking and planning.
*   **CDU (Context Datas Unit):** The atomic, immutable data model for all information in the ecosystem.
*   **Community Node:** A trusted, highly-available third-party node providing stateless signaling and rendezvous services.
*   **Council of Veracity:** The Mixture of Experts (MoE) team of specialized agents, each an expert in a specific `FSSFF` veracity.
*   **Experience (CDU):** A high-level CDU that captures the entire history of a GCR workflow, including the user's final choice, used for agent learning.
*   **FSSFF:** The veracity system: **F**actual, **S**emi-**F**actual, **S**emi-**F**iction, **F**iction, or **F**alse.
*   **GCR (Generate-Critique-Refine):** The core hierarchical reasoning protocol where a generated "first draft" is reviewed by other expert agents before being refined into multiple user-facing options.
*   **HLC (Hybrid Logical Clock):** A distributed timestamping mechanism that ensures a verifiable, causal order of events.
*   **Kinetic Score:** A calculated metric representing a CDU's influence and generative energy, derived from its `QoS` data.
*   **LSM Tree (Log-Structured Merge-Tree):** The disk-first database architecture used by `cdqnDB`.
*   **Personal Agent Fabric:** A user's private, secure, P2P network of their own Personal Nodes.
*   **Personal Node:** A user's own device (phone, laptop, server) running the `cdqnRuntime`.
*   **PIG (Preference-Informed Generation):** The protocol where the `OrchestrAgent` uses past `Experience` records to write better, more personalized prompts for the GCR workflow.
*   **Tool:** A stateless, ephemeral, deterministic function defined in `cdqnLang`, executed in a secure WASM sandbox.
*   **Vibe Coding:** A paradigm of development focused on expressing high-level intent and outcomes, rather than low-level algorithms.
*   **WASM/WASI:** WebAssembly/WebAssembly System Interface. The portable, sandboxed binary format for the `cdqnRuntime` and all `Tools`.
