# 05-NETWORK: The Dark Forest Protocol

* **File:** `docs/05-NETWORK.md`
* **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
* **Version:** 1.0 (Initial Architecture)
* **Date:** November 26, 2025
* **Author:** Christophe Duy Quang Nguyen

> **The Consensual Connection Protocol (CCP) and Lattice Routing.**

---

## 1. Philosophy: The Dark Forest

The Public Internet (TCP/IP) is a hostile environment ("The Abyss"). Attackers have infinite resources to scan and exploit open ports.

**CDQN adopts a "Dark Forest" posture.**
*   **Default Silence:** A Node is invisible by default. It does not respond to pings.
*   **Consent First:** Connection is never imposed; it is granted. You cannot talk to a Node unless it agrees to listen.
*   **Identity Routing:** We do not route by Location (IP); we route by Identity (Lattice Signature).

---

## 2. The Connection States

Every Node maintains a **Listening Posture** managed by the **Diplomat Bot**.

### 2.1 State: SILENT (The Ghost)
*   **Behavior:** The Node is effectively offline to the public.
*   **Traffic:** It ignores *all* incoming Echoes that are not from whitelisted signatures (e.g., User's other devices).
*   **Usage:** High-security operations, Deep Work, or "Hiding" from an attack.

### 2.2 State: GUARDED (The Bunker)
*   **Behavior:** The Node is online but cautious.
*   **Traffic:**
    *   *Known Contacts (Whitelist):* Auto-Accepted.
    *   *Strangers:* Ignored *unless* they include a high-value **"Tribute"** (Energy deposit) in the Handshake.
*   **Usage:** Standard operating mode.

### 2.3 State: OPEN (The Merchant)
*   **Behavior:** The Node actively emits a **Beacon** to attract peers.
*   **Traffic:** It accepts "Knocks" from strangers to negotiate trades.
*   **Usage:** During active trading sessions or when publishing content.

---

## 3. The Protocol: Consensual Connection (CCP)

We replace standard TCP Handshakes with a **Cryptographic Knock**.

### Step 1: The Knock (RTS - Request To Speak)
The Sender emits a micro-signal to the Target's Lattice Sector.
*   **Payload:** `[Sender_Signature] + [Intent_Hash] + [Optional_Tribute]`.
*   **Cost:** Minimal Energy.

### Step 2: The Evaluation (The Diplomat)
The Target's **Diplomat Bot** intercepts the Knock.
*   **Check 1 (Identity):** Is this sender blocked? (Blacklist).
*   **Check 2 (Reputation):** Is this sender trusted? (Lattice Distance).
*   **Check 3 (Intent):** Do I care about this topic? (Resonance).

### Step 3: The Invitation (CTS - Clear To Speak)
*   *If Rejected:* The Diplomat does nothing. The Sender hears silence. (No "Error Message" allowed, as that leaks info).
*   *If Accepted:* The Diplomat opens a temporary **P2P Tunnel** (Encrypted Wormhole) and sends a "Key" back to the Sender.

### Step 4: The Exchange
Data flows through the Tunnel. Once complete, the Tunnel collapses. The Node returns to Silence.

---

## 4. Network Topology: The Cosmopolitan Lattice

### 4.1 Multi-Origin Identity
A Node acts as a Bridge between isolated networks.
*   **Root Origin:** The Sovereign Self.
*   **Guest Origins:** "Visas" to connect to Private Networks (Corporate/National).

### 4.2 Bridge Nodes
Nodes that hold keys to multiple Lattices (e.g., Public + Google) act as **Translators**.
*   They receive a signal on one frequency.
*   They validate/sanitize it.
*   They re-emit it on the other frequency.
*   *Incentive:* They earn `cdqnStar` fees for bridging the gap.

---

## 5. Security Against "The Horde"

### 5.1 The Sybil Defense (Hardware Pulse)
Because Minting/messaging is tied to the **Hardware Salt**, an attacker cannot spin up 1 million VMs to flood the network. Their Pulse would slow to a halt.

### 5.2 Digital Ostracism
If a Node bypasses protocol or sends malicious data after the Handshake:
1.  The Victim records the "Crime" in their local log.
2.  The Victim propagates a **Negative Echo** to their neighbors.
3.  The Network updates the Attacker's **Reputation Vector**.
4.  **Result:** The Attacker pushes themselves to the "Periphery." Their future Knocks are auto-ignored by the entire network.

---

> *"In the forest, silence is survival. Only speak to those you know."*
