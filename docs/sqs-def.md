[⬡ Back to Documentation Portal](index.html)

# The Formal Definition of a Self-Quang System (SQS)

[Next: Canonical Glossary →](glossary.md)

## Metadata

| Field | Value |
| :--- | :--- |
| **Version** | `1.0` |
| **Timestamp** | `2026-08-06` |
| **Author** | Christophe Duy Quang Nguyen |
| **Location** | Bao Loc, Vietnam |
| **Repository** | [`cdqn5249/cdqn`](https://github.com/cdqn5249/cdqn) |
| **AI Engine** | Qwen — Qwen Studio, Alibaba Group |
| **License** | [Scaling Source License (SSL v1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

## ⬡ Document Profile

- **id:** `sqs-def`
- **version:** `1.0`
- **type:** `spec`
- **layer:** `middle`
- **depends:** `simemp-manifesto` · `qn-def` · `qn-base` · `qn-primitives`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

This specification defines the **Self-Quang System (SQS)** — an autonomous, self-similar, capability-bounded composite structure created by chaining primitive Quang Numbers across a physical computing substrate. It defines substrate neutrality, structural topologies (`Q_tree` / `Q_graph`), cybernetic feedback with zero-day immunization, zero-trust security with a substrate replay shield, the layered signature scopes, the Freshness Law, and the Ceiling-Boundary Renderer (Gateway) for pure legacy nodes. It adopts the cautious derive-then-verify epistemology of the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## 1. Epistemological Foundation

> **Governed by:** `[I]` · `[M]`

A Self-Quang System (SQS) is an autonomous, self-similar, capability-bounded composite structure created by chaining primitive Quang Numbers across a physical computing substrate. Because the Existence Triad `[I] × [S] × [M]` is **fractal**, an SQS represents the bridge between micro-level hardware register bit-atoms (`Qn,64`) and macro-level distributed AI swarms.

An SQS is strictly governed by the SIMEMP Manifesto and is **constructible** from the layers below it (manifesto §VI): every composite state preserves `I × S × M`. It functions as an active cybernetic system that ingests execution feedback to adapt its local constraints dynamically.

---

## 2. Universal Substrate Neutrality

> **Governed by:** `[P]`

To a Self-Quang System, physical hardware architecture is fractal and substrate-neutral. An SQS governs chained Quang Numbers whether they move across intra-chip silicon registers or global wireless networks.

> **`Substrate Difference = ⟨ Δτ (Latency Scale) , ⟨ I_src → I_dst ⟩ (Addressing Space) ⟩`**

- **Local Hardware Substrate (Intra-SoC):** latency `Δτ ∼ 0.2 ns to 100 μs`; addressing uses hardware register offsets (`r0:r1`) or memory arena addresses.
- **Remote Network Substrate (Inter-Node Mesh):** latency `Δτ ∼ 1 ms to 10 s`; addressing uses cryptographic public-key hashes or IP network endpoints.

Because the underlying state payload `⟨ S , V(Qexpr) , M ⟩` is identical across substrates, local bus operations and remote network packets execute on the exact same code contracts without re-serialization loss.

---

## 3. SQS Structural Topologies

> **Governed by:** `[Mod]`

The Dependencies property (`D_dep ∈ M`) dictates how Qn atoms link together, forming two fundamental structural topologies:

**Inheritance Trees (`Q_tree`)**
- **Monolithic Encapsulation:** child nodes depend directly on parent nodes to exist.
- **Private Scope Lock:** children are strictly private to the parent tree scope and cannot be shared externally.
- **Atomic Completeness:** if any leaf node fails or evaluates to `∅`, the entire tree collapses to `∅`.
- **Use Case:** high-security hardware enclaves, isolated financial transactions, private keys.

**Composition Graphs (`Q_graph`)**
- **Shared Capability DAGs:** self-contained 64-bit Qn atoms link via capability matching.
- **Public Node Sharing:** individual nodes can be referenced simultaneously across multiple independent graphs.
- **Localized Resilience:** failure of one node does not invalidate unrelated subgraphs.
- **Use Case:** multi-agent memory swarms, sublinear network search, global knowledge graphs.

---

## 4. Execution Feedback Loops & Zero-Day Immunization

> **Governed by:** `[S]` · `[E]`

Every SQS execution cycle generates a native Execution Feedback Signal (`F`) that feeds directly into the Best-of-Choices (BoC) policy engine:

1. **Execution Anomaly / Invariant Failure Occurs** — an unauthorized payload or zero-day exploit attempts an invalid register or state transition.
2. **Instant Null Law Isolation (`Q_fault = ∅`)** — the `I × S × M` invariant check fails; state creation is blocked in **O(1)** (one gatekeeper cycle) before hardware registers execute.
3. **Telemetry Captured in `Q(reuse)`** — error feedback telemetry (`F_error`) is saved in local SRAM `Q(reuse)` memory.
4. **BoC Dynamic Immunization** — BoC ingests `F_error`, tightens local DCC constraints (`C_con`), and propagates updated rules across neighboring SQS nodes in sublinear O(log N) time.

---

## 5. Zero-Trust Security & Substrate Replay Shield

> **Governed by:** `[S]` · `[I]`

To eliminate confused-deputy vulnerabilities, cross-substrate privilege escalation, and replay fraud, every SQS enforces four zero-trust security rules:

- **Zero Ambient Authority:** every atomic Qn word carries intrinsic signed provenance (`S`). No local hardware bus is trusted implicitly.
- **Substrate & Epoch Binding (transport-level Σ):** signatures bind source identity, destination, and monotonic temporal epoch:

> **`Σ_transport = Sign( I_src ‖ I_dst ‖ I_epoch ‖ V(Qexpr) ‖ M )`**

  Replaying a local SRAM state over a remote network socket breaks `Σ`, triggering instant rejection (`Q_fault = ∅`).
- **Capability Scope Locking:** capabilities (`K_cap`) are cryptographically bound to origin `I_src`. Remote nodes cannot invoke local hardware capabilities.
- **Constant-Time Execution:** operations within zoom window `z` execute in a **bounded constant cycle count independent of secret input values**; the bound is fixed by `z`, not by the secret data, eliminating timing side-channel leaks (`Δτ` leakage). Soft-Halt residuals are persisted rather than extending secret-dependent time.

### 5.1 Layered Signature Scopes (reconciliation)

> **Governed by:** `[S]`

Two layered scopes coexist and compose:

- **Atom-level Σ** ([`qn-def.md`](qn-def.md) §2.1): integrity proof over `I ‖ V ‖ M` — self-contained; verifies the atom itself.
- **Transport-level Σ** (this §5): wraps the atom Σ and adds `I_dst ‖ I_epoch` for cross-substrate replay protection.

### 5.2 The Freshness Law

> **Governed by:** `[S]`

Every operation/transfer has a source `I_src` and destination `I_dst`, each bound to a monotonic epoch `I_epoch`. A state is valid only if its source epoch is current (`state_epoch == I_epoch`). If `state_epoch < I_epoch`, the state is **stale** → `Null Law (∅)`. Stale state is a source of non-halting and must never be reused. Freshness is a precondition of the halt guarantee.

### 5.3 Immutable Silicon Harness & Human-Accountability Gate

> **Governed by:** `[S]` · `[I]`

A composite SQS agent (`I_machine`) may *generate* within its bounds but is cryptographically forbidden from *altering* its own constraints (`C_con`, `T0`, BoC weights). Any such modification strictly requires an accountable **`I_human`** signature (manifesto §VII.3). The sandbox is a physical law, not a software policy.

---

## 6. Ceiling-Boundary Renderer (Gateway) for Legacy Nodes

> **Governed by:** `[P]` · `[E]`

Two receiver classes are distinguished at the Ceiling Boundary:

- **Weak CDQN node:** has a `QnIR` runtime but limited compute. Uses the **BoC Adaptive Fallback**: it requests a pre-evaluated coarse view (`V_z_macro`) from an adjacent high-capacity SQS node via sublinear O(log N) routing, then renders locally.
- **Pure legacy node:** has **zero** CDQN capability (a 2010 browser, a simple HTTP client). It cannot interpret a raw `Qexpr` token. A **Ceiling-Boundary Renderer (Gateway)** — a CDQN-capable SQS node — evaluates the `Qexpr` and emits the exact fixed legacy format the receiver understands (HTML, JSON, PDF). The legacy node never sees the raw `Qexpr`; the Gateway signs the rendered output with its own `I_src`, preserving provenance across the boundary.

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Documentation portal & navigation map → [`index.md`](index.md)
- Qn Trinity & Metric Anchor → [`qn-def.md`](qn-def.md)
- Digit seeds, Q(harvest), Dual-Ring PQC → [`qn-base.md`](qn-base.md)
- Qn,64 ABI & Status codes → [`qn-primitives.md`](qn-primitives.md)
- Canonical term definitions → [`glossary.md`](glossary.md)
