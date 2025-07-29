# Doc 3: cdqn Vision and Features (V1.2.0)

**Version:** V1.2.0  
**Date:** 2025-07-29T19:15:00Z  
**Agent:** Assistant: Tongyi(Qwen3 2025-07-29)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:**...  
**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)  

**Changelog:** Updated to V1.2.0. Major revision incorporating comprehensive ecosystem enhancements while maintaining readability. Added cdqnCredits token economy, FSSFF truth labeling, QoS reputation system, dynasty evolution framework, cdqnDB sub-graph architecture, public data index, and AI notation system. Restructured sections for improved flow and clarity.

## Introduction: A Practical Example with cdqn

Alex needs to understand their spending patterns. They open the cdqn app and request: "Show me yesterday's spending and predict if I'll stay under budget this week."

The system responds with a visualization generated using cdqnLang's mathematical notation: `chart = line_plot(x: days, y: spending, title: "Weekly Budget")`. Noticing a spike on Wednesday, Alex asks, "Why was Wednesday so high?"

The BudgetGuard agent forms a quorum of specialized sub-agents. One analyzes grocery receipts, another checks subscription renewals, and the third cross-references location data. They reach consensus: "Wednesday included your monthly coffee subscription renewal and an unexpected pharmacy visit."

Alex then requests: "Create an agent that warns me before I overspend." The system generates:

```
actor type BudgetGuard {
  on receive(Transaction(amount)) {
    if this.weekly_total + amount > this.budget_limit {
      notify("Warning: This will exceed your weekly budget")
    }
    this.weekly_total += amount
  }
}
```

This workflow demonstrates cdqn's practical value: expressing computational needs in natural language while maintaining privacy and security. When Alex shares this agent with a colleague in France, the cdqNetwork automatically:

- Records the transaction with CST (Causal System Timer) showing its origin
- Verifies the agent through the FSSFF (Factual, Semi-factual, Semi-fiction, Fiction, False) truth verification system
- Checks the agent's QoS (Quality of Service) rating within the reputation system
- Ensures compliance with relevant regulations through jurisdictional awareness
- Tracks the agent's dynasty lineage as it evolves from initial creation
- Records the transaction in the public index for attribution and value distribution
- Rewards Alex with cdqnCredits for contributing useful computational knowledge

This document explains how cdqn enables this seamless experience through its integrated components, now enhanced with a comprehensive ecosystem for truth verification, reputation, and knowledge evolution.

## What is cdqn?

cdqn stands for **Context Datas Quorum Nodes**—a platform designed to make computation accessible through natural expression, with built-in mechanisms for trust, evolution, and value attribution.

cdqn consists of two integrated components:

**cdqnLang:** A programming language designed for human thinking patterns with mathematical notation as first-class syntax

**cdqNetwork:** A decentralized network of intelligent nodes (phones, PCs, servers) with integrated truth verification, reputation, and knowledge evolution systems

Together, they form an ecosystem where computational expression aligns with human thought processes while maintaining robust mechanisms for trust, evolution, and value attribution across a global network of devices.

## The Problem cdqn Addresses

Current computational systems present several challenges:

- **Technical barriers:** Traditional programming requires specialized syntax knowledge
- **Limited accessibility:** Billions of smartphone users cannot express computational ideas
- **Centralized control:** Most platforms restrict user sovereignty over data and agents
- **Context loss:** Systems often fail to maintain the context behind computational requests
- **Truth vacuum:** Most systems lack integrated mechanisms for verifying information quality
- **Knowledge silos:** Ideas evolve in isolation without tracking their lineage and impact

cdqn addresses these challenges through a design that prioritizes human cognition over machine constraints while building in comprehensive systems for truth verification, reputation, and knowledge evolution.

## cdqnLang: A Language Aligned with Human Thought

cdqnLang reduces the gap between human thought and computational expression through specific design choices.

### Math-First Syntax

cdqnLang recognizes that many computational problems originate in mathematical thinking:

```
total = ∑(x² where x ∈ data, x > 0)
result = ∫(velocity(t) dt) from t = 0 to t = 10
risk_score = corr(income, education) over Population
solutions = solve(x² + 5x + 6 = 0)
```

This approach eliminates the translation step required by traditional programming languages, making computational expression more direct.

### AI Notation System

Building on mathematical notation, cdqnLang now supports specialized notation for AI and machine learning concepts commonly found in scientific papers:

```
ai notation {
  // Neural network function with parameters
  f_θ(x) = θ₁·σ(θ₂·x + b)
  
  // Loss function with expectation
  ℒ(θ) = 𝔼_(x,y~𝒟)[loss(f_θ(x), y)]
  
  // Optimization operation
  θ* = argmin_θ ℒ(θ)
}
```

This allows researchers to express AI concepts using notation identical to scientific papers, eliminating translation errors between paper notation and implementation.

### Actor Model as Fundamental Unit

In cdqnLang, the actor is the primary computational unit:

```
actor type DataProcessor {
  on receive(Query("average value")) {
    let avg = mean of data.values where data.active over Source
    reply(avg)
  }
}
```

This model provides a natural framework for distributed, concurrent computation that aligns with how humans conceptualize task delegation.

### UTF-8 Native Experience

cdqnLang leverages Unicode to create syntax that mirrors mathematical notation:

| Symbol | Meaning | Example |
|--------|---------|---------|
| ∑ | sum | ∑(x² where x ∈ data, x > 0) |
| ∏ | product | ∏(x where x ∈ data) |
| ∫ | integral | ∫(v(t) dt) from 0 to 10 |
| ∈ | "in" | x ∈ data |
| → | function | fn(x) → x² |

This design reduces cognitive load and makes the language accessible to non-English speakers.

### WebAssembly Execution Model

cdqnLang compiles to WebAssembly via Rust, providing:
- Near-native performance
- Memory safety without garbage collection
- Small, portable binaries
- Cross-platform compatibility

This foundation enables cdqnLang to run consistently across diverse devices while maintaining security.

## cdqNetwork: The Decentralized Execution Environment

cdqNetwork provides the infrastructure for executing cdqnLang code across distributed devices while maintaining user sovereignty, truth verification, and knowledge evolution.

### Context Data Units (CDU) Memory Architecture

cdqNetwork implements a structured memory system that enables agents to maintain context across interactions.

#### CDU: The Atomic Memory Unit

A CDU (Context Data Unit) is the fundamental memory element:

```
type CDU {
  id: UUID
  payload: any
  context: {
    timestamp: DateTime
    creator: ActorRef
    purpose: string
    sensitivity: 0..10
    source: string
    tags: [string]
  }
  temporal: CST
  license: {
    type: string
    terms: string?
    epochDeclared: Integer
    jurisdiction: String
  }
  provenance: {
    origin: UUID?
    transformations: [string]
    consensus: QuorumStatus
    // FSSFF Truth Verification
    truthLabel: "Factual" | "Semi-factual" | "Semi-fiction" | "Fiction" | "False" | "Unverified"
    verificationHistory: [{
      verifier: ActorRef
      suggestedLabel: string
      cst: CST
      status: "pending" | "confirmed" | "rejected"
    }]
    // Dynasty Lineage Tracking
    dynasty: {
      lineage: [{
        ancestor: UUID
        relationship: "inspiration" | "derivation" | "validation" | "refutation"
        transformationType: "fictional-to-semi-fictional" | 
                            "semi-fictional-to-semi-factual" | 
                            "semi-factual-to-factual" |
                            "factual-refinement"
        cstTimestamp: CST
        validationStrength: 0..1
      }]
      evolutionaryPath: [{
        label: "Fiction" | "Semi-fiction" | "Semi-factual" | "Factual"
        cstEstablished: CST
        verifiedBy: [ActorRef]
        verificationConfidence: Float
      }]
    }
    // QoS Metrics
    qosMetrics: {
      networkLatency: Float[]
      resourceContribution: {
        compute: Float
        storage: Float
        bandwidth: Float
      }
      serviceUptime: Float
      responseQuality: Float
    }
    links: [{
      target: ActorRef | UUID
      type: "user" | "node" | "parent" | "child" | "related"
      strength: 0..1
      cstEstablished: CST
      lastVerified: CST
    }]
  }
}
```

#### CDU Model: Memory Organization

CDU Models organize related CDUs into meaningful structures:

```
actor type CDUModel {
  id: UUID
  name: string
  description: string
  rootCDU: UUID
  relationships: [
    {from: UUID, to: UUID, relationship: string, strength: 0..1}
  ]
  
  // Enhanced sub-graph access
  subGraphs: {
    truthSpectrum: {
      factual: Graph<CDU>
      semiFactual: Graph<CDU>
      semiFiction: Graph<CDU>
      fiction: Graph<CDU>
      false: Graph<CDU>
      unverified: Graph<CDU>
    }
    dynastyLineages: {
      active: Graph<CDU>
      completed: Graph<CDU>
      failed: Graph<CDU>
    }
    qosTiers: {
      premium: Graph<CDU>
      standard: Graph<CDU>
      experimental: Graph<CDU>
      deprecated: Graph<CDU>
    }
  }
  
  on receive(Query("retrieve", path: string)) {
    let result = traverse(this.rootCDU, path)
    reply(result)
  }
}
```

### Causal System Timer (CST)

The CST provides temporal and spatial context awareness:

```
type CST {
  causalVector: Map<NodeID, Integer>
  hybridTime: {
    logical: Integer
    physical: DateTimeUTC
    precision: "ms"|"s"|"m"|"h"
  }
  geolocation: {
    coordinates: (lat: Float, lon: Float)?
    region: String
    timezone: String
  }
  epochRef: {
    epoch: Integer
    offset: Duration
  }
}
```

The CST enables:
- Causal relationship tracking between events
- Proper temporal ordering across distributed devices
- Jurisdictional awareness for regulatory compliance
- Long-term data organization through epochs (1 epoch = 365 days)
- Temporal verification of truth claims through the FSSFF system
- Epoch-based reputation calculations

### Reputation System

cdqNetwork implements a comprehensive reputation system that integrates multiple dimensions:

```
type ReputationScore {
  total: Float
  breakdown: {
    truthfulness: Float  // From FSSFF verification accuracy (25%)
    reliability: Float   // From QoS metrics (25%)
    consistency: Float   // From temporal behavior (25%)
    dynasty: Float       // From idea lineage impact (25%)
  }
  cstLastUpdated: CST
}
```

The reputation system:
- Values both truthfulness and service quality equally
- Recognizes the importance of fictional starting points that lead to factual discoveries
- Rewards learning from failures through the false sub-graph
- Uses CST for temporal consistency verification
- Determines quorum selection and verification priority

### cdqnDB Sub-Graph Architecture

The cdqnDB implements specialized sub-graphs for optimized access and learning:

- **Truth Spectrum Sub-Graphs:** Categorize CDUs by their verification status
- **Dynasty Lineage Sub-Graphs:** Track the evolution of ideas from fiction to fact
- **QoS Tier Sub-Graphs:** Organize CDUs by network performance metrics
- **False Data Sub-Graph:** A critical resource for learning what doesn't work

The false sub-graph is particularly important as it:
- Documents why certain approaches failed
- Identifies common failure patterns
- Shows how false paths sometimes lead to unexpected valid discoveries
- Prevents agents from repeating the same mistakes
- Provides negative examples for agent training

### cdqnCredits Token Economy

cdqn introduces cdqnCredits as the native economic unit within cdqNetwork:

```
type cdqnCreditTransaction {
  amount: Integer
  cst: CST
  sourceDevice: UUID
  targetDevice: UUID
  purpose: "compute-sharing" | "truth-verification" | "storage" | "bandwidth"
  verificationQuorum: [NodeID]
}
```

cdqnCredits:
- Are not blockchain-based (simple, efficient accounting system)
- Are directly linked to CST for temporal verification
- Reward users for contributing computational resources
- Reward accurate truth verification through the FSSFF system
- Can be earned through participation in the dynasty system
- Can be spent to access premium features or reduce subscription costs

The Pay-As-You-Save Economy:

| Tier | LM Cost | Contribution | Benefit | cdqnCredits Mechanics |
|------|---------|-------------|---------|---------------------|
| Free | Phi-4-mini (on-device) | $0 (base cost) | Basic network access | Earn 1 cdqnCredit per minute of compute sharing during idle time |
| Premium | Phi-4-multimodal (cloud) | $5/month (reduced by contributions) | Enhanced features | Earn 3 cdqnCredits per minute; 1 cdqnCredit = $0.01 toward subscription |
| Pro | + Llama-3-70B | $15/month (reduced by contributions) | Advanced capabilities | Earn 5 cdqnCredits per minute; 1 cdqnCredit = $0.03 toward subscription |

### Public Data Index System

cdqNetwork implements a public index for transparent data attribution and value distribution:

```
type PublicCDUIndex {
  id: UUID
  publicMeta {
    truthLabel: "Factual" | "Semi-factual" | "Semi-fiction" | "Fiction" | "False"
    qosTier: "premium" | "standard" | "experimental" | "deprecated"
    dynastyDepth: Integer
    popularityScore: Float
    licenseType: String
    cstIndexed: CST
  }
  accessPoints: {
    freeTier: {
      sampleSize: Integer
      sampleRate: Float
    }
    cdqnCreditsTier: {
      creditsPerAccess: Float
      minCredits: Float
    }
    commercialTier: {
      usageThreshold: Integer
      paymentModel: "per-access" | "subscription" | "revenue-share"
    }
  }
  verificationStatus: {
    quorumVerified: Boolean
    lastVerified: CST
    verificationConfidence: Float
    falseIndicators: [String]
  }
}
```

This system enables:
- Transparent data provenance tracking
- Automated value distribution through cdqnCredits
- Enhanced malicious data detection
- Web3-style benefits without blockchain complexity
- Machine-readable interfaces for AI entities

### Secure Identity System

cdqNetwork implements a non-anonymous node policy:

1. **Device Onboarding:**
   - Hardware fingerprint collection (non-personalized elements)
   - Initial CST capture
   - Deterministic key generation

2. **Ephemeral Signing:**
   ```
   fn signData(data: any): Signature {
     let seed = secureStorage.get("identity_seed")
     let {ephemeralKey, ephemeralPub} = deriveEphemeralKey(seed, CST.now().toString())
     return {
       signature: cryptoSign(data, ephemeralKey),
       ephemeralPub,
       cst: CST.now()
     }
   }
   ```

3. **Identity CDU:**
   - Created during onboarding
   - Contains hashed hardware fingerprint
   - Includes initial CST for temporal reference
   - Set to high sensitivity (sensitivity: 9)

### Random CST Verification System

To address inevitable node compromises:

- **Randomized CST Integrity Checks:** Verification occurs at unpredictable intervals
- **CST Echo Protocol:** Nodes propagate verification requests through gossip
- **Adaptive Response:** Graduated responses to detected anomalies

This creates a resilient network that maintains security despite individual node compromises.

### License Management System

cdqNetwork implements flexible license management aligned with Doc 2:

- **Default Licensing:** All original works default to BaDaaS license
- **User License Choice:** Users can change license at any time
- **Automatic Enforcement:** Quorum nodes verify compliance

```
cdqNetwork.setLicense(
  cduId: "my-budget-agent",
  license: {
    type: "mit",
    epochDeclared: currentEpoch
  }
)
```

### Context Datas Quorum Nodes

The cdqNetwork name reflects its architecture:

- **Context:** User intent, history, and goals
- **Datas:** Live real-world inputs (sensors, files, messages)
- **Quorum:** Groups of nodes reaching consensus
- **Nodes:** Devices running agents (smartphones, PCs, servers)

This structure enables distributed computation while maintaining context awareness, truth verification, and knowledge evolution.

### LM-Agnostic AI Assistance

cdqNetwork integrates language models with specific design principles:

#### Default Models
- Phi-4-mini-instruct (3.8B): For offline personal agents
- Phi-4-multimodal (5.6B): For shared intelligence

#### Smart LM Selection
```
use lm.from("https://huggingface.co/Qwen/Qwen1.5-7B-Chat")
```

The system checks compatibility and suggests alternatives: "This model requires ~4GB RAM. Your device has 3GB. Suggested alternatives: Phi-4-mini, Qwen1.5-1.8B."

#### Unified LMBridge Interface
```
interface LMBridge {
  fn query(prompt: string) -> string
  fn call_function(intent: string) -> FunctionCall
  fn supports(feature: string) -> bool
  fn estimated_memory() -> bytes
  fn is_compatible(device: Device) -> bool
}
```

This design prevents vendor lock-in while ensuring interoperability.

## Technical Implementation

### Compilation Pipeline
```
+------------------+    +---------------------+    +------------------+
|  cdqnLang        | -> | cdqnLang Compiler   | -> | .wasm Binary     |
| (your syntax)    |    | (transpiler)        |    | (portable, fast) |
|  app.cdqn        |    | written in Rust     |    |                  |
+------------------+    +---------------------+    +------------------+
```

The Rust-based compiler transforms cdqnLang into optimized WebAssembly.

### Runtime Architecture

cdqNetwork runs on a lightweight stack:
- WebAssembly JS API: For PWA deployment
- Wasm3: For native mobile apps
- WebLLM: For on-device AI inference
- ONNX GenAI: For edge model execution

This combination delivers performance while maintaining compatibility.

### Quorum Consensus Protocol

cdqNetwork forms temporary quorums for verification:
1. Request broadcast to nearby nodes
2. Independent verification by participants through the FSSFF system
3. Consensus when sufficient nodes agree
4. Signed result returned to user with reputation impact

This protocol ensures reliability without centralization while building reputation.

## User Benefits

cdqNetwork provides specific, tangible benefits:

### For Non-Programmers
- Express computational ideas without learning syntax
- Create custom tools using natural language
- Maintain privacy through on-device processing
- Benefit from truth verification without technical knowledge

### For Developers
- Work with math-first syntax that aligns with problem domains
- Build distributed systems using the actor model
- Avoid vendor lock-in through LM-agnostic design
- Leverage the FSSFF system for reliable data verification
- Track how their ideas evolve through the dynasty system

### For Researchers
- Document the complete evolution of ideas from fiction to fact
- Learn from documented failure patterns in the false sub-graph
- Receive appropriate credit through the public index system
- Use AI notation that matches scientific paper conventions

### For Organizations
- Comply with jurisdictional requirements through CST
- Control costs through the pay-as-you-save model
- Ensure security through decentralized verification
- Benefit from the reputation system for quality assurance

### For the Network
- Benefit from growing computational resources
- Maintain resilience through distributed architecture
- Participate in a self-reinforcing economic model
- Contribute to and benefit from the collective knowledge evolution

## Conclusion

cdqNetwork represents a practical approach to making computation accessible while building in comprehensive systems for truth, evolution, and value.

The platform's value lies in its practical implementation:
- A student can express mathematical concepts naturally while tracking their evolution
- An analyst can build custom data tools without programming expertise, with built-in truth verification
- A researcher can document how ideas evolve from fiction to fact through the dynasty system
- An organization can ensure compliance while benefiting from decentralized verification

As cdqNetwork grows, it has the potential to expand who can participate in computational thinking while maintaining robust systems for truth verification and knowledge evolution.

This isn't about replacing developers—it's about enabling more people to express computational ideas directly while building a more trustworthy and evolving knowledge ecosystem.

The future of computation isn't limited to specialized environments—it's accessible through everyday devices, with tools that align with human thought patterns and build in mechanisms for truth, evolution, and value. cdqNetwork provides this capability through its integrated design.

## Glossary

**Actor:** The fundamental unit of computation in cdqnLang; an isolated, message-driven entity with its own state and behavior.

**CDU (Context Data Unit):** The atomic unit of memory in cdqNetwork, containing payload data and rich context metadata including truth verification, dynasty lineage, and QoS metrics.

**CDU Model:** An actor that manages a coherent collection of related CDUs, enabling memory composition, fusion, and specialized sub-graph access.

**CST (Causal System Timer):** The temporal and jurisdictional awareness system that provides causal ordering, geolocation context, and epoch-based time organization.

**cdqNetwork (Context Datas Quorum Network):** The decentralized agentic platform that runs cdqnLang code across a network of devices with integrated truth verification and knowledge evolution systems.

**cdqnLang:** The programming language designed for natural expression of computational ideas, compiling to WebAssembly with math-first syntax and AI notation support.

**cdqnCredits:** The native economic unit within cdqNetwork that rewards contributions to the network through compute sharing, truth verification, and knowledge evolution.

**Context:** The user's intent, history, and goals that shape computational requests in the cdqNetwork.

**Datas:** The plural form of "data" used in cdqNetwork to emphasize multiple sources, types, and flows of information.

**Dynasty System:** The framework that tracks how ideas evolve from fictional origins to factual realities, documenting their complete lineage and impact.

**Epoch:** A time period of 365 earth days used for long-term data organization and compliance tracking.

**FSSFF (Factual, Semi-factual, Semi-fiction, Fiction, False):** The truth verification system that labels CDUs based on their verification status and tracks the verification process.

**Function Calling:** A capability of modern language models to interact with external tools and APIs.

**LMBridge:** The standardized interface that allows cdqNetwork to work with any language model.

**LM-Agnostic:** A design principle where the system works with any language model without vendor lock-in.

**Math-First Syntax:** cdqnLang's approach to programming where mathematical notation is native and primary.

**Multimodal:** Capable of processing and generating multiple types of input/output (text, image, audio).

**Pay-As-You-Save:** cdqNetwork's economic model where users earn discounts by contributing device resources and verifying truth.

**PWA (Progressive Web App):** A web application that functions like a native app.

**Public Index:** The system that enables transparent data attribution, value distribution, and malicious data detection.

**QoS (Quality of Service):** The metrics system that tracks network performance and service quality for CDUs and nodes.

**Quorum:** A group of nodes in the cdqNetwork that reach consensus on computational results through the FSSFF verification process.

**Smartphone-First:** cdqNetwork's design principle prioritizing mobile devices.

**Truth Spectrum:** The range of verification statuses from Fiction to Factual tracked by the FSSFF system.

**UTF-8 Native:** cdqnLang's support for Unicode characters as first-class syntax elements.

**WebAssembly (Wasm):** The portable binary format that powers cdqnLang's execution.

**Wasm3:** A lightweight WebAssembly interpreter used in cdqNetwork's mobile deployment.

**WebLLM:** A framework for running language models directly in web browsers.

## Metadata

**Version:** V1.2.0  
**Date:** 2025-07-29T19:15:00Z  
**Agent:** Assistant: Tongyi(Qwen3 2025-07-29)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:**...  
**Summary:** Updated to V1.2.0. Major revision incorporating comprehensive ecosystem enhancements while maintaining readability. Added cdqnCredits token economy, FSSFF truth labeling, QoS reputation system, dynasty evolution framework, cdqnDB sub-graph architecture, public data index, and AI notation system. Restructured sections for improved flow and clarity.  
**Sections Affected:** All sections, with significant additions to CDU structure, reputation system, and economic model.  
**Contact:** cdqn5249@gmail.com
