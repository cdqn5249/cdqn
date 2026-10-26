---
layout: default
title: Qs — Quang Semantics (Constructive Relational Intent and Epistemic Grounding)
description: Canonical specification of the constructive semantic base domain, multi-axial Clifford language frames, epistemic modal spectrum, human-agent intent contracts, and AI alignment under SIMEMP constraints.
version: 1.3.0
updated: 2026-10-26
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qs.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qs.html
terms_used:
  - boc-policy
  - cae-dag
  - causal-arrow
  - cdqn
  - chronosa
  - computational-consistency
  - compute-unit-u
  - dcc-profile
  - dependencies-determinism
  - epistemic-anchorage
  - identity-class
  - intent-contract
  - layer-0
  - layer-1
  - licensed-work
  - metric-exhaustion
  - no-implicit-rule
  - open-core-invariant
  - paternity-reference
  - q-reuse
  - q0
  - q1
  - qm
  - qn
  - qn-rsi
  - qs
  - simemp-gateway
  - ssl
  - zoom-z
---

# Qs — Quang Semantics: Constructive Relational Intent and Epistemic Grounding

| Field | Specification |
|---|---|
| **Document Title** | Qs — Quang Semantics: Constructive Relational Intent and Epistemic Grounding |
| **Version** | 1.3.0 |
| **Last Updated** | 2026-10-26 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Domain Specification — Category D (Local-First Base Domain) |

---

## Review Integration Record (v1.3.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.1.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.1.md) — transcript of the external adversarial review (Google Deep Review, October 2026) of the condensed Qn universe specification and QnIR v0.1 candidate draft. This update integrates the parser-boundary findings of that review (reviewer open question `QO-11`, Segment 3 Point 2, together with the boundary-trust verdict finding on Objection 10): the epistemic ceiling for parser-derived candidates, the Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake, and the typed parser-boundary failure receipts. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place. New constructions and semantic refinements introduced below are tagged **[v1.3.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Parser-boundary modality ceiling**: a candidate DAG produced by a natural-language parser enters the pipeline typed no higher than `MODAL_HYPOTHESIS`; direct admission at `MODAL_EMPIRICAL` or `MODAL_CONSTRUCTIVE` is barred regardless of parser-attached tags | §4.4 (new) | QO-11 / Segment 3 Point 2 | CANDIDATE |
| **Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake** — deterministic verbalizer read-back of `DCC_INTENT_CONTRACT_v1` and $$W_{\mathrm{allowed}}$$ to the principal, Inner-Ring cryptographic co-signature required prior to `SEALED` | §6.1 (new) | QO-11 / Segment 3 Point 2 | CANDIDATE |
| **Typed parser-boundary failure receipts** — closed-catalog extension with `RECEIPT_INTENT_PARSE_AMBIGUOUS`, `RECEIPT_INTENT_MODALITY_MISMATCH`, `RECEIPT_INTENT_SCHEMA_MISMATCH`, `RECEIPT_ROUNDTRIP_CONFIRMATION_REJECTED` | §6.2 (new) | QO-11 / Segment 3 Point 2 | CANDIDATE |
| **BOC degenerate-tie discipline**: an argmax tie within the compliance threshold emits `RECEIPT_INTENT_PARSE_AMBIGUOUS` instead of silently sealing an arbitrary winner | §6 | QO-11 / Segment 3 Point 2 | CANDIDATE |
| **Logical typing of $r_{\mathrm{social}}$** — candidate deontic anchoring (contrary-to-duty obligation sub-DAG) recorded without amending §5.1 or §5.2 | §5.3 (new) | QO-11 / Segment 3 Point 2 | CANDIDATE |
| **Fixed normative axes vs. open-vocabulary lexical payload**: the ≤ 16 basis dimensions are fixed thematic axes; open-vocabulary identifiers reside as content-addressed commitments inside blade coefficients | §3.4 (new) | QO-11 / Segment 3 Point 2 (gap 3) | CANDIDATE |
| Pipeline metering note: every stage (parsing, measurement, selection, verbalization, co-signature verification) consumes finite $$U$$ under Axiom 6; billing attribution deferred to `simemp.md` | §6 | Segment 1 Point 4 (noted, deferred) | CANDIDATE |
| Editorial: "An ai model" → "An AI model" | §6 | — | editorial |
| Editorial: pre-existing 1-column overflow in the §5 Intent Contract figure (Epistemic-Scope row 81 cols vs 80-col borders); row re-padded to the uniform 80-col grid, field content preserved | §5 | — | editorial |

**Receipt-naming record (BOC).** Of the reviewer's proposed alternates, `RECEIPT_INTENT_PARSE_AMBIGUOUS` is adopted over `RECEIPT_SEMANTIC_AMBIGUITY_INDETERMINATE` (the `INTENT_PARSE` stem matches the boundary it polices and the `DCC_INTENT_CONTRACT` vocabulary of §5). The reviewer's pair `RECEIPT_INTENT_MODALITY_MISMATCH` / `RECEIPT_INTENT_SCHEMA_MISMATCH` is adopted as **two distinct receipts** rather than one, because they denote disjoint failure modes: an epistemic-scope violation (the §4 spectrum versus the contract's Epistemic Scope field) and a structural contract-validation failure, respectively. All four receipts are Qs-domain parser-boundary receipts; they are distinct from the Layer-2/3 circuit-breaker receipts registered in [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) v1.3.0 §5.9, which they neither modify nor extend.

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline, to be integrated in subsequent versioned updates): glossary registration of the four new receipt names (deferred under the standing no-dead-entries discipline of [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §8); `docs/simemp.md` — formal billing attribution of gateway-crossing validation work against the invoking principal's budget (Segment 1 Point 4), to be integrated at that file's next versioned update; `docs/abstractionLayers.md` — Inner-Ring co-signature mechanics (QO-13 queue item); QnIR v0.2 — the closed receipt catalog of the candidate VM must carry the four parser-boundary receipts, and design requirement R12 ($H(\text{Intent} \mid \text{Execution}) = 0$) holds **only after** the §6.1 handshake has supplied the human-anchored Intent object.

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing $\mathrm{Qs}$. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs. References are deliberately version-unpinned: each target document's front matter carries its own authoritative version, so this specification never duplicates drift-prone patch state.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [Complexity Degree Stratification]({{ '/glossary.html' | relative_url }}#complexity-degree) | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and optimization primitives | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG specification | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `docs/chronosa.md` | Emergent non-local causal intelligence and human empowerment | [chronosa.html]({{ '/chronosa.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Constitutional Grounding

In classical artificial intelligence, natural language processing relies on statistical correlation across continuous floating-point vectors ($\mathbb{R}^d$). Transformer models predict tokens based on statistical consensus rather than physical truth or causal provenance. Consequently, classical models suffer from hallucinations, lack causal accountability, and cannot distinguish between verified empirical facts and unanchored fiction.

In the {% include term.html id="qn" %} computational universe, **language, meaning, and intent are not ungrounded statistical approximations**. Under [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}), semantic meaning must be constructed from finite, verifiable primitives governed by {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

{% include term.html id="qs" text="Qs (Quang Semantics)" %} formalizes language, human-machine communication, and AI agent alignment as a **constructive, multi-axial relational domain**:
* **Language is not a black-box tensor:** It is a structured graph of discrete semantic blades and relational morphisms.
* **Meaning is not statistical similarity:** It is an explicit, verifiable {% include term.html id="intent-contract" text="Intent Contract" %} bound to human authority.
* **The Non-Rogue Invariant:** Machines and AI agents cannot originate unanchored intent; they operate strictly as delegated proxies bounded by human sovereignty and thermodynamic metric budgets.

Operational validity is verified empirically via {% include term.html id="computational-consistency" %}. No unmeasured semantic assumption may cross a {% include term.html id="simemp-gateway" %}.

```
          STATISTICAL MIMICRY (LLMs) vs. GOVERNED MEANING (Qs)
 Classical LLM / Transformer:
   Prompt ──► [ Unmetered Continuous Latents R^(d×d) ] ──► Probabilistic Token
   • Opaque weights (Billions of unverified parameters).
   • Meaning = Statistical similarity. Hallucinations unconstrained.
   • Violates No-Implicit Rule: H(Meaning | Reality) > 0.

 Qs Constructive Semantic Framework:
   Prompt ──► [ Candidate Parser ] ──► Structured Qexpr (CAE-DAG)
                                            │
                                            ▼ Evaluated across
                ┌───────────────────────────┼───────────────────────────┐
                ▼                           ▼                           ▼
         Qs (Semantics)              Qm (Mathematics)           Qphy (Physics)
    "What intent is bound?"     "Is it non-contradictory?"   "Can energy execute?"
                │                           │                           │
                └───────────────────────────┼───────────────────────────┘
                                            ▼
                   Sealed DCC Intent Contract + Signed Receipt
```

---

## 2. Vertical Abstraction Layer Projection of Qs

Mirroring the mathematical domain ([`docs/qm.md`]({{ '/qm.html' | relative_url }})), {% include term.html id="qs" %} does not reside in a single, isolated tier. It is an orthogonal vertical projection crossing the execution core ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §2.5):

```
                  VERTICAL PROJECTION OF Qs ACROSS THE EXECUTION CORE (L1–L3)
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ CD 3+ COMPOSITION DEPTH — beyond the execution ceiling, not a layer         │
 │ Multi-party contracts, modal epistemology (Fact vs. Fiction),               │
 │ human-agent sociology, Chronosa Outer-Ring intent synthesis.                │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 3 — EXECUTION CEILING: Symbolic Semantic Graphs (CD 2 / Qexpr CAE-DAG)│
 │ Content-Addressed Semantic DAGs: sentences, dialogues, and contexts         │
 │ structured without floating-point drift; agnostic to generating models.     │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 2: Governed Relational Morphisms (Complexity Degree 1)                │
 │ Atomic actions, verbs, capability delegations, and legal liability          │
 │ state transitions (preserving conserved social residuals r_social).         │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ LAYER 1: Ontological Genesis Context (Complexity Degree 0)                  │
 │ Foundational distinction between Empty Context Q(0) and Unit Intent Q(1).   │
 └─────────────────────────────────────────────────────────────────────────────┘
```

* **CD vs. Layer Note:** Complexity Degree (CD) indexes structural composition depth, not layer numbers. The LAYER 1–3 bands name the execution-core strata crossed by the Qs projection; their CD 0/1/2 labels measure composition depth accumulated at each stratum, and CD 3+ marks depth accumulated above the ceiling without implying a Layer 4.

### 2.1. Layer 1 — Contextual Genesis (Complexity Degree 0)
At {% include term.html id="layer-1" %}, $\mathrm{Qs}$ establishes the primitive origin of meaning:
* {% include term.html id="q0" text="Q(0)" %} acts as the **Empty Context** (the unwritten page, the silent void, zero unfulfilled obligation).
* {% include term.html id="q1" text="Q(1)" %} acts as the **Unit Intent Measure** along dimensional axis $d_1$, calibrating the baseline quantum of intentional focus and thermodynamic attention.

### 2.2. Layer 2 — Governed Relational Morphisms (Complexity Degree 1)
At Layer 2, $\mathrm{Qs}$ establishes atomic relational transformations:
* Verbs, capability delegations, and authorizations executed under explicit {% include term.html id="dcc-profile" text="DCC Profiles" %}.
* Every action partitions an obligation into an authorized transition and an exact **conserved social residual ($r_{\mathrm{social}}$)**, ensuring that unfulfilled liabilities are never silently discarded.

### 2.3. Layer 3 — Symbolic Composition Graphs (Complexity Degree 2)
At Layer 3, $\mathrm{Qs}$ structures compound propositions, dialogues, and multi-agent interaction histories as **Content-Addressed Expression DAGs ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }}))**:
* Identical semantic sub-trees and legal clauses share immutable cryptographic hashes via {% include term.html id="cae-dag" %} hash-consing.
* Incurs zero redundant bit allocation across the Memory Wall via {% include term.html id="q-reuse" text="Q(reuse)" %}.

### 2.4. CD 3+ Composition Depth — Epistemic Modality and Social Systems
Above the Layer 3 execution ceiling, $\mathrm{Qs}$ accumulates Complexity Degree 3+ composition depth — emergent social capability, not additional execution layers:
* The formal separation between physical facts and sovereign fiction via {% include term.html id="epistemic-anchorage" %}.
* Multi-party consensus synthesized by {% include term.html id="chronosa" %} over public road receipts.

---

## 3. Words and Grammar as Multi-Axial Clifford Blades

In $\mathrm{Qs}$, vocabulary and linguistic relationships are formalized through **Clifford Geometric Algebra ($\mathcal{C}\ell_{p,q}$)** over an orthogonal frame of semantic dimensions ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §2.1):

$$\mathcal{D}_{\mathrm{semantic}} = \{ d_{\mathrm{intent}}, \, d_{\mathrm{consent}}, \, d_{\mathrm{obligation}}, \, d_{\mathrm{temporal}}, \, d_{\mathrm{agent}}, \, \dots \}$$

```
                  LINGUISTIC CLUSTER AS CLIFFORD BLADES
 Noun Blade v_noun : Grade-1 vector along basis e_k (e.g., e_device)
 Modifier Rotor R  : Rational Cayley Rotor in plane e_i ∧ e_j (e.g., "secure")
 Transitive Verb M : Bivector contraction mapping Subject to Object
                     ⟨ v_subject ∧ M_verb,  v_object ⟩ + r_social
```

### 3.1. Nouns as Graded Vector Blades
An entity or concept ("battery", "drone", "contract") is a discrete vector blade evaluated at declared {% include term.html id="zoom-z" text="Zoom z" %}:

$$\vec{v}_{\mathrm{noun}} = \sum_{k=1}^n \langle \sigma_k, \, q_{z,k}, \, z, \, r_{z,k}, \, d_k \rangle \, \vec{e}_k$$

Nouns carry explicit identities, coordinate axes, and conserved residuals.

### 3.2. Adjectives and Modifiers as Rational Cayley Rotors
Modifiers ("cold", "verified", "urgent") do not mutate the fundamental type of a concept. A modifier is a **Rational Cayley Rotor** ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5.2) rotating the noun blade within semantic sub-spaces:

$$\vec{v}' = R \vec{v}_{\mathrm{noun}} R^{\dagger}, \quad \text{where } R = \frac{Q(1) - B}{Q(1) + B}$$

Because $R R^{\dagger} \equiv Q(1)$, **semantic modifications preserve unitary norm by construction**: meaning transforms with zero precision drift, zero semantic degradation, and zero floating-point rounding.

### 3.3. Verbs as Conserved Relational Morphisms
A verb is not an ambiguous text string. In $\mathrm{Qs}$, a transitive verb ("Human A delegates task T to Agent B with budget W") is a governed morphism evaluated through Diophantine conservation:

$$\text{Action} = \big( \text{Delegation} \times \text{Actor} \big) + r_{\mathrm{social}}$$

If the task fails, encounters a fault, or halts, the residual $r_{\mathrm{social}}$ remains sealed in the lineage. The system holds an immutable record of unfulfilled obligations, precluding silent state abandonment.

### 3.4. Fixed Normative Axes vs. Open-Vocabulary Lexical Payloads [v1.3.0 — CANDIDATE]

The dimensional ceiling on semantic frames (

$$\mathrm{Dim}_{\max} \le 16$$

, [`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5.2, since the blade count scales as $\mathcal{O}(2^n)$ ) is **not a vocabulary limit**.

The basis dimensions of 

$$\mathcal{D}_{\mathrm{semantic}}$$

are fixed **normative and thematic axes** ( $d_{\mathrm{intent}}$, $d_{\mathrm{consent}}$, $d_{\mathrm{obligation}}$, $d_{\mathrm{temporal}}$, $d_{\mathrm{agent}}$, … ), not per-word slots: allocating a distinct basis blade to every open-vocabulary noun would exhaust the ceiling after the first handful of terms, collapsing the frame algebra.

Open-vocabulary lexical identifiers ("battery", "drone", "contract", …) instead reside as **content-addressed commitments inside the blade coefficients**: a noun blade's coefficient tuples (§3.1) carry the $\mathcal{H}$-commitments of the entity's Layer-2 and numeric-leaf definitions, and two occurrences of the same term share one commitment via CAE-DAG hash-consing (§2.3). Adding a new word to the discourse therefore allocates coefficient data inside existing axes and never a new basis vector. Under the {% include term.html id="no-implicit-rule" %}, any application that genuinely requires an additional normative axis must declare it explicitly in its frame specification and accept the $\mathcal{O}(2^n)$ blade-count consequence, rather than silently overloading an existing axis.

---

## 4. The Epistemic Anchorage Spectrum: Anti-Hallucination Firewall

To eliminate AI hallucinations, $\mathrm{Qs}$ outlaws ungrounded assertions under the {% include term.html id="no-implicit-rule" %}. Every statement in $\mathrm{Qs}$ must declare its explicit **{% include term.html id="epistemic-anchorage" %}**:

```
                       THE EPISTEMIC ANCHORAGE SPECTRUM
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. MODAL_EMPIRICAL (Grounding G = 1.0)                                      │
 │    Anchored directly to Layer 0 physical sensor receipts (NIST entropy,     │
 │    thermocouple, camera, bus telemetry) via a SIMEMP Gateway.               │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. MODAL_CONSTRUCTIVE (Grounding G = 1.0)                                   │
 │    Algebraic closure proven via Qm / DEC; remainder r = Q(0) (Terminal      │
 │    Exactness receipt). Cannot drift or hallucinate by construction.         │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. MODAL_HYPOTHESIS (Grounding 0.3 < G < 1.0)                               │
 │    Explicitly bounded within the Recursive Search Loop (simemp.md §2).      │
 │    Declared as a working conjecture awaiting physical binary falsification. │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 4. MODAL_NARRATIVE (Contextually Sealed Creative Worldbuilding)             │
 │    Internally consistent narrative anchored to author identity Q(0)_author. │
 │    True WITHIN the declared context; barred from triggering actuators.      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 5. MODAL_UNANCHORED (Grounding G = ∅)                                       │
 │    Claims empirical or logical truth without parent lineage receipts.       │
 │    REJECTED AT THE GATEWAY! (RECEIPT_UNANCHORED_ASSERTION_REJECTED)         │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 4.1. The Lineage Verification Mandate
Under **Axiom 4** and **Axiom 5** ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }})), a statement cannot claim empirical truth through latent probability alone.  
If an artificial agent asserts: *"The motor temperature is 450°C"*, the gateway queries its lineage:
* If the lineage contains an unbroken causal chain terminating in a verified {% include term.html id="layer-0" %} hardware sensor receipt, it is admitted as `MODAL_EMPIRICAL`.
* If the assertion traces only to an internal neural probability distribution without sensor receipts, the gateway intercepts it:

$$\text{Gateway Action: } \mathrm{Reject}\Big(\mathcal{H}(\mathrm{Statement})\Big) \implies \text{Emits Receipt}$$

The gateway emits the rejection receipt `RECEIPT_UNANCHORED_ASSERTION_REJECTED`; the claim is physically barred from entering the execution pipeline.

### 4.2. Contextual Isolation for Creative Fiction
$\mathrm{Qs}$ does not prohibit human imagination, creative writing, or artistic worldbuilding:
* A creative work (such as a fantasy novel or game narrative) is typed as `MODAL_NARRATIVE` anchored to the author's sovereign identity $Q(0)_{\mathrm{author}}$ and publisher receipt.
* The internal consistency of the fictional universe is preserved within its closed `Qexpr` sub-DAG.
* The gateway guarantees **contextual containment**: a fictional sub-DAG is barred from ever triggering real-world physical actuators, financial registers, or legal delegations.

### 4.3. The Source-Receipt Invariant (Proposed, CANDIDATE)

**Status.** This subsection records a proposed extension of the §4 firewall from statements to governed artifacts, sourced from [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §3. It carries `State: CANDIDATE` under **Axiom 14**: it remains subject to deterministic measurement and {% include term.html id="boc-policy" text="BOC" %} selection through the pipeline of §6, and it amends nothing in §4.1 or §4.2, which continue to apply exactly as written.

It is proposed that every governed artifact of the {% include term.html id="qn" %} universe crossing a {% include term.html id="simemp-gateway" %} declare an explicit pair:

**(content modality, source class)**

where the content modality is drawn from the {% include term.html id="epistemic-anchorage" %} spectrum of §4, and the source class is a declared expression of where the artifact's asserted content ultimately anchors. Under the {% include term.html id="no-implicit-rule" %}, the declaration makes explicit — at the level of composite artifacts and worlds — what the gateway already enforces implicitly at the level of single statements.

```
        ARTIFACT-LEVEL SOURCE DECLARATION (PROPOSED, STATE: CANDIDATE)
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ Governed Artifact  :=  ( ContentModality , SourceClass )                    │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ ContentModality : one modality of the §4 spectrum, assigned per             │
 │                   atomic claim decomposition of the artifact                │
 │ SourceClass     : declared expression of where the asserted content         │
 │                   ultimately anchors (candidate vocabulary I / P / S,       │
 │                   docs/qs_history.md §2)                                    │
 │ SourceReceipts  : unbroken chain terminating in physically receipted        │
 │                   reality events; no dangling source references             │
 └─────────────────────────────────────────────────────────────────────────────┘
```

The invariant requires that the source receipt chain terminate in physically receipted reality events, with the terminating receipt type fixed by the declared modality:

* **Empirical content** terminates in verified {% include term.html id="layer-0" %} sensor or artifact receipts, exactly as mandated by §4.1.
* **Constructive content** terminates in algebraic closure receipts whose remainder $r = Q(0)$ (Terminal Exactness).
* **Contextually sealed creative content** terminates in the author-identity ($Q(0)_{\mathrm{author}}$) and publisher-receipt anchoring already mandated by §4.2.

The extension therefore applies the §4.1 statement-level rejection rule to composite artifacts and worlds: no unsourceable fiction, no unsourced anchor, no history-less constant. An artifact whose source receipt chain cannot be completed is rejected at the gateway exactly as an unanchored statement is, emitting the same class of rejection receipt.

Two cautions are recorded, and they are binding on any future implementation of the invariant:

1. **Payload sovereignty is untouched.** The invariant governs artifacts that cross the gateway as governed objects of the {% include term.html id="licensed-work" %}. Sovereign human [`Payloads`]({{ '/glossary.html' | relative_url }}#payload) remain uninspected behind the safe-harbor air gap ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.1). Where a creative work is declared as a governed `MODAL_NARRATIVE` artifact, its anchoring to author identity and publisher receipt is **declared by its author**, not extracted by inspection.
2. **Simulation modality is two-faced.** A simulation's execution is `MODAL_CONSTRUCTIVE` (deterministic algebraic closure), while its claims *about reality* are `MODAL_HYPOTHESIS` until physically falsified. The pair declaration makes this two-faced split explicit rather than leaving it implicit, in accordance with the {% include term.html id="no-implicit-rule" %}.

The proposed Anchor Grounding Classes of [`docs/qs_history.md`]({{ '/qs_history.html' | relative_url }}) §2 (Classes I / P / S) supply the candidate vocabulary for the source-class field. Glossary registration of the newly named concepts — and the final sealing of this invariant itself — remain deferred to the integration queue of that document (§8), preserving its discipline against dead glossary entries for unmeasured concepts.

### 4.4. The Parser-Boundary Modality Ceiling [v1.3.0 — CANDIDATE]

The §4 firewall inspects statements and artifacts that are already formalized. The remaining trust gap lies **upstream of formalization**: translating a natural-language utterance into a structured CAE-DAG and a `DCC_INTENT_CONTRACT_v1` relies on a statistical or neural **candidate parser** (§6, step 1) crossing the **Axiom 14** boundary. No statistical parser guarantees $H(\text{Intent}_{\mathrm{human}} \mid \text{Utterance}) = 0$ — {% include term.html id="dependencies-determinism" %} governs the formalized state, not the pre-formalization act of interpretation. If the parser hallucinates or inverts the human's actual intent while attaching a valid human origin signature $Q(0)_{\mathrm{author}}$ and well-formed `MODAL_EMPIRICAL` or `MODAL_CONSTRUCTIVE` tags, then every downstream check — the SIMEMP gateways, the Triadic Grounding of §7 ($\mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$), and the QnIR anti-drift invariant $H(\text{Intent} \mid \text{Execution}) = 0$ — will validate the *wrong* intent with perfect rigor, because the error occurred prior to formalization.

**Parser-boundary modality ceiling (rule).** Any candidate DAG produced by a natural-language parser enters the generative pipeline of §6 typed **no higher than `MODAL_HYPOTHESIS`**. Direct admission at `MODAL_EMPIRICAL` or `MODAL_CONSTRUCTIVE` is barred — including when the parser itself attaches those tags — until the candidate passes the Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake of §6.1.

* A parsed intent is, epistemically, a **hypothesis about what the principal meant**; `MODAL_HYPOTHESIS` is its correct home in the §4 spectrum — a working conjecture whose only competent falsifier is the principal's own confirmation. Parser confidence scores are not {% include term.html id="epistemic-anchorage" %}: under the {% include term.html id="no-implicit-rule" %} they confer no grounding value and must not be mapped onto $G$.
* The upgrade path is exactly one: §6.1, where the sole authority over the intent's meaning — the human principal — supplies the anchorage that the parser cannot.
* `MODAL_NARRATIVE` (§4.2) is unaffected: creative content remains author-declared and contextually contained. `MODAL_UNANCHORED` claims remain rejected at the gateway (§4, class 5), and the §4.3 artifact-level invariant continues to apply exactly as written.

---

## 5. Human-Agent Alignment: The Intent Contract in DCC Profiles

In the Qn universe, alignment between humans and artificial agents is not achieved by fine-tuning neural weights (RLHF). It is enforced through **Structural Intent Contracts** embedded inside the {% include term.html id="dcc-profile" %}:

```
                     ANATOMY OF AN INTENT CONTRACT
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ DCC_INTENT_CONTRACT_v1                                                      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ • Sovereign Anchor : Hash(pk_human_principal) [Human Identity Q(0)_H]       │
 │ • Agent Proxy      : Hash(pk_agent) [Identity Class: AI_AGENT_IDENTITY]     │
 │ • Permitted Domain : Multi-axial frame D_allowed (Spatial / Financial / Act)│
 │ • Metric Budget    : Max Work Allocation W_allowed in Q(1) units            │
│ • Epistemic Scope  : Allowed Modalities {MODAL_EMPIRICAL, MODAL_HYPOTHESIS}  │
 │ • Revocation Path  : Immediate un-delegation on human signature             │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 5.1. Subordination of Machine Agency
Under the {% include term.html id="identity-class" %} rules ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §8.1):
1. An AI agent possesses **zero independent root sovereignty**. It cannot instantiate an unanchored genesis origin $Q(0)_{\mathrm{agent}}$.
2. An agent executes strictly as an authorized proxy under an unbroken delegation chain originating from a human principal ({% include term.html id="q0" text="Q(0)_H" %}).
3. If the human principal revokes the capability token, the agent's execution authority terminates instantly.

### 5.2. The Thermodynamic Leash
Autonomous agents cannot self-replicate or consume unmetered resources. Under **Axiom 6** (Totality of Governed Operations), every agentic workflow is bounded by a finite work budget in units of {% include term.html id="compute-unit-u" text="U" %} via {% include term.html id="metric-exhaustion" %}. When the allocated budget reaches zero, the agent physically halts, emitting a signed termination receipt.

### 5.3. Logical Typing of the Social Residual (Proposed, CANDIDATE) [v1.3.0 — CANDIDATE]

**Status.** This subsection records a candidate logical anchoring for the conserved social residual $r_{\mathrm{social}}$ of §2.2 and §3.3. It carries `State: CANDIDATE` under **Axiom 14**: it remains subject to deterministic measurement and {% include term.html id="boc-policy" text="BOC" %} selection through the pipeline of §6, and it amends nothing in §5.1 or §5.2, which continue to apply exactly as written.

Sections §5.2 and §3.3 conserve $r_{\mathrm{social}}$ but leave its logical type unspecified: representing $\text{Action} = (\text{Delegation} \times \text{Actor}) + r_{\mathrm{social}}$ as a Diophantine conservation constraint does not by itself say how multi-clause legal or safety obligations, deadline expirations, or secondary reparations are encoded into the residual once a primary obligation is only partially violated. The reviewer proposal recorded here anchors the semantics of `DCC_INTENT_CONTRACT_v1` in **resource-bounded dyadic deontic logic with contrary-to-duty (CTD) obligations**, composed with **dynamic epistemic logic (DEL)** for multi-agent state updates.

Under this candidate reading:

* Primary obligations $\mathcal{O}_{\mathrm{actor}}(\alpha \mid W_{\mathrm{allowed}})$, permissions $P(\alpha)$, and prohibitions $F(\alpha)$ carry explicit work budgets drawn from the contract's Metric Budget field.
* When a primary obligation goes unfulfilled, a **reparational secondary obligation** $\mathcal{O}(\beta \mid \neg\alpha)$ takes effect by construction — the contrary-to-duty structure.
* This gives $r_{\mathrm{social}} \neq Q(0)$ a rigorous logical type: a **persistent CTD obligation sub-DAG** that remains in the lineage until discharged by a matching receipt. Multi-clause obligations, deadline expirations, and secondary reparations then encode as additional CTD structure on the residual instead of silently vanishing at partial violation, which is exactly the anti-abandonment property §3.3 mandates.

The full formal calculus — sequent presentation, discharge rules, and the interaction of CTD structure with the **Axiom 6** terminal states — is deferred to a dedicated versioned update. This subsection records the candidate anchoring only, so that downstream files do not hard-code an untyped reading of $r_{\mathrm{social}}$ in the interim.

---

## 6. The Generative Candidate Pipeline (Q(rsi) Integration)

$\mathrm{Qs}$ is completely paradigm-agnostic: it does not reject generative AI models (transformers, diffusion, active inference). It incorporates them into the **Recursive Search Loop** ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2, §6):

```
                     THE GENERATIVE Q(rsi) RECURSIVE LOOP
                      ┌───────────────────────────────┐
                      │ PRIOR STATE / CONTEXT S_k     │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ GENERATIVE EXPLORATION STEP   │
                      │ (Neural / Stochastic Engine)  │
                      │ Proposes Candidate C_1 ... C_n│
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ SIMEMP MEASUREMENT & AUDIT    │
                      │ • Check Depth & Node Ceilings │
                      │ • Verify Intent Constraints   │
                      │ • Measure Metric Burn (W)     │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ BOC SELECTION (simemp.md §7)  │
                      │ ArgMax( SIMEMP Compliance )   │
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │ SEALS SUCCESSOR ARTIFACT      │
                      │ State S_(k+1) via Q(rsi)      │
                      │ • Advances Causal Index κ     │
                      │ • Emits Verification Receipt  │
                      └───────────────┬───────────────┘
                                      │
                                      └───────► (Feeds next generation loop)
```

1. **Agnostic Generation:** An AI model (e.g., an LLM or multi-agent swarm) parses messy natural human speech and proposes a candidate semantic graph. Under **Axiom 14**, this graph is tagged as `State: CANDIDATE`; and as a parser-derived artifact it is admitted **typed no higher than `MODAL_HYPOTHESIS`** (§4.4). A failure at the parse boundary itself — ambiguity, modality conflict, or schema violation — emits a typed receipt (§6.2) rather than an untyped rejection.
2. **Deterministic Measurement:** The candidate is translated into a Content-Addressed Expression DAG ([`Qexpr`]({{ '/qexpr.html' | relative_url }})) and audited against the declared Intent Contract, checking depth ceilings, metric budgets, and geometric consistency.
3. **Selection via BOC Policy:** The candidate that maximizes compliance with SIMEMP constraints is selected under the {% include term.html id="boc-policy" %}:

$$\text{Selection} = \arg\max_{c \in \{C_1, \dots, C_n\}} \left( \text{SIMEMP Compliance}(c) \right)$$

If the argmax is **degenerate** — multiple non-isomorphic candidate DAGs fit the prompt within the BOC compliance threshold — the pipeline does not silently seal an arbitrary winner. It emits `RECEIPT_INTENT_PARSE_AMBIGUOUS` (§6.2), surfaces the ambiguity to the principal, and defers to the §6.1 handshake before any sealing decision. This is the intent-level analogue of the numeric indeterminacy discipline of [`docs/qm.md`]({{ '/qm.html' | relative_url }}): an unresolved tie is a measured outcome, not a hidden coin flip.

4. **Sealing via {% include term.html id="qn-rsi" text="Q(rsi)" %}:** The selected candidate is sealed into `State: SEALED` as successor artifact $S_{k+1}$, advancing the monotonic causal index along the {% include term.html id="causal-arrow" %}. For parser-derived candidates carrying a `DCC_INTENT_CONTRACT_v1`, sealing additionally requires that the §6.1 handshake has returned a principal co-signature; without it, the candidate remains `MODAL_HYPOTHESIS` and is never sealed. Sub-optimal candidates emit negative receipts, ensuring the system never repeats failed paths.

**Metering.** Every stage of the pipeline — parsing, deterministic measurement, BOC selection, verbalization, and co-signature verification — consumes finite compute and memory work and is metered under **Axiom 6**. Generation behind the stochastic boundary remains free; the attribution of gateway-crossing validation work to the invoking principal's remaining budget is a [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) concern and is deferred to that document's next versioned update.

### 6.1. The Deterministic Canonical Round-Trip Paraphrase & Co-Signing Handshake [v1.3.0 — CANDIDATE]

The deterministic pipeline of steps 1–4 closes every gap *inside* the formalized universe, but it cannot close the one gap that arises *before* formalization: whether the formal object matches what the human actually meant (§4.4). The handshake closes that gap by returning the formal object to its author for confirmation, before the universe treats it as intent.

**Protocol.**

1. **Verbalization.** A **deterministic verbalizer** $\mathcal{V}$ renders the formal `DCC_INTENT_CONTRACT_v1` — including its Permitted Domain, Metric Budget $W_{\mathrm{allowed}}$, and Epistemic Scope — back into an unambiguous **controlled natural language template** $\mathcal{T}$, the *canonical read-back*. $\mathcal{V}$ is a pure function of the formal contract, $H(\mathcal{T} \mid \text{Contract}) = 0$: no neural or stochastic component participates in the read-back path, per the {% include term.html id="no-implicit-rule" %}.
2. **Presentation.** $\mathcal{T}$ is presented to the human principal $Q(0)_{\mathrm{H}}$ inside the **Inner Ring** ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §3.3), alongside the candidate DAG it was verbalized from.
3. **Co-signature.** The principal either co-signs the pair $(\text{Contract}, \mathcal{T})$ cryptographically, or rejects it. Silence and absence of signature are not consent: without a co-signature the candidate remains `MODAL_HYPOTHESIS` and is **never sealed** (§6, step 4).
4. **Outcome.** On co-signature, the human-anchored intent is established: the co-signed contract becomes the parent lineage receipt that §4.1 chains previously lacked, downstream statements are typed per §4 exactly as any governed statement, and sealing proceeds under Q(rsi). On rejection, `RECEIPT_ROUNDTRIP_CONFIRMATION_REJECTED` (§6.2) is emitted, the candidate is discarded, and the negative receipt is sealed in the lineage so the failed parse is never silently retried.

**Why a round trip.** The direction of the check matters. Re-asking the parser "did I understand you right?" would only measure the parser's agreement with itself. Rendering the *formal* contract back into language and requiring the principal's signature instead tests the one thing the parser cannot self-certify: that the formalization and the intent coincide. The handshake is mandatory for parser-derived candidates carrying a `DCC_INTENT_CONTRACT_v1`; candidates formalized directly by the principal, with no parser step, are unaffected.

### 6.2. Typed Parser-Boundary Failure Receipts [v1.3.0 — CANDIDATE]

The receipt catalog of the Qn universe is closed but extensible by versioned updates. This update extends it with four Qs-domain parser-boundary receipts (naming record in the Review Integration Record):

| Receipt | Trigger | Disposition |
|---|---|---|
| `RECEIPT_INTENT_PARSE_AMBIGUOUS` | Multiple non-isomorphic candidate DAGs fit the prompt within the BOC compliance threshold (§6, step 3) — the intent-level analogue of `ORDERING_INDETERMINATE_AT_ZOOM` ([`docs/qm.md`]({{ '/qm.html' | relative_url }})) | No seal. The ambiguity is surfaced to the principal; the §6.1 handshake runs on the disambiguated set, or a fresh utterance is requested |
| `RECEIPT_INTENT_MODALITY_MISMATCH` | The candidate's declared epistemic modality conflicts with the Epistemic Scope of `DCC_INTENT_CONTRACT_v1` or with the §4.4 ceiling | The candidate is rejected at the boundary and re-typed no higher than `MODAL_HYPOTHESIS`, or re-proposed |
| `RECEIPT_INTENT_SCHEMA_MISMATCH` | The candidate fails structural validation against the `DCC_INTENT_CONTRACT_v1` schema (missing or ill-typed contract fields) | The candidate is rejected at the boundary; no contractual obligations attach to it |
| `RECEIPT_ROUNDTRIP_CONFIRMATION_REJECTED` | The principal rejects the canonical read-back in §6.1 | The candidate is discarded; a negative receipt is sealed in the lineage, and the failed parse path is recorded |

These receipts make parser failures first-class, auditable lineage events. Under the **Non-Rogue Invariant**, an agent that repeatedly fails at the parse boundary is visible in the very lineage it emits; and the negative receipts of §6, step 4 gain the typed granularity needed to distinguish "the parser was ambiguous" from "the principal said no" — two failure modes that demand different remediation, and that an untyped rejection would conflate.

---

## 7. Triadic Intent Synthesis (Qs ⊗ Qm ⊗ Qphy)

True communication between humans and computational tools requires simultaneous verification across the foundational triad:

$$\mathbf{C}_{\text{Intent}} = \mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$$

```
                         THE TRIADIC GROUNDING OF A COMMAND
 Proposed Command: "Accelerate the drone to 120 km/h across sector 4"
                                       │
            ┌──────────────────────────┼──────────────────────────┐
            ▼                          ▼                          ▼
     Qs (Semantics)              Qm (Mathematics)           Qphy (Physics)
 ┌──────────────────────┐   ┌──────────────────────┐   ┌──────────────────────┐
 │ Human Intent Contract│   │ Formal Decidability  │   │ Thermodynamic Work   │
 │ • Is User Q(0)_H     │   │ Is rotor trajectory  │   │ Does battery have    │
 │   authorized to fly? │   │   algebraically exact│   │   sufficient Q(1)    │
 │ • DCC capability     │   │   without division by│   │   metric budget?     │
 │   token verified.    │   │   zero?              │   │ • Landauer heat safe.│
 └──────────┬───────────┘   └───────────┬──────────┘   └───────────┬──────────┘
            └──────────────────────────┼──────────────────────────┘
                                       ▼
                       GATEWAY DETERMINISTIC CONSENSUS
      [ Valid: Executes Action ]  OR  [ Invalid: Emits Rejection Receipt ]
```

When an intent command crosses a gateway:
1. **{% include term.html id="qs" %} validates intent:** Verifies the human authority chain, the declared modal grounding, and the contractual goal.
2. **{% include term.html id="qm" %} validates decidability:** Proves that the spatial rotors and coordinate trajectories contain no singularities, division-by-zero, or precision drift.
3. **$\mathrm{Qphy}$ validates realizability:** Proves that the command fits within the physical battery energy and Landauer dissipation budgets ($\Delta S \le \mathrm{Budget}(Q(1))$).

If all three domains verify the transition, the command executes. If any domain fails, execution halts cleanly. The tool cannot decouple from the human because the mathematical definition of an executable action is structurally bound to human intent — and, since v1.3.0, the intent object itself is anchored at its source: it is either formalized directly by the principal, or parser-derived, held at `MODAL_HYPOTHESIS` (§4.4), and confirmed through the §6.1 co-signature before any triadic verification can execute it.

---

## 8. License and Invariant Lineage

This domain specification enforces the legal and operational conditions of the {% include term.html id="ssl" %} governing the {% include term.html id="licensed-work" %} across the {% include term.html id="cdqn" %} network:

1. **{% include term.html id="paternity-reference" %}:** All semantic parsing engines, intent contract compilers, and human-agent alignment modules derived from this document must embed:
   > Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn
2. **Anti-Patent Defense:** Royalty-free rights terminate automatically if a licensee initiates patent litigation regarding $\mathrm{Qs}$ semantic methods, intent contracts, or alignment mechanisms against the Author or community under the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}.
3. **Statutory Safe Harbor:** $\mathrm{Qs}$ operates as passive, neutral semantic infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign human creative works, natural speech, or external [`Payloads`]({{ '/glossary.html' | relative_url }}#payload).
