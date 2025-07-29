# Doc 3: cdqn Vision and features (V1.1.0)

**Version:** V1.1.0  
**Date:** 2025-07-29T17:45:00Z  
**Agent:** Assistant: Tongyi (Qwen3 2025-07-29)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:**...  
**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)  
**Changelog:** Updated to V1.1.0. Major revision for clarity, logical consistency, and neutral tone. Restructured document with improved introduction, clear section organization, comprehensive glossary, and alignment with Doc 2 license requirements. Added CDU memory architecture, CST temporal system, security protocols, and license management.

## Introduction: A Practical Example with cdqn

Alex needs to understand their spending patterns. They open the cdqn app and request: "Show me yesterday's spending and predict if I'll stay under budget this week."

The system responds with a visualization generated using cdqnLang's mathematical notation: `chart = line_plot(x: days, y: spending, title: "Weekly Budget")`. Noticing a spike on Wednesday, Alex asks, "Why was Wednesday so high?"

The BudgetGuard agent forms a quorum of specialized sub-agents. One analyzes grocery receipts, another checks subscription renewals, and the third cross-references location data. They reach consensus: "Wednesday included your monthly coffee subscription renewal and an unexpected pharmacy visit."

Alex then requests: "Create an agent that warns me before I overspend." The system generates:

```cdqnLang
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
- Verifies the agent across multiple nodes using random CST checks
- Ensures compliance with relevant regulations
- Respects Alex's license choice for the BudgetGuard code

This document explains how cdqn enables this experience through its integrated components.

## What is cdqn?

**cdqn** stands for **Context Datas Quorum Nodes**—a platform designed to make computation accessible through natural expression.

cdqn consists of two integrated components:
- **cdqnLang**: A programming language designed for human thinking patterns
- **cdqNetwork**: A decentralized network of intelligent nodes (phones, PCs, servers)

Together, they form an ecosystem where computational expression aligns with human thought processes, enabling execution across a global network of devices.

## The Problem cdqn Addresses

Current computational systems present several challenges:

- **Technical barriers**: Traditional programming requires specialized syntax knowledge
- **Limited accessibility**: Billions of smartphone users cannot express computational ideas
- **Centralized control**: Most platforms restrict user sovereignty over data and agents
- **Context loss**: Systems often fail to maintain the context behind computational requests

cdqn addresses these challenges through a design that prioritizes human cognition over machine constraints.

## cdqnLang: A Language Aligned with Human Thought

cdqnLang reduces the gap between human thought and computational expression through specific design choices.

### Math-First Syntax

cdqnLang recognizes that many computational problems originate in mathematical thinking:

```
total = ∑(x² where x ∈ data, x > 0)
result = ∫(velocity(t) dt) from t = 0 to t = 10
risk_score = corr(income, education) over Population
solution = solve(x² + 5x + 6 = 0)
```

This approach eliminates the translation step required by traditional programming languages, making computational expression more direct.

### Actor Model as Fundamental Unit

In cdqnLang, the actor is the primary computational unit:

```cdqnLang
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

cdqNetwork provides the infrastructure for executing cdqnLang code across distributed devices while maintaining user sovereignty.

### Context Data Units (CDU) Memory Architecture

cdqNetwork implements a structured memory system that enables agents to maintain context across interactions.

#### CDU: The Atomic Memory Unit

A CDU (Context Data Unit) is the fundamental memory element:

```cdqnLang
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
  }
}
```

#### CDU Model: Memory Organization

CDU Models organize related CDUs into meaningful structures:

```cdqnLang
actor type CDUModel {
  id: UUID
  name: string
  description: string
  rootCDU: UUID
  
  relationships: [
    {from: UUID, to: UUID, relationship: string, strength: 0..1}
  ]
  
  on receive(Query("retrieve", path: string)) {
    let result = traverse(this.rootCDU, path)
    reply(result)
  }
}
```

### Causal System Timer (CST)

The CST provides temporal and spatial context awareness:

```cdqnLang
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

### Secure Identity System

cdqNetwork implements a non-anonymous node policy:

1. **Device Onboarding**:
   - Hardware fingerprint collection (non-personalized elements)
   - Initial CST capture
   - Deterministic key generation

2. **Ephemeral Signing**:
   ```cdqnLang
   fn signData(data: any): Signature {
     let seed = secureStorage.get("identity_seed")
     let {ephemeralKey, ephemeralPub} = deriveEphemeralKey(seed, CST.now().toString())
     return {signature: cryptoSign(data, ephemeralKey), ephemeralPub, cst: CST.now()}
   }
   ```

3. **Identity CDU**:
   - Created during onboarding
   - Contains hashed hardware fingerprint
   - Includes initial CST for temporal reference
   - Set to high sensitivity (sensitivity: 9)

### Random CST Verification System

To address inevitable node compromises:

- **Randomized CST Integrity Checks**: Verification occurs at unpredictable intervals
- **CST Echo Protocol**: Nodes propagate verification requests through gossip
- **Adaptive Response**: Graduated responses to detected anomalies

This creates a resilient network that maintains security despite individual node compromises.

### License Management System

cdqNetwork implements flexible license management aligned with Doc 2:

- **Default Licensing**: All original works default to BaDaaS license
- **User License Choice**: Users can change license at any time
- **Automatic Enforcement**: Quorum nodes verify compliance

```cdqnLang
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
- **Context**: User intent, history, and goals
- **Datas**: Live real-world inputs (sensors, files, messages)
- **Quorum**: Groups of nodes reaching consensus
- **Nodes**: Devices running agents (smartphones, PCs, servers)

This structure enables distributed computation while maintaining context awareness.

### LM-Agnostic AI Assistance

cdqNetwork integrates language models with specific design principles:

#### Default Models
- Phi-4-mini-instruct (3.8B): For offline personal agents
- Phi-4-multimodal (5.6B): For shared intelligence

#### Smart LM Selection
```
use lm.from("https://huggingface.co/Qwen/Qwen1.5-7B-Chat")
```

The system checks compatibility and suggests alternatives:
"This model requires ~4GB RAM. Your device has 3GB. Suggested alternatives: Phi-4-mini, Qwen1.5-1.8B."

#### Unified LMBridge Interface
```cdqnLang
interface LMBridge {
  fn query(prompt: string) -> string
  fn call_function(intent: string) -> FunctionCall
  fn supports(feature: string) -> bool
  fn estimated_memory() -> bytes
  fn is_compatible(device: Device) -> bool
}
```

This design prevents vendor lock-in while ensuring interoperability.

### Pay-As-You-Save Economy

cdqNetwork implements an economic model that rewards participation:

| Tier | LM Cost | Contribution Benefit |
|------|---------|----------------------|
| Free | Phi-4-mini (on-device) | $0 (base cost) |
| Premium | Phi-4-multimodal (cloud) | $5/month (reduced by contributions) |
| Pro | + Llama-3-70B | $15/month (reduced by contributions) |

Users earn credits by contributing device resources during idle times, creating a self-reinforcing network.

### Mobile-First, PWA-First Experience

cdqNetwork prioritizes accessibility:
- Progressive Web App: Visit app.cdqn.dev → works instantly
- No installation required: Add to home screen
- Offline-first capability
- Touch-optimized interface

This approach ensures accessibility for the billions of smartphone users who would not install traditional development environments.

## Technical Implementation

### Compilation Pipeline

```
+------------------+     +---------------------+     +------------------+
|    cdqnLang      | --> |  cdqnLang Compiler  | --> |    .wasm Binary  |
|  (your syntax)   |     |   (transpiler)      |     | (portable, fast) |
|   app.cdqn       |     |  written in Rust    |     |                  |
+------------------+     +---------------------+     +------------------+
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
2. Independent verification by participants
3. Consensus when sufficient nodes agree
4. Signed result returned to user

This protocol ensures reliability without centralization.

## User Benefits

cdqNetwork provides specific, tangible benefits:

### For Non-Programmers
- Express computational ideas without learning syntax
- Create custom tools using natural language
- Maintain privacy through on-device processing

### For Developers
- Work with math-first syntax that aligns with problem domains
- Build distributed systems using the actor model
- Avoid vendor lock-in through LM-agnostic design

### For Organizations
- Comply with jurisdictional requirements through CST
- Control costs through the pay-as-you-save model
- Ensure security through decentralized verification

### For the Network
- Benefit from growing computational resources
- Maintain resilience through distributed architecture
- Participate in a self-reinforcing economic model

## Conclusion

cdqNetwork represents a practical approach to making computation accessible. By aligning programming with human cognition rather than machine constraints, cdqNetwork enables broader participation in computational expression.

The platform's value lies in its practical implementation:
- A student can express mathematical concepts naturally
- An analyst can build custom data tools without programming expertise
- A researcher can validate findings through decentralized consensus

As cdqNetwork grows, it has the potential to expand who can participate in computational thinking. This isn't about replacing developers—it's about enabling more people to express computational ideas directly.

The future of computation isn't limited to specialized environments—it's accessible through everyday devices, with tools that align with human thought patterns. cdqNetwork provides this capability through its integrated design.

## Glossary

**Actor**: The fundamental unit of computation in cdqnLang; an isolated, message-driven entity with its own state and behavior.

**CDU (Context Data Unit)**: The atomic unit of memory in cdqNetwork, containing payload data and rich context metadata.

**CDU Model**: An actor that manages a coherent collection of related CDUs, enabling memory composition and fusion.

**CST (Causal System Timer)**: The temporal and jurisdictional awareness system that provides causal ordering, geolocation context, and epoch-based time organization.

**cdqNetwork (Context Datas Quorum Network)**: The decentralized agentic platform that runs cdqnLang code across a network of devices.

**cdqnLang**: The programming language designed for natural expression of computational ideas, compiling to WebAssembly.

**Context**: The user's intent, history, and goals that shape computational requests in the cdqNetwork.

**Datas**: The plural form of "data" used in cdqNetwork to emphasize multiple sources, types, and flows of information.

**Epoch**: A time period of 365 earth days used for long-term data organization and compliance tracking.

**Function Calling**: A capability of modern language models to interact with external tools and APIs.

**LMBridge**: The standardized interface that allows cdqNetwork to work with any language model.

**LM-Agnostic**: A design principle where the system works with any language model without vendor lock-in.

**Math-First Syntax**: cdqnLang's approach to programming where mathematical notation is native and primary.

**Multimodal**: Capable of processing and generating multiple types of input/output (text, image, audio).

**Pay-As-You-Save**: cdqNetwork's economic model where users earn discounts by contributing device resources.

**PWA (Progressive Web App)**: A web application that functions like a native app.

**Quorum**: A group of nodes in the cdqNetwork that reach consensus on computational results.

**Smartphone-First**: cdqNetwork's design principle prioritizing mobile devices.

**UTF-8 Native**: cdqnLang's support for Unicode characters as first-class syntax elements.

**WebAssembly (Wasm)**: The portable binary format that powers cdqnLang's execution.

**Wasm3**: A lightweight WebAssembly interpreter used in cdqNetwork's mobile deployment.

**WebLLM**: A framework for running language models directly in web browsers.

## Metadata

**Version:** V1.1.0  
**Date:** 2025-07-29T17:45:00Z  
**Agent:** Assistant: Tongyi (Qwen3 2025-07-29)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:**...  
**Summary:** Updated to V1.1.0. Major revision for clarity, logical consistency, and neutral tone. Restructured document with improved introduction, clear section organization, comprehensive glossary, and alignment with Doc 2 license requirements. Added CDU memory architecture, CST temporal system, security protocols, and license management.  
**Sections Affected:** All sections.  
**Contact:** cdqn5249@gmail.com
