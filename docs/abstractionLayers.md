---
title: Abstraction Layers
description: Structural thesis defining the abstraction-layer framework for the Qn and cdqn stack under SIMEMP constraints.
version: 1.0.0
updated: 2026-08-09
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/abstractionLayers.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /abstractionLayers.html
status: Thesis / constraint guide — not a specification
depends_on:
  - docs/index.md
  - docs/simemp.md
  - LICENSE.md
---

<link rel="stylesheet" href="{{ '/assets/css/style.css' | relative_url }}">

# Abstraction Layers — Structural Thesis for the Qn and cdqn Stack

## Navigation

Use the GitHub Pages links for reading the published documentation. Use the source links for reviewing the committed Markdown files.

| Document | GitHub Pages | Source file |
|---|---:|---|
| Documentation portal | [index.html]({{ '/' | relative_url }}) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| SIMEMP constraints | [simemp.html]({{ '/simemp.html' | relative_url }}) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| This document | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) |
| Scaling Source License | Not published from `docs/` | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Public repository | External | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |

---

## Document Metadata

| Field | Value |
|---|---|
| **Document Title** | Abstraction Layers — Structural Thesis for the Qn and cdqn Stack |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-08-09 |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | Scaling Source License (SSL) 1.0 |
| **Repository Path** | `docs/abstractionLayers.md` |
| **Parent Repository** | [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |
| **Status** | Thesis / constraint guide — not a specification |
| **Depends On** | `docs/index.md`, `docs/simemp.md`, `LICENSE.md` |

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Table of Contents

- [Purpose and Scope](#purpose-and-scope)
- [Normative References](#normative-references)
- [1. Foundational Position](#1-foundational-position)
- [2. Layer Model](#2-layer-model)
- [3. Structural Principles](#3-structural-principles)
- [4. Set and Category Abstraction](#4-set-and-category-abstraction)
- [5. Complexity Degree Stratification](#5-complexity-degree-stratification)
- [6. Numeric Representation and Compilation](#6-numeric-representation-and-compilation)
- [7. Arithmetic Foundation as Structural Example](#7-arithmetic-foundation-as-structural-example)
- [8. Identity, Security, and cdqn Exposure](#8-identity-security-and-cdqn-exposure)
- [9. Structure-Generativity Balance](#9-structure-generativity-balance)
- [10. External Precedents and Recent Convergence](#10-external-precedents-and-recent-convergence)
- [11. Future Qn Definition Files](#11-future-qn-definition-files)
- [12. License Alignment](#12-license-alignment)
- [13. Open Items](#13-open-items)
- [Glossary](#glossary)

---

## Purpose and Scope

This document defines the **structural framework** of the abstraction layers for the Qn and cdqn stack.

It is a thesis and constraint guide. It is not a finalized engineering specification, implementation manual, or formal proof.

This document does not define the internal anatomy of every future Qn primitive. It defines the layer structure, boundaries, gateways, and governing rules that all future Qn definitions and primitives must satisfy.

The foundational numeric couple used by this structural thesis is:

- `Q(0)` — local genesis, origin, causal index zero, empty birth context.
- `Q(1)` — first unit, unity measure, reference for the abstract compute unit.

The `Q(0), Q(1)` couple is sufficient for defining the abstraction-layer framework.

Higher Qn primitives and properties — including but not limited to `Q(2)` through `Q(9)`, prime Qn, twin-prime Qn, parity Qn, fractal dimensions, fractional Qn, probabilistic Qn, branching Qn, advanced operations, patterns, data structures, domain abstractions, and runtime abstractions — are content that populates the framework. They belong in their own dedicated documents.

---

## Normative References

The following documents are normative for this thesis. If a technical conflict exists, `docs/simemp.md` governs the SIMEMP constraint system. If a legal conflict exists, `LICENSE.md` governs.

| Document | Role | GitHub Pages | Source file |
|---|---|---:|---|
| `docs/index.md` | Root documentation portal and project stance. | [index.html]({{ '/' | relative_url }}) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| `docs/simemp.md` | SIMEMP constraints: Security, Identity, Metric, Efficiency, Modularity, Portability. | [simemp.html]({{ '/simemp.html' | relative_url }}) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| `LICENSE.md` | Scaling Source License (SSL) 1.0. Governs licensing, Paternity References, Open Core Invariants, and Scaling Thresholds. | Not published from `docs/` | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Foundational Position

The Qn universe is grounded in the following operational conjecture:

> A number system can abstract any computable phenomenon within a computational environment if, and only if, that abstraction is strictly governed by finite physical realities and constrained by the SIMEMP framework.

A Qn object is not a passive mathematical value. It is a governed, finite, identifiable, measurable, and security-constrained computational entity.

The Qn universe does not assume actual infinity, unrestricted sets, implicit interpretation, or unbounded computation.

Every abstraction must be constructed, bounded, identified, measured, secured, and verified within finite resource envelopes.

The validity of the Qn universe will be checked through Computational Consistency:

> If the logic is internally consistent, and if computations authored in QnLang and compiled to QnIR execute successfully and deterministically within their declared finite boundaries, then the abstraction is considered valid within the Qn universe.

---

## 2. Layer Model

The abstraction layers are structural boundaries. They are not specific Qn primitives. They define where governed Qn objects are born, how they transform, and what may cross from one layer to another.

### 2.1 Layer 0 — Commodity Physical Substrate

Layer 0 is the hardware-agnostic physical substrate.

It includes:

- commodity CPU, GPU, memory, storage, and network interfaces;
- consumer devices, edge devices, and server clusters;
- physical limits such as finite memory, finite compute, finite energy, thermal constraints, and the Memory Wall;
- entropy sources and OS-visible telemetry.

Layer 0 is not yet part of the governed Qn universe.

It is the acknowledged Root of Finiteness.

Raw hardware state, raw entropy, raw telemetry, and raw physical noise are not Qn objects until they are finitely mediated, measured, and sealed through the Layer 0 to Layer 1 gateway.

### 2.2 Layer 0 to Layer 1 Gateway — Node Onboarding and Fault Translation

The gateway between Layer 0 and Layer 1 performs two structural roles.

#### Node onboarding

During onboarding, the node constructs its local genesis object:

- bounded entropy sampling;
- entropy health checking;
- device context collection;
- DCC profile initialization;
- PQC root key generation or commitment;
- construction of local `Q(0)`;
- emission of a genesis receipt.

Possible terminal states include:

- `SUCCESS`
- `ENTROPY_INSUFFICIENT`
- `ENTROPY_SOURCE_FAULT`
- `HARDWARE_FAULT`
- `TIMEOUT`
- `PQC_KEYGEN_FAILURE`
- `GENESIS_RECEIPT_FAILURE`

#### Fault translation

Physical faults and host-level failures must be translated into bounded Qn receipts.

Examples include:

- out-of-memory conditions;
- storage I/O faults;
- network timeouts;
- thermal throttling;
- OS-level termination signals;
- entropy source failures.

No physical fault may silently become an implicit Qn state.

### 2.3 Layer 1 — Node Genesis Layer

Layer 1 is the first governed Qn layer.

It contains the local genesis and the first derived structures:

- `Q(0)` as local origin and causal index zero;
- `Q(1)` as the first unit;
- the abstract compute unit `U`, defined by the minimal governed transition from `Q(0)` to `Q(1)`;
- sign polarity derived from the directed relation between `Q(0)` and `Q(1)`;
- the first arithmetic operations, including additive and multiplicative structures;
- the first dimensional axis `d1`;
- local hardware calibration of the abstract compute unit.

`Q(0)` is local. It is not a global universal zero.

Each node may have its own local `Q(0)` instance, with a common semantic type and a local identity.

`Q(1)` must have deterministic numeric and metric meaning. Entropy may contribute to local identity or receipt generation, but it must not alter the canonical value or unit meaning of `Q(1)`.

### 2.4 Higher Layers

Higher layers are not fixed by this document.

They will be derived sequentially as the project matures.

Any future layer must satisfy the same structural rules:

- explicit Qn objects only;
- DCC profiles at every boundary;
- SIMEMP Gateway validation;
- bounded metric envelopes;
- halt-capable operations;
- causal lineage;
- SSL lineage preservation.

Higher layers may include domain abstractions, runtime abstractions, data-structure abstractions, and advanced transformation layers, but these are content instances of the framework, not replacements for the framework.

---

## 3. Structural Principles

### 3.1 All governed artifacts are Qn

Inside the Qn computational universe, every accepted artifact is a Qn object or is reducible to one.

This includes:

- numeric values;
- operations;
- expressions;
- logic relations;
- receipts;
- identities;
- capabilities;
- metric envelopes;
- DCC profiles;
- attestations;
- public cdqn commitments;
- compiled QnIR artifacts as governed runtime objects;
- QnLang constructs after bounded compilation;
- SSL lineage metadata.

Layer 0 raw physical substrate remains outside the Qn universe until mediated by the Layer 0 to Layer 1 gateway.

### 3.2 The No-Implicit Rule

The following rule is enforced across all abstraction layers:

> In the Qn universe, no implicit entity, behavior, assumption, default, interpretation, or convention may cross a SIMEMP Gateway. Only explicit Qn objects may pass between layers.

Implicitness is treated as a pattern that must be decomposed or recomposed into an explicit pattern under its context before it may cross a layer boundary.

Implicitness violates the Tier 1 Existential Invariants:

- **Identity:** implicit entities have no distinct identifier;
- **Metric:** implicit entities cannot be measured;
- **Security:** implicit assumptions are attack surfaces.

Implicitness also introduces undecidability, non-halting behavior, and non-deterministic interpretation.

Defaults must be explicit Qn objects declared in the relevant DCC profile.

Type coercion, assumed ordering, assumed units, assumed context, assumed operators, and assumed error handling are forbidden unless explicitly represented as governed Qn objects.

Unverified candidates inside a local generation step are allowed only if they are explicitly classified as unverified candidates with declared origin, budget, uncertainty, and status.

They become full explicit Qn objects before crossing any SIMEMP Gateway.

### 3.3 Local-first and controlled exposure

All Qn instances are born locally and indexed locally.

The first identifier component of a Qn object is local to the node.

Remote cdqn nodes must not see raw local Qn objects or raw local Qexpr structures by default.

The cdqn network may only receive:

- bounded attestations;
- higher-complexity Qexpr commitments;
- capability proofs;
- delegation chains;
- metric summaries;
- security policy identifiers;
- SSL lineage commitments;
- pseudonymous issuer identities.

Common type identifiers may be shared across the Qn universe, but instances remain local unless explicitly exported.

### 3.4 DCC profiles everywhere

Every layer, object, operation, morphism, functor, and exported attestation must expose a DCC profile.

DCC means:

- **Dependencies:** what must exist and be verified first;
- **Constraints:** finite limits, exclusions, budgets, precision bounds, and halt conditions;
- **Capabilities:** permitted operations, export rights, delegation rights, and transformation rights.

No DCC profile may be implicit.

### 3.5 Causal arrow and birth order

Every Qn object is born into a local Causal Arrow.

The birth order is finite, bounded, and acyclic.

The foundational birth order is:

- `Q(0)`;
- `Q(1)`;
- first additive operations;
- first multiplicative operations;
- first dimensional axis;
- first ordering relations;
- first set partitions;
- higher-complexity Qexpr.

No Qn object may reference a later-born object as a parent unless explicitly governed through a future-compatible constructive rule.

### 3.6 Totality by budget

Every governed computation must terminate in a defined state.

Defined terminal states may include:

- `SUCCESS`
- `FAILURE`
- `NO_SOLUTION`
- `TIMEOUT`
- `BUDGET_EXHAUSTED`
- `INCONCLUSIVE`
- `QUARANTINED`
- `PRECISION_INSUFFICIENT`
- `EXPRESSION_TOO_COMPLEX`
- `REJECTION_BY_GATEWAY`

Silent non-termination is forbidden.

---

## 4. Set and Category Abstraction

The Qn abstraction layers use a hybrid formal inspiration:

- constructive finite sets for membership and construction;
- locally finite categories for transformation, composition, and layer relationships.

This is an inspirational formal model, not a mandatory external mathematical system.

### 4.1 Local Qn set

For each local node `N`, the local Qn universe is a finite constructive set:

- `Q_N` is the set of governed Qn objects born at node `N`;
- `Q(0)_N` is the first member of `Q_N`;
- membership in `Q_N` is witnessed by construction, identity, metric envelope, security validation, and lineage.

No universal set of all Qn objects is admitted.

### 4.2 Local Qn category

For each node `N`, a local category may be described as:

- objects: governed Qn objects in `Q_N`;
- morphisms: governed transformations between Qn objects;
- identity morphisms: explicit governed identities;
- composition: bounded, DCC-constrained Qexpr composition.

Under the all-is-Qn rule, morphisms may themselves be reified as Qn objects when required by governance, audit, identity, metric, or security.

### 4.3 Functors and gateways

Layer transitions are modeled as functors constrained by the SIMEMP Gateway.

A valid layer-transition functor must preserve:

- identity;
- composition;
- lineage;
- metric bounds;
- security policy;
- DCC constraints;
- SSL lineage metadata.

A functor that introduces implicit interpretation is invalid.

### 4.4 cdqn as a category of local categories

The cdqn network is not a single global category containing all local Qn objects.

It is a higher-level structure composed of:

- local categories;
- public attestations;
- exported higher-complexity Qexpr commitments;
- identity-class certificates;
- capability proofs;
- revocation statements;
- cross-node receipts.

An exposure functor maps selected local Qexpr into public attestations.

The exposure functor must not expose raw local Qn objects unless explicitly authorized by policy and bounded by SIMEMP constraints.

---

## 5. Complexity Degree Stratification

Abstraction layers are organized as families of Qexpr stratified by complexity degree.

The complexity degree is a structural measure, not a semantic judgment.

| Degree | Structural content | Examples |
|---|---|---|
| 0 | Primitives | `Q(0)`, `Q(1)` |
| 1 | Primitive operations | additive and multiplicative operations |
| 2 | Compositions of degree 1 | simple Qexpr compositions |
| 3 | Higher compositions | bounded constants, ordering relations, set partitions |
| n | Compositions of degree n-1 | advanced structures, domain abstractions, runtime attestations |

Each degree is born from previous degrees.

The maximum complexity degree must be bounded and declared.

Complexity degree may be used as an exposure threshold:

> Only Qexpr above a declared complexity degree, and satisfying export policy, may be eligible for public cdqn attestation.

Complexity degree must not be confused with value superiority.

---

## 6. Numeric Representation and Compilation

### 6.1 Floating-point prohibition at the Qn semantic layer

Floating-point notation is forbidden at the Qn semantic layer.

Floating-point introduces implicit rounding, ambiguous identity, hidden precision loss, and platform-dependent behavior.

QnIR may use binary hardware representations internally, but it must not expose floating-point semantics to the Qn semantic layer.

### 6.2 Qexpr notation with zoom, remainder, and dimension

Every governed numeric expression should carry:

- symbolic structure;
- zoom `z`, declaring precision or scale;
- remainder `r`, declaring residual value at the declared zoom;
- dimension `d`, declaring dimensional context;
- DCC profile;
- identity and lineage.

Zoom may be expressed in decimal notation at the QnLang layer for human readability, but the core representation must remain base-independent.

### 6.3 Collapse at compilation time

Qexpr transformations may remain symbolic during QnLang authoring and governed transformation.

The Qexpr collapses to a concrete governed value at compilation time, specifically during the QnLang to QnIR transition.

The collapse must be:

- deterministic;
- bounded;
- total;
- declared at a specific zoom level;
- accompanied by a remainder;
- recorded in lineage;
- preserved under SSL metadata requirements.

No implicit precision loss may occur during transformation.

Precision is finite, but all precision bounds must be explicit.

### 6.4 Bounded symbolic expressions

Symbolic expression size must be bounded.

The DCC profile must declare:

- maximum expression size;
- maximum expression depth;
- maximum complexity degree;
- maximum collapse budget.

If the expression exceeds declared bounds, the system must either perform a governed intermediate collapse or halt with a defined terminal state.

Intermediate collapse is not implicit rounding. It is an explicit governed operation with a receipt.

---

## 7. Arithmetic Foundation as Structural Example

The arithmetic foundation derived from `Q(0)` and `Q(1)` is used here as a structural example of how the abstraction layers govern content.

This document does not fully define all arithmetic operations.

### 7.1 First operations

From `Q(1)`, the first operations include:

- additive operations;
- subtractive operations;
- multiplicative operations;
- divisive operations.

These operations define the additive and multiplicative structure of the first dimensional axis `d1`.

### 7.2 Division as constrained operation

Division may be represented as a bounded constraint:

- given `a` and `b`, find `c` and `r` such that `a = b * c + r`;
- `c` is the quotient;
- `r` is the remainder;
- the remainder must be explicit;
- the solving process must be bounded;
- the denominator must not be `Q(0)`.

`Q(0)` must carry explicit DCC constraints forbidding its use as a denominator or reciprocal.

Rejected division by zero must produce a defined terminal state, such as:

- `DIVISION_BY_ZERO_REJECTED`

### 7.3 Ordering and set partition

The first dimensional axis may be partitioned into:

- negative Qn set;
- zero set;
- positive Qn set.

This ordering is positional, not hierarchical.

Positive Qn are ordered above `Q(0)` on the axis. Negative Qn are ordered below `Q(0)` on the axis.

Positional ordering must not be interpreted as superiority unless a separate explicit superiority relation is declared and governed.

If ordering is indeterminate at the declared zoom level because remainders overlap, the system must halt with a defined state, such as:

- `ORDERING_INDETERMINATE_AT_ZOOM`

---

## 8. Identity, Security, and cdqn Exposure

### 8.1 Identity classes

The Qn stack distinguishes at least three identity classes:

| Identity class | Represents | Must not imply |
|---|---|---|
| Machine identity | node, runtime, container, device, virtual instance | human identity or legal personhood |
| Human identity | natural person or authorized operator where explicitly declared | machine execution without delegation |
| AI agent identity | delegated autonomous or semi-autonomous process | human identity or undelegated authority |

Each identity class must have:

- explicit type;
- pseudonymous or governed identifier;
- delegation chain where applicable;
- capability limits;
- metric budget;
- halt authority;
- revocation path;
- audit receipts.

### 8.2 No anonymous entities, but identity privacy preserved

The cdqn network must not contain anonymous entities.

Every network-visible object or attestation must be accountable to an identity class.

Accountability does not require public disclosure of human identity.

Identity privacy is preserved through:

- accountable pseudonymity;
- minimal disclosure;
- capability proofs;
- selective export;
- revocation without unnecessary disclosure.

### 8.3 Dual-ring PQC boundary

The Qn stack uses a dual-ring cryptographic boundary.

#### Inner ring

The inner ring governs local state:

- local `Q(0)`;
- local root keys;
- local sealing keys;
- local signing keys;
- local Qn objects;
- local Qexpr receipts;
- local Causal Arrow.

#### Outer ring

The outer ring governs network exposure:

- network pseudonymous identities;
- attestation keys;
- delegation certificates;
- exported Qexpr attestations;
- cross-node receipts;
- revocation statements.

The two rings must be cryptographically separated.

The Qn stack must be Post-Quantum Cryptography capable, but no specific PQC algorithm is mandated by this document.

---

## 9. Structure-Generativity Balance

The abstraction layers must not suppress generativity.

The governing principle is:

> Fixed structure, flexible content.

The structure is explicit:

- layers;
- gateways;
- DCC profiles;
- metric envelopes;
- identity rules;
- security policies;
- halt conditions;
- SSL lineage rules.

The content generated inside the structure may remain flexible:

- probabilistic candidates;
- emergent patterns;
- heuristic proposals;
- AI-generated Qexpr;
- search-based constructions;
- symbolic transformations;
- domain-specific derivations.

Generation is allowed to propose candidates.

The SIMEMP Gateway does not govern the private act of proposing an unverified candidate, provided the candidate is explicitly classified as unverified.

The SIMEMP Gateway governs what may cross layers, become sealed, be exposed, be reused, or be accepted as verified.

This preserves the generative pipeline while preventing hallucination, undecidability, and unbounded computation from entering the governed stack.

---

## 10. External Precedents and Recent Convergence

This section is non-normative.

The Qn abstraction-layer framework does not depend on any external paper, framework, or standard.

However, several established and recent areas of computer science corroborate the structural pattern used by Qn: fixed structure with flexible content.

### 10.1 Abstract interpretation

Abstract interpretation provides a lattice of abstraction levels where each level is a sound approximation of concrete behavior.

This supports:

- zoom `z`;
- remainder `r`;
- bounded precision;
- sound approximation;
- probabilistic generation inside governed envelopes.

Recent work in probabilistic abstract interpretation and neural-network verification continues to apply this pattern to AI safety and bounded verification.

### 10.2 Domain theory and operational semantics

Domain theory and operational semantics provide models for bounded evaluation, fixed points, and stepwise computation.

These support:

- symbolic Qexpr transformation;
- collapse at compilation time;
- deterministic evaluation;
- totality by budget.

### 10.3 Sheaf theory and distributed consensus

Sheaf theory models local data and global consistency through gluing conditions.

This supports:

- local-first Qn objects;
- exposure functor;
- cdqn network attestations;
- gluing local receipts into global consistency only under explicit conditions.

Recent distributed-systems and multi-agent research continues to use sheaf-theoretic structures for local-to-global consistency.

### 10.4 Algebraic effects and handlers

Algebraic effects separate effect signatures from effect handlers.

This supports:

- DCC capabilities and constraints;
- QnIR as a bounded handler;
- pluggable hardware profiles;
- governed execution without exposing unbounded operations.

Recent work on governed execution, capability-bounded effects, and WebAssembly-style sandboxing aligns closely with this structural separation.

### 10.5 Linear logic and quantitative resources

Linear logic treats premises as consumable resources.

This supports:

- explicit resource accounting;
- no implicit duplication;
- no implicit discarding;
- metric envelopes;
- bounded transaction accounting.

Recent quantitative linear logic research supports the mapping between logical derivation and measurable resource cost.

### 10.6 Rewriting systems

Rewriting systems provide rules for transforming expressions under termination and confluence constraints.

This supports:

- symbolic simplification;
- future Q(bypass)-style operations;
- bounded simplification;
- deterministic collapse.

Any rewrite-style optimization must be terminating, confluent where required, explicit, and receipted.

### 10.7 Kolmogorov complexity

Kolmogorov complexity provides a notion of bounded descriptive complexity.

This supports:

- complexity degree stratification;
- bounded expression size;
- exposure thresholds;
- metric cost of description.

### 10.8 Homotopy type theory and constructive foundations

Homotopy type theory and univalent foundations provide constructive models where identity is path-like and objects are built explicitly.

This supports:

- constructivist birth order;
- governed identity;
- formal verification;
- traceability from complex Qexpr back to `Q(0)`.

### 10.9 Neuro-symbolic verification

Neuro-symbolic approaches combine neural generation with symbolic verification.

This supports:

- generative candidates;
- explicit verification;
- hallucination containment;
- auditable AI outputs;
- separation between proposal and acceptance.

The Qn abstraction layers provide a governed numeric substrate for this separation.

---

## 11. Future Qn Definition Files

The following categories are acknowledged as future content. They are not defined by this document.

### Category A — Numeric primitives and properties

Future files may define:

- `Q(0)` detailed anatomy;
- `Q(1)` detailed anatomy;
- `Q(2)` through `Q(9)`;
- prime Qn;
- twin-prime Qn;
- parity Qn;
- fractional Qn;
- probabilistic Qn;
- branching Qn;
- other mathematical properties.

### Category B — Operations and transformations

Future files may define:

- base arithmetic operations;
- advanced operations;
- transformations excluding base arithmetic;
- logic operations;
- optimization operations such as reuse and bypass.

### Category C — Patterns, structures, and data

Future files may define:

- Q(patterns);
- Q(set) constructions;
- Quang data structures;
- higher-order Qexpr sets.

### Category D — Domain abstractions

Future files may define domain instances born from foundational Qn primitives:

- Qm — Quang maths;
- Qs — Quang semantics;
- Qphy — Quang physics;
- future domains.

Domain abstractions are instances of the abstraction-layer framework. They are not replacements for the structural framework.

### Category E — Runtime and execution

Future files may define:

- Q(runtime);
- QnIR detailed behavior;
- QnLang detailed syntax;
- cdqn chaining rules;
- data-structure runtime behavior.

This list is not finished.

New categories may appear as the project matures.

---

## 12. License Alignment

This document aligns with the Scaling Source License (SSL) 1.0.

The license governs legal rights and obligations. If this document conflicts with the license, the license prevails.

### 12.1 Paternity Reference

Derivative distributions and governed runtime artifacts must preserve the required Paternity Reference where applicable:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

In software, APIs, compiled binary headers, or runtime specification files such as QnIR, the Paternity Reference must be preserved in source comments and metadata headers.

### 12.2 Open Core Invariants

The abstraction-layer framework must not weaken the SSL Open Core Invariants:

1. **Anti-Patent Defense** — license termination upon patent assertion against the Author or project community.
2. **Non-Scaling Open Access** — royalty-free access for non-scaling academic, open-source, personal, research, and small-scale development.

### 12.3 Scale auditing

The identity-class separation and metric accounting support native auditing of SSL Scaling Thresholds.

Relevant technical counters may include:

- active compute nodes;
- active containers or instances;
- active autonomous agents;
- monthly active users where applicable;
- monthly API or Qn transactions.

The abstraction layers must allow such counters to be explicit, bounded, privacy-preserving, and auditable.

---

## 13. Open Items

The following items remain open and must be addressed in future documents:

1. Internal anatomy of the Qn primitive.
2. Full Qexpr grammar.
3. Detailed QnLang syntax.
4. Detailed QnIR execution model.
5. Detailed cdqn chaining and distributed attestation rules.
6. Exact complexity-degree thresholds for public exposure.
7. Fractional precision model between `Q(0)` and `Q(1)`.
8. Remainder conventions for negative operands.
9. Physical unit system for future physical-domain abstractions.
10. Cross-dimensional comparison and projection rules.
11. PQC algorithm registry and migration policy.
12. Q(reuse) and Q(bypass) detailed rules.
13. Q(dataStruc) definitions.
14. Q(patterns) definitions.
15. Domain abstraction rules for Qm, Qs, and Qphy.

---

## Glossary

**Abstraction Layer**  
A structural boundary in the Qn stack where governed Qn objects are born, transformed, or exposed under SIMEMP constraints.

**SIMEMP**  
Security, Identity, Metric, Efficiency, Modularity, Portability. The constitutional constraint system of the Qn and cdqn stack.

**SIMEMP Gateway**  
The validation boundary between layers. Only explicit, bounded, governed Qn objects may pass.

**Qn**  
A governed numeric entity in the Qn universe.

**Q(0)**  
Local genesis object, origin, causal index zero, empty birth context.

**Q(1)**  
First unit object, unity measure, reference for the abstract compute unit.

**Qexpr**  
A governed expression composed of Qn objects.

**DCC Profile**  
Dependencies, Constraints, Capabilities. The explicit governance envelope of a layer, object, operation, or attestation.

**No-Implicit Rule**  
The rule that no implicit entity, behavior, assumption, default, interpretation, or convention may cross a SIMEMP Gateway.

**Complexity Degree**  
A structural measure of Qexpr composition depth and governance level.

**Causal Arrow**  
The persistent, bounded, ordered sequence of local construction events.

**Zoom z**  
A declared precision or scale level for a governed numeric expression.

**Remainder r**  
The explicit residual value at a declared zoom level or division constraint.

**Dimension d**  
A governed dimensional axis or dimensional context.

**Exposure Functor**  
The governed mapping from local Qexpr structures to public cdqn attestations.

**Local-first**  
The principle that Qn instances are born locally, indexed locally, and not exposed publicly by default.

**Dual-ring PQC Boundary**  
The separation between local cryptographic governance and network-facing attestation.

**Identity Class**  
A governed category of actor, such as machine, human, or AI agent.

**QnLang**  
The future domain-specific language used to author Qn computations.

**QnIR**  
The future intermediate representation used for bounded hardware abstraction and execution.

**cdqn**  
Chained and Distributed Quang Numbers. The distributed composition layer of the Qn stack.

**SSL**  
Scaling Source License 1.0. The license governing the project, derivative works, Open Core Invariants, Paternity References, and Scaling Thresholds.
