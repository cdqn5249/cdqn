* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Presentation & External Connections Layer**

## **Introduction: The Sovereign Interface**

The previous layers define the internal "mind" and "society" of the `cdqn` ecosystem. This final layer defines how that powerful inner world **connects to the user and the outside world.** It is the system's **senses** and its **voice**.

The core principle of this layer is **sovereign mediation**. The agent and its data never directly touch the user's screen or the open internet. Every interaction is mediated through secure, auditable, and sovereign components that act as a protective "airlock," ensuring the integrity and privacy of the user's node.

---

## **1. The Presentation System: The Local-First UI**

*   **What it is:** The user interface for a `cdqn` node is not a website. It is a **sovereign, local application**, rendered by the `cdqnRuntime`'s integrated **Tauri shell**.
*   **Why we do this (Best Practice):** This is a direct implementation of **Local-First Software** principles. By rendering the UI locally and eliminating the need for a web server, we **eliminate the entire class of web-based security vulnerabilities** (XSS, CSRF, etc.) by design. This architecture also ensures that the application is always available and performant, even when offline, and that the user's `PrivPGM` data never needs to be sent to a third-party server for rendering.
*   **The Workflow:**
    1.  The user interacts with the UI (e.g., clicks a button).
    2.  The UI sends a simple, local command to the `cdqnRuntime`.
    3.  The runtime routes this command to the user's `ProxyAgent`.
    4.  The `ProxyAgent` performs the necessary cognitive work (querying the `memCDU`, etc.).
    5.  It then calls a specialized, stateless **`cdu-view` `worker`** (e.g., `chat-renderer.wasi`).
    6.  This `worker` takes the processed `cdu` data and generates a block of standard HTML/CSS.
    7.  The `cdqnRuntime` securely injects this generated HTML into the Tauri webview for the user to see.
*   **A Practical Use Case (The Math Input Panel):** A user is writing a `cduMath` expression. The UI detects that the input field is for mathematics. It displays a contextual panel of clickable UTF-8 math symbols (`∑`, `∫`, `√`, etc.). This panel is not hardcoded; its layout is defined by a `config` `cdu` in the user's `memCDU`, making it fully customizable. This overcomes the limitations of standard keyboards and makes writing formal mathematics accessible.

---

## **2. The Developer Environment: The Sovereign Component Repository**

*   **What it is:** A local, on-disk, and namespaced repository for all WASI components, located at `~/.cdqn_node/components/`.
*   **Why we do this (Best Practice):** This provides a secure and organized **software supply chain**. By namespacing components by the `node-id` of their original publisher (e.g., the `cdqn` Foundation vs. a third-party developer), the `cdqnRuntime` can enforce a strict security model, preventing any possibility of a third-party component spoofing a trusted system component.
*   **`cdqnLang` Example (Explicit, Namespaced Calls):**
    ```cdqnlang
    agent MyFinancialAgent {
      on cdu where cdu.type = task and cdu.subject = "analyze"
        // This is a call to a trusted, standard component from the Foundation.
        emit cdu {
          cdu_type: task,
          subject: "<cdqn-foundation-node-id>::deepconf-automata::execute",
          content: { ... },
        };

        // This is a call to a third-party component we acquired via barter.
        // It runs with a different, lower level of trust.
        emit cdu {
          cdu_type: task,
          subject: "<alice-node-id>::volatility-calculator::calculate",
          content: { ... },
        };
      /on
    }
    ```

---

## **3. The External Connections System: The Secure Gateways**

The `cdqn` ecosystem interacts with the outside world through three specialized, secure, and auditable `Sys-L` `Automata`. **`cdqnLang` has no special keywords for these operations**; they are all called via the standard, explicit `emit cdu { cdu_type: task, ... }` mechanism.

### **A. The `http-gateway.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Sovereign Web Client**| `Automata` | Manages all **outgoing** requests to standard web services (e.g., REST APIs). |
**Workflow:** An `Agent` needs data from a weather API. It creates a `cduTask` with an `http-request` record as its `content`. The `http-gateway` picks up this task, makes the actual network call using the `wasi:http` interface, and receives the response. The response is treated as untrusted, immediately wrapped in a new `cdu`, and passed through the full **Security Gauntlet (`validator` -> `sanitizer`)** before being made available to the rest of the system. This prevents any malicious data from a remote API from ever directly touching the `memCDU`.

### **B. The `ingress-gateway.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Sovereign Listener** | `Automata` | Manages all **incoming** connections from non-`cdqn` systems (e.g., webhooks, MCP servers, IoT devices). |
**Workflow:** The gateway listens on a configured port. When external data arrives, it is immediately wrapped in a `cdu` with its `source` field populated to mark its external origin. This `cdu` is then passed through the **full Security Gauntlet**. This provides a secure "airlock" for all data entering the ecosystem from the outside world.

### **C. The `host-bridge.wasi`**
| Role | Entity Type | Core Function |
| :--- | :--- | :--- |
| **The Permissioned Tool Runner**| `Automata` | Manages the secure execution of local commands on the host operating system. |
**Workflow:** This is the most sensitive gateway. It is governed by an **explicit whitelist** model.
1.  The sovereign user *must* first create a `config` `cdu` that explicitly lists the host commands the agent is allowed to run (e.g., `["/usr/bin/git", "/usr/bin/ffmpeg"]`).
2.  When an `Agent` wants to run a tool, it creates a `cduTask` with a `host-command` record.
3.  The `host-bridge` first verifies that the command is on the whitelist. It then rigorously sanitizes all arguments to prevent command injection attacks. Only if both checks pass does the `cdqnRuntime` execute the command in a sandboxed sub-process.
4.  This gives the user absolute, auditable control over the agent's ability to interact with the host system.

---

## **4. Formal Specification: `workflows.wit` (Relevant Sections)**
```wit
world workflow-world {
    // ...
    // For External Connections
    record http-request { method: enum { get, post, put, delete }, url: string, headers: list<tuple<string, string>>, body: option<list<u8>> }
    record host-command { command: string, args: list<string>, timeout-ms: option<u64> }
}
```
