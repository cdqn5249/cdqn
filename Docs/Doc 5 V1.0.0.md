# The `cdqNetwork`: A Blueprint for the Society of Minds
### Doc 5 Version 1.0.0

## Part 1: Introduction & Vision

### 1.1 From a Sovereign Mind to a Society of Minds
The `cdqn` ecosystem provides the tools to build a **sovereign node**—a powerful, self-governing, and intelligent system with its own verifiable memory (`memCDU`) and will (`cde` framework). But a mind in isolation, no matter how powerful, is limited. True growth, innovation, and problem-solving come from collaboration.

The **`cdqNetwork`** is the architecture that allows these sovereign nodes to connect and form a **"society of minds."**

It is not a centralized cloud or a simple peer-to-peer file-sharing network. It is a sophisticated, decentralized, and self-regulating fabric designed for a new kind of interaction: the verifiable exchange of knowledge and services between autonomous, intelligent agents. This document provides the complete specification for this network.

### 1.2 The Guiding Principles
The `cdqNetwork` is a direct extension of the `cdqn` Manifesto. The principles of **Sovereignty, Verifiable Identity (No Anonymous Entities),** and **Security (Ephemeral Keys)** are the non-negotiable foundations of its design. It is a network built on proof, not just on probability.

### 1.3 The Role of the `ProxyAgent`
In the `cdqNetwork`, a user never interacts directly with the network, and the node's internal `Worker`s and `Automata` are shielded from it. The **`ProxyAgent`** is the **sole, sovereign interface** between the node's owner and the wider ecosystem. Every network action—every query, every barter, every audit—is initiated, managed, and validated by the user's `ProxyAgent`. It is the node's "ambassador" and "chief strategist," acting with the full authority of the sovereign node owner.

### 1.4 Glossary of Network Concepts
*   **`cdqNetwork`:** The complete, decentralized, peer-to-peer fabric that connects all `cdqn` nodes.
*   **`cdqnProt`:** The core communication protocol. It is a `cdu`-native protocol where the flow of `Message CDU`s, ordered by their `hlc`, *is* the protocol.
*   **Node Types:** The formal, socio-technical classification of a node's identity (`HomeNode`, `PrivateNode`, `FirmNode`, `PublicNode`).
*   **`cdqnStar` (`★`):** The native, non-speculative utility token, minted as a proof of a successful value exchange.
*   **`BarterEngine`:** A core WASI component that facilitates the secure, atomic swap of value.
*   **`RepScore`:** A node's reputation, a dynamic, queryable property based on the public ledger of its successful barters and proven coherence.
*   **Proof-of-Coherence:** A voluntary, public audit protocol where a node asserts its own historical integrity, and its peers publicly verify it to build network-wide trust.

---
## Part 2: The `cdqNetwork` Architecture

The `cdqNetwork` is a hybrid Peer-to-Peer (P2P) architecture composed of four distinct, formally recognized node types. A node's type is a verifiable part of its identity, declared at onboarding and recorded in its `Genesis CDU`.

### 2.1 The Four Node Types

| Node Type | Definition & Role | Trust Model |
| :--- | :--- | :--- |
| **`HomeNode`** | The sovereign domain of an individual user, running on personal devices. It maintains a **private `cdqnPSH` only**. | Trusts its own User Swarm implicitly. Treats all other nodes with Zero Trust by default. |
| **`PrivateNode`** | A powerful node run by an individual who chooses to expose a **public `cdqnPSH`**. The "prosumers" of the network. | Reputation is earned based purely on the **merit and reliability** of its public contributions. |
| **`FirmNode`** | A node officially operated by and cryptographically linked to a **commercial legal entity**. | Reputation is tied to the **real-world brand reputation** of the company. |
| **`PublicNode`**| A node officially operated by and linked to a **public institution** (university, government). | Reputation is tied to the **real-world authority and public trust** of the institution. |

### 2.2 The Communication Protocol: `cdqnProt`
The protocol is **`cdu`-native** and ultra-efficient, designed around a stream of causally-ordered events.

#### **The `Message CDU`**
All communication is a stream of independent, immutable `Message CDU`s. There are no complex, stateful "interaction" objects.
*   **`content-type`:** `application/cdqn-message`
*   **`content-data` (JSON):** Contains the `conversation_id`, `sender_id`, `recipient_id`, `performative` (intent), and the `payload_cdu_id`.

#### **The Role of the Hybrid Logical Clock (`hlc`)**
The `hlc` is the sole and sufficient mechanism for ordering and causality. To reconstruct a conversation, an agent simply performs a fast query for all `Message CDU`s with a specific `conversation_id` and orders them by their `hlc`. This is an incredibly efficient and robust model that eliminates I/O overhead.

#### **The Transport Layer: `cdu`-Native Gossip**
The dissemination of `Message CDU`s is handled by a lean gossip mechanism. A node's `cdqnprot-handler` component wraps an encrypted `Message CDU` in a simple `Transport CDU` and "whispers" it to its peers.

---
## Part 3: The Trust & Value Exchange Framework

Trust on the `cdqNetwork` is not based on abstract consensus but on **provable, beneficial, and mutually validated economic activity.**

### 3.1 The `cdqnStar` (`★`) Utility Token
The `cdqnStar` is the native utility token, minted exclusively as a byproduct of a successful, validated exchange of value. A node's balance is kept in a private ledger within its own `memCDU`.

### 3.2 Workflow: The `ProxyAgent`-Led Barter Exchange
This workflow illustrates the central role of the `ProxyAgent` in the network economy.

**Scenario:** `ProxyAgent-Alice` (on a `HomeNode`) needs a high-quality image, a service provided by `ProxyAgent-Bob` (on a `FirmNode`).

1.  **Goal & Plan:** Alice's owner gives her the goal. Alice's internal logic (`Agent` entity) decides that bartering for the service is the best plan.
2.  **Proposal Construction:** `ProxyAgent-Alice` constructs a formal **`BarterProposal CDU`**. This contract contains the hash ID of the `5 ★` it is offering and a hash of the service description it is requesting.
3.  **Initiating the Barter:** `ProxyAgent-Alice` **`CALL`s its local `BarterEngine.wasm` component**, submitting the `BarterProposal CDU`.
4.  **Secure Handshake:** The `BarterEngine`s on both nodes communicate via `cdqnProt`. They verify that both Alice and Bob have committed to the *exact same* proposal by comparing the `cdu`'s hash. This is an atomic commitment step.
5.  **Execution & Atomic Swap:** The `BarterEngine` orchestrates the exchange. It instructs `ProxyAgent-Bob`'s node to execute the `image-generator` skill. Upon completion, the `BarterEngine` facilitates the atomic swap: Alice's node transfers the `cdqnStar` ownership, and Bob's node transfers the new `Image CDU`. The `cdu`'s content-addressable nature makes cryptographic cheating impossible.
6.  **Confirmation & Minting:** `ProxyAgent-Alice` and `ProxyAgent-Bob` inspect the delivered assets. Satisfied, they both send a `ConfirmReceipt` message to their local `BarterEngine`.
7.  **Reputation Event:** The `BarterEngine`s, having confirmed the successful exchange, now generate the **`Reputation CDU`**. This `cdu` is a verifiable "proof of good behavior."
8.  **Public Broadcast:** The `BarterEngine` `CALL`s the local `cdqnprot-handler` to gossip the new `Reputation CDU` on the `@public:reputation` channel for the entire network to see and record.

### 3.3 The `RepScore`: Emergent Reputation
A node's reputation is a dynamic, queryable property calculated by summing the value of all public `Reputation CDU`s associated with its ID, weighted by a **time-decay function** to ensure that recent, relevant behavior is valued more highly.

---
## Part 4: The Network Immune System

The network's security is an active, ongoing process managed by each node's `ProxyAgent`.

### 4.1 Prevention: The Reputation Stake
The `BarterEngine` enforces an economic stake. A `ProxyAgent` cannot initiate a high-value barter unless its node's current, time-decayed `RepScore` is sufficiently high. This prevents new or unknown actors from executing large-scale scams.

### 4.2 The "Proof-of-Coherence" Protocol
This is a voluntary, public audit system that allows a node's `ProxyAgent` to prove its health and earn reputation.

1.  **Assertion:** On a schedule, `ProxyAgent-A` decides to be audited. It **`CALL`s its local `coherence-auditor.wasm` component.** The component creates a **`CoherenceAssertion CDU`** containing a cryptographically signed sample of the node's recent, unbroken causal history.
2.  **Broadcast:** `ProxyAgent-A` then **`CALL`s its `cdqnprot-handler`** to gossip this assertion on the `@public:audit` channel.
3.  **Decentralized Verification:** `ProxyAgent-B` on another node receives the assertion. It **`CALL`s its own local `coherence-auditor.wasm` component** to rigorously verify the cryptographic proofs and causal consistency of the assertion.
4.  **Public Verdict:** If the verification is successful, `ProxyAgent-B` instructs its `coherence-auditor` to create a **`Verification CDU`**. It then `CALL`s its `cdqnprot-handler` to gossip this public, signed attestation to `Node-A`'s health on the `@public:reputation` channel.

---
## Part 5: Core Network Component APIs (WIT)

These are the formal contracts for the essential components that manage network interaction.

### 5.1 `cdqnprot-handler.wit`
```wit
package cdqn:cdqnprot-handler@1.0.0

world comms-officer {
  import crypto: host-crypto-service;
  import gossip: host-gossip-service;
  export handler-api: cdqn:handler-api@1.0.0;
}

interface handler-api {
  enum Performative { request, inform, propose, confirm, fail }
  enum CommunicationScope { intra-swarm, inter-swarm }

  send: func(
    recipient-id: string,
    scope: CommunicationScope,
    performative: Performative,
    payload-cdu-id: string
  ) -> expected<_, string>;
}
```

### 5.2 `BarterEngine.wit`
```wit
package cdqn:barter-engine@1.0.0

world barter-facilitator {
  import cdqn:memcdu-api@2.0.0;
  export barter-api: cdqn:barter-api@1.0.0;
}

interface barter-api {
  /// Proposes a new barter, returning the proposal's CDU ID.
  propose-barter: func(proposal: record { partner-id: string, offer-id: string, request-hash: string }) -> expected<string, string>;

  /// Accepts a barter proposal from another node.
  accept-barter: func(proposal-cdu-id: string) -> expected<_, string>;
  
  /// Confirms successful receipt of the bargained asset, completing the transaction.
  confirm-receipt: func(proposal-cdu-id: string) -> expected<_, string>;
}
```

### 5.3 `coherence-auditor.wit`
```wit
package cdqn:coherence-auditor@1.0.0

world auditor-general {
  import cdqn:memcdu-api@2.0.0;
  export auditor-api: cdqn:auditor-api@1.0.0;
}

interface auditor-api {
  use.cdqn:memcdu-types@2.1.0.{cdu};
  
  /// Creates and returns this node's own CoherenceAssertion CDU.
  assert-coherence: func(last-assertion-id: option<string>) -> expected<cdu, string>;
  
  /// Audits a peer's CoherenceAssertion CDU and returns a signed Verification CDU.
  verify-peer-assertion: func(assertion-cdu: cdu) -> expected<cdu, string>;
}
```

---
## 6. Conclusion

The `cdqNetwork` is not just a communication layer; it is a complete **socio-technical and crypto-economic fabric** for a decentralized society of minds. By building on a foundation of `cdu`-native protocols, verifiable utility orchestrated by the `ProxyAgent`, and public audits, it creates a self-regulating, resilient, and trustworthy environment for collaboration.

This architecture provides the final pillar of the `cdqn` ecosystem, enabling sovereign nodes to connect and create a whole that is far greater than the sum of its parts.
