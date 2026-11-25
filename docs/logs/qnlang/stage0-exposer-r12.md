---
layout: default
title: Stage 0 Exposer — qir-verbalizer-1 Read-Back and R12' Certification Interlock
description: The explicit Checker -> Exposer -> Compiler pipeline, deterministic full-program exposition for all seven Stage 0 programs, and the Obj 15 read-back certification gate that refuses mismatched emission
nav_exclude: true
---

# Stage 0 `Exposer` — `qir-verbalizer-1` Read-Back and `R12'` Certification Interlock

Wave record for `src/qnlangc/` under `docs/qnlang.md` v0.1.0-draft.3. F19-separated: this wave touches only `src/qnlangc/` and this QnLang log. `src/qnir/`, `src/qnir-py/`, `docs/qnir_goldens.md`, `docs/qnlang.md`, and the five QnIR fixtures are unchanged. No `Q(0)_H` material is generated, read, or signed, and no licensing gate is introduced (`docs/qnlang_custody.md` §4).

## The three-part pipeline

```text
--emit-qir <file> [-o <out.cbor>] [--certify-readback <sha256>]

  1. Checker    parse -> G1–G14 + E_NAMESPACE_COLLISION
  2. Exposer    qir-verbalizer-1 full-program read-back (Obj 15)
  3. Interlock  optional exact-digest confirmation of the read-back
  4. Compiler   R11' lowering -> qir-enc-4 CBOR (committed only here)
```

The `Exposer` is pure: no clock, no RNG, no environment, no absolute paths, no `HashMap` iteration. Its body is rendered from `Program`/source order and hashed with the in-tree `SHA-256`; the `readback_sha256` footer is outside the hashed body, so it is never self-referential. `Obj 15` binds the read-back as `(qir-verbalizer-1, SHA-256 of the exact text bytes)`; the generation string is `qnlang-expose-1`.

## Invocations

```text
qnlangc --check <file> [--name-map] [--gate-list]
qnlangc --expose <file>
qnlangc --emit-qir <file> [-o <out.cbor>] [--certify-readback <sha256>]
cargo test --offline
```

`--expose` prints the full exposition and its footer:

```text
readback_lines: 28
readback_bytes: 986
readback_sha256: sha-256:bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89
```

`--emit-qir` always runs `Checker` and `Exposer` before the `Compiler` commits:

- **Uncertified (default).** Prints `EXPOSE lines=… bytes=… sha256=sha-256:…`, then `LOWER …`, then `EPISTEMIC MODAL_HYPOTHESIS (uncertified dev emit)`, and writes the CBOR.
- **Certified.** `--certify-readback <sha256>` compares the supplied digest (bare hex or `sha-256:<hex>`, case-insensitive) to the exact exposition digest. A match prints `EPISTEMIC READBACK_CERTIFIED`; the CBOR is byte-identical to the uncertified emit.
- **Mismatch.** Refuses emission with `E_CERTIFICATION_MISMATCH` on both `R12'` channels, prints `EPISTEMIC MODAL_HYPOTHESIS (uncertified; emission refused)`, returns exit 1, and writes **zero** CBOR bytes.

## What the exposition covers

`[header]` `Q(dcc)`, `Q(kappa)`, `Q(domain)`, `Q(mode)` and the epistemic tag `MODAL_HYPOTHESIS (uncertified; co-signature pending)`; the `Checker` interlock marker `H(T | Contract) = 0`. `[declarations]` types with bounds/units, anchors, metrics, variants, blankets. `[containers]` affine containers with `cap` and `status` (`dropped` / `consumed` / `live`). `[control]` `Q(pattern)`, `Q(workflow)`, `Q(falsify)`, entry statements, and every `Q(fold)`/`Q(load)`/`Q(loop)` trip plus call arity. `[budget]` each declared `![budget]` against the source-level `b_hat` the budget gate used, and the emitted entry `B_hat` admission surrogate separately.

## Tests (verbatim)

`cargo test --offline` in `src/qnlangc/` — **21/21** green:

```text
running 3 tests
test exposition_is_deterministic_and_bounded_for_all_seven ... ok
test wrong_certification_refuses_emission_with_zero_bytes ... ok
test exact_certification_emits_identical_cbor ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 3 tests
test declared_budget_exceedance_is_loud ... ok
test emits_deterministic_cbor_on_declared_subset ... ok
test all_five_worked_programs_emit_cbor ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 15 tests
test gate_list_is_exact ... ok
test neg_equals_outside_binary ... ok
test neg_naked_numeric ... ok
test neg_qs_identifier_in_qchem ... ok
test neg_affine_missing_drop ... ok
test neg_bound_overflow ... ok
test neg_namespace_collision ... ok
test neg_affine_double_consume ... ok
test neg_undeclared_bound ... ok
test q_falsify_is_now_a_top_decl ... ok
test worked_qbio ... ok
test worked_qm ... ok
test worked_qchem ... ok
test worked_qphy ... ok
test worked_qs_qlog ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The three new tests assert: (i) identical body and digest across independent parses for all seven programs, path-independent digests, distinct digests, and the required exposition surfaces; (ii) a wrong certification refuses emission with zero bytes and no `LOWER` line; (iii) the exact certification emits CBOR byte-identical to both the uncertified emit and in-process `lower::lower`.

## Seven read-backs (`qnlang-expose-1`)

| Fixture | Lines | Bytes | `readback_sha256` |
|---|---:|---:|---|
| minimal | 23 | 633 | `f9a6fc5ebe391281db64c83911fd8e5c0b86b6f4e8fbe475ab3258cc38468a7b` |
| qm | 32 | 918 | `3441066f72b1cd76c3265e7a090fc89160365c054875ce318316fa3a604079bf` |
| qphy | 26 | 765 | `cf00525436891a8866045a40576d7beedf831a6d3fcb117eb230ce3ba57494a2` |
| qbio | 27 | 791 | `e91736b66757ba454464738c13e31d1eecd2f3430dfbe74e6bb80079b989be38` |
| qchem | 40 | 1451 | `e94b2decf6af952b0ccc3ba88b43e811ff2b4ea25b25863523b7dc14cb33b105` |
| qs_qlog | 37 | 1430 | `574f376f62e5deb48f746cd57d6492ddc9abf1e2876cd4e483cb6117ea6bf381` |
| call_worked | 28 | 986 | `bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89` |

## CBOR invariance (unchanged Stage 0 bytes)

| Fixture | CBOR bytes | SHA-256[:16] | `B_hat` | `Work(E)` | ledger |
|---|---:|---|---:|---:|---|
| minimal | 391 | `f0c2074817e58e28` | 40 | 40 | 40/40/0 |
| qm | 751 | `40ca53e63cc1adb4` | 60 | 60 | 60/60/0 |
| qphy | 278 | `b58477753896c921` | 36 | 36 | 36/36/0 |
| qbio | 278 | `e1993819a6626c45` | 36 | 36 | 36/36/0 |
| qchem | 1775 | `92214a13eaed8fe2` | 144 | 144 | 144/144/0 |
| qs_qlog | 1318 | `f00ee50ae12ea5d5` | 32 | 32 | 32/32/0 |
| call_worked | 856 | `69aa70a483a28beb` | 78 | 78 | 78/78/0 |

`B_hat` and `Work(E)` were read back with the unchanged QnIR artifacts (`qnir --eval-cbor`); certification never perturbs these bytes.

## Certification interlock (verbatim, `call_worked`)

Wrong digest:

```text
EXPOSE lines=28 bytes=986 sha256=sha-256:bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89
CHANNEL_A (E_CERTIFICATION_MISMATCH, 0:0, qnlangc/E_CERTIFICATION_MISMATCH, sha-256:bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89, 0000000000000000000000000000000000000000000000000000000000000000, re-run --expose and certify the exact readback_sha256; no CBOR is written, receipt:qnlangc/E_CERTIFICATION_MISMATCH/0:0)
CHANNEL_B qir-verbalizer-1: E_CERTIFICATION_MISMATCH at tests\fixtures\call_worked.ql:0:0 — read-back certification does not match the Exposer digest; expected sha-256:bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89, got 0000000000000000000000000000000000000000000000000000000000000000; remedy re-run --expose and certify the exact readback_sha256; no CBOR is written
EPISTEMIC MODAL_HYPOTHESIS (uncertified; emission refused)
```

Exit 1; the target file does not exist. Exact digest:

```text
EXPOSE lines=28 bytes=986 sha256=sha-256:bef1debab67986d3584b4bb4d033461f2dcdcc8ae391886ce0120fd1055eaf89
BUDGET step declared=64 U b_hat=50 U
LOWER tests\fixtures\call_worked.ql ok=true bytes=856 sha256=69aa70a483a28beb words=33 opcodes=11 blocks=1 callees=1 ext=0 b_hat=Some(78) budgets=[("step", 64)]
EPISTEMIC READBACK_CERTIFIED
WROTE …\call_worked.cbor (856 bytes)
```

## Fresh `--eval-cbor` differential verification (certified emit)

All seven fixtures were re-emitted with `--emit-qir <fixture.ql> -o <out.cbor> --certify-readback <exact readback_sha256>` into `%TEMP%\opencode\t15-*.cbor`. Every emit printed `EPISTEMIC READBACK_CERTIFIED` and reproduced the CBOR sizes/digests in the table above exactly:

```
minimal     CERTIFIED bytes=391  sha=f0c2074817e58e28 exists=True
qm          CERTIFIED bytes=751  sha=40ca53e63cc1adb4 exists=True
qphy        CERTIFIED bytes=278  sha=b58477753896c921 exists=True
qbio        CERTIFIED bytes=278  sha=e1993819a6626c45 exists=True
qchem       CERTIFIED bytes=1775 sha=92214a13eaed8fe2 exists=True
qs_qlog     CERTIFIED bytes=1318 sha=f00ee50ae12ea5d5 exists=True
call_worked CERTIFIED bytes=856  sha=69aa70a483a28beb exists=True
```

Each file was then executed with `qnir.exe --eval-cbor` and with the Python reference artifact. On this Windows host `python3` is an unresolved app-alias, so the identical command was run with `python` (Python 3.12.10). Full stdout was compared line-for-line:

```
minimal     outcome=SUCCESS B_hat=40  Work=40  ledger=40/40/0 P1_eq=True trace_eq=True full_eq=True
qm          outcome=SUCCESS B_hat=60  Work=60  ledger=60/60/0 P1_eq=True trace_eq=True full_eq=True
qphy        outcome=SUCCESS B_hat=36  Work=36  ledger=36/36/0 P1_eq=True trace_eq=True full_eq=True
qbio        outcome=SUCCESS B_hat=36  Work=36  ledger=36/36/0 P1_eq=True trace_eq=True full_eq=True
qchem       outcome=SUCCESS B_hat=144 Work=144 ledger=144/144/0 P1_eq=True trace_eq=True full_eq=True
qs_qlog     outcome=SUCCESS B_hat=32  Work=32  ledger=32/32/0 P1_eq=True trace_eq=True full_eq=True
call_worked outcome=SUCCESS B_hat=78  Work=78  ledger=78/78/0 P1_eq=True trace_eq=True full_eq=True
```

Exact roots from the Rust run (each byte-identical in the Python run):

```
minimal     P1=c5e1b268fe924da2e90adde5f5e9c2a935fbabfae1d6bc551e60c9739971e37f trace=0bdc2dcc58aeb6e01e8a27049a1c9f14a6c3787bd63e97152b4add50fc75a4d0
qm          P1=cdbcc72bc7421331b39e4af36d068d1ba616583b226298155c5d49b3e431c1e4 trace=36f194785d4f07df216f5628bae4dda8ec6f125e1d01dc95f2a148beea682f65
qphy        P1=6711d59fe83827563a5a40e7a72ee9473c3cae8ce91ff022014adbb514a288e9 trace=1b7c6258398b423c275d98b96511cc55e0c29585704c35a4cfbc623222c19e79
qbio        P1=6711d59fe83827563a5a40e7a72ee9473c3cae8ce91ff022014adbb514a288e9 trace=45089785aefbc1f1d8745cdf357eb828fe34bacaf59c6cdca912177395a7f2b4
qchem       P1=83140b814a83d42646cbb2de5e146ae9c339dbdd3c211bf5da2dfb493229a28b trace=21a15d93a4d6b1becfe9aec9ace1d7cd16e4b9a5e54f39c48198cb117a154643
qs_qlog     P1=aad352464c395e86198ce603ee13992ab89125bc2c3ebbe67449730baa746361 trace=d24f8402224e2ff0a8ab7559bf5d8e6ade8986da1c47fafb7c3d3ff9a943fe3a
call_worked P1=eee81d0212dac448d2bef7de2fad80315243ef8f0a01e8e99a03170223a2f7d5 trace=2726089ebb9699603c9e3f73d45b1f995492833f2b45f32e10c7393a1be363ca
```

`B_hat >= Work(E)` (equality on all seven), `reserved == B_hat == charged + refunded`, and `outcome = SUCCESS` hold throughout. This is a QnLang-stage verification record only: no QnIR source, golden, or fixture byte was changed.

## Non-goals

No real `Q(0)_H` key, leaf, seed, signature, or migration receipt is produced or consumed. No licensing gate is added. The Exposer is a read-back surface, not a compiler attestation: security still rests on the verifier re-running `R11'` and `qir-artifact-cmp-1` on committed bytes. QnLang remains `[PENDING]`.
