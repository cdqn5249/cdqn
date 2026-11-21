---
layout: default
title: Step 6.1 — The Container Artifact/Validator Share (v0.3.0-draft.34)
description: The qir-enc-3 / qir-prof-ext-containers-1 artifact share — thirteen container opcodes in both implementations, the byte-identical C1–C6 fixture block, and core qir-enc-2 invariance
nav_exclude: true
---

# Step 6.1 — The Container Artifact/Validator Share (v0.3.0-draft.34)

Wave record of the Step-6.1 Share-2 artifact/validator increment under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.34. **Share 2 is DISCHARGED**: both implementations accept `qir-enc-3` / `qir-prof-ext-containers-1`, implement the thirteen extension opcodes and value tag `5`, and reproduce every `C1`–`C6` expectation of `docs/qnir_containers.md` §7.6 in a byte-identical UTF-8/LF fixture block (SHA-256[:16] `2c96b152f38a7d44`). The golden corpus and all core encodings are untouched; QnIR remains `[PENDING]`.

## The selection

`docs/qnir.md` §1.3's Containers row and `docs/qnir_containers.md` §7.7 kept the artifact/validator share forward of the specification share. Under the golden-first discipline, §7.3–§7.6 were treated as read-only ground truth: every implementation number in this log is an assertion against those declared values, never a retuning of them. The selected increment is the smallest one that discharges the register row without touching the committed core: an additive decode/execute path gated on encoding version (`qir-enc-3` only), a new `--fixture C` mode reading its expectations from `docs/qnir_containers.md`, and version metadata bumps. Opcode tags `17`–`29` and value tag `5` remain statically illegal under `qir-enc-2`, so `84/84` core vectors cannot be reached by extension behavior.

## What was emitted

**Rust (`src/qnir/`).** `Cargo.toml` version `0.3.0-draft.34`; new `src/qnir/src/containers.rs` implements `TAG_CONTAINER = 5` subtypes `ARRAY_RRB16 = 1` / `TABLE_MBTREE16 = 2`, copy-on-write persistent pair vectors, the §7.3.1 canonical `[5, subtype, cap_bnd, elem_bnd, pairs]` encoding, `0x00` leaf and `0x20` pairwise SHA-256 commitments with odd promotion and `SHA-256(0x02)` for the empty root, the static `depth_16(cap_bnd)` charge function, the trip-ceiling `B_hat` DP with `TBL_GET`'s fallback edge joined exactly as `CLOAD`'s, and the eight declared fixture runs. `main.rs` accepts `--fixture C` and prints the deterministic block. Two new `#[test]` functions check `qir-enc-2` rejection of tags `17`–`29` and container bounds/batch edge cases.

**Python (`src/qnir-py/qnir_ref.py`).** `IMPL_VERSION = "0.3.0-draft.34"`; an independently written mirror of the same declared engine; `--fixture C` renders the identical block. Standard library only, redirected stdout pinned to UTF-8/LF.

**The fixture block of record** (byte-identical across Rust and Python, 1,923 bytes, 9 lines):

```text
fixture C1 : B_hat=94 U Work(E)=94 U n=16 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C2 : B_hat=94 U Work(E)=94 U n=16 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C3 : B_hat=44 U Work(E)=44 U n=5 outcome=SUCCESS len=0 miss=1/1 root=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C4a : B_hat=72 U Work(E)=72 U n=10 outcome=SUCCESS len=2 miss=0/0 root=e4c1626b02c04db1eda6f7aca69273a84c875caa8106d6a3e3a96fda1be044d2 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C4b : B_hat=72 U Work(E)=68 U n=9 outcome=EXPRESSION_TOO_COMPLEX len=0 miss=0/0 root=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C5 : B_hat=58 U Work(E)=54 U n=6 outcome=EXPRESSION_TOO_COMPLEX len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C6a : B_hat=54 U Work(E)=54 U n=7 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C6b : B_hat=54 U Work(E)=54 U n=7 outcome=SUCCESS len=1 miss=0/0 root=3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4 empty=dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986 p1=32B OK
fixture C: PASS — 8/8 declared runs reproduce under qir-enc-3 / qir-prof-ext-containers-1 (spec OK)
```

Block SHA-256: `2c96b152f38a7d4418ccd3856232064c3c96bfe2a9b119e7f96179db184562be`; first 16 hex: `2c96b152f38a7d44`. The `C1`/`C2` folds traverse with `ARR_LEN`/`ARR_GET` and `TBL_LEN`/`TBL_AT` under `CMP`/`BR` and one trip-ceiling back-edge; `C3` births and consumes exactly one Tier-O miss token; `C4b`/`C5` terminate with the existing `EXPRESSION_TOO_COMPLEX` token; `C6a`/`C6b` share the empty root `dbc1b4c9…d986` and the one-member root `3874f4bb…c8d4`; every run asserts a 32-byte P1 state root.

## The evidence (runs of record, this commit state)

`cargo test --offline`: **6 passed; 0 failed** (the four pre-existing tests plus `qir_enc2_rejects_extension_opcodes` and `container_bounds_and_batch_edges`). `cargo build --release --offline` succeeds with `rustc 1.99.1` (`x86_64-pc-windows-msvc`); Python is CPython 3.12.10, standard library only. All eight runs exit 0: normal 84/84, `--fixture F27`, `--fixture F6`, and `--fixture C` for each artifact.

| Artifact | Mode | Bytes (LF) | Lines | SHA-256 (LF transcript) |
|---|---|---:|---:|---|
| Rust | normal | 36,472 | 260 | `91df181caaa6e8014094e9a4679019638e5f386bce72c3e671d12c7c24b62223` |
| Rust | `--fixture F27` | 37,622 | 270 | `bf231b9a434ae7c0e89116039ad73a457630cd3b76b5c7e0843176adbc33d49f` |
| Rust | `--fixture F6` | 37,090 | 269 | `4831b00f175b3af1ff0b33a80ab9082bed59ca8f65cb1937935b075b132a96f3` |
| Rust | `--fixture C` | 38,395 | 269 | `320b928ffa6eadeb58d344de837993db62d0d6b01e6ea7f7d12468286092cde4` |
| Python | normal | 18,550 | 123 | `f7852c97b56eeaa275e39e2f5fb455c31d3212a7ca30e0a6a194177dc78bcb5f` |
| Python | `--fixture F27` | 19,700 | 133 | `deb03d97d063e994f090e253b9588b8e29a0d6ffcb408c95d0f8fc1de731c7d4` |
| Python | `--fixture F6` | 19,168 | 132 | `e469ed7e42fc9a93db7399a4a6af1e3f42337b314a3069351c1c800bd9ce8f0d` |
| Python | `--fixture C` | 20,473 | 132 | `01b6a26340e0606e2ff472cffb5114f96d79c906aff3e1bc5a81d9ff4756b058` |

Core invariance is byte-exact outside the expected implementation-identity line: corpus `246bc01c3337afb8ba930e3738650e05f1aa412b4d8d495c9b27f05c6bcc1d92`, `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`, `E = 5a438424c7988dc2a5f0e771c495b8fa786da8a506e006aa0b90f8c1ca9915da`, `n = 336`, `scope = run` root `369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866`, F27 block `6f6bd8f5ea344b31`, F6 block `5041b3741ade3b9a` — all unchanged from `6cc1ef1`. The normal-mode transcripts differ from the draft.32 transcripts only in the implementation-identity line (Rust `9d7608e1…e484d0`, Python `27e6dc2b…2f4d4a9`), as the new source bytes require under §6.2. The corpus was read-only; `git status --porcelain` reflects only the three budgeted implementation/spec files plus this log and the index bullet.

## Disposition

Step 6.1 is **closed at v0.3.0-draft.34** for its specification and artifact shares: `docs/qnir_containers.md` §7.7 records Share 2 as DISCHARGED, and the byte-identical fixture block above is the evidence of record. No structure is sealed, no extension opcode is canonized into the frozen core, and no promotion is claimed: QnIR remains `[PENDING]`; `Q1`, `Q3`, and `Q6` remain `QUEUED`. The remaining Step-6 families — Calls, Saturation, Fork–join, and Road proofs — remain forward with their own studies, as does any sealing of the container extension through a Q(rsi) loop. No new term include is introduced. Declared non-edits: `docs/qnir_goldens.md`, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/qnir_commitments.md`, `docs/index.md`, `docs/_data/*`, and all historical logs. No push is made until the maintainer approves.
