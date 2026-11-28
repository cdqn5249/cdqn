---
layout: default
title: T45 — `Q(2:split)` + `.0`/`.1` lowering
nav_exclude: true
---

# T45 — `Q(2:split)` and `.0`/`.1`

**F19:** `src/qnlangc/`, `docs/logs/qnlang/`. No `src/qnir*`,`src/qnlangc-stage1`,`docs/qnir*`,`docs/qnir_goldens.md`,`docs/_data/glossary`,spec `.md`. 

## Truth check
`docs/qnlang.md` §4 `structured := Q(2:split)(expr){T->block[;] F->block[;]}` (`parser.rs:792`,`:797`); `Q(2:split)` `Q(2:cons)`/`Q(2:split)`/`Q(2:chirality)` `§3.1`. `NodeKind::Split`. `NodeKind::Postfix`. `.exact?`. **No `.0`/`.1` spec in `docs/qnlang.md`; `.0`/`.1` parser `parser.rs` `Tok::Dot`+`Tok::Num`→`.{n}`.** `Q(2:cons)(a,b)`→`TUPLE`+`FIELD`. QnIR `OP_FIELD=4`. No new QnIR opcode needed. `gates.rs` `Split` walk. 

**`Q(2:split)`.** scrut `bool` `Q(1)=T`,`Q(0)=F`; arm `block`s `T->{->..}`,`F->{->..}`. 

## Lowering (`src/qnlangc/src/lower.rs`)
- **`NodeKind::Split`.** scrut→`sv`,`Q(0)`/`Q(1)`, `CMP(sv,Q1,rng0,rng0)`,`CMP(sv,Q0,rng0,rng0)`, run `T`/`F` blocks (bind+tail), `MUL`,`MUL`,`ADD`. guarded sum, exhaustive, single block-0. 
- **`NodeKind::Postfix` `.0`/`.1`.** `FIELD base idx(0|1)`. `.exact?` unchanged. 
gates type/purity/cost unchanged; no `E_*`. 

## Tests
`cargo test --offline` `src/qnlangc` **37/37** (+`src/qnlangc/tests/split.rs` 4: `q2split_lowers…`,`postfix_01…`,`q2split_and_projection…`,`reject…`). `src/qnir` unchanged 25/25. eval-cbor `SUCCESS`,`Rust==Python`. 

## Digests
`lower.rs bddfe009ea38b9a7d5b48051ec75870be251aa0f917a98f0a78e24cd36a39381`; `gates.rs 5705ba84…`; `parser.rs 9b1024b5ff768d0e71b84ecdf59120c1fea6b3d0a54a0778465707e0efc479db`; `tests/split.rs 1ac34afebb9c871671ace0aadcd49c601a4d07a9df8016388dfc117981c67a96`. 