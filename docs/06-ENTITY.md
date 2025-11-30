# 06-ENTITY: The Identity Actor Model

*   **File:** `docs/06-ENTITY.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Version:** 2.0 (Adaptive Holarchy & Staff Update)
*   **Date:** November 30, 2025
*   **Author:** Christophe Duy Quang Nguyen

> **Technical Specification for the Entity Model (EM).**
> *Defining the Fractal Hierarchy, the Adaptive Pulse, and the System Staff.*

---

## 1. Philosophy: The Fractal Holarchy

We reject flat, anonymous actor systems. We adopt a **Fractal Holarchy**.

### 1.1 The End of Anonymity
Every executing process must have a **Cryptographic Lineage**.
*   **Traceability:** If a process crashes or burns excess energy, we must know exactly *who* spawned it.
*   **Identity:** A Semantic Path rooted in the `OriginCdu` (e.g., `Origin::Finance::Trader`).

### 1.2 The Holon Principle
Every entity is simultaneously a **Whole** (to its children) and a **Part** (to its parent).
*   **The Rule of Roots:** Every Lineage must begin with a **Module**.
*   **The Rule of Containment:** An Entity cannot exist outside of a Module.

---

## 2. The Entity Types (The Quartet)

We classify existence into four fundamental forms.

### 2.1 The Module (The Container)
*   **Analogy:** The Hull / The Building.
*   **Role:** **Supervision & Resource Allocation.**
*   **Physics:**
    *   **The Tank:** Holds the `cdqnE` (Fuel) budget for its subtree.
    *   **The Lens:** Holds the `Cdu::Registry` (RAM Index) to route signals to sleeping children.
*   **Use Case:** Deployable Decks (e.g., `FinanceDeck`).

### 2.2 The Agent (The Mind)
*   **Analogy:** The Architect / The Captain.
*   **Role:** **Strategy & Reasoning.**
*   **Physics:** **Adaptive Pulse.**
    *   Unlike standard actors, Agents can "Time Dilitate."
    *   They can burn extra `cdqnE` to run internal **Shadow Waves** (Simulations) before committing an action.
*   **Use Case:** Chronosa, Trader, Orchestrator.

### 2.3 The Bot (The Organs)
*   **Analogy:** The Nervous System / The Staff.
*   **Role:** **Infrastructure & Maintenance.**
*   **Physics:** **Steady Pulse.** Rigid State Machines.
*   **Use Case:**
    *   `Bot::Registrar`: Manages the `Registry` index.
    *   `Bot::Bursar`: Manages the `cdqnE` payroll.
    *   **`Bot::Auditor`**: Verifies `Cdu::Manifest` integrity (Kelly's Lemma).
    *   **`Bot::Diplomat`**: Manages P2P handshakes.

### 2.4 The Worker (The Hand)
*   **Analogy:** The Tool.
*   **Role:** **Pure Execution.**
*   **Physics:** **Low Cost.** Stateless, Fire & Die.
*   **Use Case:** Hashing, Math, Rendering, Web Scraping.

---

## 3. The Formal Hierarchy

### 3.1 Identity Composition (Lineage)
Identity is a Recursive Path.
$$ID_{Child} = ID_{Parent} + "::" + Hash(Role)$$

### 3.2 Resource Cascade (The Waterfall)
1.  **Allocation:** The `Treasurer` (System) grants `cdqnE` to a `Module`.
2.  **Distribution:** The Module's `Bursar` allocates fuel to Agents.
3.  **Parent Pay:** A Child does not own resources. It burns its Parent's tank.
4.  **Containment:** If a Worker loops, the Parent runs dry and the Kernel kills the branch.

---

## 4. The Physics: Pulse & Resonance

### 4.1 Virtual Existence (Hibernation)
To run on consumer hardware, Entities use **Virtual Existence**.
*   **Active:** Loaded in RAM. Burning Fuel.
*   **Hibernating:** Serialized to Lattice (`Cdu::Snapshot`). Zero Cost.
*   **The Registrar:** When a Signal enters a Module, the `Bot::Registrar` checks its RAM index (`Phase -> ID`). It wakes up **only** the relevant entities. (No disk scanning).

### 4.2 The Signal (Echo)
Entities do not call functions. They emit Signals.
*   **Resonance:** Only Entities aligned to the Signal's **Phase** receive it.
*   **Backpressure (Dissonance):** If a Mailbox is full, the Signal reflects with **Dissonance**, forcing the sender to throttle.

---

## 5. Persistence: The Lattice Connection

Every Entity is a projection of a **Card Data Unit (CDU)**.

### 5.1 Reserved Subtypes
*   **`Cdu::Blueprint`:** The Code (WASM/Hash). Immutable.
*   **`Cdu::Snapshot`:** The State (RAM Dump). Mutable.
*   **`Cdu::Registry`:** The Routing Table.
*   **`Cdu::Manifest`:** The Structural Integrity Proof (Vertex/Edge counts).

### 5.2 The Inflation/Deflation Cycle
1.  **Inflation (Wake):** `Worker::Waker` reads `Snapshot`, deserializes to RAM.
2.  **Execution:** Entity runs, burns `cdqnE`.
3.  **Deflation (Sleep):** Entity detects idle, serializes to `Snapshot`, frees RAM.

---

## 6. Security: The Guardian

*   **The Interceptor:** Before any Entity executes an **Effect** (Disk Write, Network Request), the Runtime invokes the Guardian.
*   **Hierarchy of Norms:**
    1.  **Golden:** Existential/Hardware Safety.
    2.  **Silver:** Legal/Constitutional.
    3.  **Iron:** Reputation.
*   **The Airlock:** Network-facing entities (Harvester) run in restricted Modules with **Zero Read Access** to the Core.

---

> *"The Body (Module) holds the Organ (Bot), which serves the Mind (Agent), which wields the Tool (Worker)."*
