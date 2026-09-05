---
layout: default
title: Qn Primitive Envelope
description: Governed Qn artifact model, definitions, and operational axioms under SIMEMP and abstraction-layer constraints.
version: 1.0.0
updated: 2026-09-05
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnPrimitive.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnPrimitive.html
terms_used:
  - qn
  - universal-envelope
  - dcc-profile
  - metric-envelope
  - security-envelope
  - receipt
  - lifecycle-state
  - causal-arrow
  - complexity-degree
  - local-first
  - simemp
  - simemp-gateway
  - q0
  - q1
  - cdqn
  - qnlang
  - qnir
  - qexpr
  - open-core-invariant
  - paternity-reference
  - ssl
  - computational-consistency
  - no-implicit-rule
  - dependencies-determinism
  - structural-indirection
  - payload
  - licensed-work
  - derivative-work
  - metric-exhaustion
  - zoom-z
  - remainder-r
  - dimension-d
  - dual-ring-pqc-boundary
  - identity-class
---

# Qn Primitive Envelope — Governed Qn Artifact Model

| Field | Specification |
|---|---|
| **Document Title** | Qn Primitive Envelope — Governed Qn Artifact Model |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-09-05 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Provisional Artifact Framework / Operational Hypothesis |

---

## Purpose and Scope

This document formalizes the provisional structural container required for governed computational entities within the Qn universe: the {% include term.html id="universal-envelope" %}. 

Because the CDQN project operates in an active search and discovery mode, this artifact model is a **working hypothesis** rather than a rigid, calcified specification. It provides the minimal structural scaffolding necessary to guide construction, transformation, and empirical testing in {% include term.html id="qnlang" %} and {% include term.html id="qnir" %}, while preserving architectural elasticity against premature optimization and zero-day vulnerabilities through {% include term.html id="structural-indirection" %}.

---

## Normative References

The following documents define the normative, physical, and legal constraints governing this framework. If a technical conflict arises, `simemp.md` governs; if a structural conflict arises, `abstractionLayers.md` governs; if a legal conflict arises, `LICENSE.md` governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) and [Payloads]({{ '/glossary.html' | relative_url }}#payload) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Definitions

### 1.1. Qn Artifact

A {% include term.html id="qn" %} artifact is a governed, finite, identifiable, measurable, and security-constrained computational entity. An artifact is not a passive mathematical value; it is a discrete state node embedded in a local {% include term.html id="causal-arrow" %}.

### 1.2. Qn Primitive Envelope

The minimal governance boundary required for any computational entity within the {% include term.html id="licensed-work" %} to exist, mutate, undergo sealing, or traverse a {% include term.html id="simemp-gateway" %}.

### 1.3. Universal Envelope

The invariant set of structural fields shared across all artifact types, parameterized via {% include term.html id="structural-indirection" %} to prevent rigid calcification:

$$\mathcal{E} = \langle \text{Identity}, \, \text{Type}, \, \text{State}, \, \text{Lineage}, \, \text{DCC}, \, \text{Metric}, \, \text{Security}, \, \text{Exposure}, \, \text{PayloadRef} \rangle$$

### 1.4. Typed Payload Profile

The domain-specific content carried by an artifact (numeric quantities, algebraic expressions, pure morphisms, state receipts, identity assertions, or capability grants). Sovereign user content remains a {% include term.html id="payload" %} insulated by protocol blindness.

### 1.5. Identity

The governed witness of an artifact's existence. Identity is {% include term.html id="local-first" %}, collision-resistant, versionable, traceable along causal lineage, and cryptographic-algorithm agile.

### 1.6. Lineage

The directed acyclic causal history of an artifact, recording parent commitments, causal birth index, and transformation receipts.

### 1.7. DCC Profile

The explicit profile defining **Dependencies**, **Constraints**, and **Capabilities**. It serves as an abstract contract rather than a hardcoded binary binding.

### 1.8. Metric Envelope

The finite measurement boundary declaring resource allocations: memory footprint, depth bounds, precision limits, and execution budgets.

### 1.9. Security Envelope

The governed protection boundary declaring authority limits, delegation proofs, Post-Quantum Cryptography (PQC) metadata, and swappable revocation paths.

### 1.10. Lifecycle State

The discrete operational status of an artifact: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, or revoked.

### 1.11. Receipt

A bounded, immutable record of a state transition, fault translation, or boundary crossing, acting as an entropy-exporting dissipative witness.

### 1.12. Unverified Candidate

A proposed computational candidate generated via heuristic or stochastic processes that has not yet passed verification and sealing. Candidates cannot cross a {% include term.html id="simemp-gateway" %}.

### 1.13. Numeric Payload

The specialized profile for numeric entities, explicitly encapsulating {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}.

### 1.14. Exposure Class

The classification governing visibility: local-only, sealable, exportable, attested, quarantined, or revoked.

---

## 2. Paradigm Neutrality

This document does not mandate an object-oriented programming (OOP) paradigm. A Qn artifact is not an OOP object.

The model rejects:
- Rigid inheritance hierarchies and dynamic dispatch tables.
- Hidden mutable state and unmetered side effects.
- Implicit polymorphic coercions.

Transformations are explicit governed morphisms. State changes yield new immutable artifacts indexed sequentially along the local {% include term.html id="causal-arrow" %}.

---

## 3. Provisional Operational Axioms

The Qn operational model is governed by 14 provisional operational axioms. These axioms represent **working search rules** to be challenged and validated via {% include term.html id="computational-consistency" %}.

### Axiom 1 — Explicit Envelope

No Qn artifact exists within the governed universe unless it carries an explicit {% include term.html id="universal-envelope" %}:

$$\forall \alpha \in \mathcal{Q}, \quad \exists ! \, \mathcal{E}(\alpha)$$

### Axiom 2 — Finiteness

Every Qn artifact must have a finite canonical representation and a bounded resource envelope. Actual infinity is not admitted as an executable or storable state:

$$\forall \alpha \in \mathcal{Q}, \quad \mathrm{Size}(\alpha) < \infty \quad \wedge \quad \mathrm{Budget}(\alpha) < \infty$$

### Axiom 3 — Typed Payload Separation

Fields specific to numeric expressions ({% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}) belong strictly to typed payload profiles and must not be made mandatory for non-numeric artifacts. External user content remains a sovereign {% include term.html id="payload" %}.

### Axiom 4 — Local Genesis Dependency

Every governed Qn artifact within a local universe must trace its lineage directly or transitively to the local genesis origin [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0):

$$\forall \alpha \in \mathcal{Q}_N, \quad Q(0)_N \in \mathrm{Lineage}(\alpha)$$

### Axiom 5 — Acyclic Causal Lineage

Lineage graphs are strictly acyclic, governed by {% include term.html id="dependencies-determinism" %}. Every artifact must be born after its causal dependencies:

$$\mathrm{Index}(\mathrm{Parent}(\alpha)) < \mathrm{Index}(\alpha)$$

### Axiom 6 — Totality of Governed Operations

Every governed operation must terminate in a declared, finite terminal state via dissipative {% include term.html id="metric-exhaustion" %} within its declared budget:
`SUCCESS`, `FAILURE`, `NO_SOLUTION`, `TIMEOUT`, `BUDGET_EXHAUSTED`, `INCONCLUSIVE`, `QUARANTINED`, `PRECISION_INSUFFICIENT`, `EXPRESSION_TOO_COMPLEX`, `REJECTION_BY_GATEWAY`, `DIVISION_BY_ZERO_REJECTED`, or `ORDERING_INDETERMINATE_AT_ZOOM`. Silent non-termination is prohibited.

### Axiom 7 — Receipted Boundary Events

Receipts are mandatory for lifecycle transitions, gateway crossings, sealing events, export decisions, fault translations, halt events, rejections, revocations, and capability transfers. Receipts serve as verifiable witnesses exporting operational entropy.

### Axiom 8 — Local-first Exposure

Artifact identifiers are rooted locally. Raw local Qn artifacts are never exposed to remote nodes in the {% include term.html id="cdqn" %} network without explicit authorization and bounded attestation.

### Axiom 9 — Numeric Precision Explicitness

Every numeric Qn artifact must declare finite precision bounds, explicit {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}. No implicit rounding or precision loss is permitted:

$$\text{NumericValue} = \langle z, \, r, \, d \rangle$$

### Axiom 10 — No Floating-Point Semantics

IEEE 754 floating-point behavior is forbidden at the Qn semantic layer. Non-deterministic rounding and platform-dependent approximations are rejected.

### Axiom 11 — Bounded Self-Description

Governance metadata may describe itself, but self-description depth must remain finite, bounded, and version-anchored to prevent infinite recursive metadata towers.

### Axiom 12 — License Lineage Where Applicable

Lineage metadata and the canonical {% include term.html id="paternity-reference" %} must be preserved in distributed artifacts, exported attestations, compiled {% include term.html id="qnir" %} runtime artifacts, and public APIs pursuant to the {% include term.html id="ssl" %}. Encapsulating a Payload does not convert it into a {% include term.html id="derivative-work" %}.

### Axiom 13 — Versioning Creates New Artifacts

Versioning an artifact produces a new immutable artifact with an advanced causal index referencing the predecessor. Mutation in place is prohibited.

### Axiom 14 — Generative Candidate Boundary

Unverified candidates proposed by stochastic or neural processes can exist locally only if explicitly tagged as unverified. Candidates cannot cross a {% include term.html id="simemp-gateway" %}.

---

## 4. Universal Envelope Structure

The Universal Envelope $\mathcal{E}$ is structured using {% include term.html id="structural-indirection" %}, ensuring that underlying cryptographic schemes or serialization layouts can be swapped without invalidating existing causal histories.

### 4.1. Identity Block
Declares local node identifier, local causal index, artifact type identifier, version identifier, content commitment hash, and cryptographic agility metadata.

### 4.2. Type Block
Declares the artifact category (numeric value, expression, operation, logic relation, receipt, identity, capability, DCC profile, metric envelope, security envelope, or attestation).

### 4.3. Lifecycle State Block
Declares current governance status conforming to Axiom 7.

### 4.4. Lineage Block
Declares parent artifact references, local causal index, birth order, derivation type, and transformation receipt references.

### 4.5. DCC Profile Reference
Machine-readable reference to an abstract capability contract declaring dependencies, hard constraints, and permitted capabilities.

### 4.6. Metric Envelope Reference
Declares finite operational bounds (storage footprint, expression depth, recursion limits, and execution budgets).

### 4.7. Security Envelope Reference
Declares authority, capability limits, trust assumptions, PQC algorithm identifiers, and swappable revocation paths.

### 4.8. Exposure Class Block
Declares visibility scope: local-only, sealable, exportable, attested, quarantined, or revoked.

### 4.9. Payload Reference
A typed pointer to the associated {% include term.html id="payload" %} profile.

---

## 5. Typed Payload Profiles

### 5.1. Numeric Payload Profile
Carries {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}, precision boundaries, uncertainty metrics, and base-independent representations.

### 5.2. Expression Payload Profile
Carries symbolic {% include term.html id="qexpr" %} trees, operator references, operand links, expression depth ceilings, structural complexity degree, and collapse budgets.

### 5.3. Operation Payload Profile
Carries pure domain-to-codomain mappings, preconditions, postconditions, budget requirements, and receipt emission behavior.

### 5.4. Receipt Payload Profile
Carries terminal execution state, consumed resource budgets ({% include term.html id="metric-exhaustion" %}), causal index, parent references, and cryptographic commitment signatures.

### 5.5. Identity Payload Profile
Carries {% include term.html id="identity-class" text="identity class" %} (machine, human, AI agent), pseudonyms, delegation chains, capability limits, and revocation endpoints.

### 5.6. Attestation Payload Profile
Carries public commitments, issuer identities, capability proofs, metric summaries, and SSL lineage assertions for network-level exposure across {% include term.html id="cdqn" %}.

### 5.7. Capability Payload Profile
Carries granular execution rights, operational constraints, delegation depths, and expiration bounds.

---

## 6. Identity and Lineage Rules

### 6.1. Local-first Identifier Structure
An artifact identifier is rooted in its local origin:

$$\mathrm{ID} = \langle \text{NodePseudonym}, \, \text{CausalIndex}, \, \text{Type}, \, \text{Version}, \, \text{Commitment} \rangle$$

### 6.2. No Global Raw Identity Exposure
Raw local node identifiers are prohibited from global network exposure. Network interaction uses accountable pseudonyms, capability proofs, and cryptographic commitments.

### 6.3. Versioning
Every modification yields a distinct artifact. Predecessors remain immutable components of the historical causal chain.

### 6.4. Algorithm Agility
Cryptographic primitives must support seamless migration via {% include term.html id="structural-indirection" %}. No single algorithm or signature suite is treated as a permanent dependency.

---

## 7. DCC Profile Rules

### 7.1. Dependencies
Explicitly lists required parent artifacts, signing identities, capability tokens, and environmental prerequisites under {% include term.html id="dependencies-determinism" %}. Dependencies are evaluated as abstract contracts to allow modular updates.

### 7.2. Constraints
Declares hard ceilings: memory allocation, recursion depth, complexity degree ceilings, precision bounds, and forbidden parameters (e.g., denominator zero exclusion).

### 7.3. Capabilities
Declares permitted morphisms: transformation rights, sealing authorization, export authorization, and delegation rights.

### 7.4. Bounded DCC Self-Description
Governance profiles referencing their own governance structures must anchor to fixed schema versions to prevent infinite self-descriptive towers.

---

## 8. Metric Envelope Rules

Every metric envelope must declare finite, measurable bounds:
- Footprint bounds (storage size, expression depth, complexity degree).
- Execution bounds (compute cost, transformation budget, collapse budget).
- Numeric bounds (precision bounds, uncertainty bounds).
- Memory Wall accounting (explicit tracking of data movement and bandwidth dissipation).

---

## 9. Security Envelope Rules

Every security envelope must define protection boundaries against finite adversaries:
- Inner-ring and outer-ring scoping conforming to the {% include term.html id="dual-ring-pqc-boundary" %}.
- Swappable revocation paths and emergency halt authorities designed for zero-day patchability.
- Post-Quantum Cryptography algorithm identifiers with explicit migration semantics.
- Accountable pseudonymity; anonymous entities are strictly forbidden from traversing network boundaries.

---

## 10. Lifecycle and Receipts

### 10.1. Lifecycle States
Standard recognized states: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, revoked.

### 10.2. Candidate State
Generated stochastic or heuristic proposal; classified strictly as unverified; prohibited from traversing a {% include term.html id="simemp-gateway" %}.

### 10.3. Constructed State
Formalized into finite canonical syntax; awaiting constraint evaluation.

### 10.4. Sealed State
Identity-committed, metric-bounded, security-checked, and immutable.

### 10.5. Verified State
Validated against explicit proof or test invariants by an authorized checker.

### 10.6. Rejected, Halted, and Quarantined States
Protective terminal states emitting mandatory failure receipts.

### 10.7. Exportable and Exported States
Certified for controlled exposure across the cdqn distributed network.

### 10.8. Revoked State
Invalidated via an explicit, signed revocation receipt that seals a compromised branch and migrates lineage.

---

## 11. Relation to Abstraction Layers

### 11.1. Layer 0
Raw physical substrate states carry no primitive envelope until mediated by a gateway.

### 11.2. Layer 0 to Layer 1 Gateway
Constructs local genesis receipts, entropy commitments, and the local [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0) envelope.

### 11.3. Layer 1
Houses foundational governed primitives: $Q(0)$, $Q(1)$, abstract compute unit $U$, and axis $d_1$.

### 11.4. Higher Layers
Inherit the invariant universal envelope while defining higher-order typed payloads and local-first domain projections.

---

## 12. Relation to QnLang

{% include term.html id="qnlang" %} serves as the high-level language authoring Qn computations:
- Must declare explicit DCC profiles and metric budgets.
- Must compile deterministically to bounded intermediate representations ({% include term.html id="qnir" %}).
- Must enforce compile-time collapse of symbolic {% include term.html id="qexpr" %} trees.
- Prohibits implicit defaults and floating-point semantics.

---

## 13. License Alignment

This specification enforces the legal provisions of the {% include term.html id="ssl" %}:

### 13.1. Paternity Reference
{% include term.html id="derivative-work" text="Derivative Works" %}, APIs, compiled binary headers, and runtime specification files must embed the notice:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 13.2. Open Core Invariants
1. **Anti-Patent Defense:** Automatic license termination upon patent assertion against the Author or community.
2. **Non-Scaling Open Access:** Royalty-free access for non-commercial and sub-threshold usage of the {% include term.html id="licensed-work" %}.

### 13.3. Scale Auditing
Metric and identity envelopes supply deterministic counters (active compute nodes, containers, agents, and monthly API transactions) to verify adherence to commercial {% include term.html id="scaling-threshold" text="Scaling Thresholds" %}.

---

## 14. Exclusions

The following items are outside the scope of this provisional framework and belong to dedicated exploratory documents:
1. Detailed anatomy and hardware sampling models of $Q(0)$ and $Q(1)$.
2. Complete grammar for Qexpr.
3. Concrete instruction set for QnIR.
4. Concrete syntax for QnLang.
5. Domain-specific layers ($\mathrm{Qm}$, $\mathrm{Qs}$, $\mathrm{Qphy}$).
6. Advanced optimization primitives ($\mathrm{Q}(\text{reuse})$, $\mathrm{Q}(\text{bypass})$).

---

## 15. Open Items

The following formal specifications remain open for subsequent releases:
1. Canonical binary serialization format for the Universal Envelope.
2. Formal type registry for Qn artifact categories.
3. Concrete receipt schema and cryptographic signature serialization.
4. PQC algorithm agility registry and migration protocol.
5. Automated verification rules for transitions from Sealed to Verified state.
