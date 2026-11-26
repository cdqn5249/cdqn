---
layout: default
title: Stage 0 — qir-enc-5 Bootstrap Lowering (v0.1.0-draft.4)
description: The explicit --target qir-enc-4|qir-enc-5 bootstrap compiler wave — bitwise tags 32-36, governed Q_HASH tag 37, bounded loops and variant/match lowering, with 27/27 tests and a nine-program Rust/Python differential
nav_exclude: true
---

# Stage 0 — `qir-enc-5` Bootstrap Lowering (`v0.1.0-draft.4`)

Wave record for `src/qnlangc/` under `docs/qnlang.md` `v0.1.0-draft.4`. F19-separated: this wave touches only `docs/qnlang.md`, `src/qnlangc/`, and this QnLang log. `src/qnir/`, `src/qnir-py/`, `docs/qnir_goldens.md`, all `docs/qnir*.md`, and the §6.4 programs are byte-unchanged. No `Q(0)_H` material is generated, read, or signed, and no licensing gate is introduced (`docs/qnlang_custody.md` §4).

## The explicit target

```
qnlangc --emit-qir <file> [-o <out.cbor>] [--certify-readback <sha256>] [--target qir-enc-4|qir-enc-5]
qnlangc --expose <file> [--target qir-enc-4|qir-enc-5]
```

The target is **explicit and never guessed**. The default remains the frozen `qir-enc-4`; `--target qir-enc-5` selects `qir-enc-5` / `qir-prof-ext-bootstrap-1`. Under `--target qir-enc-4`, a bootstrap operation raises `E_TARGET_MISMATCH` and writes zero CBOR bytes. The `Checker → Exposer → Compiler` interlock of the previous wave is unchanged: certification still precedes emission, and the read-back must match exactly.

## What was added (Axiom 13, additive)

- **§4 EBNF**: five fixed-width built-ins `Q(band)`, `Q(bor)`, `Q(bxor)`, `Q(bshl)`, `Q(bshr)`; governed `Q(hash)(x, Q(b_bnd))`; and `@[trip(K)]` as an alternate bounded-loop spelling alongside `Q(loop) trip(K)`.
- **§5.2 signatures**: bitwise over `u8`/`u16`/`u32`/`u64` with `0 <= x < 2^w` and `s < w`; `Q(hash)` returns `Q(tuple)<u64, u64, u64, u64>` with static `1 <= b_bnd <= 65536` and F14 charge `160 + 130 * ceil(b_bnd / 64)` U.
- **§6.2 gates**: `G2` checks the hash byte bound, `G14` checks `s >= w` and applies the SCC-additive `trip(K)` ceiling under `S_max = 4096`.
- **§7.1 `R11'` table**: tags `32`/`33`/`34` (`BAND`/`BOR`/`BXOR`), `35`/`36` (`BSHL`/`BSHR`), `37` (`Q_HASH`), bounded loops with BR/JMP back-edge and SCC-additive admission (Stage 0 emits the equivalent straight-line `trip(K)` unrolling in block 0), and variant/match as `TUPLE [tag, payload]` + `FIELD`/`CMP`/`MUL`/`ADD` exhaustive dispatch.
- **§7.3**: records the Step-6.3a/b evidence, `qir-enc-5` as the provisional fallback floor, and Step 6.3b (`OP_SATURATE`/`SAT_*`) as open to multi-host re-evaluation without mutating `qir-enc-5`.

New fixtures: `src/qnlangc/tests/fixtures/boot_ops.ql` and `boot_loop.ql`. New integration surface: `src/qnlangc/tests/bootstrap.rs` (6 tests covering positive lowering, target rejection, gate rejections, `qir-enc-4` byte stability, and the 12-loop SCC-vs-product admission case).

## Tests (verbatim)

```
src/qnlangc/  cargo test --offline → 6 + 3 + 3 + 15 = 27 passed; 0 failed
src/qnir/     cargo test --offline → test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
docs/qnlang.md: used=50 unregistered=NONE
```

All 21 pre-existing `qnlangc` tests remain green; the six new bootstrap tests pass. Zero unregistered `term.html` slugs in `docs/qnlang.md`.

## Nine-program `--target qir-enc-5 --certify-readback` + `--eval-cbor` differential

Every program was exposed with `--expose --target qir-enc-5`, emitted with the exact `readback_sha256` under `--certify-readback`, and executed by both QnIR artifacts. All runs printed `EPISTEMIC READBACK_CERTIFIED` and ended `RustEqPy=True` on **full stdout** (envelope header, `B_hat`, outcome, `Work(E)`, `n`, ledger, P1, trace root). `B_hat == qnlangc_B_hat >= Work(E)` and `reserved == B_hat == charged + refunded` throughout.

| Fixture | CBOR bytes | SHA-256[:16] | qnlangc B_hat | eval B_hat | Work(E) | n | Rust == Py |
|---|---:|---|---:|---:|---:|---:|---|
| minimal | 396 | `454b8ae129b56b66` | 40 | 40 | 40 | 5 | yes |
| qm | 756 | `b41de6a78cbe4dc7` | 60 | 60 | 60 | 10 | yes |
| qphy | 283 | `5856c3b242944a0c` | 36 | 36 | 36 | 4 | yes |
| qbio | 283 | `0deccfcfe03d0c25` | 36 | 36 | 36 | 4 | yes |
| qchem | 1780 | `c00a8d680114605d` | 144 | 144 | 144 | 26 | yes |
| qs_qlog | 1323 | `3b741c6465bc4549` | 32 | 32 | 32 | 3 | yes |
| call_worked | 861 | `269824a0865b230c` | 78 | 78 | 78 | 11 | yes |
| boot_ops | 860 | `985d812eaf5bf0eb` | 358 | 358 | 358 | 13 | yes |
| boot_loop | 3300 | `e2843831c499732a` | 222 | 222 | 222 | 49 | yes |

The seven pre-existing programs keep their `qir-enc-4` bytes exactly under the explicit default target (spot check: `minimal` → 391 B, SHA-256[:16] `f0c2074817e58e28`); the `qir-enc-5` bytes above differ only in the declared version/profile strings. `boot_ops` exercises all five bitwise tags plus `Q_HASH` with `b_bnd = 64` (290 U op charge); `boot_loop` exercises `Q(variant)`, `Q(match)`, `Q(array)`, and `Q(loop) @[trip(3)]`.

## Scratch and disposition

All emitted `.cbor` files and transcripts live only under `%TEMP%\opencode\`; no scratch file is staged and no `target/` artifact is committed. The bootstrap primitives are emitted only under the explicit `qir-enc-5` target; no new token type, terminal outcome, envelope key, or `qir-enc-6` tag is introduced. QnLang remains `[PENDING]`; no extension is canonized and no golden byte is re-versioned.
