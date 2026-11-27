---
layout: default
title: QnLang Toolchain — Unified `cdqn`-to-Toolchain Architecture (v0.1.0-draft.1)
description: CANDIDATE specification — `Q(rsi)` refinement, four-zone architecture, `.qnl` toolchain artifact ladder, `Q(use)` loading cascade, and complexity/supply-chain guardrails for QnLang/`qnlangch`
version: 0.1.0-draft.1
updated: 2026-11-26
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnlang_toolchain.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnlang_toolchain.html
status: Candidate specification (Axiom 14); not canon; QnIR and QnLang remain [PENDING]
terms_used: []
---

# QnLang Toolchain — Unified `cdqn`-to-Toolchain Architecture (v0.1.0-draft.1)

This document is **documentation only**. It carries `State: CANDIDATE` under **Axiom 14** and is not canon: it amends no committed artifact, promotes no primitive, allocates no opcode/envelope key, and adds no implementation. `QnIR` and `QnLang` remain `[PENDING]`; `qnlangc` is a Stage-0 Rust seed, and `qnlangch` is not yet built. It cross-references [`docs/qnlang_custody.md`](qnlang_custody.md), [`docs/qnlang.md`](qnlang.md), and [`docs/qnir_bootstrap.md`](qnir_bootstrap.md) without modifying any of them.

---

## 1. Epistemic Stance & `Q(rsi)` Refinement Rule

1. **Candidate, not canon.** Everything below is a `State: CANDIDATE` under **Axiom 14**; it is a plan of record, not a discharge. No sealed execution, promotion, or `Q1`/`Q3`/`Q4` advance is claimed.
2. **`Q(rsi)` iterative refinement.** The architecture evolves iteratively through `Q(rsi)` (recursive self-improvement workflows): a successor is a **new immutable versioned artifact** under **Axiom 13**, never a mutation in place. When a refinement step would invalidate a prior assumption, it is recorded as provisional and re-adjudicated under BOC; nothing silently rewrites an earlier receipt.
3. **`Axiom 13` fallback.** Because no target is sealed, every phase has a safe fallback: keep the currently committed artifact (`qir-enc-5`, the existing `qnlangc`, the committed corpus) unchanged and emit a new linked version rather than mutating. In particular `qir-enc-5` remains the safe fallback floor under `docs/qnlang.md` §7.3 and `docs/qnir_bootstrap.md` §9.7; Step 6.3b (`OP_SATURATE` / `SAT_*`) stays open to multi-host re-evaluation without mutating `qir-enc-5`.
4. **Custody and source openness.** Open-plaintext `.qnl` source and the hash-chained, offline `WOTS+` root `Q(0)_H` described in `docs/qnlang_custody.md` are preserved; signing hashes, never closes, source. Public AI agents (`OpenCode`, cloud chat) are barred from real `Q(0)_H` material (§2 Zone 1).

---

## 2. Unified 4-Zone Architecture

### Zone 1 — Inner Ring (offline sovereignty & human co-signing)

- **Offline `Q(0)_H` root sovereignty.** The human principal's custody root `Q(0)_H` (pure-SHA-256 `WOTS+` Merkle root, `sha-256:<hex>`, `0x00`/`0x20` domain separation, strict monotonic `CausalIndex`, one-time leaf use; `docs/qnlang_custody.md` §2) is generated and held **offline**. **Public AI coding agents (`OpenCode`) and cloud chat sessions must never generate, read, hold, transmit, or reconstruct a real `Q(0)_H` private seed, leaf seed, or master-seed backup**; only ephemeral dummy fixtures are used for building/testing verifiers.
- **`Exposer` read-back + co-signing.** Before any `Compiler` output is committed, the **`Exposer`** (`qir-verbalizer-1`, `R12'`, `Obj 15`) renders a deterministic full-program read-back with `readback_sha256`. Emission requires the exact-digest **co-signing checkpoint**: `qnlangc` (and later `qnlangch`) enters a `PAUSED` state and the human principal confirms the exact read-back; `--certify-readback <sha256>` models this. A mismatch refuses emission (`E_CERTIFICATION_MISMATCH`) and writes zero bytes. Zone-1 keys never leave Zone 1.
- **Mandatory custody migration.** Provisional `WOTS+` v1 custody must migrate to the true Qn-PQC Dual-Ring design (`docs/qnlang_custody.md` §3) as soon as `Q4` exists, via a signed `docs/simemp.md` §4.3 Migration Receipt.

### Zone 2 — Local Execution & Intra-Node `cdqn` (Layers 0–3)

- **Pure 3-part `qnlangc`.** The compiler is exactly **`Checker` → `Exposer` → `Compiler`**: `Checker` (strict parse + `G1`–`G14` + `E_NAMESPACE_COLLISION`), `Exposer` (deterministic read-back + certification interlock), `Compiler` (`R11'` lowering). It is **pure**: no OS I/O, no network, no ambient state, and **no `build.rs` or install-time scripts**. The Stage-0 seed already realizes this; `qnlangch` preserves it.
- **`qir-enc-5` artifacts.** Emitted `qir-enc-5` / `qir-prof-ext-bootstrap-1` Intents use `ARRAY_RRB16` linear arenas (Layer-1–3 arena-AST `Q(array)`), `TABLE_MBTREE16` sparse digest store (`Q(table)`, canonical 32-byte SHA-256 roots), `OP_CALL`/`OP_RET` with `0x30` hash-chained frame stack (`C_max = 8`, `F_max = 64`), SCC-additive `bhat`, and `Q(reuse)` two-axis admission over memory tiers **`τ_reg .. τ_storage`** (locally, `τ_remote` is absent). Value equality is canonical-encoding comparison with `qir-artifact-cmp-1` collision buckets; `Q_HASH`/`sha-256:<hex>` is a commitment, not equality.
- **Local-first.** Everything in Zone 2 runs offline; no wall-clock or unmetered source; work is metered in `U` with `reserved == B_hat == charged + refunded`.

### Zone 3 — Toolchain Artifact Ladder (`CD 3+`, above `Qexpr`)

A deterministic `CD 3+` ladder over the `Qexpr` `CAE-DAG`, each rung an immutable versioned artifact:

1. **`Qn`** primitive → **Layer-2 morphisms** → **`Qexpr`** (`CAE-DAG`, content-addressed, `OP_BYPASS`-normalized).
2. **`Q(pattern)`** — the unit semantic transformation (key-9 callee under `qir-enc-5`).
3. **`Module`** — a `.qnl` file. **Header-first**: a canonical header `Q(use) @ sha-256:<digest>` pins every import's exact content digest; the body is an independent `CAE-DAG`. `O(1)` header preflight precedes any body read.
4. **`Library`** — a **pure acyclic DAG of `Module`s**. **Zero top-level `emit`** anywhere in the library; only `Q(pattern)`/`Q(workflow)` exports; unused-import rejection; deterministic topological order; `key 9` `callees` aggregated acyclically (`C_max = 8`, `S_max = 4096`).
5. **`Package`** — a `Library` plus a **Universal Envelope**: custody (`Q(0)_H`, provenance, signed read-back), **SSL-1.0** license lineage (Axiom 12), and a committed conformance-vector set. Tagged **`Beta`**.
6. **`Workspace` / `qnlangch`** — a `Q(workflow)` lock graph over `Package`s plus a **local `Q1` receipt index**; the harness drives reproducible builds. **Never-Feeds-Back**: build outputs are never inputs to the same build (no self-reference, no mutation-in-place); rebuilding a workspace re-emits identical bytes. Tagged **`Beta`**.

### Zone 4 — Outer-Ring `cdqn` (`Q4`; deferred)

Origin-quotiented transport of `H_pub` / `R_road` receipts over **`τ_remote`**, under the **same** `Q(reuse)` two-axis admission, `qir-artifact-cmp-1` comparison, and `RECEIPT_ANCHOR_REVOKED` rules as locally. No new token type, no implicit global state. Deferred until QnIR promotion and `Q1`–`Q3` (`Q4` builds on `Q1` + `Q3`); nothing here claims Outer-Ring consensus.

---

## 3. `.qnl` Shaping, Storage & `cdqn` Loading Cascade

- **`.qnl` shaping.** Source files use the `.qnl` extension (resolving the CodeQL collision; see `src/qnlangc/tests/fixtures/`). **Header-first `O(1)` preflight**: read the canonical header and pinned `Q(use)` digests before reading any body; reject an invalid/revoked header without touching body bytes.
- **Storage.** **`ARRAY_RRB16`** is the persistent arena for `CAE-DAG` nodes; **`TABLE_MBTREE16`** is the sparse content-addressed store for `Q_HASH`/digest-keyed entries; both expose canonical 32-byte SHA-256 roots and COW semantics with `cap(N)`/`max_cap = 65536`, `max_batch = 64`.
- **3-step loading cascade** for a `Q(use)`/`Module` load:
  1. **Header / revocation check** — verify the header digest and that the artifact is not revoked (`RECEIPT_ANCHOR_REVOKED`); refuse before body read.
  2. **`Q(reuse)` two-axis cache hit** — `Admit(Q(reuse)) = (ΔW_compute > β_dcc·(κ_U + ε_U)) ∧ (Δt_bound > β_t·(κ_t + ε_t))`; `β_dcc = 2`, `β_t = 4`, ties resolve to Direct Compute; check `qir-artifact-cmp-1` on the complete artifact before reuse.
  3. **`STATUS_LOOKUP_MISS` deterministic re-compile** — on a miss, run the pure `Checker → Exposer → Compiler`; a successful fallback consumes the miss token; no unbounded re-derivation.
- **Origin-quotiented `Q(reuse)`.** Cache keys are content digests, origin-quotiented so a reused artifact's provenance cannot fork identity; verification is independent of where it was fetched.

---

## 4. Explicit Complexity & Supply-Chain Guardrails (Anti-NP-Hard & Anti-`ARTEX`)

1. **Digest-pinned `Q(use)`.** Every `Q(use)` pins `@ sha-256:<digest>`. No semver range, no version-solving SAT, no lockfile solver. Graph admission is a single `O(|V| + |E|)` topological check for acyclicity plus `qir-artifact-cmp-1` identity; cycles are rejected.
2. **Rooted canonical Merkle DAGs.** Artifacts are rooted canonical Merkle DAGs: `Q(reuse)` and `qir-artifact-cmp-1` give `O(1)`/`O(log N)` lookup plus `O(bytes)` verification. **No general subgraph isomorphism**; value equality is canonical encoding, not pattern matching.
3. **Linear pre-DP structural gate.** `bhat` admission uses the SCC-additive count
   `Σ_i |SCC_i| · Π_{e ∈ SCC_i} (ceil_e + 1) <= S_max = 4096`, computed before any DP table is allocated; Tarjan condensation is `O(|V| + |E|)`. This bounds single-SCC loop nesting; globally non-admissible graphs are rejected, not approximated.
4. **Root-local `OP_BYPASS`.** `OP_BYPASS` canonical contraction is **root-local `O(1)`**; **no global AC subgraph matching** (no e-graph rewriting to fixpoint inside `qnlangc`/`QnIR`).
5. **`OP_SATURATE` (`QO-28`) outside core.** Bounded equality saturation is kept **outside `qnlangc` core and `QnIR`**, as a Layer-3 `Q(rsi)` compile-time transform emitting a new Axiom 13 artifact, using **polynomial-time greedy hash-tie-broken extraction** under `K_max`/`N_max`/`D_max`; optimal extraction is out of scope. `SAT_*` remains a separate candidate macro-lowered to `qir-enc-5` core. (See `docs/qnir_bootstrap.md` §9.7, `docs/qexpr.md` §4.6.2.)
6. **Anti-`ARTEX`.** No implicit halting-problem promises, no NP-hard `B_hat`/extraction, no unbounded re-derivation, no unbounded repetition, no implicit fallback, no unbounded capacity; every ceiling is finite and declared. Structural inability maps to a typed `EXPRESSION_TOO_COMPLEX`/`REJECTION_BY_GATEWAY`, never silent approximation.

---

## 5. Phased Rollout (`Alpha` → `Beta` → `cdqn Q4`)

| Phase | Contents | Status here |
|---|---|---|
| **`Alpha`** | `Module`/`Library` with header-first `Q(use) @ sha-256:<digest>`; `qir-enc-5` `qnlangc` target; **Stage-1/Stage-2 self-hosting** (`docs/qnlang.md` §7.2): Rung-0 source → generator → self-hosted `qnlangc` reproducing Stage-0 bytes | design target; not implemented |
| **`Beta`** | **`Package`** + **`Workspace` / `qnlangch`**; local `Q1` receipt index; **`WOTS+` v1 custody** (`Q(0)_H`, signed read-back, Migration Receipt) over a pinned reproducible `qnlangc`; Never-Feeds-Back; SSL-1.0 | `CANDIDATE`; `qnlangch` not built; `qnlang_custody.md` `CANDIDATE` |
| **`cdqn Q4`** | **`τ_remote`** Outer-Ring transport of `H_pub`/`R_road`; **Dual-Ring PQC migration** to the Qn-PQC Dual Ring (`docs/abstractionLayers.md` §8.3) via signed `docs/simemp.md` §4.3 Migration Receipt | deferred until QnIR promotion + `Q1`–`Q3`; `Q4` `QUEUED` |

Mandatory migration binding `WOTS+` v1 → Qn-PQC Dual-Ring at `Q4`, and the `Q(rsi)` reversibility rule, are inherited unchanged from `docs/qnlang_custody.md`.

---

## License and Invariant Lineage

This document is a new `CANDIDATE` under **Axiom 13/14**, license **Scaling Source License (SSL 1.0)** (`LICENSE.md`, repository root), preserving the canonical paternity reference. It changes no byte of `src/qnir/`, `src/qnir-py/`, `src/qnlangc/`, `docs/qnir_goldens.md`, `docs/qnlang.md`, `docs/qnir_bootstrap.md`, `docs/qnlang_custody.md`, `docs/index.md`, `docs/_data/navigation.yml`, the glossary, or any `docs/logs/` record; no new glossary term, opcode, envelope key, token type, or terminal outcome is introduced; `QnIR` and `QnLang` remain `[PENDING]`, and `Q1`/`Q3`/`Q4` remain `QUEUED`. `QnIR`'s `C_max`, `F_max`, `S_max`, `max_cap`, `max_batch`, `max_hash_bytes`, and charge coefficients are profile parameters riding `V`, not core validity limits.
