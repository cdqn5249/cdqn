---
layout: default
title: Step 5, Share 2 — The Trace-Root Declaration (Wave Record)
description: The full-transition trace-root construction qir-trace-root-1 declared in full — domain separation, carry promotion, empty constant, closed scopes, publication rendering, and known-answer pins
nav_exclude: true
---

# Step 5, Share 2 — The Trace-Root Declaration (Wave Record)

Wave record of Step 5's second share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.27. **The full-transition trace-root construction is declared in §6.6 of [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md)** — the format before any implementation, per the draft's ordering discipline. No code, transcript, or corpus byte changes; the 84 committed vectors are byte-unchanged.

## The selection

The draft.26 wave's recorded next increment is discharged: the trace-root declaration carries `ImplicitDeps` = 0 — the construction consumes only the committed §2.6 canonical record encodings (the same bytes the evidence digest accumulates) and the already-declared SHA-256 policy, and the declaration itself fixes every convention a later implementation would otherwise have to infer (domain bytes, the carry rule, the empty-trace constant, the scope vocabulary, the publication rendering, the generation identity) — the smallest `ΔW_worst` of the admissible set (one companion subsection plus the record and status updates; no code, transcript, or corpus byte), the least coupling (the root rides alongside the §6.3 binding without touching any frozen encoding), and it is the third of Step 5's four deliverables' specification shares, leaving the artifact share of the root and the collision bucket.

## What §6.6 declares

- **The trace.** A finite ordered sequence of transition records, each as its §2.6 canonical fourteen-pair encoding — the records of one executed vector, or of a declared set of executed vectors in committed document order. Order and multiplicity are part of the object; rejection transitions are leaves like any other; the `F27` fixture's injected runs are never in any published trace.
- **The construction (`qir-trace-root-1`).** Leaf `L(r) = SHA-256(0x00 || r)`; node `N(x, y) = SHA-256(0x01 || x || y)`; levels formed pairwise and left to right, with an odd level's final value **promoted unchanged** (never duplicated, never hashed against itself); the root is the single remaining value; a one-record trace's root is its leaf value; the empty trace's root is the declared constant `SHA-256(0x02)`. The three single-byte prefixes place leaves, nodes, and the empty constant in disjoint domains. No new cryptographic assumption is introduced, and no security proof is claimed.
- **Known-answer pins.** Six declared pins — empty, `[A]`, `[A,B]`, `[A,B,C]`, `[A,B,C,D]`, `[A,B,C,D,E]` over the toy byte strings `A`–`E` — with the `[A,B,C]` row exercising one level-1 carry and the `[A,B,C,D,E]` row exercising a carry after a node pair. These are construction examples, not corpus vectors: not counted in the declared seed, no conformance status, no committed record or receipt byte. The pins are the hand-derived known answers the future implementations must reproduce.
- **Closed scopes and publication.** A published root is the triple `(qir-trace-root-1, scope, root)`; the admissible scopes are closed to `run` (the X-family execution sequence the evidence digest accumulates, minus the per-vector receipts) and `vector <id>`. The rendering is fixed: `lineage: trace root (qir-trace-root-1, scope = <run|vector id>, n = <records>, root = <64 lowercase hex>)`, one line per published scope, as external evidence — a root without its scope and its lineage binding is not a committed claim.
- **Boundaries.** The root binds the trace's records structurally and **not** the terminal receipts, the corpus identity, `V`, the implementation identity, or the transcript text; receipts remain bound by the §6.3 evidence digest and the transcript's declared receipt rendering. `E` and the root are two commitments over the same record bytes at different structures — flat evidence versus ordered structural root — neither superseding the other; a wave cites both. The `QO-24` separation is unchanged (the root is provenance-side evidence, never a value key), and `qir-trace-root-1` is a third generation under §6.4's versioning rule.

## Two declaration boundaries (recorded so they are not read as more than they are)

1. **Nothing is emitted or implemented.** The declaration changes no transcript line; the implementations still emit the draft.26 five-line block and nothing more. The artifact share — both implementations computing and emitting the declared roots, verified against the pins and by cross-implementation agreement, and cited by the logs — is the register's next remainder and is not pre-empted here.
2. **The construction is not a conformance rule.** The trace root is evidence metadata: it adds no transition rule, no encoding, and no vector. The 84 committed vectors and every declared bound are unchanged, and a run that disagrees in its root between implementations is a recorded divergence (§6.3's discipline), never a corpus edit.

## Evidence and non-edits

No run is required or made: the wave changes no corpus, artifact, or specification rule. The bound evidence remains the draft.26 runs — normal 84/84 both artifacts, `--fixture F27` PASS both, evidence `E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72` with `n = 336`, fixture block `ed67e6cbd242c69c`, corpus `fbefaddb17903c18` over 1,066,409 LF bytes. Declared non-edits: the corpus, both implementations, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.11 note records this share).

## Disposition

Step 5 remains **open**: the trace-root row now reads *specification share DISCHARGED (v0.3.0-draft.27); artifact share FUTURE*, and the collision-safe comparison remains FUTURE (F24's bucket deferral stands). The next BOC increment, selected under the same standing key: **the trace-root artifact share** — both implementations computing and emitting the declared roots for the `run` scope and each `vector` scope, with the known-answer pins checked and the published roots cited in the wave record — then the collision bucket, the register's last future row. No push is made until the maintainer approves.
