---
title: Qn Primitive Envelope
description: Governed Qn artifact model, definitions, and provisional operational axioms under SIMEMP and abstraction-layer constraints.
version: 0.1.0
updated: 2026-08-09
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnPrimitive.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnPrimitive.html
status: Review draft / provisional axiom guide — not a finalized specification
depends_on:
  - docs/index.md
  - docs/simemp.md
  - docs/abstractionLayers.md
  - LICENSE.md
---

<link rel="stylesheet" href="assets/css/style.css">

# Qn Primitive Envelope — Governed Qn Artifact Model

## Quick Navigation

Use the published-page links when reading the documentation on GitHub Pages. Use the source-file links when reviewing the committed Markdown files.

| Document | Published page | Source file |
|---|---|---|
| Documentation portal | [index.html](index.html) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| SIMEMP constraints | [simemp.html](simemp.html) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| Abstraction layers | [abstractionLayers.html](abstractionLayers.html) | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) |
| This document | [qnPrimitive.html](qnPrimitive.html) | [docs/qnPrimitive.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qnPrimitive.md) |
| Scaling Source License | Not published from docs/ | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Public repository | [Repository](https://github.com/cdqn5249/cdqn) | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |

> License location note:  
> `LICENSE.md` is located at the root of the `cdqn` repository, not inside `docs/`. Because GitHub Pages is published from `docs/`, the license is linked through the GitHub repository rather than served as a local GitHub Pages file.

---

## Document Metadata

| Field | Value |
|---|---|
| Document Title | Qn Primitive Envelope — Governed Qn Artifact Model |
| Version | 0.1.0 |
| Last Updated | 2026-08-09 |
| Author | Christophe Duy Quang Nguyen |
| License | Scaling Source License (SSL) 1.0 |
| Repository Path | docs/qnPrimitive.md |
| Parent Repository | https://github.com/cdqn5249/cdqn |
| Status | Review draft / provisional axiom guide — not a finalized specification |
| Depends On | docs/index.md, docs/simemp.md, docs/abstractionLayers.md, LICENSE.md |

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Purpose and Scope

This document defines the minimal governed envelope shared by Qn artifacts.

It is a thesis-level document and a provisional axiom guide. It is not a finalized engineering specification, implementation manual, formal proof, or complete mathematical axiomatization.

This document defines:

- what a governed Qn artifact is;
- the universal envelope carried by Qn artifacts;
- typed payload profiles for different artifact categories;
- provisional operational axioms governing existence, transformation, receipts, and exposure;
- lifecycle states;
- lineage rules;
- DCC, metric, and security envelope requirements;
- the relation of the Qn envelope to abstraction layers and future QnLang design.

This document does not define:

- the detailed anatomy of `Q(0)`;
- the detailed anatomy of `Q(1)`;
- the full Qexpr grammar;
- QnLang syntax;
- QnIR execution behavior;
- cdqn distributed chaining rules;
- specific cryptographic algorithms;
- specific PQC algorithms;
- specific binary serialization formats;
- detailed numeric properties of `Q(2)` through `Q(9)`;
- domain abstractions such as Qm, Qs, or Qphy.

The term “Qn object” is used in this document as a governed computational artifact. It is not an object in the classical object-oriented programming sense.

---

## Normative References

The following documents are normative for this draft. If a technical conflict exists, `docs/simemp.md` governs the SIMEMP constraint system. If a structural conflict exists, `docs/abstractionLayers.md` governs the layer framework. If a legal conflict exists, `LICENSE.md` governs.

| Document | Role | Published page | Source file |
|---|---|---|---|
| docs/index.md | Root documentation portal and project stance. | [index.html](index.html) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| docs/simemp.md | SIMEMP constraints: Security, Identity, Metric, Efficiency, Modularity, Portability. | [simemp.html](simemp.html) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| docs/abstractionLayers.md | Abstraction-layer framework, gateways, local-first exposure, and structural principles. | [abstractionLayers.html](abstractionLayers.html) | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) |
| LICENSE.md | Scaling Source License 1.0. Governs licensing, Paternity References, Open Core Invariants, and Scaling Thresholds. | Not published from docs/ | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Definitions

### 1.1 Qn artifact

A Qn artifact is a governed, finite, identifiable, measurable, and security-constrained entity inside the Qn universe.

A Qn artifact may represent:

- a numeric value;
- an operation;
- an expression;
- a logic relation;
- a receipt;
- an identity;
- a capability;
- a DCC profile;
- a metric envelope;
- a security envelope;
- an attestation;
- an exported cdqn commitment;
- SSL lineage metadata;
- a compiled QnIR artifact where governed by the stack.

A Qn artifact is not a passive mathematical value. It is a governed computational entity.

### 1.2 Qn primitive envelope

The Qn primitive envelope is the minimal governance structure required for any Qn artifact to exist, transform, halt, be sealed, or be exposed.

The envelope includes:

- identity;
- type;
- lifecycle state;
- lineage;
- DCC profile;
- metric envelope;
- security envelope;
- exposure class;
- payload reference.

### 1.3 Universal envelope

The universal envelope is the set of envelope fields required by every Qn artifact.

Universal envelope fields are independent of the payload type.

### 1.4 Typed payload profile

A typed payload profile is the type-specific content carried by a Qn artifact.

Examples include:

- numeric payload;
- expression payload;
- operation payload;
- receipt payload;
- identity payload;
- attestation payload;
- capability payload.

Typed payload profiles must be explicitly declared.

### 1.5 Identity

Identity is the governed reference of a Qn artifact.

Identity must be:

- local-first;
- collision-resistant to the extent required by its security level;
- versionable;
- traceable to lineage;
- algorithm-agile.

Identity is not merely a name. It is a governed witness of existence.

### 1.6 Lineage

Lineage is the causal history of a Qn artifact.

Lineage includes:

- parent references;
- causal index;
- birth order;
- derivation references;
- version ancestry;
- transformation receipts.

Lineage must be finite, bounded, and acyclic unless a future bounded recursive rule is explicitly governed.

### 1.7 DCC profile

A DCC profile is an explicit governance profile containing:

- Dependencies: what must exist and be verified first.
- Constraints: finite limits, exclusions, budgets, precision bounds, and halt conditions.
- Capabilities: permitted operations, export rights, delegation rights, and transformation rights.

No DCC profile may be implicit.

### 1.8 Metric envelope

A metric envelope is the finite measurement boundary of a Qn artifact.

It may include:

- size bounds;
- depth bounds;
- complexity degree;
- compute budget;
- storage budget;
- transmission budget;
- precision bounds;
- uncertainty bounds;
- collapse budget;
- verification budget.

No Qn artifact may be unmeasurable.

### 1.9 Security envelope

A security envelope is the governed security boundary of a Qn artifact.

It may include:

- capability limits;
- export rights;
- delegation rights;
- halt authority;
- revocation path;
- identity class;
- trust assumptions;
- cryptographic algorithm identifiers;
- algorithm agility metadata.

Security is defined against finite adversaries, not absolute adversaries.

### 1.10 Lifecycle state

A lifecycle state declares the governance status of a Qn artifact.

Examples include:

- candidate;
- constructed;
- sealed;
- verified;
- rejected;
- halted;
- quarantined;
- exportable;
- exported;
- revoked.

Lifecycle transitions must be explicit.

### 1.11 Receipt

A receipt is a governed record of a transition, fault, halt, rejection, gateway decision, or exposure event.

Receipts are Qn artifacts when they must be audited, sealed, verified, or exported.

### 1.12 Unverified candidate

An unverified candidate is a generated or proposed artifact that has not yet passed construction, evaluation, selection, and sealing.

Unverified candidates are allowed locally only when explicitly classified as unverified.

They may not cross a SIMEMP Gateway.

### 1.13 Numeric payload

A numeric payload is a typed payload profile for numeric Qn artifacts.

A numeric payload may carry:

- symbolic structure;
- value structure;
- zoom `z`;
- remainder `r`;
- dimension `d`;
- precision bounds;
- uncertainty bounds;
- base-independent canonical representation.

Numeric payload fields are not required for non-numeric Qn artifacts.

### 1.14 Exposure class

An exposure class declares whether a Qn artifact is:

- local-only;
- sealable;
- exportable;
- attested;
- revoked;
- quarantined.

Exposure class is governed by local-first and controlled-exposure rules.

---

## 2. Paradigm Neutrality

This document does not mandate an object-oriented programming paradigm.

A Qn artifact is not an OOP object.

A Qn artifact does not imply:

- classes;
- inheritance;
- private mutable state;
- methods;
- constructors;
- destructors;
- dynamic dispatch;
- implicit polymorphism;
- hidden side effects.

Operations on Qn artifacts are explicit governed transformations.

A reified operation is a Qn artifact describing a transformation. It is not a method bound to an object instance.

State changes are explicit, bounded, and receipted where required by lifecycle or gateway rules.

QnLang remains paradigm-neutral at this stage. The current BOC inclination is toward explicit, bounded, data-oriented, receipt-producing computation, but QnLang paradigm selection remains open and must be validated later under SIMEMP and the BOC policy.

---

## 3. Provisional Operational Axioms

The following axioms are provisional operational axioms.

They are not claimed as absolute mathematical truths. They are accepted as structural rules of the Qn universe and must be validated through Computational Consistency in future QnLang and QnIR derivations.

If any axiom conflicts with `LICENSE.md`, the license prevails. If any axiom conflicts with `docs/simemp.md`, the SIMEMP constraint system prevails unless the conflict is resolved by a future validated revision.

### Axiom 1 — Explicit Envelope

No Qn artifact exists unless it carries an explicit universal envelope.

The universal envelope must declare at least:

- identity;
- type;
- lifecycle state;
- lineage;
- DCC profile;
- metric envelope;
- security envelope;
- exposure class;
- payload reference.

### Axiom 2 — Finiteness

Every Qn artifact must have a finite canonical representation and bounded resource envelope.

Actual infinity is not admitted as an executable or storable state.

Infinite concepts may be represented only as finite symbolic abstractions with declared limits.

### Axiom 3 — Typed Payload Separation

Payload-specific fields must belong to typed payload profiles.

Fields specific to numeric expressions, such as zoom `z`, remainder `r`, and dimension `d`, are not mandatory for non-numeric Qn artifacts.

Every payload profile must declare its type.

### Axiom 4 — Local Genesis Dependency

Every governed Qn instance in a local universe must trace lineage, directly or indirectly, to the local `Q(0)` genesis object.

This axiom does not define the detailed anatomy of `Q(0)`. It defines the dependency rule only.

### Axiom 5 — Acyclic Causal Lineage

Every Qn artifact must be born after its dependencies.

Lineage must be finite, bounded, and acyclic unless a future bounded recursive rule is explicitly governed and validated.

No Qn artifact may reference a later-born object as a parent unless permitted by an explicit future constructive rule.

### Axiom 6 — Totality of Governed Operations

Every governed operation involving a Qn artifact must terminate in a defined state.

Defined states may include:

- SUCCESS;
- FAILURE;
- NO_SOLUTION;
- TIMEOUT;
- BUDGET_EXHAUSTED;
- INCONCLUSIVE;
- QUARANTINED;
- PRECISION_INSUFFICIENT;
- EXPRESSION_TOO_COMPLEX;
- REJECTION_BY_GATEWAY;
- DIVISION_BY_ZERO_REJECTED;
- ORDERING_INDETERMINATE_AT_ZOOM.

Silent non-termination is forbidden.

### Axiom 7 — Receipted Boundary Events

Receipts are mandatory for:

- lifecycle transitions;
- gateway crossings;
- sealing events;
- export decisions;
- fault translations;
- halt events;
- rejections;
- revocations;
- capability changes;
- security-relevant state changes.

Pure internal reads and non-mutating local inspections may be metric-tracked without requiring a persistent receipt unless audit policy demands one.

### Axiom 8 — Local-first Exposure

The first identifier component of a Qn artifact is local to the node.

Raw local Qn artifacts and raw local Qexpr structures are not exposed to remote cdqn nodes by default.

Remote exposure requires explicit authorization, bounded attestation, and compliance with the exposure class.

### Axiom 9 — Numeric Precision Explicitness

Every numeric Qn artifact must declare finite precision bounds.

Where applicable, numeric artifacts must declare:

- zoom `z`;
- remainder `r`;
- dimension `d`;
- precision bound;
- uncertainty bound.

No implicit rounding, truncation, or precision loss is permitted.

### Axiom 10 — No Floating-Point Semantics at the Qn Semantic Layer

Floating-point behavior must not appear at the Qn semantic layer.

Lower layers may use binary hardware representations internally, but they must not expose floating-point semantics to governed Qn artifacts.

### Axiom 11 — Bounded Self-description

Qn governance metadata may describe itself, but self-description must be finite, bounded, and versioned.

Infinite recursive metadata towers are forbidden.

A DCC profile, metric envelope, or security envelope may reference a primitive schema or declared profile version without requiring infinite nested envelopes.

### Axiom 12 — License Lineage Where Applicable

SSL lineage metadata and Paternity References must be preserved where applicable under `LICENSE.md`.

This applies particularly to:

- distributed artifacts;
- exported attestations;
- Derivative Works;
- QnIR runtime specification files;
- compiled binary headers;
- public documentation;
- API surfaces where required by the license.

This axiom does not require every local internal Qn artifact to carry a full human-readable Paternity Reference.

### Axiom 13 — Versioning Creates New Artifacts

Versioning a Qn artifact produces a new Qn artifact with lineage referencing the previous version.

Versioning does not mutate the previous artifact.

Previous artifacts remain governed by their original envelope unless explicitly revoked or superseded by a governed receipt.

### Axiom 14 — Generative Candidate Boundary

Unverified candidates may exist locally only if explicitly classified as unverified.

An unverified candidate must declare at least:

- origin;
- budget;
- uncertainty where applicable;
- lifecycle state;
- restriction from gateway crossing.

A candidate becomes a full explicit Qn artifact only after construction, evaluation, selection, and sealing.

---

## 4. Universal Envelope

The universal envelope is required for every Qn artifact.

The following components are conceptual, not a finalized serialization schema.

### 4.1 Identity block

The identity block must declare:

- local node identifier component;
- local causal index;
- artifact type identifier;
- version identifier;
- content commitment;
- lineage reference;
- identity algorithm identifier;
- algorithm agility metadata.

Identity must support collision resistance, traceability, and migration without breaking lineage.

### 4.2 Type block

The type block must declare the artifact category.

Examples include:

- numeric value;
- expression;
- operation;
- logic relation;
- receipt;
- identity;
- capability;
- DCC profile;
- metric envelope;
- security envelope;
- attestation;
- lineage metadata.

Type identifiers must be explicit and versionable.

### 4.3 Lifecycle state block

The lifecycle state block declares the current governance status of the artifact.

Lifecycle states must be explicit.

Lifecycle transitions must be governed by Axiom 7, Receipted Boundary Events.

### 4.4 Lineage block

The lineage block must declare:

- parent references;
- causal index;
- birth order;
- derivation type;
- version ancestry;
- receipt references where applicable.

Lineage must preserve causal consistency.

### 4.5 DCC profile reference

The envelope must reference a DCC profile.

The DCC profile may be embedded or referenced by identity, depending on future schema decisions.

Self-description must remain bounded.

### 4.6 Metric envelope reference

The envelope must reference a metric envelope.

The metric envelope must declare finite bounds appropriate to the artifact type.

### 4.7 Security envelope reference

The envelope must reference a security envelope.

The security envelope must declare capabilities, restrictions, trust assumptions, and export policy.

### 4.8 Exposure class block

The exposure class block declares whether the artifact is local-only, sealable, exportable, attested, revoked, or quarantined.

Exposure class must be explicit.

### 4.9 Payload reference

The envelope must reference a typed payload profile.

The payload must not be interpreted without its type declaration.

---

## 5. Typed Payload Profiles

Typed payload profiles carry content-specific data.

The following profiles are recognized as initial categories. They are not exhaustive.

### 5.1 Numeric payload profile

A numeric payload profile may include:

- symbolic structure;
- value structure;
- zoom `z`;
- remainder `r`;
- dimension `d`;
- precision bound;
- uncertainty bound;
- base-independent canonical reference.

Numeric payload profiles must satisfy Axiom 9, Numeric Precision Explicitness.

### 5.2 Expression payload profile

An expression payload profile may include:

- operator reference;
- operand references;
- expression depth;
- expression size;
- complexity degree;
- symbolic state;
- collapse budget;
- intermediate collapse receipts where applicable.

Expression payloads must remain bounded.

### 5.3 Operation payload profile

An operation payload profile may include:

- domain references;
- codomain references;
- preconditions;
- postconditions;
- budget constraints;
- capability requirements;
- halt behavior;
- receipt behavior.

An operation payload describes a governed transformation. It is not an OOP method.

### 5.4 Receipt payload profile

A receipt payload profile may include:

- terminal state;
- cause reference;
- consumed budget;
- remaining budget;
- parent artifact references;
- timestamp uncertainty where physical time is included;
- causal index;
- signature or commitment reference where applicable.

Receipts must preserve causal ordering.

### 5.5 Identity payload profile

An identity payload profile may include:

- identity class;
- pseudonym;
- delegation chain;
- capability limits;
- revocation path;
- audit references.

Identity classes include at least:

- machine identity;
- human identity;
- AI agent identity.

Machine identity must not imply human identity. AI agent identity must not imply undelegated authority.

### 5.6 Attestation payload profile

An attestation payload profile may include:

- commitment;
- issuer identity;
- identity class;
- capability proof;
- metric summary;
- security policy identifier;
- SSL lineage commitment;
- exposure scope.

Attestations are intended for controlled cdqn exposure.

### 5.7 Capability payload profile

A capability payload profile may include:

- granted rights;
- restrictions;
- delegation limits;
- expiration or renewal rules;
- revocation path;
- metric budget;
- audit requirements.

Capabilities must be explicit and revocable.

---

## 6. Identity and Lineage Rules

### 6.1 Local-first identifier structure

A Qn artifact identifier should be structured as a local-first identifier.

The identifier may include:

- local node pseudonym;
- local causal index;
- artifact type;
- version;
- content commitment.

The exact identifier syntax is not defined by this document.

### 6.2 No global raw identity exposure

Global exposure of raw local identifiers is forbidden by default.

Remote cdqn exposure should use bounded attestations, commitments, capability proofs, and pseudonymous issuer identities.

### 6.3 Versioning

A new version of a Qn artifact is a new Qn artifact.

The new version must reference the previous version through lineage.

The previous version remains part of the causal history.

### 6.4 Algorithm agility

Identity and security mechanisms must support algorithm migration.

No single cryptographic algorithm should be treated as a permanent, non-replaceable dependency.

---

## 7. DCC Profile Rules

### 7.1 Dependencies

Dependencies may include:

- parent artifacts;
- required identities;
- required capabilities;
- required profiles;
- required algorithm identifiers;
- required environmental constraints;
- required lineage commitments.

### 7.2 Constraints

Constraints may include:

- maximum size;
- maximum depth;
- maximum complexity degree;
- maximum compute budget;
- maximum storage budget;
- maximum transmission budget;
- precision bounds;
- uncertainty bounds;
- halt conditions;
- exclusions such as denominator restrictions.

### 7.3 Capabilities

Capabilities may include:

- transformation rights;
- sealing rights;
- export rights;
- delegation rights;
- verification rights;
- revocation rights.

Capabilities must not be implicit.

### 7.4 Bounded DCC self-description

If a DCC profile is itself a Qn artifact, its own governance metadata must remain bounded.

A DCC profile may reference a primitive schema or declared profile version to avoid infinite recursive metadata.

---

## 8. Metric Envelope Rules

A metric envelope must declare finite bounds relevant to the artifact type.

Possible metric fields include:

- object size;
- expression depth;
- complexity degree;
- verification cost;
- transformation cost;
- collapse cost;
- storage cost;
- transmission cost;
- precision bound;
- uncertainty bound;
- budget consumed;
- budget remaining.

Metric envelopes must be explicit.

Metric envelopes must not declare unbounded resources.

Metric envelopes must support the Memory Wall constraint by accounting for data movement where relevant.

---

## 9. Security Envelope Rules

A security envelope must declare the finite adversarial context and the governed protections of the artifact.

Possible security fields include:

- capability limits;
- export rights;
- delegation rights;
- halt authority;
- revocation path;
- identity class;
- trust assumptions;
- algorithm identifiers;
- algorithm agility metadata;
- inner-ring or outer-ring scope.

Security envelopes must respect the dual-ring PQC boundary defined in `docs/abstractionLayers.md`.

Security envelopes must support Post-Quantum Cryptography capability, but no specific PQC algorithm is mandated by this document.

Security envelopes must not introduce anonymous exported entities.

Accountable pseudonymity is permitted.

---

## 10. Lifecycle and Receipts

### 10.1 Lifecycle states

Recognized lifecycle states include:

- candidate;
- constructed;
- sealed;
- verified;
- rejected;
- halted;
- quarantined;
- exportable;
- exported;
- revoked.

Additional lifecycle states may be defined later if they satisfy SIMEMP constraints.

### 10.2 Candidate state

A candidate is a generated or proposed artifact.

A candidate must be explicitly classified as unverified.

A candidate may not cross a SIMEMP Gateway.

### 10.3 Constructed state

A constructed artifact has been formalized into a finite canonical structure.

Construction does not imply verification.

### 10.4 Sealed state

A sealed artifact has been canonicalized, identity-bound, metric-bounded, and security-checked.

A sealed artifact may be referenced by later artifacts.

### 10.5 Verified state

A verified artifact has passed the required validation rules for its context.

Verification is context-dependent and may be local or layered.

### 10.6 Rejected, halted, and quarantined states

Rejected, halted, and quarantined states are terminal or protective states.

They must produce receipts where required by Axiom 7.

### 10.7 Exportable and exported states

An exportable artifact satisfies the requirements for controlled exposure.

An exported artifact has crossed a governed exposure boundary.

Export must preserve local-first rules and SSL lineage requirements where applicable.

### 10.8 Revoked state

A revoked artifact has had its capability, trust, or exposure rights withdrawn.

Revocation must be explicit and receipted.

---

## 11. Relation to Abstraction Layers

This document supports the abstraction-layer framework defined in `docs/abstractionLayers.md`.

### 11.1 Layer 0

Layer 0 remains outside the governed Qn universe.

Raw Layer 0 state does not carry a Qn primitive envelope until mediated by the Layer 0 to Layer 1 gateway.

### 11.2 Layer 0 to Layer 1 gateway

The gateway uses the Qn primitive envelope to construct:

- local genesis receipts;
- device context artifacts;
- entropy health receipts;
- PQC root commitments;
- local `Q(0)` instances.

This document does not define the detailed anatomy of `Q(0)`.

### 11.3 Layer 1

Layer 1 contains the first governed Qn artifacts.

Future documents will define `Q(0)` and `Q(1)` in detail using the envelope defined here.

### 11.4 Higher layers

Higher layers must use the same universal envelope and typed payload model.

Higher layers may introduce new typed payload profiles, but they must not weaken the universal envelope.

---

## 12. Relation to QnLang

This document informs future QnLang design but does not define QnLang syntax.

QnLang should be able to:

- declare Qn artifacts;
- compose Qexpr;
- reference DCC profiles;
- declare metric budgets;
- declare security capabilities;
- express lifecycle transitions;
- emit receipts;
- remain bounded and total by budget;
- avoid implicit defaults;
- avoid floating-point semantics at the Qn semantic layer.

QnLang is not required to be object-oriented.

QnLang may be expression-oriented, data-oriented, capability-oriented, or another paradigm, provided the final design satisfies SIMEMP and the BOC policy.

---

## 13. License Alignment

This document aligns with the Scaling Source License 1.0.

The license governs legal rights and obligations. If this document conflicts with the license, the license prevails.

### 13.1 Paternity Reference

Where applicable under `LICENSE.md`, derivative distributions and governed runtime artifacts must preserve the required Paternity Reference:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

In software, APIs, compiled binary headers, or runtime specification files such as QnIR, the Paternity Reference must be preserved in source comments and metadata headers.

### 13.2 Open Core Invariants

This document must not weaken the SSL Open Core Invariants:

1. Anti-Patent Defense — license termination upon patent assertion against the Author or project community.
2. Non-Scaling Open Access — royalty-free access for non-scaling academic, open-source, personal, research, and small-scale development.

### 13.3 Scale auditing

The Qn primitive envelope supports explicit metric and identity accounting.

This can support native auditing of SSL Scaling Thresholds, including:

- active compute nodes;
- active containers or instances;
- active autonomous agents;
- monthly active users where applicable;
- monthly API or Qn transactions.

Scale auditing must remain explicit, bounded, privacy-preserving, and legally scoped by the SSL.

---

## 14. Exclusions

The following items are intentionally excluded from this document:

1. Detailed anatomy of `Q(0)`.
2. Detailed anatomy of `Q(1)`.
3. Full Qexpr grammar.
4. QnLang syntax.
5. QnIR execution model.
6. cdqn chaining rules.
7. Specific hash algorithms.
8. Specific signature algorithms.
9. Specific PQC algorithms.
10. Specific binary serialization formats.
11. Detailed numeric properties of `Q(2)` through `Q(9)`.
12. Domain abstractions such as Qm, Qs, and Qphy.
13. Detailed rules for Q(reuse).
14. Detailed rules for Q(bypass).
15. Detailed rules for Q(dataStruc).
16. Detailed rules for Q(patterns).

These items belong in future dedicated documents.

---

## 15. Open Items

The following items remain open and must be addressed in future documents:

1. Exact canonical encoding of the universal envelope.
2. Exact identity syntax and local identifier format.
3. Type registry for Qn artifact categories.
4. Typed payload schema registry.
5. DCC profile serialization model.
6. Metric envelope serialization model.
7. Security envelope serialization model.
8. Receipt schema and minimal receipt requirements.
9. Lifecycle transition rules and permitted state graph.
10. Exposure threshold rules and complexity-degree policy.
11. Algorithm agility registry.
12. PQC algorithm registry and migration policy.
13. Verification rules for sealed and verified states.
14. Relationship between Qn primitive envelope and Qexpr grammar.
15. Relationship between Qn primitive envelope and QnLang constructs.
16. Relationship between Qn primitive envelope and QnIR runtime artifacts.

---

## Glossary

**Qn artifact**  
A governed, finite, identifiable, measurable, and security-constrained entity inside the Qn universe.

**Qn primitive envelope**  
The minimal governance structure required for a Qn artifact to exist, transform, halt, be sealed, or be exposed.

**Universal envelope**  
The envelope fields required by every Qn artifact.

**Typed payload profile**  
The type-specific content carried by a Qn artifact.

**Identity block**  
The governed reference structure of a Qn artifact.

**Lineage**  
The causal history and parent references of a Qn artifact.

**DCC profile**  
Dependencies, Constraints, Capabilities. The explicit governance profile of a Qn artifact.

**Metric envelope**  
The finite measurement boundary of a Qn artifact.

**Security envelope**  
The governed security boundary of a Qn artifact.

**Lifecycle state**  
The governance status of a Qn artifact.

**Receipt**  
A governed record of a transition, fault, halt, rejection, gateway decision, or exposure event.

**Unverified candidate**  
A generated or proposed artifact explicitly classified as not yet verified.

**Numeric payload**  
A typed payload profile for numeric Qn artifacts, including zoom, remainder, and dimension where applicable.

**Exposure class**  
The declaration of whether a Qn artifact is local-only, sealable, exportable, attested, revoked, or quarantined.

**Provisional operational axiom**  
A structural rule accepted for the Qn universe pending validation through Computational Consistency.

**Computational Consistency**  
The verification method where internally consistent QnLang computations compiled to QnIR execute successfully and deterministically within declared finite boundaries.

**SIMEMP**  
Security, Identity, Metric, Efficiency, Modularity, Portability. The constitutional constraint system of the Qn and cdqn stack.

**SSL**  
Scaling Source License 1.0. The license governing the project, derivative works, Open Core Invariants, Paternity References, and Scaling Thresholds.

---

## Bottom Navigation

| Destination | Link |
|---|---|
| Documentation portal | [index.html](index.html) |
| SIMEMP constraints | [simemp.html](simemp.html) |
| Abstraction layers | [abstractionLayers.html](abstractionLayers.html) |
| This document | [qnPrimitive.html](qnPrimitive.html) |
| License source | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Public repository | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |
