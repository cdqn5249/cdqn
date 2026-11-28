---
layout: default
title: T44 — `tmp_dir` concurrent isolation + `Q(use)` §7.1 truth check
nav_exclude: true
---

# T44 — `src/qnlangc/tests/modules.rs` `tmp_dir` atomic isolation

**F19:** `src/qnlangc/`, `docs/logs/qnlang/`, optional `CONTEXT_BOOT`. No `src/qnir*`,`docs/qnir*`,`docs/qnir_goldens.md`,`docs/_data/glossary`,spec `.md`. 

## Fix
`tmp_dir(tag)` (`modules.rs` tests) was `std::env::temp_dir()/qnlangc-modules-{pid}-{tag}` + `remove_dir_all`; `cyclic_import_is_rejected`(~:57) and `call_cycle_and_self_call_are_rejected_statically`(~:134) shared `tag="cycle"`, same `{pid}`, cargo concurrent → flaky `E_*`. Now `static TMP_COUNTER: AtomicU64`; `fetch_add(SeqCst)` + `SystemTime::now() nanos` + `tag` + `pid` → unique per invocation; no shared tag. Callers `tmp_dir("digest"/"cycle"/"impure"/"intra")` isolated; `remove_dir_all` then `create_dir_all`. No other `tmp_dir` in `src/qnlangc/tests/`. 

## Verified
`cargo test --offline` `src/qnlangc` **33/33**; **5 consecutive full + 5 `--test modules` `modules` 6/6, 0 flakes.** 

## `docs/qnlang.md` §7.1 `Q(use)` (v0.1.0-draft.5)
`Q(use) alias : "<path.qnl>" @ sha-256:<hex64>`. path relative; bytes exact else `E_DIGEST_MISMATCH`. pure `Library` `Q(type)/Q(anchor)/Q(variant)/Q(pattern)/Q(workflow)/Q(falsify)`, zero `emit`, no side-effect statements else `E_IMPURE_LIBRARY_IMPORT`. **Acyclic DAG `E_CYCLIC_IMPORT`. domain subset `E_MODAL_OR_DOMAIN_LEAK`. Checker G1-G14+E_NAMESPACE_COLLISION + Exposer before linking.** **`alias::name`.** `Q(pattern)->OP_CALL`,`Q(workflow)`,`Q(falsify)` key9 once `callees`. **Namespacing:** `alias::` applies to `Q(pattern)/Q(workflow)/Q(falsify)`; `Q(type)/Q(anchor)/Q(metric)/Q(variant)`/`Q(variant)` constructors NOT `alias::`-qualified; **duplicate `Q(type)/Q(anchor)/Q(metric)/Q(variant)` names (`Q(variant)` ctor names) → `E_NAMESPACE_COLLISION`; no diamond dedup.** (spec lists `Q(anchor)/Q(type)/Q(variant)`, code `modules.rs` `rename_top` + `gates.rs`.) 