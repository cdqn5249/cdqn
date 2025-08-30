# Doc 3 V1.0.0 — memCDU (design-only)
## Executive summary

memCDU is the node-local Memory Context Data Unit store for agents on a cdqNetwork. It defines the canonical CDU record, deterministic canonicalization, confidence semantics, token summaries, HLC/timestamp alignment, provenance/signature rules, CRDT-first merge posture, validator compatibility policy, and the memCDU WASI API. Design-only.
---

## 1. Purpose and scope

* Purpose: node-local memory for agents.
* Scope: design-level normative contracts for CDU shape, canonicalization, confidence semantics, retention summaries, HLC rules, provenance/signing, CRDT semantics, validator policies, and WASI API. No implementation.

---

## 2. Core design principles (normative)

1. Content-addressable immutability. `id` = SHA-256(hex) of canonical JCS bytes of CDU record excluding `id`.
2. Single canonical path: JCS mandatory. CBOR forbidden in canonical path.
3. Confidence-first memory with normative formulas.
4. Asynchronous WASI components and non-blocking APIs.
5. CRDT-first merger semantics for eventual cluster consistency.
6. Provenance signing, redaction proofs, and validator rules for compatibility.
7. Explicit validator policy for unsupported content variants.

---

## 3. Key definitions (normative)

* **CDU:** Context Data Unit.
* **memCDU:** node-local memory component for CDUs.
* **Retention η:** fraction threshold used in retention selection.
* **Warmup\_proof:** signed compact digest summarizing warmup selection.

---

## 4. Canonical CDU schema (WIT excerpt — normative)

Includes full content variant list from v1.0.0.

```wit
package cdu:memcdudm;

interface cdu {
  record cdu {
    id: string,
    content: cdu-content,
    vector: list<f32>,
    hlc: record { wall-seconds: u64, wall-nanos: u32, logical: u64 },
    cdu-type: string,
    confidence-metrics: record {
      overall: f32,
      tail: f32,
      bottom_percentile: f32,
      group_confidences: list<f32>
    },
    token_confidences: option<list<f32>>,
    retention_metrics: option<record {
      retention_eta: f32,
      group_avg: f32,
      lowest_group: f32,
      warmup_proof: option<string>
    }>,
    provenance: list<string>,
    metadata: record { data-rate: f32, mime: option<string>, max_token_conf_len: option<u32> },
    quality-score: f32,
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
}

world cdu-world { export cdu; }
```

Notes: restored full variants for backward compatibility with v1.0.0.

---

## 5. Canonicalization rules (normative)

1. **Format:** JSON Canonicalization Scheme (RFC-8785) mandatory. CBOR forbidden in canonical path.
2. **Unicode:** Normalize all strings to NFC prior to canonicalization.
3. **Float formatting:** `float32` use shortest round-trip decimal with up to 9 significant digits. Lowercase `e`.
4. **Binary fields (`list<u8>`):** encode as base64url in the JSON field prior to JCS. Document chosen encoding.
5. **ID:** compute SHA-256 over JCS bytes of record with `id` omitted; encode digest as lowercase hex; set `id`.
6. **Signing:** signatures are external. Sign `id` and selected immutable claims; store signature metadata in provenance or separate signed envelope.
7. **Timestamp/HLC:** `timestamp == hlc.wall-seconds` (Unix seconds). `wall-nanos` for sub-second.

Validator note: validators must canonicalize using these exact rules to recompute and verify `id`.

---

## 6. Confidence model (normative: DeepConf-adapted)

* **Token score:** `s_t = -ln p(token)` (natural log).
* **Group confidence:** `G_i = mean(s_t for tokens in group_i)`. Store `group_confidences`.
* **Tail:** default `tail = percentile(group_confidences, 10)`.
* **Bottom percentile:** default `bottom_percentile = percentile(group_confidences, 5)`.
* **Raw overall:** `raw_overall = mean(group_confidences)`.
* **Calibration:** apply `calibrated_map` (temperature scaling minimum) to map raw → \[0,1] for `confidence-metrics.overall`. Calibration params and version must be recorded in `confidence-manager`.
* **Ranges:** calibrated metrics ∈ \[0,1].

---

## 7. Token summary and storage policy (normative)

* **Mandatory:** `group_confidences`, `tail`, `bottom_percentile`, `retention_eta` summary required for all CDUs.
* **Optional:** `token_confidences` allowed but capped; default `max_token_conf_len = 512`. If exceeded, store first N, last M, and a compact sketch (histogram + reservoir sample).
* **Warmup\_proof:** signed compact digest of warmup selection; required when retention optimization used and when exchanged across nodes.

---

## 8. Warmup, early-stop, retention (design recommendations; must be recorded)

* **Defaults:** M=8; W=32; g=16; tail percentile 10%; bottom 5%; η default 0.8. Deployments may override but must publish config record.
* **Conceptual flow (design-level):** generate warmup prefixes across M traces; compute `group_confidences`; pick warmup threshold top-η; continue generation with per-group early-stop checks; compute `retention_metrics` and sign `warmup_proof`; publish accepted traces.

---

## 9. HLC, time decay, reputation (design)

* HLC: `{wall-seconds, wall-nanos, logical}`; `timestamp == wall-seconds`.
* Default decay half-life 30 days: `decay(hours) = 2^{-(hours/(30*24))}` for reputation/trust weighting.

---

## 10. Provenance, signatures, redaction (normative)

* **Provenance records** are separate signed objects referenced by `provenance` list.
* ED25519 recommended; provenance record includes algorithm id and public key reference. Key mgmt out of scope.
* **Redaction proofs:** provide signed Merkle-style digests. Redaction must not change canonical `id`. Validators must be able to verify redaction proofs against original signed provenance.

---

## 11. CRDT-first merging (design)

* Model cluster state as append-only events. Merge functions must be commutative/associative/idempotent.
* Provide abstract ops: `add_member(cdu_id, embedding, confidence)`, `update_centroid(delta, weight)`, `merge(stateA,stateB)`. Implementers must document algebra per deployment.
* Quorum required only for governance actions.

---

## 12. memCDU WASI API (design)

All calls return `expected<T, Error>` and are async-friendly.

```wit
world memcdu { export memcdu: self.memcdu }

interface memcdu {
  publish: func(new_cdu: cdu) -> expected<string, record{code:u32,message:string}>;
  publish_traces: func(candidates: list<cdu>, retention_eta: f32) -> expected<list<string>, record{code:u32,message:string}>;
  get: func(id: string) -> expected<cdu, record{code:u32,message:string}>;
  query_by_vector: func(query: list<f32>, top_k: u32, min_confidence: f32) -> expected<list<cdu>, record{code:u32,message:string}>;
  list_crdt_events: func(since_hlc: cdu.hlc, limit: u32) -> expected<list<string>, record{code:u32,message:string}>;
  validate_proof: func(warmup_proof: string, signature: string) -> expected<bool, record{code:u32,message:string}>;
}
```

Error model: numeric `code` plus short `message`.

---

## 13. Validation and compatibility policy (normative)

* **Unknown content variants:** validator action = **quarantine** by default. Quarantine stores record in a low-trust store flagged for operator review.
* **Fallbacks:** policy options (configurable) include `accept-by-reference` (store pointer), `transcode` (if transcoders available), or `reject`.
* **Migration:** producers must supply `mime` metadata. Consumers may request re-encoded or referenced content via exchange APIs.
* **Canonicalization enforcement:** validators must recompute JCS per section 5 and compare SHA-256; mismatch → reject/quarantine per policy.

---

## 14. Security, anti-poison, rate controls (design)

* Sign `retention_metrics` and `warmup_proof`; verify on receipt.
* Rate-limit trace generation per node. Governance config controls limits.
* Reputation-weighted merges. Reputation changes require quorum.
* Poison detection: flag high-confidence but low-agreement CDUs for quarantine and lower reputation.

---

## 15. Governance & monitoring (design)

* Config records store defaults and rationales.
* KPIs: calibration drift, false-accept/reject rates, query latencies, exchange success rate, CRDT merge conflicts.
* Deployments record chosen overrides in governance registry.

---

## 16. Open design items (small appendices)

1. Warmup\_proof format (Merkle digest + signature) — appendix needed.
2. Token\_confidences sketch format (histogram + reservoir) — appendix.
3. Formal CRDT op signatures and tie-break rules — appendix.
4. Reputation numeric model and decay semantics — appendix.

---

## 17. Example CDU (illustrative)

```json
{
  "id":"<hex-sha256>",
  "content":{"text":"Result R."},
  "vector":[0.0012,-0.233],
  "hlc":{"wall-seconds":1710000000,"wall-nanos":0,"logical":1},
  "cdu-type":"semantic",
  "confidence-metrics": {
    "overall":0.92,"tail":0.85,"bottom_percentile":0.81,
    "group_confidences":[0.95,0.93,0.89]
  },
  "token_confidences":null,
  "retention_metrics":{"retention_eta":0.8,"group_avg":0.92,"lowest_group":0.89,"warmup_proof":"signed-digest"},
  "provenance":["prov-abc-1"],
  "metadata":{"mime":"text/plain","max_token_conf_len":512},
  "quality-score":0.88,
  "timestamp":1710000000
}
```
