---
layout: default
title: Step 6.3a — The Bootstrap Artifact/Validator Share (qir-enc-5)
description: The qir-enc-5 artifact share — mandatory V.encoding gating of tags 32-37, the byte-identical B1-B6 fixture block, SCC-additive admission, governed Q_HASH, and byte-exact core/container/call lineage invariance
nav_exclude: true
---

# Step 6.3a — The Bootstrap Artifact/Validator Share (`qir-enc-5`)

Wave record of the Step-6.3a Share-2 artifact/validator increment under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: `v0.3.0-draft.37` (specification share), implementations still declare `v0.3.0-draft.36`. **Share 2 is DISCHARGED**: both implementations accept `qir-enc-5`/`qir-prof-ext-bootstrap-1`, enforce the mandatory `V.encoding` gate on tags `32`–`37`, and reproduce every `B1`–`B6` expectation of `docs/qnir_bootstrap.md` §9.6 in a byte-identical UTF-8/LF fixture block (1,209 bytes, SHA-256[:16] `b6e8626488aa936a`). The golden corpus and all core/container/call encodings are untouched; QnIR remains `[PENDING]`.

## The selection

`docs/qnir_bootstrap.md` §9.7 kept the artifact/validator share forward of the specification share. Under golden-first discipline, §9.1–§9.6 were treated as read-only ground truth: every implementation assertion was checked against the declared values, and no fixture number was retuned to fit code. The gate closes the same class of finding as Step 6.2 — version text must dispatch admission, not merely be stored: `qir-enc-5` admits tags `32`–`37`, while `qir-enc-2`/`3`/`4` reject them.

## What was emitted

**Rust (`src/qnir/`).** New `src/qnir/src/bootstrap.rs` implements `version_bounds`/`validate_version` for `qir-enc-5`, the Tarjan/SCC-admission checks, the exact `B3` digest/word pins, and the `B1`–`B6` engine plus three `#[test]` functions. `insn.rs` gains tags `32`–`37` and their §2.6 decoder arms; `bhat.rs` gains `hash_charge`, `charge_ins`, and `bhat_scc`; `interp.rs` gains `Bitwise`/`Hash` execution and the §9.4 admission selection; `main.rs` accepts `--fixture B`. The committed global-product `bhat` and call composition remain active for `qir-enc-4`.

**Python (`src/qnir-py/qnir_ref.py`).** Independently written mirror: opcode map, §2.6 operands, `hash_charge`, Tarjan condensation and exit-aware per-SCC DP, `Machine` dispatch for `BAND`/`BOR`/`BXOR`/`BSHL`/`BSHR`/`Q_HASH`, and `--fixture B`. Standard library only; stdout pinned to UTF-8/LF. `IMPL_VERSION` and `Cargo.toml` both remain `0.3.0-draft.36` (no version bump in this wave).

**The fixture block of record** (byte-identical across Rust and Python, 1,209 bytes, 10 lines):

```text
fixture B1 : B_hat=56 U Work(E)=56 U n=9 outcome=SUCCESS tokens=0/0 reserve=56 charged=56 refund=0 p1=32B OK
fixture B2a: B_hat=32 U Work(E)=28 U n=3 outcome=EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=32 charged=28 refund=4 p1=32B OK
fixture B2b: B_hat=32 U Work(E)=28 U n=3 outcome=EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=32 charged=28 refund=4 p1=32B OK
fixture B2c: B_hat=32 U Work(E)=28 U n=3 outcome=EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=32 charged=28 refund=4 p1=32B OK
fixture B3 : B_hat=318 U Work(E)=318 U n=3 outcome=SUCCESS tokens=0/0 reserve=318 charged=318 refund=0 digest=1fab775418d8ed7a71155cd2d0afb0998718f32b4bd55271eb24daa9bbe20eff OK
fixture B3b: B_hat=318 U Work(E)=24 U n=2 outcome=EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=318 charged=24 refund=294 p1=32B OK
fixture B4 : B_hat=164 U Work(E)=116 U n=24 outcome=SUCCESS tokens=0/0 reserve=164 charged=116 refund=48 global=REFUSED scc_states=24 OK
fixture B5 : B_hat=48 U Work(E)=32 U n=3 outcome=SUCCESS tokens=0/0 reserve=48 charged=32 refund=16 parity=OK OK
fixture B6 : B6a=PASS B6b=PASS B6c=PASS B_hat=null Work(E)=0 n=0
fixture B: PASS — 6/6 declared fixtures reproduce under qir-enc-5 / qir-prof-ext-bootstrap-1 (spec OK)
```

Block SHA-256: `b6e8626488aa936af67ccca89454c13c1f33c887a504f823503617ce717aca80`; first 16 hex: `b6e8626488aa936a`. `B3` binds the canonical 24-byte `INT_POS 5` encoding `0000000000000001 0000000000000001 0000000000000005` to the four 64-bit words `(0x1fab775418d8ed7a, 0x71155cd2d0afb099, 0x8718f32b4bd55271, 0xeb24daa9bbe20eff)`; `B4` is refused by the global product (`12·2^12 = 49152 > 4096`) and admitted by the SCC sum (`12·2 = 24 <= 4096`) with `B_hat = 164 U`; `B5` verifies `B_hat_scc == B_hat_global = 48 U`. Every run asserts `reserved = B_hat = charged + refunded`.

## The evidence (runs of record, this commit state)

`cargo test --offline` in `src/qnir/`: **23 passed; 0 failed** (the 17 pre-existing tests plus six new bootstrap tests). `cargo test --offline` in `src/qnlangc/`: **21 passed; 0 failed** (3 exposer + 3 lower + 15 worked, untouched). Both artifacts exit 0 on all six conformance/fixture modes.

```
mode=normal rust=0 py=0 runroot_eq=True
mode=F27 rust=0 py=0 runroot_eq=True sha16=641b2cb6c08c805b eq=True
mode=F6 rust=0 py=0 runroot_eq=True sha16=5a562124ee953687 eq=True
mode=C rust=0 py=0 runroot_eq=True sha16=19fd9ac6a678ce7b eq=True
mode=K rust=0 py=0 runroot_eq=True sha16=88c14b102dcfb4a1 eq=True
```

The F27/F6/C/K fixture-block digests are unchanged from the previous artifact share, and the `scope = run` trace root is byte-identical between artifacts.

**Standalone `--eval-cbor` (`qir-enc-5`), full stdout byte-identical between Rust and Python:**

```
b1   outcome=SUCCESS B_hat=56 Work=56 n=9 RustEqPy=True P1=d42559fa5ba90f864f05c4ae3f8934e81253114236c3cf1d2bb9d15f50c305d5 trace=9af4bad0b6829a0a611306f6728a53a7cc55755b6af4d991422db8da6621eb9a
b3   outcome=SUCCESS B_hat=318 Work=318 n=3 RustEqPy=True P1=60edc4b7a438c5af760d90506d4069ea59580f221574e5b7c5ace9859540fc8e trace=48a97c0ce44d4781acaedffd90243c9798eea379979bf149ee26f476cf3ecab1
b3b  outcome=EXPRESSION_TOO_COMPLEX B_hat=318 Work=24 n=2 RustEqPy=True P1=f359bce29a9715c74e29f38511f9840c92bbf2d4e88af1b31029b19d6f82a9d5 trace=2e0fd37d8cecbf1c01ea0c18d9c18d943dce4ac76cd37a9b842691bc2877d1d1
```

**`V.encoding` gating** (each line `rust_reject`/`py_reject`/`same` all true): `enc2 tag32`, `enc3 tag32`, and `enc4 tag=32..37`. Under `qir-enc-4` the six new tags remain unknown opcodes; the pre-existing key-9 and tag-30/31 behavior is unchanged. Standalone scratch envelopes and transcripts live only under `%TEMP%\opencode\` and are not staged.

## Disposition

`docs/qnir_bootstrap.md` §9.7 Share 2 is **DISCHARGED** by this record. Step 6.3b (`OP_SATURATE` / `QO-28`), Fork–join, and Road proofs remain FUTURE with their own evidence. `Q1`, `Q3`, `Q6`, and promotion are untouched; no opcode is canonized and no golden byte is re-versioned. This log introduces no term include for an unregistered slug. No source file outside `src/qnir/` and `src/qnir-py/` was edited; `src/qnlangc/`, `docs/qnlang.md`, and `docs/qnir_goldens.md` are byte-unchanged.
