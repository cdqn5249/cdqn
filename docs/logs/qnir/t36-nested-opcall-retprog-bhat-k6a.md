---
layout: default
title: T36 — nested OP_CALL ret_prog, decoded reverse-topological B_hat, static K6a (QnIR both artifacts)
description: Source Tasks T35,T36,T36b,T36c,T36d,T36e,T36f; Commit of Record 09c96df; src/qnir + src/qnir-py; 25/25; 84/84; F27/F6/C/K/B unchanged; nested OP_CALL Rust==Python
nav_exclude: true
---

# T36 — nested `OP_CALL` `ret_prog`, decoded reverse-topological `B_hat`, static `K6a`

**Source Tasks:** `T35, T36, T36b, T36c, T36d, T36e, T36f`. 
**Commit of Record:** `09c96df95d0bffe3d395ab1297c19a441fd37553` (`T36 — fix nested OP_CALL ret_prog, decoded B_hat, and static K6a`; 3 files: `src/qnir/src/interp.rs`, `src/qnir/src/main.rs`, `src/qnir-py/qnir_ref.py`, +170/-38). F19: only `src/qnir/` + `src/qnir-py/` edited. QnIR `[PENDING]`. `qnir_goldens.md` (659880 bytes), `docs/qnir*.md`, `src/qnlangc*`, `docs/logs/qnlang`, `docs/_data/glossary` untouched. 

## Root causes fixed (both artifacts; `src/qnir/src/interp.rs`, `src/qnir-py/qnir_ref.py`)

1. **Frame `ret_prog` save/restore.** `Frame.ret_prog` saved at `OP_CALL` (caller `prog`); `OP_RET` restores `prog = f.ret_prog` / `prog = f["ret_prog"]` (not hardcoded `0`); `Transfer::Call prog=callee+1`; `pc=(blk,idx)` restored `f.ret_pc.0, f.ret_pc.1+1`. Matches Step-6.2 hash-chained frame stack + `K2`. (Python `_current_prog`.) 
2. **Instruction-decoded reverse-topological `B_hat`.** Rust `folded_vec`; Python `folded`. Per key-9 callee `own_cost` (`Σ charge(tag)`, no `gateway`); `nested`/`OP_CALL` targets `callee_id > i`; reverse order high→low; `folded[cid]=own+Σ folded[nested]`; caller `b += folded[cid]` (no `gateway` double-count). Replaces raw `ws[wi]==30` scan + caller-only scan. 
3. **Static `K6a` & call-graph validation** (`docs/qnir_calls.md` §8.2). `OP_CALL` inside callee `i` ⇒ `callee_id > i`; no self-call; no cycle; unknown `callee_id`; `argc>64`. `K6a=PASS`. No empty-`arg_bnds`/`ret_bnds` rejection added (`nested_calls` uses empty `arg_bnds`/`ret_bnds`). 
4. Conservation `reserved = charged + refunded`; `fatal` refunds `outstanding_escrow`. 

## File digests (sha-256, current tree)
- `src/qnir/src/interp.rs` `9c7f716ec5a8b69fc5a9830f6883e97aa35f0e93ddfe800393acd33ccf4438e2`
- `src/qnir/src/main.rs` `b249cf1af0283ce303bd1ed84e2b142df6e789eab01b2411225e4d97eec3dd86`
- `src/qnir-py/qnir_ref.py` `955d2859e843793168a42bb95eb794e547c1bdec1593c4132a9b050545903b8b`

## Tests
`cargo test --offline` `src/qnir`: **25/25** (23 baseline + `nested_op_call_multilevel_success_and_frame_restore`, `k6a_rejects_reverse_order_and_self_call`; `nested_calls_chain_0x30_stack_root` `n=11 SUCCESS`; `K1-K6`). 

## Both artifacts, all 6 modes (Rust & Python; `0.3.0-draft.36`; `qir-prof-ext-*`; `docs/qnir_goldens.md`)
Corpus `246bc01c…`/659880 B (`committed at 0.3.0-draft.32` Python); `n=336`, `qir-trace-root-1 scope=run root=369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866`. Implementation identity Rust `(implementation,0.3.0-draft.36,b05b2d368862a9521b4e260b6eba078ea2fd02b88d040facccea8b1c543a2f12)`, Python `(implementation,0.3.0-draft.36,955d2859e843793168a42bb95eb794e547c1bdec1593c4132a9b050545903b8b)`. 

| Mode | Rust | Python | Block digest |
|---|---|---|---|
| normal `84/84` | `result: PASS` | `result: PASS` | unchanged |
| `--fixture F27` | PASS | PASS | `F27` `6f6bd8f5ea344b31` (§4.4; `B_hat=40`,`W=36`,`B_injected=32`) |
| `--fixture F6` | PASS | PASS | `F6` `5041b3741ade3b9a` (§6.7) |
| `--fixture C` | PASS 8/8 `qir-enc-3` | PASS 8/8 | `C` **`2c96b152f38a7d44`** |
| `--fixture K` | PASS 6/6 `qir-enc-4` | PASS 6/6 | `K` **`e83189cb00a56497`** (`K1-K6`,`K6a`) |
| `--fixture B` | PASS 6/6 `qir-enc-5` | PASS 6/6 | `B` **`b6e8626488aa936a`** |

`F27` `--fixture F27` (test-only fault injection `B_injected`, §4.4) and `F6` (`qir-artifact-cmp-1`, §6.7) PASS; `K` `K1=50 n=6`,`K2=76 n=11`,`K3=356/260 n=51`,`K4=44/30 n=3`,`K5=54 n=7`,`K6a-K6d=PASS`. 

## Nested `OP_CALL` cross-impl parity (`nested_ok`/`nested_ab`, `x::B=0 -> x::A=1`, `qir-enc-5`, 764 B, `callees=2`)
Both **Rust `qnir --eval-cbor`** and **Python `qnir_ref.py --eval-cbor`**: `outcome=SUCCESS`, **`B_hat=60`**, **`Work(E)=60 U`**, **`n=8`**, **`reserved/charged/refunded=60/60/0 U`**, `final P1 = b85eb1ac6d236d09663d10ea9077a33fab8a556bf0ce5335d0dcb5b8087caf57`, `qir-trace-root-1 (scope=vector,n=8,root=8e7bf2b0944c94bb9e872abbd90c371716427650055b932668d0e4f90a648f67)`. **Rust == Python.** (x::B calls x::A, instruction after x::A returns; K6a `1>0`.) 

## Unchanged
`qnir_goldens.md` `84/84`; pre-existing blocks `F27`,`F6`,`C=2c96b152f38a7d44`,`K=e83189cb00a56497`,`B=b6e8626488aa936a`; `docs/qnir*.md`. 