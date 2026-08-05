# CDQN Documentation Portal — Chained & Distributed Quang Numbers

[Next: The SIMEMP Manifesto →](simemp-manifesto.md)

## Metadata

| Field | Value |
| :--- | :--- |
| **Version** | `1.0` |
| **Timestamp** | `2026-08-05` |
| **Author** | Christophe Duy Quang Nguyen |
| **Location** | Bao Loc, Vietnam |
| **Repository** | [`cdqn5249/cdqn`](https://github.com/cdqn5249/cdqn) |
| **AI Engine** | Qwen — Qwen Studio, Alibaba Group |
| **License** | [Scaling Source License (SSL v1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

## ⬡ Document Profile

- **id:** `index`
- **version:** `1.0`
- **type:** `portal`
- **layer:** `ceiling`
- **depends:** `simemp-manifesto`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

This is the root entry point and navigation map of the CDQN (Chained & Distributed Quang Numbers) project. It presents the inquiring thesis, the abstraction stack, and the documentation suite from which every specification derives. It is the progressive-disclosure portal: an agent or human loads this file first, then follows links only to the documents it needs. All documents share the same Markdown blueprint and the cautious derive-then-verify epistemology defined in the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## I. The Thesis & Epistemology

### I.1 The Inquiring Thesis

> *"Can a computing number system abstract arbitrary state and reasoning?"*

The CDQN Stack answers by shifting abstraction from **concealing complexity** (opaque black boxes) to **structuring complexity** (luminescent, verifiable clarity — "Quang"). At its core is the **Quang Number (Qn)** — a capability-bounded algebraic type that unifies abstract algebra, zero-drift arithmetic, and persistent memory.

### I.2 The Epistemological Stance

> **Governed by:** `[S]`

We **do not claim mathematical proofs.** We define axioms, derive consequences logically, and verify by computation: the QnLang compiler shall execute our constructions and confirm they reproduce the results of IRL mathematics, semantics, and physics. All hardness assumptions are labeled **conjectures**. See [`simemp-manifesto.md`](simemp-manifesto.md) §I.4.

### I.3 The Canonical Qn State

A Qn is instantiated as the Trinity Tuple, governed by the Existence Triad:

> **`Qn = ⟨ S , V(Qexpr) , M ⟩`** — exists if `[I] × [S] × [M]` hold.

- **`[I]` Identity** — topological existence, rooted in `Q(harvest)`.
- **`[S]` Security** — provenance `⟨I, Σ⟩` + enforced termination.
- **`[V]` Value** — homoiconic `Qexpr` payload, un-truncated.
- **`[M]` Metric** — finite bounds, halting space.

---

## II. The Abstraction Stack

> **Governed by:** `[I]` · `[M]`

The stack is organized by **constructibility / emergence** (ascending-D grounding), not dimensionality alone. Every upper layer is constructible from the layers below and preserves `I × S × M`.

```text
   ▲ higher D
 ┌──────────────────────────────────────────────────────────┐
 │  CEILING — Mind / Matter / Joint  (AI-agent epistemics)  │
 ├──────────────────────────────────────────────────────────┤
 │  CDQN Network  (Dual-Ring PQC · Qnet(crypto))            │
 ├──────────────────────────────────────────────────────────┤
 │  SQS — Self-Quang System  (chained Qn across substrate)  │
 ├──────────────────────────────────────────────────────────┤
 │  QnLang (compiler) → QnIR (total runtime)                │
 ├──────────────────────────────────────────────────────────┤
 │  Qexpr = chained Qn = cdqn structure (the GLUE)          │
 │     Qm (Maths) ◄── C-hook ──► Qs (Semantics)             │
 ├──────────────────────────────────────────────────────────┤
 │  Qn Type — Trinity ⟨ S , V(Qexpr) , M ⟩                  │
 ├──────────────────────────────────────────────────────────┤
 │  Base-10 Seeds & Origin  (Q0–Q9 · Qorigin · Q(harvest))  │
 ├──────────────────────────────────────────────────────────┤
 │  FLOOR — Qn,64 silicon bit-atom (WHERE ABSTRACTION ENDS) │
 └──────────────────────────────────────────────────────────┘
   ▼ lower D
```

---

## III. Foundational Pillars

> **Governed by:** `[E]` · `[Mod]` · `[P]`

Three breakthroughs bridge high-level cognitive semantics down to bare-metal registers.

### III.1 Space-Time Memory Dilation — `[E]`
Respects the physical Memory Wall. Uses Two-Way Zoom `z` for spatial bandwidth compression and exploits the 250-cycle DRAM fetch delay as an active execution window for `Q(reuse)` residual folding and `Q(bypass)` identity short-circuiting.

### III.2 Quang Expressions (Qexpr) — `[Mod]`
Unified intermediate representation bridging exact math (`Qm`) and natural-language semantics (`Qs`). Encodes context as multivariate polynomials solved via substitution and elimination, enabling lossless lowering of legacy data.

### III.3 Substrate-Neutral SQS ABIs — `[P]`
Self-Quang Systems operate fractally across local registers, SRAM arenas, and remote networks. Physical bit-atoms (`Qn,64`) enforce zero-trust at the register boundary, eliminating confused-deputy and replay attacks.

---

## IV. Documentation Suite

> **Governed by:** `[M]`

Select a document to explore the foundational laws, formal specifications, hardware ABIs, and legal terms. Each follows the same Markdown blueprint and derive-then-verify epistemology.

| Document | Scope & Contents | Status |
| :--- | :--- | :--- |
| [`simemp-manifesto.md`](simemp-manifesto.md) | **The SIMEMP Manifesto** — root axiom. Existence Triad `[I]×[S]×[M]`, agiles `[E]·[Mod]·[P]`, BoC, constructibility, trust & epistemic laws. | ✅ committed |
| [`qn-def.md`](qn-def.md) | **The Formal Qn Definition** — Trinity Tuple, `Q(reuse)`, Soft/Hard Halting, `Q(bypass)`, Qm/Qs domains, distance `d`. | 🕐 planned |
| [`qn-base.md`](qn-base.md) | **Base-10 Seeds & Cryptography** — digit seeds `Q0–Q9`, `Qorigin`, `Q(harvest)`, Dual-Ring PQC, dimensional laws. | 🕐 planned |
| [`qn-primitives.md`](qn-primitives.md) | **Qn Primitives ABI** — bit-level `Qn,64` layout, register mapping, SIMD scaling, status flags, cache alignment. | 🕐 planned |
| [`sqs-def.md`](sqs-def.md) | **The Formal SQS Definition** — substrate-neutral composite systems, `Q_tree`/`Q_graph`, zero-day immunization, Gateway. | 🕐 planned |
| [`glossary.md`](glossary.md) | **Canonical Glossary** — single source of truth for all Qn/CDQN terms. | 🕐 planned |

---

## V. Repository Structure

> **Governed by:** `[Mod]`

The documentation follows a flat, progressive-disclosure layout. An agent loads `index.md` first, then navigates to only the documents it needs.

```text
docs/
├── index.md                 <- this portal (root)
├── simemp-manifesto.md      <- root axiom
├── qn-def.md                <- Formal Qn definition
├── qn-base.md               <- Base-10 seeds & crypto
├── qn-primitives.md         <- Binary ABI
├── sqs-def.md               <- Formal SQS definition
└── glossary.md              <- Canonical term definitions
```

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Qn Trinity & Metric Anchor → [`qn-def.md`](qn-def.md)
- Digit seeds, Q(harvest), Dual-Ring PQC → [`qn-base.md`](qn-base.md)
- Qn,64 ABI & Status codes → [`qn-primitives.md`](qn-primitives.md)
- SQS topologies & zero-trust → [`sqs-def.md`](sqs-def.md)
- Canonical term definitions → [`glossary.md`](glossary.md)
