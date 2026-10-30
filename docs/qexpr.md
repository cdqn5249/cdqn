---
layout: default
title: Qexpr — Content-Addressed Expression DAG (Layer 3 Symbolic Container)
description: Layer 3 specification — the content-addressed expression DAG with hash-consing and six canonical node classes
version: 1.4.0
updated: 2026-10-30
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qexpr.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qexpr.html
terms_used:
  - boc-policy
  - cae-dag
  - causal-arrow
  - complexity-degree
  - compute-unit-u
  - dcc-profile
  - dependencies-determinism
  - dimension-d
  - existential-invariant
  - hash-consing
  - hca-collapse
  - layer-0
  - layer-1
  - licensed-work
  - metric-envelope
  - metric-exhaustion
  - no-implicit-rule
  - paternity-reference
  - payload
  - q-bypass
  - q-reuse
  - q0
  - q1
  - qexpr
  - qm
  - qn
  - qs
  - receipt
  - remainder-r
  - security-envelope
  - ssl
  - structural-indirection
  - terminal-exactness
  - universal-envelope
  - zoom-z
---

# Qexpr — Content-Addressed Expression DAG: Layer 3 Symbolic Container

| Field | Specification |
|---|---|
| **Document Title** | Qexpr — Content-Addressed Expression DAG: Layer 3 Symbolic Container |
| **Version** | 1.4.0 |
| **Last Updated** | 2026-10-28 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Canonical Specification — Layer 3 Structural Container ({% include term.html id="complexity-degree" text="Complexity Degree 2" %}) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing this specification. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs. References are deliberately version-unpinned: each target document's front matter carries its own authoritative version, so this specification never duplicates drift-prone patch state.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and [Optimization Morphisms]({{ '/glossary.html' | relative_url }}#q-reuse) | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/q2_q9.md` | Single-digit secondary DCC anchors and parity partitions | [q2_q9.html]({{ '/q2_q9.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qm_calculus.md` | Discrete Exterior Calculus, continued fractions, and invariant dynamics | [qm_calculus.html]({{ '/qm_calculus.html' | relative_url }}) |
| `docs/qs.md` | Constructive relational intent and [Intent Contracts]({{ '/glossary.html' | relative_url }}#intent-contract) | [qs.html]({{ '/qs.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## Review Integration Record (v1.3.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.1.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.1.md) — transcript of the external adversarial review (Google Deep Review, October 2026) of the condensed Qn universe specification and QnIR v0.1 candidate draft. This update integrates the Layer-3 rewrite-semantics findings of that review into the canon. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place. New constructions and semantic refinements introduced below are tagged **[v1.3.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| `OP_REWRITE` split into `OP_BYPASS` (canonical contraction, hash-governing) and `OP_SATURATE` (bounded, `Q(rsi)`-gated equality saturation); `OP_REWRITE` retained as deprecated alias until QnIR v0.2 | §4.6 | QO-5 / Segment 2 Point 3 (definitive theoretical constraint) | CANDIDATE |
| Saturation artifact discipline: output is a new versioned artifact (Axiom 13), never a canonical node identity; provenance (schedule, ceilings, extractor generation) recorded | §4.6.2 | QO-5 / Segment 2 Point 3 | CANDIDATE |
| Canonical child sorting for `DCC_SYM_COMMUTATIVE` opcodes + hashing-generation tag on the DCC profile | §2.2 | QO-5 / Segment 2 Point 3 | CANDIDATE |
| Bypass suite count fixed at **16 rules** (first-root rule restored); QnIR v0.1 PDF condensation reconciled | §4.6.1 | `qm.md` v1.3.0 cross-file flag / data-loss audit | editorial |
| Confluence of the bypass suite tagged CANDIDATE; machine-checked confluence certificate set as QnIR v0.2 sealing prerequisite | §4.6.1 | QO-5 (termination/confluence demand) | CANDIDATE |
| Fourth circuit breaker: saturation iteration ceiling $$K_{\max}$$; reason codes `K_MAX_REACHED` / `N_MAX_REACHED` / `D_MAX_REACHED`; receipt `RECEIPT_SATURATION_CONVERGED` | §6 | QO-5 / Segment 2 Point 3 | CANDIDATE |
| §6 breaker-box row 3 box-alignment correction (pre-existing 1-char drift) | §6 | — | editorial |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline, to be integrated in subsequent versioned updates): `docs/qnPrimitive.md` (Q(bypass) execution precedence sentence and macro-DAG "equality saturation" wording aligned to the split; third circuit-breaker receipt naming); `docs/qlog.md` §6.1 ("Background Equality Saturation" resolves to `OP_SATURATE`); `docs/qm.md` v1.3.0 one-line touch-up (fixed-point identity attribution `OP_REWRITE` → `OP_BYPASS` in the Fixed-Point Invariant entry); glossary term additions for the new §2.2/§4.6 constructs; and the QnIR v0.2 candidate must carry distinct opcodes for `OP_BYPASS` and `OP_SATURATE` (Reviewer Segment 4 Point 1, Requirement R11 opcode-coverage finding, resolved in the QnIR v0.2 cycle, not in this file).

---

## Review Integration Record (v1.4.0)

Source: [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md) — verbatim transcript and artifact of record of the Loop-2 adversarial review by Kimi K3.1-Pro (October 27th, 2026) of the QnIR v0.2 candidate draft v0.2.2 — as integrated upstream by the two committed anchors this file mirrors: the constitutional two-axis Layer-0 Hardware Telemetry Envelope of `docs/simemp.md` v1.3.0 and the operational two-axis admission predicate of `docs/qnPrimitive.md` v1.4.0 (§1.15 term provenance, §5.9 margin governance and Admission Ledger). The `docs/qm_geometry.md` v1.3.0 record flags this file's mirror obligation — the §2.3 breakeven admission bound mirror and the same zero-cost scoping for the "zero Landauer dissipation (W = 0)" normalization sentence — discharged here. Qexpr makes no new policy adjudication of its own: the margin range `[1, 64]` and the recommended preset (`κ_dcc = 2`, `κ_t = 4`) are inherited unchanged from `docs/qnPrimitive.md` §5.9. Under **Axiom 13**, this file is a new versioned artifact; no prior content is mutated in place; the withdrawn v1.3.0-era wordings are preserved verbatim in this record. New constructions introduced below are tagged **[v1.4.0 — CANDIDATE]** pending BOC sealing; editorial corrections are applied directly and recorded here.

**Withdrawn wording (v1.3.0-era, preserved verbatim):**
1. the §2.3 item title "The Breakeven Admission Bound" with its admission sentence "Querying a cryptographic hash commitment across L3 caches or DRAM incurs measurable bus dissipation. Under the **Metric Invariant** (`simemp.md` §3.1), hash-consing lookup via $$Q(\text{reuse})$$ is admissible if and only if the computational work of re-evaluation exceeds retrieval dissipation:" and its display inequality `$$\mathrm{Admit}\big(Q(\text{reuse})\big) \iff \Delta W_{\mathrm{compute}} \gt \Delta W_{\mathrm{lookup}} + \epsilon_{\mathrm{overhead}}$$`;
2. the §2.3 normalization bullet "Reversible symbolic normalization within the CAE-DAG incurs **zero Landauer dissipation** ($$W = 0$$), directly operationalizing Q(reuse) across the Memory Wall." (the bullet's live `term.html` include is rendered as plain "Q(reuse)" here to keep the record inert);
3. the §2 comparison-diagram cell "• Direct coupling to Q(reuse) (W = 0)";
4. the §2.3 direct-path fallback sentence `If direct computation is cheaper than memory traversal (such as single-cycle integer additions $$Q(2) + Q(3)$$ or trivial powers), the compiler bypasses hash-table probes, lowering the operation directly to integer machine instructions via the HCA-Collapse engine.` (re-anchored below to the two-axis verdict, its examples and HCA-Collapse anchor preserved).

| Change | Section | Reviewer source | Type |
|---|---|---|---|
| **Two-axis predicate mirror**: the §2.3 admission bound is re-issued as the Two-Axis Consequentiality Breakeven — advantage declared on **both axes** of the `docs/simemp.md` §4.4 envelope ("consequently faster") over the canonical tokens of `docs/qnPrimitive.md` §1.15/§5.9 (`ΔW_compute`, `Δt_compute_bound`, tier-indexed `τ_U[tier(h)]`/`τ_t[tier(h)]`, gateway slacks `ε_U`/`ε_t`, margin range `[1, 64]` with preset `κ_dcc = 2`/`κ_t = 4` inherited unchanged), with compact term provenance anchoring the declared cost model to this file's own §2.1 `DCC_ref` storage binding and the declared per-opcode bounds of `docs/qm.md` / `docs/qm_geometry.md` §8.1; undeclared quantities force the Direct-Compute fallback, ties resolve to Direct Compute, and admitted reuse executes under the Admission Ledger of `docs/qnPrimitive.md` §5.9; the work-axis-only "if and only if" admission sentence, its display inequality, and its fallback trigger are withdrawn (the bare form carries no latency-axis verdict — the QO-17 defect) | §2.3 | `QO-17` (via `docs/qnPrimitive.md` v1.4.0); consequentiality constraint (`docs/simemp.md` §4.4); F1 declared-bound discipline | CANDIDATE |
| **Zero-cost scoping**: the §2.3 normalization bullet's W = 0 claim is confined to the reversible-symbolic domain (no bit erased in abstract state evolution — outside the admission path, the canon-consistency class of the `docs/qm_geometry.md` v1.3.0 EDITORIAL row) and decoupled from Q(reuse): tier traversal always dissipates, and Q(reuse) harvests the Memory Wall only under the two-axis predicate, executing under the Admission Ledger (`W_net = W_actual_charge` — charged declared work, never free); the §2 diagram cell is re-scoped to `(symbolic W = 0)` with column alignment preserved | §2.3, §2 | F1 declared-bound discipline; Admission Ledger (`docs/qnPrimitive.md` §5.9); `docs/qm_geometry.md` v1.3.0 EDITORIAL precedent | CANDIDATE |
| Editorial: the touched admission statement is re-encoded to engine-independent plain text per the `docs/abstractionLayers.md` v1.8.1 convention (code-span predicate, Unicode operators); the remaining LaTeX spans of this file remain queued for file-wide re-encoding at its next versioned update | §2.3 | Rendering Fix Record (`docs/abstractionLayers.md` v1.8.1); `docs/qm_geometry.md` v1.3.0 EDITORIAL precedent | EDITORIAL |
| Editorial (post-commit rendering repair): the six Jekyll link targets introduced into §2.3 by this record's mirror were committed with halved brace pairs — one opening and one closing brace instead of two around the relative_url target — a build-script f-string escaping defect (the LaTeX braces of the same block were doubled correctly, the Liquid braces were not quadrupled), leaving literal unresolvable targets in the rendered page; all six are restored to the canonical double-brace relative_url form (simemp ×1, qnPrimitive ×3, qm ×1, qm_geometry ×1); no semantic content, withdrawn wording, or canonical token is touched | §2.3 | Site rendering defect; `docs/index.md` v1.13.0 terms_used formatting-correction precedent | EDITORIAL |

Cross-file dependencies flagged (deferred under the one-file-at-a-time discipline): `docs/qstructures.md` §6 work-row precedent and `docs/qphy.md` physical-impedance grounding inherit the two-axis form at their next versioned updates; the unbudgeted `docs/_data/navigation.yml` review-program entry remains optional; the substantive next horizon is the QnIR v0.3 minimal executable core per the Loop-2 repair sequence. No new `term.html` includes are used in this update — the q-reuse include of §2.3 pre-dates this wave — under the standing no-dead-entries discipline of `docs/qs_history.md` §8; front matter `terms_used` unchanged.

---

## 1. Epistemic Stance and Constitutional Placement

Within the master abstraction hierarchy ([`docs/abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) §2.5, §5), **{% include term.html id="qexpr" text="Qexpr" %} is situated strictly at Layer 3 ({% include term.html id="complexity-degree" text="Complexity Degree 2" %})** — the terminal layer of the physical execution core. Domains are orthogonal vertical projections across Layers 1–3, not horizontal layers above the stack.

```
 +-------------------------------------------------------------------------------+
 | ORTHOGONAL DOMAIN PROJECTIONS: Qm (Mathematics), Qs (Semantics),              |
 | Qphy (Physics), Qlog, Qchem, Qbio — vertical slices, NOT horizontal layers    |
 +---------------------------------------^---------------------------------------+
                                         │ Vertical projections across Layers 1–3
 +---------------------------------------+---------------------------------------+
 | LAYER 3 — EXECUTION CEILING: SYMBOLIC COMPOSITION LAYER (Qexpr / CAE-DAG)     |
 | • Structural Container: Content-Addressed Expression DAG                      |
 | • Role: Wires Layer 2 atomic morphisms into bounded, evaluatable graphs       |
 | • Mechanics: Hash-consing, algebraic rewrite, compile-time collapse budget    |
 +---------------------------------------^---------------------------------------+
                                         │ Gateway Constraint & Budget Metering
 +---------------------------------------+---------------------------------------+
 | Layer 2: Governed Morphism Layer (Atomic Operations: +, -, ×, ÷, Rotors)     |
 +---------------------------------------^---------------------------------------+
                                         │ Gateway Onboarding
 +---------------------------------------+---------------------------------------+
 | Layer 1: Node Genesis Layer (Static Existence: Q(0), Q(1), Q(2)...Q(9), Axis d1)|
 +-------------------------------------------------------------------------------+
```

### 1.1. The Structural Role of Layer 3
* **{% include term.html id="layer-1" %}** defines **what exists** (static ontological primitives: local origin {% include term.html id="q0" text="Q(0)" %}, unit {% include term.html id="q1" text="Q(1)" %}, single digits $$Q(2)\dots Q(9)$$ along axis {% include term.html id="dimension-d" text="d1" %}).
* **Layer 2** defines **atomic actions** (isolated morphisms: addition, subtraction, multiplication, Diophantine division, exponents, roots, factorials, Clifford wedge products, spatial rotors, {% include term.html id="q-reuse" text="Q(reuse)" %}, {% include term.html id="q-bypass" text="Q(bypass)" %}).
* **Layer 3 (`Qexpr`)** defines **compositional structure**: how multiple Layer 2 atomic actions are wired into directed acyclic networks *before* they are evaluated or collapsed into physical memory.

### 1.2. Acknowledgment of Universal Fallibility
A core design tenet of the {% include term.html id="qn" %} architecture is the explicit recognition that **both human authors and artificial agents are fallible entities**. Neither is an absolute arbitrator:
* Humans are prone to specification errors, unmeasured assumptions, and logical oversights.
* Artificial agents and compilers are prone to algorithmic hallucinations, non-terminating expansions, and parameter misalignments.

Under [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}), safety cannot rely on presumed actor infallibility. `Qexpr` serves as the **mechanical error-containment envelope**: it enforces strict syntactic cycle freedom, explicit graph depth bounds, and finite execution budgets ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) Axiom 6), ensuring that an error by any actor halts deterministically at the structural boundary without inducing systemic divergence.

### 1.3. Universal Semantic Membrane and Cross-Domain Contexts
`Qexpr` is not restricted to arithmetic calculations; it is the **universal structural membrane for all computable domains**.

A computational **Context** is formalized as a multi-domain Content-Addressed Expression DAG:

$$\text{Context} \equiv \mathcal{G}_{\text{Qexpr}} = \mathbf{C}_{\mathrm{Qs}} \otimes \mathbf{C}_{\mathrm{Qm}} \otimes \mathbf{C}_{\mathrm{Qphy}}$$

1. **Semantic Root ({% include term.html id="qs" text="Qs" %}):** Encapsulates intent, linguistic pregroups, and human-agent capability delegations.
2. **Relational Root ({% include term.html id="qm" text="Qm" %}):** Verifies that proposed state transitions are mathematically decidable, Diophantine-constrained, and exact.
3. **Physical Root ($$\mathrm{Qphy}$$):** Verifies that the required computational work can physically execute within finite Landauer dissipation bounds ($$W \ge k_B T \ln 2$$), parameterized by the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}.

By representing cross-domain contexts within the unified `Qexpr` structure, heterogeneous actors achieve consensus on shared root commitment hashes $$\mathcal{H}(\text{Context})$$ without unmeasured semantic ambiguity.

---

## 2. The Content-Addressed Expression DAG (CAE-DAG) Architecture

Under the {% include term.html id="boc-policy" text="Best of Choices (BOC) Policy" %} ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §7), `Qexpr` explicitly rejects naive pointer-heap trees in favor of a **{% include term.html id="cae-dag" text="Content-Addressed Expression Directed Acyclic Graph (CAE-DAG)" %}**.

```
    CLASSICAL POINTER-AST                         CONTENT-ADDRESSED EXPRESSION DAG (CAE-DAG)
    (Ephemeral, Redundant, Lossy)                 (Canonical, Deduplicated, Dissipation-Free)
          [ Operation * ]                                        [ Operation * ]
          /             \                                        /             \
    [ Sub-term A ]  [ Sub-term A ]                        ┌────►[ Sub-term A ]◄────┐
      (Address 0x1)   (Address 0x2)                       │     (Hash H_A)         │
         /     \         /     \                          │        /     \         │
       [x]     [y]     [x]     [y]                        └──────[x]     [y]───────┘
   • 2x Memory bit allocations                             • Exact 1x Memory bit allocation
   • RAM pointer drift across reboots                     • Platform-invariant cryptographic identity
   • Dissipates 2·W Landauer heat                         • Direct coupling to Q(reuse) (symbolic W = 0)
```

### 2.1. Canonical Node Tuple and Arity-Indexed Dependencies
Under the {% include term.html id="no-implicit-rule" %}, symbolic nodes cannot assume a rigid binary schema that forces dummy padding hashes for non-binary operators. Every symbolic node $$\alpha$$ within a `Qexpr` DAG is an immutable, content-addressed tuple with an explicit arity-indexed child commitment vector:

$$\alpha = \Big\langle \mathrm{OpCode}, \, \mathbf{D}_{\mathrm{children}}, \, z, \, r, \, d, \, \mathrm{DCC}_{\mathrm{ref}} \Big\rangle$$

$$\mathbf{D}_{\mathrm{children}} = \big[ \mathcal{H}_1, \, \mathcal{H}_2, \, \dots, \, \mathcal{H}_k \big], \quad k = \mathrm{Arity}(\mathrm{OpCode}) \in \mathbb{N}$$

The child commitment vector $$\mathbf{D}_{\mathrm{children}}$$ strictly maps operator arity without implicit padding:
- **Nullary ($$k = 0$$):** Leaf constants, primary primitives ($$Q(0), Q(1)$$), and continued fraction recurrence generators (`OP_CF` for $$\pi, e, \tau, \ln 2$$). $$\mathbf{D}_{\mathrm{children}} = []$$.
- **Unary ($$k = 1$$):** Morphisms over a single operand, including radical root constraints (`OP_ROOT`), discrete factorials, Clifford reflection involutions ($$\mathcal{I}_d$$), discrete exterior derivatives ($$\mathbf{d}$$), and Hodge stars ($$\star$$). $$\mathbf{D}_{\mathrm{children}} = [\mathcal{H}_1]$$.
- **Binary ($$k = 2$$):** Elementary $$\mathcal{R}$$-algebra morphisms ($$+, -, \times, \div$$, exponents), Clifford geometric products ($$\vec{u} \vec{v}$$), and wedge products ($$\vec{u} \wedge \vec{v}$$). $$\mathbf{D}_{\mathrm{children}} = [\mathcal{H}_1, \, \mathcal{H}_2]$$.
- **Variadic ($$k \ge 3$$):** Bounded monoidal folds (`OP_FOLD` for $$\sum, \prod$$), multi-axial coordinate frames, and multi-agent intent sequences. $$\mathbf{D}_{\mathrm{children}} = [\mathcal{H}_1, \, \dots, \, \mathcal{H}_k]$$.

The remaining tuple elements preserve physical and scale invariants:
- **$$z, r, d$$:** Target scale quantum {% include term.html id="zoom-z" text="Zoom z" %}, conserved residual {% include term.html id="remainder-r" text="Remainder r" %}, and coordinate frame {% include term.html id="dimension-d" text="Dimension d" %}.
- **$$\mathrm{DCC}_{\mathrm{ref}}$$:** Machine-readable {% include term.html id="dcc-profile" text="DCC Profile" %} governing transformation ceilings, execution budgets, and capability delegation rights.

### 2.2. Structural Hash-Consing and $\mathcal{O}(1)$ Equivalence
Node identity is strictly mathematical under {% include term.html id="dependencies-determinism" %}, operationalized via {% include term.html id="hash-consing" %}:

$$\mathcal{H}(\alpha) = \mathrm{Hash}\Big(\mathrm{OpCode} \,\|\, k \,\|\, \mathcal{H}_1 \,\|\, \dots \,\|\, \mathcal{H}_k \,\|\, z \,\|\, r \,\|\, d \,\|\, \mathrm{DCC}_{\mathrm{ref}}\Big)$$

Because the arity $$k$$ is explicitly serialized, nullary and unary operators produce canonical hashes with zero residual entropy ($$H(S_t \mid \mathcal{D}(S_t)) = 0$$).

**Canonical child ordering [v1.3.0 — CANDIDATE, Review Integration QO-5]:** When a node's `OpCode` is declared commutative under its {% include term.html id="dcc-profile" text="DCC Profile" %} (flag `DCC_SYM_COMMUTATIVE`), the child commitment vector is replaced by its lexicographically sorted image $$\sigma_{\mathrm{sym}}$$ prior to hashing:

$$\mathcal{H}(\alpha) = \mathrm{Hash}\Big(\mathrm{OpCode} \,\|\, k \,\|\, \sigma_{\mathrm{sym}}(\mathcal{H}_1, \dots, \mathcal{H}_k) \,\|\, z \,\|\, r \,\|\, d \,\|\, \mathrm{DCC}_{\mathrm{ref}}\Big)$$

so that $$A \circ B$$ and $$B \circ A$$ commit identically whenever the DCC declares $$\circ$$ commutative: one commitment exists per semantic value, and algebraic equality of commutative permutations collapses to the same $$\mathcal{O}(1)$$ machine-word comparison. Non-commutative opcodes hash in declaration order, unchanged. Because this changes commitment values, a DCC profile enabling sorting **MUST** declare a hashing-generation tag; commitments across distinct hashing generations compare only through signed migration receipts under {% include term.html id="structural-indirection" %} and the versioned-artifact discipline of **Axiom 13** — never by raw hash equality.

1. **Deduplication:** Any sub-expression with identical semantics, arity, and child commitments yields the identical commitment hash. Through {% include term.html id="hash-consing" %}, the runtime memory allocator maps it to the exact same physical node.
2. **$$\mathcal{O}(1)$$ Structural Equality:** Verifying whether two sub-expressions $$A$$ and $$B$$ are identical requires zero recursive graph traversal:

$$A \equiv B \iff \mathcal{H}(A) == \mathcal{H}(B)$$

Testing algebraic or semantic equality collapses to a single machine-word integer comparison.

### 2.3. Thermodynamic Work Minimization and Breakeven Bounds
In physical {% include term.html id="layer-0" %} substrates, allocating and deallocating memory across physical buses dissipates electrical capacitance ($$\mathcal{O}(C V^2 f)$$) and Landauer work ($$W \ge k_B T \ln 2$$).
* By enforcing {% include term.html id="hash-consing" %}, repeated sub-terms in multi-dimensional Clifford geometric products ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }})) or multi-agent dialogue graphs are allocated **exactly once**.
* Reversible symbolic normalization within the CAE-DAG incurs **zero Landauer dissipation** ($$W = 0$$) **within the reversible-symbolic domain only** [v1.4.0 — CANDIDATE] — no bit is erased in the abstract machine's state evolution, a claim that lives outside the admission path. It does not by itself operationalize {% include term.html id="q-reuse" text="Q(reuse)" %}: retrieving an existing commitment across L3 caches or DRAM always dissipates per the two-axis envelope of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4, and Q(reuse) harvests the Memory Wall only under the declared two-axis admission of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15, executing under the Admission Ledger of §5.9 (`W_net = W_actual_charge` — charged declared work, never a free operation). The Memory Wall is harvested, never suffered, and never for free.

#### The Two-Axis Consequentiality Breakeven **[v1.4.0 — CANDIDATE]**
Querying a cryptographic hash commitment across L3 caches or DRAM incurs declared lookup work (bus dissipation, $$\mathcal{O}(C V^2 f)$$) and declared lookup latency (the Memory Wall). Under the **Metric Invariant** (`simemp.md` §3.1) and the consequentiality constraint of [`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §4.4, hash-consing lookup via $$Q(\text{reuse})$$ is admissible if and only if its advantage over direct re-evaluation is declared on **both axes** of the Layer-0 Hardware Telemetry Envelope — the work axis and the latency axis, the **consequentiality constraint** ("consequently faster") — per the canonical admission predicate of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §1.15/§5.9:

`Admit(Q(reuse)) ⟺ ( ΔW_compute > κ_dcc · (τ_U[tier(h)] + ε_U) ) ∧ ( Δt_compute_bound > κ_t · (τ_t[tier(h)] + ε_t) )`

**Term provenance [v1.4.0 — CANDIDATE].** `h` is the referenced sub-expression commitment and `tier(h)` its declared memory tier, fixed by the node tuple's $$\mathrm{DCC}_{\mathrm{ref}}$$ storage binding (§2.1); `ΔW_compute` and `Δt_compute_bound` are the DCC-declared worst-case work and latency bounds of the direct re-evaluation of the referenced sub-expression under the profile's declared cost model (the per-opcode concrete bounds are supplied by the numeric substrate of [`docs/qm.md`]({{ '/qm.html' | relative_url }}) and, where applicable, the blade-multiplication scaling of [`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §8.1); `τ_U[tier(h)]` and `τ_t[tier(h)]` are the declared per-tier worst-case lookup bounds of the two-axis envelope, calibrated at Layer-0 onboarding; `ε_U` and `ε_t` are the declared gateway overhead slacks; `κ_dcc` and `κ_t` are the declared advantage margins within the governance-bounded range `[1, 64]` (recommended preset `κ_dcc = 2`, `κ_t = 4`, inherited unchanged from [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §5.9). All quantities are declared, never runtime-sampled; an undeclared quantity forces the safe Direct-Compute fallback, and ties resolve to Direct Compute. Every admitted reuse executes under the Admission Ledger of [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) §5.9.

If the direct path fails to clear either margin (such as single-cycle integer additions $$Q(2) + Q(3)$$ or trivial powers against an L1-resident commitment), the compiler bypasses hash-table probes, lowering the operation directly to integer machine instructions via the HCA-Collapse engine (§5); ties resolve to Direct Compute.

### 2.4. Acyclicity by Construction (Axiom 5)
In classical graphs, circular references create non-halting loops and stack crashes. In a Content-Addressed DAG, **a circular dependency is mathematically impossible**. Because $$\mathcal{H}(\alpha)$$ requires each child hash $$\mathcal{H}_i$$ as an input, no child can declare $$\alpha$$ as a dependency without breaking the pre-image resistance of cryptographic hash functions. Acyclicity along the {% include term.html id="causal-arrow" %} is guaranteed by construction, satisfying **Axiom 5** ([`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }})).

---

## 3. Elimination of Rounding and the Ontological Status of Irrationals

In classical floating-point systems (IEEE 754), rounding is an unmetered, lossy artifact of fixed register widths. Rounding silently discards trailing bits, generating non-deterministic drift and violating **Axiom 10**.

```
          CLASSICAL IEEE 754 vs. Qn CONSTRUCTIVE CONSERVATION
 Classical Lossy Rounding:
   1 / 3  ──►  0.33333333...  ──►  [ Truncated to 53 bits ]  ──►  Silent Entropy Loss
                                                                   (Drift accumulates)

 Qn Constructive Remainder Conservation (qm.md §4):
   1 / 3  ──►  Qexpr(OP_RATIO: 1/3)  ──►  Exact Symbolic State (Drift = 0)
                      │
                      ▼ Boundary Demand at Zoom z
               ⟨ ⊕, q_z, z, r_z, d ⟩ + Remainder Envelope
               • q_z : Discrete Integer Quotient
               • r_z : Conserved Residual (Total argument for scale z+1)
               • Zero bits discarded. Zero drift possible.
```

### 3.1. Prohibition of Rounding Drift
In the Qn universe, **implicit rounding is outlawed by the {% include term.html id="no-implicit-rule" %} and Axioms 9 and 10**:
1. Within Layer 3, expressions are held in unevaluated, exact symbolic form. Drift is identically zero.
2. When projected onto physical hardware boundaries, quantities are partitioned into an integer quotient $$q_z$$ and an **exact conserved remainder** $$r_z$$ ([`docs/qm.md`]({{ '/qm.html' | relative_url }}) §4):

$$X = \big( q_z \cdot \delta_z(d) \big) + r_z, \quad \mathrm{where} \; 0 \le \lvert r_z \rvert \lt \delta_z(d)$$

The remainder $$r_z$$ is never discarded; it is sealed within the {% include term.html id="universal-envelope" %} as the exact causal input state for subsequent precision expansions.

### 3.2. Ontological Status of Irrationals and Transcendentals
Under **Axiom 2**, actual infinities cannot exist as storable or executable machine states. Consequently:
* An irrational number **cannot be an infinite decimal string** ($$3.14159\dots$$), as storing infinite digits requires infinite energy.
* **Transcendental and irrational quantities exist permanently as `Qexpr` symbolic generators.**

```
       TRANSCENDENTAL & IRRATIONAL ONTOLOGY IN THE Qn STACK
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ LAYER 3: SYMBOLIC GENERATOR (Permanent Invariant State)                     │
 │ • Minimal Metric Root √2 : Qexpr(OP_ROOT: D² - Q(2) = Q(0), k=1)            │
 │ • Golden Ratio φ         : Qexpr(OP_ROOT: φ² - φ - Q(1) = Q(0), k=1)        │
 │ • Circular Ratio π       : Qexpr(OP_CF: [3; 7, 15, 1, 292, ...], k=0)       │
 │ • Base e                 : Qexpr(OP_CF: [2; 1, 2, 1, 1, 4, ...], k=0)       │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │               │ Boundary Collapse Demand (Target Zoom z, Budget W)          │
 │               ▼                                                             │
 │ LAYER 1 / 2: DISCRETE NUMERIC LEAF (On-Demand Physical Projection)          │
 │ • Quotient   : q_z ∈ ℕ (Exact discrete integer units at scale z)            │
 │ • Remainder  : r_z (Exact residual boundary; |r_z| < δ_z(d))                │
 │ • Terminal   : If r_z = Q(0), early halt via Terminal Exactness (ΔH = 0)    │
 └─────────────────────────────────────────────────────────────────────────────┘
```

1. **Algebraic Irrationals ($$\sqrt{2}, \phi$$):** Stored as exact polynomial constraint equations ($$D^2 - Q(2) = Q(0)$$) under `OP_ROOT` ($$k = 1$$). They are manipulated symbolically via radical identities with zero rounding drift.
2. **Transcendental Constants ($$\pi, \tau, e, \ln 2$$):** Stored as deterministic continued fraction recurrence generators under `OP_CF` ($$k = 0$$). Evaluating $$\pi$$ to zoom level $$z$$ computes an exact rational quotient and a conserved residual without floating-point error, verifying {% include term.html id="terminal-exactness" %} where remainders collapse.

---

## 4. The Six Canonical Node Classes

To achieve computational completeness across all abstraction layers, `Qexpr` defines **Six Canonical Node Classes**:

```
                       THE 6 CANONICAL Qexpr NODE CLASSES
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. OP_RATIO     : Symbolic rational pairs (p / q), k = 2.                   │
 │ 2. OP_ROOT      : Constrained Diophantine radical polynomials, k = 1.       │
 │ 3. OP_CF        : Projective continued fraction generators, k = 0.          │
 │ 4. OP_FOLD      : Bounded monoidal accumulation (∑, ∏, n!) over arrays, k≥3.│
 │ 5. OP_FRAME_MAP : Clifford geometric products and rotor conjugations, k = 2.│
 │ 6. OP_BYPASS    : Canonical identity contraction suite (confluent, O(1)).   │
 │    OP_SATURATE  : Bounded equality saturation (Q(rsi)-gated, versioned).    │
 └─────────────────────────────────────────────────────────────────────────────┘
```

### 4.1. Node Class 1: Exact Rational Pairs (`OP_RATIO`, $$k = 2$$)
Represents unevaluated rational fractions $$\frac{p}{q}$$ ($$q \neq Q(0)$$) without premature projection onto decimal or binary scale lattices.

Multiplicative and additive morphisms execute via exact Diophantine cross-multiplication:

$$\frac{p_1}{q_1} \times \frac{p_2}{q_2} = \frac{p_1 \times p_2}{q_1 \times q_2}$$

$$\frac{p_1}{q_1} + \frac{p_2}{q_2} = \frac{(p_1 \times q_2) + (p_2 \times q_1)}{q_1 \times q_2}$$

### 4.2. Node Class 2: Constrained Algebraic Roots (`OP_ROOT`, $$k = 1$$)
Represents algebraic radical roots as exact polynomial Diophantine equations ($$P(X) = Q(0)$$).

Algebraic roots (such as $$\sqrt{2}$$ via `DCC_ANCHOR_SQRT2_v1`) are held symbolically with a single operand commitment. Multiplication by identical roots ($$D \times D$$) resolves immediately to integer constants ($$Q(2)$$) via {% include term.html id="q-bypass" text="Q(bypass)" %} without invoking numerical root-extraction routines.

### 4.3. Node Class 3: Projective Recurrence Generators (`OP_CF`, $$k = 0$$)
Generates transcendental numbers ($$\pi, e, \tau, \ln 2$$) through deterministic continued fraction expansions:

$$X = a_0 + \cfrac{1}{a_1 + \cfrac{1}{a_2 + \cfrac{1}{\ddots}}}$$

Partial convergents $$\frac{p_m}{q_m}$$ are generated iteratively as nullary generators ($$k = 0$$) containing zero child dependencies. The truncation error is strictly bounded:

$$0 \le \lvert r_m \rvert \lt \frac{1}{q_m q_{m+1}}$$

and the residual is preserved in the remainder envelope.

### 4.4. Node Class 4: Bounded Monoidal Folds (`OP_FOLD`, $$k \ge 3$$)
Governs accumulation ($$\sum$$, $$\prod$$, vector concatenation, and discrete factorials $$n!$$) over arrays, streams, or multi-axial frames.

Requires an explicit step ceiling ($$m \le m_{\max}$$) and declared compute budget. The child vector $$\mathbf{D}_{\mathrm{children}}$$ encapsulates the full array of term commitments. Unbounded loops or infinite series are syntactically invalid under **Axiom 2** and **Axiom 6**.

### 4.5. Node Class 5: Frame Transformations and Permutations (`OP_FRAME_MAP`, $$k = 2$$)
Governs multi-axial Clifford geometric products, Cayley rotor conjugations ($$R \vec{v} R^\dagger$$), and coordinate permutations ($$S_n$$ group).

Rotors maintain unitary norm ($$\lVert R \rVert^2 \equiv Q(1)$$) by construction, eliminating trigonometric normalization cycles ([`docs/qm_geometry.md`]({{ '/qm_geometry.html' | relative_url }}) §5).

### 4.6. Node Class 6: Canonical Rewrite Operations (`OP_BYPASS` and `OP_SATURATE`, $$k \in \{1, 2\}$$) **[v1.3.0 — CANDIDATE]**

**[Review Integration QO-5]** Version 1.2.1 grouped two fundamentally different rewriting mechanisms under the single `OP_REWRITE` label: (a) a fixed, terminating, $$\mathcal{O}(1)$$ algebraic identity suite, and (b) general equality saturation. These carry opposite canonical semantics, and their conflation is not benign. Once bidirectional structural axioms (commutativity, associativity, distributivity) are admitted to a saturation e-graph, the e-graph can grow exponentially or without bound, and optimal DAG extraction from a saturated e-graph is NP-hard; two nodes saturating under different budgets or rule schedules can therefore extract different trees and commit different hashes — silently breaking $$\mathcal{O}(1)$$ hash-consing equivalence and $$H(S_t \mid \mathcal{D}(S_t)) = 0$$ (Tate et al. 2009; Willsey et al. 2021, `egg`). Node Class 6 is accordingly split into **two distinct operations**. The former umbrella name `OP_REWRITE` is retained as a **deprecated alias** (so that QnIR v0.1 and [`docs/qlog.md`]({{ '/qlog.html' | relative_url }}) references remain resolvable) and is retired at the QnIR v0.2 candidate.

#### 4.6.1. `OP_BYPASS` — Canonical Contraction
Executes reversible identity short-circuiting prior to numerical collapse. Under the **No-Implicit Rule**, the engine evaluates the full canonical suite of $$\mathcal{O}(1)$$ algebraic identity bypasses:

```
                    CANONICAL Q(bypass) IDENTITY RULES
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ ADDITIVE & MULTIPLICATIVE BYPASSES   │ EXPONENTIATION & RADICAL BYPASSES    │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • A + Q(0) ──► A                     │ • A^Q(0) ──► Q(1)   (A ≠ Q(0))       │
 │ • A × Q(1) ──► A                     │ • A^Q(1) ──► A                       │
 │ • A × Q(0) ──► Q(0)                  │ • Q(1)^k ──► Q(1)                    │
 │ • Q(0)! ──► Q(1)                     │ • Q(0)^k ──► Q(0)   (k > 0)          │
 │ • Q(1)! ──► Q(1)                     │ • ^Q(1)√(A) ──► A                    │
 │ • Q(2)! ──► Q(2)                     │ • ^n√(Q(1)) ──► Q(1)                 │
 ├──────────────────────────────────────┴──────────────────────────────────────┤
 │ GEOMETRIC & TOPOLOGICAL INVARIANTS                                          │
 │ • Rotor Normalization : R R† ──► Q(1)                                       │
 │ • Exterior Nilpotency : e_i ∧ e_i ──► Q(0)                                  │
 │ • Bivector Inversion  : (I_2)² ──► -Q(1)   (Grade 2 collapses to Grade 0)   │
 │ • Poincaré Coboundary : d ∘ d ──► Q(0)                                      │
 └─────────────────────────────────────────────────────────────────────────────┘
```

All listed transformations emit `RECEIPT_BYPASS_IDENTITY` at zero metric cost ($$\Delta W = 0$$), short-circuiting execution before any computation or memory lookup is scheduled.

**Rule-count reconciliation:** the canonical count is **16 rules**, including the first-root rule $$^{Q(1)}\sqrt{A} \to A$$. The condensed QnIR v0.1 review draft (PDF §4.3) printed 15 rules — the first-root row was lost in condensation; that PDF is a draft artifact and this specification is the authority.

Mandatory properties of the `OP_BYPASS` suite:

1. **Terminating:** every rule strictly reduces expression size or height, so the contraction always reaches a normal form in linear rule applications.
2. **Confluent [v1.3.0 — CANDIDATE, sealing item]:** the suite's critical pairs (e.g., $$Q(1)^{Q(1)}$$ under both $$A^{Q(1)} \to A$$ and $$Q(1)^k \to Q(1)$$) are locally joinable, and a unique normal form is asserted. A machine-checked confluence certificate (Newman's lemma over the critical-pair set) is a **QnIR v0.2 sealing prerequisite** under QO-5; until sealed, this property is a CANDIDATE proposition, not a theorem.
3. **Hash-governing:** the bypass normal form, together with the `DCC_SYM_COMMUTATIVE` child sorting of §2.2, is the input to canonical node hashing $$\mathcal{H}(\alpha)$$. One commitment exists per semantic value; `OP_BYPASS` never emits a new artifact — it discloses an identity that already held.
4. **Cost:** zero metric cost ($$\Delta W = 0$$), as receipted above.

The Layer-2 operational morphism and its execution precedence ($$Q(\text{bypass}) \prec Q(\text{reuse}) \prec \text{Direct Compute}$$) remain defined in [`docs/qnPrimitive.md`]({{ '/qnPrimitive.html' | relative_url }}) (Q(bypass) Optimization Morphism); this section governs the Layer-3 node-class semantics.

#### 4.6.2. `OP_SATURATE` — Bounded Equality Saturation
General equality saturation is a **workflow-scoped optimization operation**, never a canonical identity operation. It is legal **only** inside an explicit {% include term.html id="q-reuse" text="Q(reuse)" %}-class optimization workflow (`Q(rsi)`) whose {% include term.html id="dcc-profile" %} pins **all** of the following, without defaults (No-Implicit Rule):

1. **Deterministic rule-application schedule** over the declared rule set (e.g., fixed-epoch breadth-first Datalog scheduling); data-dependent or adaptive scheduling is invalid.
2. **Iteration ceiling** $$K_{\max}$$ (§6, circuit breaker 4).
3. **E-node ceiling** $$N_{\max}$$ and graph depth ceiling $$D_{\max}$$ (§6, circuit breakers 1–2).
4. **Deterministic polynomial-time greedy tree-cost extractor with hash tie-breaking.** Optimal extraction from a saturated e-graph is NP-hard, so "optimal extraction" is out of scope by construction; extraction is required to be greedy, polynomial-time, and totally tie-broken by commitment hash.

**Termination and receipts:** saturation halting before any ceiling emits `RECEIPT_SATURATION_CONVERGED` **[v1.3.0 — CANDIDATE]**; ceiling exhaustion emits the existing dissipative receipts `RECEIPT_BUDGET_EXHAUSTED` or `RECEIPT_EXPRESSION_TOO_COMPLEX` with reason codes `K_MAX_REACHED`, `N_MAX_REACHED`, or `D_MAX_REACHED`. Totality in the Axiom-6 sense is preserved: every saturation halts with a receipt.

**Artifact discipline:** the extractor's output is emitted as a **new versioned artifact (Axiom 13)** that records its input commitments, pinned schedule, ceilings, and extractor generation. `OP_SATURATE` never mutates canonical node identity and never participates in $$\mathcal{H}(\alpha)$$. Two saturation runs under different budgets or schedules MAY legitimately produce different versioned artifacts; this is measured, provenance-recorded divergence between artifacts — not state drift — provided each artifact carries its full saturation provenance.

---

## 5. Hardware Capability-Aware Collapse Engine (HCA-Collapse)

While `Qexpr` remains an abstract, substrate-agnostic DAG at Layer 3 via {% include term.html id="structural-indirection" %}, its physical execution on physical SoCs is governed by the **{% include term.html id="hca-collapse" text="Hardware Capability-Aware Collapse Engine (HCA-Collapse Engine)" %}**.

```
                               THE COMPILATION PIPELINE
 LAYER 3: INVARIANT ROAD (Qexpr CAE-DAG)
   Abstract, substrate-agnostic mathematical graph (Exact Diophantine Relations)
                               │
                               ▼ Hardware Capability-Aware Collapse Engine
 ══════════════════════════════╪══════════════════════════════════════════════════════
 HARDWARE-SPECIFIC LOWERING    │ (Reads Local DCC Hardware Profile: NPU / DSP / ASIC)
 ┌─────────────────────────────┴─────────────────────────────┐
 │ OPTION A: NPU / Systolic Array                            │ OPTION B: Integer ALU / RISC-V
 │ • Rewrite to integer Matrix-Multiply-Accumulate (MAC)     │ • Rewrite to CORDIC / Shift-and-Add
 │ • Constant-time, parallel execution over INT16/INT32      │ • Exact Diophantine reciprocal table
 └───────────────────────────────────────────────────────────┘
                               │
                               ▼
 LAYER 0 / 1: PHYSICAL SOC EXECUTION
   Zero floating-point drift · Constant-time execution · Bounded Landauer dissipation
```

### 5.1. Target-Specific Lowering Profiles
The compiler queries the node's local {% include term.html id="dcc-profile" text="DCC Hardware Profile" %} (established during Layer 0 onboarding):
1. **NPU / Systolic Arrays:** Lowers multi-axial Clifford geometric products ($$\vec{u} \cdot \vec{v} = \sum \eta_{jj} u_j v_j$$) directly into **integer Matrix-Multiply-Accumulate (MAC)** instructions (INT16/INT32), completing contractions in constant time.
2. **Cryptographic Co-Processors:** Offloads {% include term.html id="hash-consing" %} deduplication and node commitment hashing to hardware SHA-3 or BLAKE3 accelerators.
3. **Integer RISC-V / Microcontrollers:** Replaces division by multiplication using Barrett integer reciprocal tables with exact remainder correction loops.

### 5.2. Side-Channel Immunity via Constant-Time Compute
Variable-time operations leak secret cryptographic keys and algorithmic state via power analysis and cache timing. Lowering `Qexpr` nodes into fixed-cycle SoC instruction sequences ensures that execution duration depends strictly on declared metric budgets, providing hardware-level side-channel immunity under the {% include term.html id="security-envelope" %}.

### 5.3. Invariance Mandate
SoC-specific lowering is permissible if, and only if, the transformation is an **algebraic isomorphism**. Lowering passes that introduce floating-point approximations, mantissa truncations, or non-deterministic rounding are intercepted by the gateway and rejected, protecting {% include term.html id="existential-invariant" text="Existential Invariants" %}.

---

## 6. Bounded Execution and Fallibility Circuit Breakers

To guarantee that fallible human directives or unaligned agentic processes cannot compromise the execution environment, `Qexpr` enforces three structural circuit breakers:

```
                      Qexpr FALLIBILITY CIRCUIT BREAKERS
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ 1. STATIC GRAPH DEPTH CEILING                                               │
 │    Depth(DAG) ≤ Depth_max (Metered at ingestion; rejects runaway recursion) │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 2. NODE COUNT BUDGET ALLOCATION                                             │
 │    Count(Nodes) ≤ Node_max (Prevents memory exhaustion across Memory Wall)  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 3. METRIC COLLAPSE BUDGET CEILING                                           │
 │    Burn(U) ≤ Budget_Q1 (Guarantees deterministic halting via Axiom 6)       │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ 4. SATURATION ITERATION CEILING                                             │
 │    Epochs(OP_SATURATE) ≤ K_max (Bounds e-graph growth; deterministic halt)  │
 └─────────────────────────────────────────────────────────────────────────────┘
```

1. **Static Graph Depth Ceilings:** If an expression tree exceeds the declared depth ceiling ($$D \gt D_{\max}$$), execution terminates immediately with receipt `RECEIPT_EXPRESSION_TOO_COMPLEX`.
2. **Node Allocation Bounds:** If {% include term.html id="hash-consing" %} attempts to instantiate node counts exceeding the local {% include term.html id="metric-envelope" %}, compilation aborts with `RECEIPT_BUDGET_EXHAUSTED`.
3. **Totality by Budget (Axiom 6):** Every collapse operation carries an explicit compute budget in units of $$Q(1)$$ via {% include term.html id="metric-exhaustion" %}. Non-terminating expansions are impossible; when the budget is consumed, the engine emits a dissipative {% include term.html id="receipt" %} and halts.
4. **Saturation Iteration Ceilings [v1.3.0 — CANDIDATE]:** `OP_SATURATE` workflows (§4.6.2) carry an explicit iteration ceiling $$K_{\max}$$ alongside the node and depth ceilings. Hitting any ceiling halts deterministically with the corresponding reason code (`K_MAX_REACHED`, `N_MAX_REACHED`, `D_MAX_REACHED`), and the partially saturated state is discarded — only a receipted, extracted versioned artifact (Axiom 13) survives.

---

## 7. License and Invariant Lineage

This specification strictly conforms to the **[Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)**:

### 7.1. Paternity Reference
All derivative implementations, symbolic rewrite engines, compilers, and intermediate representation parsers derived from the {% include term.html id="licensed-work" %} must preserve the canonical {% include term.html id="paternity-reference" %}:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 7.2. Open Core Invariants
1. **Anti-Patent Defense:** Commercial or derivative licenses terminate automatically upon initiating patent litigation against the Author or project ecosystem.
2. **Non-Scaling Open Access:** Royalty-free licensing is guaranteed for non-commercial, academic, and sub-threshold usage under the {% include term.html id="ssl" %}.

### 7.3. Passive Infrastructure Safe Harbor
The `Qexpr` symbolic container acts as passive, neutral technical infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, models, or {% include term.html id="payload" text="Payloads" %}.
