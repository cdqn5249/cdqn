[⬡ Back to Documentation Portal](index.html)

# The Binary ABI Specification of Qn Primitives

[Next: Base-10 Seeds & Cryptography →](qn-base.md)

## Metadata

| Field | Value |
| :--- | :--- |
| **Version** | `1.0` |
| **Timestamp** | `2026-08-06` |
| **Author** | Christophe Duy Quang Nguyen |
| **Location** | Bao Loc, Vietnam |
| **Repository** | [`cdqn5249/cdqn`](https://github.com/cdqn5249/cdqn) |
| **AI Engine** | Qwen — Qwen Studio, Alibaba Group |
| **License** | [Scaling Source License (SSL v1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |

## ⬡ Document Profile

- **id:** `qn-primitives`
- **version:** `1.0`
- **type:** `abi`
- **layer:** `floor`
- **depends:** `simemp-manifesto` · `qn-def`
- **invariants:** `[I]` · `[S]` · `[M]`
- **agiles:** `[E]` · `[Mod]` · `[P]`
- **trust:** `draft`
- **status:** `active`

## Abstract

This specification defines the bare-metal binary ABI of the Quang Number universe: the universal 64-bit bit-atom `Qn,64`, its register mappings, the 2-bit execution status field, hardware alignment rules, and the substrate-neutral SQS transport ABI. At this layer software abstraction ends and physical silicon begins. It adopts the cautious derive-then-verify epistemology of the SIMEMP Manifesto.

> **Governed by:** `[I]` · `[S]` · `[M]` · `[E]` · `[Mod]` · `[P]`

---

## 1. Physical Atomicity & The Silicon Gatekeeper

> **Governed by:** `[I]` · `[S]`

At the bare-metal hardware register level, software abstraction ends. The 64-bit word (`Qn,64`) is the single, universal physical bit-atom of the Quang Number universe. Every higher-level structure (Qexpr, `Qm` vectors, `Qs` graphs, CDQN packets) is composed strictly from arrays of this primitive.

> **Silicon Gatekeeper Law:** `Lower(Data) ∉ { Qn,64 } ⇒ Qn = ∅` (rejected at the hardware register interface).

---

## 2. Universal 64-Bit Register Format & Hardware Mappings

> **Governed by:** `[M]` · `[P]`

### 2.1 The Single Universal Bit-Atom (Qn,64)

Every device — from 32-bit microcontrollers to 512-bit vector engines — uses the exact same 64-bit bit-field contract:

```text
UNIVERSAL Qn,64 PACKED REGISTER WORD (64 BITS TOTAL)
Bits 0–15  (16b)  Security Anchor [S]   (gatekeeper handle)
   Bits 0–11   Origin Identity Hash (I_src/Epoch) -> resolves to full I via T0/Q(harvest)
   Bits 12–15  4-bit truncated HMAC (Σ pre-check), evaluated O(1) on r0
Bits 16–31 (16b)  Metric Anchor [M]     (gatekeeper handle)
   Bits 16–17  2-bit Status (execution feedback)
   Bits 18–19  2-bit Domain {00 Maths, 01 Semantics, 10 Hybrid, 11 reserved}
   Bits 20–25  6-bit z_macro
   Bits 26–30  5-bit z_micro
   Bit  31     V_mode (0 = ratio [N,D] ; 1 = arena offset)
Bits 32–63 (32b)  Value Payload [V]
   V_mode=0 -> 16:16 signed Numerator/Denominator ratio [N,D]
   V_mode=1 -> 32-bit Qexpr arena offset
```

**`[V]` mode discriminator (fix):** the `V_mode` bit (bit 31) unambiguously selects the payload interpretation, resolving the prior "ratio **or** offset" ambiguity. *Exact bit allocation to be finalized and verified by QnLang.*

### 2.2 Gatekeeper-Handle Semantics ([S] and [M] resolution)

> **Governed by:** `[S]` · `[M]`

The 16-bit atom `[S]` and `[M]` are **gatekeeper handles**, not the full anchors:

- **`[S]` handle:** the 12-bit identity hash resolves to the full Identity `I` via Genesis Tile `T0`/`Q(harvest)`; the 4-bit truncated HMAC is a 1-cycle O(1) fast pre-check. The full signature `Σ` lives in the arena / `T0`.
- **`[M]` handle:** the atom carries `Status + Domain + z + V_mode` for O(1) register-level gating; the full logical `[M] = ⟨Domain, z, D, C, DCC, P⟩` lives in the Qexpr arena reached via the arena-offset `[V]`.

### 2.3 32-Bit Hardware Edge Alignment (r0:r1 Register Pair)

On 32-bit architectures (ARM Cortex-M, ESP32, RISC-V RV32I), `Qn,64` maps cleanly into a 2-register pair:

- **Control register `r0` (bits 0–31):** holds `[S]` (0–15) and `[M]` (16–31). The CPU evaluates the invariant check in **1 clock cycle (O(1))** using `r0`.
- **Data register `r1` (bits 32–63):** holds the 32-bit Value Payload `[V]`.

### 2.4 Multi-Lane SIMD Vector Register Scaling

Wider registers introduce no new formats; they are multi-lane SIMD arrays of `Qn,64`:

- **128-bit SIMD** (ARM Neon / Apple AMX / WASM): 2-lane `Qn,64[0] ‖ Qn,64[1]`.
- **256-bit AVX:** 4-lane array.
- **512-bit vector engine:** 8-lane array.

---

## 3. Execution Status Field & Zero-Day Isolation

> **Governed by:** `[S]`

Bits 16–17 specify the 2-bit Execution Status Field. **Clarification:** `Status` is *execution feedback* written by the hardware after `EVAL`, co-located in `[M]` for O(1) readout; it is **not** an input metric.

| Status | Symbolic Name | Hardware Execution & BoC Action |
| :--- | :--- | :--- |
| `00` | `VALID_EXACT` | Hard Halt (R = 0). Exact evaluation complete; zero residual memory. |
| `01` | `VALID_RESIDUAL` | Soft Halt (R ≠ 0). Active view `V_z` returned; residual `R` persisted in `Q(reuse)` SRAM. |
| `10` | `INVARIANT_FAULT` | `[S]` or `[M]` failure. Null Law (`Qn = ∅`); payload dropped at register interface. |
| `11` | `ANOMALY_QUARANTINE` | Zero-day pattern detected. BoC engine invoked for network-wide immunization in O(1). |

---

## 4. Hardware Alignment & Space-Time Dilation Rules

> **Governed by:** `[E]`

### 4.1 The 64-Byte L1 Cache Line Fit
A physical 64-byte cache line fits **exactly 8** packed `Qn,64` words (8 × 8 = 64 bytes) with 0% padding waste.

### 4.2 Register Calling Convention
- **Operands:** registers `r0`–`r3`.
- **Active View output (`V_z`):** returned in `r0`.
- **Residual output (`R`):** returned in `r1` (persisted in `Q(reuse)`).
- **Status flags:** written to CPU condition-code flags.

### 4.3 Universal Endianness Standard
All binary payloads and structs mandate **Little-Endian** across ARM64, RISC-V, x86_64, and WebAssembly.

---

## 5. Aperiodic Arenas & Substrate-Neutral SQS Transport

> **Governed by:** `[Mod]` · `[P]`

### 5.1 Aperiodic Einstein Tile Memory Allocation
Physical RAM is partitioned using aperiodic monotiles (Einstein tiles / quasicrystals). Arenas are contiguous `Qn,64` arrays; depth indices are encoded in zoom `z`, eliminating periodic hash collisions and `malloc` overhead.

### 5.2 Substrate-Neutral SQS Transport ABI
To an SQS, local registers, memory buses, and network sockets differ only by latency `Δτ` and addressing `I_src → I_dst`. The `Qn,64` layout is the **universal SQS payload**: a packed word executes in a register, copies to an SRAM arena, or crosses a socket with zero re-serialization loss.

---

## Cross-References

- Root axiom & boundary laws → [`simemp-manifesto.md`](simemp-manifesto.md)
- Documentation portal & navigation map → [`index.md`](index.md)
- Qn Trinity & Metric Anchor → [`qn-def.md`](qn-def.md)
- Digit seeds, Q(harvest), Dual-Ring PQC → [`qn-base.md`](qn-base.md)
- SQS topologies & zero-trust → [`sqs-def.md`](sqs-def.md)
- Canonical term definitions → [`glossary.md`](glossary.md)

