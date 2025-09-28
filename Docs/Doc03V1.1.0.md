*   **Version** : 1.0.0
*   **Date** : 28 September 2025
*   **Author** : Christophe Duy Quang Nguyen
*   **Vibe Coding Engine** : Gemini Flash-Lite Latest, Google
---
# The Chronos Model

The Chronos Model is a **Symbolic, Self-Correcting, Asynchronous Reasoning Engine** operating on an immutable ledger, governed by explicit Axioms and optimized for speed using fast arithmetic.

## Section 1: Architectural Mandate (K-C-S-U Hierarchy)

### K-Module (Kernel: Pure Logic and Reasoning)

The K-Module executes the core intelligence using the Four Primitives. It initiates asynchronous calls to the C-Module for all data access.

| Component | Role | Primary Workflow Step |
| :--- | :--- | :--- |
| **Primitives ($\mathcal{V}, \mathcal{C}, \mathcal{I}, \mathcal{L}$)** | Execute the core mathematical logic of the system. | **Synchronous Execution** within the K-Module's context, but all I/O calls are non-blocking futures awaited by the C-Module. |
| **Axiom Set Storage** | Stores all validated rules (Axioms) that govern logic, semantics, and security. | Accessed by $\mathcal{I}$ and $\mathcal{V}$ via asynchronous requests to the C-Module ledger. |
| **Schema Registry** | Stores the mapping between concepts (Primes) and data structures. | Accessed by $\mathcal{D}$ (conceptual) and $\mathcal{I}$ to interpret CDU payloads. |

### C-Module (Core Runtime: Execution and Security Enforcement)

The C-Module is the asynchronous backbone, managing state, time, security, and resource allocation.

| Component | Role | Primary Workflow Step |
| :--- | :--- | :--- |
| **Asynchronous Runtime** | Manages task scheduling and concurrency for all K-Module primitive calls. | Polls for new CDU events and schedules K-Module tasks. |
| **K.HLC Generator** | Generates the next sequential, globally consistent HLC value upon request. | **Synchronous** generation, but the request/response is asynchronous to the main flow. |
| **K.CryptoCore** | Handles all cryptographic operations (hashing, signing, verification) using ephemeral keys. | Called by $\mathcal{V}$ and S/U modules to secure new CDUs. |
| **Gas Metering** | Tracks resource consumption for every operation requested by K, S, or U. | Enforces Axiom A7; rejects operations exceeding allocated Gas. |
| **Replay Cache** | Stores recently processed CDU IDs to prevent re-execution of signed actions. | Checked by $\mathcal{V}$ and $\mathcal{C}$ before processing/resolving. |
| **Local Reputation Ledger** | Stores the **mutable** scores for all known authors. | Updated only when a CDU authored by that entity is successfully appended. |
| **Harm Guardrail** | Intercepts proposed actions from $\mathcal{C}$ and vetoes execution if harm is predicted. | **Synchronous Veto** based on Harm Axioms. |

### S/U Modules (Application Layer: Sovereignty and Interaction)

These modules are the interface between the Chronos Model and the external world (users, other systems, CI/CD).

| Component | Role | Primary Workflow Step |
| :--- | :--- | :--- |
| **CDU Factory** | Packages raw data into the final CDU structure, requests HLC from C, and requests signing from K.CryptoCore. | Creates the final, signed CDU ready for submission. |
| **Local Reputation Manager** | Tracks the node's own reputation score and initiates updates based on local success/failure. | Updates local score based on $\mathcal{V}$'s validation result. |
| **`cdqnLang` Transpiler** | Translates high-level code into sequences of **Data CDUs** (e.g., `RELATIONAL_TRIPLE`s or `QUERY_REQUEST`s). | Generates the input data for the K-Module. |
| **Network Gossip Handler** | Manages sending and receiving CDUs over the P2P network using `cdqnProt`. | Relays validated CDUs to neighbors and forwards requests to C-Module. |

### Operational Workflow Example: Processing a New Fact

This illustrates the asynchronous flow across the modules:

1.  **Creation (S/U):** An S/U module creates raw data, packages it into a proposed CDU structure, and requests an HLC from the C-Module.
2.  **Signing (C $\rightarrow$ K):** The C-Module passes the data to **K.CryptoCore** to generate the signature using an ephemeral key.
3.  **Validation ($\mathcal{V}$):** The C-Module passes the signed CDU to **Primitive $\mathcal{V}$ (K-Module)**. $\mathcal{V}$ checks the signature (via K.CryptoCore), verifies HLC ordering (via K.HLC), checks reputation (via C-Module Ledger), and assigns initial Polarity.
4.  **Persistence (C):** If $\mathcal{V}$ passes, the C-Module commits the CDU to the immutable ledger.
5.  **Asynchronous Processing (K):** The C-Module notifies the **Asynchronous Runtime** of the new CDU. The runtime schedules Primitive $\mathcal{C}$ and $\mathcal{L}$ tasks to process the new data point against the existing state and learning models.
6.  **Inference Trigger ($\mathcal{I}$):** The new CDU might trigger Primitive $\mathcal{I}$ to check if this new fact allows any existing Axioms to resolve a pending query or generate a new conclusion.

This structure ensures that every step is either a fast, pure logical operation (K) or a securely managed, non-blocking execution step (C).
---

## Section 2: The Causal Data Unit (CDU) Schema

The CDU is the immutable data structure, optimized for fast comparison and arithmetic operations.

| Field Name | Data Type (Conceptual) | Purpose |
| :--- | :--- | :--- |
| `id` | `[u8; 32]` (Hash) | Content Address. |
| `parent_ids` | `Vec<[u8; 32]>` | Causal Links. Must satisfy Axiom A1. |
| `hlc_timestamp` | `u64` | Temporal Ordering. |
| `author_id` | `[u8; 16]` | Persistent Identity Handle. |
| `polarity` | `i8` ($\{-1, 0, 1\}$) | Guardrail Flag (Urgency). |
| `dynamic_valence` | `f32` | Calculated semantic position $V_{\text{dynamic}}$. |
| `prime_indices` | `Vec<u32>` | List of Prime Numbers anchoring concepts. |
| `data_type_id` | `u8` | Identifies payload structure (e.g., AXIOM, TRIPLE, REPUTATION\_UPDATE). |
| `payload_hash` | `[u8; 32]` | Integrity hash of the data payload. |
| **`author_reputation`** | `f32` | Author's reputation score **at the time of signing**. |
| `payload_data` | `Vec<u8>` (Opaque) | The actual data content. |

---

## Section 3: The Four Core Mathematical Primitives (K-Module Logic)

These primitives execute logic using only **Addition, Multiplication, Comparison, and Counting**.

### Primitive 1: The Discrete Polarity Assigner ($\mathcal{P}$)
*   **Function:** Assigns discrete Polarity ($\{-1, 0, 1\}$) based on explicit **Polarity Assignment Axioms** via **Comparison**.

### Primitive 2: The HLC-Constrained State Resolver ($\mathcal{C}$)
*   **Function:** Resolves the current state by maximizing HLC, filtering based on World Context (A4), and checking for predicted harm.
*   **Key Operation:** **Comparison** (HLC maximization) and **Counting** (for simulation depth limit). **Crucially, it resolves reputation scores by searching the immutable Reputation CDU Chain.**

### Primitive 3: The Symbolic Inference Engine ($\mathcal{I}$)
*   **Function:** Performs deductive proof by matching premises against **Inference Axioms**.
*   **Key Operation:** **Lookup/Comparison** against known facts. Orchestrates **parallel asynchronous searches** and uses **Abstract Pattern Matching** for adaptation.

### Primitive 4: The Pattern Induction Operator ($\mathcal{L}$)
*   **Function:** Learns by analyzing successful **Bpaths** to generate new **Meta-Axioms**.
*   **Key Operation:** **Counting** sequence frequencies and checking against thresholds ($T$), modulated by the **Average Valence** of the path's terminal CDUs.

---

## Section 4: Dynamic Semantics and Learning

The system's intelligence is derived from its ability to calculate meaning and self-correct its rules.

*   **Dynamic Valence:** Calculated via **Weighted Summation (Multiplication/Addition)** of neighbor Polarity, decayed by HLC distance (Axiom A10). Meaning is anchored by **Prime Elements** ($P_3, P_5, \dots$).
*   **Self-Correction:** Contradictions generate **`IMPOSSIBILITY_CDU`s** (Polarity -2), triggering Primitive $\mathcal{L}$ to propose revisions to Knowledge Axioms.
*   **Trust Maintenance:** Reputation is **immutable per CDU**. New reputation scores are established by creating new **Reputation CDUs** linked causally to the previous one. The C-Module enforces reputation thresholds during CDU creation validation.

---

## Section 5: Validated Core Axioms (The TwinWorld Foundation)

In the Chronos Model, **Worlds** are not continuous sensory streams (like in traditional World Models); they are **Parameterized Contexts** that define the specific set of **Axioms and Constraints** under which the Chronos Model performs its symbolic reasoning.

A World is essentially a **specific, isolated configuration of the Rule Set** that governs how the Primitives interpret the immutable CDU ledger.

### The Three Types of Worlds

The Chronos Model manages three distinct ontological layers, all rooted in the same underlying CDU ledger:

#### 1. The TwinWorld (The Ground Truth Reality)

The TwinWorld represents the system's best, most trusted understanding of the actual, shared reality.

*   **Definition:** The set of all CDUs whose state is resolved using the **Primary Axiom Set** (the core, high-certainty rules) and where **Axiom A4 (TwinWorld Filter)** is active, pruning all Negative Polarity CDUs from the operational state.
*   **Purpose:** To guide real-world actions, maintain system stability, and serve as the source of truth for reputation and core knowledge.

#### 2. Cloned Worlds (Persistent Hypothesis Testing)

Cloned Worlds allow for the exploration of counterfactuals or alternative physical/logical laws.

*   **Definition:** A persistent fork of the ledger state, anchored at a specific **HLC** in the main chain.
*   **Parameterization:** A Cloned World is defined by loading the TwinWorld's base Axioms and then **overriding or injecting specific Constraint CDUs** (e.g., a `WORLD_CONSTRAINT` CDU that changes the value of Axiom A16: Kinematics).
*   **Purpose:** To run long-term simulations or test complex theories (like the impact of a new Meta-Axiom) without affecting the operational TwinWorld.

#### 3. Virtual Worlds (Ephemeral Calculation Space)

Virtual Worlds are temporary, in-memory contexts for rapid, low-cost calculation.

*   **Definition:** A temporary, in-memory projection initialized with a subset of CDUs (the initial state) from the TwinWorld or a Cloned World.
*   **Purpose:** To perform rapid, short-term simulations or test immediate consequences of a proposed action *before* it is submitted for full validation. They allow the system to explore paths that might temporarily violate TwinWorld constraints, provided the final result is either discarded or committed as a new, validated CDU.

### How Worlds Affect Chronos Primitives

The World Context dictates which rules the Primitives use:

*   **Primitive $\mathcal{C}$ (Resolver):** Uses the World Context to apply the correct **Polarity Filter** (A4).
*   **Primitive $\mathcal{I}$ (Inference Engine):** Uses the World Context to load the correct **Axiom Set** (e.g., TwinWorld Axioms vs. Cloned World Physics Axioms) for deductive proof.
*   **Primitive $\mathcal{D}$ (Valence Calculator):** The calculation of $V_{\text{dynamic}}$ is influenced by the World Context, as the Axioms defining neighbor influence (A10) and Prime Mappings (A11) can be context-dependent.

In essence, **Worlds are the semantic environments** that define the rules of engagement for the Chronos Model's symbolic reasoning.

These Axioms define the system's fundamental logic, security, and semantic structure.

| Axiom Name | Type | Purpose | Key Operation |
| :--- | :--- | :--- | :--- |
| **A1: Causal Precedence** | `LINKAGE_RULE` | Enforces strict temporal ordering. | Comparison (HLC) |
| **A2: Replay Prevention** | `LINKAGE_RULE` | Prevents re-execution of signed actions. | Comparison (ID Check) |
| **A3: State Resolution Rule** | `STATE_RULE` | Defines current reality via MAX HLC. | Comparison (HLC) |
| **A4: TwinWorld Filter** | `WORLD_CONSTRAINT` | Prunes Negative Polarity CDUs from operational planning. | Comparison (Polarity) |
| **A5: Harmful Inference Block** | `INFERENCE_RULE` | Prevents deriving positive conclusions from negative premises. | Comparison (Polarity) |
| **A6: Pure Function Guarantee** | `RUNTIME_RULE` | Enforces deterministic execution in K-Module. | Architectural Constraint |
| **A7: Gas Budget Enforcement** | `RUNTIME_RULE` | Prevents DoS by metering operations. | Comparison (Cost vs. Gas) |
| **A8: Identity Chain Rule** | `LINKAGE_RULE` | Enforces traceability back to the MC-sponsored Origin. | Comparison (Author ID Chain) |
| **A9: Pattern Induction Threshold** | `LEARNING_RULE` | Defines when a sequence becomes a reliable Meta-Axiom. | **Counting** & Comparison ($\bar{V}$) |
| **A10: Causal Distance Weighting** | `TEMPORAL_WEIGHT_RULE` | Modulates neighbor influence on $V_{\text{dynamic}}$ based on HLC distance (using **Multiplication**). |
| **A11: Core Lexical Anchors** | `LEXICAL_ANCHOR_RULE` | Maps fundamental vocabulary to initial Prime Elements. |
| **A12-A14:** | `INFERENCE_RULE` | Foundational Mathematics (Commutativity, Transitivity, Non-Contradiction). |
| **A15-A16:** | `INFERENCE_RULE` | Basic Applied Science (Conservation, Kinematics). |

---

## Section 6: Glossary of Core Concepts

| Term | Definition | Key Mechanism |
| :--- | :--- | :--- |
| **CDU** | **Causal Data Unit.** The immutable, content-addressed container. | Immutability |
| **HLC** | **Hybrid Logical Clock.** Ensures strict, distributed temporal ordering. | K.HLC Module |
| **K-Module** | **Kernel Module.** Hosts the **Four Core Primitives** ($\mathcal{V}, \mathcal{C}, \mathcal{I}, \mathcal{L}$). | Root of Logic |
| **C-Module** | **Core Runtime.** Manages asynchronous execution, I/O, Gas Metering, and enforces the **Harm Guardrail**. | Execution Environment |
| **Axiom CDU** | A special CDU defining a rule. These are the system's explicit knowledge base. | Symbolic Logic |
| **Polarity** | A discrete value ($\{-1, 0, 1\}$) indicating immediate semantic valence for guardrail checks. | CDU Metadata |
| **Dynamic Valence ($V_{\text{dynamic}}$)** | A real number calculated locally based on neighbor influence, representing nuanced semantic position relative to Prime Anchors. | Primitive $\mathcal{D}$ |
| **Prime Elements** | Fixed, discrete numbers ($P_3, P_5, \dots$) that define hierarchical anchors for semantic certainty. | Semantic Anchors |
| **Prime Ideal** | A concept whose $V_{\text{dynamic}}$ is consistently near a Prime Anchor, representing a stable, high-certainty truth. | Emergent Property |
| **Gpath** | **Golden Path.** A complete reasoning sequence analyzed by Primitive $\mathcal{L}$ to find successful (Bpath) and failed (Dpath) outcomes. | Learning Input |
| **Meta-Axiom** | A new Axiom CDU generated by Primitive $\mathcal{L}$ that codifies a successful, abstract pattern found in Gpaths. | Learned Knowledge |
| **Reputation Score** | A dynamic score stored immutably in a **Reputation CDU Chain**, reflecting author trust earned via successful actions. | Local Trust Metric |
