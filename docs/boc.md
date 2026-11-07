---
layout: default
title: BOC Decision and Comparison Specification
description: BOC outcomes, evidence applicability, Tier-2 metrics, and lexicographic comparison rules
version: 0.2.0
updated: 2026-11-07
author: "Christophe Duy Quang Nguyen; GPT-6 (OpenAI), under the direction of Christophe Duy Quang Nguyen"
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

# BOC: Decision Domain and Comparison Semantics

| Field | Specification |
|---|---|
| **Document Title** | BOC: Decision Domain and Comparison Semantics |
| **Version** | 0.2.0 |
| **Last Updated** | 2026-11-07 |
| **Author** | Christophe Duy Quang Nguyen; GPT-6 (OpenAI), under the direction of Christophe Duy Quang Nguyen |
| **Status** | Candidate Specification — BOC-1 and BOC-2 |
| **Parent Policy** | `docs/simemp.md` §7 |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Purpose and scope

The Best of Choices (BOC) policy defined in `docs/simemp.md` §7 uses two stages:

1. Tier 1 is a hard admission gate.
2. Tier 2 selects by constrained lexicographic minimization among admissible candidates.

This document defines the BOC invocation domain, result states, evidence discipline, temporal applicability, lineage requirements, Tier-2 metric domains, and comparison semantics. It operationalizes the parent policy in `docs/simemp.md` §7. The parent policy and this document MUST be updated together when a constitutional Tier-2 metric changes.

A BOC result is a justified decision under a bound conditional context. It is not an eternal property of a candidate. Historical outcomes remain immutable, while their applicability to a later invocation may change as dependencies, workload, policy, evidence, or operating conditions change.

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

## 9. Explicit non-goals

This version does not yet define canonical receipt encoding or cryptographic structure, the exact budget schedule for each predicate or metric evaluation, or a canonical Qn-universe representation of time, epochs, lineage transitions, and evidence-validity intervals. Those remain separately specified obligations. BOC MUST NOT be claimed fully mechanically decidable where these unresolved implementation contracts are required for execution.

## 10. Conformance tests

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

## 11. Normative references

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

## 12. Versioning and integration

This document is a versioned BOC specification artifact subordinate to `docs/simemp.md` §7. Version 0.2.0 defines the Tier-2 metric domains and comparison semantics and is coordinated with the parent policy correction from `P_coverage` to `P_gap`.

Any future change to result states, invocation inputs, evidence semantics, or lineage/applicability rules MUST be made through a new versioned artifact under the repository's versioning discipline. Historical decision receipts remain immutable.

The parent policy is coordinated in `docs/simemp.md` v1.4.0. The two documents MUST remain aligned: changes to the Tier-2 key or metric direction require coordinated versioned updates.
