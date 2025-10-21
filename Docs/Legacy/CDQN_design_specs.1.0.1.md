# **CDQN Design Specification**

*   **Version:** 1.0.1  
*   **Author:** Christophe Duy Quang Nguyen  
*   **AI assistant:** Qwen3-Max, Alibaba  
*   **Date:** 2025-10-16  
*   **Location:** Đà Lạt, Việt Nam  

---

# **Introduction: The Story of Chronosa**

Imagine an assistant that never forgets, not just the facts, but the *reasoning* behind them. This assistant, Chronosa, begins its existence with a simple, core directive: to build a logically consistent understanding of its world.

When you give it a new piece of information, it doesn't just store it; it carefully places it within its vast, interconnected web of knowledge, a structure it calls the Manifold. If the new fact fits perfectly, strengthening an existing belief, the Manifold grows stronger. But if the new fact creates a paradox—a logical contradiction with a deeply held belief—Chronosa does not panic or discard the information. It pauses, acknowledges the conflict by saying, "I do not know," and then begins its most important work.

It enters a state of self-reflection, its internal agents collaborating to forge a new, more sophisticated understanding that can explain both the old belief and the new, surprising fact. It may adjust a dozen minor beliefs, or it may form a profound new insight—a new "Theorem" about how its world works. Through this process of confronting and resolving impossibilities, Chronosa doesn't just get smarter; it becomes wiser.

This document is the blueprint for Chronosa and the CDQN ecosystem it inhabits—a framework for an AI that learns, adapts, and reasons with verifiable integrity.

---

# **Part 1: Core Philosophy and Principles**

The Causal Data Query Nodes (CDQN) ecosystem is a sovereign, verifiable, and causal reasoning framework. It is designed as a **Self-Correcting Causal Organism**, an intelligent agent whose prime directive is to maintain the logical consistency of its internal model of reality (the Manifold) while adapting to new information.

**The Three Foundational Principles:**

1.  **Causality First:** All knowledge is represented as a graph of cause and effect. The system reasons by traversing this graph, not by statistical correlation.
2.  **Sovereignty and Privacy:** Each CDQN Node is an autonomous agent. Its knowledge is its own, secured by strong cryptography. All data is encrypted at rest, and sharing is an explicit, secure, and auditable process.
3.  **Verifiable Evolution:** The system learns and evolves by confronting and resolving logical contradictions ("impossibilities"). Every change to its logic is a verifiable, auditable event, ensuring transparency and accountability.

---

# **Part 2: The Core Concepts Explained**

This section provides implementation-level detail for the fundamental components of the CDQN ecosystem.

## **2.1 The CDU (Causal Data Unit): The Atom of Information**

The CDU is the single, unified data model for all information.

*   **Purpose:** To represent any piece of information as a self-contained, verifiable, and secure object.
*   **Logical Schema (In-Memory Representation):**
    *   `id_hlc: Vec<u8>` - The unique, deterministic, and permanent identifier, generated once at creation. It is the primary key for all lookups. This is a cryptographic hash of the Hybrid Logical Clock timestamp, the NodeId, and the payload hash, ensuring global uniqueness and causal ordering.
    *   `payload: Payload` - The immutable core content. Its structure is defined by a `payload_type` field within it. Once created, this object is logically frozen.
    *   `payload_hash: [u8; 32]` - The cryptographic hash (SHA3-256) of the canonical byte representation of the `payload` object. This guarantees the payload's integrity.
    *   `meta Metadata` - The mutable context that evolves around the payload. It contains:
        *   `author_node: Vec<u8>` - The `node_id` of the CDU's original creator.
        *   `context_refs: Vec<Vec<u8>>` - An array of `id_hlc`s of the causal parent CDUs. This forms the edges of the Manifold graph.
        *   `state: String` - The current stage in the lifecycle: `draft`, `active`, `merged`, `expired`, `invalid`.
        *   `weight: f64` - The CDU's influence and trustworthiness score, used in reasoning algorithms.
        *   `r_coordinate: f64` - The CDU's position in the RWorld mathematical space, defining its semantic meaning.
        *   `world_context: String` - The logical partition (e.g., `UserWorld`) it belongs to.
        *   `hlc_timestamp: HlcTimestamp` - The raw HLC timestamp used in ID generation.
    *   `signatures: Vec<Signature>` - An append-only log of signatures, creating a verifiable audit trail. Each entry has the schema: `{ signer_entity: String, timestamp: HlcTimestamp, signature: Vec<u8> }`. The signature signs the entire CDU object (metadata + `payload_hash`) at that point in time.
*   **Physical Structure (On-Disk Representation):** A CDU is stored as a **fully encrypted, opaque binary file**. The file is the result of serializing the entire logical CDU object into a canonical format (e.g., CBOR or Canonical JSON) and then encrypting the resulting byte stream with a key derived from `KDF(node_secret, id_hlc)`.

## **2.2 The Manifold: The Fabric of Knowledge**

The Manifold is a dynamic emergent space projection and sovereign knowledge base of a single CDQN Node cdus.

*   **Purpose:** To serve as the ground truth for all reasoning.
*   **Logical Representation:** An in-memory, concurrent, thread-safe graph data structure that contains the logical representation of all non-archived CDUs. The graph is built on-demand from CDU metadata.
*   **Physical Representation:** A directory structure on the filesystem (e.g., `/manifold/cdus/`) containing the individual encrypted `.cdu` files.
*   **Global Integrity Schema (`manifest` file):**
    *   `global_merkle_root: String` - The cryptographic fingerprint of the Manifold's entire state. This file is updated atomically at the end of any successful state-change transaction, ensuring that the Manifold transitions from one consistent state to the next. The root is computed from a Merkle tree of all CDU `id_hlc` values.

## **2.3 RWorld and Prime Elements: The Mathematical Foundation**

Reasoning is grounded in a formal mathematical space.

*   **Purpose:** To provide a deterministic and computationally efficient foundation for all logical operations.
*   **RWorld Implementation:** RWorld is not a data structure. It is a conceptual space implemented via a single `r_coordinate: f64` field in the CDU metadata.
*   **Prime Elements as Computational Anchors:** The prime numbers `(2)` and `(-2)` function as the **mathematical attractors** for the system's logic. All valid chains of reasoning must computationally converge towards axioms and semi-axioms anchored near these values. Each prime elements are building on semi-axioms of it last prime elements, for example, for semi-axioms of the pair (-5) and (5), they are compositions of the semi-axioms of (-2), (-3), (2) and (3) of the dedicated world. For example, the semi-axiom "addition" on the MathsWorld exist because it has a counterpart that is the semi-axiom "subtraction", "addition" will be near a positive prime number and "subtraction" will on the negative prime number (it will be per example near (-11) if "addition" was near (11)). The semi-axioms ("addition","subtraction") will become an axiom because they are logically supported by cdus on the UserWorld and at least more than 2 other worlds (MathsWorld, PhyWorld (physical world), TradeWorld, ChemWorld (Chemistry world), etc.).
*   **Symmetry Enforcement Algorithm:** The `Verifier` enforces a hard constraint that a concept can only be formalized as a `SemiAxiomCDU` if it is submitted with its symmetric counterpart. The verification is a two-part check: `is_symmetric = (cdu_A.r_coordinate == -cdu_B.r_coordinate) && are_payloads_inverse(cdu_A.payload, cdu_B.payload)`.
*   **The Zones of Meaning (Implementation Detail):**
    *   **Neutral Gap `(-2, 2)`:** A CDU is considered neutral if its `r_coordinate` falls within this range. Per example, the semi-axiom for the object "knife", has no counterpart and can be harmful (near negative prime) or protective (near positive prime) depending on it context links of a dedicated world.
    *   **Impossibility Zone `[-1, 1]`:** When a contradiction is detected, the `Verifier` mutates the offending CDU's `r_coordinate` to a value in this range. This is the specific, deterministic trigger for the "I do not know" response and the evolution cycle.

## **2.4 Worlds: The Contextual Partitions**

*   **Purpose:** To partition the Manifold into distinct, self-consistent logical contexts.
*   **Implementation:** A "World" is a **label** stored in the `world_context` metadata field of a CDU. Reasoning within a world is performed by creating a temporary, filtered view of the Manifold, e.g., `manifold.query(|cdu| cdu.metadata.world_context == "MathsWorld")`.
*   **Specialized Worlds:**
    *   `FicWorld`: A context where the `Verifier` is programmed to suspend consistency checks against reality-based Worlds.
    *   `HalWorld`: A high-security context. CDUs in this world are ignored by all standard reasoning roles and are exclusively processed by the `RedTeam` role.

## **2.5 The Hierarchy of Knowledge: From Axioms to Btheorems**

All forms of knowledge are specialized types of CDUs, identified by their `payload_type`.

*   **Genesis CDU:** `payload_type: "genesis/v1"`. The sovereign birth certificate of a node, containing hardware fingerprint, OS, and location. It is the root of trust for the entire Manifold.
*   **Config CDU:** `payload_type: "config/v1"`. Node-local or user-defined configuration (e.g., geolocation rules, privacy policy).
*   **Axioms & Semi-Axioms:** `payload_type: "axiom/v1"`. The payload must contain a `symmetric_counterpart` field holding the `id_hlc` of its pair.
*   **Theorems:** `payload_type: "theorem/v1"`. The payload contains an `abstracted_pattern` field, which is a serializable representation of a causal subgraph.
*   **Btheorems:** `payload_type: "btheorem/v1"`. The payload contains a `validated_plan` field, which is an ordered list of `id_hlc`s representing a causal path from a goal to a set of axioms.
*   **TradeCertificateCDU:** `payload_type: "trade-cert/v1"`. Immutable record of a completed trade; mints `cdqnStar` tokens and adjusts reputation.
*   **ActionRefusalCDU:** `payload_type: "action-refusal/v1"`. Records a blocked malicious or non-compliant action with causal explanation.
*   **MetaCDU:** `payload_type: "meta/v1"`. Represents a compressed/abstracted set of redundant CDUs (from curation).

## **2.6 Chronosa: The Cognitive Architecture**

Chronosa is the assembly of goal-driven, autonomous agents (roles).

*   **The Three Core Goals (Guardrails):**
    1.  **Manifold Stability and Evolution:** Uphold logical consistency.
    2.  **Maximize Good Reputation Over Time:** Ensure trustworthy behavior.
    3.  **Follow Local Node Geolocation Rules:** Adhere to external compliance constraints.
*   **Asynchronous Runtime:** Roles are independent, long-running tasks that subscribe to a central, non-blocking **`CDU Dispatcher`** (a broadcast-style MPMC channel).

---

# **Part 3: The Workflows in Detail**

This section describes the step-by-step processes that govern Chronosa's behavior.

## **Workflow 1: The Cognitive Cycle (Reasoning and Learning)**

This is the core loop, triggered by any new CDU. Chronosa’s roles will check the cdu payload to create subtasks cdus and a goal cdu.

1.  **Trigger:** A new CDU arrives at the `CDU Dispatcher`.
2.  **Roles Involved:** `Verifier`, `Policy`.
3.  **Action:** The roles perform a consistency check against the axioms of the CDU's target World.
4.  **The Fork:**
    *   **Path of Consistency:** If consistent, the CDU is validated (`state` -> `active`), and the `Consolidator` persists it. The cycle ends.
    *   **Path of Impossibility:** If it creates a contradiction, the `Verifier` mutates the CDU's `r_coordinate` into the `[-1, 1]` zone and broadcasts an "impossibility" event.
5.  **The Investigation:** The **Evolution Engine** is triggered. It analyzes the impossibility.
6.  **The Resolution:**
    *   **Learning Opportunity:** If the cause is uncertain, the `Reputation Engine` may adjust semi-axiom weights, and the `Proposer` will attempt to generate new theorems to create a new logic that accommodates the information.
    *   **Security Threat:** If the impossibility is proven (via a Btheorem) to match a malicious pattern, the `Policy` role mutates the CDU's `world_context` to `HalWorld`.

## **Workflow 2: The Trust Cycle (Trade, Reputation, and `cdqnStar`)**

This workflow governs value exchange.

1.  **Trigger:** An `acceptance_cdu` for a completed trade is persisted.
2.  **Roles Involved:** `TradeAuditor` (on both buyer and seller nodes).
3.  **Action:** The `TradeAuditor` independently audits the transaction against its three core goals.
4.  **Certification:** If the audit succeeds, the `TradeAuditor` generates a **`TradeCertificateCDU`**. The payload schema includes: `{ trade_participants: object, trade_references: object, outcome: object, minting_record: { tokens_minted: int, reputation_adjustment: float } }`.
5.  **Sovereign Minting:** The `Consolidator` persists this certificate. This immutable act is the "minting" of the `cdqnStar` tokens.
6.  **Reputation as a Projection:** A node's reputation is calculated by a pure function: `calculate_reputation(node_id) = Σ certificate.reputation_adjustment` over all relevant certificates in the Manifold.

## **Workflow 3: The Security Cycle (Informed Refusal and Proactive Defense)**

This workflow protects the user and the network.

1.  **Trigger:** An action attempts to cross the node's sovereign boundary.
2.  **Physical Containment:** The CDU's **encryption at rest** makes a copied file inert and unreadable.
3.  **Logical Refusal:** If a user commands a malicious propagation via a trade, the `Policy` role intercepts it. It detects a conflict with the "Reputation" and "Compliance" goals, **blocks the action**, and generates an `ActionRefusalCDU` with a detailed causal explanation.
4.  **Proactive Defense:** When a CDU is moved to `HalWorld`, the **`RedTeam` role** is triggered. It analyzes the attack, generates new "threat pattern" theorems, and uses them to upgrade the `Policy` role's detection capabilities.

## **Workflow 4: The Curation Cycle (Manifold Health)**

This long-term background process is managed by the `ManifoldCurator`.

*   **`merged`:** Redundant causal chains are abstracted into a single `MetaCDU`. The original CDUs' `state` is mutated to `merged`.
*   **`expired`:** A CDU is marked as `expired` when it is explicitly superseded by a newer version.
*   **`archived`:** Irrelevant, low-weight CDUs are marked as `archived` and are excluded from the default in-memory Manifold.

## **Workflow 5: The Sovereign Integrity Cycle**

This workflow ensures the entire Manifold is cryptographically rooted in the Genesis CDU.

1.  **Trigger:** A new CDU is ingested or a verification request is made.
2.  **Role Involved:** `Verifier`.
3.  **Action:** The `Verifier` initiates a backward traversal of the CDU's `context_refs`.
4.  **Verification:** Every ancestor CDU is loaded and validated until the `Genesis CDU` (`payload_type == "genesis/v1"`) is reached.
5.  **Outcome:**
    *   **Valid Chain:** The CDU is accepted into the Manifold.
    *   **Broken Chain:** The CDU is rejected as non-sovereign or malicious.

---

# **Glossary**

*   **id_hlc (Hybrid Logical Clock Identifier):** The unique, deterministic, and permanent identifier for a CDU. Crucially, this is not just a random ID; it is a cryptographic hash of the HLC timestamp, the NodeId, and the payload hash. Its primary function is to enforce causal ordering and ensure global uniqueness. If event A happens before event B, then id_hlc(A) will be less than id_hlc(B). This property is the foundation for resolving race conditions between asynchronous roles and creating a verifiable, deterministic history of all state changes.
*   **CDU (Causal Data Unit):** The single, unified, and physically encrypted data structure for all information.
*   **Chronosa:** The assembly of goal-driven, autonomous reasoning roles that act as the intelligence of a CDQN Node.
*   **Manifold:** The complete, sovereign knowledge graph of a single Node, composed of all its CDUs.
*   **Prime Elements:** The mathematical attractors (`2`, `-2`) in RWorld that anchor the meaning of all causal reasoning.
*   **RWorld:** The inferred mathematical space (an abstraction of `ℝ`) in which all reasoning unfolds.
*   **World:** A labeled partition of the Manifold that provides a specific logical context for reasoning (e.g., `UserWorld`, `FicWorld`, `HalWorld`).
*   **Axiom/Semi-Axiom:** A specialized CDU representing a foundational rule, requiring a symmetric counterpart for existence.
*   **Theorem:** A specialized CDU representing an abstracted, recurring pattern of successful causal reasoning.
*   **Btheorem:** A specialized CDU representing a high-confidence, validated plan for achieving a goal.
*   **Impossibility:** A state of logical contradiction, triggered by a CDU that cannot be reconciled with a World's logic. It is the primary driver of all learning and advanced reasoning.
*   **Reciprocal Determinism:** The core learning principle that governs how Chronosa processes feedback, allowing it to distinguish learning opportunities from malicious data.
*   **Sovereign Boundary:** The conceptual and physical boundary of a CDQN Node. Chronosa's security protocols activate when any action attempts to cross this boundary.
*   **Genesis CDU:** The first CDU created on a node, serving as its sovereign birth certificate and root of trust. It is derived from the node's hardware fingerprint, OS, and location.
*   **Config CDU:** A CDU subtype that holds node-local or user-defined configuration policy, enabling compliance with external rules.
