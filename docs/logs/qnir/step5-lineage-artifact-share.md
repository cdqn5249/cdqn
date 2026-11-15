---
layout: default
title: Step 5 — The Lineage Artifact Share (Wave Record)
description: Both implementations emit the declared five-line lineage block and the generation-bound read-back pairs; the evidence digest agrees across implementations and modes
nav_exclude: true
---

# Step 5 — The Lineage Artifact Share (Wave Record)

Wave record of the Step-5 artifact share under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.26. **Both implementations now emit the declared five-line lineage block after their `result:` line, in normal and fixture mode alike, and the wave records cite each transcript's generation-bound read-back pair `(qir-verbalizer-1, SHA-256 of the exact transcript text)`.** The 84 committed vectors are byte-unchanged; the F27 fixture block stays byte-identical to its pre-wave recording.

## The selection

The draft.25 wave's own recorded selection is discharged: the artifact share of the declared lineage and identity semantics carries `ImplicitDeps` = 0 (every format it needs — the identity triples, the binding `L = (corpus A, implementation A, V, E, n)`, the generation rule — is declared in §6 of [`docs/qnir_commitments.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_commitments.md)), the smallest `ΔW_worst` of the admissible set (transcript additions only; no corpus, vector, or §1–§5 byte), the least coupling (the emission is external evidence metadata — the frozen encodings are untouched by construction), and it completes the discharge path of two of Step 5's four deliverables while the other two (the trace-root construction, the collision-safe comparison) each still require a declared format first.

## What was emitted

**The lineage block** (the declared rendering of §6.3's binding): five lines appended after the final `result:` line of each artifact's transcript, in normal and `--fixture F27` mode alike:

```
lineage: corpus identity (corpus, 0.3.0-draft.23, fbefaddb17903c18bad839c73f756e74f95f08967a8c25db1148d9c991d75b43)
lineage: implementation identity (implementation, 0.3.0-draft.26, <the artifact's own declared source-set digest>)
lineage: V = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1)
lineage: evidence (n = 336 records, E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72)
lineage: generations (qir-extractor-1, qir-verbalizer-1)
```

- **Corpus identity** — the corpus document's declared `version:` front-matter value and the SHA-256 over the LF-normalized repository bytes (1,066,409 bytes; the working copy's line endings are a checkout artifact, §6.2).
- **Implementation identity** — the artifact's declared version and the SHA-256 over the concatenation of its **declared source set** in path order, LF-normalized: the Rust artifact declares `src/qnir/Cargo.toml`, `src/qnir/rust-toolchain.toml`, and the seven `src/qnir/src/*.rs` modules (generated files excluded: `Cargo.lock` is generated, `target/` is build output); the Python artifact declares `src/qnir-py/qnir_ref.py`. The line's format is identical across artifacts; its content is artifact-specific by construction — the two implementations are different artifacts with different declared source sets, and §6.2's triple equality never identifies them.
- **`V`** — the first X vector's declared version tuple, uniform across the committed envelopes (both artifacts check the uniformity of each vector's own binding at the agreement points).
- **Evidence line** — `n` and `E` over the X-family execution evidence only: for each executed vector in committed document order, the run's own canonical record encodings (fourteen `[tag, value]` pairs, §2.6's F5 list) followed by the canonical receipt encoding, accumulated immediately after the successful run and **before** any agreement-point check, so a failing check never changes the evidence. Run errors contribute nothing; families E/I/R/M contribute nothing; the fixture's own injected runs contribute nothing (the fixture-mode evidence line is byte-identical to normal mode's).
- **Generations line** — both transformation identities of §6.4.

**The emission sites.** Rust: `encode_record` in `src/qnir/src/interp.rs` (the canonical record encoding, byte-matching the second implementation's `rebuild_record` — independently verified by the evidence-digest agreement below), the accumulator threaded `suite_x_exec` → `exec_vector_check`, and the block emitted in `main()`. Python: the accumulation in `run_x_family` immediately after `Machine(...).run()`, and the block in `main()`. `src/qnir/Cargo.toml` and the Python `IMPL_VERSION` constant both declare `0.3.0-draft.26`.

## The evidence (runs of record, this commit state)

All four modes **PASS** — normal 84/84 both artifacts (X execution 51/51 at the five agreement points), `--fixture F27` PASS both. The generation-bound read-back pairs (Obj 15):

| Run | `(qir-verbalizer-1, SHA-256 of the exact transcript text)` |
|---|---|
| Rust, normal | `(qir-verbalizer-1, 07e2c5332e2efe7f551e4150d0fa776161ed34d7b62d8edc61a9da193b3f8230)` |
| Python, normal | `(qir-verbalizer-1, a5b5c6f574665d5b3ded4bab669361381b14ef7ee7812bfa1a6cd157ed213784)` |
| Rust, `--fixture F27` | `(qir-verbalizer-1, cb5a635037abe2353e0a2756441d27c9416308c6ad4756324e7706a2bd37ff42)` |
| Python, `--fixture F27` | `(qir-verbalizer-1, 2a2da83833f484b761ff1a87b23372d18206815df639d5d7d9a58e3b0191757a)` |

The binding-level agreement: `E = 8acdeaa1fadd7b3b0b672d11e2ed0e67599ff7a260d7fe9dcd04f97823ffcc72` and `n = 336` are byte-identical between the two implementations and between normal and fixture mode — the cross-implementation agreement of the lineage binding under equal corpus identity and equal `V`, recorded here as evidence, never as the criterion (§4.1). The implementation identities of record: Rust `(implementation, 0.3.0-draft.26, 8345d0bfb1f6fd32781acb3f52ea429c9fb890d56396929828b026f2e4ce64c5)`, Python `(implementation, 0.3.0-draft.26, 36df5ba80f8142e67e29aed052d172fa733c42cbbe75f606cd0aa4e1bbc57549)` — digests differ by construction, per §6.2. The F27 fixture block remains byte-identical to its pre-wave recording (`ed67e6cbd242c69c` over the block's lines).

**The generation rule in action (recorded so it is not read as drift).** The transcripts changed this wave — the block is a declared output addition — so each transcript's read-back pair is a new generation-bound commitment, never compared against a pre-wave transcript digest. Symmetrically, both implementation identities changed because the emitting code changed: the implementation artifact's digest covers its own source, so an identity change here is the declared behavior of §6.2, recorded at this commit state.

## Disposition

Two of §6.5's four rows are now fully **DISCHARGED** (specification v0.3.0-draft.25, artifact v0.3.0-draft.26): normative lineage, and the versioned verbalizer/extractor identities. Two rows remain **FUTURE**: the full-transition trace root — whose construction must be declared before implementation, the §6.3 evidence digest explicitly not being it — and the collision-safe artifact comparison (the §1.4 F6 bucket resolution; F24's deferral stands). Step 5 remains **open** until its register empties; the promotion criterion is unchanged and unmet.

The next BOC increment, selected under the same standing key (`LexKey(c) = (ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)))`, π = Efficiency, Modularity, Portability): **the trace-root declaration** — the format declared before any implementation, per the draft's ordering discipline — followed by the collision bucket, the register's last future row. No push is made until the maintainer approves.

Declared non-edits: the corpus (84 vectors byte-unchanged; digest `fbefaddb17903c18bad839c73f756e74f95f08967a8c25db1148d9c991d75b43` over 1,066,409 LF bytes), `docs/qnir_semantics.md`, `docs/qnir_accounting.md`, `docs/qnir_conformance.md`, `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/boc.md`, and the AiReview bodies (append-only — the v0.1.10 note records this share).
