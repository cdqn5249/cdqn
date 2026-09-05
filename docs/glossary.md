# CDQN Unified Glossary — Single Source of Truth
# Author: Christophe Duy Quang Nguyen
# License: Scaling Source License (SSL) 1.0
# Updated: 2026-09-05 (Bao Loc, Vietnam)

- term: "Dependencies Determinism"
  slug: "dependencies-determinism"
  category: "Constraints"
  definition: >
    The formal property whereby a computational state transition S_t possesses
    zero conditional entropy given its complete declared set of causal
    dependencies: H(S_t | D(S_t)) = 0. If dependencies are incomplete, implicit,
    or unmeasured, residual entropy produces non-deterministic state drift.
  sources:
    - doc: "simemp.md"
      section: "1. The Foundational Conjecture and Dependencies Determinism"
      anchor: "#1-the-foundational-conjecture-and-dependencies-determinism"
    - doc: "index.md"
      section: "1. Scope, Thermodynamics, and Architectural Stance"
      anchor: "#1-scope-thermodynamics-and-architectural-stance"

- term: "Structural Indirection"
  slug: "structural-indirection"
  category: "Constraints"
  definition: >
    The architectural decoupling of abstract capability contracts from concrete
    cryptographic or serialization primitives. Allows the runtime to patch
    zero-day vulnerabilities and swap algorithms without mutating abstract DCC
    contracts or fracturing causal lineage.
  sources:
    - doc: "simemp.md"
      section: "4.3. The DCC Profile and Structural Indirection"
      anchor: "#43-the-dcc-profile-and-structural-indirection"
    - doc: "abstractionLayers.md"
      section: "8.3. Dual-Ring PQC Boundary"
      anchor: "#83-dual-ring-pqc-boundary"
    - doc: "qnPrimitive.md"
      section: "4. Universal Envelope Structure"
      anchor: "#4-universal-envelope-structure"

- term: "SIMEMP"
  slug: "simemp"
  category: "Constraints"
  definition: >
    Security, Identity, Metric, Efficiency, Modularity, Portability. The
    constitutional constraint system governing the Qn and cdqn stack. Divided
    into Tier 1 Existential Invariants (non-negotiable) and Tier 2 Operational
    Agilities (trade-off-able).
  sources:
    - doc: "simemp.md"
      section: "3. The SIMEMP Constraint Architecture"
      anchor: "#3-the-simemp-constraint-architecture"
    - doc: "abstractionLayers.md"
      section: "3.2. The No-Implicit Rule"
      anchor: "#32-the-no-implicit-rule"
    - doc: "qnPrimitive.md"
      section: "Normative References"
      anchor: "#normative-references"

- term: "DCC Profile"
  slug: "dcc-profile"
  category: "Constraints"
  definition: >
    Dependencies, Constraints, Capabilities. A machine-readable abstract
    contract defining prerequisites, hard operational bounds, and permitted
    morphisms for every layer, object, and gateway. No DCC profile may be
    implicit.
  sources:
    - doc: "simemp.md"
      section: "4.3. The DCC Profile and Structural Indirection"
      anchor: "#43-the-dcc-profile-and-structural-indirection"
    - doc: "abstractionLayers.md"
      section: "3.4. DCC Profiles Everywhere"
      anchor: "#34-dcc-profiles-everywhere"
    - doc: "qnPrimitive.md"
      section: "1.7. DCC Profile"
      anchor: "#17-dcc-profile"
    - doc: "qnPrimitive.md"
      section: "7. DCC Profile Rules"
      anchor: "#7-dcc-profile-rules"

- term: "Existential Invariant"
  slug: "existential-invariant"
  category: "Constraints"
  definition: >
    A Tier 1 SIMEMP constraint that is non-negotiable. The three Existential
    Invariants are Identity, Metric, and Security. If a proposed design
    violates or lacks these, it is fundamentally invalid.
  sources:
    - doc: "simemp.md"
      section: "3.1. Tier 1: Existential Invariants"
      anchor: "#31-tier-1-existential-invariants"
    - doc: "simemp.md"
      section: "5.3. Security Constraints"
      anchor: "#53-security-constraints"

- term: "Operational Agility"
  slug: "operational-agility"
  category: "Constraints"
  definition: >
    A Tier 2 SIMEMP constraint that is trade-off-able. The three Operational
    Agilities are Efficiency, Modularity, and Portability. They may be
    sacrificed to preserve Tier 1 Existential Invariants.
  sources:
    - doc: "simemp.md"
      section: "3.2. Tier 2: Operational Agilities"
      anchor: "#32-tier-2-operational-agilities"

- term: "BOC Policy"
  slug: "boc-policy"
  category: "Constraints"
  definition: >
    Best of Choices. The formal decision framework used to resolve design
    conflicts by selecting the implementation path that maximizes SIMEMP
    compliance and minimizes implicit state.
  sources:
    - doc: "simemp.md"
      section: "7. The BOC Policy"
      anchor: "#7-the-boc-policy"

- term: "Scaling Threshold"
  slug: "scaling-threshold"
  category: "Constraints"
  definition: >
    Usage boundaries defined by the SSL 1.0 triggering mandatory commercial
    licensing: >10,000 active compute nodes/agents, >100,000 MAU, or
    >10,000,000 monthly API/Qn transactions.
  sources:
    - doc: "simemp.md"
      section: "8. Alignment with the Scaling Source License"
      anchor: "#8-alignment-with-the-scaling-source-license"
    - doc: "abstractionLayers.md"
      section: "12.3. Scale Auditing"
      anchor: "#123-scale-auditing"
    - doc: "LICENSE.md"
      section: "1.5. Scale / Scaling Usage"
      anchor: "#15-scale--scaling-usage"

- term: "No-Implicit Rule"
  slug: "no-implicit-rule"
  category: "Constraints"
  definition: >
    The constitutional rule that no implicit entity, default, coercion, or
    unmeasured convention may cross a SIMEMP Gateway. Only explicit Qn objects
    are admitted between abstraction layers.
  sources:
    - doc: "abstractionLayers.md"
      section: "3.2. The No-Implicit Rule"
      anchor: "#32-the-no-implicit-rule"
    - doc: "qnPrimitive.md"
      section: "Axiom 1 — Explicit Envelope"
      anchor: "#axiom-1--explicit-envelope"
    - doc: "qnPrimitive.md"
      section: "Axiom 9 — Numeric Precision Explicitness"
      anchor: "#axiom-9--numeric-precision-explicitness"

- term: "Computational Consistency"
  slug: "computational-consistency"
  category: "Constraints"
  definition: >
    The empirical verification methodology of the Qn universe. If logic is
    internally consistent and computations authored in QnLang and compiled to
    QnIR execute deterministically within declared finite budgets, the
    abstraction is provisionally valid.
  sources:
    - doc: "simemp.md"
      section: "2. Method of Verification: The Recursive Search Loop"
      anchor: "#2-method-of-verification-the-recursive-search-loop"
    - doc: "abstractionLayers.md"
      section: "1. Foundational Position"
      anchor: "#1-foundational-position"
    - doc: "qnPrimitive.md"
      section: "3. Provisional Operational Axioms"
      anchor: "#3-provisional-operational-axioms"

- term: "Qn"
  slug: "qn"
  category: "Primitives"
  definition: >
    Quang Number. A governed, finite, identifiable, measurable, and
    security-constrained computational artifact within the Qn universe.
  sources:
    - doc: "abstractionLayers.md"
      section: "3.1. All Governed Artifacts are Qn"
      anchor: "#31-all-governed-artifacts-are-qn"
    - doc: "qnPrimitive.md"
      section: "1.1. Qn Artifact"
      anchor: "#11-qn-artifact"

- term: "Q(0)"
  slug: "q0"
  category: "Primitives"
  definition: >
    Local genesis object, causal origin zero, empty birth context. Strictly
    local to each node; no universal global zero is admitted.
  sources:
    - doc: "abstractionLayers.md"
      section: "2.3. Layer 1 — Node Genesis Layer"
      anchor: "#23-layer-1--node-genesis-layer"
    - doc: "qnPrimitive.md"
      section: "Axiom 4 — Local Genesis Dependency"
      anchor: "#axiom-4--local-genesis-dependency"

- term: "Q(1)"
  slug: "q1"
  category: "Primitives"
  definition: >
    First unit artifact, unity measure, and baseline reference for the
    abstract compute unit U along dimensional axis d1.
  sources:
    - doc: "abstractionLayers.md"
      section: "2.3. Layer 1 — Node Genesis Layer"
      anchor: "#23-layer-1--node-genesis-layer"
    - doc: "LICENSE.md"
      section: "1.7. Q(1) Utility Metric"
      anchor: "#17-q1-utility-metric"

- term: "cdqn"
  slug: "cdqn"
  category: "Primitives"
  definition: >
    Chained and Distributed Quang Numbers. The distributed network layer
    composed of local categories linked via bounded exposure functors,
    attestations, and cross-node receipts.
  sources:
    - doc: "abstractionLayers.md"
      section: "4.4. cdqn as a Category of Local Categories"
      anchor: "#44-cdqn-as-a-category-of-local-categories"
    - doc: "qnPrimitive.md"
      section: "Axiom 8 — Local-first Exposure"
      anchor: "#axiom-8--local-first-exposure"

- term: "QnLang"
  slug: "qnlang"
  category: "Primitives"
  definition: >
    The high-level domain-specific language used to author Qn computations,
    enforcing explicit DCC profiles and compile-time collapse.
  sources:
    - doc: "simemp.md"
      section: "2. Method of Verification: The Recursive Search Loop"
      anchor: "#2-method-of-verification-the-recursive-search-loop"
    - doc: "qnPrimitive.md"
      section: "12. Relation to QnLang"
      anchor: "#12-relation-to-qnlang"

- term: "QnIR"
  slug: "qnir"
  category: "Primitives"
  definition: >
    Intermediate Representation used for hardware abstraction, bound
    enforcement, and deterministic execution without exposing floating-point
    semantics.
  sources:
    - doc: "simemp.md"
      section: "4.1. The Memory Wall, Semantic Gap, and Landauer's Principle"
      anchor: "#41-the-memory-wall-semantic-gap-and-landauers-principle"
    - doc: "qnPrimitive.md"
      section: "12. Relation to QnLang"
      anchor: "#12-relation-to-qnlang"

- term: "Qexpr"
  slug: "qexpr"
  category: "Primitives"
  definition: >
    Governed expression tuple: <SymbolicStructure, z, r, d, DCC, ID, Lineage>.
    Collapses deterministically to concrete values at compilation time.
  sources:
    - doc: "abstractionLayers.md"
      section: "6.2. Qexpr Notation"
      anchor: "#62-qexpr-notation"
    - doc: "qnPrimitive.md"
      section: "5.2. Expression Payload Profile"
      anchor: "#52-expression-payload-profile"

- term: "Abstraction Layer"
  slug: "abstraction-layer"
  category: "Architecture"
  definition: >
    A structural boundary in the Qn stack governing artifact genesis,
    transformation, and inter-layer transmission under SIMEMP constraints.
  sources:
    - doc: "abstractionLayers.md"
      section: "2. Layer Model"
      anchor: "#2-layer-model"
    - doc: "qnPrimitive.md"
      section: "11. Relation to Abstraction Layers"
      anchor: "#11-relation-to-abstraction-layers"

- term: "SIMEMP Gateway"
  slug: "simemp-gateway"
  category: "Architecture"
  definition: >
    The validation and translation boundary between abstraction layers. Enforces
    onboarding, fault translation, and the No-Implicit Rule.
  sources:
    - doc: "abstractionLayers.md"
      section: "2.2. Layer 0 to Layer 1 Gateway — Node Onboarding and Fault Translation"
      anchor: "#22-layer-0-to-layer-1-gateway--node-onboarding-and-fault-translation"
    - doc: "qnPrimitive.md"
      section: "Axiom 14 — Generative Candidate Boundary"
      anchor: "#axiom-14--generative-candidate-boundary"

- term: "Layer 0"
  slug: "layer-0"
  category: "Architecture"
  definition: >
    The commodity physical execution substrate (CPU, GPU, memory, physical
    entropy, Memory Wall). The acknowledged Root of Finiteness outside the
    governed universe.
  sources:
    - doc: "abstractionLayers.md"
      section: "2.1. Layer 0 — Commodity Physical Substrate"
      anchor: "#21-layer-0--commodity-physical-substrate"
    - doc: "qnPrimitive.md"
      section: "11.1. Layer 0"
      anchor: "#111-layer-0"

- term: "Layer 1"
  slug: "layer-1"
  category: "Architecture"
  definition: >
    The first governed Qn layer containing local genesis Q(0), first unit Q(1),
    abstract compute unit U, sign polarity, and elementary operations along d1.
  sources:
    - doc: "abstractionLayers.md"
      section: "2.3. Layer 1 — Node Genesis Layer"
      anchor: "#23-layer-1--node-genesis-layer"
    - doc: "qnPrimitive.md"
      section: "11.3. Layer 1"
      anchor: "#113-layer-1"

- term: "Causal Arrow"
  slug: "causal-arrow"
  category: "Architecture"
  definition: >
    A monotonic, persistent ordering of computational events independent of
    physical wall-clock time. Every Qn artifact is born at a discrete causal
    index.
  sources:
    - doc: "simemp.md"
      section: "5.3. Security Constraints"
      anchor: "#53-security-constraints"
    - doc: "abstractionLayers.md"
      section: "3.5. Causal Arrow and Birth Order"
      anchor: "#35-causal-arrow-and-birth-order"
    - doc: "qnPrimitive.md"
      section: "1.6. Lineage"
      anchor: "#16-lineage"

- term: "Complexity Degree"
  slug: "complexity-degree"
  category: "Architecture"
  definition: >
    Structural measure of Qexpr composition depth and stratification (Degree 0
    primitives through Degree n domain structures).
  sources:
    - doc: "abstractionLayers.md"
      section: "5. Complexity Degree Stratification"
      anchor: "#5-complexity-degree-stratification"

- term: "Local-first"
  slug: "local-first"
  category: "Architecture"
  definition: >
    The architectural principle that artifacts are born locally, indexed
    locally, and shielded from public network exposure by default.
  sources:
    - doc: "abstractionLayers.md"
      section: "3.3. Local-first and Controlled Exposure"
      anchor: "#33-local-first-and-controlled-exposure"
    - doc: "qnPrimitive.md"
      section: "6.1. Local-first Identifier Structure"
      anchor: "#61-local-first-identifier-structure"

- term: "Universal Envelope"
  slug: "universal-envelope"
  category: "Governance"
  definition: >
    The invariant governance structure shared across all Qn artifacts: <ID,
    Type, State, Lineage, DCC, Metric, Security, Exposure, PayloadRef>.
  sources:
    - doc: "qnPrimitive.md"
      section: "1.3. Universal Envelope"
      anchor: "#13-universal-envelope"
    - doc: "qnPrimitive.md"
      section: "4. Universal Envelope Structure"
      anchor: "#4-universal-envelope-structure"

- term: "Metric Envelope"
  slug: "metric-envelope"
  category: "Governance"
  definition: >
    The finite measurement boundary declaring resource limits on storage,
    expression depth, precision, uncertainty, and compute budgets.
  sources:
    - doc: "qnPrimitive.md"
      section: "1.8. Metric Envelope"
      anchor: "#18-metric-envelope"
    - doc: "qnPrimitive.md"
      section: "8. Metric Envelope Rules"
      anchor: "#8-metric-envelope-rules"

- term: "Security Envelope"
  slug: "security-envelope"
  category: "Governance"
  definition: >
    The governed security boundary declaring authority limits, delegation
    chains, PQC algorithm identifiers, and swappable revocation paths.
  sources:
    - doc: "qnPrimitive.md"
      section: "1.9. Security Envelope"
      anchor: "#19-security-envelope"
    - doc: "qnPrimitive.md"
      section: "9. Security Envelope Rules"
      anchor: "#9-security-envelope-rules"

- term: "Receipt"
  slug: "receipt"
  category: "Governance"
  definition: >
    A governed, immutable record of an execution transition, fault, halt, or
    boundary traversal. Acts as a dissipative witness exporting operational
    entropy to maintain internal system order.
  sources:
    - doc: "simemp.md"
      section: "4.2. Open Systems, Entropy Export, and Dissipative Error Correction"
      anchor: "#42-open-systems-entropy-export-and-dissipative-error-correction"
    - doc: "abstractionLayers.md"
      section: "3.6. Totality by Budget"
      anchor: "#36-totality-by-budget"
    - doc: "qnPrimitive.md"
      section: "1.11. Receipt"
      anchor: "#111-receipt"

- term: "Lifecycle State"
  slug: "lifecycle-state"
  category: "Governance"
  definition: >
    The operational status of an artifact: candidate, constructed, sealed,
    verified, rejected, halted, quarantined, exportable, exported, revoked.
  sources:
    - doc: "qnPrimitive.md"
      section: "1.10. Lifecycle State"
      anchor: "#110-lifecycle-state"
    - doc: "qnPrimitive.md"
      section: "10. Lifecycle and Receipts"
      anchor: "#10-lifecycle-and-receipts"

- term: "Open Core Invariant"
  slug: "open-core-invariant"
  category: "Governance"
  definition: >
    Non-negotiable conditions preserved in all Derivative Works under the SSL
    1.0: (a) Anti-Patent Defense, and (b) Non-Scaling Open Access.
  sources:
    - doc: "index.md"
      section: "4. License and Open Core Invariants"
      anchor: "#4-license-and-open-core-invariants"
    - doc: "abstractionLayers.md"
      section: "12.2. Open Core Invariants"
      anchor: "#122-open-core-invariants"
    - doc: "LICENSE.md"
      section: "3.3. Open Core Invariant Propagation"
      anchor: "#33-open-core-invariant-propagation"

- term: "Paternity Reference"
  slug: "paternity-reference"
  category: "Governance"
  definition: >
    Mandatory attribution notice for derivative distributions and runtime
    headers citing Christophe Duy Quang Nguyen under the Scaling Source License.
  sources:
    - doc: "index.md"
      section: "4. License and Open Core Invariants"
      anchor: "#4-license-and-open-core-invariants"
    - doc: "abstractionLayers.md"
      section: "12.1. Paternity Reference"
      anchor: "#121-paternity-reference"
    - doc: "qnPrimitive.md"
      section: "13.1. Paternity Reference"
      anchor: "#131-paternity-reference"
    - doc: "LICENSE.md"
      section: "3.1. Paternity Reference"
      anchor: "#31-paternity-reference"

- term: "SSL"
  slug: "ssl"
  category: "Governance"
  definition: >
    Scaling Source License 1.0. The commercial and open-core license governing
    the CDQN project, derivative distributions, and scaling thresholds.
  sources:
    - doc: "simemp.md"
      section: "8. Alignment with the Scaling Source License"
      anchor: "#8-alignment-with-the-scaling-source-license"
    - doc: "abstractionLayers.md"
      section: "12. License Alignment"
      anchor: "#12-license-alignment"
    - doc: "qnPrimitive.md"
      section: "13. License Alignment"
      anchor: "#13-license-alignment"
    - doc: "LICENSE.md"
      section: "1.1. Preamble"
      anchor: "#11-preamble"
