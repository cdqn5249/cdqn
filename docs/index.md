---
title: CDQN Documentation Portal
description: Root documentation portal for the CDQN project, including the SIMEMP constraints and future Qn/cdqn specifications.
version: 1.0.0
updated: 2026-08-31
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/index.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /
---

<link rel="stylesheet" href="{{ '/assets/css/style.css' | relative_url }}">

# CDQN Documentation Portal

**Project:** [cdqn]({{ '/glossary.html#cdqn' | relative_url }}) — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)  
**Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)  
**Status:** Canonical Foundation Guide

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Navigation Matrix

| Document | Description | Status | Published Page | Source File |
|---|---|---|---|---|
| **Portal** | Documentation root and architectural index | Current | [index.html]({{ '/index.html' | relative_url }}) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| **[SIMEMP Constraints]({{ '/glossary.html#simemp' | relative_url }})** | Foundational physical and computational constraints | Verified | [simemp.html]({{ '/simemp.html' | relative_url }}) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| **[Abstraction Layers]({{ '/glossary.html#abstraction-layer' | relative_url }})** | Structural framework, gateways, and categorical model | Verified | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) |
| **[Qn Primitive Envelope]({{ '/glossary.html#universal-envelope' | relative_url }})** | Governed artifact envelope, axioms, and lifecycle | Verified | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) | [docs/qnPrimitive.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qnPrimitive.md) |
| **[CDQN Glossary]({{ '/glossary.html' | relative_url }})** | Unified glossary and bidirectional source registry | Canonical | [glossary.html]({{ '/glossary.html' | relative_url }}) | [docs/glossary.md](https://github.com/cdqn5249/cdqn/blob/main/docs/glossary.md) |
| **[License (SSL 1.0)]({{ '/glossary.html#ssl' | relative_url }})** | Scaling Source License terms and open core invariants | Canonical | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

> **License Location Note:**  
> `LICENSE.md` resides at the root of the repository. On GitHub Pages, it is accessed via the direct repository link above.

---

## 1. Scope and Architectural Stance

The CDQN framework establishes a finite, governed computational foundation for numerical abstraction and distributed consensus.

Traditional computational models often assume unbounded sets, unmetered memory access, and implicit precision semantics. The [Qn]({{ '/glossary.html#qn' | relative_url }}) architecture explicitly rejects infinite assumptions, constructing every abstraction within measurable physical and logical limits:

- **Strict Finiteness:** No execution state admits actual infinity.
- **[SIMEMP]({{ '/glossary.html#simemp' | relative_url }}) Governance:** All operations are constrained by Tier 1 [Existential Invariants]({{ '/glossary.html#existential-invariant' | relative_url }}) (Identity, Metric, Security) and optimized via Tier 2 [Operational Agilities]({{ '/glossary.html#operational-agility' | relative_url }}) (Efficiency, Modularity, Portability).
- **[No-Implicit Rule]({{ '/glossary.html#no-implicit-rule' | relative_url }}):** No implicit defaults, coercions, or unmeasured state transitions may cross a [SIMEMP Gateway]({{ '/glossary.html#simemp-gateway' | relative_url }}).
- **[Totality by Budget]({{ '/glossary.html#receipt' | relative_url }}):** Non-terminating execution is prohibited; every computation must resolve to an explicit, receipted terminal state within a declared metric budget.
- **[Local-first Principle]({{ '/glossary.html#local-first' | relative_url }}):** Artifacts originate in a local context with origin [`Q(0)`]({{ '/glossary.html#q0' | relative_url }}) and compute unit [`Q(1)`]({{ '/glossary.html#q1' | relative_url }}), exposing higher-order commitments to the [cdqn]({{ '/glossary.html#cdqn' | relative_url }}) network strictly through bounded functors and attestations.

Validation across the stack is established through [Computational Consistency]({{ '/glossary.html#computational-consistency' | relative_url }}): an abstraction is valid if and only if computations authored in [QnLang]({{ '/glossary.html#qnlang' | relative_url }}) and compiled to [QnIR]({{ '/glossary.html#qnir' | relative_url }}) execute deterministically within their bounded envelopes.

---

## 2. Recommended Reading Order

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

1. **[SIMEMP Constraints]({{ '/simemp.html' | relative_url }})**: Foundational physical bounds (Memory Wall, semantic gaps, bounded adversarial models) and the [BOC Policy]({{ '/glossary.html#boc-policy' | relative_url }}).
2. **[Abstraction Layers]({{ '/abstractionLayers.html' | relative_url }})**: Structural hierarchy from Layer 0 (Physical Substrate) to Layer 1 (Node Genesis), layer gateways, and complexity degree stratification.
3. **[Qn Primitive Envelope]({{ '/qnPrimitive.html' | relative_url }})**: Structural anatomy of governed artifacts, typed payload profiles, and the 14 provisional operational axioms.
4. **[CDQN Glossary]({{ '/glossary.html' | relative_url }})** and **[LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)**: Canonical terminology, attribution mechanics, and commercial [Scaling Thresholds]({{ '/glossary.html#scaling-threshold' | relative_url }}).

---

## 3. License and Open Core Invariants

The CDQN project is published under the **[Scaling Source License (SSL) 1.0]({{ '/glossary.html#ssl' | relative_url }})**.

Derivative Works must preserve the [Paternity Reference]({{ '/glossary.html#paternity-reference' | relative_url }}) in source comments and metadata headers:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

Every downstream implementation must strictly preserve the two [Open Core Invariants]({{ '/glossary.html#open-core-invariant' | relative_url }}):
1. **Anti-Patent Defense:** Automatic license termination upon initiating patent litigation against the Author or community.
2. **Non-Scaling Open Access:** Royalty-free access for non-commercial and sub-threshold operations.

Entities exceeding the [Scaling Threshold]({{ '/glossary.html#scaling-threshold' | relative_url }}) (e.g., >10,000 active instances/agents, >100,000 MAU, or >10,000,000 monthly transactions) are required to execute a Commercial License Agreement.

---

## Bottom Navigation

| Destination | Link |
|---|---|
| Documentation Portal | [index.html]({{ '/index.html' | relative_url }}) |
| SIMEMP Constraints | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| Abstraction Layers | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| Qn Primitive Envelope | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| CDQN Glossary | [glossary.html]({{ '/glossary.html' | relative_url }}) |
| Scaling Source License | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Repository Source | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |
