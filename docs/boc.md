---
layout: default
title: BOC Decision Domain
description: Decision outcomes, evidence, context, and lineage rules for Best of Choices
version: 0.1.0
updated: 2026-11-06
author: Christophe Duy Quang Nguyen
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

# BOC: Decision Domain and Invocation Boundary

| Field | Specification |
|---|---|
| **Document Title** | BOC: Decision Domain and Invocation Boundary |
| **Version** | 0.1.0 |
| **Last Updated** | 2026-11-06 |
| **Author** | Christophe Duy Quang Nguyen |
| **Status** | Candidate Specification — BOC-1 |
| **Parent Policy** | `docs/simemp.md` §7 |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Purpose and scope

The Best of Choices (BOC) policy defined in `docs/simemp.md` §7 uses two stages:

1. Tier 1 is a hard admission gate.
2. Tier 2 selects by constrained lexicographic minimization among admissible candidates.

This document defines the BOC invocation domain, result states, evidence discipline, temporal applicability, and lineage requirements. It refines the parent policy; it does not replace or silently amend its Tier-1 predicates, Tier-2 metrics, or priority permutation.

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

Tier-2 values require an equivalent explicit evidence classification in the metric specification. Unknown values MUST NOT be replaced with zero, an assumed best value, or an unstated default.

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

## 7. Explicit non-goals

This document does not yet define:

- exact mathematical domains or units of `ImplicitDeps`, `ΔW_worst`, `M_coupling`, or `P_coverage`;
- whether Tier-2 components use total orders, partial orders, or bounded ordinal rubrics;
- normalization and comparison of heterogeneous quantitative and qualitative evidence;
- canonical receipt encoding or cryptographic structure;
- exact budget schedule for each predicate or metric evaluation; or
- canonical Qn-universe representation of time, epochs, lineage transitions, and evidence-validity intervals.

These are subsequent BOC specification steps. BOC MUST NOT be claimed fully mechanically decidable until the required domains and comparison rules are specified.

## 8. Conformance tests

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

## 9. Normative references

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

## 10. Versioning and integration

This document is a versioned BOC specification artifact subordinate to `docs/simemp.md` §7. Its rules refine the invocation and evidence lifecycle without changing the parent policy's Tier-1 predicates or Tier-2 key.

Any future change to result states, invocation inputs, evidence semantics, or lineage/applicability rules MUST be made through a new versioned artifact under the repository's versioning discipline. Historical decision receipts remain immutable.

The parent SIMEMP document should reference this specification in a subsequent coordinated update. Until that integration is committed, `docs/simemp.md` §7 remains the constitutional parent policy and this document is its candidate operational refinement.
