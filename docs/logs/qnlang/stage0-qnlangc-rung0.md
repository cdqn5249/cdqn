---
layout: default
title: Stage 0 qnlangc — Rung-0 Parser, Gates and R11' Lowering (v0.1.0-draft.3)
description: Deterministic 3-step lowering of seven QnLang programs to qir-enc-4 CBOR with 18/18 green tests and byte-identical Rust/Python differential execution
nav_exclude: true
---

# Stage 0 `qnlangc` — Rung-0 Parser, Gates and `R11'` Lowering

Wave record for `src/qnlangc/` at `docs/qnlang.md` **v0.1.0-draft.3**. F19-separated: this wave touches only `src/qnlangc/` and this QnLang log; `src/qnir*`, `docs/qnir_goldens.md`, `docs/qnlang.md` §6.4 programs and the five existing fixtures were not modified. Standard library only; no external crate.

## Invocations

```text
qnlangc --check <file> [--name-map] [--gate-list]
qnlangc --emit-qir <file> [-o <out.cbor>]
cargo test --offline                         # in src/qnlangc/
```

`--check` parses the reconciled §4 EBNF, builds the hash-consed `CAE-AST` DAG, runs `E_NAMESPACE_COLLISION` plus `G1`–`G14`, §3.1 hygiene, §5.2 signatures and §2.3 affine linearity, and emits `R12'` dual-channel diagnostics. `--emit-qir` executes the closed §7.1 `R11'` table: (1) normalize (`Q(bypass)` identities, commutative sort, `z_qnir = -z_min`), (2) domain erasure into bounds/units/receipts, (3) deterministic RFC 8949 CBOR with keys `0..=9` and compositional `B_hat`. All temporary `.cbor` files live in `%TEMP%\opencode\`.

## Tests (verbatim)

`cargo test --offline` in `src/qnlangc/`:

```
running 3 tests   (tests/lower.rs)
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
running 15 tests  (tests/worked.rs)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests qnlangc ... test result: ok. 0 passed; 0 failed
```
Total **18/18** green. The suite covers the five §6.4 programs through the strict parser and full gates, eight negative diagnostics (`E_NAMESPACE_COLLISION`, `E_NO_NAKED_NUM`, `E_OP_HYGIENE`, `E_MODAL_OR_DOMAIN_LEAK`, `E_AFFINE_DOUBLE_CONSUME`, `E_AFFINE_MISSING_DROP`, `E_BOUND_OVERFLOW` ×2), deterministic CBOR emission, and loud `E_SMAX_OR_BUDGET` on declared-budget exceedance.

## Seven lowered programs

`--emit-qir` sizes/hashes and the Rust==Python `--eval-cbor` differential (`B_hat` and `Work(E)` in U; all ledger identities `reserved == B_hat == charged + refunded` hold):

| Fixture | CBOR bytes | SHA-256[:16] | B_hat | Work(E) | Rust == Py |
|---|---:|---|---:|---:|---|
| minimal | 391 | `f0c2074817e58e28` | 40 | 40 | yes |
| qm | 751 | `40ca53e63cc1adb4` | 60 | 60 | yes |
| qphy | 278 | `b58477753896c921` | 36 | 36 | yes |
| qbio | 278 | `e1993819a6626c45` | 36 | 36 | yes |
| qchem | 1775 | `92214a13eaed8fe2` | 144 | 144 | yes |
| qs_qlog | 1318 | `f00ee50ae12ea5d5` | 32 | 32 | yes |
| call_worked | 856 | `69aa70a483a28beb` | 78 | 78 | yes |

`call_worked` defines a `Q(pattern)` that uses `Q(tbl_new)`/`Q(store)`/`Q(load)` and invokes it at top level; the emitted entry block therefore contains a real `OP_CALL` and key‑9 callee blocks, executed differentially by both QnIR artifacts. `qs_qlog`'s declared `budget(64 U)`/`budget(80 U)` are satisfied under compositional `B_hat`; no §6.4 program or budget annotation was changed.
