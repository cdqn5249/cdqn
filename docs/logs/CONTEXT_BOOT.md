# CONTEXT_BOOT

Truth-source-first. Verify `git rev-parse HEAD`, `git status -s`, run `cargo test --offline`, recompute `sha-256`, re-read source & logs before claiming. Receipt log per task. F19.

## Current
`HEAD` `f0a773831fb0b9d7b92bec57ebf0fd9db3e4786e`, clean. QnLang `[PENDING]`, QnIR `[PENDING]`. Option B QnLang Alpha. Stage-1 `src/qnlangc-stage1/`.

## F19
- `src/qnir/`+`src/qnir-py/`: QnIR only; NOT `src/qnlangc*`,`docs/qnir*`,`docs/qnir_goldens.md`,`docs/_data/glossary`. 
- `src/qnlangc/`+`src/qnlangc-stage1/`: QnLang; NOT `src/qnir*`,`docs/qnir*`,`docs/qnir_goldens.md`,`docs/_data/glossary`. 
- `docs/logs/qnir/`: QnIR receipts. `docs/logs/qnlang/`: QnLang receipts. `docs/logs/`. no `docs/index.md`. 

## Baseline (verified)
| area | count | truth |
|---|---|---|
| `src/qnir` | 25/25 | `cargo test --offline`; `prog=f.ret_prog`,`folded_vec`,`K6a`,`84/84`,`C/K/B/F27/F6`; `T36` `09c96df` |
| `src/qnir-py` | PASS | `qnir_ref.py`; `nested`,`prog=f["ret_prog"]`,`folded`,`K6a`; `Rust==Python` |
| `src/qnlangc` | 33/33 | `modules.rs` arena merge+`alias::`; `lower.rs` topological `callees`+nested `b_hat`; `Q(use)`; `Q(2:cons)`,`Q(match)`,`Q(fold)`,`Q(loop)`,`Q(arr_new)`,`Q(store)`,`Q(load)`,`Q(band/bor/bxor/bshl/bshr)`,`Q(hash)`,`Q(workflow)` |
| `src/qnlangc-stage1` | `driver_test` | `types.qnl`916/54`50c1960d`,`cbor.qnl`1055/45`d6624d2c`,`driver_test.qnl`552/17`786e8fd5`; `b_hat=124 n=23 124/124/0 Rust==Python` |

Digests `K e83189cb00a56497`,`C 2c96b152f38a7d44`,`B b6e8626488aa936a`,`F27`,`F6`. 

## Logs
- `docs/logs/qnir/index.md`,`step6-2-call-artifact-share`,`t36-nested-opcall-retprog-bhat-k6a`. 
- `docs/logs/qnlang/index.md`,`stage0-qnlangc-rung0`,`stage0-qir-enc5-bootstrap`,`stage0-exposer-r12`,`stage1-t33-t34-t37-modules-calls-parity`,`stage0-t39-t40-readiness-audit`. 

## Stage-1 `D(Task)`
- syntax lowering: `docs/qnlang.md` §3/§4 + `src/qnlangc/src/{lexer,parser,ast,gates,lower}.rs`. 
- modules `Q(use)`: `docs/qnir_calls.md` §8.2/8.4 + `src/qnlangc/src/modules.rs`. 
- readiness: `docs/logs/qnlang/stage0-t39-t40-readiness-audit.md`. 
- QnIR nested `OP_CALL`: `docs/qnir_calls.md` §8.2/8.4 + `src/qnir/src/interp.rs` + `docs/logs/qnir/t36-…`. 

## Open gaps (Stage-1)
`Q(6:sym)` no; `Q(2:split)` unlowered; `.0/.1` unlowered; `Q(reuse)` `0U` no `QO-21`; `Q(bypass)` no `OP_BYPASS`; `Q(2:split)`; no `Q(arr_get)/ARR_GET`; `Q(load)` table only; append-only `Q(store)`; no string→`Q(array)<u8>`,`CLOAD`,byte/char; single block-0 `Q(loop)/Q(fold)`,`trip(K)`,no `BR/JMP`; `trip(K)<=S_max`,no `scc_dp_states`,`F_max` QnIR-side; `Q(variant)` not `alias::`,`E_NAMESPACE_COLLISION`. UNVERIFIED: `modules.rs` diamond `Q(use)`; `F_max=64` across `BR/JMP`. 