* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Network & Social Layer**

## **Introduction: A Society of Sovereign Minds**

The Foundational and Cognitive Layers define a `cdqn` node as a sovereign, intelligent entity. The Network and Social Layer answers the next critical question: **How do these sovereign minds connect to form a society?**

This layer provides the protocols and workflows for nodes to discover each other, build trust, communicate securely, and collaborate, all without a central server or authority. It is the decentralized nervous system and the social fabric of the `cdqn` ecosystem.

---

## **1. The `cdqNetwork` Architecture: A Hybrid P2P Model**

*   **What it is:** The `cdqNetwork` is a hybrid peer-to-peer architecture composed of **Nodes** (sovereign peers) and **SuperNodes** (`privateNode` or `publicNode` types) that act as optional rendezvous points.
*   **Why we do this (Best Practice):** A hybrid model, similar in principle to how BitTorrent uses trackers, provides the best of both worlds. SuperNodes make the network **efficient and easy to join**, while the core peer-to-peer communication ensures it remains **decentralized, censorship-resistant, and has no single point of failure.**
*   **A Practical Use Case:** A new `homeNode` joins the network by connecting to a well-known `publicNode` to get its first list of peers. However, once connected, its day-to-day communication (like chat or `cdu` trades) happens directly, peer-to-peer, without the SuperNode being involved.

---

## **2. The Onboarding Workflow: The Secure Invitation**

*   **What it is:** A new node joins the `cdqNetwork` using a **Bootstrap URI**, a secure, shareable link that acts as a formal invitation to a community.
*   **Why we do this (Best Practice):** This model is inspired by **Discord's invite link system**. It is a user-friendly, decentralized, and highly secure alternative to the rigid practice of hardcoding bootstrap IP addresses. The URI is cryptographically signed, making it a verifiable and phishing-resistant "warm introduction" to the network.
*   **`cdqnLang` Example (The logic of a `bootstrap-uri-generator` `Automata`):**
    ```cdqnlang
    // This automata runs on a publicNode to generate invite links.
    automata BootstrapUriGenerator {
      on cdu where cdu.type = task and cdu.subject = "generate-invite"
        // 1. Create a single-use token.
        string: token ← CryptoWorker.generate_random_token()
        string: node_id ← self.node_id() // Get our own node ID.

        // 2. Sign the token and node ID to prove authenticity.
        string: signature ← CryptoWorker.sign(f"{node_id}:{token}")
        
        // 3. Assemble the final URI.
        string: uri ← f"cdqn://{node_id}?token={token}&signature={signature}"

        // 4. Emit the result back to the administrator who requested it.
        emit cdu {
          cdu_type: log,
          subject: "Bootstrap URI Generated",
          content: uri,
          causal_links: [cdu.id],
        };
      /on
    }
    ```

---

## **3. The `cdqnProt`: The Multi-Layered Gossip Protocol**

*   **What it is:** `cdqnProt` is a sophisticated, multi-layered gossip protocol that governs all communication. It is managed by dedicated `Sys-L` `Automata` on each node.
*   **Why we do this (Best Practice):** A "one-size-fits-all" protocol is inefficient. By using a multi-layered approach, we use the right tool for the right job, inspired by real-world protocols like **SWIM (for liveness)** and **Secure Scuttlebutt (for data sync)**.
*   **`cdqnLang` Example (The logic of the `cdu-synchronizer` `Automata`):**
    ```cdqnlang
    automata CduSynchronizer {
      state {
        // ...
      }

      // This workflow is triggered by a timer.
      on cdu where cdu.subject = "Timer:SyncCycle"
        // 1. Get a list of online peers from the liveness layer.
        list<entity_id>: online_peers ← LivenessMonitor.get_online_peers()
        
        // 2. Select a random peer to sync with.
        entity_id: target_peer ← online_peers.random_choice()

        // 3. Emit a task to the runtime's networking interface to start the sync.
        // This is a fire-and-forget, non-blocking call.
        emit cdu {
          cdu_type: task,
          subject: "wasi:cdqn-net::request_sync",
          content: target_peer,
        };
      /on

      // 4. A separate block handles the result of the sync when it arrives.
      on cdu where cdu.subject = "SyncResult"
        // ... process the received PubPGM and decide if we want to barter for cdu's ...
      /on
    }
    ```

---

## **4. Decentralized Discovery: The "Web of Trust"**

*   **What it is:** An agent discovers new, unknown peers not through a central directory, but by traversing its own private social graph. This is the "Ask-a-Friend" protocol.
*   **Why we do this (Best Practice):** This is a direct implementation of the **"web of trust"** model. It makes reputation a prerequisite for visibility, providing a strong, organic defense against Sybil attacks and malicious actors.
*   **`cdqnLang` Example (The logic of a `ProxyAgent` trying to find an expert):**
    ```cdqnlang
    agent MyProxyAgent {
      // ...
      on cdu where cdu.subject = "FindExpert"
        string: topic ← cdu.content.topic

        // 1. First, query our own PrivPGM.
        semantic-context: local_results ← query memCDU { subject: topic };
        
        // 2. Check if we already know an expert.
        if local_results.primary_points.is_empty()
          ↦
          // 3. If not, find our most trusted peer.
          semantic-context: trusted_peers ← query memCDU {
            label_biases: [{label: relationship, weight: 1.0}],
            // Here, we would sort by trust_score, a feature of the query engine.
          };
          entity_id: best_friend ← trusted_peers.primary_points[0].cdu_cid;

          // 4. Emit a task to ask our trusted peer for a referral.
          emit cdu {
            cdu_type: task,
            subject: "ReferralRequest",
            content: { topic: topic },
            // Send it to the "best_friend" node.
            // (Syntax for addressing a remote node would be formalized here)
          };
        /if
      /on
    }
    ```

---

## **5. Formal Specifications: `entity.wit` and `workflows.wit`**

These WIT schemas define the "language" of the social layer—the formal data contracts that make these interactions possible.

### **`entity.wit` (Relevant Sections)**
```wit
world entity-world {
    // ...
    enum node-type { homeNode, privateNode, firmNode, publicNode }
    record node-profile { node-id: entity-id, node-type: node-type, display-name: string, pub-pgm-cid: cid }
}
```

### **`workflows.wit` (Relevant Sections)**
```wit
world workflow-world {
    // ...
    // For the onboarding workflow
    record onboarding-announcement { node-id: entity-id, node-type: node-type, display-name: string }

    // For the social graph within the PrivPGM
    record relationship-profile { trust-score: float64, reputation-score: float64, inferred-specialties: list<string>, last-interaction: option<cid> }

    // For the decentralized discovery workflow
    record referral { referred-node: entity-id, on-topic: string, comment: option<string> }
}
```
