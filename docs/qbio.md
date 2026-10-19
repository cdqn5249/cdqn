---
layout: default
title: Qbio — Quang Biology (Biosemiotics, Metabolic Homeostasis, and Active Inference)
description: Canonical domain specification for Qbio, formalizing the categorical coupling of chemistry and semantics, genetic code translation, Markov blankets, and active inference under SIMEMP constraints.
version: 1.0.1
updated: 2026-10-19
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qbio.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qbio.html
terms_used:
  - anchor-cut
  - boc-policy
  - cae-dag
  - complexity-degree
  - compute-unit-u
  - dependencies-determinism
  - discrete-exterior-calculus
  - historical-calibration-benchmark
  - historical-horizon
  - intent-contract
  - metric-exhaustion
  - non-rogue-invariant
  - open-core-invariant
  - payload
  - q-anchor
  - q-bypass
  - q-patterns
  - q-reuse
  - qbio
  - qchem
  - simemp-gateway
  - ssl
  - structural-indirection
---

# Qbio — Quang Biology: Biosemiotics, Metabolic Homeostasis, and Active Inference

| Field | Specification |
|---|---|
| **Document Title** | Qbio — Quang Biology: Biosemiotics, Metabolic Homeostasis, and Active Inference |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-10-18 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Candidate Domain Specification — Category D ({% include term.html id="complexity-degree" text="Complexity Degree 3" %}) |

---

## Normative References

The following documents establish the physical, structural, and legal constraints governing this specification. If a technical conflict arises, [`simemp.md`]({{ '/simemp.html' | relative_url }}) governs; if a structural conflict arises, [`abstractionLayers.md`]({{ '/abstractionLayers.html' | relative_url }}) governs; if a legal conflict arises, [`LICENSE.md`](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) governs.

| Document | Role | Target |
|---|---|---|
| `docs/simemp.md` | Constitutional constraints, thermodynamics, and [Dependencies Determinism]({{ '/glossary.html' | relative_url }}#dependencies-determinism) (v1.1.1) | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| `docs/abstractionLayers.md` | Layer architecture and [SIMEMP Gateway]({{ '/glossary.html' | relative_url }}#simemp-gateway) validation (v1.5.4) | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| `docs/qnPrimitive.md` | Universal Envelope, operational axioms, and [Optimization Morphisms]({{ '/glossary.html' | relative_url }}#q-reuse) (v1.2.2) | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| `docs/q0_q1.md` | Primary genesis origin [Q(0)]({{ '/glossary.html' | relative_url }}#q0) and first unit [Q(1)]({{ '/glossary.html' | relative_url }}#q1) (v1.1.2) | [q0_q1.html]({{ '/q0_q1.html' | relative_url }}) |
| `docs/q2_q9.md` | Single-digit secondary DCC anchors and parity partitions (v1.1.1) | [q2_q9.html]({{ '/q2_q9.html' | relative_url }}) |
| `docs/qm.md` | Constructive numeric leaf substrate and Diophantine division constraints (v1.2.9) | [qm.html]({{ '/qm.html' | relative_url }}) |
| `docs/qm_geometry.md` | Multi-axial frames, Clifford geometric algebra, and float-free rotations (v1.2.1) | [qm_geometry.html]({{ '/qm_geometry.html' | relative_url }}) |
| `docs/qm_calculus.md` | Discrete Exterior Calculus, continued fractions, and invariant dynamics (v1.1.0) | [qm_calculus.html]({{ '/qm_calculus.html' | relative_url }}) |
| `docs/qexpr.md` | Layer 3 Content-Addressed Expression DAG container (v1.1.1) | [qexpr.html]({{ '/qexpr.html' | relative_url }}) |
| `docs/qs.md` | Constructive relational intent and [Intent Contracts]({{ '/glossary.html' | relative_url }}#intent-contract) (v1.0.0) | [qs.html]({{ '/qs.html' | relative_url }}) |
| `docs/qlog.md` | Constructive type theory, pattern verification, and inferential modes (v1.0.4) | [qlog.html]({{ '/qlog.html' | relative_url }}) |
| `docs/qphy.md` | Discrete field dynamics, computational thermodynamics, and physical entropy (v1.0.3) | [qphy.html]({{ '/qphy.html' | relative_url }}) |
| `docs/qchem.md` | Discrete stoichiometry, molecular graphs, and reaction energetics (v1.0.1) | [qchem.html]({{ '/qchem.html' | relative_url }}) |
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Working Hypothesis Notice

### 1.1. Constructive Working Hypothesis Baseline
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It establishes **{% include term.html id="qbio" text="Qbio (Quang Biology)" %}** as an emergent biosemiotic domain derived from the categorical tensor product of chemistry and semantics:

$$\mathbf{C}(\text{Qbio}) \equiv \mathbf{C}(\text{Qchem}) \otimes \mathbf{C}(\text{Qs})$$

All formulations herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until biological simulation feedback is derived across living cellular runtimes. The architecture preserves structural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. The Biosemiotic Boundary: Life as Coded Chemistry
A living organism is not merely a complex chemical mixture; it is a **biosemiotic system** wherein molecular reaction networks are coupled to **informational code channels**:
- Abiotic molecules in {% include term.html id="qchem" %} react spontaneously under physical thermodynamic potential ($\Delta G \le 0$).
- Biological systems in $\mathrm{Qbio}$ process **informational intent**: nucleic acid sequences represent symbolic instructions that guide metabolic synthesis.

Living entities maintain non-equilibrium steady states against environmental entropy by consuming metabolic energy. Every biological state transition satisfies {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL CONTINUOUS BIOLOGY         │ Qbio CONSTRUCTIVE BIOSEMIOTICS       │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous floating-point ODEs and │ • Exact Diophantine metabolic graphs │
 │   stochastic gene network drifts.    │   and discrete codon translation.    │
 │ • Vitalist or heuristic agency      │ • Active inference under formal      │
 │   lacking thermodynamic metering.    │   Friston free energy bounds.        │
 │ • Unmetered growth loops ignoring    │ • Metabolic totality by budget:      │
 │   substrate Landauer limits.         │   ATP hydrolysis metered in U units. │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Categorical Formulation: Qchem Coupled to Qs

### 2.1. The Biosemiotic Tensor Product
$\mathrm{Qbio}$ is constructed constructively through the tensor product:

$$\mathbf{C}(\text{Qbio}) = \big( \mathbf{C}(\text{Qphy}) \otimes \mathbf{C}(\text{Qm}) \big) \otimes \mathbf{C}(\text{Qs})$$

- **The Chemical Substrate ($\mathrm{Qchem}$):** Provides discrete atomic molecular graphs, Diophantine stoichiometry ($\mathbf{S} \cdot \vec{v} \equiv Q(0)$), and Gibbs free energy dissipation ($\Delta G \le 0$).
- **The Semantic Channel ($\mathrm{Qs}$):** Provides symbolic intent contracts, functional pregroup grammars, and teleological agency (self-preservation and replication).

### 2.2. Genetic Code as Content-Addressed Translation
In $\mathrm{Qbio}$, DNA and RNA sequences are represented as linear **Content-Addressed Expression DAGs ({% include term.html id="cae-dag" %})**:
- **Codons as Discrete Tokens:** Triplet nucleotide bases $\{A, U, G, C\}^3$ form an exact discrete alphabet of 64 tokens.
- **The Translation Morphism:** Ribosomal translation is an invariant discrete mapping:

$$\mathcal{M}_{\mathrm{translation}}: \text{Codon}^{64} \longrightarrow \text{AminoAcid}^{20} \cup \{ \text{STOP} \}$$

- **Hash-Consing Deduplication:** Identical genetic motifs and conserved protein domains share the exact same cryptographic hash $\mathcal{H}(\text{Domain})$, enabling $\mathcal{O}(1)$ structural comparison and zero marginal Landauer allocation via {% include term.html id="q-reuse" %}.

---

## 3. Non-Equilibrium Metabolic Homeostasis and Free Energy

### 3.1. Variational Free Energy and Active Inference
A living organism persists by minimizing **Variational Free Energy** (Karl Friston formulation). Under `docs/simemp.md` (§4.2), an organism is an open dissipative system that avoids internal entropy maximization by generating predictive models of its environment:

$$F = \text{Internal Bound Energy} - T \cdot \text{Information Entropy}$$

1. **Active Inference Loops:** An organism generates actions to fulfill its metabolic {% include term.html id="intent-contract" %}, ensuring that environmental surprises (entropy spikes) remain bounded.
2. **Deterministic Selection via BOC:** Internal homeostatic adaptations are selected via the {% include term.html id="boc-policy" text="BOC Policy" %}, maximizing survival compliance while minimizing Landauer dissipation.

### 3.2. Entropy Export Across the Cellular Membrane
Living systems survive by acting as **Prigogine dissipative engines**:

$$dS = dS_{\text{int}} + dS_{\text{ext}}, \quad \text{where } dS_{\text{ext}} \lt 0$$

The cell consumes nutrient molecules possessing high chemical potential and excretes degraded waste products, exporting operational entropy via signed metabolic receipts ($\mathcal{R}_{\mathrm{metabolic}}$).

---

## 4. The Canonical Biological Q(anchor) Spectrum

$\mathrm{Qbio}$ grounds living systems in five universal {% include term.html id="q-anchor" text="Q(anchor)" %} entities derived from the {% include term.html id="historical-calibration-benchmark" %}:

<pre>
                  THE CANONICAL BIOLOGICAL Q(anchor) SPECTRUM
 ┌──────────┬─────────────────────────────┬────────────────────────────────────┐
 │ ARTIFACT │ ANCHOR CONTRACT DESIGNATION │ INVARIANT BIOLOGICAL REALITY       │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(codon) │ DCC_ANCHOR_GENETIC_CODE_v1  │ Discrete Universal Genetic Code:   │
 │          │ (Codon-to-Peptide Mapping)  │ Exact 64-triplet to 20-amino map.  │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(blanket│ DCC_ANCHOR_MARKOV_BLANKET_v1│ Topological Boundary Isolation:    │
 │          │ (Membrane Containment)      │ H(S_int | S_blanket, S_ext) = 0.   │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(atp)   │ DCC_ANCHOR_METABOLIC_ATP_v1 │ Cellular Energy Currency:          │
 │          │ (Hydrolysis Work Quantum)   │ ΔG_ATP ≈ 20 U per reaction event.  │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(homeo) │ DCC_ANCHOR_HOMEOSTASIS_v1   │ Active Inference & Free Energy:    │
 │          │ (Variational Free Energy)   │ Internal entropy bounded: S ≤ S_max│
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(autocat│ DCC_ANCHOR_AUTOCATALYTIC_v1 │ Autocatalytic Network Closure:     │
 │          │ (Self-Sustaining Cycles)    │ Closed catalytic loops in Qchem.   │
 └──────────┴─────────────────────────────┴────────────────────────────────────┘
</pre>

### 4.1. Universal Biological Anchors and Capability Contracts
- **`Q(codon)` (The Genetic Code Anchor):** Enforces the universal translation mapping between nucleotide sequences and peptide synthesis without unmetered amino acid drift.
- **`Q(blanket)` (The Markov Blanket Anchor):** Formalizes cellular membranes as topological boundaries ensuring statistical and causal insulation between internal physiology and external noise.
- **`Q(atp)` (The Metabolic Work Quantum):** Calibrates ATP hydrolysis to the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}: $\Delta G_{\mathrm{ATP}} \approx 20 \, U$, grounding metabolic work directly into Landauer limits ($W \ge k_B T \ln 2$).
- **`Q(homeo)` (The Homeostasis Anchor):** Enforces hard ceilings on internal operational entropy, preventing metabolic collapse.
- **`Q(autocat)` (The Autocatalytic Closure Anchor):** Enforces self-sustaining reaction loops where metabolic enzymes catalyze their own production.

### 4.2. Biological Inconsistency Cascade and Metabolic Halting
If a biological simulation or genetic algorithm proposes a non-viable organism:
1. **Immediate Inconsistency Cascade:** The homeostasis or ATP anchor destabilizes ($\chi \lt \Theta_\chi$), threatening systemic physiological collapse.
2. **Gateway Interception:** The {% include term.html id="simemp-gateway" %} halts execution deterministically, emitting `RECEIPT_METABOLIC_STARVATION` or `RECEIPT_HOMEOSTATIC_COLLAPSE`.
3. **Organism Death as Clean Halting:** Under **Axiom 6** (Totality by Budget), biological death is modeled as an explicit, bounded terminal receipt, preventing unmetered necrotic execution loops.

---

## 5. Morphogenesis on Discrete Simplicial Complexes

### 5.1. Reaction-Diffusion Cochain Systems
Morphogenesis (e.g., Turing pattern formation, embryonic tissue layering) is modeled in $\mathrm{Qbio}$ using Discrete Exterior Calculus ({% include term.html id="discrete-exterior-calculus" text="DEC" %}) on simplicial meshes $K$ (`docs/qm_calculus.md`):
- Morphogen concentrations are represented as discrete $0$-cochains on mesh vertices.
- Diffusion is evaluated via the discrete Laplace–Beltrami operator: $\Delta = \mathbf{d}\delta + \delta\mathbf{d}$.
- Chemical activation and inhibition evaluate through Diophantine integer kinetics in $\mathrm{Qchem}$.

### 5.2. Topological Closure and the Markov Blanket
A biological tissue or single cell possesses an explicit **Markov Blanket**:
- The boundary is discretized as an oriented $(n-1)$-simplicial sub-complex $\partial K$.
- Information and chemical transport across the boundary must cross an explicit gateway.
- Sensory inputs and active motor outputs are metered, preventing unmodeled external state drift ($H(S_{\text{int}} \mid S_{\text{blanket}}, S_{\text{ext}}) = 0$).

---

## 6. Metabolic Pathways as Q(workflows) and Catalytic Networks

### 6.1. Enzymatic Cascades as Q(patterns)
An enzymatic transformation (e.g., glycolysis, Krebs cycle step) is an invariant **{% include term.html id="q-patterns" text="Q(pattern)" %}**:

$$\mathcal{P}_{\mathrm{enzyme}}: \mathcal{H}(\text{Substrate}) \xrightarrow{\quad \mathcal{M}_{\mathrm{enzyme}} \quad} \mathcal{H}(\text{Product})$$

Each enzyme acts as an execution contract requiring an explicit co-factor and an ATP energy packet, dissipating heat according to $\mathrm{Qphy}$ constraints.

### 6.2. Autocatalytic Sets and Category B Optimization
Complex metabolic networks form closed autocatalytic sets (Gánti Chemoton model):
- In $\mathrm{Qbio}$, recurring metabolic cycles leverage {% include term.html id="q-reuse" %}, retrieving cached intermediate enzyme hashes rather than reallocating memory bits across the Memory Wall.
- Allosteric regulation operates via {% include term.html id="q-bypass" %}: feedback inhibition short-circuits redundant metabolic synthesis in $\mathcal{O}(1)$ dispatch time once concentration thresholds are satisfied.

---

## 7. Biological Agency and the Non-Rogue Invariant

### 7.1. Cellular Agency Bounded by Intent Contracts
Biological agency (chemotaxis, immune defense, cellular differentiation) is governed by **Structural Intent Contracts** (`docs/qs.md` §5):
- An immune cell hunting a pathogen operates under an explicit capability contract (`DCC_BIO_IMMUNE_v1`).
- The cell possesses finite authority: it cannot mutate its foundational genesis origin $Q(0)$ or attack self-tissue without violating its DCC boundary.

### 7.2. The Thermodynamic Death Barrier (Metabolic Exhaustion)
Under the **{% include term.html id="non-rogue-invariant" %}**:
1. No biological agent or synthetic cellular automaton can achieve unmetered runaway growth.
2. Every cellular division and signaling step burns an explicit work quota in units of $U$ via {% include term.html id="metric-exhaustion" %}.
3. When local ATP or nutrient budgets reach zero, the agent physically halts, emitting an immutable termination receipt (`RECEIPT_METABOLIC_EXHAUSTION`).

---

## 8. Causal Trajectories: Forward Development vs. Inverse Phylogenetics

$\mathrm{Qbio}$ accommodates both directions across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qbio
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD DEVELOPMENT : Morphogenesis (Zygote ──► Organism)                   │
 │ • Domain : Embryology, synthetic biology, tissue engineering.               │
 │ • Pipeline: Genetic Code ──► Q(workflows) ──► Verified Phenotype.           │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Phylogenetics (Observed Organism ──► Ancestor)     │
 │ • Domain : Paleontology, molecular evolution, viral tracking.               │
 │ • Pipeline: Genomic Receipt S_obs ──► Qlog Abduction ──► Last Common Ancestor│
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 8.1. Forward Embryogenesis and Morphogenesis (Q(workflows))
Forward biological development compiles an initial genetic program into a mature multicellular phenotype:
1. Zygote state $S_0$ is initialized with genetic expression DAGs.
2. Cell divisions and tissue differentiation proceed as an assembled `Q(workflow)`.
3. Checkpoints are audited using the {% include term.html id="anchor-cut" text="Anchor-Cut Invariant" %}, verifying developmental fidelity in $\mathcal{O}(K)$ receipt validations without tracking billions of individual cellular micro-steps.

### 8.2. Inverse Causal Reconstruction of Evolutionary Lineage
In evolutionary biology across the {% include term.html id="historical-horizon" %}, researchers measure modern genomic sequences $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`) while ancestral lineages remain unobserved:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle S_0, \, \mathcal{W}_{\mathrm{phylogenetic}} \big\rangle \quad \text{such that} \quad \mathcal{W}(S_0) \equiv S_{\mathrm{obs}}$$

1. **Receipt Ingestion:** Sequenced modern DNA is ingested via verified laboratory sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic mutation models propose ancestral genomes ($A_1 \dots A_n$).
3. **BOC Abductive Resolution:** The {% include term.html id="boc-policy" text="BOC Policy" %} selects the phylogenetic tree that maximizes SIMEMP compliance, selecting the evolutionary trajectory that minimizes parsimony mutations and thermodynamic dissipation.
4. **Lineage Sealing:** The reconstructed ancestral node is sealed as an emergent `Q(pattern)` in the evolutionary tree.

---

## 9. License and Lineage Alignment

### 9.1. Paternity Reference
All derivative biological simulation models, synthetic biology compilers, metabolic solvers, and genetic execution modules derived from $\mathrm{Qbio}$ must embed the canonical attribution:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 9.2. Open Core Invariants
The core biological translation rules, discrete metabolic mechanics, and active inference algorithms remain royalty-free for academic, non-commercial, and sub-threshold deployments under the {% include term.html id="open-core-invariant" text="Open Core Invariants" %} of the {% include term.html id="ssl" %}.

### 9.3. Statutory Safe Harbor and Passive Infrastructure
$\mathrm{Qbio}$ operates as passive, neutral biological and computational modeling infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external genomic datasets, or sovereign {% include term.html id="payload" text="Payloads" %}.
