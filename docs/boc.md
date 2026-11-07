---
layout: default
title: BOC Decision, Comparison, and Receipt Specification
description: BOC decision outcomes, comparison rules, and immutable auditable receipt requirements
version: 0.4.0
updated: 2026-11-07
author: "Christophe Duy Quang Nguyen via GPT-6 (OpenAI)"
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/boc.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /boc.html
terms_used:
  - boc-policy
  - dcc-profile
  - simemp-gateway
  - receipt
  - dependencies-determinism
  - lineage
---

# BOC: Decision, Comparison, and Receipt Specification

| Field | Specification |
|---|---|
| **Document Title** | BOC: Decision, Comparison, and Receipt Specification |
| **Version** | 0.4.0 |
| **Last Updated** | 2026-11-07 |
| **Author** | Christophe Duy Quang Nguyen via GPT-6 (OpenAI) |
| **Status** | Provisional implementation specification for the Qn-universe implementation; subject to replacement or binding by equivalent Qn artefacts |
| **Parent Policy** | `docs/simemp.md` §7 |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Purpose and scope

The Best of Choices (BOC) policy defined in `docs/simemp.md` §7 uses two stages:

1. Tier 1 is a hard admission gate.
2. Tier 2 selects by constrained lexicographic minimization among admissible candidates.

This document defines a provisional implementation-level BOC contract: invocation domain, result states, evidence discipline, temporal applicability, lineage requirements, Tier-2 metric domains, comparison semantics, decision-receipt requirements, and operational evaluation protocol. It operationalizes the parent policy in `docs/simemp.md` §7 for current Qn-universe implementation and integration work. It is not a declaration that these structures, encodings, algorithms, or receipt forms are native or canonical Qn artefacts. The parent policy and this document MUST be kept aligned while this provisional contract is in use.

A BOC result is a justified decision under a bound conditional context. It is not an eternal property of a candidate. Historical outcomes remain immutable, while their applicability to a later invocation may change as dependencies, workload, policy, evidence, or operating conditions change.

### 1.1. Provisional status and Qn artefact precedence

All implementation-facing specifications in this document are **provisional for the Qn-universe implementation**. They provide an explicit, testable bridge while equivalent native Qn artefacts, primitives, semantic rules, accounting constructs, lineage structures, and integrity mechanisms are unavailable, incomplete, or not yet adopted for these purposes.

This document MUST NOT be interpreted as defining new foundational Qn semantics or as superseding an existing canonical Qn artefact. When an equivalent Qn artefact is identified and accepted for a given concern, that Qn artefact becomes authoritative for the corresponding implementation semantics. This document MUST then reference it, map the affected BOC requirement to it, and remove or mark as superseded any duplicate provisional mechanism. A BOC-level wrapper MAY remain where needed to express decision-policy requirements not already supplied by the Qn artefact.

Equivalence MUST be demonstrated, not inferred from similar names or data shapes. The mapping MUST identify at least: (a) the BOC requirement; (b) the Qn artefact and version; (c) the semantic coverage; (d) any uncovered or divergent behavior; and (e) the disposition of the provisional rule. Until that mapping is reviewed, the provisional rule remains an implementation proposal and MUST NOT be presented as native Qn behavior.

Where a native Qn artefact conflicts with this provisional document, do not silently adapt or reinterpret the Qn artefact. Record the conflict and resolve it through the governing Qn policy and versioning process. BOC MUST defer to established Qn semantics unless an authorized Qn-level change explicitly establishes otherwise.

The word **MUST** in this document expresses a requirement for implementations claiming conformance to this provisional BOC profile. It does not, by itself, make the requirement a foundational or universal Qn-universe rule.

## 2. Invocation domain

A BOC invocation is a finite decision instance:

`D = (C, P, W, π, B, A, T)`

| Symbol | Meaning | Requirement |
|---|---|---|
| `C` | Candidate set | Finite, non-empty, explicitly enumerated candidate identities |
| `P` | DCC decision profile | Versioned profile declaring applicable invariants, metric bounds, and evaluation rules |
| `W` | Workload contract | Declared workload or workload envelope |
| `π` | Tier-2 priority permutation | One declared permutation of the three operational-agility components |
| `B` | Evaluation budget | Finite budget available to complete the invocation |
| `A` | Adjudication authority | Principal or governance role authorized to resolve a residual full-key tie |
| `T` | Temporal/evidence context | Observation point or interval, relevant environment state, evidence provenance, and validity conditions |

These inputs MUST be bound before candidate evaluation begins and MUST remain immutable for the invocation. A change to any bound input creates a new invocation. Candidates MUST NOT be silently added, removed, or replaced after evaluation begins.

An empty candidate set is a malformed invocation. It does not produce a BOC result.

### 2.1. Candidate identity

Each candidate MUST have:

- a stable candidate identifier;
- an immutable identity commitment for the artifact or design description evaluated;
- a declared dependency-lineage commitment;
- references to its Tier-1 and Tier-2 evidence.

Different declared identities remain distinct even if their descriptions appear equivalent. BOC MUST NOT infer candidate identity from a short hash comparison alone. Any canonical equivalence rule MUST be separately specified and evidenced.

### 2.2. Identity, lineage, and applicability

BOC MUST distinguish three concepts:

- **Identity:** which exact candidate artifact or design revision is evaluated.
- **Lineage:** the candidates, dependencies, versions, and transformations from which it derives.
- **Applicability:** whether evidence from an earlier evaluation remains relevant under the current invocation context.

A candidate derived from a previously rejected candidate does not automatically inherit that rejection. Its lineage MUST preserve the relationship, including relevant historical findings and the changes made. The new candidate is evaluated under its own identity and current context.

Lineage does not establish equivalence. Shared ancestry does not make two candidates identical, and a changed dependency does not by itself establish that a prior failure has been resolved.

### 2.3. Temporal and conditional context

The result MUST be interpreted relative to `T` and the other bound invocation inputs. The receipt MUST identify the evaluation observation point, using a declared physical timestamp, logical clock, epoch, or other Qn-universe time reference. It MUST also identify relevant operating conditions and any evidence-validity interval or invalidation conditions that can be established.

A receipt records what was justified under its original context. It MUST NOT be silently rewritten when that context later changes. A later evaluation creates a new receipt linked to the earlier receipt by lineage or supersession references.

A previous result MAY be reused as evidence in a later invocation only after checking its applicability against the later candidate identity, dependency lineage, policy/profile, workload, environment, measurement method, and evidence-validity conditions. Reuse of evidence is not reuse of the old decision. The later invocation MUST independently determine whether the evidence remains applicable and whether the candidate satisfies the current gates and comparison rules.

A changed dependency lineage, workload, policy, environment, or measurement method requires reassessment of applicability; it does not automatically prove that the previous result is false or that the candidate now passes. Unchanged lineage does not guarantee that time-sensitive conditions or evidence remain valid.

### 2.4. Common evaluation context

All candidates in one invocation MUST be evaluated against the same `P`, `W`, `π`, and budget-accounting rules. Candidate-specific evidence and declared bounds may differ, but the evaluation contract MUST NOT be changed to favor a candidate.

A candidate that cannot be evaluated against the common workload contract MUST NOT be assigned a favorable metric by default. Its evidence state MUST be recorded as unresolved or failed under the applicable rule.

## 3. Result domain

The result of a BOC invocation is exactly one of:

`Result_BOC ∈ { SELECTED, INCONCLUSIVE, NO_ADMISSIBLE_CANDIDATE, BUDGET_EXHAUSTED }`

Every result MUST be accompanied by a decision receipt.

### 3.1. `SELECTED`

At least one candidate passed Tier 1, the required Tier-2 comparison was completed, and one candidate was selected by the declared lexicographic rule or by an explicitly recorded adjudication of a residual full-key tie.

The receipt MUST identify the selected candidate, Tier-1 evidence for every candidate, Tier-2 keys and evidence for every candidate admitted to Tier 2, the declared `π`, the selection basis, invocation inputs, and accounting record.

### 3.2. `INCONCLUSIVE`

The invocation cannot justify a unique selection under its declared rules and available evidence.

Examples include:

- a required Tier-1 fact is unresolved and could affect admission;
- a Tier-2 component cannot be compared and no later component resolves the comparison;
- multiple candidates have equal complete lexicographic keys and no authorized adjudication is recorded; or
- evidence is internally inconsistent and cannot be resolved within the invocation.

The receipt MUST identify the unresolved predicate or metric, affected candidate or candidates, and evidence needed to continue. An unresolved fact MUST NOT be silently converted into a Tier-1 pass, a Tier-1 failure, a zero-valued metric, or a best-case estimate.

A failure that cannot be reproduced MUST NOT automatically become a pass. Its evidential status remains unresolved until the discrepancy is explained or the governing evaluation rule determines otherwise.

### 3.3. `NO_ADMISSIBLE_CANDIDATE`

Every candidate in `C` was evaluated at Tier 1, and no candidate satisfied all Tier-1 admission predicates.

Every failure contributing to this result MUST have a traceable justification: failed predicate, applicable threshold or rule, observed or formally derived value, evidence provenance, measurement method and conditions, and candidate/dependency versions to which the finding applies.

If any candidate has unresolved Tier-1 evidence that could change the admission outcome, the invocation returns `INCONCLUSIVE`, not `NO_ADMISSIBLE_CANDIDATE`.

A candidate rejected by Tier 1 receives the existing terminal status `REJECTION_BY_GATEWAY`. This BOC result does not create or replace a Qn-universe terminal status.

### 3.4. `BUDGET_EXHAUSTED`

The declared evaluation budget was exhausted before the procedure could establish one of the other result states.

The receipt MUST record the initial budget, amount consumed, stage and operation at exhaustion, completed evaluations, and remaining unevaluated work where determinable within budget.

A partial ranking MUST NOT be promoted to `SELECTED`. A later attempt with a larger budget or revised evaluation plan is a new invocation.

The gateway fee, where applicable, remains governed by the SIMEMP gateway-accounting rule. `BUDGET_EXHAUSTED` does not refund or waive a fee already incurred.

## 4. Invocation procedure

The following order applies:

1. **Bind the invocation.** Freeze `C`, `P`, `W`, `π`, `B`, `A`, and `T`.
2. **Apply gateway accounting.** Charge the applicable declared gateway fee before candidate inspection, in accordance with the governing SIMEMP accounting rule.
3. **Evaluate Tier 1.** Evaluate every required admission predicate for each candidate, subject to budget. Record `PASS`, `FAIL`, or `UNRESOLVED`.
4. **Determine admissibility.** A candidate enters Tier 2 only if every required Tier-1 predicate is `PASS`.
5. **Resolve an empty admissible set.** If all candidates are fully evaluated and none is admissible, return `NO_ADMISSIBLE_CANDIDATE`. If unresolved evidence could change admission, return `INCONCLUSIVE`.
6. **Evaluate Tier 2.** Evaluate the declared Tier-2 key for every admissible candidate under the common workload and comparison contract.
7. **Select or report uncertainty.** Return `SELECTED` only when the declared ordering identifies a unique minimum or valid adjudication resolves a residual full-key tie. Otherwise return `INCONCLUSIVE`.
8. **Enforce budget exhaustion.** If the budget is exhausted before the applicable result is established, return `BUDGET_EXHAUSTED`.
9. **Seal the receipt.** Bind the result, evidence references, accounting, and invocation inputs into the decision receipt.

Budget exhaustion takes precedence whenever further work is required to establish a result. A result fully established before budget exhaustion remains valid.

## 5. Evidence and failure discipline

### 5.1. Tier-1 evidence states

Each required Tier-1 predicate has exactly three evidence states:

- `PASS`: evidence establishes that the predicate is satisfied.
- `FAIL`: evidence establishes that the predicate is violated.
- `UNRESOLVED`: evidence is absent, insufficient, contradictory, or unverifiable.

`UNRESOLVED` is not equivalent to `PASS` or `FAIL`. It MUST NOT be used as a favorable default or as a proven violation.

Tier-2 values MUST use the evidence modalities and comparison requirements defined in §8. Unknown values MUST NOT be replaced with zero, an assumed best value, or an unstated default.

### 5.2. Failure justification and post-mortem

For every Tier-1 `FAIL`, and every Tier-2 finding that materially excludes or disadvantages a candidate, the receipt MUST retain or reference enough information to reconstruct the finding.

The justification SHOULD include:

- exact predicate, constraint, or metric and its version;
- measured value, formally derived bound, or rubric outcome;
- units, instrument/tool version, calibration status, sampling method, and operating conditions, where applicable;
- candidate identity and dependency lineage evaluated;
- evidence provenance, observation point, and integrity commitment;
- reproduction procedure and result, where feasible;
- post-mortem classification; and
- known remediation paths or context changes that justify reevaluation.

Post-mortem classifications MAY include `CONFIRMED_REPRODUCIBLE`, `TRANSIENT`, `NON_REPRODUCIBLE`, `CONTRADICTED_BY_LATER_EVIDENCE`, or `UNRESOLVED`. These classify the evidence and diagnosis; they are not additional BOC result states.

A non-reproducible failure is not automatically a pass. It normally requires `INCONCLUSIVE` for the affected predicate until the discrepancy is resolved.

A post-mortem MAY be completed after the invocation. It MUST be recorded as a linked follow-up artifact and MUST NOT mutate the original receipt or retroactively change its result.

## 6. Adjudication boundary

Adjudication does not modify the lexicographic key and is not an implicit tie-breaker.

Where two or more candidates have equal complete keys, authority `A` MAY resolve the tie only if:

1. equality is recorded;
2. authority and scope were bound in the invocation;
3. rationale identifies the criteria used beyond the exhausted BOC key;
4. selected candidate and alternatives are recorded; and
5. adjudication is included in the receipt.

Without valid adjudication, the result is `INCONCLUSIVE`.

Adjudication MUST NOT waive a Tier-1 failure, alter `π` after evaluation, or change the candidate set retroactively.

## 7. Tier-2 metric domains

Tier 2 minimizes the following key:

`LexKey(c) = ( ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_gap(c)) )`

`ImplicitDeps` is always the first component. The calling DCC profile declares `π`, a permutation of the three operational-agility components. Every invocation MUST also bind a finite required target-profile set `R` in its DCC decision context.

### 7.1. `ImplicitDeps`

`ImplicitDeps(c)` is the count of distinct dependencies used by the candidate's construction or required execution path that are absent from its declared dependency commitment but are discoverable and attributable to the candidate.

- The value is a non-negative integer count over a declared dependency graph and observation boundary.
- Each dependency is counted once by its canonical dependency identity.
- A dependency that is present but undisclosed contributes to this count.
- A dependency whose identity or causal origin cannot be verified is not merely a Tier-2 burden: it makes the relevant Tier-1 identity/lineage predicate `UNRESOLVED` or `FAIL`, according to the evidence.
- Declaring a dependency does not make it free of cost; declared external dependencies may contribute to `M_coupling`.

The dependency graph, discovery boundary, canonical identity rule, and evidence snapshot MUST be bound to the metric record. The value MUST NOT be inferred from an incomplete scan presented as exhaustive.

### 7.2. `ΔW_worst`

`ΔW_worst(c)` is the candidate's declared worst-case logical work bound for the common workload contract `W`, expressed in Abstract Compute Units (`U`). It is a static or formally derived bound, not a runtime sample and not physical energy.

The bound MUST identify:

- the workload contract and covered input domain;
- the cost model and its version;
- the derivation or measurement basis used to establish the bound;
- the declared worst-case value and unit (`U`); and
- any assumptions or preconditions required for the bound to hold.

Candidate values are comparable only when their workload domains and cost-model semantics are compatible. A candidate lacking a justified finite bound cannot receive a favorable default; if the missing bound can affect the decision, the invocation is `INCONCLUSIVE`.

### 7.3. `M_coupling`

`M_coupling(c)` is the count of distinct external coupling edges in the candidate's declared architecture graph. An edge represents either an external dependency contract or an externally visible interface boundary crossing the candidate boundary.

- The graph boundary and edge-classification rule MUST be declared by the DCC profile.
- Each canonical edge is counted once; multiple references to the same edge do not multiply the count.
- Internal edges wholly within the candidate boundary are excluded.
- A declared external dependency remains a coupling even though it is not an implicit dependency.

The metric is a non-negative integer. Candidates are comparable only when evaluated against the same graph-boundary and edge-classification rules.

### 7.4. `P_gap` — portability gap

The parent policy's former `P_coverage` count rewarded a larger supported-profile count even though the complete Tier-2 key is minimized. That direction is inconsistent with the intended portability objective. `P_coverage` is therefore retired as the optimization component and replaced by `P_gap`.

For the finite required target-profile set `R` bound by the invocation:

`P_gap(c) = | R \ SupportedProfiles(c) |`

`SupportedProfiles(c)` contains only profiles for which the candidate has admissible compatibility evidence under the DCC profile's declared compatibility predicate. `P_gap` is a non-negative integer; zero means every required profile is supported. BOC minimizes `P_gap`.

The receipt MUST identify `R`, the compatibility predicate and version, the supported subset, and evidence for each claimed supported profile. Profiles outside `R` do not affect this metric. The set `R` MUST be fixed before candidate evaluation and MUST be common to all candidates in the invocation.

`P_coverage` MAY appear in historical receipts but MUST NOT be used as a current minimization component. A migration from historical `P_coverage` values to `P_gap` requires the original required-profile set and per-profile support evidence; a scalar coverage count alone is insufficient to reconstruct `P_gap`.

## 8. Evidence modalities and comparison

Every Tier-2 metric value MUST carry one evidence modality:

- `MEASURED`: observed using a declared method, instrument/tool version, calibration state, and operating conditions;
- `FORMALLY_DERIVED`: established by a reproducible derivation, proof, or valid upper bound;
- `QUALITATIVE`: assigned through a versioned ordinal rubric with recorded rationale and evidence;
- `UNKNOWN`: insufficient, absent, contradictory, or inapplicable evidence.

`UNKNOWN` MUST NOT be converted to zero, a best-case value, or a favorable ordinal level. A metric record MUST include its definition/version, value or ordinal level, unit/domain, evidence modality, provenance, candidate/dependency identity, observation context, and validity conditions.

### 8.1. Quantitative comparability

Quantitative values are comparable only when they share compatible metric definitions, domains, units, workload/profile context, and evaluation semantics. A unit conversion is permitted only when the conversion is explicit, deterministic, and preserves the metric's meaning. Runtime observations MUST NOT replace a required worst-case declared bound.

### 8.2. Qualitative rubric mode

A qualitative value MUST be assigned using the same versioned rubric for every candidate compared on that component. The rubric MUST define an ordinal ordering and the evidence required for each level. A rubric level is an ordinal judgment, not a measured quantity.

Qualitative values from different rubric versions are not comparable unless a declared, validated mapping establishes equivalence. A qualitative value MUST NOT be compared directly with a quantitative value merely by assigning it a convenient number. Mixed modalities require an explicit, versioned mapping that is justified independently of the candidate outcome; absent such a mapping, the comparison is unresolved.

### 8.3. Lexicographic comparison

Comparison follows the exact component order in `LexKey(c)`:

1. Compare `ImplicitDeps`.
2. If equal, compare the operational-agility components in the order declared by `π`.
3. Stop at the first component with a strict ordering.
4. Proceed to the next component only when the current component is established equal under compatible semantics.

If the first component needed to distinguish two candidates is unknown or incomparable, BOC MUST NOT skip it and use a later component to manufacture an ordering. If that unresolved comparison could affect the selected candidate, the result is `INCONCLUSIVE`.

No weighted sum, averaging, hidden normalization, or implicit tie-break is permitted. A complete-key tie is handled only under §6.

### 8.4. Evidence applicability

The applicability checks in §2.3 apply to each metric observation independently. Historical evidence may be reused only when its candidate identity or explicitly justified derivation mapping, dependency lineage, metric definition, workload/profile set, method, and validity conditions remain applicable. Reuse of a metric observation does not reuse a previous BOC ranking or result.

## 9. Decision receipt and evidence commitment

Every completed BOC invocation MUST produce one immutable decision receipt, regardless of whether its result is `SELECTED`, `INCONCLUSIVE`, `NO_ADMISSIBLE_CANDIDATE`, or `BUDGET_EXHAUSTED`. A malformed invocation that never begins evaluation MUST produce a validation error record, not a BOC decision receipt.

The receipt is the authoritative record of what the invocation evaluated, what evidence it relied on, how it applied the BOC rules, and why it returned its result. A digest or signature alone is not a substitute for the inspectable receipt payload or the referenced evidence.

### 9.1. Receipt requirements

A conforming receipt MUST contain the following logical fields. Implementations MAY encode these fields in a schema-specific representation, but MUST preserve their meaning and distinguish absent, unknown, not applicable, and empty values.

| Field | Required content |
|---|---|
| `receipt_schema` | Stable schema identifier and version |
| `receipt_id` | Identifier derived from the canonical receipt payload commitment |
| `invocation_id` | Unique identifier for this BOC invocation |
| `created_at` | Observation/sealing time with declared time basis; it MUST NOT be treated as a substitute for the invocation's bound logical or physical time context |
| `specification` | BOC specification version, parent SIMEMP policy version, and applicable DCC profile identifier/version |
| `invocation` | Bound `C`, `P`, `W`, `π`, `B`, `A`, and `T`, including the required target-profile set `R` |
| `candidates` | Candidate identities, artifact commitments, dependency-lineage commitments, and per-candidate evaluation records |
| `tier1` | Predicate-level `PASS`, `FAIL`, or `UNRESOLVED` states with evidence references and justifications |
| `tier2` | Metric definitions/versions, values or ordinal levels, units/domains, evidence modalities, applicability, and comparison outcomes for every admitted candidate evaluated |
| `decision_trace` | Ordered procedure steps, comparisons, first decisive component where applicable, and selection or inconclusiveness rationale |
| `result` | Exactly one member of `Result_BOC` |
| `adjudication` | Explicit `NONE` or the authority, scope, rationale, selected candidate, and supporting evidence for a permitted tie adjudication |
| `accounting` | Gateway charge, evaluation budget, consumed budget, and accounting references required by the parent policy |
| `evidence_manifest` | Evidence item identifiers, media/content types, provenance, applicability conditions, and integrity commitments |
| `history_links` | Parent, superseded, reassessment, post-mortem, or related receipt references; empty when none exist |
| `integrity` | Canonicalization identifier, hash algorithm identifier, payload commitment, and any required signature records |

A field that does not apply MUST be represented using the schema's explicit not-applicable or empty representation. Implementations MUST NOT omit a required field merely because its value is unknown. Unknown values MUST be explicit and accompanied by a reason where the schema requires one.

### 9.2. Candidate evaluation record

Each candidate record MUST include:

- stable candidate identifier and candidate revision;
- artifact identity commitment and the identity method/version;
- dependency-lineage commitment and dependency manifest reference;
- Tier-1 predicate outcomes and evidence references;
- Tier-2 metric records if and only if the candidate was admitted to Tier 2;
- candidate-level status (`ADMITTED`, `REJECTED`, or `UNRESOLVED`);
- any historical receipt references used as evidence, with an explicit applicability determination; and
- failure/post-mortem references where applicable.

A rejected or unresolved candidate MUST NOT be assigned a fabricated Tier-2 key. A candidate not admitted to Tier 2 may have diagnostic measurements recorded separately, but those measurements MUST NOT be represented as a valid Tier-2 comparison key.

### 9.3. Evidence manifest

Every evidence item relied upon to establish a Tier-1 predicate, Tier-2 value, comparison, or adjudication MUST have a manifest entry. At minimum, an entry MUST identify:

- a stable evidence-item identifier;
- evidence type and media/content type;
- producer, source, or collection process;
- source/tool/instrument version and calibration information where relevant;
- candidate and dependency context to which the evidence applies;
- observation time or interval and relevant operating conditions;
- validity interval or explicit invalidation conditions, if known;
- provenance locator or retained-content reference; and
- integrity algorithm identifier and content commitment.

A locator is not proof that evidence remains available. Evidence needed to independently verify a decision MUST be retained or made retrievable under the repository's evidence-retention policy. If evidence is unavailable at sealing time, the receipt MUST mark the affected evidence unresolved and MUST NOT claim that the corresponding predicate or metric was established by that evidence.

Evidence may be external, but the receipt MUST bind the exact version or content commitment relied upon. A mutable URL without a version, timestamped capture, or content commitment is insufficient as the sole evidence identity.

### 9.4. Decision trace

The decision trace MUST be sufficient for an independent reviewer to reconstruct the invocation's BOC procedure without relying on undocumented tie-breaking or hidden state. It MUST record:

1. the frozen invocation inputs and candidate enumeration;
2. the applicable gateway accounting event;
3. every required Tier-1 predicate outcome and the evidence that supports it;
4. the exact admissible candidate set;
5. Tier-2 metric records and their applicability/comparability determinations;
6. the effective component order, including `ImplicitDeps` first and the bound `π` thereafter;
7. the first decisive component for each comparison that determines the selected candidate, or the precise unresolved/tied component where no unique result is established;
8. any valid adjudication; and
9. the final result and budget/accounting state.

The trace MUST NOT imply that candidates were compared on a later component when an earlier component was unresolved or incomparable. It MUST NOT suppress candidates, failed predicates, or material evidence that could change the result.

### 9.5. Canonical serialization

The receipt payload MUST have a deterministic canonical byte representation before it is committed. For the initial JSON representation, implementations MUST use the JSON Canonicalization Scheme (JCS), RFC 8785, over a schema-validated JSON object encoded as UTF-8.

In addition:

- duplicate object member names MUST be rejected;
- non-finite numeric values MUST be rejected;
- values whose exactness cannot be preserved by the selected JSON number representation MUST be encoded as schema-defined decimal/integer strings or another exact typed representation;
- unordered collections MUST be sorted by a schema-defined stable key before canonicalization;
- arrays whose order is semantically significant MUST retain that order;
- timestamps MUST use the schema's single declared normalized representation and retain the source time-basis metadata where needed;
- absent, `null`, empty, unknown, and not-applicable MUST remain semantically distinct where the schema distinguishes them; and
- implementations MUST NOT canonicalize an incomplete or schema-invalid receipt as a final receipt.

If a future Qn-native canonical serialization is adopted, it MUST be introduced as a versioned canonicalization profile with deterministic test vectors and an explicit migration rule. It MUST NOT silently change the bytes or interpretation of already sealed receipts.

### 9.6. Cryptographic commitment and receipt identity

The integrity commitment MUST bind the complete canonical receipt payload, including the evidence manifest and decision trace, but excluding all derived envelope fields: `receipt_id`, `integrity.payload_commitment`, and signature bytes that sign that commitment. The receipt schema MUST define this projection precisely to avoid self-reference. The `receipt_id` is derived only after the payload commitment has been computed and is carried in the outer receipt envelope; it is not part of the committed payload.

The commitment operation is:

`payload_commitment = H(domain_separator || canonical_payload_bytes)`

where:

- `H` is the hash algorithm identified in the receipt and permitted by the applicable DCC/security profile;
- `domain_separator` is the exact ASCII byte sequence `CDQN-BOC-RECEIPT-v1` followed by one NUL byte (`0x00`); and
- `canonical_payload_bytes` are the bytes produced under §9.5 from the receipt payload projection defined above.

The initial interoperable profile MUST support SHA-256 (`sha-256`). A deployment MAY require an additional algorithm under its security profile. Algorithm identifiers MUST be explicit; implementations MUST NOT infer an algorithm from digest length. Algorithm migration MUST preserve the original commitment and add a new linked commitment rather than rewriting a historical receipt.

`receipt_id` MUST be encoded as the algorithm identifier, a colon, and the lowercase hexadecimal payload digest (for example, `sha-256:<lowercase-hex-digest>`). Because it is an envelope field derived from the commitment, it MUST NOT be included in the bytes being committed. It MUST NOT be treated as proof of semantic equality between candidates or as a substitute for comparing canonical candidate encodings. A hash match is an integrity/indexing result; where identity equality is material, the underlying canonical identity data or an explicitly defined collision-resolution procedure MUST also be checked.

### 9.7. Signatures and authority

A cryptographic signature MAY be attached to the payload commitment when required by the DCC/security profile or when an adjudication authority must attest to its action. A signature record MUST identify the signature algorithm, signer key identifier, signature bytes, key-validity context, and the role or authority scope under which it was produced.

A valid signature establishes that the corresponding key signed the commitment; it does not independently establish that the evidence is true, the candidate is safe, or the BOC reasoning is correct. Signature validation and signer authorization MUST be recorded separately. An unsigned receipt remains structurally valid unless the applicable profile requires a signature; it MUST then be marked as not satisfying that profile's sealing requirement.

### 9.8. Immutability, correction, and supersession

Once sealed, a receipt MUST be immutable. Corrections, new measurements, post-mortem findings, dependency changes, revised policies, and reassessments MUST be represented by new linked records or a new BOC invocation. They MUST NOT overwrite the original payload or commitment.

A later receipt MAY reference an earlier receipt as `REASSESSES`, `SUPERSEDES_FOR_CURRENT_CONTEXT`, `POST_MORTEM_OF`, or another explicitly defined relation. Such a relation MUST NOT imply that the earlier receipt was invalid under its original context. The new receipt MUST state what changed and which prior evidence remains applicable, is invalidated, or remains unresolved.

If a receipt is found malformed or its commitment fails verification, a separate integrity finding MUST identify the failed check. The original bytes and observed commitment SHOULD be retained for audit; a corrected receipt is a new artifact and MUST link to the affected receipt where possible.

### 9.9. Receipt verification

A verifier MUST be able to:

1. validate the receipt against its declared schema version;
2. reconstruct the canonical payload bytes;
3. recompute and compare the payload commitment;
4. validate evidence-manifest commitments for available evidence;
5. validate signatures and signer authorization when present or required;
6. reconstruct the candidate admission set and Tier-2 comparison trace; and
7. determine whether the recorded result follows from the bound rules and evidence.

A commitment match alone is not a complete BOC verification. A receipt is fully verified only to the extent that its schema, integrity, evidence availability, authorization, and decision logic have each been checked and their verification outcomes recorded separately.

### 9.10. Illustrative receipt shape

The following is an illustrative outer-envelope shape, not a complete JSON Schema. The committed payload is the projection that excludes `receipt_id`, `integrity.payload_commitment`, and signature bytes. Field names and enumerations are normative only where defined in this document; implementations MUST publish and version a machine-readable schema before claiming interoperable receipt conformance.

```json
{
  "receipt_schema": "cdqn.boc.receipt/1.0",
  "receipt_id": "sha-256:<encoded-digest>",
  "invocation_id": "<stable-invocation-id>",
  "created_at": {
    "value": "2026-11-07T00:00:00Z",
    "time_basis": "UTC"
  },
  "specification": {
    "boc": "0.4.0",
    "simemp": "1.4.0",
    "dcc_profile": {
      "id": "<profile-id>",
      "version": "<profile-version>"
    }
  },
  "invocation": {
    "candidate_set_commitment": "<commitment>",
    "workload_commitment": "<commitment>",
    "priority_permutation": ["<component-1>", "<component-2>", "<component-3>"],
    "budget": {"unit": "<declared-unit>", "limit": "<exact-value>"},
    "required_profiles": ["<profile-id>"],
    "temporal_context_commitment": "<commitment>"
  },
  "candidates": [],
  "decision_trace": [],
  "result": "INCONCLUSIVE",
  "adjudication": {"status": "NONE"},
  "accounting": {},
  "evidence_manifest": [],
  "history_links": [],
  "integrity": {
    "canonicalization": "RFC8785-JCS",
    "hash_algorithm": "sha-256",
    "payload_commitment": "<digest>"
  }
}
```

The example intentionally uses placeholders and an illustrative unresolved result. It is not a receipt for an actual evaluation and MUST NOT be committed or presented as evidence of a completed BOC invocation.

## 10. Operational evaluation protocol

This section defines a provisional execution protocol for implementations of the BOC policy. It makes evaluation ordering, budget reservation, checkpoints, interruption, and recovery explicit without claiming that the protocol is a native Qn execution primitive. An equivalent accepted Qn artefact takes precedence under §1.1.

### 10.1. Execution lifecycle

Execution lifecycle state is distinct from `Result_BOC`. A BOC invocation has one of these operational states:

- `PREPARED`: invocation inputs and the evaluation plan have been bound; evaluation has not begun.
- `RUNNING`: an evaluation operation is in progress or the next planned operation is ready.
- `PAUSED`: execution has stopped at a valid checkpoint and may be resumed.
- `COMPLETED`: one of the four BOC result states has been established and its receipt sealed.
- `ABORTED`: execution was terminated without establishing a BOC result.

`PREPARED`, `RUNNING`, `PAUSED`, and `ABORTED` are lifecycle states, not additional BOC outcomes. An invocation that reaches `COMPLETED` MUST have exactly one `Result_BOC` under §3. A paused or aborted invocation MUST NOT be represented as `SELECTED`, `NO_ADMISSIBLE_CANDIDATE`, or another completed decision result.

A `PAUSED` invocation MAY resume only from a verified checkpoint and under the same immutable invocation inputs and evaluation-plan version. If those inputs or the plan must change, the prior invocation is not resumed; a new invocation is created and linked to the prior execution record.

An `ABORTED` invocation MUST retain an execution record containing the reason, last valid checkpoint, completed operations, budget accounting, and any partial evidence. It MUST NOT be represented as a completed decision receipt. If a later invocation uses its partial evidence, the applicability checks in §2.3 and §8.4 apply.

### 10.2. Evaluation plan binding

Before the first candidate is inspected, the invocation MUST bind a versioned evaluation plan. The plan MUST declare:

- the canonical candidate ordering;
- the ordered Tier-1 predicate list and the evaluation rule for each predicate;
- the Tier-2 metric definitions and their required evidence modalities;
- the deterministic operation ordering for candidate and metric evaluation;
- the budget unit, operation-cost model, reservation rule, and settlement rule;
- the checkpoint boundaries and recoverable state;
- the implementation/tool versions that can affect evaluation; and
- any permitted concurrency and its deterministic scheduling rule.

The plan MUST be compatible with the bound DCC profile, workload, and BOC specification version. It MUST NOT be changed after evaluation begins. A plan correction or version change requires a new invocation.

Candidate order MUST be derived from a declared canonical ordering of candidate identifiers, not from insertion order, discovery order, network arrival, or evaluator preference. The ordering is procedural only and MUST NOT affect candidate identity or comparison priority.

Tier-1 predicates MUST be evaluated in the order declared by the bound plan. Tier-2 metrics MUST be evaluated in the exact component order prescribed by `LexKey` and `π`, subject to the comparison requirements in §8. A plan MAY declare a deterministic sub-operation order within a metric, but it MUST NOT reorder the BOC priority key.

### 10.3. Budget model and reservation

The invocation budget `B` MUST declare a finite non-negative limit and an explicit accounting unit. The evaluation plan MUST define how every chargeable operation consumes that unit. The budget model MUST distinguish at least:

- gateway invocation charges governed by `docs/simemp.md`;
- Tier-1 predicate evaluation;
- Tier-2 metric evaluation;
- evidence acquisition or validation when charged to the invocation; and
- receipt construction, verification, and sealing when charged to the invocation.

The plan MUST state whether a charge is a fixed amount, a measured amount, or a reserved upper bound settled against actual consumption. No charge may be introduced after the operation has begun unless the bound plan explicitly defines the measurement and settlement rule.

Before a chargeable operation starts, the evaluator MUST reserve its declared cost or upper bound. If the reservation cannot be satisfied, that operation MUST NOT begin. The invocation then returns `BUDGET_EXHAUSTED` if the unfinished operation is required to establish the BOC result. A partial evaluation MUST NOT be promoted to a winner.

Where an operation is charged by measured consumption, the measurement source and accounting granularity MUST be fixed by the plan. Where an upper bound is reserved and actual consumption is lower, the plan MUST specify whether and when the unused reservation is released. An operation that exceeds its declared upper bound is an accounting/protocol violation; the evaluator MUST stop that operation safely, record the overrun, and MUST NOT claim a conforming completed selection from the affected execution.

Gateway fees MUST be charged exactly as required by the parent SIMEMP policy. They MUST NOT be conflated with candidate execution work or silently refunded on failure, pause, abort, or budget exhaustion. A resumed invocation MUST NOT incur the same invocation gateway fee a second time unless the parent policy explicitly defines a separate charge for a distinct invocation.

### 10.4. Deterministic operation scheduling

The default provisional execution mode is sequential and deterministic. Given identical bound invocation inputs, evaluation plan, evidence snapshot, implementation/tool versions, and deterministic operation outcomes, a conforming evaluator MUST execute the same ordered operation sequence and produce the same decision trace.

Parallel evaluation MAY be used only when the bound plan defines deterministic task partitioning, budget reservation, result collection, and checkpoint semantics. Completion order, thread scheduling, or resource contention MUST NOT change candidate ordering, metric priority, budget allocation, or the final comparison rule. If deterministic parallel accounting cannot be established, the evaluator MUST use sequential mode.

An evaluator MUST NOT selectively evaluate a candidate earlier because it appears promising, nor change operation order in response to intermediate candidate scores, unless that adaptive strategy is itself explicitly specified and versioned in the bound plan and is proven not to alter the BOC decision semantics. The default profile does not permit adaptive candidate prioritization.

### 10.5. Tier-1 execution and short-circuiting

Tier-1 predicates MUST be evaluated using the bound predicate order and evidence rules. An implementation MAY stop evaluating further Tier-1 predicates for a candidate after a decisive `FAIL` only if the plan declares that short-circuit rule and the remaining predicates are recorded as `NOT_EVALUATED` rather than as `PASS`, `FAIL`, or `UNRESOLVED`.

`NOT_EVALUATED` is an execution-coverage status, not a fourth evidence state. It MUST be kept distinct from the three Tier-1 evidence states in §5.1. The receipt MUST identify the decisive failure and the unevaluated predicates.

A candidate with a decisive Tier-1 `FAIL` is rejected and MUST NOT enter Tier 2. If no candidate is admitted, BOC may return `NO_ADMISSIBLE_CANDIDATE` only when every candidate's admissibility is established. A candidate with unresolved evidence that could change the empty-admissible-set conclusion requires `INCONCLUSIVE`, as specified in §3.2 and §3.3.

### 10.6. Tier-2 execution and comparison completion

Every Tier-1-admissible candidate MUST receive the Tier-2 evidence required to establish its comparison position under the complete lexicographic key, unless budget exhaustion or an unresolved comparison prevents completion.

A metric later in `π` MAY be skipped for a candidate pair only after an earlier component has established a strict ordering for that pair. Such a skip MUST be recorded as `NOT_REQUIRED_BY_LEXICOGRAPHIC_ORDER`, not as an unknown value or an inferred metric result. If a component is unresolved or incomparable and could determine the ordering, later components MUST NOT be used to bypass it.

An implementation MAY reuse applicable metric evidence under §8.4. The receipt MUST record the evidence reference and the applicability determination. Evidence reuse does not reduce the required comparison coverage unless the plan's deterministic accounting rule explicitly assigns a lower cost to validated reuse.

### 10.7. Atomic operations and checkpoints

Each planned evaluation operation MUST have a stable operation identifier and a defined completion boundary. Its result and budget settlement MUST be committed atomically: either the operation is recorded as completed with its evidence and charge, or it remains incomplete and MUST be safely retried or abandoned under the plan.

A checkpoint MUST contain enough state to resume without changing the invocation's meaning. At minimum it MUST bind:

- invocation identifier and immutable input commitments;
- evaluation-plan identifier/version and commitment;
- last completed operation and next planned operation;
- candidate and predicate/metric progress states;
- budget consumed, reserved, and remaining;
- evidence references and their integrity/applicability status; and
- checkpoint creation time or logical observation point.

A checkpoint MUST be integrity-protected using an implementation mechanism declared by the provisional profile or its accepted Qn equivalent. A checkpoint is not a completed decision receipt and MUST NOT be treated as one.

### 10.8. Interruption, resumption, and recovery

Interruption MAY result from resource limits, process termination, infrastructure failure, or an authorized pause. An interruption MUST NOT change the bound invocation inputs or completed operation records.

On resumption, the evaluator MUST:

1. validate the checkpoint integrity and invocation/plan commitments;
2. verify that the same specification, profile, plan, and required implementation versions remain available;
3. verify that evidence required for the next operations remains accessible and applicable;
4. reconcile reserved and settled budget charges without double charging completed operations;
5. resume at the first incomplete operation; and
6. record the resumption event in the execution history.

If any required check fails, the evaluator MUST NOT continue as though the checkpoint were valid. It MUST either retain the invocation as `PAUSED` pending resolution or mark it `ABORTED` with a reason. A changed workload, candidate set, DCC profile, priority permutation, budget contract, or evaluation plan requires a new invocation.

Completed operations MUST be idempotent across recovery: replay MUST NOT duplicate their evidence records or charges. If the implementation cannot establish idempotency, it MUST stop and record an unresolved recovery condition rather than risk double accounting or altered results.

### 10.9. Budget exhaustion and incomplete execution

If the remaining budget cannot reserve the next required operation, the invocation MUST stop before that operation begins. If the missing operation could affect admission, comparison, or receipt validity, the completed BOC result is `BUDGET_EXHAUSTED`.

The receipt MUST include all completed operations, the blocked next operation, its required reservation, remaining budget, and any candidates or comparisons left incomplete. A partial ranking MUST be retained as diagnostic trace only; it MUST NOT be exposed as a selected result.

If the BOC result was already fully established and the remaining work is optional or not required by the bound plan, budget exhaustion of that optional work does not invalidate the established result. The receipt MUST distinguish required work from optional diagnostics.

### 10.10. Operational reproducibility

A decision is operationally reproducible only when an independent evaluator can recover the bound invocation, evaluation plan, evidence snapshot or equivalent immutable evidence, implementation/tool versions, operation order, and accounting semantics sufficiently to reconstruct the same decision trace.

The receipt SHOULD classify reproducibility as `REPRODUCIBLE`, `PARTIALLY_REPRODUCIBLE`, `NON_REPRODUCIBLE`, or `NOT_YET_TESTED`, with reasons and missing dependencies. This classification is metadata about the receipt and is not a BOC result state.

A later reproduction attempt MUST create a linked verification or post-mortem record. It MUST NOT overwrite the original receipt or change the original result. A discrepancy MUST be recorded and investigated; it MUST NOT silently rewrite historical evidence.

## 11. Explicit non-goals

This provisional version does not yet define a complete machine-readable JSON Schema, the exact budget schedule for each predicate or metric evaluation, or the canonical Qn-universe representations of time, epochs, lineage transitions, and evidence-validity intervals. The receipt requirements in §9 define the logical content and a provisional implementation canonicalization/commitment profile; they do not establish that RFC 8785 JCS, SHA-256, or the proposed envelope is the native Qn representation. Equivalent Qn artefacts MUST be preferred when established and mapped as specified in §1.1. Interoperable implementations still require a published schema and conformance vectors. BOC MUST NOT be claimed fully mechanically decidable where these unresolved implementation contracts are required for execution.

## 12. Conformance tests

| Test | Input condition | Required result |
|---|---|---|
| BOC-1-T01 | Candidate set is empty | Malformed invocation; no BOC result |
| BOC-1-T02 | Every candidate fully fails at least one Tier-1 predicate | `NO_ADMISSIBLE_CANDIDATE` |
| BOC-1-T03 | No candidate passes and unresolved Tier-1 evidence could affect admission | `INCONCLUSIVE` |
| BOC-1-T04 | One candidate passes Tier 1 and is uniquely minimal under the complete Tier-2 key | `SELECTED` |
| BOC-1-T05 | Candidates have equal complete keys and no adjudication | `INCONCLUSIVE` |
| BOC-1-T06 | Equal complete keys are resolved by valid adjudication | `SELECTED`, with rationale recorded |
| BOC-1-T07 | Budget is exhausted before required evaluation completes | `BUDGET_EXHAUSTED`; no partial winner |
| BOC-1-T08 | Candidate set or profile changes after evaluation begins | Original invocation invalidated; new invocation required |
| BOC-1-T09 | Unknown Tier-2 value is silently replaced with zero | Conformance failure |
| BOC-1-T10 | Tier-1 failure is overridden by Tier-2 score or adjudication | Conformance failure |
| BOC-1-T11 | Prior receipt is reused without checking context and dependency applicability | Conformance failure |
| BOC-1-T12 | Non-reproducible failure is silently converted to `PASS` | Conformance failure |
| BOC-1-T13 | Dependency change overwrites a historical receipt | Conformance failure |
| BOC-1-T14 | A failure determining rejection has no traceable justification or evidence reference | Conformance failure |
| BOC-2-T01 | Candidate has one discoverable dependency absent from its commitment | `ImplicitDeps` increases by one |
| BOC-2-T02 | Dependency identity or origin cannot be verified | Tier-1 identity/lineage is `UNRESOLVED` or `FAIL`; not a favorable Tier-2 count |
| BOC-2-T03 | Two candidates have compatible worst-case logical-work bounds | Compare `ΔW_worst` in `U` under the same workload and cost model |
| BOC-2-T04 | Candidate architecture adds one distinct external coupling edge | `M_coupling` increases by one |
| BOC-2-T05 | Required profile set has five entries and candidate supports four | `P_gap = 1` |
| BOC-2-T06 | Candidate supports more profiles outside the required set | No change to `P_gap` |
| BOC-2-T07 | A historical receipt contains only scalar `P_coverage` without profile identities | It cannot be converted to `P_gap` |
| BOC-2-T08 | Tier-2 value is `UNKNOWN` and could change the ordering | `INCONCLUSIVE`; no favorable default |
| BOC-2-T09 | Qualitative values use different rubric versions without a validated mapping | Incomparable; `INCONCLUSIVE` if decision-relevant |
| BOC-2-T10 | Earlier lexicographic component is unknown but a later component favors one candidate | Later component MUST NOT decide the comparison |
| BOC-2-T11 | `P_coverage` is minimized as the current portability component | Conformance failure |
| BOC-2-T12 | A measured runtime sample is substituted for a required worst-case bound | Conformance failure |
| BOC-3-T01 | Completed invocation has no receipt | Conformance failure |
| BOC-3-T02 | Receipt omits one required invocation input or the bound result | Conformance failure |
| BOC-3-T03 | Candidate rejected at Tier 1 is assigned a Tier-2 comparison key | Conformance failure |
| BOC-3-T04 | Decision trace skips an unresolved earlier lexicographic component | Conformance failure |
| BOC-3-T05 | Evidence used for a decision has no evidence-manifest entry | Conformance failure |
| BOC-3-T06 | Mutable evidence URL is the sole identity of decision-critical evidence | Conformance failure |
| BOC-3-T07 | Duplicate JSON member names or non-finite numbers are accepted in canonical payload | Conformance failure |
| BOC-3-T08 | Two conforming encoders produce different canonical bytes for the same receipt data | Conformance failure |
| BOC-3-T09 | Payload commitment excludes a decision-critical field or evidence-manifest entry | Conformance failure |
| BOC-3-T10 | Hash algorithm is inferred rather than declared by identifier | Conformance failure |
| BOC-3-T11 | Receipt is mutated after sealing to add a post-mortem result | Conformance failure; create a linked record |
| BOC-3-T12 | Hash equality is treated as proof of candidate semantic equality | Conformance failure |
| BOC-3-T13 | Signature validity is treated as proof that evidence is true or reasoning is correct | Conformance failure |
| BOC-3-T14 | Commitment verifies but decision logic or evidence applicability is not checked | Receipt MUST NOT be labelled fully verified |
| BOC-3-T15 | Historical receipt is replaced during algorithm migration | Conformance failure; add a linked commitment |


| BOC-4-T01 | Candidate order is supplied in a different insertion order but canonical IDs and invocation inputs are unchanged | Evaluation operation order is unchanged |
| BOC-4-T02 | Evaluation plan is modified after the first operation begins | Original invocation MUST NOT continue under the modified plan |
| BOC-4-T03 | Required operation cannot reserve its declared cost before starting | Operation does not begin; result is `BUDGET_EXHAUSTED` if decision-critical |
| BOC-4-T04 | Operation exceeds its declared upper-bound reservation | Safe stop, overrun recorded, no conforming selection from affected execution |
| BOC-4-T05 | Candidate has a decisive Tier-1 failure and plan permits short-circuiting | Remaining predicates are `NOT_EVALUATED`; candidate is not admitted |
| BOC-4-T06 | Tier-1 predicate is skipped but recorded as `PASS` | Conformance failure |
| BOC-4-T07 | Later Tier-2 metric is skipped after an earlier strict lexicographic ordering | Skip is recorded as `NOT_REQUIRED_BY_LEXICOGRAPHIC_ORDER` |
| BOC-4-T08 | Earlier Tier-2 component is unresolved and later component favors one candidate | Later component MUST NOT bypass the unresolved comparison |
| BOC-4-T09 | Execution is interrupted after an atomic operation checkpoint | Resume continues at the next incomplete operation |
| BOC-4-T10 | Resume attempts to change candidate set, workload, DCC profile, `π`, budget, or plan | Resume is rejected; a new invocation is required |
| BOC-4-T11 | Completed operation is replayed during recovery | Evidence and charges are not duplicated |
| BOC-4-T12 | Checkpoint integrity or invocation commitment cannot be verified | Execution remains paused or is aborted; it does not continue |
| BOC-4-T13 | Budget exhaustion leaves a required candidate comparison incomplete | `BUDGET_EXHAUSTED`; partial ranking is not selected |
| BOC-4-T14 | Budget exhaustion occurs only after the result is fully established and affects optional diagnostics | Established result remains valid; optional work is recorded as incomplete |
| BOC-4-T15 | Parallel evaluator completion order differs while deterministic plan and evidence are identical | Decision trace and result remain unchanged |
| BOC-4-T16 | Same invocation is resumed and gateway fee is charged twice without parent-policy authorization | Conformance failure |
| BOC-4-T17 | Reproduction attempt contradicts a sealed historical receipt | Original receipt remains immutable; discrepancy is recorded in a linked record |

## 13. Normative references

| Document | Role |
|---|---|
| `docs/simemp.md` §7 | Parent BOC policy and two-stage selection rule |
| `docs/abstractionLayers.md` | Layer architecture and terminal-status registry |
| `docs/qnir.md` | QnIR identity and lineage model |
| `docs/qnir_semantics.md` | QnIR semantic constraints |
| `docs/qnir_accounting.md` | Work-budget and accounting semantics |
| `docs/qnPrimitive.md` | Primitive admission and optimization rules |
| `docs/qs.md` §6.1 | Semantic-intent alignment boundary |

---

## 14. Versioning and integration

This document is a versioned, provisional implementation specification subordinate to `docs/simemp.md` §7. Version 0.3.0 added immutable decision-receipt content, evidence-manifest, canonicalization, integrity-commitment, signature, and verification requirements to the BOC-1 and BOC-2 decision and comparison rules. Version 0.3.1 clarified that these implementation specifications remain provisional pending equivalent native Qn artefacts, established Qn artefact precedence and mapping requirements, and limited conformance language to this provisional BOC profile. Version 0.4.0 adds a provisional operational evaluation protocol for deterministic scheduling, budget reservation and settlement, Tier-1 short-circuit recording, Tier-2 comparison completion, atomic checkpoints, interruption, recovery, idempotency, and operational reproducibility. It retains the parent policy correction from `P_coverage` to `P_gap`.

Any future change to result states, invocation inputs, evidence semantics, lineage/applicability rules, receipt fields, canonicalization, or commitment semantics MUST be made through a new versioned artifact under the repository's versioning discipline. When an equivalent Qn artefact becomes authoritative, update the mapping and disposition through a versioned change; do not leave duplicate competing definitions. Historical decision receipts remain immutable.

The parent policy is coordinated in `docs/simemp.md` v1.4.0. The two documents MUST remain aligned: changes to the Tier-2 key or metric direction require coordinated versioned updates. This BOC document remains provisional for implementation until its requirements have been mapped to, and where appropriate replaced by, equivalent accepted Qn artefacts.
