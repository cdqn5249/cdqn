---
layout: default
title: Step 6.2 — The Call Trade Study (QO-21 / Obj 6, native Windows)
description: The read-only native-Windows measurement of call-versus-inline storage, compositional B_hat scaling, P1 state-root runtime, and sub-budget/failure conservation, with the BOC verdict retaining bounded acyclic calls under qir-enc-4
nav_exclude: true
---

# Step 6.2 — The Call Trade Study (QO-21 / Obj 6, native Windows)

Measurement record of the Step-6.2 call trade study under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. The harness is a standalone crate at `C:\Users\dd_an\AppData\Local\Temp\opencode\call-trade-study\`, built with `rustc 1.99.1` (`x86_64-pc-windows-msvc`, `--release --offline`, LTO, codegen-units=1), thread pinned to core 2, 2 warm-up + 9 measured trials, cycles at 1.80 GHz. It models real §2.6 64-bit word encodings, deterministic CBOR envelopes, `P1 = SHA-256(P2)` 32-byte state roots per transition (including serialized frame-stack bytes in P2 for Candidate C), and `qir-trace-root-1`. **No repository file was created, edited, or deleted by the run**; `git status --porcelain` was empty at the start and end. The record changes no QnIR semantics and seals no extension.

## The selection

`QO-21` requires calls to be moved to a profile-gated extension; `Obj 6` is fatal because frame state, return conventions, verification reuse, and transitive budgets were unspecified in v0.2. `docs/qnir.md` §1.3 prices the call trade study at Step 6 with `F_max`, `C_max`, receipt order, failure propagation, and sub-budget conservation specified before retention. The study measures four dimensions against real encodings rather than asymptotic claims, then applies the recorded lexicographic BOC key to decide whether bounded acyclic `OP_CALL`/`OP_RET` survive.

## What was emitted

The artifact of record is `call_trade_results.json`, **60,607 bytes, SHA-256 `ef1857484c21b21d040a4b3cc41999418bbc2c00f6adacf53be63d0efd4645ab`**, plus a 26,204-byte CSV companion and the full printed transcript `run_stdout.txt`. It contains 30 storage rows, 5 surrogate rows, 270 execution rows, and 4 deterministic invariants.

**1. Storage / CBOR break-even.** Exact §2.6 word lengths measured: CONST 72 B, ADD 64 B, MUL 64 B, CMP 72 B, JMP 32 B; study-side candidate OP_CALL 48 B, OP_RET 16 B. Across `P ∈ {2,4,8,16,32,64}` and `m ∈ {1,2,4,8,16}`, the empirical break-even is **`P* = 2` at every `m ≥ 2`** — above the theoretical `P > m/(m−1)` curve (2.000 at m=2, 1.333 at m=4, 1.143 at m=8, 1.067 at m=16), because the real call form also carries a 16 B callee header and a 16 B RET. At `m = 1` call form is always larger (e.g. 4,354 B → 4,437 B at P=64). At `P=64, m=16` inline envelopes are 68,796 B versus 5,157 B call-form.

**2. `B_hat` scaling and `S_max = 4096`.** With a 4-block callee containing `k = 2` back-edges at `trip_ceiling = 1`, the inline structural product is `4m · 2^(2m)`. Inline is admitted for `m = 1` (16 states), `m = 2` (128), `m = 4` (4,096); **the exact inline refusal first occurs at `m = 5`** (20,480 > 4,096), with `m = 6/8` refused in ~4 ns after the gate. Compositional `B_hat` computes the callee once (16 states) plus `O(1)` per call site, giving 17/18/20/22/24 states for `m = 1/2/4/6/8`, bounds 96/100/108/116/124 U, and **remains admitted throughout**. Admitted inline DP latency is ~4.6–5.9 µs; compositional evaluation is ~0.39 ns per repetition.

**3. Execution / P1 runtime.** Across `P ∈ {4,16,64}`, `m ∈ {2,8}`, `F ∈ {4,16,64}`, `C ∈ {1,4,8}`, and modes Inline / CallB (no frame bytes in P2) / CallC (frame bytes in P2) / MemoB / MemoC, the median CallC overhead by P is **P=4: +336%, P=16: +59%, P=64: −14%** → **`P_exec* = 64`** (first P below the 10% threshold). The dominant variable is depth: Candidate C's P2 grows from 111 B at `C=1` to **5,512 B at `C=8, F=64`**, so the extra SHA-256 preimage work dominates at high depth even for large P. Memoized verification reuse wins broadly: at `m=8, C=1`, MemoC is 26–79% faster than Inline; at `P=64, m=8, C=8` it is 20% faster while dynamic calls are still +171%.

**4. Failure / conservation.** All **270/270** execution rows satisfy `reserved = charged + refunded`. Four deterministic checks pass: inner `EXPRESSION_TOO_COMPLEX` and `DOMAIN_ERROR` each seal fatally at their own transition with `reserved=52, charged=28, refunded=24` and no outer-refund corruption; unconsumed `STATUS_NEG_EVEN_ROOT` and `STATUS_LOOKUP_MISS` propagate across `OP_RET` with outcome `NO_SOLUTION`, no new token types; a successful fallback consumes the miss token while the semantic token stays outstanding.

**BOC verdict.** Stage 1: the candidate carries no undeclared dependency when the callee table, frame stack, sub-budget verbs, and token rules are declared — `ImplicitDeps = 0`. Stage 2: unbounded recursion and implicit stack growth are rejected (unbounded state violates Obj 2 and F26's trip-ceiling certification); direct inlining loses at scale (68,796 B vs 5,157 B at P=64, m=16, plus `S_max` refusal at m=5). The retained candidate is **bounded acyclic `OP_CALL`/`OP_RET` under `qir-enc-4`** with a statically acyclic callee graph, `C_max = 8`, `F_max = 64` (512 B ceiling), compositional `B_hat`, and exact §3.3 sub-budget settlement. This is a retained candidate for specification, not a canonized opcode.

## The evidence (runs of record, this commit state)

The harness exits 0; `call_trade_results.json` parses with 30 + 5 + 270 rows and 4 checks, all conservation flags true. The measurement uses portable SHA-256 (QO-33) and a single host, so absolute constants are not portability claims; the orderings (storage, state-cardinality, depth-driven P2 bloat, memoization) are the transferable findings. The `V.encoding` inspection of `src/qnir/src/insn.rs`, `src/qnir/src/containers.rs`, `src/qnir/src/main.rs`, and `src/qnir-py/qnir_ref.py` confirmed the current artifacts only store the encoding text and dispatch extension behavior by CLI fixture, not by `V.encoding`; the specification share therefore closes that gap by requiring `decode_envelope`/`dec_insn` to reject tags `30`–`31` and envelope key `9` outside `qir-enc-4`.

## Disposition

The study is recorded as measurement evidence for `docs/qnir_calls.md` §8 (`v0.3.0-draft.35`). No extension is sealed, no opcode is canonized into the frozen core, and no promotion is claimed: QnIR remains `[PENDING]`; `Q1`, `Q3`, and `Q6` remain `QUEUED`. Share 2 (`--fixture K`) and the remaining Step-6 extensions (Saturation, Fork–join, Road proofs) are forward. This log introduces no term include. Declared non-edits: `src/`, `docs/qnir_goldens.md`, `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/qnir_commitments.md`, `docs/qnir_containers.md`, `docs/index.md`, and `docs/_data/glossary/*`. No push is made until the maintainer approves.
