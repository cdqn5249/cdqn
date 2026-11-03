---
layout: default
title: QnIR Artifact Logs
description: Run transcripts of the QnIR conformance artifacts
nav_exclude: true
---

# QnIR Artifact Logs

This folder holds the run transcripts and reproducibility evidence of the QnIR draft's conformance artifacts (`src/qnir/` — the artifact share of conformance §4). Each log records what ran, under which pinned toolchain, against which committed corpus bytes, with what verdicts. The artifacts are checked against the committed golden corpus (`docs/qnir_goldens.md`) and never generate it; a same-binary rerun recorded here is reproducibility evidence only — never independent reproduction, never conformance (§4.2). Paths inside the transcripts are the run environment's paths; the corpus bytes they name are the committed bytes of the cited file at the cited draft version.

## Logs

- [R6 increment 2 — the assembler and validator (v0.3.0-draft.11)](r6-increment-2-assembler-validator.html) — first run against the committed draft.10 corpus found the M4 byte defect; 34/34 after the declared repair; same-binary rerun byte-identical.
- [R6 increment 3 — the reference interpreter (v0.3.0-draft.12)](r6-increment-3-reference-interpreter.html) — 8/8 committed executions reproduced at the five agreement points; one decoder defect found in the increment-2 artifact and repaired in-wave; same-binary rerun byte-identical.
- [R6 increment 4 — the second implementation (v0.3.0-draft.13)](r6-increment-4-second-implementation.html) — the standard-library Python implementation reproduces all 34 committed vectors at the five agreement points; same-source rerun byte-identical.
- [R6 expansion increment 1 — the QO-22/F12 combination completion (v0.3.0-draft.14)](r6-expansion-1-qo22-f12-combinations.html) — the expanded corpus (64 vectors): both artifacts pass 38/38 executions after the registered adjudication repaired the shared fatal-exit terminalization defect in both; same-binary and same-source reruns byte-identical.
- [R6 expansion increment 2 — the CLOAD-hit and multi-block fallback completion (v0.3.0-draft.15)](r6-expansion-2-cload-hit-multiblock.html) — the expanded corpus (68 vectors): both artifacts pass 42/42 executions on the first run, no adjudication registered; same-binary and same-source reruns byte-identical.
- [R6 expansion increment 3 — the TUPLE/FIELD/zoom-CMP/EMIT-witness completion (v0.3.0-draft.16)](r6-expansion-3-tuple-field-zoom-cmp-emit.html) — the expanded corpus (75 vectors): both artifacts pass 49/49 executions; two pre-publication defects registered and repaired in-wave (the instrument's dependency-list default, exposed by the assembler's re-encoding check, and the reference artifact's zoom-literal source parse); same-binary and same-source reruns byte-identical.
