# CDQN — Chained and Distributed Quang Numbers

**Author:** Christophe Duy Quang Nguyen  
**Portal:** [https://cdqn5249.github.io/cdqn/](https://cdqn5249.github.io/cdqn/)  
**License:** [Scaling Source License (SSL) 1.0](LICENSE.md)  

---

## The Inquiry

> *Can a discrete number system abstract any computable phenomenon in physical reality if every operation is strictly bound by finite physical laws?*

CDQN reconstructs computation from the ground up on that question: exact discrete rationals instead of IEEE 754 drift, no unmeasured state across abstraction boundaries, every state transition metered to thermodynamic limits, and all artifacts sealed inside a governed envelope. The full architectural thesis lives on the documentation portal — this repository hosts its sources.

*Status: formal theoretical and specification phase. No pre-compiled binaries or runtimes are hosted here.*

---

## Documentation Map

Each specification exists as a source `.md` (this repository) and a rendered `.html` (the portal). Details, definitions, and reading order live there.

| Document | Specification (.md) | Portal (.html) |
|---|---|---|
| Documentation Portal & Roadmap | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) | [Portal](https://cdqn5249.github.io/cdqn/) |
| SIMEMP Constraints | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) | [simemp.html](https://cdqn5249.github.io/cdqn/simemp.html) |
| Abstraction Layers | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) | [abstractionLayers.html](https://cdqn5249.github.io/cdqn/abstractionLayers.html) |
| Qn Primitive Envelope | [docs/qnPrimitive.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qnPrimitive.md) | [qnPrimitive.html](https://cdqn5249.github.io/cdqn/qnPrimitive.html) |
| Q(0) Origin & Q(1) Calibration | [docs/q0_q1.md](https://github.com/cdqn5249/cdqn/blob/main/docs/q0_q1.md) | [q0_q1.html](https://cdqn5249.github.io/cdqn/q0_q1.html) |
| Single-Digit Integers Q(2)…Q(9) | [docs/q2_q9.md](https://github.com/cdqn5249/cdqn/blob/main/docs/q2_q9.md) | [q2_q9.html](https://cdqn5249.github.io/cdqn/q2_q9.html) |
| Qm — Quang Mathematics | [docs/qm.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qm.md) | [qm.html](https://cdqn5249.github.io/cdqn/qm.html) |
| Qm Geometry | [docs/qm_geometry.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qm_geometry.md) | [qm_geometry.html](https://cdqn5249.github.io/cdqn/qm_geometry.html) |
| Qm Calculus | [docs/qm_calculus.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qm_calculus.md) | [qm_calculus.html](https://cdqn5249.github.io/cdqn/qm_calculus.html) |
| Qexpr — CAE-DAG | [docs/qexpr.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qexpr.md) | [qexpr.html](https://cdqn5249.github.io/cdqn/qexpr.html) |
| Qs — Quang Semantics | [docs/qs.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qs.md) | [qs.html](https://cdqn5249.github.io/cdqn/qs.html) |
| Qlog — Quang Logics | [docs/qlog.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qlog.md) | [qlog.html](https://cdqn5249.github.io/cdqn/qlog.html) |
| Qphy — Quang Physics | [docs/qphy.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qphy.md) | [qphy.html](https://cdqn5249.github.io/cdqn/qphy.html) |
| Qchem — Quang Chemistry | [docs/qchem.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qchem.md) | [qchem.html](https://cdqn5249.github.io/cdqn/qchem.html) |
| Qbio — Quang Biology | [docs/qbio.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qbio.md) | [qbio.html](https://cdqn5249.github.io/cdqn/qbio.html) |
| Chronosa Vision | [docs/chronosa.md](https://github.com/cdqn5249/cdqn/blob/main/docs/chronosa.md) | [chronosa.html](https://cdqn5249.github.io/cdqn/chronosa.html) |
| Reflection — H10 & Finite Computation | [docs/reflections/h10_finite_computation.md](https://github.com/cdqn5249/cdqn/blob/main/docs/reflections/h10_finite_computation.md) | [h10_finite_computation.html](https://cdqn5249.github.io/cdqn/reflections/h10_finite_computation.html) |
| Unified Glossary | [docs/glossary.md](https://github.com/cdqn5249/cdqn/blob/main/docs/glossary.md) | [glossary.html](https://cdqn5249.github.io/cdqn/glossary.html) |

---

## Repository Structure

```
cdqn/
├── docs/       # Architectural specifications, documentation portal, and essays
├── src/        # QnIR conformance artifacts (Rust reference interpreter, Python second implementation)
├── LICENSE.md  # Scaling Source License (SSL) 1.0 terms
└── README.md   # Project inquiry and entry points
```

---

## Implementation Progress (Milestone 2 substrate)

The following is **implementation evidence only**; it does not promote either specification to canon. `QnIR` and `QnLang` both remain `[PENDING]`.

- **QnIR v0.3 Step 6.1** — Containers, `qir-enc-3` / `qir-prof-ext-containers-1`, fixture `C`: implemented and differentially verified in Rust and Python.
- **QnIR v0.3 Step 6.2** — Calls, `qir-enc-4` / `qir-prof-ext-calls-1`, fixture `K`: implemented and differentially verified in Rust and Python.
- **QnIR v0.3 Step 6.3a** — Bootstrap primitives (`BAND`, `BOR`, `BXOR`, `BSHL`, `BSHR`, `Q_HASH`), Tarjan SCC-additive `bhat`, and `qir-enc-5` / `qir-prof-ext-bootstrap-1`, fixture `B`: implemented and differentially verified across Rust (`23/23` tests) and the Python reference implementation.
- **QnIR v0.3 Step 6.3b** — `OP_SATURATE` / fixed-width saturating arithmetic are retained provisionally as a Layer-3 `Q(rsi)` compile-time transform under `docs/boc.md` §2.3 / Axiom 13; `qir-enc-5` remains the safe fallback, with `P_gap = UNRESOLVED` until multi-host evidence exists.
- **QnLang Rung-0 Milestone 2** — Stage 0 completed: `src/qnlangc/` Rust seed with the explicit three-part `Checker` → `Exposer` → `Compiler` pipeline, 14 semantic gates, `R12'` dual-channel diagnostics, `--target qir-enc-4|qir-enc-5`, and `qir-enc-5` lowering over `.qnl` source files. `27/27` compiler tests pass; all nine emitted programs are byte-identical across both QnIR artifacts under `--eval-cbor`.

### Attribution

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL).  
> Parent Repository: [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
