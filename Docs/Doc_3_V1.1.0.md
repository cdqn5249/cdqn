# Doc 3 V1.1.0 — Unified CDU (cdqnPS-aligned)

This revision unifies the memCDU design with cdqnPS requirements and adds fields, APIs, and policies needed for cdqnPS to use the CDU as the canonical, extensible data model for semantic anchors, proposals, votes, and governance.

## Executive summary

Updates in this version: schema versioning, prime/anchor fields, scope labels, support-weight and vote references, kpis-record definition, governance metadata, extension slot for forward compatibility, API additions used by cdqnPS orchestrator, and normative clarifications to canonicalization and validator behavior.

## Important compatibility note

- Backwards compatibility: all records MUST include `schema_version`. Validators in `strict` mode may reject records with unknown `schema_version`.
- Unknown extension keys MUST be ignored in `lenient` mode and quarantined in `strict` mode. Deployments must publish governance policy for validator behavior.

## 1. Purpose and scope

Same as v1.0.0 but expanded to include canonical CDU usage across cdqnPS components: Orchestrator, Validator, ScopeManager, WeightSupport, Reputation, Monitor, and memCDU subgraph. The CDU is now the single canonical record for proposals, votes, anchors, and artifacts exchanged across the network.

## 2. Core design principles (normative)

Retain prior principles from memCDU plus:

1. CDU is authoritative for semantic anchor lifecycle and for vote/proposal metadata.
2. CDU must carry stable extension slots for cross-component metadata used by implementations like cdqnPS.

## 3. Key definitions (normative)

Add:

* **Anchor CDU**: a CDU whose `anchor_status == "anchor"` and which represents the canonical semantic object for a meaning.
* **Proposal CDU**: a CDU with `anchor_status == "candidate"` and associated `support` metadata.
* **Vote CDU**: a CDU whose `cdu-type == "vote"` and which references a target CDU id and vote outcome.

## 4. Canonical CDU schema (WIT excerpt — normative)

```wit
package cdu:memcdudm;

interface cdu {
  record cdu {
    // identity
    schema_version: string,            // e.g. "memCDU-1.1.0"
    id: string,

    // content and embedding
    content: cdu-content,
    vector: list<f32>,

    // hybrid logical clock
    hlc: record { wall-seconds: u64, wall-nanos: u32, logical: u64 },

    // type and sub-type (semantic role)
    cdu-type: string,                  // e.g. "semantic", "vote", "proposal", "anchor"
    cdu-subtype: option<string>,       // implementation-specific subtype

    // anchor/prime semantics
    prime_host: option<s32>,           // prime integer anchor host (cdqnPS uses primes on Z axis)
    prime_position: option<f32>,       // real position on primary axis if fractional
    scope_label: option<string>,       // e.g. "factual","fiction","false"
    anchor_status: option<string>,     // "candidate","anchor","deprecated","quarantined"
    anchor_id: option<string>,         // reference to canonical anchor CDU id when this is a derivative
    anchor_score: option<f32>,         // aggregated anchor score used by orchestrator/validator

    // confidence metrics
    confidence-metrics: record {
      overall: f32,
      tail: f32,
      bottom_percentile: f32,
      group_confidences: list<f32>
    },
    token_confidences: option<list<f32>>,

    // retention and warmup
    retention_metrics: option<record {
      retention_eta: f32,
      group_avg: f32,
      lowest_group: f32,
      warmup_proof: option<string>
    }>,

    // support and voting
    support: option<record {
      support_weight: u64,              // total weight from WeightSupport component
      support_sketch_ref: option<string>,// reference to sketch object
      support_refs: list<string>,       // CDU ids that contributed weight
      vote_refs: list<string>           // CDU ids for votes referencing this CDU
    }>,

    // provenance, governance, monitoring
    provenance: list<string>,
    governance: option<record {
      proposer: option<string>,
      approval_threshold: option<f32>,
      status: option<string>
    }>,

    metadata: record {
      data-rate: f32,
      mime: option<string>,
      max_token_conf_len: option<u32>,
      extensions: option<list<record{key:string,value:string}>>
    },

    // quality and reputation
    quality-score: f32,
    reputation_impact: option<f32>,    // optional impact on reputation when accepted

    // auditing
    origin_node: option<string>,
    sequence: option<u64>,

    // timestamps
    timestamp: u64
  }

  variant cdu-content {
    // Plain text
    text(string),
    html(string),
    csv(string),
    json(string),
    xml(string),

    // Mathematical representations
    math-utf8(string),
    math-latex(string),
    math-expr-struct(math-expr),

    // Images
    image-png(list<u8>),
    image-jpeg(list<u8>),
    image-gif(list<u8>),
    image-svg(list<u8>),
    image-webp(list<u8>),

    // Audio / Video
    audio-mp3(list<u8>),
    audio-wav(list<u8>),
    audio-ogg(list<u8>),
    video-mp4(list<u8>),
    video-webm(list<u8>),

    // Documents
    pdf(list<u8>),
    rtf(list<u8>),
    doc(list<u8>),
    docx(list<u8>),
    odt(list<u8>),
    ods(list<u8>),
    odp(list<u8>),
    odg(list<u8>),
    odf(list<u8>),
    abw(list<u8>),
    epub(list<u8>),

    // Archives
    archive-zip(list<u8>),
    archive-tar(list<u8>),
    archive-gzip(list<u8>)
  }

  record math-expr {
    op: string,
    args: list<math-expr>,
    value: option<f32>,
  }

  // KPIs record used by cdqnPS Monitor/Orchestrator
  record kpis-record {
    calibration_drift: f32,
    false_accept_rate: f32,
    false_reject_rate: f32,
    query_latency_ms: f32,
    exchange_success_rate: f32,
    crdt_conflict_rate: f32
  }
}

world cdu-world { export cdu; }
```

Notes: additions are `schema_version`, `prime_host`, `prime_position`, `scope_label`, `anchor_*` fields, `support` record, `governance`, `reputation_impact`, `origin_node`, `sequence`, `metadata.extensions`, and `kpis-record` declaration.

## 5. Canonicalization rules (normative updates)

- JCS (RFC-8785) remains mandatory.
- Binary fields encoded as base64url as before.
- `schema_version` MUST appear in canonical record prior to `id` computation.
- `extensions` keys must be ordered lexicographically before JCS.
- Validators must expose a `mode` option: `lenient` ignores unknown extensions; `strict` quarantines unknown fields and unknown schema_version values.

## 6. Confidence model

Retained from v1.0.0. When CDUs carry `prime_host` and `prime_position`, confidence calibration MUST consider prime-host-specific calibration curves. Calibration manager must record per-prime calibration version.

## 7. Token summary and storage policy

Unchanged. Implementers may store vote CDUs as separate CDUs of `cdu-type == "vote"` and refer via `support.vote_refs`.

## 8. Warmup, early-stop, retention

Warmup proof format defined in appendix A: `warmup_proof = base64url(MERKLE_ROOT || provenance_signature)`.

## 9. HLC, time decay, reputation

Add consideration: when computing reputation impact use `reputation_impact` and `origin_node` for CRDT merges. Reputation updates should reference CDU `id`.

## 10. Provenance, signatures, redaction

Provenance records should reference `schema_version` and `id`. Signatures should cover `id` plus `schema_version` and critical anchor claims: `anchor_status`, `prime_host`, `prime_position`, `support.support_weight`.

Redaction proofs MUST be published as CDUs of `cdu-type == "redaction-proof"` referencing the target `id` and providing a Merkle-compatible proof. Redaction must not alter original `id`.

## 11. CRDT-first merging

When merging CDUs, prefer deterministic tie-breakers: compare `hlc`, then `origin_node` lexicographically, then `sequence`. Merges must not drop `vote_refs` or `support_refs`.

## 12. memCDU WASI API (normative updates)

Additions to existing memcdu API used by cdqnPS components.

```wit
interface memcdu {
  publish: func(new_cdu: cdu) -> expected<string, record{code:u32,message:string}>;
  publish_traces: func(candidates: list<cdu>, retention_eta: f32) -> expected<list<string>, record{code:u32,message:string}>;
  get: func(id: string) -> expected<cdu, record{code:u32,message:string}>;
  query_by_vector: func(query: list<f32>, top_k: u32, min_confidence: f32, min_reputation: option<f32>) -> expected<list<cdu>, record{code:u32,message:string}>;
  list_crdt_events: func(since_hlc: cdu.hlc, limit: u32) -> expected<list<string>, record{code:u32,message:string}>;
  validate_proof: func(warmup_proof: string, signature: string) -> expected<bool, record{code:u32,message:string}>;

  // cdqnPS helper calls
  find_by_scope: func(scope_label: string, limit: u32) -> expected<list<cdu>, record{code:u32,message:string}>;
  list_pending_proposals: func() -> expected<list<cdu>, record{code:u32,message:string}>;
  get_kpis: func() -> expected<kpis-record, record{code:u32,message:string}>;
  find_cdu_by_id: func(cdu_id: string) -> expected<optional<cdu>, record{code:u32,message:string}>;
}
```

## 13. Validation and compatibility policy

- Validators must verify `schema_version` and conform to declared validator policy.
- For cdqnPS use cases, validators must support `vote` CDU verification, `support_weight` verification against WeightSupport, and warmup-proof verification.

## 14. Security, anti-poison, rate controls

Policy additions: quarantine criteria for proposals that attempt anchor replacement without matching `support.support_weight` or missing `support_sketch_ref`.

## 15. Governance & monitoring

`kpis-record` added and Monitor API must expose it. Governance fields added to CDU and governance registry must declare anchor replacement thresholds and validator modes.

## 16. Open design items (appendices)

A. Warmup_proof format (MERKLE_ROOT || signature) and verification algorithm.
B. Support sketch canonical reference format.
C. Formal CRDT op signatures and tie-break rules.
D. Per-prime calibration curves and storage format.

## 17. Example CDU (illustrative)

Below is a full example CDU in canonical JCS-ish JSON notation. This example intentionally includes common fields used by cdqnPS. It is illustrative only and not a signed/valid production CDU.

```json
{
  "schema_version": "memCDU-1.1.0",
  "id": "cdu:sha256:3b1f...exampleid",
  "cdu-type": "proposal",
  "cdu-subtype": "definition-submission",
  "content": {
    "text": "Water freezes at 0°C under standard pressure."
  },
  "vector": [0.00234, -0.00122, 0.87321, ...],
  "hlc": { "wall-seconds": 1700000000, "wall-nanos": 123456789, "logical": 42 },
  "prime_host": 101,
  "prime_position": 12.5,
  "scope_label": "factual",
  "anchor_status": "candidate",
  "anchor_id": null,
  "anchor_score": 0.0,
  "confidence-metrics": { "overall": 0.92, "tail": 0.03, "bottom_percentile": 0.05, "group_confidences": [0.9,0.95] },
  "token_confidences": [0.98,0.95,0.93,0.96],
  "retention_metrics": {
    "retention_eta": 0.75,
    "group_avg": 0.8,
    "lowest_group": 0.6,
    "warmup_proof": "b64url(MERKLEROOT||SIG)"
  },
  "support": {
    "support_weight": 1240,
    "support_sketch_ref": "sketch:cm:abc123",
    "support_refs": ["cdu:sha256:vote1","cdu:sha256:vote2"],
    "vote_refs": ["cdu:sha256:vote1","cdu:sha256:vote2"]
  },
  "provenance": ["node:us-east-1/agent-7","node:eu-west-3/agent-2"],
  "governance": {
    "proposer": "did:example:alice",
    "approval_threshold": 0.66,
    "status": "open"
  },
  "metadata": {
    "data-rate": 0.5,
    "mime": "text/plain",
    "max_token_conf_len": 1024,
    "extensions": [{"key":"cdqnps.priority","value":"high"}]
  },
  "quality-score": 0.88,
  "reputation_impact": 0.02,
  "origin_node": "node:us-east-1/agent-7",
  "sequence": 1024,
  "timestamp": 1700000000123
}
```

Notes about the example:
- `id` is shown truncated. Actual `id` must be computed over the canonical JCS including `schema_version`.
- `vector` ellipsis indicates a full float list.
- `support.support_weight` is the aggregated weight coming from WeightSupport. Validators should verify it by checking referenced support_refs or the support_sketch_ref.

---

### Appendix A — Warmup_proof verification (sketch)

1. Warmup_proof format: `warmup_proof = base64url(MERKLE_ROOT || provenance_signature)`.
2. Verification steps:
   - Decode base64url to get MERKLE_ROOT and signature blob.
   - Verify signature against known provenance key(s) listed in the CDU provenance chain.
   - Recompute Merkle path for claimed warmup traces supplied with the candidate CDU and compare root.

### Appendix B — Support sketch canonical reference

- Use Count-Min or HyperLogLog sketches for large-scale aggregation.
- Canonical reference format: `sketch:<type>:<base64url(serialized)>`.
- Validators may accept a sketch ref and perform probabilistic verification within tolerance levels defined by governance.

### Appendix C — CRDT op signatures and tie-break

- Every CRDT event must include `hlc` and `origin_node`.
- Merge tie-break order: highest `hlc` wins; if equal compare `origin_node` lexicographically; if equal compare `sequence` numeric.
- CRDT ops should not remove `vote_refs` or `support_refs` during merges.

### Appendix D — Per-prime calibration curves

- Calibration manager stores per-prime calibration versions and curves as small JSON documents:
```json
{
  "prime_host": 101,
  "calibration_version": "2025-08-01-v1",
  "curve": { "x":[0,0.5,1.0], "y":[0,0.6,1.0] }
}
```

## Change log

- v1.1.0 — Added schema_version, prime/anchor fields, support record, governance metadata, kpis-record, memcdu API helpers, canonicalization clarifications, warmup_proof spec, and CRDT tie-break rules.
- v1.0.0 — Original memCDU specification baseline.

---
End of Doc 3 V1.1.0
