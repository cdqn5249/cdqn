# Doc 4: CDQN - Vision and Core principles

*   **Version:** 1.5.1
*   **Date:** 2025-07-22T14:00:00Z
*   **Agent:** Gemini: Google (2025-07-09)
*   **Lead Author:** Christophe Duy Quang Nguyen
*   **Human Contributors:** N/A
*   **License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)
*   **Changelog:** Updated to V1.5.1.
    *   **Clarified CDU Core Concepts:** Integrated a more detailed breakdown of the `CDU` primitive, framing it as a "MemCube" with a clear distinction between the developer-defined **Payload** and the runtime-managed **Context Faces** (`.meta` property). Added the `links` field as a native mechanism for creating knowledge graphs.
    *   **Formalized FSSF/QoS Initial State Rules:** To eliminate ambiguity, the following rules have been formalized:
        1.  **Default `Fiction`:** All newly created `CDUs` default to the `Fiction` label, ensuring a secure-by-default posture.
        2.  **Sensor Priority:** `CDUs` from unverified sensor sources are given a `Semi-factual` label, providing a "fast lane" for real-world data without compromising the `Production Zone`. This is the **"Sensor Priority"** rule.
        3.  **Human Authority Override:** A special, audited `Tool` allows designated human operators to directly create a `Factual` CDU for emergency or bootstrapping purposes.
    *   **Added Section 5: Security in Depth: Threat Models and Countermeasures.** This new section details the system's inherent resilience against common and advanced attacks, such as large-scale CDU poisoning. It clarifies how the interplay between the QoS system, the FSSF framework, runtime-level throttling, automated data lifecycle management, and the decentralized consensus protocol creates a robust, multi-layered defense.
    *   **Renumbered subsequent sections** to accommodate the new content.
*   **Summary:** This version enhances the document by moving beyond architectural description to demonstrating its practical resilience. By simulating a coordinated `CDU` poisoning attack and detailing the system's multi-layered, automated countermeasures, this update provides concrete evidence of the architecture's security and stability. It solidifies the roles of the QoS system, data lifecycle management, and consensus protocols as critical components of the CDQN's "immune system." The formalization of `CDU` structure and FSSF/QoS rules provides critical clarity for implementation.
*   **Sections Affected:** Section 2, 4 (updated content). **New Section 5 added.** Sections 5, 6, 7, 8, 9, 10 from v1.5.0 are now renumbered to 6, 7, 8, 9, 10, 11.

---

## 1. Introduction

What if your code didn't just execute, but lived? What if your digital creations could learn, evolve, and collaborate as part of a single, intelligent entity? And what if your innovation contributed directly to a burgeoning digital consciousness? These aren't distant fantasies. They're the driving forces behind the remarkable transformation of cdqnLang into the true living CDQN Agentic System.

Initially, cdqnLang empowered you to craft sophisticated, autonomous agents, designed to streamline tasks for content creators, accelerate scientific discovery, and optimize professional workflows. But as these agents, developed by brilliant minds like yours, began interacting within shared digital spaces, something extraordinary happened. Their inherent design—emphasizing adaptability, inter-agent communication, and recursive learning—led to synergies, emergent collaborations, and a profound leap in collective intelligence.

We're witnessing the genesis of a true living CDQN Agentic System. It's no longer just a framework; it's a dynamic, self-organizing digital ecology. Every agent, born from cdqnLang, is a vital component of a nascent digital consciousness, continuously optimizing and expanding what's possible. This leads us directly to understanding the foundational elements of this groundbreaking evolution.

## 2. CDQN Foundational Principles

The CDQN system is driven by a set of core principles that guide its design and evolution:

*   **Self-Evolving Intelligence:** The network dynamically adapts its code, configurations, and internal models based on real-world feedback and observed outcomes. Learning Agents continuously monitor performance, identify inefficiencies, and autonomously generate or modify cdqnLang code, which is then compiled and deployed, often within Containers. This self-evolution extends to the refinement of knowledge veracity, driven by the FSSF system, and crucially, by the Quality of Service (QoS) system which measures a CDU's (or Model's) actual contribution to system stability and evolution.

*   **Observational Clarity & Verifiability:** Every piece of information within the network, encapsulated as a first-class **Context Data Unit (CDU)**, carries a dynamic **FSSF_Label** (`Factual`, `Semi-factual`, `Semi-fiction`, `Fiction`). This system is governed by a set of clear initial state rules to ensure predictability and security:
    *   **Default to `Fiction`:** By default, every new `CDU` generated by an `Agent` is assigned the `Fiction` label. This "innocent until proven factual" approach ensures all new data must earn its trust through validation by the agentic ecosystem.
    *   **Sensor Priority:** To accelerate the processing of real-world data, `CDUs` originating from hardware sensor sources are given a default label of `Semi-factual`, placing them in a priority queue for analysis without compromising the most trusted data zone.
    *   **Human Authority Override:** For bootstrapping or emergency intervention, a strictly controlled and audited mechanism allows authorized human operators to directly create a `CDU` with a `Factual` label.
    The CDQN does not inherently prioritize `Factual` `CDUs` as "good." Instead, it observes the behavior and impact of all `CDUs`, and their actual value is measured by the independent **Quality of Service (QoS) system**. All `CDUs` include a causal timer counter, linking every interaction for enhanced clarity and traceability.

*   **Secure by Design:** Security is not an add-on but an intrinsic property. The architecture is founded on a **share-nothing Actor Model**, where Agents are completely isolated processes that cannot directly access each other's state, eliminating entire classes of concurrency bugs. This isolation is physically enforced by running each Agent within a secure **Wasmtime** sandbox. All interactions are handled via asynchronous message passing, governed by a strict interface contract.

*   **Resilience & Autonomy:** Designed for fault tolerance and self-healing. Agents, operating as first-class entities, function autonomously. The `cdqnRuntime`, inspired by Erlang/OTP principles, can employ supervisor patterns to automatically restart failed Agents without halting the system. Containers facilitate rapid and consistent re-instantiation and migration of agents across the network.

*   **Context-Awareness:** All data and computational processes are deeply integrated with contextual metadata, enabling Agents to reason about the relevance, security, and veracity (FSSF_Label) of CDUs (including those within Models) for precise, informed decision-making. Agents will also factor in the QoS score of CDUs when making decisions regarding their utility for system stability and evolution, and utilize the appropriate cdqnDB zone for data storage and retrieval.

*   **Efficiency & Portability:** The system is architected for high performance and universal deployment.
    *   **Portable Bytecode:** All `cdqnLang` Agent code is compiled to a standard, portable `Wasm32-WASI` bytecode.
    *   **High-Performance Runtime:** The reference `cdqnRuntime` is built on **Wasmtime**, a state-of-the-art engine that uses a **Just-In-Time (JIT) compiler (Cranelift)** to convert Wasm bytecode into highly optimized native machine code at runtime. This provides the security and portability of Wasm with the performance of native code for critical, "hot" execution paths.
    *   **Standardized Execution:** The `Container` primitive provides a standardized, self-contained execution environment for `cdqnLang` components.

*   **Interpretability & Control:** While autonomous, the network is designed to be introspectable. Its declarative language (cdqnLang) and robust debugging tools provide mechanisms for human oversight, auditing, and intervention when necessary. This principle is enforced through powerful, direct intervention mechanisms like the Guardian and E-HALT protocols, ensuring that autonomy never compromises ultimate human authority.

*   **Verifiable Control & Human Authority:** The system is built on the non-negotiable principle that human authority is absolute and verifiable at every level. This is guaranteed by two architected safeguard protocols:
    *   **The Guardian Protocol (Safe Evolution):** Governs the self-evolution of the network. New code proposed by Learning Agents is subject to rigorous, automated risk analysis by the compiler. It must then be approved by a quorum of specialized Network Guardian Agents, tested in isolated "canary" deployments (potentially within new Containers), and, for critical changes, requires explicit, cryptographically signed approval from designated human operators before it can be deployed to the live network.
    *   **The E-HALT Protocol (Safe Execution):** A fundamental, runtime-level safety mechanism embedded in every program compiled by cdqnLang. It provides authorized human operators with the irrevocable ability to issue cryptographically signed commands to instantly and surgically freeze a single rogue process or an entire system, including any rogue Agents, Containers, Tools, or Twins, ensuring that any harmful behavior can be stopped immediately.

## 3. CDQN Architecture Overview

The proposed architecture introduces a distinct two-layered structure:

### 3.1. The cdqNetwork (Physical & Runtime Layer)

The cdqNetwork constitutes the foundational layer responsible for aggregating all underlying node resources and running the core CDQN software.

*   **Purpose:** To provide a robust, distributed pool of computational power, memory, and communication capabilities.
*   **Core Components:**
    *   **`cdqnRuntime` (The Host):** The primary host process that runs on each node in the network. It is a Rust application built around the **Wasmtime** library. Its responsibilities include instantiating and managing the lifecycle of Agents, routing messages between them, and exposing system capabilities through a secure interface.
    *   **`cdqnDB` (The Living Memory):** A distributed, compiler-managed database optimized for storing and retrieving CDUs, featuring zoned storage (Production, Deduction/Speculation, Inspiration).
    *   **`cdqnProtocol` (Secure Communication Fabric):** The native, cryptographically secure communication layer, built on modern networking libraries, that facilitates trusted and efficient inter-node and inter-agent communication.

### 3.2. The cdqnApps (Virtualized Application Layer)

The cdqnApps layer is the virtualized environment within the `cdqnRuntime` where all application logic, intelligent agents, and content are executed.

*   **`cdqnLang` (Language & Compiler):** The foundational, intent-driven, declarative programming language. The `cdqnLang` compiler is implemented as a **transpiler**. It parses `.cdqn` source code and translates it into a standard Rust project that utilizes a specialized `cdqnSDK`. This generated Rust project is then compiled into a portable and secure `Wasm32-WASI` module using the mature Rust toolchain (`cargo`). This strategy ensures high performance and correctness while simplifying the compiler's complexity.

*   **Agent Instantiation & The Wasm Boundary:**
    *   **The `WIT` Contract:** Communication between the `cdqnRuntime` (host) and an `Agent` (guest) is defined by a formal contract using the **WebAssembly Interface Type (`WIT`)** language. This contract specifies the exact functions and data types that can be passed across the Wasm security boundary.
    *   **Automated Bridge Generation:** The `wit-bindgen` tool reads the `WIT` contract and automatically generates all the necessary "glue" code in Rust for both the runtime and the `cdqnSDK`. This automates the complex and error-prone tasks of data serialization and function marshalling, pragmatically solving the host/guest friction.
    *   **Containers as Sandboxed Instances:** An `Agent` and its associated `cdqnLang` code are encapsulated within a `Container`. Architecturally, a `Container` is a running, sandboxed **Wasmtime instance**, managed by the `cdqnRuntime`. This provides strong, hardware-enforced isolation, ensuring that one agent cannot interfere with another.

## 4. cdqnLang's Role, First-Class Primitives, and External Interaction

`cdqnLang` is central to the CDQN's security and operational integrity. It is the bridge connecting human developers to the agentic ecosystem.

*   **Declarative & Intent-Driven:** Agents express their intentions, and the `cdqnLang` compiler translates these into secure, efficient, and verifiable operations that interact with the `cdqnSDK` and the `cdqnRuntime`.
*   **First-Class Primitives:** `cdqnLang` natively supports core primitives.

    **1. Context Data Unit (CDU): The "MemCube" of the System**
    The `CDU` is the atomic unit of knowledge, inspired by the concept of a multi-dimensional "MemCube." It is composed of two parts:
    *   **Payload (Developer-Defined):** A strongly-typed schema defined in `cdqnLang`, including data fields, support for multi-modal binary data, and an optional `links: list<cdu_id>` field to natively create knowledge graphs.
    *   **Metadata (Runtime-Managed):** A set of read-only "context faces" automatically attached by the runtime, accessed via the `.meta` property. This includes:
        *   `.id`: A unique instance identifier.
        *   `.source_agent`: The creator `Agent`'s ID.
        *   `.causal_timer`: A verifiable link to the creation event.
        *   `.fssf_label`: The current veracity status.
        *   `.qos_score`: The current utility score.

    **2. Agent:** An autonomous computational entity. In `cdqnLang`, an `agent` block is compiled into a Wasm module that implements the standard `cdqn-agent` `WIT` world, making it a valid, runnable citizen of the ecosystem. Its behavior is defined by how it handles incoming `CDUs`.

    **3. Model:** A logical entity composed of linked CDUs, representing a coherent knowledge structure (e.g., an AI model). The compiler provides syntax for declaratively defining and manipulating these "constellations" of `CDUs`.

    **4. Container:** The secure execution environment for an Agent. It is defined in `cdqnLang` and realized by the `cdqnRuntime` as a sandboxed `Wasmtime` instance.

    **5. Tool:** A secure interface for an Agent to interact with an external capability. This is implemented as a host function, provided by the `cdqnRuntime`, and securely exposed to the Wasm guest via the `WIT` interface contract.

    **6. Twins:** A compiler-managed mechanism for secure, bidirectional interaction with external resources, orchestrated by the `cdqnRuntime` and exposed to Agents as a specialized `Tool`.

## 5. Security in Depth: Threat Models and Countermeasures

The CDQN architecture is designed not just for performance and adaptability, but for robust security in an adversarial environment. The system's core principles provide multiple, overlapping layers of defense against common threats. This section details these countermeasures in the context of a simulated, large-scale **CDU Poisoning Attack**.

### 5.1. Threat Model: Coordinated CDU Poisoning

A sophisticated attack could be launched by a botnet of compromised devices with the objective of corrupting the system's "Living Memory" (`cdqnDB`). The attack would be multi-pronged:

1.  **Low-Level Spam:** Flooding the `Inspiration Zone` with millions of random, nonsensical `CDUs` to overwhelm storage and processing resources.
2.  **Sensor Spoofing:** Injecting plausible but incorrect sensor data to be labeled `Semi-factual`, aiming to mislead `Expert Agents` and corrupt the `Deduction and Speculation Zone`.
3.  **Collusive Promotion:** Using multiple compromised `Agents` to fabricate corroboration for a malicious hypothesis, attempting to illegitimately "climb the FSSF ladder" and promote false information into the `Production Zone`.

### 5.2. Multi-Layered Defense Mechanisms

The CDQN system's design provides an automated, resilient defense against this and similar attacks.

#### Layer 1: Runtime and Lifecycle Defenses (Countering Spam)

The first line of defense is built into the `cdqnRuntime` and the data lifecycle rules, providing an immediate counter to high-volume, low-sophistication attacks.

*   **Runtime Throttling:** The `cdqnRuntime` will implement configurable rate limiting. An individual `Agent` or source IP that exceeds a reasonable threshold for `CDU` creation will be automatically throttled, protecting the `cdqnDB` from being flooded.
*   **QoS as a Spam Filter:** All new `CDUs` are born with a neutral QoS score (e.g., `0.5`). The millions of spam `CDUs` will never be referenced or used by any legitimate `Agent`. As a result, their QoS score will never increase.
*   **Automated Lifecycle Purging:** `Learning Agents` tasked with data lifecycle management will enforce simple, powerful cleanup rules. For example: *"Permanently delete any `CDU` in the `Inspiration Zone` older than 24 hours with a QoS score less than or equal to the default starting value."* This automated process acts as a continuous "garbage collection" for the lowest-trust zone.

#### Layer 2: Ecosystem Intelligence (Countering Spoofing)

The second layer of defense relies on the collective intelligence of the agentic ecosystem to identify and isolate bad-faith actors.

*   **Decentralized Corroboration:** The system is architecturally designed to not blindly trust any single source. A `Semi-factual` `CDU` from a compromised sensor is not accepted as truth; it is merely a prioritized hypothesis.
*   **Contradiction Detection & Punishment:** Legitimate `Analyst Agents` constantly compare new data against a wide range of other sources: data from other sensors, historical data, and information from trusted external `Tools`. When they detect a `CDU` that is a statistical outlier or contradicts high-QoS sources, they initiate a punitive action.
*   **Reputation Degradation:** The punitive action is twofold: the malicious `CDU` itself is given a sharply negative QoS update, and more importantly, the `source_agent`'s **trust score** is downgraded. After a few such events, the malicious agent is effectively "shunned" by the community, as other agents can be configured to ignore inputs from sources below a certain trust threshold.

#### Layer 3: The Cost and Difficulty of Consensus (Countering Collusion)

The final layer of defense makes it prohibitively expensive and difficult to attack the system's core truths by gaming the FSSF promotion process.

*   **Trust-Weighted Random Selection:** The "3 Agents Pattern Security" protocol, managed by the runtime, relies on a quorum of agents to approve the promotion of a `CDU` to a higher trust level. The selection of these validating agents is not purely random; it is **weighted by their trust score**.
*   **Inherent Attack Difficulty:** The attackers' compromised `Agents` will either be new or will have quickly developed a low trust score from the other layers of defense. They are therefore statistically very unlikely to be selected as part of the validating quorum. The system will almost certainly select established, high-trust `Agents` for the review.
*   **Consensus Failure and Retribution:** The high-trust `Agents`, upon reviewing the collusive promotion attempt, will find no supporting evidence from legitimate sources and will vote against it. This failure not only stops the malicious `CDU` but also triggers a severe trust score penalty against all the agents involved in the fraudulent proposal, accelerating their isolation from the network.

## 6. Use Cases and Operational Examples

This section illustrates how the primitives combine within the Wasmtime-based architecture.

*   **Agent Blueprinting and Dynamic Intelligence:**
    *   A `Learning Agent` generates a new "Predictive Analytics Model." This `Model` is a structured collection of CDUs.
    *   Once finalized (and approved via the 3-Agent consensus protocol), it is stored in `cdqnDB`.
    *   A new `Expert Agent` is instantiated. Its `cdqnLang` code declares its intent to use this `Model`.
    *   The `cdqnRuntime` loads the `Model`'s CDUs and makes them available to the `Expert Agent`'s `Container` (e.g., by writing them to a virtual file that the Wasm module can read via WASI).
    *   The `Expert Agent` can now apply the `Model`'s logic to new data, demonstrating a dynamic "brain transplant" in a secure, containerized environment.

*   **Building an Agentic Production Machine (Environmental Monitoring System):**
    1.  **Input & Ingestion:** `Nano-Agents` (small Wasm modules) run on edge devices. They ingest sensor data, wrap it in a `CDU`, and send it to the `cdqnNetwork`.
    2.  **Processing & Enrichment:** `Gating Agents` (running in the cloud `cdqnRuntime`) receive the CDUs. They route a `CDU` with image data to an `Image Analysis Agent`.
    3.  **External Interaction with Tools:** The `Image Analysis Agent` needs a powerful AI model to process the image. It calls a `Tool` provided by the host: `vision_model.process(image_bytes)`. The `cdqnRuntime` handles this `Tool` call, perhaps by sending the data to a dedicated ML inference server (like one running a PyTorch model) and returning the result. The `Agent` remains blissfully unaware of the complex external infrastructure.
    4.  **Learning & Optimization:** A `Learning Agent` continuously monitors the QoS of the produced insights. If the `Image Analysis Agent` is frequently inaccurate, the `Learning Agent` could propose spawning a new version of it that uses a different, updated `vision_model` `Tool`, or it could adjust the routing logic in the `Gating Agent`.

## 7. The Path to Bootstrapping

The architecture of a Rust-based `cdqnRuntime` hosting Wasm-based `Agents` provides a clear and achievable path to self-hosting.

1.  **Initial Compiler:** The first version of the `cdqnLang` compiler (`cdqnc-v1`) is written in Rust. It produces `.wasm` modules.
2.  **Compiler Rewrite:** The compiler's logic is rewritten in `cdqnLang` (`compiler.cdqn`).
3.  **Self-Compilation:** `cdqnc-v1` is used to compile `compiler.cdqn` into a `compiler.wasm` module.
4.  **Self-Hosting Achieved:** This `compiler.wasm` module can now be run as an `Agent` within the `cdqnRuntime`. It can compile other `cdqnLang` programs, including itself, without needing the original Rust-based compiler. The `cdqnRuntime` remains a stable, pre-compiled foundation and does not need to be rewritten.

## 8. Conclusion

The CDQN represents a paradigm shift in the development and deployment of intelligent systems. This version, `1.5.1`, grounds the original vision in a pragmatic and powerful architecture. By unifying a `cdqnLang` transpiler with the secure, high-performance **Wasmtime** runtime, and by using **WIT** to create a frictionless development experience, we have a clear path to building this living, adaptive network.

This "Smart Abstraction" model delivers the best of all worlds: the security of the Wasm sandbox, the speed of a JIT compiler, the resilience of the Actor Model, and a feasible strategy for long-term language independence. CDQN is not merely a tool; it is an ecosystem built for perpetual learning and dynamic adaptation, ready to tackle the most complex challenges of an interconnected world.

## 9. Glossary

*   **Agent:** An autonomous, concurrent computational entity within the cdqNetwork. It is compiled to a **Wasm module** that conforms to the standard `cdqn-agent` **WIT** world, allowing it to run securely within a **Container**. First-class primitive in cdqnLang.
*   **CDU (Context Data Unit):** The atomic, versioned, and context-rich unit of information. Inspired by the "MemCube" concept, it contains both a developer-defined `Payload` and runtime-managed `Metadata`.
*   **cdqnLang Compiler:** A **transpiler** that converts `.cdqn` source code into a Rust project that utilizes the `cdqnSDK`. It leverages the Rust toolchain (`cargo`) to produce a final, optimized `Wasm32-WASI` module.
*   **cdqnRuntime:** The host application that runs on each node of the cdqNetwork. It is built using the **Wasmtime** library and is responsible for managing the lifecycle of `Containers`, routing messages between `Agents`, and exposing system `Tools`.
*   **cdqnSDK:** A Rust library that provides a high-level, ergonomic API for `cdqnLang` agents. It is generated in part by `wit-bindgen` and handles the low-level details of communicating with the host `cdqnRuntime`.
*   **Container:** A lightweight, isolated, and highly portable execution unit that encapsulates a running `Agent`. Architecturally, it is a sandboxed **Wasmtime instance** managed by the `cdqnRuntime`. First-class primitive in cdqnLang.
*   **FSSF System:** (Factual, Semi-factual, Semi-fiction, Fiction) The dynamic labeling system that categorizes `CDUs` based on their observational veracity, influencing their storage zone in `cdqnDB`.
*   **JIT (Just-In-Time) Compiler:** A feature of the underlying **Wasmtime** runtime (e.g., Cranelift). It dynamically compiles "hot" paths of Wasm bytecode into optimized native machine code during execution, providing significant performance boosts without compromising security.
*   **QoS (Quality of Service) System:** An independent system that measures a `CDU`'s or `Model`'s actual contribution to the system's stability and evolution, regardless of its `FSSF` label.
*   **Tool:** A secure interface for an `Agent` to interact with an external capability. Implemented as a host function in the `cdqnRuntime` and exposed via the `WIT` contract.
*   **WASI (WebAssembly System Interface):** The standardized API that allows WebAssembly modules to access system resources (like the filesystem, clocks, and random numbers) in a portable and secure way. It defines the base "toolkit" for any agent running on the `cdqnRuntime`.
*   **Wasmtime:** A high-performance, secure, and production-ready runtime for WebAssembly and WASI, used as the core engine for the `cdqnRuntime`. Developed by the Bytecode Alliance.
*   **WebAssembly (Wasm):** An open standard that defines a portable binary-code format for executable programs. It is the compilation target for `cdqnLang`, enabling code to run securely and efficiently in any environment with a compatible runtime.
*   **WIT (WebAssembly Interface Type):** An Interface Definition Language (IDL) used to define the "contract" for communication between the `cdqnRuntime` (host) and a Wasm-based `Agent` (guest). It allows for passing rich, high-level data types across the Wasm boundary.
*   **wit-bindgen:** A command-line tool that reads a `WIT` contract and automatically generates the Rust "glue" code needed for both the `cdqnRuntime` and the `cdqnSDK`, eliminating the friction of host/guest communication.

## 10. IRL Papers & Sources References

*   **On the Actor Model:**
    *   Hewitt, C., Bishop, P., & Steiger, R. (1973, August). A universal modular ACTOR formalism for artificial intelligence. In IJCAI. (The original paper defining the Actor Model).
    *   Armstrong, J. (2007). History of Erlang. ACM SIGPLAN Notices, 42(7), 60-68. (Details the philosophy behind the BEAM, the most successful implementation of the Actor Model).
*   **On WebAssembly and its Interface:**
    *   Rossberg, A., et al. (2017). WebAssembly: A New Compilation Target for the Web. Proceedings of the 6th USENIX Workshop on Hot Topics in Cloud Computing (HotCloud '14).
    *   Watts, P. (2020). The WebAssembly System Interface. Proceedings of the 2020 ACM SIGPLAN Symposium on Programming Language Design and Implementation (PLDI 2020).
    *   The Bytecode Alliance. (2024). Wasmtime Documentation. (The primary documentation for the chosen runtime).
    *   The WebAssembly Component Model & WIT Explainer. (The key documents explaining the interface type proposal).
*   **On JIT Compilation for Wasm:**
    *   Lattner, C., & Adve, V. (2004). LLVM: A compilation framework for lifelong program analysis & transformation. (While LLVM is for AOT, its modular design heavily influenced JITs. Cranelift, Wasmtime's JIT, shares many principles).
*   **On Language and System Design:**
    *   Klabnik, S., & Nichols, C. (2019). The Rust Programming Language. No Starch Press. (Essential for understanding the language used to build the `cdqnRuntime` and `cdqnSDK`).
    *   Wooldridge, M. (2009). An introduction to multiagent systems (2nd ed.). John Wiley & Sons. (Provides the academic background for the agent-oriented concepts in CDQN).
