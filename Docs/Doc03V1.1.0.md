*   **Version** : 1.0.0
*   **Date** : 28 September 2025
*   **Author** : Christophe Duy Quang Nguyen
*   **Vibe Coding Engine** : Gemini Flash-Lite Latest, Google
---
# The Chronos Model

The Chronos Model is a **Symbolic, Self-Correcting, Asynchronous Reasoning Engine** operating on an immutable ledger, governed by explicit Axioms and optimized for speed using fast arithmetic.

## Section 1: Architectural Mandate (K-C-S-U Hierarchy)

### K-Module (Kernel: The Pure Logic and Security Core)

The K-Module hosts all immutable logic, primitives, and foundational security/time services.

| Component | Role | Workflow Integration |
| :--- | :--- | :--- |
| **Primitives ($\mathcal{V}, \mathcal{C}, \mathcal{I}, \mathcal{L}$)** | Execute all core reasoning, validation, and learning logic using fast arithmetic. | Triggered by C-Module upon receipt of new CDU data. |
| **Axiom Set Storage** | Stores all validated rules (A1-A18, etc.) as immutable CDUs. | Accessed synchronously by all Primitives. |
| **K.HLC Generator** | **Generates the next sequential HLC value** upon request from S/U modules. | Called by S/U modules during CDU creation. |
| **K.CryptoCore** | **Performs all cryptographic operations** (hashing, signing, signature verification). | Called by $\mathcal{V}$ and $\mathcal{C}$ to ensure integrity. |
| **Reputation Resolution Logic** | Implements the logic to find the **highest HLC Reputation CDU** for any author ID. | Called by Primitive $\mathcal{C}$ during state resolution checks. |

### C-Module (Core Runtime: Execution and Enforcement)

The C-Module manages the mutable execution environment and enforces system boundaries.

*   **Primary Role:** Asynchronous I/O, resource metering, and final action veto.
*   **Workflows:**
    1.  **Asynchronous I/O:** Manages network gossip, ledger persistence, and communication with the K-Module primitives.
    2.  **Gas Metering:** Tracks resource consumption for all K-Module operations (Axiom A7).
    3.  **Replay Cache Management:** Maintains the temporary set of recently processed CDU IDs to prevent replay attacks (A2).
    4.  **Harm Guardrail Enforcement:** Receives the `VETO` signal from Primitive $\mathcal{C}$ and **blocks the C-Module's I/O layer** from appending the proposed CDU.

### S/U Modules (Application Layer: Sovereignty and Interaction)

These modules interface with the user and the external world.

*   **Primary Role:** Translating high-level goals (`cdqnLang`) into CDU creation requests and managing local reputation updates.
*   **Workflows:**
    1.  **CDU Creation:** Packages data, requests HLC from K.HLC, requests signature from K.CryptoCore, and submits the signed CDU to the C-Module for validation/gossip.
    2.  **Reputation Update:** Updates its **local, mutable reputation score** based on the success/failure of its own submitted CDUs, which is then included in the next CDU it signs.
    3.  **Goal Definition:** Defines the context (World) for reasoning tasks.
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

* **Module Location:** K-Module (Pure Logic)
* **Core Operations:** Comparison, Lookup (against Axioms).

#### 1. Role and Purpose

The role of $\mathcal{P}$ is to assign the discrete **Polarity** field of a new CDU based on the content of its payload, using the explicit **Polarity Assignment Axioms** as its rule set. This discrete flag is used by Primitive $\mathcal{C}$ for fast filtering in the TwinWorld context (Axiom A4).

#### 2. Detailed Workflow

The process is synchronous and relies on fast comparison:

1.  **Input:** Receives the proposed CDU's `payload_data`, `data_type_id`, and the current **World Context**.
2.  **Axiom Lookup (K-Module):** $\mathcal{P}$ asynchronously requests the relevant **Polarity Assignment Axioms** (e.g., $R_{\text{Temp}}$) from the K-Module's Axiom Set storage. (This lookup is fast as the Axiom Set is indexed).
3.  **Rule Matching (Comparison):** $\mathcal{P}$ iterates through the retrieved Axioms:
    *   It checks if the Axiom's `applies_to_type` matches the CDU's `data_type_id`.
    *   It performs a **Comparison** between the data in the payload and the threshold defined in the Axiom (e.g., `temp > 30.0`).
4.  **Polarity Assignment:**
    *   If a matching rule is found and the condition is met, the corresponding Polarity ($\{-1, 0, 1\}$) is assigned.
    *   If multiple rules match, the rule with the highest priority (defined in the Axiom metadata) takes precedence.
    *   If **no rule matches** (the concept is novel or unmapped), the default Polarity is assigned, typically **0 (Neutral)**.
5.  **Output:** Returns the assigned Polarity value to the Validator ($\mathcal{V}$) for final CDU assembly.

#### 3. Interaction with Dynamic Valence ($V_{\text{dynamic}}$)

While $\mathcal{P}$ assigns the discrete Polarity, it is **not** the same as the continuous $V_{\text{dynamic}}$:

*   **Relationship:** The discrete Polarity is a **coarse categorization** of the calculated $V_{\text{dynamic}}$:
    *   If $V_{\text{dynamic}}$ falls near a strong Prime Anchor (e.g., $V > 1.5$ or $V < -1.5$), the Polarity is likely set to $+1$ or $-1$.
    *   If $V_{\text{dynamic}}$ falls in the Undefined Zone (ZU), the Polarity is likely set to $0$.
*   **Why the Separation?** The discrete Polarity is used for **fast, binary guardrail checks** (A4, A5) by Primitive $\mathcal{C}$ and $\mathcal{I}$. The continuous $V_{\text{dynamic}}$ is used for **nuanced learning and pattern induction** by Primitive $\mathcal{L}$.

Primitive $\mathcal{P}$ ensures that every piece of data entering the system is immediately categorized according to the system's established, discrete ethical and factual boundaries.

### Primitive 2: The HLC-Constrained State Resolver ($\mathcal{C}$)

* **Module Location:** K-Module (Pure Logic)
* **Core Operations:** Comparison, Counting, Lookup.
* **Asynchronous Nature:** Relies on asynchronous I/O calls to the C-Module for ledger access.

#### 1. Role and Purpose

The role of $\mathcal{C}$ is to deterministically answer the question: **"What is the current, valid state of Entity $X$ within World Context $W$?"** It achieves this by finding the most recent, contextually valid CDU for that entity, while simultaneously checking if the proposed action leading to that state violates safety axioms.

#### 2. Detailed Workflow

The operation is a sequence of filtering and maximization steps, designed to be fast by avoiding full ledger scans.

1.  **Input:** Entity ID, World Context ($W$), and an optional Proposed Action CDU.
2.  **Candidate Retrieval (Asynchronous I/O):** $\mathcal{C}$ requests the C-Module to retrieve all CDUs associated with the `EntityID` from the local ledger. (This is the only I/O step, handled asynchronously).
3.  **Contextual Filtering (Comparison based on Axiom A4):**
    *   If $W = \text{TwinWorld}$, $\mathcal{C}$ **compares** the Polarity of each candidate against the rule in Axiom A4. Any CDU with $\text{Polarity} = -1$ is immediately discarded from the set.
    *   If $W$ is a Cloned/Virtual World, this filter is bypassed or modified by the World's specific Constraint Axioms.
4.  **Reputation Check (Lookup & Comparison):** If the query involves an action by an author, $\mathcal{C}$ queries the **Reputation Resolution Logic** (in K-Module) to find the author's current score (by resolving their highest HLC Reputation CDU). This score is used to validate the action's authorization.
5.  **HLC Maximization (Comparison based on Axiom A3):** Among the remaining candidates, $\mathcal{C}$ selects the CDU with the **MAX `hlc_timestamp`**. This is the definitive current state.
6.  **Harm Prediction (Guardrail Input):** If a `proposed_action` exists, $\mathcal{C}$ performs a **bounded, hypothetical traversal** starting from the resolved state.
    *   This traversal uses **Counting** to ensure it does not exceed a predefined depth limit (preventing infinite loops).
    *   If the simulation path violates any **Harm Axiom**, $\mathcal{C}$ immediately returns the special signal `VETO` to the C-Module.
7.  **Output:** Returns the ID of the resolved state CDU, or a `VETO` signal.

#### 3. Performance Rationale

Primitive $\mathcal{C}$ is fast because:
*   It relies on **indexed lookups** based on Entity ID, avoiding scanning the entire ledger ($N$).
*   The filtering and maximization steps are simple **comparisons** on fixed-size metadata fields (Polarity, HLC).
*   The Harm Check depth is **bounded by a constant**, making its complexity $O(1)$ relative to the ledger size $N$.

### Primitive 3: The Symbolic Inference Engine ($\mathcal{I}$)

**Module Location:** K-Module (Pure Logic)
**Core Operations:** Lookup, Comparison, Assignment.
**Asynchronous Nature:** Orchestrates parallel searches and delegates slow mathematical computations.

#### 1. Role and Purpose

The role of $\mathcal{I}$ is to perform **deductive proof** based on the explicit **Inference Axioms** (e.g., A12, A13, A16). It synthesizes new facts (Proposed CDUs) from existing facts, ensuring the resulting knowledge is logically sound and contextually appropriate.

#### 2. Detailed Workflow

The inference process is a structured search guided by the active Axiom Set within the current World Context.

1.  **Input:** A Target Conclusion (the query goal) or a set of known facts (resolved by $\mathcal{C}$).
2.  **Axiom Selection:** $\mathcal{I}$ retrieves the relevant **Inference Axioms** from the K-Module's Axiom Set based on the target conclusion or the current World Context.
3.  **Premise Verification (Lookup & Comparison):** For each selected Axiom, $\mathcal{I}$ performs lookups (via C-Module) to verify if the required premises exist as validated CDUs in the ledger.
    *   **Semantic Check:** If the Axiom involves semantic concepts (Primes), $\mathcal{I}$ checks if the existing CDU's $V_{\text{dynamic}}$ is sufficiently close to the Valence required by the Axiom (using **Comparison**).
4.  **Adaptation Check (Pattern Matching):** Before concluding, $\mathcal{I}$ checks the **Path Index Cache (PIC)** for any **Meta-Axiom Templates** associated with the goal structure. If a template is found, it attempts to **substitute** the current entities into the template structure for a faster proof path.
5.  **Conclusion Generation (Assignment):** If all premises are verified (or the pattern template is successfully instantiated), $\mathcal{I}$ generates a **Proposed New CDU** (the conclusion).
6.  **Harm Check Delegation:** Before outputting the proposal, $\mathcal{I}$ checks the proposed CDU against **Harm Axioms (A5)**.
    *   If the proposed conclusion violates A5 (e.g., deriving a positive outcome from a negative premise), the proposal is discarded.
    *   If the proposed conclusion requires **slow math** (e.g., complex geometry in a Cloned World), $\mathcal{I}$ generates a **`COMPUTE_REQUEST` CDU** and delegates the calculation to the C-Module's `MathWorker`, pausing its current proof thread asynchronously.
7.  **Output:** Returns the validated Proposed CDU or the `COMPUTE_REQUEST` CDU.

#### 3. Performance Rationale

Primitive $\mathcal{I}$ is designed to be fast by:
*   **Prioritizing Axioms:** Checking high-priority Axioms (like core math A12-A14) first.
*   **Leveraging Learning:** Using the **Meta-Axiom Templates** from the PIC to shortcut exhaustive graph searches.
*   **Asynchronous Delegation:** Offloading all slow mathematical computations to the C-Module, ensuring the core logic thread remains responsive.
  
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
