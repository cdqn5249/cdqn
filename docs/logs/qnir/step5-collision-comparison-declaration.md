---
layout: default
title: Step 5, Share 4 — The Collision-Safe Comparison Declaration (Wave Record)
description: The qir-artifact-cmp-1 procedure declared in full — complete-artifact comparison, the closed projection, the genuine-collision condition, quarantine semantics, the bucket register, the publication rendering, and four decision-rule pins
nav_exclude: true
---

# Step 5, Share 4 — The Collision-Safe Comparison Declaration (Wave Record)

Wave record of Step 5's fourth share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.29. **The collision-safe artifact comparison is declared in §6.7 of [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md)** — F6's obligation answered as a construction, before the committed test surface §4.4 owns. No code, transcript, or corpus byte changes; the 84 committed vectors are byte-unchanged.

## The selection

The draft.28 wave's recorded next increment is discharged: the collision comparison's declaration carries `ImplicitDeps` = 0 (the comparison unit — the §6.2 identity triple and the kind's canonical byte basis — and the equality vocabulary are already committed, and the declaration itself fixes every convention a later implementation would otherwise infer: the comparison surface, the verdicts, the collision condition, the quarantine meaning, the bucket shape, the rendering, the generation identity, and the pins), the smallest `ΔW_worst` of the admissible set (one companion subsection plus the record and status updates; no code, transcript, or corpus byte), the least coupling (the procedure rides the committed identities and encodings without touching any frozen byte), and it completes the last of Step 5's four deliverables' specification shares, leaving only the collision comparison's artifact share and test surface.

## What §6.7 declares

- **The comparison unit.** A **complete artifact** is the §6.2 identity triple — the declared **summary** — together with the artifact's complete canonical bytes — the artifact itself. Every committed artifact of the draft is retained in its complete form, so no comparison is ever forced to rest on a summary alone.
- **The closed projection.** The only comparison surface is the complete canonical byte string. Coarser projections — a tuple's arity or element tags, a record's leading state-local fields, an implementation's first header line — are explicitly not comparison surfaces, and no equality claim may be drawn from one. F6's finding is thereby fixed at the level it names: a shallow header comparison never decides equality.
- **The procedure (`qir-artifact-cmp-1`).** Total, deterministic, and symmetric up to the declared entry order: **`same`** (kind, version, and complete bytes agree — the equality claim permitted); **`different`** (kinds, versions, or bytes differ under differing summaries — no quarantine, no bucket entry); **`collision`** (identity triples agree while complete bytes differ — the **genuine collision**, a SHA-256 collision under §2.6's policy, both artifacts **quarantined**). Case 3 is the only case where a declared summary reports equality over differing payloads, and the bucket exists for exactly it: both complete artifacts are retained so the question stays decidable by bytes.
- **Quarantine semantics.** No equality claim between the members; neither may stand as *the* canonical artifact of its kind in place of the other; neither is discarded; the pair enters the **collision bucket register** and the divergence is recorded in the adjudication register (§6.3); an entry is resolved only by a recorded wave adjudication — never by deletion, prose, an "equivalent" label, or re-running.
- **The bucket register.** Ordered pairs — triples sorted ascending, each member with its complete-byte digest — with one admitted class at this generation, `digest-collision`; the register is empty over the committed corpus.
- **Publication rendering.** `lineage: collision bucket (qir-artifact-cmp-1, entries = <n>)` after the trace-root lines, one `lineage: collision bucket entry (qir-artifact-cmp-1, pair = (<A>, <B>), class = digest-collision)` line per entry; the transcript header carries `qir-artifact-cmp-1 known-answer pins: 4/4 reproduced (commitments §6.7)`.
- **Four decision-rule pins** over `(kind agrees, version agrees, complete bytes agree)`: `(equal, equal, equal)` → `same`; `(equal, equal, differ)` with equal supplied triples → `collision`; `(equal, equal, differ)` with differing triples → `different`; `(differ, —, differ)` → `different`. Construction examples, not corpus vectors; the collision row supplies equal triples because no SHA-256 collision is constructible and none is claimed.
- **The boundaries.** The procedure binds complete artifacts and nothing else — not a value relation (`QO-24` unchanged), not a lineage claim (§6.3 unchanged), and not a new encoding — and it strengthens §6.2's identity rule without contradicting it: identity equality remains triple equality; the comparison adds the equality-evidence rule over complete bytes. `qir-artifact-cmp-1` is the fourth generation under §6.4's versioning rule.

## Two declaration boundaries (recorded so they are not read as more than they are)

1. **Nothing is emitted or implemented.** No transcript line changes; the implementations still emit the draft.28 header, lineage, and trace-root lines. The artifact share — both implementations' comparison core, the pins line, the bucket state line, and the committed test surface — is the register's last remainder and is not pre-empted here.
2. **The test surface is still owed.** §4.4's register row fixes the test obligation (header-equal/payload-different comparison), and §6.7 answers the construction; a committed surface that exercises it cannot exist before this answer, and now can. It will be test-only artifacts (the F27 precedent), and a genuine collision is not constructible, so the collision verdict is pinned as a decision rule, not observed.

## Evidence and non-edits

No run is required or made: the wave changes no corpus, artifact, or specification rule. The bound evidence remains the draft.28 runs — normal 84/84 both artifacts (X execution 51/51), `--fixture F27` PASS both; evidence `E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72` with `n = 336`; 52 published trace lines per run; fixture block `ed67e6cbd242c69c`; corpus `fbefaddb17903c18` over 1,066,409 LF bytes. Declared non-edits: the corpus, both implementations, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md` (§4.4's test row still owns the test surface), `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.13 note records this share).

## Disposition

Step 5 remains **open**: the collision row now reads *specification share DISCHARGED (v0.3.0-draft.29); artifact share FUTURE*, and it is the register's only remaining row. The next BOC increment, selected under the same standing key: **the collision comparison's artifact share** — both implementations' comparison core, the pins line, the bucket state line, and the committed test surface §4.4 fixes (test-only header-equal/payload-different artifacts) — which empties the register and closes Step 5. No push is made until the maintainer approves.
