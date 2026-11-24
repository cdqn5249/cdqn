---
layout: default
title: QnLang — Rung-0 Candidate Specification (High-Level Authoring Language)
description: QnLang v0.1 Rung-0 — CAE-AST semantic model, operator hygiene, formal EBNF, six-domain type system, zero-concession compiler, and R11' lowering overview
version: 0.1.0-draft.3
updated: 2026-11-24
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/qnlang.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /qnlang.html
status: Candidate Specification — QnLang Rung-0 (Not Promoted; QnLang remains [PENDING])
terms_used:
  - boc-policy
  - cae-dag
  - causal-arrow
  - complexity-degree
  - computational-consistency
  - compute-unit-u
  - dcc-profile
  - dependencies-determinism
  - derivative-work
  - dimension-d
  - existential-invariant
  - hca-collapse
  - hash-consing
  - intent-contract
  - layer-0
  - layer-1
  - licensed-work
  - metric-envelope
  - metric-exhaustion
  - no-implicit-rule
  - open-core-invariant
  - paternity-reference
  - q-anchor
  - q-bypass
  - q-datastruc
  - q-patterns
  - q-reuse
  - q-workflows
  - q0
  - q1
  - qbio
  - qchem
  - qexpr
  - qlog
  - qm
  - qnir
  - qnlang
  - qphy
  - qs
  - qs-history-library
  - receipt
  - remainder-r
  - scaling-threshold
  - simemp
  - simemp-gateway
  - ssl
  - structural-indirection
  - terminal-exactness
  - universal-envelope
  - zoom-z
---

# QnLang — Rung-0 Candidate Specification (High-Level Authoring Language)

| Field | Specification |
|---|---|
| **Document Title** | QnLang — Rung-0 Candidate Specification (High-Level Authoring Language) |
| **Version** | 0.1.0-draft.3 |
| **Last Updated** | 2026-11-24 |
| **Author** | Christophe Duy Quang Nguyen |
| **License** | [Scaling Source License (SSL 1.0)](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| **Status** | Candidate Specification — QnLang Rung-0 (Not Promoted; QnLang remains [PENDING]) |

This document materializes the Rung-0 candidate surface language of the Qn universe. It is a **new versioned artifact** under **Axiom 13** and a candidate construct under **Axiom 14**; no prior file is mutated in place. It records the separation demanded by F19: {% include term.html id="qnlang" text="QnLang" %} is a surface language with its own syntax, typing, and compilation relation, not an extension of `src/qnir/`, not a Rust or Python dialect, and not a shadow implementation of the intermediate representation. The program remains `[PENDING]` in `docs/index.md`; this candidate does not promote it.

---

## 1. Epistemic Stance, `F19` Separation & Bootstrap Boundary

### 1.1 Rung-0 stance

Rung-0 is a **minimal, total, human-authorable specification** sufficient to express exact computations with explicit profiles, budgets, and causal state. Its validity claim is internal consistency within declared finite boundaries under {% include term.html id="computational-consistency" %} — not a theorem, not promoted canon, and not an execution proof.

The language sits **outside Layer 3**. Layer 3 is the {% include term.html id="qexpr" text="Qexpr" %} / {% include term.html id="cae-dag" text="CAE-DAG" %} symbolic composition layer; QnLang authors surface terms which the compiler collapses into that layer and then lowers to {% include term.html id="qnir" text="QnIR" %}. It is a {% include term.html id="complexity-degree" text="Complexity Degree 2" %} authoring membrane, not a fourth execution layer. The {% include term.html id="boc-policy" text="BOC policy" %} of `docs/simemp.md` §7 governs later selection; none is made here.

### 1.2 F19 separation from `src/qnir/`

F19 bindingly requires: "QnIR is an execution and verification substrate; QnLang is a surface language with its own syntax, typing, and compilation relation." Therefore:

- No QnIR opcode, register schedule, or `qir-enc-4` byte rule is copied into surface syntax.
- `src/qnir/`, `src/qnir-py/`, and all QnIR artifacts are untouched by Rung-0.
- The only shared commitment is the lowering target `qir-enc-4` under the `R11'` simulation obligation.
- A Rust or Python oracle is not a QnLang generation.

### 1.3 Bootstrap boundary and deferred work

| Deferred item | Owner | Rung-0 boundary |
|---|---|---|
| `OP_SATURATE`, `Q(rsi)`, `Qn-SIM`, fixed-width bitwise ops | Step 6.3 | Named as the optimization surface that may later choose linear vs COW and `z_min`; not specified here. |
| Fork–join | Step 6.4 | No parallel construct; receipt order remains sequential. |
| Road proofs | Step 6.5 | No proof-system realization selected. |
| `Q1`–`Q4`, `Q6` | Standing Governance Queue | All `QUEUED`; zero new glossary slugs and zero new term includes here. |
| Final promotion | Loop-3 / BOC | QnLang stays `[PENDING]`. |

The bootstrap boundary is also the hardware boundary: {% include term.html id="layer-1" text="Layer 1" %} genesis and {% include term.html id="layer-0" text="Layer 0" %} physical execution are described by their owning documents; QnLang only names declared profiles and budgets.

---

## 2. Core Semantic Model — `CAE-AST`, `QO-24` NameMap, Affine Memory Purity

### 2.1 CAE-AST

The QnLang abstract syntax tree is a **hash-consed, content-addressed expression DAG** (`CAE-AST`) plus a **De-Bruijn control DAG** for bounded structured control. Nodes are immutable and identified by complete canonical bytes, not heap address; {% include term.html id="hash-consing" %} gives structural sharing and {% include term.html id="dependencies-determinism" %} gives zero conditional entropy at the commitment boundary. Every node cites its {% include term.html id="dcc-profile" text="DCC profile" %}, bounds, and causal parent edge under the {% include term.html id="causal-arrow" text="causal arrow" %} discipline; no implicit default is admitted under the {% include term.html id="no-implicit-rule" text="No Implicit Rule" %}.

Control uses De-Bruijn indices: a bound name is an index into the enclosing binder, never a mutable cell. No pointer arithmetic, no `&'a` lifetime, no implicit heap, and no garbage-collection pause exists. Physical realization belongs to the {% include term.html id="hca-collapse" text="HCA-Collapse Engine" %} and {% include term.html id="simemp-gateway" text="SIMEMP Gateway" %}.

### 2.2 NameMap sidecar (`QO-24`)

Value identity and derivation identity are separated. `NameMap` is an **optional deterministic sidecar** mapping a canonical value digest to human-facing names and provenance annotations; it is not consulted by equality, typing, or evaluation. Two values are equal only by full canonical encoding under minimal-zoom normalization. The sidecar may be emitted as a receipt annotation but never changes a commitment.

### 2.3 Affine memory purity and COW fallback

**Immutable scalars** (`Qm`, `Qleaf`, `rat`, `TermH`) are copyable: they carry no mutable state and can be referenced repeatedly. **Governed containers** (`Q(array)`, `Q(table)`) obey linear ownership across `Q(store)`, `Q(batch)`, and `Q(drop)`: every container value is consumed exactly once by its terminal transition. `Q(fold)` and `Q(load)` take an immutable borrow and do not consume the container. SSA names are **never rebound**; a second mint of the same name is `E_NAMESPACE_COLLISION`, a standing well-formedness invariant enforced ahead of the 14 semantic gates.

Rung-0 default memory discipline is affine: a binding is consumed exactly once unless declared persistent. When structural sharing across versions is needed, the source names an admitted persistent container:

- `Q(array)` → `RRBTree-16` under `qir-enc-3` / `qir-prof-ext-containers-1`;
- `Q(table)` → `MerkleBTree-16` under the same profile;
- the general family is the {% include term.html id="q-datastruc" text="Q(dataStruc)" %} contract class, whose no-implicit-heap rule is inherited verbatim.

Choosing linear vs COW, and deciding when `z_min` normalization pays for itself, are Step-6.3 `Q(rsi)` obligations; neither is implicit.

### 2.4 Domain substrate

The six orthogonal domains are vertical projections across Layers 1–3: {% include term.html id="qm" text="Qm" %}, {% include term.html id="qs" text="Qs" %}, {% include term.html id="qlog" text="Qlog" %}, {% include term.html id="qphy" text="Qphy" %}, {% include term.html id="qchem" text="Qchem" %}, and {% include term.html id="qbio" text="Qbio" %}. All share the CAE-AST carrier, the work unit {% include term.html id="compute-unit-u" text="U" %}, and the receipt discipline of §6.

---

## 3. Operator Hygiene & `Q(0)..Q(9)` Structural Anchors

### 3.1 Zero overload

Rung-0 fixes one meaning per token; a collision is a compile-time diagnostic:

| Token | Rung-0 meaning | Forbidden reading |
|---|---|---|
| `expr -> name` | causal SSA mint: bind an exact value to a fresh name | assignment / mutation / rebinding |
| `:` | strict type and declaration separator (headers, top decls, annotations) | runtime inference |
| `=` | **exact value equality only** (`CMP EQ` in `binary`) | assignment, hash equality, declaration |
| `!=` | exact value inequality | pointer inequality |
| `~=` | hash equality over canonical bytes | value equality without pre-image |
| `cap(N)`, `trip(K)`, `budget(W U)` | declared ceilings/envelopes | automatic growth or elapsed time |
| `[lo..hi]`, `[lo..<hi]` | declared ranges | unspecified endpoint |
| `div_euc`, `mod_euc` | exact Euclidean quotient/remainder | floating division or `rat` |
| `rat` | exact rational node construction | decimal approximation |
| `-Q(k)` | negative literal of a declared digit | unary allocator or flag |

`=` occurs **only** in `binary`. No declaration uses `=`. The canonical zero is {% include term.html id="q0" text="Q(0)" %}; the canonical multiplicative identity is {% include term.html id="q1" text="Q(1)" %}. No naked numeral is admitted as a literal.

### 3.2 `Q(0)..Q(9)` anchor table

| Anchor | Rung-0 structural role |
|---|---|
| `Q(0)` | zero / empty / absolute; canonical {% include term.html id="q0" text="Q(0)" %} |
| `Q(1)` | unit / multiplicative identity / true |
| `Q(2:cons)` / `Q(2:split)` / `Q(2:chirality)` | pair construction, affine pair elimination, orientation bit |
| `Q(3:cmp)` | exact trivalent comparison (`-1/0/1`) |
| `Q(4:zoom_cmp)` | comparison under minimal-zoom normal form |
| `Q(5:leaf)` | canonical content leaf (`0x00` domain) |
| `Q(6:sym)` | declared symmetry / `Q(bypass)` identity class |
| `Q(7)` | bounded fold anchor |
| `Q(8)` | frame/permutation anchor |
| `Q(9)` | agent/delegation anchor |

The {% include term.html id="q-bypass" text="Q(bypass)" %} suite of `Q(6:sym)` is the zero-cost identity surface; {% include term.html id="q-reuse" text="Q(reuse)" %} is the declared lookup surface; direct exact compute is the fallback under the two-axis breakeven of `docs/qnPrimitive.md` §1.15/§5.9.

---

## 4. Formal EBNF Grammar — Rung-0 Complete

The grammar is unambiguous and profile-bound. `:=` is the **meta-production** operator and never appears in source programs. `⟨...⟩` marks prose non-terminals; quoted tokens are literals; `[...]` is optional; `{...}` is repetition with a declared ceiling; `|` is alternation. Every bound is a source token, never an inferred default. The only `/` admitted in source is the dimensional ratio inside `unit(...)`; it is never value division. The only `..` admitted is inside `qtype` bounds and `range`.

```ebnf
program          := { header_decl } { top_decl | stmt } ;

header_decl      := "Q(dcc)" ident ";"
                  | "Q(kappa)" ( "dyadic" | "dec" | "fact" ) ";"
                  | "Q(domain)" ( ident | "(" ident { "," ident } ")" ) ";"
                  | "Q(mode)" ( "L0" | "L1" | "L2" | "L3" ) ";" ;

top_decl         := "Q(type)" ident ":" qtype ";"
                  | "Q(anchor)" ident ":" qprim ":" qtype ";"
                  | "Q(metric)" ident ":" "eta(" sint "," sint "," sint ")" ";"
                  | "Q(blanket)" ident "{" "sense" ":" param_list ";"
                                              "state" ":" param_list ";"
                                              "act" ":" param_list ";" "}" [ ";" ]
                  | "Q(variant)" ident "{" variant_arm { "," variant_arm } "}" [ ";" ]
                  | "Q(pattern)" ident "(" [ param_list ] ")" "->" qtype contract_tag block [ ";" ]
                  | "Q(workflow)" ident "(" [ param_list ] ")" "->" qtype contract_tag block [ ";" ]
                  | "Q(conjecture)" ident "(" [ param_list ] ")" contract_tag block [ ";" ]
                  | "Q(falsify)" ident "(" [ param_list ] ")" contract_tag block [ ";" ]
                  | "Q(experiment)" ident "for" ident contract_tag "{"
                        { "case" ident ":" expr "->" ident ";" } "}" [ ";" ] ;

contract_tag     := "![" contract_item { "," contract_item } "]" ;
contract_item    := "Qs(" ident "," ident ")" | "Qphy(" ident ")" | "Qchem"
                  | "Qbio(" ident ")" | "Qlog"
                  | "budget(" uint "U)"
                  | "leash(" ident ":" uint ident ")"
                  | "metabolic_cap(" ident ")"
                  | "may_emit(" ident { "," ident } ")" ;

param_list       := ident ":" qtype { "," ident ":" qtype } ;
variant_arm      := ident "(" [ param_list ] ")" ;
ident            := letter { letter | digit | "_" } ;
uint             := digit { digit } ;
sint             := [ "-" ] uint ;
digit            := "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
bound            := sint ;
letter           := "a" | ... | "z" | "A" | ... | "Z" ;
unit_atom        := "J" | "s" | "kg" | "m" | "K" | "mol" | ident ;
unit_expr        := unit_atom { "/" unit_atom } ;   (* dimensional ratio only *)

qprim            := [ "-" ] "Q(" ( uint | ident ) [ ":" ident ] ")" [ "<unit(" unit_expr ")>" ] ;

qtype            := ident | "u8" | "u16" | "u32" | "u64" | "bool"
                  | "Qm[" bound ".." bound "]"
                  | "Qd[" bound ".." bound "," bound "]"
                  | "Qleaf<" "kappa(" ident ")" "," "z[" bound ".." bound "]"
                       "," "q[" bound ".." bound "]" "," "axis(" ident ")" ">"
                  | "rat[" bound "," bound "]"
                  | "Q(tuple)<" qtype { "," qtype } ">"
                  | "Q(array)<" qtype "," "cap(" uint ")" ">"
                  | "Q(table)<" qtype "," qtype "," "cap(" uint ")" ">"
                  | "Q(blade)<" "axes(" uint ")" "," qtype ">"
                  | "Qphy<" "unit(" unit_expr ")" "," qtype ">"
                  | "sim<" ident "," ident ">"
                  | qtype "![" contract_item { "," contract_item } "]" ;

container_ctor   := "Q(arr_new)<" qtype "," "cap(" uint ")" ">"
                  | "Q(tbl_new)<" qtype "," qtype "," "cap(" uint ")" ">" ;

target_pat       := ident | "(" ident { "," ident } ")" ;
expr_list        := expr { "," expr } ;

primary          := qprim | ident | "(" expr { "," expr } ")"
                  | container_ctor | builtin_call | structured ;

builtin_call     := "Q(metric_dot)" "(" expr "," expr ")"
                  | "Q(reuse)" "(" expr ")"
                  | "Q(store)" "(" expr "," expr ")"
                  | "Q(drop)" "(" expr ")"
                  | "Q(halt_with)" "(" expr ")"
                  | "Q(cut)" "(" expr ")"
                  | "Q(2:cons)" "(" expr "," expr ")"
                  | "Q(2:chirality)" "(" expr ")"
                  | "Q(3:cmp)" "(" expr "," expr ")"
                  | "Q(4:zoom_cmp)" "(" expr "," expr ")"
                  | "emit" ident ;

structured       := "Q(2:split)" "(" expr ")" "{" "T" "->" block [ ";" ] "F" "->" block [ ";" ] "}"
                  | "Q(match)" "(" expr ")" "{" { ident "->" expr ";" } "}"
                  | "Q(fold)" expr "over" range "init(" expr ")"
                       "as" "(" ident { "," ident } ")" block
                  | "Q(loop)" "trip(" uint ")" block
                  | "Q(load)" "(" expr "," expr ")" "@[" "trip(" uint ")" "]"
                       "->" target_pat [ "or_else" block ] ;

postfix          := primary { "." ( uint | ident | ident "?" ) | "(" [ expr_list ] ")" } ;
unary            := ( "-" | "~" ) postfix ;
binary           := unary ( "+" | "-" | "*" | "=" | "!=" | "~="
                         | "<" | "<=" | ">" | ">=" | "&&" | "||" ) unary ;
range            := "[" expr ( ".." | "..<" ) expr "]" ;
expr             := binary | range ;

bind             := expr "->" target_pat [ ":" qtype ] ";" ;
stmt             := bind | expr ";" ;
block            := "{" { stmt } [ "->" expr ] "}" ;
```

**Revision record (v0.1.0-draft.2, Axiom 13 — additive §4 reconciliation).** Three proven gaps between the literal §4 EBNF and the §6.4 worked programs are closed here without changing any §6.4 program: (1) `program` now admits top-level `stmt` alongside `top_decl`, matching the bare bindings in every §6.4 example; (2) `top_decl` now includes the `Q(falsify)` production already declared in §5.2, named in the §4 prose, and used by the `Qs + Qlog` example (`Q(conjecture)` and `Q(experiment)` were already present); (3) every braced declaration (`Q(blanket)`, `Q(variant)`, `Q(pattern)`, `Q(workflow)`, `Q(conjecture)`, `Q(falsify)`, `Q(experiment)`) ends in `}` with an optional `[ ";" ]`, so the `Qbio` blanket and the `Qchem`/`Qs + Qlog` variants follow one uniform rule. No semantic, type, or affine rule is relaxed.

No production introduces an implicit value, a pointer, a floating literal, an unbounded repetition, a Unicode multiplication dot, `===`, or a bare `/`. `=` is admitted **only** in `binary`; declarations use `:`. `Q(conjecture)` / `Q(falsify)` / `Q(experiment)` are proof-shaped declarations; they emit receipts under §6 and never change a value without an explicit follow-on binding.

---

## 5. Type System, Built-in Signatures & Six Orthogonal Domains

### 5.1 Type system and `Q(variant)`

A QnLang type is a declared tuple `(domain, tag, bound, unit?, liveness)`; inference is absent. `Q(variant)` declares a closed sum, `Q(match)` eliminates it with exhaustive branches, and `Q(type)` names a structural type. A variant value carries its constructor tag and payload canonical bytes. Token-state transitions are explicit in source. The five-level `C|E|H|N|U` lattice is carried as a declared type attribute.

### 5.2 Explicit built-in signatures

All built-ins used by §4 or §6 have exactly one declared signature:

| Built-in | Signature | Notes |
|---|---|---|
| `Q(anchor)` | `Q(anchor) a : qprim : qtype;` | Declares a local genesis anchor; no hardcoded genesis (`E_LOCAL_GENESIS_HARDCODE`). |
| `Q(metric)` | `Q(metric) m : eta(s : sint, p : sint, r : sint);` | Declares a metric tensor signature. |
| `Q(metric_dot)` | `(a : Qd[..], b : Qd[..]) -> Qm[..]` | Exact contraction; never `·`. |
| `Q(blanket)` | `Q(blanket) b { sense:P; state:P; act:P };` | Declares a three-phase blanket type. |
| `Q(mode)` | `Q(mode) L0..L3;` | Header declaring the permitted inference mode. |
| `Q(0)<unit(...)>` | `qprim` with unit suffix | Canonical typed zero; `/` inside unit is dimensional ratio. |
| `scale_leaf` | `(x : Qleaf<..>) -> Qleaf<..>` | Rescales leaf at declared zoom. |
| `align` | `(x : Qleaf<..>) -> Qleaf<..>` | Aligns zoom/axis tuple. |
| `zoom` | `(x : Qm[..], z : z_min) -> Qm[..]` | Exact zoom projection. |
| `z_min` | `ident` constant | Minimal-zoom selector. |
| `.exact?` | postfix `(Qm[..]) -> bool` | True iff remainder is `Q(0)`. |
| `modal_as` | `(v : T, m : ident) -> T!m` | Declares a modal view; no domain leak. |
| `Q(arr_new)` | `<T, cap(N)> -> Q(array)<T, cap(N)>` | Linear container constructor. |
| `Q(tbl_new)` | `<K, V, cap(N)> -> Q(table)<K, V, cap(N)>` | Linear table constructor. |
| `Q(halt_with)` | `(r : receipt) -> !` | Terminal exit; requires `may_emit(r)`. |
| `Q(cut)` | `(v : T) -> T` | Declared proof/search cut; preserves value. |
| `Q(falsify)` | `Q(falsify) f(params) contract_tag block` | Proof-shaped declaration; requires `may_emit`. |
| `Q(reuse)` | `(v : T) -> T` | Two-axis memoized reuse at declared tier. |
| `may_emit` | `(v : T, tag : ident) -> bool` | Emission authorization under `Φ_status`. |
| `leash` | `(e : Qphy<unit(U),T>, Emax : uint) -> Qphy<unit(U),T>` | Physical dissipation clamp. |
| `metabolic_cap` | `(b : Blanket, cap : uint) -> Blanket` | Qbio metabolic clamp. |

### 5.3 `Qs`

`Qs` authors intent and epistemic delegation: `Q(blade) <= 16`, `TermH`, the `C|E|H|N|U` lattice, `N` containment, `sim<C, H>`, `Q(agent)`, and the `I/P/S` grounding ladder of {% include term.html id="qs-history-library" text="Qs History Library" %}. Emission is mediated by `may_emit(...)` and projected through `Φ_status`; the projection is a reporting boundary. The intent carrier is the {% include term.html id="intent-contract" text="Intent Contract" %} of `docs/qs.md`.

### 5.4 `Qlog`

`Qlog` contributes `Q(pattern)` (`R_road` segment) and `Q(workflow)` (chained receipt vector), with `Q(mode) L0..L3`. `Q(abduce)`, `Q(anchor)`, `Q(prefetch)` are Rung 1 except where `Q(anchor)` is used as the local genesis built-in above. The two-axis `Q(reuse)` macro-hop is the only admitted Rung-0 compression morphism. The {% include term.html id="q-patterns" text="Q(pattern)" %} / {% include term.html id="q-workflows" text="Q(workflow)" %} glossary definitions bind these terms.

### 5.5 `Qm`

`Qm` provides exact arithmetic: `Q(kappa)`, `Qleaf`, `zoom`, `.exact?`, `z_min`, `align`, `scale_leaf`, `eta(p,q,s)`, `Q(root)`, `Q(cf)`. {% include term.html id="zoom-z" text="zoom z" %}, {% include term.html id="remainder-r" text="remainder r" %}, and {% include term.html id="dimension-d" text="dimension d" %} are canonical CAE node attributes; {% include term.html id="terminal-exactness" text="Terminal Exactness" %} discharges a remainder of `Q(0)`. `Q(root)` and `Q(cf)` are symbolic generators of `docs/qm.md`.

### 5.6 `Qphy`

`Qphy` declares `Qphy<unit(...), T>` and `Q(0)<unit(...)>` typed quantities, five declared physical anchors, and `leash(...)`. Units are part of the type; `Q(0)<J>` and `Q(0)<s>` are distinct. No wall-clock or unmetered energy source is admitted. The physical anchors are those registered in `docs/qphy.md`; QnLang names them, it does not re-derive them.

### 5.7 `Qchem`

`Qchem` requires `∩ C_Qs = empty`, `S·v = Q(0)` expressed in source as `Q(metric_dot)(S, v) = Q(0)` (zero Unicode dot), explicit `Q(2:chirality)`, and `Q(bypass)` catalysis. A `Q(domain) (Qchem, Qm)` header enforces abiotic isolation mechanically: no `Qs` identifier may appear in a `Qchem` example. Stoichiometry is exact under `Qm`.

### 5.8 `Qbio`

`Qbio` contributes `Q(blanket) { sense; state; act }` and `metabolic_cap`. The blanket partitions observation, internal state, and action; all three are explicit morphisms. `metabolic_cap` is a declared budget ceiling; it forbids implicit growth. {% include term.html id="qbio" text="Qbio" %} is an orthogonal projection, not a Layer 4.

### 5.9 Cross-domain contracts

`qs_history.md` contributes the `I/P/S` ladder, `Q̂(0)` schema portability, and a wall-clock ban. `qstructures.md` contributes the five-row contract schema; Rung-0 checks **Locality**, **Determinism**, and **Work** against the `QO-17` cache tiers. Footprint is checked through {% include term.html id="metric-envelope" text="Metric Envelope" %}, exhaustion through {% include term.html id="metric-exhaustion" text="metric exhaustion" %}, and all crossings through the {% include term.html id="simemp" text="SIMEMP" %} constraints. {% include term.html id="structural-indirection" text="Structural Indirection" %} lets these contracts evolve independently of concrete hardware.

---

## 6. Zero-Concession Compiler (`qnlangc`) & Dual-Channel Diagnostics (`R12'`)

### 6.1 Compiler principle

`qnlangc` admits no concession: every bound, unit, domain, and lifetime is declared. It lowers CAE-AST to `Qexpr` and then to `qir-enc-4`; it never invents a default and never silently drops a receipt. Diagnostic totality is the `R12'` obligation: every rejected program receives both a machine tuple and a human summary. Immutable scalars are copyable; governed containers are linear across `Q(store)` / `Q(batch)` / `Q(drop)`; SSA names are never rebound (`E_NAMESPACE_COLLISION`, enforced ahead of the 14 gates).

### 6.2 Fourteen semantic gates

| # | Gate | Trigger |
|---:|---|---|
| 1 | `E_NO_NAKED_NUM` | A numeral appears outside `Q(k)` / `-Q(k)`. |
| 2 | `E_BOUND_OVERFLOW` | A value or anchor exceeds its declared `qtype` bound. |
| 3 | `E_POSSIBLE_DIV_ZERO` | A quotient is admitted without a nonzero proof. |
| 4 | `E_UNPROMOTED_ROOT` | A negative even root reaches an exit without `PROMOTE`. |
| 5 | `E_UNPROVED_INDEX` / `E_UNHANDLED_MISS` | An indexed/loaded access lacks a proof or an `or_else` branch. |
| 6 | `E_UNBOUNDED_CONTROL` | A loop or fold lacks `trip(K)` / `cap(N)`. |
| 7 | `E_UNCONSERVED_REMAINDER` / `E_UNALIGNED_ZOOM` | Zoom drops or misaligns a remainder. |
| 8 | `E_MODAL_OR_DOMAIN_LEAK` | A value crosses a declared domain or modal view. |
| 9 | `E_UNIT_MISMATCH` | A `Qphy<unit(...)>` operation mixes units. |
| 10 | `E_GROUNDING_VIOLATION` | An `I/P/S` or domain grounding declaration is absent. |
| 11 | `E_LOCAL_GENESIS_HARDCODE` | A genesis constant is used without `Q(anchor)`. |
| 12 | `E_IMPLICIT_WALLCLOCK` | Host time or unmetered energy is referenced. |
| 13 | `E_UNDECLARED_TERMINAL_STATUS` | A terminal exit lacks `may_emit(...)` authorization. |
| 14 | `E_SMAX_OR_BUDGET` | Structural DP states exceed `S_max = 4096` or declared budget is insufficient. |

Plus the standing invariant `E_NAMESPACE_COLLISION`: SSA names are never rebound. A program passing all gates is *admissible*, not proven.

### 6.3 Dual-channel diagnostics

**Channel A — agent tuple:** `(code, span, term_id, expected, actual, remedy, receipt_ref)`.

**Channel B — `qir-verbalizer-1` NL summary:**

```text
qir-verbalizer-1: <code> at <file>:<line>:<col> — <short>; expected <expected>, got <actual>; remedy <remedy>
```

Both channels are emitted for every diagnostic. The {% include term.html id="receipt" text="receipt" %} is the shared evidence boundary; the NL summary is a rendering, not a second diagnosis.

### 6.4 Worked reference examples

Each example below is a complete `program` under §4 and satisfies all 14 gates plus the standing namespace invariant.

**Qm** — exact arithmetic, declared bounds, ranges, remainders:

```text
Q(dcc) dcc0;
Q(kappa) dyadic;
Q(domain) Qm;
Q(mode) L0;
Q(type) T : Qm[-255..255];
Q(anchor) zero : Q(0) : T;
Q(metric) met : eta(2,3,5);
Q(1) -> lo : T;
Q(4) -> hi : T;
[lo..hi] -> rng : Qm[-255..255];
-Q(3) -> a : T;
scale_leaf(a) -> sl : T;
align(sl) -> al : T;
zoom(al, z_min) -> v : T;
v.exact? -> exact : bool;
emit receipt;
```

**Qphy** — typed physical quantity with leash:

```text
Q(dcc) dcc1;
Q(kappa) dyadic;
Q(domain) Qphy;
Q(mode) L0;
Q(type) Jt : Qphy<unit(J), Qm[-255..255]>;
Q(anchor) jzero : Q(0)<unit(J)> : Jt;
Q(metric) ph : eta(1,1,1);
Q(0)<unit(J)> -> energy : Jt;
leash(energy, Q(4)) -> leashed : Jt;
may_emit(leashed, receipt) -> authorized : bool;
emit receipt;
```

**Qchem** — abiotic stoichiometry, linear array, metric_dot flux fold:

```text
Q(dcc) dcc2;
Q(kappa) fact;
Q(domain) (Qchem, Qm);
Q(mode) L1;
Q(type) Stoich : Qm[-255..255];
Q(type) Pair : Q(tuple)<Stoich, Stoich>;
Q(anchor) zero : Q(0) : Stoich;
Q(metric) ch : eta(1,1,1);
Q(variant) Species { A(s : Stoich), B(s : Stoich) }
Q(2:cons)(Q(0), Q(1)) -> pa : Pair;
Q(2:cons)(Q(1), Q(0)) -> pb : Pair;
Q(arr_new)<Pair, cap(2)> -> arr : Q(array)<Pair, cap(2)> ![budget(20 U)];
Q(store)(arr, pa) -> arr1 : Q(array)<Pair, cap(2)>;
Q(store)(arr1, pb) -> arr2 : Q(array)<Pair, cap(2)>;
Q(fold) arr2 over [Q(0)..<Q(2)] init(Q(0)) as (acc, i) { -> acc + Q(1) } -> flux : Stoich;
Q(drop)(arr2);
Q(metric_dot)(pa, pb) = zero -> balanced : bool;
emit receipt;
```

**Qbio** — blanket phases and metabolic clamp:

```text
Q(dcc) dcc3;
Q(kappa) dyadic;
Q(domain) Qbio;
Q(mode) L2;
Q(type) BioT : Qm[0..255];
Q(blanket) Blanket {
  sense : x : BioT;
  state : s : BioT;
  act : a : BioT;
}
Q(anchor) bzero : Q(0) : BioT;
Q(metric) bio : eta(1,1,1);
Blanket -> inst : BioT;
metabolic_cap(inst, Q(64)) -> clamped : BioT;
may_emit(clamped, receipt) -> ok : bool;
emit receipt;
```

**Qs + Qlog** — pattern/workflow, table, load miss, modal view, authorized exits:

```text
Q(dcc) dcc4;
Q(kappa) dyadic;
Q(domain) (Qs, Qlog);
Q(mode) L3;
Q(type) KeyT : u64;
Q(type) StepOut : Q(tuple)<bool, u64>;
Q(anchor) kzero : Q(0) : KeyT;
Q(metric) lg : eta(1,1,1);
Q(variant) Step { Hit(o : StepOut), Miss(o : StepOut) }
Q(falsify) bad(k : KeyT) ![Qlog, may_emit(receipt)] {
  Q(halt_with)(receipt);
}
Q(pattern) step(k : KeyT) -> StepOut ![Qlog, budget(64 U)] {
  Q(tbl_new)<KeyT, bool, cap(2)> -> tbl : Q(table)<KeyT, bool, cap(2)> ![budget(20 U)];
  Q(store)(tbl, (k, Q(1))) -> tbl1 : Q(table)<KeyT, bool, cap(2)>;
  Q(load)(tbl1, k) @[trip(1)] -> hit or_else {
    Q(cut)(Q(0));
    -> (Q(0), Q(0))
  } -> out : StepOut;
  Q(drop)(tbl1);
  -> hit
}
Q(workflow) route(k : KeyT) -> StepOut ![Qlog, budget(80 U), may_emit(receipt)] {
  step(k) -> res : StepOut;
  Q(reuse)(res) -> memo : StepOut;
  -> memo
}
modal_as(route, Qlog) -> view : Qlog;
emit receipt;
```

The {% include term.html id="terminal-exactness" text="Terminal Exactness" %} rule is exercised when a remainder is `Q(0)`; the {% include term.html id="universal-envelope" text="Universal Envelope" %} carries receipt bytes; {% include term.html id="derivative-work" text="Derivative Works" %} and {% include term.html id="scaling-threshold" text="scaling threshold" %} governance remain under the SSL, not source syntax.

---

## 7. `R11'` Lowering Overview & Self-Bootstrap Ladder

### 7.1 Lowering overview

`R11'` requires a simulation relation from source to QnIR. Rung-0 expresses it as three deterministic steps, not a proof of a theorem:

1. **CAE-AST normalization** — hash-cons, apply `Q(bypass)` identities bottom-up, sort commutative children, freeze `z_min`.
2. **Domain erasure** — `Qm`/`Qs`/`Qlog`/`Qphy`/`Qchem`/`Qbio` annotations become declared bounds, units, and receipt tags.
3. **Encoding lowering** — normalized DAG flattens to `qir-enc-4` with explicit `trip(K)`, `cap(N)`, `budget(W U)`; byte grammar is owned by QnIR companions.

**Closed Rung-0 `R11'` Lowering Table [v0.1.0-draft.3].** Each row is the one declared mapping for its construct; nothing below is resolved by convention or implementation accident. Stage 0B emits a single entry block (block 0): bounded structured control is flattened into that block with explicit declared ceilings, and the fallback block id of a load is carried as an operand witness. Multi-block CFG synthesis remains a Step-6.3 obligation and is not improvised here. Charges are `qir-prof-ext-calls-1` (§8.4): `gateway = 20 U`; core `4 U`; `ARR_NEW`/`TBL_NEW`/`ARR_LEN`/`*_ROOT` `4 U`; `ARR_GET`/`TBL_GET`/`TBL_AT` `8 U`; `ARR_SET`/`TBL_SET` `10 U`; `*_BATCH` `16 U`; `OP_CALL = 4 + 2·argc`; `OP_RET = 4 + 2·retc`. `B_hat` of a declaration is `gateway + Σ` site charges; at a call site the callee's **interior** bound is folded as one site-stable charge (§8.4), never its gateway twice. A `![budget(W U)]` on a top declaration is checked as `B_hat <= W`; one embedded in a `qtype` constrains only that container site. No budget is silently raised.

| Source construct | Declared `qir-enc-4` lowering | Charge |
|---|---|---|
| `Q(k)`, `-Q(k)`, `Q(k)<unit(u)>` | `CONST` (tag 1) with `imm` = `INT_POS`/`INT_NEG` magnitude encoding and `bnd` from the declared qtype; the unit is erased into the bound, not an `EXT` payload | 4 U |
| `qprim` into an anchor type | same `CONST`, with the anchor's declared `bnd` | 4 U |
| `[lo..hi]`, `[lo..<hi]` | `TUPLE` (tag 3) of two `CONST` endpoint values with the declared qtype bound; `..<` records the exclusive endpoint as declared (no implicit decrement) | 4 + 2·4 U |
| `.exact?` on `Qm[..]` | `CMP` (tag 10) of the value against canonical zero `Q(0)` with `rng = [0..0]`, result `bnd bool <= 1` | 4 + 4 U |
| `scale_leaf`, `align` | §7.1 normalization identities: fold into the operand's frozen mantissa and `z_min`; no opcode, same SSA ref | 0 U |
| `zoom(al, z_min)` | `CONST` with `imm = ZOOM(N(q), z_qnir)`, `z_qnir = -z_min` (`INT_NEG` for `z_min > 0`, `INT_POS 0` for `z_min = 0`), odd mantissa `N(q)`; canonical zero has `q = 0, z_min = 0` (qm.md §4.3) | 4 U |
| `leash`, `metabolic_cap`, `may_emit`, `modal_as`, `cut`, `drop`, `reuse` | domain-erasure identities (§7.1 step 2): same SSA ref, authorization/affine burn recorded as a receipt witness; no opcode | 0 U |
| `Q(2:cons)(a,b)` | `TUPLE` (tag 3), `cnt = 2`, two field `bnd`s, refs `a, b` | 4 U |
| `Q(metric_dot)(a,b)` | `MUL` (tag 7) with the declared element bound, then the enclosing `=` becomes `CMP` (tag 10) against `Q(0)` with `rng = [-1..1]`, result `bnd bool <= 1` | 4 + 4 U |
| `Q(arr_new)<T,cap(N)>` | tag 17 with `bnd(cap N)`, `bnd(elem bound of T)`, `ext = 0` | 4 U |
| `Q(tbl_new)<K,V,cap(N)>` | tag 23 with `bnd(cap N)`, `bnd(elem bound of V)`, `ext = 0` | 4 U |
| `Q(store)(arr,v)` | tag 20 `ARR_SET`: ref(container), ref(append index), ref(value); the append index is `CONST i` with `i` the source-order store ordinal for that container | 4 + 10 U |
| `Q(store)(tbl,(k,v))` | tag 27 `TBL_SET`: ref(table), ref(key), ref(value) — the tuple is the declared `(key,value)` pair | 10 U |
| `Q(fold) arr over [lo..<hi] init(e) as (acc,i)` | tag 18 `ARR_LEN`, then `hi-lo` fully unrolled iterations of tag 19 `ARR_GET(arr, CONST idx)` + tag 5 `ADD(acc, val)`; each iteration carries one `CMP` (tag 10) of `idx` against `hi` under `trip(1)` derived from `cap(N)`; result binds `acc` | 4 + n·(8 + 4 + 4) U |
| `Q(load)(tbl,k) @[trip(K)] -> hit or_else B -> out` | tag 25 `TBL_GET`: ref(table), ref(key), `fallback_blk` witness (declared id 1), `bnd(val)`, `ext = 0`; the `or_else` body `B` is emitted in the join continuation; `hit` names the hit-path binding and `out` binds the join result (`hit` on the hit path, `B`'s tail on the miss path); `Q(0)`/`Q(1)` inhabit `bool`/`u64` by the typed-zero rule of §3.2; `K` bounds the miss traversal | 8 U + body |
| `Q(halt_with)(r)` | `EMIT` (tag 15) `kind = 1` (the declared witness kind of X49), `k = 0`, then `HALT` (tag 16); admitted only in the caller entry (§8.3) | 4 + 4 U |
| `emit receipt;` | `EMIT` (tag 15) `kind = 1`, `k = 0`; a single terminal `HALT` (tag 16) closes the entry block | 4 + 4 U |
| `Q(pattern)`, `Q(workflow)`, `Q(falsify)` | one key-9 callee record `[UInt id, Bytes arg_bnds, Bytes ret_bnds, Bytes blocks]`; call sites use `OP_CALL` (tag 30), callee exits `OP_RET` (tag 31), dense ascending acyclic ids | as charged above |

Two §6.4 ambiguities are resolved here explicitly, not by inference:

- **`Qbio`, `Blanket -> inst : BioT`.** A zero-argument `Q(blanket)` instantiation lowers to `CONST` with `imm = Q(0)` and the `bnd` of the blanket's declared field type (`BioT` = `Qm[0..255]`); the blanket name is erased to that bound. The declared `Q(anchor) bzero : Q(0) : BioT` supplies genesis, so no implicit constant is introduced.
- **`Qs + Qlog`, `Q(load)(tbl1,k) @[trip(1)] -> hit or_else { … } -> out : StepOut`.** `hit` is the `TBL_GET` hit binding; `-> out : StepOut` binds the join result; `(Q(0), Q(0))` inhabits `StepOut = Q(tuple)<bool,u64>` through the typed-zero rule of §3.2. The declared `![budget(64 U)]` on `step` and `![budget(80 U)]` on `route` are checked mechanically against compositional `B_hat` (§8.4) by `qnlangc --emit-qir`; a genuine exceedance is reported as `E_SMAX_OR_BUDGET` and the §6.4 annotation is flagged rather than silently raised. No `Q(pattern)`/`Q(workflow)` is admitted above its declared ceiling.

{% include term.html id="q-anchor" text="Q(anchor)" %} commitments from `qs_history.md` may be cited as provenance; {% include term.html id="existential-invariant" text="Existential Invariants" %} reject a lowering that introduces approximate arithmetic, implicit state, or non-deterministic ordering through the {% include term.html id="simemp-gateway" text="SIMEMP Gateway" %}.

### 7.2 Self-bootstrap ladder

| Stage | Input | Output | Status |
|---|---|---|---|
| **Stage 0** | Hand-authored Rung-0 source | `Qexpr` CAE-DAG + `qir-enc-4` stream | Specified; no implementation claimed. |
| **Stage 1** | Rung-0 source describing the AST writer | Generator for `Q(array)` arena-AST and `Q(variant)` constructors | Future; respects `C_max = 8`. |
| **Stage 2** | Stage-1 generator | Self-hosted `qnlangc` reproducing Stage-0 bytes | Future; requires Step-6.3 prerequisites. |

The iterative compiler uses `Q(array)` as arena and `Q(variant)` as tagged union; recursion is a trip-bounded worklist under `C_max = 8`.

### 7.3 Step-6.3 / Bootstrap-Profile prerequisites

1. **Per-site `trip(K)` + SCC-additive `bhat`.**
2. **Fixed-width bitwise operations.**
3. **Governed `Q(hash)`** with profile-bound cost and collision-bucket discipline.

Until these have BOC evidence, Stage 1 and Stage 2 remain design targets. QnLang stays `[PENDING]`.

---

## License and Invariant Lineage

This companion is part of the {% include term.html id="licensed-work" text="Licensed Work" %} and aligns with the {% include term.html id="ssl" text="Scaling Source License (SSL 1.0)" %}. Derivatives must preserve the canonical {% include term.html id="paternity-reference" text="Paternity Reference" %}; non-scaling access is governed by the {% include term.html id="open-core-invariant" text="Open Core Invariants" %}. Its lineage is the committed Qn universe at `92487c2`: it adds a Rung-0 candidate under **Axiom 13/14**, changes no byte of `src/qnir/`, `docs/qnir*.md`, `docs/qnir_goldens.md`, the glossary, or `docs/index.md`, and leaves QnLang `[PENDING]` and `Q6` `QUEUED`.
