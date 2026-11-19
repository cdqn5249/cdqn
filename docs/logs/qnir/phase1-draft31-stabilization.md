---
layout: default
title: Phase 1 — Pre-Step-6 Stabilization, Canon Sync, and the F6 Fixture (v0.3.0-draft.31)
description: The draft.31 stabilization wave — corpus-wide canonical sync, the qir-prof-golden-2 profile with the S_max gate, the declared F6 collision fixture, and the V re-declaration across all 58 sites; both artifacts pass all six runs and QnIR remains [PENDING]
nav_exclude: true
---

# Phase 1 — Pre-Step-6 Stabilization, Canon Sync, and the F6 Fixture (v0.3.0-draft.31)

Wave record of the pre-Step-6 stabilization wave under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.31. **The wave is committed at `441b73f`** — 22 files, +7,500/−6,888 — and closes the gap between the canon's declared parameters and the behavior the two committed artifacts already implement. It seals the `qir-prof-golden-2` profile and its `S_max` gate, declares and implements the `F6` collision fixture, and re-declares `V` across every declaration site. **Both artifacts pass all six runs of record; the corpus is byte-identical in every mode, and QnIR remains `[PENDING]` — no promotion is claimed or approached.**

## The selection

Step 5 closed at v0.3.0-draft.30 with an empty register. The recorded next increment was not Step 6: before any pricing of the Step-6 extension agenda, the working tree carried three divergences between what the canon declared and what the frozen encodings actually did. The wave selected the smallest increment that removes all three:

1. The profile row still named the legacy surrogate shape, while both implementations already gated `B_hat(P)` at a hard ceiling. The wave seals **`qir-prof-golden-2`** — `gateway 20, charge 4, trip_ceiling 1, s_max 4096` — and makes refusal explicit: `b_hat` returns `Option<u64>`, and `None` is terminalized as `EXPRESSION_TOO_COMPLEX`, never as the gateway default.
2. `V` had drifted: the version tuple was declared in 58 places with pre-draft.8.4 scalars. The wave re-declares **`V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)`** at all 58 sites.
3. The `F6` bucket-comparison fixture existed only as a Step-4 disposition ("deferred to Step 5"); Step 5 closed without it. The wave declares it in §4.4, implements it in both artifacts, and confines it to `--fixture F6`.

The increment carries no new encoding, no new opcode, and no corpus vector. It is canon-and-artifact alignment plus one declared test-only surface: the smallest `ΔW_worst` of the admissible set, with the frozen `qir-enc-1` bytes untouched by construction, and it leaves the promotion criterion exactly where draft.30 left it.

## What was emitted

1. **`qir-prof-golden-2` and the `S_max` gate.** `src/qnir/src/bhat.rs` computes the exact surrogate and refuses with `None` when the certified bound exceeds `s_max = 4096`; `src/qnir-py/qnir_ref.py` mirrors the same path, including the `parse_profile` `s_max` field. Refusal is `EXPRESSION_TOO_COMPLEX` in both artifacts; the gateway default is never substituted for a refused expression. Three unit tests cover the ceiling, the refusal path, and the admission path (`cargo test --offline`: **3/3**).

2. **The `V` re-declaration.** `V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)` now stands at all 58 declaration sites, and the lineage block of every run prints it verbatim.

3. **The `F6` collision fixture.** Declared in §4.4 as a test-only surfacing of `qir-artifact-cmp-1`, implemented in both artifacts, and reachable only through `--fixture F6`. It exercises the four comparison verdicts — `same`, `collision` (`class = digest-collision`), `different` (same kind), `different` (differing kind) — and reports a one-entry bucket inside the fixture block. Normal mode is unchanged: `lineage: collision bucket (qir-artifact-cmp-1, entries = 0)`. The fixture normalizes no defective execution and is never a conforming golden.

4. **Canonical sync (Wave 1A).** The declarations were advanced additively, preserving every historical record: `docs/qnir_accounting.md` v0.3.0-draft.8.3, `docs/qnir_semantics.md` v0.3.0-draft.8.4, `docs/qnir_conformance.md` v0.3.0-draft.31, `docs/qnir_commitments.md` v0.3.0-draft.31, `docs/qnir.md` v0.3.0-draft.31, `docs/AiReviews/AiReviewQnIRv0.1.md` v0.1.14; and the companions `docs/simemp.md` v1.4.1, `docs/boc.md` v0.4.2, `docs/qnPrimitive.md` v1.4.1, `docs/abstractionLayers.md` v1.9.2, `docs/qm.md` v1.4.1, `docs/qexpr.md` v1.4.1, `docs/q0_q1.md` v1.2.2, `docs/history_library.md` v0.1.2, `docs/qs.md` v1.3.1.

5. **Two declared byte consequences, both recorded rather than hidden.** The semantics string's growth moved the `F27` envelope from 385 to 387 bytes, so that fixture block's digest moved deliberately from `ed67e6cbd242c69c` to `6f6bd8f5ea344b31`; and all 51 envelope byte-count annotations in the corpus were corrected by +2. No envelope hex byte was edited. The `draft.23`–`draft.30` records and the `F12, v0.3.0-draft.5` citation remain verbatim.

## The evidence (runs of record, this commit state)

Both artifacts were run against the committed corpus at `441b73f` in all three modes — normal, `--fixture F27`, `--fixture F6`. **All six runs exit 0 and PASS.** The toolchains of record: Rust reference interpreter (`rustc 1.99.1`, `x86_64-pc-windows-msvc`, release profile, standard library only) and the Python second implementation (CPython 3.12.10, AMD64, Windows; standard library only, `QO-35`). Transcripts are LF-normalized before hashing; every transcript is valid UTF-8 and already LF-only.

| Run | Bytes (LF) | Lines | `(qir-verbalizer-1, SHA-256 of the LF transcript)` |
|---|---:|---:|---|
| Rust, normal | 36,481 | 260 | `9e1c09d20cfddf69281e48a5f718846ece0b5c605a0290e81a65233e9a2199ee` |
| Rust, `--fixture F27` | 37,631 | 270 | `0b2fe19b893543615a77ff236cc2eb3f77d7614a4491cc8af71a0e856ecb7bc3` |
| Rust, `--fixture F6` | 37,099 | 269 | `4726f51d1712faad494e824794a8607a586a4aac3eeb38377a5b0606afd667e1` |
| Python, normal | 18,551 | 123 | `b8c36a62156b3f879da47cb1055cb974cddb364effbcae76d0e76edcc26000cf` |
| Python, `--fixture F27` | 19,701 | 133 | `d643be27e08760ca305e1ffffe12dee0f972549c7122f7b9b4938bc40b52ae4d` |
| Python, `--fixture F6` | 19,169 | 132 | `b0ad2b112f270a6d06d44b9e09ca8e379f41ddbd61cd101032cad6c078ca9e06` |

A same-binary and same-source rerun reproduced all six digests byte-identically (reproducibility evidence only, never independent reproduction, §4.2).

The binding-level facts, identical in all six transcripts:

- **`declared seed: 84 vectors`**; suite X execution `51/51` at §4.1's five agreement points; both artifacts PASS.
- Header pins **`6/6`** (trace roots, §6.6), **`4/4`** (comparison pins, §6.7), and **`4/4`** (comparison self-test, §6.7).
- **52 trace-root lines** — one `scope = run` plus `X1`–`X51` — byte-identical across implementations and modes.
- `lineage: collision bucket (qir-artifact-cmp-1, entries = 0)` in every normal and F27 run; the F6 fixture block alone reports `entries = 1`, and states `normal mode unchanged: n = 336, entries = 0`.
- Corpus identity: `lineage: corpus identity (corpus, 0.3.0-draft.31, fa72f316c2f3bccc94260bbe6f75abecfe18895ab22a551705301d60070c92b3)` over **1,080,404 LF bytes** (22,151 lines, 0 CR bytes).
- Evidence: `lineage: evidence (n = 336 records, E = 0765cd5e665ae77303b5bcfce21b3d7a686af8726a1e365d2687220dc97dee75)`.
- `lineage: V = (v0.3.0-draft.8.4, qir-enc-1, qir-prof-golden-2)`.
- Run root: `lineage: trace root (qir-trace-root-1, scope = run, n = 336, root = 5c6c0b21fba2df0bff219d01270848038046dcfb33eb509375684e9d57408a79)`.
- Implementation identities (differ by construction, §6.2): Rust `(implementation, 0.3.0-draft.31, 7d204647280a49e03fed16a19fb552e3094b3672f9770d564b37e3c88796bf0c)`; Python `(implementation, 0.3.0-draft.31, f9cf76560da5a263b3f3690a724ff806ed0ab6091c2dd197e788e80f61a9cbff)`.

The two fixture-block digests, recomputed from the transcripts by the §4.4 block rule (SHA-256 over the fixture block's lines, header through the line before its `fixture …: PASS` verdict, LF-joined with a trailing LF, first 16 hex printed):

| Fixture | Block digest | Cross-implementation |
|---|---|---|
| `F27` fuel backstop | `6f6bd8f5ea344b31` | identical in Rust and Python |
| `F6` artifact comparison | `498609033d35cf03` | identical in Rust and Python |

The `F27` block reads `B_injected = 32 U`, `W_budget = 36 U`, `B_hat(P_f) = 40 U`; control run REFUSED, injected run ADMITTED with the backstop firing at step 4, `charged <= reserved` at equality, and the verdict "validator defect evidenced; containment held". The `F6` block reads the four declared pairs and `collision bucket: entries = 1 (class = digest-collision)`.

## Disposition

**Phase 1 is closed at `441b73f`.** The stabilization wave is complete and committed; Step 6 is not started, and this record prices nothing in it. The promotion criterion is unchanged and unmet: QnIR remains `[PENDING]` in `docs/index.md`, no claim of this draft is styled a theorem, and Step 5's `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional. The `F6` fixture is a declared test surface, not a conformance path; the `S_max` gate is a declared refusal, not a new arithmetic.

Declared non-edits: the corpus's envelope hex bytes (only the 51 +2 byte-count annotations changed, recorded above), the historical `draft.23`–`draft.30` wave records, the `F12, v0.3.0-draft.5` citation, `docs/index.md` (QnIR `[PENDING]` stands), `docs/_data/glossary/*`, `docs/_data/navigation.yml`, and the AiReview bodies (append-only — the v0.1.14 note records this wave). No push is made until the maintainer approves.
