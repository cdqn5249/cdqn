# 06-ENTITY: The Identity Actor Model

*   **File:** `docs/06-ENTITY.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 1.2 (System Holarchy)
*   **Date:** November 28, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **Technical Specification for the Entity Model (EM).**
> *Defining the Fractal Hierarchy of Modules, Agents, Bots, and Workers, and their physical persistence in the Lattice.*

---

## 1. Philosophy: The Fractal Holarchy

We reject flat, anonymous actor systems (like standard Erlang/Akka). We adopt a **Fractal Holarchy**.

### 1.1 The End of Anonymity
In CDQN, every executing process must have a **Cryptographic Lineage**.
*   **Traceability:** If a process crashes or burns excess energy, we must know exactly *who* spawned it.
*   **Sovereignty:** Identity is not a random UUID. It is a Semantic Path rooted in the `OriginCdu`.

### 1.2 The Holon Principle
Every entity is simultaneously a **Whole** (to its children) and a **Part** (to its parent).
*   **The Rule of Roots:** Every Lineage must begin with a **Module**.
*   **The Rule of Containment:** An Entity cannot exist outside of a Module.

---

## 2. The Entity Types (The Quartet)

We classify existence into four fundamental forms, ranging from static infrastructure to kinetic execution.

### 2.1 The Module (The Container / Root)
*   **Analogy:** The Building / The Hull.
*   **Role:** **Supervision & Resource Allocation.**
*   **Identity:** Root Level (e.g., `FinanceDeck`). anchored to the `SystemNode`.
*   **Physics:**
    *   **The Tank:** Holds the master Energy Budget (`cdqnStar`) for its entire subtree.
    *   **The Bulkhead:** Acts as a failure barrier. If a Module crashes, it resets internally; it does not crash the Node.
*   **Use Case:** Deployable "Decks" or Libraries.

### 2.2 The Agent (The Architect)
*   **Analogy:** The Brain / The Captain.
*   **Role:** **Strategy & Decision.**
*   **Parent:** Spawned by a Module or another Agent.
*   **Physics:** **High Cost.** Agents simulate futures (Shadow Waves) and manage ambiguity.
*   **Use Case:** Chronosa, Strategic Planning, User Interaction.

### 2.3 The Bot (The Machine)
*   **Analogy:** The Organs / The Nervous System.
*   **Role:** **Maintenance & State Management.**
*   **Parent:** Spawned by Module or Agent.
*   **Physics:** **Medium Cost.** They run steady pulses (listeners, cron jobs).
*   **Use Case:** The Diplomat (Network Listener), The Janitor (Cleanup), The Harvester (Scraper).

### 2.4 The Worker (The Hand)
*   **Analogy:** The Tool.
*   **Role:** **Pure Execution.**
*   **Parent:** Spawned by any Entity.
*   **Physics:** **Low Cost.** They are stateless, fire-and-forget execution units.
*   **Use Case:** Hashing, Math, Rendering, Parsing HTML.

---

## 3. The Formal Hierarchy

To ensure **Modularity** and **Performance**, we define the hierarchy rules mathematically.

### 3.1 Identity Composition (Lineage)
Identity is a Recursive Path.
$$ID_{Child} = ID_{Parent} + "::" + Hash(Role)$$

*   **Example Lineage:**
    1.  **Origin:** `OriginCdu` (The Node)
    2.  **Module:** `Origin::FinanceDeck` (The Application)
    3.  **Agent:** `Origin::FinanceDeck::SniperAgent` (The Strategist)
    4.  **Bot:** `Origin::FinanceDeck::SniperAgent::TickerListener` (The Sensor)
    5.  **Worker:** `...::TickerListener::ParseJSON` (The Hand)

### 3.2 Resource Cascade (The Waterfall)
Resources (`cdqnStar`) flow down. Penalties bubble up.
1.  **Allocation:** The Kernel gives `FinanceDeck` 1000 Star. The Module distributes this to its Agents.
2.  **The Parent Pay Principle:** A Child does not own resources. It burns its Parent's budget.
3.  **Containment:** If a Worker leaks memory or energy, the Parent Agent detects the drain and kills the Worker.

---

## 4. The Physics: Pulse & Resonance

### 4.1 Virtual Existence (Hibernation)
To run on consumer hardware (Laptops) rather than Cloud Clusters, Entities use **Virtual Existence**.
*   **Active State:** Loaded in RAM. Processing Pulses.
*   **Hibernation State:** Serialized to Disk (Lattice). Zero RAM Cost.
*   **The Trigger:**
    *   `if energy < threshold` -> **Die**.
    *   `if idle_time > 10s` -> **Hibernate**.
    *   `if signal_received` -> **Wake**.

### 4.2 The Signal (Echo)
Entities do not call functions directly. They emit Signals.
*   **Mechanism:** An Entity emits a `Signal { phase, payload }`.
*   **Resonance:** Only Entities aligned to that **Phase** (Context) receive the signal.
*   **Backpressure (Dissonance):** If a target Mailbox is full, it reflects the signal with **Dissonance**, forcing the sender to throttle.

---

## 5. Persistence: The Lattice Connection

The Runtime (RAM) is volatile. The Lattice (Disk) is eternal.
Every Entity is a projection of a **Card Data Unit (CDU)**.

### 5.1 The Origin Root
Every Node begins with the **`OriginCdu`**. This is the Genesis Block of your Personal Sovereignty. All Modules are anchored as children of the Origin.

### 5.2 Reserved CDU Subtypes
The Kernel recognizes three specific CDU layouts for Entity Management:

#### A. The Blueprint (`Cdu::Blueprint`)
*   **Role:** The DNA / Class Definition.
*   **Content:** The WASM binary or Rust dylib hash, default config, and permissions.
*   **Immutability:** Strict. (Changing code = New Prime).

#### B. The Snapshot (`Cdu::Snapshot`)
*   **Role:** The Save File / Hibernation Pod.
*   **Content:**
    *   **Target:** Link to the `Blueprint`.
    *   **Memory:** Serialized vector of internal variables.
    *   **Mailbox:** Pending messages.
    *   **Energy:** Remaining budget.
*   **Mutability:** Fluid. Overwritten on Hibernation.

#### C. The Ledger (`Cdu::Ledger`)
*   **Role:** The Black Box.
*   **Content:** Security logs (Guardian Violations) and Economic history (Minting).
*   **Mutability:** Append-Only.

### 5.3 The Inflation/Deflation Cycle
1.  **Inflation (Wake):** Loader reads `Snapshot` from Disk -> Deserializes to RAM.
2.  **Deflation (Sleep):** Runtime Serializes RAM -> Writes `Snapshot` to Disk.

---

## 6. Security: The Guardian

*   **The Interceptor:** Before any Entity executes an **Effect** (Disk Write, Network Request), the Runtime invokes the Guardian.
*   **Hierarchy of Norms:**
    1.  **Golden Layer:** Hardware/Existential Safety.
    2.  **Silver Layer:** Legal/Constitutional Constraints.
    3.  **Iron Layer:** Reputation/Social Constraints.
*   **Veto:** If an action violates a Norm, the Guardian freezes the Entity and logs the attempt in the `Ledger`.

---

> *"The Body (Module) holds the Organ (Bot), which serves the Mind (Agent), which wields the Tool (Worker)."*
