---
layout: default
title: QnIR Containers — The Step-6.1 Container Extension Specification (v0.3.0-draft.33)
description: QnIR v0.3 draft, §7 Step-6.1 extension share 1 — qir-enc-3, TAG_CONTAINER = 5, the thirteen closed extension opcodes, their §2.6 word grammar and static charges, and the hand-derived C1–C6 fixtures
version: 0.3.0-draft.33
updated: 2026-11-21
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_containers.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_containers.html
status: Working Draft — Step 6.1 Extension Share 1 (Not Promoted; QnIR remains [PENDING])
terms_used:
  - boc-policy
  - receipt
  - computational-consistency
---

# QnIR Containers — The Step-6.1 Container Extension Specification (v0.3.0-draft.33)

This file carries §7 of the QnIR v0.3 working draft, introduced at v0.3.0-draft.33 as **Step 6.1, Share 1** under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Like `docs/qnir_commitments.md`, this companion is a **new section**, not a moved monolith block: it has no predecessor in the v0.3.0-draft.5 monolith, and the §7 body below is its initial committed content. It is grounded in the measured trade study of [`docs/logs/qnir/step6-1-container-boc-study.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-1-container-boc-study.md) and in the five-row contract surfaces of [`docs/qstructures.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qstructures.md) v0.3.0; it does not edit either.

---

## 7. Container Extension (Step 6.1 — Extension Share 1)

This section declares the **Step-6.1 Container Extension**: encoding version **`qir-enc-3`** and profile version **`qir-prof-ext-containers-1`**. It is a specification share only. The artifact/validator share that implements the thirteen opcodes and the `C1`–`C6` fixtures is **Share 2**, forward of this wave. The extension is admitted to the draft's deferral register under the default-deny closure rule of `docs/qnir.md` §1.1: a construct enters only by an amendment to the repair sequence, adjudicated under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7, with the Step-6.1 study as the binding evidence. The amendment here is **additive and extension-scoped**; it does not amend core §1–§6.

### 7.1 Core Preservation and Amendment Binding

`qir-enc-3` **strictly extends `qir-enc-2`**. Every frozen core construct is byte-unchanged:

| Frozen surface | Status under `qir-enc-3` |
|---|---|
| The 16 core opcodes, tags `1`–`16` | Unchanged; no opcode is re-tagged, removed, or redefined |
| The four core value tags, tags `1`–`4` | Unchanged; tag `5` is added as the single new value tag (§7.2) |
| The 10-component configuration of §2.2 | Unchanged; container state is carried in component (3), the `Dynamic value store` — no new configuration component |
| The 14-field transition record of §2.6 | Unchanged in order, domain tags, and widths; no field is added or re-purposed |
| The `qir-trace-root-1` construction of §6.6 | Unchanged; container bytes enter only as §2.6 value bytes inside existing record fields |
| The 84 committed core vectors | Remain valid under `qir-enc-2`; no envelope, record, {% include term.html id="receipt" text="receipt" %}, or golden byte is re-versioned by Share 1 |

The version tuple on an extension Intent carries encoding `qir-enc-3` and profile `qir-prof-ext-containers-1`; a core-only Intent continues to carry `qir-enc-2` and `qir-prof-golden-2`. Mixed use is not a new semantic mode: the extension opcodes are simply unavailable to a `qir-enc-2` Intent, and an opcode tag in `17`–`29` under `qir-enc-2` is a statically illegal envelope (unknown opcode tag, §2.6). No new status token is introduced: the closed token vocabulary of §2.5 and the four terminal outcomes remain total over the extension's rejection surfaces.

### 7.2 The Fifth Value Tag and Container Subtypes

**`TAG_CONTAINER = 5`.** A container value is a declared value form in the §2.2 `Dynamic value store`, carrying a subtype word, a declared static capacity ceiling **`cap_bnd`**, and a declared element bound **`elem_bnd`** over the canonical value bytes of its members. Its canonical shape, declared inside P2, is the ordered structure **`[5, subtype, cap_bnd, elem_bnd, [[k_0, v_0], ...]]`**, with the `(key, value)` pairs in strictly ascending key/index order. The current **length `len`** is the pair count of that last component — it is derived canonically from the value, never a separate mutable field, and it satisfies `0 <= len <= cap_bnd`. The two subtypes are closed:

| Subtype word | Name | Backing structure | Niche |
|---:|---|---|---|
| `1` | `ARRAY_RRB16` | Persistent 16-way radix / relaxed-radix balanced tree | `Q(array)` — dense positional keys `0..N` |
| `2` | `TABLE_MBTREE16` | Persistent 16-way sorted Merkle B-tree | `Q(table)` — sparse 64-bit keys |

For `ARRAY_RRB16` each `k_i` is an `INT_POS` index in `0..len−1`; for `TABLE_MBTREE16` each `k_i` is an `INT_POS` key in `[0, 2^64)`. `elem_bnd` is an `INT_POS` upper bound on each member value under §2.7. `HAMT` and `ChunkedFlat` are **recorded as rejected** by the Step-6.1 BOC adjudication and are not subtype words: the rejection is a finding on the register, not an erasure (§3 step 3 of `docs/qstructures.md`). The quantitative grounds are in the study: 30×–300× lookup and ~50× deterministic fold with an 18.9 MB DRAM spill for `HAMT`, and the `N ≥ 4096` chunk-copy penalty for `ChunkedFlat`. A future subtype requires a new encoding generation, never an in-place addition to this closed set.

`cap_bnd` and `elem_bnd` are declared static profile parameters under Obj 2 and §2.7: finiteness enters only by declaration, no runtime `len` is a bound, and exceedance is a typed pre-dispatch rejection (§7.3). The profile ceiling `max_cap = 65536` is a declared profile bound, not a core validity limit; the measured evidence supports ceilings up to that value.

### 7.3 The Thirteen Closed Extension Opcodes (Tags 17–29)

The opcode set is closed at thirteen words, split by container type as the Step-6 Containers row of `docs/qnir.md` §1.3 requires. All thirteen operate inside component (3)'s `Dynamic value store`: `*_NEW` binds an empty container (`len = 0`), `*_LEN` binds the current length as an `INT_POS`, `*_GET`/`TBL_AT` bind members, `*_SET` binds a new version by overwrite or insert, `*_BATCH` binds one coalesced new version, and `*_ROOT` binds the 32-byte commitment as a four-word tuple. Every result is a **new value binding** in the store under §2.4's minting rule; no container is mutated in place. There is no `ARR_FOLD`/`TBL_FOLD` opcode and no dependence on Step-6.2 `OP_CALL`, closures, or call frames: deterministic sequential traversal is expressed with the core `ARR_LEN`/`ARR_GET` or `TBL_LEN`/`TBL_AT` plus `CMP`, `BR`, and block arguments (§7.6, `C1`/`C2`).

| Tag | Opcode | Operands | Declared deterministic effect |
|---:|---|---|---|
| `17` | `ARR_NEW` | `cap_bnd: bnd, elem_bnd: bnd` | Binds an empty `ARRAY_RRB16` (`len = 0`); a declared `cap_bnd > max_cap` is statically illegal |
| `18` | `ARR_LEN` | `arr_ref: ref, bnd: bnd` | Binds `INT_POS` current `len`, declared bound `<= cap_bnd` |
| `19` | `ARR_GET` | `arr_ref: ref, idx_ref: ref, bnd: bnd` | Binds the value at `0 <= idx < len`, re-checking `bnd`; `idx < 0`, `idx >= len`, or `idx >= cap_bnd` rejects as `EXPRESSION_TOO_COMPLEX` |
| `20` | `ARR_SET` | `arr_ref: ref, idx_ref: ref, val_ref: ref` | **Overwrite** when `0 <= idx < len`, or **contiguous append** when `idx == len` and `len < cap_bnd`; rejects `idx < 0`, `idx > len`, `idx >= cap_bnd`, or `val > elem_bnd` as `EXPRESSION_TOO_COMPLEX` |
| `21` | `ARR_BATCH` | `arr_ref: ref, batch_ref: ref, batch_cnt: cnt` | `batch_ref` is a §2.6 `TUPLE` of `batch_cnt` pairs `[idx: INT_POS, val]` in **strictly ascending unique index order**, `1 <= batch_cnt <= max_batch`; each index must overwrite or contiguously append within `cap_bnd`; one coalesced bottom-up Merkle rehash |
| `22` | `ARR_ROOT` | `arr_ref: ref` | Binds a 4-element `TUPLE` of 64-bit `INT_POS` big-endian words `(w0, w1, w2, w3)` encoding the container's 32-byte SHA-256 Merkle root (`SHA-256(0x02)` when empty) |
| `23` | `TBL_NEW` | `cap_bnd: bnd, elem_bnd: bnd` | Binds an empty `TABLE_MBTREE16` (`len = 0`); a declared `cap_bnd > max_cap` is statically illegal |
| `24` | `TBL_LEN` | `tbl_ref: ref, bnd: bnd` | Binds `INT_POS` current entry count `len`, declared bound `<= cap_bnd` |
| `25` | `TBL_GET` | `tbl_ref: ref, key_ref: ref, fallback_blk: blk, bnd: bnd` | `key_ref` is `INT_POS` in `[0, 2^64)`; hit binds the member value; miss births Tier-O `STATUS_LOOKUP_MISS`, checks/increments `(src_blk, src_idx, 0)` under the declared trip ceiling (F25), and routes to `fallback_blk` |
| `26` | `TBL_AT` | `tbl_ref: ref, rank_ref: ref, key_bnd: bnd, val_bnd: bnd` | Binds a 2-element `TUPLE(key_u64, val)` at ascending sorted rank `0 <= rank < len`, enabling deterministic sequential `BR` + block-argument fold loops with zero `OP_CALL`/closure dependency; out-of-bounds rank rejects as `EXPRESSION_TOO_COMPLEX` |
| `27` | `TBL_SET` | `tbl_ref: ref, key_ref: ref, val_ref: ref` | Inserts or replaces `(key_u64, val)`; on insert `len + 1 <= cap_bnd`, and `val <= elem_bnd`; otherwise `EXPRESSION_TOO_COMPLEX` |
| `28` | `TBL_BATCH` | `tbl_ref: ref, batch_ref: ref, batch_cnt: cnt` | `batch_ref` is a §2.6 `TUPLE` of `batch_cnt` pairs `[key: INT_POS, val]` in **strictly ascending unique `u64` key order**, `1 <= batch_cnt <= max_batch`; post-insert cardinality `<= cap_bnd` and every `val <= elem_bnd` |
| `29` | `TBL_ROOT` | `tbl_ref: ref` | Binds a 4-element `TUPLE` of 64-bit `INT_POS` words encoding the 32-byte SHA-256 root (`SHA-256(0x02)` when empty) |

#### 7.3.1 Exact §2.6 64-Bit Word Grammar

Per §2.6, every instruction is the opcode tag word, then its declared operand fields in declared order, then one uniform trailing **`ext`** word (the `EXT`-side-table bitmask). The operand kinds used here are exactly the §2.6 kinds: **`ref`** = one word (a static value id), **`blk`** = one word (a block id), **`cnt`** = one word (a declared count), **`bnd`** = the type-tag word followed by one bound value encoding (`INT_POS`: count word plus magnitude words). The exact sequences are:

| Tag | §2.6 word sequence (tag first, `ext` last) |
|---:|---|
| `17` | `17, bnd(cap_bnd), bnd(elem_bnd), ext` |
| `18` | `18, ref(arr), bnd(len <= cap_bnd), ext` |
| `19` | `19, ref(arr), ref(idx), bnd(val <= elem_bnd), ext` |
| `20` | `20, ref(arr), ref(idx), ref(val), ext` |
| `21` | `21, ref(arr), ref(batch), cnt(batch_cnt), ext` |
| `22` | `22, ref(arr), ext` |
| `23` | `23, bnd(cap_bnd), bnd(elem_bnd), ext` |
| `24` | `24, ref(tbl), bnd(len <= cap_bnd), ext` |
| `25` | `25, ref(tbl), ref(key), blk(fallback), bnd(val <= elem_bnd), ext` |
| `26` | `26, ref(tbl), ref(rank), bnd(key <= 2^64−1), bnd(val <= elem_bnd), ext` |
| `27` | `27, ref(tbl), ref(key), ref(val), ext` |
| `28` | `28, ref(tbl), ref(batch), cnt(batch_cnt), ext` |
| `29` | `29, ref(tbl), ext` |

For example, `cap_bnd = 16` as a `bnd` is the three words `0000000000000001` (type `INT_POS`), `0000000000000001` (count 1), `0000000000000010` (magnitude 16). A `cnt` operand is the count word itself, not a `ref`. The `ext` word is present on every instruction, including those with no other operands, exactly as §2.6 requires.

The canonical `TAG_CONTAINER = 5` value inside P2 is the §2.6 word encoding of **`[5, subtype, cap_bnd, elem_bnd, [[k_0, v_0], ...]]`**: type-tag word `5`, then the subtype word (`1` or `2`), then `cap_bnd` as an `INT_POS` value, then `elem_bnd` as an `INT_POS` bound, then one count word `n`, then `n` pair encodings, each `[key, value]` with `key` an `INT_POS` word encoding and `value` a complete §2.6 value encoding, strictly ascending by `key`/index. `n = len` is derived from that count; no separate length word exists, and a re-ordered or duplicated pair makes the P2 encoding malformed. An `ARR_ROOT`/`TBL_ROOT` result is the §2.6 `TUPLE` encoding `3, count=4, (1,1,w0), (1,1,w1), (1,1,w2), (1,1,w3)` — fourteen words — of the digest's big-endian 64-bit words.

**Rejection surface.** `*_GET` out-of-bounds access, `*_SET` invalid index or bound exceedance, `*_AT` out-of-range rank, `cap_bnd`/`elem_bnd` exceedance, and a `*_BATCH` whose indices/keys are duplicated, unsorted, or otherwise out of declared range are **typed rejections** yielding `EXPRESSION_TOO_COMPLEX` under §2.3/§2.5 — the token already exists and its terminalization is unchanged. A rejected batch binds no new version and posts only its own site-stable charge. `TBL_GET` is the sole miss-routing opcode and adds **zero** new status tokens: the miss is the existing F25 Tier-O token, observed/consumed only by a successful fallback completion (§2.5). Container values appear in the dynamic value store's canonical order under §2.6's `(static value id, instance id)` key; no container-specific store or configuration field is created.

### 7.4 The Five Binding Obligations

The Step-6 Containers row fixes five identical rules across the alternatives. All five are declared here once, binding both subtypes.

1. **Canonical Serialization (`qir-enc-3`).** The wire artifact of a container value is structure-independent: an `ARRAY_RRB16` serializes as a canonical ordered sequence of `(index, canonical_val_bytes)` pairs and a `TABLE_MBTREE16` as the canonical CBOR map `{u64 => canonical_val_bytes}`, in both cases alongside the declared subtype word and `cap_bnd`, exactly as the `[5, subtype, cap_bnd, elem_bnd, pairs]` layout of §7.2/§7.3.1 requires. Encoding is deterministic under §2.6's canonical rules; two containers with the same members, subtype, and declared ceilings have byte-equal artifacts regardless of their internal tree shape. The study's shared artifacts (9,195 / 151,275 bytes for dense `N = 256 / 4096`; 11,003 / 176,123 bytes for sparse `N = 256 / 4096`) are measured evidence, not encoding constants.

2. **Hash Function.** SHA-256, matching P1 and `qir-trace-root-1`: a content leaf commits `SHA-256(0x00 || key_le || val_bytes)`; each internal node commits `SHA-256(dom || meta || pairwise-root)` where the pairwise level is `SHA-256(0x20 || left || right)` with an odd final node **promoted unchanged** (never duplicated). The **empty root is `SHA-256(0x02)`**; a non-empty container with one member has the root equal to that member's `0x00` leaf, and larger containers fold pairwise with `0x20`. The root is a 32-byte string whose pre-image is retained per F15; equality decisions remain over canonical pre-images, and a genuine collision remains the Step-5 `qir-artifact-cmp-1` bucket's case. No new hash algorithm, width, or domain-separation rule is admitted.

3. **Persistence Policy.** Copy-on-write structural persistence: every `*_SET`/`*_BATCH` allocates a new root-to-leaf path and shares every untouched subtree; no node is mutated in place. A version is a new value binding under §2.4 and is committed by the existing pre/post state roots. **Long-term multi-tier retention classes remain `QUEUED` under `Q1` and `Q3`**: their trigger has not fired, and nothing here declares residency, eviction, or tier promotion. The measured footprint classes of `qir-prof-ext-containers-1` (obligation 4) are access-path cache tiers, not retention policy.

4. **Cache Model (`qir-prof-ext-containers-1`).** The profile binds the `QO-17` Layer-0 tier ladder of the record host (Intel Core i7-8565U): `L1d 3.4 ns`, `L2 4.8 ns`, `L3 11.2 ns`, `DRAM 125.0 ns`, measured by dependent pointer chase; cycles remain `ns × 1.80` estimates. The study's `ARRAY_RRB16` at `N = 65,536` is 7,549,656 bytes and therefore still **L3-resident inside the 8 MiB L3** through `N = 65536`, with a 15.5 ns median indexed `get`; the `TABLE_MBTREE16` counterpart at 8,445,528 bytes crosses into DRAM (54.8 ns median keyed `get`). These are declared tier bindings for cost accounting, not portability claims about other silicon.

5. **Batch Rules.** A batch is a `TUPLE` of `batch_cnt` strictly ascending unique `(key/index, value)` pairs with `1 <= batch_cnt <= max_batch = 64`, normalized by the declared order before the transition; duplicates and unsorted input are rejected (§7.3). The commit coalesces the bottom-up Merkle path rehash: shared ancestors are hashed once, bounded by the declared `batch_cnt` and never by a runtime `len`. The study measured `B = 64` (384–1,920 SHA-256 calls depending on subtype and N); `max_batch` is a profile constant, and the measured value is evidence for its declaration, not a core validity limit.

### 7.5 Static Cost Domain, Bounds, and `charge(t)` (§3.2 / §3.4)

The profile `qir-prof-ext-containers-1` declares the concrete constants: **`gateway = 20 U`**, **`trip_ceiling = 1`** per back-edge site (the single `CLOAD`/`TBL_GET` fallback slot `0` included), **`S_max = 4096`**, **`max_cap = 65536`**, and **`max_batch = 64`**. Core opcodes keep their `qir-prof-golden-2` charge of `4 U` each; the extension charges below replace that single base figure for tags `17`–`29`.

The declared static depth function over the **declared** `cap_bnd` is:

`depth_16(cap_bnd) = max(1, ceil(log_16(max(cap_bnd, 1))))`

giving exactly `1` for `cap_bnd <= 16`, `2` for `<= 256`, `3` for `<= 4096`, and `4` for `<= 65536`. It is a pure function of the declared `cap_bnd` — never the runtime `len`, never the runtime path depth, member length, or digest — so §3.2's cost domain is preserved and charges are site-stable and positive.

| Tags | Opcode(s) | Exact `charge(t)` |
|---:|---|---:|
| `17`, `23` | `ARR_NEW`, `TBL_NEW` | `4 U` |
| `18`, `24` | `ARR_LEN`, `TBL_LEN` | `4 U` |
| `22`, `29` | `ARR_ROOT`, `TBL_ROOT` | `4 U` |
| `19`, `25`, `26` | `ARR_GET`, `TBL_GET`, `TBL_AT` | `4 + 4 * depth_16(cap_bnd) U` |
| `20`, `27` | `ARR_SET`, `TBL_SET` | `4 + 6 * depth_16(cap_bnd) U` |
| `21`, `28` | `ARR_BATCH`, `TBL_BATCH` | `4 + batch_cnt * (4 + 2 * depth_16(cap_bnd)) U` |

Every charge is positive, so §3.4's fuel arithmetic remains strictly decreasing and the §3.5 termination certification unchanged. `B_hat(P) = B_cfg(P) >= B_sem(P)` is computed by the committed trip-ceiling DP with `gateway = 20 U`: a straight-line extension program contributes `20 + Σ charge(t)`, and a back-edge site may be traversed at most `trip_ceiling = 1` times, so the DP's maximizing path includes the body at most twice. **`TBL_GET`'s fallback edge `(src_blk, src_idx, 0)` is added to `B_hat(P)` identically to `CLOAD` in `bhat`** — it is a branch point at the `TBL_GET` instruction, the hit path continues in-block and the miss path enters `fallback_blk`, and the two are joined by the max rule; the miss token itself adds no charge class. The reserve/charge/refund verbs of §3.3 are inherited unchanged: no second budget, surrogate, or accumulator is introduced. The `QO-14` domination question stays where Layer-0 calibration left it: `U` orders declared work but does not by itself dominate wall time; the study's ns/cycles/SHA/allocation surface is diagnostic evidence for a future U calibration, not a substitute for one.

### 7.6 Declared Extension Fixture Surface (`C1`–`C6`, Share 2)

Six **hand-derived** fixtures are declared here for implementation in Share 2 (`src/qnir/` and `src/qnir-py/`) under the same test-only discipline as `F27`/`F6`: they are **not conforming golden vectors**, contribute nothing to the 84-vector declared seed, and require **no edit to `docs/qnir_goldens.md`**. All use `cap_bnd = 16`, `elem_bnd = 255`, hence `depth_16(16) = 1`; core opcodes charge `4 U`, `ARR_NEW`/`TBL_NEW`/`*_LEN`/`*_ROOT` charge `4 U`, `ARR_GET`/`TBL_GET`/`TBL_AT` charge `8 U`, `ARR_SET`/`TBL_SET` charge `10 U`, and `ARR_BATCH` charges `16 U` at `batch_cnt = 2`. Listing notation is `block:index MNEMONIC operands -> bound value id`; `vN` are static value ids and `aN`/`bN`/`cN` are block-argument ids. `B_hat` is the declared CFG bound including the gateway; `Work(E)` is the exact charged path; `n` counts executed transitions (records), including a fatal rejection record where applicable.

**`C1` — `ARRAY_RRB16` new/set/len and a `BR`-loop fold via `ARR_LEN`/`ARR_GET`.**

```
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5,  bnd INT_POS<=255 -> v1
B0:2  CONST imm=0,  bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3              (append at idx 0; len=1)
B0:4  ARR_LEN v3, bnd INT_POS<=16 -> v4      (binds 1)
B0:5  CONST imm=0, bnd INT_POS<=16 -> v5     (idx counter)
B0:6  JMP edge->B1(v3, v5, v4)              (arity 3)
B1:0  CMP a1, a2, rng[-1,1] -> v6
B1:1  BR LT v6, true->B2(a0,a1,a2), false->B3(a0)
B2:0  ARR_GET b0, b1, bnd INT_POS<=255 -> v7
B2:1  CONST imm=1, bnd INT_POS<=16 -> v8
B2:2  ADD b1, v8, bnd INT_POS<=16 -> v9
B2:3  JMP edge->B1(b0, v9, b2)              (back-edge, traversal #1)
B3:0  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=100 U`.
- Hand-derived `B_hat(P) = 20 + 74 = 94 U`; `Work(E) = 94 U`; `n = 16`; terminal outcome `SUCCESS`.
- Fold proof: the header `B1` is entered with `idx=0, len=1`, the `BR` takes `true`, `B2` reads index `0` with `ARR_GET`, increments to `1`, and the single back-edge traversal returns to `B1`; the second header test is `1 < 1 = false`, so `B3` halts. The loop body runs once with **zero** `OP_CALL`/closure dependency.

**`C2` — `TABLE_MBTREE16` new/set/len and a `BR`-loop fold via `TBL_LEN`/`TBL_AT`.**

```
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=2^64-1 -> v2   (key 0)
B0:3  TBL_SET v0, v2, v1 -> v3                  (insert; len=1)
B0:4  TBL_LEN v3, bnd INT_POS<=16 -> v4         (binds 1)
B0:5  CONST imm=0, bnd INT_POS<=16 -> v5        (rank)
B0:6  JMP edge->B1(v3, v5, v4)
B1:0  CMP a1, a2, rng[-1,1] -> v6
B1:1  BR LT v6, true->B2(a0,a1,a2), false->B3(a0)
B2:0  TBL_AT b0, b1, bnd INT_POS<=2^64-1, bnd INT_POS<=255 -> v7
B2:1  CONST imm=1, bnd INT_POS<=16 -> v8
B2:2  ADD b1, v8, bnd INT_POS<=16 -> v9
B2:3  JMP edge->B1(b0, v9, b2)                  (back-edge, traversal #1)
B3:0  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=100 U`.
- Hand-derived `B_hat(P) = 20 + 74 = 94 U`; `Work(E) = 94 U`; `n = 16`; terminal outcome `SUCCESS`.
- Fold proof: identical shape to `C1`, with `TBL_AT` binding the rank-0 `(key, val)` tuple and the same single allowed back-edge traversal; the key-order fold needs no closure or call frame.

**`C3` — `TBL_GET` miss fallback and the F25 trip ceiling.**

```
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=0, bnd INT_POS<=2^64-1 -> v1
B0:2  TBL_GET v0, v1, fallback_blk=1, bnd INT_POS<=255 -> v2
B0:3  HALT                         (hit path only; not executed on this miss)
B1:0  CONST imm=9, bnd INT_POS<=255 -> v3
B1:1  HALT                         (successful fallback completion)
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=50 U`.
- Hand-derived `B_hat(P) = 20 + max(24 miss, 20 hit) = 44 U`; `Work(E) = 44 U`; `n = 5`; terminal outcome `SUCCESS`.
- Miss behavior: the empty table misses at `B0:2`; Tier-O `STATUS_LOOKUP_MISS` is born, the `(B0, 2, 0)` trip-ceiling check permits the one fallback-edge traversal (ceiling 1), `B1` completes successfully and consumes the token; the receipt itemizes it as consumed, and no new token type or rejection is produced.

**`C4` — coalesced `ARR_BATCH` and duplicate rejection (two declared runs).**

```
C4a (accepted):
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  CONST imm=7, bnd INT_POS<=255 -> v3
B0:4  CONST imm=1, bnd INT_POS<=16  -> v4
B0:5  TUPLE 2 fields [INT_POS<=16, INT_POS<=255] (v2, v1) -> v5
B0:6  TUPLE 2 fields [INT_POS<=16, INT_POS<=255] (v4, v3) -> v6
B0:7  TUPLE 2 fields [TUPLE, TUPLE] (v5, v6) -> v7
B0:8  ARR_BATCH v0, v7, cnt=2 -> v8
B0:9  HALT

C4b (rejected) — identical to C4a except B0:6 uses (v2, v3),
                 i.e. duplicate index 0 instead of strictly ascending 0,1.
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=80 U`.
- `C4a`: `B_hat(P) = 20 + 52 = 72 U`; `Work(E) = 72 U`; `n = 10`; outcome `SUCCESS`; one coalesced commit, bottom-up rehash of the shared ancestor once, `len = 2`.
- `C4b`: `B_hat(P) = 72 U` (same static program); `Work(E) = 68 U`; `n = 9`; outcome `EXPRESSION_TOO_COMPLEX` at `B0:8`, before any version is bound; the static `HALT` on the success path is not executed.

**`C5` — `cap_bnd` and `ARR_GET` bounds rejections.**

```
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3                 (append; len=1)
B0:4  CONST imm=1, bnd INT_POS<=16  -> v4      (idx == len)
B0:5  ARR_GET v3, v4, bnd INT_POS<=255 -> fatal
B0:6  HALT                                    (success path only)
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=60 U`.
- Hand-derived `B_hat(P) = 20 + 38 = 58 U` (the static `B0:6 HALT` is on the success path); `Work(E) = 54 U`; `n = 6`; terminal outcome `EXPRESSION_TOO_COMPLEX` at `B0:5` (`idx == len`), itemized with the existing fatal token. A declared `cap_bnd > max_cap = 65536` is additionally rejected statically at envelope validation (not an execution vector), so it has no `B_hat`/`Work`/`n`.

**`C6` — canonical and root equivalence across subtypes.**

```
C6a (array):
B0:0  ARR_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=16  -> v2
B0:3  ARR_SET v0, v2, v1 -> v3
B0:4  ARR_ROOT v3 -> v4
B0:5  EMIT cnt(kind)=0, cnt(1), ref v4
B0:6  HALT

C6b (table):
B0:0  TBL_NEW cap_bnd=16, elem_bnd=255 -> v0
B0:1  CONST imm=5, bnd INT_POS<=255 -> v1
B0:2  CONST imm=0, bnd INT_POS<=2^64-1 -> v2
B0:3  TBL_SET v0, v2, v1 -> v3
B0:4  TBL_ROOT v3 -> v4
B0:5  EMIT cnt(kind)=0, cnt(1), ref v4
B0:6  HALT
```

- Declared `cap_bnd=16`, `elem_bnd=255`, `W_budget=60 U` for both.
- Each run: `B_hat(P) = 20 + 34 = 54 U`; `Work(E) = 54 U`; `n = 7`; outcome `SUCCESS`.
- Exact roots. Empty root before the insert, both subtypes: `SHA-256(0x02) = dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986`, words `(0xdbc1b4c900ffe48d, 0x575b5da5c6380401, 0x25f65db0fe3e2449, 0x4b76ea986457d986)`. After inserting `key = 0`, `val = INT_POS 5` (canonical `val_bytes` = `0000000000000001 0000000000000001 0000000000000005`), the one-member root is the `0x00` leaf itself, `SHA-256(0x00 || 0u64_le || val_bytes) = 3874f4bb0de71f1135773da3d2f9fdf985c3f9862aa5ad6fbb689f7b9127c8d4`, words `(0x3874f4bb0de71f11, 0x35773da3d2f9fdf9, 0x85c3f9862aa5ad6f, 0xbb689f7b9127c8d4)`, **identical for `C6a` and `C6b`**; no `0x20` node is formed at cardinality one. The serialized P2 shapes differ only as declared (ordered sequence versus map) while the `(key, value)` pairs, leaf hash, and root are equal.

The fixture transcripts' fixture blocks must be byte-identical between the two artifacts under the §4.4 read-back discipline; their digests will be published in the Share 2 log. No fixture is a golden, no fixture changes `E`, `n`, the corpus identity, or the 52 published trace-root lines of normal mode.

### 7.7 Acceptance Condition and Non-Discharge Register

Share 1 is **DISCHARGED** when this section is committed: the extension version pair is declared, the five-row contract is filled for both admitted subtypes, the thirteen opcodes and the fifth value tag are closed with their exact §2.6 word grammar, the five binding obligations and the concrete profile bounds/`charge(t)` formulas are fixed, and `C1`–`C6` are hand-derived with exact sequences and numbers before any implementation. The register remains open:

- **Share 2 — artifact/validator share**: FUTURE. Both implementations must accept `qir-enc-3`/`qir-prof-ext-containers-1`, emit the C-fixture blocks, and reproduce every declared check byte-identically; the corpus stays untouched.
- **Remaining Step-6 extensions**: FUTURE — Calls, Saturation, Fork–join, and Road proofs, each with its own owning study and evidence before admission.
- **`Q1` / `Q3`**: `QUEUED` — no long-term multi-tier retention/residency contract is declared here.
- **`Q6`**: `QUEUED` — this section introduces no term include for any unregistered slug; the only term includes used are `boc-policy`, `receipt`, and `computational-consistency`, all already in `docs/qnir.md`/`docs/qnir_semantics.md`.
- **Sealing / promotion**: not claimed. Under {% include term.html id="computational-consistency" %}, the validity claimed here is internal consistency within the declared extension boundary: §7 is additive to §1–§6, its opcodes are closed and typed, its bindings repeat the Step-6.1 study's rules verbatim in substance, and nothing in it amends a canon file, a committed vector, or a glossary entry.

---

## License and Invariant Lineage

This companion is licensed under the [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the program of record. Its invariant lineage is the program of record's committed state at v0.3.0-draft.33: this file adds a new section (§7) whose declared semantics bind the committed §1–§6 and change no byte of them — no opcode is re-tagged, no record field is added, no state component is introduced, no corpus vector is re-versioned, and no glossary or navigation term is created. The step's own acceptance register in §7.7 is the sole discharge path for Share 1; the landing conditions are those of the draft — no promotion, no canon mutation, and QnIR remains `[PENDING]`.
