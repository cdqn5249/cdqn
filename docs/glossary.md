[⬡ Back to Documentation Portal](index.html)

# Canonical Glossary — Qn / CDQN Terminology

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

- **id:** `glossary`
- **version:** `1.0`
- **type:** `glossary`
- **layer:** `middle`
- **depends:** `simemp-manifesto` · `qn-def` · `qn-base` · `qn-primitives` · `sqs-def`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

This glossary is the **single source of truth** for all Qn/CDQN terms. Every other document links here for canonical definitions; no term is defined in more than one place. It adopts the cautious derive-then-verify epistemology of the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## 1. Governance & Epistemology

> **Governed by:** `[I]` · `[S]` · `[M]`

- **SIMEMP** — the axiomatic framework of the Qn/CDQN universe: invariants `[I]·[S]·[M]`, agiles `[E]·[Mod]·[P]`, and the BoC policy engine.
- **Existence Triad (`[I]×[S]×[M]`)** — the coupled existence criterion; a Qn exists iff all three hold.
- **`[I]` Identity** — topological existence rooted in physical silicon (`Q(harvest)`); *who/what computes*.
- **`[S]` Security** — cryptographic provenance `⟨I, Σ⟩` **plus enforced termination**; *is it authorized and will it halt*.
- **`[M]` Metric** — finite, computable bounds and halting space; *within what finite bounds*.
- **`[E]` Efficiency** — Space-Time Memory Dilation; respects the physical Memory Wall.
- **`[Mod]` Modularity** — arena value-semantics; zero garbage collection.
- **`[P]` Portability** — target-neutral execution via `Qexpr → QnIR`.
- **BoC (Best of Choices)** — dynamic policy engine tuning `E/Mod/P` without breaching the Triad floor; also the three-way cost selector `{BYPASS, REUSE, COMPUTE}`.
- **Derive-then-Verify** — the epistemology: define axioms, derive consequences, verify by computation via QnLang; no proof claims.
- **Constructibility / Ascending-D Grounding** — organizing criterion; each layer is constructed from layers below, preserving `I×S×M`; `D` is derived, not imposed.
- **Null Law (∅)** — invalid `Σ`, undefined `M`, or stale state ⇒ `Qn = ∅`.
- **Freshness Law** — a state is valid only if its source epoch is current; stale ⇒ `∅`.
- **Law of Authenticated Topology** — the network is a topology of identified, hardware-bound nodes; no anonymous ghosts.
- **Causal Verification Asymmetry Law** — valid states are causally gated behind verification; brute force is structurally slower than key-holding.
- **Immutable Silicon Harness / Human-Accountability Gate** — an agent (`I_machine`) cannot alter its own constraints; an `I_human` signature is required.
- **Mind / Matter / Joint** — epistemic ceiling: unverified intent (Mind), verified result (Matter), lawful transition (Joint).

---

## 2. Core Qn Type

> **Governed by:** `[I]` · `[S]` · `[M]`

- **Qn (Quang Number)** — capability-bounded algebraic type abstracting state, operations, semantics, and intent.
- **Quang** — luminescent/clear; structured, verifiable clarity.
- **Trinity Tuple** — `Qn = ⟨ S , V(Qexpr) , M ⟩`.
- **Qexpr (Quang Expression)** — homoiconic chained-Qn payload; a multivariate polynomial resolved via `SUBST`/`ELIM`; the glue between `Qm` and `Qs`.
- **Qm (Quang Maths)** — deterministic algebraic, geometric, and tensor evaluations.
- **Qs (Quang Semantics)** — natural language, context, intent, knowledge graphs, logic trajectories.
- **Qs ⇒ Qm Symbiosis** — intent chains delegate exact calculation to math chains; the anti-hallucination mechanism.
- **DCC Profile** — `⟨ D_dep, C_con, K_cap ⟩` self-describing profile inside `M`.
- **D_dep / C_con / K_cap** — Dependencies / Constraints (Negative Capabilities) / Capabilities.
- **V_z** — active view at zoom `z`.
- **R (Residual)** — `⟨R_macro, R_micro⟩` persisted on Soft Halt.
- **z (Two-Way Zoom)** — `⟨z_macro, z_micro⟩` scale ceiling and resolution floor.
- **D (Dimensionality)** — universal dimension parameter; two regimes (integer Hurwitz / fractional fractal).
- **C (Chaining Hook)** — topological link specifying operators (`Qm`) or predicates (`Qs`).
- **P (Sparse Pivots)** — vantage-point vector enabling O(1) Triangle Inequality Pruning for O(log N) search.
- **Canonical Projected Ratios `[N, D]`** — exact representation replacing IEEE-754.
- **Lazy Canonicalization** — defer reduction until crossing an `S × M` boundary.
- **Total Functionality** — `QnIR` is total (Turing Incomplete); provable time/space bounds.

---

## 3. Execution & Memory

> **Governed by:** `[E]` · `[S]`

- **QnLang** — the compiler.
- **QnIR** — the total intermediate representation / runtime.
- **EVAL / SUBST / ELIM / FOLD** — the four fundamental opcodes.
- **Q(reuse)** — retrieval of past computed states (only when cheaper than recompute) plus residual persistence.
- **Q(bypass)** — O(1) short-circuit for proven identities (`x+0`, `x·1`, `x·0`, `x÷1`).
- **Hard Halt** — `R = 0`; exact evaluation; zero residual memory.
- **Soft Halt** — `R ≠ 0`; residual persisted for O(1) depth expansion.
- **Space-Time Memory Dilation** — spatial (zoom) bandwidth compression + temporal (250-cycle DRAM) active execution window.
- **Aperiodic Einstein Tile** — quasicrystalline memory partitioning eliminating periodic hash collisions.

---

## 4. Base-10 Seeds & Cryptography

> **Governed by:** `[I]` · `[S]` · `[M]`

- **Qorigin** — topological/cryptographic origin anchor `(0,0,…,0)`; root provenance `I_origin`.
- **Qzero (Q0)** — arithmetic scalar zero; additive identity; multiplicative annihilator.
- **Digit Seeds Q0–Q9** — base-10 ontological atoms with explicit DCC profiles.
- **𝕆_genesis** — sequential genesis ordering axiom.
- **Q(harvest)** — physical silicon entropy harvesting at onboarding.
- **e_harvest** — master hardware entropy vector `⟨e_origin, e_digits, e_ops, e_logics⟩`.
- **Q(ID)_local** — local device identifier bound to silicon.
- **Genesis Tile T0** — tile zero; root of outward memory inflation.
- **Q(ops) / Q(logics)** — primary operators `{+,·}` / unary inverses `{-x, x⁻¹}` and boolean gates.
- **Q(anchors)** — sparse landmark fences (`Q(primes)`, `Q(twinPrimes)`).
- **Isometric Mirror Anchor Law** — `d(Qorigin, +Q) ≡ d(Qorigin, −Q)`.
- **Q(crypto) / Qnet(crypto)** — Ring 1 local / Ring 2 network Post-Quantum Cryptography.
- **B_net** — composite network proof `∑ b_i (mod q)`.
- **Dual-Ring PQC Hardness Conjecture** — NP-hard (conjecture); concrete attack cost scales `2^Ω(k)`.

---

## 5. Hardware ABI

> **Governed by:** `[M]` · `[P]`

- **Qn,64** — the universal 64-bit physical bit-atom.
- **Silicon Gatekeeper Law** — `Lower(Data) ∉ {Qn,64} ⇒ Qn = ∅`.
- **r0:r1** — 32-bit register-pair mapping (control / data).
- **Status Field** — 2-bit execution feedback (`VALID_EXACT`, `VALID_RESIDUAL`, `INVARIANT_FAULT`, `ANOMALY_QUARANTINE`).
- **V_mode** — payload discriminator (ratio `[N,D]` vs arena offset).
- **Little-Endian** — universal byte ordering.

---

## 6. SQS & Network

> **Governed by:** `[P]` · `[S]`

- **SQS (Self-Quang System)** — autonomous, self-similar, capability-bounded composite of chained Qn.
- **Substrate Neutrality** — identical payload across registers, buses, and sockets.
- **Δτ** — latency scale distinguishing substrates.
- **I_src / I_dst / I_epoch** — source / destination / monotonic temporal epoch identity.
- **Q_tree / Q_graph** — inheritance tree / composition graph topologies.
- **F / F_error** — execution feedback signal / error telemetry.
- **Zero-Day Immunization** — BoC dynamic constraint tightening propagated in O(log N).
- **Gateway / Ceiling-Boundary Renderer** — CDQN node that renders `Qexpr` into legacy formats for pure legacy nodes.
- **I_machine / I_human** — agent identity / accountable human identity.

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Documentation portal & navigation map → [`index.md`](index.md)
- Qn Trinity & Metric Anchor → [`qn-def.md`](qn-def.md)
- Digit seeds, Q(harvest), Dual-Ring PQC → [`qn-base.md`](qn-base.md)
- Qn,64 ABI & Status codes → [`qn-primitives.md`](qn-primitives.md)
- SQS topologies & zero-trust → [`sqs-def.md`](sqs-def.md)
