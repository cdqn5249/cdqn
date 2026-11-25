---
layout: default
title: Step 6.3a — Bootstrap-Profile BOC Trade Study (Read-Only)
description: Native-Windows measurement of SCC-additive B_hat, fixed-width bitwise operations, and governed in-IR SHA-256 against the committed global-product surrogate and arithmetic emulation, with F14 cost-model recommendations and BOC adjudication; OP_SATURATE/QO-28 remains Step 6.3b
nav_exclude: true
---

# Step 6.3a — Bootstrap-Profile BOC Trade Study (Read-Only)

Measurement record for the three Step-6.3 / Bootstrap-Profile prerequisites of `docs/qnlang.md` §7.3: **(1)** per-site `trip(K)` + SCC-additive `bhat`, **(2)** fixed-width bitwise operations, **(3)** governed `Q(hash)` with profile-bound cost and collision-bucket discipline. The study is **read-only** against `src/qnir/`, `src/qnir-py/`, `src/qnlangc/`, `docs/qnir_goldens.md`, and all `docs/*.md`: the harness lives only at `%TEMP%\opencode\step6-3-bootstrap-study\`, built with `rustc 1.99.1` (`+1.99.1`, `x86_64-pc-windows-msvc`, `--release --offline`, LTO, `codegen-units=1`), thread pinned to core 2, **2 warm-up + 9 measured trials**, cycles at the declared **1.80 GHz** estimate — the `step6-1`/`step6-2` methodology. No repository byte was changed by the run; this log is the only new file. Nothing is committed and no extension is sealed.

## 1. Read-only spec inspection (boundaries and next tags)

- **`docs/qnir.md` §1.2** closes the core at nine families under default-deny: exact arithmetic (family 3) excludes floating point and **saturation**; bounded structured control (family 5) excludes calls, call frames, fork–join, and unbounded iteration, and forbids budget admission by simple per-instruction summation (F4). Capacity constants are profile parameters, never core validity limits (Obj 2).
- **`docs/qnir.md` §1.3** registers the five deferred families and their landing slots. Calls and containers were discharged at Step 6.2 (`qir-enc-4`, `--fixture K`) and Step 6.1 (`qir-enc-3`, `--fixture C`). **Saturation, fork–join, and road proofs remain forward** with no landing slot invented. Re-entry of saturation requires the full `QO-28` tuple; fixed-width saturating arithmetic is a separate candidate.
- **`docs/qexpr.md` §4.6** splits Node Class 6: `OP_BYPASS` is the O(1) canonical contraction that governs hash-consing; `OP_SATURATE` (§4.6.2) is a work-scoped, `Q(rsi)`-gated bounded equality saturation. It requires a pinned deterministic schedule, `K_max`/`N_max`/`D_max`, a deterministic polynomial-time greedy extractor with hash tie-breaking, and emits a **new versioned artifact** (Axiom 13) that never mutates canonical node identity or `H(α)`. The v1.4.1 binding adds the §2.2 hashing-generation tag and comparator version to the saturation provenance.
- **`docs/qnlang.md` §7.3** names the three prerequisites measured here and keeps Stage 1/Stage 2 design targets until BOC evidence exists; QnLang stays `[PENDING]`.
- **Encoding headroom.** `src/qnir/src/insn.rs` commits opcode tags `1`–`31` (`OP_CONST`…`OP_RET`) and envelope keys `0`–`9` (key `9` = callee records under `qir-enc-4`). The next unallocated opcode tag is **`32`** and the next unallocated envelope key is **`10`**. This study does **not** allocate either; allocation is a specification act, not a measurement result.

## 2. (a) SCC-additive `bhat` vs. global-product `bhat`

The harness copies the committed global-product model read-only (dominators, declared back-edges, overflow-checked `|B|·(trip+1)^|BE|` gate, memoized max-path DP, `S_max = 4096`). The candidate implements Tarjan condensation `O(|V|+|E|)`, a per-SCC trip-ceiling DP, **exit-aware** local summaries (maximum charge per boundary: HALT inside the SCC, or the component entered by each cross-SCC edge), and max-path aggregation over the condensation DAG. Exit-awareness is load-bearing: a naive "sum of local maxima" over-counts fallback edges that leave an SCC (first model attempt: 35/51 mismatches; corrected: 0/51).

**Parity on the 51 core X vectors** (X2 is declared byte-identical to X1 and is bound to X1's parsed program):

```
X_VECTORS_PARSED=51
PARITY_51=true mismatches=0
X1   blocks=3   be=0  global=100  scc=100  eq=true states=3/3
X7   blocks=3   be=1  global=60   scc=60   eq=true states=6/5
X39  blocks=3   be=0  global=32   scc=32   eq=true states=3/3
X51  blocks=1   be=1  global=32   scc=32   eq=true states=2/2
```

All 51 rows are in `step6_3_bootstrap_results.json`; every row has `equal: true`. `dp_states` is the structural bound: the SCC form uses the **sum** of per-component `|SCC|·(trip+1)^|BE_SCC|`, the global form the **product** over all sites.

Synthetic `k` sequential loops (trip 1 and 3) and a nested two-loop SCC (`states=…` gives global/SCC; `u128::MAX` is the overflow sentinel at the pre-DP gate, reported as `>S_max`):

```
GRAPH sequential   k=1  trip=1 B=1  BE=1  SCC=1  states=2/2 adm=true B=32/32 eq=true ns=699.1/2472.2
GRAPH sequential   k=2  trip=1 B=2  BE=2  SCC=2  states=8/4 adm=true B=44/44 eq=true ns=2104.2/4075.6
GRAPH sequential   k=4  trip=1 B=4  BE=4  SCC=4  states=64/8 adm=true B=68/68 eq=true ns=11544.1/7385.3
GRAPH sequential   k=6  trip=1 B=6  BE=6  SCC=6  states=384/12 adm=true B=92/92 eq=true ns=47798.6/11845.5
GRAPH sequential   k=8  trip=1 B=8  BE=8  SCC=8  states=2048/16 adm=true B=116/116 eq=true ns=214115.8/15644.8
GRAPH sequential   k=12 trip=1 B=12 BE=12 SCC=12 states=>S_max/24 adm=false B=null/null eq=true ns=4204.9/7506.6
GRAPH sequential   k=16 trip=1 B=16 BE=16 SCC=16 states=>S_max/32 adm=false B=null/null eq=true ns=6205.1/9371.9
GRAPH nested-2-scc k=2  trip=1 B=3  BE=2  SCC=2  states=12/9 adm=true B=48/48 eq=true ns=4628.1/7016.8
GRAPH sequential   k=1  trip=3 B=1  BE=1  SCC=1  states=4/4 adm=true B=40/40 eq=true ns=1432.0/3518.4
GRAPH sequential   k=2  trip=3 B=2  BE=2  SCC=2  states=32/8 adm=true B=60/60 eq=true ns=7284.0/7085.7
GRAPH sequential   k=4  trip=3 B=4  BE=4  SCC=4  states=1024/16 adm=true B=100/100 eq=true ns=115790.1/15063.8
GRAPH sequential   k=6  trip=3 B=6  BE=6  SCC=6  states=>S_max/24 adm=false B=null/null eq=true ns=1806.7/3334.1
GRAPH sequential   k=8  trip=3 B=8  BE=8  SCC=8  states=>S_max/32 adm=false B=null/null eq=true ns=3083.1/11357.0
GRAPH sequential   k=12 trip=3 B=12 BE=12 SCC=12 states=>S_max/48 adm=false B=null/null eq=true ns=4719.9/7212.3
GRAPH sequential   k=16 trip=3 B=16 BE=16 SCC=16 states=>S_max/64 adm=false B=null/null eq=true ns=6157.1/9351.6
GRAPH nested-2-scc k=2  trip=3 B=3  BE=2  SCC=2  states=48/33 adm=true B=80/80 eq=true ns=16199.4/23549.4
```

The global gate refuses at `k=12` (trip 1) and `k=6` (trip 3); the SCC construction admits all 16 graphs under `S_max = 4096`. `B_hat` is identical wherever the global form is admitted, so the SCC form widens admission without changing any committed result. Latency crossover is between `k=4` and `k=6` (trip 1) and at `k=2` (trip 3).

Tarjan vs. the committed iterative `dominators()` on the same graphs:

```
DOM N=8    dominators=2126.9 ns (3828.4 cyc) tarjan=1099.3 ns (1978.7 cyc)
DOM N=32   dominators=15965.9 ns (28738.6 cyc) tarjan=3318.5 ns (5973.3 cyc)
DOM N=128  dominators=153464.4 ns (276235.9 cyc) tarjan=11436.4 ns (20585.5 cyc)
DOM N=512  dominators=2073611.9 ns (3732501.4 cyc) tarjan=44103.6 ns (79386.5 cyc)
```

At `|B| = 512`, Tarjan is ~47× faster (44.1 µs vs 2.07 ms), matching the `O(|V|+|E|)` vs `O(|B|^3)` split.

## 3. (b) Fixed-width bitwise vs. arithmetic emulation

Measured `u64` ops with bounded operands (`INT_POS <= 2^64−1`). The emulation uses only real `DIV`/`MOD`/`MUL`/`ADD` (binary digit extraction with run-time divisors `POW2[i]`), no native bitwise operator in the measured body, and was checked for exact equivalence (`BITOP_EMULATION_EQUIVALENCE=true`). §2.6 candidate encodings: binary bitwise = 8 words / 64 B (as `ADD`/`MUL`); immediate shifts = 9 words / 72 B (as `CONST`); the emulation expands to 3,072 words / 24,576 B for AND/OR/XOR (`64 × 6 × 8` words) and 8 words / 64 B for a single shift.

```
BASE MUL=1.85 ns DIV=8.20 ns MOD=8.65 ns median_4U=8.20 ns ns_per_U=2.05 (bounds 0.463/2.05 )
BITOP BAND   native=1.20 ns (2.16 cyc, 64 B, 1 U) emul=855.5 ns (1539.9 cyc, 24576 B, 418 U)
BITOP BOR    native=0.800 ns (1.44 cyc, 64 B, 1 U) emul=807.5 ns (1453.5 cyc, 24576 B, 394 U)
BITOP BXOR   native=0.750 ns (1.35 cyc, 64 B, 1 U) emul=794.0 ns (1429.2 cyc, 24576 B, 388 U)
BITOP BSHL   native=0.750 ns (1.35 cyc, 72 B, 1 U) emul=0.500 ns (0.900 cyc, 64 B, 1 U)
BITOP BSHR   native=0.750 ns (1.35 cyc, 72 B, 1 U) emul=6.50 ns (11.7 cyc, 64 B, 4 U)
BITOP_EMULATION_EQUIVALENCE=true
```

Native bitwise is ~0.75–1.2 ns (1.35–2.16 cycles) versus 794–856 ns (1,429–1,540 cycles) for the arithmetic emulation of AND/OR/XOR — roughly **three orders of magnitude** — with a 384× encoding-size reduction (64 B vs 24,576 B). Under the median-4U anchor (`1 U = 2.05 ns`) all five native ops charge `1 U`; under the fastest-op (MUL) anchor (`1 U = 0.463 ns`) they charge `2`–`3 U`. BSHR emulation is only 4 U because a single `DIV` suffices; BSHL emulation is 1 U because a single `MUL` suffices.

## 4. (c) Governed `Q(hash)` — in-IR SHA-256 over canonical bytes

Portable pure-Rust SHA-256 (QO-33), canonical byte payloads, declared algorithm tag `sha-256:<hex>`:

```
QHASH bytes=32   ns=371.7 cycles=669.0 charge=182 U ns/32B-block=371.7
QHASH bytes=64   ns=673.1 cycles=1211.5 charge=329 U ns/32B-block=336.5
QHASH bytes=128  ns=916.7 cycles=1650.0 charge=448 U ns/32B-block=229.2
QHASH bytes=256  ns=1469.2 cycles=2644.6 charge=717 U ns/32B-block=183.7
QHASH bytes=512  ns=2482.1 cycles=4467.7 charge=1211 U ns/32B-block=155.1
QHASH bytes=1024 ns=4530.0 cycles=8154.0 charge=2210 U ns/32B-block=141.6
QHASH_FIT charge(n) = 160.3 + 129.2 * ceil(n/64) U
```

A flat `4 U` charge would under-bill by 45×–550× on this host. The per-64-byte-block cost amortizes from 371.7 ns at 32 B to 141.6 ns at 1024 B, so the declared charge must be parameterized by the **declared byte bound**, never by a runtime length and never by a flat constant.

## 5. F14 cost-model recommendations

F14 (`docs/qnir_accounting.md` §3.2/§3.3/§3.5) forbids runtime quantities in a charge function and requires the ledger to bill exactly the declared charge: charges take only declared immediates, header bounds, and static type bounds. Accordingly:

1. **Bitwise.** Declare one site-stable class per opcode, independent of operand values:
   `charge(BAND) = charge(BOR) = charge(BXOR) = charge(BSHL) = charge(BSHR) = 1 U` under the published median anchor (`1 U = 2.05 ns` on this host). Because the fastest-op anchor gives `2`–`3 U`, a profile that publishes that anchor must declare `2 U` for AND/OR/XOR and `1 U` for the shift forms. `W_worst` for BOC uses the declared class value, never the 0.75 ns sample.
2. **`Q(hash)`.** Parameterize by the static byte bound `b` carried in the instruction header/type declaration:
   `charge(Q_HASH, b) = round(160.3 + 129.2 · ceil(b/64)) U` under the median anchor (equivalently `182/329/448/717/1211/2210 U` at `b = 32/64/128/256/512/1024`). A profile using the MUL anchor scales by `2.05/0.463 ≈ 4.43`. The digest must be emitted as `sha-256:<hex>`; a raw digest is not an admissible identity (the `docs/simemp.md` §4.3 migration-witnessing rule). Under F6/`QO-24`, the in-IR hash is an identity/commitment primitive, **not** a value-equality test: equality remains full canonical-encoding comparison with `qir-artifact-cmp-1` collision buckets.
3. **SCC-additive admission.** Replace the global structural product `|B|·(trip+1)^|BE|` with the sum over SCCs `Σ_i |SCC_i|·(trip+1)^|BE_i|`, computed before the DP, while keeping the same `S_max = 4096` gate. The DP itself is exit-aware per SCC and then aggregated along the condensation DAG. This is a *soundness-preserving* widening: on every admitted committed vector and every admitted synthetic graph the bound is numerically identical; on refused-by-product graphs it replaces an overflow gate with a finite admission.

## 6. Formal BOC adjudication

**Stage 1 (Tier-1 existential invariants, `docs/simemp.md` §7):** `I_Id` **PASS** — the artifact is content-addressed with a SHA-256 fingerprint and a read-only causal record; `I_Met` **PASS** — every ceiling is finite and declared (`S_max = 4096`, `C_max = 8`, trip ceilings, bounded byte bounds, bounded `U` charges); `I_Sec` **PASS (no new contract)** — the study introduces no signing capability and no PQC claim; `Q_HASH` is a content-addressing primitive, not a signature. No candidate is discarded at Stage 1.

**Stage 2 (lexicographic `LexKey = (ImplicitDeps, perm_Ĩ(W_worst, M_coupling, P_gap))`):**

| Candidate | `ImplicitDeps` | `W_worst` (declared) | `M_coupling` | `P_gap` |
|---|---:|---|---:|---|
| SCC-additive `bhat` | 0 (stdlib Tarjan + DP) | additive structural bound ≤ `S_max`; unchanged `B_hat` on admitted graphs | 0 | UNRESOLVED — one host; required profile set `R` not declared by this read-only study |
| Fixed-width bitwise | 0 (pure `u64` ops) | 1 U/op (median anchor) or 2–3 U (MUL anchor) | 0 | UNRESOLVED — single `x86_64-pc-windows-msvc` host |
| Governed `Q(hash)` | 0 (in-tree SHA-256) | `160 + 129·ceil(b/64) U` (median anchor) | 0 | UNRESOLVED — one host |

Because `P_gap` is unresolved for all three (the required profile set is not declared here, and portability cannot be inferred from one host), BOC cannot issue a final total ordering. The transferable findings are the *orderings*: `Σ` vs `Π` state growth, `O(V+E)` vs `O(B³)`, native bitwise vs arithmetic emulation, and flat-`4 U` vs byte-bounded hashing. All three are **retained measurement candidates for specification**, not canonized opcodes: no `qir-enc-4` tag or key is allocated, no source file is edited, and QnIR remains `[PENDING]`.

## 7. Step 6.3b — `OP_SATURATE` / `QO-28` priced separately

`OP_SATURATE` is **not** part of this wave. Per `docs/qexpr.md` §4.6.2 it requires, without defaults, a deterministic pinned rule schedule, `K_max`, `N_max`, `D_max`, a polynomial-time greedy extractor with hash tie-breaking, and a new versioned artifact carrying input commitments, schedule, ceilings, extractor generation, the §2.2 hashing-generation tag, and the comparator version (`QO-28`, v1.4.1 binding). It is a `Q(rsi)`-class workflow operation; `OP_BYPASS` canonical contraction and fixed-width saturating arithmetic remain distinct candidates. It is deferred to **Step 6.3b** with its own BOC evidence; this study allocates neither opcode tag `32` nor envelope key `10` to it.

## 8. Artifacts of record

Standalone crate: `%TEMP%\opencode\step6-3-bootstrap-study\` (only location of code and build output).

```
ARTIFACT json=13253 bytes sha-256:3f49ed9e1430b8cee36e73dbc106d521e6cbddfa05b58e7683bd69816d0cc671
ARTIFACT csv=4965 bytes
```

`step6_3_bootstrap_results.json` parses with 51 parity rows, 16 graph rows, 4 dominator rows, 5 bitwise rows, and 6 `Q_HASH` rows; `step6_3_bootstrap_results.csv` is the flat companion; `run_stdout.txt` is the full transcript; `step6_3_bootstrap_results.json.sha256` holds the fingerprint. Absolute constants are single-host observations, not portability claims.

## 9. Zero-drift confirmation

```
src/qnir/     test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
src/qnlangc/  test result: ok. 3 passed + 3 passed + 15 passed = 21/21; 0 failed
git status -s → ?? docs/logs/qnir/step6-3-bootstrap-trade-study.md  (only entry)
```

No commit, no push, no source edit, no spec edit, no golden edit. `docs/qnir_goldens.md` was opened read-only for the 51 Intent transcriptions and is byte-unchanged.
