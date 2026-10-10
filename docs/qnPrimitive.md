---
layout: default
title: Qn Primitive Envelope — Governed Qn Artifact Model
description: Governed Qn artifact model, operational axioms, typed payload profiles, and Category B optimization primitives under SIMEMP constraints.
version: 1.2.1
updated: 2026-10-10
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
  - compute-unit-u
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
  - chronosa
  - qn-workflow
  - qn-rsi
  - q-reuse
  - q-bypass
  - terminal-exactness
  - layer-0
  - layer-1
  - existential-invariant
  - operational-agility
  - boc-policy
---

# Qn Primitive Envelope — Governed Qn Artifact Model

| Field | Specification |
|---|---|
| **Document Title** | Qn Primitive Envelope — Governed Qn Artifact Model |
| **Version** | 1.2.1 |
| **Last Updated** | 2026-10-10 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Primitive Specification — Category A/B Envelope Foundation |

---

## Purpose and Scope

This document formalizes the invariant structural container required for every governed computational entity within the Qn universe: the {% include term.html id="universal-envelope" %}. 

The primitive envelope provides the minimal structural scaffolding necessary to guide artifact construction, algebraic transformation, and empirical execution across [`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}). Governed by {% include term.html id="dependencies-determinism" %}, the model eliminates unmeasured ambient state while preserving architectural elasticity against zero-day vulnerabilities through {% include term.html id="structural-indirection" %}.

---

## Normative References

The following documents define the normative, physical, and legal constraints governing this framework. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/q2_q9.md` | Single-digit secondary DCC anchors and single-digit spectrum | [q2_q9.html]({{ '/q2_q9.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG specification | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Definitions

### 1.1. Qn Artifact
A {% include term.html id="qn" %} artifact is a governed, finite, identifiable, measurable, and security-constrained computational entity. An artifact is not a passive mathematical scalar; it is an immutable state node embedded in a local {% include term.html id="causal-arrow" %}.

### 1.2. Qn Primitive Envelope
The minimal governance boundary required for any computational entity within the {% include term.html id="licensed-work" %} to exist, undergo mutation, achieve sealing, or traverse a {% include term.html id="simemp-gateway" %}, governed under Tier 1 {% include term.html id="existential-invariant" text="Existential Invariants" %} and optimized via Tier 2 {% include term.html id="operational-agility" text="Operational Agilities" %} within the {% include term.html id="simemp" %} framework.

### 1.3. Universal Envelope
The invariant set of structural blocks shared across all artifact types, parameterized via {% include term.html id="structural-indirection" %}:

$$\mathcal{E} = \langle \text{Identity}, \, \text{Type}, \, \text{State}, \, \text{Lineage}, \, \text{DCC}, \, \text{Metric}, \, \text{Security}, \, \text{Exposure}, \, \text{PayloadRef} \rangle$$

### 1.4. Typed Payload Profile
The specialized content block carried by an artifact (numeric quantities, symbolic expression graphs, pure morphisms, state receipts, identity assertions, capability grants, optimization pointers, or composite workflows). External user data remains a sovereign, protocol-blind {% include term.html id="payload" %}.

### 1.5. Identity
The governed witness of an artifact's existence. Identity is {% include term.html id="local-first" %}, collision-resistant, versionable, traceable along causal lineage, and cryptographic-algorithm agile.

### 1.6. Lineage
The directed acyclic causal history of an artifact, recording parent commitment hashes, causal birth index, and transformation receipt references.

### 1.7. DCC Profile
The machine-readable contract defining **Dependencies**, **Constraints**, and **Capabilities**. It serves as an abstract contract rather than a hardcoded implementation binding via {% include term.html id="dcc-profile" text="DCC Profiles" %}.

### 1.8. Metric Envelope
The finite measurement boundary declaring resource limits on storage, expression graph depth, precision bounds, and execution budgets via the {% include term.html id="metric-envelope" %}.

### 1.9. Security Envelope
The governed protection boundary declaring authority limits, delegation proofs, Post-Quantum Cryptography (PQC) algorithm identifiers, and swappable revocation paths via the {% include term.html id="security-envelope" %}.

### 1.10. Lifecycle State
The discrete operational status of an artifact: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, or revoked via the {% include term.html id="lifecycle-state" %}.

### 1.11. Receipt
A bounded, immutable record of a state transition, fault translation, or boundary traversal, acting as a dissipative {% include term.html id="receipt" %} exporting operational entropy.

### 1.12. Unverified Candidate
A proposed computational structure generated via heuristic or stochastic processes that has not passed gateway verification. Candidates cannot cross a {% include term.html id="simemp-gateway" %}.

### 1.13. Numeric Payload
The specialized profile for numeric entities, explicitly encapsulating {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}.

### 1.14. Exposure Class
The classification governing artifact visibility: local-only, sealable, exportable, attested, quarantined, or revoked.

### 1.15. Q(reuse) Optimization Morphism
A Category B primitive referencing existing immutable content commitment hashes within the local causal DAG, eliminating redundant physical memory allocation across the Memory Wall at zero marginal work ($W = 0$) via {% include term.html id="q-reuse" text="Q(reuse)" %}.

### 1.16. Q(bypass) Optimization Morphism
A Category B primitive allowing a {% include term.html id="simemp-gateway" %} to short-circuit execution steps upon verifying algebraic identity preconditions, emitting a zero-cost bypass receipt without consuming compute budgets via {% include term.html id="q-bypass" text="Q(bypass)" %}.

---

## 2. Paradigm Neutrality

This specification does not mandate an object-oriented programming (OOP) paradigm. A Qn artifact is not an OOP object.

The model explicitly rejects:
- Hierarchical class inheritance and dynamic dispatch tables.
- Hidden mutable state and unmetered side effects.
- Implicit type coercions and ambient runtime environments.

Transformations are explicit governed morphisms. State changes yield new immutable artifacts indexed monotonically along the local {% include term.html id="causal-arrow" %}.

---

## 3. Provisional Operational Axioms

The Qn operational model is governed by 14 operational axioms. These axioms represent **non-negotiable search rules** verified empirically via {% include term.html id="computational-consistency" %}.

### Axiom 1 — Explicit Envelope
No Qn artifact exists within the governed universe unless it carries an explicit {% include term.html id="universal-envelope" %}:

$$\forall \alpha \in \mathcal{Q}, \quad \exists ! \, \mathcal{E}(\alpha)$$

### Axiom 2 — Finiteness
Every Qn artifact must possess a finite canonical representation and a bounded resource envelope. Actual infinities are physically non-realizable and excluded from execution:

$$\forall \alpha \in \mathcal{Q}, \quad \mathrm{Size}(\alpha) \lt \infty \quad \wedge \quad \mathrm{Budget}(\alpha) \lt \infty$$

### Axiom 3 — Typed Payload Separation
Fields specific to numeric representations ({% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}) belong strictly to typed payload profiles and must not be made mandatory for non-numeric entities. Sovereign user data remains an uninspected {% include term.html id="payload" %}.

### Axiom 4 — Local Genesis Dependency
Every governed Qn artifact within a local universe must trace its lineage directly or transitively to the local genesis origin {% include term.html id="q0" text="Q(0)" %}:

$$\forall \alpha \in \mathcal{Q}_N, \quad Q(0)_N \in \mathrm{Lineage}(\alpha)$$

Local artifacts form single-rooted directed trees originating at physical silicon root $Q(0)_N$. Higher-order network entities operating across the Outer Ring (such as {% include term.html id="chronosa" %}) are topological colimits (sheaf gluings) over acyclic forests of mutually verified local Axiom-4 trees.

### Axiom 5 — Acyclic Causal Lineage
Lineage graphs are strictly acyclic, governed by {% include term.html id="dependencies-determinism" %}. Every artifact must be born after its causal dependencies along the {% include term.html id="causal-arrow" %}:

$$\mathrm{Index}(\mathrm{Parent}(\alpha)) \lt \mathrm{Index}(\alpha)$$

### Axiom 6 — Totality of Governed Operations
Every governed operation must terminate in a declared, finite terminal state via dissipative {% include term.html id="metric-exhaustion" %} within its declared budget:
`SUCCESS`, `FAILURE`, `NO_SOLUTION`, `TIMEOUT`, `BUDGET_EXHAUSTED`, `INCONCLUSIVE`, `QUARANTINED`, `PRECISION_INSUFFICIENT`, `EXPRESSION_TOO_COMPLEX`, `REJECTION_BY_GATEWAY`, `DIVISION_BY_ZERO_REJECTED`, or `ORDERING_INDETERMINATE_AT_ZOOM`. Silent non-termination is prohibited.

### Axiom 7 — Receipted Boundary Events
Receipts are mandatory for lifecycle state transitions, gateway crossings, sealing events, export decisions, fault translations, and capability invocations. Receipts serve as verifiable {% include term.html id="receipt" text="receipts" %} exporting operational entropy.

### Axiom 8 — Local-first Exposure
Artifact identifiers are rooted locally. Raw local Qn artifacts are never exposed directly to remote nodes across {% include term.html id="cdqn" %} without explicit authorization and bounded public attestation under {% include term.html id="local-first" %} rules.

### Axiom 9 — Numeric Precision Explicitness
Every numeric Qn artifact must declare finite precision bounds, explicit {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, and {% include term.html id="dimension-d" text="Dimension d" %}. Implicit rounding and truncation are prohibited:

$$\text{NumericValue} = \langle z, \, r, \, d \rangle$$

### Axiom 10 — No Floating-Point Semantics
IEEE 754 floating-point representations are forbidden at the Qn semantic layer. Non-deterministic mantissa rounding and platform-dependent approximations are rejected.

### Axiom 11 — Bounded Self-Description
Governance metadata may describe its own schema, but self-description depth must remain finite, bounded, and version-anchored to prevent infinite recursive reflection towers.

### Axiom 12 — License Lineage Where Applicable
Lineage metadata and the canonical {% include term.html id="paternity-reference" %} must be preserved in distributed artifacts, exported attestations, compiled {% include term.html id="qnir" %} binaries, and public APIs pursuant to the {% include term.html id="ssl" %}. Encapsulating a Payload does not convert it into a {% include term.html id="derivative-work" %}.

### Axiom 13 — Versioning Creates New Artifacts
Versioning an artifact produces a new immutable artifact with an advanced causal index referencing the predecessor. Mutation in place is prohibited. Successor workflows derived through {% include term.html id="qn-rsi" text="Qn(rsi)" %} do not overwrite existing execution paths; they are emitted as distinct causal versions.

### Axiom 14 — Generative Candidate Boundary
Unverified candidates proposed by stochastic, generative, or neural processes can exist locally only if explicitly tagged as unverified. Candidates cannot cross a {% include term.html id="simemp-gateway" %}.

---

## 4. Universal Envelope Structure

The Universal Envelope $\mathcal{E}$ is structured using {% include term.html id="structural-indirection" %}, ensuring that cryptographic suites or binary layouts can be upgraded without invalidating existing causal DAGs.

```
                    UNIVERSAL ENVELOPE STRUCTURE
 ┌─────────────────────────────────────────────────────────────────┐
 │ 1. IDENTITY BLOCK      : NodeID, CausalIndex, Type, Version, Hash │
 ├─────────────────────────────────────────────────────────────────┤
 │ 2. TYPE BLOCK          : Primitive, Morphism, Workflow, Receipt │
 ├─────────────────────────────────────────────────────────────────┤
 │ 3. STATE BLOCK         : Candidate, Sealed, Verified, Quarantined│
 ├─────────────────────────────────────────────────────────────────┤
 │ 4. LINEAGE BLOCK       : ParentCommitmentHashes, BirthOrder      │
 ├─────────────────────────────────────────────────────────────────┤
 │ 5. DCC PROFILE REF     : Dependencies, Constraints, Capabilities │
 ├─────────────────────────────────────────────────────────────────┤
 │ 6. METRIC ENVELOPE     : MemoryCeiling, DepthLimit, ComputeBudget│
 ├─────────────────────────────────────────────────────────────────┤
 │ 7. SECURITY ENVELOPE   : SecurityRing, PQCAlgoID, RevocationPath │
 ├─────────────────────────────────────────────────────────────────┤
 │ 8. EXPOSURE BLOCK      : LocalOnly, Exportable, Attested         │
 ├─────────────────────────────────────────────────────────────────┤
 │ 9. PAYLOAD REFERENCE   : Typed pointer to Payload Profile        │
 └─────────────────────────────────────────────────────────────────┘
```

### 4.1. Identity Block
Declares local node identifier, local causal index, artifact type identifier, version identifier, content commitment hash, and cryptographic agility metadata.

### 4.2. Type Block
Declares the artifact category (numeric scalar, expression DAG, morphism, relational assertion, receipt, identity, capability, DCC contract, optimization pointer, or composite workflow).

### 4.3. Lifecycle State Block
Declares current governance status conforming to Axiom 7 and the {% include term.html id="lifecycle-state" %}.

### 4.4. Lineage Block
Declares parent artifact references, local causal index, birth order, derivation type, and transformation receipt references.

### 4.5. DCC Profile Reference
Machine-readable reference to an abstract capability contract declaring prerequisites, operational ceilings, and permitted transformations via the {% include term.html id="dcc-profile" %}.

### 4.6. Metric Envelope Reference
Declares finite operational bounds (storage footprint, expression graph depth, recursion limits, and execution budgets) via the {% include term.html id="metric-envelope" %}.

### 4.7. Security Envelope Reference
Declares authority scope, delegation limits, trust assumptions, PQC algorithm identifiers, and swappable revocation endpoints via the {% include term.html id="security-envelope" %}.

### 4.8. Exposure Class Block
Declares visibility scope: local-only, sealable, exportable, attested, quarantined, or revoked.

### 4.9. Payload Reference
A typed, content-addressed pointer to the associated {% include term.html id="payload" %} profile.

---

## 5. Typed Payload Profiles

### 5.1. Numeric Payload Profile
Carries {% include term.html id="zoom-z" text="Zoom z" %}, {% include term.html id="remainder-r" text="Remainder r" %}, {% include term.html id="dimension-d" text="Dimension d" %}, precision boundaries, and base-independent scale lattice representations ([`docs/qm.md`]({{ '/qm.html' | relative_url }}) §3).

### 5.2. Expression Payload Profile (Content-Addressed Term DAG)
Under the {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %} ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §7), symbolic expressions ({% include term.html id="qexpr" %}) are represented as **Content-Addressed Directed Acyclic Graphs (Term DAGs)** with cryptographic hash-consing ([`docs/qexpr.md`]({{ '/qexpr.html' | relative_url }})):

$$\text{QexprNode} = \langle \text{OpCode}, \, \mathcal{H}(\text{LeftChild}), \, \mathcal{H}(\text{RightChild}), \, z, \, r, \, d \rangle$$

- **Deduplication:** Identical sub-expressions share identical commitment hashes, eliminating redundant leaf allocation across the Memory Wall.
- **{% include term.html id="complexity-degree" text="Complexity Bound" %}:** Declares strict graph depth ceilings, maximum node counts, and compile-time collapse budgets.
- **Thermodynamic Invariance:** Reversible normalization within the DAG incurs zero Landauer dissipation ($W = 0$).

### 5.3. Operation Payload Profile
Carries domain-to-codomain morphism signatures, formal preconditions, postconditions, metric consumption rates, and receipt emission behavior.

### 5.4. Receipt Payload Profile
Carries terminal execution state, consumed resource budgets ({% include term.html id="metric-exhaustion" %}), causal birth index, parent commitment references, and gateway signatures on each emitted {% include term.html id="receipt" %}.

### 5.5. Identity Payload Profile
Carries {% include term.html id="identity-class" text="Identity Class" %} (Machine Identity, Human Identity, AI Agent Identity), pseudonyms, capability bounds, and revocation endpoints, bounded by {% include term.html id="existential-invariant" text="Identity Invariants" %}.

### 5.6. Attestation Payload Profile
Carries public commitments, issuer pseudonyms, capability proofs, metric summaries, and SSL lineage assertions for network exposure across {% include term.html id="cdqn" %}.

### 5.7. Capability Payload Profile
Carries granular execution rights, operational constraints, delegation depths, and expiration bounds.

### 5.8. Workflow Payload Profile
Carries a directed acyclic graph of governed morphisms, input state prerequisites, execution checkpoints, intermediate collapse budgets, and terminal receipt criteria. Underpins composite state workflows ({% include term.html id="qn-workflow" text="Qn(workflow)" %}) and recursive self-optimization loops ({% include term.html id="qn-rsi" text="Qn(rsi)" %}).

### 5.9. Optimization Payload Profile (Category B Primitives)
Governs explicit sub-graph reuse and gateway short-circuiting:

#### Q(reuse) Profile
Carries an immutable target content hash $\mathcal{H}_{\mathrm{target}}$, a DAG node reference, and an execution capability proof for {% include term.html id="q-reuse" text="Q(reuse)" %}:
- Proves that the requested state already exists in the local causal history.
- Emits `RECEIPT_REUSE_WITNESS`, registering zero marginal memory bit allocation across the Memory Wall.

#### Q(bypass) Profile
Carries an algebraic identity proof hash (e.g., proof of {% include term.html id="terminal-exactness" %} $r = Q(0)$, unit scaling $\times Q(1)$, or inverse rotor annihilation $R R^{\dagger} = Q(1)$) for {% include term.html id="q-bypass" text="Q(bypass)" %}:
- Proves that source and target states are algebraically isomorphic.
- Emits `RECEIPT_BYPASS_IDENTITY`, short-circuiting gateway execution stages at zero compute cost.

---

## 6. Identity and Lineage Rules

### 6.1. Local-first Identifier Structure
An artifact identifier is rooted strictly in its {% include term.html id="local-first" %} genesis:

$$\mathrm{ID} = \langle \text{NodePseudonym}, \, \text{CausalIndex}, \, \text{Type}, \, \text{Version}, \, \text{Commitment} \rangle$$

### 6.2. No Global Raw Identity Exposure
Raw local node identifiers are prohibited from direct network exposure. Network interaction across {% include term.html id="cdqn" %} uses accountable pseudonyms, cryptographic commitments, and zero-knowledge capability proofs.

### 6.3. Versioning
Every modification produces a distinct artifact with an advanced causal index. Predecessors remain immutable components of historical lineage.

### 6.4. Algorithm Agility
Cryptographic primitives must support seamless migration via {% include term.html id="structural-indirection" %}. No single signature suite or hash function is treated as a permanent dependency.

---

## 7. DCC Profile Rules

### 7.1. Dependencies
Explicitly declares required parent artifacts, signing identities, capability grants, and physical context prerequisites under {% include term.html id="dependencies-determinism" %}. Dependencies are evaluated as abstract contracts via {% include term.html id="structural-indirection" %} to allow modular implementation swaps.

### 7.2. Constraints
Declares hard operational ceilings: memory allocation bounds, recursion limits, {% include term.html id="complexity-degree" text="complexity degree" %} ceilings, precision limits, and excluded states (e.g., denominator zero exclusion).

### 7.3. Capabilities
Declares permitted transformations: morphism rights, sealing authorization, network export authorization, and delegation scopes.

### 7.4. Bounded DCC Self-Description
Governance profiles referencing their own schemas must anchor to fixed schema versions to prevent infinite self-descriptive recursion towers.

---

## 8. Metric Envelope Rules

Every {% include term.html id="metric-envelope" %} must declare finite, measurable boundaries:
- **Footprint Bounds:** Storage allocation size, expression DAG depth ceilings, and {% include term.html id="complexity-degree" text="complexity degree" %} bounds.
- **Execution Bounds:** Step ceilings, transformation budgets, and compilation collapse allocations.
- **Numeric Bounds:** Scale zoom ceilings ($z \le z_{\max}$) and uncertainty boundaries.
- **Memory Wall Accounting:** Explicit tracking of data movement, cache locality, and bus transmission dissipation. Reversible DAG reuse ({% include term.html id="q-reuse" text="Q(reuse)" %}) incurs zero marginal Landauer cost.

---

## 9. Security Envelope Rules

Every {% include term.html id="security-envelope" %} must define protection parameters against computationally bounded adversaries:
- Inner-ring and outer-ring scoping conforming to the {% include term.html id="dual-ring-pqc-boundary" %}.
- Swappable revocation paths and emergency halt authorities designed for zero-day patchability.
- Post-Quantum Cryptography (PQC) algorithm identifiers with explicit migration semantics.
- Accountable pseudonymity; anonymous entities are strictly forbidden from traversing network boundaries.

---

## 10. Lifecycle and Receipts

```
                      ARTIFACT LIFECYCLE FLOW
 [Candidate] ──► [Constructed] ──► [Sealed] ──► [Verified] ──► [Exportable] ──► [Exported]
      │               │              │             │
      ▼               ▼              ▼             ▼
  [Rejected]      [Halted]     [Quarantined]   [Revoked]
 (All fault and terminal states emit immutable dissipative Receipts)
```

### 10.1. Lifecycle States
Standard recognized operational states conforming to the {% include term.html id="lifecycle-state" %}: candidate, constructed, sealed, verified, rejected, halted, quarantined, exportable, exported, revoked.

### 10.2. Candidate State
A proposed stochastic or heuristic structure; classified strictly as unverified; prohibited from crossing a {% include term.html id="simemp-gateway" %}.

### 10.3. Constructed State
Formalized into canonical envelope syntax; awaiting constraint evaluation.

### 10.4. Sealed State
Identity-committed, metric-bounded, security-checked, and immutable.

### 10.5. Verified State
Validated against explicit formal proof invariants or test suites by an authorized verifier.

### 10.6. Rejected, Halted, and Quarantined States
Protective terminal states emitting mandatory failure receipts exporting operational entropy.

### 10.7. Exportable and Exported States
Certified by the {% include term.html id="simemp-gateway" %} for controlled exposure across the {% include term.html id="cdqn" %} network via the Exposure Functor.

### 10.8. Revoked State
Invalidated via an explicit, signed revocation receipt that permanently seals a compromised branch and migrates lineage.

---

## 11. Relation to Abstraction Layers

### 11.1. Layer 0 (Physical Substrate)
Raw physical hardware noise and clock signals in {% include term.html id="layer-0" %} carry no primitive envelope until processed across the onboarding gateway ([`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }}) §2).

### 11.2. Layer 0 to Layer 1 Gateway
Harvests physical entropy, generates Inner-Ring PQC keys, and constructs local origin {% include term.html id="q0" text="Q(0)" %} and first unit {% include term.html id="q1" text="Q(1)" %} across the {% include term.html id="simemp-gateway" %}.

### 11.3. Layer 1 (Node Genesis)
Houses foundational Category A primitives in {% include term.html id="layer-1" %} ({% include term.html id="q0" text="Q(0)" %}, {% include term.html id="q1" text="Q(1)" %}, single digits $Q(2)\dots Q(9)$, the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}, and genesis axis $d_1$).

### 11.4. Layer 2 (Governed Morphisms)
Hosts elementary arithmetic morphisms ($+, -, \times, \div$), spatial reflection involutions ($\mathcal{I}_d$), and Category B optimization morphisms ({% include term.html id="q-reuse" text="Q(reuse)" %}, {% include term.html id="q-bypass" text="Q(bypass)" %}).

### 11.5. Layer 3 (Symbolic Composition)
Hosts Content-Addressed Expression DAGs ({% include term.html id="qexpr" %}), multi-axial Clifford geometric frames, and compile-time reduction engines.

---

## 12. Relation to QnLang and QnIR

The primitive envelope provides the formal execution target for the programming substrate:
- **{% include term.html id="qnlang" %}:** Authors computations declaring explicit DCC profiles, metric budgets, and payload profiles. Prohibits implicit defaults and floating-point semantics under the {% include term.html id="no-implicit-rule" %}.
- **{% include term.html id="qnir" %}:** Intermediate representation flattening Content-Addressed Expression DAGs into linear, bounded instruction streams for deterministic hardware evaluation.
- **Compile-Time Collapse:** Symbolic expression DAGs must collapse deterministically to concrete canonical leaf tuples prior to terminal {% include term.html id="receipt" %} emission.

---

## 13. License Alignment

This specification strictly enforces the legal provisions of the {% include term.html id="ssl" %}:

### 13.1. Paternity Reference
All derivative implementations, hardware drivers, runtime specifications, and APIs derived from the {% include term.html id="licensed-work" %} must embed the canonical {% include term.html id="paternity-reference" %}:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 13.2. Open Core Invariants
1. **Anti-Patent Defense:** Commercial or {% include term.html id="derivative-work" text="Derivative Works" %} licenses terminate automatically upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free access is guaranteed for non-commercial, academic, and sub-threshold usage under the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}.

### 13.3. Scale Auditing
The metric and identity envelopes supply verifiable counters (active compute instances, containers, autonomous agents, and monthly API transactions) to verify adherence to commercial scaling thresholds ([`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) §1.5).

---

## 14. Exclusions

The following items are outside the scope of this foundational primitive envelope and belong to dedicated specifications:
1. Detailed physical entropy sampling and NIST SP 800-90B health test implementations (specified in [`docs/q0_q1.md`]({{ '/q0_q1.html' | relative_url }})).
2. High-level grammar and syntax rules for QnLang (Milestone 2).
3. Virtual machine instruction set architecture and register schedules for QnIR (Milestone 2).
4. Domain-specific categorical dictionaries for Quang Semantics ($\mathrm{Qs}$) and physical simulation equations for Quang Physics ($\mathrm{Qphy}$).
5. Concrete multi-party threshold signature ceremony protocols across distributed swarms (specified in [`docs/chronosa.md`]({{ '/chronosa.html' | relative_url }})).

---

## 15. Open Items

The following formal specifications remain open for subsequent releases:
1. Canonical binary serialization format (e.g., deterministic CBOR or custom packed Qn-binary) for the Universal Envelope.
2. Formal schema definition for Post-Quantum Cryptography algorithm agility migration certificates.
3. Automated verification checker specification for state transitions from Sealed to Verified.
4. Execution mechanics and termination proofs for higher-order recursive self-optimization workflows ({% include term.html id="qn-rsi" text="Qn(rsi)" %}).
