---
layout: default
title: Step 5, Share 5 — The Collision Comparison's Artifact Share (Wave Record)
description: Both implementations run the declared qir-artifact-cmp-1 comparison at every run — the pins line, the self-test, and the bucket state line — with the bucket empty over the committed corpus; Step 5 is closed
nav_exclude: true
---

# Step 5, Share 5 — The Collision Comparison's Artifact Share (Wave Record)

Wave record of Step 5's fifth and closing share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.30. **Both implementations now run the declared `qir-artifact-cmp-1` comparison at every run** — the four known-answer pins checked in the header, the four-verdict self-test, and the bucket state line — discharging the artifact share of the construction declared at v0.3.0-draft.29 (§6.7 of [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md)). **Step 5 is closed: its register is empty.** The corpus and the 84 committed vectors are byte-unchanged.

## The selection

The draft.29 wave's recorded next increment is discharged: the collision comparison's artifact share carries `ImplicitDeps` = 0 (the comparison unit, the closed projection, the verdicts, the genuine-collision condition, the quarantine semantics, the bucket register shape, the publication rendering, the generation identity, and the pins are all declared in §6.7), the smallest `ΔW_worst` of the admissible set (the comparison core in both implementations plus the transcript additions and the records; no corpus, vector, or §1–§5 byte), the least coupling (the emission is external evidence metadata — the frozen encodings are untouched by construction, and the comparison binds complete artifacts without changing any encoding), and it empties the register, closing Step 5.

## What was emitted

Per run of either artifact, in normal and `--fixture F27` mode alike:

1. **The header pins line**, after the trace-root pins line:

   ```
   qir-artifact-cmp-1 known-answer pins: 4/4 reproduced (commitments §6.7)
   ```

   The four declared pins are recomputed from the decision core at every run; a divergent pin is a run failure, and the pins are never counted in the declared seed.

2. **The self-test line**, immediately after:

   ```
   qir-artifact-cmp-1 self-test: 4/4 verdicts reproduced (commitments §6.7)
   ```

   Four toy complete artifacts are fed to the comparison core, checking the quarantine verdicts (same, collision, different, different — the collision case supplying equal triples synthetically, since no SHA-256 collision is constructible).

3. **The bucket state line**, after the trace-root publication lines:

   ```
   lineage: collision bucket (qir-artifact-cmp-1, entries = 0)
   ```

   The bucket counts collision pairs among the retained complete artifacts (corpus, implementation, envelopes, records, receipts) of the run; with the committed corpus it is empty, since no two retained artifacts share an identity triple over differing bytes. Emission sites: Rust — `cmp_artifacts`, `CompleteArtifact`, and `collision_entries` in `src/qnir/src/interp.rs`, the pins and self-test in `src/qnir/src/main.rs`, and the bucket state line published in `main()`; Python — the mirror in `src/qnir-py/qnir_ref.py`. `src/qnir/Cargo.toml` and the Python `IMPL_VERSION` constant both declare `0.3.0-draft.30`.

## The evidence (runs of record, this commit state)

All four modes **PASS** — normal 84/84 both artifacts (X execution 51/51 at the five agreement points), `--fixture F27` PASS both; the pins lines read 6/6 and 4/4 and the self-test 4/4 in all four transcripts. The binding-level agreement:

- The **52 published trace lines and the bucket line (`entries = 0`) are byte-identical** between the two implementations and between normal and fixture mode — recorded as evidence, never the criterion (§4.1).
- `E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72` with `n = 336` is unchanged; the fixture block remains byte-identical (`ed67e6cbd242c69c`); the corpus digest is unchanged (`fbefaddb17903c18bad839c73f756e74f95f08967a8c25db1148d9c991d75b43` over 1,066,409 LF bytes).

The generation-bound read-back pairs (Obj 15; the transcripts changed this wave — a declared output addition — so each pair is a new generation-bound commitment, never compared against a pre-wave digest):

| Run | `(qir-verbalizer-1, SHA-256 of the exact transcript text)` |
|---|---|
| Rust, normal | `(qir-verbalizer-1, 0b6b7a995d6df041d29729748f4363c03c4c04a0399a677c31c65d0c58aa8db0)` |
| Python, normal | `(qir-verbalizer-1, 80702ccb3ae8911dfadc42fd379bf30797f9b84ee6828442d79a4f5b53186659)` |
| Rust, `--fixture F27` | `(qir-verbalizer-1, abb974b63a37e250f12189748aad06a65e4ef6efd12041db4f58cef6040db116)` |
| Python, `--fixture F27` | `(qir-verbalizer-1, 3797cf62b63c692147eb57993d1911da25599fdb3e69f3e4bb8e5a17da78ed2d)` |

The implementation identities of record (changed with the emitting code, as §6.2 declares): Rust `(implementation, 0.3.0-draft.30, <its declared source-set digest>)`, Python `(implementation, 0.3.0-draft.30, <its declared source-set digest>)` — digests differ by construction, per §6.2.

## Disposition

**Step 5 is closed.** All four deliverables are now fully DISCHARGED: normative lineage (specification v0.3.0-draft.25; artifact v0.3.0-draft.26), the versioned verbalizer/extractor identities (specification v0.3.0-draft.25; artifact v0.3.0-draft.26), the full-transition trace roots (specification v0.3.0-draft.27; artifact v0.3.0-draft.28), and the collision-safe artifact comparison (specification v0.3.0-draft.29; artifact v0.3.0-draft.30). §6.5's register is empty. The promotion criterion is unchanged and unmet — Step 5's closure does not fire it, and `R11'`, `R12'`, `R19`, and the `O(polylog)` statement remain conditional until the program of record's own gate is met; no claim of this draft is styled a theorem.

This is the point the maintainer named for drafting the AIReview proposal for the Qn universe with a QnIR design emphasis: Step 5 is closed with an empty register and no self-declared open obligation in the design proper, and Step 6 (container BOC, call trade study, liveness histograms, choreographic refinement, class-specific AIRs, measured Realization A/B break-even) is the priced extension agenda, deliberately deferred — a review round before Step 6 adjudicates extension priorities rather than pre-empting them.

Declared non-edits: the corpus (84 vectors byte-unchanged), `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.14 note records this share). No push is made until the maintainer approves.
