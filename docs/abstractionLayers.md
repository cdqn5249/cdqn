---
layout: default
title: Abstraction Layers
description: Structural thesis defining the abstraction-layer framework for the Qn and cdqn stack under SIMEMP constraints.
version: 1.0.0
updated: 2026-09-05
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/abstractionLayers.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /abstractionLayers.html
terms_used:
  - abstraction-layer
  - simemp
  - simemp-gateway
  - layer-0
  - layer-1
  - q0
  - q1
  - qn
  - cdqn
  - causal-arrow
  - complexity-degree
  - no-implicit-rule
  - dcc-profile
  - universal-envelope
  - metric-envelope
  - security-envelope
  - receipt
  - lifecycle-state
  - open-core-invariant
  - paternity-reference
  - ssl
  - computational-consistency
  - dependencies-determinism
  - structural-indirection
  - payload
  - licensed-work
  - metric-exhaustion
---

# Abstraction Layers — Structural Thesis for the Qn and cdqn Stack

| Field | Specification |
|---|---|
| **Document Title** | Abstraction Layers — Structural Thesis for the Qn and cdqn Stack |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-09-05 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Provisional Structural Thesis / Exploratory Layer Architecture |

---

## 1. Foundational Position

The Qn computational universe is established upon a governing operational conjecture:

> A number system can abstract any computable phenomenon within a computational environment if, and only if, that abstraction is strictly governed by finite physical realities and constrained by the {% include term.html id="simemp" %} framework.

A {% include term.html id="qn" %} artifact is not an ungrounded mathematical abstraction. It is a governed, finite, identifiable, measurable, and security-constrained computational entity.

The architecture rejects actual infinities, unconstrained sets, and unmetered execution. Layer boundaries represent **provisional structural hypotheses** governed by {% include term.html id="dependencies-determinism" %} ($H(S_t \mid \mathcal{D}(S_t)) = 0$) and structured with {% include term.html id="structural-indirection" %} to prevent premature calcification. Every abstraction must be constructed, bounded, identified, measured, and verified within explicit resource bounds via {% include term.html id="computational-consistency" %}:

$$\text{Validity}(\mathcal{A}) \iff \left( \text{Consistent}(\mathcal{A}) \wedge \forall p \in \text{QnIR}(\mathcal{A}), \, \text{ExecutesDeterministicallyWithinBounds}(p) \right)$$

---

## 2. Layer Model

{% include term.html id="abstraction-layer" text="Abstraction layers" %} define structural boundaries governing artifact genesis, transformation, and transmission across both local and distributed boundaries.

```
+-------------------------------------------------------------------------------+
| Layer 1: Node Genesis Layer (Q(0), Q(1), Abstract Compute Unit U, Axis d1)    |
+---------------------------------------^---------------------------------------+
                                        |  SIMEMP Gateway
+---------------------------------------+---------------------------------------+
| Layer 0: Physical Substrate (Commodity Hardware, Memory Wall, Entropy Sources)|
+-------------------------------------------------------------------------------+
```

### 2.1. Layer 0 — Commodity Physical Substrate

{% include term.html id="layer-0" text="Layer 0" %} is the physical, hardware-agnostic execution substrate:

- Commodity processors (CPU, GPU, accelerators), memory hierarchies, storage media, and network interfaces.
- Physical boundaries: finite memory, clock latency, thermal dissipation limits, and the Memory Wall.
- Raw entropy sources and operating system telemetry.

Layer 0 remains outside the governed Qn universe. It constitutes the **Root of Finiteness**. Raw substrate states, hardware counters, and noise channels become governed Qn artifacts only upon formal mediation by a gateway.

### 2.2. Layer 0 to Layer 1 Gateway — Node Onboarding and Fault Translation

The {% include term.html id="simemp-gateway" %} bridging Layer 0 and Layer 1 enforces two mandatory functions:

#### Node Onboarding
During initialization, a node instantiates its local genesis artifact through:
1. Bounded entropy sampling and statistical health validation.
2. Device execution context collection.
3. {% include term.html id="dcc-profile" %} initialization with abstract capability interfaces.
4. Post-Quantum Cryptography (PQC) root key generation or commitment.
5. Construction of local genesis [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0).
6. Emission of a signed genesis {% include term.html id="receipt" %}.

Terminal onboarding states: `SUCCESS`, `ENTROPY_INSUFFICIENT`, `ENTROPY_SOURCE_FAULT`, `HARDWARE_FAULT`, `TIMEOUT`, `PQC_KEYGEN_FAILURE`, `GENESIS_RECEIPT_FAILURE`.

#### Fault Translation
Hardware and host execution faults are translated into bounded receipts:
- Out-of-memory events, I/O timeouts, network partition states, thermal throttling interrupts, and entropy failures.

No physical fault may cross the gateway as an unmeasured, implicit state.

### 2.3. Layer 1 — Node Genesis Layer

{% include term.html id="layer-1" text="Layer 1" %} is the primary governed layer:

- [`Q(0)`]({{ '/glossary.html' | relative_url }}#q0): Local genesis artifact, causal origin zero, empty birth context.
- [`Q(1)`]({{ '/glossary.html' | relative_url }}#q1): First unit artifact, unity measure, and baseline reference for the abstract compute unit $U$.
- Abstract compute unit $U$: Minimal governed state transition from $Q(0)$ to $Q(1)$.
- Sign polarity: Induced directed relation between $Q(0)$ and $Q(1)$.
- Elementary arithmetic morphisms and the first dimensional axis $d_1$.

$Q(0)$ is strictly local to its node:

$$\forall A \neq B \implies Q(0)_A \neq Q(0)_B$$

No universal global zero is admitted. $Q(1)$ maintains a deterministic numeric and metric value across all nodes; local entropy may parameterize its identity witness but cannot mutate its unit value.

### 2.4. Higher Layers

Higher abstraction layers are derived sequentially from Layer 1. Every layer boundary must enforce:
- Bounded {% include term.html id="universal-envelope" text="Universal Envelopes" %}.
- Explicit {% include term.html id="dcc-profile" text="DCC Profiles" %} with structural indirection.
- Strict {% include term.html id="simemp-gateway" text="Gateway" %} validation.
- Preservation of the {% include term.html id="causal-arrow" %}.
- Cryptographic preservation of the {% include term.html id="ssl" %} lineage.

---

## 3. Structural Principles

### 3.1. All Governed Artifacts are Qn

Within the {% include term.html id="licensed-work" %}, every accepted artifact is a {% include term.html id="qn" %} computational entity or is reducible to one: numeric values, operators, expression trees, logical propositions, receipts, identities, capabilities, and compiled runtime objects.

Raw user data, external applications, and creative works remain sovereign, uninspected {% include term.html id="payload" text="Payloads" %} separated from the technical substrate by an epistemic and legal safe-harbor air gap.

### 3.2. The No-Implicit Rule

> In the Qn universe, no implicit entity, behavior, assumption, default, interpretation, or convention may cross a {% include term.html id="simemp-gateway" %}. Only explicit Qn objects may pass between layers.

Implicitness violates Dependencies Determinism ($H(S_t \mid \mathcal{D}') > 0$) and the Tier 1 Existential Invariants:
- **Identity:** Implicit entities lack cryptographic identity.
- **Metric:** Implicit entities cannot be measured.
- **Security:** Implicit parameters represent unverified attack vectors.

Implicit type coercion, inferred operator precedence, assumed units, and unmeasured error states are prohibited.

### 3.3. Local-first and Controlled Exposure

Artifacts are instantiated and indexed locally. Remote nodes in the network never inspect raw local artifacts. Network exposure is restricted to:
- Bounded attestations and cryptographic commitments.
- Exported higher-order {% include term.html id="qexpr" %} commitments.
- Capability delegation certificates and metric summaries.
- Accountable pseudonymous identities.

### 3.4. DCC Profiles Everywhere

Every layer, object, morphism, and exported attestation must expose an explicit {% include term.html id="dcc-profile" %} utilizing structural indirection:
- **Dependencies:** Abstract capability contracts, parent hashes, and cryptographic anchors.
- **Constraints:** Hard ceilings on memory, execution steps, recursion depth, and precision bounds.
- **Capabilities:** Permitted operations, transformation rights, and export permissions.

### 3.5. Causal Arrow and Birth Order

Artifact genesis is indexed along a strictly monotonic {% include term.html id="causal-arrow" %}:

$$Q(0) \prec Q(1) \prec \text{First Operations} \prec d_1 \prec \text{Higher } \text{Qexpr}$$

No artifact may declare a parent or dependency born later in the causal sequence.

### 3.6. Totality by Budget

Every governed operation must terminate within its declared metric budget as a dissipative thermodynamic step ({% include term.html id="metric-exhaustion" %}), resolving to an explicit state:
`SUCCESS`, `FAILURE`, `NO_SOLUTION`, `TIMEOUT`, `BUDGET_EXHAUSTED`, `INCONCLUSIVE`, `QUARANTINED`, `PRECISION_INSUFFICIENT`, `EXPRESSION_TOO_COMPLEX`, `REJECTION_BY_GATEWAY`. Silent non-termination is prohibited.

---

## 4. Set and Category Abstraction

The structural framework is formally modeled via constructive set theory and locally finite category theory.

### 4.1. Local Qn Set

For node $N$, the local universe is a finite constructive set $\mathcal{Q}_N$:
- Genesis member: $Q(0)_N \in \mathcal{Q}_N$.
- Set membership requires an explicit identity, metric envelope, security envelope, and lineage witness.
- No universal set of all Qn artifacts exists.

### 4.2. Local Qn Category

Each node defines a locally finite category $\mathbf{C}_N$:
- **Objects:** Governed artifacts $\alpha \in \mathcal{Q}_N$.
- **Morphisms:** Governed transformations $f: \alpha \to \beta$ conforming to DCC constraints.
- **Identities:** Explicit identity morphisms $\mathrm{id}_\alpha$.

### 4.3. Functors and Gateways

Inter-layer transitions are modeled as functors constrained by gateway validation:

$$\mathcal{F}: \mathbf{C}_L \to \mathbf{C}_{L+1}$$

A valid layer-transition functor preserves identity, metric bounds, causal lineage, and SSL metadata. Functors introducing implicit semantic interpretations are invalid.

### 4.4. cdqn as the Fractal Protocol of Governed Data Movement

{% include term.html id="cdqn" %} is the universal protocol of governed data movement, functioning across two structural scopes:

```
                    FRACTAL SCOPE OF cdqn
 ┌─────────────────────────────────────────────────────────────────┐
 │ REMOTE cdqn (Inter-Node)                                        │
 │ Cross-network attestations, public PQC outer ring, consensus    │
 ├─────────────────────────────────────────────────────────────────┤
 │ LOCAL cdqn (Intra-Node)                                         │
 │ Cross-layer receipts, inner-ring sealing, memory bus transport │
 └─────────────────────────────────────────────────────────────────┘
```

1. **Local Scope (Intra-Node Chaining):** Governs data movement and morphism transitions between local abstraction layers and internal domains ($\mathrm{Qm}, \mathrm{Qs}, \mathrm{Qphy}$) via chained receipts.
2. **Distributed Scope (Inter-Node Attestation):** Connects autonomous local categories ($\mathbf{C}_N$) into a distributed sheaf-like structure via public attestations:

$$\mathcal{E}_{\text{export}}: \mathbf{C}_N \to \mathbf{Attestations}_{\text{cdqn}}$$

$\mathrm{cdqn}$ acts as a neutral, protocol-blind conduit. It warrants transit non-malleability ($\mathrm{Commitment}(P_{\text{source}}) \equiv \mathrm{Commitment}(P_{\text{dest}})$) while remaining blind to payload semantics.

---

## 5. Complexity Degree Stratification

Artifacts and expressions ({% include term.html id="qexpr" %}) are stratified by structural {% include term.html id="complexity-degree" %}:

| Degree | Structural Content | Examples |
|---|---|---|
| **0** | Primitives | $Q(0)$, $Q(1)$ |
| **1** | Primitive Operations | Elementary addition, subtraction, multiplication |
| **2** | Degree 1 Compositions | Elementary Qexpr expressions |
| **3** | Higher Compositions | Bounded constants, ordering relations, set partitions |
| **$n$** | Degree $n-1$ Compositions | Advanced domain structures, compiled runtime attestations |

Public exposure across the cdqn network may enforce an explicit minimum complexity degree threshold.

---

## 6. Numeric Representation and Compilation

### 6.1. Floating-Point Prohibition

Floating-point representations (IEEE 754) are prohibited at the Qn semantic layer. Floating-point semantics introduce implicit rounding, platform-dependent behavior, and non-deterministic identity. Lower hardware layers may execute binary operations internally, provided floating-point semantics are never exposed to the Qn semantic layer.

### 6.2. Qexpr Notation

Governed numeric expressions ({% include term.html id="qexpr" %}) maintain the explicit tuple:

$$\text{Qexpr} = \langle \text{SymbolicStructure}, \, z, \, r, \, d, \, \text{DCC}, \, \text{Identity}, \, \text{Lineage} \rangle$$

- **Zoom ($z$):** Declared scale or precision boundary.
- **Remainder ($r$):** Exact residual quantity at the declared zoom level.
- **Dimension ($d$):** Dimensional coordinate system.

### 6.3. Collapse at Compilation Time

Qexpr expressions remain symbolic during authoring and collapse deterministically to concrete values at compilation time ({% include term.html id="qnlang" %} $\to$ {% include term.html id="qnir" %}). Collapse operations must be total, finite, and accompanied by a remainder receipt.

### 6.4. Bounded Symbolic Expressions

Symbolic expression trees are strictly bounded. DCC profiles declare maximum node count, recursion depth, and evaluation budgets. Exceeding bounds triggers an intermediate governed collapse or execution termination.

---

## 7. Arithmetic Foundation as Structural Example

### 7.1. First Operations

From $Q(1)$, elementary operations are defined along axis $d_1$: additive, subtractive, multiplicative, and divisive morphisms.

### 7.2. Division as Constrained Operation

Division is defined as a bounded constraint equation:

$$\text{Given } a, b \implies \text{find } c, r \quad \text{such that } a = (b \times c) + r$$

- Denominators equal to $Q(0)$ are forbidden by DCC constraint.
- Attempted division by zero emits the terminal receipt `DIVISION_BY_ZERO_REJECTED`.

### 7.3. Positional Ordering

Axis $d_1$ admits three disjoint partitions: negative Qn, origin $\{Q(0)\}$, and positive Qn. Ordering is positional, not hierarchical. When zoom precision prevents order determination due to remainder overlap, the operation terminates with `ORDERING_INDETERMINATE_AT_ZOOM`.

---

## 8. Identity, Security, and cdqn Exposure

### 8.1. Identity Classes

| Identity Class | Substrate Entity | Prohibition |
|---|---|---|
| **Machine Identity** | Runtime node, execution container, hardware device | Must not imply legal personhood |
| **Human Identity** | Natural person, authorized operator | Must not execute without delegation |
| **AI Agent Identity** | Delegated autonomous or semi-autonomous process | Must not possess undelegated authority |

### 8.2. Accountable Pseudonymity

The cdqn network forbids anonymous actors. All interactions are signed by accountable pseudonymous identities backed by capability proofs, cryptographic commitments, and explicit revocation paths.

### 8.3. Dual-Ring PQC Boundary

```
┌────────────────────────────────────────────────────────┐
│ Outer Ring: cdqn Network Exposure                      │
│ (Pseudonyms, Attestation Keys, Delegation Proofs)      │
│   ┌────────────────────────────────────────────────┐   │
│   │ Inner Ring: Local Node Substrate               │   │
│   │ (Root Keys, Q(0), Local Lineage, Sealing Keys) │   │
│   └────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────┘
```

The Inner Ring governs local execution and genesis. The Outer Ring governs network-facing commitments. Both rings implement structural indirection to support non-disruptive migration across Post-Quantum Cryptography (PQC) standards.

---

## 9. Structure-Generativity Balance

The architecture enforces the separation principle: **Fixed Structure, Flexible Content**.

- **Structure:** Non-negotiable layers, gateways, DCC profiles, metric envelopes, and causal lineage.
- **Content:** Heuristic proposals, neural network inferences, and stochastic candidate generation.

Stochastic engines may generate proposals, provided they are classified as *unverified candidates*. Unverified candidates cannot cross a {% include term.html id="simemp-gateway" %}. They become governed Qn artifacts only after formal construction, constraint evaluation, and sealing.

---

## 10. External Precedents and Formal Convergence

The architecture aligns with established theoretical computer science frameworks:
- **Abstract Interpretation:** Sound approximation across discrete abstraction lattices (Zoom $z$, Remainder $r$).
- **Domain Theory:** Bounded evaluation, fixed-point semantics, and totality by budget.
- **Sheaf Theory:** Local category consistency and restricted global gluing via exposure functors.
- **Linear Logic:** Strict resource consumption without unmetered duplication or discarding.
- **Neuro-Symbolic Verification:** Strict operational isolation between neural candidate generation and symbolic verification.

---

## 11. Future Qn Definition Files and Local-First Base Domains

Development proceeds sequentially, separating local foundational domains from distributed networking:

### Category A — Numeric Primitives
- Detailed field anatomy of $Q(0)$ (silicon entropy harvesting, fuzzy extractors).
- Detailed field anatomy of $Q(1)$ (calibration of compute unit $U$ along $d_1$).
- Elementary numeric properties ($Q(2) \dots Q(9)$).

### Category B — Operations and Morphisms
- Multi-dimensional axes ($d_k$), advanced transformations, rational constraint solvers, and bounded simplification.

### Category C — Data Structures
- Directed acyclic state graphs, immutable memory containers (`Q(dataStruc)`), and pattern matching (`Q(patterns)`).

### Category D — Local-First Base Domains
Base domains project directly from Layer 1 and execute 100% locally via local $\mathrm{cdqn}$ data movement, requiring zero network consensus:
- **$\mathrm{Qm}$ (Quang Mathematics):** Constructive proof engines, discrete calculus, and exact numeric proofs.
- **$\mathrm{Qs}$ (Quang Semantics):** Explicit knowledge graphs, semantic linguistic ontologies, and formal assertion verification.
- **$\mathrm{Qphy}$ (Quang Physics):** Thermodynamic simulations, discrete quantum models, and physical entropy tracking.

### Category E — Runtime and Distributed Networking
- Intermediate representation ({% include term.html id="qnir" %}) instruction set and bounded virtual execution handler.
- High-level authoring language ({% include term.html id="qnlang" %}) syntax.
- $\mathrm{cdqn}$ distributed chaining and public attestation protocol.

---

## 12. License Alignment

This specification strictly conforms to the {% include term.html id="ssl" %}:

### 12.1. Paternity Reference
All derivative distributions, compiled binaries, APIs, and runtime specification files must retain the canonical attribution:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 12.2. Open Core Invariants
1. **Anti-Patent Defense:** Commercial or derivative licenses terminate automatically upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free licensing is preserved for non-commercial, academic, and sub-threshold usage.

### 12.3. Scale Auditing
The metric and identity envelopes provide native, cryptographically verifiable telemetry (active compute nodes, containers, agents, and monthly API transactions) to verify adherence to commercial {% include term.html id="scaling-threshold" text="Scaling Thresholds" %}.

---

## 13. Open Items

The following formal specifications remain open for subsequent releases:
1. Internal field anatomy and hardware sampling models for $Q(0)$ and $Q(1)$.
2. Complete grammar specification for {% include term.html id="qexpr" %}.
3. Formal instruction set and operational semantics for {% include term.html id="qnir" %}.
4. Syntax and type-checking rules for {% include term.html id="qnlang" %}.
5. PQC algorithm agility registry and migration protocol.
6. Formal specification of local base domains ($\mathrm{Qm}$, $\mathrm{Qs}$, $\mathrm{Qphy}$).
