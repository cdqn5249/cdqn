# Doc 3 - CDQN Ecosystem 
* **Version:** V1.0.0
* **Status:** DRAFT

## Abstract

The `cdqn` (Context Data Query Nodes) ecosystem is a new paradigm for building secure, intelligent, and auditable software. In a world where data is decentralized and AI agents must reason under uncertainty, `cdqn` provides a framework founded on three key principles: **Sovereign Nodes**, where each participant owns and controls its own data; **Behavioral Identity**, where trust is earned through verifiable actions, not secrets; and **Vibe Coding**, a new way of programming that focuses on expressing declarative intent through a simple, safe, and readable language. This document provides a comprehensive specification of the `cdqn` architecture, its philosophy, its core components, and its operational workflows, intended as a foundational guide for implementation.

---

## Part I: The Philosophy - A New Way of Thinking

### 1.1 The Problem: The Insecurity of "Trust Me"
Traditional software is built on the concept of the "function"—a block of code that we call and trust to perform an action. This trust is often blind. A function can have unintended side effects, access data it shouldn't, or contain hidden security flaws. In a decentralized, AI-driven world, this model of "trust me" is no longer tenable.

### 1.2 The `cdqn` Solution: "Show Me" through Verifiable Intent
`cdqn` inverts this model. Instead of passing data to trusted code, we pass **data that describes an intent** through a zero-trust runtime. This data is packaged in a secure, verifiable, and immutable structure called a `cdu`. The runtime then interprets this intent and orchestrates the execution of small, sandboxed, and stateless capabilities (WASI Components) to fulfill it.

This means every significant action in the system creates a permanent, auditable record, moving from a world of opaque operations to one of a verifiable, causal history. It’s a "show me" architecture.

---

## Part II: The Core Component - The `cdu`

Everything in the `cdqn` ecosystem revolves around the `cdu` (Context Data Unit). It is the atomic unit of information.

### 2.1 Anatomy of a `cdu`
Imagine a single, tamper-proof digital notecard with three sections:

1.  **ID:** A unique, cryptographic serial number (a SHA-256 hash). This makes it **immutable**.
2.  **Vibe:** The main content of the card—the "what."
3.  **Context:** The metadata in the margins—the "about," including its causal timestamp (`hlc`) and security permissions (`acl`).

### 2.2 Technical Schema: `cdu`
| Component | Description | Data Type Specification |
| :--- | :--- | :--- |
| **`id`** | The unique, content-addressed hash. | `String` |
| **`vibe`** | The core data payload. | `VibeData` Enum |
| **`context`**| The metadata. | `Map<String, ContextData>` |

```cdqn
// Data types available within the 'vibe'
enum VibeData { Null, Bool, Int, Float, String, Bytes, Array<VibeData>, Object }

// Data types available within the 'context'
enum ContextData { Scalar(VibeData), Relation(String) } // Relation points to another cdu's id
```

### 2.3 Relationships Part 1: The Lineage System (Causal Transformation)
*   **Purpose:** To record **causal, direct transformation**. It answers the question, "What was the direct input that produced this output?"
*   **Mechanism:** A `Relation` key named `derived_from` is added to a child `cdu`'s `context`, pointing to the hash `id` of its parent.
*   **Workflow:**
    1.  A `worker` is invoked with `input_cdu_A`.
    2.  The `worker`'s WASI component runs and produces `output_cdu_B`.
    3.  The `Orchestrator` finalizes `cdu_B`, automatically inserting `{ "derived_from": Relation("id_of_cdu_A") }` into its context before saving it. This creates an unbreakable, auditable chain of events.

### 2.4 Relationships Part 2: The Link System (Semantic Relationship)
*   **Purpose:** To declare a **conceptual, semantic relationship**. It answers the question, "How does the idea in this `cdu` relate to the idea in that `cdu`?"
*   **Mechanism:** A standalone `cdu` is created with the `link` schema.
*   **`Link` `cdu` Schema (`cdqn.io/schemas/link/v1.0`):**
    *   `vibe.targets` (Array<Relation>): The `cdu`s being linked.
    *   `vibe.link_type` (String Enum): The nature of the relationship (`equivalence`, `refutation`, `elaboration`, `inspiration`).
*   **Workflow:**
    1.  An `agent` analyzes `cdu_X` (an astrology text about the Sun) and `cdu_Y` (a sci-fi story about the Sun).
    2.  It determines they share a core subject.
    3.  It creates a `Link` `cdu` with `targets: [cdu_X, cdu_Y]` and `link_type: 'equivalence'`. This enriches the data graph, allowing future queries to find both `cdu`s when searching for information about the Sun.

---

## Part III: `cdqnLang` - The Language of Vibe Coding

`cdqnLang` is the primary interface for humans and agents to interact with the ecosystem. It is a declarative, statically-typed language with a unique, literate, pure-UTF-8 syntax designed for maximum clarity and safety.

### 3.1 Syntax Fundamentals
*   **Keywords and Indentation:** All code blocks are explicitly scoped with `define...end` or other keyword pairs. Scope is defined by consistent indentation.
*   **Explicit Typing:** Every variable must have an explicitly declared type (e.g., `String: name`).
*   **Top-Level Declarations:** All `import`, `define aliases`, and `define variable` statements must appear at the top of their scope, before any executable logic.
*   **Operator Set:**
    | Concept | Symbol | Example |
    | :--- | :--- | :--- |
    | Assignment | `←` | `Int: x ← 10` |
    | Member Access | `.` | `input.vibe.title` |
    | Type Association | `:` | `String: name` |
    | Data Flow / Pipe | `→` | `input → use "component:resizer"` |
    | Equality | `=` | `if x = 10` |

### 3.2 Key Constructs and Workflows

#### A. Data Definition
You define data using `cdu` literals. Dot-notation provides shortcuts for defining specific parts.

```cdqn
// A full cdu definition
define cdu matching "schema:user-profile"
    define vibe
        String: name, email ← "Alice", "alice@example.com"
        Int: age ← 30
    end vibe
end cdu

// A shortcut to define just the vibe
define cdu.vibe
    String: message ← "Hello, World!"
end cdu.vibe
```

#### B. Control Flow
Logic is expressed with explicit blocks for branching (`if`) and iteration (`for each`, `map`).

*   **Use Case: Conditional Action**
    ```cdqn
    // This workflow receives a user profile and invokes different workers based on age.
    define workflow process-user
        define aliases
            age ← input.vibe.age
        end aliases

        return if age ≥ 18
            input → use "worker:adult-processor"
        else
            input → use "worker:minor-processor"
        end if
    end workflow
    ```

#### C. Reusable Logic with `flow`
A `flow` is a pure, stateless, reusable block of logic, like a safe function.

*   **Use Case: A Reusable Tax Calculation**
    ```cdqn
    define flow calculate-tax
        expects
            Float: amount
        end expects
        returns Float

        Float: tax_rate ← 0.20
        return amount × tax_rate
    end flow

    // Usage in a workflow:
    Float: sale_tax ← calculate-tax using amount ← 150.75
    ```

#### D. Symbolic Aliasing for Science
`cdqnLang`'s most unique feature allows mapping scientific symbols to `worker`s.

*   **Use Case: Making Calculus Readable**
    ```cdqn
    // At the top of the file, the user "teaches" the language what '∫' means.
    import worker "community/math/numerical-integrator"
    alias ∫(f, from, to) using "worker:numerical-integrator"

    // Later, the user can invoke the worker using the mathematical symbol.
    // The compiler translates this into a standard worker invocation.
    cdu: integral_result ← ∫(my_function, from: 0.0, to: 1.0)
    ```

---

## Part IV: The Actors of the Ecosystem

### 4.1 `worker`: The Deterministic Executor
A `worker` is a non-autonomous entity that executes pre-defined `workflows`. It is the "hands" of the system.

*   **`worker` Definition Schema (`cdqn.io/schemas/worker-definition/v1.0`):**
    *   `vibe.workerID` (String): The worker's stable, logical name.
    *   `vibe.capability_manifest` (Array<String>): An explicit allow-list of the WASI component IDs it is authorized to run. This is a hard security boundary.
    *   `vibe.workflows` (Object): A library of named, multi-step workflows.

### 4.2 `agent`: The Cognitive Planner
An `agent` is the "brain." It is an autonomous entity, powered by a Language Model, for planning and decision-making.

*   **`agent` Definition Schema (`cdqn.io/schemas/agent-definition/v2.2`):**
    *   `vibe.agentID` (String): The agent's logical name.
    *   `vibe.lm_component_id` (String): The WASI component providing the Language Model capability.
    *   `vibe.tool_manifest` (Object): A curated map of conceptual tool names to the specific, whitelisted `worker` Logical IDs the agent is allowed to command.
    *   `vibe.trigger_policy` (Object): The conditions that activate the agent.

*   **Core Workflow: Agent Commanding a Worker**
    1.  A `user-chat-message` `cdu` is created that matches the `ProxyAgent`'s `trigger_policy`.
    2.  The `Orchestrator` activates the `ProxyAgent`.
    3.  The `agent`'s LM component is invoked. The LM's response is a decision: "I need to use the `resize_image` tool."
    4.  The `agent` consults its `tool_manifest`, finds that `resize_image` maps to `"worker:image-processor"`.
    5.  The `agent`'s final action is to create a `worker-invocation` `cdu`, commanding the `image-processor` worker to run its `create-thumbnail` workflow.
    6.  The `Orchestrator` detects this new `cdu` and begins the durable workflow execution for the worker.

---

## Part V: The Infrastructure - The `CDQN Node`

A `CDQN Node` is a sovereign, operational instance of the ecosystem.

### 5.1 The `cdqnRuntime`: The Modular Monolith
The primary executable, built in Rust on an asynchronous (`tokio`) foundation. It includes:
*   **The `Orchestrator`:** A durable, Temporal-like workflow engine. It manages the reliable, asynchronous execution of all `agent` and `worker` activities, persisting workflow state to `cdqnDB` so that tasks can survive restarts.
*   **Guardian Modules:** Trusted background services that manage the node's dynamic state and security.
*   **Executor Pool:** A pool of secure, sandboxed environments for running WASI components.

### 5.2 `cdqnDB`: The Causal Datastore
A specialized database binary designed for `cdu`s.
*   **Disk-First Durability:** Utilizes a **Write-Ahead Log (WAL)**. Every write is first committed to an on-disk log before being acknowledged, guaranteeing data is safe even in a crash.
*   **Optimized I/O:** Employs a **Log-Structured Merge-Tree (LSM-Tree)** architecture. All new data is written sequentially to disk, maximizing write throughput and hardware longevity, which is ideal for immutable data.
*   **Neural Graph Capabilities:** Maintains physical **subgraphs** for high-performance queries on dynamic metadata.

---

## Part VI: The Dynamic State Layer - The Living Network

A `cdu` is immutable, but its meaning and relevance evolve over time. This dynamic evolution is managed by the Guardian Modules, which store their findings in special, versioned "State" `cdu`s.

### 6.1 The Veracity System: Tracking Truthfulness
*   **Purpose:** To dynamically track the ecosystem's consensus on a `cdu`'s truthfulness.
*   **Mechanism:** The `VeracityGuardian` manages a `VeracityState` `cdu` for each piece of data. This state can be challenged by any entity creating a `VeracityClaim` `cdu`, which provides new `proof`. The Guardian weighs the new evidence against the old, considering the reputation and quality of the proof, and can demote a `Factual` `cdu` to `Semi-Factual` if the evidence is strong enough.
*   **Use Case: The Evolution of a "Fact"**
    1.  A `cdu` about a scientific study (`cdu_A`) is initially classified as `Factual`.
    2.  Months later, new studies (`cdu_B`, `cdu_C`) are published that refine the original findings.
    3.  A `ResearchAgent` submits a `VeracityClaim` to demote `cdu_A` to `Semi-Factual`, using `cdu_B` and `cdu_C` as `proof`.
    4.  The `VeracityGuardian` analyzes the claim. Seeing that the `proof` comes from highly-cited, `Factual` sources, it agrees and updates `cdu_A`'s `VeracityState`. Now, any agent using `cdu_A` will know its information is historical, not the current state-of-the-art.

### 6.2 The QoS System: Measuring Influence
*   **Purpose:** To quantify a `cdu`'s influence, relevance, and utility over time.
*   **Mechanism:** The `QoSGuardian` manages a `QoSState` `cdu` for each piece of data, tracking metrics like `views`, `derivations` (how much new work it inspired), `citations` (its authority), and `corroborations` (how many other nodes hold a copy).
*   **Use Case: Identifying Influential Ideas**
    An `agent` is researching a topic. It finds `cdu_X` (a dry, `Factual` report) and `cdu_Y` (a speculative, `Semi-Fiction` think-piece). By checking their `QoSState`, the agent sees that `cdu_Y` has a `derivations` score of 500, while `cdu_X` has a score of 2. The agent can now reason: "While X is the literal fact, Y was the truly influential idea that sparked a wave of innovation."

---

## Part VII: The `cdqNetwork` - A Federation of Sovereign Nodes

### 7.1 Behavioral Identity and the `Genesis cdu`
A node's identity is not based on a secret master key. It is anchored by its **`Genesis cdu`**, a public "birth certificate" created upon installation. Trust is **earned over time** based on consistent, verifiable behavior.

*   **`Genesis cdu` Schema (`cdqn.io/schemas/genesis-node/v1.0`):**
    *   `vibe.nodeID` (String): The node's chosen logical name.
    *   `vibe.initial_hlc` (HLC Object): The starting point of the node's history.
    *   `vibe.claimed_capabilities` (Object): A declaration of the node's hardware.
    *   `vibe.node_instance_id` (PublicKey): An ephemeral public key for the current runtime instance.

### 7.2 The `cdqnChallenge` and Reputation System
The network is self-policing through a proactive audit workflow.

*   **Workflow: The Challenge**
    1.  Node A's `NetworkGuardian` sends a `ChallengeRequest` `cdu` to Node B.
    2.  Node B's `cdqnRuntime` must construct a `ChallengeResponse` `cdu`. This response includes its current `hlc`, its database's `db_state_root` (a Merkle root fingerprint of its entire state), and is co-signed by the responding `agent` and the node's current `node_instance_id` key.
    3.  Node A receives the response and **cross-verifies** it with other peers. If Node B's claimed state is inconsistent, Node A can demand a **replay** of Node B's causal history to justify the discrepancy.
    4.  Failure to provide a consistent history is definitive proof of a compromise.
*   **Reputation Score:** The results of these continuous challenges are used to calculate a multi-faceted **Reputation Score** (`Liveness`, `Consistency`) for each peer, allowing the network to organically isolate and shun bad actors.

---

## Part VIII: The User Experience

The primary interface for Vibe Coding is the `ProxyAgent`, and its initial UI is the **OS Terminal**. The `CDQN SDK` will provide a `cdqn-chat` application for this purpose, which supports the full UTF-8 character set of `cdqnLang`.

---

## Glossary
*   **`agent`:** An autonomous, cognitive entity powered by a Language Model, responsible for planning.
*   **Behavioral Identity:** The principle that trust is earned through verifiable, consistent actions, anchored by a `Genesis cdu`.
*   **`cdu` (Context Data Unit):** The core, immutable data structure of the ecosystem.
*   **`cdqnLang`:** The simple, declarative language used for "Vibe Coding," featuring UTF-8 operators like `←` and `→`.
*   **`flow`:** A reusable, pure, stateless block of data transformation logic.
*   **Genesis `cdu`:** A node's public "birth certificate."
*   **Guardian Modules:** Trusted internal services of the `cdqnRuntime`.
*   **HLC (Hybrid Logical Clock):** A causally-aware timestamp.
*   **Lineage:** A causal, procedural link between `cdu`s (`derived_from`).
*   **Link `cdu`:** A conceptual, semantic link between `cdu`s, created by an `agent`.
*   **Nodal Sovereignty:** The principle that each node is the sole owner and authority for its own data.
*   **Orchestrator:** The durable workflow engine within the `cdqnRuntime`.
*   **QoS (Quality of Service):** A dynamic measure of a `cdu`s influence and utility.
*   **Veracity:** A dynamic measure of the ecosystem's consensus on a `cdu`'s truthfulness.
*   **`worker`:** A non-autonomous entity that deterministically executes pre-defined `workflows`.
