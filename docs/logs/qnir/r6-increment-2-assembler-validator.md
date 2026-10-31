---
layout: default
title: R6 Increment 2 — The Assembler and Validator (Run Log)
description: Run transcripts of the QnIR conformance artifact src/qnir over the committed golden corpus
nav_exclude: true
---

# R6 Increment 2 — The Assembler and Validator (Run Log)

Artifact: `src/qnir/` (v0.3.0-draft.11, R6 increment 2) — the normative assembler and validator of conformance §4.2. Corpus: `docs/qnir_goldens.md`, the committed golden corpus (§4.4), consumed as repository bytes; the artifact never generates or edits it.

## What ran

Three runs of one release binary. Run 1 executed against the corpus as committed at v0.3.0-draft.10 (130,833 bytes); runs 2 and 3 executed against the same corpus after the declared M4 repair committed in this wave (v0.3.0-draft.11, 131,122 bytes). Runs 2 and 3 are the same binary executed twice — the §4.2 same-binary rerun, reproducibility evidence only.

## Toolchain and build

Pinned per the registered toolchain record (QO-33: compiler, target triple, flags, dependency lock status):

- rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1 (5f94df478 2026-08-27)
- target x86_64-unknown-linux-gnu, release profile (`opt-level = 2`), `--offline`
- dependencies: none (standard library only; `Cargo.lock` present, empty of external packages)
- the crate pins the channel in `rust-toolchain.toml` (`1.99.1`)

Build command, from `src/qnir/`: `cargo build --release --offline`. The build is warning-clean.

## How to reproduce

```
cargo build --release --offline
./target/release/qnir <path to docs/qnir_goldens.md>
```

The tool reads the corpus file given on the command line, parses the declared vectors from the committed bytes, and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## Run 1 — against the corpus as committed at v0.3.0-draft.10

Verdict: 33/34; one failing check. Transcript (paths are the run environment's):

```
suite E (value encodings): PASS — 11/11 committed vectors re-encode byte-exactly
suite I (instruction encodings): PASS — 3/3 committed vectors re-encode byte-exactly
suite M (malformed surfaces): FAIL — 11/12 committed verdicts reproduced
  FAIL M4 value-non-normalized-zoom: declared verdict "malformed — non-normalized zoom encoding" but tool verdict "malformed — count exceeds the enclosing artifact's remaining length"
suite X (executions): assembly PASS — 7/7 committed source forms re-encode byte-exactly; validation PASS — 8/8 admitted or refused per the declared bounds
result: FAIL — 1 failing check(s)
```

The finding, in the terms of §4.1's adjudication rule (a disagreement is adjudicated against the golden result and the specification, with the vector id and the disagreeing point named): vector **M4**. The committed §2.6 value grammar encodes a zoom's components as complete `INT_POS`/`INT_NEG` encodings — type-tag word, count word, magnitude words — as the corpus's own canonical vectors E10 and E11 commit. M4's committed draft.10 bytes carried the pair (6, −2) in a count-less component form; read under the committed grammar, the mantissa's count word declared six magnitude words where two remained, so the structural count surface (M5's verdict) fired before the normalization check could run, and the vector's declared verdict was unreachable from its own bytes. The specification adjudicates against the bytes: the draft.10 form was a corpus defect.

## The declared repair (committed in this wave)

M4's bytes are re-issued as the full §2.6 encoding of (6, −2) — tag `ZOOM`, mantissa `INT_POS` count 1 carrying 6, exponent `INT_NEG` count 1 carrying 2 — seven words, whose normalization N(6, −2) = (3, −1) differs from the encoded pair, so the declared verdict "malformed — non-normalized zoom encoding" becomes the one the bytes reach. The repair is noted in the vector's cites line and in the draft.11 wave record of `docs/qnir.md`; every other vector is byte-unchanged.

## Runs 2 and 3 — the repaired corpus (same binary, twice)

Verdict: 34/34, both runs. Transcript of run 2:

```
suite E (value encodings): PASS — 11/11 committed vectors re-encode byte-exactly
suite I (instruction encodings): PASS — 3/3 committed vectors re-encode byte-exactly
suite M (malformed surfaces): PASS — 12/12 committed verdicts reproduced
  M4 value-non-normalized-zoom: verdict match — malformed — non-normalized zoom encoding
suite X (executions): assembly PASS — 7/7 committed source forms re-encode byte-exactly; validation PASS — 8/8 admitted or refused per the declared bounds
result: PASS — every committed vector reproduces under the committed rules; the artifact never generates or edits goldens (§4.2), and the transition relation is not executed here (§4.3's increment follows)
```

Per-vector highlights (full lines in the transcript): the seven committed program envelopes (X1, X3–X8) re-assemble byte-exactly from their committed source forms with matching instruction streams; the validator computes `B_hat` at 100, 100, 32, 40, 36, 32, 60, 32 U across X1–X8 — equal to each vector's declared admission bound; X2 refuses at `W_budget` 60 < `B_hat` 100 with the typed EXPRESSION_TOO_COMPLEX token, matching its expected terminal outcome tag 3.

Reproducibility: run 3 is the same binary on the same corpus; its transcript is byte-identical to run 2's (transcript digests, 12-hex prefixes — run 1 `564fb52f993f`, runs 2 and 3 `46ef93c24e3d`). This is reproducibility evidence only (§4.2); independence and conformance ride §4.1's protocol over the full suite as the later increments land.

## Scope declarations

- The artifact validates and assembles; it does not execute the transition relation — records, receipts, state roots, and ledgers are the reference interpreter's agreement points (§4.3, next increment).
- For the X vectors' assembly share, the version tuple comes from each vector's declared parameters line, the dependency list is derived from the program's cited load addresses over the corpus's declared P9 content address, and the declared initial status pool rides the vector's committed envelope (key 8 — vector data, not program text). The program encoding is the check target; the envelope compare is byte-exact.
- The validator implements the §2.1 static legality surfaces and the §3.2 dispatch-time operand bounds the seed exercises (dense static value ids, control targets, declared dependency addresses, closed tag vocabularies, the dangling side-table surface); the full dominance-binding analysis of §2.3's static-binding postulate is an expansion obligation, not exercised by any committed vector.
- The seed's expansion obligations (the suite register of `docs/qnir_goldens.md`) are untouched by this increment.
