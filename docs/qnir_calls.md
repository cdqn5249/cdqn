---
layout: default
title: QnIR Calls — The Step-6.2 Call Extension Specification (v0.3.0-draft.36)
description: QnIR v0.3 draft, §8 Step-6.2 extension share 1 — qir-enc-4, the acyclic callee table, OP_CALL/OP_RET frame stack, compositional B_hat, sub-budget conservation, and hand-derived K1–K6 fixtures
version: 0.3.0-draft.36
updated: 2026-11-21
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_calls.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_calls.html
status: Working Draft — Step 6.2 Extension Share 1 (Not Promoted; QnIR remains [PENDING])
terms_used:
  - boc-policy
  - receipt
  - computational-consistency
---

# QnIR Calls — The Step-6.2 Call Extension Specification (v0.3.0-draft.36)

This file carries §8 of the QnIR v0.3 working draft, introduced at v0.3.0-draft.35 as **Step 6.2, Share 1** under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Like `docs/qnir_containers.md`, this companion is a **new section**, not a moved monolith block: it has no predecessor in the v0.3.0-draft.5 monolith, and the §8 body below is its initial committed content. It is grounded in the measured trade study of [`docs/logs/qnir/step6-2-call-trade-study.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-2-call-trade-study.md), in `QO-21`/Obj 6 of [`docs/AiReviews/AiReviewQnIRv0.2.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/AiReviews/AiReviewQnIRv0.2.md), and in the BOC policy of `docs/simemp.md` §7; it does not edit any of them.

---

## 8. Call Extension (Step 6.2 — Extension Share 1)

This section declares the **Step-6.2 Call Extension**: encoding version **`qir-enc-4`** and profile version **`qir-prof-ext-calls-1`**. It is a specification share only; the artifact/validator share that implements tags `30`–`31`, envelope key `9`, and the `--fixture K` surface is **Share 2**, forward of this wave. The extension is admitted to the deferral register under the default-deny closure rule of `docs/qnir.md` §1.1, adjudicated under the {% include term.html id="boc-policy" text="BOC policy" %} with the Step-6.2 study as the binding evidence. The retained candidate is **bounded acyclic calls only**: unbounded recursion and implicit stack growth are rejected as findings, not silently admitted.

### 8.1 Core and Container Preservation; the `V.encoding` Dispatch Gap

`qir-enc-4` **strictly extends `qir-enc-3`**. Every frozen core and container surface is byte-unchanged:

| Frozen surface | Status under `qir-enc-4` |
|---|---|
| Core opcodes `1`–`16` and container opcodes `17`–`29` | Unchanged; no opcode is re-tagged, removed, or redefined |
| Value tags `1`–`5` | Unchanged; no sixth value tag is introduced |
| Envelope keys `0`–`8` | Unchanged; key `9` is added only under `qir-enc-4` (§8.2) |
| The 10-component configuration of §2.2 and the 14-field record of §2.6 | Unchanged in component count, field order, domain tags, and widths |
| `qir-enc-2` core-only Intents | Remain valid and byte-identical; key `9` and tags `30`–`31` are illegal there |
| `qir-enc-3` container Intents | Remain valid and byte-identical; key `9` and tags `30`–`31` are illegal there |
| The 84 core vectors plus the `C1`–`C6` container fixtures | Remain valid under their own encoding versions; no envelope, record, {% include term.html id="receipt" text="receipt" %}, or golden byte is re-versioned |

**Mandatory validator rule (closing the dispatch gap).** The Gate-0 inspection of the committed artifacts showed that `decode_envelope` reads envelope key `2` only as text and stores it as `V.encoding`; extension behavior is selected by CLI fixture mode, not by the envelope. Under `qir-enc-4` this is closed: `decode_envelope` and `dec_insn` MUST inspect `V.encoding` and reject envelope key `9` and opcode tags `30`–`31` whenever `V.encoding` is `qir-enc-2` or `qir-enc-3`; conversely an envelope with key `9` or tags `30`–`31` is admissible only when `V.encoding = qir-enc-4`. A tag outside `1`–`31`, or an envelope key outside `0`–`9` under `qir-enc-4` (`0`–`8` under the earlier versions), is malformed under §2.6's total unknown-field rejection. No new status token is introduced: the closed token vocabulary of §2.5 and the four terminal outcomes remain total over the extension's rejection surfaces.

### 8.2 Envelope Callee Table and the Acyclic Call-Graph Rule

The Intent envelope gains exactly one **closed** key under `qir-enc-4`:

- **Key `9 = callees`**: an ordered array of callee records, each a four-element CBOR array with the exact types **`[callee_id: UInt, arg_bnds_bytes: Bytes, ret_bnds_bytes: Bytes, blocks_bytes: Bytes]`**:
  - `callee_id` a dense word `>= 0`, unique, ordered ascending;
  - `arg_bnds_bytes` carries the canonical §2.6 `bnd` word sequences (the same word grammar key `6` uses) for the callee's typed argument signature, in declared order;
  - `ret_bnds_bytes` carries the canonical §2.6 `bnd` word sequences for the callee's typed return signature, in declared order;
  - `blocks_bytes` carries the §2.6 block-stream byte string in the callee's own static-id space, with length a multiple of 8.

The caller entry remains outside the `callees` array and uses the existing key `6` instruction stream. **Strict topological acyclicity** is a static legality rule: an `OP_CALL` inside callee `i` may only target `callee_id > i`; the caller entry may target any `callee_id >= 0`. A self-call (`callee_id <= i` inside callee `i`), a cycle, an unknown `callee_id`, an arity mismatch against `arg_bnds`, or a return-count/type mismatch against `ret_bnds` makes the Intent statically illegal. There is **no runtime recursion**: F26 retired depth ceilings because no operational nesting measure existed, so finiteness is certified here by the static call graph plus the declared `C_max` bound, not by a runtime depth counter with unbounded growth (Obj 2). The call graph is a DAG; `C_max = 8` bounds the longest admitted chain.

**Callee entry binding and P10 return binding.** `callee.blocks[0]` MUST have block-argument arity exactly equal to `argc`, and `OP_CALL` binds `arg_refs[0..argc]` **positionally** into `callee.blocks[0]`'s argument environment (§2.2 component (2)) inside the callee's fresh frame; an arity mismatch is the static illegality named above. Under P10 (dense static value ids over value-defining instructions in block-then-index scan order), `OP_CALL` defines **at most one** static value id: `retc == 0` defines **0** ids and therefore has no `-> vN` target; `retc == 1` defines **1** id binding the returned value directly; and `retc >= 2` defines **1** id binding a single canonical `TAG_TUPLE` of `retc` fields in declared `ret_bnds` order, whose members are unpacked with the core `FIELD` opcode. This preserves P10's one-id-per-defining-instruction discipline without multi-result renumbering, and it mirrors the container extension's `TBL_AT`/`*_ROOT` tuple convention (§7.3).

### 8.3 The Two Call Opcodes and the Hash-Chained Frame Stack

Two opcode words are added, both using §2.6's uniform trailing `ext` word:

| Tag | Opcode | §2.6 64-bit word sequence |
|---:|---|---|
| `30` | `OP_CALL` | `30, cnt(callee_id), cnt(argc), argc × ref(arg_refs), cnt(retc), retc × bnd(ret_bnds), ext` |
| `31` | `OP_RET` | `31, cnt(retc), retc × ref(ret_refs), ext` |

Explicit §2.6 expansion: **`cnt` = 1 word**, **`ref` = 1 word**, **each `bnd` = variable `1 + (1 + len)` words** (one type-tag word, one count word, then `len` magnitude words under §2.6's INT_POS value grammar), and **`ext` = 1 word**; the opcode tag itself is 1 word. Thus `OP_CALL` occupies `3 + argc + retc + Σ_bnd_words(ret_bnds)` words and `OP_RET` occupies `3 + retc` words, the same closed word grammar as the core instruction stream.

`OP_CALL` binds the callee's declared return values as fresh value instances in the caller's §2.2 component (3) store. `OP_RET` is the **terminator of a callee exit block**; it binds nothing in the caller beyond the declared returns and settles the sub-budget (§8.4). A `HALT` inside a callee is statically malformed unless a typed fatal rejection seals the execution directly at that transition (§2.5): a callee cannot successful-halt the whole program, because only the caller entry owns the terminal receipt. `OP_CALL` in the caller entry and `OP_RET` in any callee are the only new control surfaces; no call frame, closure value, or continuation object is admitted as a value form.

**Hash-chained frame stack (P2).** The stack is a persistent commitment carried inside the §2.6 canonical configuration encoding P2, never a new configuration component:

- `stack_root_0 = SHA-256(0x02)` is the empty base (the same domain byte the empty-trace constant uses).
- Pushing a frame with serialized bytes `suspended_frame_bytes` yields **`stack_root_{d+1} = SHA-256(0x30 || stack_root_d || suspended_frame_bytes)`** — the `0x30` byte domain-separates the frame chain from the `0x00`/`0x20` leaf/node domains and the `0x01` trace domain.
- At `depth == 0`, existing **P2 Component (2)** remains exactly its canonical `active_args` encoding, so core and container P2 bytes stay **100% invariant**. At `depth > 0`, Component (2) is the two-element array **`[active_args, parent_stack_root]`**, where `parent_stack_root = stack_root_d` and `active_args` is the current top frame's argument environment; the suspended frame bytes are **not** duplicated in live P2 but are retained in the execution's F15 pre-image chain and are reconstructible deterministically. P2 size is therefore `O(F_top)` rather than `O(C · F)`.
- Frame `d` binds its arguments in its own §2.4 instance context: every value is keyed by `(static value id, instance id)`, instance ids are minted in transition order and never reused, so a callee can never alias or overwrite a suspended caller binding. On `OP_RET` the top frame is popped by restoring `parent_stack_root` as the current stack root; the suspended caller frame bytes ride in the retained pre-image, not in the live P2.

`depth <= C_max = 8` is enforced at `OP_CALL` before any effect posts; frame live values `<= F_max = 64` are enforced by the frame's declared bounds; and `C_max · F_max = 512` is the declared memory ceiling for frame storage. Exceeding any of them is the typed `EXPRESSION_TOO_COMPLEX` rejection of §2.5. The trade study measured the cost of this choice directly: Candidate C's P2 grows from 111 B at `C=1` to **5,512 B at `C=8, F=64`**, which is why high-depth calls lose to inlining on P1 hashing even when the callee body is large.

### 8.4 Compositional `B_hat`, Sub-Budget Conservation, Failure and Token Rules

The profile `qir-prof-ext-calls-1` declares **`gateway = 20 U`, `trip_ceiling = 1`, `S_max = 4096`, `C_max = 8`, `F_max = 64` (so `C_max · F_max = 512`), `max_cap = 65536`, and `max_batch = 64`**. Core opcodes keep `4 U`; container opcodes keep their `qir-prof-ext-containers-1` charges under §7.5. The two new charges are **site-stable and positive**:

- `charge(OP_CALL) = 4 + 2 · argc` U
- `charge(OP_RET) = 4 + 2 · retc` U

**Compositional `B_hat`.** Compute `bhat(callee_id)` once per callee in reverse topological order of the key-9 DAG; each callee's own trip-ceiling DP must satisfy `dp_bound(callee) <= S_max` and its state product `|B_callee| · Π_i(ceil_i + 1) <= S_max`. At each `OP_CALL` site in a caller, fold the callee's worst-case interior bound `B_callee_body` as one site-stable charge; the caller's DP therefore needs one state per call site rather than one copy of the callee's CFG per site. The total DP stays `<= S_max` even when `m >= 5`: inline state product is `4m · 2^(2m)` and refuses at `m = 5` (20,480 > 4096), while compositional states are `Σ dp_states(callee) + #call_sites + caller_states` — 22 states at `m = 6` in the recorded study. `B_hat(P) = B_cfg(P) >= B_sem(P)` and `S_max` are otherwise unchanged; a `None` bound still refuses with `EXPRESSION_TOO_COMPLEX`.

**§3.3 sub-budget settlement.** `OP_CALL` checks that the caller's open admission escrow covers `B_callee_body + charge(OP_RET)` before any effect posts, then allocates a transition-local reserve `r_sub` equal to that covered amount. The callee executes inside `r_sub`; `OP_RET` refunds the unused residue `r_sub − (charged_inside + charge(OP_RET))` back to the caller's escrow. The four §3.3 verbs are inherited without redefinition: reserve, charge, refund, and terminal residue settlement. At every settlement boundary, including the final one, **`reserved = charged + refunded`** exactly; the harness verified this on 270/270 execution rows. Frame escrows are transition-local reserves in the sense of §3.3, not a second ledger and not a second budget.

**Failure propagation.** A typed rejection inside a callee (including a zero divisor or a bound exceedance) is fatal at its own transition: it births `DOMAIN_ERROR` or `EXPRESSION_TOO_COMPLEX`, seals the fatal state in that transition's own record, and prevents any later `OP_RET` or caller `HALT` from re-sealing it (F7). The sub-budget residue is still settled by that final transition, and the outer caller's refund accounting is not corrupted: the study measured `reserved=52, charged=28, refunded=24` for both fatal classes. Unconsumed tokens — Tier-S `STATUS_NEG_EVEN_ROOT` and Tier-O `STATUS_LOOKUP_MISS` — stay in the **global** token pool across `OP_RET`; they are not frame-local. A Tier-S token determines the terminal outcome by the §2.5 earliest-birth rule (`NO_SOLUTION` for an unpromoted `STATUS_NEG_EVEN_ROOT`), while a Tier-O miss carries no verdict and is consumed only by a successful fallback completion. Zero new token types are introduced.

**Memoization rule (`QO-21`).** `CLOAD`-style verified-receipt reuse at an `OP_CALL` site is admitted only when the callee is typed, context-independent, and its receipt is valid at the call site — the amortization condition `QO-21` declares. The reuse key is the content digest of `(callee_id, arg bytes, qir-prof-ext-calls-1 version)`; a hit charges the `CLOAD` path (including its fallback-edge trip ceiling) instead of re-executing the callee body, and the retained receipt is checked under §6.7's complete-artifact rule before reuse. When the callee is context-dependent, the receipt is invalid, or the measured lookup cost is not below re-execution, direct `OP_CALL` execution is used. The study found reuse wins broadly at `m = 8, C = 1` (26–79% below Inline) but dynamic calls still lose at `C = 8` without reuse (+171% at `P = 64, m = 8`); memoization is therefore a declared optimization rule, not an unconditional default.

### 8.5 The Five Binding Obligations

The Step-6 Containers row fixes five rules; the call extension inherits them and adds its frame rule:

1. **Canonical serialization.** callee tables and frame bytes use the §2.6 word grammar and deterministic CBOR; a `(callee_id, arg bytes)` pair has one canonical encoding.
2. **Hash function.** SHA-256, 32 bytes: `0x02` empty/root base, `0x00` leaf, `0x20` pairwise, `0x01` trace nodes — matching P1 and `qir-trace-root-1`.
3. **Persistence.** COW structural persistence with no in-place mutation; a frame push creates a new commitment, and the suspended frame's pre-image is retained per F15.
4. **Cache model.** the `QO-17` tiers (L1d 3.4 ns, L2 4.8 ns, L3 11.2 ns, DRAM 125.0 ns) bound the access path; the 5,512 B P2 at `C=8,F=64` is a DRAM-bound measurement, not a core constant.
5. **Batch / frame rules.** sorted `batch_cnt` behavior is unchanged from §7; frame live values `<= F_max`, depth `<= C_max`, and `C_max · F_max` memory are declared profile ceilings (Obj 2).

### 8.6 Hand-Derived Fixtures `K1`–`K6` (Share 2 `--fixture K`)

The fixtures below are **test-only**, outside the 84-vector declared seed and the `C1`–`C6` container fixture set; they require no edit to `docs/qnir_goldens.md`. Listing notation is `block:index MNEMONIC operands -> bound value id`; `aN` are callee argument ids in their own instance space, `cN` are callee-local ids, `vN` caller ids. `B_hat` is the declared admission surrogate; `Work(E)` is the exact charged path; `n` counts executed transitions including a fatal rejection record where applicable. All amounts are U.

**`K1` — basic `OP_CALL`/`OP_RET` (`argc = 2 → retc = 1`).**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2  OP_CALL callee_id=0, argc=2, arg_refs=(v0,v1), retc=1, ret_bnds=(INT_POS<=255) -> v2
B0:3  HALT
C0:0  ADD a0, a1, bnd INT_POS<=255 -> c2
C0:1  OP_RET retc=1, ret_refs=(c2)
```

Declared `C_max=8`, `F_max=64`, `W_budget=60`. Hand-derived `B_hat = 50` (gateway 20 + 2·CONST 8 + CALL 8 + ADD 4 + RET 6 + HALT 4), `Work(E) = 50`, `n = 6`, outcome `SUCCESS`, tokens none. Sub-budget `r_sub = B_callee_body + charge(RET) = 4 + 6 = 10`; RET residue `10 − (4+6) = 0`; ledger `reserved=50, charged=50, refunded=0`.

**`K2` — nested acyclic calls, `depth = 3 <= C_max`; hash chain and SSA isolation.**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2  OP_CALL callee_id=0, argc=1, arg_refs=(v1), retc=1, ret_bnds=(INT_POS<=255) -> v2
B0:3  HALT
C0:0  OP_CALL callee_id=1, argc=1, arg_refs=(a0), retc=1, ret_bnds=(INT_POS<=255) -> c0
C0:1  OP_RET retc=1, ret_refs=(c0)
C1:0  OP_CALL callee_id=2, argc=1, arg_refs=(a0), retc=1, ret_bnds=(INT_POS<=255) -> c1
C1:1  OP_RET retc=1, ret_refs=(c1)
C2:0  CONST imm=1, bnd INT_POS<=255 -> c2
C2:1  ADD a0, c2, bnd INT_POS<=255 -> c3
C2:2  OP_RET retc=1, ret_refs=(c3)
```

Declared `W_budget=90`. `B_hat = Work(E) = 76` (gateway 20 + caller CONST 8 + 3·CALL 18 + C2 CONST/ADD 8 + 3·RET 18 + HALT 4), `n = 11`, outcome `SUCCESS`. Frame roots: `S0 = SHA-256(0x02)`, `S1 = SHA-256(S0 || frame0)`, `S2 = SHA-256(S1 || frame1)`, `S3 = SHA-256(S2 || frame2)`; P2 carries only `frame2` plus `parent_stack_root = S2` (1 live value, `<= 64`). Instance ids are monotone across all frames, so `v0/v1`, `c0/c1`, and `c2/c3` never alias; ledger `reserved=76, charged=76, refunded=0`.

**`K3` — `m = 6` sequential calls to a `k = 2`-loop callee; compositional admission.**

```
B0:0  CONST imm=5, bnd INT_POS<=255 -> v0
B0:1  CONST imm=7, bnd INT_POS<=255 -> v1
B0:2..B0:7  OP_CALL callee_id=0, argc=2, arg_refs=(v0,v1), retc=1, ret_bnds=(INT_POS<=255) -> v2..v7
B0:8  HALT
C0 block 0: ADD 4 U + BR 4 U -> block 1 / back-edge site 0 (trip ceiling 1)
C0 block 1: ADD 4 U + BR 4 U -> block 2 / back-edge site 1 (trip ceiling 1)
C0 block 2: ADD 4 U + JMP 4 U -> block 3
C0 block 3: OP_RET retc=1, ret_refs=(c_ret) = 6 U
```

Declared `W_budget=400`. The callee's worst-case interior bound is `B_callee_body = 16 + 16 + 8 + 6 = 46 U` with `dp_states(C0) = 16`. Inline state product `24 · 2^12 = 98,304 > 4096` → **refused**; compositional `B_hat(K3) = 20 + 4 + 4 + 6·(8 + 46) + 4 = 356 U` with caller + callee DP states `1 + 16 = 17 <= 4096` → **admitted**. On the fast non-looping runtime path (`8 + 8 + 8 + 6 = 30 U` per call), `Work(E) = 20 + 8 + 6·(8 + 30) + 4 = 260 U`, `n = 2 + 6·(1 + 7) + 1 = 51`, outcome `SUCCESS`. Ledger `reserved=356, charged=260, refunded=96` (`356 = 260 + 96`), using `charge(OP_CALL) = 4 + 2·2 = 8 U` and `charge(OP_RET) = 4 + 2·1 = 6 U` exactly. The first inline refusal remains `m = 5`.

**`K4` — inner fatal propagation and sub-budget conservation (two variants).**

```
K4a (DOMAIN_ERROR):
B0:0  CONST imm=1, bnd INT_POS<=255 -> v0
B0:1  OP_CALL callee_id=0, argc=1, arg_refs=(v0), retc=1, ret_bnds=(INT_POS<=255) -> v1
C0:0  DIV a0, ZERO, bnd INT_POS<=255 -> fatal
K4b (EXPRESSION_TOO_COMPLEX):
C0:0  CONST imm=256, bnd INT_POS<=255 -> fatal   (pre-dispatch bound exceedance)
```

Declared `W_budget=60`. Both variants: `B_hat = 20 + 4 + 6 + (4 + 6) + 4 = 44 U` (gateway + caller CONST + `OP_CALL` at `argc=1` + callee interior `DIV`/`CONST` 4 U + `OP_RET` at `retc=1` 6 U + caller HALT), `Work(E) = 20 + 4 + 6 + 0 = 30 U`, `n = 3`, no `OP_RET` and no caller `HALT` executed; terminal outcome `DOMAIN_ERROR` (tag 4) for `K4a` and `EXPRESSION_TOO_COMPLEX` (tag 3) for `K4b`, sealed at the fatal transition. Ledger `reserved=44, charged=30, refunded=14` (`44 = 30 + 14`) — conservation PASS with the outer caller's escrow intact. Zero new tokens.

**`K5` — token propagation across `OP_RET`.**

```
B0 initial pool: [STATUS_NEG_EVEN_ROOT]
B0:0  CONST imm=0, bnd INT_POS<=2^64-1 -> v0
B0:1  OP_CALL callee_id=0, argc=1, arg_refs=(v0), retc=0, ret_bnds=()   (no -> target: P10 defines 0 ids)
B0:2  HALT
C0:0  TBL_GET tbl_ref, key_ref=v0, fallback_blk=C1, bnd INT_POS<=255 -> c0
C0:1  OP_RET retc=0, ret_refs=()
C1:0  CONST imm=9, bnd INT_POS<=255 -> c9
C1:1  JMP edge->C0:1 (arity 0; successful fallback completion)
```

Declared `W_budget=70`, `cap_bnd=16` (so `TBL_GET` charges `4 + 4·depth_16(16) = 8 U`). `B_hat = Work(E) = 20 + 4 + 6 + 8 + 4 + 4 + 4 + 4 = 54 U` (gateway + caller CONST + `OP_CALL` at `argc=1` + `TBL_GET` + fallback CONST + JMP + `OP_RET` at `retc=0` 4 U + caller HALT), `n = 7`, outcome `NO_SOLUTION`, tokens: `STATUS_NEG_EVEN_ROOT` outstanding from the initial pool and never consumed; `STATUS_LOOKUP_MISS` born at `C0:0`, consumed only after `C1` completes successfully, and absent from the terminal verdict. No new token types; ledger `reserved=54, charged=54, refunded=0`. With no successful fallback completion the miss token stays outstanding and the outcome is still `NO_SOLUTION` — Tier O never alters the outcome.

**`K6` — static rejections (no execution).**

| Subcase | Declared input | Expected validator verdict |
|---|---|---|
| `K6a` | `OP_CALL callee_id=i` inside callee `i` (self-call), or any cycle `callee_id <= caller_id` | statically illegal — acyclic call-graph violation |
| `K6b` | call chain of length `9 > C_max = 8` | statically illegal — `C_max` exceedance |
| `K6c` | callee frame declaring `> F_max = 64` live values | statically illegal — `F_max` exceedance |
| `K6d` | `V.encoding = qir-enc-2` or `qir-enc-3` with envelope key `9` or opcode tag `30`/`31` | malformed / statically illegal — encoding-version dispatch |

Each: `B_hat = null`, `Work(E) = 0`, `n = 0` (zero inner postings), no terminal receipt beyond the declared refusal. These are validator-side checks; no execution transition is counted.

The fixture transcript's fixture blocks must be byte-identical between the two artifacts under §4.4's read-back discipline; their digests will be published in the Share 2 log. No fixture is a golden, and no fixture changes `E`, `n`, the corpus identity, or the 52 published trace-root lines of normal mode.

### 8.7 Acceptance Condition and Non-Discharge Register

Share 1 is **DISCHARGED** when this section is committed: the extension version pair is declared, preservation and the `V.encoding` dispatch gap are fixed, the key-9 callee table and acyclic rule are closed, the two opcode word grammars and frame commitment are fixed, the profile bounds/charges/compositional surrogate/sub-budget rules are declared, and `K1`–`K6` are hand-derived before implementation. The register remains open:

- **Share 2 — artifact/validator share**: **DISCHARGED at v0.3.0-draft.36**. Both implementations accept `qir-enc-4`/`qir-prof-ext-calls-1`, enforce key-9/tag-30–31 version gating, emit the byte-identical `K1`–`K6` fixture block (SHA-256[:16] `e83189cb00a56497`), and reproduce every §8.6 check exactly. The verification record is [`docs/logs/qnir/step6-2-call-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-2-call-artifact-share.md).
- **Remaining Step-6 extensions**: FUTURE — Saturation, Fork–join, and Road proofs, each with its own evidence.
- **`Q1` / `Q3`**: `QUEUED` — long-term multi-tier retention/residency remains deferred.
- **`Q6`**: `QUEUED` — this section introduces no term include for any unregistered slug; the only term includes used are `boc-policy`, `receipt`, and `computational-consistency`, all already in `docs/qnir.md`/`docs/qnir_semantics.md`.
- **Sealing / promotion**: not claimed. Under {% include term.html id="computational-consistency" %}, the validity claimed here is internal consistency within the declared extension boundary: §8 is additive to §1–§7, its call graph is statically acyclic and depth-bounded, its sub-budgets inherit §3.3 exactly, and nothing in it amends a canon file, a committed vector, or a glossary entry.

---

## License and Invariant Lineage

This companion is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the program of record. Its invariant lineage is the program of record's committed state at v0.3.0-draft.36: this file adds a new section (§8) whose declared semantics bind the committed §1–§7 and change no byte of them — no core or container opcode is re-tagged, no record field is added, no state component is introduced, no corpus or container fixture byte is re-versioned, and no glossary or navigation term is created. The step's own acceptance register in §8.7 is the sole discharge path for Share 1; the landing conditions are those of the draft — no promotion, no canon mutation, and QnIR remains `[PENDING]`.
