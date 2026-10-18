---
layout: default
title: Qchem — Quang Chemistry (Discrete Stoichiometry, Molecular Graphs, and Reaction Energetics)
description: Canonical domain specification for Qchem, formalizing discrete Diophantine stoichiometry, content-addressed molecular graphs, stereochemical Clifford chirality, and reaction kinetics under SIMEMP constraints.
version: 1.0.1
updated: 2026-10-18
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qchem.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qchem.html
terms_used:
  - qchem
  - qphy
  - qm
  - qs
  - qlog
  - qn
  - q0
  - q1
  - compute-unit-u
  - dependencies-determinism
  - no-implicit-rule
  - structural-indirection
  - simemp
  - universal-envelope
  - metric-envelope
  - security-envelope
  - receipt
  - causal-arrow
  - complexity-degree
  - local-first
  - terminal-exactness
  - zoom-z
  - remainder-r
  - dimension-d
  - q-anchor
  - q-reuse
  - q-bypass
  - qexpr
  - cae-dag
  - hash-consing
  - epistemic-anchorage
  - intent-contract
  - non-rogue-invariant
  - qn-rsi
  - boc-policy
  - existential-invariant
  - operational-agility
  - ssl
  - paternity-reference
  - payload
  - licensed-work
  - simemp-gateway
  - discrete-exterior-calculus
  - anchor-cut
  - layer-0
  - layer-1
  - identity-class
  - chronosa
  - q-patterns
  - q-workflows
  - q-event
  - historical-calibration-benchmark
  - historical-horizon
---

# Qchem — Quang Chemistry: Discrete Stoichiometry, Molecular Graphs, and Reaction Energetics

| Field | Specification |
|---|---|
| **Document Title** | Qchem — Quang Chemistry: Discrete Stoichiometry, Molecular Graphs, and Reaction Energetics |
| **Version** | 1.0.1 |
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
| `LICENSE.md` | Scaling Source License 1.0 governing the [Licensed Work]({{ '/glossary.html' | relative_url }}#licensed-work) | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

---

## 1. Epistemic Stance and Working Hypothesis Notice

### 1.1. Constructive Working Hypothesis Baseline
This specification is formulated within the Recursive Search Loop ([`docs/simemp.md`]({{ '/simemp.html' | relative_url }}) §2). It establishes **$\mathrm{Qchem}$ (Quang Chemistry)** as an emergent composite domain formed by the categorical tensor product of physical thermodynamics and constructive mathematics:

$$\mathbf{C}(\text{Qchem}) \equiv \mathbf{C}(\text{Qphy}) \otimes \mathbf{C}(\text{Qm})$$

All formulations herein represent provisionally consistent hypotheses. In accordance with the Anti-Calcification Principle, permanent finality is reserved until physical execution feedback is derived across chemical simulation runtimes. The architecture preserves structural elasticity through {% include term.html id="structural-indirection" %}.

### 1.2. Abiotic Boundary and the Exclusion of Semantic Intent
Chemistry models the organization of matter governed by valence electrons, molecular bonds, and thermodynamic equilibria. Unlike biological or cognitive domains, **abiotic chemical matter possesses zero semantic agency, legal liability, or teleological intent**:

$$\mathbf{C}(\text{Qchem}) \cap \mathbf{C}(\text{Qs}) \equiv \emptyset$$

Chemical reaction steps transition spontaneously under physical thermodynamic potential ($\Delta G \le 0$) and Diophantine stoichiometric balance. Attributing intent ($\mathrm{Qs}$) to abiotic reactions is a category error. All transitions satisfy {% include term.html id="dependencies-determinism" %}:

$$H(S_t \mid \mathcal{D}(S_t)) = 0$$

<pre>
 ┌──────────────────────────────────────┬──────────────────────────────────────┐
 │ CLASSICAL COMPUTATIONAL CHEMISTRY    │ Qchem CONSTRUCTIVE CHEMICAL PHYSICS  │
 ├──────────────────────────────────────┼──────────────────────────────────────┤
 │ • Continuous floating-point energy   │ • Exact Diophantine stoichiometry    │
 │   surfaces with accumulated drift.   │   matrix balance: S · v ≡ Q(0).      │
 │ • Lossy atomic coordinate models     │ • Content-Addressed Molecular DAGs   │
 │   vulnerable to geometric roundoff.  │   with exact Clifford stereochemistry│
 │ • Unmetered simulation loops         │ • Reaction steps as Q(patterns);     │
 │   ignoring Landauer dissipation.     │   catalysis as Category B Q(bypass). │
 └──────────────────────────────────────┴──────────────────────────────────────┘
</pre>

---

## 2. Discrete Molecular Graphs and Stereochemistry

### 2.1. Atoms and Valence Bonds as Content-Addressed DAGs
Molecules are represented within {% include term.html id="qexpr" %} Content-Addressed Expression DAGs ({% include term.html id="cae-dag" %}):
- **Atomic Vertices:** Each atom is an immutable leaf artifact carrying atomic number $Z \in \mathbb{N}$, discrete mass number $A \in \mathbb{N}$, and formal oxidation state.
- **Valence Bonds as Clifford Blades:** Covalent, ionic, and coordination bonds are modeled as localized Clifford multivector blades (`docs/qm_geometry.md`), declaring discrete bond order ($1, 2, 3$) and directional orientation.
- **Deduplication via Hash-Consing:** Identical molecular sub-structures share identical commitment hashes $\mathcal{H}(\text{Molecule})$. Common functional groups are allocated once in memory, maximizing {% include term.html id="q-reuse" %} at zero marginal Landauer cost ($\Delta W = 0$).

### 2.2. Stereochemical Chirality via Clifford Sign Parity (σ)
Enantiomers (chiral mirror-image molecules such as D- and L-amino acids) share identical scalar connectivity, bond lengths, and atom counts. Classical cheminformatics often relies on ambiguous textual conventions (R/S or D/L labels).

In $\mathrm{Qchem}$, molecular chirality is an exact algebraic invariant governed by the spatial reflection involution $\mathcal{I}_d$ and the 3D pseudoscalar $I_3 = \vec{e}_1 \wedge \vec{e}_2 \wedge \vec{e}_3$:
- The spatial inversion of a chiral center inverts its oriented grade-3 volume blade.
- The resulting enantiomer is distinguished by its **Sign Parity** ($\sigma \in \{\ominus, \oplus\}$).
- Enantiomeric transformations preserve exact unitary norms without floating-point trigonometric drift.

---

## 3. Exact Diophantine Stoichiometry

### 3.1. The Stoichiometric Null-Space Invariant
In classical software, reaction mass balances frequently accumulate numerical roundoff errors. In $\mathrm{Qchem}$, chemical reactions are **exact Diophantine balance equations**:
- Let $M$ elements react across $N$ molecular species. The stoichiometric matrix $\mathbf{S} \in \mathbb{Z}^{M \times N}$ maps integer elemental coefficients.
- A valid chemical reaction flux vector $\vec{v} \in \mathbb{N}^N$ satisfies the Diophantine null-space constraint under {% include term.html id="terminal-exactness" %} ($r = Q(0)$):

$$\mathbf{S} \cdot \vec{v} \equiv Q(0)$$

Elemental mass, atom counts, and net electric charges balance identically. Truncation error is zero by construction.

### 3.2. Redox Electron Quanta and Charge Conservation
Oxidation-reduction reactions involve discrete electron transfers:
- Electrons exist as indivisible integer charge carriers ($z \in \mathbb{Z}$). Fractional electron transfers are barred.
- The net charge transition across reactants and products evaluates to genesis origin $Q(0)$, guaranteeing exact conservation of electric charge under `DCC_ANCHOR_DEC_D_v1`.

---

## 4. Non-Equilibrium Chemical Thermodynamics

### 4.1. Gibbs Free Energy Reaction Spontaneity
Chemical reaction feasibility is governed by **Non-Equilibrium Thermodynamics** (`docs/qphy.md` §4):
- **Spontaneity Criterion:** At constant temperature and pressure, an abiotic chemical reaction transitions spontaneously if and only if its change in Gibbs free energy is negative or zero:

$$\Delta G = \Delta H - T \Delta S \le Q(0)$$

- **Thermal Activation Barriers ($E_a$):** Reactions possessing an activation energy barrier require absorbing a discrete thermal work packet from the local environment, metered by the local {% include term.html id="metric-envelope" %}.

### 4.2. Chemical Equilibrium as Continued Fraction Convergents
Chemical equilibrium constants ($K_{\mathrm{eq}}$) are held symbolically inside `Qexpr` as deterministic continued fraction partial convergents (`OP_CF`):

$$K_{\mathrm{eq}} = [a_0; \, a_1, \, a_2, \, \dots]$$

Evaluating reaction quotients to zoom level $z$ generates exact rational pairs without continuous exponential floating-point approximations.

---

## 5. The Chemical Q(anchor) Spectrum

$\mathrm{Qchem}$ grounds its physical and chemical constants in universal {% include term.html id="q-anchor" text="Q(anchor)" %} entities derived from the {% include term.html id="historical-calibration-benchmark" %}:

<pre>
                  THE CANONICAL CHEMICAL Q(anchor) SPECTRUM
 ┌──────────┬─────────────────────────────┬────────────────────────────────────┐
 │ ARTIFACT │ ANCHOR CONTRACT DESIGNATION │ INVARIANT CHEMICAL REALITY         │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(N_A)   │ DCC_ANCHOR_AVOGADRO_v1      │ Exact Integer Mole Entity Count:   │
 │          │ (Avogadro Constant)         │ N_A ≡ 602,214,076 × 10¹⁵ mol⁻¹     │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(stoich)│ DCC_ANCHOR_STOICHIOMETRY_v1 │ Lavoisier-Dalton Mass Invariant:   │
 │          │ (Atom Count Conservation)   │ Diophantine null-space S·v ≡ Q(0). │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(th_2)  │ DCC_ANCHOR_SECOND_LAW_v1    │ Gibbs Dissipation Spontaneity:     │
 │          │ (Thermodynamic Invariant)   │ ΔG ≤ Q(0) (Inherited from Qphy).   │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(e)     │ DCC_ANCHOR_E_CHARGE_v1      │ Discrete Faraday Redox Quanta:     │
 │          │ (Elementary Charge Unit)    │ Integer electron exchange (z ∈ ℤ). │
 ├──────────┼─────────────────────────────┼────────────────────────────────────┤
 │ Q(chiral)│ DCC_ANCHOR_CHIRALITY_v1     │ Pasteur Stereochemical Symmetry:   │
 │          │ (Enantiomeric Parity)       │ Clifford reflection involution I_d.│
 └──────────┴─────────────────────────────┴────────────────────────────────────┘
</pre>

### 5.1. Universal Chemical Anchors and Capability Contracts
- **`Q(N_A)` (The Avogadro Anchor):** Formalizes the exact discrete bridge between microscopic atomic integer counts and macroscopic molar thermodynamics: $N_A \equiv 602{,}214{,}076 \times 10^{15}\text{ entities/mol}$.
- **`Q(stoich)` (The Stoichiometric Anchor):** Enforces nuclear and elemental conservation across chemical transformations without fractional atom leakage.
- **`Q(th_2)` (The Second Law Gibbs Anchor):** Mandates that endergonic reactions ($\Delta G \gt 0$) cannot occur without coupled exergonic work.
- **`Q(e)` (The Redox Charge Anchor):** Restricts electrochemical electron transfers to discrete integer quanta of $Q(1)$.
- **`Q(chiral)` (The Stereochemical Anchor):** Enforces spatial reflection parity over chiral tetrahedral carbon centers.

### 5.2. Chemical Inconsistency Cascade and Gateway Interception
If an automated reaction proposal or molecular simulation claims an unphysical reaction:
1. **Immediate Inconsistency Cascade:** The stoichiometric or thermodynamic anchor destabilizes, threatening downstream mass-action models.
2. **Gateway Interception:** The {% include term.html id="simemp-gateway" %} intercepts the invalid balance, emitting `RECEIPT_STOICHIOMETRIC_VIOLATION` or `RECEIPT_THERMODYNAMIC_EXHAUSTION`.
3. **Execution Termination:** The invalid reaction workflow is rejected before consuming physical compute or dispatching automated laboratory synthesis actuators.

---

## 6. Reaction Kinetics as Q(patterns) and Q(workflows)

### 6.1. Elementary Reaction Steps as Q(patterns)
An elementary chemical reaction step is an invariant **{% include term.html id="q-patterns" text="Q(pattern)" %}**:

$$\mathcal{P}_{\mathrm{rxn}}: \mathcal{H}(\text{Reactants}) \xrightarrow{\quad \mathcal{M}_{\mathrm{kinetics}} \quad} \mathcal{H}(\text{Products})$$

It defines a reproducible, discrete transformation mapping reactant commitment hashes to product hashes under explicit thermodynamic constraints.

### 6.2. Reaction Cascades as Q(workflows) and Catalysis as Q(bypass)
Multi-step chemical syntheses, polymerizations, and reaction networks are assembled as **{% include term.html id="q-workflows" text="Q(workflows)" %}**:
- **Work Budgets:** Reaction sequences operate within a declared Metric Envelope, tracking thermal work and Landauer dissipation.
- **Catalytic Optimization via {% include term.html id="q-bypass" %}:** A catalyst lowers the activation energy barrier ($E_a' \ll E_a$) without altering overall equilibrium ($\Delta G$). In $\mathrm{Qchem}$, catalysis is formalized as an execution short-circuit: the gateway recognizes the catalytic agent, bypasses the slow uncatalyzed route, and emits `RECEIPT_CATALYTIC_BYPASS` in $\mathcal{O}(1)$ dispatch time.

---

## 7. The Boundary Between Qchem and Qbio

### 7.1. Stratification of Abiotic Matter vs. Living Systems
To preserve the **No-Implicit Rule**, the architecture maintains a strict epistemic boundary between chemistry and biology:
- **$\mathrm{Qchem}$ (Abiotic Chemistry):** Models molecular graphs, valence interactions, and reaction kinetics. It operates with zero intentional agency ($\mathrm{Qs} = \emptyset$).
- **{% include term.html id="qbio" text="Qbio" %} (Living Biosemiotics):** Emerges when chemical networks are coupled to **informational semantics ($\mathrm{Qs}$)**:

$$\mathbf{C}(\text{Qbio}) \equiv \mathbf{C}(\text{Qchem}) \otimes \mathbf{C}(\text{Qs})$$

### 7.2. The Biosemiotic Leap
Living systems differ from abiotic chemistry because their molecules serve as **informational codes**:
- An abiotic crystal grows purely via $\Delta G$ minimization ($\mathrm{Qchem}$).
- A strand of DNA guides protein synthesis via a codon translation code ($\mathrm{Qbio} = \mathrm{Qchem} \otimes \mathrm{Qs}$).
- Active inference and metabolic regulation introduce teleological intent contracts, which are strictly barred from abiotic $\mathrm{Qchem}$.

---

## 8. Causal Trajectories: Forward Synthesis vs. Inverse Spectroscopy

$\mathrm{Qchem}$ accommodates both directions across the causal arrow:

<pre>
               THE DUAL CAUSAL DIRECTIONS IN Qchem
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ FORWARD SYNTHESIS : Retrosynthesis Planning (Reactants ──► Target Molecule) │
 │ • Domain : Chemical manufacturing, automated synthesis, materials design.   │
 │ • Pipeline: Precursors ──► Q(workflows) ──► Verified Product Molecule.      │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ INVERSE RECONSTRUCTION : Analytical Chemistry (Spectrogram ──► Molecule)   │
 │ • Domain : Mass spectrometry, NMR spectroscopy, environmental forensics.    │
 │ • Pipeline: Spectral Receipt S_obs ──► Qlog Abduction ──► Molecular Graph   │
 └─────────────────────────────────────────────────────────────────────────────┘
</pre>

### 8.1. Forward Synthesis Planning (Q(workflows))
Forward chemical synthesis compiles a path from available chemical precursors to a target molecule:
1. Target molecular graph $\mathcal{H}(\text{Target})$ is declared.
2. $\mathrm{Qlog}$ compiles a sequence of elementary reaction `Q(patterns)`.
3. Each step verifies $\Delta G \le Q(0)$ and stoichiometric balance, emitting a chained road receipt vector.

### 8.2. Inverse Causal Reconstruction of Molecular Graphs
In analytical chemistry across the {% include term.html id="historical-horizon" %}, an instrument measures an empirical spectral signature $S_{\mathrm{obs}}$ (anchored as `MODAL_EMPIRICAL`, e.g., mass-to-charge ratios $m/z$, NMR chemical shifts) while the molecular identity remains unknown:

$$\text{Given } S_{\mathrm{obs}} \implies \text{Find } \big\langle \mathcal{H}(\text{Molecule}), \, \mathcal{M}_{\mathrm{frag}} \big\rangle \quad \text{such that} \quad \mathcal{M}(\text{Molecule}) \equiv S_{\mathrm{obs}}$$

1. **Receipt Ingestion:** Mass spectrometer readings are anchored as verified sensor receipts (`MODAL_EMPIRICAL`, $G = 1.0$).
2. **Candidate Hypothesis Generation:** Stochastic or neural engines propose candidate molecular graphs ($M_1 \dots M_n$).
3. **BOC Abductive Resolution:** The {% include term.html id="boc-policy" text="BOC Policy" %} selects the candidate graph that matches the observed fragmentation peaks while minimizing unmeasured conditional entropy ($H(S \mid \mathcal{D}) \to 0$).
4. **Structure Sealing:** The reconstructed molecular graph is sealed into the local chemical registry.

---

## 9. License and Lineage Alignment

### 9.1. Paternity Reference
All derivative chemical simulation models, synthesis planners, spectroscopy engines, and intermediate execution modules derived from $\mathrm{Qchem}$ must embed the canonical attribution:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

### 9.2. Open Core Invariants
The core chemical constants, discrete stoichiometry solvers, and thermodynamic balance rules remain royalty-free for academic, non-commercial, and sub-threshold deployments under the {% include term.html id="open-core-invariant" text="Open Core Invariants" %} of the {% include term.html id="ssl" %}.

### 9.3. Statutory Safe Harbor and Passive Infrastructure
$\mathrm{Qchem}$ operates as passive, neutral chemical and computational modeling infrastructure. It exercises no editorial inspection over, and makes no claim upon, sovereign user content, external chemical libraries, or sovereign {% include term.html id="payload" text="Payloads" %}.
