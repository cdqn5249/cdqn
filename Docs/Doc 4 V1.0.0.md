* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Foundational Layer (Technical Specification)**

## **Introduction: A New Foundation for Digital Interaction**

In our digital lives, we constantly create and exchange information. Yet, this information is often fragmented, controlled by third parties, and lacks a verifiable history. It can be altered or deleted without our consent, and proving the true origin of a piece of digital content is a monumental challenge. The `cdqn` (Context Datas Query Nodes) ecosystem is designed to solve this problem by building a new, more robust foundation for our digital world.

This document provides the formal specification for this foundational layer, detailing the three pillars upon which the entire ecosystem rests: the **`cdu`** (the data), the **`Entity`** (the actor), and the **`Manifesto`** (the rules).

---

## **1. The `cdu`: The Context Datas Unit**

*   **Definition:** A `cdu` is the atomic, immutable, and verifiable unit of data in the ecosystem. It is a self-contained digital container that represents a single, specific event. All state in the `cdqn` ecosystem is the result of replaying a causally-ordered history of these `cdu`s.

### **Key Properties & Rationale**

#### **A. Immutable and Permanent**
*   **What it is:** Once a `cdu` is created, it can never be altered or deleted. "Editing" a document simply means creating a *new* `cdu` that is causally linked to the old one.
*   **Why this is a Best Practice:** This principle, known as **Event Sourcing**, is the architectural foundation for the world's most demanding systems, including high-performance financial ledgers and mission-critical databases. By treating history as an unchangeable log of events, it provides a perfect, auditable trail, which is the ultimate guarantee of data integrity.
*   **A Practical Use Case:** For a team of scientists collaborating on a research project, this is non-negotiable. Every experimental result, every hypothesis, and every line of analysis is a permanent `cdu`. This creates a fully reproducible and verifiable research history, eliminating any ambiguity about how a conclusion was reached.

#### **B. Content-Addressed (`cid`)**
*   **What it is:** A `cdu` is identified not by its name or location, but by a unique fingerprint (a cryptographic hash) of its own content and metadata. This fingerprint is its `cid` (Content Identifier).
*   **Why this is a Best Practice:** This is the core principle of globally successful systems like **Git** (the world's version control standard) and **IPFS** (the InterPlanetary File System). It makes the data itself the proof of its own integrity. If even a single bit of a `cdu` were to change, its `cid` would change completely, making tampering mathematically impossible to hide.
*   **A Practical Use Case:** When a legal firm uses the `cdqn` ecosystem to manage a contract, the `cid` of the final, signed document is a perfect, non-repudiable fingerprint. Both parties can independently verify that their copy is identical and unaltered, simply by re-calculating the hash. This removes the need for trusted third-party escrow services.

#### **C. Verifiable Provenance & Causality (`creator`, `hlc-id`)**
*   **What it is:** Every `cdu` is cryptographically signed by its creator and contains a Hybrid Logical Clock (`hlc-id`), which gives it a strict, unambiguous position in the causal history of the system, even across a distributed network.
*   **Why this is a Best Practice:** In an era of sophisticated AI-generated content, verifiable provenance is the only reliable defense against misinformation. Digital signatures provide non-repudiation, while HLCs solve the complex problem of event ordering in distributed systems.
*   **A Practical Use Case:** An artist creates a new piece of digital art and shares it as a `cdu`. The `creator` and `hlc-id` fields act as a permanent, verifiable "birth certificate." Months later, if a dispute over ownership arises, the artist can present this original `cdu`. Its early timestamp and their cryptographic signature provide undeniable proof of their authorship.

### **Formal Specification: `cdu.wit`**
This WIT schema is the definitive, language-agnostic contract for the `cdu` data structure.

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

### **Key Properties & Rationale**

*   **What it is:** The ecosystem is a "society of components," not a single, monolithic application. Each `Entity` has a specific form (`worker`, `automata`, `agent`, `node`) and runs within a strict security sandbox.
*   **Why this is a Best Practice:** This architecture is a fusion of the **Actor Model** (used to build resilient, highly concurrent systems like WhatsApp and Erlang/OTP) and modern **Component-Based Architecture**. This makes the system robust, scalable, and adaptable. A failure in one component is isolated and cannot crash the entire node.
*   **A Practical Use Case:** A developer wants to add a new capability to their agent, such as the ability to analyze a new type of data file. They do not need to modify the agent itself. They simply write a new, self-contained `worker` component. Once this component is acquired and trusted, any `Agent` on the node can start sending it `cduTask`s. This allows for rapid, secure, and decentralized innovation.

### **Formal Specification: `entity.wit`**
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

### **Key Laws & Rationale**

*   **The Node is Sovereign:**
    *   **What it is:** Each user's `cdqn` node is their own sovereign territory. No outside entity can write data to it, change its state, or access its private memory without its explicit, verifiable consent.
    *   **Why it's a Best Practice:** This is a direct response to the failings of the centralized web, where user data is a commodity. By making the user the absolute owner of their data and digital identity, we build a foundation of trust and privacy.
    *   **A Practical Use Case:** A user receives a chat message proposal. Their `ProxyAgent` uses the sender's reputation score to decide whether to even show the message to the user. This **Sovereign Ingestion Workflow** acts as an automatic, intelligent spam filter, giving the user ultimate control over their digital space.

*   **Absolutely Explicit:**
    *   **What it is:** There is no "hidden magic" in the system. Every significant action is represented by an explicit, auditable `cdu`.
    *   **Why it's a Best Practice:** This principle is core to creating transparent, debuggable, and auditable systems.
    *   **A Practical Use Case:** An `Agent` makes a decision that leads to a failure. A developer can use the `cdqn-cli`'s `lineage` command to trace the exact, step-by-step history of `cdu`s that led to that decision. The agent's "thought process" is not a black box but a clear, readable log of events, making it possible to perform a root cause analysis.
