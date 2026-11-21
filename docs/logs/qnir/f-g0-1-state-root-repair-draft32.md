---
layout: default
title: F-G0-1 — The P1 State-Root Repair at v0.3.0-draft.32 (Obj 15 Read-Back Pairs)
description: The committed 6cc1ef1 wave — qir-enc-2 P1 = SHA-256(P2) 32-byte state roots, the six Obj 15 transcript digests, both fixture blocks, and the QO-17 draft.32 interleaved A/B delta
nav_exclude: true
---

# F-G0-1 — The P1 State-Root Repair at v0.3.0-draft.32 (Obj 15 Read-Back Pairs)

Wave record of finding `F-G0-1` under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.32. **The wave is committed at `6cc1ef1`** and repairs the P1 state root of record: the record fields that carry the pre/post configuration now carry the **SHA-256 digest of the canonical P2 configuration** — exactly 32 bytes — instead of configuration bytes in another width. The corpus, the opcode set, the accounting model, and every committed expected byte are otherwise unchanged; QnIR remains `[PENDING]` and no Step-6 opcode is canonized by this record.

## The selection

F-G0-1 records that `qir-enc-2` declares P1 to be a 32-byte state root over the canonical configuration encoding P2, while the draft.31 artifacts still wrote the configuration encoding itself into record fields 1/7/8. That mismatch is an artifact-share agreement defect: the Obj 15 read-back pair `(qir-verbalizer-1, transcript digest)` is bound to the exact field widths, so a transcript that renders those fields as non-32-byte strings cannot be the transcript of the declared encoding.

The selected repair is the smallest one that closes the mismatch without touching the frozen bytes:

1. rename the interpreter's `config_now` to `config_p2`, making the P2 pre-image explicit;
2. add `state_root(pc) = SHA-256(config_p2(pc))`, returning a 32-byte digest;
3. substitute that call at the five record-building sites (fields 1, 7, and 8 across pre/post configurations).

No envelope hex byte, no golden expected byte, no opcode, no U charge, and no template surface changed. The Python second implementation's companion fix pins redirected stdout to UTF-8 with LF newlines (`sys.stdout.reconfigure(encoding="utf-8", newline="\n")`) so its Obj 15 read-back pair is byte-comparable on Windows. Under **Axiom 13** the update is a new versioned artifact; the draft.31 wordings are preserved in their historical logs, not mutated in place.

## What was emitted

**The commit.** `6cc1ef1` — 10 files, +3,362/−10,956 — of which the semantic content is the three-point repair above plus the corpus's regenerated canonical annotations. The corpus at this state is **`docs/qnir_goldens.md`: 659,880 LF bytes, 14,477 LF lines, 0 CR bytes, SHA-256 `246bc01c3337afb8ba930e3738650e05f1aa412b4d8d495c9b27f05c6bcc1d92`**. `cargo test` passes **4/4** at this state. No glossary, navigation, portal index, or `src/` file is changed by this log.

**The binding-level facts of record**, identical in all six transcripts (Rust implementation identity `9b784dc003ef9e97f37fedfbfa2330b33d59bc146cfa3c0f4f290abed8feb4ab`; Python implementation identity `5b8d600be66f25d7d99c632f3f574e21527f737485b145e71fad28e430aee613`):

- `declared seed: 84 vectors`; suite X execution `51/51` at §4.1's five agreement points; both artifacts PASS.
- header pins `6/6` (trace roots), `4/4` (comparison pins), `4/4` (comparison self-test); 52 published trace-root lines.
- `V = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2)`.
- evidence `n = 336 records`, **`E = 5a438424c7988dc2a5f0e771c495b8fa786da8a506e006aa0b90f8c1ca9915da`**.
- **`qir-trace-root-1`, `scope = run`, `n = 336`, `root = 369ada2bde4d65f0d9e7706a940586b84331982cbf9ed94ebc5e5f9b01fd5866`**.
- fixture blocks: `F27` fuel backstop **`6f6bd8f5ea344b31`**, `F6` collision comparison **`5041b3741ade3b9a`** — each byte-identical between the two artifacts and unchanged from draft.31; both fixtures remain test-only surfaces, never conforming goldens.

**The six Obj 15 read-back pairs.** All six runs exit 0. Transcripts are LF-normalized before hashing; each is valid UTF-8 and already LF-only.

| Run | Bytes (LF) | Lines | `(qir-verbalizer-1, SHA-256 of the LF transcript)` |
|---|---:|---:|---|
| Rust, normal | 36,472 | 260 | `0c8d3530592b7b728e709bd8923ea75c94df502cc31817c4acf7a99f9efe1c5b` |
| Rust, `--fixture F27` | 37,622 | 270 | `f962ad17d7a49c531dbae2b796fb08da4e4724809325b21ceba19d4063f7a653` |
| Rust, `--fixture F6` | 37,090 | 269 | `3fd4f53d0aa1c94676959f8e52fd73720dce363dd96c1fd88b43e2d6235c4bac` |
| Python, normal | 18,550 | 123 | `8e54c028887f833732b0e66a55acf4fced5a4969e4e431fc6f5227239a7083c9` |
| Python, `--fixture F27` | 19,700 | 133 | `29c309a4673db01ccf152350b436bab995857c345952ecea5784b63766173979` |
| Python, `--fixture F6` | 19,168 | 132 | `3b995f7e62210e751e4fe4eb151a21cbf3055a9ee9082af94d4210f82953f3e3` |

**The `QO-17` Part 1 delta on draft.32.** The measurement is an interleaved same-session A/B (draft.31 binary → draft.32 → draft.31 → draft.32, two runs each) because the host's between-session drift is larger than the P1 effect. Over the 49 vectors with `Work(E) > 0`:

| Quantity | draft.31 (mean of two) | draft.32 (mean of two) | Delta |
|---|---:|---:|---:|
| aggregate median `ns/U` | 2,426.0 | 2,540.0 | **+4.7 %** (paired-vector median ratio 1.044) |
| aggregate min `ns/U` | 1,580.7 | 1,656.2 | +4.8 % |
| `X32` (U=72) | 4,434.7 ns/U | 4,107.6 ns/U | −7.4 % |
| `X2` (admission refusal, U=0) | 31,750 ns | 33,450 ns | **+1,700 ns** |
| `X50` (escrow-never-opened, U=0) | 26,950 ns | 33,850 ns | +6,900 ns (single-run bound +10.9 %) |

The published draft.31 baseline is median 2,525.0 / max 4,994.4 ns/U; the saved draft.32 JSON records median 2,503.1 (−0.9 %) and a distribution max of 6,606.2 whose source is a low-`U` stall, not a structural regression. Restricted to high-work vectors (`U ≥ 68`) the maxima are draft.31 5,600 / 6,333 versus draft.32 4,791 / 5,275. Same-binary repeats moved individual vectors by as much as 2.5×, so the median-level +4.7 % is the defensible delta; the per-record digest adds a fixed 1–15 µs while the smaller record fields (32-byte roots rather than full configurations) return part of it through `encode_record` and `trace_root`. Full per-vector figures and the harness protocol are in the companion harness artifacts (`container_boc_results.json`, SHA-256 `abed3d5d05788ed7427ebb4df090cbf4265809ee306e72b8e8caa9e73605f36c`, and `run_stdout.txt`), whose container trade study is recorded in `docs/logs/qnir/step6-1-container-boc-study.md`.

## The evidence (runs of record, this commit state)

Both artifacts were rebuilt and rerun against the committed corpus at `6cc1ef1`: the Rust reference interpreter (`rustc 1.99.1`, `x86_64-pc-windows-msvc`, release profile, standard library only) and the Python second implementation (CPython 3.12.10, AMD64, Windows; standard library only). Invocation is `qnir <path to docs/qnir_goldens.md> [--fixture F27|--fixture F6]` and its Python counterpart; all six exit 0, every normal-mode result is PASS, and both fixture verdict lines are PASS. The corpus identity was recomputed from the committed bytes before the runs: 659,880 bytes, 0 CR, digest `246bc01c…bcc1d92` as declared above. The two fixture-block digests were recomputed from the transcripts by the §4.4 block rule and agree across implementations. A same-binary and same-source rerun reproduced all six transcript digests byte-identically (reproducibility evidence only, never independent reproduction, §4.2).

The `QO-17` measurement harness is a standalone crate in `C:\Users\dd_an\AppData\Local\Temp\opencode\container-boc\`, compiled with `rustc 1.99.1` `--release --offline` (LTO, one codegen unit), with 2 warm-up and 9 measured trials. It copies the committed modules read-only; it never writes into the repository. Its Part A fidelity gate is the `qir-trace-root-1` run root above, recomputed from 336 records with **0** root-width violations at every repetition. `git status --porcelain` was empty at the start and at the end of all runs.

## Disposition

`F-G0-1` is **CLOSED** at `6cc1ef1` in v0.3.0-draft.32. The P1 root width now agrees with `qir-enc-2` at the artifact boundary, the six Obj 15 read-back pairs are pinned above, and the QO-17 delta is recorded as a measurement, not as an amendment to the accounting model. This wave does **not** promote QnIR (`[PENDING]` stands), canonize any container opcode, discharge `QO-19`, or seal any ladder selection: the Step-6.1 container evidence is recorded separately in `docs/logs/qnir/step6-1-container-boc-study.md`, and the `qir-enc-3` opcode contract remains a future wave. `Q1` and `Q3` keep their residency contract pending; `Q6` remains `QUEUED` — this log introduces no new term include.

Declared non-edits: `docs/qnir*.md`, `docs/index.md`, `docs/_data/*`, `src/`, `docs/qnir_goldens.md`, the glossary, and all historical wave records. No push is made until the maintainer approves.
