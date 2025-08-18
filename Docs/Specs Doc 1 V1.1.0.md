# Specs Doc 1: The CDQN Ecosystem (V1.1.0)

* **Version:** V1.1.0
* **Date:** 2025-08-18T16:00:00Z
* **Agent:** Gemini: Google (2025-08-18)
* **Lead Author:** Christophe Duy Quang Nguyen
* **Human Contributors:** ...

**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)

**Changelog:** This is a consolidated, self-contained release that replaces V1.0.0. It integrates a complete, multi-layered security framework, a robust node bootstrapping protocol, and significant enhancements to `cdqnLang` for improved readability and capability. This document serves as the new single source of truth for the ecosystem's architecture and philosophy.

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
*   **Explicitness:** All language, protocols, and tool dependencies are unambiguous. There is no implicit or "magic" behavior.
*   **User Agency:** The user is always in control. The AI is a creative partner that presents choices, not a predictor that makes decisions for the user.

---

### Section 2: The Data Layer

#### 2.1. The Context Datas Unit (CDU)
The atomic, immutable data model for all information in the ecosystem. Every piece of data, from a tool's source code to a simple message, is a CDU.

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
*   **Architecture:** An embedded, disk-first **Log-Structured Merge-Tree (LSM Tree)** database. Its append-only nature is critical for data integrity and auditability.
*   **Memory Profile:** Prioritizes freeing up RAM for Agent inference by keeping its own memory footprint minimal.
*   **Logical Subgraphs:**
    *   **Veracity Subgraphs:** Partitioned based on the `FSSFF` field.
    *   **Security Subgraphs:** A **Configuration Subgraph** for strictly-validated system definitions and a **Data Subgraph** for operational data.

#### 2.3. Key System CDU Types
The ecosystem's functionality relies on several predefined CDU schemas:
*   `cdqn.sys.ToolDefinition`: The complete manifest for a reusable Tool, including its source code and security requirements.
*   `cdqn.sys.ToolAlias`: A persistent, user-scoped "pointer" or shortcut to a specific version of a Tool.
*   `cdqn.sys.Experience`: A record of a full AI reasoning workflow (PIG+GCR), used by the `OrchestrAgent` for learning.
*   `cdqn.sys.DeviceRoster`: The definitive list of cryptographically trusted devices in a user's Personal Agent Fabric.
*   `cdqn.sys.CognitiveSnapshot`: A pointer to a specific HLC timestamp, marking a known-good state of an agent's learned model, enabling rollback.
*   `cdqn.sys.HazardReport`: A standardized report generated by security analysis tools detailing potential threats in a data payload.

---

### Section 3: The `cdqnLang` Language of Intent

#### 3.1. Syntax Philosophy
Visually intuitive, using indentation, explicit `end` tags, and symbolic UTF-8 operators. No brackets `{}`.

#### 3.2. Code Structure & Reusability
*   **`goal` Block:** Top-level container for user intent.
*   **`tool` Definition Block:** Declarative syntax for defining new Tools.
    *   **Permanent Tool:** A standard, reusable `Tool` stored in the `cdqnDB`, discoverable by all agents.
    *   **Goal-Scoped Tool:** A `tool` defined inside a `goal` block for single-use, ephemeral logic. It provides code reuse within a plan but is not persisted. It is subject to the strictest security constraints.
*   **`function` Construct:** Explicitly **not supported**. All executable logic must be encapsulated in a `Tool` to ensure it passes through the system's security, sandboxing, and audit protocols.
*   **`using` Block:** A mandatory "imports list" at the top of a scope for declaring short, readable `Tool Aliases`.
    ```coffeescript
    using
        http-get: "tools.web.http-get.v1.2"
        summarize: "tools.ai.summarize.v2.1"
    end using
    ```
*   **`variables` Block:** A mandatory "ingredients list" for declaring variables and their types.
    ```coffeescript
    variables
        let Number: a, b, c
        let String: message
    end variables
    ```
*   **`plan` Block:** The main block containing the sequence of actions.
*   **`task` Declaration:** A single unit of work performed by a Tool.
*   **Loop Constructs:** `On each ... end each` and `Repeat until ... end repeat`.

#### 3.3. Validated Operators

| Operation | Symbol | Example |
| :--- | :--- | :--- |
| **Value Assignment** | `←` | `let x ← 10` |
| **Flow Capture** | `→` | `let result → task "http-get"` |
| **Equality** | `=` | `if x = 10` |
| **Inequality** | `≠` | `if x ≠ 10` |
| **Comparison** | `≤`, `≥`, `<`, `>` | `if x ≥ 10` |
| **Logical** | `∧` (AND), `∨` (OR), `¬` (NOT) | `if (x > 0) ∧ (¬is_valid)` |
| **Basic Math** | `+`, `−`, `×`, `÷` | `(x × 2) + y` |
| **Powers** | `²`, `³`, `ⁿ` | `x² + yⁿ3` |

#### 3.4. Advanced Mathematical Constructs

| Operation | Type | Syntax |
| :--- | :--- | :--- |
| **Summation** | Block | `∑ (i from 1 to 10) ... end ∑` |
| **Product** | Block | `∏ (item in my_list) ... end ∏` |
| **Set Logic** | Infix | `∪` (Union), `∩` (Intersection), `∈` (Element Of) |
| **Integral** | Syntactic Sugar for Tool | `∫ (x from 0 to 10) of "x²"` |
| **Derivative** | Syntactic Sugar for Tool | `∂/∂x of "x²"` |

---

### Section 4: The Actors (Agents & Tools)

#### 4.1. Agents: The "Thinkers"
Stateful, long-lived, cognitive entities powered by AI models.
*   **`ProxyAgent`:** The user's sole interface. Translates vibes into `cdqnLang` `goal`s.
*   **`OrchestrAgent`:** The "brain." Analyzes `goal`s, orchestrates protocols, and learns from `CDU Experience` records.
*   **`SecurAgent`:** The guardian. Manages Tool approval, security policies, and active defense protocols.
*   **`NetAgent`:** The conductor. Monitors network health, manages the `HlcEcho` protocol, and maintains node reputation.
*   **The Council of Veracity (Mixture of Experts):** A team of specialist agents for nuanced content generation and analysis.

#### 4.2. Tools: The "Doers"
Stateless, ephemeral, deterministic functions. A Tool is defined as data in a `cdqn.sys.ToolDefinition` CDU.

| Field | Detailed Purpose |
| :--- | :--- |
| `ToolID` | The unique, versioned identifier (e.g., `tools.web.http-get.v1.2`). |
| `GoalDescription`| A rich, natural language description for semantic discovery by agents. |
| `InputSchema`, `OutputSchema` | The `CDUType`s defining the Tool's "API." |
| `SourceID`, `SourceSignature`| Links the Tool to a trusted `ToolSource` and guarantees its authenticity. |
| `RequiredPermissions` | A structured list of the exact, scoped capabilities the Tool needs (e.g., `{ capability: "network.http.get", scope: "api.weather.com", ... }`). |
| `FunctionBody` | The `cdqnLang` script containing the Tool's logic. |

---

### Section 5: The Runtime & Network Layer

#### 5.1. `cdqnRuntime`: The Portable Operating System
*   **Architecture:** A single, portable **WASM/WASI binary**.
*   **5.1.1. The Primordial Toolset:** The runtime binary is embedded with a minimal, pre-verified set of tools (e.g., `db-write`, `crypto-verify`, `static-analyzer`) required for a new node to bootstrap itself securely.
*   **5.1.2. Execution Tiers (Capability Ceilings):** The runtime enforces a passive, structural security model that limits the maximum possible permissions for a tool based on its origin. This is a non-negotiable backstop to the `SecurAgent`.
    *   **Tier 3 (Ephemeral):** Goal-Scoped Tools. `compute`-only, no I/O access.
    *   **Tier 1 (Trusted):** Tools from a whitelisted `ToolSource`. Can request any permission.
    *   **Tier 0 (Kernel):** Primordial system tools.

#### 5.2. `cdqNetwork` & `cdqnProt`: The Asynchronous Fabric
*   **Architecture:** A hybrid P2P network of **Personal Nodes** (forming the private "Personal Agent Fabric") and **Community Nodes** (stateless signaling/rendezvous servers).
*   **Protocol (`cdqnProt`):** An agent-aware gossip protocol that enables **Asynchronous Distributed Data Escrow** for secure, disconnected communication.

---

### Section 6: Key Protocols & Workflows

#### 6.1. Core AI Reasoning
*   **Hierarchical Reasoning (PIG + GCR):** A protocol where the `OrchestrAgent` uses past preferences (`PIG`) to inform a `Generate-Critique-Refine` (`GCR`) cycle among the Council of Veracity, resulting in multiple refined options for the user to choose from.
*   **The `CDU Experience` & The Learning Loop:** The entire PIG+GCR process is captured in a `cdqn.sys.Experience` CDU, allowing the `OrchestrAgent` to learn and personalize its responses over time.

#### 6.2. Node & Tool Lifecycle Management
*   **Genesis Protocol:** The mandatory, automated process for a new node. It uses the embedded `Primordial Toolset` to hydrate its database, establish its identity, and join or create a Personal Agent Fabric, ensuring a secure-from-scratch start.
*   **Secure Tool Installation:** A workflow analogous to a secure package manager (`apt-get`).
    1.  **Fetch:** The `cdqn.sys.tool-fetch` tool downloads a `ToolDefinition` CDU.
    2.  **Quarantine & Analyze:** The `SecurAgent` initiates the **Content Hazard Analysis Protocol**, using its primordial security tools (`static-analyzer`, etc.) to scan the tool's code for malware.
    3.  **Approve:** Based on the hazard reports and user policy, the `SecurAgent` either approves the tool (moving it to the `cdqnDB`) or rejects it.

#### 6.3. Security & Integrity Protocols
*   **Decentralized Witness Protocol:** An active defense for high-risk actions. A `SecurAgent` cannot approve a sensitive operation without a concurrent, signed approval from a "witness" `SecurAgent` on another trusted device in the user's fabric.
*   **`HlcEcho` Protocol:** A decentralized integrity check orchestrated by `NetAgent`s to detect compromised nodes by comparing HLC timelines across the fabric.
*   **Cognitive Firewall:** A preventative protocol where all `Experience` CDUs are first peer-reviewed by the Council of Veracity for anomalies before being used for `OrchestrAgent` learning, thus preventing data poisoning attacks.
*   **Cognitive Re-Projection:** A recovery mechanism. An agent's cognitive state can be instantly rebuilt from the `cdqnDB`'s immutable log, starting from the last known-good `CognitiveSnapshot`. This allows for a "reload" to a sane state, surgically excising any corrupted learning.

---

### Conclusion

The cdqn ecosystem is a holistic, vertically integrated architecture designed from first principles to support a new paradigm of human-computer interaction. By combining a sovereign data model, a secure and decentralized network, an intent-driven language, and a sophisticated hierarchy of intelligent agents, it provides a foundation for a new class of applications that are not just tools, but true creative and analytical partners.

---

### Glossary

*   **Agent:** A stateful, long-lived, cognitive entity powered by an AI model, responsible for thinking and planning.
*   **CDU (Context Datas Unit):** The atomic, immutable data model for all information in the ecosystem.
*   **Cognitive Firewall:** The protocol that prevents agent data poisoning by having the Council of Veracity peer-review `Experience` CDUs before learning.
*   **Cognitive Re-Projection:** The recovery process of rebuilding an agent's cognitive state from a known-good snapshot in the immutable database log.
*   **Cognitive Snapshot:** A CDU that "tags" a specific point in the `cdqnDB`'s history as a known-good state for an agent.
*   **Content Hazard Analysis Protocol:** The mandatory "antivirus" scan, orchestrated by the `SecurAgent`, for all incoming tools and data.
*   **Council of Veracity:** The Mixture of Experts (MoE) team of specialized agents, each an expert in a specific `FSSFF` veracity.
*   **Decentralized Witness:** The active security protocol requiring a peer `SecurAgent`'s signature for high-risk operations.
*   **Execution Tiers:** A passive security hierarchy in the `cdqnRuntime` that limits the maximum capabilities of a tool based on its origin (e.g., Goal-Scoped vs. Trusted).
*   **FSSFF:** The veracity system: **F**actual, **S**emi-**F**actual, **S**emi-**F**iction, **F**iction, or **F**alse.
*   **GCR (Generate-Critique-Refine):** The core hierarchical reasoning protocol where a generated "first draft" is reviewed by other expert agents.
*   **Genesis Protocol:** The automated bootstrapping process for a new, clean node to securely provision itself.
*   **Goal-Scoped Tool:** An ephemeral tool defined inside a `goal` block for single-use logic, subject to the highest security restrictions.
*   **HLC (Hybrid Logical Clock):** A distributed timestamping mechanism that ensures a verifiable, causal order of events.
*   **OrchestrAgent:** The primary "thinking" agent that manages goals and learning.
*   **Personal Agent Fabric:** A user's private, secure, P2P network of their own Personal Nodes.
*   **PIG (Preference-Informed Generation):** The protocol where the `OrchestrAgent` uses past `Experience` records to write better prompts for the GCR workflow.
*   **Primordial Toolset:** The minimal set of essential, trusted tools embedded in the runtime binary, required for the Genesis Protocol.
*   **SecurAgent:** The security-focused agent responsible for permissions, approvals, and orchestrating defense protocols.
*   **Tool:** A stateless, ephemeral, deterministic function defined in `cdqnLang`, executed in a secure WASM sandbox.
*   **Tool Alias:** A short, user-defined name for a specific, versioned tool, declared in a `using` block for readability.
*   **Vibe Coding:** A paradigm of development focused on expressing high-level intent and outcomes, rather than low-level algorithms.
