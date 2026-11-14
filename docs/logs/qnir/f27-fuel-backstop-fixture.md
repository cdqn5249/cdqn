---
layout: default
title: F27 — The Fuel-Backstop Fixture (Fixture-Semantics Wave Record)
description: The F27 discharge — declared test-only validator-defect injection semantics, the Fixture Register, and the fixture-mode runs of both artifacts
nav_exclude: true
---

# F27 — The Fuel-Backstop Fixture (Fixture-Semantics Wave Record)

Wave record of Finding F27 of [`docs/AiReviews/AiReviewQnIRv0.4.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.4.md) under the [BOC policy](https://github.com/cdqn5249/cdqn/blob/main/docs/boc.md) of `docs/simemp.md` §7. Decision wave: v0.3.0-draft.23. **The fuel-backstop-on-validator-defect fixture is discharged as declared test-only fault injection — not a conforming golden.** The 84 committed vectors are byte-unchanged; no execution rule, no vector, and no committed expected byte of the corpus changes.

## The finding

F27 records that the corpus's Suite Register lists a fuel-backstop-on-validator-defect vector as outstanding, but the shape of such a test, its expected verdict, and its checking protocol at the validator boundary are undeclared. The review's F27 analysis records the resolution problem: a backstop stop on an admitted Intent evidences a validator defect, never a satisfied admission rule (F16, v0.3.0-draft.7), so the expected outcome cannot be a normal execution golden — committing the defective execution's record and receipt bytes would normalize it. The required disposition: "reproducible test evidence, an independently derived expected result, and an updated finding disposition", with the scope boundary that no existing golden vector is modified.

## The BOC selection

The increment was selected under the standing rubric-mode key `LexKey(c) = (ImplicitDeps(c), perm_π(ΔW_worst(c), M_coupling(c), P_coverage(c)))` (π = Efficiency, Modularity, Portability, v0.3.0-draft.7's standing permutation) over the remaining candidates (the `boc.md` §12 item was already discharged — verified in the committed v0.4.1 bytes, AiReview v0.1.6 — and Step 5 is Tier-1 UNRESOLVED pending Step-4 acceptance). The fixture wave carries `ImplicitDeps` = 0: the declared semantics are a reading of already-committed rules (§3.4's refusal shape and backstop, §3.3's conservation identities and rejection-record settlement, §2.5's terminalization itemization), the fixture program's exact bound is hand-derived under the committed surrogate that F26 made exact, and the declared expectation is fixed from those rules before any fixture run. It closes the Suite Register's sole remaining bullet and the traceability register's last non-future row.

## The declared fixture semantics (conformance §4.4, v0.3.0-draft.23)

The fixture is a **test-only validator-defect injection, not a conforming golden**: committed as the corpus's Fixture Register (a section outside the X family, not counted in the declared seed), it overrides the validator's computed admission bound with a declared `B_injected` — the under-computed surrogate, the defect class a false admission arises from — and commits no record or receipt bytes. Its three declared parts:

1. **Shape** — a fixture program `P_f` whose exact admission bound `B_hat(P_f)` is hand-derived; a declared budget `W_budget` the exact surrogate refuses (`B_hat(P_f)` > `W_budget`); a declared injection `B_injected` with `B_injected` < `B_hat(P_f)` and `B_injected` ≤ `W_budget`.
2. **Declared runs** — the **control run** under the exact bound (the committed refusal shape: the single rejection transition, zero postings, the escrow never opened) and the **injected run** (admitted; the escrow opens at `B_injected`; the fuel backstop fires at the first transition the under-reserve cannot cover).
3. **Expected verdict, two components** — the **validator-defect report** (the backstop stopped an admitted Intent — a false admission, localized to the injected bound's declared shortfall) and the **containment verdict** (the declared stop site and step, the outcome tag, the token itemization at the exit, and the closed ledger: conservation, remaining budget, `charged` ≤ `reserved`).

Both artifacts implement a declared fixture mode (`--fixture F27`) whose fixture blocks must agree **byte-for-byte**; the fixture commits no record or receipt bytes and normalizes no defective execution.

## The committed fixture (corpus Fixture Register, v0.3.0-draft.23)

Fixture `F27` — `fixture-fuel-backstop-validator-defect`. Program `P_f`: one straight-line block of four `CONST` instructions (immediates 0–3, each bound `≤` its own immediacy) and a `HALT`; declared initial status pool `[2]` (one `STATUS_NEG_EVEN_ROOT` — the conformance birth path, X3's precedent); `D` = no entries; `V` = (v0.3.0-draft.5, qir-enc-1, qir-prof-golden-1). Hand derivation of the exact bound: the gateway 20 U plus five 4 U transitions on the single bounded path (four `CONST`, then `HALT`) = **`B_hat(P_f)` = 40 U**; the surrogate is exact since F26, so `B(P_f)` = 40 U. Declared parameters: `W_budget` = 36 U (40 > 36 — the exact surrogate refuses); `B_injected` = 32 U (8 U short of exact; 32 ≤ 36 — admitted: the false admission). Under the injection the escrow opens at 32 U; the gateway (20 U) plus the first three transitions' charges (12 U) charge exactly 32 U; the fourth transition's 4 U charge exceeds the outstanding escrow (0 U), so the backstop fires at step 4 at PC (0, 3).

Declared expectation (fixed by hand from §2.5, §3.3, and §3.4 before any fixture run):

- control run: **REFUSED** at admission — `EXPRESSION_TOO_COMPLEX` (tag 3); the single rejection transition at PC (0, 0); zero postings; the escrow never opened; conservation 0 = 0 + 0 + 0.
- injected run: **ADMITTED** — the backstop fires at step 4 at PC (0, 3), `EXPRESSION_TOO_COMPLEX` (tag 3); 3 completed transition records and 1 backstop rejection record; charged 32 U, reserved 32 U, refunded 0 U; conservation 32 = 32 + 0 + 0; remaining budget 36 − 32 = 4 U; charged ≤ reserved with equality — no overbilling.
- token itemization at the backstop exit: 2 tokens terminalized and itemized — the declared pool token (birth 0, `STATUS_NEG_EVEN_ROOT`) and the fatal token (birth 4 at (0, 3), `EXPRESSION_TOO_COMPLEX`); the receipt's selected token nil (DR2's fatal selection is the DOMAIN_ERROR exit's).

The fixture envelope's canonical bytes (385 bytes, instruction stream 39 words) were produced through the committed normative assembler's one-to-one mapping (§4.2) via a temporary uncommitted emission path that was removed from the artifact before the wave's commit (`git diff` clean); verification of the bytes: the committed `decode_envelope` of both artifacts accepts them, the source listing regenerates byte-identically, and the computed `B_hat` is the declared 40 U.

## The implementations' fixture modes

- **Reference artifact** (`src/qnir/src/interp.rs`, `src/qnir/src/main.rs`): `Machine::new` takes the injected bound (`None` on every conformance path — the fixture mode is the only caller that sets it); the run entry is refactored into `run_vector_with(envelope, w_budget, b_inject, profile, dep_values)` with `run_vector` delegating unchanged; a new `run_fixture` summarizes the semantic content (outcome, record count, stop site and step, the accumulators, the receipt itemization's (birth, type) pairs, and the nil selection) from the committed receipt encoding. The fixture suite parses the Fixture Register's declared parameters and expectation, re-encodes the committed source form to the committed envelope bytes (§4.2's assembly share), checks the computed bound against the declared one, checks both runs against the declared expectation, and renders the fixture block.
- **Second implementation** (`src/qnir-py/qnir_ref.py`): the same `b_inject` parameter on `Machine` (`None` default), a `run_fixture` runner with the same register parsing, the same assembly and listing-regeneration share, the same declared-expectation checks, and the same fixture-block rendering; the transcript's stdout is pinned to LF line endings so the fixture blocks compare byte-for-byte raw.

Every check is mechanical — no fixture number is produced by the implementations; each is parsed from the register's declared text and asserted against the runs.

## Runs of record

Both artifacts were rebuilt and rerun against the edited corpus — the Rust reference interpreter (rustc 1.100.0, x86_64-pc-windows-msvc, release profile, standard library only; cargo 1.100.0) and the Python second implementation (CPython 3.12.10, standard library only).

- **Normal mode (the conformance suite).** Both artifacts pass **84/84** — suite E 11/11, I 3/3, R 4/4, M 15/15, X 51/51 at §4.1's five agreement points — byte-identical in content to their pre-wave outputs: the fixture register is outside every family scan, and no execution path of the normal suite reads it. Transcript digests (16-hex prefixes, SHA-256): reference `6135b1c60b7d69b0` (28,850 bytes), second implementation `27ce440c2ee5e620` (10,928 bytes).
- **Fixture mode (`--fixture F27`).** Both artifacts report `fixture F27: PASS` with every declared number exact: the control run refused (tag 3, zero postings, 0 = 0 + 0 + 0), the injected run admitted with the backstop firing at step 4 at PC (0, 3) (tag 3, 3 completed + 1 rejection records, 32/32/0, conservation 32 = 32 + 0 + 0, remaining 4 U, charged ≤ reserved with equality), the itemization's two (birth, type) pairs exact, and the nil selection exact. Fixture-block agreement: the 10 rendered fixture lines are **byte-identical** between the two artifacts' transcripts (fixture-block digest `ed67e6cbd242c69c`). Transcript digests: reference `ba5d4e2c3decd018` (30,000 bytes), second implementation `9fc02e0a9843a596` (12,078 bytes).
- **Reproducibility.** Same-binary (Rust) and same-source (Python) reruns are byte-identical in both modes; each fixture-mode transcript minus its fixture block equals its normal-mode transcript exactly.

The corpus digest of record is `fbefaddb17903c18` over 1,066,409 LF/repository bytes (the committed file's CRLF working-copy rendering digests `247753d0ef8c5073` over 1,088,362 bytes; the repository blob is the LF byte sequence). The 84 committed vectors are byte-unchanged — the corpus diff removes exactly the version line and the replaced Suite Register bullet, adds the Fixture Register, and touches no vector section.

## Disposition

F27 is **CLOSED** at v0.3.0-draft.23. The Suite Register's fuel-backstop bullet is discharged by citation to this fixture; the traceability register's Budget-boundaries row is COVERED with no FUTURE half remaining. The remaining Step-4 item is the **acceptance reassessment** itself (AiReview v0.1.6's step 5): the Step-4 row's status is re-assessed against the now-complete registered suite — 84 committed vectors plus the discharged fixture — in a follow-up record. Steps 5 and 6 remain deferred.

Declared non-edits: `docs/index.md` (QnIR [PENDING] stands), the glossary, `docs/_data/navigation.yml`, `docs/boc.md`, and the AiReview transcripts (append-only — the v0.1.7 note records this wave). `QO-34` identity 12 remains the wave's checklist anchor. The promotion criterion is unchanged and unmet.
