---
layout: default
title: CDQN Documentation Portal
description: Root documentation portal and provisional architectural thesis for the CDQN project and SIMEMP constraints.
version: 1.0.0
updated: 2026-09-05
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/index.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /
terms_used:
  - cdqn
  - simemp
  - qn
  - qnlang
  - qnir
  - computational-consistency
  - existential-invariant
  - operational-agility
  - no-implicit-rule
  - receipt
  - local-first
  - q0
  - q1
  - boc-policy
  - scaling-threshold
  - open-core-invariant
  - paternity-reference
  - ssl
  - dependencies-determinism
  - structural-indirection
  - payload
  - licensed-work
  - derivative-work
  - metric-exhaustion
---

# CDQN Documentation Portal

**Project:** {% include term.html id="cdqn" %} — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)  
**Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)  
**Status:** Provisional Architectural Portal / Exploratory Framework

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Document Metadata

| Field | Specification |
|---|---|
| **Portal Title** | CDQN Documentation Portal |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-09-05 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Active Research Framework |

---

## 1. Scope, Thermodynamics, and Architectural Stance

The CDQN framework investigates a foundational question in computational physics and computer science: *Can a discrete number system abstract any computable phenomenon within physical reality?*

Classical computing abstractions frequently rely on non-constructive assumptions: unbounded memory spaces, continuous unmetered precision, and unmodeled side-channel dissipation. The {% include term.html id="qn" %} architecture rejects actual infinities, establishing computational boundaries grounded in physical laws:

- **Strict Physical Finiteness:** Computation is a non-equilibrium thermodynamic process subject to Landauer's bound ($W \ge k_B T \ln 2$). Unbounded state spaces are physically non-realizable and excluded from execution.
- **{% include term.html id="dependencies-determinism" %}:** Every state transition $S_t$ must possess zero conditional entropy given its declared dependencies ($H(S_t \mid \mathcal{D}(S_t)) = 0$). Implicit or unmeasured states violate this bound, resulting in non-deterministic drift.
- **{% include term.html id="simemp" %} Governance:** Execution within the {% include term.html id="licensed-work" %} is constrained by Tier 1 {% include term.html id="existential-invariant" text="Existential Invariants" %} (Identity, Metric, Security) and optimized via Tier 2 {% include term.html id="operational-agility" text="Operational Agilities" %} (Efficiency, Modularity, Portability).
- **The {% include term.html id="no-implicit-rule" %}:** No implicit type coercions, unmeasured error states, or undeclared assumptions may cross a {% include term.html id="simemp-gateway" %}.
- **Totality by Budget & Dissipative Receipts:** Divergent or infinite loops are rejected. Computations are total by budget via dissipative {% include term.html id="metric-exhaustion" %}, exporting operational entropy via signed, verifiable {% include term.html id="receipt" text="receipts" %}.
- **{% include term.html id="local-first" %} Origin:** Artifacts originate in local node universes rooted in genesis origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) and unity unit [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1).
- **Fractal Protocol:** {% include term.html id="cdqn" %} functions as the universal protocol of governed data movement, chaining state transitions locally between abstraction layers and base domains, while distributing public attestations across physical networks.
- **Substrate vs. Payload Air Gap:** The protocol acts as a passive, neutral conduit. Raw user content, applications, and digital assets remain sovereign {% include term.html id="payload" text="Payloads" %} insulated by protocol blindness and statutory safe-harbor protections.

---

## 2. Method of Exploration: The Recursive Search Loop

The documentation suite does not claim closed mathematical finality. The system operates as an active, recursive discovery pipeline:

$$\text{Proposal} \xrightarrow{\quad} \text{Construction} \xrightarrow{\quad} \text{Evaluation} \xrightarrow{\quad} \text{Selection}$$

Abstractions and axioms represent **provisional hypotheses** formulated using {% include term.html id="structural-indirection" %} to avoid premature structural calcification while retaining agility against zero-day vulnerabilities. Verification is conducted operationally via {% include term.html id="computational-consistency" %}:

$$\text{Valid}(\mathcal{A}) \iff \left( \text{Consistent}(\mathcal{A}) \wedge \forall p \in \text{QnIR}(\mathcal{A}), \, \text{TerminatesWithinBudget}(p) \right)$$

If an abstraction fails to compute within declared finite budgets, the abstraction is refuted and returned to the search loop.

---

## 3. Recommended Reading Sequence

```
[1. SIMEMP Constraints] 
       │
       ▼
[2. Abstraction Layers] 
       │
       ▼
[3. Qn Primitive Envelope] 
       │
       ▼
[4. Glossary & SSL License]
```

1. **[SIMEMP Constraints]({{ '/simemp.html' | relative_url }})**: Thermodynamic realities (Memory Wall, Landauer dissipation), {% include term.html id="dependencies-determinism" %}, {% include term.html id="structural-indirection" %}, and the {% include term.html id="boc-policy" %}.
2. **[Abstraction Layers]({{ '/abstractionLayers.html' | relative_url }})**: Structural hierarchy from Layer 0 (Physical Substrate) to Layer 1 (Node Genesis), fractal {% include term.html id="cdqn" %} data movement, local-first base domains ($\mathrm{Qm}, \mathrm{Qs}, \mathrm{Qphy}$), and complexity degree stratification.
3. **[Qn Primitive Envelope]({{ '/qnPrimitive.html' | relative_url }})**: Structural anatomy of the {% include term.html id="universal-envelope" %}, working operational axioms, typed payload profiles, and lifecycle state machines.
4. **[CDQN Glossary]({{ '/glossary.html' | relative_url }})** and **[LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)**: Canonical terminology, concordance index, attribution rules, and commercial {% include term.html id="scaling-threshold" text="Scaling Thresholds" %}.

---

## 4. License and Open Core Invariants

The CDQN project is licensed under the **[Scaling Source License (SSL) 1.0]({{ '/glossary.html' | relative_url }}#ssl)**.

{% include term.html id="derivative-work" text="Derivative Works" %} and compiled runtime environments must preserve the canonical {% include term.html id="paternity-reference" %} in source comments and metadata headers:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

Every downstream implementation must preserve the two {% include term.html id="open-core-invariant" text="Open Core Invariants" %}:
1. **Anti-Patent Defense:** Automatic license termination upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free access for non-commercial, academic, and sub-threshold deployments of the {% include term.html id="licensed-work" %}.

Entities exceeding the {% include term.html id="scaling-threshold" %} (>10,000 active instances/agents, >100,000 MAU, or >10,000,000 monthly transactions) must execute a separate Commercial License Agreement. Processing or tokenizing a {% include term.html id="payload" %} does not by itself trigger scaling usage.
