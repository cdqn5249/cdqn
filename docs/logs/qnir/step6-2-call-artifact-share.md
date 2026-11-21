---
layout: default
title: Step 6.2 — The Call Artifact/Validator Share (v0.3.0-draft.36)
description: The qir-enc-4 artifact share — mandatory V.encoding gating, the byte-identical K1–K6 fixture block, and core/container lineage invariance across all ten runs
nav_exclude: true
---

# Step 6.2 — The Call Artifact/Validator Share (v0.3.0-draft.36)

Wave record of the Step-6.2 Share-2 artifact/validator increment under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.36. **Share 2 is DISCHARGED**: both implementations accept `qir-enc-4`/`qir-prof-ext-calls-1`, enforce the mandatory `V.encoding` gate, and reproduce every `K1`–`K6` expectation of `docs/qnir_calls.md` §8.6 in a byte-identical UTF-8/LF fixture block (SHA-256[:16] `e83189cb00a56497`). The golden corpus and all core/container encodings are untouched; QnIR remains `[PENDING]`.

## The selection

`docs/qnir_calls.md` §8.7 kept the artifact/validator share forward of the specification share. Under golden-first discipline, §8.1–§8.6 were treated as read-only ground truth: every implementation assertion is checked against those declared values, and no fixture number was retuned to fit code. The mandatory gate closes the Gate-0 finding that `V.encoding` was stored as text but never dispatched: version now determines admissible envelope keys, opcode tags, and value tags in both artifacts.

## What was emitted

**Rust (`src/qnir/`).** `Cargo.toml` version `0.3.0-draft.36`; new `src/qnir/src/calls.rs` implements `version_bounds`, `validate_version`, `validate_envelope_cbor` and `validate_block_tags` (the mandatory dispatch gate), the K1–K6 engine, and two `#[test]` functions (`version_gating_is_exact`, `k6_static_rejections`). `insn.rs` gains `OP_CALL = 30`/`OP_RET = 31` and their §2.6 decoder arms; `main.rs` wires the gate into the envelope validator and accepts `--fixture K`.

**Python (`src/qnir-py/qnir_ref.py`).** `IMPL_VERSION = "0.3.0-draft.36"`; an independently written mirror of the gate and the K-fixture engine; `--fixture K` renders the identical block. Standard library only, redirected stdout pinned to UTF-8/LF.

**The fixture block of record** (byte-identical across Rust and Python, 760 bytes, 7 lines):

```text
fixture K1 : B_hat=50 U Work(E)=50 U n=6 outcome=SUCCESS tokens=0/0 reserve=50 charged=50 refund=0 p1=32B OK
fixture K2 : B_hat=76 U Work(E)=76 U n=11 outcome=SUCCESS tokens=0/0 reserve=76 charged=76 refund=0 p1=32B OK
fixture K3 : B_hat=356 U Work(E)=260 U n=51 outcome=SUCCESS tokens=0/0 reserve=356 charged=260 refund=96 p1=32B OK
fixture K4 : B_hat=44 U Work(E)=30 U n=3 outcome=DOMAIN_ERROR|EXPRESSION_TOO_COMPLEX tokens=0/0 reserve=44 charged=30 refund=14 p1=32B OK
fixture K5 : B_hat=54 U Work(E)=54 U n=7 outcome=NO_SOLUTION tokens=1/1 reserve=54 charged=54 refund=0 p1=32B OK
fixture K6 : K6a=PASS K6b=PASS K6c=PASS K6d=PASS B_hat=null Work(E)=0 n=0
fixture K: PASS — 6/6 declared fixtures reproduce under qir-enc-4 / qir-prof-ext-calls-1 (spec OK)
```

Block SHA-256: `e83189cb00a564974381a828340d9ddc7b527bfd8a8beb65e6a7ac3e8f95ebcd`; first 16 hex: `e83189cb00a56497`. The frame-chain check uses `stack_root_0 = SHA-256(0x02)` and `stack_root_{d+1} = SHA-256(0x30 || stack_root_d || suspended_frame_bytes)`; every run asserts a 32-byte P1 root and exact §3.3 conservation (`reserved = charged + refunded`).

## The evidence (runs of record, this commit state)

`cargo test --offline`: **8 passed; 0 failed** (the four pre-existing tests, the two container tests, and the two new call tests). `cargo build --release --offline` succeeds with `rustc 1.99.1`; Python is CPython 3.12.10. All ten runs exit 0: normal 84/84, `--fixture F27`, `--fixture F6`, `--fixture C`, and `--fixture K` for each artifact. Core lineage is byte-exact outside the implementation-identity line (which necessarily changes with the version bump): corpus `246bc01c…bcc1d92`, `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`, `E = 5a438424…15da`, `n = 336`, `scope = run` root `369ada2b…5866`, F27 block `6f6bd8f5ea344b31`, F6 block `5041b3741ade3b9a`, C block `2c96b152f38a7d44` — all unchanged from `4106b88`.

| Artifact | Mode | Bytes (LF) | Lines | SHA-256 (LF transcript) |
|---|---|---:|---:|---|
| Rust | normal | 36,472 | 260 | `c43d686bab00ea25510de540fa6e9368215b74215b0451f5c3ff278bc73892e0` |
| Rust | `--fixture F27` | 37,622 | 270 | `75cbf688a1c50c4e29d28ce8fadec9500ee8026c415739afbdc51b37247c678b` |
| Rust | `--fixture F6` | 37,090 | 269 | `dfba58e4c2ecc59eeab732391c76ee4c4bcaea36a2399d5d9438ff4529414f87` |
| Rust | `--fixture C` | 38,395 | 269 | `0af78375701b83a9c5188b2b0572127e08bca35e11698f3c162b5ef06640430e` |
| Rust | `--fixture K` | 37,232 | 267 | `0edca08ededa34ce3b1b09c0ef9a533f09e75485d5ffe7dcf05fda4057dee657` |
| Python | normal | 18,550 | 123 | `f4df820f67c8eed6018bdf45086e8500be6e859c78c74bfe9a8968e77e24220a` |
| Python | `--fixture F27` | 19,700 | 133 | `3ac851ceaecc6988d9ed6fa0c8281a4babd7993e5b1e8dc4b6bd208a2d97a8d6` |
| Python | `--fixture F6` | 19,168 | 132 | `53e414bc87aa977657ad45744526737af40ad1995ca9cd14297b7e19fbeffe5e` |
| Python | `--fixture C` | 20,473 | 132 | `1034a6a5af615cc8e6a7aca5c2cf9866357cd026b4d975a72c12a3d6a0c77ebf` |
| Python | `--fixture K` | 19,310 | 130 | `9234909bf82c2fd9073f63b958aee9e103cc01bd3bfc495af143a793e6b59bc6` |

The corpus was read-only; no repository file outside this log, the index bullet, and the two completion notes was edited.

## Disposition

Step 6.2 is **closed at v0.3.0-draft.36** for its specification and artifact shares: `docs/qnir_calls.md` §8.7 records Share 2 as DISCHARGED, and the byte-identical fixture block above is the evidence of record. No structure is sealed, no extension opcode is canonized into the frozen core, and no promotion is claimed: QnIR remains `[PENDING]`; `Q1`, `Q3`, and `Q6` remain `QUEUED`. The remaining Step-6 extensions — Saturation, Fork–join, and Road proofs — and any sealing through a Q(rsi) loop are forward. No new term include is introduced. Declared non-edits: `docs/qnir_goldens.md`, `docs/qnir_containers.md`, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/qnir_commitments.md`, `docs/index.md`, and `docs/_data/*`. No push is made until the maintainer approves.
