* **Version:** 1.0.1 Final Blueprint
* **Date:** September 6, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The `cdqnLang` & Toolchain Layer**

## **Introduction: The Language of Thought**

The `cdqn` ecosystem is a powerful, decentralized architecture. The `cdqnLang` and its accompanying toolchain are the keys to unlocking that power. This layer provides the complete set of tools for developers and agents to create, manage, and deploy the secure, intelligent, and component-based entities that are the lifeblood of the ecosystem.

The core philosophy is to provide a language that is **beginner-friendly and architecturally honest.** It is designed to make building complex, concurrent, and data-centric systems feel simple and natural, not by hiding a complex reality with "magic," but by providing a syntax that is a direct, elegant reflection of the powerful architecture it runs upon.

---

## **1. The `cdqnRuntime`: The Sovereign Operating System**

*   **What it is:** The `cdqnRuntime` is a secure, high-performance, native application built in Rust. It is the **Wasm Host** or "Operating System" that runs on every node, managing all resources and executing all other components.
*   **Why we do this (Best Practice):** This is a direct implementation of the **secure host runtime model**. By creating a single, trusted, native binary to manage the "unsafe" parts of computing (like disk I/O, networking, and security enforcement), we can allow all application logic to run as completely sandboxed, unprivileged WASI components. This provides a powerful guarantee of security and stability for the entire node.
*   **A Practical Use Case:** When a `cdqnLang` component needs to perform a cryptographic operation, it doesn't contain the crypto library itself. It makes a call to the `wasi:crypto` interface. The `cdqnRuntime`, in its role as the host, intercepts this call and executes it using its own hardened, constant-time, native cryptographic library, returning only the result to the sandbox.

---

## **2. The `cdqnLang`: The Cognitive Orchestration Language**

*   **What it is:** `cdqnLang` is a high-level Domain Specific Language (DSL) that is transpiled into standard, safe Rust and then compiled into a verifiable WASI Component.
*   **Why we do this (Best Practice):** This **transpiler-based approach** provides the best of both worlds:
    *   **For the Developer:** A clean, simple syntax that abstracts away the most difficult parts of systems programming (e.g., Rust's borrow checker).
    *   **For the System:** The full performance, memory safety, and security guarantees of the underlying Rust compiler.
*   **The WIT-First Design:** Development in `cdqnLang` is architecturally anchored to the **WASI Component Model**. This workflow is non-negotiable: first, you define the public interface in a `.wit` file; second, you implement the logic in a `.cdqn` file.

### **Core Syntax and Constructs: A Complete Reference**

*   **Code Blocks: Indentation & Explicit Closures**
    All code blocks use a `2-space indent` and are closed with a `/` followed by the opening keyword. This provides the clarity of indentation with the safety of explicit closures.
    ```cdqnlang
    if x = 10
      // This is inside the if block
      if y = 20
        // This is a nested block
      /if
    /if
    ```

*   **Typing, Declaration, and Assignment: `type: variable ← value`**
    The language uses a "type-first" syntax for clarity, and the unambiguous leftward arrow (`←` U+2190) for assignment.
    ```cdqnlang
    // Single declaration and assignment
    string: name ← "Alice"
    int: age ← 30

    // Multiple declaration of the same type
    int: x, y, z ← 1, 2, 3

    // Collection types
    list<string>: names ← ["Alice", "Bob", "Carol"]
    ```

*   **Comparison: `=`**
    The equals sign is used exclusively for logical comparison.
    ```cdqnlang
    if name = "Alice"
      // ...
    /if
    ```

*   **The Core `cdqn` Verbs: `emit` and `on`**
    These are the two fundamental, explicit actions that drive the entire event-driven ecosystem. `emit` creates a `cdu` (the only side-effect), and `on` reacts to one.
    ```cdqnlang
    automata MyAutomata {
      on cdu where cdu.type = task and cdu.subject = "ping"
        // This block is activated by an incoming task.
        emit cdu {
          license default;
          cdu_type: log,
          subject: "pong",
          causal_links: [cdu.id], // Explicitly link to the triggering event.
        };
      /on
    }
    ```

*   **Control Flow: The Universal Pipe (`↦`) and Conditionals**
    The pipe (`↦` U+21A6) is the universal "consequence" operator for all conditional logic.
    ```cdqnlang
    if
      (score > 90) ↦
        string: grade ← "A"
      or (score > 80) ↦
        string: grade ← "B"
      or () ↦
        string: grade ← "C"
    /if
    ```

*   **Data Flow: The Universal Pipe (`↦`) and Transformations**
    The same pipe operator provides a clean, readable syntax for data transformation.
    ```cdqnlang
    list<int>: numbers ← [1, 2, 3, 4, 5, 6]
    list<int>: even_squares ← numbers
      ↦ filter(n -> n % 2 = 0)
      ↦ map(n -> n²)
    ```

*   **Looping: The Universal `do...until` Block**
    A single construct handles all forms of iteration safely and declaratively.
    ```cdqnlang
    // Simple iteration over a collection.
    do name in names
      emit cdu { cdu_type: log, content: f"Hello, {name}" }
    until end of names

    // Stateful workflow loop.
    do
      call ApiStatusWorker.check_status() ↦ self.update_status()
    until self.state.api_status = "online"
    ```

*   **Explicit Parallelism: The `parallel` Block**
    A structured block is used for "fan-out" parallel operations.
    ```cdqnlang
    parallel
      do url in url_list
        // Emit a task for each url to be processed concurrently.
        emit cdu { cdu_type: task, subject: "downloader::fetch", content: url }
      until end of url_list
    /parallel
    ```

*   **Explicit Error Handling: The `handle` Block**
    Compile-time checked error handling, inspired by Rust's `Result` type.
    ```cdqnlang
    handle a_function_that_might_fail()
      on Ok(success_value) ↦
        emit cdu { cdu_type: log, subject: "Success", content: success_value }
      on Err(error_string) ↦
        emit cdu { cdu_type: log, subject: "Failure", content: error_string }
    /handle
    ```

*   **The `memCDU` Interface: `query` and `replay`**
    First-class keywords for interacting with the agent's cognitive core.
    ```cdqnlang
    agent MyPlanner {
      on cdu where cdu.type = project
        // `replay` is the declarative, high-level command to execute a learned playbook.
        replay {
          context: cdu,
          with_filters: [{label: procedure, weight: 1.0}],
        }
      /on
      
      on cdu where cdu.subject = "replay::NotFound"
          // `query` is the more granular way to search the PrivPGM.
          semantic-context: guidance ← query memCDU { subject: cdu.subject };
      /on
    }
    ```

*   **Traits (Interfaces): Polymorphism for Components**
    A `trait` system for building generic, decoupled components.
    ```cdqnlang
    trait Validator { validate(data: list<u8>) -> Result<bool, string> }
    worker ImageValidator implements Validator { /* ... */ }
    worker TextValidator implements Validator { /* ... */ }
    ```

*   **Mathematical Reasoning: Calculation vs. `prove`**
    The language natively understands UTF-8 math and separates pure calculation from auditable proof.
    ```cdqnlang
    // A pure calculation with no side-effects.
    float64: similarity ← (v ⋅ u) ÷ (‖v‖ × ‖u‖)

    // The `prove` block performs the calculation AND creates a permanent `cduMath`.
    prove "Cosine Similarity between v and u"
      let result = (v ⋅ u) ÷ (‖v‖ × ‖u‖)
    /prove
    ```

---

## **3. The Toolchain and Decentralized Package Management**

*   **What it is:** The `cdqn` Toolchain is a command-line interface (`cdqn-cli`) that provides developers with all the necessary tools (`new`, `build`, `test`, `package`). The core of the toolchain is the **`cdqn-transpiler`**, which converts clean `cdqnLang` code into robust, safe Rust code.
*   **Why we do this (Best Practice):** This creates a complete, user-friendly developer experience. The transpiler approach provides high-level simplicity while retaining the low-level performance and security guarantees of the Rust ecosystem.
*   **Decentralized Package Management:** `cdqnLang` has no central package manager.
    *   **Why:** A central repository is a single point of failure and control, which violates the manifesto's principle of sovereignty.
    *   **The `cdqn` Solution:** To use a third-party component, a developer must first **acquire its `cduComponent` through the `cdqNetwork`'s sovereign barter system**. Once the `cduComponent` is in their local, namespaced **Sovereign Component Repository** (`~/.cdqn_node/components/`), their `cdqn-cli` can securely link against it. This makes code a valuable, tradeable asset and creates a truly decentralized, self-sustaining developer ecosystem.
