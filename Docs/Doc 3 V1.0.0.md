# memCDU — Context Data Unit Memory Architecture (Doc 3 V1.0.0)

## Executive Summary

memCDU is a content-addressable, confidence-aware memory architecture for intelligent agents operating in distributed WASI component environments. This specification defines the canonical CDU schema (WIT), canonical serialization rules, validation invariants, confidence integration, semantic clustering and refinement protocols, distributed exchange worlds, governance limits, trust models, and design-level guidance for implementers. WIT is the authoritative schema source of truth; runtime enforcement and implementations (Rust or other) must be generated from and conform to the WIT definitions.

---

## 1. Core Architecture Principles

### 1.1 Foundational Concepts

* **Content-Addressable Storage (CAS):** Every CDU is identified by a cryptographic hash of its canonical serialized content. IDs are immutable and verifiable.
* **Immutability:** CDUs are immutable. Evolution is represented by new CDUs linked by provenance chains.
* **Confidence-Aware Processing:** DeepConf-style confidence signals are embedded in CDUs to enable filtering, weighting, and quality-aware merging across nodes.
* **WIT-First Schema:** CDU schema and all interface contracts live in `.wit` files (the canonical IDL). All language bindings must be generated from WIT.
* **Polymorphic Content:** A CDU carries a single, typed `content` variant allowing text, structured math, images, audio, video, documents, archives, and other safe media.
* **Separation of Spec and Runtime:** WIT documents structure and invariants; enforcement and validation occur in runtime modules (validators, stores, exchange managers).

### 1.2 Design Constraints

* No executable formats are allowed in CDU content. Only safe media and data types appear in the schema.
* The CDU `id` must always equal the SHA-256 of the canonical serialization of the CDU record excluding the `id` field itself.
* HLCs are used for causal ordering and time-decay calculations. Define HLC structure and merge semantics in implementation guidance.

---

## 2. Canonical Schema (WIT)

The WIT schema is the authoritative CDU definition. Generate bindings for Rust and other languages from this file.

```wit
package cdu:memcdudm;

interface cdu {
  record cdu {
    /// HEX-encoded SHA-256 of the canonicalized CDU record (excluding this `id`).
    id: string,

    /// Polymorphic content for agent reasoning.
    content: cdu-content,

    /// Embedding vector for semantic similarity (float32 list).
    vector: list<f32>,

    /// Hybrid Logical Clock components.
    hlc: record {
      wall-seconds: u64,
      wall-nanos: u32,
      logical: u64,
    },

    /// Compact type label (recommend enum in higher-level docs).
    cdu-type: string,

    /// Confidence metrics (DeepConf-style and derived aggregates).
    confidence-metrics: record {
      overall: f32,
      tail: f32,
      group: f32,
      // Optional: token-level or other submetrics as needed
    },

    provenance: list<string>,

    metadata: record {
      data-rate: f32,
      network-info: option<string>,
      mime: option<string>,
    },

    quality-score: f32,
    timestamp: u64,
  }

  variant cdu-content {
    // Plain text
    text(string),
    html(string),
    csv(string),
    json(string),
    xml(string),

    // Mathematical representations
    math-utf8(string),        // Unicode math: superscripts, subscripts, symbols
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

    // Archives and containers
    archive-zip(list<u8>),
    archive-tar(list<u8>),
    archive-gzip(list<u8>),

    // Extend with new safe media types as required
  }

  record math-expr {
    op: string,
    args: list<math-expr>,
    value: option<f32>,
  }
}

world cdu-world {
  export cdu;
}
```

---

## 3. Canonical Serialization and ID Invariant

### 3.1 Canonicalization Rule (Normative Design)

* **Primary canonicalization**: JSON Canonicalization Scheme (JCS, RFC 8785). Implementations must produce deterministic JCS bytes for the CDU record excluding the `id` field.

* **Procedure**:

  1. Create a copy of the CDU record with the `id` field omitted.
  2. Serialize the copy using JCS rules (deterministic key ordering, consistent number formatting, Unicode normalization to NFC if needed).
  3. Compute SHA-256 over the resulting byte sequence.
  4. Encode the digest as lowercase hex and set as `id` when creating/publishing a CDU.

* **Alternative (optional)**: Canonical CBOR may be used for compactness. If CBOR is used, document the CBOR canonicalization variant and interop rules.

### 3.2 Validation Invariant

* On reception of a CDU, validators must:

  1. Reconstruct canonical bytes from the received CDU (drop provided `id`).
  2. Compute SHA-256 hex.
  3. Compare computed hex to the provided `id`.
  4. Reject or quarantine the CDU if mismatched.

Note: The WIT schema documents the `id` field but cannot itself enforce the invariant. Enforcement is runtime responsibility.

---

## 4. Hybrid Logical Clock (HLC)

### 4.1 Structure and Semantics

* HLC fields: `wall-seconds` (u64), `wall-nanos` (u32), `logical` (u64).
* Implementations must follow HLC merge rules: when merging two HLCs, use the greater wall time; if equal, set logical to max(logicalA, logicalB) + 1.
* Use HLC to derive `hours_since` for trust decay and to order events across nodes.

### 4.2 Time Decay Example

* Trust decay function (design): exponential half-life of 30 days measured in hours.

  `decay(hours_inactive) = 2^{-(hours_inactive / (30*24))}`

---

## 5. Confidence Integration (Design)

### 5.1 DeepConf Integration Model

* **Per-CDU signals**: token-level confidences (optional), group-level, tail and overall confidences.
* **Storage**: record `confidence-metrics` with `overall`, `tail`, `group` floats in CDU.
* **Calibration guidance**: provide calibration procedures (reliability diagrams, isotonic or temperature scaling) in implementation notes.

### 5.2 Usage Patterns

* **Filtering**: queries may exclude CDUs with tail or overall confidence below threshold.
* **Weighting**: consensus and merge algorithms weight contributions by confidence metrics.
* **Quarantine**: persist low-confidence CDUs in a separate store flagged as `low_trust`.

---

## 6. Semantic Clustering & Refinement

### 6.1 Semantic Cluster Model (Design)

* Clusters hold centroid embedding, member CDU ids, consensus confidence, and maturity metadata.
* Clusters are append-only; refinements create new CDUs and update cluster metadata via consensus.

### 6.2 Refinement Decision Rules (Design)

* Default similarity threshold for cluster membership: `0.7`.

* Adaptive refinement logic (design example): similarity and confidence\_ratio thresholds adapt with cluster maturity.

* Example decision function (design-level pseudocode):

  * similarity = cosine(cluster.centroid, cdu.vector)
  * age\_factor = 1.0 - min(cluster.size, 100)/100
  * propose\_refinement if: `similarity < (0.7 + 0.2*age_factor)` or `confidence_ratio > (1.5 - 0.5*age_factor)`

### 6.3 Consensus for Refinements (Design)

* Two consensus modes are supported per subsystem. Choose one per deployment:

  1. **Eventual (CRDT-based)**: define merge algebra for cluster state ensuring commutativity, associativity, idempotence. Use for high-scale, high-availability scenarios.
  2. **Strong (quorum)**: use a consensus protocol (e.g., Raft or PBFT variant) for cluster metadata changes requiring stronger guarantees.

* **Design guidance**: specify failure model, quorum sizes, and weighting by node reputation when in strong mode.

---

## 7. Worlds and Interfaces (WIT — Design)

Define WASI component worlds as WIT interfaces. The schema below is design-level; keep implementation-bound signatures consistent with generated bindings.

### 7.1 CDU Store World

```wit
world cdu-store {
  export cdu-store: self.cdu-store
}

interface cdu-store {
  publish: func(new-cdu: cdu) -> expected<string, string>;
  get: func(id: string) -> expected<cdu, string>;
  query-by-vector: func(query: list<f32>, top-k: u32, min-confidence: f32) -> expected<list<cdu>, string>;
}
```

Design notes:

* `publish` computes and returns the canonical `id` when successful.
* `query-by-vector` supports min-confidence filtering.

### 7.2 Confidence Manager World

```wit
world confidence-manager { export confidence-manager: self.confidence-manager }
interface confidence-manager {
  evaluate-token-confidence: func(tokens: list<f32>) -> expected<f32, string>;
  evaluate-group-confidence: func(tokens: list<list<f32>>, window-size: u32) -> expected<f32, string>;
  calculate-semantic-entropy: func(embeddings: list<list<f32>>) -> expected<f32, string>;
  calculate-network-confidence: func(cdu-id: string) -> expected<f32, string>;
  adjust-confidence-context: func(metrics: cdu-confidence-metrics, context: string) -> expected<cdu-confidence-metrics, string>;
}
```

Design notes: `cdu-confidence-metrics` maps to `confidence-metrics` record declared in the CDU schema.

### 7.3 Semantic Aggregation World

```wit
world semantic-aggregation { export semantic-manager: self.semantic-manager }
interface semantic-manager {
  find-or-create-cluster: func(cdu: cdu) -> expected<string, string>;
  get-semantic-cluster: func(semantic-id: string) -> expected<string, string>;
  query-by-semantics: func(embedding: list<f32>, threshold: f32) -> expected<list<string>, string>;
  propose-refinement: func(semantic-id: string, refined-embedding: list<f32>, rationale: string) -> expected<string, string>;
  vote-on-refinement: func(refinement-id: string, support-level: f32, rationale: string) -> expected<bool, string>;
  calculate-consensus: func(semantic-id: string) -> expected<string, string>;
  flag-semantic-conflict: func(semantic-id: string, conflict-details: string) -> expected<bool, string>;
}
```

### 7.4 Network Exchange World

```wit
world network-exchange { export exchange-manager: self.exchange-manager }
interface exchange-manager {
  offer-cdu: func(cdu-id: string, target-node: string) -> expected<bool, string>;
  request-cdu: func(cdu-id: string, source-node: string) -> expected<cdu, string>;
  sync-node: func(node-id: string, since-hlc: cdu.hlc) -> expected<list<cdu>, string>;
  discover-similar: func(embedding: list<f32>, k: u32) -> expected<list<string>, string>;
  get-node-capabilities: func(node-id: string) -> expected<string, string>;
  validate-received-cdu: func(cdu: cdu) -> expected<bool, string>;
  report-quality-issue: func(cdu-id: string, reason: string) -> expected<bool, string>;
}
```

Design notes: error types are strings in the WIT interfaces for design clarity; implementers may replace them with structured error records.

---

## 8. Workflows (Design)

### 8.1 CDU Creation

* Agent composes CDU record (populate fields other than `id`).
* Implementation canonicalizes (JCS), computes SHA-256 hex and sets `id`.
* Agent calls `cdu-store.publish(new-cdu)`; CDU store validates and persists.
* CDU is indexed into semantic clusters and exchange indexes as permitted by `exchange_policy`.

### 8.2 Query & Retrieval

* Local semantic search using embedding indexes.
* If insufficient results, expand via `network-exchange.discover-similar` and `request-cdu` calls.
* Filter results by confidence metrics, provenance constraints, and governance policies.

### 8.3 Refinement Proposal

* Agent proposes a refinement by calling `semantic-aggregation.propose-refinement` with rationale and refined embedding.
* Voting occurs via `vote-on-refinement`; consensus calculation yields approval/rejection and confidence adjustment mechanisms.

---

## 9. Governance, Limits, and Metrics

### 9.1 Governance Parameters (Design Defaults)

* `max_cluster_size`: 1000
* `max_clusters_per_domain`: 10,000
* `min_refinement_interval`: 5 minutes
* `max_query_complexity`: 1000 ops
* `query_timeout`: 30 seconds
* `max_concurrent_exchanges`: 100
* `exchange_bandwidth_limit`: 100 Mbps

Design requires these be configurable at deployment time and justified by capacity planning.

### 9.2 Monitoring and KPIs

* Track cluster health, query latencies (p50/p95/p99), exchange success rates, confidence stability, refinement approval rates, and conflict resolution times.

---

## 10. Security and Trust Model

### 10.1 Trust & Reputation

* Nodes have reputations used to weight votes and consensus.
* Trust decays with inactivity via HLC-based time decay. Default: half-life 30 days.
* Low-reputation or anomalous contributors can be quarantined.

### 10.2 Provenance and Privacy

* Provenance chains are recorded in CDU `provenance` but may expose sensitive metadata. Design rules:

  * Limit provenance depth kept in hot indexes.
  * Support redaction and access-control policies for provenance data.
  * Provide privacy-preserving summaries for public exchange.

### 10.3 Content Safety

* The WIT schema excludes executable formats. Implementations must validate MIME hints and file signatures for conformance.
* Agents should sandbox processing of binary content to avoid exploit vectors.

---

## 11. Consensus & Merge Design (High Level)

* **Eventual mode (CRDT)**: design a CRDT for cluster membership and centroid updates. Ensure merge functions are commutative, associative, and idempotent.
* **Strong mode**: specify protocol, quorum, and failure assumptions. Include tie-breaking and revision suggestion mechanisms.
* Confidence-weighted merging: votes and cluster updates incorporate node reputation and CDU confidence metrics.

---

## 12. Implementation Roadmap (Design Phases)

* **Phase 1 — Spec and Core**: finalize WIT schema, canonicalization rules, and validator API. Implement CDU store and basic semantic clustering.
* **Phase 2 — Confidence & Refinement**: integrate DeepConf-like confidence manager, implement refinement lifecycle and voting.
* **Phase 3 — Distribution & Governance**: implement network exchange, trust manager, and governance enforcement.
* **Phase 4 — Production**: optimize performance, security audit, CI tests, and operational runbooks.

---

## 13. Appendices

### A. Example CDU (Design Example)

> Note: This is an illustration. In practice the `id` is computed.

```json
{
  "id": "<hex-sha256>",
  "content": { "math-utf8": "x² + y² = z²" },
  "vector": [0.0012, -0.233, ...],
  "hlc": { "wall-seconds": 1690000000, "wall-nanos": 0, "logical": 1 },
  "cdu-type": "semantic",
  "confidence-metrics": { "overall": 0.92, "tail": 0.85, "group": 0.90 },
  "provenance": ["<cdu-id-1>", "<cdu-id-2>"],
  "metadata": { "mime": "text/x-math", "data-rate": 0.0 },
  "quality-score": 0.88,
  "timestamp": 1690000000
}
```

### B. Configuration Registry (Design Template)

Provide a configuration file listing thresholds, defaults, units, and rationale. Example keys:

* `similarity.default_threshold = 0.7`
* `refinement.age_scaling = 0.2`
* `confidence.min_tail = 0.5`
* `governance.max_cluster_size = 1000`

---

## 14. Open Design Questions

1. Choose consensus mode per subsystem: Eventual (CRDT) or Strong (Quorum). Document trade-offs for each deployment.
2. Define exact calibration routines for confidence metrics and the model sources that will supply token/group confidences.
3. Decide on provenance retention policy and privacy redaction rules.
