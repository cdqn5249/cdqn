---
layout: default
title: QnLang Custody — Qn-Native Custody & qnlangch Lineage Plan (v0.1.0-draft.1)
description: CANDIDATE custody specification — open plaintext source invariant, PROVISIONAL algorithm-tagged WOTS+ Merkle root Q(0)_H over SHA-256, monotonic CausalIndex leaf discipline, signed migration receipts, reproducible verifier-side re-checking, and a mandatory migration to the Dual-Ring PQC design
version: 0.1.0-draft.1
updated: 2026-11-25
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnlang_custody.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnlang_custody.html
status: Candidate specification (Axiom 14); not canon; QnIR and QnLang remain [PENDING]
terms_used:
  - causal-arrow
  - dual-ring-pqc-boundary
  - licensed-work
  - no-implicit-rule
  - open-core-invariant
  - paternity-reference
  - receipt
  - ssl
  - structural-indirection
---

# QnLang Custody — Qn-Native Custody & `qnlangch` Lineage Plan (v0.1.0-draft.1)

This document is **documentation only**. It carries `State: CANDIDATE` under **Axiom 14** and is not canon: it amends no committed artifact, promotes no primitive, and adds no implementation. `QnIR` and `QnLang` remain `[PENDING]`, the five committed QnIR fixtures are untouched, and no byte of `src/qnir/`, `src/qnir-py/`, or `src/qnlangc/` is changed by this wave. The concrete custody tooling described here is **explicitly deferred until `qnlangch` — the QnLang compiler harness — exists**; until then this file is a plan of record, not a discharge.

Custody here means *lineage custody*: the ability to prove, offline and with the repository's own primitives, that a wave's source and emitted artifacts are exactly what a human principal approved, in causal order, without closing the source and without depending on an external signing stack.

---

## 1. Scope & Open-Source Invariant

### 1.1 What custody covers

1. **Wave commitments.** Each custody wave binds a canonical manifest of `(path, sha-256:<hex>)` entries to its parent wave, forming a hash chain over the repository's own history.
2. **Approval root.** Each wave is approved by a one-time `WOTS+` leaf whose Merkle root `Q(0)_H` is the human principal's custody root for that lineage segment.
3. **Causal position.** The leaf's position is the wave's strict `CausalIndex`, so approval order is a property of the commitment, not of a filesystem timestamp.
4. **Verification.** A verifier re-derives the chain, the leaf, and the root from committed bytes alone.

### 1.2 The open-source invariant (hard)

Source `.ql` files, `docs/`, the emitted `qir-enc-4` artifact bytes, and every reproduced transcript remain **100% open plaintext**. Custody hashes and signatures are *witnesses over* plaintext; they never encrypt, obfuscate, minify, or otherwise close source. This is the operational form of **Axiom 12** — the canonical {% include term.html id="paternity-reference" %} and lineage metadata survive export — and it is consistent with the {% include term.html id="ssl" %} and the {% include term.html id="open-core-invariant" %}.

Consequences, stated as prohibitions:

- No custody step may require a binary-only or source-available-but-closed distribution of QnLang source.
- A failed signature check is a *verification* failure, never a license or access failure: the plaintext remains readable and the failure is reported as a typed {% include term.html id="receipt" %}.
- `Q(0)_H` proves *who approved which bytes in which order*; it does not restrict reading them.

### 1.3 Deferral boundary

Implementation is deferred until `qnlangch` (QnLang compiler harness) exists, because the harness is where the three pieces meet: the pinned toolchain, the deterministic `R11'` lowering, and the verifier-side re-check. This file therefore specifies *interfaces and invariants*, not modules, CLIs, or file formats that would bind a future implementation prematurely. Under the {% include term.html id="no-implicit-rule" %} no such interface is assumed to exist today.

---

## 2. Algorithm-Tagged Custody v1 `[PROVISIONAL]`

Everything in this section is tagged **`[PROVISIONAL]`**: it is a candidate construction to be superseded by the migration of §3, not a long-lived primitive. It uses **pure `SHA-256` only** — no OpenSSH, no GPG, no external cryptographic crate, no vendored binary.

### 2.1 Manifest chain and wave commitments

For wave `w`, let `M_w` be the canonical manifest: a lexicographically sorted sequence of `(repo-relative path, sha-256:<hex>)` pairs covering the wave's added or modified files, plus the manifest identity of the parent wave.

```
C_0   = SHA-256("qnlang-custody-manifest-v1" || 0x00 || canonical(M_0))
C_w   = SHA-256("qnlang-custody-manifest-v1" || 0x00 || C_{w-1} || canonical(M_w))
```

- `canonical(...)` is deterministic, length-prefixed, and independent of locale, filesystem case-folding, and line-ending normalization: the digest is over committed bytes, so a CRLF/LF rewrite is a *different* commitment and must be visible.
- Omitting a file that the wave changed is a manifest defect, not a permissible minimization; the verifier enumerates the wave's diff from the committed history and compares sets exactly.
- Each `C_w` is rendered with an explicit algorithm tag, `sha-256:<hex>`; a bare un-tagged digest is not admissible across a gateway (the rule of `docs/simemp.md` §4.3 *Migration Witnessing*).

### 2.2 `WOTS+` Merkle root `Q(0)_H`

`Q(0)_H` is the root of a `WOTS+`-style one-time-signature Merkle tree, built over `SHA-256` with domain separation:

| Role | Preimage | Domain byte |
|---|---|---|
| Leaf secret / chain step | `SHA-256(0x00 \|\| ADRS \|\| x_j)` | `0x00` |
| Merkle internal node | `SHA-256(0x20 \|\| left \|\| right)` | `0x20` |

- `ADRS` is a fixed-width, 32-byte address encoding `(leaf_index, chain_index)`; it is part of the preimage so that chain steps are not transferable between leaves.
- Leaf seeds are derived from the offline master seed by a domain-tagged `SHA-256` PRF; no seed material is ever written to the repository.
- The root is published as `sha-256:<hex>` and is the only custody value that may appear in committed bytes.
- This is a **custody-integrity** construction, not a general-purpose replacement for a standardized signature scheme. Its one-time nature is the load-bearing property, and it is exactly why §2.3 exists.

### 2.3 Strict monotonic `CausalIndex` — no leaf reuse

Under **Axiom 5** a successor's index strictly exceeds its parent's along the {% include term.html id="causal-arrow" %}. Custody binds that index directly:

- `leaf_index := CausalIndex(wave)`, with `CausalIndex(w) > CausalIndex(w-1)` strictly.
- A `WOTS+` leaf is used **exactly once**. Re-signing the same wave, re-signing a corrected wave under the same index, or signing two manifests at one index is prohibited and is a custody failure even if every signature verifies.
- Consequently, a rejected wave is superseded by a *new* wave at a *new* index; the rejected manifest is retained as a historical anchor, never overwritten (Axiom 13).
- The verifier rejects any chain in which an index repeats, decreases, or is reused across sibling branches.

### 2.4 Master seed custody, backup, and rotation

- The `Q(0)_H` master seed is generated and held **offline**, on a machine that is not network-attached, under human control.
- Backup is a human-legible offline artifact (paper or metal), stored separately from the signing device. Backup integrity is checked by re-deriving `Q(0)_H` from the backup and comparing it to the published root.
- Loss, compromise, or suspected compromise of the signing device is handled by **rotation**, never by silent re-use: the old root is retired, a new root is published at a strictly greater `CausalIndex`, and the transition is recorded as a **Migration Receipt** signed under both the superseded and successor material, following the migration-receipt discipline of `docs/simemp.md` §4.3.
- The receipt records the superseded and successor algorithm identifiers, the last consumed leaf index, the first index of the new segment, and the re-witnessing plan for prior commitments. Historical commitments are preserved by re-witnessing: re-digest under the successor primitive and re-seal with fresh signatures, each record anchoring back to the original commitment so lineage remains verifiable across the full algorithm history.

### 2.5 Reproducible `qnlangc` and verifier-side re-checking

Custody of an emitted artifact is only meaningful if the artifact is reproducible and the verifier does not trust the compiler:

1. **Pinned toolchain.** `qnlangch` builds with a committed `rust-toolchain.toml` pin and no floating channel. Today `src/qnlangc/` is already **standard-library only** (empty `[dependencies]` in `Cargo.toml`); pinning the toolchain file is part of the deferred harness work, not a claim about the current tree.
2. **Byte-reproducible lowering.** For a fixed source and toolchain, `R11'` lowering of `docs/qnlang.md` §7.1 must emit byte-identical CBOR, including the compositional `B_hat`. Reproducibility is checked by emission twice into separate scratch directories and comparing bytes, not by trusting a cache.
3. **Verifier authority.** Security rests on the verifier, never on compiler DRM or compiler attestation. The verifier independently re-runs the `R11'` checks and the `qir-artifact-cmp-1` comparison defined in `docs/qnir_commitments.md` §6.7, re-derives the manifest chain and the `Q(0)_H` root, and reports a typed receipt on any mismatch.
4. **No trusted `--trust-me` path.** An artifact that only the producing compiler can validate is not admissible; this follows the {% include term.html id="structural-indirection" %} discipline — the contract is the verification procedure, and the implementation is replaceable.

---

## 3. Mandatory Migration to a True Qn-PQC Dual-Ring Design

**The `WOTS+` v1 custody construction of §2 MUST be migrated as soon as possible.** It is provisional by construction: a hash-based one-time scheme with hand-rolled domain separation is an acceptable *bootstrap* custody mechanism, but it is not the target security architecture.

The migration trigger is the implementation of `cdqn` (`Q4`). When `Q4` lands, custody moves to a true Qn-PQC **Dual Ring** design as declared in `docs/abstractionLayers.md` §8.3:

- **Inner Ring.** The human root and co-signing operations — including the deterministic read-back / co-signing handshake — stay inside the Inner Ring; private lineage, root keys, and payload content never cross outward.
- **Outer Ring.** Node attestations, pseudonyms, and delegation proofs are carried by stateless PQC primitives and verify against the public root.
- **Structural indirection.** The custody contract is expressed as an abstract `(interface, proof requirement)` pair with a versioned PQC instantiation, so a primitive swap is a migration event rather than a lineage break.

The migration itself is **mandatory, not discretionary**, and is discharged by a signed **Migration Receipt** in the sense of `docs/simemp.md` §4.3, recording both the superseded and successor algorithm identifiers and the re-witnessing of prior commitments. Until that receipt verifies, §2 remains explicitly `[PROVISIONAL]` and must not be cited as the final custody design. See the {% include term.html id="dual-ring-pqc-boundary" %} for the boundary this migration must preserve.

---

## 4. Hard Dual-Ring Public-AI Isolation Rule

This is a **hard rule**, not a recommendation:

> Public AI coding agents (including `OpenCode`) and cloud chat sessions must **never** generate, read, hold, transmit, or reconstruct a real `Q(0)_H` private seed, leaf seed, or master-seed backup.

Operational form:

1. **Ephemeral dummy fixtures only.** Public agents may build, test, and debug custody verifiers exclusively against generated dummy fixtures — throwaway seeds created in-process for the test, never persisted, never the real root.
2. **No real material in prompts, repos, or CI.** A real seed is never pasted into a chat, committed, placed in an environment variable that a cloud agent can read, or stored in CI secrets consumed by an agent.
3. **Real signing stays offline and human.** Production `Q(0)_H` signing runs on the offline machine, under human control, and produces only the public root and the signed commitment — never the secret, and never a transcript that reveals it.
4. **Verification is safe to delegate.** Verifying a root, a chain, or a signature is a public operation and may be automated; *producing* a signature over real custody material may not.

This rule is the concrete application of the {% include term.html id="dual-ring-pqc-boundary" %} to the tooling layer: automation is confined to the Outer Ring and to public verification, while the Inner Ring human root stays out of machine reach.

---

## 5. Non-Goals and Status Register

| Item | Status |
|---|---|
| Any custody, signing, or `qnlangch` code | **NOT IMPLEMENTED** — deferred until the harness exists |
| Changes to `src/qnir/`, `src/qnir-py/`, `src/qnlangc/` | **NONE** in this wave |
| Changes to `docs/qnir_goldens.md`, `docs/qnlang.md` §6.4 | **NONE** |
| New glossary terms or navigation entries | **NONE** — every term used above is already registered |
| Promotion of `WOTS+` v1 to canon | **PROHIBITED** — `[PROVISIONAL]` pending §3 migration |
| Real `Q(0)_H` material exposed to public agents | **PROHIBITED** — §4 |

Discharge conditions for this candidate: (a) `qnlangch` exists with a pinned, stdlib-only, byte-reproducible build; (b) the §2 construction has an independent verifier and dummy-fixture test coverage; (c) the §3 migration receipt is specified against a real `Q4` capability set; (d) a BOC measurement selects the final construction. Until (a)–(d) hold, this file stays `State: CANDIDATE` under Axiom 14 and is not cited as sealed.

---

## License and Invariant Lineage

This document is part of the {% include term.html id="licensed-work" %} under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the QnLang program of record. Its lineage is the committed Qn universe at `e1ddc7a`, the state in which Stage 0 `qnlangc` and the `qir-enc-4` call runtime were last committed. It is a **new** candidate artifact under **Axiom 13/14**: it adds no implementation, changes no byte of `src/qnir/`, `src/qnir-py/`, `src/qnlangc/`, `docs/qnir*.md`, `docs/qnir_goldens.md`, the glossary, `docs/index.md`, or `docs/_data/navigation.yml`, and leaves `QnIR` and `QnLang` `[PENDING]`. Derivative works must preserve the canonical {% include term.html id="paternity-reference" %}; reuse of the open plaintext source remains governed by the {% include term.html id="open-core-invariant" %}.
