* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Foundational Layer (Technical Specification)**

## Introduction: A New Foundation for Digital Interaction

Current digital systems suffer from data fragmentation, a lack of verifiable history, and centralized points of control. This creates fragility, erodes trust, and limits user sovereignty. The `cdqn` (Context Datas Query Nodes) ecosystem is a decentralized architecture designed from first principles to solve these problems.

This document provides the formal specification for the foundational layer of the `cdqn` ecosystem. This layer is comprised of three core pillars: the **`cdu`** (the universal data structure), the **`Entity`** (the universal logic primitive), and the **`Manifesto`** (the architectural laws enforced by the runtime). These pillars provide the stable, secure, and trustworthy bedrock upon which all higher-level cognitive, social, and economic functions are built.

---

## **1. The `cdu`: The Context Datas Unit**

*   **Definition:** A `cdu` is the atomic, immutable, and verifiable unit of data in the ecosystem. It is a self-contained digital container that represents a single, specific event. All state in the `cdqn` ecosystem is the result of replaying a causally-ordered history of these `cdu`s.

**Key Architectural Properties:**

*   **Event Sourcing Model:** The `cdu` log is the single source of truth, an immutable, append-only record of all historical events. This provides a perfect, auditable history, a core requirement for resilient systems.
*   **Content-Addressed Storage (CAS):** A `cdu` is identified by its `cid` (Content Identifier), a cryptographic hash of its content and metadata. This is the same principle used by Git and IPFS to guarantee data integrity; any tampering would be immediately detectable as a hash mismatch.
*   **Verifiable Provenance & Causality:** Every `cdu` is cryptographically signed and contains a Hybrid Logical Clock (`hlc-id`), providing a non-repudiable record of its creator and a strict, unambiguous position in the causal history of the system.

### **Formal WIT Schema: `cdu.wit`**

This is the definitive, language-agnostic contract for the `cdu` data structure. All components, regardless of their implementation language, must adhere to this interface.

```wit
// WIT schema for the Context Data Unit (cdu).
// Version: 2.3.0 (Final)
// Status: Validated

package cdqn:ecosystem

world cdu-world {
    import entity-world.{entity-id, entity-form, execution-context};

    // --- Core Type Definitions ---
    type hlc-id = string; type mime-type = string; type cid = string;

    // --- CDU Subtype System ---
    enum cdu-type {
        system, config, log, chat, task, project, contract, procedure, math, component, license,
        world, chapter, publication,
    }

    // --- Entity & Provenance Records ---
    record creator-info { id: entity-id, form: entity-form, context: execution-context }
    record source-provenance { node-id: entity-id, cid: cid, hlc-id: hlc-id, creator: creator-info }
    
    // --- Licensing Records ---
    enum standard-license { badaas-v1, mit, apache-2-0, gpl-3-0, cc-by-sa-4-0 }
    variant license-type { standard(standard-license), custom(cid) }

    // --- Generative AI Metadata ---
    enum confidence-metric { token, group, trace }
    record generation-info { prompt-cid: cid, score: float64, metric: confidence-metric, model-cid: option<cid> }

    // --- The Main Intrinsic Metadata Structure ---
    record intrinsic-metadata {
        license: license-type,
        cdu-type: cdu-type,
        id: hlc-id,
        lineage-id: hlc-id,
        causal-links: list<hlc-id>,
        subject: string,
        tags: list<string>,
        creator: creator-info,
        content-type: mime-type,
        source: option<source-provenance>,
        generation: option<generation-info>,
    }
    
    // --- The Complete CDU Record ---
    record cdu {
        cid: cid, // HASH(content + metadata)
        content: list<u8>,
        metadata: intrinsic-metadata,
        provenance-signature: list<u8>, // Signature of the cid
    }
}
```

---

## **2. The `Entity`: The Sovereign Logic Primitive**

*   **Definition:** An `Entity` is a secure, sandboxed WASI component that acts as the universal logic primitive. It is the "being" that performs actions by creating, reading, and reacting to `cdu`s.

**Key Architectural Properties:**

*   **Actor Model:** The `cdqn` ecosystem is an actor-based system. Each `Entity` has a private state and communicates with other entities asynchronously by emitting `cdu`s. This design eliminates data races and shared-memory concurrency issues by construction.
*   **Component-Based:** The entire system is a "society of components." Each `Entity` has a specific, verifiable form and purpose, making the architecture modular, resilient, and extensible.
*   **Sovereign & Secure:** Every `Entity` runs within a strict sandbox managed by the `cdqnRuntime`. Its capabilities and permissions are defined by its `execution-context`.

### **Formal WIT Schema: `entity.wit`**

This schema defines the "who" and "where"—the identity and security context of the actors in the system.

```wit
// WIT schema for an Entity's identity and properties.
// Version: 2.0.0 (Final)
// Status: Validated

package cdqn:ecosystem

world entity-world {
    import cdu-world.{cid};

    // --- Core Entity Definitions ---
    type entity-id = string;

    enum entity-form {
        worker,   // Stateless
        automata, // Stateful, event-driven
        agent,    // Stateful, planning & reasoning
        node,     // The sovereign host entity
    }

    enum execution-context {
        sovereign-system,  // Sys-L: Trusted kernel
        sovereign-user,    // User-L: Sandboxed user space
        remote-system,     // Sys-R: Untrusted foreign system
        remote-user,       // User-R: Untrusted foreign user
    }

    // --- Node-Specific Definitions ---
    enum node-type {
        homeNode, privateNode, firmNode, publicNode,
    }

    // A node's public-facing profile.
    record node-profile {
        node-id: entity-id,
        node-type: node-type,
        display-name: string,
        pub-pgm-cid: cid,
    }
}
```

---

## **3. The `cdqn` Manifesto: The Unbreakable Laws**

*   **Definition:** The Manifesto is a set of foundational, non-negotiable architectural laws that are enforced by the native `cdqnRuntime` binary. These laws ensure the long-term integrity, security, and sovereignty of the entire ecosystem.

**Key Enforced Laws (Illustrated by Use Cases):**

*   **The Node is Sovereign:** This is the highest law.
    *   **Use Case:** A remote node cannot write to another node's `cdu` log. It can only propose a `cdu` copy. The local node's `ProxyAgent` makes the final, sovereign decision to accept or reject the data, creating its own local, verifiable copy. This is the **Sovereign Ingestion Workflow**.

*   **No Public Functions; Reusability is Componentization:** All reusable logic *must* be a verifiable WASI component acquired via a `cdqnContract`.
    *   **Use Case:** A developer cannot simply "import" a library. To use another developer's `volatility-calculator.wasi`, they must first discover its `cduComponent` on the network and execute a formal barter to acquire a licensed copy. This creates a secure, auditable software supply chain.

*   **Absolutely Explicit:** There is no "hidden magic." Every significant action is represented by an explicit, auditable `cdu`.
    *   **Use Case:** An `Agent` calling another component does not use a simple function call. It **`emit`s a `cdu` of type `task`**. This makes all inter-component communication an explicit, asynchronous, and auditable event in the immutable log, which is the foundation of the system's debuggability and self-optimization capabilities.

*   **Asynchronous First, Non-Blocking Always:** No component is ever allowed to block on I/O.
    *   **Use Case:** When a `cdu` is created, it is sent to the `cdu-wal-writer` in a "fire-and-forget" operation. The component is immediately free to continue its work while the slow, background process of disk I/O is handled by a dedicated, asynchronous `Sys-L` `Automata` (`cdu-persistor`). This ensures the entire system remains responsive.

This foundational layer provides the stable, secure, and trustworthy bedrock upon which all the advanced cognitive, social, and economic layers of the `cdqn` ecosystem are built.
