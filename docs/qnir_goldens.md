---
layout: default
title: QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1)
description: Hand-derived conformance vectors — expected bytes and outcomes fixed from the normative rules of the QnIR draft before any code
version: 0.3.0-draft.32
updated: 2026-11-19
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_goldens.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_goldens.html
status: Working Draft under the Loop-2 Repair Sequence and the Loop-3 Repair Program (R1–R6) — Not Promoted to the Canon
terms_used:
  - boc-policy
  - qnir
  - no-implicit-rule
  - compute-unit-u
  - receipt
---

# QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1)

| Field | Specification |
|---|---|
| **Document Title** | QnIR Goldens — The Hand-Derived Golden Corpus of the QnIR v0.3 Working Draft (Artifact Share, Increment 1) |
| **Version** | 0.3.0-draft.32 |
| **Last Updated** | 2026-11-14 (Bao Loc, Vietnam) |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Working Draft under the Loop-2 Repair Sequence and the Loop-3 Repair Program (R1–R6) — Not Promoted to the Canon |

*Christophe Duy Quang Nguyen note:* This file is the committed golden corpus of the QnIR v0.3 working draft. At its initial publication (v0.3.0-draft.10), the goldens preceded all implementation artifacts; the assembler, validator, reference interpreter, and second implementation were added only in the subsequent declared order. The corpus remains hand-derived and implementation-independent: implementations check it but never generate or edit its expected results. Every expected byte and outcome below is derived from the normative rules of §1–§3 and §4 and cites the rule that produces it. At v0.3.0-draft.14 the corpus hosts its first registered expansion (R6 expansion increment 1, the `QO-22`/F12 terminalization-combination completion): the independently reviewed seed vectors are retained unchanged, and the expansion vectors are hand-derived under the same discipline after the four-artifact order completed, before the artifact runs that check them. At v0.3.0-draft.15 the corpus hosted its second registered expansion (R6 expansion increment 2, the `CLOAD`-hit and multi-block fallback completion), at v0.3.0-draft.16 its third (R6 expansion increment 3, the `TUPLE`/`FIELD`/zoom-`CMP`/`EMIT`-witness completion), at v0.3.0-draft.17 its fourth (R6 expansion increment 4, the per-class ledger trace publications and the budget-boundary per-combination completion), at v0.3.0-draft.18 its fifth (R6 expansion increment 5, the `EXT`-payload encoding vectors), and at v0.3.0-draft.20 its sixth (R6 expansion increment 6, the `CLOAD` fallback trip-ceiling regression vector); the independently reviewed seed vectors remain byte-unchanged through every expansion. Nothing herein redefines, amends, or overrides any canonical specification.

## Scope and Derivation Discipline

This corpus began as the seed suite of §4.4: its initial vectors were derived from the normative rules of §1–§3 and §4, reviewed independently of both implementations, and committed before any implementation existed — the golden-first order (F17/F18), discharged here under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7 as artifact-share increment 1 of the {% include term.html id="qnir" text="QnIR" %} draft. Under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline, every vector cites the committed rule that produces each expected value; where the committed text leaves a rendering choice open, the choice is declared in the parameter block below and binds the vectors exactly as a version component of `V` (§2.1) — a change to any of them is a new encoding version. Charges are measured in the {% include term.html id="compute-unit-u" text="Abstract Compute Unit U" %}; the terminal {% include term.html id="receipt" text="receipt" %} of every execution vector is a committed artifact of §2.5/§2.6.

**The corpus binds 84 vectors: 11 value encodings (family E), 3 instruction encodings (family I), 4 `EXT` side-table record encodings (family R), 15 malformed surfaces (family M), 51 executions (family X — the 8 seed vectors of increment 1, the 30 expansion vectors of R6 expansion increment 1, v0.3.0-draft.14, the 4 expansion vectors of R6 expansion increment 2, v0.3.0-draft.15, the 7 expansion vectors of R6 expansion increment 3, v0.3.0-draft.16, the 1 expansion vector of R6 expansion increment 4, v0.3.0-draft.17, the 7 encoding vectors of R6 expansion increment 5, v0.3.0-draft.18 — R1–R4 in family R and M13–M15 in family M — and the 1 execution vector of R6 expansion increment 6, v0.3.0-draft.20).** Each vector is committed data, citable by repository bytes; equal vectors are byte-equal vectors (§4.4).

## Expansion Increment 1 — the QO-22/F12 Combination Completion (v0.3.0-draft.14)

This expansion is R6 expansion increment 1 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.14), discharging the Suite Register's first bullet — the remaining `QO-22`/F12 terminalization combinations of conformance §4.4's closure — over the seed surface only: no new opcode execution, no populated dependency store, no witness receipts. Its vectors are X9–X38 below, appended after the seed's X1–X8 and derived after the declared four-artifact order completed (F18), from the normative rules of §2.3/§2.5/§2.6 and §3 through the anchored derivation instrument of record (a third codebase, sharing no code with either implementation and consulting neither; over the committed seed it re-derives every seed byte byte-exact before any expansion byte is derived), and checked by both implementations only after these bytes were fixed — the golden-first order applied to expansions. The seed's independently reviewed vectors are retained byte-unchanged.

**The declared readings** (declared here under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline where the committed text leaves the point open; each cites its rule, and each binds the expansion vectors as a version component of `V`):

- **DR1 — terminalization at every exit (§2.5 T case 3).** T case (3) declares itemization *at every exit*: every outstanding token terminalizes and appears in the terminal receipt with its tier, type, birth site, and disposition. At a terminalizing birth or fuel-backstop exit the outstanding tokens therefore terminalize exactly as at a `HALT` exit — the receipt itemization shows the state word 3 (P6) for each, and the final transition record's status field carries the transition's own events followed by the terminalize events in the pool's canonical identifier order (the committed `HALT`-exit shape of the seed's X3/X8 records, applied uniformly). The seed's committed fatal-exit records (X2, X5, X7) are silent on the point — none declares an outstanding token at its exit — so the reading is fixed here, by declaration, and both implementations were conformance-repaired to it in the same wave (the adjudication registered in the wave record).
- **DR2 — the selected token (§2.5 T cases 1–3; §2.6 receipt key 3).** The receipt's selected token is the *outcome-determining* token: at a `HALT` exit, the earliest-birth outstanding Tier S token that T case (2) names (the seed's committed shape, unchanged); at a terminalizing-birth exit, the fatal birthed token whose outcome T case (1) declares. The X5 precedent (the only seed vector with a selected token) does not disambiguate — its fatal token is its only candidate — so the reading is fixed here, by declaration, and both implementations were conformance-repaired to it in the same wave.

**The grid disposition.** The F12 closure declares a vector for every combination of {outstanding Tier S token class: `STATUS_NEG_EVEN_ROOT`, `EXPRESSION_TOO_COMPLEX`, `STATUS_DOMAIN_ERROR`} × {promoted, not promoted} × {fallback success, fallback repeated miss, fallback typed rejection} × {exit event: `HALT`, terminalizing birth, fuel backstop} — 54 cells, all accounted for:

- **30 cells as vectors (X9–X38).** The `HALT` and terminalizing-birth exit events over the three fallback dispositions, the three pool-declared Tier S classes, and the promotion pair. The promotion/no-promotion pair rides the declared initial status pool (§2.5 — the conformance birth path); a vector with a promoted `EXPRESSION_TOO_COMPLEX` or `STATUS_DOMAIN_ERROR` pool token exercises the `PROMOTE` row's no-token-effect branch (none outstanding: no token effect, the frame advances). The residue rule of §4.4 — a success-after-recovery outcome with an outstanding Tier O token, the outcome unaffected and the token itemized — is realized by X16 (and, for the terminalizing-birth exits, the outstanding Tier O token is itemized alike). The cells are listed in the vector register below.
- **6 cells declared unreachable (no vector can exist).** {any class} × {promoted, not promoted} × {fallback typed rejection} × {`HALT`}: the fallback's typed rejection is fatal — the machine cannot proceed past a rejected instruction (§2.5 failure behavior) — so no `HALT` exit exists after a fallback typed rejection. The closure's "at least one vector exists" is vacuously satisfied by the declared unreachability; the disposition is declared, not silent.
- **18 cells owned by the registered fuel-backstop fixture (Suite Register bullet 4).** {any class} × {promoted, not promoted} × {any fallback disposition} × {fuel backstop}: on a sound admission the fuel backstop is provably unfirable (§3.4 — the actual path is one of the bounded paths over which the maximum was taken), and per F16 an execution the backstop stops on an admitted Intent evidences a validator defect, never a satisfied admission rule — a conformance golden cannot commit a backstop stop as its expected outcome without embedding the very defect F16 excludes. These cells are therefore not reachable by goldens of this increment; they complete when the registered `fuel-backstop-on-validator-defect` fixture vector of Suite Register bullet 4 lands with its own declared fixture semantics.

## The Expansion Vector Register (X9–X38)

Every expansion vector declares `W_budget` = 100 U, `D` = no entries, the declared initial status pool of its class axis, and `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) under the declared profile `qir-prof-golden-2`; every program is a CLOAD-miss program over the empty store (the seed's single declared content address, P9), so every declared fallback disposition is exercised exactly.

| Vectors | Class axis | Promotion | Fallback disposition | Exit event | Outcome surface |
|---|---|---|---|---|---|
| X9 | `STATUS_NEG_EVEN_ROOT` | not promoted | fallback success | `HALT` | `NO_SOLUTION`, the pool token selected |
| X10 | `STATUS_NEG_EVEN_ROOT` | not promoted | fallback success | terminalizing birth | `EXPRESSION_TOO_COMPLEX` (the birthed token's outcome), nil selected |
| X11 | `STATUS_NEG_EVEN_ROOT` | not promoted | fallback repeated miss | `HALT` | `NO_SOLUTION`, the residual Tier O miss token itemized |
| X12 | `STATUS_NEG_EVEN_ROOT` | not promoted | fallback repeated miss | terminalizing birth | `EXPRESSION_TOO_COMPLEX`, the residual miss token itemized |
| X13 | `STATUS_NEG_EVEN_ROOT` | not promoted | fallback typed rejection | terminalizing birth | `DOMAIN_ERROR` (the fatal birth's outcome), the fatal token selected (DR2) |
| X14 | `STATUS_NEG_EVEN_ROOT` | promoted | fallback success | `HALT` | `SUCCESS` (the promotion consumed the token) |
| X15 | `STATUS_NEG_EVEN_ROOT` | promoted | fallback success | terminalizing birth | `EXPRESSION_TOO_COMPLEX`, no outstanding token at the exit |
| X16 | `STATUS_NEG_EVEN_ROOT` | promoted | fallback repeated miss | `HALT` | `SUCCESS` with the outstanding Tier O miss token itemized — the §4.4 residue-rule vector |
| X17 | `STATUS_NEG_EVEN_ROOT` | promoted | fallback repeated miss | terminalizing birth | `EXPRESSION_TOO_COMPLEX`, the residual miss token itemized |
| X18 | `STATUS_NEG_EVEN_ROOT` | promoted | fallback typed rejection | terminalizing birth | `DOMAIN_ERROR`, the fatal token selected (DR2) |
| X19–X23 | `EXPRESSION_TOO_COMPLEX` | not promoted | all three | `HALT` / terminalizing birth | the class's own verdict at a `HALT` exit; the birthed token's outcome at a terminalizing birth |
| X24–X28 | `EXPRESSION_TOO_COMPLEX` | promoted | all three | `HALT` / terminalizing birth | as X19–X23 — the `PROMOTE` row's no-token-effect branch (none outstanding: no token effect) |
| X29–X33 | `STATUS_DOMAIN_ERROR` | not promoted | all three | `HALT` / terminalizing birth | the class's own verdict at a `HALT` exit (`DOMAIN_ERROR`, the pool token selected); the birthed token's outcome at a terminalizing birth |
| X34–X38 | `STATUS_DOMAIN_ERROR` | promoted | all three | `HALT` / terminalizing birth | as X29–X33 — the `PROMOTE` row's no-token-effect branch |

## Expansion Increment 2 — the CLOAD-Hit and Multi-Block Fallback Completion (v0.3.0-draft.15)

This expansion is R6 expansion increment 2 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.15), discharging the Suite Register's second bullet — the `CLOAD` hit vectors over a populated dependency store with the bound value checked against its declared bound, and the multi-block fallback completion vector fixing the consumption site beyond a HALT-terminated fallback block. Its vectors are X39–X42 below, appended after X38 and derived after the committed four-artifact order and expansion increment 1 (F18 applied to expansions), from the normative rules of §2.1–§2.6 and §3 through the same anchored derivation instrument of record (a third codebase, sharing no code with either implementation and consulting neither; over the committed corpus it re-derives every committed execution vector byte-exact — all 38 — before any expansion byte was derived), and checked by both implementations only after these bytes were fixed. The seed's and increment 1's independently derived vectors are retained byte-unchanged. The increment adds no execution rule to either artifact: the hit path with its pre-dispatch bound check, the store binding, and the multi-block fallback routing are committed §2.3/§2.4/§2.5 rules both implementations already execute — the increment's artifact share is the declared dependency table's harness plumbing (the declared rendering parameter P12 below), not a new execution surface.

**The declared reading** (declared here under the No Implicit Rule discipline where the committed text leaves the point open; it cites its rule and binds the expansion vectors as a version component of `V`):

- **DR3 — the fallback completion site (§2.5 tier rule).** The committed rule consumes the miss token "only when that fallback entry completes successfully — the fallback block completes, births no token, and ends in no typed rejection". The fallback block's completion is its final instruction's own transition: the consume event rides that transition's record whether the block ends in `HALT` (the seed's X6 shape) or in a control transfer — X9's committed bytes already carry the consume on the fallback block's `JMP` (record 3, pc [1, 1]) — and X42 fixes the site decisively beyond that shape: the consume event rides the fallback block's `JMP` while the `SUCCESS` outcome is declared two blocks later, so the consumption site is the fallback block's own completion, never the execution's eventual exit. The rule's own negatives stand: a fallback block that misses again, births, or ends in a typed rejection never consumes (the committed §2.5 F12 rule; the X11/X12/X16/X17 residue shape).

**The declared rendering parameter** (declared here under §4.3's own delegation, binding the vectors exactly as a version component of `V`; no canon byte changes):

- **P12 — the declared dependency table:** a vector whose Declared parameters declare `D` = { addr i → v, … } populates the dependency environment's constant-table entry cited by the i-th declared content address (the addresses themselves ride the Intent's declared dependency list, envelope key 4, per P9) with the canonical integer value v, encoded per §2.6 (`INT_POS`); a `CLOAD` hit binds exactly that canonical table value to a fresh instance (§2.3), and the store component of the canonical configuration encoding (P2) carries it from the hit transition onward. Every vector declaring `D` = no entries — every seed and expansion-increment-1 vector — carries the empty table. The declared table is vector input (§2.1's `D`), never envelope content: the Intent envelope of a hit vector is byte-identical to the same program's miss envelope (X39's and X40's envelopes are byte-equal to X6's), the three vectors differing in the declared `D` and hence in the entire expected outcome — `Eval(Intent, D, V) = Execution` with `D` a first-class declared input.

**The scope disposition.** The bullet's declared scope is realized in full by four vectors: the hit path within the bound (X39 — the first populated-store execution of the corpus, the store component populated by the hit's fresh instance from the hit transition onward), the bound check's boundary (X40 — the hit value equal to its declared bound is admitted; "exceeds" is strict), the bound check's rejection side (X41 — the hit value exceeding its declared bound: the typed rejection before dispatch, no binding and no store effect, the fatal `EXPRESSION_TOO_COMPLEX` birth terminalizing at its own transition with the nil-selected receipt of the X10 shape, and the terminal settlement's refund riding the rejection record where the escrow is open — the X5 precedent, the `CONST` lead transition having completed), and the multi-block fallback completion (X42 — the consume event at the fallback block's `JMP`, the `SUCCESS` two blocks later, DR3). The escrow-never-opened shape (a typed rejection at the execution's first attempted transition, before any completed transition) is not part of this bullet's declared scope and stays unexercised — the reserve's first-completed-transition opening (§3.3) is fixed by the committed text and by every committed vector's settlement trace, and a vector for the unopened-escrow shape belongs to the budget-boundary family's registered expansion (Suite Register bullet 5), not to this increment.

## The Expansion Vector Register (X39–X42)

Every expansion vector declares `W_budget` = 100 U and `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) under the declared profile `qir-prof-golden-2`; the declared initial status pool is empty in every vector of this increment (no vector declares a pool — the pool machinery is increment 1's surface); the declared dependency table rides P12. Every program cites the seed's single declared content address (P9) in its Intent dependency list; the hit vectors populate its constant-table entry, X42 leaves the store empty.

| Vectors | Surface | Declared `D` | Outcome surface |
|---|---|---|---|
| X39 | `CLOAD` hit, value within the bound | { addr 0 → 7 } | `SUCCESS`; the store gains the hit's fresh instance (static value id 0, instance 0, value 7) from the hit transition onward |
| X40 | `CLOAD` hit at the bound's boundary | { addr 0 → 9 } | `SUCCESS`; the value equal to the declared bound does not exceed it — the strict reading of the §2.3 rejection clause is fixed |
| X41 | `CLOAD` hit, value exceeding the bound | { addr 0 → 10 } | `EXPRESSION_TOO_COMPLEX` (the typed rejection before dispatch, fatal at its own transition), nil selected; the rejection posts nothing and the open escrow's residue (12 U) refunds at the rejection record |
| X42 | multi-block fallback completion | no entries | `SUCCESS`; the miss token is consumed at the fallback block's `JMP` (record 3) and the outcome is declared two blocks later (record 4) — DR3's consumption site |

## Expansion Increment 3 — the TUPLE/FIELD/zoom-CMP/EMIT-witness Completion (v0.3.0-draft.16)

This expansion is R6 expansion increment 3 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.16), discharging the Suite Register's third bullet — the `TUPLE`/`FIELD` value and execution vectors, the zoom `CMP` execution vectors (the boundary comparison under N), and the `EMIT` witness vectors (the receipt ordinals and the witness payload). Its vectors are X43–X49 below, appended after X42 and derived after the committed four-artifact order and expansion increments 1–2 (F18 applied to expansions), from the normative rules of §2.1–§2.6 and §3 through the same anchored derivation instrument of record (a third codebase, sharing no code with either implementation and consulting neither; over the committed corpus it re-derives every committed execution vector byte-exact — all 42 — before any expansion byte was derived), and checked by both implementations only after these bytes were fixed. The seed's and increments 1–2's independently derived vectors are retained byte-unchanged. Unlike increments 1–2, whose artifact shares changed no execution rule, this increment's artifact share implements new opcode execution surfaces in both artifacts: the `TUPLE`, `FIELD`, and `EMIT` rows of §2.3's closed inventory and the zoom-carrying operand surface of the `CMP` row (the zoom-boundary comparison under N with the declared-range rejection) are committed rules no committed vector exercised before this corpus byte — the corpus adds no execution rule of its own, and the implementations are bound to exactly the committed rows.

**The declared readings and rendering parameters** (declared here under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline where the committed text leaves the point open; each cites its rule and binds the expansion vectors as a version component of `V`; collected as parameter **P13** in the declared-parameter block below):

- **P13(a) — the declared zoom ranges (§2.3's zoom declaration: "each within the pair's declared ranges").** For every zoom-carrying value of this increment's vectors, the normalized exponent z lies within [−4, 4] and the normalized mantissa q within [0, 2^64). The ranges are declared here because no envelope key carries them — the declared-vector-input shape of P12's `D` (§2.1's declared inputs ride the vector, not the envelope).
- **P13(b) — the CMP pre-dispatch declared-range check (§2.3 CMP row).** A zoom-carrying operand whose normalized exponent or mantissa lies outside the declared ranges of P13(a) fires the row's typed rejection before dispatch — no binding, no result instance, the fatal `EXPRESSION_TOO_COMPLEX` birth terminalizing at its own transition.
- **P13(c) — the bound reading over a non-integer result (§2.3 CONST row, "literal exceeds its declared bound"; §2.3 TUPLE row, "any component exceeds its declared bound").** The declared INT_POS upper bound of a zoom-result declaration constrains the normalized exponent — the pair's range-bearing component; z > bound fires the producing row's typed rejection before dispatch. A tuple-result declaration's or a TUPLE-typed field's bound word is present and vacuous: the check compares an INT_POS quantity, and a tuple carries none.
- **P13(d) — the witness rendering (§2.3 EMIT row; P7; P2 component 9).** The receipt-order queue carries the witness records in emission order, each entry `[ordinal, kind, [the cited values' current-instance canonical encodings as byte strings of their §2.6 word encodings], [block, index]]` — the ordinal P7's (dense from 0), kind the declared kind immediate, the emission position the emitting instruction's pc; record field 13 carries the ordinals the transition emitted (dense from 0; `[]` when it emits none). The terminal receipt is unchanged: closed keys 0–4, tokens only — the witness queue rides the state roots, not the receipt.

**The scope disposition.** The bullet's declared scope is realized in full by seven vectors: the flat tuple build over two in-bound integer components with both field extractions (X43), the component-bound typed rejection before dispatch with the open escrow's residue refunding at the rejection record — the X5/X41 precedent (X44), the nested tuple construction with a tuple-valued component and the chained field extraction through it (X45 — two store entries carrying byte-equal canonical values under distinct names, the §2.2 name discipline), the zoom boundary comparison's equal branch (X46 — the corpus's first zoom-carrying executions, two instances of one normalized pair, result 0), the ordered branch with the exact shift decided in both directions ((3, −1) against (1, 0): 3 against 2, and its symmetric −1) (X47), the CMP declared-range rejection with the residue refunding at the rejection record (X48), and two witness records with their ordinals, payload, and queue (X49).

## The Expansion Vector Register (X43–X49)

Every expansion vector declares `W_budget` = 100 U and `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) under the declared profile `qir-prof-golden-2`; the declared initial status pool is empty in every vector of this increment; `D` = no entries in every vector (the dependency surface is increments 1–2's). Every program is a single-block program; no block declares arguments.

| Vectors | Surface | Outcome surface |
|---|---|---|
| X43 | `TUPLE` build over two in-bound integer components; both `FIELD` extractions | `SUCCESS`; the store carries the tuple value and both extracted instances |
| X44 | `TUPLE` component-bound rejection | `EXPRESSION_TOO_COMPLEX` (the typed rejection before dispatch, fatal at its own transition), nil selected; the rejection posts nothing and the open escrow's residue (8 U) refunds at the rejection record |
| X45 | nested `TUPLE` construction; chained `FIELD` extractions | `SUCCESS`; two store entries carry byte-equal tuple canonical values under distinct names |
| X46 | zoom `CMP`, the equal branch under N | `SUCCESS`; the corpus's first zoom-carrying executions — two zoom instances, result 0 |
| X47 | zoom `CMP`, the ordered branch under N (the exact shift, both directions) | `SUCCESS`; results 1 and −1 |
| X48 | zoom `CMP` declared-range rejection | `EXPRESSION_TOO_COMPLEX` (the typed rejection before dispatch), nil selected; the rejection posts nothing and the open escrow's residue (8 U) refunds at the rejection record |
| X49 | `EMIT` witness records | `SUCCESS`; receipt ordinals 0 and 1; the queue rides the post-state roots from the first emission onward; field 13 cites each transition's own ordinals |

## Expansion Increment 4 — the Per-Class Ledger Trace Publications and the Budget-Boundary Per-Combination Completion (v0.3.0-draft.17)

This expansion is R6 expansion increment 4 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.17), discharging the Suite Register's fifth bullet — the per-class ledger trace publications beyond the seed's summaries, and the per-combination expansion of the budget-boundary family. Its vector is X50 below, appended after X49 and derived after the committed four-artifact order and expansion increments 1–3 (F18 applied to expansions), from the normative rules of §2.1–§2.6 and §3 through the same anchored derivation instrument of record (a third codebase, sharing no code with either implementation and consulting neither; over the committed corpus it re-derives every committed execution vector byte-exact — all 49 — before any expansion byte was derived), and checked by both implementations only after these bytes were fixed. The seed's and increments 1–3's independently derived vectors are retained byte-unchanged. The increment adds no execution rule to either artifact and declares no new rendering parameter: the escrow-never-opened settlement shape rides the committed rules — §3.3's reserve opening (once, by the execution's first completed transition), §3.4's admission and refusal, §2.5's typed-rejection failure behavior, and P11's settlement discipline (the refund riding the rejection record where the escrow is open) — and the dependency table rides P12; the corpus adds no execution rule of its own.

**The scope disposition.** The bullet's declared scope is realized in full by one vector and the register below. The family's settlement surface closes per combination: the open-escrow terminal settlements are the committed precedents (X1 — refund at `HALT`; X3, X10 — refund at the terminalizing exits), the open-escrow rejection-record refunds are the committed precedents (X5, X41, X44, X48 — the residue refunding at the rejection record), the admission refusal's zero-posting ledger is X2's (no execution — the refusal is the single rejection transition), and the last reachable cell — the escrow-never-opened typed rejection at the execution's first attempted transition — is X50: the declared table value (10) exceeds the `CLOAD`'s declared bound (9) before any transition completes, so the gateway never posts, the reserve never opens (§3.3: the reserve is opened once, by the execution's first completed transition), and no refund posts anywhere (Work(E) = 0 U; the program bytes are X39's — the envelope is byte-identical, the entire expected outcome differing by the declared `D`, the X39/X40/X6 precedent). The never-opened terminal settlement is unreachable by declaration: a terminal settlement requires a completed transition, and any completed transition opens the escrow. The publications beyond the seed's summaries are the Per-Class Ledger Trace Register below: one ledger trace per declared rule class, each citing the committed vector whose ledger block carries the class's postings, each mechanically checkable against the equation of §3.1 through the protocol of §4.1 — the cited vector's run reproducing the class trace at §4.1's agreement point 5.

## The Expansion Vector Register (X50)

The vector declares `W_budget` = 100 U and `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) under the declared profile `qir-prof-golden-2`; the declared initial status pool is empty; the dependency table rides P12. The program is a three-block program; no block declares arguments.

| Vectors | Surface | Outcome surface |
|---|---|---|
| X50 | `CLOAD` declared-bound rejection at the execution's first attempted transition (the declared table value exceeds the bound) | `EXPRESSION_TOO_COMPLEX` (the typed rejection before dispatch, fatal at its own transition), nil selected; the rejection posts nothing, the gateway never posts, the reserve never opens, and no refund posts anywhere — the escrow-never-opened settlement shape, Work(E) = 0 U |

## The Per-Class Ledger Trace Register (expansion increment 4)

The seed's vectors carry per-vector ledger summaries — each vector's reconstructed-ledger block lists the per-class traces of its own postings. This register is the publication beyond those summaries that §4.4 requires: **one ledger trace per declared rule class, each publication citing the committed vector whose ledger block carries the class's postings, each mechanically checkable against the equation of §3.1 through the protocol of §4.1** — the cited vector's run reproduces the class trace at §4.1's agreement point 5 (the reconstructed ledger with every §3.3 conservation identity). The declared class inventory of the profile `qir-prof-golden-2` is thereby published in full: the seed's six classes (the gateway and `q_const … q_halt`) and the four classes the expansions added (`q_load` — expansion increment 2; `q_tuple`, `q_field`, `q_emit` — expansion increment 3). Per §3.1 the gateway charge posts with the first completed transition's own rule charge as one posted quantity, so the gateway's posting site is that transition's; every class charge below is site-stable at 4 U (the declared profile).

| Class | Publication — postings in the cited vector's committed ledger | Class trace | The cited ledger's §3.1/§3.3 check |
|---|---|---|---|
| `gateway` | X1 — step 1 at [0, 0]: 20 U, posted with the first completed transition's own `q_const` charge as one 24 U quantity (§3.1) | 20 U (1 posting) | X1: Σ classes + gateway = 40 + 20 = Work(E) = 60 U; 100 = 60 + 40 + 0; remaining 100 − 60 = 40; refund 100 − 60 = 40; outstanding 0 |
| `q_const` | X1 — steps 1, 2 at [0, 0], [0, 1]: 4 U each | 8 U (2 postings) | X1 (as above) |
| `q_add` | X1 — steps 3, 6, 7, 8, 9 at [0, 2], [2, 0], [2, 1], [2, 2], [2, 3]: 4 U each | 20 U (5 postings) | X1 (as above) |
| `q_cmp` | X1 — step 4 at [0, 3]: 4 U (the zoom-carrying postings: X46–X48) | 4 U (1 posting) | X1 (as above); X46: 36 = 36 + 0 + 0 |
| `q_br` | X1 — step 5 at [0, 4]: 4 U | 4 U (1 posting) | X1 (as above) |
| `q_jmp` | X39 — step 2 at [0, 1]: 4 U (the fallback-completion posting: X42, the fallback block's own `JMP`, DR3) | 4 U (1 posting) | X39: Σ classes + gateway = 12 + 20 = Work(E) = 32 U; 32 = 32 + 0 + 0; remaining 100 − 32 = 68; refund 32 − 32 = 0; outstanding 0 |
| `q_halt` | X1 — step 10 at [2, 4]: 4 U (also X39 — step 3 at [2, 0]) | 4 U (1 posting) | X1 (as above) |
| `q_load` | X39 — step 1 at [0, 0]: 4 U, posted with the gateway as one 24 U quantity (also X40, X42; the rejections X41, X50 post nothing — P11) | 4 U (1 posting) | X39 (as above) |
| `q_tuple` | X43 — step 3 at [0, 2]: 4 U (also X45) | 4 U (1 posting) | X43: Σ classes + gateway = 24 + 20 = Work(E) = 44 U; 44 = 44 + 0 + 0; remaining 100 − 44 = 56; refund 44 − 44 = 0; outstanding 0 |
| `q_field` | X43 — steps 4, 5 at [0, 3], [0, 4]: 4 U each (also X45) | 8 U (2 postings) | X43 (as above) |
| `q_emit` | X49 — steps 4, 5 at [0, 3], [0, 4]: 4 U each | 8 U (2 postings) | X49: Σ classes + gateway = 24 + 20 = Work(E) = 44 U; 44 = 44 + 0 + 0; remaining 100 − 44 = 56; refund 44 − 44 = 0; outstanding 0 |

The X50 ledger closes the register's settlement dimension: its zero-posting block (Work(E) = 0 U, no per-class postings, the escrow never opened) is the committed rendering of the family's last reachable settlement cell.

## Expansion Increment 5 — the `EXT`-Payload Encoding Vectors (v0.3.0-draft.18)

This expansion is R6 expansion increment 5 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.18), discharging the Suite Register's sixth bullet — the encoding vectors for `EXT` record payloads whose reading types the corpus declares with its encoding version. Its vectors are R1–R4 and M13–M15 below — R1–R4 in the new Family R (`EXT` side-table record encodings, between Families I and M), M13–M15 appended to Family M — derived after the committed four-artifact order and expansion increments 1–4 (F18 applied to expansions), from the normative rules of §2.1–§2.6 and §3 through the same anchored derivation instrument of record (a third codebase, sharing no code with either implementation and consulting neither; over the committed corpus it re-derives every committed execution vector byte-exact — all 50 — before any expansion byte was derived), and checked by both implementations only after these bytes were fixed. The seed's and increments 1–4's independently derived vectors are retained byte-unchanged. The increment adds no execution rule to either artifact: the vectors ride the committed envelope grammar (key 7), the committed value grammar, and the committed duplicate-key rule, with the payload reading declared as P14 below — the corpus adds no execution rule of its own, and the implementations' artifact share is the declared reading check (the payload reading and the record-index uniqueness in the envelope decode path) plus the new Family R suite surface.

**The declared reading** (declared here under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline where the committed text leaves the point open; it cites its rule and binds the expansion vectors as a version component of `V`; collected as parameter **P14** in the declared-parameter block below): the reading types of an `EXT` side-table record's payload byte string under the encoding version `qir-enc-2` are the closed two-type set — the empty payload (the reading: no content) and the value payload (exactly one complete §2.6 value encoding, the reading: that value) — with the type determined by the payload itself, so the reading is total over the payload bytes and no per-record type tag exists or is needed.

**The scope disposition.** The bullet's declared scope is realized in full by four canonical vectors and three malformed counterparts: the empty-payload record (R1 — the reading "no content"), the integer-payload record (R2), the tuple-payload record (R3 — the composite form of the value grammar as payload), and the populated-table envelope (R4 — the committed M9 program with its side table carrying exactly the record the bitmask references, the dangling reference resolving); the malformed counterparts at the payload extent (M13 — trailing bytes after the complete value; M14 — the non-canonical value, M3's rule at the payload extent) and at the record structure (M15 — the duplicate record index, the committed duplicate-key rule's case). The instruction-bitmask coupling's committed surface is fixed by M9 (the dangling reference, statically illegal) and R4 (the reference resolving); no reading type beyond the declared two-type set is reachable — a payload is empty or it is bytes, and the reading is total over the payload.

## The Expansion Vector Register (R1–R4, M13–M15)

The vectors carry no declared parameters beyond P14 — no budget line: the families R and M fix encodings and verdicts, not executions. `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) rides each vector's encoding as committed.

| Vectors | Surface | Outcome surface |
|---|---|---|
| R1 | the empty-payload record (index 0) | canonical; the reading: no content |
| R2 | the integer-payload record (index 1; the exact integer 42) | canonical; the reading: the value 42 |
| R3 | the tuple-payload record (index 2; the tuple (1, −2)) | canonical; the reading: the tuple value |
| R4 | the M9 complement — the populated-table envelope (the record the bitmask references) | canonical; the dangling reference resolved, the record's payload reading empty |
| M13 | the value payload with trailing bytes after the complete value | `malformed — trailing bytes after the record's value payload` |
| M14 | the value payload non-canonical (magnitude words [0, 5]) | `malformed — leading zero magnitude word (non-canonical)` |
| M15 | the duplicate record index in one side table | `malformed — duplicate side-table record index` |
## Expansion Increment 6 — the `CLOAD` Fallback Trip-Ceiling Regression Vector (v0.3.0-draft.20)

This expansion is R6 expansion increment 6 of the Loop-3 stage (the program of record, `docs/qnir.md`, wave record v0.3.0-draft.20), discharging the Suite Register's `CLOAD` fallback trip-ceiling regression bullet — the regression vector registered when the F25 resolution adopted Reading R1 at v0.3.0-draft.19. Its single vector is X51 below, appended after X50 and derived from the amended normative text of §2.3 (the `CLOAD` row's rejection column and the derived-count paragraph), §2.5 (the Tier O repeated-miss rule), §3.3, and §3.4 — the amendment the F25 adjudication record fixes in `docs/logs/qnir/f25-cload-fallback-trip-ceiling-proposal.md` v0.2.0 — after the committed four-artifact order and expansion increments 1–5 (F18 applied to expansions). The vector was derived by hand from the amended rules before either artifact executed it, and both implementations were then run against the fixed bytes — the golden-first order applied to expansions. Unlike increments 1–5, the derivation of record here is the maintainer's hand derivation from the amended text: no third derivation instrument was used, and the two artifacts are the vector's independent checks, not its source. The seed's and increments 1–5's independently derived vectors are retained byte-unchanged. The increment adds no execution rule of its own to either artifact — the ceiling on the `CLOAD` miss edge, the zero-posting rejection, the open-escrow settlement, and the admission DP's counting of fallback back-edges are the amended or already-committed rules — but it completes the artifact share of the F25 resolution: verifying the vector exposed that the Rust artifact's `B_hat` admitted the self-loop while traversing the miss edge without consuming the site's remaining budget, so the correction the vector pins is the miss edge's `step_edge` treatment, which the Python artifact already applied.

**The declared reading** (declared here under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %} discipline where the committed text leaves the point open; it cites its rule and binds the expansion vector as a version component of `V`):

- **DR4 — the rejected miss carries no input/output ids (field 6 = `[]`).** The F25 adjudication record's DR-a parenthetical — that "the address still appears in the transition's input/output ids (field 6)" — is corrected here: a typed rejection posts nothing and binds nothing, so its record's field 6 is the empty list. The committed X41 bytes are decisive (the hit-bound rejection's record 2 carries `[]`); both artifacts' rejection path emits exactly that; and the address is already cited by the first, completed miss transition's field 6, where the binding occurred. The second traversal is rejected before any binding and therefore cites nothing.

**The scope disposition.** The bullet's declared scope is realized in full by one vector: X51 — the minimal self-looping-fallback program (`block 0`: `CLOAD addr=0 fallback=block 0` at index 0, `HALT` at index 1; no unreachable block, honoring observation O1 of the review that registered F25), `D` = no entries, trip ceiling 1, `W_budget` = 100 U. The first traversal completes — the miss token is born, the fallback site's count moves 0 → 1, and the transition charges the gateway and the `q_load` class (24 U). The second traversal is refused at the declared ceiling — the typed rejection fires after the lookup resolves to a miss and before a second token is born (§2.3's amended row and §2.5's amended Tier O rule), the fatal `EXPRESSION_TOO_COMPLEX` birth terminalizes at its own transition together with the outstanding miss token, the rejection posts nothing, and the open escrow's residue refunds there (32 − 24 = 8 U, the X5/X41 precedent). The vector pins the admission side as well: `B_hat(P) = 32` — gateway 20 + miss `CLOAD` 4 + hit `CLOAD` 4 + `HALT` 4 — counts the fallback back-edge exactly once, so the admission passes and the fuel backstop stays unfirable (§3.4/F16).

## The Expansion Vector Register (X51)

The vector declares `W_budget` = 100 U, `D` = no entries, the empty initial status pool, and `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2) under the declared profile `qir-prof-golden-2`. The program cites the seed's single declared content address (P9) in its Intent dependency list; the declared table stays empty, so every traversal of the fallback edge is a miss.

| Vector | Surface | Declared `D` | Outcome surface |
|---|---|---|---|
| X51 | `CLOAD` miss on a self-looping fallback edge under trip ceiling 1 | no entries | the first miss completes (one Tier O `STATUS_LOOKUP_MISS` born, site count 0 → 1, 24 U charged = gateway + `q_load`); the second traversal is rejected under §2.3 before the second token is born — `EXPRESSION_TOO_COMPLEX` (tag 3), nil selected, the open escrow's residue (8 U) refunded at the rejection record, the outstanding miss token terminalized there; `B_hat(P) = 32` (gateway 20 + miss `CLOAD` 4 + hit `CLOAD` 4 + `HALT` 4) |

## Declared Artifact-Wave Parameters

Declared here under §4.3's own delegation ("the hash's own algorithm and output width are declared with the encoding version at the artifact wave"); they ride the encoding version `qir-enc-2` of every vector's `V` and bind the vectors — no canon byte changes:

- **P1 — state-root function:** SHA-256, output width 32 bytes, over the canonical configuration encoding of P2.
- **P2 — canonical configuration encoding:** the ten components of §2.2's table in that table's order — program position `[block, index]`; block-argument environment (values in argument order); dynamic value store in `(static value id, instance id)` order, each entry `[static id, instance id, value bytes]`; status token pool in the full token-identifier order of §2.6, each entry `[birth step, block, index, serial, type, tier, state]`; work accumulators `[charged, reserved, refunded]`; clock; step index; halt state `[running flag (0 running, 1 halted), outcome tag (0 when running)]`; receipt-order queue in emission order; version bindings `[semantics, encoding, profile]`. Every component is encoded through deterministic CBOR, and a value inside CBOR is carried as the byte string of its §2.6 word encoding (§2.6: "every component through the value encoding above").
- **P3 — program root:** SHA-256 of the Intent envelope's canonical CBOR bytes (record field 2; §2.6 F5 field list).
- **P4 — first record's prev root:** the root of the declared initial configuration (§2.1: the initial configuration is declared, not computed).
- **P5 — step indices:** 1-based counts of completed transitions; a record's step index and clock field both carry the post-transition count (§2.2: one tick per transition; the fields stay distinct). Birth sites carry the birthing transition's own step index; the declared initial pool births at step 0 (§2.5).
- **P6 — token tier words:** Tier O = 1, Tier S = 2 (§2.5's two tiers); state words are §2.6's own receipt vocabulary: outstanding 1, consumed 2, terminalized 3.
- **P7 — receipt identifiers:** the emission ordinal, dense from 0 (§2.6 canonical maps); the receipt-order queue and record field 13 carry ordinals. No seed vector emits a witness, so both stay empty — witness payloads are an expansion obligation.
- **P8 — halt-state encoding:** `[running flag, outcome tag]` per P2's component 8, outcome tags per §2.6's closed terminal vocabulary (SUCCESS 1 … DOMAIN_ERROR 4).
- **P9 — dependency addresses:** the seed's single declared content address is the 8-byte string `h'0000000000000001'` (a declared vector parameter; §2.1 cites `D` by content address).
- **P10 — static value ids:** dense from 0 over the Intent's value-defining instructions in block-then-index scan order (§2.6: "static value ids are dense words over the Intent"); a `ref` operand cites these ids; an operand reads the current instance of its static id (§2.4: redefinition mints, never overwrites).
- **P11 — rejection records:** a typed rejection emits its own record with `work_charged = 0` (§3.1: rejection transitions post nothing), the terminal settlement's refund in `work_refunded` where the escrow is open (§3.3: settled by the execution's final transition, whatever its posting status), and the fatal birth traced in the status field (§2.5).

- **P12 — the declared dependency table (expansion increment 2):** the rendering of the declared dependency environment's populated constant-table entries in a vector's Declared parameters line — `D` = { addr i → v, … } over the Intent's declared dependency list (envelope key 4), the values canonical integers encoded per §2.6 (`INT_POS`); the empty table renders as the committed `D` = no entries form. The table is declared vector input (§2.1's `D`), never envelope content; the `CLOAD` hit binds exactly the declared canonical table value (§2.3). Declared by expansion increment 2 (v0.3.0-draft.15); every vector committed before this increment carries the empty table.

- **P13 — the declared zoom ranges, the CMP pre-dispatch check, the non-integer bound reading, and the witness rendering (expansion increment 3):** (a) the declared zoom ranges of §2.3's zoom declaration ("each within the pair's declared ranges") for this increment's vectors — the normalized exponent z within [−4, 4], the normalized mantissa q within [0, 2^64) — declared here because no envelope key carries them (the P12 declared-vector-input shape); (b) the CMP row's pre-dispatch check compares each zoom-carrying operand's normalized pair against (a) — outside, the row's typed rejection before dispatch; (c) the bound reading over a non-integer result — a zoom-result declaration's INT_POS upper bound constrains the normalized exponent (z > bound fires the producing row's typed rejection before dispatch), and a tuple-result or TUPLE-typed field declaration's bound word is present and vacuous (the check compares an INT_POS quantity, and a tuple carries none); (d) the witness rendering — the receipt-order queue (P2 component 9) carries the witness records in emission order, each entry [ordinal, kind, [the cited values' current-instance canonical encodings as byte strings of their §2.6 word encodings], [block, index]], the ordinal P7's (dense from 0), record field 13 carrying the ordinals the transition emitted (dense from 0, `[]` when it emits none). Declared by expansion increment 3 (v0.3.0-draft.16); every vector committed before this increment carries the empty queue and the empty field-13 list.

- **P14 — the declared `EXT` payload reading and the record structure (expansion increment 5):** (a) under the encoding version `qir-enc-2`, the reading types of an `EXT` side-table record's payload byte string (envelope key 7: "whose reading is the record's own declared type under the encoding version") are the closed two-type set: **the empty payload** (length 0 — the reading is no content) and **the value payload** (exactly one complete §2.6 value encoding — the reading is that value); the type is determined by the payload itself, so the reading is total over the payload bytes and no per-record type tag exists or is needed; (b) a payload decoding otherwise — a non-canonical value, a truncated value, a payload whose length is not a multiple of the 64-bit word, or trailing bytes after a complete value — makes the carrying Intent malformed, the value grammar's own error surfacing at the payload extent; (c) the record structure is the committed two-element array of the record index word and the payload byte string, the index word the record's key (the instruction bitmask's selection target, §2.6): a duplicate record index in one side table is the committed duplicate-key rule's case — malformed; (d) every vector committed before this increment carries the empty `EXT` side table, so the reading is exercised by this increment's vectors only. Declared by expansion increment 5 (v0.3.0-draft.18).
## The Declared Profile (`qir-prof-golden-2`)

One profile for every execution vector; the coefficients are profile parameters riding `V` (§3.2/§3.6 — no numeric charge table exists in the draft itself, and the corpus does not create one):

- gateway charge `g = 20` U, posted once by the first completed transition (§3.1);
- every rule charge class `q_const … q_halt = 4` U, site-stable, positive, inclusive of the base dispatch quantity (§3.1/§3.2);
- trip ceiling 1 per back-edge site (the depth ceiling of earlier versions is retired — F26, v0.3.0-draft.22: trip ceilings alone certify finiteness, and no operational measure of nesting depth exists for arbitrary block graphs);
- state-cardinality ceiling `S_max` = 4096, checked before the admission dynamic program (draft.31; §3.4 of `docs/qnir_accounting.md`);
- `W_budget` is a per-vector declared parameter (X1–X51 below);
- the surrogate DP is exact — `B_hat(P) = B(P)` for every well-formed Intent, with no slack since the depth ceiling's retirement (F26, v0.3.0-draft.22) — so the reserve opens at `B(P)` and no false admission or false rejection arises from the static bound (§3.4).

## Family E — Value Encodings

Each vector fixes the expected canonical bytes of one §2.6 value; the four §2.6 worked encodings are committed here as named vectors.

### E1 enc-zero

Cites: §2.6 value grammar; §2.6 worked encoding (canonical zero = INT_POS with count 0).

Input: the exact integer 0. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000000
```

### E2 enc-neg-one

Cites: §2.6 value grammar; §2.6 worked encoding (sign rides the type tag, magnitude words carry the absolute value).

Input: the exact integer −1. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000002 0000000000000001 0000000000000001
```

### E3 enc-int-2^64

Cites: §2.6 value grammar; §2.6 worked encoding (multiword magnitude, minimal big-endian).

Input: the exact integer 2^64. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000002 0000000000000001 0000000000000000
```

### E4 enc-tuple-1-minus-2

Cites: §2.6 value grammar; §2.6 worked encoding (field count word, then k complete field encodings in declared order).

Input: the tuple (1, −2). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000003 0000000000000002 0000000000000001 0000000000000001
0000000000000001 0000000000000002 0000000000000001 0000000000000002
```

### E5 enc-int-255

Cites: §2.6 value grammar (one magnitude word).

Input: the exact integer 255. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000001 00000000000000ff
```

### E6 enc-int-max-u64

Cites: §2.6 value grammar (one full magnitude word, no leading zero).

Input: the exact integer 2^64 − 1. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000001 ffffffffffffffff
```

### E7 enc-int-2^128

Cites: §2.6 value grammar (three magnitude words, minimal big-endian, no leading zero word).

Input: the exact integer 2^128. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000001 0000000000000003 0000000000000001 0000000000000000
0000000000000000
```

### E8 enc-neg-2^64

Cites: §2.6 value grammar (INT_NEG carries the absolute magnitude).

Input: the exact integer −2^64. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000002 0000000000000002 0000000000000001 0000000000000000
```

### E9 enc-tuple-nested

Cites: §2.6 value grammar (nested tuples: each field a complete value encoding).

Input: the tuple (0, (1, −2)). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000003 0000000000000002 0000000000000001 0000000000000000
0000000000000003 0000000000000002 0000000000000001 0000000000000001
0000000000000001 0000000000000002 0000000000000001 0000000000000002
```

### E10 enc-zoom-normalized

Cites: §2.6 value grammar (ZOOM: mantissa then exponent, each an INT_POS/INT_NEG encoding, in §2.3's normalized form); §2.3 normalizer N.

Input: the zoom-carrying value with mantissa 6, exponent −2, in the normalized form of §2.3's N (→ (3, −1)). Expected verdict: canonical. Expected canonical bytes:

```
0000000000000004 0000000000000001 0000000000000001 0000000000000003
0000000000000002 0000000000000001 0000000000000001
```

### E11 enc-zoom-canonical-zero

Cites: §2.3 normalizer N (N(0, z) = (0, 0)); §2.6 value grammar.

Input: the zoom-carrying value (0, 5): N(0, z) = (0, 0), the canonical zero of §2.6. Expected verdict: canonical. Expected canonical bytes:

```
0000000000000004 0000000000000001 0000000000000000 0000000000000001
0000000000000000
```

## Family I — Instruction Encodings

Each vector fixes the expected canonical word sequence of one instruction: opcode tag word, declared operand fields in declared order, one trailing ext word (§2.6 instruction grammar).

### I1 enc-insn-const-zero

Cites: §2.6 instruction grammar (CONST = imm, bnd; trailing ext word).

Input: CONST with literal 0, result bound (int, ≤ 0), no extension. Expected verdict: canonical. Expected canonical words:

```
0000000000000001 0000000000000001 0000000000000000 0000000000000001
0000000000000001 0000000000000000 0000000000000000
```

### I2 enc-insn-add

Cites: §2.6 instruction grammar (ADD = ref, ref, bnd).

Input: ADD of static value ids 0 and 1, result bound (int, ≤ 2), no extension. Expected verdict: canonical. Expected canonical words:

```
0000000000000005 0000000000000000 0000000000000001 0000000000000001
0000000000000001 0000000000000001 0000000000000002 0000000000000000
```

### I3 enc-insn-branch

Cites: §2.6 instruction grammar (BR = cc, ref, edge true, edge false; edge = blk, arity cnt, refs; condition codes LT 0 … GE 5).

Input: BR on condition code NE over static value id 0, true edge to block 1, false edge to block 2, both edges arity 0, no extension. Expected verdict: canonical. Expected canonical words:

```
000000000000000b 0000000000000003 0000000000000000 0000000000000001
0000000000000000 0000000000000002 0000000000000000 0000000000000000
```

## Family R — EXT Side-Table Record Encodings

Each vector fixes the expected canonical bytes of one `EXT` side-table record over the declared payload reading P14, or the canonical envelope bytes of the committed dangling-side-table program (M9) with its side table carrying the record the bitmask references — the M9 complement (§2.6 blocks-and-`EXT`-side-table: envelope key 7, "each a two-element array of the record index word and the payload byte string, whose reading is the record's own declared type under the encoding version"; the instruction's trailing `ext` bitmask word).

### R1 enc-ext-record-empty

Cites: §2.6 blocks-and-`EXT`-side-table (the record's two-element array form); P14 (the empty payload's reading: no content).

Input: the `EXT` side-table record (index 0, the empty payload). Expected verdict: canonical. Expected canonical bytes:

```
820040
```

### R2 enc-ext-record-int

Cites: §2.6 blocks-and-`EXT`-side-table (the payload byte string); §2.6 value grammar (INT_POS, one magnitude word); P14 (the value payload's reading: exactly one complete §2.6 value encoding).

Input: the `EXT` side-table record (index 1, the value payload carrying the exact integer 42). Expected verdict: canonical. Expected canonical bytes:

```
82015818 00000000 00000001 00000000
00000001 00000000 0000002a
```

### R3 enc-ext-record-tuple

Cites: §2.6 blocks-and-`EXT`-side-table (the payload byte string); §2.6 value grammar (tuples: the field count word, then k complete field encodings in declared order); P14.

Input: the `EXT` side-table record (index 2, the value payload carrying the tuple (1, −2)). Expected verdict: canonical. Expected canonical bytes:

```
82025840 00000000 00000003 00000000
00000002 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000002 00000000 00000001 00000000
00000002
```

### R4 enc-ext-envelope-populated

Cites: §2.6 blocks-and-`EXT`-side-table (the instruction's trailing `ext` bitmask word: bit i set means the instruction carries side-table record i); M9's committed input (the same one-block program — the declared-difference precedent of X50's ride on X39's envelope); P14 (the record's payload reading: no content — the reference resolving, the Intent statically legal).

Input: the M9 program's Intent envelope with its `EXT` side table carrying exactly the record (index 0, the empty payload). Expected verdict: canonical. Expected canonical bytes:

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 32037171 69722d70
726f662d 676f6c64 656e2d31 04800501
06585000 00000000 00000000 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000107 81820040 0880
```

## Family M — The Malformed Surface

Each vector fixes the expected verdict for one declared malformed or statically illegal class (§2.6: unknown-field rejection is total over every structure of the grammar).

### M1 insn-unknown-opcode

Cites: §2.6 closed tag assignments (the word 0 is no opcode; an opcode tag outside 1–16 is malformed).

Input bytes: an instruction whose opcode tag word is 17. Expected verdict: **malformed — unknown opcode tag**. Bytes:

```
0000000000000011 0000000000000000
```

### M2 value-negative-tagged-zero

Cites: §2.6 sign convention (a negative-tagged zero is malformed).

Input bytes: an INT_NEG encoding whose magnitude is 0. Expected verdict: **malformed — negative-tagged zero**. Bytes:

```
0000000000000002 0000000000000001 0000000000000000
```

### M3 value-leading-zero-word

Cites: §2.6 value grammar (minimal big-endian, no leading zero word).

Input bytes: an INT_POS encoding with magnitude words [0, 5]. Expected verdict: **malformed — leading zero magnitude word (non-canonical)**. Bytes:

```
0000000000000001 0000000000000002 0000000000000000 0000000000000005
```

### M4 value-non-normalized-zoom

Cites: §2.6 value grammar (a non-normalized zoom encoding is malformed); §2.3 N(6, −2) = (3, −1). The bytes are re-issued at v0.3.0-draft.11: the draft.10 form omitted the two component count words and did not parse as the pair it declares (the draft.11 wave record of docs/qnir.md names the disagreeing point and its §4.1 adjudication against §2.6).

Input bytes: a ZOOM encoding of the pair (6, −2) without normalization. Expected verdict: **malformed — non-normalized zoom encoding**. Bytes:

```
0000000000000004 0000000000000001 0000000000000001 0000000000000006
0000000000000002 0000000000000001 0000000000000002
```

### M5 value-count-exceeds-artifact

Cites: §2.6 structural bounds (a count must not exceed the remaining length of its enclosing artifact).

Input bytes: an INT_POS encoding declaring 3 magnitude words with only one word remaining in its artifact. Expected verdict: **malformed — count exceeds the enclosing artifact's remaining length**. Bytes:

```
0000000000000001 0000000000000003 00000000000000ff
```

### M6 value-unknown-type-tag

Cites: §2.6 closed tag assignments (no other type tag exists).

Input bytes: a value whose type-tag word is 5. Expected verdict: **malformed — unknown type tag**. Bytes:

```
0000000000000005 0000000000000000
```

### M7 insn-unknown-condition-code

Cites: §2.6 closed tag assignments (condition codes LT 0 … GE 5); §2.6 unknown-field rejection is total.

Input bytes: a BR instruction whose condition-code word is 6. Expected verdict: **malformed — unknown condition code**. Bytes:

```
000000000000000b 0000000000000006 0000000000000000 0000000000000001
0000000000000000 0000000000000002 0000000000000000 0000000000000000
```

### M8 envelope-unknown-key

Cites: §2.6 envelope grammar (an envelope carrying a key outside 0–8 is malformed).

Input bytes: an envelope map carrying key 9. Expected verdict: **malformed — unknown envelope key**. Bytes:

```
a10901
```

### M9 envelope-dangling-ext

Cites: §2.6 words-and-values (an unknown flag combination or a dangling side-table reference makes the Intent statically illegal, §2.1).

Input bytes: a one-block program whose CONST carries ext bit 0 (ext word 1) while the Intent's EXT side table is empty. Expected verdict: **statically illegal — dangling side-table reference**. Bytes:

```
a9004b51 6e49522d 496e7465 6e74016e
76302e33 2e302d64 72616674 2e350269
7169722d 656e632d 32037171 69722d70
726f662d 676f6c64 656e2d31 04800501
06585000 00000000 00000000 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000100 00000000
00000000 00000000 00000100 00000000
00000100 00000000 00000000 00000000
00000107 800880
```

### M10 stream-trailing-partial

Cites: §2.6 instruction grammar (a trailing partial instruction is malformed).

Input bytes: a block declaring instruction count 2 whose stream carries one complete CONST and no further words. Expected verdict: **malformed — trailing partial instruction**. Bytes:

```
0000000000000000 0000000000000000 0000000000000002 0000000000000001
0000000000000001 0000000000000000 0000000000000001 0000000000000001
0000000000000000 0000000000000000
```

### M11 envelope-duplicate-key

Cites: §2.6 canonical maps (a duplicate key in any encoded artifact is malformed).

Input bytes: an envelope map carrying key 5 twice. Expected verdict: **malformed — duplicate envelope key**. Bytes:

```
a2050205 03
```

### M12 envelope-unknown-token-type

Cites: §2.6 closed tag assignments (token type tags are the closed vocabulary of §2.5); §2.6 unknown-field rejection is total.

Input bytes: an Intent whose declared initial status pool contains token type tag 7. Expected verdict: **malformed — unknown token type tag**. Bytes:

```
a9004b516e49522d 496e74656e74016e 76302e332e302d64 726166742e350269
7169722d656e632d 3203717169722d70 726f662d676f6c64 656e2d3104800501
0658280000000000 0000000000000000 0000000000000000 0000010000000000
0000100000000000 0000000780088107
```

### M13 ext-payload-trailing-bytes

Cites: P14 (the value payload's reading: exactly one complete §2.6 value encoding — no trailing bytes); §2.6 value grammar.

Input bytes: an `EXT` side-table record of declared value reading whose payload carries trailing bytes after its complete value encoding. Expected verdict: **malformed — trailing bytes after the record's value payload**. Bytes:

```
82005820 00000000 00000001 00000000
00000001 00000000 00000001 00000000
00000000
```

### M14 ext-payload-leading-zero

Cites: P14 (the value payload's reading: the §2.6 value grammar's own canonicality at the payload extent); §2.6 value grammar (minimal big-endian, no leading zero word — the M3 rule).

Input bytes: an `EXT` side-table record of declared value reading whose payload carries a non-canonical integer encoding (magnitude words [0, 5]). Expected verdict: **malformed — leading zero magnitude word (non-canonical)**. Bytes:

```
82005820 00000000 00000001 00000000
00000002 00000000 00000000 00000000
00000005
```

### M15 ext-record-duplicate-index

Cites: §2.6 canonical maps (a duplicate key in any encoded artifact is malformed — the record index word is the record's key, the bitmask's selection target); §2.6 blocks-and-`EXT`-side-table.

Input bytes: an `EXT` side-table array carrying two records with the same record index word. Expected verdict: **malformed — duplicate side-table record index**. Bytes:

```
82820040 820040
```

## Family X — Execution Vectors

Each vector commits a complete expected outcome under §4.4's format: the encoded Intent, the declared version tuple `V` with the profile's charge parameters, the complete ordered transition-record sequence, the terminal receipt with its token itemization, the reconstructed ledger, and the expected terminal outcome. A vector passes only when both implementations independently match it at the five agreement points of §4.1.

### X1 — `exec-conservation-100-60-40`

Cites: §3.3 conservation identities (F13, v0.3.0-draft.7) — the finding's own worked settlement, realized; §3.4 admission; §4.4 conservation family.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=2 bnd=(int, <=2)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: CMP ref=0 ref=2 rng=[-1, 1]
  4: BR cc=GT ref=3 true=block 1 false=block 2
block 1 (arity 0):
  0: ADD ref=0 ref=1 bnd=(int, <=3)
  1: ADD ref=0 ref=1 bnd=(int, <=3)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: ADD ref=0 ref=1 bnd=(int, <=3)
  4: ADD ref=0 ref=1 bnd=(int, <=3)
  5: ADD ref=0 ref=1 bnd=(int, <=3)
  6: ADD ref=0 ref=1 bnd=(int, <=3)
  7: ADD ref=0 ref=1 bnd=(int, <=3)
  8: ADD ref=0 ref=1 bnd=(int, <=3)
  9: ADD ref=0 ref=1 bnd=(int, <=3)
  10: ADD ref=0 ref=1 bnd=(int, <=3)
  11: ADD ref=0 ref=1 bnd=(int, <=3)
  12: ADD ref=0 ref=1 bnd=(int, <=3)
  13: ADD ref=0 ref=1 bnd=(int, <=3)
  14: HALT
block 2 (arity 0):
  0: ADD ref=0 ref=1 bnd=(int, <=3)
  1: ADD ref=0 ref=1 bnd=(int, <=3)
  2: ADD ref=0 ref=1 bnd=(int, <=3)
  3: ADD ref=0 ref=1 bnd=(int, <=3)
  4: HALT
```

Intent envelope (canonical CBOR, 1682 bytes; instruction stream entry a byte string of 201 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05030659 06480000 00000000 00000000
  00000000 00000000 00000000 00050000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00020000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00020000 00000000 00000000
  00000000 00050000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 000a0000 00000000 00000000
  00000000 00020000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 000b0000 00000000 00040000
  00000000 00030000 00000000 00010000
  00000000 00000000 00000000 00020000
  00000000 00000000 00000000 00000000
  00000000 00010000 00000000 00000000
  00000000 000f0000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00050000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00030000
  00000000 00000000 00000000 00100000
  00000000 00000000 00000000 00020000
  00000000 00000000 00000000 00050000
  00000000 00050000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 00050000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 00050000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 00050000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 00100000 00000000 00000780
  0880
```

Admission: no back-edges; the maximum bounded path is block 0 (5 transitions, 20 U) plus block 1 (15 transitions, 60 U) = 80 U, plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (10 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   a17132e9 ddc83cf1 33565601 4b9ce08f
                     0da31b02 7a175d76 791ac95f b27e018d
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         a17132e9 ddc83cf1 33565601 4b9ce08f
                     0da31b02 7a175d76 791ac95f b27e018d
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000002 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a17132e9 ddc83cf1 33565601 4b9ce08f
                     0da31b02 7a175d76 791ac95f b27e018d
 8  post_state_root   b188613a 425f672a 0afb6e18 380aeb0b
                     03931720 507674c8 1731f38c 1f637f48
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         b188613a 425f672a 0afb6e18 380aeb0b
                     03931720 507674c8 1731f38c 1f637f48
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b188613a 425f672a 0afb6e18 380aeb0b
                     03931720 507674c8 1731f38c 1f637f48
 8  post_state_root   3da4b1d5 1102a624 fcb83b7b 197b142a
                     df15a693 2bdc4a78 821b2766 b7663e72
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         3da4b1d5 1102a624 fcb83b7b 197b142a
                     df15a693 2bdc4a78 821b2766 b7663e72
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000002
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3da4b1d5 1102a624 fcb83b7b 197b142a
                     df15a693 2bdc4a78 821b2766 b7663e72
 8  post_state_root   5926f41f 071d5fa9 53d45e62 56660530
                     31732b9c 9fc84648 929a66b1 fc3e6f01
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         5926f41f 071d5fa9 53d45e62 56660530
                     31732b9c 9fc84648 929a66b1 fc3e6f01
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      000000000000000b 0000000000000004 0000000000000003
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5926f41f 071d5fa9 53d45e62 56660530
                     31732b9c 9fc84648 929a66b1 fc3e6f01
 8  post_state_root   7aa51db1 ab0a3e99 53e64d96 e981f567
                     0c75e5c5 0c27473d 08a7fb30 e37f594f
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         7aa51db1 ab0a3e99 53e64d96 e981f567
                     0c75e5c5 0c27473d 08a7fb30 e37f594f
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        6
 4  pc                [2, 0]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7aa51db1 ab0a3e99 53e64d96 e981f567
                     0c75e5c5 0c27473d 08a7fb30 e37f594f
 8  post_state_root   bd205e28 80bdd72c c7232098 3b95fcc0
                     1bfb3969 77228aad 82a219d7 35ecc383
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         bd205e28 80bdd72c c7232098 3b95fcc0
                     1bfb3969 77228aad 82a219d7 35ecc383
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        7
 4  pc                [2, 1]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    bd205e28 80bdd72c c7232098 3b95fcc0
                     1bfb3969 77228aad 82a219d7 35ecc383
 8  post_state_root   e774450a b26edd13 8899de56 a708138b
                     8f35b662 0bc3d4e5 a2db2a65 0271b660
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         e774450a b26edd13 8899de56 a708138b
                     8f35b662 0bc3d4e5 a2db2a65 0271b660
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        8
 4  pc                [2, 2]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e774450a b26edd13 8899de56 a708138b
                     8f35b662 0bc3d4e5 a2db2a65 0271b660
 8  post_state_root   f453f224 43a8aec6 f2b7fa89 d37e7ffb
                     3c45fc6c a9c723be 8817df8d 62ed3203
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         f453f224 43a8aec6 f2b7fa89 d37e7ffb
                     3c45fc6c a9c723be 8817df8d 62ed3203
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        9
 4  pc                [2, 3]
 5  decoded_word      0000000000000005 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f453f224 43a8aec6 f2b7fa89 d37e7ffb
                     3c45fc6c a9c723be 8817df8d 62ed3203
 8  post_state_root   e5afa464 e0559707 3ce22f38 ea0ec39b
                     af2f03cb 024e0fa2 14fd9db4 139c4971
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         e5afa464 e0559707 3ce22f38 ea0ec39b
                     af2f03cb 024e0fa2 14fd9db4 139c4971
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        10
 4  pc                [2, 4]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e5afa464 e0559707 3ce22f38 ea0ec39b
                     af2f03cb 024e0fa2 14fd9db4 139c4971
 8  post_state_root   bf961175 d6c3a0f4 5ec978d2 b6cc2d1b
                     765db16f a3d5ceea 12e4a94a fd21147c
 9  work_charged      4
10  work_refunded     40
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization []
  3  selected_token   nil   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 60 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_add        20 U (5 postings)
    q_const       8 U (2 postings)
    q_cmp         4 U (1 posting)
    q_br          4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  100 /    0
    step  2:   28 /  100 /    0
    step  3:   32 /  100 /    0
    step  4:   36 /  100 /    0
    step  5:   40 /  100 /    0
    step  6:   44 /  100 /    0
    step  7:   48 /  100 /    0
    step  8:   52 /  100 /    0
    step  9:   56 /  100 /    0
    step 10:   60 /  100 /   40
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 100 = 60 + 40 + 0
    remaining_budget = W_budget - charged               : 100 - 60 = 40
    refund_final = B(P) - charged                       : 100 - 60 = 40
    reserve_outstanding after terminal settlement       : 0
```

### X2 — `exec-admission-refusal`

Cites: §3.4 admission rule and refusal (the refusal execution is the single rejection transition, the ledger closes at zero postings).

Declared parameters: `W_budget` = 60 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent: byte-identical to X1's committed envelope (equal vectors are byte-equal vectors, §4.4; the envelope is citable at X1).

Admission: identical program bytes to X1 — the same B(P) = 100 U; the declared budget 60 U is below it.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (1 record, complete ordered sequence; field values canonical per §2.6):

```
record 1 — rejection (posts nothing)
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      c3f0a573 182c4ff2 d8b84be0 356b2175
                     46edf5e5 3bac2b71 6ad38ecc c35465e1
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   a043b9ec 5535b895 797f2ddc a2fc86eb
                     098c5366 b0537636 5a02bf9c 9ba04caf
 9  work_charged      0
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87010000 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 0 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
  settlement trace (charged / reserved / refunded after each transition):
    step  1:    0 /    0 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 0 = 0 + 0 + 0
    remaining_budget = W_budget - charged               : 60 - 0 = 60
    refund_final                                        : the escrow never opened (admission refusal) — no terminal refund
    reserve_outstanding after terminal settlement       : 0
```

### X3 — `exec-halt-no-solution`

Cites: §2.5 total terminalization function T, case (2) with an outstanding STATUS_NEG_EVEN_ROOT; §2.5 declared initial status pool (the conformance birth path); §4.4 QO-22 test list — unresolved debt at every exit.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: HALT
```

Intent envelope (canonical CBOR, 234 bytes; instruction stream entry a byte string of 20 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010658 a0000000 00000000 00000000
  00000000 00000000 00000000 03000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 00000000
  00000000 00000000 00000000 0d000000
  00000000 02000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 10000000
  00000000 00078008 8102
```

Admission: no back-edges; the single path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      45d319fc 7e55c76b fea0456f d4ab0d64
                     37f98054 60d07022 91e10d7b 703eb02e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   68458551 6a25b314 c5318947 a31717a1
                     7bdf224b c7efae62 bf096fd5 6c1a3de1
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         68458551 6a25b314 c5318947 a31717a1
                     7bdf224b c7efae62 bf096fd5 6c1a3de1
 2  program_root      45d319fc 7e55c76b fea0456f d4ab0d64
                     37f98054 60d07022 91e10d7b 703eb02e
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    68458551 6a25b314 c5318947 a31717a1
                     7bdf224b c7efae62 bf096fd5 6c1a3de1
 8  post_state_root   ef6d659e 7067197a 4645a889 612cfb9f
                     e8164dde e766d2c3 2f56a4c0 e8111275
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         ef6d659e 7067197a 4645a889 612cfb9f
                     e8164dde e766d2c3 2f56a4c0 e8111275
 2  program_root      45d319fc 7e55c76b fea0456f d4ab0d64
                     37f98054 60d07022 91e10d7b 703eb02e
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ef6d659e 7067197a 4645a889 612cfb9f
                     e8164dde e766d2c3 2f56a4c0 e8111275
 8  post_state_root   116f83d7 87a7f6b1 fbf81c46 d41e3602
                     dd566495 297f0c15 8933102d 775bdf0e
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[3, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020281 87000000 00020203 03850000
  00000204 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
    q_obs         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X4 — `exec-promote-success`

Cites: §2.5 lifecycle verbs (consumption by the declared PROMOTE instruction); §2.3 PROMOTE row; §4.4 QO-22 test list — the success-after-promotion path.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: PROMOTE
  3: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  4: HALT
```

Intent envelope (canonical CBOR, 314 bytes; instruction stream entry a byte string of 30 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010658 f0000000 00000000 00000000
  00000000 00000000 00000000 05000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 00000000
  00000000 00000000 00000000 0d000000
  00000000 02000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 0e000000
  00000000 00000000 00000000 0d000000
  00000000 02000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 10000000
  00000000 00078008 8102
```

Admission: no back-edges; the single path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      2bebefc8 274c45f3 89c2d7ab dceb4132
                     89ad5148 f8843ccd c2d55bc6 9a67e194
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   33aa4da5 8e869311 efe59326 d4b15de9
                     03cb386e bf057b3b 2b97db2e 66dcdd64
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         33aa4da5 8e869311 efe59326 d4b15de9
                     03cb386e bf057b3b 2b97db2e 66dcdd64
 2  program_root      2bebefc8 274c45f3 89c2d7ab dceb4132
                     89ad5148 f8843ccd c2d55bc6 9a67e194
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    33aa4da5 8e869311 efe59326 d4b15de9
                     03cb386e bf057b3b 2b97db2e 66dcdd64
 8  post_state_root   465ac418 9da05a38 9d55de69 74c0833f
                     c6e17126 7afc2d4c c23e199e 6dec0793
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         465ac418 9da05a38 9d55de69 74c0833f
                     c6e17126 7afc2d4c c23e199e 6dec0793
 2  program_root      2bebefc8 274c45f3 89c2d7ab dceb4132
                     89ad5148 f8843ccd c2d55bc6 9a67e194
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    465ac418 9da05a38 9d55de69 74c0833f
                     c6e17126 7afc2d4c c23e199e 6dec0793
 8  post_state_root   49f8d1a1 ccd9c08e 95bc85bd 128a78fc
                     6ac9bcf1 03bd5629 d49d2995 e1473be5
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         49f8d1a1 ccd9c08e 95bc85bd 128a78fc
                     6ac9bcf1 03bd5629 d49d2995 e1473be5
 2  program_root      2bebefc8 274c45f3 89c2d7ab dceb4132
                     89ad5148 f8843ccd c2d55bc6 9a67e194
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    49f8d1a1 ccd9c08e 95bc85bd 128a78fc
                     6ac9bcf1 03bd5629 d49d2995 e1473be5
 8  post_state_root   cd32d72d 5630c8aa d1a18c92 216b0e73
                     b3667451 9651e09d ea235ef2 04e7e95c
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         cd32d72d 5630c8aa d1a18c92 216b0e73
                     b3667451 9651e09d ea235ef2 04e7e95c
 2  program_root      2bebefc8 274c45f3 89c2d7ab dceb4132
                     89ad5148 f8843ccd c2d55bc6 9a67e194
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cd32d72d 5630c8aa d1a18c92 216b0e73
                     b3667451 9651e09d ea235ef2 04e7e95c
 8  post_state_root   16e0ca0a 2ec30c8c 48289171 20ca908e
                     396d48c8 6630e72c 2971e0fd d46c48c9
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
  3  selected_token   nil   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010281 87000000 00020202 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_obs         8 U (2 postings)
    q_const       4 U (1 posting)
    q_promote     4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   40 /    0
    step  2:   28 /   40 /    0
    step  3:   32 /   40 /    0
    step  4:   36 /   40 /    0
    step  5:   40 /   40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X5 — `exec-domain-error`

Cites: §2.3 DIV row (zero divisor: the STATUS_DOMAIN_ERROR birth, terminalizing); §2.5 T case (1) — a terminalizing birth exits at its own transition; §3.1 rejection transitions post nothing.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=7)
  1: CONST imm=0 bnd=(int, <=0)
  2: DIV ref=0 ref=1 bnd=(int, <=7)
  3: HALT
```

Intent envelope (canonical CBOR, 305 bytes; instruction stream entry a byte string of 29 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010658 e8000000 00000000 00000000
  00000000 00000000 00000000 04000000
  00000000 01000000 00000000 01000000
  00000000 01000000 00000000 07000000
  00000000 01000000 00000000 01000000
  00000000 01000000 00000000 07000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 00000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 00000000
  00000000 08000000 00000000 00000000
  00000000 01000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 07000000 00000000 00000000
  00000000 10000000 00000000 00078008
  80
```

Admission: no back-edges; the single path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      2401f4b3 a6ca6135 e0849167 37c575d9
                     a3bcf7d0 090f18c1 62e49b89 daceff15
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000007 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   b4430d16 4fc3bf93 86dec439 1532cc64
                     d2d057a5 913fca53 1872cff9 a24b6d2a
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         b4430d16 4fc3bf93 86dec439 1532cc64
                     d2d057a5 913fca53 1872cff9 a24b6d2a
 2  program_root      2401f4b3 a6ca6135 e0849167 37c575d9
                     a3bcf7d0 090f18c1 62e49b89 daceff15
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b4430d16 4fc3bf93 86dec439 1532cc64
                     d2d057a5 913fca53 1872cff9 a24b6d2a
 8  post_state_root   72f9e221 da361f72 28cd07d5 bc2b738a
                     0df3ba91 62e36e8f 68f8e386 63389d71
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         72f9e221 da361f72 28cd07d5 bc2b738a
                     0df3ba91 62e36e8f 68f8e386 63389d71
 2  program_root      2401f4b3 a6ca6135 e0849167 37c575d9
                     a3bcf7d0 090f18c1 62e49b89 daceff15
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000008 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    72f9e221 da361f72 28cd07d5 bc2b738a
                     0df3ba91 62e36e8f 68f8e386 63389d71
 8  post_state_root   1e55c265 f272a090 861a75c0 ac01d828
                     c70819c6 2cb40fe7 3685bc2a 6d7029e7
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 0, 2, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [3, 0, 2, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [3, 0, 2, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040281 87030002 00040203 03850300
  02000404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   36 /    0
    step  2:   28 /   36 /    0
    step  3:   28 /   36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X6 — `exec-lookup-miss-fallback-success`

Cites: §2.3 CLOAD row (miss: births STATUS_LOOKUP_MISS, routes to the declared fallback edge); §2.5 tier rule (the token is consumed only when that fallback entry completes successfully — F12, v0.3.0-draft.5); §4.4 QO-22 test list — success after recovery.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 355 bytes; instruction stream entry a byte string of 34 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59011000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  800880
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 8  post_state_root   b9a00b0d c56398eb ded13055 ad948f90
                     10f1320a 8f757b03 ae75b162 7a217d62
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         b9a00b0d c56398eb ded13055 ad948f90
                     10f1320a 8f757b03 ae75b162 7a217d62
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b9a00b0d c56398eb ded13055 ad948f90
                     10f1320a 8f757b03 ae75b162 7a217d62
 8  post_state_root   615e9d01 b4e51031 08d922d3 9d54e559
                     205ef633 2e2d375f 3afff3ec f072dc1f
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010281 87010000 00010102 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X7 — `exec-trip-ceiling-rejection`

Cites: §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge); §2.3 derived counters (read from the record sequence); §3.3 terminal settlement at a rejection; §4.4 loops family.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=0 ref=1 rng=[-1, 1]
  3: BR cc=NE ref=2 true=block 1 false=block 2
block 1 (arity 0):
  0: JMP block 0
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 466 bytes; instruction stream entry a byte string of 49 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05030659 01880000 00000000 00000000
  00000000 00000000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00000000
  00000000 000a0000 00000000 00000000
  00000000 00010000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 000b0000 00000000 00030000
  00000000 00020000 00000000 00010000
  00000000 00000000 00000000 00020000
  00000000 00000000 00000000 00000000
  00000000 00010000 00000000 00000000
  00000000 00010000 00000000 000c0000
  00000000 00000000 00000000 00000000
  00000000 00000000 00000000 00020000
  00000000 00000000 00000000 00010000
  00000000 00100000 00000000 00000780
  0880
```

Admission: one back-edge (block 1 to block 0, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the true edge, one back-edge traversal, block 0 again, the false edge, and block 2 — 10 transitions (40 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (10 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   489b4da6 d401f413 4776f025 76352b62
                     4b896fdf ac1071e3 609d07da 73aeff10
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         489b4da6 d401f413 4776f025 76352b62
                     4b896fdf ac1071e3 609d07da 73aeff10
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    489b4da6 d401f413 4776f025 76352b62
                     4b896fdf ac1071e3 609d07da 73aeff10
 8  post_state_root   0df912d8 f69c9786 0ddea235 b9fe0232
                     91928a87 ee2a526b 006fa8c7 65040943
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         0df912d8 f69c9786 0ddea235 b9fe0232
                     91928a87 ee2a526b 006fa8c7 65040943
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0df912d8 f69c9786 0ddea235 b9fe0232
                     91928a87 ee2a526b 006fa8c7 65040943
 8  post_state_root   b37560ba d140baf6 8ac1564d 48221bd0
                     532afbb6 8a5563db 77f3ed3f 192ea26f
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         b37560ba d140baf6 8ac1564d 48221bd0
                     532afbb6 8a5563db 77f3ed3f 192ea26f
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000002
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b37560ba d140baf6 8ac1564d 48221bd0
                     532afbb6 8a5563db 77f3ed3f 192ea26f
 8  post_state_root   bdc021d5 7b49a047 8eb3bf46 07d3cb68
                     db6bd2dd 08ef055d c8d7921e 9502b3b1
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         bdc021d5 7b49a047 8eb3bf46 07d3cb68
                     db6bd2dd 08ef055d c8d7921e 9502b3b1
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        5
 4  pc                [1, 0]
 5  decoded_word      000000000000000c 0000000000000000 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    bdc021d5 7b49a047 8eb3bf46 07d3cb68
                     db6bd2dd 08ef055d c8d7921e 9502b3b1
 8  post_state_root   f29f9700 d871fba5 7ca6a005 e150dec2
                     85d0b974 ef93b377 e932e081 3b4127bc
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         f29f9700 d871fba5 7ca6a005 e150dec2
                     85d0b974 ef93b377 e932e081 3b4127bc
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        6
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f29f9700 d871fba5 7ca6a005 e150dec2
                     85d0b974 ef93b377 e932e081 3b4127bc
 8  post_state_root   8a61b29f ee015e41 23ae9007 6665b9f8
                     d452fb4d bc6693f2 61e811d6 b90e98ba
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         8a61b29f ee015e41 23ae9007 6665b9f8
                     d452fb4d bc6693f2 61e811d6 b90e98ba
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        7
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a61b29f ee015e41 23ae9007 6665b9f8
                     d452fb4d bc6693f2 61e811d6 b90e98ba
 8  post_state_root   509c2a13 3e26e0e8 489df6c4 92aad018
                     0e8d5515 a5deb159 49cfd2c5 b1e1629c
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         509c2a13 3e26e0e8 489df6c4 92aad018
                     0e8d5515 a5deb159 49cfd2c5 b1e1629c
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        8
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    509c2a13 3e26e0e8 489df6c4 92aad018
                     0e8d5515 a5deb159 49cfd2c5 b1e1629c
 8  post_state_root   563f7287 46c22603 f155b477 e278a114
                     0bd37598 7bf61de4 bc8e3c6d 8f788095
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         563f7287 46c22603 f155b477 e278a114
                     0bd37598 7bf61de4 bc8e3c6d 8f788095
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        9
 4  pc                [0, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000002
                     0000000000000001 0000000000000000 0000000000000002
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    563f7287 46c22603 f155b477 e278a114
                     0bd37598 7bf61de4 bc8e3c6d 8f788095
 8  post_state_root   f85c1f30 1d48a5e7 b845af6a 8464b97b
                     835e4f0f 970e502a 89a8dc33 96b55bb5
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — rejection (posts nothing)
 1  prev_root         f85c1f30 1d48a5e7 b845af6a 8464b97b
                     835e4f0f 970e502a 89a8dc33 96b55bb5
 2  program_root      4184b6e7 1b38a2e3 ad11193b bac3711e
                     15874ec0 1e119fbd 8d17af60 9f1b009f
 3  step_index        10
 4  pc                [1, 0]
 5  decoded_word      000000000000000c 0000000000000000 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f85c1f30 1d48a5e7 b845af6a 8464b97b
                     835e4f0f 970e502a 89a8dc33 96b55bb5
 8  post_state_root   0e9cb2af ac28becd 87edb254 eb7052ac
                     d7dbc5a1 9026ba67 7d62521f 6e638ace
 9  work_charged      0
10  work_refunded     4
11  clock             10
12  status            [[1, [10, 1, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [10, 1, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 870a0100 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 56 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      16 U (4 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_jmp         4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   60 /    0
    step  2:   28 /   60 /    0
    step  3:   32 /   60 /    0
    step  4:   36 /   60 /    0
    step  5:   40 /   60 /    0
    step  6:   44 /   60 /    0
    step  7:   48 /   60 /    0
    step  8:   52 /   60 /    0
    step  9:   56 /   60 /    0
    step 10:   56 /   60 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 60 = 56 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 56 = 44
    refund_final = B(P) - charged                       : 60 - 56 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X8 — `exec-multi-token-precedence`

Cites: §2.5 T case (2) — the earliest-birth outstanding Tier S token decides (ordered lexicographically by the full token identifier, F15); §4.4 QO-22 test list — multiple outstanding tokens.

Declared parameters: `W_budget` = 100 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: OBS ttype=STATUS_NEG_EVEN_ROOT rng=[0, 1]
  2: HALT
```

Intent envelope (canonical CBOR, 235 bytes; instruction stream entry a byte string of 20 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010658 a0000000 00000000 00000000
  00000000 00000000 00000000 03000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 00000000
  00000000 00000000 00000000 0d000000
  00000000 02000000 00000000 01000000
  00000000 00000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 00000000 00000000 10000000
  00000000 00078008 820203
```

Admission: no back-edges; the single path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         faf92abc 5ac97012 df9dbd23 b39847db
                     5b14cc1d 4ce90faf 4889224e ae5a47e6
 2  program_root      b28d2534 5aba8050 ca8c1c3e 56bc52aa
                     93f01aa5 dddf1499 8620cd09 336d886c
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    faf92abc 5ac97012 df9dbd23 b39847db
                     5b14cc1d 4ce90faf 4889224e ae5a47e6
 8  post_state_root   d9a0cfee 087df2c6 7802a54b abd86976
                     ec5ab262 35f1f42f b4b94c58 bc63ca41
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         d9a0cfee 087df2c6 7802a54b abd86976
                     ec5ab262 35f1f42f b4b94c58 bc63ca41
 2  program_root      b28d2534 5aba8050 ca8c1c3e 56bc52aa
                     93f01aa5 dddf1499 8620cd09 336d886c
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000d 0000000000000002 0000000000000001
                     0000000000000000 0000000000000001 0000000000000001
                     0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d9a0cfee 087df2c6 7802a54b abd86976
                     ec5ab262 35f1f42f b4b94c58 bc63ca41
 8  post_state_root   1cd47af8 482cc64f a20b21a7 18335232
                     fae908e3 d0caf168 476fe5a3 184cd21f
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         1cd47af8 482cc64f a20b21a7 18335232
                     fae908e3 d0caf168 476fe5a3 184cd21f
 2  program_root      b28d2534 5aba8050 ca8c1c3e 56bc52aa
                     93f01aa5 dddf1499 8620cd09 336d886c
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1cd47af8 482cc64f a20b21a7 18335232
                     fae908e3 d0caf168 476fe5a3 184cd21f
 8  post_state_root   e98261b8 689b987f 34013393 eba72ed3
                     04a50e42 52c72288 a062b8dd e2d83940
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[3, [0, 0, 0, 0, 2]], [3, [0, 0, 0, 1, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [0, 0, 0, 1, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020282 87000000 00020203 87000000
  01030203 03850000 00000204 83707630
  2e332e30 2d647261 66742e38 2e346971
  69722d65 6e632d32 71716972 2d70726f
  662d676f 6c64656e 2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
    q_obs         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /   32 /    0
    step  2:   28 /   32 /    0
    step  3:   32 /   32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X9 — `exec-combo-ner-noprom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_NEG_EVEN_ROOT) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {not promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 372 bytes; instruction stream entry a byte string of 36 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59012000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      956456fd 1f518c9b 67544237 32005edc
                     6e38653c ffd9beec 6688d926 d7fbc3e9
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 2  program_root      956456fd 1f518c9b 67544237 32005edc
                     6e38653c ffd9beec 6688d926 d7fbc3e9
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 8  post_state_root   b4118662 48a6d806 4b7a7be0 e1c828ea
                     a7804038 9ddc5f3d 82fe1054 06db0773
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         b4118662 48a6d806 4b7a7be0 e1c828ea
                     a7804038 9ddc5f3d 82fe1054 06db0773
 2  program_root      956456fd 1f518c9b 67544237 32005edc
                     6e38653c ffd9beec 6688d926 d7fbc3e9
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b4118662 48a6d806 4b7a7be0 e1c828ea
                     a7804038 9ddc5f3d 82fe1054 06db0773
 8  post_state_root   abdc001d ec067242 fc6289d0 84444982
                     9707df8b 0f59d6ac 1f492c42 b74cf0dc
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         abdc001d ec067242 fc6289d0 84444982
                     9707df8b 0f59d6ac 1f492c42 b74cf0dc
 2  program_root      956456fd 1f518c9b 67544237 32005edc
                     6e38653c ffd9beec 6688d926 d7fbc3e9
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    abdc001d ec067242 fc6289d0 84444982
                     9707df8b 0f59d6ac 1f492c42 b74cf0dc
 8  post_state_root   2d4f0cf4 e58119e8 f1514eac 2bc5fcba
                     10e41f8a b5c8a44c 846db7a3 9eb325d0
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[3, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020282 87000000 00020203 87010000
  01010102 03850000 00000204 83707630
  2e332e30 2d647261 66742e38 2e346971
  69722d65 6e632d32 71716972 2d70726f
  662d676f 6c64656e 2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 36 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   32 /  36 /    0
    step  4:   36 /  36 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 36 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 36 = 64
    refund_final = B(P) - charged                       : 36 - 36 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X10 — `exec-combo-ner-noprom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {not promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 724 bytes; instruction stream entry a byte string of 80 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59028000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 13 transitions (52 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (13 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   6058017b 7b51d2d5 756ed24d aee40b50
                     30dbb0d5 ec6e7bd8 0c41be2c 2d811eba
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         6058017b 7b51d2d5 756ed24d aee40b50
                     30dbb0d5 ec6e7bd8 0c41be2c 2d811eba
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6058017b 7b51d2d5 756ed24d aee40b50
                     30dbb0d5 ec6e7bd8 0c41be2c 2d811eba
 8  post_state_root   c358dc93 6f7b7103 b6f5c158 ef7d0679
                     6b9bcea3 75446f50 cae633b7 5bb97aa7
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         c358dc93 6f7b7103 b6f5c158 ef7d0679
                     6b9bcea3 75446f50 cae633b7 5bb97aa7
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c358dc93 6f7b7103 b6f5c158 ef7d0679
                     6b9bcea3 75446f50 cae633b7 5bb97aa7
 8  post_state_root   64f9402e 52819550 220b2efe d59385e5
                     4817874f 9fa8409c 35ae4456 24ff2171
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         64f9402e 52819550 220b2efe d59385e5
                     4817874f 9fa8409c 35ae4456 24ff2171
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    64f9402e 52819550 220b2efe d59385e5
                     4817874f 9fa8409c 35ae4456 24ff2171
 8  post_state_root   90cbf9cb c3a6b054 5a8d13f0 b93d1c03
                     dbb5cba8 d9f418d7 8d1e99b6 a8b4d92d
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         90cbf9cb c3a6b054 5a8d13f0 b93d1c03
                     dbb5cba8 d9f418d7 8d1e99b6 a8b4d92d
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    90cbf9cb c3a6b054 5a8d13f0 b93d1c03
                     dbb5cba8 d9f418d7 8d1e99b6 a8b4d92d
 8  post_state_root   f3f6a2ce 52090708 53f665df f5a95f5f
                     24817046 deb8dc89 6a77eaa4 2772aa7b
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         f3f6a2ce 52090708 53f665df f5a95f5f
                     24817046 deb8dc89 6a77eaa4 2772aa7b
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        6
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f3f6a2ce 52090708 53f665df f5a95f5f
                     24817046 deb8dc89 6a77eaa4 2772aa7b
 8  post_state_root   fd7feda1 b8217f77 45071930 d4aadeaa
                     26014714 5ce718b8 fc6cb0d3 28211584
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         fd7feda1 b8217f77 45071930 d4aadeaa
                     26014714 5ce718b8 fc6cb0d3 28211584
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        7
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    fd7feda1 b8217f77 45071930 d4aadeaa
                     26014714 5ce718b8 fc6cb0d3 28211584
 8  post_state_root   7934bc00 c6131683 11acdc4f 46255c99
                     5bebea34 ce7afe2e cce471eb c906b638
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         7934bc00 c6131683 11acdc4f 46255c99
                     5bebea34 ce7afe2e cce471eb c906b638
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        8
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7934bc00 c6131683 11acdc4f 46255c99
                     5bebea34 ce7afe2e cce471eb c906b638
 8  post_state_root   d79133f7 d3300508 5fd6da47 869c98b2
                     275706ae f6e28bf9 1aac5a94 da8a2bbb
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         d79133f7 d3300508 5fd6da47 869c98b2
                     275706ae f6e28bf9 1aac5a94 da8a2bbb
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        9
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d79133f7 d3300508 5fd6da47 869c98b2
                     275706ae f6e28bf9 1aac5a94 da8a2bbb
 8  post_state_root   a3018352 6fe921ca b57c3d85 eaeafcb4
                     7de430d1 e0d93a5c 06d484b7 de9ec420
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         a3018352 6fe921ca b57c3d85 eaeafcb4
                     7de430d1 e0d93a5c 06d484b7 de9ec420
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        10
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a3018352 6fe921ca b57c3d85 eaeafcb4
                     7de430d1 e0d93a5c 06d484b7 de9ec420
 8  post_state_root   077018f8 bef19597 200d7388 2340d4be
                     26577e53 97e954c0 2cfba1b8 d2cf2086
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         077018f8 bef19597 200d7388 2340d4be
                     26577e53 97e954c0 2cfba1b8 d2cf2086
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        11
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    077018f8 bef19597 200d7388 2340d4be
                     26577e53 97e954c0 2cfba1b8 d2cf2086
 8  post_state_root   37dd7859 a8dbe7fc 76e3e178 7eec69ae
                     bade8cce 9d35311d 48f6259c 39ac780b
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         37dd7859 a8dbe7fc 76e3e178 7eec69ae
                     bade8cce 9d35311d 48f6259c 39ac780b
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        12
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    37dd7859 a8dbe7fc 76e3e178 7eec69ae
                     bade8cce 9d35311d 48f6259c 39ac780b
 8  post_state_root   d444572e 0ed3744b 95686621 c8fb4913
                     f840e1f8 6fa3341b 1e78033d 7e1ba5cf
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — rejection (posts nothing)
 1  prev_root         d444572e 0ed3744b 95686621 c8fb4913
                     f840e1f8 6fa3341b 1e78033d 7e1ba5cf
 2  program_root      4a17437c 28a2f756 d1d00e61 6c91ad2d
                     514c774d 4f0b3c19 07b2962b 24bcac52
 3  step_index        13
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d444572e 0ed3744b 95686621 c8fb4913
                     f840e1f8 6fa3341b 1e78033d 7e1ba5cf
 8  post_state_root   f8e23dfa c4cc19fb 6f2b797d e2706dda
                     0dfafc16 e8c59657 c6c81cdc 1b2a0363
 9  work_charged      0
10  work_refunded     4
11  clock             13
12  status            [[1, [13, 3, 0, 2, 3]], [3, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [13, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00020203 87010000
  01010102 870d0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 68 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  72 /    0
    step  2:   28 /  72 /    0
    step  3:   32 /  72 /    0
    step  4:   36 /  72 /    0
    step  5:   40 /  72 /    0
    step  6:   44 /  72 /    0
    step  7:   48 /  72 /    0
    step  8:   52 /  72 /    0
    step  9:   56 /  72 /    0
    step 10:   60 /  72 /    0
    step 11:   64 /  72 /    0
    step 12:   68 /  72 /    0
    step 13:   68 /  72 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 72 = 68 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 68 = 32
    refund_final = B(P) - charged                       : 72 - 68 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X11 — `exec-combo-ner-noprom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_NEG_EVEN_ROOT) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {not promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 492 bytes; instruction stream entry a byte string of 51 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 59019800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088102
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **NO_SOLUTION** (tag 2).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      c8ebb697 53763d68 78f0b727 85c8f244
                     83ba29c6 fe186b64 b51c7657 114ca1f0
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   13543284 e0ca3d57 766edae1 c1406188
                     2cd8cd7f 2c1ff37c 34ccea97 5b84fb32
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         13543284 e0ca3d57 766edae1 c1406188
                     2cd8cd7f 2c1ff37c 34ccea97 5b84fb32
 2  program_root      c8ebb697 53763d68 78f0b727 85c8f244
                     83ba29c6 fe186b64 b51c7657 114ca1f0
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    13543284 e0ca3d57 766edae1 c1406188
                     2cd8cd7f 2c1ff37c 34ccea97 5b84fb32
 8  post_state_root   add8127c 7c3bf8f5 235c3356 4220c841
                     b8e9381a c5869f2e fd60a747 2d49f0ef
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         add8127c 7c3bf8f5 235c3356 4220c841
                     b8e9381a c5869f2e fd60a747 2d49f0ef
 2  program_root      c8ebb697 53763d68 78f0b727 85c8f244
                     83ba29c6 fe186b64 b51c7657 114ca1f0
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    add8127c 7c3bf8f5 235c3356 4220c841
                     b8e9381a c5869f2e fd60a747 2d49f0ef
 8  post_state_root   988426f9 74df6fb5 a8b3a61f 544c738f
                     12d3e65a ec3a335c e4052ddf 8e796129
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         988426f9 74df6fb5 a8b3a61f 544c738f
                     12d3e65a ec3a335c e4052ddf 8e796129
 2  program_root      c8ebb697 53763d68 78f0b727 85c8f244
                     83ba29c6 fe186b64 b51c7657 114ca1f0
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    988426f9 74df6fb5 a8b3a61f 544c738f
                     12d3e65a ec3a335c e4052ddf 8e796129
 8  post_state_root   c547d9b9 a45afdf9 ca4d96ae e4b5e61b
                     b74b159f 27d60dab 38504cc7 c9331685
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         c547d9b9 a45afdf9 ca4d96ae e4b5e61b
                     b74b159f 27d60dab 38504cc7 c9331685
 2  program_root      c8ebb697 53763d68 78f0b727 85c8f244
                     83ba29c6 fe186b64 b51c7657 114ca1f0
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c547d9b9 a45afdf9 ca4d96ae e4b5e61b
                     b74b159f 27d60dab 38504cc7 c9331685
 8  post_state_root   387e596b 3bb8aa01 d8beeb7a aa160d99
                     a7c3325e f913b4b4 baedb210 31dcdb31
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[3, [0, 0, 0, 0, 2]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      2 (NO_SOLUTION)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 2]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01020283 87000000 00020203 87010000
  01010103 87020100 02010102 03850000
  00000204 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X12 — `exec-combo-ner-noprom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {not promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 844 bytes; instruction stream entry a byte string of 95 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 5902f800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088102
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   bf02dfda 72aa948d 912cfdff 4bda75af
                     9d876b0c cb042b8e 62af8d3e 5e76ccaf
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         bf02dfda 72aa948d 912cfdff 4bda75af
                     9d876b0c cb042b8e 62af8d3e 5e76ccaf
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    bf02dfda 72aa948d 912cfdff 4bda75af
                     9d876b0c cb042b8e 62af8d3e 5e76ccaf
 8  post_state_root   90a55813 5ec67997 25785d2c c4a5707a
                     197f0b63 7848f0e9 fd0cf944 07c11f65
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         90a55813 5ec67997 25785d2c c4a5707a
                     197f0b63 7848f0e9 fd0cf944 07c11f65
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    90a55813 5ec67997 25785d2c c4a5707a
                     197f0b63 7848f0e9 fd0cf944 07c11f65
 8  post_state_root   3b2f96ca 97a7e44e a7bbb2aa 49d776f5
                     f2aa6fd5 cee0244c babffc8a 6d3dedd6
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         3b2f96ca 97a7e44e a7bbb2aa 49d776f5
                     f2aa6fd5 cee0244c babffc8a 6d3dedd6
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3b2f96ca 97a7e44e a7bbb2aa 49d776f5
                     f2aa6fd5 cee0244c babffc8a 6d3dedd6
 8  post_state_root   764356e5 0111f22c 7b37210c 588a4a74
                     81b46790 59610a25 96e7be87 092baa40
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         764356e5 0111f22c 7b37210c 588a4a74
                     81b46790 59610a25 96e7be87 092baa40
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    764356e5 0111f22c 7b37210c 588a4a74
                     81b46790 59610a25 96e7be87 092baa40
 8  post_state_root   455efaad 0e1a2bce b5fcbfad e3668abb
                     d495c287 190c835b 664cc37d 0de73b19
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         455efaad 0e1a2bce b5fcbfad e3668abb
                     d495c287 190c835b 664cc37d 0de73b19
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        6
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    455efaad 0e1a2bce b5fcbfad e3668abb
                     d495c287 190c835b 664cc37d 0de73b19
 8  post_state_root   5e842524 924ef8d1 adea6917 6354c8f0
                     993b6811 3f907bd1 e348e4b1 097f7962
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         5e842524 924ef8d1 adea6917 6354c8f0
                     993b6811 3f907bd1 e348e4b1 097f7962
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        7
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5e842524 924ef8d1 adea6917 6354c8f0
                     993b6811 3f907bd1 e348e4b1 097f7962
 8  post_state_root   8183692e bfd59d1c c36a2fa1 b94fcb41
                     28399488 7cf4b9b9 73e8bd81 6c69968b
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         8183692e bfd59d1c c36a2fa1 b94fcb41
                     28399488 7cf4b9b9 73e8bd81 6c69968b
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        8
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8183692e bfd59d1c c36a2fa1 b94fcb41
                     28399488 7cf4b9b9 73e8bd81 6c69968b
 8  post_state_root   8ea4622b 280f8ee6 01ac6cbf 8e888c3e
                     35781e03 aa62986e 321c942a 36122c49
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         8ea4622b 280f8ee6 01ac6cbf 8e888c3e
                     35781e03 aa62986e 321c942a 36122c49
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        9
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8ea4622b 280f8ee6 01ac6cbf 8e888c3e
                     35781e03 aa62986e 321c942a 36122c49
 8  post_state_root   4388f6d7 4b583498 4957b9cb 382f2e5c
                     c5874c3c 082284d8 f2e8d7ab d39835f6
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         4388f6d7 4b583498 4957b9cb 382f2e5c
                     c5874c3c 082284d8 f2e8d7ab d39835f6
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        10
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4388f6d7 4b583498 4957b9cb 382f2e5c
                     c5874c3c 082284d8 f2e8d7ab d39835f6
 8  post_state_root   dca9685d a6516f36 761c8c4b 107bab42
                     79646f9f 5b7b67a8 d4a9d56f 61dedc00
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         dca9685d a6516f36 761c8c4b 107bab42
                     79646f9f 5b7b67a8 d4a9d56f 61dedc00
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        11
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dca9685d a6516f36 761c8c4b 107bab42
                     79646f9f 5b7b67a8 d4a9d56f 61dedc00
 8  post_state_root   4c8e31de c4e548ad 07af6429 e4dd4fb4
                     35e89d4a 8bbc506f a890aaf2 58cabee7
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         4c8e31de c4e548ad 07af6429 e4dd4fb4
                     35e89d4a 8bbc506f a890aaf2 58cabee7
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        12
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4c8e31de c4e548ad 07af6429 e4dd4fb4
                     35e89d4a 8bbc506f a890aaf2 58cabee7
 8  post_state_root   c2c30d72 c1a105b4 066fc83f 914906da
                     f1b9be98 f68f3fad 7de2a6ea 8fce3c75
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         c2c30d72 c1a105b4 066fc83f 914906da
                     f1b9be98 f68f3fad 7de2a6ea 8fce3c75
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        13
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c2c30d72 c1a105b4 066fc83f 914906da
                     f1b9be98 f68f3fad 7de2a6ea 8fce3c75
 8  post_state_root   6e7fb286 fecf22b9 bbc1f6dc 14b1d3b4
                     7f55ea37 0d810443 5c517e14 e5562869
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         6e7fb286 fecf22b9 bbc1f6dc 14b1d3b4
                     7f55ea37 0d810443 5c517e14 e5562869
 2  program_root      9615ed2f 284cf693 3645bc6c 24f53b78
                     3ac88b76 40ef47d3 90c3cf39 e5cbe664
 3  step_index        14
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6e7fb286 fecf22b9 bbc1f6dc 14b1d3b4
                     7f55ea37 0d810443 5c517e14 e5562869
 8  post_state_root   844277aa 3e19c991 27fe3dfb b09c3333
                     c8827b48 fc8cf004 d61d77e7 bcb146f9
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 4, 0, 3, 3]], [3, [0, 0, 0, 0, 2]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00020203 87010000
  01010103 87020100 02010102 870e0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X13 — `exec-combo-ner-noprom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {not promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 404 bytes; instruction stream entry a byte string of 40 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59014000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      61586cae a77f9a45 b566137d 3f4ade9f
                     b5ba6d01 64922a86 be92bd74 665c6800
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 2  program_root      61586cae a77f9a45 b566137d 3f4ade9f
                     b5ba6d01 64922a86 be92bd74 665c6800
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e316ec70 f7072ee5 f7afb638 40de6c04
                     358b5427 d1974bff 548aa3c6 f6cdabfc
 8  post_state_root   c36448ae 3399ea06 62e218b2 f68e1879
                     5f849323 1ac64340 18c10cde 6ddc9115
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         c36448ae 3399ea06 62e218b2 f68e1879
                     5f849323 1ac64340 18c10cde 6ddc9115
 2  program_root      61586cae a77f9a45 b566137d 3f4ade9f
                     b5ba6d01 64922a86 be92bd74 665c6800
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c36448ae 3399ea06 62e218b2 f68e1879
                     5f849323 1ac64340 18c10cde 6ddc9115
 8  post_state_root   e9b11fac c64df98d 7a184065 5f065320
                     eb76a767 0507e686 c5b52918 e7d131c5
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 1, 1, 2, 4]], [3, [0, 0, 0, 0, 2]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 3]   # STATUS_NEG_EVEN_ROOT tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [3, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00020203 87010000
  01010103 87030101 02040203 03850301
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   28 /  36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X14 — `exec-combo-ner-prom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_NEG_EVEN_ROOT) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 388 bytes; instruction stream entry a byte string of 38 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59013000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      c5d3e0b0 9402742c 44f69628 085d01fd
                     a120e53a 0df08b7d 56ed57f2 bc053175
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 2  program_root      c5d3e0b0 9402742c 44f69628 085d01fd
                     a120e53a 0df08b7d 56ed57f2 bc053175
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 8  post_state_root   11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 2  program_root      c5d3e0b0 9402742c 44f69628 085d01fd
                     a120e53a 0df08b7d 56ed57f2 bc053175
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 8  post_state_root   e93016a0 58f6b08f 70ed9d45 17bea7a1
                     614bd8d7 70a5d7f2 847630c8 babcd6b5
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         e93016a0 58f6b08f 70ed9d45 17bea7a1
                     614bd8d7 70a5d7f2 847630c8 babcd6b5
 2  program_root      c5d3e0b0 9402742c 44f69628 085d01fd
                     a120e53a 0df08b7d 56ed57f2 bc053175
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e93016a0 58f6b08f 70ed9d45 17bea7a1
                     614bd8d7 70a5d7f2 847630c8 babcd6b5
 8  post_state_root   462760e7 a7c2b5df 5a2028a1 0d70c9b1
                     2c393523 7643731e 656abf20 a80ca563
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         462760e7 a7c2b5df 5a2028a1 0d70c9b1
                     2c393523 7643731e 656abf20 a80ca563
 2  program_root      c5d3e0b0 9402742c 44f69628 085d01fd
                     a120e53a 0df08b7d 56ed57f2 bc053175
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    462760e7 a7c2b5df 5a2028a1 0d70c9b1
                     2c393523 7643731e 656abf20 a80ca563
 8  post_state_root   9c25c2ae 97d7cc5a 7ead9354 4305ece9
                     26a94842 efe46282 40773ce0 59114326
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010282 87000000 00020202 87020001
  01010102 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X15 — `exec-combo-ner-prom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 740 bytes; instruction stream entry a byte string of 82 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59029000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   c898099b 259e5528 d4d5e357 2b9db20f
                     af153e9f e2b45270 255c9e9f c857aabc
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         c898099b 259e5528 d4d5e357 2b9db20f
                     af153e9f e2b45270 255c9e9f c857aabc
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    c898099b 259e5528 d4d5e357 2b9db20f
                     af153e9f e2b45270 255c9e9f c857aabc
 8  post_state_root   15b53987 92564d69 8388185b a696796b
                     df204deb e10ae786 27f1327d 1af62d51
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         15b53987 92564d69 8388185b a696796b
                     df204deb e10ae786 27f1327d 1af62d51
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    15b53987 92564d69 8388185b a696796b
                     df204deb e10ae786 27f1327d 1af62d51
 8  post_state_root   08f0059c dd52c33e 815983f9 b3d7e7f6
                     d3f08707 7dc22d05 ad5f5201 c0b67427
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         08f0059c dd52c33e 815983f9 b3d7e7f6
                     d3f08707 7dc22d05 ad5f5201 c0b67427
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    08f0059c dd52c33e 815983f9 b3d7e7f6
                     d3f08707 7dc22d05 ad5f5201 c0b67427
 8  post_state_root   d3ac46db 00f1b813 4a59fd09 0f863442
                     ff90124f a67c3870 cb658747 62c4827c
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         d3ac46db 00f1b813 4a59fd09 0f863442
                     ff90124f a67c3870 cb658747 62c4827c
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d3ac46db 00f1b813 4a59fd09 0f863442
                     ff90124f a67c3870 cb658747 62c4827c
 8  post_state_root   0d8491e7 7cba69bd f59d859d 18e6463c
                     01ff5fc9 8623babb 0138e2d2 46959b17
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         0d8491e7 7cba69bd f59d859d 18e6463c
                     01ff5fc9 8623babb 0138e2d2 46959b17
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        6
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0d8491e7 7cba69bd f59d859d 18e6463c
                     01ff5fc9 8623babb 0138e2d2 46959b17
 8  post_state_root   92ca6b59 52418316 b64d1cf6 7e05d174
                     c4c949d4 d973ea28 51f64318 7bc61739
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         92ca6b59 52418316 b64d1cf6 7e05d174
                     c4c949d4 d973ea28 51f64318 7bc61739
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        7
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    92ca6b59 52418316 b64d1cf6 7e05d174
                     c4c949d4 d973ea28 51f64318 7bc61739
 8  post_state_root   3aaf25a3 7579647d d5896bce 2ff29666
                     1e0d7886 10c554d8 89d5f3af a5a0c066
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         3aaf25a3 7579647d d5896bce 2ff29666
                     1e0d7886 10c554d8 89d5f3af a5a0c066
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        8
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3aaf25a3 7579647d d5896bce 2ff29666
                     1e0d7886 10c554d8 89d5f3af a5a0c066
 8  post_state_root   cfd1d6a5 00a44054 6e581c18 6ba7a122
                     0a25b3df ee2e18ee 332241fa bce2ee17
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         cfd1d6a5 00a44054 6e581c18 6ba7a122
                     0a25b3df ee2e18ee 332241fa bce2ee17
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        9
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cfd1d6a5 00a44054 6e581c18 6ba7a122
                     0a25b3df ee2e18ee 332241fa bce2ee17
 8  post_state_root   65f5b3e0 49781054 8853ad94 3cbe3820
                     93b0e4db c61921d1 9df58fec 2cedc7a6
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         65f5b3e0 49781054 8853ad94 3cbe3820
                     93b0e4db c61921d1 9df58fec 2cedc7a6
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        10
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    65f5b3e0 49781054 8853ad94 3cbe3820
                     93b0e4db c61921d1 9df58fec 2cedc7a6
 8  post_state_root   b3b1a4ec 9ef222b5 b0a8765e b633dbfd
                     2116311f f4336704 0e2877e6 9698dd48
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         b3b1a4ec 9ef222b5 b0a8765e b633dbfd
                     2116311f f4336704 0e2877e6 9698dd48
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        11
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b3b1a4ec 9ef222b5 b0a8765e b633dbfd
                     2116311f f4336704 0e2877e6 9698dd48
 8  post_state_root   09c4d775 c73b6331 e06e6a6e 3e75d348
                     93b17d42 eea1713e 88c2e532 e0604c50
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         09c4d775 c73b6331 e06e6a6e 3e75d348
                     93b17d42 eea1713e 88c2e532 e0604c50
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        12
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    09c4d775 c73b6331 e06e6a6e 3e75d348
                     93b17d42 eea1713e 88c2e532 e0604c50
 8  post_state_root   77c1000d 718bad4f 9c773c71 805e89e4
                     77ef2b15 94c81eb6 c45ec761 c213f4ea
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         77c1000d 718bad4f 9c773c71 805e89e4
                     77ef2b15 94c81eb6 c45ec761 c213f4ea
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        13
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    77c1000d 718bad4f 9c773c71 805e89e4
                     77ef2b15 94c81eb6 c45ec761 c213f4ea
 8  post_state_root   5e784483 2468df8b 6a5bf873 87a42ecb
                     dc1b2d6b 59394d25 1cbcc0be a46f96ab
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         5e784483 2468df8b 6a5bf873 87a42ecb
                     dc1b2d6b 59394d25 1cbcc0be a46f96ab
 2  program_root      6e2f7c31 9b98fde6 6f43daea 56e2b3c8
                     052d5372 0a3a805a 59ae6c8d 0cc68eb2
 3  step_index        14
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5e784483 2468df8b 6a5bf873 87a42ecb
                     dc1b2d6b 59394d25 1cbcc0be a46f96ab
 8  post_state_root   51cbf45d e52f8f85 027b9405 f20238b3
                     29dcfc34 1fd28f11 c9420ce7 f337f1c8
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 3, 0, 2, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00020202 87020001
  01010102 870e0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X16 — `exec-combo-ner-prom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_NEG_EVEN_ROOT) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 508 bytes; instruction stream entry a byte string of 53 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 5901a800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088102
```

Admission: no back-edges; the maximum path is 6 transitions (24 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (6 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   aff3db99 a50b15f2 ce9097c9 90649e5a
                     875ecc6a 4ac8cdbc e5de997e 7b50d5e5
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         aff3db99 a50b15f2 ce9097c9 90649e5a
                     875ecc6a 4ac8cdbc e5de997e 7b50d5e5
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    aff3db99 a50b15f2 ce9097c9 90649e5a
                     875ecc6a 4ac8cdbc e5de997e 7b50d5e5
 8  post_state_root   42d1870b f454fb10 e5fdd0e5 be2bc778
                     72225586 637d0a2c 3f17b375 3cb94321
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         42d1870b f454fb10 e5fdd0e5 be2bc778
                     72225586 637d0a2c 3f17b375 3cb94321
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    42d1870b f454fb10 e5fdd0e5 be2bc778
                     72225586 637d0a2c 3f17b375 3cb94321
 8  post_state_root   92be0052 25a1b2fa df283725 7f24d71a
                     f32d2571 08c11a74 9614d809 344fec1a
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         92be0052 25a1b2fa df283725 7f24d71a
                     f32d2571 08c11a74 9614d809 344fec1a
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    92be0052 25a1b2fa df283725 7f24d71a
                     f32d2571 08c11a74 9614d809 344fec1a
 8  post_state_root   42772996 7f56b741 7239673b e2abdc86
                     63f769ff d23b5102 dbe5d0a8 ce7d8007
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         42772996 7f56b741 7239673b e2abdc86
                     63f769ff d23b5102 dbe5d0a8 ce7d8007
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    42772996 7f56b741 7239673b e2abdc86
                     63f769ff d23b5102 dbe5d0a8 ce7d8007
 8  post_state_root   b1f7fa2e 116f35c8 e99ebf8f 800a4e80
                     c845c24f 7acd73c6 fe424f15 0224e1ff
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         b1f7fa2e 116f35c8 e99ebf8f 800a4e80
                     c845c24f 7acd73c6 fe424f15 0224e1ff
 2  program_root      9c0bbc13 e8aa9d06 57692805 0ecde2f4
                     65070cd0 50df6e28 d2de843d d5f53f45
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b1f7fa2e 116f35c8 e99ebf8f 800a4e80
                     c845c24f 7acd73c6 fe424f15 0224e1ff
 8  post_state_root   13f8f3a0 1000ebf0 fcaf0b16 b2ee350e
                     6bc7d886 4f047daa f2cafd9d 189f0db9
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            [[3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010283 87000000 00020202 87020001
  01010103 87030100 02010102 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 44 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_promote     4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  44 /    0
    step  2:   28 /  44 /    0
    step  3:   32 /  44 /    0
    step  4:   36 /  44 /    0
    step  5:   40 /  44 /    0
    step  6:   44 /  44 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 44 = 44 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 44 = 56
    refund_final = B(P) - charged                       : 44 - 44 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X17 — `exec-combo-ner-prom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 860 bytes; instruction stream entry a byte string of 97 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 59030800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088102
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 15 transitions (60 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (15 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   e1a220a6 5864663a b66beb72 043af582
                     06587758 661de151 82a489b8 0af7580e
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         e1a220a6 5864663a b66beb72 043af582
                     06587758 661de151 82a489b8 0af7580e
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    e1a220a6 5864663a b66beb72 043af582
                     06587758 661de151 82a489b8 0af7580e
 8  post_state_root   610950d8 c3927f18 1af26477 c9c65e61
                     b4778e59 a923db30 b9c3b141 5c8be099
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         610950d8 c3927f18 1af26477 c9c65e61
                     b4778e59 a923db30 b9c3b141 5c8be099
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    610950d8 c3927f18 1af26477 c9c65e61
                     b4778e59 a923db30 b9c3b141 5c8be099
 8  post_state_root   5bb00bd1 be7ec7ce 6290a71c 31a16f6e
                     939a3be1 1ca5ab34 ae576357 e9895e00
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         5bb00bd1 be7ec7ce 6290a71c 31a16f6e
                     939a3be1 1ca5ab34 ae576357 e9895e00
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5bb00bd1 be7ec7ce 6290a71c 31a16f6e
                     939a3be1 1ca5ab34 ae576357 e9895e00
 8  post_state_root   4c17afce 4038ae6f 6fe88fa2 936de73b
                     08b0ae33 276514fe 07e4fd77 f7f72b93
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         4c17afce 4038ae6f 6fe88fa2 936de73b
                     08b0ae33 276514fe 07e4fd77 f7f72b93
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4c17afce 4038ae6f 6fe88fa2 936de73b
                     08b0ae33 276514fe 07e4fd77 f7f72b93
 8  post_state_root   09b4f3b3 9c470241 20763d9d 00c4bfe3
                     a440c29e e6c59525 206d2b54 b883976b
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         09b4f3b3 9c470241 20763d9d 00c4bfe3
                     a440c29e e6c59525 206d2b54 b883976b
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    09b4f3b3 9c470241 20763d9d 00c4bfe3
                     a440c29e e6c59525 206d2b54 b883976b
 8  post_state_root   b6e1e907 08bae9c5 967dd78b 24713728
                     1109a7fb 3ea1b23a ae4d7db6 e420866f
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         b6e1e907 08bae9c5 967dd78b 24713728
                     1109a7fb 3ea1b23a ae4d7db6 e420866f
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        7
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b6e1e907 08bae9c5 967dd78b 24713728
                     1109a7fb 3ea1b23a ae4d7db6 e420866f
 8  post_state_root   288be8b7 15395b4e 545dbf2d 51baebc3
                     21a06ca2 fa10685d 4fbf8ec4 0c25db24
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         288be8b7 15395b4e 545dbf2d 51baebc3
                     21a06ca2 fa10685d 4fbf8ec4 0c25db24
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        8
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    288be8b7 15395b4e 545dbf2d 51baebc3
                     21a06ca2 fa10685d 4fbf8ec4 0c25db24
 8  post_state_root   9fa58410 4877838a a04512a4 2049638d
                     88da462a 711f6ebe 1a5d1cda 423b8f45
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         9fa58410 4877838a a04512a4 2049638d
                     88da462a 711f6ebe 1a5d1cda 423b8f45
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        9
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9fa58410 4877838a a04512a4 2049638d
                     88da462a 711f6ebe 1a5d1cda 423b8f45
 8  post_state_root   58a30ef0 96e77fdc 4ea114bf 9d49920b
                     dc73c055 33b1c206 2ae58580 10547b0a
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         58a30ef0 96e77fdc 4ea114bf 9d49920b
                     dc73c055 33b1c206 2ae58580 10547b0a
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        10
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    58a30ef0 96e77fdc 4ea114bf 9d49920b
                     dc73c055 33b1c206 2ae58580 10547b0a
 8  post_state_root   e4c339ce dbe45a3a 03a1e327 acde1279
                     59898500 2de8cae9 c72dedf8 5d115f70
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         e4c339ce dbe45a3a 03a1e327 acde1279
                     59898500 2de8cae9 c72dedf8 5d115f70
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        11
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e4c339ce dbe45a3a 03a1e327 acde1279
                     59898500 2de8cae9 c72dedf8 5d115f70
 8  post_state_root   8a9e3131 53030bef 3dd15121 b7b7bb1d
                     397b7e71 3786e37b 94dae5fe cdba1274
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         8a9e3131 53030bef 3dd15121 b7b7bb1d
                     397b7e71 3786e37b 94dae5fe cdba1274
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        12
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a9e3131 53030bef 3dd15121 b7b7bb1d
                     397b7e71 3786e37b 94dae5fe cdba1274
 8  post_state_root   f279eaec f86cd407 f157ffc7 4911bdd4
                     7811fe9e 555e5db4 3c470559 d68b1e2e
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         f279eaec f86cd407 f157ffc7 4911bdd4
                     7811fe9e 555e5db4 3c470559 d68b1e2e
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        13
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f279eaec f86cd407 f157ffc7 4911bdd4
                     7811fe9e 555e5db4 3c470559 d68b1e2e
 8  post_state_root   958ddf4f 12a0fa7a 21fbdf05 2135bbde
                     ccc6a837 d504238e 154e0008 d046d79f
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — completed
 1  prev_root         958ddf4f 12a0fa7a 21fbdf05 2135bbde
                     ccc6a837 d504238e 154e0008 d046d79f
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        14
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    958ddf4f 12a0fa7a 21fbdf05 2135bbde
                     ccc6a837 d504238e 154e0008 d046d79f
 8  post_state_root   53827ac4 731359c1 5d1b608b b40da594
                     1aa180ba 75f7fadb 0b02ebeb 3a5371f7
 9  work_charged      4
10  work_refunded     0
11  clock             14
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 15 — rejection (posts nothing)
 1  prev_root         53827ac4 731359c1 5d1b608b b40da594
                     1aa180ba 75f7fadb 0b02ebeb 3a5371f7
 2  program_root      e8811de9 85a313d0 cd0209cb 669774ab
                     646cefdb d486613f 6bfc9e5a 3d076a56
 3  step_index        15
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    53827ac4 731359c1 5d1b608b b40da594
                     1aa180ba 75f7fadb 0b02ebeb 3a5371f7
 8  post_state_root   b974ebf6 2fe7ed5c 67a58ec0 9da376a1
                     dad83528 6ac6e76b a0533194 71ac8cff
 9  work_charged      0
10  work_refunded     4
11  clock             15
12  status            [[1, [15, 4, 0, 3, 3]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [15, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00020202 87020001
  01010103 87030100 02010102 870f0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 76 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  80 /    0
    step  2:   28 /  80 /    0
    step  3:   32 /  80 /    0
    step  4:   36 /  80 /    0
    step  5:   40 /  80 /    0
    step  6:   44 /  80 /    0
    step  7:   48 /  80 /    0
    step  8:   52 /  80 /    0
    step  9:   56 /  80 /    0
    step 10:   60 /  80 /    0
    step 11:   64 /  80 /    0
    step 12:   68 /  80 /    0
    step 13:   72 /  80 /    0
    step 14:   76 /  80 /    0
    step 15:   76 /  80 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 80 = 76 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 76 = 24
    refund_final = B(P) - charged                       : 80 - 76 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X18 — `exec-combo-ner-prom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_NEG_EVEN_ROOT token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_NEG_EVEN_ROOT} × {promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [2] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 420 bytes; instruction stream entry a byte string of 42 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59015000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088102
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 2  program_root      99ce71cf eec0f5c0 769ea437 f5ae9c5a
                     4bd6f2ff ceb58525 9e040c86 cec3d78f
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04dbd184 1b69d73e 6b089078 00ad96f1
                     7a6dd953 c4ac5578 1533ad34 3b07b7a1
 8  post_state_root   7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[2, [0, 0, 0, 0, 2]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 2  program_root      99ce71cf eec0f5c0 769ea437 f5ae9c5a
                     4bd6f2ff ceb58525 9e040c86 cec3d78f
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    7b0d3940 dfee386d f422f18c 6f75ec72
                     a1d95e3e ffaa5586 1d46fe5c e298fe05
 8  post_state_root   11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 2  program_root      99ce71cf eec0f5c0 769ea437 f5ae9c5a
                     4bd6f2ff ceb58525 9e040c86 cec3d78f
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    11f0e018 94238f1c 519ab0a7 02b22902
                     2b35e7e0 4746b74f 97962bb8 c97a531c
 8  post_state_root   aff6c061 e571aced 658c1a06 8838bb56
                     f008c90b 9a8762d4 7a2db3b9 6a053e3e
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — rejection (posts nothing)
 1  prev_root         aff6c061 e571aced 658c1a06 8838bb56
                     f008c90b 9a8762d4 7a2db3b9 6a053e3e
 2  program_root      99ce71cf eec0f5c0 769ea437 f5ae9c5a
                     4bd6f2ff ceb58525 9e040c86 cec3d78f
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    aff6c061 e571aced 658c1a06 8838bb56
                     f008c90b 9a8762d4 7a2db3b9 6a053e3e
 8  post_state_root   8f15177f b6c5eaa1 c8cad6fa 35fbcb10
                     a374516b ff78becd a287b3ff b4349708
 9  work_charged      0
10  work_refunded     8
11  clock             4
12  status            [[1, [4, 1, 1, 2, 4]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 2, 2, 2]   # STATUS_NEG_EVEN_ROOT tier S state consumed
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [4, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [4, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00020202 87020001
  01010103 87040101 02040203 03850401
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   32 /  40 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 32 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 40 - 32 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X19 — `exec-combo-etc-noprom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (EXPRESSION_TOO_COMPLEX) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {not promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 372 bytes; instruction stream entry a byte string of 36 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59012000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      9a0afedb 7d35ecb7 713c8911 8693a23e
                     8073685d 15c5d3e0 e3495545 719739b9
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 2  program_root      9a0afedb 7d35ecb7 713c8911 8693a23e
                     8073685d 15c5d3e0 e3495545 719739b9
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 8  post_state_root   bef12035 6e0cd951 d2d299bf 15159269
                     03d47cbc 05d48d51 7993e16e aa7aab48
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         bef12035 6e0cd951 d2d299bf 15159269
                     03d47cbc 05d48d51 7993e16e aa7aab48
 2  program_root      9a0afedb 7d35ecb7 713c8911 8693a23e
                     8073685d 15c5d3e0 e3495545 719739b9
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    bef12035 6e0cd951 d2d299bf 15159269
                     03d47cbc 05d48d51 7993e16e aa7aab48
 8  post_state_root   b5aff0e1 7252f0ce e083c5cf 8df1f861
                     6130876e e7d2d8ff b5ab22a3 8783f340
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         b5aff0e1 7252f0ce e083c5cf 8df1f861
                     6130876e e7d2d8ff b5ab22a3 8783f340
 2  program_root      9a0afedb 7d35ecb7 713c8911 8693a23e
                     8073685d 15c5d3e0 e3495545 719739b9
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b5aff0e1 7252f0ce e083c5cf 8df1f861
                     6130876e e7d2d8ff b5ab22a3 8783f340
 8  post_state_root   906fb885 b9424825 e0df87c2 e76a5a44
                     2f6f74fd b163a855 f5a0362b a8a4db81
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[3, [0, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030282 87000000 00030203 87010000
  01010102 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 36 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   32 /  36 /    0
    step  4:   36 /  36 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 36 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 36 = 64
    refund_final = B(P) - charged                       : 36 - 36 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X20 — `exec-combo-etc-noprom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {not promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 724 bytes; instruction stream entry a byte string of 80 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59028000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 13 transitions (52 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (13 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   a70a7eb5 b1ce5a36 29a050b4 54dcc71c
                     f4e5dd7b 7f46cd07 cf2d3db6 917ae82a
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         a70a7eb5 b1ce5a36 29a050b4 54dcc71c
                     f4e5dd7b 7f46cd07 cf2d3db6 917ae82a
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a70a7eb5 b1ce5a36 29a050b4 54dcc71c
                     f4e5dd7b 7f46cd07 cf2d3db6 917ae82a
 8  post_state_root   da53ca70 b9da0553 1a0d5585 6281981a
                     e362b5b6 04ee7f60 0815076b 2c1f38e8
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         da53ca70 b9da0553 1a0d5585 6281981a
                     e362b5b6 04ee7f60 0815076b 2c1f38e8
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    da53ca70 b9da0553 1a0d5585 6281981a
                     e362b5b6 04ee7f60 0815076b 2c1f38e8
 8  post_state_root   50e35489 4563966e 7d10e2fc 1451b13c
                     f1c0e218 027af17f d73fba7c 8ffb370d
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         50e35489 4563966e 7d10e2fc 1451b13c
                     f1c0e218 027af17f d73fba7c 8ffb370d
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    50e35489 4563966e 7d10e2fc 1451b13c
                     f1c0e218 027af17f d73fba7c 8ffb370d
 8  post_state_root   ab94e7e5 05634bae 5250498f fe75976e
                     63948bed c92544c5 f6c11754 832e8dcc
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         ab94e7e5 05634bae 5250498f fe75976e
                     63948bed c92544c5 f6c11754 832e8dcc
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ab94e7e5 05634bae 5250498f fe75976e
                     63948bed c92544c5 f6c11754 832e8dcc
 8  post_state_root   863d7e23 3707dac2 b7242f72 f7d8f7be
                     a3f5a2e8 1f8bfd34 b760785d 312585f5
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         863d7e23 3707dac2 b7242f72 f7d8f7be
                     a3f5a2e8 1f8bfd34 b760785d 312585f5
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        6
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    863d7e23 3707dac2 b7242f72 f7d8f7be
                     a3f5a2e8 1f8bfd34 b760785d 312585f5
 8  post_state_root   e165b58e c1f41e75 656f97c8 dcbccba3
                     9e6940c3 ba73f7ac 5f98ad7c f9216133
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         e165b58e c1f41e75 656f97c8 dcbccba3
                     9e6940c3 ba73f7ac 5f98ad7c f9216133
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        7
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e165b58e c1f41e75 656f97c8 dcbccba3
                     9e6940c3 ba73f7ac 5f98ad7c f9216133
 8  post_state_root   dd93a7e6 84b3152a 92484347 f032fe56
                     83cf1ea5 fc8972d0 fabbd10a c1edac81
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         dd93a7e6 84b3152a 92484347 f032fe56
                     83cf1ea5 fc8972d0 fabbd10a c1edac81
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        8
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dd93a7e6 84b3152a 92484347 f032fe56
                     83cf1ea5 fc8972d0 fabbd10a c1edac81
 8  post_state_root   db5f8970 a44fa9d4 567ea105 51214293
                     a4274abc d4db7307 626ef667 98252786
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         db5f8970 a44fa9d4 567ea105 51214293
                     a4274abc d4db7307 626ef667 98252786
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        9
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    db5f8970 a44fa9d4 567ea105 51214293
                     a4274abc d4db7307 626ef667 98252786
 8  post_state_root   1d94574d c443e492 975a269e 16d3a868
                     2fc36d59 95297262 48a1dd46 12fe1cb8
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         1d94574d c443e492 975a269e 16d3a868
                     2fc36d59 95297262 48a1dd46 12fe1cb8
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        10
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1d94574d c443e492 975a269e 16d3a868
                     2fc36d59 95297262 48a1dd46 12fe1cb8
 8  post_state_root   4358a8e7 f1794946 d014349d 090be3d1
                     43367caa bbb7b0ba 9c722fb6 ee7e74bb
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         4358a8e7 f1794946 d014349d 090be3d1
                     43367caa bbb7b0ba 9c722fb6 ee7e74bb
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        11
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4358a8e7 f1794946 d014349d 090be3d1
                     43367caa bbb7b0ba 9c722fb6 ee7e74bb
 8  post_state_root   a3c4503d 3babf79e 2f367bb6 83fccdc7
                     d5930c01 7adffafe 784b7024 1dd38955
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         a3c4503d 3babf79e 2f367bb6 83fccdc7
                     d5930c01 7adffafe 784b7024 1dd38955
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        12
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a3c4503d 3babf79e 2f367bb6 83fccdc7
                     d5930c01 7adffafe 784b7024 1dd38955
 8  post_state_root   c732ac46 eff9e083 1118ddd4 d05a2eaa
                     c2607a78 1f1eec55 e2788220 699f1d36
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — rejection (posts nothing)
 1  prev_root         c732ac46 eff9e083 1118ddd4 d05a2eaa
                     c2607a78 1f1eec55 e2788220 699f1d36
 2  program_root      dff1797e a5f89fe8 59e100ba a6e213fa
                     36e39367 26520057 79e3be2b df99cee6
 3  step_index        13
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c732ac46 eff9e083 1118ddd4 d05a2eaa
                     c2607a78 1f1eec55 e2788220 699f1d36
 8  post_state_root   20578665 77e2c5f7 30169ad0 2864a44a
                     9b943d02 d2df1ae4 a444a836 a136a36b
 9  work_charged      0
10  work_refunded     4
11  clock             13
12  status            [[1, [13, 3, 0, 2, 3]], [3, [0, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [13, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00030203 87010000
  01010102 870d0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 68 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  72 /    0
    step  2:   28 /  72 /    0
    step  3:   32 /  72 /    0
    step  4:   36 /  72 /    0
    step  5:   40 /  72 /    0
    step  6:   44 /  72 /    0
    step  7:   48 /  72 /    0
    step  8:   52 /  72 /    0
    step  9:   56 /  72 /    0
    step 10:   60 /  72 /    0
    step 11:   64 /  72 /    0
    step 12:   68 /  72 /    0
    step 13:   68 /  72 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 72 = 68 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 68 = 32
    refund_final = B(P) - charged                       : 72 - 68 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X21 — `exec-combo-etc-noprom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (EXPRESSION_TOO_COMPLEX) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {not promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 492 bytes; instruction stream entry a byte string of 51 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 59019800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088103
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      312a50dc 65182c35 7236ebb6 7f7f76b7
                     8263850c 9dc90378 dabd4b88 fa5321e8
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   6469e3bf 6490774b 44b3cd0c 23bc7279
                     0025c8df 41b8eedd c41577b0 6eaf44b3
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         6469e3bf 6490774b 44b3cd0c 23bc7279
                     0025c8df 41b8eedd c41577b0 6eaf44b3
 2  program_root      312a50dc 65182c35 7236ebb6 7f7f76b7
                     8263850c 9dc90378 dabd4b88 fa5321e8
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    6469e3bf 6490774b 44b3cd0c 23bc7279
                     0025c8df 41b8eedd c41577b0 6eaf44b3
 8  post_state_root   9045de41 a78b3fab 6d09c023 6d846880
                     68bdd8f3 238c056e 4bcf0d38 04fc3a07
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         9045de41 a78b3fab 6d09c023 6d846880
                     68bdd8f3 238c056e 4bcf0d38 04fc3a07
 2  program_root      312a50dc 65182c35 7236ebb6 7f7f76b7
                     8263850c 9dc90378 dabd4b88 fa5321e8
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9045de41 a78b3fab 6d09c023 6d846880
                     68bdd8f3 238c056e 4bcf0d38 04fc3a07
 8  post_state_root   cfb9cd58 544a0200 7debd697 cd83d118
                     77f42391 8b65a387 18aa45d6 601a36c6
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         cfb9cd58 544a0200 7debd697 cd83d118
                     77f42391 8b65a387 18aa45d6 601a36c6
 2  program_root      312a50dc 65182c35 7236ebb6 7f7f76b7
                     8263850c 9dc90378 dabd4b88 fa5321e8
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cfb9cd58 544a0200 7debd697 cd83d118
                     77f42391 8b65a387 18aa45d6 601a36c6
 8  post_state_root   1a7ae980 00be35ae af63fed5 b4a5cd10
                     9c583047 b8c14230 d35d880a d161065a
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         1a7ae980 00be35ae af63fed5 b4a5cd10
                     9c583047 b8c14230 d35d880a d161065a
 2  program_root      312a50dc 65182c35 7236ebb6 7f7f76b7
                     8263850c 9dc90378 dabd4b88 fa5321e8
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1a7ae980 00be35ae af63fed5 b4a5cd10
                     9c583047 b8c14230 d35d880a d161065a
 8  post_state_root   a841bb47 b9c4b9ce 4e060ac0 e8136c76
                     99adabff d80d090f 0ce0784c e2a550d0
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[3, [0, 0, 0, 0, 3]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00030203 87010000
  01010103 87020100 02010102 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X22 — `exec-combo-etc-noprom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {not promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 844 bytes; instruction stream entry a byte string of 95 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 5902f800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088103
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   da9c9ad3 06b4c82b 809f0430 620f30eb
                     6a3578ce df5f9cff 0c996a79 a0129a94
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         da9c9ad3 06b4c82b 809f0430 620f30eb
                     6a3578ce df5f9cff 0c996a79 a0129a94
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    da9c9ad3 06b4c82b 809f0430 620f30eb
                     6a3578ce df5f9cff 0c996a79 a0129a94
 8  post_state_root   a6cc678a 159f1e18 06450ac4 3b3c54a1
                     7b730384 d738649e 858df2b0 cdc3e97b
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         a6cc678a 159f1e18 06450ac4 3b3c54a1
                     7b730384 d738649e 858df2b0 cdc3e97b
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a6cc678a 159f1e18 06450ac4 3b3c54a1
                     7b730384 d738649e 858df2b0 cdc3e97b
 8  post_state_root   a8c890c3 481d2dd2 57f252d9 956371b8
                     04cd2c0f a1030bd2 a70f3adf a14d8eb0
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         a8c890c3 481d2dd2 57f252d9 956371b8
                     04cd2c0f a1030bd2 a70f3adf a14d8eb0
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a8c890c3 481d2dd2 57f252d9 956371b8
                     04cd2c0f a1030bd2 a70f3adf a14d8eb0
 8  post_state_root   ec88eacc 1356866a 6db8271f 5ede3b95
                     143662ca 6858689c 5066f787 347150a1
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         ec88eacc 1356866a 6db8271f 5ede3b95
                     143662ca 6858689c 5066f787 347150a1
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ec88eacc 1356866a 6db8271f 5ede3b95
                     143662ca 6858689c 5066f787 347150a1
 8  post_state_root   8f936245 5ece8dbf b22f1c79 3ca2abac
                     4d441e25 5d1d2ad2 60e26e48 52706dda
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         8f936245 5ece8dbf b22f1c79 3ca2abac
                     4d441e25 5d1d2ad2 60e26e48 52706dda
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        6
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8f936245 5ece8dbf b22f1c79 3ca2abac
                     4d441e25 5d1d2ad2 60e26e48 52706dda
 8  post_state_root   e5fdb98f a046c39e abccf420 631923e5
                     94615336 0c0919b6 e7d69152 60fa3058
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         e5fdb98f a046c39e abccf420 631923e5
                     94615336 0c0919b6 e7d69152 60fa3058
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        7
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e5fdb98f a046c39e abccf420 631923e5
                     94615336 0c0919b6 e7d69152 60fa3058
 8  post_state_root   7d0f58f5 c30577d6 1dca6e05 37adbd76
                     4f0646b2 8bc6409d d7578f18 ff236f52
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         7d0f58f5 c30577d6 1dca6e05 37adbd76
                     4f0646b2 8bc6409d d7578f18 ff236f52
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        8
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7d0f58f5 c30577d6 1dca6e05 37adbd76
                     4f0646b2 8bc6409d d7578f18 ff236f52
 8  post_state_root   cb84308e 14c13f32 d9862a6b 7f52d94f
                     5ba02d4e 5e6ab08b 08ed3a57 5aa0fa61
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         cb84308e 14c13f32 d9862a6b 7f52d94f
                     5ba02d4e 5e6ab08b 08ed3a57 5aa0fa61
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        9
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cb84308e 14c13f32 d9862a6b 7f52d94f
                     5ba02d4e 5e6ab08b 08ed3a57 5aa0fa61
 8  post_state_root   b4e9f41e 540e21d4 34f122ba e65b7629
                     1940741d faf8ce03 fbe76c96 816e9e53
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         b4e9f41e 540e21d4 34f122ba e65b7629
                     1940741d faf8ce03 fbe76c96 816e9e53
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        10
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b4e9f41e 540e21d4 34f122ba e65b7629
                     1940741d faf8ce03 fbe76c96 816e9e53
 8  post_state_root   8f5982da 39e7c279 fc0a4478 7d7433e8
                     6ee7fca2 900ef432 876e6b9c 56f79674
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         8f5982da 39e7c279 fc0a4478 7d7433e8
                     6ee7fca2 900ef432 876e6b9c 56f79674
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        11
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8f5982da 39e7c279 fc0a4478 7d7433e8
                     6ee7fca2 900ef432 876e6b9c 56f79674
 8  post_state_root   1cecec48 29d1c520 d97b1149 0ca5624b
                     060ea967 0a042a11 47a4be0e 37912ea2
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         1cecec48 29d1c520 d97b1149 0ca5624b
                     060ea967 0a042a11 47a4be0e 37912ea2
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        12
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1cecec48 29d1c520 d97b1149 0ca5624b
                     060ea967 0a042a11 47a4be0e 37912ea2
 8  post_state_root   d2229498 e2deda92 6d154ac0 2d7358cc
                     2528d4c1 3b22816b 97e73002 7e7d2acf
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         d2229498 e2deda92 6d154ac0 2d7358cc
                     2528d4c1 3b22816b 97e73002 7e7d2acf
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        13
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d2229498 e2deda92 6d154ac0 2d7358cc
                     2528d4c1 3b22816b 97e73002 7e7d2acf
 8  post_state_root   80238e64 a0da433d 1468ff08 b1bd3fa6
                     11f1f1d3 6b9c7e1b 160a8f27 422570ae
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         80238e64 a0da433d 1468ff08 b1bd3fa6
                     11f1f1d3 6b9c7e1b 160a8f27 422570ae
 2  program_root      40077ea5 364201e2 8d7ac033 bd041cb6
                     31700168 44dfd0b7 03ecba1c 99e3b8dd
 3  step_index        14
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    80238e64 a0da433d 1468ff08 b1bd3fa6
                     11f1f1d3 6b9c7e1b 160a8f27 422570ae
 8  post_state_root   df4d5950 4431d549 8600ef18 60fc0f5a
                     94e279d5 e2b3664b 0ad367ae f44971d5
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 4, 0, 3, 3]], [3, [0, 0, 0, 0, 3]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00030203 87010000
  01010103 87020100 02010102 870e0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X23 — `exec-combo-etc-noprom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {not promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 404 bytes; instruction stream entry a byte string of 40 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59014000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      57a425af 6750ff37 12fbda74 ef7bd0b5
                     5b1c9d88 d056dede 3485dde0 714538cf
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 2  program_root      57a425af 6750ff37 12fbda74 ef7bd0b5
                     5b1c9d88 d056dede 3485dde0 714538cf
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    46cc9760 20c40588 6862c195 9e097b38
                     6501763b 0f3ac905 f846dc81 f5e749bb
 8  post_state_root   445bbfeb 2817209d af66caac 4af6fc77
                     0956f456 bf517488 abb53bd5 7cc73b4e
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         445bbfeb 2817209d af66caac 4af6fc77
                     0956f456 bf517488 abb53bd5 7cc73b4e
 2  program_root      57a425af 6750ff37 12fbda74 ef7bd0b5
                     5b1c9d88 d056dede 3485dde0 714538cf
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    445bbfeb 2817209d af66caac 4af6fc77
                     0956f456 bf517488 abb53bd5 7cc73b4e
 8  post_state_root   5bd33a21 bf8824d1 ba0bf832 bdbfe973
                     353a18ca ee822a02 0f059237 3f18d916
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 1, 1, 2, 4]], [3, [0, 0, 0, 0, 3]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [3, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00030203 87010000
  01010103 87030101 02040203 03850301
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   28 /  36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X24 — `exec-combo-etc-prom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (EXPRESSION_TOO_COMPLEX) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 388 bytes; instruction stream entry a byte string of 38 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59013000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      3bd3a485 3fcb30e5 2668fa8c 49bcfa5b
                     4cae0c45 c1c34771 f3549977 71f43ef6
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 2  program_root      3bd3a485 3fcb30e5 2668fa8c 49bcfa5b
                     4cae0c45 c1c34771 f3549977 71f43ef6
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 8  post_state_root   f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 2  program_root      3bd3a485 3fcb30e5 2668fa8c 49bcfa5b
                     4cae0c45 c1c34771 f3549977 71f43ef6
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 8  post_state_root   e504f6ba 81916ee0 bdd88b1d 0e51012c
                     2c1c4083 97b77613 a0f09c4b 18e501b9
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         e504f6ba 81916ee0 bdd88b1d 0e51012c
                     2c1c4083 97b77613 a0f09c4b 18e501b9
 2  program_root      3bd3a485 3fcb30e5 2668fa8c 49bcfa5b
                     4cae0c45 c1c34771 f3549977 71f43ef6
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e504f6ba 81916ee0 bdd88b1d 0e51012c
                     2c1c4083 97b77613 a0f09c4b 18e501b9
 8  post_state_root   3a270a48 efe74436 93ba5c07 ac0289ed
                     0ceb35f7 fe72cbac a97091f4 6d920084
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         3a270a48 efe74436 93ba5c07 ac0289ed
                     0ceb35f7 fe72cbac a97091f4 6d920084
 2  program_root      3bd3a485 3fcb30e5 2668fa8c 49bcfa5b
                     4cae0c45 c1c34771 f3549977 71f43ef6
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3a270a48 efe74436 93ba5c07 ac0289ed
                     0ceb35f7 fe72cbac a97091f4 6d920084
 8  post_state_root   2fa42d85 871270d5 c0af70c2 22addec9
                     438c53b2 b7939dc3 846307ea 88cbe487
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[3, [0, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030282 87000000 00030203 87020001
  01010102 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X25 — `exec-combo-etc-prom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 740 bytes; instruction stream entry a byte string of 82 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59029000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   25fdd75e 53874d16 c546c95c 60d5bd45
                     1343cb98 1a3f19f1 9ec5a57e 092c21df
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         25fdd75e 53874d16 c546c95c 60d5bd45
                     1343cb98 1a3f19f1 9ec5a57e 092c21df
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    25fdd75e 53874d16 c546c95c 60d5bd45
                     1343cb98 1a3f19f1 9ec5a57e 092c21df
 8  post_state_root   b7a41b64 dc75effc 6cf3fc12 730ecd57
                     f8813287 14e1ea45 87888909 503d11be
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         b7a41b64 dc75effc 6cf3fc12 730ecd57
                     f8813287 14e1ea45 87888909 503d11be
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b7a41b64 dc75effc 6cf3fc12 730ecd57
                     f8813287 14e1ea45 87888909 503d11be
 8  post_state_root   1a4ae919 ade9cee8 76c725a2 c43e30cd
                     93ed1b0e bd94c091 2ece9c08 11e9925a
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         1a4ae919 ade9cee8 76c725a2 c43e30cd
                     93ed1b0e bd94c091 2ece9c08 11e9925a
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1a4ae919 ade9cee8 76c725a2 c43e30cd
                     93ed1b0e bd94c091 2ece9c08 11e9925a
 8  post_state_root   43a304fc 52a005ad 3b879aa7 acf179b4
                     17829835 3e198cc5 707018cb f08a6ae3
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         43a304fc 52a005ad 3b879aa7 acf179b4
                     17829835 3e198cc5 707018cb f08a6ae3
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    43a304fc 52a005ad 3b879aa7 acf179b4
                     17829835 3e198cc5 707018cb f08a6ae3
 8  post_state_root   6ed3dd9f 7975cf15 b5b8b7a6 dbb79979
                     8e93d9f3 6ea49eee 36116b39 73e9061f
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         6ed3dd9f 7975cf15 b5b8b7a6 dbb79979
                     8e93d9f3 6ea49eee 36116b39 73e9061f
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        6
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6ed3dd9f 7975cf15 b5b8b7a6 dbb79979
                     8e93d9f3 6ea49eee 36116b39 73e9061f
 8  post_state_root   d7fe6264 f7b9f74b f529df6f 381d4639
                     a4ba08b5 8ed660fe c8a3ca2b 117d0b03
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         d7fe6264 f7b9f74b f529df6f 381d4639
                     a4ba08b5 8ed660fe c8a3ca2b 117d0b03
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        7
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d7fe6264 f7b9f74b f529df6f 381d4639
                     a4ba08b5 8ed660fe c8a3ca2b 117d0b03
 8  post_state_root   c7364ace a1f846ec 780074fa 33321430
                     861f4d90 c99ffb64 d3b28279 217f5129
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         c7364ace a1f846ec 780074fa 33321430
                     861f4d90 c99ffb64 d3b28279 217f5129
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        8
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c7364ace a1f846ec 780074fa 33321430
                     861f4d90 c99ffb64 d3b28279 217f5129
 8  post_state_root   77697ebb 058e8c8c 44033345 757efb76
                     be3ad77c e2fac47a 586e23bb c4b0f99e
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         77697ebb 058e8c8c 44033345 757efb76
                     be3ad77c e2fac47a 586e23bb c4b0f99e
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        9
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    77697ebb 058e8c8c 44033345 757efb76
                     be3ad77c e2fac47a 586e23bb c4b0f99e
 8  post_state_root   14c6cd36 d2ca0dcc c225cd71 cbc8a277
                     4ce9c04c db1d68a4 4cb6ad3f fbf6bf32
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         14c6cd36 d2ca0dcc c225cd71 cbc8a277
                     4ce9c04c db1d68a4 4cb6ad3f fbf6bf32
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        10
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    14c6cd36 d2ca0dcc c225cd71 cbc8a277
                     4ce9c04c db1d68a4 4cb6ad3f fbf6bf32
 8  post_state_root   cd901165 b6b9b888 b7f17c57 5919190f
                     8b88b60e e73d95fb 6ef1fe9b b0d0c089
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         cd901165 b6b9b888 b7f17c57 5919190f
                     8b88b60e e73d95fb 6ef1fe9b b0d0c089
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        11
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cd901165 b6b9b888 b7f17c57 5919190f
                     8b88b60e e73d95fb 6ef1fe9b b0d0c089
 8  post_state_root   18fe8040 b852edb7 684ecc06 90dcb058
                     b21b66eb 504c2721 617bafee 8f358435
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         18fe8040 b852edb7 684ecc06 90dcb058
                     b21b66eb 504c2721 617bafee 8f358435
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        12
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    18fe8040 b852edb7 684ecc06 90dcb058
                     b21b66eb 504c2721 617bafee 8f358435
 8  post_state_root   b324306b 4f7fd051 a3662740 38d9dced
                     2c853b84 dfbdb855 1e637a3a 46e60a25
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         b324306b 4f7fd051 a3662740 38d9dced
                     2c853b84 dfbdb855 1e637a3a 46e60a25
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        13
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b324306b 4f7fd051 a3662740 38d9dced
                     2c853b84 dfbdb855 1e637a3a 46e60a25
 8  post_state_root   3215c77a 6cb9da08 e70e6c9a 324339e8
                     cbe401b8 dd84a27e 7fa11486 ecc453fb
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         3215c77a 6cb9da08 e70e6c9a 324339e8
                     cbe401b8 dd84a27e 7fa11486 ecc453fb
 2  program_root      eb01cc53 ca34b27f d32fea52 39d702d3
                     5b676ae5 a057af30 17d5787b ab668889
 3  step_index        14
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3215c77a 6cb9da08 e70e6c9a 324339e8
                     cbe401b8 dd84a27e 7fa11486 ecc453fb
 8  post_state_root   c1ae1b9d 6af49058 b11c0fb8 54a2c634
                     aab6c94f cb762d2b 1db5520f 6b7d219f
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 3, 0, 2, 3]], [3, [0, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00030203 87020001
  01010102 870e0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X26 — `exec-combo-etc-prom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (EXPRESSION_TOO_COMPLEX) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 508 bytes; instruction stream entry a byte string of 53 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 5901a800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088103
```

Admission: no back-edges; the maximum path is 6 transitions (24 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (6 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   5c980b5a 613671d0 c9fc924f 1a6958cb
                     506594aa c562d2ee 4d49ab97 5173632a
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         5c980b5a 613671d0 c9fc924f 1a6958cb
                     506594aa c562d2ee 4d49ab97 5173632a
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    5c980b5a 613671d0 c9fc924f 1a6958cb
                     506594aa c562d2ee 4d49ab97 5173632a
 8  post_state_root   8daa970b a99ab4be 9bb82720 bd768756
                     124e91df 0c0b02e5 ab762deb 76772201
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         8daa970b a99ab4be 9bb82720 bd768756
                     124e91df 0c0b02e5 ab762deb 76772201
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    8daa970b a99ab4be 9bb82720 bd768756
                     124e91df 0c0b02e5 ab762deb 76772201
 8  post_state_root   0bab0579 c5b947a3 d4286091 5d2369c1
                     ce2d33d0 fc111747 66495c87 0b7aa537
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         0bab0579 c5b947a3 d4286091 5d2369c1
                     ce2d33d0 fc111747 66495c87 0b7aa537
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0bab0579 c5b947a3 d4286091 5d2369c1
                     ce2d33d0 fc111747 66495c87 0b7aa537
 8  post_state_root   e1e601c7 4cac2bd8 b322ac5e 5fe22d78
                     e1d10f52 0c4e538c 0e93ba4e 04c59821
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         e1e601c7 4cac2bd8 b322ac5e 5fe22d78
                     e1d10f52 0c4e538c 0e93ba4e 04c59821
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e1e601c7 4cac2bd8 b322ac5e 5fe22d78
                     e1d10f52 0c4e538c 0e93ba4e 04c59821
 8  post_state_root   7d9eddb5 1b301a3b 37233fc1 e258ae6c
                     c429b77c 0c2ea58f 13463272 c3c5f919
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         7d9eddb5 1b301a3b 37233fc1 e258ae6c
                     c429b77c 0c2ea58f 13463272 c3c5f919
 2  program_root      804e9697 ee397012 776bbf24 179ec71a
                     f510bd3c 87c87e77 82cfe140 1147b622
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7d9eddb5 1b301a3b 37233fc1 e258ae6c
                     c429b77c 0c2ea58f 13463272 c3c5f919
 8  post_state_root   d0e00014 d94ae9b7 2eecd0d7 d853f5cd
                     f5050149 50a92acc 021af364 43b0c8d7
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            [[3, [0, 0, 0, 0, 3]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00030203 87020001
  01010103 87030100 02010102 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 44 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_promote     4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  44 /    0
    step  2:   28 /  44 /    0
    step  3:   32 /  44 /    0
    step  4:   36 /  44 /    0
    step  5:   40 /  44 /    0
    step  6:   44 /  44 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 44 = 44 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 44 = 56
    refund_final = B(P) - charged                       : 44 - 44 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X27 — `exec-combo-etc-prom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 860 bytes; instruction stream entry a byte string of 97 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 59030800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088103
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 15 transitions (60 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (15 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   79483728 3f53d3db cdb05a57 ce0a6e26
                     e0e74b92 be35f04a 3fa32aa3 d756f6e0
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         79483728 3f53d3db cdb05a57 ce0a6e26
                     e0e74b92 be35f04a 3fa32aa3 d756f6e0
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    79483728 3f53d3db cdb05a57 ce0a6e26
                     e0e74b92 be35f04a 3fa32aa3 d756f6e0
 8  post_state_root   7cd8590e d2c0b38c f1848729 b6f751eb
                     bf34cfe8 e3fe1859 f9a540ce f7d7ff6f
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         7cd8590e d2c0b38c f1848729 b6f751eb
                     bf34cfe8 e3fe1859 f9a540ce f7d7ff6f
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    7cd8590e d2c0b38c f1848729 b6f751eb
                     bf34cfe8 e3fe1859 f9a540ce f7d7ff6f
 8  post_state_root   902cfb07 9edd8635 48c5b841 c70c877c
                     8dfc71da 8bc434bd d255bb26 43f11fde
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         902cfb07 9edd8635 48c5b841 c70c877c
                     8dfc71da 8bc434bd d255bb26 43f11fde
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    902cfb07 9edd8635 48c5b841 c70c877c
                     8dfc71da 8bc434bd d255bb26 43f11fde
 8  post_state_root   8487879f 531dce49 f583f5ff 1e4b37a4
                     075406f8 2573a76e a6d836f9 6ab2d60e
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         8487879f 531dce49 f583f5ff 1e4b37a4
                     075406f8 2573a76e a6d836f9 6ab2d60e
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8487879f 531dce49 f583f5ff 1e4b37a4
                     075406f8 2573a76e a6d836f9 6ab2d60e
 8  post_state_root   67a99332 898e2f74 3a4b2f57 9346c0ca
                     67a5c8ac 782477a7 0b0a1639 af62c5db
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         67a99332 898e2f74 3a4b2f57 9346c0ca
                     67a5c8ac 782477a7 0b0a1639 af62c5db
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    67a99332 898e2f74 3a4b2f57 9346c0ca
                     67a5c8ac 782477a7 0b0a1639 af62c5db
 8  post_state_root   9426229a 98ffee70 063527dc de16533d
                     0496fb8f 6a1cc38b d23c9797 e83b84e4
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         9426229a 98ffee70 063527dc de16533d
                     0496fb8f 6a1cc38b d23c9797 e83b84e4
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        7
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9426229a 98ffee70 063527dc de16533d
                     0496fb8f 6a1cc38b d23c9797 e83b84e4
 8  post_state_root   0b6e2149 215b6c1a 33801275 6316c9dc
                     03c207b8 245fbba4 53bf976e b7c14ec3
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         0b6e2149 215b6c1a 33801275 6316c9dc
                     03c207b8 245fbba4 53bf976e b7c14ec3
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        8
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0b6e2149 215b6c1a 33801275 6316c9dc
                     03c207b8 245fbba4 53bf976e b7c14ec3
 8  post_state_root   6585fd92 cf16a043 3a5f7b9f ceab5377
                     f24d5e94 20029d39 df228cfd f25abac3
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         6585fd92 cf16a043 3a5f7b9f ceab5377
                     f24d5e94 20029d39 df228cfd f25abac3
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        9
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6585fd92 cf16a043 3a5f7b9f ceab5377
                     f24d5e94 20029d39 df228cfd f25abac3
 8  post_state_root   6633095d 3955ae00 c680c0ac e82476a7
                     6e70acbe 9d02b653 7f2d0063 b7442b33
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         6633095d 3955ae00 c680c0ac e82476a7
                     6e70acbe 9d02b653 7f2d0063 b7442b33
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        10
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6633095d 3955ae00 c680c0ac e82476a7
                     6e70acbe 9d02b653 7f2d0063 b7442b33
 8  post_state_root   ad86020a c122d7e0 69fdffd8 aa735400
                     81a5f2f9 87f95f64 53f851ff 59a5f20a
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         ad86020a c122d7e0 69fdffd8 aa735400
                     81a5f2f9 87f95f64 53f851ff 59a5f20a
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        11
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ad86020a c122d7e0 69fdffd8 aa735400
                     81a5f2f9 87f95f64 53f851ff 59a5f20a
 8  post_state_root   f13863be a46227ad b7127990 bd21f094
                     92a3f672 a9d485b9 ebbb986b 884a81c5
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         f13863be a46227ad b7127990 bd21f094
                     92a3f672 a9d485b9 ebbb986b 884a81c5
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        12
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f13863be a46227ad b7127990 bd21f094
                     92a3f672 a9d485b9 ebbb986b 884a81c5
 8  post_state_root   16cd0d06 18b2c8bd 8d553080 6a55e4b1
                     73d9037f 2cf679e9 0f6b8485 4a333b1e
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         16cd0d06 18b2c8bd 8d553080 6a55e4b1
                     73d9037f 2cf679e9 0f6b8485 4a333b1e
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        13
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    16cd0d06 18b2c8bd 8d553080 6a55e4b1
                     73d9037f 2cf679e9 0f6b8485 4a333b1e
 8  post_state_root   a71ff976 7e4da461 ec446d3d da98b4f0
                     1cd22f30 b6e0d959 ac3818ec fb13dd66
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — completed
 1  prev_root         a71ff976 7e4da461 ec446d3d da98b4f0
                     1cd22f30 b6e0d959 ac3818ec fb13dd66
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        14
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a71ff976 7e4da461 ec446d3d da98b4f0
                     1cd22f30 b6e0d959 ac3818ec fb13dd66
 8  post_state_root   dd17ed66 27f826b9 79e80edd 7a95ae1b
                     0b2b3de3 74aa3fcb 26dfe66a 5bcb2d05
 9  work_charged      4
10  work_refunded     0
11  clock             14
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 15 — rejection (posts nothing)
 1  prev_root         dd17ed66 27f826b9 79e80edd 7a95ae1b
                     0b2b3de3 74aa3fcb 26dfe66a 5bcb2d05
 2  program_root      95985e73 95390f13 63aa942f 456dae9d
                     70fee89a 2b332b2a 251ca1f2 70e41e8d
 3  step_index        15
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dd17ed66 27f826b9 79e80edd 7a95ae1b
                     0b2b3de3 74aa3fcb 26dfe66a 5bcb2d05
 8  post_state_root   728882a4 a6d69b54 5d2d0133 ab9c643e
                     57780e7f 0f2bf412 f3acd420 92c79adc
 9  work_charged      0
10  work_refunded     4
11  clock             15
12  status            [[1, [15, 4, 0, 3, 3]], [3, [0, 0, 0, 0, 3]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [15, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00030203 87020001
  01010103 87030100 02010102 870f0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 76 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  80 /    0
    step  2:   28 /  80 /    0
    step  3:   32 /  80 /    0
    step  4:   36 /  80 /    0
    step  5:   40 /  80 /    0
    step  6:   44 /  80 /    0
    step  7:   48 /  80 /    0
    step  8:   52 /  80 /    0
    step  9:   56 /  80 /    0
    step 10:   60 /  80 /    0
    step 11:   64 /  80 /    0
    step 12:   68 /  80 /    0
    step 13:   72 /  80 /    0
    step 14:   76 /  80 /    0
    step 15:   76 /  80 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 80 = 76 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 76 = 24
    refund_final = B(P) - charged                       : 80 - 76 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X28 — `exec-combo-etc-prom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding EXPRESSION_TOO_COMPLEX token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {EXPRESSION_TOO_COMPLEX} × {promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [3] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 420 bytes; instruction stream entry a byte string of 42 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59015000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088103
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 2  program_root      80f2ef35 c0cc247f c8858202 f3f142ac
                     8883f201 a5c58692 d232b394 ebfe64ec
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    04b98fdf e97cf1e4 c98f9d28 5df12c83
                     788ba6db b845c5b6 eb49d21c e126421b
 8  post_state_root   ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 2  program_root      80f2ef35 c0cc247f c8858202 f3f142ac
                     8883f201 a5c58692 d232b394 ebfe64ec
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    ec67226d ec0c4d4a 7bd571e0 37d65c38
                     0d1dd13b cc2bf26e 6dad4e8d 5fdc3c1f
 8  post_state_root   f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 2  program_root      80f2ef35 c0cc247f c8858202 f3f142ac
                     8883f201 a5c58692 d232b394 ebfe64ec
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f56cc227 c39ef9cc 3c8e7935 cbda2cfc
                     51616769 4b1c8ab8 4dcf2bd3 e7198d85
 8  post_state_root   4ae6a06f 123363e6 0f05e623 a5de70cc
                     7a3c4f32 50884eeb f3e649b1 0519a9de
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — rejection (posts nothing)
 1  prev_root         4ae6a06f 123363e6 0f05e623 a5de70cc
                     7a3c4f32 50884eeb f3e649b1 0519a9de
 2  program_root      80f2ef35 c0cc247f c8858202 f3f142ac
                     8883f201 a5c58692 d232b394 ebfe64ec
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4ae6a06f 123363e6 0f05e623 a5de70cc
                     7a3c4f32 50884eeb f3e649b1 0519a9de
 8  post_state_root   a202ca38 f35b8e3f 59a1ee47 e1114b64
                     225c4435 52734e5b c8685c0f 94fb0ff9
 9  work_charged      0
10  work_refunded     8
11  clock             4
12  status            [[1, [4, 1, 1, 2, 4]], [3, [0, 0, 0, 0, 3]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [4, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [4, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00030203 87020001
  01010103 87040101 02040203 03850401
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   32 /  40 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 32 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 40 - 32 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X29 — `exec-combo-dom-noprom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_DOMAIN_ERROR) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {not promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 372 bytes; instruction stream entry a byte string of 36 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59012000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      79652cfc ba98dd28 4a033081 d0d1b5ce
                     75d2f277 8053e904 4ea772f0 45b30f5e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 2  program_root      79652cfc ba98dd28 4a033081 d0d1b5ce
                     75d2f277 8053e904 4ea772f0 45b30f5e
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 8  post_state_root   3eb783d0 7d20c2c5 2fc66487 a11fd516
                     60ae2007 790e56d1 cc82350e d0a5eb26
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         3eb783d0 7d20c2c5 2fc66487 a11fd516
                     60ae2007 790e56d1 cc82350e d0a5eb26
 2  program_root      79652cfc ba98dd28 4a033081 d0d1b5ce
                     75d2f277 8053e904 4ea772f0 45b30f5e
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3eb783d0 7d20c2c5 2fc66487 a11fd516
                     60ae2007 790e56d1 cc82350e d0a5eb26
 8  post_state_root   69e10cf3 7479610b f4af6e79 f82a6119
                     1a4ac187 bd2e6ce8 47a10ef4 c6f6eca6
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         69e10cf3 7479610b f4af6e79 f82a6119
                     1a4ac187 bd2e6ce8 47a10ef4 c6f6eca6
 2  program_root      79652cfc ba98dd28 4a033081 d0d1b5ce
                     75d2f277 8053e904 4ea772f0 45b30f5e
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    69e10cf3 7479610b f4af6e79 f82a6119
                     1a4ac187 bd2e6ce8 47a10ef4 c6f6eca6
 8  post_state_root   089adb72 b5d361cb ed1876f8 326f8177
                     1c760696 6825e0e0 38cbcaf4 ca1ecc00
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[3, [0, 0, 0, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040282 87000000 00040203 87010000
  01010102 03850000 00000404 83707630
  2e332e30 2d647261 66742e38 2e346971
  69722d65 6e632d32 71716972 2d70726f
  662d676f 6c64656e 2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 36 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   32 /  36 /    0
    step  4:   36 /  36 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 36 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 36 = 64
    refund_final = B(P) - charged                       : 36 - 36 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X30 — `exec-combo-dom-noprom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {not promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 724 bytes; instruction stream entry a byte string of 80 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59028000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 13 transitions (52 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (13 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   4a26a496 3dcaf53f 75ee100e ef77d458
                     fe531718 5584b895 665e3898 6045aa5a
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         4a26a496 3dcaf53f 75ee100e ef77d458
                     fe531718 5584b895 665e3898 6045aa5a
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4a26a496 3dcaf53f 75ee100e ef77d458
                     fe531718 5584b895 665e3898 6045aa5a
 8  post_state_root   373dea23 5abc253d 109c8367 c9bff524
                     9ff80587 48055132 c0485bf3 2d78792a
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         373dea23 5abc253d 109c8367 c9bff524
                     9ff80587 48055132 c0485bf3 2d78792a
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    373dea23 5abc253d 109c8367 c9bff524
                     9ff80587 48055132 c0485bf3 2d78792a
 8  post_state_root   ccdf4a3b 99da71cf 2ae4ffea c6a668e1
                     e164d6c6 219bf42f f9969cba 36676e10
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         ccdf4a3b 99da71cf 2ae4ffea c6a668e1
                     e164d6c6 219bf42f f9969cba 36676e10
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ccdf4a3b 99da71cf 2ae4ffea c6a668e1
                     e164d6c6 219bf42f f9969cba 36676e10
 8  post_state_root   4cbc165b a45d445d 2f5cc299 1fd41292
                     f2f24961 268f412d bc4bb487 2e48804f
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         4cbc165b a45d445d 2f5cc299 1fd41292
                     f2f24961 268f412d bc4bb487 2e48804f
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4cbc165b a45d445d 2f5cc299 1fd41292
                     f2f24961 268f412d bc4bb487 2e48804f
 8  post_state_root   f2706f05 9dc078b0 9404a663 15016a81
                     27530af7 eb5f0d1f fc8a0bb1 a0c2c7a6
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         f2706f05 9dc078b0 9404a663 15016a81
                     27530af7 eb5f0d1f fc8a0bb1 a0c2c7a6
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        6
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f2706f05 9dc078b0 9404a663 15016a81
                     27530af7 eb5f0d1f fc8a0bb1 a0c2c7a6
 8  post_state_root   cdcc8acd aae63f36 6511ad16 1cce9bf6
                     ddc56ab4 592bb499 2803d3f4 247daee5
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         cdcc8acd aae63f36 6511ad16 1cce9bf6
                     ddc56ab4 592bb499 2803d3f4 247daee5
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        7
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    cdcc8acd aae63f36 6511ad16 1cce9bf6
                     ddc56ab4 592bb499 2803d3f4 247daee5
 8  post_state_root   92f93cfb e259bd07 dffaa617 eab2bdef
                     78a2f33e 70b2b3dd 21b675a7 da9d6685
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         92f93cfb e259bd07 dffaa617 eab2bdef
                     78a2f33e 70b2b3dd 21b675a7 da9d6685
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        8
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    92f93cfb e259bd07 dffaa617 eab2bdef
                     78a2f33e 70b2b3dd 21b675a7 da9d6685
 8  post_state_root   16a83c57 86e723fc d0ef134e fe0b32da
                     dc6941ad 05c49b86 ee6e5b3b 71cfff5a
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         16a83c57 86e723fc d0ef134e fe0b32da
                     dc6941ad 05c49b86 ee6e5b3b 71cfff5a
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        9
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    16a83c57 86e723fc d0ef134e fe0b32da
                     dc6941ad 05c49b86 ee6e5b3b 71cfff5a
 8  post_state_root   f3e3390a b8a0cbd8 3672eaa7 5c77c888
                     8f2db9b9 1852f4d1 1f2719a8 a142ac9e
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         f3e3390a b8a0cbd8 3672eaa7 5c77c888
                     8f2db9b9 1852f4d1 1f2719a8 a142ac9e
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        10
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f3e3390a b8a0cbd8 3672eaa7 5c77c888
                     8f2db9b9 1852f4d1 1f2719a8 a142ac9e
 8  post_state_root   1771c061 ec0f65b3 730f66b7 3ded4e47
                     36b1684a 78f83564 adf2a57b 24c44896
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         1771c061 ec0f65b3 730f66b7 3ded4e47
                     36b1684a 78f83564 adf2a57b 24c44896
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        11
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1771c061 ec0f65b3 730f66b7 3ded4e47
                     36b1684a 78f83564 adf2a57b 24c44896
 8  post_state_root   6714c140 02bcf730 c59c85d4 1a3d11e1
                     84260e06 dcbe78f7 df96831f 2c80a749
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         6714c140 02bcf730 c59c85d4 1a3d11e1
                     84260e06 dcbe78f7 df96831f 2c80a749
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        12
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6714c140 02bcf730 c59c85d4 1a3d11e1
                     84260e06 dcbe78f7 df96831f 2c80a749
 8  post_state_root   5e9bc740 77f53d00 a42e6490 36cea0bc
                     d7762b2b fca56017 c4d686d1 b7334fe9
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — rejection (posts nothing)
 1  prev_root         5e9bc740 77f53d00 a42e6490 36cea0bc
                     d7762b2b fca56017 c4d686d1 b7334fe9
 2  program_root      92112576 39aeec20 ecc03220 f0134523
                     a0cfa886 07ac8634 ce7960c1 a889ba20
 3  step_index        13
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5e9bc740 77f53d00 a42e6490 36cea0bc
                     d7762b2b fca56017 c4d686d1 b7334fe9
 8  post_state_root   1c97abe6 85d23feb 6091cadd a24da543
                     9324c8e3 82c0b82c 09a2fd1b eff7cee2
 9  work_charged      0
10  work_refunded     4
11  clock             13
12  status            [[1, [13, 3, 0, 2, 3]], [3, [0, 0, 0, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [1, 0, 0, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [13, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00040203 87010000
  01010102 870d0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 68 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  72 /    0
    step  2:   28 /  72 /    0
    step  3:   32 /  72 /    0
    step  4:   36 /  72 /    0
    step  5:   40 /  72 /    0
    step  6:   44 /  72 /    0
    step  7:   48 /  72 /    0
    step  8:   52 /  72 /    0
    step  9:   56 /  72 /    0
    step 10:   60 /  72 /    0
    step 11:   64 /  72 /    0
    step 12:   68 /  72 /    0
    step 13:   68 /  72 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 72 = 68 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 68 = 32
    refund_final = B(P) - charged                       : 72 - 68 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X31 — `exec-combo-dom-noprom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_DOMAIN_ERROR) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {not promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 492 bytes; instruction stream entry a byte string of 51 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 59019800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088104
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      985ab8a0 774dc06f 5d13241d ad303550
                     e056e4b2 7393d8ab 2b823410 44956f86
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   ff9b6bbe 2417d96f a972abad 076054bd
                     f8cbe007 82fc0aab 79de5d75 1dae3ee7
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         ff9b6bbe 2417d96f a972abad 076054bd
                     f8cbe007 82fc0aab 79de5d75 1dae3ee7
 2  program_root      985ab8a0 774dc06f 5d13241d ad303550
                     e056e4b2 7393d8ab 2b823410 44956f86
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    ff9b6bbe 2417d96f a972abad 076054bd
                     f8cbe007 82fc0aab 79de5d75 1dae3ee7
 8  post_state_root   b0fc2acc 33c506a6 5c5ed373 138117bd
                     2754217b 59fb03b9 c5803b7a bfd6c7a6
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         b0fc2acc 33c506a6 5c5ed373 138117bd
                     2754217b 59fb03b9 c5803b7a bfd6c7a6
 2  program_root      985ab8a0 774dc06f 5d13241d ad303550
                     e056e4b2 7393d8ab 2b823410 44956f86
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b0fc2acc 33c506a6 5c5ed373 138117bd
                     2754217b 59fb03b9 c5803b7a bfd6c7a6
 8  post_state_root   1e5fa856 a1124603 c9aec515 04d708e2
                     63167e03 4e764567 6ef1f4ea 7cfb22ba
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         1e5fa856 a1124603 c9aec515 04d708e2
                     63167e03 4e764567 6ef1f4ea 7cfb22ba
 2  program_root      985ab8a0 774dc06f 5d13241d ad303550
                     e056e4b2 7393d8ab 2b823410 44956f86
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1e5fa856 a1124603 c9aec515 04d708e2
                     63167e03 4e764567 6ef1f4ea 7cfb22ba
 8  post_state_root   92c5b424 9d3f8c68 fa81919e 862ca08f
                     d2bca0ae 15b754ff b7798c44 8a624363
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         92c5b424 9d3f8c68 fa81919e 862ca08f
                     d2bca0ae 15b754ff b7798c44 8a624363
 2  program_root      985ab8a0 774dc06f 5d13241d ad303550
                     e056e4b2 7393d8ab 2b823410 44956f86
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    92c5b424 9d3f8c68 fa81919e 862ca08f
                     d2bca0ae 15b754ff b7798c44 8a624363
 8  post_state_root   81c5e811 4021248a 18ec1c89 53e85314
                     6728e512 3cc227ac 08e6d722 8feacdbb
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[3, [0, 0, 0, 0, 4]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00040203 87010000
  01010103 87020100 02010102 03850000
  00000404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X32 — `exec-combo-dom-noprom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {not promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 844 bytes; instruction stream entry a byte string of 95 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 5902f800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088104
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   2f70091b e1047de8 4535e5c5 0e458ff0
                     b7c83e62 97be01b5 ed75c425 19b156ab
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         2f70091b e1047de8 4535e5c5 0e458ff0
                     b7c83e62 97be01b5 ed75c425 19b156ab
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    2f70091b e1047de8 4535e5c5 0e458ff0
                     b7c83e62 97be01b5 ed75c425 19b156ab
 8  post_state_root   c757411d 3bc0e8ba 6ab8547e a8805a82
                     27213fed 906d04ef 22b22018 35fc7e47
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         c757411d 3bc0e8ba 6ab8547e a8805a82
                     27213fed 906d04ef 22b22018 35fc7e47
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c757411d 3bc0e8ba 6ab8547e a8805a82
                     27213fed 906d04ef 22b22018 35fc7e47
 8  post_state_root   7677e295 384a75ba b0ff3698 784e000f
                     0d3e2b5b c5ed65f2 aa749785 b12e7bb9
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         7677e295 384a75ba b0ff3698 784e000f
                     0d3e2b5b c5ed65f2 aa749785 b12e7bb9
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        4
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7677e295 384a75ba b0ff3698 784e000f
                     0d3e2b5b c5ed65f2 aa749785 b12e7bb9
 8  post_state_root   7a6cc327 d3ed7225 7a5703a7 c1801c0f
                     7f95671f c440cdce 5a6b7bdf 0ec05772
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         7a6cc327 d3ed7225 7a5703a7 c1801c0f
                     7f95671f c440cdce 5a6b7bdf 0ec05772
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        5
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7a6cc327 d3ed7225 7a5703a7 c1801c0f
                     7f95671f c440cdce 5a6b7bdf 0ec05772
 8  post_state_root   bfac25f2 e0592943 40ba277f 4c03eaaa
                     25aabaa2 9bbded12 c99c0024 91b7a4c9
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         bfac25f2 e0592943 40ba277f 4c03eaaa
                     25aabaa2 9bbded12 c99c0024 91b7a4c9
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        6
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    bfac25f2 e0592943 40ba277f 4c03eaaa
                     25aabaa2 9bbded12 c99c0024 91b7a4c9
 8  post_state_root   f716e77d 672fc00b 84263046 404e746e
                     f9a76f45 8671f993 a9315ccc 5fac7abf
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         f716e77d 672fc00b 84263046 404e746e
                     f9a76f45 8671f993 a9315ccc 5fac7abf
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        7
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f716e77d 672fc00b 84263046 404e746e
                     f9a76f45 8671f993 a9315ccc 5fac7abf
 8  post_state_root   550e3ef9 44f40b03 c2e43fb9 93d87497
                     e97a875b 24589246 34cb075a e4fcd885
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         550e3ef9 44f40b03 c2e43fb9 93d87497
                     e97a875b 24589246 34cb075a e4fcd885
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        8
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    550e3ef9 44f40b03 c2e43fb9 93d87497
                     e97a875b 24589246 34cb075a e4fcd885
 8  post_state_root   3a629d43 6bd7d282 c549585c da316774
                     c9e847a3 b1a9c9e9 9b816ba2 7d2959e5
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         3a629d43 6bd7d282 c549585c da316774
                     c9e847a3 b1a9c9e9 9b816ba2 7d2959e5
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        9
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3a629d43 6bd7d282 c549585c da316774
                     c9e847a3 b1a9c9e9 9b816ba2 7d2959e5
 8  post_state_root   1ba652cb 2dc0cdd8 9f301cdb 7ffe50bf
                     ecdd3b27 95911df3 383330f8 7bafb13b
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         1ba652cb 2dc0cdd8 9f301cdb 7ffe50bf
                     ecdd3b27 95911df3 383330f8 7bafb13b
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        10
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1ba652cb 2dc0cdd8 9f301cdb 7ffe50bf
                     ecdd3b27 95911df3 383330f8 7bafb13b
 8  post_state_root   5915705c aab30b4d 6bc3b313 097c9619
                     e0122f4f 2cd2fdd1 ab74cbdf 6b5ca589
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         5915705c aab30b4d 6bc3b313 097c9619
                     e0122f4f 2cd2fdd1 ab74cbdf 6b5ca589
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        11
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    5915705c aab30b4d 6bc3b313 097c9619
                     e0122f4f 2cd2fdd1 ab74cbdf 6b5ca589
 8  post_state_root   58b995f5 6620158a f3e4f4da 44ac46a2
                     41d0cd25 cfe8a8a6 a853ea32 8f4e05c2
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         58b995f5 6620158a f3e4f4da 44ac46a2
                     41d0cd25 cfe8a8a6 a853ea32 8f4e05c2
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        12
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    58b995f5 6620158a f3e4f4da 44ac46a2
                     41d0cd25 cfe8a8a6 a853ea32 8f4e05c2
 8  post_state_root   9fdc8b34 0c252054 b6774760 ea7b97d9
                     4b788393 4fd92480 7d2247b9 12d3f8b7
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         9fdc8b34 0c252054 b6774760 ea7b97d9
                     4b788393 4fd92480 7d2247b9 12d3f8b7
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        13
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9fdc8b34 0c252054 b6774760 ea7b97d9
                     4b788393 4fd92480 7d2247b9 12d3f8b7
 8  post_state_root   1ea04003 0e3a8a56 c0d4feae dfcc2b43
                     a6f915fb f449d2d8 61b4d8e3 afcc740b
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         1ea04003 0e3a8a56 c0d4feae dfcc2b43
                     a6f915fb f449d2d8 61b4d8e3 afcc740b
 2  program_root      c72bc8a6 ecba1425 08195e09 309c76c8
                     20a4e8fc f82cfcf5 f65a38b7 945a3ae7
 3  step_index        14
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1ea04003 0e3a8a56 c0d4feae dfcc2b43
                     a6f915fb f449d2d8 61b4d8e3 afcc740b
 8  post_state_root   08a40101 e8ece5af 28d40ce3 b86a12b5
                     5abcf96a a0d9ee2d 2e4d5ced 3e3845cb
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 4, 0, 3, 3]], [3, [0, 0, 0, 0, 4]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00040203 87010000
  01010103 87020100 02010102 870e0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X33 — `exec-combo-dom-noprom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {not promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 404 bytes; instruction stream entry a byte string of 40 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59014000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      09ad68e1 b10357ae ac15e9fd 32a154a1
                     4af72dba 9c43663c 63f934f8 683393c7
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 2  program_root      09ad68e1 b10357ae ac15e9fd 32a154a1
                     4af72dba 9c43663c 63f934f8 683393c7
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    83c083bb 26a33b5e 0bc1abe1 c4d2d0b7
                     14da825d 9d2240dc 1ee83337 6e4989d5
 8  post_state_root   eb337624 c5778cb3 8b87d323 1c18a50e
                     15611092 d6038a7a d4d15ac6 9d289c76
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         eb337624 c5778cb3 8b87d323 1c18a50e
                     15611092 d6038a7a d4d15ac6 9d289c76
 2  program_root      09ad68e1 b10357ae ac15e9fd 32a154a1
                     4af72dba 9c43663c 63f934f8 683393c7
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    eb337624 c5778cb3 8b87d323 1c18a50e
                     15611092 d6038a7a d4d15ac6 9d289c76
 8  post_state_root   eca26510 1318aeaa 4a9e98bb 8e6f540f
                     be875e02 f91674e3 93dff513 2feaf469
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 1, 1, 2, 4]], [3, [0, 0, 0, 0, 4]], [3, [1, 0, 0, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [1, 0, 0, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [3, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00040203 87010000
  01010103 87030101 02040203 03850301
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   28 /  36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X34 — `exec-combo-dom-prom-fbsuccess-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_DOMAIN_ERROR) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {promoted} × {fallback success} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 388 bytes; instruction stream entry a byte string of 38 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59013000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      eef02664 c64c81e5 bf46ec0c 55883c7a
                     14d5fdbd 7e4d0b54 adccd33b b5d7136b
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 2  program_root      eef02664 c64c81e5 bf46ec0c 55883c7a
                     14d5fdbd 7e4d0b54 adccd33b b5d7136b
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 8  post_state_root   e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 2  program_root      eef02664 c64c81e5 bf46ec0c 55883c7a
                     14d5fdbd 7e4d0b54 adccd33b b5d7136b
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 8  post_state_root   055b5a77 15103e1d b65c15ae f727bf42
                     77d04be8 88ab2cb5 582076ab 9fb8417b
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         055b5a77 15103e1d b65c15ae f727bf42
                     77d04be8 88ab2cb5 582076ab 9fb8417b
 2  program_root      eef02664 c64c81e5 bf46ec0c 55883c7a
                     14d5fdbd 7e4d0b54 adccd33b b5d7136b
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    055b5a77 15103e1d b65c15ae f727bf42
                     77d04be8 88ab2cb5 582076ab 9fb8417b
 8  post_state_root   1f9cbaf8 48ef2d62 94cb1258 42434ef0
                     1248cdc2 ab4acc0d 0ef75769 1fe35ef1
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         1f9cbaf8 48ef2d62 94cb1258 42434ef0
                     1248cdc2 ab4acc0d 0ef75769 1fe35ef1
 2  program_root      eef02664 c64c81e5 bf46ec0c 55883c7a
                     14d5fdbd 7e4d0b54 adccd33b b5d7136b
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    1f9cbaf8 48ef2d62 94cb1258 42434ef0
                     1248cdc2 ab4acc0d 0ef75769 1fe35ef1
 8  post_state_root   e75ad1f9 94b102bf 51895ec7 884ea80a
                     4d0c3a0b f101d355 00721fe5 ecf79395
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[3, [0, 0, 0, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040282 87000000 00040203 87020001
  01010102 03850000 00000404 83707630
  2e332e30 2d647261 66742e38 2e346971
  69722d65 6e632d32 71716972 2d70726f
  662d676f 6c64656e 2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X35 — `exec-combo-dom-prom-fbsuccess-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {promoted} × {fallback success} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 2
block 2 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=2 ref=3 rng=[-1, 1]
  3: BR cc=NE ref=4 true=block 3 false=block 4
block 3 (arity 0):
  0: JMP block 2
block 4 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 740 bytes; instruction stream entry a byte string of 82 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050506 59029000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000400
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000000
  00000000 00000a00 00000000 00000200
  00000000 00000300 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000b00 00000000 00000300
  00000000 00000400 00000000 00000300
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: one back-edge (block 3 to block 2, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback, block 2, the true edge, block 3, block 2 again, the false edge, and block 4 — 14 transitions (56 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (14 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   b0b20276 e28b8000 8b44e600 3341df04
                     9da818ac c5b698fc 97067933 c2dbfe01
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         b0b20276 e28b8000 8b44e600 3341df04
                     9da818ac c5b698fc 97067933 c2dbfe01
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    b0b20276 e28b8000 8b44e600 3341df04
                     9da818ac c5b698fc 97067933 c2dbfe01
 8  post_state_root   ab13892e c4605cf3 9f886c34 8f18bb62
                     065aefed 566d69e0 c5790c9e bc7984b6
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         ab13892e c4605cf3 9f886c34 8f18bb62
                     065aefed 566d69e0 c5790c9e bc7984b6
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ab13892e c4605cf3 9f886c34 8f18bb62
                     065aefed 566d69e0 c5790c9e bc7984b6
 8  post_state_root   39f93944 914a8568 3d09afe8 b2b860ff
                     4196f6b1 5efc11d3 6e3e36c0 7f662fb1
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         39f93944 914a8568 3d09afe8 b2b860ff
                     4196f6b1 5efc11d3 6e3e36c0 7f662fb1
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    39f93944 914a8568 3d09afe8 b2b860ff
                     4196f6b1 5efc11d3 6e3e36c0 7f662fb1
 8  post_state_root   81786452 513c00d1 8f35bc69 ded48914
                     b637d09f 5dfa5197 bb0bcca4 346b05cf
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            [[2, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         81786452 513c00d1 8f35bc69 ded48914
                     b637d09f 5dfa5197 bb0bcca4 346b05cf
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        5
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    81786452 513c00d1 8f35bc69 ded48914
                     b637d09f 5dfa5197 bb0bcca4 346b05cf
 8  post_state_root   7cb9ce0f ec70d592 ad08b68e 6232d8ef
                     d3633c1a 8a65a28f 558fedd8 e8497f88
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         7cb9ce0f ec70d592 ad08b68e 6232d8ef
                     d3633c1a 8a65a28f 558fedd8 e8497f88
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        6
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7cb9ce0f ec70d592 ad08b68e 6232d8ef
                     d3633c1a 8a65a28f 558fedd8 e8497f88
 8  post_state_root   b5fdb0bd fee79490 1613d5c2 590d88ed
                     8d4cff16 ed021147 cf46539f 99ac6957
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         b5fdb0bd fee79490 1613d5c2 590d88ed
                     8d4cff16 ed021147 cf46539f 99ac6957
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        7
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b5fdb0bd fee79490 1613d5c2 590d88ed
                     8d4cff16 ed021147 cf46539f 99ac6957
 8  post_state_root   7d1df88d 97096453 6fc21659 19fe2909
                     31ffd304 6e0ab74a 2a060e7f dd118ecc
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         7d1df88d 97096453 6fc21659 19fe2909
                     31ffd304 6e0ab74a 2a060e7f dd118ecc
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        8
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7d1df88d 97096453 6fc21659 19fe2909
                     31ffd304 6e0ab74a 2a060e7f dd118ecc
 8  post_state_root   e7edffe2 810a8fd3 d219774e 95201605
                     6126f772 2698cc0c ec8aabd8 2b6cdeb8
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         e7edffe2 810a8fd3 d219774e 95201605
                     6126f772 2698cc0c ec8aabd8 2b6cdeb8
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        9
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e7edffe2 810a8fd3 d219774e 95201605
                     6126f772 2698cc0c ec8aabd8 2b6cdeb8
 8  post_state_root   223bc4f4 1488aef3 6b8b2705 80322466
                     d0feecd6 7ca2ca42 769da42d 8f998624
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         223bc4f4 1488aef3 6b8b2705 80322466
                     d0feecd6 7ca2ca42 769da42d 8f998624
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        10
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    223bc4f4 1488aef3 6b8b2705 80322466
                     d0feecd6 7ca2ca42 769da42d 8f998624
 8  post_state_root   8e34330a 55326d12 501f5b5d e1cb843c
                     eed44772 5c9c71ca cdaf5f37 28fd9652
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         8e34330a 55326d12 501f5b5d e1cb843c
                     eed44772 5c9c71ca cdaf5f37 28fd9652
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        11
 4  pc                [2, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8e34330a 55326d12 501f5b5d e1cb843c
                     eed44772 5c9c71ca cdaf5f37 28fd9652
 8  post_state_root   2cbda997 ad6f877e 344b6874 6ff52967
                     81f08b9c 9c0576b3 f053113a e0652ce2
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         2cbda997 ad6f877e 344b6874 6ff52967
                     81f08b9c 9c0576b3 f053113a e0652ce2
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        12
 4  pc                [2, 2]
 5  decoded_word      000000000000000a 0000000000000002 0000000000000003
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    2cbda997 ad6f877e 344b6874 6ff52967
                     81f08b9c 9c0576b3 f053113a e0652ce2
 8  post_state_root   03216e49 aa647018 1e5e5eaf 9d988cd4
                     8144b763 872a37fd 77b95e02 b62bf29a
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         03216e49 aa647018 1e5e5eaf 9d988cd4
                     8144b763 872a37fd 77b95e02 b62bf29a
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        13
 4  pc                [2, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000004
                     0000000000000003 0000000000000000 0000000000000004
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    03216e49 aa647018 1e5e5eaf 9d988cd4
                     8144b763 872a37fd 77b95e02 b62bf29a
 8  post_state_root   75de2201 716b2550 adb4b0cf 1a20f4e0
                     8078b2a8 8b34b4d8 94f8c7e7 88826987
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — rejection (posts nothing)
 1  prev_root         75de2201 716b2550 adb4b0cf 1a20f4e0
                     8078b2a8 8b34b4d8 94f8c7e7 88826987
 2  program_root      4b100b98 b6cb3f7f 20631d43 6ab9cc7d
                     e3acabb5 e202480a fe3249e2 76fbfad9
 3  step_index        14
 4  pc                [3, 0]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    75de2201 716b2550 adb4b0cf 1a20f4e0
                     8078b2a8 8b34b4d8 94f8c7e7 88826987
 8  post_state_root   a72526b3 b17f67c4 3f129889 b90dd077
                     c439d9f6 5e12b522 7d87747b e6ad0e88
 9  work_charged      0
10  work_refunded     4
11  clock             14
12  status            [[1, [14, 3, 0, 2, 3]], [3, [0, 0, 0, 0, 4]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [2, 0, 1, 1, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [14, 3, 0, 2, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030283 87000000 00040203 87020001
  01010102 870e0300 02030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 72 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  76 /    0
    step  2:   28 /  76 /    0
    step  3:   32 /  76 /    0
    step  4:   36 /  76 /    0
    step  5:   40 /  76 /    0
    step  6:   44 /  76 /    0
    step  7:   48 /  76 /    0
    step  8:   52 /  76 /    0
    step  9:   56 /  76 /    0
    step 10:   60 /  76 /    0
    step 11:   64 /  76 /    0
    step 12:   68 /  76 /    0
    step 13:   72 /  76 /    0
    step 14:   72 /  76 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 76 = 72 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 72 = 28
    refund_final = B(P) - charged                       : 76 - 72 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X36 — `exec-combo-dom-prom-fbremiss-halt`

Cites: §2.5 total terminalization function T, case (2) — the earliest-birth outstanding Tier S token (STATUS_DOMAIN_ERROR) decides at the HALT exit; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 tier rule (the token is consumed when the fallback entry completes successfully); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {promoted} × {fallback repeated miss} × {HALT}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 508 bytes; instruction stream entry a byte string of 53 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 5901a800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088104
```

Admission: no back-edges; the maximum path is 6 transitions (24 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (6 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   f21db261 de97cce1 0760a508 f5f9346f
                     0328ade9 d84e6efa 21faa195 2a225e42
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         f21db261 de97cce1 0760a508 f5f9346f
                     0328ade9 d84e6efa 21faa195 2a225e42
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    f21db261 de97cce1 0760a508 f5f9346f
                     0328ade9 d84e6efa 21faa195 2a225e42
 8  post_state_root   21c60a68 1b7c5cd9 ce93a767 d7ec9a6c
                     cdcc7bf6 83693d12 c16ebadf 05173c5c
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         21c60a68 1b7c5cd9 ce93a767 d7ec9a6c
                     cdcc7bf6 83693d12 c16ebadf 05173c5c
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    21c60a68 1b7c5cd9 ce93a767 d7ec9a6c
                     cdcc7bf6 83693d12 c16ebadf 05173c5c
 8  post_state_root   59b75d50 f387ff9d ef467d10 30e43848
                     bee8a06d 8ec7fd41 6a295da1 8e73cb2b
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         59b75d50 f387ff9d ef467d10 30e43848
                     bee8a06d 8ec7fd41 6a295da1 8e73cb2b
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    59b75d50 f387ff9d ef467d10 30e43848
                     bee8a06d 8ec7fd41 6a295da1 8e73cb2b
 8  post_state_root   0080ae89 a6c6a85e 9b1e24bb 56acfd79
                     e9742f3d c43eaf10 e48d2fd7 bd36627e
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         0080ae89 a6c6a85e 9b1e24bb 56acfd79
                     e9742f3d c43eaf10 e48d2fd7 bd36627e
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0080ae89 a6c6a85e 9b1e24bb 56acfd79
                     e9742f3d c43eaf10 e48d2fd7 bd36627e
 8  post_state_root   69b2657f 3ba4462f 5b257c2c c197cf6a
                     685c6433 f979e3bb b6c17acc fe5f022a
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         69b2657f 3ba4462f 5b257c2c c197cf6a
                     685c6433 f979e3bb b6c17acc fe5f022a
 2  program_root      5fa19fc4 257011dc 4516767c bd7b8905
                     385d46ee 23a43671 7928d88f 3bfda9a1
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    69b2657f 3ba4462f 5b257c2c c197cf6a
                     685c6433 f979e3bb b6c17acc fe5f022a
 8  post_state_root   f8c78990 651717e5 f8103fd6 810937be
                     ce2c20a9 62197390 9cd59a73 f802073c
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            [[3, [0, 0, 0, 0, 4]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   [0, 0, 0, 0, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00040203 87020001
  01010103 87030100 02010102 03850000
  00000404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 44 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        8 U (2 postings)
    q_promote     4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  44 /    0
    step  2:   28 /  44 /    0
    step  3:   32 /  44 /    0
    step  4:   36 /  44 /    0
    step  5:   40 /  44 /    0
    step  6:   44 /  44 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 44 = 44 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 44 = 56
    refund_final = B(P) - charged                       : 44 - 44 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X37 — `exec-combo-dom-prom-fbremiss-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.5 tier rule (F12 — a fallback that misses again leaves the earlier token outstanding and births a fresh miss token with its own birth site); §2.3 BR/JMP rejections (an exhausted declared trip ceiling on a back-edge) and §2.3 derived counters; §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {promoted} × {fallback repeated miss} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 3
block 1 (arity 0):
  0: CLOAD addr=0 fallback=block 2 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 3 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CONST imm=0 bnd=(int, <=0)
  2: CMP ref=3 ref=4 rng=[-1, 1]
  3: BR cc=NE ref=5 true=block 4 false=block 5
block 4 (arity 0):
  0: JMP block 3
block 5 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 860 bytes; instruction stream entry a byte string of 97 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050606 59030800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000400 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000a00
  00000000 00000300 00000000 00000400
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000b00
  00000000 00000300 00000000 00000500
  00000000 00000400 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000400
  00000000 00000000 00000000 00000100
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000500 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 80088104
```

Admission: one back-edge (block 4 to block 3, whose target dominates its source) at the declared trip ceiling 1; the maximum bounded path is block 0, the fallback chain, block 3, the true edge, block 4, block 3 again, the false edge, and block 5 — 15 transitions (60 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (15 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   fb8f4bb7 c8a3a47a e7dd9aa1 781fe412
                     f2c5a16b 139b68c1 543083f4 b70bd090
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         fb8f4bb7 c8a3a47a e7dd9aa1 781fe412
                     f2c5a16b 139b68c1 543083f4 b70bd090
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    fb8f4bb7 c8a3a47a e7dd9aa1 781fe412
                     f2c5a16b 139b68c1 543083f4 b70bd090
 8  post_state_root   dbf116a9 1fd685a2 e578cacb 4d165875
                     f7536f0c 45265b1e 56bac333 1923bf40
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         dbf116a9 1fd685a2 e578cacb 4d165875
                     f7536f0c 45265b1e 56bac333 1923bf40
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000002
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    dbf116a9 1fd685a2 e578cacb 4d165875
                     f7536f0c 45265b1e 56bac333 1923bf40
 8  post_state_root   9352df7c eee482c5 7cc28948 e6c6a355
                     9e88b7b6 e995f293 c1b6217e 3912e1b7
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[1, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         9352df7c eee482c5 7cc28948 e6c6a355
                     9e88b7b6 e995f293 c1b6217e 3912e1b7
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        4
 4  pc                [2, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9352df7c eee482c5 7cc28948 e6c6a355
                     9e88b7b6 e995f293 c1b6217e 3912e1b7
 8  post_state_root   47595e38 73120a1b 13d6483a d990c535
                     c7fa3723 15936b99 4348b7d5 416380a2
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         47595e38 73120a1b 13d6483a d990c535
                     c7fa3723 15936b99 4348b7d5 416380a2
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        5
 4  pc                [2, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    47595e38 73120a1b 13d6483a d990c535
                     c7fa3723 15936b99 4348b7d5 416380a2
 8  post_state_root   3c40a46d 35bcf9f3 d44d03a7 fa765d13
                     2522b7ec c50b1bc0 19e67dd8 b341656a
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            [[2, [3, 1, 0, 2, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         3c40a46d 35bcf9f3 d44d03a7 fa765d13
                     2522b7ec c50b1bc0 19e67dd8 b341656a
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        6
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    3c40a46d 35bcf9f3 d44d03a7 fa765d13
                     2522b7ec c50b1bc0 19e67dd8 b341656a
 8  post_state_root   987f0e76 9f384e3f 51ed3664 154b9056
                     11a6524d da1987de ae142955 fa7b31e0
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         987f0e76 9f384e3f 51ed3664 154b9056
                     11a6524d da1987de ae142955 fa7b31e0
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        7
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    987f0e76 9f384e3f 51ed3664 154b9056
                     11a6524d da1987de ae142955 fa7b31e0
 8  post_state_root   c4eb06a1 7bcc6fa3 2e8d0e86 89771063
                     8f0ae87e a89ba5ef e540b56b 7205a2b9
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         c4eb06a1 7bcc6fa3 2e8d0e86 89771063
                     8f0ae87e a89ba5ef e540b56b 7205a2b9
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        8
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c4eb06a1 7bcc6fa3 2e8d0e86 89771063
                     8f0ae87e a89ba5ef e540b56b 7205a2b9
 8  post_state_root   149b691a 60013f53 f324a50c e3f5f638
                     699f5430 73fc539f 0ed42504 9fa5dbb1
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 9 — completed
 1  prev_root         149b691a 60013f53 f324a50c e3f5f638
                     699f5430 73fc539f 0ed42504 9fa5dbb1
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        9
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    149b691a 60013f53 f324a50c e3f5f638
                     699f5430 73fc539f 0ed42504 9fa5dbb1
 8  post_state_root   9987b847 b698da4f f5155ec0 3819b911
                     cacddd07 80107f50 070ff4a7 010b409f
 9  work_charged      4
10  work_refunded     0
11  clock             9
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 10 — completed
 1  prev_root         9987b847 b698da4f f5155ec0 3819b911
                     cacddd07 80107f50 070ff4a7 010b409f
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        10
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9987b847 b698da4f f5155ec0 3819b911
                     cacddd07 80107f50 070ff4a7 010b409f
 8  post_state_root   20c1598b 6041acc7 22074ca2 7cc7a346
                     6b1a961f e2343fba 2ebe03ac 66c90e0f
 9  work_charged      4
10  work_refunded     0
11  clock             10
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 11 — completed
 1  prev_root         20c1598b 6041acc7 22074ca2 7cc7a346
                     6b1a961f e2343fba 2ebe03ac 66c90e0f
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        11
 4  pc                [3, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    20c1598b 6041acc7 22074ca2 7cc7a346
                     6b1a961f e2343fba 2ebe03ac 66c90e0f
 8  post_state_root   f548f78a fe621703 ecf8a81c efa83b9f
                     95e9a4a2 2f54354b 1e7fce54 70469265
 9  work_charged      4
10  work_refunded     0
11  clock             11
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 12 — completed
 1  prev_root         f548f78a fe621703 ecf8a81c efa83b9f
                     95e9a4a2 2f54354b 1e7fce54 70469265
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        12
 4  pc                [3, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f548f78a fe621703 ecf8a81c efa83b9f
                     95e9a4a2 2f54354b 1e7fce54 70469265
 8  post_state_root   9382819a a780a407 ee1f3ce8 fc4c675b
                     3f8a8fa0 4e00726b 612dc4c4 77945a2d
 9  work_charged      4
10  work_refunded     0
11  clock             12
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 13 — completed
 1  prev_root         9382819a a780a407 ee1f3ce8 fc4c675b
                     3f8a8fa0 4e00726b 612dc4c4 77945a2d
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        13
 4  pc                [3, 2]
 5  decoded_word      000000000000000a 0000000000000003 0000000000000004
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9382819a a780a407 ee1f3ce8 fc4c675b
                     3f8a8fa0 4e00726b 612dc4c4 77945a2d
 8  post_state_root   8dfca642 7318a59f 7d0c9008 ab9ba265
                     56aa328b ef21756f 5ed50e8f b2ad0589
 9  work_charged      4
10  work_refunded     0
11  clock             13
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 14 — completed
 1  prev_root         8dfca642 7318a59f 7d0c9008 ab9ba265
                     56aa328b ef21756f 5ed50e8f b2ad0589
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        14
 4  pc                [3, 3]
 5  decoded_word      000000000000000b 0000000000000003 0000000000000005
                     0000000000000004 0000000000000000 0000000000000005
                     0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8dfca642 7318a59f 7d0c9008 ab9ba265
                     56aa328b ef21756f 5ed50e8f b2ad0589
 8  post_state_root   18739035 aacb7c22 7bb6867b 6e44e0f3
                     00511680 44dc8617 447f0b94 e64e0d87
 9  work_charged      4
10  work_refunded     0
11  clock             14
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 15 — rejection (posts nothing)
 1  prev_root         18739035 aacb7c22 7bb6867b 6e44e0f3
                     00511680 44dc8617 447f0b94 e64e0d87
 2  program_root      46c4ee1a a0791471 252d7e77 c103ec2e
                     2d1a4577 be5ee8cd ebca7516 0e234d3c
 3  step_index        15
 4  pc                [4, 0]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    18739035 aacb7c22 7bb6867b 6e44e0f3
                     00511680 44dc8617 447f0b94 e64e0d87
 8  post_state_root   22f4eb41 199aaf8a 650e283e ebcbe4eb
                     316bb40a ac498866 1d86a7ac 8ecb3e49
 9  work_charged      0
10  work_refunded     4
11  clock             15
12  status            [[1, [15, 4, 0, 3, 3]], [3, [0, 0, 0, 0, 4]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [3, 1, 0, 2, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
      [15, 4, 0, 3, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030284 87000000 00040203 87020001
  01010103 87030100 02010102 870f0400
  03030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 76 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      20 U (5 postings)
    q_load        8 U (2 postings)
    q_jmp         8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_br          8 U (2 postings)
    q_promote     4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  80 /    0
    step  2:   28 /  80 /    0
    step  3:   32 /  80 /    0
    step  4:   36 /  80 /    0
    step  5:   40 /  80 /    0
    step  6:   44 /  80 /    0
    step  7:   48 /  80 /    0
    step  8:   52 /  80 /    0
    step  9:   56 /  80 /    0
    step 10:   60 /  80 /    0
    step 11:   64 /  80 /    0
    step 12:   68 /  80 /    0
    step 13:   72 /  80 /    0
    step 14:   76 /  80 /    0
    step 15:   76 /  80 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 80 = 76 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 76 = 24
    refund_final = B(P) - charged                       : 80 - 76 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X38 — `exec-combo-dom-prom-fbreject-tbirth`

Cites: §2.5 total terminalization function T, case (1) — a terminalizing birth exits at its own transition with the birthed token's outcome; §2.5 declared initial status pool (the conformance birth path — the outstanding STATUS_DOMAIN_ERROR token); §2.3 CLOAD row (the miss births STATUS_LOOKUP_MISS and routes to the declared fallback edge); §2.3 DIV row (the zero-divisor STATUS_DOMAIN_ERROR birth, terminalizing; the fallback entry ends in a typed rejection); §2.3 PROMOTE row (the valid promotion consumes the earliest-birth outstanding STATUS_NEG_EVEN_ROOT; none outstanding: no token effect); §2.5 T case (3) itemization at every exit under the declared reading DR1; the selected token under DR2; §4.4 F12 combination closure — {STATUS_DOMAIN_ERROR} × {promoted} × {fallback typed rejection} × {terminalizing birth}.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is [4] (§2.5 conformance birth path, at most one token per declared type); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: PROMOTE
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: DIV ref=1 ref=1 bnd=(int, <=9)
  2: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 420 bytes; instruction stream entry a byte string of 42 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59015000
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000e00
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000300
  00000000 00000100 00000000 00000100
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000000 00000000 00000800
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  80088104
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **DOMAIN_ERROR** (tag 4).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 2  program_root      d1dd2d63 f755228c 1c6a800e 032b66c4
                     918661c3 ce870d72 46da7125 9274f0db
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      000000000000000e 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    dba021d1 d605eb04 13e9f3e2 db4752b2
                     2737dc36 049f2441 a8c57731 399b778f
 8  post_state_root   be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 2  program_root      d1dd2d63 f755228c 1c6a800e 032b66c4
                     918661c3 ce870d72 46da7125 9274f0db
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    be404a00 7284b542 af5333e5 0be05a92
                     8342e952 b98f9383 aef4bea7 6506b8c1
 8  post_state_root   e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            [[1, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 2  program_root      d1dd2d63 f755228c 1c6a800e 032b66c4
                     918661c3 ce870d72 46da7125 9274f0db
 3  step_index        3
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000000
                     0000000000000001 0000000000000001 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e64c1682 61496c1b 0c5bcd3f bae6ca1a
                     932e4314 a3d420bd da712639 25390304
 8  post_state_root   ef69ca6d 4ac73527 a9f9e514 2d0b7658
                     35e400c2 8947d21e a50d905a 1aa8aa7e
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — rejection (posts nothing)
 1  prev_root         ef69ca6d 4ac73527 a9f9e514 2d0b7658
                     35e400c2 8947d21e a50d905a 1aa8aa7e
 2  program_root      d1dd2d63 f755228c 1c6a800e 032b66c4
                     918661c3 ce870d72 46da7125 9274f0db
 3  step_index        4
 4  pc                [1, 1]
 5  decoded_word      0000000000000008 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    ef69ca6d 4ac73527 a9f9e514 2d0b7658
                     35e400c2 8947d21e a50d905a 1aa8aa7e
 8  post_state_root   893a6621 8eb9ba09 55a70c90 b4ea85df
                     9ec916e1 77115dd7 b70f7ec7 ffb102af
 9  work_charged      0
10  work_refunded     8
11  clock             4
12  status            [[1, [4, 1, 1, 2, 4]], [3, [0, 0, 0, 0, 4]], [3, [2, 0, 1, 1, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      4 (DOMAIN_ERROR)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [0, 0, 0, 0, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
      [2, 0, 1, 1, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [4, 1, 1, 2, 4, 2, 3]   # STATUS_DOMAIN_ERROR tier S state terminalized
  3  selected_token   [4, 1, 1, 2, 4]   # the outcome-determining token (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01040283 87000000 00040203 87020001
  01010103 87040101 02040203 03850401
  01020404 83707630 2e332e30 2d647261
  66742e38 2e346971 69722d65 6e632d32
  71716972 2d70726f 662d676f 6c64656e
  2d32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_promote     4 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   32 /  40 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 32 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 40 - 32 = 8
    reserve_outstanding after terminal settlement       : 0
```

### X39 — `exec-cload-hit-success`

Cites: §2.3 CLOAD row (hit: binds `D`'s canonical table value to a fresh instance; fall-through); §2.1 (the declared dependency environment, each entry cited by content address) and §2.2 (the dynamic value store component of the canonical configuration); §2.4 (redefinition mints, never overwrites — the fresh instance); §4.4 Suite Register bullet 2 — the CLOAD-hit surface over a populated store; declared parameter P12.

Declared parameters: `W_budget` = 100 U; `D` = { addr 0 → 7 }; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency table rides the declared rendering parameter P12 (the constant-table entry of the declared content address h'0000000000000001', encoded per §2.6).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 355 bytes; instruction stream entry a byte string of 34 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59011000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  800880
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 8  post_state_root   c5eaf559 f4fc6aad 7f1be0a3 50df1907
                     62dbc5ee 17741a11 8cc88460 d77f64ed
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         c5eaf559 f4fc6aad 7f1be0a3 50df1907
                     62dbc5ee 17741a11 8cc88460 d77f64ed
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c5eaf559 f4fc6aad 7f1be0a3 50df1907
                     62dbc5ee 17741a11 8cc88460 d77f64ed
 8  post_state_root   f3414963 d1ea2244 090f2826 ecf83630
                     736edf2c e9cf854c 2b7f7eb7 5ddfb675
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  32 /    0
    step  2:   28 /  32 /    0
    step  3:   32 /  32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X40 — `exec-cload-hit-boundary`

Cites: §2.3 CLOAD row (hit value exceeds its declared bound: typed rejection before dispatch — the value equal to its bound does not exceed it and is admitted); §2.4 instance minting; §4.4 Suite Register bullet 2 — the bound check's boundary; declared parameter P12.

Declared parameters: `W_budget` = 100 U; `D` = { addr 0 → 9 }; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency table rides the declared rendering parameter P12 (the constant-table entry of the declared content address h'0000000000000001', encoded per §2.6).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 355 bytes; instruction stream entry a byte string of 34 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59011000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  800880
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   423d2e45 7c0f7d83 45db2f41 6b5a4641
                     04c7d0f1 7aceb010 5a9f84a1 e20f9b88
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         423d2e45 7c0f7d83 45db2f41 6b5a4641
                     04c7d0f1 7aceb010 5a9f84a1 e20f9b88
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      000000000000000c 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    423d2e45 7c0f7d83 45db2f41 6b5a4641
                     04c7d0f1 7aceb010 5a9f84a1 e20f9b88
 8  post_state_root   4b8250ee 7d815fb9 d5758891 8c1326af
                     9ec411bc 341f305a 93356ede 640bbf06
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         4b8250ee 7d815fb9 d5758891 8c1326af
                     9ec411bc 341f305a 93356ede 640bbf06
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        3
 4  pc                [2, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    4b8250ee 7d815fb9 d5758891 8c1326af
                     9ec411bc 341f305a 93356ede 640bbf06
 8  post_state_root   a0d5df32 9708810c c3e77c81 93fd696e
                     848a29e9 78ce6fc1 143f7989 77b3ecd5
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 32 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  32 /    0
    step  2:   28 /  32 /    0
    step  3:   32 /  32 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 32 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 32 = 68
    refund_final = B(P) - charged                       : 32 - 32 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X41 — `exec-cload-hit-bound-rejection`

Cites: §2.3 CLOAD row (hit value exceeds its declared bound: typed rejection before dispatch — no binding, no store effect); §2.5 failure behavior (the typed rejection is fatal, births EXPRESSION_TOO_COMPLEX, terminalizing at its own transition) and T case (1); §2.6 receipt key 3 (the selected token's identifier rides `NO_SOLUTION`/`DOMAIN_ERROR` exits only — nil at an `EXPRESSION_TOO_COMPLEX` exit, the X10 shape); §3.3 with P11 (the rejection posts nothing; the escrow is open — the `CONST` completed first — so the terminal settlement's refund rides the rejection record, the X5 precedent); §4.4 Suite Register bullet 2; declared parameter P12.

Declared parameters: `W_budget` = 100 U; `D` = { addr 0 → 10 }; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency table rides the declared rendering parameter P12 (the constant-table entry of the declared content address h'0000000000000001', encoded per §2.6).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=1 bnd=(int, <=1)
  1: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  2: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 427 bytes; instruction stream entry a byte string of 43 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59015800
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00000c00 00000000 00000200
  00000000 00000000 00000000 00000000
  00000000 00000100 00000000 00000000
  00000000 00000200 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000900 00000000 00000000
  00000000 00001000 00000000 00000000
  00000000 00000200 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 800880
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (2 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      0e28e820 05fc19d1 e714be9a a94c4ef8
                     f353ff40 43524826 95b72a90 6edaa565
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   fb212711 0642d60f b9015753 57b4c3ea
                     4502b14b 8dbc58aa 37718ce8 ac84879e
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — rejection (posts nothing)
 1  prev_root         fb212711 0642d60f b9015753 57b4c3ea
                     4502b14b 8dbc58aa 37718ce8 ac84879e
 2  program_root      0e28e820 05fc19d1 e714be9a a94c4ef8
                     f353ff40 43524826 95b72a90 6edaa565
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    fb212711 0642d60f b9015753 57b4c3ea
                     4502b14b 8dbc58aa 37718ce8 ac84879e
 8  post_state_root   b11b76c5 01c3592b 494399c2 60f4dcf2
                     ac32de82 db5eb0b0 39e7d134 8a3eddfb
 9  work_charged      0
10  work_refunded     12
11  clock             2
12  status            [[1, [2, 0, 1, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [2, 0, 1, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87020001 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 24 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   24 /  36 /   12
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 24 + 12 + 0
    remaining_budget = W_budget - charged               : 100 - 24 = 76
    refund_final = B(P) - charged                       : 36 - 24 = 12
    reserve_outstanding after terminal settlement       : 0
```
### X42 — `exec-cload-miss-multiblock-completion`

Cites: §2.3 CLOAD row (miss: births `STATUS_LOOKUP_MISS`, routes to the declared fallback edge); §2.5 tier rule under the declared reading DR3 (the fallback entry completes successfully at the fallback block's final instruction — the consume event rides that transition's record whether the block ends in `HALT`, the seed's X6 shape, or in a control transfer; a fallback block that rejects never consumes); §4.4 Suite Register bullet 2 — the multi-block fallback completion vector fixing the consumption site beyond a HALT-terminated fallback block.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency table rides the declared rendering parameter P12 (the constant-table entry of the declared content address h'0000000000000001', encoded per §2.6).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: JMP block 3
block 2 (arity 0):
  0: ADD ref=0 ref=0 bnd=(int, <=18)
  1: JMP block 3
block 3 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 491 bytes; instruction stream entry a byte string of 51 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050406 59019800
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000300 00000000 00000000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000200
  00000000 00000500 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00001200 00000000 00000000
  00000000 00000c00 00000000 00000300
  00000000 00000000 00000000 00000000
  00000000 00000300 00000000 00000000
  00000000 00000100 00000000 00001000
  00000000 00000007 800880
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      a0ac59c4 65712a24 e38e337e b7a0d3c1
                     48ca57ef 65dd31ae 5a60128a ad9697c4
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   415bf98f 71cd585d edf897a4 a3b33173
                     6c72356b e2951ea1 dfb43168 9639572f
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         415bf98f 71cd585d edf897a4 a3b33173
                     6c72356b e2951ea1 dfb43168 9639572f
 2  program_root      a0ac59c4 65712a24 e38e337e b7a0d3c1
                     48ca57ef 65dd31ae 5a60128a ad9697c4
 3  step_index        2
 4  pc                [1, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    415bf98f 71cd585d edf897a4 a3b33173
                     6c72356b e2951ea1 dfb43168 9639572f
 8  post_state_root   2a1f5c11 91176b00 ccae57be 7d904fa0
                     37be219c 56de5966 20231ea0 1061c7cf
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         2a1f5c11 91176b00 ccae57be 7d904fa0
                     37be219c 56de5966 20231ea0 1061c7cf
 2  program_root      a0ac59c4 65712a24 e38e337e b7a0d3c1
                     48ca57ef 65dd31ae 5a60128a ad9697c4
 3  step_index        3
 4  pc                [1, 1]
 5  decoded_word      000000000000000c 0000000000000003 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    2a1f5c11 91176b00 ccae57be 7d904fa0
                     37be219c 56de5966 20231ea0 1061c7cf
 8  post_state_root   827a171c a800a830 266e558e eba6b975
                     6369c9e8 41dbd9e5 97a441a4 b54d8367
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            [[2, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         827a171c a800a830 266e558e eba6b975
                     6369c9e8 41dbd9e5 97a441a4 b54d8367
 2  program_root      a0ac59c4 65712a24 e38e337e b7a0d3c1
                     48ca57ef 65dd31ae 5a60128a ad9697c4
 3  step_index        4
 4  pc                [3, 0]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    827a171c a800a830 266e558e eba6b975
                     6369c9e8 41dbd9e5 97a441a4 b54d8367
 8  post_state_root   75dbddde c19dcf16 52b1dcaf 229f8cca
                     5e3f0edb 71415fd8 b0117607 e011385b
 9  work_charged      4
10  work_refunded     4
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 1, 1, 2]   # STATUS_LOOKUP_MISS tier O state consumed
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010281 87010000 00010102 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 36 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
    q_const       4 U (1 posting)
    q_jmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    4
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 36 + 4 + 0
    remaining_budget = W_budget - charged               : 100 - 36 = 64
    refund_final = B(P) - charged                       : 40 - 36 = 4
    reserve_outstanding after terminal settlement       : 0
```

### X43 — `exec-tuple-field-extract`

Cites: §2.3 TUPLE row (binds the canonical tuple value over the components to a fresh instance; fall-through; each component checked against its declared bound before dispatch) and FIELD row (binds the i-th component's canonical value to a fresh instance; no run-time rejection — postulate (v)); §2.2 (the dynamic value store component carrying the tuple value) and §2.4 (redefinition mints, never overwrites); §2.6 (the tuple value grammar, fields in declared order); §4.4 Suite Register bullet 3 — the `TUPLE`/`FIELD` execution surface; declared parameter P13(c) (the bound reading).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=9)
  1: CONST imm=9 bnd=(int, <=9)
  2: TUPLE k=2 fields=[(int, <=9), (int, <=9)] ref=0 ref=1
  3: FIELD ref=2 idx=0
  4: FIELD ref=2 idx=1
  5: HALT
```

Intent envelope (canonical CBOR, 426 bytes; instruction stream entry a byte string of 44 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01600000 00000000 00000000
  00000000 00000000 00000000 00060000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00070000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00000000
  00000000 00030000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00010000
  00000000 00000000 00000000 00040000
  00000000 00020000 00000000 00000000
  00000000 00000000 00000000 00040000
  00000000 00020000 00000000 00010000
  00000000 00000000 00000000 00100000
  00000000 00000780 0880
```

Admission: no back-edges; the maximum path is 6 transitions (24 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (6 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 8  post_state_root   e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000003 0000000000000002 0000000000000001
                     0000000000000001 0000000000000001 0000000000000009
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 8  post_state_root   d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      0000000000000004 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 8  post_state_root   9fb8d502 c3482425 af004b34 f225afe2
                     47ae638f 7ab1b907 aec43312 8351303c
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         9fb8d502 c3482425 af004b34 f225afe2
                     47ae638f 7ab1b907 aec43312 8351303c
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      0000000000000004 0000000000000002 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9fb8d502 c3482425 af004b34 f225afe2
                     47ae638f 7ab1b907 aec43312 8351303c
 8  post_state_root   9ebec459 3e4f5605 a5ba4697 f79569e0
                     8e97b072 3f7fff68 4aa24117 46b03f06
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         9ebec459 3e4f5605 a5ba4697 f79569e0
                     8e97b072 3f7fff68 4aa24117 46b03f06
 2  program_root      459f45b6 130b3eb8 4be5ebf8 2411212c
                     c3260944 656dd9fa 3fc6227f 65aff21a
 3  step_index        6
 4  pc                [0, 5]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    9ebec459 3e4f5605 a5ba4697 f79569e0
                     8e97b072 3f7fff68 4aa24117 46b03f06
 8  post_state_root   faa54294 ed80bfa8 864c276e 2c65a0e9
                     782fdbea a20c142d 7030cd74 912f6fea
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 44 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
    q_field       8 U (2 postings)
    q_tuple       4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  44 /    0
    step  2:   28 /  44 /    0
    step  3:   32 /  44 /    0
    step  4:   36 /  44 /    0
    step  5:   40 /  44 /    0
    step  6:   44 /  44 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 44 = 44 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 44 = 56
    refund_final = B(P) - charged                       : 44 - 44 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X44 — `exec-tuple-component-bound-rejection`

Cites: §2.3 TUPLE row (any component exceeds its declared bound: typed rejection before dispatch — no binding, no store effect); §2.5 failure behavior (the typed rejection is fatal, births EXPRESSION_TOO_COMPLEX, terminalizing at its own transition) and T case (1); §2.6 receipt key 3 (nil at an `EXPRESSION_TOO_COMPLEX` exit, the X10 shape); §3.3 with P11 (the rejection posts nothing; the escrow is open — the `CONST` completed first — so the terminal settlement's refund rides the rejection record, the X5 precedent); §4.4 Suite Register bullet 3; declared parameter P13(c) (the bound reading).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=9)
  1: TUPLE k=2 fields=[(int, <=9), (int, <=4)] ref=0 ref=0
  2: HALT
```

Intent envelope (canonical CBOR, 289 bytes; instruction stream entry a byte string of 27 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010658 d8000000 00000000 00000000
  00000000 00000000 00000000 03000000
  00000000 01000000 00000000 01000000
  00000000 01000000 00000000 07000000
  00000000 01000000 00000000 01000000
  00000000 01000000 00000000 09000000
  00000000 00000000 00000000 03000000
  00000000 02000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 09000000 00000000 01000000
  00000000 01000000 00000000 01000000
  00000000 04000000 00000000 00000000
  00000000 00000000 00000000 00000000
  00000000 10000000 00000000 00078008
  80
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (2 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      1ccad1d5 44b60a79 1e276b25 d2df7782
                     5b901e92 a0278353 964839d3 6b763c84
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — rejection (posts nothing)
 1  prev_root         eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 2  program_root      1ccad1d5 44b60a79 1e276b25 d2df7782
                     5b901e92 a0278353 964839d3 6b763c84
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000003 0000000000000002 0000000000000001
                     0000000000000001 0000000000000001 0000000000000009
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000004 0000000000000000 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    eec5558c 00a9abda d04e601c cabbb384
                     6ef8e169 38947bca 2c27835a 353c9f61
 8  post_state_root   2b0a0f06 a18cbad1 20bede31 e354a966
                     d2d2cb19 981ce6d0 991c1d1e 6d13982f
 9  work_charged      0
10  work_refunded     8
11  clock             2
12  status            [[1, [2, 0, 1, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [2, 0, 1, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87020001 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 24 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  32 /    0
    step  2:   24 /  32 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 24 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 24 = 76
    refund_final = B(P) - charged                       : 32 - 24 = 8
    reserve_outstanding after terminal settlement       : 0
```
### X45 — `exec-tuple-nested-field-extract`

Cites: §2.3 TUPLE row (the canonical tuple value over the components, one component itself a tuple value) and FIELD row (the i-th component's canonical value bound to a fresh instance — here a tuple, then an integer from that tuple); §2.2 (the store's entries are names (static value id, instance id): two entries may carry byte-equal canonical values under distinct names); §2.4 (redefinition mints, never overwrites); §2.6 (the nested tuple value grammar); §4.4 Suite Register bullet 3; declared parameter P13(c) (a TUPLE-typed field's bound word is present and vacuous — no INT_POS quantity to compare).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=9)
  1: CONST imm=3 bnd=(int, <=9)
  2: CONST imm=5 bnd=(int, <=9)
  3: TUPLE k=2 fields=[(int, <=9), (int, <=9)] ref=1 ref=2
  4: TUPLE k=2 fields=[(int, <=9), (tuple, <=0)] ref=0 ref=3
  5: FIELD ref=4 idx=1
  6: FIELD ref=5 idx=0
  7: HALT
```

Intent envelope (canonical CBOR, 594 bytes; instruction stream entry a byte string of 65 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 02080000 00000000 00000000
  00000000 00000000 00000000 00080000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00070000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00050000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00030000
  00000000 00020000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00010000
  00000000 00020000 00000000 00000000
  00000000 00030000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00030000 00000000 00010000
  00000000 00000000 00000000 00000000
  00000000 00030000 00000000 00000000
  00000000 00040000 00000000 00040000
  00000000 00010000 00000000 00000000
  00000000 00040000 00000000 00050000
  00000000 00000000 00000000 00000000
  00000000 00100000 00000000 00000780
  0880
```

Admission: no back-edges; the maximum path is 8 transitions (32 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (8 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   7f9ad008 0ec1879f 5c899c56 80e9c8da
                     6e70f4b8 1275e51f b60efe76 12726dc4
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7f9ad008 0ec1879f 5c899c56 80e9c8da
                     6e70f4b8 1275e51f b60efe76 12726dc4
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000003 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7f9ad008 0ec1879f 5c899c56 80e9c8da
                     6e70f4b8 1275e51f b60efe76 12726dc4
 8  post_state_root   d7b9c437 739a0076 8d41eb8c e063a9c7
                     36e50f9b 8a3ec19d a4efdaff c90573df
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         d7b9c437 739a0076 8d41eb8c e063a9c7
                     36e50f9b 8a3ec19d a4efdaff c90573df
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000005 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d7b9c437 739a0076 8d41eb8c e063a9c7
                     36e50f9b 8a3ec19d a4efdaff c90573df
 8  post_state_root   6e62fbb2 65a37042 5fe5e89d 642057b0
                     c2845128 2ed9ffd5 60f9b71f b0f93249
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         6e62fbb2 65a37042 5fe5e89d 642057b0
                     c2845128 2ed9ffd5 60f9b71f b0f93249
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      0000000000000003 0000000000000002 0000000000000001
                     0000000000000001 0000000000000001 0000000000000009
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000002
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    6e62fbb2 65a37042 5fe5e89d 642057b0
                     c2845128 2ed9ffd5 60f9b71f b0f93249
 8  post_state_root   010b1811 010b1da6 2d312ccb 4f23326a
                     14f6e65e ce3ebad0 b0fa427d bbc2dc46
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         010b1811 010b1da6 2d312ccb 4f23326a
                     14f6e65e ce3ebad0 b0fa427d bbc2dc46
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      0000000000000003 0000000000000002 0000000000000001
                     0000000000000001 0000000000000001 0000000000000009
                     0000000000000003 0000000000000001 0000000000000000
                     0000000000000000 0000000000000003 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    010b1811 010b1da6 2d312ccb 4f23326a
                     14f6e65e ce3ebad0 b0fa427d bbc2dc46
 8  post_state_root   24272fd5 6aad0ea1 8d089e0e 5a530184
                     b8dfd5da 728f7291 39fb469c f92904a5
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         24272fd5 6aad0ea1 8d089e0e 5a530184
                     b8dfd5da 728f7291 39fb469c f92904a5
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        6
 4  pc                [0, 5]
 5  decoded_word      0000000000000004 0000000000000004 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    24272fd5 6aad0ea1 8d089e0e 5a530184
                     b8dfd5da 728f7291 39fb469c f92904a5
 8  post_state_root   a64d562c ff807cb9 65f1ba87 c969aa48
                     6442006d 745526f4 d20ed532 87414aa8
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 7 — completed
 1  prev_root         a64d562c ff807cb9 65f1ba87 c969aa48
                     6442006d 745526f4 d20ed532 87414aa8
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        7
 4  pc                [0, 6]
 5  decoded_word      0000000000000004 0000000000000005 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a64d562c ff807cb9 65f1ba87 c969aa48
                     6442006d 745526f4 d20ed532 87414aa8
 8  post_state_root   423c2a8e 11d577ad 1ab1b98b bad8a8b4
                     9b7b6e8a 6eca58aa 9b291248 84fb41e4
 9  work_charged      4
10  work_refunded     0
11  clock             7
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 8 — completed
 1  prev_root         423c2a8e 11d577ad 1ab1b98b bad8a8b4
                     9b7b6e8a 6eca58aa 9b291248 84fb41e4
 2  program_root      34e0dcae 624fa6a7 f19c272b 06c9b497
                     1289d353 c1663f8f 16815945 0030054d
 3  step_index        8
 4  pc                [0, 7]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    423c2a8e 11d577ad 1ab1b98b bad8a8b4
                     9b7b6e8a 6eca58aa 9b291248 84fb41e4
 8  post_state_root   a3f99ef4 60ee71d1 add2d3f6 928d400d
                     d25cce6e e3846e50 f25feb12 e0321390
 9  work_charged      4
10  work_refunded     0
11  clock             8
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 52 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const      12 U (3 postings)
    q_tuple       8 U (2 postings)
    q_field       8 U (2 postings)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  52 /    0
    step  2:   28 /  52 /    0
    step  3:   32 /  52 /    0
    step  4:   36 /  52 /    0
    step  5:   40 /  52 /    0
    step  6:   44 /  52 /    0
    step  7:   48 /  52 /    0
    step  8:   52 /  52 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 52 = 52 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 52 = 48
    refund_final = B(P) - charged                       : 52 - 52 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X46 — `exec-cmp-zoom-boundary-equal`

Cites: §2.3 CONST row (the literal's canonical value bound to a fresh instance — a zoom literal in its normalized form per §2.6) and CMP row (the comparison result −1, 0, or 1 bound to a fresh instance; the zoom-boundary comparison under N: equal exactly when the normalized pairs are equal); §2.3's zoom declaration (a program declaring zoom-carrying values; the declared ranges ride P13(a)) and the minimal-zoom normalizer N; §2.6 (the ZOOM value grammar, the normalized form); §4.4 Suite Register bullet 3 — the zoom `CMP` execution surface; declared parameter P13(c) (the zoom bound reading).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=zoom(1, 0) bnd=(zoom, <=4)
  1: CONST imm=zoom(1, 0) bnd=(zoom, <=4)
  2: CMP ref=0 ref=1 rng=[-1, 1]
  3: HALT
```

Intent envelope (canonical CBOR, 386 bytes; instruction stream entry a byte string of 39 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01380000 00000000 00000000
  00000000 00000000 00000000 00040000
  00000000 00010000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00040000 00000000 00000000
  00000000 00010000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00040000 00000000 00000000
  00000000 000a0000 00000000 00000000
  00000000 00010000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00100000 00000000 00000780
  0880
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (4 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      43d318a8 44ece6cb d32430d3 d513dc79
                     44f6c7e6 d595915c 03b035f1 8bbb6ff6
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000 0000000000000004 0000000000000001
                     0000000000000001 0000000000000004 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   8a97dc3b ec0b1c3f 3f3a9525 98a6d610
                     7d0b8872 949f8766 5b37b38c 8e57ea6d
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         8a97dc3b ec0b1c3f 3f3a9525 98a6d610
                     7d0b8872 949f8766 5b37b38c 8e57ea6d
 2  program_root      43d318a8 44ece6cb d32430d3 d513dc79
                     44f6c7e6 d595915c 03b035f1 8bbb6ff6
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000 0000000000000004 0000000000000001
                     0000000000000001 0000000000000004 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8a97dc3b ec0b1c3f 3f3a9525 98a6d610
                     7d0b8872 949f8766 5b37b38c 8e57ea6d
 8  post_state_root   8f099107 0f7a4114 ba7bafdc b3e6b969
                     70ca3b0b ae162376 9af2d5cb 6ea528c9
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         8f099107 0f7a4114 ba7bafdc b3e6b969
                     70ca3b0b ae162376 9af2d5cb 6ea528c9
 2  program_root      43d318a8 44ece6cb d32430d3 d513dc79
                     44f6c7e6 d595915c 03b035f1 8bbb6ff6
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    8f099107 0f7a4114 ba7bafdc b3e6b969
                     70ca3b0b ae162376 9af2d5cb 6ea528c9
 8  post_state_root   a189abd5 55531f65 da621acb bb6c6a51
                     023fddc3 017f5a07 70e0725c 6cd79336
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         a189abd5 55531f65 da621acb bb6c6a51
                     023fddc3 017f5a07 70e0725c 6cd79336
 2  program_root      43d318a8 44ece6cb d32430d3 d513dc79
                     44f6c7e6 d595915c 03b035f1 8bbb6ff6
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    a189abd5 55531f65 da621acb bb6c6a51
                     023fddc3 017f5a07 70e0725c 6cd79336
 8  post_state_root   7006e907 e56e8fd4 518bca8a 154ec2f0
                     d29d7aaa 25a46ee3 e4002d51 c41703e2
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 36 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
    q_cmp         4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   32 /  36 /    0
    step  4:   36 /  36 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 36 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 36 = 64
    refund_final = B(P) - charged                       : 36 - 36 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X47 — `exec-cmp-zoom-order-exact-shift`

Cites: §2.3 CMP row (the zoom-boundary comparison under N: for normalized (q1, z1), (q2, z2) with z1 ≤ z2 the ordering is decided by comparing q1 against q2·2^(z2−z1) in exact integer arithmetic — the exponent difference is non-negative, so the shift is exact and the comparison never rounds); §2.3's zoom declaration and the minimal-zoom normalizer N; §2.6 (the ZOOM value grammar, the normalized form); §4.4 Suite Register bullet 3 — the zoom `CMP` execution surface; declared parameter P13(c) (the zoom bound reading).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=zoom(3, -1) bnd=(zoom, <=4)
  1: CONST imm=zoom(1, 0) bnd=(zoom, <=4)
  2: CMP ref=0 ref=1 rng=[-1, 1]
  3: CMP ref=1 ref=0 rng=[-1, 1]
  4: HALT
```

Intent envelope (canonical CBOR, 474 bytes; instruction stream entry a byte string of 50 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01900000 00000000 00000000
  00000000 00000000 00000000 00050000
  00000000 00010000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00040000
  00000000 00000000 00000000 00010000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00040000
  00000000 00000000 00000000 000a0000
  00000000 00000000 00000000 00010000
  00000000 00020000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 000a0000
  00000000 00010000 00000000 00000000
  00000000 00020000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00100000
  00000000 00000780 0880
```

Admission: no back-edges; the maximum path is 5 transitions (20 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (5 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      fc588793 ac1c4cfe a7f17715 b4a177cd
                     c6baffa3 0f1e6f4d 09a95af1 547977a0
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000003 0000000000000002
                     0000000000000001 0000000000000001 0000000000000004
                     0000000000000001 0000000000000001 0000000000000004
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   c1dfbc05 0f11d4a6 fda364b7 0f2f75f3
                     75b667ba 47e2ace2 6af393f0 425b76b1
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         c1dfbc05 0f11d4a6 fda364b7 0f2f75f3
                     75b667ba 47e2ace2 6af393f0 425b76b1
 2  program_root      fc588793 ac1c4cfe a7f17715 b4a177cd
                     c6baffa3 0f1e6f4d 09a95af1 547977a0
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000 0000000000000004 0000000000000001
                     0000000000000001 0000000000000004 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    c1dfbc05 0f11d4a6 fda364b7 0f2f75f3
                     75b667ba 47e2ace2 6af393f0 425b76b1
 8  post_state_root   d1c58f1c 056269f1 fdecc31c daab2225
                     40b8e93d 421699e1 5417d76d 3587233e
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         d1c58f1c 056269f1 fdecc31c daab2225
                     40b8e93d 421699e1 5417d76d 3587233e
 2  program_root      fc588793 ac1c4cfe a7f17715 b4a177cd
                     c6baffa3 0f1e6f4d 09a95af1 547977a0
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d1c58f1c 056269f1 fdecc31c daab2225
                     40b8e93d 421699e1 5417d76d 3587233e
 8  post_state_root   bd4510c4 4449d4fa dc329dc8 f2831e46
                     1ead693d 4bb24e71 04d2db25 90a87311
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         bd4510c4 4449d4fa dc329dc8 f2831e46
                     1ead693d 4bb24e71 04d2db25 90a87311
 2  program_root      fc588793 ac1c4cfe a7f17715 b4a177cd
                     c6baffa3 0f1e6f4d 09a95af1 547977a0
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000a 0000000000000001 0000000000000000
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    bd4510c4 4449d4fa dc329dc8 f2831e46
                     1ead693d 4bb24e71 04d2db25 90a87311
 8  post_state_root   2b52a7f5 4d7f3f54 c11132db 5d31ebbb
                     4d536fe8 e5fa60b7 13397f72 86d40475
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         2b52a7f5 4d7f3f54 c11132db 5d31ebbb
                     4d536fe8 e5fa60b7 13397f72 86d40475
 2  program_root      fc588793 ac1c4cfe a7f17715 b4a177cd
                     c6baffa3 0f1e6f4d 09a95af1 547977a0
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    2b52a7f5 4d7f3f54 c11132db 5d31ebbb
                     4d536fe8 e5fa60b7 13397f72 86d40475
 8  post_state_root   8c8cd74f b1a5e422 21075f7a e55f336e
                     ff69d89f 9c6469b3 3a354e46 93c3459a
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 40 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
    q_cmp         8 U (2 postings)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  40 /    0
    step  2:   28 /  40 /    0
    step  3:   32 /  40 /    0
    step  4:   36 /  40 /    0
    step  5:   40 /  40 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 40 = 40 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 40 = 60
    refund_final = B(P) - charged                       : 40 - 40 = 0
    reserve_outstanding after terminal settlement       : 0
```
### X48 — `exec-cmp-zoom-range-rejection`

Cites: §2.3 CMP row (a zoom-carrying operand outside its declared zoom range: typed rejection before dispatch — no binding, no result instance) and CONST row (the zoom literal admitted at its own declared bound 8 under P13(c) — the producing instructions completed); §2.5 failure behavior (the typed rejection is fatal, births EXPRESSION_TOO_COMPLEX, terminalizing at its own transition) and T case (1); §2.6 receipt key 3 (nil at an `EXPRESSION_TOO_COMPLEX` exit, the X10 shape); §3.3 with P11 (the rejection posts nothing; the escrow is open — the two `CONST`s completed — so the terminal settlement's refund rides the rejection record, the X5 precedent); §4.4 Suite Register bullet 3; declared parameters P13(a) (the declared ranges) and P13(b) (the pre-dispatch check).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=zoom(1, 5) bnd=(zoom, <=8)
  1: CONST imm=zoom(1, 0) bnd=(zoom, <=4)
  2: CMP ref=0 ref=1 rng=[-1, 1]
  3: HALT
```

Intent envelope (canonical CBOR, 394 bytes; instruction stream entry a byte string of 40 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01400000 00000000 00000000
  00000000 00000000 00000000 00040000
  00000000 00010000 00000000 00040000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00050000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00080000
  00000000 00000000 00000000 00010000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00040000 00000000 00010000
  00000000 00010000 00000000 00040000
  00000000 00000000 00000000 000a0000
  00000000 00000000 00000000 00010000
  00000000 00020000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00100000
  00000000 00000780 0880
```

Admission: no back-edges; the maximum path is 4 transitions (16 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (3 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      7159aaf2 423a6275 3e7d0224 dec1b233
                     c1f9c49f be349e1e 185d254d ac84b11a
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000001 0000000000000005 0000000000000004
                     0000000000000001 0000000000000001 0000000000000008
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   b9774c6e c879ac4f 1b75aa4f a36abb27
                     c565e585 182132fa ed6cebf1 1cdb23ab
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         b9774c6e c879ac4f 1b75aa4f a36abb27
                     c565e585 182132fa ed6cebf1 1cdb23ab
 2  program_root      7159aaf2 423a6275 3e7d0224 dec1b233
                     c1f9c49f be349e1e 185d254d ac84b11a
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000004 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000 0000000000000004 0000000000000001
                     0000000000000001 0000000000000004 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b9774c6e c879ac4f 1b75aa4f a36abb27
                     c565e585 182132fa ed6cebf1 1cdb23ab
 8  post_state_root   f0b53ab3 955272a9 6ae6981f f5a20439
                     bb780007 5cfc3ce4 68f8f0a4 6d6e4733
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — rejection (posts nothing)
 1  prev_root         f0b53ab3 955272a9 6ae6981f f5a20439
                     bb780007 5cfc3ce4 68f8f0a4 6d6e4733
 2  program_root      7159aaf2 423a6275 3e7d0224 dec1b233
                     c1f9c49f be349e1e 185d254d ac84b11a
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      000000000000000a 0000000000000000 0000000000000001
                     0000000000000002 0000000000000001 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    f0b53ab3 955272a9 6ae6981f f5a20439
                     bb780007 5cfc3ce4 68f8f0a4 6d6e4733
 8  post_state_root   8f5c7d7e 442fe961 85d489df 33cd0e90
                     98585305 3c06f604 573f858a 9e9de3c5
 9  work_charged      0
10  work_refunded     8
11  clock             3
12  status            [[1, [3, 0, 2, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [3, 0, 2, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87030002 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 28 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  36 /    0
    step  2:   28 /  36 /    0
    step  3:   28 /  36 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 36 = 28 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 28 = 72
    refund_final = B(P) - charged                       : 36 - 28 = 8
    reserve_outstanding after terminal settlement       : 0
```
### X49 — `exec-emit-witness-queue`

Cites: §2.3 EMIT row (appends the witness record to the receipt-order queue — the declared witness payload: (the declared kind, the cited values' current instance canonical encodings, the emission position) — ordered per the state tuple's receipt ordering; no run-time rejection — binding-before-use is a static legality rule, postulate (v)); P7 (the emission ordinal, dense from 0; the receipt-order queue and record field 13 carry ordinals); §2.2 (the receipt-order queue component of the canonical configuration, in emission order); §4.4 Suite Register bullet 3 — the `EMIT` witness surface; declared parameter P13(d) (the witness rendering).

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The declared zoom ranges ride P13(a) (the normalized exponent within [-4, 4], the normalized mantissa within [0, 2^64)); the witness rendering rides P13(d).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CONST imm=7 bnd=(int, <=9)
  1: CONST imm=9 bnd=(int, <=9)
  2: TUPLE k=2 fields=[(int, <=9), (int, <=9)] ref=0 ref=1
  3: EMIT kind=1 k=2 ref=2 ref=0
  4: EMIT kind=2 k=0
  5: HALT
```

Intent envelope (canonical CBOR, 442 bytes; instruction stream entry a byte string of 46 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01700000 00000000 00000000
  00000000 00000000 00000000 00060000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00070000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00090000 00000000 00000000
  00000000 00030000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00010000
  00000000 00000000 00000000 000f0000
  00000000 00010000 00000000 00020000
  00000000 00020000 00000000 00000000
  00000000 00000000 00000000 000f0000
  00000000 00020000 00000000 00000000
  00000000 00000000 00000000 00100000
  00000000 00000780 0880
```

Admission: no back-edges; the maximum path is 6 transitions (24 U) plus the gateway 20 U.

Expected terminal outcome: **SUCCESS** (tag 1).

Expected transition records (6 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000007 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — completed
 1  prev_root         7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        2
 4  pc                [0, 1]
 5  decoded_word      0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000001 0000000000000001
                     0000000000000001 0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7d8e7fa5 12e06ccf 62689c78 44baa95d
                     1d4d4b1f a885dab3 ccd1d918 a1876833
 8  post_state_root   e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 9  work_charged      4
10  work_refunded     0
11  clock             2
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 3 — completed
 1  prev_root         e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        3
 4  pc                [0, 2]
 5  decoded_word      0000000000000003 0000000000000002 0000000000000001
                     0000000000000001 0000000000000001 0000000000000009
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000 0000000000000001
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    e5d40871 1ae839d9 177bd791 4486efc5
                     0bff6721 6e99509e a04a7454 470466c8
 8  post_state_root   d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 9  work_charged      4
10  work_refunded     0
11  clock             3
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 4 — completed
 1  prev_root         d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        4
 4  pc                [0, 3]
 5  decoded_word      000000000000000f 0000000000000001 0000000000000002
                     0000000000000002 0000000000000000 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    d58518b7 71891c9c fdc39f10 e45657d2
                     1d509ec0 0935ada5 850c65e2 72d01d86
 8  post_state_root   b8ad2425 77700c98 b550def4 c955406d
                     bebc5949 f4284b87 9707be3a 33eba45e
 9  work_charged      4
10  work_refunded     0
11  clock             4
12  status            []
13  receipt_ids       [0]
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 5 — completed
 1  prev_root         b8ad2425 77700c98 b550def4 c955406d
                     bebc5949 f4284b87 9707be3a 33eba45e
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        5
 4  pc                [0, 4]
 5  decoded_word      000000000000000f 0000000000000002 0000000000000000
                     0000000000000000
 6  input_output_ids  []
 7  pre_state_root    b8ad2425 77700c98 b550def4 c955406d
                     bebc5949 f4284b87 9707be3a 33eba45e
 8  post_state_root   91290a44 c17243bb e27bb2e6 3ff7125b
                     33c55de8 261ed11a f6c6e8c4 88d19d67
 9  work_charged      4
10  work_refunded     0
11  clock             5
12  status            []
13  receipt_ids       [1]
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 6 — completed
 1  prev_root         91290a44 c17243bb e27bb2e6 3ff7125b
                     33c55de8 261ed11a f6c6e8c4 88d19d67
 2  program_root      b426a2d0 abf99991 53715b3a fa394873
                     77be9b12 4d49b97a 866306a6 65527cc9
 3  step_index        6
 4  pc                [0, 5]
 5  decoded_word      0000000000000010 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    91290a44 c17243bb e27bb2e6 3ff7125b
                     33c55de8 261ed11a f6c6e8c4 88d19d67
 8  post_state_root   0e471c85 b6b6a7f9 3de7f639 479a49cd
                     7d16bdcf 5b72a360 cb4f9943 a1ef2f1f
 9  work_charged      4
10  work_refunded     0
11  clock             6
12  status            []
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      1 (SUCCESS)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01010280 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 44 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_const       8 U (2 postings)
    q_emit        8 U (2 postings)
    q_tuple       4 U (1 posting)
    q_halt        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  44 /    0
    step  2:   28 /  44 /    0
    step  3:   32 /  44 /    0
    step  4:   36 /  44 /    0
    step  5:   40 /  44 /    0
    step  6:   44 /  44 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 44 = 44 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 44 = 56
    refund_final = B(P) - charged                       : 44 - 44 = 0
    reserve_outstanding after terminal settlement       : 0
```

### X50 — `exec-escrow-never-opened-rejection`

Cites: §2.3 CLOAD row (hit value exceeds its declared bound: typed rejection before dispatch — no binding, no store effect); §2.5 failure behavior (the typed rejection is fatal, births EXPRESSION_TOO_COMPLEX, terminalizing at its own transition) and T case (1); §2.6 receipt key 3 (nil at an `EXPRESSION_TOO_COMPLEX` exit, the X10 shape); §3.4 (the admission passes — `W_budget` ≥ `B_hat(P)` — and on a pass the transition completes normally and opens the escrow; the rejected first attempted transition completes nothing); §3.3 with P11 (the rejection posts nothing; the escrow is never opened — no completed transition precedes the rejection — so no refund posts anywhere: the escrow-never-opened settlement shape, the X2 precedent's zero-posting ledger at the execution's first attempted transition); §4.4 Suite Register bullet 5 — the budget-boundary family's escrow-never-opened settlement shape; declared parameter P12.

Declared parameters: `W_budget` = 100 U; `D` = { addr 0 → 10 }; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency table rides the declared rendering parameter P12 (the constant-table entry of the declared content address h'0000000000000001', encoded per §2.6).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6); the envelope is byte-identical to X39's committed envelope (equal programs are byte-equal envelopes; the declared `D` is vector input, never envelope content — the X39/X40/X6 precedent):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 1 bnd=(int, <=9)
  1: JMP block 2
block 1 (arity 0):
  0: CONST imm=9 bnd=(int, <=9)
  1: HALT
block 2 (arity 0):
  0: HALT
```

Intent envelope (canonical CBOR, 355 bytes; instruction stream entry a byte string of 34 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050306 59011000
  00000000 00000000 00000000 00000000
  00000000 00000200 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00000c00
  00000000 00000200 00000000 00000000
  00000000 00000000 00000000 00000100
  00000000 00000000 00000000 00000200
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000100 00000000 00000100
  00000000 00000100 00000000 00000900
  00000000 00000000 00000000 00001000
  00000000 00000000 00000000 00000200
  00000000 00000000 00000000 00000100
  00000000 00001000 00000000 00000007
  800880
```

Admission: no back-edges; the maximum path is 3 transitions (12 U) plus the gateway 20 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (1 record, complete ordered sequence; field values canonical per §2.6):

```
record 1 — rejection (posts nothing)
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      3f2d0b1b 567efc63 35c7e196 efb9889e
                     d12bab2c 8af92293 cec5152e 7d51921e
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000001
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   a043b9ec 5535b895 797f2ddc a2fc86eb
                     098c5366 b0537636 5a02bf9c 9ba04caf
 9  work_charged      0
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 3]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030281 87010000 00030203 03f60483
  7076302e 332e302d 64726166 742e382e
  34697169 722d656e 632d3271 7169722d
  70726f66 2d676f6c 64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 0 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
  settlement trace (charged / reserved / refunded after each transition):
    step  1:    0 /   0 /    0
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 0 = 0 + 0 + 0
    remaining_budget = W_budget - charged               : 100 - 0 = 100
    refund_final                                        : the escrow never opened (typed rejection at the execution's first attempted transition) — no terminal refund
    reserve_outstanding after terminal settlement       : 0
```

### X51 — `exec-cload-fallback-trip-ceiling-self-loop`

Cites: §2.3 `CLOAD` row (a miss whose fallback edge is a back edge with an exhausted declared trip ceiling: typed rejection before the miss token is born — the F25 amendment) and the derived-count paragraph (the `CLOAD` miss edge's site is `(block id, instruction index, slot 0)`; a hit traverses no edge and increments no count; a completed miss increments its site's count); §2.5 failure behavior (the typed rejection is fatal, births `EXPRESSION_TOO_COMPLEX`, terminalizing at its own transition) and T case (1); §2.5 Tier O repeated-miss rule (each repeated miss is a further traversal of the fallback edge subject to the same declared trip ceiling, and the traversal that would exceed it is rejected before its token is born); §2.6 receipt key 3 (nil at an `EXPRESSION_TOO_COMPLEX` exit, the X10 shape); §3.3 with P11 (the rejection posts nothing; the escrow is open — the miss completed first — so the terminal settlement's refund rides the rejection record, the X5 precedent); §3.4 (the admission bound counts the fallback back-edge once, so the admission passes and the fuel backstop stays unfirable — F16); §4.4 Suite Register — the `CLOAD` fallback trip-ceiling regression bullet (Finding F25, Reading R1); declared reading DR4.

Declared parameters: `W_budget` = 100 U; `D` = no entries; the declared initial status pool is empty (§2.5: empty unless the Intent declares entries); `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2). The dependency environment rides the declared rendering parameter P12: the empty table is the committed `D` = no entries form, and the program still cites the seed's declared content address in its Intent dependency list (P9).

Intent — program in the source grammar's textual form, rendered from the committed bytes (the assembler's mapping is one-to-one, §2.6):

```text
block 0 (arity 0):
  0: CLOAD addr=0 fallback=block 0 bnd=(int, <=9)
  1: HALT
```

Intent envelope (canonical CBOR, 186 bytes; instruction stream entry a byte string of 13 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320481
  48000000 00000000 01050106 58680000
  00000000 00000000 00000000 00000000
  00000000 00020000 00000000 00020000
  00000000 00000000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00090000
  00000000 00000000 00000000 00100000
  00000000 00000780 0880
```

Admission: one back-edge — the `CLOAD` fallback self-loop (block 0 → block 0, slot 0 of the site identity) — under the trip ceiling 1; the maximum bounded path is 2 transitions (8 U) plus the gateway 20 U: B(P) = 32 U.

Expected terminal outcome: **EXPRESSION_TOO_COMPLEX** (tag 3).

Expected transition records (2 records, complete ordered sequence; field values canonical per §2.6):

```
record 1 — completed
 1  prev_root         0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 2  program_root      30f32781 2a145c18 090388d8 9faf061d
                     c9b0c6f6 466373e2 6be8eea1 0c5d610a
 3  step_index        1
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000000
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  [h'0000000000000001']
 7  pre_state_root    0f9524ff 8400ab93 5dbb66f0 a0c93f58
                     a99ff7f0 8c3c066d 5fee9c56 e101064f
 8  post_state_root   7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 9  work_charged      24
10  work_refunded     0
11  clock             1
12  status            [[1, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

```
record 2 — rejection (posts nothing)
 1  prev_root         7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 2  program_root      30f32781 2a145c18 090388d8 9faf061d
                     c9b0c6f6 466373e2 6be8eea1 0c5d610a
 3  step_index        2
 4  pc                [0, 0]
 5  decoded_word      0000000000000002 0000000000000000 0000000000000000
                     0000000000000001 0000000000000001 0000000000000001
                     0000000000000009 0000000000000000
 6  input_output_ids  []
 7  pre_state_root    7ceeaec8 44cdd24f b59ec155 f441a804
                     ef01f50b 05fd4439 378ed4ce 156dc082
 8  post_state_root   89ca35d6 c03e4ace db3933c7 9ca27844
                     18dcd798 831c657b f483a859 19ae1cf8
 9  work_charged      0
10  work_refunded     8
11  clock             2
12  status            [[1, [2, 0, 0, 1, 3]], [3, [1, 0, 0, 0, 1]]]
13  receipt_ids       []
14  versions          ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]
```

Expected terminal receipt:

```
terminal receipt (deterministic CBOR map, closed keys 0–4):
  0  domain_tag       13 bytes — the byte string "QnIR-Terminal" (canonical bytes below)
  1  outcome_tag      3 (EXPRESSION_TOO_COMPLEX)
  2  token_itemization (each entry: [birth step, block, index, serial, type, tier, state word])
      [1, 0, 0, 0, 1, 1, 3]   # STATUS_LOOKUP_MISS tier O state terminalized
      [2, 0, 0, 1, 3, 2, 3]   # EXPRESSION_TOO_COMPLEX tier S state terminalized
  3  selected_token   nil   # no outcome-determining token for this outcome (§2.5)
  4  versions         ["v0.3.0-draft.8.4", "qir-enc-2", "qir-prof-golden-2"]

canonical bytes:
  a5004d51 6e49522d 5465726d 696e616c
  01030282 87010000 00010103 87020000
  01030203 03f60483 7076302e 332e302d
  64726166 742e382e 34697169 722d656e
  632d3271 7169722d 70726f66 2d676f6c
  64656e2d 32
```

Expected reconstructed ledger and conservation:

```
reconstructed ledger (§3.1: derived from the records' charged and
refunded fields plus the terminal receipt's itemization):
  Work(E) = 24 U (sum over completed transitions; the
           rejection/refusal transition posts nothing)
  per-class traces (one trace per declared class, §3.1/§4.4):
    gateway      20 U (1 posting)
    q_load        4 U (1 posting)
  settlement trace (charged / reserved / refunded after each transition):
    step  1:   24 /  32 /    0
    step  2:   24 /  32 /    8
  conservation identities at the terminal settlement (§3.3):
    reserved = charged + refunded + reserve_outstanding : 32 = 24 + 8 + 0
    remaining_budget = W_budget - charged               : 100 - 24 = 76
    refund_final = B(P) - charged                       : 32 - 24 = 8
    reserve_outstanding after terminal settlement       : 0
```


## Suite Register

This seed is increment 1 of the golden corpus (§4.5: "a small suite"). The
following declared obligations of §4.4 are registered as expansion
increments of the same stage, hand-derived under this file's discipline and
committed before the artifacts that need them:

- the remaining `QO-22`/F12 terminalization combinations (every combination of {outstanding Tier S token class} × {promoted, not promoted} × {fallback success, fallback repeated miss, fallback typed rejection} × {exit event}) — **discharged at v0.3.0-draft.14 as expansion increment 1 (X9–X38) for every cell a conforming golden can fix**: the 30 reachable `HALT` and terminalizing-birth cells are vectors above, the 6 fallback-typed-rejection-`HALT` cells are declared unreachable (§2.5 failure behavior), and the 18 fuel-backstop cells remain registered to this register's fuel-backstop fixture bullet below (§3.4/F16 — unobservable by a conforming golden);
- `CLOAD` hit vectors (D entries present, bound value checked against its declared bound), and a multi-block fallback completion vector fixing the consumption site beyond a HALT-terminated fallback block — **discharged at v0.3.0-draft.15 as expansion increment 2 (X39–X42)**: the hit path within the bound, the bound check's strict boundary, the bound check's typed rejection before dispatch, and the multi-block completion fixing the consumption site at the fallback block's own completion under the declared reading DR3, over the declared dependency-table rendering P12;
- `TUPLE`/`FIELD` value and execution vectors, zoom `CMP` execution vectors (boundary comparison under N), and `EMIT` witness vectors (receipt ordinals and the witness payload) — **discharged at v0.3.0-draft.16 as expansion increment 3 (X43–X49)**: the flat and nested tuple constructions with their field extractions, the component-bound typed rejection before dispatch, the zoom boundary comparison under N in both branches (equal; ordered with the exact shift), the `CMP` declared-range rejection, and two witness records with their ordinals and payload, over the declared parameter P13;
- depth-ceiling vectors — **discharged by retirement at v0.3.0-draft.22 (F26)**: the depth ceiling is withdrawn as redundant (the trip ceilings alone certify finiteness, and no complete operational measure of nesting depth exists for arbitrary block graphs — the adjudication record is [`docs/logs/qnir/f26-depth-ceiling-retirement.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/f26-depth-ceiling-retirement.md)), so no vector has a depth-ceiling target to fix;
- the fuel-backstop-on-validator-defect fixture (a backstop stop on an admitted Intent evidences a validator defect, never a satisfied admission rule — F16) — **discharged at v0.3.0-draft.23 as the Fixture Register below**: the declared test-only fault injection (`B_injected` = 32 U against the exact `B_hat(P_f)` = 40 U, at `W_budget` = 36 U the exact surrogate refuses), the false admission the injected run's backstop stop evidences, and the containment verdict checked against the declared semantic expectation — a fixture, not a conforming golden, not counted in the declared seed;
- `CLOAD` fallback trip-ceiling regression vector (Finding F25: self-looping fallback edge under trip ceiling 1, resolving Reading R1) — **discharged at v0.3.0-draft.20 as expansion increment 6 (X51)**: the minimal self-looping-fallback program under the declared trip ceiling 1, with the first miss completing (one Tier O `STATUS_LOOKUP_MISS` born, site count 0 → 1, 24 U charged), the second traversal rejected under §2.3 before the second token is born (`EXPRESSION_TOO_COMPLEX`, nil selected, the open escrow's residue of 8 U refunded at the rejection record), the outstanding miss token terminalized at that record, and `B_hat(P) = 32` counting the fallback back-edge exactly once — under the declared reading DR4 (the rejected transition's field 6 is `[]`, the X41 precedent, correcting the F25 adjudication record's DR-a parenthetical);
- per-class ledger trace publications beyond the seed's summaries, and the per-combination expansion of the budget-boundary family — **discharged at v0.3.0-draft.17 as expansion increment 4 (X50 and the Per-Class Ledger Trace Register)**: the escrow-never-opened settlement shape (the typed rejection at the execution's first attempted transition — the family's last reachable settlement cell, over the declared rendering P12) with the remaining cells fixed by declaration (the open-escrow terminal settlements and rejection-record refunds are the committed X1/X3/X5/X10/X41/X44/X48 precedents, the admission refusal's zero-posting ledger is X2's, and the never-opened terminal settlement is unreachable — a terminal settlement requires a completed transition, which opens the escrow), and one ledger-trace publication per declared rule class (the Per-Class Ledger Trace Register — the gateway and `q_const`, `q_add`, `q_cmp`, `q_br`, `q_jmp`, `q_halt`, `q_load`, `q_tuple`, `q_field`, `q_emit`), each mechanically checkable against the equation of §3.1 through the protocol of §4.1;
- encoding vectors for `EXT` record payloads whose reading types the corpus declares with its encoding version — **discharged at v0.3.0-draft.18 as expansion increment 5 (R1–R4, M13–M15)**: the declared payload reading P14 (the closed two-type set — the empty payload and the exactly-one-§2.6-value payload — total over the payload bytes, no per-record type tag existing or needed), the canonical record encodings over both reading types (R1–R3, the new Family R), the populated-table envelope resolving M9's dangling reference (R4 — the committed M9 program, the declared-difference precedent), and the malformed counterparts at the payload extent and the record structure (M13 — trailing bytes after the complete value; M14 — the non-canonical value, M3's rule at the payload extent; M15 — the duplicate record index, the committed duplicate-key rule's case).

The independently reviewed goldens of this increment are retained unchanged
by every later expansion (§4.5); the assembler and validator are implemented
only after this corpus, are checked against it, and never generate it.

## Fixture Register — the F27 Fuel-Backstop Fixture (v0.3.0-draft.23)

This register discharges the Suite Register's last outstanding bullet — the
fuel-backstop-on-validator-defect fixture (F27) — under the fixture semantics
declared at v0.3.0-draft.23 in [`docs/qnir_conformance.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir_conformance.md) §4.4. A backstop
stop on an admitted Intent evidences a validator defect, never a satisfied
admission rule (F16, §3.4), so the fixture is **not a conforming golden**: it
is a test-only validator-defect injection, its id is outside the X family, it
is not counted in the declared seed (84 vectors), and no execution rule, no
vector, and no committed expected byte of the corpus changes by it. The
declared injection overrides the validator's computed admission bound with
the declared `B_injected` — an under-computed surrogate, the defect class a
false admission arises from — while the declared expectation below is fixed
by hand from the committed rules of §2.5, §3.3, and §3.4 before any fixture
run. The fixture commits no record or receipt bytes (a fixture that
committed the defective execution's bytes would normalize it); its declared
expectation is the semantic content of its fixture section, checked
mechanically by both artifacts in their declared fixture mode
(`--fixture F27`), whose fixture blocks must agree byte-for-byte. The
fixture envelope's canonical bytes are input data produced through the
committed normative assembler's one-to-one mapping (§4.2) and are verified
by the same committed assembly and listing-regeneration checks the X
vectors carry.

### F27 — `fixture-fuel-backstop-validator-defect`

Cites: §3.4 (the fuel backstop — defense in depth after admission; a false admission is a validator defect); §3.3 (the escrow settlement identities and the rejection-record settlement); §2.5 (the terminalization function — itemization at every exit); F16; F27; the F26 corollary (the surrogate is exact — `B_hat(P) = B(P)` for every well-formed Intent).

Declared parameters: `W_budget` = 36 U; `B_injected` = 32 U; `D` = no entries; `V` = (v0.3.0-draft.8.4, qir-enc-2, qir-prof-golden-2); declared initial status pool [2] (one STATUS_NEG_EVEN_ROOT — the conformance birth path, X3's precedent).

Admission: the exact bound `B_hat(P_f)` = 40 U — the gateway 20 U plus five 4 U transitions on the program's single bounded path (four `CONST`, then `HALT`); the surrogate is exact since F26, so `B(P_f)` = 40 U. The exact surrogate therefore refuses this Intent at `W_budget` = 36 U (40 > 36). The declared injection sets the validator's computed bound to `B_injected` = 32 U — 8 U short — so the same Intent is admitted (32 ≤ 36): the false admission the fixture evidences. Under the injected bound the escrow opens at 32 U, the gateway (20 U) and the first three transitions' charges (12 U) bring the charged total to exactly 32 U, and the fourth transition's declared charge (4 U) exceeds the outstanding escrow (0 U): the fuel backstop fires at the first transition the under-reserve cannot cover, exactly as §3.4 declares containment.

Fixture program — the source grammar's textual form:

```text
block 0 (arity 0):
  0: CONST imm=0 bnd=(int, <=0)
  1: CONST imm=1 bnd=(int, <=1)
  2: CONST imm=2 bnd=(int, <=2)
  3: CONST imm=3 bnd=(int, <=3)
  4: HALT
```

Intent envelope (canonical CBOR, 387 bytes; instruction stream entry a byte string of 39 words):

```
  a9004b51 6e49522d 496e7465 6e740170
  76302e33 2e302d64 72616674 2e382e34
  02697169 722d656e 632d3203 71716972
  2d70726f 662d676f 6c64656e 2d320480
  05010659 01380000 00000000 00000000
  00000000 00000000 00000000 00050000
  00000000 00010000 00000000 00010000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00000000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00020000
  00000000 00010000 00000000 00010000
  00000000 00010000 00000000 00020000
  00000000 00000000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00010000
  00000000 00010000 00000000 00010000
  00000000 00030000 00000000 00000000
  00000000 00100000 00000000 00000780
  088102
```

Declared expectation (semantic content only — the fixture commits no record or receipt bytes):

- control run (the exact bound, no injection): **REFUSED** at admission — `EXPRESSION_TOO_COMPLEX` (tag 3); the single rejection transition at PC (0, 0); zero postings; the escrow never opened; conservation 0 = 0 + 0 + 0.
- injected run: **ADMITTED** — the false admission the fixture evidences; the backstop fires at step 4 at PC (0, 3) — `EXPRESSION_TOO_COMPLEX` (tag 3); 3 completed transition records and 1 backstop rejection record; charged 32 U, reserved 32 U, refunded 0 U, conservation 32 = 32 + 0 + 0; remaining budget 36 - 32 = 4 U; charged ≤ reserved with equality — no overbilling.
- token itemization at the backstop exit: 2 tokens terminalized and itemized — the declared pool token (birth 0, STATUS_NEG_EVEN_ROOT, Tier S) and the fatal token (birth 4 at (0, 3), EXPRESSION_TOO_COMPLEX); the receipt's selected token nil (DR2's fatal selection is the DOMAIN_ERROR exit's).

The fixture verdict separates its two declared components: the **validator-defect report** — the backstop stopped an admitted Intent, evidencing a false admission localized to the injected bound's 8 U shortfall against the exact 40 U — and the **containment verdict** — the declared stop site and step, the outcome tag 3, the itemization, and the closed ledger with charged ≤ reserved (no overbilling: charged never exceeds the under-reserve it opened against, and the defective execution was contained exactly as §3.4 declares).


### F6 — `fixture-artifact-cmp-collision`

Cites: §2.6 full-canonical-encoding equality; §6.7 of `docs/qnir_commitments.md` (`qir-artifact-cmp-1`); §4.4 of `docs/qnir_conformance.md` (F6 fixture semantics); draft.31.

Declared artifacts: `A = (intent, v1, sha256("payload-a"), "payload-a")`; `B = (intent, v1, sha256("payload-a"), "payload-b")`; `C = (intent, v1, sha256("payload-b"), "payload-b")`; `D = (intent, v1, sha256("payload-c"), "payload-c")`; `E = (receipt, v1, sha256("payload-c"), "payload-c")`.

Declared comparison pairs:

- pair 1 — `same`: A, A
- pair 2 — `collision`: A, B (class = `digest-collision`, quarantine, entries = 1)
- pair 3 — `different`: C, D (same kind, differing bytes and digest)
- pair 4 — `different`: D, E (differing kind)

Verdict: 4/4 declared verdicts reproduced by `qir-artifact-cmp-1`; `entries = 1`; normal mode's `n = 336` and `entries = 0` are untouched. The fixture commits no record or receipt bytes.

## License and Invariant Lineage

This corpus is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and inherits the invariant lineage of the QnIR v0.3 working draft: the frozen core of §1, the semantics of §2, the accounting of §3, and the conformance specification of §4 bind every expected value above, and no vector redefines, amends, or overrides any canonical specification. The derivation records of this increment are held by the maintainer alongside the draft's offline archive; every expected byte is recomputable from the committed rules plus the declared parameters of this file.
