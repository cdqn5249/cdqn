---
layout: default
title: Stage 1 T33+T34+T37 — modules + topological callees + nested b_hat + parity
description: QnLang Stage-1 scaffold (types.qnl/cbor.qnl/driver_test.qnl), modules.rs arena merge + intra-lib rename + Q(use) sha-256, lower.rs topological key-9 callees + reverse-topological nested b_hat, cycle rejection; Rust==Python
nav_exclude: true
---

# Stage 1 — T33+T34+T37 (QnLang Alpha scaffold, `qnlangc` only)

## Source Tasks
T33, T34, T37, T37c. F19: `src/qnlangc/`, `src/qnlangc-stage1/`, `docs/logs/qnlang/` only. `src/qnir/`, `src/qnir-py/`, `docs/qnir*.md`, `docs/qnir_goldens.md`, `docs/_data/glossary/`, `docs/logs/qnir/` untouched. QnLang `[PENDING]`. No `argc`/`retc` empty-`arg_bnds` rejection added (does not break `nested_calls`). `E_NAMESPACE_COLLISION` for duplicate `Q(variant)`/`Q(type)`/`Q(anchor)`/`Q(metric)` preserved.

## Added / modified (sha-256)
- `src/qnlangc/src/modules.rs` `ec041969485ea76ca5e2c471a031e26af925ca126c95223c6a772406733ec5d2`: `Q(use)` sha-256 pin; `Program.arena` merge + `shift_top`/`shift_node`; intra-library `rename_node` `alias::name`; cycle-before-digest; `E_IMPURE_LIBRARY_IMPORT`.
- `src/qnlangc/src/lower.rs` `aef159c543fe35f3ed05e0e7c929c6d0bd40a2d9e27744e96a9851c80a262bd7`: deterministic topological `callee_ids` over `Q(pattern)`/`Q(workflow)`/`Q(falsify)` (`p.tops` tie-break); `OP_CALL` `name=="call"` `Ident` `alias::name`; cycle/self-call `E_LOWERING_AMBIGUOUS`; `own_cost`+`direct` `folded_interior` reverse-topological (no `gateway` double-count); `b_hat`; key-9 order matches `callee_id`.
- `src/qnlangc/tests/modules.rs` `019ab8384dfb0aa391f0ccc1b10c01a57289661ba99374196874f54840b73a8d`: `intra_library_cross_call_resolves_and_lowers` (`A` before `B`,`B`→`A`,`x::B`=0,`x::A`=1); `call_cycle_and_self_call_are_rejected_statically`.
- `src/qnlangc-stage1/types.qnl` `50c1960df8077a81507280d851b4059bce0bf935ebcd0df18d8a78b853d56c53`; `cbor.qnl` `d6624d2c5238ff12477a2cc476bc01f1b6c3bc98b60a182932029cc62652c75a`; `driver_test.qnl` `786e8fd537e6028d064e805b4bbc40fff1a0162521f66b7e30b698b003777ab7`.

## Tests
`cargo test --offline` `src/qnlangc` 33/33; `src/qnir` 25/25 read-only. Checker G1-G14+E_NAMESPACE_COLLISION -> Exposer -> R11'; `READBACK_CERTIFIED`.

## Parity
- `driver_test.qnl` (qir-enc-5): qnlangc `b_hat=Some(124)`, `callees=6`, `sha256=c0e415fc…`; `SUCCESS`,`B_hat=124`,`Work=124`,`n=23`,`124/124/0`, `P1=c89737dd451836dbb2c807e95456acfca80b1a263cac7675f9de6bd0c75adf6c`, `trace-root=dcac1e89692915ab9b941c1127ae418ab5de7920f2152d4f7b957f46bea4592a`, `Rust==Python`.
- `nested_ab` (`x::A` before `x::B`, `x::B->x::A`, qir-enc-5): qnlangc `b_hat=Some(60)`,`callees=2`,`sha256=6695da…`; `SUCCESS`,`B_hat=60`,`Work=60`,`n=8`,`60/60/0`, `P1=b85eb1ac6d236d09663d10ea9077a33fab8a556bf0ce5335d0dcb5b8087caf57`, `trace-root=8e7bf2b0944c94bb9e872abbd90c371716427650055b932668d0e4f90a648f67`, `Rust==Python`.
- `nested_ok` (`x::B=0->x::A=1`, instruction after `x::A` returns): `SUCCESS`,`60/60/0`,`n=8`,`Rust==Python`. K6a (`callee_id>i`, no self/cycle, unknown `callee_id`, `argc>64`; `src/qnir`-side T36) + `E_NAMESPACE_COLLISION`. 