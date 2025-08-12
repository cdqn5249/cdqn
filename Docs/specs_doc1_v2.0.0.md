# Specs Doc 1 — V2.0.0

**Version:** V2.0.0  
**Date:** 2025-08-12T00:00:00Z  
**Agent:** GPT-5 (OpenAI, 2025-08-12)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:** …  
**License:** BaDaaS License — The Agile Commercial Open-Core License (Doc 2)  
**Changelog:**  
- Base: V1.0.0  
- Updates: CST evolution (Context State Token), node roles & hierarchy, cards system, orchestrator agents, overlays (capacity, reputation), copy_cdu & anti-entropy, attestation cadence, policy CDUs, mediation flows, orchestration/sharding, schemas and API contracts.

---

## 1. Vision

The **cdqn** (context datas query nodes) ecosystem is a secure, accountable, and context-rich distributed platform designed to onboard beginners into coding through **vibe coding** — a high-abstraction, beginner-friendly approach powered by **cdqnLang**, an actor-model language inspired by Rust principles, expressed in UTF-8 mathematical notation with explicit strong typing.

Guided by the **ProxyAgent**, users can express intent in natural language, which is interpreted into cdqnLang and executed by the **cdqnRuntime**. All actions and persistent states are recorded in **cdqnDB**, a neural knowledge graph where each data unit (CDU) and model (CDUModel) carries a **Context State Token (CST)** for cryptographic provenance, jurisdictional context, and causal ordering.

Nodes form the **cdqNetwork**, a hybrid P2P architecture optimized for offline-first, asynchronous workflows, with public nodes serving as legal anchors and cdqn nodes providing technical stewardship. Governance is ensured through immutable lineage tracking, ACL-based rights management, and a reputation system for nodes and agents.

The ecosystem enables seamless interaction across devices and jurisdictions, combining **learning, execution, persistence, and governance** in a unified workflow. Its end goal is to transition users smoothly from high-level, AI-assisted coding to full Rust/WASM proficiency while preserving performance, modularity, and legal compliance.

---

## 2. Design Goals

- Beginner-friendly, safe, and explicit coding.  
- High mathematical expressiveness (UTF-8 symbols).  
- Strong typing for all variables, actors, and structures.  
- Actor-model concurrency built into the language.  
- Offline-first hybrid networking with secure aggregation online.  
- Immutable, event-sourced persistence with clear provenance.  
- Guardrails to ensure compliance with laws and policies.

**Additional V2 goals:**

- Provable causal/contextual provenance (minimal CST + overlays).  
- Non-anonymous, hardware-attested nodes.  
- Capacity & reputation overlays for scheduling and governance.  
- Policy-as-CDU with auditable lifecycle and mediation.  
- Cards system for async actor messaging and workflow channels.

---

## 3. cdqnLang Specification

### 3.1 Purpose

cdqnLang is the primary interface language of the cdqn ecosystem, designed for beginners but powerful enough for advanced workflows. Compiles to Rust/WASM via cdqnRuntime.

### 3.2 Syntax Cheat Sheet

| Construct          | Symbol(s)       | Example                        |
|--------------------|-----------------|--------------------------------|
| Assignment         | ≔               | `x: i32 ≔ 5`                   |
| Equality           | =               | `x = 5`                        |
| Inequality         | ≠               | `x ≠ 0`                        |
| Comparisons        | `< , > , ≤ , ≥` | `x ≤ 10`                        |
| Sequential flow    | :>>             | `step1() :>> step2()`           |
| Mathematical power | superscript     | `x² , x³`                       |
| Sum/Product        | ∑ , ∏           | `∑_{i=1}^{n} xᵢ`               |
| Root/Integral      | √ , ∫           | `√x , ∫ f(x) dx`               |

### 3.3 Core Primitives

- **CST (Context State Token)** — Minimal token with HLC, node_id, agent_id, jurisdiction, signature. Overlays referenced separately.
- **CDU** — Immutable smallest persistence unit.
- **CDUModel** — Higher-order structure linking CDUs.

### 3.4 Actor Model

- Typed, non-anonymous actors.  
- Persist state only via CDU/CDUModel.  
- Interact via **Cards**.

Actor categories:

1. **cdqnSys** — System functions (security, data, networking, policy, orchestration).  
2. **Custom** — User-defined actors.  
3. **agent** — Autonomous actors.

Example:

```
actor Sensor: cdqnSysData with agent ≔ { receive(input: CDU) :>> process(input) }
```

### 3.5 Control Structures

Preserved from V1: If/Else, Loops, `:>>` sequencing, deterministic replay.

### 3.6 Integration

- ProxyAgent: NL → cdqnLang translation.  
- cdqnRuntime: Executes, generates CST.  
- cdqnDB: Stores CDUs, supports lineage queries.

---

## 4. ProxyAgent Specification

- One per user.  
- Translates NL to cdqnLang, validates, routes to runtime.  
- Manages learning profile as CDU.  
- Signs requests; ephemeral keys linked to attestation.

---

## 5. cdqnRuntime & cdqnCore

### 5.1 cdqnCore

Framework enforcing primitives, actor rules, networking, persistence semantics.

### 5.2 cdqnRuntime

- Executes WASM/WASI deterministically.  
- On first install: `attest_hardware()`, set `node_id`, initial CST.  
- Hardware change detection → re-attestation.  
- APIs: `persist_cdu`, `publish_card`, `claim_card`, `attest_hardware`, etc.  
- Guardrails enforced via InternalProxy.


---

## 6. cdqnDB

### 6.1 Purpose
cdqnDB is the neural knowledge graph database for the ecosystem. All persistent data (CDU, CDUModel) is stored here with full lineage and ACL metadata.

### 6.2 Properties
- Immutable payloads, mutable metadata.
- Queryable by CST, lineage, type, tags.
- Supports replication via async gossip, copy_cdu only (no raw transfer).
- Designed as a neural graph for contextual linking.

### 6.3 Lineage
- Source CDU/CDUModel → child derivations.
- Root nodes have creator/owner as parent reference.
- Payload hash + CST to verify integrity.

### 6.4 APIs
```
store_cdu(cdu: CDU) -> CST
get_cdu(cst: CST) -> CDU
query_graph(filters: Query) -> GraphView
copy_cdu(cst: CST, target_node: NodeID) -> CST
```

---

## 7. Cards Messaging System

### 7.1 Concept
Cards are CDU-wrapped messages for async actor communication.  
Channels: independent logical flows, orchestrated without global sync.

### 7.2 Flow
1. Actor publishes card to channel.
2. Orchestrator routes to eligible subscribers.
3. Cards persisted for audit.

### 7.3 APIs
```
publish_card(channel: ID, card: CDU) -> Ack
subscribe_channel(channel: ID, filter: Filter) -> Stream
```

---

## 8. Orchestrator & InternalProxy

### 8.1 Orchestrator
- Based on ideas from Temporal/Warp.
- Manages workflows, retries, compensations.
- Runs as cdqnSysOrchestrator.

### 8.2 InternalProxy
- System-level ProxyAgent for security, policy enforcement.
- Namespace: cdqnSysSecurity.

---

## 9. Node Roles & Hierarchy

### 9.1 Types
- Personal
- Private Group
- Legal Firm
- Public Node (country authority)
- cdqn Node (technical steward)

### 9.2 Governance
- Public Node has legal oversight in jurisdiction.
- cdqn Node mediates technical disputes.

---

## 10. Hardware Attestation

### 10.1 Process
- On install: attest hardware → set node_id.
- On use: verify hardware; re-attest if changed.

### 10.2 APIs
```
attest_hardware() -> AttestationReport
verify_hardware(report: AttestationReport) -> bool
```

---

## 11. Reputation System

### 11.1 Scope
- For nodes and agents.
- Score based on uptime, compliance, peer feedback.

### 11.2 Storage
- Stored as overlay in cdqnDB.
- Queried by orchestrator for routing.

---

## 12. Governance via Policy CDUs

- Policies stored as CDU with ACL.
- Changes recorded via CST lineage.
- Enforced by InternalProxy.

---

## 13. Communication Protocol

### 13.1 Principles
- Offline & async first.
- Gossip between near peers with random intervals.
- No raw transfer; only copy_cdu.

### 13.2 Channels
- Separate by workflow type.

---

## 14. Security Model

- No anonymous nodes.
- Hardware attestation required.
- All payloads scanned at ingress.

---

## 15. Changelog V2.0.0

- Replaced epoch in CST with minimal real-time via HLC.
- Added overlays: capacity, reputation.
- Cards messaging + orchestration.
- Node role hierarchy + cdqn node mediator.
- Policy-as-CDU for governance.
- Async-first gossip protocol.
