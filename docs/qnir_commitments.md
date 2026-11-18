---
layout: default
title: QnIR Commitments — The Commitment Specification of the QnIR v0.3 Working Draft (Repair Step 5)
description: QnIR v0.3 draft, §6 commitments — the minimum lineage and identity semantics (artifact identities, normative lineage, verbalizer and extractor identities)
version: 0.3.0-draft.31
updated: 2026-11-18
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

**Revision record (v0.3.0-draft.31 — Wave 1A specification alignment, 2026-11-18).** This additive amendment closes §6.7's reporting surface: the case-3 `collision` verdict is cross-referenced to the Terminal Outcome Lift Table `Φ_status` (`qnir_semantics.md` §2.5) and aligned with the unified outer `QUARANTINED` disposition tuple; it also points the test surface at the committed `F6` fixture (`qnir_conformance.md` §4.4). No comparison rule, class, register entry, or transcript byte changes; the disposition tuple is a projection of the procedure already declared here.

---

## 6. Commitments (Repair Step 5)

This section discharges the specification share of the Step-5 row's first surface — the **minimum lineage and identity semantics needed by the commitment design** — on the byte-unchanged freeze of §1, the committed semantics of §2, the committed accounting of §3, and the accepted oracle of §4 (Step-4 acceptance reassessment, v0.3.0-draft.24). The review's own order fixes this share first: "Only then should BOC select Step 5, beginning with the minimum lineage and identity semantics needed by the commitment design."

The Step-5 row closes on four deliverables: full-transition trace roots, collision-safe artifact comparison, normative lineage, and versioned verbalizer/extractor identities. This section **declares** the minimum of the last two (the identity vocabulary and the lineage binding, §6.2–§6.4), the **trace-root construction** in §6.6 (v0.3.0-draft.27, artifact share v0.3.0-draft.28), and the **collision-safe comparison** in §6.7 (v0.3.0-draft.29, artifact share v0.3.0-draft.30). The declaration is deliberately external-evidence metadata: it changes no Intent, record, or {% include term.html id="receipt" text="receipt" %} encoding of §2.6, no transition rule, and no vector byte — lineage rides the run's evidence record, never the frozen encodings — so the 84 committed vectors remain byte-unchanged and every declared bound is unchanged. Step 5 is closed: its register is empty.

Why the minimum is this and not more: a commitment claim is only meaningful if the bytes it commits to have stable identities and the evidence it binds declares what produced it. Without the identity vocabulary, a read-back (Obj 15) has no stable object, and a cross-artifact comparison (F6) has no declared equality relation. Declaring the vocabulary now lets the later shares — the trace-root construction and the collision bucket — ride fixed definitions instead of inventing their own.

### 6.1 The commitment design's minimum

The commitment design needs three things that the earlier steps do not supply: (i) an **identity** for every artifact a claim refers to, comparable across runs and artifacts by canonical bytes alone; (ii) a **lineage binding** that states, for a run's evidence, which corpus, which versions, and which implementation produced it; and (iii) declared, versioned **transformation identities** at the two byte boundaries — extraction (bytes to structure) and verbalization (bytes to text) — so that a read-back or a re-extraction claim is a claim about a named generation. The three are declared below. The full-transition trace root (a single published commitment over the complete transition trace) is declared in §6.6 (v0.3.0-draft.27), and the collision-safe comparison of complete artifacts in §6.7 (v0.3.0-draft.29). The artifact share of the two declared deliverables — the committed lineage block and the generation-bound read-back pairs in each run's transcript — is emitted and cited at v0.3.0-draft.26 (§6.5).

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

**Identity equality** is triple equality, and nothing else. No filename, path, modification time, locale, platform, or environment variable participates in an identity; two artifacts are the same artifact for every claim in this draft when their triples are equal, and different when any component differs. A differing digest is never reconciled by prose, by an "equivalent" label, or by re-running until the digests agree — the run that disagrees is a divergence and is recorded as one (§6.3). The identity relation is a claim about the declared **summary**; the comparison of complete artifacts — identities together with their complete canonical bytes — is the subject of §6.7, which adds the equality-evidence rule and the collision bucket without changing this identity relation.

### 6.3 Normative lineage

A **lineage binding** is the quintuple `L = (corpus A, implementation A, V, E, n)`:

- `corpus A` — the corpus artifact identity the run consumed;
- `implementation A` — the producing implementation's artifact identity;
- `V` — the run's declared version tuple (semantics, encoding, profile), as §2.1 and §2.6 declare it;
- `n` — the number of transition records in the evidence;
- `E` — the **evidence digest**: the SHA-256 over the concatenation, in record order, of the canonical record encodings, followed by the canonical {% include term.html id="receipt" text="receipt" %} encoding, each item self-delimiting under §2.6's canonical grammar.

The **binding rule**: a claim that a run reproduces a committed expected outcome is a claim about its lineage binding, and equality of outcomes is claimable only between bindings with equal `corpus A` and equal `V` — each implementation's evidence reproduced from its own execution under §4.1's protocol (mutual agreement never the criterion), with each implementation identity recorded. A binding with a different corpus identity or a different `V` is a different object of comparison; prose never bridges two lineages, and a re-derivation on new bytes is a new binding, not an amendment of the old one. Where two bindings with equal corpus identity and equal `V` disagree in `E`, the wave records a **divergence** in its adjudication register — the committed home for divergence — and no equality claim is made.

**Lineage is external evidence.** A lineage binding is recorded as the run transcript's declared header block and in the wave record that cites the run — the practice the artifact logs already follow (corpus path and bytes, digests, versions, rerun equality). It is never embedded in the frozen encodings: no envelope, record, or receipt byte changes, and no vector moves. The **evidence digest** is a lineage-level digest over the evidence bytes; it is **not** the trace root, and no construction claimed in this subsection may be read as the trace-root format — the root's own construction is declared in §6.6.

**Current-lineage binding note (v0.3.0-draft.31, Wave 1B).** This is a new binding, not an amendment of the draft.23–draft.30 bindings, which remain verbatim and are cited only as history. After the one-pass corpus migration, `corpus A` is the 1,080,404-byte LF form of `docs/qnir_goldens.md` with SHA-256 `fa72f316c2f3bccc94260bbe6f75abecfe18895ab22a551705301d60070c92b3`; `V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)`; `n = 336`; `E = 0765cd5e665ae77303b5bcfce21b3d7a686af8726a1e365d2687220dc97dee75`. The two implementation identities under the same corpus and `V` are Rust `(implementation, 0.3.0-draft.31, 7d204647280a49e03fed16a19fb552e3094b3672f9770d564b37e3c88796bf0c)` and Python `(implementation, 0.3.0-draft.31, f9cf76560da5a263b3f3690a724ff806ed0ab6091c2dd197e788e80f61a9cbff)`; they are artifact-specific by §6.2's triple equality and are not compared for equality with each other. The `scope = run` trace root is `5c6c0b21fba2df0bff219d01270848038046dcfb33eb509375684e9d57408a79`, with 52 published roots byte-identical across both artifacts; the migrated F27 block digest is `6f6bd8f5ea344b31` and the F6 block digest is `498609033d35cf03` (normal mode's `entries = 0` unchanged).

**The `QO-24` separation.** Denotational value identity is the minimal-zoom normal form of §2.6's comparison rule; provenance — zoom expansion history, the declared dependency table `D`, profile parameters, and every other input that shaped a run — rides the lineage binding, never the value key. Any two lineages that denote the same value carry the same value key; the binding is the only committed home for provenance, and a conservativity audit queries the binding (the evidence under its corpus and `V`), not the value key.

### 6.4 Verbalizer and extractor identities

Two deterministic transformations name the byte boundaries, each with a declared generation identity:

- **Extractor `qir-extractor-1`** — artifact bytes to the declared structure: the §2.6 decode of values, instructions, envelopes, records, and receipts. Declared properties: deterministic; total on committed artifact bytes; and on every committed vector its output is the structure whose normative assembly re-encodes to the same bytes (`extractor ∘ assembly = identity` on committed artifacts). The conformance suite's decode and listing-regeneration share is the evidence of the round-trip on committed bytes (§4.4: every committed envelope decodes and every committed source listing regenerates).
- **Verbalizer `qir-verbalizer-1`** — artifact bytes to the declared text form: the declared rendering of the run logs (the listing lines, the record and receipt renderings, the declared UTF-8/LF byte form of §6.2's `transcript` kind). Declared properties: deterministic; total on committed artifact bytes; its output bytes declared rather than incidental.

**The binding rule (Obj 15).** A read-back is stable only as the pair `(verbalizer identity, SHA-256 of the exact text bytes)`: a text without its generation identity and digest is not a committed read-back, and two texts of different verbalizer generations are never compared as equal. Symmetrically, an extraction claim is the pair `(extractor identity, artifact identity)`. This is the minimum the signed-read-back constraint requires: the exact generation and the exact text are both committed, and neither is inferred from a filename or a default.

**The versioning rule.** The two identity strings are **generations**: any change to a transformation's declared output on any committed byte requires a new generation identifier, never an in-place edit, and generations are never compared as equal. The generation identifiers ride the lineage binding alongside `V`. The initial generations are `qir-extractor-1` and `qir-verbalizer-1`; the trace-root construction declared in §6.6 is the third generation, `qir-trace-root-1`, and the comparison procedure declared in §6.7 is the fourth, `qir-artifact-cmp-1`, under this same rule.

The saturation extension's own versioned extractor — the `QO-28` determinism tuple (schedule, `K_max`, `N_max`, extractor generation, canonical output identity, measured divergence) — is that candidate's Step-6 re-entry contract (§1.3's saturation row) and is not fixed, implied, or pre-empted here; this section names only the core's boundary identities.

### 6.5 Step-5 acceptance condition and non-discharge register

The Step-5 row's acceptance condition is the four deliverables named above. The register below fixes each deliverable's state, its discharge site, and — for the future row — the shape of what is still owed:

| Deliverable | State | Site and remainder |
|---|---|---|
| Normative lineage | **DISCHARGED** (specification v0.3.0-draft.25; artifact v0.3.0-draft.26) | §6.2/§6.3 declare identities and the binding rule; the artifact share is the declared five-line lineage block emitted after each artifact's `result:` line, in normal and fixture mode alike (v0.3.0-draft.26) — corpus identity, implementation identity, `V`, the evidence line `(n, E)`, and the generations line — cited by the wave records of [`docs/logs/qnir/step5-lineage-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-lineage-artifact-share.md) |
| Versioned verbalizer/extractor identities | **DISCHARGED** (specification v0.3.0-draft.25; artifact v0.3.0-draft.26) | §6.4 declares `qir-extractor-1` and `qir-verbalizer-1`, their properties, and the binding rule; the artifact share is each run's committed read-back pair `(qir-verbalizer-1, SHA-256 of the exact transcript text)` and the block's `generations` line naming both generation identities (v0.3.0-draft.26) |
| Full-transition trace roots | **DISCHARGED** (specification v0.3.0-draft.27; artifact v0.3.0-draft.28) | §6.6 declares the construction in full — leaf and node domain separation, the declared carry-promotion rule, the empty-trace constant, the closed scope vocabulary, the publication rendering, the generation identity `qir-trace-root-1`, and its known-answer pins; the artifact share is both implementations computing and emitting the declared roots (the header pins line, the `run` scope, and each `vector` scope in document order — 52 published lines per run), cited by the wave record of [`docs/logs/qnir/step5-trace-root-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-trace-root-artifact-share.md) |
| Collision-safe artifact comparison (header-equal, payload-different) | **DISCHARGED** (specification v0.3.0-draft.29; artifact v0.3.0-draft.30) | §6.7 declares the procedure in full — the complete-artifact unit, the closed projection, the three verdicts, the genuine-collision condition, quarantine semantics, the bucket register shape, the publication rendering, the generation identity `qir-artifact-cmp-1`, and its four decision-rule pins; the artifact share is both implementations' comparison core, the pins line, the bucket state line, and the committed test surface (test-only header-equal/payload-different artifacts fed to the core in a self-test, the F27 precedent), cited by the wave record of [`docs/logs/qnir/step5-collision-comparison-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-collision-comparison-artifact-share.md) |

The acceptance condition is met when all four are discharged and the artifact share of the two declared deliverables is emitted and cited. That artifact share is emitted and cited as of v0.3.0-draft.26, the trace root's artifact share as of v0.3.0-draft.28, the collision row's specification share at v0.3.0-draft.29, and its artifact share at v0.3.0-draft.30. **All four deliverables are now discharged; the Step-5 register is empty and Step 5 is closed.** What this section deliberately does not deliver: **no promotion** — the promotion criterion is the program of record's own gate, unchanged and unmet by this section, and `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional on it; no claim of this section is styled a theorem. No §2.6 encoding changes.

### 6.6 Trace roots

The **full-transition trace root** — the Step-5 row's first deliverable — is a single published commitment over the complete transition trace, constructed by the generation identity **`qir-trace-root-1`** from the canonical record encodings that §6.3's evidence digest already accumulates. This subsection declares the construction in full, as the draft's ordering discipline requires (the format is declared before any implementation); it emits nothing, implements nothing, and changes no transcript byte.

**The trace.** A *trace* is a finite ordered sequence of transition records — exactly the records of an executed vector, or of a declared set of executed vectors in committed document order, each as its §2.6 canonical fourteen-pair encoding. Order and multiplicity are part of the object: the same record appearing twice contributes two leaves. Rejection transitions (records with zero charged work) are leaves like any other; nothing is filtered. The trace root's scope is declared with every published root; the X-family execution evidence — the same records §6.3's evidence digest accumulates — is the scope of record here, and the `F27` fixture's injected runs are never in any published trace (test-only, §4.4).

**The construction.** Every hash is SHA-256 under §2.6's declared policy, and the only domain prefixes of `qir-trace-root-1` are the single bytes `0x00`, `0x01`, and `0x02`:

1. **Leaf.** For a trace record with canonical encoding `r`, the leaf value is `L(r) = SHA-256(0x00 || r)`.
2. **Node.** For two level values `x` and `y` in that order, the node value is `N(x, y) = SHA-256(0x01 || x || y)`.
3. **Levels.** Level 0 is the sequence of leaf values in trace order. While a level holds more than one value, the next level is formed pairwise and left to right — `N(v[0], v[1]), N(v[2], v[3]), …` — and when the level holds an odd number of values, the final value is **promoted unchanged** to the next level (never duplicated, never hashed against itself).
4. **Root.** The root is the single value remaining. A one-record trace's root is its leaf value `L(r)`.
5. **Empty trace.** The root of the empty trace is the constant `SHA-256(0x02)`.

The three prefix bytes place leaf values, node values, and the empty-trace constant in disjoint domains, so a leaf never verifies as an internal node or as the empty root; and the promotion rule is declared rather than conventional, so no implementation infers the shape of an odd level. No new cryptographic assumption is introduced — the construction uses the SHA-256 that §2.6 already declares — and it is declared for **determinism and structural commitment**: this subsection claims no security proof and styles no claim a theorem.

**Known-answer pins.** The construction carries the following declared pins (toy byte strings: `A` = `0x41`, `B` = `0x42`, `C` = `0x43`, `D` = `0x44`, `E` = `0x45`). They are construction examples, not corpus vectors: they are not counted in the declared seed, carry no conformance status, and commit no record or receipt byte.

| Trace | Root |
|---|---|
| empty | `dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986` |
| `[A]` | `c00b4d3c929cb5cc316691ed4636f634576f2c9b2954767234c5274e9dde185d` |
| `[A,B]` | `ed692f01f7f6c46930d7ad8f9adad3f9f38b7379cf6a8d2f399a0ba1e914fe25` |
| `[A,B,C]` | `961d2e2be20f538ffdf56962a86d1bd165498f222684ee4c5e02c1e9f852adc5` |
| `[A,B,C,D]` | `5c8dc617d287a4297eb2bcb81b37644b5138e57ad461c657db152109e3fc9fca` |
| `[A,B,C,D,E]` | `2bdbcd79bf92b8dad52ba685f01bef520f0cda04468a13a6bd27bfbeebff64d7` |

The `[A,B,C]` row exercises a carry at level 1; the `[A,B,C,D,E]` row exercises a carry after a node pair — `N(A,B)` and `N(C,D)` combine at level 2, and `L(E)` is promoted to be their partner at the top level.

**Scopes and publication.** A root is meaningful only with its declared scope, and a published root is the triple `(qir-trace-root-1, scope, root)`. The admissible scopes are closed: `run` — the trace of every executed X-family vector in committed document order and then record order (the sequence §6.3's evidence digest accumulates, minus the per-vector receipt encodings); `vector <id>` — the trace of one executed X-family vector's records in record order. The publication rendering is fixed as the transcript line

```
lineage: trace root (qir-trace-root-1, scope = <run|vector id>, n = <records>, root = <64 lowercase hex>)
```

one line per published scope, published as external evidence alongside the §6.3 binding and never embedded in any frozen encoding. A root without its scope and its lineage binding is not a committed claim. Like the extractor and verbalizer identities, `qir-trace-root-1` is a generation under §6.4's versioning rule: any change to the construction's output on any declared trace requires a new generation identifier, never an in-place edit.

**What the root binds and does not bind.** The root binds the trace's records — their bytes, order, and multiplicity — structurally. It does **not** bind the terminal {% include term.html id="receipt" text="receipt" %} encodings, the corpus identity, the version tuple, the implementation identity, or the transcript text: receipts remain bound by §6.3's evidence digest and the transcript's declared receipt rendering, and the corpus, versions, and implementation remain bound by the lineage binding `L`. The evidence digest and the trace root are two commitments over the same record bytes at different structures — flat evidence versus ordered structural root — and neither supersedes the other; a wave cites both. The `QO-24` separation is unchanged: the root is provenance-side evidence, never a value key, and two lineages that denote the same value carry the same value key whatever their roots.

**Artifact share (emitted, v0.3.0-draft.28).** The declaration above emits nothing by itself; the artifact share is both implementations computing and emitting the declared roots, discharged at v0.3.0-draft.28. Each run of either artifact now prints the header line `qir-trace-root-1 known-answer pins: 6/6 reproduced (commitments §6.6)` — the six declared pins recomputed at every run, a run failure if any diverges — and, after the §6.3 lineage block, the publication line for the `run` scope followed by one line per executed `vector` scope in committed document order, exactly the rendering declared above (52 published lines per run over the 51 committed X vectors; the vector `n` values sum to the run's `n = 336`). The roots bind the trace's records only; the fixture's injected runs contribute nothing, and the 84 committed vectors remain byte-unchanged. The published roots are recorded as evidence by the wave record of [`docs/logs/qnir/step5-trace-root-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-trace-root-artifact-share.md), whose verification includes the pins and the cross-implementation agreement of every published line (recorded as evidence, never the criterion); the emission is a declared output addition whose transcript read-back pairs are new generation commitments per §6.4.

### 6.7 Collision-safe artifact comparison

The **collision-safe artifact comparison** — the Step-5 row's second deliverable — is the declared procedure by which two complete artifacts are compared and, when their declared summaries agree while their bytes do not, quarantined. F6's finding fixes the obligation: a shallow header comparison cannot detect different payloads under equal headers, so the draft must **compare full canonical encodings or retain a collision bucket of complete artifacts, and quarantine both artifacts on a genuine collision** (§1.4). This subsection declares the procedure in full, as the ordering discipline requires — the answer before the committed test surface; it emits nothing, implements nothing, and changes no transcript byte at v0.3.0-draft.29. It answers the construction; the conformance register's test row (§4.4) remains the owner of the test surface.

**The comparison unit.** A **complete artifact** is an artifact identity `A = (kind, version, digest)` (§6.2) together with the artifact's **complete canonical bytes** — the kind's §6.2 byte basis. The identity triple is the artifact's declared **summary**; the canonical bytes are the artifact itself. Every committed artifact of this draft — the corpus, each envelope, each record, each receipt, each transcript, each implementation's declared source set — is retained in its complete form, so no comparison in this draft is ever forced to rest on a summary alone.

**The declared projection is closed.** The only summary the draft declares is the identity triple, and the only comparison surface is the complete canonical byte string. Coarser projections — a tuple's arity or element tags, a record's leading state-local fields, an implementation's first header line — are explicitly **not** comparison surfaces: a comparison may never terminate on one, and no equality claim may be drawn from one. Any claim that two artifacts are equal is a claim about their complete canonical bytes.

**The procedure (`qir-artifact-cmp-1`).** For two complete artifacts `x` and `y`, the comparison is total, deterministic, and symmetric up to the declared entry order:

1. **`same`** — `kind`, `version`, and the complete canonical bytes all agree. The equality claim is permitted; the triple is necessarily equal too, the digest being a function of the bytes.
2. **`different`** — the kinds differ, or the versions differ, or the complete canonical bytes differ while the summaries differ. The artifacts are simply different; no quarantine and no bucket entry.
3. **`collision`** — the complete identity triples agree (so `kind`, `version`, and `digest` agree) while the complete canonical bytes differ. This is the **genuine collision**: a SHA-256 collision under §2.6's declared policy. Both artifacts are **quarantined**.

Because no projection coarser than the identity triple is ever consulted, case 3 is the only case in which a declared summary reports equality over differing payloads — and it is the one case the summary cannot decide. The bucket exists for exactly that case: it retains both complete artifacts so the question is decidable by bytes, and a comparison on complete bytes always settles it.

**The outer lift and the unified disposition (v0.3.0-draft.31).** `Φ_status` (`qnir_semantics.md` §2.5) lifts case 3 to the Axiom 6 Sealed 12-Status Registry as `QUARANTINED`; the commitment-side disposition is the single unified outer tuple **`<Status=QUARANTINED, Scope=SCOPE_COMMITMENT_COLLISION, Receipt=RECEIPT_DIGEST_COLLISION_QUARANTINED, ReEntry=PERMANENT_CRYPTO_HOLD>`**. The tuple is a reporting projection of the procedure above, not a new rule: `Scope` names the collision commitment, `Receipt` names the emitted quarantine receipt, and `ReEntry=PERMANENT_CRYPTO_HOLD` records that neither member is re-admitted as canonical without a recorded adjudication (§6.3). The `F6` fixture declared at `qnir_conformance.md` §4.4 exercises the tuple directly: its collision pair quarantines with `entries = 1`, class `digest-collision`, while its three non-collision pairs add no entry.

**Quarantine.** A quarantined artifact remains citable by its full triple and its complete bytes, but:

- no equality claim may be made between the two members, and neither may be cited as *the* canonical artifact of its kind in place of the other;
- neither member is discarded — both complete byte strings stay in the bucket;
- the pair enters the **collision bucket register** (below), and the divergence is recorded in the adjudication register (§6.3) — never reconciled by prose, by an "equivalent" label, or by re-running until the bytes agree;
- an entry is resolved only by a wave act that records its adjudication (for example, by declaring a defective encoding and a new generation); deleting an entry is not a resolution.

**The collision bucket register.** The bucket is a committed register of **ordered pairs** — the pair's triples sorted ascending, each member with the digest of its complete bytes — together with the entry's class. One class is admitted at this generation: `digest-collision` (the case-3 condition). With the committed corpus the register is empty: no two retained artifacts of this draft share an identity triple over differing bytes. The register's state is published with every run and cited by the wave record that adjudicates it.

**Publication rendering.** The bucket's state is an external-evidence line, printed after the trace-root publication lines:

```
lineage: collision bucket (qir-artifact-cmp-1, entries = <n>)
```

and, when `n > 0`, one line per entry in ascending pair order:

```
lineage: collision bucket entry (qir-artifact-cmp-1, pair = (<A>, <B>), class = digest-collision)
```

with each `<A>`/`<B>` rendered as `(kind, version, digest)`. The construction's pins are published in the transcript header: `qir-artifact-cmp-1 known-answer pins: 4/4 reproduced (commitments §6.7)`.

**Known-answer pins.** The procedure's decision core is pinned by a four-row table over `(kind agrees, version agrees, complete bytes agree)`; the collision row supplies equal triples, since an agreeing digest is what makes case 3 a collision at all — no SHA-256 collision is constructed, and none is claimed:

| `kind` | `version` | complete bytes | triple | verdict |
|---|---|---|---|---|
| equal | equal | equal | equal | `same` |
| equal | equal | differ | equal (supplied) | `collision` — quarantine both |
| equal | equal | differ | differ | `different` |
| differ | — | differ | differ | `different` |

The rows are construction examples, not corpus vectors: not counted in the declared seed, no conformance status, and no committed record or receipt byte. The collision row is a **declared rule check** over supplied triples, not an observed collision.

**What the comparison binds and does not bind.** The procedure binds complete artifacts — identities and bytes — and nothing else. It is not a value relation (`QO-24`'s separation is unchanged: value identity remains the minimal-zoom normal form), not a lineage claim (§6.3's binding rule is unchanged), and not a new encoding: it changes no Intent, record, or {% include term.html id="receipt" text="receipt" %} byte. It **strengthens** §6.2's identity rule without contradicting it — identity equality remains triple equality, and the comparison procedure adds that an equality claim over retained artifacts is a claim about their complete canonical bytes, which the digest already summarizes. Like the generations before it, `qir-artifact-cmp-1` is a **generation** under §6.4's versioning rule: any change to the procedure's verdicts on any declared pair requires a new generation identifier, never an in-place edit.

**Artifact share (emitted, v0.3.0-draft.30).** The declaration above emits nothing by itself; the artifact share is both implementations' comparison core, the pins line, the bucket state line, and the committed test surface, discharged at v0.3.0-draft.30. Each run of either artifact now prints the header line `qir-artifact-cmp-1 known-answer pins: 4/4 reproduced (commitments §6.7)` and `qir-artifact-cmp-1 self-test: 4/4 verdicts reproduced (commitments §6.7)` — the four declared pins recomputed at every run and the quarantine verdicts checked against four toy complete artifacts, a run failure if any diverges — and, after the trace-root publication lines, `lineage: collision bucket (qir-artifact-cmp-1, entries = 0)` over the retained artifacts of the committed corpus. The comparison core, the pins, the self-test, and the bucket state line are cited by the wave record of [`docs/logs/qnir/step5-collision-comparison-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step5-collision-comparison-artifact-share.md), whose verification includes the pins, the self-test, and the cross-implementation agreement of every published line (recorded as evidence, never the criterion); the emission is a declared output addition whose transcript read-back pairs are new generation commitments per §6.4. The 84 committed vectors remain byte-unchanged.

Under {% include term.html id="computational-consistency" %}, the validity claimed for this section is internal consistency within declared boundaries: the identity vocabulary is consistent with §2.6's canonical encodings and hash policy, the lineage binding is consistent with §4.1's protocol and the logs' recorded practice, the transformation identities name only already-exercised boundaries, the trace-root construction uses only the declared hash policy and the declared canonical record encodings, the comparison procedure uses only the declared identity triples and the complete canonical bytes, and nothing herein contradicts a canon file or changes a committed byte.

---

## License and Invariant Lineage

This companion is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the program of record. Its invariant lineage is the program of record's committed state at v0.3.0-draft.30: this file adds a new section (§6) whose declared semantics bind the committed §1–§4 and change no byte of them — §6.6's trace-root declaration adds a construction over the same canonical record bytes without touching any encoding, its artifact share (v0.3.0-draft.28) is emitted transcript evidence, never a frozen byte, §6.7's comparison declaration adds an evidence procedure over complete artifacts without touching any encoding or emitting a byte, and its artifact share (v0.3.0-draft.30) is emitted transcript evidence, never a frozen byte. Step 5 is closed: its register is empty. The landing conditions are those of the draft: no promotion, no canon mutation, and the step's own acceptance register above is the sole discharge path.
