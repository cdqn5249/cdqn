---
layout: default
title: Step 5, Share 3 — The Trace-Root Artifact Share (Wave Record)
description: Both implementations compute and emit the declared qir-trace-root-1 roots — the header pins line, the run scope, and each vector scope — with all 52 published lines byte-identical across implementations and modes
nav_exclude: true
---

# Step 5, Share 3 — The Trace-Root Artifact Share (Wave Record)

Wave record of Step 5's third share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.28. **Both implementations now compute and emit the declared `qir-trace-root-1` roots at every run** — the six known-answer pins checked in the header, the `run` scope, and each `vector` scope in committed document order — discharging the artifact share of the construction declared at v0.3.0-draft.27 (§6.6 of [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md)). The corpus and the 84 committed vectors are byte-unchanged.

## The selection

The draft.27 wave's recorded selection is discharged: the trace-root artifact share carries `ImplicitDeps` = 0 (the construction, its scopes, its rendering, its pins, and its generation identity are all declared in §6.6 — nothing is left for the implementation to infer), the smallest `ΔW_worst` of the admissible set (the emitting code in both implementations plus the transcript additions and the records; no corpus, vector, or §1–§5 byte), the least coupling (the emission is external evidence metadata — the frozen encodings are untouched by construction, and the roots bind the records the evidence digest already accumulates), and it completes the discharge path of the third of Step 5's four deliverables, leaving only the collision bucket.

## What was emitted

Per run of either artifact, in normal and `--fixture F27` mode alike:

1. **The header pins line**, after the `declared seed:` line:

   ```
   qir-trace-root-1 known-answer pins: 6/6 reproduced (commitments §6.6)
   ```

   The six declared pins are recomputed from the construction at every run (toy byte strings `A`–`E`, the §6.6 table); a divergent pin is a run failure, and the pins are never counted in the declared seed. Rust: `trace_pins_ok()` in `src/qnir/src/main.rs` over `trace_root` in `src/qnir/src/interp.rs`; Python: `trace_pins_ok()`/`TRACE_ROOT_PINS` over `trace_root` in `src/qnir-py/qnir_ref.py`.

2. **The publication lines**, after the §6.3 five-line lineage block — the `run` scope first, then one line per executed `vector` scope in committed document order, exactly the rendering §6.6 fixed — **52 published lines per run**:

   ```
   lineage: trace root (qir-trace-root-1, scope = run, n = 336, root = d42b53c860c15592280c5979be3423f15d259b2ed05aeb185d6ef76ca4e42ab6)
   lineage: trace root (qir-trace-root-1, scope = X1, n = 10, root = 5b72ccdf84ef8f4104fe79d4ec2da0c5373db986ee0fb022081ef78b1b64da6c)
   ...
   lineage: trace root (qir-trace-root-1, scope = X51, n = 2, root = 6de6a60a4cc2a62a14c8723251fd2b85d9dd54340c3f0545cf2f0911403c085d)
   ```

   The vector scopes' `n` values sum to 336 — the run scope's `n`. The roots bind the trace's records only (bytes, order, multiplicity), never the receipts: the record collection happens immediately after each successful run and before any agreement check, the same encodings the §6.3 evidence digest accumulates, so a failing check never changes a published root, and a pre-run failure publishes no scope. The fixture's injected runs contribute nothing (the fixture-mode publication is byte-identical to normal mode's). Emission sites: Rust — the record collection in `exec_vector_check`/`suite_x_exec`, the publication in `main()`; Python — the mirror in `run_x_family`/`main()` of `qnir_ref.py`. `src/qnir/Cargo.toml` and the Python `IMPL_VERSION` constant both declare `0.3.0-draft.28`.

## The evidence (runs of record, this commit state)

All four modes **PASS** — normal 84/84 both artifacts (X execution 51/51 at the five agreement points), `--fixture F27` PASS both; the pins line reads 6/6 in all four transcripts. The binding-level agreement:

- The **52 published trace lines are byte-identical** between the two implementations and between normal and fixture mode — the cross-implementation agreement of every published root under equal corpus identity and equal `V`, recorded here as evidence, never as the criterion (§4.1).
- `E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72` with `n = 336` is unchanged from draft.26 — the evidence digest and the trace root are two commitments over the same record bytes at different structures (§6.6), and only the second is new this wave.
- The F27 fixture block remains byte-identical to its pre-wave recording (`ed67e6cbd242c69c` over the block's lines).
- The corpus digest is unchanged (`fbefaddb17903c18bad839c73f756e74f95f08967a8c25db1148d9c991d75b43` over 1,066,409 LF bytes).

The generation-bound read-back pairs (Obj 15; the transcripts changed this wave — a declared output addition — so each pair is a new generation commitment, never compared against a pre-wave digest):

| Run | `(qir-verbalizer-1, SHA-256 of the exact transcript text)` |
|---|---|
| Rust, normal | `(qir-verbalizer-1, c108f51aa5603abb3581cbafa553000b3f1dc78f781f7ad1d89f28409a73cd7c)` |
| Python, normal | `(qir-verbalizer-1, 3e2bec617fc43d016102b024531a6103c6c9d24db286493b9a76f67264ec7603)` |
| Rust, `--fixture F27` | `(qir-verbalizer-1, b619bb3386332fbbfbb819ad316a20c258bfa478f2d276b461cc81798ad9f3b6)` |
| Python, `--fixture F27` | `(qir-verbalizer-1, 004ded3559561ba9217550737d4dd627b94a5913e1b7533b2341386a58406a7e)` |

The implementation identities of record (changed with the emitting code, as §6.2 declares): Rust `(implementation, 0.3.0-draft.28, 0feec679058a310f023d7892de5c7e31a12bd641a736109c70f2b3e6be9afe99)`, Python `(implementation, 0.3.0-draft.28, 01d9318da2f384d3b9426752fc94f5dafa473d7a5aa53bd122b45228a8264103)` — digests differ by construction, per §6.2.

## Disposition

Three of §6.5's four rows are now fully **DISCHARGED**: normative lineage (spec v0.3.0-draft.25; artifact v0.3.0-draft.26), the versioned verbalizer/extractor identities (spec v0.3.0-draft.25; artifact v0.3.0-draft.26), and the full-transition trace roots (spec v0.3.0-draft.27; artifact v0.3.0-draft.28). One row remains **FUTURE**: the collision-safe artifact comparison (the §1.4 F6 bucket resolution; F24's deferral stands). Step 5 remains **open** until its register empties; the promotion criterion is unchanged and unmet.

The next BOC increment, selected under the same standing key (`LexKey(c) = (ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)))`, π = Efficiency, Modularity, Portability): **the collision-safe artifact comparison** — its construction must be declared before its artifact/test share, per the draft's ordering discipline. No push is made until the maintainer approves.

Declared non-edits: the corpus (84 vectors byte-unchanged), `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.12 note records this share).
