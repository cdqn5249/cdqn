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
