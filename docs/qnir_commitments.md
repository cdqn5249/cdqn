---
layout: default
title: QnIR Commitments — The Commitment Specification of the QnIR v0.3 Working Draft (Repair Step 5)
description: QnIR v0.3 draft, §6 commitments — the minimum lineage and identity semantics (artifact identities, normative lineage, verbalizer and extractor identities)
version: 0.3.0-draft.26
updated: 2026-11-15
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_commitments.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_commitments.html
status: Working Draft under the Loop-2 Repair Sequence and the Loop-3 Repair Program (R1–R6) — Not Promoted to the Canon
terms_used:
  - receipt
  - computational-consistency
---

# QnIR Commitments — The Commitment Specification of the QnIR v0.3 Working Draft (Repair Step 5)

This file carries §6 of the QnIR v0.3 working draft, introduced at v0.3.0-draft.25 under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Unlike the three split companions (which reproduce committed monolith bytes), this companion is a **new section**: it has no monolith predecessor, and the §6 body below is its initial committed content.

---

## 6. Commitments (Repair Step 5)

This section discharges the specification share of the Step-5 row's first surface — the **minimum lineage and identity semantics needed by the commitment design** — on the byte-unchanged freeze of §1, the committed semantics of §2, the committed accounting of §3, and the accepted oracle of §4 (Step-4 acceptance reassessment, v0.3.0-draft.24). The review's own order fixes this share first: "Only then should BOC select Step 5, beginning with the minimum lineage and identity semantics needed by the commitment design."

The Step-5 row closes on four deliverables: full-transition trace roots, collision-safe artifact comparison, normative lineage, and versioned verbalizer/extractor identities. This section **declares** the minimum of the last two (the identity vocabulary and the lineage binding, §6.2–§6.4) and **registers** the first two as future shares with their obligation shape fixed and their answers explicitly absent (§6.5). The declaration is deliberately external-evidence metadata: it changes no Intent, record, or {% include term.html id="receipt" text="receipt" %} encoding of §2.6, no transition rule, and no vector byte — lineage rides the run's evidence record, never the frozen encodings — so the 84 committed vectors remain byte-unchanged and every declared bound is unchanged.

Why the minimum is this and not more: a commitment claim is only meaningful if the bytes it commits to have stable identities and the evidence it binds declares what produced it. Without the identity vocabulary, a read-back (Obj 15) has no stable object, and a cross-artifact comparison (F6) has no declared equality relation. Declaring the vocabulary now lets the later shares — the trace-root construction and the collision bucket — ride fixed definitions instead of inventing their own.

### 6.1 The commitment design's minimum

The commitment design needs three things that the earlier steps do not supply: (i) an **identity** for every artifact a claim refers to, comparable across runs and artifacts by canonical bytes alone; (ii) a **lineage binding** that states, for a run's evidence, which corpus, which versions, and which implementation produced it; and (iii) declared, versioned **transformation identities** at the two byte boundaries — extraction (bytes to structure) and verbalization (bytes to text) — so that a read-back or a re-extraction claim is a claim about a named generation. The three are declared below. The full-transition trace root (a single published commitment over the complete transition trace) and the collision-safe comparison of complete artifacts remain future shares; this section fixes only what those shares must bind, not their constructions. The artifact share of the two declared deliverables — the committed lineage block and the generation-bound read-back pairs in each run's transcript — is emitted and cited at v0.3.0-draft.26 (§6.5).

### 6.2 Artifact identities

An **artifact identity** is the triple `A = (kind, version, digest)`:

- **`kind`** is drawn from the closed vocabulary `corpus`, `intent`, `record`, `receipt`, `transcript`, `implementation`. Each kind names one byte basis:
  - `corpus` — a committed corpus document's repository bytes (the LF byte sequence the repository stores; its working-copy line endings are a checkout artifact and never participate);
  - `intent` — an Intent envelope's canonical CBOR bytes under §2.6;
  - `record` — one transition record's canonical fourteen-pair encoding under §2.6;
  - `receipt` — a terminal receipt's canonical encoding under §2.6;
  - `transcript` — a run transcript's declared byte form (UTF-8 text, LF line endings, the declared rendering of §6.4);
  - `implementation` — an implementation artifact's declared source set: the SHA-256 over the concatenation of its declared source files' repository bytes, in the declared path order (the artifact declares its own file set and order; the declaration rides its inventory).
- **`version`** is the artifact's declared version: the document's declared `version:` for repository documents; the producing Intent's version tuple `V` for `intent`, `record`, and `receipt`; the producing implementation's declared version for `transcript` and `implementation`.
- **`digest`** is the SHA-256 of the kind's byte basis, under §2.6's declared hash policy.

**Identity equality** is triple equality, and nothing else. No filename, path, modification time, locale, platform, or environment variable participates in an identity; two artifacts are the same artifact for every claim in this draft when their triples are equal, and different when any component differs. A differing digest is never reconciled by prose, by an "equivalent" label, or by re-running until the digests agree — the run that disagrees is a divergence and is recorded as one (§6.3).

### 6.3 Normative lineage

A **lineage binding** is the quintuple `L = (corpus A, implementation A, V, E, n)`:

- `corpus A` — the corpus artifact identity the run consumed;
- `implementation A` — the producing implementation's artifact identity;
- `V` — the run's declared version tuple (semantics, encoding, profile), as §2.1 and §2.6 declare it;
- `n` — the number of transition records in the evidence;
- `E` — the **evidence digest**: the SHA-256 over the concatenation, in record order, of the canonical record encodings, followed by the canonical {% include term.html id="receipt" text="receipt" %} encoding, each item self-delimiting under §2.6's canonical grammar.

The **binding rule**: a claim that a run reproduces a committed expected outcome is a claim about its lineage binding, and equality of outcomes is claimable only between bindings with equal `corpus A` and equal `V` — each implementation's evidence reproduced from its own execution under §4.1's protocol (mutual agreement never the criterion), with each implementation identity recorded. A binding with a different corpus identity or a different `V` is a different object of comparison; prose never bridges two lineages, and a re-derivation on new bytes is a new binding, not an amendment of the old one. Where two bindings with equal corpus identity and equal `V` disagree in `E`, the wave records a **divergence** in its adjudication register — the committed home for divergence — and no equality claim is made.

**Lineage is external evidence.** A lineage binding is recorded as the run transcript's declared header block and in the wave record that cites the run — the practice the artifact logs already follow (corpus path and bytes, digests, versions, rerun equality). It is never embedded in the frozen encodings: no envelope, record, or receipt byte changes, and no vector moves. The **evidence digest** is a lineage-level digest over the evidence bytes; it is **not** the Step-5 trace root, and no construction claimed here may be read as the trace-root format (§6.5 keeps that future).

**The `QO-24` separation.** Denotational value identity is the minimal-zoom normal form of §2.6's comparison rule; provenance — zoom expansion history, the declared dependency table `D`, profile parameters, and every other input that shaped a run — rides the lineage binding, never the value key. Any two lineages that denote the same value carry the same value key; the binding is the only committed home for provenance, and a conservativity audit queries the binding (the evidence under its corpus and `V`), not the value key.

### 6.4 Verbalizer and extractor identities

Two deterministic transformations name the byte boundaries, each with a declared generation identity:

- **Extractor `qir-extractor-1`** — artifact bytes to the declared structure: the §2.6 decode of values, instructions, envelopes, records, and receipts. Declared properties: deterministic; total on committed artifact bytes; and on every committed vector its output is the structure whose normative assembly re-encodes to the same bytes (`extractor ∘ assembly = identity` on committed artifacts). The conformance suite's decode and listing-regeneration share is the evidence of the round-trip on committed bytes (§4.4: every committed envelope decodes and every committed source listing regenerates).
- **Verbalizer `qir-verbalizer-1`** — artifact bytes to the declared text form: the declared rendering of the run logs (the listing lines, the record and receipt renderings, the declared UTF-8/LF byte form of §6.2's `transcript` kind). Declared properties: deterministic; total on committed artifact bytes; its output bytes declared rather than incidental.

**The binding rule (Obj 15).** A read-back is stable only as the pair `(verbalizer identity, SHA-256 of the exact text bytes)`: a text without its generation identity and digest is not a committed read-back, and two texts of different verbalizer generations are never compared as equal. Symmetrically, an extraction claim is the pair `(extractor identity, artifact identity)`. This is the minimum the signed-read-back constraint requires: the exact generation and the exact text are both committed, and neither is inferred from a filename or a default.

**The versioning rule.** The two identity strings are **generations**: any change to a transformation's declared output on any committed byte requires a new generation identifier, never an in-place edit, and generations are never compared as equal. The generation identifiers ride the lineage binding alongside `V`. The initial generations are `qir-extractor-1` and `qir-verbalizer-1`.

The saturation extension's own versioned extractor — the `QO-28` determinism tuple (schedule, `K_max`, `N_max`, extractor generation, canonical output identity, measured divergence) — is that candidate's Step-6 re-entry contract (§1.3's saturation row) and is not fixed, implied, or pre-empted here; this section names only the core's boundary identities.

### 6.5 Step-5 acceptance condition and non-discharge register

The Step-5 row's acceptance condition is the four deliverables named above. The register below fixes each deliverable's state, its discharge site, and — for the two future rows — the shape of what is still owed:

| Deliverable | State | Site and remainder |
|---|---|---|
| Normative lineage | **DISCHARGED** (specification v0.3.0-draft.25; artifact v0.3.0-draft.26) | §6.2/§6.3 declare identities and the binding rule; the artifact share is the declared five-line lineage block emitted after each artifact's `result:` line, in normal and fixture mode alike (v0.3.0-draft.26) — corpus identity, implementation identity, `V`, the evidence line `(n, E)`, and the generations line — cited by the wave records of [`docs/logs/qnir/step5-lineage-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-lineage-artifact-share.md) |
| Versioned verbalizer/extractor identities | **DISCHARGED** (specification v0.3.0-draft.25; artifact v0.3.0-draft.26) | §6.4 declares `qir-extractor-1` and `qir-verbalizer-1`, their properties, and the binding rule; the artifact share is each run's committed read-back pair `(qir-verbalizer-1, SHA-256 of the exact transcript text)` and the block's `generations` line naming both generation identities (v0.3.0-draft.26) |
| Full-transition trace roots | **FUTURE** (next Step-5 share) | The root's construction must be declared before implementation; the §6.3 evidence digest commits the same evidence bytes but is explicitly not the trace-root construction, and no root format is pre-empted here |
| Collision-safe artifact comparison (header-equal, payload-different) | **FUTURE** (§1.4's F6 row; the conformance register's last future row) | The bucket resolution fixes how two complete artifacts that share headers but differ in payload are compared and quarantined; the register row of §4.4 fixes the test obligation, not the answer (F24's deferral stands) |

The acceptance condition is met when all four are discharged and the artifact share of the two declared deliverables is emitted and cited. That artifact share is emitted and cited as of v0.3.0-draft.26; the two FUTURE rows below keep the condition unmet. What this section deliberately does not deliver: **no trace-root format**, **no collision bucket**, **no change to any §2.6 encoding** — and therefore no promotion: the promotion criterion is unchanged and unmet while the two FUTURE rows stand, `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional, and no claim of this section is styled a theorem. The Step-5 row remains open until its register empties.

Under {% include term.html id="computational-consistency" %}, the validity claimed for this section is internal consistency within declared boundaries: the identity vocabulary is consistent with §2.6's canonical encodings and hash policy, the lineage binding is consistent with §4.1's protocol and the logs' recorded practice, the transformation identities name only already-exercised boundaries, and nothing herein contradicts a canon file or changes a committed byte.

---

## License and Invariant Lineage

This companion is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the program of record. Its invariant lineage is the program of record's committed state at v0.3.0-draft.25: this file adds a new section (§6) whose declared semantics bind the committed §1–§4 and change no byte of them. The landing conditions are those of the draft: no promotion, no canon mutation, and the step's own acceptance register above is the sole discharge path.
