---
layout: default
title: Stage 1 readiness audit (T39+T40)
description: read-only exact-status audit of QnLang Stage-1 `.qnl` surface vs implemented lexer/parser/ast/gates/lower; Q(reuse)/Q(bypass)/Q(6:sym)/Q(2:split)/Postfix/Q(loop)/Q(fold)/ARR_GET/CLOAD/S_max/F_max
nav_exclude: true
---

# Stage 1 readiness audit — Source Tasks `T39`, `T40`

Verified read-only at `HEAD` `f0a773831fb0b9d7b92bec57ebf0fd9db3e4786e`, tree clean, `cargo test --offline` `src/qnlangc` **33/33**, `src/qnir` **25/25**. 

## `src/qnlangc-stage1/`
- `types.qnl` 916 B/54 lines sha256 `50c1960df8077a81507280d851b4059bce0bf935ebcd0df18d8a78b853d56c53`; `Q(type) T`,`Q(anchor) z`,`Q(metric) m`,`Q(variant) Type/Expr/Stmt/Token/Error`. 
- `cbor.qnl` 1055 B/45 lines sha256 `d6624d2c5238ff12477a2cc476bc01f1b6c3bc98b60a182932029cc62652c75a`; `Q(pattern) cbor_uint_head/cbor_uint_hi_lo/cbor_byte_header/cbor_array_header/cbor_map_header/cbor_bytes_storage`; `Q(band)`,`Q(bshl)`,`Q(bshr)`,`Q(arr_new)`,`Q(array)`,`Q(drop)`. 
- `driver_test.qnl` 552 B/17 lines sha256 `786e8fd537e6028d064e805b4bbc40fff1a0162521f66b7e30b698b003777ab7`; `Q(use)` `types`,`cbor` `@ sha-256`,`Q(2:cons)`,`cbor::*`,`emit receipt`. `qnlangc b_hat=Some(124)`,`SUCCESS n=23 124/124/0 Rust==Python`. 

## Exact status (`src/qnlangc/src/`)
- **`Q(reuse)`.** `parser.rs:10`,`gates.rs:540`,`lower.rs:333`,`lower.rs:444`: domain-erased identity `0 U`, same SSA ref. NOT hash-cons `τ_U/τ_t`, no `QO-21`. 
- **`Q(bypass)`.** `lower.rs:131`,`lower.rs:137-141`,`lower.rs:248-250` (`unwrap_id`): `CAE-AST` `Q(bypass)` normalize + strip; NO `OP_BYPASS`. 
- **`Q(6:sym)`.** NOT implemented; no `sym`/`6:sym`; `Q(6)` qprim only; no `R11'`; docs `Q(6:sym)`. 
- **`Q(2:split)`.** `parser.rs:13`,`parser.rs:797`,`NodeKind::Split`,`gates`; NO `lower.rs::value` arm; `E_LOWERING_AMBIGUOUS`. 
- **`.0`/`.1`.** `parser.rs` `Dot`/`NodeKind::Postfix`,`lower.rs:474-489`: only `.exact?`; `.0`/`.1` parsed NOT lowered. 
- **`Q(loop)`.** `parser.rs`,`NodeKind::Loop`,`lower.rs:557-589`,`gates.rs:531` `trip(K)<=S_max`; straight-line unroll, single block-0, NO `BR`/`JMP`. 
- **`Q(fold)`.** `NodeKind::Fold`,`lower.rs:491`,`gates`; unrolled `ARR_LEN/CMP/ARR_GET/ADD`,`trip(1)`,`cap(N)`,single block; NO BR/JMP. 
- **Arrays.** `Q(arr_new)`→17,`Q(store)`→`ARR_SET`/`TBL_SET`,`Q(drop)`,`Q(fold)`,`Q(load)`→`TBL_GET`,`Q(tbl_new)`. NO standalone `Q(arr_get)/Q(ARR_GET)/Q(arr_len)/Q(arr_set)/Q(arr_batch)/Q(arr_root)`; element read only inside `Q(fold)`; append-only `Q(store)`. 
- **`Q(match)`.** exhaustive `Q(variant)`,`TUPLE[tag,payload]`,`FIELD`,`CMP`,`MUL`,`ADD`. **`Q(2:cons)`.** `lower.rs:388`. `Q(band/bor/bxor/bshl/bshr)`,`Q(hash)`. `Q(workflow)`→key-9. `Q(use)` recursive sha256 + `E_*`. 
- **Strings/bytes.** `Tok::Str`; no `.qnl` string→`Q(array)<u8,cap(N)>`, no `CLOAD` `.qnl`, no `Q(store)` seed, no char/byte literal. `Q(array)<u8,cap(N)>` + `Q(store)`. 
- **`S_max`.** `gates.rs:526-532`,`lower.rs:905`,`b_hat`,`E_SMAX_OR_BUDGET`,`S_max=4096`; single-block unroll; `Q(bshl/bshr) s<w`,`Q(hash 1..65536`. NO `scc_dp_states` in `src/qnlangc`. 
- **`F_max`.** NO `src/qnlangc` check; QnIR `src/qnir` `F_max=64`,`C_max=8`,`C_max*F_max=512`,`depth`,`P10`. 
- **`E_NAMESPACE_COLLISION`.** preserved; duplicate `Q(variant)`/`Q(type)`/`Q(anchor)`/`Q(metric)`. 

## UNVERIFIED
- `modules.rs` diamond `Q(use)` deduplication. 
- `src/qnir` `F_max=64` counting across `BR`/`JMP` loop iterations. 