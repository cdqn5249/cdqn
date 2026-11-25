---
layout: default
title: QnIR Bootstrap — The Step-6.3a Bootstrap Profile Specification (v0.3.0-draft.37)
description: QnIR v0.3 draft, §9 Step-6.3a extension share 1 — qir-enc-5, six closed bootstrap opcodes (BAND/BOR/BXOR/BSHL/BSHR/Q_HASH), SCC-additive B_hat, the qir-prof-ext-bootstrap-1 static cost domain, and hand-derived B1-B6 fixtures
version: 0.3.0-draft.37
updated: 2026-11-25
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL 1.0)
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnir_bootstrap.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnir_bootstrap.html
status: Working Draft — Step 6.3a Extension Share 1 (Not Promoted; QnIR remains [PENDING])
terms_used:
  - boc-policy
  - computational-consistency
  - licensed-work
  - receipt
---

# QnIR Bootstrap — The Step-6.3a Bootstrap Profile Specification (v0.3.0-draft.37)

This file carries **§9** of the QnIR v0.3 working draft, introduced at `v0.3.0-draft.37` as **Step 6.3a, Share 1** under the committed multi-file convention with global section numbering preserved. The program of record — status, Wave records, repair ladder, and References — lives in [`docs/qnir.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/qnir.md); this companion inherits its DRAFT status and is not promoted to the canon. Like `docs/qnir_containers.md` and `docs/qnir_calls.md`, this companion is a **new section**, not a moved monolith block: it has no predecessor in the v0.3.0-draft.5 monolith, and the §9 body below is its initial committed content.

The section is grounded in the read-only measurement record [`docs/logs/qnir/step6-3-bootstrap-trade-study.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-3-bootstrap-trade-study.md) and the three Step-6.3 Bootstrap-Profile prerequisites of `docs/qnlang.md` §7.3. No §1–§8 byte, golden vector, fixture, glossary entry, or navigation entry is edited by this share. No `docs/qnir.md` §1.3 pointer is added: the §1.3 register names the five *deferred* families (Calls, Containers, Saturation, Fork–join, Road proofs), whereas this share introduces a new §9 section for three *prerequisite* operations; the Step-6.1/6.2 pointers were inline amendments to existing deferral rows, and there is no corresponding row to amend here. Saturation (`OP_SATURATE`, `QO-28`) remains explicitly deferred to **Step 6.3b** (§9.7).

---

## 9. Bootstrap Profile Extension (Step 6.3a — Extension Share 1)

This section declares the **Step-6.3a Bootstrap Extension**: encoding version **`qir-enc-5`** and profile version **`qir-prof-ext-bootstrap-1`**. It is a specification share only; the artifact/validator share that implements tags `32`–`37` and the `--fixture B` surface is **Share 2**, forward of this wave. The extension is adjudicated under the {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7 with the Step-6.3a study as binding evidence; the retained candidate is **closed, fixed-width, statically bounded** — no general bitwise vector width, no unbounded hash payload, and no saturation construct enters the extension.

### 9.1 Core, Container & Call Preservation; the `V.encoding` Gate

`qir-enc-5` **strictly extends `qir-enc-4`**. Every frozen surface below is byte-unchanged:

| Frozen surface | Status under `qir-enc-5` |
|---|---|
| Core opcodes `1`–`16` | Unchanged; no core opcode is re-tagged, removed, or redefined |
| Container opcodes `17`–`29` and callee-table key `9` | Unchanged |
| Call opcodes `30`–`31` and all §8 frame/sub-budget/token rules | Unchanged |
| Value tags `1`–`5` and the `ARR_ROOT`/`TBL_ROOT` 4-word tuple layout | Unchanged; no sixth value tag is introduced |
| Envelope keys `0`–`9` | Unchanged; no new envelope key is introduced by this share |
| The 10-component configuration of §2.2 and the 14-field record of §2.6 | Unchanged in component count, field order, domain tags, and widths |
| `qir-enc-2` core-only Intents | Remain valid and byte-identical; tags `32`–`37` are illegal there |
| `qir-enc-3` container Intents | Remain valid and byte-identical; tags `32`–`37` are illegal there |
| `qir-enc-4` call Intents | Remain valid and byte-identical; tags `32`–`37` are illegal there |
| The 84 core vectors plus the `C1`–`C6`, `K1`–`K6` fixtures | Remain valid under their own encoding versions; no envelope, record, {% include term.html id="receipt" text="receipt" %}, or golden byte is re-versioned |

**Mandatory validator rule.** `decode_envelope` and `dec_insn` MUST inspect `V.encoding`: tags `32`–`37` are admissible **only** when `V.encoding = qir-enc-5` and are rejected by `qir-enc-2`, `qir-enc-3`, and `qir-enc-4` as malformed under §2.6's total unknown-field rejection. A tag outside `1`–`37`, or an envelope key outside `0`–`9`, is malformed. **Zero new status tokens and zero new terminal outcomes are introduced**: the closed token vocabulary of §2.5 and the four terminal outcomes remain total over every rejection surface declared below. A rejected instruction posts zero under F14 (`docs/qnir_accounting.md` §3.2/§3.5), and the escrow settlement shape is unchanged.

### 9.2 The Five Bitwise Opcodes (Tags `32`–`36`)

All five use §2.6's uniform trailing `ext` word. `ref` is one word, `bnd` is the canonical `INT_POS` bound sequence (type-tag word, count word, magnitude words), and `tag`, `ext` are one word each. Operand values are exact integers carried over a `w`-bit transport with `w ∈ {8, 16, 32, 64}`; `bnd` MUST be `INT_POS` with `0 <= bnd <= 2^w - 1`.

| Tag | Opcode | A§2.6 64-bit word sequence | Result |
|---:|---|---|---|
| `32` | `BAND` | `32, ref(a), ref(b), bnd(w), ext` | bitwise AND of `a`, `b`, masked to `w` bits |
| `33` | `BOR` | `33, ref(a), ref(b), bnd(w), ext` | bitwise OR of `a`, `b`, masked to `w` bits |
| `34` | `BXOR` | `34, ref(a), ref(b), bnd(w), ext` | bitwise XOR of `a`, `b`, masked to `w` bits |
| `35` | `BSHL` | `35, ref(a), ref(s), bnd(w), ext` | `(a << s) & (2^w - 1)` |
| `36` | `BSHR` | `36, ref(a), ref(s), bnd(w), ext` | logical `a >> s` over `w` bits |

**Pre-dispatch rejection.** Before any effect posts, the validator checks (a) `bnd` is a canonical `INT_POS` whose value is in `1..=2^w-1` for the declared `w` and (b) for `BAND`/`BOR`/`BXOR`, both operand values are non-negative and `< 2^w`; for `BSHL`/`BSHR`, the shift amount `s` satisfies `0 <= s < w` and `a` is non-negative and `< 2^w`. A negative operand, an operand `>= 2^w`, `s >= w`, or an invalid `bnd` births `EXPRESSION_TOO_COMPLEX` fatally at that transition (§2.5); the rejecting transition is counted in `n` but posts **`0 U`** (§3.1), no value instance is bound, and no existing instruction or golden byte is re-versioned. No new token is born. The four choices of `w` are the **closed** width set: `w` outside that set is a static illegality at encode time, not a runtime branch.

### 9.3 `Q_HASH` (Tag `37`)

| Tag | Opcode | A§2.6 64-bit word sequence | Result |
|---:|---|---|---|
| `37` | `Q_HASH` | `37, ref(v), cnt(b_bnd), ext` | 4-word `TUPLE` of the SHA-256 digest of `v`'s canonical §2.6 bytes |

`b_bnd` is a declared immediate carried in one `cnt` word with `1 <= b_bnd <= max_hash_bytes = 65536`. The hash input is the **exact canonical §2.6 encoding bytes** of the referenced value `v` (not its rendered text, not a digest of a digest, and not a header-only comparison). `Q_HASH` computes `SHA-256(canonical_bytes(v))` and binds a 4-element `TUPLE` of 64-bit `INT_POS` big-endian words `(w0, w1, w2, w3)` — the identical layout used by `ARR_ROOT` (tag `22`) and `TBL_ROOT` (tag `29`, §7.3.1), each `w_i` the corresponding 8-byte big-endian digest chunk. The algorithm tag **`sha-256`** is part of the emitted identity (`sha-256:<hex>`); a bare un-tagged digest is not admissible across a gateway under the migration-witnessing rule of `docs/simemp.md` §4.3.

If `canonical_bytes(v).len() > b_bnd`, the operation rejects **pre-dispatch within its own transition** with `EXPRESSION_TOO_COMPLEX`: that transition is counted in `n` but posts **`0 U`** (§3.1), binds no tuple, and leaves the unspent escrow to be refunded at terminal settlement (§3.3). The check uses the declared bound, never a runtime length in a charge formula (F1/F14). Under F6 and `QO-24`, `Q_HASH` is solely an identity/commitment primitive: value equality remains full canonical-encoding comparison with `qir-artifact-cmp-1` collision buckets, never digest comparison.

### 9.4 SCC-Additive `B_hat` Admission

The committed global-product surrogate of §3.4 computes the structural state count as `|B| · Π_i (ceil_i + 1)` over all declared back-edge sites, which overflows the admission gate on graphs the SCC structure can bound finitely. This section replaces that **structural count** with an exact SCC decomposition while preserving the DP semantics:

1. **Tarjan condensation.** Compute strongly connected components over the static block graph in `O(|V| + |E|)`, deterministically by ascending block id. Every declared back-edge lies inside one SCC (its target dominates its source).
2. **Per-SCC trip-ceiling DP.** For component `i`, run the existing exit-aware trip-ceiling DP restricted to that component: each local path is summarized by the maximum charge at each boundary (a `HALT` inside the component, or the component entered by each cross-SCC edge), and each local back-edge consumes its declared `trip` ceiling. A fallback edge leaving the component is charged exactly once — the local summary carries the charge to the boundary and the successor's solved value is added during aggregation.
3. **Condensation-DAG max-path aggregation.** Condensation is acyclic; aggregate the local summaries along the DAG, taking the maximum over reachable exits. The gateway charge is posted once at the front.

**Structural gate (pre-DP, overflow-checked).**

```
Σ_i |SCC_i| · Π_{e ∈ SCC_i} (ceil_e + 1)  <=  S_max = 4096
```

An overflow or a sum above `S_max` refuses before any DP table is allocated; no partial or truncated DP is admitted. The per-component DP and the gate are profile data riding `V`, not core validity limits (Obj 2). **Value identity is preserved**: `B_hat_scc(P) == B_hat_global(P)` on every globally admissible graph — verified on all **51/51** committed core `X` vectors in the Step-6.3a study, and on all admissible synthetic sequential/nested graphs (§9.6 `B4`, `B5`). Where the global product refuses, the SCC form admits with a finite bound and a numerically identical `B_hat` for every path the global DP could have certified. `B_hat(P) >= Work(E)` and `reserved = charged + refunded` (§3.3) are unchanged.

### 9.5 Static Cost Domain (`qir-prof-ext-bootstrap-1`)

The profile declares: `gateway = 20 U`, `trip_ceiling = 1`, `S_max = 4096`, `C_max = 8`, `F_max = 64`, `max_cap = 65536`, `max_batch = 64`, and `max_hash_bytes = 65536`. Core opcodes keep their §3.2 `4 U` class; container opcodes keep their `qir-prof-ext-containers-1` charges; call opcodes keep their `qir-prof-ext-calls-1` charges. The new charges are site-stable, positive, and functions of declared data only:

- **Tags `32`–`36` (bitwise): `4 U` each.** This preserves the core minimum positive base of `docs/qnir_accounting.md` §3.2 while replacing a measured `388`–`418 U` + `24,576 B` arithmetic emulation with a `64 B` (or `72 B` for an immediate-shift candidate) native form — a ~100× work reduction on the recorded host. No operand value or runtime width enters the charge; the declared `w` selects admissibility, not price.
- **Tag `37` (`Q_HASH`): `160 + 130 * ceil(b_bnd / 64)` U.** Calibrated under F14 from the Step-6.3a measurements (`182/329/448/717/1211/2210 U` at `b_bnd = 32/64/128/256/512/1024`, fit intercept `160.3 U`, slope `129.2 U` per 64-byte block). The argument is the **declared** `b_bnd`, never the runtime byte length; a flat `4 U` hash charge is prohibited as under-billing. The digest's emitted identity is `sha-256:<hex>`.

`W_worst` for BOC Stage 2 is computed from these declared charges, never from the measured nanosecond samples. Portability (`P_gap`) remains UNRESOLVED until a required profile set `R` is declared with single-host-independent evidence (Step-6.3a Stage-2 record).

### 9.6 Hand-Derived Fixtures `B1`–`B6` (Share 2 `--fixture B`)

The fixtures below are **test-only**, outside the 84-vector declared seed and the `C1`–`C6`/`K1`–`K6` sets; they require no edit to `docs/qnir_goldens.md`. Listing notation is `block:index MNEMONIC operands -> bound value id`. `B_hat` is the declared admission surrogate, `Work(E)` the exact charged path, `n` the executed transitions including a fatal rejection transition (a static rejection has `n = 0`). Under §3.1 a fatal rejecting instruction posts **`0 U`**: it is counted in `n` but contributes nothing to `Work(E)`, and the unspent reserve is refunded at terminal settlement, so `reserved = B_hat = charged + refunded` throughout. All amounts are U.

**`B1` — `u64` pack/unpack round-trip with all five bitwise ops.**

```
B0:0  CONST imm=165 (0xA5), bnd INT_POS<=2^64-1 -> v0
B0:1  CONST imm=15  (0x0F), bnd INT_POS<=2^64-1 -> v1
B0:2  CONST imm=4, bnd INT_POS<=64 -> v2
B0:3  BAND v0, v1, w=64 -> v3        (= 0x05)
B0:4  BSHR v0, v2, w=64 -> v4        (= 0x0A)
B0:5  BSHL v4, v2, w=64 -> v5        (= 0xA0)
B0:6  BOR  v5, v3, w=64 -> v6        (= 0xA5, the repacked original)
B0:7  BXOR v6, v0, w=64 -> v7        (= 0x00, round-trip witness)
B0:8  HALT
```

Hand-derived: `B_hat = 56` (gateway 20 + 3×CONST 12 + 5×bitwise 20 + HALT 4), `Work(E) = 56`, `n = 9`, outcome `SUCCESS`, `v6 = 0xA5`, `v7 = 0`, tokens none, `reserved=56, charged=56, refunded=0`.

**`B2` — pre-dispatch bitwise rejections (three cases).**

```
B2a B0:0 CONST imm=1, bnd INT_POS<=2^64-1 -> v0
    B0:1 CONST imm=8, bnd INT_POS<=64 -> v1
    B0:2 BSHL v0, v1, w=8 -> REJECT (s = 8 >= w = 8)
B2b B0:0 CONST imm=256, bnd INT_POS<=2^64-1 -> v0
    B0:1 CONST imm=1, bnd INT_POS<=2^64-1 -> v1
    B0:2 BAND v0, v1, w=8 -> REJECT (256 >= 2^8)
B2c B0:0 CONST imm=1, bnd INT_POS<=2^64-1 -> v0
    B0:1 CONST imm=16, bnd INT_POS<=64 -> v1
    B0:2 BSHR v0, v1, w=16 -> REJECT (s = 16 >= w = 16)
```

Each: `B_hat = 32` (gateway 20 + 2×CONST 8 + bitwise 4), `Work(E) = 28` (the rejecting instruction posts zero), `n = 3`, outcome `EXPRESSION_TOO_COMPLEX`, no value bound, `reserved=32, charged=28, refunded=4`.

**`B3` — `Q_HASH` on canonical `INT_POS 5`, plus exceedance `B3b`.**

```
B3  B0:0 CONST imm=5, bnd INT_POS<=2^64-1 -> v0
    B0:1 Q_HASH v0, b_bnd=64, ext -> v1
    B0:2 HALT
B3b B0:0 CONST imm=5, bnd INT_POS<=2^64-1 -> v0
    B0:1 Q_HASH v0, b_bnd=16, ext -> REJECT (24 bytes > 16)
```

The canonical §2.6 bytes of `INT_POS 5` are the 24 bytes

```
0000000000000001 0000000000000001 0000000000000005
```

with exact SHA-256

```
1fab775418d8ed7a71155cd2d0afb0998718f32b4bd55271eb24daa9bbe20eff
```

bound as the 4-word `TUPLE` of full 64-bit `INT_POS` words `(0x1fab775418d8ed7a, 0x71155cd2d0afb099, 0x8718f32b4bd55271, 0xeb24daa9bbe20eff)`, matching the §7.3.1 `ARR_ROOT`/`TBL_ROOT` layout.

`B3`: `Q_HASH` charge `160 + 130·ceil(64/64) = 290 U`; `B_hat = 318` (20 + CONST 4 + 290 + HALT 4), `Work(E) = 318`, `n = 3` (CONST, `Q_HASH`, HALT), outcome `SUCCESS`, tokens none, `reserved=318, charged=318, refunded=0`.

`B3b`: `B_hat = 318` (the declared bound is unchanged), `Work(E) = 24` (20 + CONST 4; `Q_HASH` posts zero), `n = 2` (CONST, rejecting `Q_HASH`; HALT is never reached), outcome `EXPRESSION_TOO_COMPLEX`, no tuple bound, `reserved=318, charged=24, refunded=294`.

**`B4` — `k = 12` sequential loop sites, `trip = 1`.**

```
B_i (i = 0..10):  CLOAD addr=0 fallback=B_i; JMP B_(i+1)
B_11:             CLOAD addr=0 fallback=B_11; HALT
```

Global product: `|B| · 2^12 = 12 · 4096 = 49152 > 4096` → **refused** before the DP. SCC-additive: `Σ_i |SCC_i| · (1+1)^1 = 12 · 2 = 24 <= 4096` → **admitted**. Hand-derived `B_hat = 164` (gateway 20 + 12 independent loop sites × `[2·CLOAD 4 + (HALT/JMP) 4] = 12×12`), `Work(E) = 116` (hit path: 12×CLOAD 4 + 11×JMP 4 + HALT 4 + gateway 20), `n = 24`, outcome `SUCCESS`, `reserved=164, charged=116, refunded=48`. No vector byte is re-versioned by this admission widening.

**`B5` — nested two-loop SCC parity.**

```
B0:0  CLOAD addr=0 fallback=B1
B0:1  JMP B2
B1:0  CLOAD addr=0 fallback=B1
B1:1  JMP B0
B2:0  HALT
```

Two back-edges in one SCC; global product `3 · 2^2 = 12`, SCC sum `2·2^2 + 1·1 = 9`, both `<= 4096`. Hand-derived `B_hat_scc = B_hat_global = 48` U, `Work(E) = 32` (gateway 20 + CLOAD 4 + JMP 4 + HALT 4), `n = 3`, outcome `SUCCESS`, `reserved=48, charged=32, refunded=16`.

**`B6` — static rejections.**

```
B6a  envelope with tag 32 under qir-enc-4  -> REJECTION_BY_GATEWAY
B6b  BAND with bnd encoding w = 128          -> REJECTION_BY_GATEWAY (w not in closed set)
B6c  Q_HASH with b_bnd = 65537               -> REJECTION_BY_GATEWAY (> max_hash_bytes)
```

Each: `B_hat = null`, `Work(E) = 0`, `n = 0`, no postings, no rejection transition (the escrow never opens), outcome `REJECTION_BY_GATEWAY`, zero value bindings, no new token type — the `K6` static-rejection shape of §8.6.

### 9.7 Acceptance Condition and Non-Discharge Register

Share 1 is **DISCHARGED** when this section is committed: the extension version pair is declared, preservation and the `V.encoding` gate are fixed, the six opcode word grammars and their rejection surfaces are closed, the SCC-additive admission rule with its pre-DP gate is declared with 51/51 parity evidence, the static cost domain is fixed, and `B1`–`B6` are hand-derived before implementation. The register remains open:

- **Share 2 — artifact/validator share (`--fixture B`)**: **DISCHARGED**. Both artifacts accept `qir-enc-5`/`qir-prof-ext-bootstrap-1`, enforce tag `32`–`37` version gating, and reproduce the byte-identical `B1`–`B6` fixture block (1,209 bytes, SHA-256[:16] `b6e8626488aa936a`) and every §9.6 check, including the `B3` digest and the `B4` global-refusal/SCC-admission pair. `cargo test --offline` is 23/23 in `src/qnir/` with no regression in the five existing modes. The verification record is [`docs/logs/qnir/step6-3a-bootstrap-artifact-share.md`](https://github.com/cdqn5249/cdqn/blob/main/docs/logs/qnir/step6-3a-bootstrap-artifact-share.md).
- **Step 6.3b — `OP_SATURATE` / `QO-28`**: **FUTURE and explicitly deferred**. Per `docs/qexpr.md` §4.6.2 it requires a pinned deterministic rule schedule, `K_max`, `N_max`, `D_max`, a polynomial-time greedy extractor with hash tie-breaking, and a **new versioned artifact** carrying input commitments, schedule, ceilings, extractor generation, the §2.2 hashing-generation tag, and the comparator version. `OP_BYPASS` canonical contraction and fixed-width saturating arithmetic remain separate candidates. This share allocates neither opcode tag `32`–`37` nor any envelope key to saturation.
- **Remaining Step-6 extensions**: Fork–join and Road proofs, FUTURE with their own evidence.
- **`Q1` / `Q3` / `Q6`**: `QUEUED` / `QUEUED` / `QUEUED`; no term include in this section uses an unregistered slug. The only term includes are `boc-policy`, `computational-consistency`, `licensed-work`, and `receipt`, all already registered in `docs/_data/glossary/`.
- **Sealing / promotion**: not claimed. Under {% include term.html id="computational-consistency" %}, the validity claimed here is internal consistency within the declared extension boundary: §9 is additive to §1–§8, its bitwise width set and hash byte bound are closed and finite, its admission widening is value-identical on every globally admissible graph, and nothing here amends a canon file, a committed vector, or a glossary entry.

---

## License and Invariant Lineage

This companion is part of the {% include term.html id="licensed-work" %} under the Scaling Source License (SSL 1.0) (`LICENSE.md` at the repository root) and introduces no license-relevant divergence from the program of record. Its invariant lineage is the program of record's committed state at `v0.3.0-draft.36`: this file adds a new §9 whose declared semantics bind the committed §1–§8 and change no byte of them — no core, container, or call opcode is re-tagged, no record field or token is added, no state component is introduced, and no corpus or fixture byte is re-versioned. Its adoption does not promote QnIR to canon: QnIR remains `[PENDING]`, and `Q1`, `Q3`, and `Q6` remain `QUEUED`.
