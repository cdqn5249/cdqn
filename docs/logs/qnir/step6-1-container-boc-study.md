---
layout: default
title: Step 6.1 — The Container BOC Trade Study (Measured Ladder Pass, v0.3.0-draft.32)
description: The standalone native-Windows measurement of ChunkedFlat, HAMT, RRBTree and MerkleBTree under one enforced rule set — Q(array) and Q(table) row measurements, the BOC Stage-1/Stage-2 adjudication, and the deferred qir-enc-3 opcode contract
nav_exclude: true
---

# Step 6.1 — The Container BOC Trade Study (Measured Ladder Pass, v0.3.0-draft.32)

Measurement record of the Step-6.1 Container BOC trade study under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) v0.4.2, run read-only against the committed `6cc1ef1` (v0.3.0-draft.32) canon. **No repository file was created, edited, or deleted by the measurement run**: the harness is a standalone crate in `C:\Users\dd_an\AppData\Local\Temp\opencode\container-boc\`, and `git status --porcelain` was empty at its start and end. The study changes no QnIR semantics, declares no opcode, and seals no structure: it discharges the **row-measurement** and **BOC-selection** parts of Ladder Steps 2–3 for two niches, leaving sealing (Step 5) and the core-extension admission to a later wave.

## The selection

`docs/qnir.md` §1.3's Containers row assigns the family to **Step 6 — Container BOC**, with the binding constraints: identical canonical serialization, hash function, persistence policy, cache model, and batch rules; measured distributions under the declared lexicographic Tier-2 order; and the opcode contract split by container type or made profile-specific if winners diverge. Obj 7 forbids canonizing a lowering before the Container BOC evidence exists, and `QO-19` records that neither classical contender is selectable from asymptotic update cost alone. The selected increment is therefore measurement only: a common workload over four persistent `u64 → 32-byte value` maps, with all differences attributable to structure rather than to rules.

**Enforced common rules** (identical in every candidate, no runtime negotiation): SHA-256 32-byte node hashes, with each content leaf hashed as `SHA-256(0x00 ‖ key_le ‖ value)` in every structure; every internal node commits `SHA-256(dom ‖ meta ‖ pairwise-Merkle-root)` over its child hashes (`0x20` pair domain, odd promotion, never duplication), so every witness is O(log n); copy-on-write structural persistence — `set` allocates a new root-to-leaf path and shares every untouched subtree; deterministic canonical CBOR as one ascending-key map `{u64 => bstr(32)}`; and one sorted-batch normalization — the same 64 updates, stable-sorted ascending before the single commit (at N = 16 the batch necessarily revisits keys). Versioning-allocation accounting excludes hash scratch through a `scratch_depth` guard; it counts persistent nodes and cloned arrays only.

**The four candidates.** `ChunkedFlat` (sorted persistent chunks of 32, binary Merkle index), `HAMT` (16-way hash trie, SHA-256 key digest), `RRBTree` (persistent 16-way radix vector, positional `0..N`), and `MerkleBTree` (persistent sorted B-tree, order/fanout 16). The two niches are `Q(array)` — dense keys `0..N`, all four candidates — and `Q(table)` — sparse keys `i * 0x9E3779B97F4A7C15`, the three keyed candidates; the positional RRB cannot represent that key space and is correctly absent there. Sizes are `N ∈ {16, 256, 4096, 65536}`. The five measured rows are point lookup, persistent update plus incremental root, sorted batch of 64, deterministic sequential fold, and footprint/witness.

## What was emitted

**The artifact.** `container_boc_results.json` (25,380 bytes, SHA-256 **`abed3d5d05788ed7427ebb4df090cbf4265809ee306e72b8e8caa9e73605f36c`**) and its 7,886-byte CSV companion, both in the temp harness directory. They hold 28 measurement rows (16 dense + 12 sparse) and 12 enforced-rule checks, all PASS:

- leaf-hash multiset identity at every `(niche, N)` — 8 checks, PASS across 4 structures dense / 3 sparse;
- canonical CBOR identity at `N = 256` and `4096` — 4 checks, PASS with digests `11ed127f…d5c1b` (dense 256), `60d0ddb1…7ab7f3` (sparse 256), `9f2c068b…80d68` (dense 4096), `35434ff9…7c54e3` (sparse 4096). The CBOR artifact is structure-independent: 9,195 / 11,003 bytes at 256 and 151,275 / 176,123 bytes at 4096.
- all 28 rows have `len_ok = true` and `values_ok = true` (ascending fold covering the identical key set).

Toolchain and host: `rustc 1.99.1` (`x86_64-pc-windows-msvc`), release, LTO, one codegen unit, offline build; Intel Core i7-8565U (4c/8t, L1d 128 KiB, L2 1 MiB, L3 8 MiB, 15.84 GiB RAM), Windows 11. Timing: 2 warm-up + 9 trials; cycles at the declared 1.80 GHz figure; `ABOVE_NORMAL_PRIORITY_CLASS` and thread affinity to core 2. Small-`N` folds are repeated (`reps = 65536/n`, clamped 1..2048) to clear the ~100 ns timer floor; the folding penalty of HAMT's required sort is included in its numbers, not hidden.

**Dense positional niche — `Q(array)`, keys `0..N`.**

| N | Structure | get med ns | get p95 ns | set med ns | set p95 ns | set SHA | alloc B/upd | batch total ns | batch ns/item | batch SHA | fold med ns | fold ns/el | mem B | tier | CBOR B | proof B |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|
| 256 | ChunkedFlat | 10.9 | 19.4 | 9475.0 | 14100.0 | 14 | 4560 | 592200 | 9253.1 | 896 | 400 | 1.6 | 30264 | L1 | 9195 | 278 |
| 256 | RRBTree | 3.1 | 3.1 | 7818.8 | 13168.8 | 11 | 3072 | 528400 | 8256.2 | 704 | 450 | 1.8 | 29400 | L1 | 9195 | 280 |
| 256 | HAMT | 516.6 | 672.2 | 10406.2 | 18768.8 | 17 | 2920 | 630700 | 9854.7 | 1006 | 4764 | 18.6 | 72808 | L1 | 9195 | 289 |
| 256 | MerkleBTree | 12.9 | 13.6 | 8987.5 | 14587.5 | 13 | 3498 | 560900 | 8764.1 | 812 | 384 | 1.5 | 33544 | L1 | 9195 | 312 |
| 4096 | ChunkedFlat | 21.7 | 35.4 | 15275.0 | 22150.0 | 22 | 5808 | 970300 | 15160.9 | 1408 | 8938 | 2.2 | 488184 | L2 | 151275 | 414 |
| 4096 | RRBTree | 4.8 | 7.8 | 11443.8 | 15837.5 | 16 | 4416 | 700100 | 10939.1 | 1024 | 11594 | 2.8 | 471768 | L2 | 151275 | 416 |
| 4096 | HAMT | 566.1 | 836.8 | 14556.2 | 23725.0 | 20 | 4316 | 1007600 | 15743.8 | 1388 | 142656 | 34.8 | 1181072 | L3 | 151275 | 426 |
| 4096 | MerkleBTree | 27.0 | 36.7 | 12812.5 | 20000.0 | 18 | 4986 | 794700 | 12417.2 | 1140 | 8400 | 2.1 | 528560 | L2 | 151275 | 448 |
| 65536 | ChunkedFlat | 58.1 | 109.6 | 21193.8 | 34943.8 | 30 | 7056 | 1320500 | 20632.8 | 1920 | 165300 | 2.5 | 7814904 | L3 | — | 550 |
| 65536 | RRBTree | 15.5 | 39.4 | 15218.8 | 24987.5 | 21 | 5760 | 998000 | 15593.8 | 1344 | 136300 | 2.1 | 7549656 | L3 | — | 554 |
| 65536 | HAMT | 662.1 | 1143.9 | 19662.5 | 32950.0 | 26 | 5786 | 1273200 | 19893.8 | 1793 | 8301700 | 126.7 | 18846640 | DRAM | — | 566 |
| 65536 | MerkleBTree | 54.8 | 120.5 | 16725.0 | 31156.2 | 23 | 6456 | 1065800 | 16653.1 | 1440 | 163300 | 2.5 | 8445528 | DRAM | — | 584 |
| 16 | ChunkedFlat | 3.9 | 6.6 | 4193.8 | 5750.0 | 6 | 1936 | 214600 | 3353.1 | 321 | 23 | 1.4 | 1864 | L1 | — | 141 |
| 16 | RRBTree | 1.6 | 1.9 | 4093.8 | 4543.8 | 6 | 1728 | 271500 | 4242.2 | 384 | 21 | 1.3 | 1752 | L1 | — | 142 |
| 16 | HAMT | 490.9 | 629.5 | 5893.8 | 16918.8 | 7 | 1426 | 457400 | 7146.9 | 600 | 235 | 14.7 | 4016 | L1 | — | 218 |
| 16 | MerkleBTree | 6.8 | 7.1 | 5281.2 | 10443.8 | 8 | 2005 | 320600 | 5009.4 | 501 | 27 | 1.7 | 2400 | L1 | — | 176 |

**Sparse keyed niche — `Q(table)`, keys `i * 0x9E3779B97F4A7C15`.**

| N | Structure | get med ns | get p95 ns | set med ns | set p95 ns | set SHA | alloc B/upd | batch total ns | batch ns/item | batch SHA | fold med ns | fold ns/el | mem B | tier | CBOR B | proof B |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|
| 256 | ChunkedFlat | 10.7 | 16.9 | 9462.5 | 12793.8 | 14 | 4560 | 597900 | 9342.2 | 896 | 411 | 1.6 | 30264 | L1 | 11003 | 278 |
| 256 | HAMT | 520.7 | 666.0 | 10525.0 | 19343.8 | 17 | 2830 | 649900 | 10154.7 | 1011 | 4638 | 18.1 | 73448 | L1 | 11003 | 252 |
| 256 | MerkleBTree | 12.9 | 21.0 | 11231.2 | 20037.5 | 13 | 3480 | 656300 | 10254.7 | 804 | 393 | 1.5 | 33544 | L1 | 11003 | 312 |
| 4096 | ChunkedFlat | 21.6 | 35.2 | 15181.2 | 24168.8 | 22 | 5808 | 962300 | 15035.9 | 1408 | 8900 | 2.2 | 488184 | L2 | 176123 | 414 |
| 4096 | HAMT | 573.5 | 897.4 | 14906.2 | 21943.8 | 23 | 4325 | 936100 | 14626.6 | 1398 | 143994 | 35.2 | 1178648 | L3 | 176123 | 426 |
| 4096 | MerkleBTree | 26.4 | 37.3 | 12987.5 | 19200.0 | 18 | 4968 | 803100 | 12548.4 | 1123 | 8219 | 2.0 | 528560 | L2 | 176123 | 448 |
| 65536 | ChunkedFlat | 57.8 | 103.8 | 22037.5 | 34081.2 | 30 | 7056 | 1389800 | 21715.6 | 1920 | 179200 | 2.7 | 7814904 | L3 | — | 550 |
| 65536 | HAMT | 674.3 | 1093.5 | 20943.8 | 32556.2 | 31 | 5814 | 1326000 | 20718.8 | 1776 | 8547000 | 130.4 | 18898352 | DRAM | — | 672 |
| 65536 | MerkleBTree | 54.8 | 107.3 | 16956.2 | 31768.8 | 23 | 6474 | 1080500 | 16882.8 | 1449 | 172800 | 2.6 | 8445528 | DRAM | — | 584 |
| 16 | ChunkedFlat | 3.8 | 7.9 | 4268.8 | 4850.0 | 6 | 1936 | 219200 | 3425.0 | 321 | 24 | 1.5 | 1864 | L1 | — | 141 |
| 16 | HAMT | 513.3 | 1094.2 | 6112.5 | 8981.2 | 8 | 1449 | 426200 | 6659.4 | 621 | 182 | 11.4 | 4288 | L1 | — | 184 |
| 16 | MerkleBTree | 6.4 | 6.4 | 5512.5 | 9156.2 | 8 | 2005 | 330700 | 5167.2 | 485 | 28 | 1.8 | 2400 | L1 | — | 176 |

SHA-256 calls per `get`: 0 for ChunkedFlat, RRBTree, and MerkleBTree; 1 per lookup for HAMT (the key digest) — the reason its 500–670 ns lookups are flat in `N`. Update and batch calls are SHA-bound throughout (set 6 → 30 calls, batch 321 → 1,920 at the largest size), at roughly 0.6–0.75 µs per portable digest. The batch gives no per-item amortization: `batch ns/item` equals `set med ns` within a few percent everywhere, because every update still rehashes its root-to-leaf path.

**BOC Stage-1/Stage-2 adjudication (v0.4.2).** At **Stage 1** all four candidates pass the sealed-gateway requirement `ImplicitDeps(c) = 0`: the enforced rules are identical and declared, and the 12/12 identity checks show no undeclared serialization or hash dependency. At **Stage 2**, over the common workload above, the declared priority permutation is applied to the lexicographic key with Efficiency first. The measured ns/cycles, SHA-call, and allocation surfaces below are recorded as diagnostic measurements at execution boundaries (§10.6); they are **not** presented as a fabricated `MEASURED ΔW_worst` in `U`, whose container-opcode cost model belongs to `qir-enc-3` (Wave 4B). Their `ΔW_worst` value therefore remains `UNKNOWN` in the Tier-2 key sense. Consistent with §8.3 frontier reduction, each rejected candidate is strictly dominated on the recorded efficiency component at the decisive sizes, which makes its later components non-decision-affecting; the outcome is an explicitly recorded adjudication, not canonization.

- **`Q(array) → RRBTree-16` (selected).** Lowest lookup at every size (1.6 → 15.5 ns median; 2.9 → 27.9 cycles at 1.80 GHz), lowest persistent update (4,093.8 → 15,218.8 ns; 6 → 21 SHA calls; 1,728 → 5,760 alloc B), lowest batch (15.6 µs/item at 65,536; 1,344 SHA calls), tied-best fold (1.3 → 2.1 ns/element), smallest footprint (7,549,656 B at 65,536, inside L3; proof 554 B). At N = 4096 the CBOR artifact is the common 151,275 B.
- **`Q(table) → MerkleBTree-16` (selected).** Best keyed lookup (6.4 → 54.8 ns; 11.5 → 98.6 cycles), best set (5,512.5 → 16,956.2 ns; 8 → 23 SHA calls; 2,005 → 6,474 alloc B), best batch (5,167.2 → 16,882.8 ns/item; 485 → 1,449 SHA calls), best deterministic fold (1.8 → 2.6 ns/element), and 8,445,528 B at 65,536 (proof 584 B; CBOR 176,123 B at 4096). RRBTree is not admissible in this niche: its positional `0..N` key space cannot carry the sparse key set.

**Ladder Step 3 rejection record — findings, not erasures (§3 step 3).**

- **`HAMT` rejected.** Lookup is **30×–300×** slower than the dense winner across `N` (490.9 vs 1.6 ns at N=16 ≈ 307×; 516.6 vs 3.1 at 256 ≈ 167×; 566.1 vs 4.8 at 4096 ≈ 118×; 662.1 vs 15.5 at 65,536 ≈ 43×; against MerkleBTree in `Q(table)` it is 12×–80× slower), entirely from one mandatory SHA-256 key digest plus trie traversal. Its deterministic fold is ~50× worse at scale (130.4 vs 2.6 ns/element at N=65,536; 8,547,000 ns versus 172,800 ns), because key order must be restored by sorting. Its footprint, inflated by per-bitmap commitments and 32-byte child hashes, reaches **18.85–18.90 MB**, spilling past the 8 MiB L3 into DRAM (≈2.2–2.5× the selected structures), while its proof grows to 672 B in the sparse niche. Its one nominal advantage — unordered insertion — buys nothing in either declared workload.
- **`ChunkedFlat` rejected.** From `N ≥ 4096` its chunk-clone update path copies 32 keys, 32 values, and the chunk's Merkle levels: 5,808 → 7,056 alloc B per update (vs RRBTree 4,416 → 5,760), set 15,275 → 21,194 ns (vs 11,444 → 15,219), batch 15.16 → 20.63 µs/item (vs 10.94 → 15.59), and lookup 21.7 → 58.1 ns (vs 4.8 → 15.5). Its fold is only 1.2–1.3× worse, but that is not enough to overcome the update/copy penalty on either common workload. It remains revisitable with a different chunk size; the measured value at chunk = 32 is what is rejected.

## The evidence (runs of record, this commit state)

The harness ran against the read-only canon at `6cc1ef1`; the Part A fidelity gate recomputed the `qir-trace-root-1` run root `369ada2b…5866` from 336 records with zero root-width violations, matching the companion log `docs/logs/qnir/f-g0-1-state-root-repair-draft32.md`. The saved JSON parses under `json.load`, carries `part_a` (51 vectors, 49 scored) and `part_b` (28 rows, 12 checks, `all_checks_ok = true`), and its fingerprint is the `abed3d5d…05f36c` value above. Same-binary reruns reproduced the structural verdicts and identities; the absolute timings are a single-host calibration and inherit the 1.80 GHz cycle estimate, the ~100 ns timer quantum, and uncontrolled thermal/cache-set state. Portable SHA-256 dominates the update rows, so absolute constants do not transfer to SHA-NI silicon; the orderings do. The `QO-17` interleaved A/B accompanying the study is recorded in the companion log: +4.7 % aggregate median `ns/U`, `X32` 4,434.7 → 4,107.6 ns/U, `X2` +1.7 µs.

## Disposition

Step 6.1's Ladder **Step 2 (row measurement)** and **Step 3 (BOC selection)** are recorded for `Q(array)` and `Q(table)`, with the two formal rejections above kept on the register as findings. The corresponding additions to `docs/qstructures.md` v0.3.0 discharge §8 item 3 for those two families only. Nothing is sealed: `Q(matrix)` and `Q(stream)` await their own row measurements, final QnIR core extension admission is **Wave 4B**, and no container opcode is canonized by this record (Obj 7). The next wave declares **`qir-enc-3`** — the Step-6.1 container opcode specification — together with the **`Q1`/`Q3` residency contract**; the U-denominated worst-case key and the two-axis breakeven row remain contractual proposals until then. QnIR stays `[PENDING]`; `Q6` stays `QUEUED` — this log introduces no new term include.

Declared non-edits: `docs/qnir*.md`, `docs/index.md`, `docs/_data/*`, `src/`, `docs/qnir_goldens.md`, the glossary, and all historical wave records. No push is made until the maintainer approves.
