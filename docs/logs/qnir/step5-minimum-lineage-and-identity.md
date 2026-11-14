---
layout: default
title: Step 5, Share 1 — The Minimum Lineage and Identity Semantics (Wave Record)
description: The first Step-5 specification share — artifact identities, the lineage binding, and the versioned extractor/verbalizer identities declared in §6
nav_exclude: true
---

# Step 5, Share 1 — The Minimum Lineage and Identity Semantics (Wave Record)

Wave record of the first Step-5 specification share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.25. **The minimum lineage and identity semantics are declared in §6 of the new companion [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md).** No Intent, record, or receipt encoding changes; no transition rule changes; the 84 committed vectors are byte-unchanged.

## The selection

The review's own §5 order fixes this share first: "Only then should BOC select Step 5, beginning with the minimum lineage and identity semantics needed by the commitment design." Under the standing rubric-mode key `LexKey(c) = (ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)))` (π = Efficiency, Modularity, Portability, v0.3.0-draft.7's standing permutation) over Step 5's four deliverables: `ImplicitDeps` = 0 — the declaration rides the committed encodings of §2.6 and its hash policy, §4.1's protocol, and the logs' already-recorded practice (corpus bytes, digests, versions, rerun equality) — while the full-transition trace root requires a construction the draft has not declared and the collision bucket requires the §1.4 F6 resolution the review defers to Step 5's own later share; `ΔW_worst` = one new companion and the record/status updates, with no corpus, artifact, or encoding byte; `M_coupling` = the identity vocabulary is the minimum the two future shares must bind; `P_coverage` = two of Step 5's four deliverables receive their specification share.

## What the section declares

- **Artifact identity (§6.2).** `A = (kind, version, digest)` over the closed kinds `corpus`, `intent`, `record`, `receipt`, `transcript`, `implementation`; SHA-256 over each kind's declared byte basis (repository LF bytes for documents, canonical §2.6 bytes for encoded artifacts, the declared source-set concatenation for implementations); triple equality is the only identity relation — no path, time, locale, or platform participates, and a differing digest is never reconciled by prose.
- **Normative lineage (§6.3).** The binding `L = (corpus A, implementation A, V, E, n)`, with the evidence digest `E` over the ordered canonical record encodings followed by the canonical receipt encoding; the binding rule (outcome equality only between bindings of equal corpus identity and `V`, each implementation reproduced from its own execution under §4.1 — mutual agreement never the criterion; divergence recorded, never reconciled); lineage as external evidence metadata, never embedded in the frozen encodings; and the `QO-24` separation — denotational identity is the minimal-zoom normal form, provenance rides the binding and never the value key.
- **Verbalizer and extractor identities (§6.4).** `qir-extractor-1` (bytes to declared structure; deterministic, total on committed bytes, `extractor ∘ assembly = identity` on committed vectors — the suite's decode/listing-regeneration share is the evidence) and `qir-verbalizer-1` (bytes to the declared UTF-8/LF text form); the Obj 15 binding rule — a read-back is `(verbalizer identity, SHA-256 of the exact text)`, an extraction claim is `(extractor identity, artifact identity)`; and the generation rule — any output change on a committed byte requires a new generation identifier, never an in-place edit.
- **The Step-5 register (§6.5).** The two declared deliverables' specification shares are discharged, their artifact-share remainders fixed (each run's committed lineage block and `(generation, text digest)` pairs); the trace roots and the collision-safe comparison remain future, with the §6.3 evidence digest explicitly not the trace-root construction and F24's bucket deferral standing.

## Two declaration boundaries (recorded so they are not read as more than they are)

1. **The evidence digest is not the trace root.** `E` commits the run's evidence bytes at the lineage level; the Step-5 trace root is a construction over the complete transition trace whose format must be declared before implementation. No root format is implied, and the future share may adopt a construction that supersedes the evidence digest as its evidence commitment — that is a later declaration's decision, not this one's.
2. **Lineage is external evidence, not an encoding change.** Binding a run to its corpus, versions, and implementation is document-level metadata (the transcript's declared lineage block and the wave record), exactly the practice the artifact logs already follow. Nothing here adds a field to any envelope, record, or receipt, so no committed vector byte moves and no declared bound changes.

## Evidence and non-edits

No run is required or made: the wave changes no corpus, artifact, or specification rule. The bound evidence remains the v0.3.0-draft.23 runs — normal mode 84/84 both artifacts, `--fixture F27` PASS both with the byte-identical fixture block — and the corpus digest of record `fbefaddb17903c18` over 1,066,409 LF/repository bytes. Declared non-edits: the corpus, both implementations, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md` (its §4.5 acceptance text remains true — Step 5's full commitments still do not exist), `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.9 note records this share).

## Disposition

Step 5 remains **open**. Its register's two future rows (the trace-root construction and the collision-safe comparison) and the two declared deliverables' artifact-share remainders stand. The next BOC increment, selected under the same key: **the artifact share of the declared lineage and identity semantics** — both implementations emitting the lineage block and the generation-bound read-back pairs, and the logs citing them — which carries `ImplicitDeps` = 0 (the formats are declared), the smallest ΔW (transcript additions only), and completes the two specifications' discharge path; the trace-root declaration follows, then the collision bucket. No implementation, vector, or promotion is implied by opening Step 5's declaration share.
