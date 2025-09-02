# The `cdqn` Toolchain: A Guide to Smart Immutable Systems
### Doc 4 Version 1.0.0

## Part 1: Introduction & Foundational Principles

This section establishes the "why" behind the `cdqn` Toolchain. It is the philosophical and constitutional foundation upon which all technical specifications are built.

### 1.1 A New Paradigm for Software
For decades, the dominant paradigm in software has been the mutable state—data that can be changed, overwritten, and deleted. This approach, while familiar, is inherently fragile. It creates complex systems that are difficult to debug, challenging to audit, and brittle in the face of failure. When something goes wrong, answering the simple question, "What was the exact state of the system an hour ago?" can be an impossible task.

**Immutability is the key to a more robust future.**

An immutable architecture, where data is never changed but only appended, provides a revolutionary advantage: the ability to roll back to any previous stable state with absolute, mathematical certainty. This makes systems not only more secure and easier to audit, but also fundamentally more agile.

The **`cdqn` ecosystem** takes this principle one step further. It combines the rock-solid foundation of immutability with the dynamic, adaptive power of **agentic technology**. We envision a future where the core of our most critical systems—from enterprise infrastructure to personal devices—is built on this synthesis.

This document is the complete specification for the **`cdqn` Toolchain**, the set of tools used to build and run these smart, immutable systems. It is intended for developers, architects, and thinkers who wish to understand and build the next generation of resilient, intelligent software.

### 1.2 The `cdqn` Manifesto
The entire toolchain is a direct implementation of the six foundational constraints of the `cdqn` ecosystem. Understanding these principles is key to understanding the design choices in the language and runtime.

1.  **Asynchronous First, Non-Blocking Always:** All I/O is asynchronous for maximum performance and scalability.
2.  **The Principle of Absolute Explicitness:** The language is strongly, statically typed and has no "magic" or hidden behaviors.
3.  **No Classic Functions; Reusability is Componentization:** All reusable logic is encapsulated in a verifiable, sandboxed software component.
4.  **No Anonymous Entities:** All actors on the network must have a verifiable identity.
5.  **The Node is Sovereign:** Each instance of the system is a self-governing entity with full authority over its own data and rules.
6.  **All Signatures Must Use Ephemeral Keys:** A security principle providing forward secrecy and protecting the integrity of historical records.

### 1.3 Glossary of Core Concepts
*   **CDU (Context Data Unit):** The universal, content-addressed "atom" of information.
*   **CDE (Context Data Entity):** A persistent, addressable "actor" managed by the runtime; the "locus of behavior."
*   **`cdqnLang`:** The official, domain-specific language of the ecosystem. It is a declarative, data-flow-oriented language with a strict, explicit, UTF-8 based syntax.
*   **`cdqnCompiler` (`cdqnc`):** The official compiler toolchain that transforms `.cdqn` source files into secure, portable `.wasm` (WASI) components.
*   **`cdqnRuntime`:** The high-performance host environment that loads, runs, and manages the lifecycle of `cdqn` WASI components and `cde` actors.
*   **WASI Component:** The unit of reusable logic in the ecosystem. A secure, sandboxed WebAssembly module with a formal WIT interface.
*   **WIT (WebAssembly Interface Type):** A language-agnostic file defining the public "API contract" of a WASI component, including its imports and exports.
*   **Sovereign Trust:** The local governance model where a node's owner defines a "Trust List" of component IDs authorized to perform privileged actions.

---
## Part 2: `cdqnLang`: The Language of Intent

`cdqnLang` is a declarative, data-flow language. Instead of writing step-by-step instructions, you describe a pipeline through which data flows and is transformed. It uses canonical UTF-8 mathematical symbols to be as clear and unambiguous as possible.

### 2.1 The "WIT-First" Development Workflow
The development process is a formal, two-step procedure that enforces a clean separation between a component's public contract and its private implementation.
1.  **Define the Contract:** First, you create a language-agnostic `.wit` file specifying the component's public "API contract." This defines all the functions the component will `export` and all the services it needs to `import`.
2.  **Implement the Logic:** Then, you create a `.cdqn` file that formally `IMPLEMENTS` the WIT contract, providing the verifiable, explicit logic that fulfills that contract.

### 2.2 The Component Definition File (`.cdqn`)
A `.cdqn` component file has a rigid, declarative structure designed for maximum clarity.

#### **Part 2.2.1: The `META` Block**
This block is the component's "identity card," containing metadata for both humans and machines.

```cdqn
META
  -- The 'Goal' is a mandatory, human-readable mission statement for the component.
  Goal: string ← "To provide a deterministic, mathematical score of a CDU's real-world factuality."
  
  -- 'Tags' are machine-readable keywords for discovery and policy enforcement by the runtime.
  Tags: list<string> ← ["math", "analysis", "scoring", "validation", "read-only"]
  
  -- Standard metadata fields.
  author: string ← "Christophe Duy Quang Nguyen"
  version: string ← "1.0.0"
  
  -- The license is a verifiable SPDX identifier. It defaults to "BaDaaS-1.1.0" if omitted.
  license: string ← "MIT"
/META
```

#### **Part 2.2.2: `USE` and `IMPLEMENTS` Declarations**
These declarations establish the component's dependencies and its public contract.

```cdqn
-- The 'USE' statement imports a dependency defined in a WIT file and gives it a local alias.
USE cdqn:math-constants@1.0.0 as constants;

-- The 'IMPLEMENTS' statement is a formal promise that this component will fulfill a specific WIT contract.
-- The cdqnCompiler will fail the build if this is not true.
IMPLEMENTS cdqn:factuality-engine@1.0.0;
```

#### **Part 2.2.3: The `PIPELINE` Block**
This is the heart of the component. It defines the logic for a single exported function as a linear data-flow pipeline.

*   **The Flow Operator (`|->`):** The output of the line above this operator becomes the input for the line below it. Only one is allowed per line, forcing a clean, top-to-bottom flow.

*   **`CALL` - Explicit I/O:** This keyword marks a non-blocking, asynchronous call to an external service (an imported WIT interface). This is the *only* way a component can produce a side effect.

*   **`TRANSFORM` - Pure Computation:** This block contains pure, deterministic logic. It has no side effects (no I/O, no state changes). The `cdqnCompiler` statically verifies this purity.

*   **`ITERATE` - Collection Processing:** A block for declarative iteration over lists, using a sub-pipeline of:
    *   `MAP`: 1-to-1 transformation.
    *   `FILTER`: 1-to-0-or-1 selection.
    *   `REDUCE`: N-to-1 aggregation.
    *   `MAP PARALLEL`: For performing concurrent I/O operations on each item in a list.

*   **`ON FAIL` - Error Handling:** This block attaches to a `CALL` operation. If the I/O call fails, the pipeline is diverted into this block, which receives a structured `Error CDU` as its input.

### 2.3 `cdqnLang` in Query Mode
This mode is used for direct, read-only interaction with a `memCDU` instance.

```cdqn
-- Find the 10 most recent, high-confidence CDUs related to a topic.
FIND CDU
WHERE
  (metadata.layer = long-term ∨ metadata.layer = mid-term) ∧
  vector is similar to "The principles of asynchronous programming" ∧
  confidence.overall ≥ 0.9
ORDER BY hlc.wall-seconds descending
LIMIT 10
```

---
## Part 3: The `cdqnCompiler` (`cdqnc`)

The compiler's job is to transform `cdqnLang` intent into a secure WASI component, acting as the guardian of the Manifesto.

### 3.1 The Five-Stage Compilation Pipeline
1.  **Parsing & Lexical Analysis:** Converts `.cdqn` source into an Abstract Syntax Tree (AST), with full UTF-8 support for canonical math symbols.
2.  **Semantic Analysis & Verification:** The "Guardian" stage. It rigorously verifies the code against the Manifesto's rules, its WIT contract, and performs purity analysis on `TRANSFORM` blocks.
3.  **Intermediate Representation (IR) Generation:** Decouples the language syntax from the code generation backend.
4.  **Rust Code Generation:** Transpiles the IR into a complete, idiomatic Rust project.
5.  **Invocation of the Rust Toolchain:** Delegates the final, heavy-duty compilation and optimization to the standard `rustc` and `wit-bindgen` tools to produce the final `.wasm` binary.

### 3.2 Project & Dependency Management
The compiler operates on projects defined by a **`Project Manifest CDU`**. This `cdu` is the immutable "source of truth" for a build, defining dependencies by the **content hash ID of their WIT file CDU**. This guarantees perfectly reproducible builds. The `cdqnc` CLI tool provides friendly commands (`new`, `add-dep`, `build`) to manage this manifest.

---
## Part 4: The `cdqnRuntime`: The Sovereign OS

The runtime is a single, native Rust binary that acts as a secure "OS for AI."

### 4.1 Core Architectural Components
*   **WASM Engine (`Wasmtime`):** Provides the low-level, secure sandbox for execution.
*   **Asynchronous Scheduler (`Tokio`):** Manages all non-blocking operations.
*   **Host Service Manager:** Provides the concrete implementations for all standard host services (`memcdu-api`, `wasi-http`, `wasi-nn`, etc.) and contains the **Secure Write-Through Cache**.
*   **Persistence & State Manager:** Enforces the **`cdu`-Native Persistence** rule, managing the state of all actors and workflows as CDUs.
*   **Identity & Security Manager:** Manages the node's sovereign identity and enforces the **Trust List**.
*   **Resource Manager:** Protects the node from runaway components via metering and throttling.

### 4.2 Workflow: The Secure Write-Through Cache
To balance performance and integrity, the runtime uses a write-through cache for all stateful entity data.
1.  **State Change Request:** An `Automata` requests to update its state.
2.  **Persistent Write:** The runtime's **Persistence Manager** first creates a new `State CDU` and publishes it to the persistent `memCDU` on disk. This is a blocking, durable operation.
3.  **Cache Update:** **Only after the disk write is confirmed successful**, the runtime's **Host Service Manager** updates its high-speed, in-memory cache to point to the new `State CDU`.
4.  **Confirmation:** The operation is confirmed as complete to the `Automata`.
This guarantees zero data loss in a crash. The cache provides high-speed reads, while the write-through policy ensures all writes are durable and secure.

### 4.3 Workflow: The Node Onboarding Process
A new runtime instance is not a blank slate. On first startup, it performs a **Hardware Capability Assessment** and prompts the user for their choice of a default Language Model. These choices, along with the node's new identity, are immutably recorded in a foundational **Genesis CDU**, the node's "birth certificate." The runtime then populates its memory with a **"Genesis Package"** of essential core components and their WITs, making it fully functional offline.

---
## Part 5: Full Code Examples

### 5.1 Use Case: A Verifiable Math Component

#### **The WIT Contract (`quadratic-solver.wit`)**
```wit
package cdqn:quadratic-solver@1.0.0

world solver {
  export solver-api: cdqn:solver-api@1.0.0;
}

interface solver-api {
  /// Solves the quadratic equation ax² + bx + c = 0.
  solve: func(a: f32, b: f32, c: f32) -> option<list<f32>>;
}
```

#### **The `cdqnLang` Implementation (`quadratic-solver.cdqn`)**
```cdqn
META
  Goal: string ← "To calculate the real roots of a quadratic equation."
  Tags: list<string> ← ["math", "solver", "deterministic"]
  license: string ← "MIT"
/META

IMPLEMENTS cdqn:quadratic-solver@1.0.0;

PIPELINE solve(a: f32, b: f32, c: f32) -> option<list<f32>>
  (a, b, c)
  |-> TRANSFORM
    (a: f32, b: f32, c: f32)
    discriminant: f32 ← b² − (4 × a × c)
    IF discriminant > 0
      root1: f32 ← (−b + √discriminant) ÷ (2 × a)
      root2: f32 ← (−b − √discriminant) ÷ (2 × a)
      [root1, root2]
    /IF
    |-> ELSE IF discriminant = 0
      root: f32 ← −b ÷ (2 × a)
      [root]
    /ELSE IF
    |-> ELSE
      null
    /ELSE
  |-> RETURN
/PIPELINE
```

### 5.2 Use Case: An AI-Powered Orchestrator Component

#### **The WIT Contract (`image-enricher.wit`)**
```wit
package cdqn:image-enricher@1.0.0

use wasi:nn;
use cdqn:memcdu-api@2.0.0;

world enricher {
  import wasi:nn/graph;
  import wasi:nn/inference;
  import wasi:nn/tensor;
  import memcdu-api;

  export enricher-api: cdqn:enricher-api@1.0.0;
}

interface enricher-api {
  /// Takes the ID of an image CDU and enriches it by adding a caption.
  enrich-image: func(image-cdu-id: string) -> expected<string, string>;
}
```

#### **The `cdqnLang` Implementation (`image-enricher.cdqn`)**
```cdqn
META
  Goal: string ← "To enrich an image CDU with a generated textual caption."
  Tags: list<string> ← ["ai", "vision", "generative", "orchestration"]
/META

IMPLEMENTS cdqn:image-enricher@1.0.0;

USE wasi:nn/inference@0.2.0 as nn-inference;
USE wasi:nn/graph@0.2.0 as nn-graph;

PIPELINE enrich-image(image_cdu_id: string) -> expected<string, string>
  
  image_cdu_id
  
  -- Step 1: Fetch the original image CDU.
  |-> CALL memcdu-api.get(it)
  ON FAIL (err: cdu) |-> RETURN as_error(err) /ON FAIL
  -> original_image_cdu: cdu
  
  -- Step 2: Load the image captioning model.
  caption_model: nn-graph.graph ← CALL nn-graph.load-by-name("image-caption-model")
  
  -- Step 3: Run inference to generate the caption.
  |-> caption_model
  |-> CALL nn-inference.run(it, original_image_cdu.content_data)
  -> caption_text: string
  
  -- Step 4: Create a new composite CDU content.
  |-> TRANSFORM with context (original_image_cdu)
    (text: string, context: cdu)
    { "source_image_id": context.id, "caption": text }
  -> new_content: json
  
  -- Step 5: Publish the new composite CDU.
  |-> CALL memcdu-api.publish(
        content_type: application-json,
        content_data: to_bytes(new_content),
        scope: factual
      )
  
  |-> RETURN as_ok
/PIPELINE
```

---
## 6. Appendix: Core Host WIT Definitions

This section provides the formal API contracts for the essential services provided by the `cdqnRuntime` to all components.

### 6.1 `memcdu-api.wit`
```wit
package cdqn:memcdu

world memory-system { export memcdu-api: cdqn:memcdu-api@2.0.0 }

interface memcdu-api {
  use.cdqn:memcdu-types@2.1.0.{cdu, cdu-params, query-params}
  async publish: func(new-cdu: cdu-params) -> expected<string, string>
  async get: func(id: string) -> expected<cdu, string>
  async query: func(params: query-params) -> expected<list<cdu>, string>
}
```

### 6.2 Standard WASI Worlds
The runtime provides secure implementations of official WASI worlds, which serve as the standard library for components:
*   **`wasi:http/proxy@0.2.0`:** For external HTTP calls.
*   **`wasi:nn`:** For local, hardware-accelerated ML inference.
*   **`wasi:filesystem`:** For secure, sandboxed file access.
*   **`wasi:clocks` and `wasi:random`:** For standard system utilities.

### 6.3 `cde-types.wit`
```wit
package cdqn:entities@1.0.0

use.cdqn:memcdu-types@2.1.0.{hlc-record}

enum entity-type { node, worker, automata, agent }
enum entity-status { running, paused, idle, completed, failed }

record cde {
  id: string, // HLC-sealed cryptographic ID
  type: entity-type,
  component-cdu-id: string,
  creation-hlc: hlc-record,
  status: entity-status,
  state-cdu-id: option<string>,
  language_model_id: option<string>, // For Agents
}
```

### 6.4 `license-manager.wit`
```wit
package cdqn:license-manager@1.0.0

world license-service { export license-api: cdqn:license-api@1.0.0 }

interface license-api {
  use.cdqn:memcdu-types@2.1.0.{cdu}
  enum UsageAction { execute-internally, distribute-unmodified, distribute-modified }
  record UsageContext { /* ... generic usage context ... */ }
  enum LicenseObligation { attribution, state-changes, disclose-source, form-partnership }
  verify-action: func(target-cdu: cdu, context: UsageContext) -> expected<list<LicenseObligation>, string>;
}
```

---
## 7. Conclusion

The `cdqn` Toolchain provides a complete, end-to-end framework for building the next generation of software. By combining the expressive, declarative `cdqnLang` with the rigorous, guardian `cdqnCompiler` and the secure, sovereign `cdqnRuntime`, the toolchain provides a direct path from human intent to a running, smart, immutable system.
