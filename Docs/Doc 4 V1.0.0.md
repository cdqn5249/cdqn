# The `cdqn` Toolchain: Language, Compiler, and Runtime
### Doc 4 Version 1.0.0

## Introduction: The Language of a Smart, Immutable World

In the `cdqn` ecosystem, every piece of knowledge is an immutable, verifiable artifact. But how do we interact with this world of smart data? How do we build the intelligent agents that live within it? We need a language and a set of tools designed for this new paradigm—a toolchain built on the same principles of explicitness, security, and verifiability as the data itself.

The **`cdqn` Toolchain** is that set of tools. It consists of three pillars:
1.  **`cdqnLang`:** A language designed for absolute clarity, where code reads like a mathematical textbook and every operation is explicit.
2.  **`cdqnCompiler`:** A "guardian" toolchain that transforms the clear intent of `cdqnLang` into a secure, optimized, and verifiable software component.
3.  **`cdqnRuntime`:** A sovereign "Operating System for AI" that securely executes these components and provides them with the services they need to think, act, and learn.

This toolchain is the bridge between human intent and the execution of a smart, immutable system. It is designed not just for programmers, but for architects, scientists, and domain experts to build the next generation of trustworthy and resilient software. This document provides the complete specification for this toolchain.

---
## 1. Glossary of Core Concepts

*   **`cdqnLang`:** The official, domain-specific language of the `cdqn` ecosystem. It is a declarative, data-flow-oriented language with a strict, explicit syntax.
*   **`cdqnCompiler` (`cdqnc`):** The official compiler toolchain that transforms `.cdqn` source files into secure, portable `.wasm` (WASI) components.
*   **`cdqnRuntime`:** The high-performance host environment that loads, runs, and manages the lifecycle of `cdqn` WASI components and the `cde` (Context Data Entity) actors.
*   **WASI Component:** The unit of reusable logic in the ecosystem. A secure, sandboxed WebAssembly module with a formal WIT interface.
*   **WIT (WebAssembly Interface Type):** A language-agnostic file that defines the public "API contract" of a WASI component, including its imports and exports.
*   **"WIT-First" Design:** The core development philosophy of the ecosystem, where the formal WIT contract for a component must be defined *before* its logic is implemented in `cdqnLang`.
*   **Pipeline (`|->`):** The primary control flow construct in `cdqnLang`. It defines a linear, top-to-bottom sequence of operations where the output of one step becomes the input of the next.
*   **`cde` (Context Data Entity):** A persistent, addressable "actor" managed by the `cdqnRuntime`. The four types are `Node`, `Agent`, `Automata`, and `Worker`.
*   **Sovereign Trust:** The local governance model where a node's owner defines a "Trust List" of component IDs that are authorized to perform privileged actions.

---
## 2. The `cdqn` Manifesto: The Guiding Principles

The entire toolchain is a direct implementation of the six foundational constraints of the `cdqn` ecosystem.
1.  **Asynchronous First, Non-Blocking Always.**
2.  **The Principle of Absolute Explicitness** (including strong, static typing).
3.  **No Classic Functions; Reusability is Componentization.**
4.  **No Anonymous Entities** (on the `cdqNetwork`).
5.  **The Node is Sovereign.**
6.  **All Signatures Must Use Ephemeral Keys.**

---
## 3. `cdqnLang`: The Language of Intent

`cdqnLang` is a dual-use language for querying memory and defining component logic.

### 3.1 The Query Syntax
A declarative syntax for retrieving CDUs from a `memCDU` instance.

**Full Syntax Structure:**
```cdqn
FIND CDU
WHERE
  <condition> [∧ | ∨ <condition> ...]
ORDER BY <field> [ascending | descending]
LIMIT <integer>
```
**Example:**
```cdqn
FIND CDU
WHERE
  (metadata.layer = long-term ∨ metadata.layer = mid-term) ∧
  vector is similar to "The principles of asynchronous programming" ∧
  confidence.overall ≥ 0.9
ORDER BY hlc.wall-seconds descending
LIMIT 10
```

### 3.2 The Component Definition Syntax
The syntax for creating a new, verifiable WASI component.

#### **Canonical Operators (UTF-8)**
| Operation | Symbol | Operation | Symbol |
| :--- | :--- | :--- | :--- |
| Assignment | `←` | Addition | `+` |
| Equality | `=` | Subtraction | `−` |
| Not Equal | `≠` | Multiplication | `×` |
| Logical AND | `∧` | Division | `÷` |
| Logical OR | `∨` | Exponentiation | `²`, `³`, `ⁿ` |
| Flow Operator | `|->` | Type Declaration | `:` |

#### **Structure of a `.cdqn` Component File**
A component file has a rigid, declarative structure.
```cdqn
// 1. Dependency Imports (Optional)
USE cdqn:some-dependency@1.0.0 as alias;

// 2. Metadata Block
META
  Goal: string ← "The component's mission statement."
  Tags: list<string> ← ["machine-readable", "tags"]
  author: string ← "..."
  version: string ← "..."
  license: string ← "BaDaaS-1.1.0" // Default if omitted
/META

// 3. Contract Implementation Declaration
IMPLEMENTS cdqn:my-component-world@1.0.0;

// 4. Logic Pipelines (one for each exported function in the WIT)
PIPELINE my_exported_function(param1: type) -> return_type
  // ... logic using the |-> operator ...
/PIPELINE
```

#### **Key Language Constructs**
*   **`PIPELINE`:** Defines the logic for an exported function as a linear data flow.
*   **`CALL`:** The keyword for making an explicit, asynchronous I/O call to an imported host service or another component.
*   **`TRANSFORM`:** A block that contains pure, deterministic, side-effect-free computation. The compiler verifies this purity.
*   **`ITERATE`:** A block for processing collections, containing a sub-pipeline of `MAP`, `FILTER`, `REDUCE`, or `MAP PARALLEL` operations.
*   **`ON FAIL`:** A block for explicit, `cdu`-native error handling for a failed `CALL`.

---
## 4. The `cdqnCompiler` (`cdqnc`): The Guardian Toolchain

The compiler transforms `cdqnLang` intent into a secure WASI component. It is both a **translator** and a **guardian** of the Manifesto's principles.

### 4.1 The Five-Stage Compilation Pipeline
1.  **Parsing & Lexical Analysis:** Converts `.cdqn` source into an Abstract Syntax Tree (AST), with full UTF-8 support.
2.  **Semantic Analysis & Verification:** The critical "Guardian" stage. It verifies the code against the Manifesto's rules:
    *   **Contract Verification:** Ensures the code matches its `IMPLEMENTS` WIT file.
    *   **Type Checking:** Enforces strong, static typing.
    *   **Flow Analysis:** Verifies the input/output types of every `|->` step.
    *   **Purity Verification:** Guarantees that `TRANSFORM` blocks have no side effects.
3.  **Intermediate Representation (IR) Generation:** Decouples the frontend syntax from the backend.
4.  **Rust Code Generation:** Transpiles the IR into a complete, idiomatic Rust project.
5.  **Invocation of the Rust Toolchain:** Delegates the final, heavy-duty compilation, optimization, and linking to the standard `rustc` and `wit-bindgen` tools to produce the final `.wasm` binary.

### 4.2 Project & Dependency Management
The compiler operates on projects defined by a **`Project Manifest CDU`**. This `cdu` is the immutable "source of truth" for a build, defining dependencies by the **content hash ID of their WIT file CDU**. This guarantees perfectly reproducible builds. The `cdqnc` CLI tool provides friendly commands (`new`, `add-dep`, `build`) to manage this manifest.

---
## 5. The `cdqnRuntime`: The Sovereign Operating System

The runtime is the secure host environment that executes `cdqn` components and manages the lifecycle of `cde` actors.

### 5.1 Core Architectural Components
*   **WASM Engine (`Wasmtime`):** Provides the low-level, secure sandbox for execution.
*   **Asynchronous Scheduler (`Tokio`):** Manages all non-blocking operations, ensuring high throughput.
*   **Host Service Manager:** Provides the concrete implementations for all standard host services (`memcdu-api`, `wasi-http`, `compiler-service`, etc.). Contains the **Secure Caching Layer** for performance.
*   **Persistence & State Manager:** Enforces the **`cdu`-Native Persistence** rule, managing the `cde` Registry and durable Worker Queues by reading from and writing to `memCDU`.
*   **Identity & Security Manager:** Manages the node's sovereign identity, enforces the **Trust List**, and handles **ephemeral key** generation for signatures.
*   **Resource Manager:** Protects the node from runaway components via metering and throttling.

### 5.2 The Node Onboarding Process
A new `cdqnRuntime` instance is not a blank slate. On its first startup, it executes an **onboarding process**, populating its empty `memCDU` with a **"Genesis Package"** of essential CDUs. This package includes the WITs and components for the core APIs, a math library, and other foundational utilities, ensuring the node is fully functional and self-sufficient from day one, even without a network connection.

---
## 6. The `cde` Framework: The Actor Model

The runtime manages four types of persistent, addressable actors (`cde`s), each with an HLC-sealed, verifiable ID.

*   **`Node`:** The sovereign root entity and network interface.
*   **`Worker`:** For stateless, durable, queue-driven tasks.
*   **`Automata`:** For stateful, event-triggered workflow management.
*   **`Agent`:** For autonomous, goal-driven cognitive tasks. Only `Agent`s are privileged to use "tools" (make `CALL`s to services like LLMs or the `http-gateway`).

---
## 7. WIT Specifications for Core Services and Components

### 7.1 Core Host API (`memcdu-api.wit`)
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

### 7.2 Standard `http` Gateway (`wasi-http`)
The runtime provides a secure implementation of the official `wasi:http/proxy@0.2.0` world, enforcing network policies and managing secrets.

### 7.3 `license-manager.wit`
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

### 7.4 `cde-types.wit`
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
}
```

---
## 8. Conclusion

The `cdqn` Toolchain provides a complete, end-to-end framework for building the next generation of software. It is a system designed not just for power and performance, but for clarity, security, and verifiability. By combining the expressive, declarative `cdqnLang` with the rigorous, guardian `cdqnCompiler` and the secure, sovereign `cdqnRuntime`, the toolchain provides a direct path from human intent to a running, smart, immutable system.
