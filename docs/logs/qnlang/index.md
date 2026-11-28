---
layout: default
title: QnLang Artifact Logs
description: Run transcripts and receipts for the QnLang `qnlangc` Stage-0/Stage-1 work
nav_exclude: true
---

# QnLang Artifact Logs

`src/qnlangc/` (+ `src/qnlangc-stage1/`); F19-separated; QnLang `[PENDING]`. 

- [Stage 0 `qnlangc` — Rung-0 parser, gates, R11' (`stage0-qnlangc-rung0.md`)](stage0-qnlangc-rung0.html) — `v0.1.0-draft.3`; `qir-enc-4`; `R12'`; `call_worked` 78/78 `Rust==Python`. 
- [Stage 0 `qir-enc-5` bootstrap (`stage0-qir-enc5-bootstrap.md`)](stage0-qir-enc5-bootstrap.html) — `v0.1.0-draft.4`; `qir-enc-5/qir-prof-ext-bootstrap-1`; tags 32-37; `Q(band/bor/bxor/bshl/bshr)`,`Q(hash)`,`Q(loop)`,`Q(variant)/Q(match)`,`Q(array)`. 
- [Stage 0 `Exposer` `qir-verbalizer-1` + `R12'` (`stage0-exposer-r12.md`)](stage0-exposer-r12.html) — Checker→Exposer→Compiler; `readback_sha256`; `--certify-readback`; `READBACK_CERTIFIED`. 
- [Stage 1 `T33+T34+T37` (`stage1-t33-t34-t37-modules-calls-parity.md`)](stage1-t33-t34-t37-modules-calls-parity.html) — `src/qnlangc-stage1/`; `modules.rs` arena merge + intra-lib `alias::`; topological `callees` + nested `b_hat`; `driver_test` 124/124/0 n=23; `nested_ab` 60/60/0 n=8; `Rust==Python`. 
- [Stage 1 readiness audit `T39+T40` (`stage0-t39-t40-readiness-audit.md`)](stage0-t39-t40-readiness-audit.html) — read-only exact-status; `Q(reuse)`/`Q(bypass)`/`Q(6:sym)`/`Q(2:split)`/`.0`/`.1`/`Q(loop)`/`Q(fold)`/`ARR_GET`/`CLOAD`/`S_max`/`F_max`. 