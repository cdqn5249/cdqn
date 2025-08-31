# memCDU - Memory Context Data Unit
## Executive Summary

**memCDU** is a sophisticated memory architecture for artificial intelligence systems that combines the reliability of content-addressable storage with the nuance of confidence-aware reasoning. It serves as the foundational data layer for decentralized knowledge systems, enabling AI agents to store, retrieve, and validate information with cryptographic certainty and contextual awareness.

This document provides a complete technical specification and conceptual framework for memCDU, designed for both technical implementers and system architects.

---

## 1. Core Concept: The Memory Unit That Understands Itself

### 1.1 The Philosophy Behind memCDU
Traditional memory systems store data passively. memCDU creates **active knowledge units** that carry their own:
- **Provenance** (where they came from)
- **Confidence** (how reliable they are)
- **Context** (how they relate to other concepts)
- **Validation** (how to verify their integrity)

### 1.2 The CDU: An Intelligent Knowledge Container
A **Context Data Unit (CDU)** is like a smart, self-aware data package. Each CDU contains:

| Component | Purpose | Example |
|-----------|---------|---------|
| **Content** | The actual information | "Water boils at 100°C at sea level" |
| **Fingerprint** | Unique content-based ID | sha256("Water boils at 100°C...") |
| **Confidence Score** | Quality assessment | 0.95 (on a 0-1 scale) |
| **Semantic Vector** | Meaning coordinates | [0.82, -0.15, 0.37, ...] |
| **Provenance Trail** | Origin history | [Agent47@2023-05-21, Validator88] |
| **Metadata** | Technical context | Format: text/plain, Source: verified |

### 1.3 Key Innovations in Simple Terms
- **Content-Addressing**: Find data by what it *is*, not where it's stored
- **Immutable History**: Once created, a CDU never changes - updates create new CDUs
- **Built-in Trust Metrics**: Every piece of information comes with its own "quality score"
- **Decentralized Consensus**: Multiple AI systems can agree on truth without a central authority

---

## 2. System Architecture: A Layered Approach

memCDU implements a three-layer architecture that separates concerns while maintaining tight integration:

### 2.1 Layer 1: Content Manager wasi component
**Purpose**: Handles the intricacies of different information formats while ensuring security by treating all content as data rather than executable code.

**Security Principle**: All content is treated as **data-only**. No content is ever executed directly. Code-like content (e.g., scripts, programs) is stored as inert data and can only be "executed" through specific, sandboxed interpreters outside the memCDU system.

**Key Responsibilities**:
- Format-specific canonicalization (preparing content for hashing)
- Content validation and integrity checking
- Type conversion and normalization
- Security screening to prevent injection attacks

**Supported Content Types**:
```wit
variant content-type {
    // Text Formats
    text-plain,        // Plain text
    text-html,         // HTML content (sanitized)
    text-csv,          // CSV data
    text-css,          // CSS stylesheets
    text-javascript,   // JavaScript code (as data)
    
    // Structured Data
    application-json,  // JSON data
    application-xml,   // XML data
    application-yaml,  // YAML data
    
    // Mathematical Representations
    math-utf8,         // Unicode math symbols
    math-latex,        // LaTeX math notation
    math-expr-struct,  // Structured math expressions
    
    // Images
    image-png,         // PNG images
    image-jpeg,        // JPEG images
    image-gif,         // GIF images
    image-svg,         // SVG vector graphics (sanitized)
    image-webp,        // WebP images
    
    // Audio
    audio-mp3,         // MP3 audio
    audio-wav,         // WAV audio
    audio-ogg,         // Ogg Vorbis audio
    
    // Video
    video-mp4,         // MP4 video
    video-webm,        // WebM video
    
    // Documents
    application-pdf,   // PDF documents
    application-rtf,   // Rich Text Format
    application-doc,   // Legacy Word documents
    application-docx,  // Modern Word documents
    
    // Archives (treated as binary blobs)
    application-zip,   // ZIP archives
    application-tar,   // TAR archives
    
    // Programming languages (as text data only)
    application-x-python,    // Python code (as data)
    application-x-java,      // Java code (as data)
    application-x-csrc,      // C source code (as data)
    application-x-rust,      // Rust code (as data)
}
```

2.2 Layer 2: memCDU Core - The Universal Data Layer

Conceptual Foundation: The memCDU Core serves as the universal data container system that standardizes how knowledge is packaged, stored, and retrieved. It implements the principle that all knowledge, regardless of type, can be represented in a standardized format that carries both its content and metadata about that content.

How It Works: The Core layer is built around the CDU(Context Data Unit) structure, which encapsulates seven key aspects of information:

1. Identity: Cryptographic content-based addressing ensures each CDU has a unique, verifiable identity
2. Content: The actual information being stored, in its canonicalized form
3. Context: Semantic vectors that position the content in relation to other knowledge
4. Confidence: Multi-level metrics that quantify the reliability of the content
5. Provenance: A verifiable history of the content's origin and transformations
6. Temporal Context: Precise timing information using Hybrid Logical Clocks
7. Management: Data needed for storage optimization and retention policies

Technical Specification:

```wit
// Core CDU Structure
record cdu {
    // Identification
    id: string,                         // SHA-256 hash of canonicalized content
    content-type: content-type,         // Type of content (from content-manager)
    content-data: content-data,         // Actual content data
    
    // Semantic Positioning
    vector: list<f32>,                  // 256+ dimension semantic vector
    prime-host: option<s32>,            // Prime number anchor for semantic concepts
    scope: option<scope-label>,         // Semantic category label
    
    // Temporal Information
    hlc: hlc-record,                    // Hybrid Logical Clock timestamp
    
    // Confidence Metrics
    confidence-metrics: confidence-metrics, // Multi-level confidence scores
    token-confidences: option<list<f32>>,   // Per-token confidence (if applicable)
    quality-score: f32,                 // Overall quality assessment (0-1)
    
    // Retention Management
    retention-metrics: option<retention-metrics>, // Storage optimization data
    
    // Provenance and Metadata
    provenance: list<string>,           // Chain of origin and transformations
    metadata: metadata-record,          // Technical metadata
}

// Supporting Structures
record hlc-record {
    wall-seconds: u64,
    wall-nanos: u32,
    logical: u64
}

record confidence-metrics {
    overall: f32,               // Primary confidence score (0-1)
    tail: f32,                  // 10th percentile group confidence
    bottom-percentile: f32,     // 5th percentile group confidence
    group-confidences: list<f32> // Confidence scores for each content group
}

record retention-metrics {
    retention-eta: f32,         // Retention threshold parameter
    group-avg: f32,             // Average group confidence
    lowest-group: f32,          // Lowest group confidence
    warmup-proof: option<string> // Cryptographic proof of retention validity
}

record metadata-record {
    data-rate: f32,             // Information density metric
    mime: option<string>,       // MIME type (if different from content-type)
    max-token-conf-len: option<u32>, // Maximum token confidence length
    semantic: option<semantic-metadata>,   // Semantic-specific metadata
    episodic: option<episodic-metadata>,   // Event sequence metadata
    procedural: option<procedural-metadata> // Process knowledge metadata
}

// Semantic categorization
variant scope-label {
    factual,
    semi-factual,
    semi-fiction,
    fiction,
    false
}
```

Data Flow Process:

1. Ingestion: Content enters through the Content Manager and gets canonicalized
2. Packaging: The Core wraps the content with metadata, confidence metrics, and semantic context
3. Storage: CDUs are stored in confidence-tiered storage with appropriate indexing
4. Retrieval: Query processing uses multiple access paths (content, vector, metadata)
5. Maintenance: Retention policies automatically manage storage based on confidence and usage

2.3 Layer 3: cdqnPS Prime Semantic Layer - Meaning and Consensus

Conceptual Foundation: The cdqnPS layer adds the dimension of meaning and truth validation to the raw data stored in the Core layer. It implements a sophisticated system for understanding how pieces of knowledge relate to each other, how reliable they are, and how they contribute to a coherent worldview.

How It Works: cdqnPS implements three fundamental semantic concepts:

1. Prime Anchoring: Concepts are anchored to prime elements that represent their verifiability level in integers Z
   * Low primes (2, 3, 5): Factual, empirically verifiable knowledge
   * High primes (97, 101): Subjective, opinion-based, fictional content or feelings like love
   * Negative primes (-2, -3): Verified falsehoods or counterfactuals or opposites like hate
2. Semantic Positioning: Each concept is positioned in a multi-dimensional space where:
   * The primary dimension represents the fact/fiction spectrum in integers Z
   * Secondary dimensions represent various semantic relationships in real numbers R between prime elements
   * Distance between concepts indicates semantic similarity
3. Consensus-Based Validation: Truth is established through a reputation-weighted voting system where:
   * Agents with proven accuracy have greater voting power
   * Proposed knowledge must meet confidence thresholds
   * Controversial concepts require broader consensus

Key Wasi Components:

* Semantic Mapper: Positions concepts in the semantic space based on content analysis
* Consensus Engine: Manages the reputation-weighted voting process
* Truth Maintenance: Continuously evaluates and updates concept validity
* Scope Manager: Dynamically categorizes content based on semantic position

Validation Workflow:

1. Proposal: New knowledge is proposed as a potential CDU
2. Positioning: The semantic mapper determines its prime anchor and vector position
3. Validation: Validator nodes evaluate the proposal against existing knowledge
4. Voting: Reputation-weighted voting determines acceptance
5. Integration: Accepted knowledge is integrated into the semantic graph

Reputation System Mechanics:

* Nodes gain reputation for accurate contributions through positive validation outcomes
* Reputation decays over time (30-day half-life) to ensure recent accuracy matters most
* Voting weight is proportional to reputation score
* Malicious or consistently inaccurate nodes lose reputation and influence

---

3. Technical Workflows: How Memory Operations Work

3.1 CDU Creation and Storage

1. Content receipt and type identification
2. Content-specific canonicalization
3. SHA-256 hash generation for unique ID
4. Metadata attachment (vector, confidence, provenance)
5. Confidence-based storage tier placement
6. ID return for future reference

3.2 Information Retrieval Process

1. Query Formation: Create semantic vector for search concept
2. Vector Similarity Search: Find CDUs with closest vectors using HNSW algorithm
3. Confidence Filtering: Exclude results below minimum confidence threshold
4. Provenance Validation: Verify origin chain for remaining results
5. Result Ranking: Sort by combined score (relevance × confidence × provenance strength)

3.3 Confidence Calculation System

Token-Level Confidence:

```
s_t = -ln p(token)  // Negative log probability of token correctness
```

Group Confidence:

```
G_i = mean(s_t for all tokens in group_i)
```

Overall Confidence:

```
raw_overall = mean(group_confidences)
overall = calibrated_map(raw_overall)  // Temperature-scaled to [0,1]
```

Retention Threshold:

```
retention_eta = 0.8  // Default: keep top 80% by confidence
```

3.4 Consensus and Validation

Anchor Replacement Rules:

1. Challenge: New CDU challenges existing anchor with higher confidence
2. Support Check: Validates semantic support from related concepts
3. Weighted Voting: Reputation-weighted consensus among validator nodes
4. Promotion: Upon success, new CDU becomes canonical anchor with prime host

Reputation System:

* Nodes gain reputation for accurate contributions
* Reputation decays over time (30-day half-life default)
* High-reputation nodes have greater voting weight

---

4. Implementation Specifications

4.1 Storage Architecture

```
Storage Tiers:
- Tier 0: High-confidence semantic anchors (prime-host ≠ None, confidence > 0.9)
- Tier 1: Medium-confidence content (confidence > retention_eta)
- Tier 2: Low-confidence/pending content (confidence ≤ retention_eta)
- Tier 3: Episodic and procedural memory (specialized access patterns)

Indexing:
- Primary: Content-addressable by SHA-256 ID
- Secondary: HNSW vector index for semantic search
- Tertiary: Confidence-based partitioning for efficient retrieval
```

4.2 Performance Optimization

* Batch Processing: Bulk operations for CDU publication and retrieval
* Tiered Caching: Multi-level caching based on access frequency and importance
* Predictive Prefetching: Anticipate needed CDUs based on semantic patterns
* Delta Encoding: Efficient storage of similar CDUs through difference encoding

4.3 Security Implementation

* Cryptographic Signatures: All provenance entries digitally signed
* Poison Detection: Automated identification of conflicting information
* Rate Limiting: Request throttling to prevent abuse
* Access Control: Fine-grained permissions based on content type and sensitivity
* Content Sandboxing: All executable content handled in isolated environments

---

5. Use Cases and Applications

5.1 Intelligent Fact-Checking System

Process:

1. Receive claim for verification
2. Query semantic graph for related knowledge
3. Calculate confidence-weighted truth probability
4. Return result with supporting evidence chain

Example:

* Claim: "Paris is the capital of France"
* Result: 0.98 confidence with 47 supporting sources
* Evidence: [GeographyTextbook2023, WorldFactbook, ...]

5.2 Decentralized Knowledge Base

Process:

1. Multiple specialized agents contribute knowledge
2. memCDU handles conflict resolution and consensus
3. Unified knowledge graph emerges through CRDT merging
4. All agents benefit from shared understanding

Example: Medical diagnosis system combining:

* Radiology AI (image analysis)
* Pathology AI (text analysis)
* Clinical AI (patient history analysis)

5.3 Automated Research Assistant

Process:

1. Gather information from multiple sources
2. Store as CDUs with confidence scores
3. Identify consensus and conflicts automatically
4. Generate literature review with provenance tracking

---

6. Technical Reference

6.1 Content Canonicalization Rules

1. Text Content: Unicode NFC normalization → UTF-8 encoding
2. JSON Data: RFC-8785 JSON Canonicalization Scheme
3. Binary Data: Base64url encoding → canonical byte representation
4. Structured Data: Type-specific canonicalization rules
5. Security Note: All content treated as data-only; no execution within system

6.2 Mathematical Formulas

Confidence Calibration:

```
calibrated_confidence = 1 / (1 + exp(-(raw_score - β) / τ))
```

Where:

* β = calibration bias parameter
* τ = temperature parameter

Semantic Similarity:

```
similarity = cosine_similarity(query_vector, cdu_vector)
```

Reputation Decay:

```
reputation(t) = reputation(t₀) × 2^(-(t - t₀) / (30 × 24 × 3600))
```

6.3 Error Conditions and Handling

Error Condition Detection Method Resolution
ID Mismatch Hash verification Reject CDU
Low Confidence Threshold check Tier 2 storage
Provenance Break Signature verification Quarantine CDU
Vector Inconsistency Semantic validation Recalculation
Executable Content Content analysis Quarantine + alert

---

7. System Integration

7.1 API Specification

Core Operations:

```wit
// Content management
publish: func(content-type, content-data, vector, ...) → expected<string, error>
get: func(id: string) → expected<cdu, error>
query: func(vector, min-confidence, limit) → expected<list<cdu>, error>

// Semantic operations
publish-semantic: func(cdu) → expected<string, error>
query-semantic: func(vector, min-reputation) → expected<list<cdu>, error>

// System management
get-metrics: func() → system-metrics
manage-storage: func(tier, operation) → expected<bool, error>
```

7.2 Deployment Options

* Standalone: Single-node implementation for individual agents
* Federated: Multi-node deployment with CRDT-based synchronization
* Hybrid: Combination of local and federated storage tiers

7.3 Monitoring and Diagnostics

Key Metrics:

* Confidence distribution across CDUs
* Storage tier utilization rates
* Query performance and accuracy
* Consensus effectiveness and reputation distribution

Diagnostic Tools:

* CDU integrity verification utilities
* Confidence calibration validation
* Provenance chain audit tools
* Semantic consistency checkers

---

8. Conclusion

memCDU represents a significant advancement in memory systems for artificial intelligence, combining robust technical foundations with sophisticated semantic understanding. Its layered architecture provides both immediate practicality and long-term extensibility, making it suitable for everything from individual AI agents to large-scale decentralized knowledge networks.

The system's unique approach to content-aware storage, confidence-based management, and distributed consensus positions it as a foundational technology for the next generation of AI systems that require reliable, auditable, and collaborative knowledge management.

```
