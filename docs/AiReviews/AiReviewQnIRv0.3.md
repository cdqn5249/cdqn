# From the Qn Universe to QnIR v0.3 — Loop-3 adversarial review

**Repository audit, findings F9–F19, and answers to QO-33–QO-35**

| Field | Value |
|---|---|
| Reviewed narrative | Nguyen, *From the Qn Universe to QnIR v0.3 — Derivation of Record*, draft v0.3.0 |
| Reviewed specification | `docs/qnir.md`, v0.3.0-draft.4 |
| Author and maintainer | Christophe Duy Quang Nguyen — cdqn |
| Reviewer | Kimi K3.1-Pro (Kimi), AI-assisted adversarial review |
| Date | 29 October 2026 |
| Overall verdict | **Major revision; hold the artifact share and do not promote QnIR v0.3 to the canon** |

---

## 1. Scope, reproduction, and verdict

This review examined the supplied 18-page v0.3.0 PDF, `docs/qnir.md` v0.3.0-draft.4, the Loop-2 transcript, `docs/simemp.md` v1.3.0 §7, `docs/abstractionLayers.md` v1.9.1, `docs/index.md` v1.14.0, the navigation registry, and the glossary sources. The repository dump was extracted and read directly. A mechanical include-parity check found all eleven distinct `term.html` identifiers used by `qnir.md` in the glossary and found exact equality between those identifiers and the file's `terms_used` set, with no duplicate entries.

The dump contains no `src/` directory and no Python or Rust implementation. Consequently, this review did not execute an assembler, validator, interpreter, differential suite, or Rust toolchain. It did not reproduce website rendering or repository commit hashes. The four declared single-token edits could not be verified as edits because no prior revision or Git history was supplied; their current targets are readable, but their historical diff shape remains unverified.

**Verdict: major revision; hold the artifact share and do not promote QnIR v0.3.** The draft improves substantially on v0.2. It narrows the core, separates specification from implementation, introduces block arguments and dynamic instances, distinguishes value identity from derivation identity, and makes the promotion criterion explicitly behavioral. These are necessary advances. They are not sufficient to support the draft's assertions that Steps 2 and 3 are discharged or that Step 4's specification share is ready for implementation.

Three defects are decisive. First, the nine families still lack an instruction-level semantics and a complete byte encoding; two independent implementers cannot derive a unique machine from the text. Second, the accounting contains an incorrect refund equation and an unjustified claim that operand-bounded charges dominate actual work. Third, the conformance protocol treats agreement between two implementations as conformance even when neither has been compared with the independently derived expected result. The first artifact should therefore not be drafted until F10–F18 are repaired.

The recommended implementation choice is nevertheless clear: **Rust, standard library only, for the reference artifacts; standard-library Python for the second implementation; WebAssembly as a later portability target rather than the primary second seed.** This choice is an explicit adjudication under qualitative evidence, not a measured BOC optimum.

## 2. Findings F9–F19

### F9 — The deferral register confuses equality saturation with saturating arithmetic

**Severity: major. Affected object: `qnir.md` §1.3, Step-1 discharge.**

The v0.2 `OP_SATURATE` performs bounded **equality saturation** over an expression graph, with a pinned schedule, epoch and node ceilings, and a versioned extractor. Section 1.3 instead describes “saturating arithmetic semantics” and binds the deferral to width and overflow policy. These are different operations with different semantics, costs, and evidence requirements. The current register therefore does not account for the extension that Loop 2 actually deferred.

**Required repair.** Replace the row with bounded equality saturation and its actual obligations: schedule identity, `K_max`, `N_max`, extractor generation, canonical output identity, measured divergence, and `QO-28` lineage. If fixed-width saturating arithmetic is desired, register it as a separate candidate with its own width and overflow contract; it remains outside the exact-integer core by default.

### F10 — Family-level descriptions are not an implementable transition relation

**Severity: fatal to Step-2 discharge. Affected object: `qnir.md` §§1.2, 2.3, 2.5.**

Section 2.3 supplies one prose row per family, not one rule per instruction. It does not enumerate the exact arithmetic operations, their operand and result types, the branch instructions and condition grammar, the tuple fields, the comparison result representation, the zoom-normalization function, the witness payload, the legal halt outcomes, or the instructions that birth and consume each status. “Integer operations over exact values” and “behavior at declared zoom boundaries is part of the declared function” name obligations; they do not define behavior.

Two implementers can therefore write different machines while each honestly cites the same nine rows. Byte disagreement would then measure under-specification rather than implementation error.

**Required repair.** Publish a closed instruction schema and a small-step rule for every instruction. Each rule must state operands, static types, preconditions, result, status effects, receipt effects, PC/block transfer, charge, and all rejection cases. Define the normalizer and zoom-boundary comparison as executable mathematical functions. The terminal outcome vocabulary must be closed and enumerated.

### F11 — The canonical encoding required by the assembler does not yet exist

**Severity: fatal to Step-2 discharge and assembler-first sequencing. Affected object: `qnir.md` §§2.1, 2.6, 4.2.**

Section 2.6 gives a partial integer representation but no complete encoding for an instruction, block, Intent, dependency environment, profile, or transition record. The following remain unspecified: opcode and type-tag assignments; field widths and order; block and edge identifiers; argument counts; the actual sign convention; the length-prefix encoding; tuple and zoom-carrying values; `EXT` record layout; profile and version encoding; and the relationship between raw 64-bit words and deterministic CBOR. A “declared sign convention” is not a sign convention, and a “canonical 64-bit word encoding” cannot be inferred from a value-magnitude convention alone.

**Required repair.** Specify a versioned byte grammar before writing the assembler. Include a normative source grammar, a unique binary grammar, canonical maps and identifiers, unknown-field rejection, size bounds, and worked byte strings. Provide hand-derived encoding vectors for zero, signed values, multiword values, tuples, blocks, back-edges, `EXT`, and malformed inputs. Deterministic CBOR may encode the outer artifacts while fixed 64-bit words encode instructions, but that boundary must be explicit.

### F12 — Token and terminal semantics contradict the freeze and permit success with unresolved debt

**Severity: fatal to Step-2 discharge. Affected object: `qnir.md` §§1.2 and 2.5.**

The freeze correctly states that `STATUS_NEG_EVEN_ROOT` maps to `NO_SOLUTION` in the real frame **unless** the program explicitly promotes the frame. Section 2.5 reverses this condition: it maps to `NO_SOLUTION` **if and only if** the program promotes the frame. Promotion is precisely the act that can recover the operation in a wider frame; it cannot be the condition for a real-frame no-solution verdict.

The terminal-wins rule also selects “the program's halt outcome” whenever no fatal token exists, even if semantic tokens remain outstanding. A program can therefore halt `SUCCESS` while the receipt merely itemizes unresolved semantic debt. That reintroduces the success-shaped unresolved outcome that `QO-22` rejected. Finally, a lookup-miss token is consumed when the fallback block “completes,” without distinguishing successful recomputation from a failed or token-bearing fallback.

**Required repair.** Define an explicit total terminalization function. Without promotion, a real-frame negative even root terminalizes as `NO_SOLUTION`; valid promotion consumes that token and applies the wider-frame rule. Outstanding semantic tokens must determine a non-success terminal outcome according to a declared priority. Consume `STATUS_LOOKUP_MISS` only after a successful fallback result; otherwise propagate or terminalize the resulting status. Add vectors for every combination.

### F13 — The escrow invariant double-counts refunds

**Severity: fatal to Step-3 discharge. Affected object: `qnir.md` §3.3.**

The draft declares the budget balance as “declared budget minus charged plus refunded.” This is incorrect if `charged` is the accumulated actual charge. For a budget of 100, a reserve of 100, a charge of 60, and a terminal refund of 40, the stated expression yields 80 although only 40 remains spendable. The refund releases escrow; it does not create new budget.

**Required repair.** Use the conservation identities:

- `reserved = charged + refunded + reserve_outstanding`;
- `remaining_budget = W_budget − charged`;
- `refund_final = B(P) − charged` when the whole admission bound was reserved and no other reserve remains;
- `reserve_outstanding = 0` after terminal settlement.

Define whether `charged` includes the gateway and all completed-transition charges, and keep that definition identical in the receipt, transition records, and ledger. Add the numerical example above as a conformance vector. If rejected instructions intentionally post zero charge, state separately that `Work(E)` is a policy charge, not a measure of all attempted physical work.

### F14 — Operand-bounded charges do not dominate the specified actual work

**Severity: fatal to the “never under-billed” claim. Affected object: `qnir.md` §§2.6, 3.2, 3.5.**

A bound on operand transport words does not bound all work in a transition. Full tuple comparison, minimal-zoom normalization, dependency lookup, transition-record encoding, receipt construction, and especially pre/post state-root computation can depend on the entire dynamic store, token pool, receipt queue, dependency environment, or execution length. The dynamic store grows with executed instructions; the receipt queue can grow at every step. A site-stable charge based only on static operand bounds can therefore remain constant while the specified commitment work grows linearly with execution history.

**Required repair.** Choose one of two honest models. In an abstract-U model, charges are declared logical metering quantities and no domination of implementation work is claimed until calibration. In a cost-bound model, add static bounds for every state-dependent operation and express the per-step commitment charge as a function of the relevant state and step bounds. Include dependency lookup and proof/hash work. `B(P)` must then include these path-dependent but statically bounded terms.

### F15 — Canonical state ordering and commitment claims are not total

**Severity: major; fatal before state-root implementation. Affected object: `qnir.md` §2.6.**

The token pool is ordered by `(birth step index, type id)`. Two tokens of the same type can be born at one step at different positions or through different operations, so this key does not define a unique order. The complete birth site and a unique token identifier are absent from the sort key. Program roots are also named without a complete Intent encoding, as F11 records.

The statement “equal commitments denote equal executions” is too strong for hashed state roots. Equal canonical record bytes can define equal transitions by construction; equal hash commitments do so only under an explicit collision-resistance assumption and full-preimage comparison policy.

**Required repair.** Give every token a unique identifier and sort by the full identifier, for example `(birth step, block id, instruction index, token serial, type id)`. Specify every other map key and duplicate rule. State hash claims conditionally and retain full canonical bytes for equality decisions and collision handling.

### F16 — `B(P)` is defined, but sound admission is not yet discharged

**Severity: major. Affected object: `qnir.md` §§3.4, 4.2.**

The definition of `B(P)` is useful, but the absence of an algorithm means that sound control-flow admission remains an obligation rather than a discharged acceptance condition. A validator cannot compute the declared maximum from the current text.

The surrogate rule is also internally inconsistent. Section 3.4 says an over-approximation is safe and an under-approximation is caught by the backstop. Section 4.2 says the surrogate must never under-approximate, then again invokes the backstop for under-approximation. The backstop limits damage; it does not make a false admission satisfy the admission rule.

**Required repair.** Define `B_hat(P)` as the validator's computed surrogate and require a proof or checked argument that `B_hat(P) ≥ B(P)`. False rejection is permitted; false admission is a validator defect even when the fuel backstop later stops execution. Keep fuel as defense in depth. Mark Step 3 discharged only after at least one sound, terminating surrogate algorithm and its complexity bound are specified and tested.

### F17 — Mutual byte agreement is not conformance to the specification

**Severity: fatal to Step-4 specification discharge. Affected object: `qnir.md` §4.1.**

Section 4.1 states that an implementation whose outputs agree byte-for-byte is conformant. Two implementations can share the same misunderstanding and agree on every incorrect output. The vector format of §4.4 contains an expected outcome, but the protocol's conformance rule does not require either implementation to match it.

The independence clause is also weaker than its rhetoric. Separate filenames and different languages reduce shared-code failure but do not establish independent derivation, independent authorship, or uncorrelated interpretation. Conversely, executing the same encoded Intent and version tuple is necessarily common input; the phrase “no common input beyond” should be limited to implementation guidance and expected outputs.

**Required repair.** A vector passes only when both implementations independently match the committed golden result at all five agreement points. Golden outcomes must be derived from the normative rules, reviewed independently, and never generated solely by either implementation. A disagreement between implementations is adjudicated against the golden result and specification. Record implementation author, toolchain, and derivation history as independence evidence.

### F18 — Assembler-first sequencing is premature, and its proposed evidence is insufficient

**Severity: major; fatal to the current artifact plan. Affected objects: PDF §§3.1–3.3 and `qnir.md` §4.**

The candidate scan rejects vector-suite-first work because encoded vectors “must come from” the assembler. That reasoning makes the first implementation normative in practice. Conformance requires the opposite dependency for at least a seed corpus: source and expected binary/output vectors must be derivable from the specification before an assembler exists. The current scan also assumes an encoding complete enough to assemble, which F11 shows is absent.

Double-encoding a corpus with one assembler tests repeatability of that assembler, not correctness. Re-executing one declared command before commit demonstrates reproducibility, not independent reproduction. Neither test detects a systematic encoding misunderstanding.

**Required repair.** Adopt this order: complete semantics and byte grammar; publish a small hand-derived golden corpus; implement assembler and validator; implement the reference interpreter; implement the independent second interpreter; then expand implementation-generated vectors while retaining independently reviewed goldens. Describe same-binary reruns as reproducibility evidence only.

### F19 — The seed-language BOC is qualitative adjudication, and QnLang is conflated with QnIR

**Severity: major. Affected object: PDF §§4.1–4.4.**

Table 6 assigns “High/Medium/Low” scores but does not tabulate Tier-1 admission, the constitutional `ImplicitDeps` key, measured `DeltaW_worst`, coupling counts, target coverage, or residual ties under `simemp.md` §7. Industry adoption and native speed do not by themselves establish BOC selection. Rust can still be selected, but the record must call the decision an explicit adjudication under qualitative rubric evidence.

The statement that the nine QnIR families are “the minimum QnLang instructions” is also unjustified. QnIR is an execution and verification substrate; QnLang is a surface language with its own syntax, typing, and compilation relation. A Rust or Python oracle is not the first generation of QnLang. Surface-language generations need not inherit the implementation language of the oracle.

**Required repair.** Record the language verdict as QO-33 below, with hard requirements and an explicit qualitative tie resolution. Replace “minimum QnLang instructions” with “minimum QnIR core families.” Define QnLang later through a source-to-QnIR compilation and simulation relation, not through recursive extension of the oracle's implementation language.

## 3. Disposition of the repair steps and repository state

| Object | Disposition | Reason |
|---|---|---|
| Step 1 — frozen minimal core | **Conditional pass after F9** | The nine-family boundary and extension deferrals are useful; the saturation row must name the actual deferred operation. |
| Step 2 — operational semantics | **Not discharged** | F10–F12 and F15 leave instruction behavior, bytes, terminalization, and canonical state ordering under-specified. |
| Step 3 — work accounting | **Not discharged** | F13 breaks conservation, F14 invalidates actual-work domination, and F16 leaves the admission algorithm open. |
| Step 4 — conformance specification | **Not discharged** | F17 invalidates the conformance rule; F18 reverses part of the required evidence order. |
| Promotion criterion | **Unchanged and unmet** | No executable artifacts or differential results exist in the dump. Steps 5 and 6 also remain forward. |
| Glossary include parity | **Pass on supplied bytes** | Eleven distinct includes; all resolve; exact `terms_used` parity; no duplicates. |
| Portal/navigation state | **Consistent on supplied bytes** | QnIR remains Draft/[PENDING], and the navigation description matches draft.4. |
| Four declared single-token edits | **Not independently verified** | Current targets exist, but no prior bytes or Git history were supplied. |
| `abstractionLayers.md` reference | **Minor citation drift** | The PDF references v1.9.0; the dump contains v1.9.1. The latter's rendering-fix record does not alter the reviewed QnIR semantics. |

## 4. Answers to the open questions

### QO-33 — Reference seed and oracle implementation language

**Verdict. Select Rust, standard library only, for the reference artifacts, but only after F10–F18 are repaired. Treat this as explicit qualitative adjudication, not a demonstrated measured BOC optimum.**

Determinism, exact arithmetic, static legality, and independent implementability are hard admission requirements. Rust satisfies them with checked integer operations, explicit data structures, no required external crates, and a small native runtime surface. Its lack of a standard bignum is acceptable: a bounded-purpose limb implementation can directly realize the canonical magnitude-word format. That implementation must be independently tested against the Python second seed rather than assumed correct because it mirrors the encoding.

WAT/wasm-wasi is a strong later portability target, but it adds a host runtime and WASI contract to the trusted and versioned base. It does not maximize arithmetic independence if it repeats the same limb algorithms. Lower assembly minimizes language runtime but multiplies architecture-specific work and memory-safety risk without improving the semantic oracle. Reserve it for a future substrate contract.

The artifact record should pin `rustc`, target triple, compile flags, encoding/profile versions, and dependency lock status. No floating point, locale, wall-clock time, randomized iteration, or platform-endian output is permitted. The choice does not authorize `src/qnir_assemble.rs` before the byte grammar and golden vectors exist.

### QO-34 — Invariants required at every generational step

**Verdict. Preserve the following twelve invariants explicitly. A generation that cannot preserve one must create a new versioned semantics and a migration proof; it must not silently reinterpret an old artifact.**

1. **Closed core:** only the declared instruction set has semantics; unknown and deferred constructs fail statically.
2. **Deterministic evaluation:** for fixed `(Intent, D, V)`, the transition sequence and terminal outcome are unique.
3. **Exact values:** no wrap, silent truncation, floating point, or implicit saturation; profile bounds have typed rejection behavior.
4. **Canonical bytes:** every value, Intent, state, record, and receipt has one versioned encoding; malformed and unknown forms are rejected.
5. **Name and block semantics:** static IDs, dynamic instances, block arguments, and edge arity/type rules remain deterministic.
6. **Status conservation:** every token has a unique birth, lifecycle, and terminal disposition; unresolved semantic debt cannot produce success.
7. **Work conservation:** one charge per completed transition; reserve, charge, refund, and outstanding escrow satisfy the corrected identities; no refund creates budget.
8. **Sound admission and fuel:** `B_hat(P) ≥ B(P)`; every completed step consumes positive fuel; backstops never legitimize a false admission.
9. **Complete execution commitments:** records bind all semantic state changes, versions, charges, statuses, and receipts; hashes never replace full-byte equality decisions.
10. **Versioned derivation:** normalizer, encoder, profile, hash, extractor, and other semantic generations are explicit in the relevant identity and lineage.
11. **Extension isolation:** each extension supplies typing, transitions, costs, rejection behavior, vectors, and a proof or measurement artifact before admission.
12. **Golden differential conformance:** every implementation matches independently derived golden vectors; mutual agreement alone never establishes correctness.

Each generation should publish an invariant checklist, old/new vectors, and an explicit compatibility statement. Immutable old artifacts retain their original semantics version.

### QO-35 — Second implementation and the role of wasm/wasi

**Verdict. Use standard-library Python for the second implementation. Use wasm/wasi later as a third portability realization, not as the principal independence contrast.**

Python provides an independently implemented arbitrary-precision integer, reducing the probability that both interpreters repeat the same limb-arithmetic defect. Its runtime and implementation style differ substantially from Rust, which increases diagnostic diversity for parsing, state transitions, token accounting, and canonical ordering. Performance is secondary for a small conformance corpus.

The second artifact must pin its Python version and use explicit sorting, fixed integer formatting, no hash-order dependence, no floating point, no locale/time input, and no reference to Rust source or Rust-generated expected outcomes. Independent authorship or at least an independent implementation review should be recorded; language diversity alone is not proof of independence.

The golden vectors must precede or be developed independently of both implementations. If Rust and Python disagree, the specification and golden derivation adjudicate. A later wasm realization can test portable execution of the already frozen semantics and can compare against the same goldens without displacing the higher-diversity Python seed.

## 5. Normative repair sequence

The order below is binding because each stage supplies the artifacts required by the next.

| Step | Action | Acceptance condition |
|---|---|---|
| R1 | Correct scope and terminal semantics | F9, F10, and F12 repaired; closed instruction inventory, per-instruction rules, and total token/outcome function published. |
| R2 | Complete deterministic encodings | F11 and F15 repaired; full byte grammars, unique map ordering, and hand-derived binary vectors published. |
| R3 | Repair accounting and admission | F13, F14, and F16 repaired; conservation identities hold on numerical vectors; actual-work claims are scoped or bounded; a sound surrogate is specified. |
| R4 | Repair the conformance contract | F17 repaired; golden-first comparison is mandatory; independence and disagreement adjudication are explicit. |
| R5 | Re-sequence the artifact share and record the BOC verdict | F18 and F19 repaired; hand-derived goldens precede code; Rust/Python choices and qualitative adjudication are registered. |
| R6 | Implement and reproduce | Rust reference and Python second implementation independently match all goldens; only then resume the remaining Step-4 artifact share and the existing Steps 5–6. |

## 6. Conclusion

QnIR v0.3 is substantially better organized than v0.2, but its present maturity is a **repair program with partial semantics**, not a frozen executable specification. The principal advance is the movement from architectural prose toward repository-checkable artifacts. The principal remaining error is premature discharge: naming a function or a checking surface is treated as equivalent to defining it sufficiently for independent implementation.

The next increment should therefore contain no production implementation. It should complete the instruction rules, bytes, terminalization, accounting identities, and golden conformance contract. Rust and Python are appropriate implementation choices once those obligations are discharged. The promotion criterion should remain unchanged and unmet until executable evidence exists.

---

## Review limitation

This is an AI-assisted technical review of the supplied repository dump. It is not human peer review, a security audit, a correctness proof, or independent execution of the proposed oracle. All claims marked as mechanically checked are limited to the file inventory and glossary/include-parity checks expressly described in §1.
