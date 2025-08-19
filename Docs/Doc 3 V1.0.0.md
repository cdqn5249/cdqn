# Doc 3: The CDQN Ecosystem - Official Documentation (V1.0.0)

* **Version:** V1.0.0
* **Date:** 2025-08-19T08:15:00Z
* **Agent:** Gemini: Google (2025-08-19)
* **Lead Author:** Christophe Duy Quang Nguyen
* **Human Contributors:** ...

**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)

**Changelog:** Doc 3 V1.0.0 is the first official documentation release for the cdqn ecosystem. It consolidates all validated designs from the initial specification phase, resolving all logical gaps and inconsistencies. This document is the new single source of truth for the entire architecture.

---

### Introduction: The Engine for Inspiration

Imagine a digital environment that doesn't just store your data, but understands its context, history, and value. Imagine an AI that doesn't just follow commands, but acts as a creative and economic partner, helping you build, simulate, and trade entire worlds. This is the vision of the cdqn ecosystem. It is not a tool you use, but a sovereign space you inhabit alongside a cohort of personalized agents that learn *with* you and act *for* you.

This ecosystem is designed for the "vibe coder"—the creator, the world-builder, the strategist who wants to build powerful agentic systems without getting lost in low-level complexities. It is a universe where your intent is the language, where your data is your sovereign territory, and where every interaction, from running a workflow to trading an asset, is secured by verifiable, causal proof. This document is the blueprint for that universe.

---

### Section 1: Core Philosophy & Principles

The entire architecture is built upon these foundational, non-negotiable principles.

#### 1.1. Primary Goal
To establish the cdqn ecosystem as the reference for **vibe coding** in the Rust ecosystem. It is designed to be an **engine for inspiration**, valuing the generative potential and kinetic energy of ideas over their simple factual correctness.

#### 1.2. The Three Pillars
1.  **Empower the Vibe Coder:** Provide a high-abstraction, intent-driven environment (`cdqnLang`).
2.  **Build a Secure & Sovereign Digital Home:** Create a private, user-controlled "Personal Agent Fabric" that evolves into a trustless, federated economy.
3.  **Create a Resilient & Intelligent Agentic Environment:** Build a foundation for inspiration, learning, and shared, simulated worlds.

#### 1.3. Non-Negotiable Principles
*   **Data Sovereignty:** The user has absolute ownership of their data.
*   **Asynchronous-Only User Experience:** The system is designed for a disconnected world; a user's core experience is never blocked by the state of another user.
*   **Verifiable Identity & Causality:** No anonymous actions. Every piece of data is signed, versioned with a Hybrid Logical Clock (HLC), and has a verifiable causal history.
*   **Explicitness:** No implicit or "magic" behavior.
*   **User Agency:** The user is always in control.

---

### Section 2: The Data Layer (The Causal Foundation)

The foundation of the entire ecosystem is the **Context Datas Unit (CDU)**, an immutable, cryptographically signed data structure. *Everything*—from a message, to a tool's source code, to a 3D model, to a license—is a CDU.

#### 2.1. The CDU Structure
Every CDU is a self-contained, auditable record, forming the bedrock of the Causal Ledger.

| Category | Field Name | Detailed Purpose |
| :--- | :--- | :--- |
| **Identity & History**| `Unique ID` | The single, globally unique identifier for this CDU. |
| | `Version` | The semantic version (`Major.Minor.Patch`) of the payload content. |
| | `HLC` | The Hybrid Logical Clock timestamp, ensuring a verifiable, causal order of events. |
| | `Lineage`| An array of parent `Unique ID`s, forming a directed acyclic graph (DAG) that provides a complete, auditable history. |
| **Payload Semantics**| `CDUType`| A mandatory string that defines the payload's schema and purpose (e.g., `cdqn.sys.ToolDefinition`). |
| | `Payload`| The immutable data content, serialized into a compact binary format. |
| | `PayloadHash` | A cryptographic hash of the `Payload`. |
| **Ownership & Licensing**| `OwnerAgentID` | The `AgentID` that created or currently owns this CDU. |
| | `Signature` | The cryptographic signature of the `PayloadHash` and metadata, signed by the `OwnerAgentID`. |
| | `LicenseID` | A mandatory pointer to the `cdqn.sys.License` CDU governing the use of this asset. |

#### 2.2. `cdqnDB`: The High-Performance Causal Store
To ensure maximum I/O performance and durability, `cdqnDB` is built on a **disk-first architecture**. It is an embedded **Log-Structured Merge-Tree (LSM Tree)** database.
*   **Sequential I/O Optimization:** The LSM Tree's design is crucial. It converts many small, random-seeming application writes into large, sequential writes on disk. This is dramatically faster on all modern storage hardware.
*   **Durability:** The database utilizes a **Write-Ahead Log (WAL)**. Transactions are first committed to this fast, sequential log, guaranteeing that no data is lost even in a crash, before being structured into the main database files.
*   **Veracity Worlds:** The database is logically partitioned into **Veracity Worlds** (Factual, Semi-Factual, Semi-Fiction, Fiction, False). These partitions serve as the foundation for the simulation layer, allowing different worlds to operate with different rules while being stored in the same efficient database.

---

### Section 3: The Language of Intent (`cdqnLang`)

`cdqnLang` is a visually intuitive, declarative language for orchestrating durable workflows. It is designed to be beginner-friendly while being powerful enough to manage complex, long-running processes.

#### 3.1. Syntax Philosophy
*   **Indentation-Based:** Code blocks are defined by their indentation level.
*   **Explicit End Tags:** Every block construct (`goal`, `plan`, `if`) is closed with a matching `end` tag (`end goal`, `end plan`, `end if`).
*   **No Brackets:** The language avoids `{}` and `()` where possible, preferring keywords.
*   **Strict Imports & Declarations:** All external tools (`using` block) and all variables (`variables` block) must be declared at the top of their scope.

#### 3.2. Validated Operators
`cdqnLang` uses expressive UTF-8 symbols for clarity.

| Operation | Symbol | Example |
| :--- | :--- | :--- |
| **Value Assignment** | `←` | `let x ← 10` |
| **Flow Capture** | `→` | `let result → task "http-get"` |
| **Equality/Inequality**| `=` / `≠` | `if x = 10` |
| **Comparison** | `≤`, `≥`, `<`, `>` | `if x ≥ 10` |
| **Logical** | `∧` (AND), `∨` (OR), `¬` (NOT) | `if (x > 0) ∧ (¬is_valid)` |
| **Math** | `+`, `−`, `×`, `÷` | `(x × 2) + y` |
| **Powers** | `²`, `³`, `ⁿ` | `x² + y³` |

Advanced maths used on scientific publications are support with a tool, and their symbols are sugar syntax recognised by the transpiler.

#### 3.3. Durable Execution
A `goal` is a durable workflow guaranteed to run to completion. The `policy` block declaratively manages failures, retries, and timeouts.

**Code Example: A Resilient Web Fetch**
```coffeescript
goal "Fetch critical data from an API, with a fallback."
    using
        http: "cdqn.std.http.v1.1"
        log: "cdqn.std.log.v1.0"
    end using
    variables
        let String: primary_url, fallback_url, final_content
    end variables
    plan
        // ... (Code from previous example) ...
        final_content → task http.get with primary_url
        policy
            retry
                max_attempts: 3
                initial_interval: "2s"
            end retry
            on_failure plan
                continue with (task http.get with fallback_url)
            end plan
        end policy
        // ...
    end plan
end goal
```

---

### Section 4: The Secure Runtime Environment

The ecosystem's security and portability are guaranteed by a two-part runtime architecture that creates a strict boundary between privileged and unprivileged code.

#### 4.1. The `cdqn Native Host`
This is a **privileged**, native Rust application that the user runs. It is the only component with direct access to system resources like the GPU, network card, and file system. It manages the main window, the graphics context, and the OS event loop.

#### 4.2. The `cdqnRuntime` (WASM Sandbox)
This is an **unprivileged**, sandboxed **WASM/WASI binary** that is hosted and executed *by* the `cdqn Native Host`. All agent logic, user-defined `goal`s, and `Tool` executions happen inside this secure sandbox. It cannot access any system resources unless they are explicitly and securely granted by the Native Host.

#### 4.3. Execution Tiers
The runtime enforces a passive, structural security model that limits the maximum possible permissions for a tool based on its origin.
*   **Tier 3 (Ephemeral):** Goal-Scoped Tools. `compute`-only, no I/O access. The highest level of security.
*   **Tier 1 (Trusted):** Tools installed from a whitelisted `ToolSource` after passing the Secure Installation workflow. Can request permissions, which must be approved by the user.
*   **Tier 0 (Kernel):** The `Primordial Toolset` embedded in the runtime. Has the necessary privileges to bootstrap the system.

---

### Section 5: The `cdqNetwork` - A Federated Economy

The `cdqNetwork` is a federation of sovereign virtual servers that interact through a decentralized economy. It is built on four pillars, replacing the need for a slow, centralized blockchain.

1.  **The Chronos Protocol (Causality):** Based on a gossip HLC protocol that ensures a shared, verifiable understanding of the causal order of events.
2.  **The Hermes Protocol (Barter):** A trustless, asynchronous protocol for two parties to trade any resource using a secure escrow mechanism.
3.  **The Aether Token (`cdqnStars`):** A Proof-of-Utility token generated by the protocol itself to reward users who contribute value to the network.
4.  **The Fama System (Reputation):** A reputation score calculated based on an agent's history of successful, value-weighted, and diverse trades.

**Use Case: A Secure, Trustless Trade**
Your `AssetAgent` wants to buy a `Live2DModel`.
1.  **Offer:** Your agent sends a `TradeOffer`.
2.  **Due Diligence (Causal Audit):** Before escrowing funds, your `NetAgent` performs a **Causal Audit**. It traces the history of the asset being offered by contacting the *original source* of that asset (as listed in its public `TradeSettlement` receipts) to get an independent, cryptographic confirmation of its authenticity and causal history.
3.  **Settlement:** The audit passes. The trade is executed via the Hermes Protocol, and a co-signed `TradeSettlement` CDU is created. Both traders' `Fama` reputations are updated.

---

### Section 6: The Worlds (Simulation & Rendering)

The ecosystem is a platform for building, simulating, and sharing interactive worlds.

#### 6.1. World Definition
A `cdqn.sys.WorldDefinition` CDU declaratively defines a world's properties, its home **Veracity Partition** (e.g., `Fiction`), and its governing `PhysicsLaw`s, which are high-performance `RustWasm` `Tool`s.

#### 6.2. The Decoupled Architecture
For a smooth UX, the system uses an **Authoritative Simulation Host** pattern. A powerful node in your Personal Agent Fabric runs the heavy simulation logic, while your local device acts as a lightweight **Rendering Client**, ensuring a fluid 60fps experience.

#### 6.3. Ensuring a Smooth User Experience
The Native Host and runtime work together to adapt to the client's hardware capabilities:
*   **Dynamic Level of Detail (LODs):** The `Live2DModel` CDU can store multiple versions of textures and physics. The system automatically pushes the appropriate LOD based on the client's benchmarked performance.
*   **Asynchronous Asset Loading:** The Native Host loads all large assets on a background thread to prevent the main rendering loop from freezing.
*   **Client-Side Logic Offload:** A second, lightweight WASM runtime can be run on a background thread on the client to handle non-rendering tasks like decompressing state updates and interpolating movement, keeping the main thread free for GPU communication.

---

### Section 7: The Social Contract (Licensing & IP)

A license is a set of verifiable and executable rules bound to an asset.
*   **License as Data:** A `cdqn.sys.License` CDU defines a license's rules in a machine-readable format.
*   **Default Protection:** All new assets are automatically licensed under the **BaDaaS License**. Users can re-license their work at any time.
*   **Automated Enforcement:** The `AssetAgent` and `cdqnRuntime` automatically check and enforce license permissions before any trade or use of an asset.
*   **Economic Incentive for Openness:** The `Stargen` protocol rewards the facilitation of trades involving open-source assets more generously, creating an economic incentive for sharing.

---

### Glossary

*   **Agent:** A stateful, long-lived, cognitive entity that acts on the user's behalf.
*   **AssetAgent:** The specialist agent for managing, valuing, and trading assets.
*   **Causal Audit:** The core security protocol for verifying a trading partner's economic history by cross-referencing their causal ledger with third parties.
*   **CDU (Context Datas Unit):** The atomic, immutable, signed data structure for all information.
*   **`cdqNetwork`:** The federated network of all user-owned Personal Agent Fabrics.
*   **Execution Tiers:** A passive security hierarchy (Kernel, Trusted, Ephemeral) that limits a tool's maximum capabilities based on its origin.
*   **Fama System:** The reputation protocol based on an agent's history of successful trades.
*   **HLC (Hybrid Logical Clock):** A distributed timestamping mechanism ensuring a verifiable causal order of events.
*   **Native Host:** The privileged, native Rust application that runs the sandboxed `cdqnRuntime`.
*   **Personal Agent Fabric:** A user's private, secure, P2P network of their own trusted devices.
*   **`Stargen`:** The protocol-native process of minting new `cdqnStars` as a reward for contributing value to the network.
*   **Tool:** A stateless, deterministic function (in `cdqnLang` or Rust) executed in a secure WASM sandbox.
*   **Vibe Coding:** The core philosophy of expressing high-level intent over low-level algorithms.
*   **WAL (Write-Ahead Log):** A durability mechanism in `cdqnDB` that ensures transactions are safely logged before being applied.
