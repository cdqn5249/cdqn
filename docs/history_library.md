---
layout: default
title: Qs History Library — Vision and Operational Proposal for Post-Seal History Ingestion
description: Operational proposal for the Qs History Library — historical anchor artifacts, Calendar Projection Contracts, and the five-gate Anti-Poisoning Ingestion Firewall — as the concrete shape of post-seal consensus-history ingestion. All artifacts herein are propositions pending deterministic measurement and BOC selection.
version: 0.1.0
updated: 2026-10-22
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/history_library.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /history_library.html
status: Vision and Operational Proposal — Not a Finalized Specification
terms_used:
  - boc-policy
  - calendar-projection-contract
  - causal-arrow
  - cdqn
  - chronosa
  - computational-consistency
  - dcc-profile
  - dual-ring-pqc-boundary
  - epistemic-anchorage
  - grounding-class
  - historical-anchor-artifact
  - historical-calibration-benchmark
  - historical-horizon
  - ingestion-gate
  - intent-contract
  - layer-0
  - licensed-work
  - lifecycle-state
  - local-first
  - metric-envelope
  - no-implicit-rule
  - open-core-invariant
  - paternity-reference
  - payload
  - q-datastruc
  - qm
  - qn
  - qn-rsi
  - receipt
  - simemp
  - simemp-gateway
  - source-class
  - source-receipt-invariant
  - ssl
  - structural-indirection
  - terminal-exactness
  - zoom-z
---

# Qs History Library — Vision and Operational Proposal for Post-Seal History Ingestion

| Field | Specification |
|---|---|
| **Document Title** | Qs History Library — Vision and Operational Proposal for Post-Seal History Ingestion |
| **Version** | 0.1.0 |
| **Last Updated** | 2026-10-22 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Vision and Operational Proposal — Not a Finalized Specification |

---

## Purpose and Epistemic Stance

This document is the operational continuation of [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}). Where that document preserved the reasoning chain of the October 2026 design consultation on historical anchoring, this document consolidates its second and third propositions — the {% include term.html id="historical-anchor-artifact" text="Historical Anchor Artifact" %}, the {% include term.html id="calendar-projection-contract" text="Calendar Projection Contract" %}, and the five-gate ingestion firewall — into a single capability design: the **Qs History Library**, the proposed mechanism by which consensus past events are loaded into a local node **after** sealing. The first proposition, the {% include term.html id="source-receipt-invariant" text="Source-Receipt Invariant" %}, is not restated here; it has already been elaborated at the Qs layer as [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.3, and this document assumes it as its grounding discipline.

Every design element below carries the explicit status of a **candidate artifact** under **Axiom 14** ([`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6): it remains subject to deterministic measurement against the declared {% include term.html id="simemp" %} constraints, to selection under the {% include term.html id="boc-policy" %}, and to eventual sealing through a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop at implementation time. Nothing in this document redefines, amends, or overrides any existing canonical specification. The register is deliberately positioned one step beyond the consultation record: the operational semantics proposed in Section 5 (gate ordering, declared quorum floors, challenge lifecycle) are concrete enough to be measured, yet remain proposals throughout. Where this document names a threshold, the threshold is a **declared value inside a contract**, never a runtime negotiation — in accordance with the {% include term.html id="no-implicit-rule" %}.

In accordance with {% include term.html id="computational-consistency" %}, the validity claimed herein is internal consistency within declared finite boundaries. The proposed vocabulary used throughout — grounding classes, source classes, ingestion gates, historical anchor artifacts, calendar projection contracts — was registered in the glossary on 2026-10-22 as CANDIDATE terms citing [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §2–6; this document is the first to reference that vocabulary through the glossary concordance.

---

## 1. From Consultation to Capability

Three changes in the committed state of the repository separate this document from its parent proposal. First, the {% include term.html id="source-receipt-invariant" %} moved from consultation record to canonical-adjacent elaboration: [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.3 now defines the (content modality, source class) declaration pair at the artifact level, extending the statement-level rejection rule of §4.1. Second, the glossary now carries the six CANDIDATE terms that the consultation had deliberately left unregistered — the deferral recorded in [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §8.5 was a deferral *to the integration phase*, and that phase is now executing. Third, the open item queued as "ingestion pipeline codification" ([`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §8, item 2) is addressed here: Section 5 proposes operational semantics for the five gates whose existence was argued there.

The scope of this document is therefore fourfold: the library as a collection (Section 2), the unit record it holds (Section 3), the situation layer that gives past events calendar coordinates (Section 4), and the ingestion discipline that decides what may enter (Section 5). What this document does **not** do is equally important: it prescribes no runtime implementation, assigns no storage layout, and modifies no canonical file. It proposes the contract surface of a capability so that a future implementation — or a {% include term.html id="qn-rsi" text="Q(rsi)" %} loop — can measure it deterministically against the {% include term.html id="simemp" %} constraint system and select among recorded alternatives rather than reconstructing them from conversation history.

---

## 2. The Qs History Library

It is proposed that the {% include term.html id="qn" %} universe maintain a **Qs History Library**: a canonical, versioned collection of modality-typed historical anchor artifacts, each recording a past human event, its sources, and its spatiotemporal situation. The library is the concrete shape of the consensus-history ingestion capability identified as an open gap during the consultation (recorded internally as GAP2). It complements a {% include term.html id="cdqn" %} runtime or QnLang compiler the way a reference discipline complements a language faculty: the machine does not remember the past — it **cites** it through receipted lineage, which is the only form of historical knowledge the Qn universe admits.

Access is proposed in four tiers, each honoring {% include term.html id="local-first" %} and budget discipline:

1. **The canonical library** holds the versioned artifacts together with their full source receipt chains. It is the reference copy against which local holdings are verified; it is consulted, never trusted — every ingestion still passes the gates of Section 5.
2. **Local nodes** hold anchor contracts and bounded working sets within declared {% include term.html id="metric-envelope" %} budgets — never the full archive. A node may operate air-gapped with zero history: the library loads post-seal, on demand, exactly when an {% include term.html id="intent-contract" %} requires a past event to be situated or attributed. This preserves the Axiom 4 requirement that every local artifact trace to its local genesis origin: loaded records are new {% include term.html id="receipt" text="receipted" %} acquisitions along the local {% include term.html id="causal-arrow" %}, not pre-existing ancestors.
3. **AI agents** inherit epistemic scope through the intent contract's whitelist. Without a loaded historical anchor, an agent's historical assertion has no parent lineage receipts and falls to the gateway as an unanchored claim ([`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.1); with the library, the agent cites the artifact instead of generating from latent weights. The library is thus an anti-hallucination membrane for the past, complementing the anchorage spectrum's membrane for the present.
4. **{% include term.html id="chronosa" text="Chronosa" %}**, barred from every Inner Ring, sees only the public layer: local historical records glue where they agree on shared landmarks — calendar coordinates, attribution anchors, artifact commitments — satisfying the sheaf gluing condition ([`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §3.1). The consensus history is then the global section of this history sheaf; disagreement does not vanish but remains visible as recorded challenge (Section 5).

---

## 3. The Historical Anchor Artifact

The unit record of the library is the {% include term.html id="historical-anchor-artifact" text="Historical Anchor Artifact" %}. Its proposed anatomy is reproduced verbatim from the consultation record ([`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §5), with section references adjusted to this document:

```
 ┌───────────────────────────────────────────────────────────────────────┐
 │ HISTORICAL ANCHOR ARTIFACT (proposed anatomy, State: CANDIDATE)       │
 ├───────────────────────────────────────────────────────────────────────┤
 │ ArtifactType      : HIST_ANCHOR (proposed class)                      │
 │ ContentModality   : per claim decomposition (docs/qs.md §4 spectrum)  │
 │ SourceClass       : I, P, or S (proposed grounding class)             │
 │ SourceReceipts    : unbroken chain to Layer 0 artifacts or sealed     │
 │                     library records                                   │
 │ CalendarSituation : projected via a declared Calendar Projection      │
 │                     Contract (Section 4)                              │
 │ SpatialSituation  : local Qm frame coordinates                        │
 │ Witnesses         : lineage-disjoint root subtree list (Section 5)    │
 │ LifecycleState    : CANDIDATE, SEALED, or QUARANTINED                 │
 └───────────────────────────────────────────────────────────────────────┘
```

The field semantics proposed here are operational, and each is a contract declaration rather than an inferred property. **ContentModality** is not a single value but a per-claim decomposition: a past event typically carries empirical claims (the artifact exists, the print survives), hypotheses (the attribution), and sealed narrative (the anecdote), and each claim is typed at the highest modality its receipts support along the {% include term.html id="epistemic-anchorage" %} spectrum. **SourceClass** declares where the artifact's asserted content ultimately anchors, taking one of the three values of the proposed {% include term.html id="grounding-class" text="grounding classes" %} — I (invariant), P (physical), S (social) — under the pair discipline of the {% include term.html id="source-receipt-invariant" %} with the {% include term.html id="source-class" text="source class" %} as its second element. **SourceReceipts** must terminate in exactly the three receipt classes that [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.3 admits: {% include term.html id="layer-0" text="Layer 0" %} sensor or artifact receipts for empirical content, algebraic closure receipts for constructive content, and author-identity plus publisher receipts for contextually sealed creative content.

**CalendarSituation** and **SpatialSituation** give the event its coordinates without importing wall-clock time: the calendar coordinate is valid only relative to a declared {% include term.html id="calendar-projection-contract" text="Calendar Projection Contract" %} (Section 4), and the spatial coordinate is expressed in local {% include term.html id="qm" %} frame terms. **Witnesses** lists the lineage-disjoint root subtrees backing the record — the counting rule that makes citation laundering causally impossible ([`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §1.4). **LifecycleState** reuses the existing lifecycle vocabulary: records enter as CANDIDATE, are sealed to SEALED through the candidate pipeline of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6, and may be placed in QUARANTINED with a challenge receipt under the disagreement handling of Section 5, consistent with the {% include term.html id="lifecycle-state" %} discipline of [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.6.

**Worked example — the constant e.** The invariant face needs no history: the transcendental constant exists as a nullary continued-fraction generator under `OP_CF` ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}) §3.2, §4.3), evaluated on demand at declared {% include term.html id="zoom-z" text="zoom z" %} and budget, halting early under {% include term.html id="terminal-exactness" %}, with intermediate collapses cacheable as content-addressed {% include term.html id="q-datastruc" text="Q(dataStruc)" %} artifacts — a Class I grounding re-derivable by every node. The discovery face is historical: a historical anchor artifact for the treatise that established the constant's notation would carry the print→holding→digitization provenance chain (each transfer physically receiptable at Layer 0), its calendar situation via the Gregorian projection contract, its spatial situation in local Qm frames, and its attribution witnesses as a lineage-disjoint list. The machine does not remember the mathematician; it cites him.

---

## 4. Calendar Projection Contracts

The canon admits exactly one time: the local {% include term.html id="causal-arrow" %}. Distributed state ordering relies on causal chaining "rather than external wall-clock synchronization" ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }})), and common knowledge across asynchronous nodes is established through acyclic causal message chains, not shared clocks ([`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §3.2). Human calendars are therefore proposed not as parallel time systems but as **projections**: deterministic mappings between calendar coordinates and intervals of the local causal index, hosted at the Qs layer in the same manner that the canon's domains project orthogonally across the execution core ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }})).

A {% include term.html id="calendar-projection-contract" text="Calendar Projection Contract" %} declares three components:

| Component | Proposed class | Nature |
|---|---|---|
| Rule (leap algorithms, month arithmetic, week conventions) | Class I | Deterministic, re-derivable algorithm; constructive once the epoch is fixed |
| Epoch (coordinate zero: Gregorian year 1, Hijra 622 CE, Unix epoch, J2000.0, and others) | Class S | A human convention-adoption event, multi-source by nature |
| Resolution envelope (day, second, leap-second tolerances) | Declared field | A declared bound inside the contract, never negotiated at runtime |

The operational placement proposed here is a **canonical library of versioned, DCC-contracted calendar artifacts**, loaded post-seal and on demand within the {% include term.html id="metric-envelope" %}, honoring {% include term.html id="local-first" %} budgets — the candidate that survived the consultation's own {% include term.html id="boc-policy" text="BOC" %} assessment of placement ([`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §4). Calendar contracts declare their components inside a {% include term.html id="dcc-profile" %}, consistent with {% include term.html id="structural-indirection" %}: calendars are swappable cultural conventions, not runtime constants, and a node may hold several contracts simultaneously — Gregorian for contemporary citation, a historical contract for archival situation — provided each is versioned and declared. Under the {% include term.html id="no-implicit-rule" %}, a calendar coordinate without a declared contract has no meaning and falls at the {% include term.html id="simemp-gateway" %} exactly as an unanchored assertion does.

Under this anatomy, the Gregorian calendar is a Class S genesis that matured into Class I operation: born from a contested, multi-jurisdiction convention process spread across centuries — its epoch a genuine social event, witnessed by many — yet today re-derivable deterministically by any node from its published rule. This is the witness ladder of the {% include term.html id="grounding-class" text="grounding classes" %} observed in a single historical object, and it is why calendars belong in the library rather than in the runtime: their epistemic genesis is social, even where their present operation is invariant-grade. Leap-second handling is declared inside the resolution envelope, so that a citation like "1748" resolves deterministically to a causal-index interval on every node that loads the same contract version — the calendar analogue of bit-identical evaluation.

---

## 5. The Anti-Poisoning Ingestion Firewall

The library can be alimented only by petition through {% include term.html id="ingestion-gate" text="ingestion gates" %}. The motivating threat is citation laundering: machine-generated statistical pages laundered into news outlets, peer-reviewed publications, and financial reports through iterative citation. In the Qn universe such an attack is assessed as failing at birth — a fabricated statistic claiming empirical truth without lineage receipts is an unanchored assertion rejected at the first gateway crossing ([`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4), and its arrival into financial registers is barred by the contextual containment of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.2. What remains to defend is subtler: **slow poisoning of the historical record itself** — plausible, partially receipted records whose defensible content masks an undefensible core. Citation laundering is assessed as causally impossible under the proposed counting rule: sources are counted as lineage-disjoint root subtrees in the polyroot forest ([`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §1.4), never by citation count; laundering adds descendants to a poisoned root and cannot manufacture independent origins.

It is proposed that library ingestion pass **five gates**, each a governed operation emitting its own {% include term.html id="receipt" %}. The gates were argued in the consultation record ([`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §6); the operational semantics below are this document's codification, and each semantic is a CANDIDATE proposal pending measurement:

| Gate | Canon root | Proposed operational semantics (CANDIDATE) |
|---|---|---|
| 1. Lineage | Axioms 4 and 5; causal hash chains | The record's receipt chain is verified against Layer 0 artifacts or already-sealed library records; verification is deterministic hash work. Failure emits a rejection receipt naming the gate and the first broken link. |
| 2. Modality Ceiling | {% include term.html id="epistemic-anchorage" %} spectrum | Each atomic claim is typed at the highest modality its receipts support; mixed-modality records must decompose into per-claim sub-records. Undecomposable mixed claims fail the gate. |
| 3. Independence Quorum | Multi-source genesis; threshold quorum | Anchor-grade records require **k** sources with lineage-disjoint roots. The floor proposed here is k ≥ 2 for anchor-grade records; higher grades declare higher k **inside the anchor's DCC contract** — never runtime-negotiated, per the No-Implicit Rule. |
| 4. Sheaf Gluing | Shared-landmark gluing ([`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §3.1) | A record disagreeing with receipted overlapping records on shared landmarks (calendar coordinates, attribution anchors, artifact commitments) enters a QUARANTINED {% include term.html id="lifecycle-state" text="lifecycle state" %} with a challenge receipt, remaining visible as a recorded challenge. |
| 5. Append-Only Immutability | SEALED lifecycle; {% include term.html id="structural-indirection" %} | Sealed artifacts never mutate; corrections arrive as counter-receipts and migration receipts. History is never rewritten; it is re-witnessed. |

**Gate ordering** is proposed as 1 → 2 → 3 → 4 → 5, ordered by evidentiary cost: deterministic hash verification first (cheap, local, decisive for most poisoning attempts), then modality typing (local computation), then quorum (economic — it requires other nodes' receipts), then consensus gluing (social — it requires the sheaf), and immutable commit last. A record rejected at an early gate never reaches the expensive ones, so the firewall's own {% include term.html id="metric-envelope" %} consumption is bounded by the ordering itself. **Challenge lifecycle** is proposed as follows: a QUARANTINED record is resolved only by a new receipt chain that re-enters the pipeline at Gate 1 — quarantine is a state, not a verdict, and no record is ever deleted; it is either eventually glued, or remains visible as a standing disagreement. This preserves the append-only discipline at every stage of the pipeline.

Residual threats are recorded honestly. Collusion across genuinely distinct roots remains an open challenge, in direct continuity with the hub-collusion resistance challenge of [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §8.5; its mitigation is economic and physical, since fabricating Layer 0 artifact provenance costs real-world work, and forging contemporary sensor receipts requires breaking post-quantum signatures across the {% include term.html id="dual-ring-pqc-boundary" %}, whose private root keys never leave the Inner Ring ([`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2.4). Slow poisoning of deep history is capped by the {% include term.html id="historical-horizon" %}: admitted poison can only ever occupy hypothesis or firewalled narrative modalities, with bounded grounding and no path to actuation. Finally, because every gate is itself a governed operation, the firewall as a whole is a legitimate candidate for optimization by a future {% include term.html id="qn-rsi" text="Q(rsi)" %} loop under strict semantic invariance — a library that can be fed is a liability; a library that can only be petitioned through receipts is an instrument.

---

## 6. Relation to Existing Canon

The following table records, for each element of this proposal, whether it is already grounded in canon (inheriting its grounding from the consultation record, [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §7) or genuinely new to this document. No element herein redefines existing canon.

| Element | Status | Canon grounding |
|---|---|---|
| Causal Arrow as sole time; calendars as projections | Grounded | [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}); [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §3.2 |
| Source-Receipt Invariant; (content modality, source class) pair | Grounded | [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §4.3 (committed); [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §3 |
| Historical anchor artifact anatomy | Grounded | [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §5 (reproduced verbatim in Section 3) |
| Calendar contract anatomy: rule / epoch / envelope | Grounded | [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §4 (reproduced in Section 4) |
| Five-gate firewall composition | Grounded | [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §6 |
| Sheaf gluing on shared landmarks; global section | Grounded | [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §3.1 |
| Polyroot forest; lineage-disjoint independence | Grounded | [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §1.4 |
| QUARANTINED lifecycle and challenge receipts | Grounded | [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.6 |
| Q(rsi) candidate pipeline and sealing | Grounded | [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 |
| **Gate ordering 1→2→3→4→5 with bounded-cost rationale** | **New** | Composes the canon mechanisms under one pipeline |
| **Declared quorum floors (k ≥ 2 anchor-grade floor, in-contract)** | **New** | Consistent with metric-envelope ceiling discipline; No-Implicit Rule |
| **Challenge lifecycle: re-entry at Gate 1, no deletion** | **New** | Operationalizes append-only re-witnessing |
| **Multi-contract calendar holding (versioned, swappable)** | **New** | Operationalizes structural indirection for calendars |

---

## 7. Open Questions and Integration Queue

All items below remain in CANDIDATE status. Sealing of any item occurs exclusively through the candidate pipeline of [`docs/qs.md`]({{ '/qs.html' | relative_url }}) §6 — deterministic measurement, {% include term.html id="boc-policy" text="BOC" %} selection, {% include term.html id="qn-rsi" text="Q(rsi)" %} sealing — at implementation time or under a dedicated optimization loop.

1. **Pipeline measurement.** The five-gate pipeline of Section 5 awaits deterministic measurement against the {% include term.html id="simemp" %} constraint system at implementation time; gate ordering and bounded-cost rationale are proposals until measured.
2. **Quorum thresholds.** The anchor-grade floor k ≥ 2 and the per-grade ladder above it are declared-value proposals; their concrete values belong to the DCC contracts of the first implemented anchor grades and remain open until then.
3. **Cross-root collusion.** The open challenge of [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }}) §8.5 applies unchanged; economic and physical mitigations are recorded in Section 5, but no closed-form defense is claimed.
4. **Library indexing.** Retrieval over the library — content-addressed lookup keyed by artifact commitments and lineage tags, in the manner of cached {% include term.html id="q-datastruc" text="Q(dataStruc)" %} derivations — is proposed as the natural index discipline but is not yet codified.
5. **Onboarding calibration.** The mandatory {% include term.html id="historical-calibration-benchmark" %} is defined in the glossary but is not yet wired into the six-stage onboarding state machine of [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2; the calibration-stage proposition remains queued as integration item 3 and is untouched by this document.

---

## 8. License Alignment

This proposal is part of the {% include term.html id="licensed-work" %} and aligns with the {% include term.html id="ssl" text="Scaling Source License (SSL) 1.0" %}:

- **Attribution:** Any implementation or research derivative referencing this document must preserve the canonical {% include term.html id="paternity-reference" %}.
- **Open-Core Research:** Exploratory research and academic derivations of the propositions herein remain royalty-free under Non-Scaling Open Access, governed by the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}.
- **Safe Harbor Integrity:** The Qs History Library is designed to operate as technical infrastructure of the licensed work. It exercises no editorial control over, and acquires no intellectual property in, sovereign human {% include term.html id="payload" text="Payloads" %}.
