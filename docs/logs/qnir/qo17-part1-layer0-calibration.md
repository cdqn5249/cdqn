---
layout: default
title: QO-17 Part 1 — The Layer-0 Physical Calibration Benchmark (Native Windows, v0.3.0-draft.31)
description: Deterministic Layer-0 timing of the committed QnIR core on an Intel Core i7-8565U — the memory-wall ladder, SHA-256, CBOR, arithmetic scaling, B_hat against S_max, and end-to-end X1–X51 ns-per-U — recorded as measurement evidence, not a canon change
nav_exclude: true
---

# QO-17 Part 1 — The Layer-0 Physical Calibration Benchmark (Native Windows, v0.3.0-draft.31)

Measurement record of the `QO-17` Part 1 core Layer-0 physical calibration benchmark, run read-only against `441b73f` (v0.3.0-draft.31) on native Windows. Decision wave: v0.3.0-draft.31. **No repository file was created, edited, or deleted by the run**: the harness is a standalone crate in the run environment's temp directory that copies the committed modules read-only, and the repository was `git status --porcelain`-clean at start and end. The measurement subject is the committed QnIR core over the frozen 16-opcode set; the record changes no QnIR semantics and seals no design.

## The selection

`QO-17` was opened because §3.6 of `docs/qnir_accounting.md` records the non-discharge register for `F14`: the accounting model charges logical work `U` by opcode class, but the register states no empirical relation between `U` and physical time on real silicon. The Standing Governance Queue's `Q5` entry is trigger-gated on **"Deterministic measurement runs available"**, and the draft.31 stabilization wave is the first commit state whose artifacts, corpus, profile, and `V` tuple all agree, so a measurement taken here is taken against a single coherent object.

Part 1 was scoped to the Layer-0 questions the canon poses and nothing else:

1. the physical memory hierarchy the `τ_t` tiers name, measured by a dependent pointer chase that defeats stride prefetch;
2. the implementation-work primitives the artifacts actually execute (SHA-256, canonical CBOR decode/re-encode, exact arithmetic);
3. the `B_hat(P)` admission surrogate against the newly sealed `S_max = 4096`;
4. end-to-end `X1`–`X51` timing against logical `Work(E)`, the `F14` domination question itself.

The harness is native Windows by construction — `rustc 1.99.1` (`x86_64-pc-windows-msvc`), release profile, `opt-level = 3`, LTO, `codegen-units = 1`, offline build — so `QueryPerformanceCounter`/`Instant` timing reflects the local silicon rather than a WSL2 virtualized clock. Two warm-up runs are discarded and nine trials are recorded for every measurement; cycles are estimated as `ns × 1.80` from the 1.80 GHz base / invariant-TSC figure and are labelled estimates throughout.

Host of record: Intel Core i7-8565U, 4 cores / 8 threads; L1d 128 KiB + L1i 128 KiB per core, L2 1 MiB per core, L3 8 MiB; 15.84 GiB RAM; Windows 11 build 26300. Harness location: `C:\Users\dd_an\AppData\Local\Temp\opencode\qo17-bench\`. Two harness-side adaptations are declared: the temp copy of `interp.rs` omits the corpus-parsing/checking tail that binds to crate-root only, and it re-exports one accounting field so the harness can read `Work(E)`; the measured functions (`b_hat`, `cbor`, `sha256`, `value`, `run_vector`, `encode_record`, `trace_root`) are the committed bytes.

## What was emitted

**1. Layer-0 memory-wall tier ladder.** Single dependent 8-byte pointer chase over a randomly permuted index array (no stride, so hardware prefetchers cannot help), plus a sequential 64 KiB buffered read from `%TEMP%`. ns shown as min / median / p95 / max.

| Tier | Working set | ns | Cycles (median) | `τ_tier / τ_L1` |
|---|---|---:|---:|---:|
| L1d | 16 KiB | 3.369 / 3.369 / 3.516 / 3.516 | 6.1 | 1.00 |
| L2 | 128 KiB | 4.761 / 4.761 / 4.767 / 4.767 | 8.6 | 1.41 |
| L3 | 2 MiB | 11.030 / 11.155 / 14.461 / 14.461 | 20.1 | 3.31 |
| DRAM | 32 MiB | 119.488 / 125.023 / 127.047 / 127.047 | 225.0 | 37.1 |
| τ_storage | 64 KiB warm read | 121,200 / 133,100 / 166,500 / 166,500 | 239,580 | — |

The ladder is monotone and close to the textbook shape: ~1.4× to L2, ~3.3× to L3, ~37× to DRAM. The DRAM ratio measured **36.5×** on an earlier repeat of the same binary and **37.1×** in the saved JSON; the spread is repeat noise, not a different working set. The storage baseline is a whole-buffer transfer, not a single dependent access, so its ratio (≈39,500×) is not comparable: normalized it is **2.03 ns/byte** (133,100 ns / 65,536 B) for a warm OS-cached read.

**2. Core implementation-work primitives.**

SHA-256 (`sha256::sha256`) across payloads, with median throughput:

| Payload | ns (min/median/p95/max) | Cycles | MB/s |
|---|---|---:|---:|
| 64 B | 920 / 952 / 996 / 996 | 1,714 | 67.2 |
| 256 B | 1,865 / 1,981 / 2,014 / 2,014 | 3,566 | 129.2 |
| 1 KiB | 4,685.5 / 5,302 / 5,894.5 / 5,894.5 | 9,544 | 193.1 |
| 4 KiB | 16,856 / 17,220 / 17,438 / 17,438 | 30,996 | 237.9 |
| 16 KiB | 75,282 / 76,532 / 96,092 / 96,092 | 137,758 | 214.1 |

The fixed cost dominates below 1 KiB (a 64 B digest already costs ~1,700 cycles); throughput saturates near **238 MB/s** at 4 KiB and softens at 16 KiB, consistent with a portable byte-at-a-time implementation rather than a hardware SHA extension.

Canonical CBOR decode + re-encode over representative `X` envelopes:

| Envelope | Envelope bytes | ns (min/median/p95/max) | Cycles |
|---|---:|---|---:|
| `X3` | 234 | 3,000 / 3,200 / 3,400 / 3,400 | 5,760 |
| `X1` | 1,682 | 3,200 / 3,500 / 9,600 / 9,600 | 6,300 |
| `F27` | 387 | 3,200 / 3,200 / 3,700 / 3,700 | 5,760 |

Decode plus canonical re-encode is flat at ~3.2 µs across a 7× byte range, i.e. envelope-size-insensitive at these sizes; the dominant cost is per-node traversal, not bytes copied. The `X1` p95 spike (9,600 ns) is a scheduler artifact of the ~100 ns Windows timer quantum, not a size effect.

Exact arithmetic scaling. **The committed core's arithmetic is `i128`** (`checked_add`/`checked_sub`/`checked_mul`, and `CMP` over the same representation), so its native width is at most 2 words; the 4-word and 16-word columns are **harness-side grade-school bignum probes** used only to exhibit the shape the core would inherit if widths grew. Medians, ns:

| Op | 1 word | 4 words | 16 words | 16w / 1w |
|---|---:|---:|---:|---:|
| `ADD` | 42.5 | 110.0 | 65.0 | 1.5× |
| `MUL` | 47.0 | 60.0 | 260.0 | 5.5× |
| `DIV` | 2,963.0 | 12,841.5 | 66,225.0 | **22.4×** |
| `CMP` | 30.0 | 8.5 | 10.0 | 0.3× |

The `ADD`/`CMP` columns are inside the timer's noise floor and should be read as "flat". The signal is the pair `MUL`/`DIV`: multiplying 16× the words costs 5.5×, because schoolbook multiplication is `O(n²)` with a small constant; dividing costs **22.4×**, and 5.2× going from 4 to 16 words. Against the accounting model's flat `q_mul`/`q_div = 4 U`, physical cost and logical charge diverge by orders of magnitude as operand width grows — the width-parameterized dominance gap `F14` names.

**3. `B_hat(P)` DP scaling against `S_max = 4096`.** Across `X1`–`X51`, the per-vector median admission-surrogate latency spans **300 ns – 5,900 ns** (median 1,300 ns, p95 4,700 ns); the surrogate is cheap on real vectors. Synthetic loop-chain graphs (trip ceiling 1; `dp_bound = |B| · 2^k`) push it to the sealed ceiling:

| `dp_bound` | Blocks / back-edges | ns (min/median/p95/max) | Cycles | Result |
|---|---|---:|---:|---|
| 16 | 4 / 2 | 5,900 / 6,000 / 6,300 / 6,300 | 10,800 | admitted, 52 U |
| 256 | 16 / 4 | 106,200 / 107,200 / 119,000 / 119,000 | 192,960 | admitted, 116 U |
| 2,048 | 32 / 6 | 763,900 / 836,800 / 1,016,300 / 1,016,300 | 1,506,240 | admitted, 196 U |
| 4,096 | 64 / 6 | 1,565,700 / **1,581,800** / 2,300,100 / 2,300,100 | 2,847,240 | admitted, 324 U |
| 8,192 | 64 / 7 | 34,300 / **34,500** / 39,100 / 39,100 | 62,100 | **refused (`None`)** |

The ceiling behaves correctly: `4096` is admitted at **1.58 ms**, `8192` is refused, and the saved JSON records `bound = null` for the refusal. But the refusal is **34.5 µs, not sub-microsecond**: `b_hat` runs its structural analysis — `back_edges()` and the iterative `dominators()` fixed point, `O(|B|³)` over 64 blocks — before it can compare against `S_max`. The DP itself is cheap; the structural pass dominates. A genuinely `O(|B|+|E|)` refusal would have to count back-edges without the full dominator pass. That is a design finding for `QO-17`, not a defect exercised by any committed vector.

**4. End-to-end `X1`–`X51` against logical `Work(E)`.** Per-vector `Work(E)`, record count `n`, envelope bytes, and wall-clock time are recorded for all 51 vectors in the saved artifacts. For the **49 vectors with `Work(E) > 0`**:

| Statistic | `ns / U` |
|---|---:|
| min | 1,595.8 (`X44`, U=24, n=2, 289 B) |
| median | 2,525.0 |
| p95 | 4,793.4 |
| max | 4,994.4 (`X32`, U=72, n=14, 844 B) |

At the median that is **4,545 cycles/U**. Worst-case outliers: `X32` 4,994, `X22` 4,890, `X35` 4,858, `X17` 4,793 (U=76, n=15, 860 B), `X15` 4,772, `X12` 4,692. Best: `X44` 1,596, `X39` 1,622, `X3` 1,644, `X40` 1,650. The high-work vectors are tight (U=72 within 1.11×, U=76 within 1.06×), but low-work vectors are not: the same `U=32` ranges 1,622–3,241 ns/U (**2.00×**), `U=36` 1,933–2,525 (1.31×), `U=44` 2,520–3,225 (1.28×). Two `Work(E) = 0` vectors — `X2` (refusal) and `X50` (escrow never opened) — still cost **32.2 µs** and **29.7 µs** respectively, because envelope decode, `B_hat`, and trace-root setup are paid before or outside the ledger.

The `F14` empirical conclusion is therefore negative in the strong form and useful in the weak form: **logical `Work(E)` alone does not uniformly bound physical execution time.** `U` is a good first-order predictor (the median band is tight relative to the 2.00× same-U spread), but a bound that holds across `X1`–`X51` must also carry record count `n`, envelope/record byte size, `B_max` (and, today, the non-linear dominator pass), and width-parameterized `DIV`/`MUL` terms. `U` orders work; it does not by itself dominate wall time.

## The evidence (runs of record, this commit state)

The saved artifacts are the output of the final harness run at this state (`rustc 1.99.1`, release, LTO), which exited 0; the six artifact transcripts recorded in the draft.31 wave record also exit 0 at the same commit state. Run artifacts of record, in the run environment's temp directory:

- `qo17_part1_results.json` — 19,625 bytes, valid JSON (`json.load` parses it), with **5** memory-tier entries, **5** SHA-256 entries, **3** CBOR entries, **12** arithmetic entries, **56** `b_hat` entries (51 vectors + 5 synthetic), and **51** end-to-end entries.
- `qo17_part1_results.csv` — 138 lines across the six section tables (each section carries its own header row), same values.
- **Audit fingerprint: `aa78f491a6b942bb6ee33bb4f9bcf390db042700256527425451145b7609571e`** (SHA-256 of `qo17_part1_results.json` as saved).

The measured object was verified unchanged before the run: `docs/qnir_goldens.md` is **1,080,404 bytes, 22,151 LF lines, 0 CR bytes, SHA-256 `fa72f316c2f3bccc94260bbe6f75abecfe18895ab22a551705301d60070c92b3`** — the draft.31 corpus identity of record. The harness read `C:\cdqn` only; `git status --porcelain` was empty at the start and at the end of the wave, and no file inside the repository was created, edited, or deleted.

These runs are deterministic in the sense the trigger requires — same binary, same corpus, same warm-up/trial discipline, repeatable verdicts and bound values — and the `X1`–`X51` per-vector `ns/U` figures are the measured input to the `F14` finding above. They are a single-host Layer-0 calibration, not a portability claim: timing estimates inherit the 1.80 GHz TSC assumption and the ~100 ns Windows timer quantum visible in the p95/max steps, and they do not control thermal state or cache set assignment.

## Disposition

**The `Q5` trigger `Deterministic measurement runs available` is satisfied by this record**, as that trigger is worded in the Standing Governance Queue at `docs/index.md`: there now exist deterministic, committed-state Layer-0 measurement runs against a single coherent `441b73f` object. Satisfying a trigger admits a design to the queue's pipeline; it seals nothing and it schedules nothing. In particular this record does **not**: promote QnIR (`[PENDING]` stands), amend any QnIR semantics, alter the accounting model's `U` charges, discharge `F14`, or seal any `Q5` artifact. The `F14` finding is recorded as evidence for the wave that prices the fix, with two concrete candidates it identifies — a width-parameterized `DIV`/`MUL` charge and a linear-time `S_max` refusal path.

Declared non-edits: `docs/qs_history.md`, `docs/qs.md`, `docs/index.md`, `docs/_data/glossary/*`, `docs/_data/navigation.yml`, all of `src/` (the harness copies modules; it does not modify them), `docs/qnir_goldens.md`, and the historical wave records. No push is made until the maintainer approves.
