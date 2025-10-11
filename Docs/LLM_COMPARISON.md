# LLM_COMPARISON.md
> Updated: CDQN Design Phase — Chronosa role-assembly edition  
> Purpose: Clarify where Large Language Models (LLMs) are useful in CDQN, what their limitations are, and how they compare to Chronosa role-assembly micro-reasoners and specialized lightweight solvers.

## Executive summary
- LLMs are powerful at **unstructured natural-language tasks** (summaries, translations, user-facing explanations).  
- For **causal, verifiable, low-data, high-precision reasoning** (theorem discovery, backward validation, reputation-sensitive actions), the CDQN design favors **role-assembly micro-reasoners** (tiny recursive models / deterministic functions) governed by Chronosa and validated by CDU evidence.  
- Use LLMs as *interfaces* and *summarizers*, not as primary validators of network state or theorem provenance. External papers on tiny recursive reasoning and interpolating neural networks are useful inspiration, but we adopt only the ideas that fit our causal & feedback-first model. 45

---

## Comparison matrix (high level)

| Capability / Property | Large LLMs (e.g., GPT-family, Gemini) | Chronosa role-assembly (tiny recursive + pure functions) | Specialized solvers (INN-like / domain models) |
|---|---:|---:|---:|
| Natural language understanding & generation | Excellent | Good (via lightweight interface) | Poor / unnecessary |
| Deterministic, auditable reasoning (theorem proofs) | Weak (probabilistic, hallucination risk) | Strong (CDU-evidence driven, traceable) | Strong for domain-specific PDE/physics |
| Data efficiency | Low (needs huge pretraining) | High (designed to work with local CDU feedback) | High for structured physics problems |
| Explainability & provenance | Low — needs post-hoc tools | High — every step emits signed CDUs & pointers | High (if designed with interpolation) |
| Compute & energy cost | High (esp. at scale) | Low (adds/multiplies, bounded recursion) | Low–medium (domain-specific efficiency) |
| Best CDQN role(s) | UI / policy-level summarizer / translator | Proposer / Verifier / Backward-Validator / Policy | Scientific modules, sim solvers, signal processing |

Notes:
- Tiny recursive/role modules have been shown to outperform LLMs on specific reasoning puzzle tasks when constrained data is available (TRM/HRM research) — we use this as inspiration for Chronosa role design but keep the CDQN causal controls firmly in place. 6
- Interpolating Neural Networks (INNs) offer very large efficiency gains in some scientific problems (PDE/simulation surrogates); they inspire our “specialized solver modules” not the Chronosa general reasoning pipeline. 7

---

## Where to use an LLM inside CDQN (approved roles)
1. **User intent parsing → CDU templates.** Convert human requests into structured CDU drafts that Chronosa roles will verify. (LLM output must be tagged `draft_cdu` and require Chronosa approval before publication.)
2. **Human-readable reports.** Summaries of node activity, reputation explanations, or license texts.
3. **Policy drafting (human-assisted).** Create human-legible policy drafts to be reviewed and approved by node administrators/Chronosa.
4. **Augmented search / retrieval.** Assist human operators when exploring large CDU histories (for convenience, not for validation).

**Hard rule:** Any LLM-produced content that would modify network state (mint tokens, publish theorems/Btheorems, change an axiom) must be converted into CDUs and pass full Chronosa role verification + Policy veto checks.

---

## Where Chronosa role-assembly (and tiny recursive models) must be used
1. **Proposer / Verifier / Backward-Validator roles** — discover theorems and Btheorems using CDU evidence, perform backward trace to axioms, and generate verifiable `btheorem` CDUs.
2. **Reputation & cdqnStar mint gating.** Local Chronosa + network consensus must validate before any token minting.
3. **Security-critical decisions** (refusal, propagation blocking, legal/regulatory enforcement). Chronosa must emit explicit `refusal` CDUs with evidence pointers.
4. **Low-energy mass inference.** Use bounded recursive micro-reasoners to cheaply reason over local CDU graphs and prime-axes.

TRM-style tiny recursive models are a good blueprint for these roles because they achieve high generalization with small compute budgets on low-data reasoning tasks — but **only** use the specific patterns that integrate with CDU evidence and the axiomatic framework; do not adopt any component that bypasses traceability. 8

---

## Practical gating & reliability design (recommended)
- **Signed evidence rule:** Any theorem/Btheorem or reputation event must include pointers to CDUs that constitute the proof path (signed HLC IDs); this is non-negotiable.
- **Two-step publication:** (1) role-level acceptance (Proposer+Verifier), (2) policy-level approval and cross-node consensus for network publication/mint. LLM drafts enter only step (0) as `draft_cdu`.
- **Budgeted compute + ACT halting:** roles must declare compute budgets; adopt an ACT-like halting criterion for recursive roles to avoid runaway compute. (Use bounded recursion + small latent `z` passing.)
- **Cross-node sanity checks:** before mint, require at least K independent corroborating CDUs or M nodes’ verifiers. This prevents single-node gaming.

---

## Energy, auditability & sustainability (brief)
- Large LLMs are expensive and brittle for trust-critical tasks. Where possible, rely on add/multiply primitives, tiny recursion, or domain INN modules for heavy numeric tasks. The INN literature highlights substantial energy/compute savings for structured numerical problems; we use that as inspiration for scientific modules. 9
- Keep LLM use strictly for human-facing or non-authoritative drafts; this controls energy costs and reduces risk.

---

## Change log (this update)
- Integrated TRM/HRM insights (tiny-recursive reasoning) as **inspiration for Chronosa roles**, not as replacements for causality-first logic. 11  
- Added INN (interpolating neural network) inspiration for specialized scientific modules.
- Hardened the LLM usage policy: explicit draft-only role + Chronosa verification gating.  
- Added compute/energy & auditability guidance.

---

## References & inspiration (local repository papers)
- Tiny Recursive / HRM / TRM — used as inspiration for bounded recursive role design.
- Interpolating Neural Networks (INN) — inspiration for specialized solver modules (PDE/simulation surrogates).
