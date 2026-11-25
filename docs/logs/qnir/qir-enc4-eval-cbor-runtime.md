---
layout: default
title: qir-enc-3 / qir-enc-4 Runtime — standalone --eval-cbor (draft.36 Wave)
description: General step rules for container and call opcodes, recursive compositional bhat, 0x30 frame stack, r_sub sub-budget, byte-identical differential execution, and zero regression across the five committed modes
nav_exclude: true
---

# qir-enc-3 / qir-enc-4 Runtime — standalone `--eval-cbor`

Wave record for the general runtime execution of `qir-enc-3` (tags `17`–`29`) and `qir-enc-4` (tags `30`–`31`, envelope key `9`) in `src/qnir/` and `src/qnir-py/qnir_ref.py`. This wave is F19-separated: it changes **only** QnIR artifacts. `docs/qnir_goldens.md`, `docs/qnlang.md` and `src/qnlangc/` are untouched. QnIR remains `[PENDING]`; no new opcode, envelope key, or token type was invented.

## Invocation

```text
qnir --eval-cbor <file.cbor> [--budget <U>]          # default 1000 U, qir-prof-ext-calls-1
python3 src/qnir-py/qnir_ref.py --eval-cbor <file.cbor> [--budget <U>]
```

Both tools decode one standalone Intent envelope with strict `V.encoding` gating (`qir-enc-2` rejects `17`–`31` and key `9`; `qir-enc-3` rejects `30`–`31` and key `9`; `qir-enc-4` admits all) and emit a canonical summary: `V`, `B_hat`, `outcome`, `Work(E)`, `n`, `reserved/charged/refunded`, final `P1` (32 B) and the vector-scope `qir-trace-root-1`.

## What was implemented

- **Containers (`17`–`29`).** `TAG_CONTAINER = 5` (`ARRAY_RRB16 = 1`, `TABLE_MBTREE16 = 2`) in the value store and P2 canonical encoding `[5, subtype, cap_bnd, elem_bnd, [[k,v],…]]`; COW versions minted under the §2.4 rule; `ARR_NEW/LEN/GET/SET/BATCH/ROOT` and `TBL_NEW/LEN/GET/AT/SET/BATCH/ROOT` with §7.5 `depth_16(cap_bnd)` charges and §7.3 typed `EXPRESSION_TOO_COMPLEX` rejections. `TBL_GET` miss births Tier-O `STATUS_LOOKUP_MISS` and checks the F25 `(src_blk, src_idx, 0)` trip-ceiling site.
- **Calls (`30`–`31`, key `9`).** Four-element callee records `[UInt id, Bytes arg_bnds, Bytes ret_bnds, Bytes blocks]`; strict topological acyclicity and arity/bounds validation; `C_max = 8`, `F_max = 64`. Runtime keeps the §8.3 `0x30` chain `stack_root_{d+1} = SHA-256(0x30 || stack_root_d || P2)` with P2 component (2) `[active_args, parent_stack_root]` at `depth > 0`, the §8.4 transition-local `r_sub` with `OP_RET` residue, and the global token pool across frames. `bhat` is recursive over key `9`, folding `B_callee_body + charge(OP_RET)` at every call site (`charge(OP_CALL) = 4 + 2·argc`, `charge(OP_RET) = 4 + 2·retc`) and enforcing `dp_bound <= S_max = 4096`.
- **Ledger invariant.** The escrow opened at the first completed transition equals the compositional `B_hat`; terminal settlement gives `reserved == B_hat == charged + refunded` with `B_hat >= Work(E)`.

## Tests (verbatim)

`cargo test --offline` in `src/qnir/`:

```
running 17 tests
test eval_cbor_tests::nested_calls_chain_0x30_stack_root ... ok
test eval_cbor_tests::inner_fatal_settles_sub_budget_and_outer_ledger ... ok
test eval_cbor_tests::miss_token_persists_across_op_ret ... ok
test eval_cbor_tests::depth_above_c_max_is_refused ... ok
test eval_cbor_tests::container_envelope_executes_standalone ... ok
test eval_cbor_tests::qir_enc3_rejects_call_and_key9 ... ok
test eval_cbor_tests::qir_enc4_key9_and_extension_tags_decode ... ok
test eval_cbor_tests::core_envelope_executes_standalone ... ok
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Zero-regression parity (verbatim)

| Mode | Rust exit | Python exit | `scope = run` root identical | fixture SHA-256[:16] | byte-identical |
|---|---:|---:|---|---|---|
| normal | 0 | 0 | yes | — | — |
| F27 | 0 | 0 | yes | `641b2cb6c08c805b` | yes |
| F6 | 0 | 0 | yes | `5a562124ee953687` | yes |
| C | 0 | 0 | yes | `19fd9ac6a678ce7b` | yes |
| K | 0 | 0 | yes | `88c14b102dcfb4a1` | yes |

`corpus identity`, `V`, `E = 5a438424c7988dc2…`, the `scope = run` trace root `369ada2bde4d65f0…`, and the collision bucket are unchanged. Seven externally emitted envelopes (the six §6.4 programs plus `call_worked`) execute with byte-identical summaries in both implementations; `B_hat >= Work(E)` and the ledger identity hold on all seven.
