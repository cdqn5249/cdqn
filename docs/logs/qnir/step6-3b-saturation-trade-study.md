---
layout: default
title: Step 6.3b — Saturation Placement & Saturating Arithmetic Trade Study (Read-Only)
description: Native-Windows measurement of workflow-scoped OP_SATURATE (QO-28) against a Layer-3 Q(rsi) transform placement, and fixed-width saturating arithmetic against qir-enc-2 core macro-lowering, with F14 break-even, memory footprints, and BOC adjudication
nav_exclude: true
---

# Step 6.3b — Saturation Placement & Saturating Arithmetic Trade Study (Read-Only)

Measurement record of the two Step-6.3b candidates under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. The harness is a standalone crate at `%TEMP%\opencode\step6-3b-saturation-study\`, built with `rustc 1.99.1` (`+1.99.1`, `x86_64-pc-windows-msvc`, `--release --offline`, LTO, `codegen-units=1`), thread pinned to core 2, **2 warm-up + 9 measured trials**, cycles at the declared **1.80 GHz** estimate — the `step6-1`/`step6-2`/`step6-3a` methodology. **No repository source or spec byte was changed by the run**; this log is the only new file. Nothing is committed and no extension is sealed.

## 1. Read-only spec inspection (boundaries)

- **`docs/qexpr.md` §4.6.2**: general equality saturation is a **workflow-scoped optimization operation**, legal only inside an explicit `Q(reuse)`-class `Q(rsi)` workflow whose DCC profile pins, without defaults, (1) a deterministic rule schedule, (2) `K_max`, (3) `N_max` and `D_max`, and (4) a deterministic polynomial-time greedy extractor with hash tie-breaking. Convergence emits `RECEIPT_SATURATION_CONVERGED`; ceiling exhaustion emits `RECEIPT_BUDGET_EXHAUSTED` or `RECEIPT_EXPRESSION_TOO_COMPLEX` with reason codes `K_MAX_REACHED`, `N_MAX_REACHED`, `D_MAX_REACHED`. The v1.4.1 binding adds the §2.2 hashing-generation tag and comparator version to the **new Axiom 13 versioned artifact**; saturation never mutates canonical node identity or participates in `H(α)`.
- **`docs/qnPrimitive.md` §5.9**: circuit breakers 1–4 are `Depth(DAG) > D_max` → `D_MAX_REACHED`, `Count(Nodes) > N_max` → `N_MAX_REACHED`, `Burn(U) > Budget_Q1` → `U_BUDGET_EXHAUSTED`, and `Epochs(OP_SATURATE) > K_max` → `K_MAX_REACHED`. `Q(bypass)` emits `RECEIPT_BYPASS_IDENTITY` at `ΔW = 0`; `Q(reuse)` two-axis admission uses `β_dcc = 2`, `β_t = 4`, with ties resolving to Direct Compute.
- **`docs/qnir.md` §1.2**: the frozen core admits exact integer arithmetic and rejects floating point and saturation; §1.3 defers Saturation to the Step-6 adjudication and explicitly records that **fixed-width saturating arithmetic is a separate candidate**, outside the exact-integer core by default. No landing slot is invented for either; this study prices them, it does not promote them.

## 2. What was measured

**(a) Mini CAE-DAG saturation.** Deterministic breadth-first epochs over ascending node ids; `OP_BYPASS` identities (`+0`, `×1`, `×0`, `x−x`), commutativity (`+`, `×`), distributivity (`a×(b+c)`); union-find e-classes with hash-consed nodes; greedy extractor with SHA-256 tie-breaking; `D_max` before allocation, `N_max` at insertion (including epoch-generated nodes), `K_max` at epoch boundary. Four source DAGs: `algebraic` (identity chain, depth 9), `commutativity` (depth 8 chain), `qchem_qm` (balanced Qchem/Qm-style tree), `adversarial` (nested distributivity, depth 13, 19 source nodes). Sweep: `K_max ∈ {1,2,4,8}` at `N=1024,D=16`; `N_max ∈ {64,256,1024,4096}` at `K=4,D=16`; `D_max ∈ {8,16,32}` at `K=4,N=1024`. 44 rows total. `U` uses the T16 Step-6.3a F14 anchor `1 U = 2.05 ns`; `direct_U` is the unoptimized DAG's core-op charge (`4 U` per op); `break_even_m = ceil(U_sat / direct_U)` is the number of reuses needed to pay back one saturation pass against direct `qir-enc-5` execution.

**(b) Saturating arithmetic.** Native `saturating_add/sub/mul` over `u8/u16/u32/u64`, versus an arithmetic emulation using `CONST + ADD/SUB/MUL + CMP + BR + CONST` (5 core opcodes, 352 B stream, `20 U`). Exact core preservation was asserted: the emulation uses only tags `1..16`, with no saturation opcode and no alteration to `qir-enc-2` exact arithmetic (`CORE_QIR_ENC2_EXACT_ARITHMETIC_PRESERVED=true`).

## 3. Exact saturation tables

`verdict` abbreviations: `K` = `K_MAX_REACHED`, `N` = `N_MAX_REACHED`, `D` = `D_MAX_REACHED`, `C` = `RECEIPT_SATURATION_CONVERGED`. `mem` = e-graph bytes (0 when refused before allocation). All four verdict classes were exercised (7 K, 1 N, 2 D, 34 C).

**K sweep (`N=1024`, `D=16`):**

```
graph         K  verdict  nodes  mem   ns       cyc      U       direct_U  m
algebraic     1  K        18     2016  25562.5  46012.5  12470   32        390
algebraic     2  K        22     2464  35547.5  63985.5  17341   32        542
algebraic     4  C        22     2464  40225.8  72406.5  19623   32        614
algebraic     8  C        22     2464  40282.5  72508.5  19650   32        615
commutativity 1  K        22     2464  29989.2  53980.5  14629   28        523
commutativity 2  C        22     2464  32661.7  58791.0  15933   28        570
commutativity 4  C        22     2464  32790.0  59022.0  15996   28        572
commutativity 8  C        22     2464  32945.0  59301.0  16071   28        574
qchem_qm      1  K        17     1904  25220.0  45396.0  12303   28        440
qchem_qm      2  K        23     2576  35092.5  63166.5  17119   28        612
qchem_qm      4  C        23     2576  38355.8  69040.5  18711   28        669
qchem_qm      8  C        23     2576  38520.0  69336.0  18791   28        672
adversarial   1  K        49     5488  76497.5  137695.5 37316   48        778
adversarial   2  K        66     7392  236432.5 425578.5 115333  48        2403
adversarial   4  C        66     7392  125335.0 225603.0 61140   48        1274
adversarial   8  C        66     7392  123090.0 221562.0 60044   48        1251
```

**N sweep (`K=4`, `D=16`):**

```
graph         N     verdict  nodes  mem   ns       cyc      U       m
algebraic     64    C        22     2464  41542.5  74776.5  20265   634
algebraic     256   C        22     2464  40824.2  73483.5  19915   623
algebraic     1024  C        22     2464  40360.8  72649.5  19689   616
algebraic     4096  C        22     2464  40608.3  73095.0  19809   620
commutativity 64    C        22     2464  32625.0  58725.0  15915   569
commutativity 256   C        22     2464  32797.5  59035.5  15999   572
commutativity 1024  C        22     2464  33960.0  61128.0  16566   592
commutativity 4096  C        22     2464  32856.7  59142.0  16028   573
qchem_qm      64    C        23     2576  38688.3  69639.0  18873   675
qchem_qm      256   C        23     2576  41004.2  73807.5  20003   715
qchem_qm      1024  C        23     2576  38790.8  69823.5  18923   676
qchem_qm      4096  C        23     2576  39476.7  71058.0  19257   688
adversarial   64    N        64     7168  110382.5 198688.5 53846   1122
adversarial   256   C        66     7392  123820.0 222876.0 60401   1259
adversarial   1024  C        66     7392  121862.5 219352.5 59446   1239
adversarial   4096  C        66     7392  123157.5 221683.5 60077   1252
```

**D sweep (`K=4`, `N=1024`):**

```
graph         D   verdict  nodes  mem  ns      cyc     U     direct_U  m
algebraic     8   D        10     0    60.0    108.0   30    32        1
algebraic     16  C        22     2464 40248.3 72447.0 19634 32        614
algebraic     32  C        22     2464 41400.0 74520.0 20196 32        632
commutativity 8   C        22     2464 33359.2 60046.5 16273 28        582
commutativity 16  C        22     2464 33071.7 59529.0 16133 28        577
commutativity 32  C        22     2464 32568.3 58623.0 15887 28        568
qchem_qm      8   C        23     2576 40321.7 72579.0 19670 28        703
qchem_qm      16  C        23     2576 41145.0 74061.0 20071 28        717
qchem_qm      32  C        23     2576 38832.5 69898.5 18943 28        677
adversarial   8   D        19     0    80.0    144.0   40    48        1
adversarial   16  C        66     7392 120917.5 217651.5 58985 48        1229
adversarial   32  C        66     7392 124535.0 224163.5 60749 48        1266
```

Ceiling behavior is honest: `K=1/2` refuses most graphs, `N=64` refuses only the expanding DAG at its 64th enode, and `D=8` refuses both deep source DAGs before allocation. E-graph memory tops out at **7,392 B** (66 enodes) for the adversarial DAG; all converged runs fit in well under 8 KiB. Break-even reuses are **390–2,403** for converged passes — this is a compile-time amortization number, not a runtime one. The extractor is a study-side unoptimized implementation; absolute constants are single-host observations, while the ordering (saturation ≫ direct execution, ceiling refusals are cheap) is the transferable finding.

## 4. Saturating arithmetic table

```
width  op      native ns  native cyc  native B  emul ns  emul cyc  emul B  emul U  native U
u8     SAT_ADD 1.25       2.25        64        2.70     4.86      352     20      1
u8     SAT_SUB 1.00       1.80        64        3.90     7.02      352     20      1
u8     SAT_MUL 1.25       2.25        64        2.30     4.14      352     20      1
u16    SAT_ADD 1.25       2.25        64        3.80     6.84      352     20      1
u16    SAT_SUB 0.950      1.71        64        3.15     5.67      352     20      1
u16    SAT_MUL 1.30       2.34        64        3.65     6.57      352     20      1
u32    SAT_ADD 1.35       2.43        64        3.60     6.48      352     20      1
u32    SAT_SUB 1.20       2.16        64        3.55     6.39      352     20      1
u32    SAT_MUL 1.50       2.70        64        2.45     4.41      352     20      1
u64    SAT_ADD 1.95       3.51        64        1.75     3.15      352     20      1
u64    SAT_SUB 1.10       1.98        64        2.40     4.32      352     20      1
u64    SAT_MUL 1.20       2.16        64        2.25     4.05      352     20      1
```

Native saturation is `0.95`–`1.95 ns` (`1.71`–`3.51 cycles`, 64 B) versus macro-lowered emulation `1.75`–`3.90 ns` (352 B, `20 U`). Under the T16 F14 anchor the native charge is `1 U`; the emulation is declared `20 U` (five `4 U` core opcodes). For `u64`, a single `DIV`-free emulation occasionally wins on latency (branch prediction), but the declared `U` cost and stream size are still 20× and 5.5× larger. Exact `qir-enc-2` arithmetic is preserved in every case.

## 5. BOC adjudication

**Stage 1 (existential invariants).** Both candidates pass `I_Id` (content-addressed artifacts with a SHA-256 fingerprint; new saturation result is an Axiom 13 versioned artifact), `I_Met` (all ceilings finite and declared: `K_max`, `N_max`, `D_max`, `Budget_Q1`, `max_hash_bytes`; saturating width set `{8,16,32,64}` closed), and `I_Sec` (no new signing capability; saturation provenance binds hashing generation and comparator version). No candidate is discarded at Stage 1 on identity/metric/security grounds.

**(a) `OP_SATURATE` placement — VM opcode `qir-enc-6` vs Layer-3 `Q(rsi)` transform.**

| Candidate | `ImplicitDeps` | `W_worst` | `M_coupling` | `P_gap` |
|---|---:|---|---:|---|
| VM opcode `qir-enc-6` | > 0 — schedule, `K_max`/`N_max`/`D_max`, extractor generation, hashing generation, and comparator version must ride in every transition commitment | 12,470–115,333 U for one pass | > 0 — new envelope key/opcode plus runtime provenance surface | UNRESOLVED (single host) |
| Layer-3 `Q(rsi)` transform emitting new Axiom 13 `qir-enc-5` Intents | 0 — all operands declared at compile time and recorded once in the versioned artifact | amortized `m = 390–2,403` reuses; converged e-graph ≤ 7,392 B | 0 — existing `qir-enc-5` envelope, no new key | UNRESOLVED (single host) |

**Verdict: retain as a Layer-3 `Q(rsi)` compiler/harness transform; do not introduce `qir-enc-6`.** The measured per-pass work is thousands of declared units and its provenance is a compile-time artifact property, not a runtime state property; a VM opcode would bloat P2/transition commitments with data the runtime never consumes, while the transform emits the same new Axiom 13 Intents the spec already requires. `OP_SATURATE` therefore remains **workflow-scoped and non-canonical**, with `OP_BYPASS` continuing to govern canonical node hashing. No opcode tag or envelope key is allocated by this study.

**(b) `SAT_ADD`/`SAT_SUB`/`SAT_MUL` — retain vs macro-lower.**

| Candidate | Stage-1 admissibility | `W_worst` | `M_coupling` | Verdict |
|---|---|---:|---:|---|
| Native `SAT_*` opcode in the frozen core | **FAIL** — §1.2's default-deny closure excludes saturation from the exact-integer core | ~1 U/op, 64 B | new opcode | not admissible as core |
| Macro-lower to existing `qir-enc-2` `CONST`/`ADD`/`CMP`/`BR` sequence | PASS — uses only tags `1..16`, exact arithmetic unchanged | 20 U/op, 352 B | 0 | **RETAIN (macro-lower)** |

**Verdict: macro-lower.** The native form's ~19 U/op advantage is real on this host, but `docs/qnir.md` §1.2 explicitly fixes exact arithmetic as core and names fixed-width saturating arithmetic as a **separate candidate**; adding it to the core would break Stage-1 admissibility, so the Stage-2 `W_worst` ordering is moot. A future profile-gated extension may price a native `SAT_*` form with its own declared width/overflow contract; this study does not allocate a tag or amend the core.

`P_gap` remains UNRESOLVED for all candidates: one host, and no required target profile set `R` is declared here. BOC therefore issues no final total ordering and seals nothing; the transferable results are the *orderings* (compilation-only saturation, cheap ceiling refusals, native-vs-emulation cost gap) plus the admissibility findings above.

## 6. Artifacts of record

Standalone crate: `%TEMP%\opencode\step6-3b-saturation-study\` (the only location of code and build output).

```
ARTIFACT json=12003 bytes sha-256:d3f23ce579dcbe76bd4538a2bfcb587ee1318467e25df73797c845f23981ccc9
ARTIFACT csv=5166 bytes
```

`step6_3b_saturation_results.json` parses with 44 saturation rows and 12 saturating-arithmetic rows; both invariants (`qir_enc2_exact_core_preserved`, `receipts_registered`) are true. `step6_3b_saturation_results.csv` is the flat companion, `run_stdout.txt` the full transcript, and `step6_3b_saturation_results.json.sha256` the fingerprint.

## 7. Zero-drift confirmation

```
src/qnir/     test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
src/qnlangc/  test result: ok. 3 passed + 3 passed + 15 passed = 21/21; 0 failed
git status -s → ?? docs/logs/qnir/step6-3b-saturation-trade-study.md  (only entry)
```

No commit, no push, no source or spec edit. `docs/qnir_goldens.md`, `src/qnir/`, `src/qnir-py/`, `src/qnlangc/`, and all `docs/*.md` other than this new log are byte-unchanged. QnIR remains `[PENDING]`; Step 6.3b candidates remain forward and unsealed.
