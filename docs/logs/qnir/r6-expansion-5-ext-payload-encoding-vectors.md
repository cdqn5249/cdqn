---
layout: default
title: R6 Expansion Increment 5 — the `EXT`-Payload Encoding Vectors (Run Log)
description: Run transcripts of both QnIR conformance artifacts over the expanded golden corpus (83 vectors), no defect class registered
nav_exclude: true
---

# R6 Expansion Increment 5 — the `EXT`-Payload Encoding Vectors (Run Log)

Artifact wave: v0.3.0-draft.18 (R6 expansion increment 5) — the expanded golden corpus [`docs/qnir_goldens.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_goldens.md) (committed at 0.3.0-draft.18; corpus sha-256 prefix `b70e4bac391d4437`, 1,045,499 bytes), carrying the seed's 34 independently reviewed vectors and the 42 expansion vectors of increments 1–4 byte-unchanged plus the 7 hand-derived encoding vectors of this increment — R1–R4 in the new Family R (`EXT` side-table record encodings) and M13–M15 appended to Family M — and the declared payload reading P14 (the Suite Register's sixth bullet — the encoding vectors for `EXT` record payloads whose reading types the corpus declares with its encoding version). Both artifacts are checked against the corpus and never generate or edit it (§4.2).

## What ran

Four steps, in order:

1. the **anchored derivation** — the derivation instrument of record re-derived every committed execution vector byte-exact from its declared source form and declared parameters (all 50, seed and increments 1–4 alike) before any expansion byte was derived — the golden-first order applied to expansions (`ANCHORING: all 50 committed execution vectors re-derived byte-exact from the declared source forms and parameters`);
2. the **conformance runs of record** — both artifacts against the expanded corpus (transcripts below); no divergence was registered on the first recorded runs of either artifact: both implementations reproduced every committed expected outcome, so §4.1's adjudication rule was never triggered;
3. the **reruns** — the same binary (reference interpreter) and the same source (second implementation) executed twice on the same corpus, byte-identical, reproducibility evidence only (§4.2).

No defect class was registered in this wave. The increment adds no execution rule to either artifact; the artifact share is the declared reading check and the new Family R suite surface (below).

## The declared metadata repair — the increment-4 log's corpus digest

Registered from the maintainer's git-clone verification of the committed wave (the maintainer's check instruction for this step): the R6 expansion increment 4 run log's declared corpus digest named the pre-version-bump corpus bytes — `d486156008580aba`, the digest of the corpus file carrying the draft.16 version line — while the committed draft.17 corpus bytes (the file the log cites, committed at 0.3.0-draft.17) hash to `f94d26ae9e3f0827` (1,034,150 bytes in both cases: the version-line edit is same-length). The declared digest is re-issued to the committed bytes' digest in that log. The runs' verdicts are unaffected: the version line is front matter, the vector bytes are identical in both files, and both artifacts' parsing touches no front-matter byte — the transcripts' `committed at` fields record the corpus version each artifact read, and every committed check parses vector content only. The log of record for this wave declares the draft.18 corpus digest `b70e4bac391d4437` over the committed bytes.

## Toolchains

Recorded per the independence and reproducibility requirements of §4.1/§4.3:

- reference interpreter: Rust, standard library only (QO-33), crate `src/qnir` v0.3.0-draft.18, toolchain pinned in the crate (channel 1.99.1); run under rustc 1.99.1 (58cf8f9c3 2026-10-13), x86_64, Linux; release profile; no network access at run time
- second implementation: CPython 3.12.16, x86_64, Linux; standard library only (modules `hashlib`, `json`, `platform`, `re`, `sys`) per `QO-35`'s registered adjudication; no build step, no dependencies, no network access at run time

## How to reproduce

```
src/qnir/target/release/qnir docs/qnir_goldens.md
python3 src/qnir-py/qnir_ref.py docs/qnir_goldens.md
```

Each artifact parses the declared vectors from the committed bytes and prints one deterministic transcript (no wall-clock, no locale, no randomization). Exit code 0 only when every committed check passes.

## The artifact share of this wave

The increment's artifact share is the declared reading check and the new suite surface; no execution rule changed and no harness surface changed:

- the **declared payload reading P14** in the envelope decode path of both implementations — each `EXT` side-table record's payload reads as no content (empty) or as exactly one complete §2.6 value encoding with no trailing bytes, the value grammar's own errors surfacing at the payload extent, and the record index words are unique per the committed duplicate-key rule (`src/qnir/src/insn.rs` `check_ext_payload`/`check_ext_table`, wired into `decode_envelope`; `src/qnir-py/qnir_ref.py` `check_ext_payload`/`check_ext_table`, wired into `decode_envelope`);
- the **Family R suite surface** — the new family slice in both artifacts: R1–R3 build the declared record through the committed CBOR and value grammars and match the committed canonical bytes, R4 decodes the committed envelope bytes and matches the declared side-table record (`src/qnir/src/main.rs` `suite_r`/`parse_r_input`; `src/qnir-py/qnir_ref.py` `run_family_r`);
- the **Family M `EXT` surface** — M13–M15 ride the same P14 check through the declared record-or-table kind (the Rust `m_shape`'s `extrec` arm; the Python's declared `M_KINDS` entries);
- the **version bumps** — the crate at 0.3.0-draft.18 (`Cargo.toml`, `Cargo.lock`); the corpus, the conformance companion, and the program of record at 0.3.0-draft.18.

## The conformance runs (both artifacts, first recorded runs, expanded corpus)

Verdict: all 83 committed vectors reproduce — suite E 11/11, suite I 3/3, and suite R 4/4 canonical bytes (agreement point 1), suite M 15/15 committed verdicts exact (agreement point 2), all 46 committed envelope byte blocks decoded with their source-grammar listings regenerated from the decoded bytes, and 50/50 execution vectors reproducing their complete committed expected outcomes: the full fourteen-field record sequence (agreement point 3), the terminal receipt's canonical bytes (agreement point 4), and the reconstructed ledger with every §3.3 conservation identity and the terminal outcome (agreement point 5). Every computed `B_hat` equals its committed declared bound.

The increment's vectors (every other vector's line is in the full transcript):

```
  R1 enc-ext-record-empty: match (3 bytes)
  R2 enc-ext-record-int: match (28 bytes)
  R3 enc-ext-record-tuple: match (68 bytes)
  R4 enc-ext-envelope-populated: match (154 bytes)
  M13 ext-payload-trailing-bytes: verdict match — malformed — trailing bytes after the record's value payload
  M14 ext-payload-leading-zero: verdict match — malformed — leading zero magnitude word (non-canonical)
  M15 ext-record-duplicate-index: verdict match — malformed — duplicate side-table record index
suite R (EXT side-table record encodings): PASS — 4/4 committed vectors re-encode byte-exactly
suite M (malformed surfaces): PASS — 15/15 committed verdicts reproduced
suite X execution (reference interpreter): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
```

```
family R (EXT side-table record encodings): PASS — 4/4 canonical bytes reproduced (agreement point 1)
family M (malformed surfaces): PASS — 15/15 committed verdicts reproduced (agreement point 2)
suite X execution (second implementation): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
```

Suite verdicts of the runs of record:

```
suite E (value encodings): PASS — 11/11 committed vectors re-encode byte-exactly
suite I (instruction encodings): PASS — 3/3 committed vectors re-encode byte-exactly
suite R (EXT side-table record encodings): PASS — 4/4 committed vectors re-encode byte-exactly
suite M (malformed surfaces): PASS — 15/15 committed verdicts reproduced
suite X (executions): assembly PASS — 49/49 committed source forms re-encode byte-exactly; validation PASS — 50/8 admitted or refused per the declared bounds
suite X execution (reference interpreter): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules, the executions matching the full committed expected outcomes; the artifact never generates or edits goldens (§4.2)
```

```
family E (value encodings): PASS — 11/11 canonical bytes reproduced (agreement point 1)
family I (instruction encodings): PASS — 3/3 canonical bytes reproduced (agreement point 1)
family R (EXT side-table record encodings): PASS — 4/4 canonical bytes reproduced (agreement point 1)
family M (malformed surfaces): PASS — 15/15 committed verdicts reproduced (agreement point 2)
suite X execution (second implementation): PASS — 50/50 committed expected outcomes reproduced at the five agreement points of §4.1
result: PASS — every committed vector reproduces under the committed rules; the second implementation never generates or edits goldens (§4.2)
```

The reruns (the same binary, the same source, twice each): byte-identical transcripts — reproducibility evidence only, never independent reproduction and never conformance (§4.2).
