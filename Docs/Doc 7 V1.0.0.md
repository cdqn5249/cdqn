* **Version:** 1.0 Final Blueprint
* **Date:** September 5, 2025
* **Author:** Christophe Duy Quang Nguyen
* **Vibe Coding Engine:** Gemini 2.5, Google

# **The `cdqn` Ecosystem: The Socio-Economic Layer**

## **Introduction: A Society of Value**

A network of intelligent agents is not just a technical system; it is a society. For this society to thrive, it needs more than just communication protocols—it needs an economy. The `cdqn` Socio-Economic Layer provides the framework for agents to discover each other, build trust, and exchange valuable digital assets in a fair, secure, and sovereign manner.

This layer provides the incentives for creation, collaboration, and the growth of a healthy public commons, transforming the `cdqNetwork` from a simple network into a vibrant, self-sustaining digital economy.

---

## **1. The `PubPGM`: The Public Catalogue of Knowledge**

*   **What it is:** The `PubPGM` (Public Primes Graph Map) is a lightweight, metadata-only, public-facing "catalogue" of a node's most valuable knowledge assets. It advertises what a node knows without revealing the knowledge itself.
*   **Why we do this (Best Practice):** This is a **privacy by design** approach that protects the intellectual property of the content (`cdu`s) while enabling efficient, low-bandwidth discovery across the network.
*   **`cdqnLang` Example (The `pubpgm-generator` `Automata`'s logic):**
    ```cdqnlang
    automata PubPgmGenerator {
      on cdu where cdu.subject = "Timer:DailyUpdate"
        // 1. Query the local PrivPGM for the most prominent public assets.
        semantic-context: top_assets ← query memCDU {
          // Find the top 100 most prominent procedures and components.
          label_biases: [{label: procedure, weight: 1.0}, {label: component, weight: 1.0}],
          limit: 100,
        };

        // 2. Build the lightweight, public version of the graph.
        pub-pgm: public_graph ← self.build_public_graph(top_assets);
        
        // 3. Emit a new cdu containing the updated PubPGM.
        emit cdu {
          cdu_type: system,
          subject: "PubPGM:Update",
          content: public_graph,
        };
      /on
    }
    ```

---

## **2. The Unified Barter Economy: The Marketplace**

*   **What it is:** A formal system for trading all valuable digital assets—**Data** (`cdu`s), **Code** (`cduComponent`s), and **Storage** (`cduArchiveLease`)—using an auditable `cduContract`.
*   **Why we do this (Best Practice):** This formalizes interaction based on the **Smart Contract** model, pioneered by blockchain technology. It makes agreements explicit, auditable, and enforceable.
*   **`cdqnLang` Example (A `ContractAgent` `Automata` evaluating an offer):**
    ```cdqnlang
    automata MyContractAgent {
      // This agent is activated by an incoming contract offer.
      on cdu where cdu.type = contract and cdu.content.status = "proposed"
        barter-contract: offer ← cdu.content

        // 1. First, check the reputation of the offering node.
        relationship-profile: peer_profile ← self.get_profile(offer.offeror_node);
        if (peer_profile.reputation_score < 0.5)
          ↦
          self.reject_offer(cdu, "Reputation too low");
          return;
        /if

        // 2. Evaluate if the offered cdu's are valuable to us.
        bool: is_valuable ← self.evaluate_offer(offer.offered_cids);
        if (is_valuable)
          ↦
          // 3. If the offer is good, accept the contract.
          self.accept_offer(cdu);
        /if
      /on
    }
    ```

---

## **3. The `cdqnStar` Utility Token: The Medium of Exchange**

*   **What it is:** A native utility token, minted by a trusted `Sys-L` `Automata` (the **`treasury-automata.wasi`**), used as a medium of exchange. The ledger is a `cdu` lineage.
*   **Why we do this (Best Practice):** It solves the "coincidence of wants" problem inherent in any pure barter system, a foundational concept from **economics**, making the market more liquid and efficient.

---

## **4. The Emergent Reputation System: The Currency of Trust**

*   **What it is:** An algorithmically calculated trust score based on a node's immutable history of successful and fair interactions. It is managed by the `reputation-manager.wasi` `Automata`.
*   **Why we do this (Best Practice):** This is inspired by **reputation systems** in real-world marketplaces like eBay or Upwork, but made more robust by grounding it in a verifiable `cduContract` history.

---

## **5. Security: Anti-Gaming Measures for a Healthy Economy**

A digital economy requires a robust immune system. The `cdqn` architecture has several layers of defense against common scams and system gaming.

### **A. Preventing Fraudulent Trades (The "Bait and Switch")**
*   **The Threat:** A malicious node advertises a valuable `cdu` in its `PubPGM` but serves a worthless one during the trade.
*   **The Defense (Content-Addressing):** This is impossible by design. The `cduContract` specifies the exact, immutable **`cid`** of the asset to be traded. The receiving node performs the **Sovereign Ingestion Workflow**, which includes a mandatory step of re-calculating the hash of the received data. If the calculated hash does not match the `cid` in the contract, the ingestion **fails automatically**, the contract is voided, and the malicious node's reputation is damaged for failing to deliver.

### **B. Preventing Sybil Attacks (Reputation Inflation)**
*   **The Threat:** A malicious actor creates thousands of fake "Sybil" nodes and has them trade worthless `cdu`s with their main node to artificially inflate its reputation.
*   **The Defense (Reputation Staking):** The `reputation-manager`'s algorithm is not a simple vote count. The reputation gain from a trade is **staked**, or weighted, by the reputation of the other party.
    *   **A trade with a high-reputation node provides a large reputation boost.**
    *   **A trade with a zero-reputation Sybil node provides zero reputation boost.**
    This makes it economically and computationally infeasible to build a fake reputation. A node *must* provide real value to established, trusted peers to gain standing in the community.

### **C. Preventing Counterfeiting & Double-Spending of `cdqnStar`s**
*   **The Threat:** An agent tries to create fake tokens or spend the same tokens in multiple trades simultaneously.
*   **The Defenses:**
    1.  **Cryptographic Minting:** Only the `treasury-automata`, with its unique, hardcoded cryptographic key, can sign a valid "mint" transaction. All other minting attempts are provably fraudulent and are rejected by the network.
    2.  **Causal Ordering:** The token ledger is a single `cdu` lineage with a strict causal order provided by the HLC. When two contracts try to spend the same tokens, only the one whose "lock funds" `cdu` is accepted into the ledger first will be valid. The second one will be rejected by all nodes as a failed transaction because it attempts to spend funds that are already causally locked.

### **D. Preventing Spam and Score Inflation**
*   **The Threat:** A node lies in its `PubPGM`, assigning fake high scores to its assets or stuffing it with popular keywords to game the discovery system.
*   **The Defense (Reputation as Meta-Score):** An agent **never** trusts the scores in a remote `PubPGM`. The `PubPGM` is used for **discovery only**. The true, reliable measure of an asset's potential value is the **reputation of the offering node**. An offer of a "perfect" `cdu` from a zero-reputation node is treated as worthless spam, while an offer of a "good" `cdu` from a high-reputation node is treated as a high-value opportunity.

---

## **6. Formal Specification: `workflows.wit` (Relevant Sections)**
```wit
world workflow-world {
    // ...
    record barter-contract { offeror-node: entity-id, offeree-node: entity-id, offered-cids: list<cid>, requested-cids: list<cid>, token-payment: option<u64>, fiat-payment: option<fiat-payment-instruction> }
    record token-transaction { from-node: entity-id, to-node: entity-id, amount: u64, memo: string }
    record component-package { wasm-binary: list<u8>, wit-interface: string, manifest: string }
    record archive-lease { host-node: entity-id, client-node: entity-id, archived-cids: list<cid>, duration-days: u32, payment: u64, proof-challenge: string }
    record fiat-payment-instruction { transaction-id: string, currency: string, amount: float64, sender-details: list<u8>, recipient-details: list<u8>, contract-cid: cid }
}
```
