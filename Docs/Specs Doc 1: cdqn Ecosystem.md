# Specs Doc 1: cdqn Ecosystem
**Version:** V1.0.0  
**Date:** 2025-08-11T00:00:00Z  
**Lead Author:** Christophe Duy Quang Nguyen  
**Agent:** ChatGPT (GPT-5)  
**License:** BaDaaS License – The Agile Commercial Open-Core License (Doc 2)  

---

## 1. Vision
The cdqn (context datas query nodes) ecosystem is designed to bring **beginners into the Rust community** through **vibe coding**—a high-abstraction, beginner-friendly approach using **cdqnLang**, a language built on Rust principles but expressed in **UTF-8 mathematical notation** and explicit strong typing.  
The ecosystem provides an AI-assisted learning and coding environment through **ProxyAgent**, which interprets natural language into cdqnLang, executes via **cdqnRuntime**, persists state in **cdqnDB**, and connects all devices via the **cdqNetwork** hybrid P2P model.  

End goal: **Seamless transition from beginner-friendly high abstraction to raw Rust proficiency**.

---

## 2. Design Goals
- Beginner-friendly, safe, and explicit coding.
- High mathematical expressiveness with UTF-8 symbols.
- Strong typing for all variables, actors, and structures.
- Actor-model concurrency built-in at language level.
- Offline-first hybrid networking with secure aggregation online.
- Immutable, event-sourced persistence with clear provenance.
- Guardrails to ensure compliance with laws and policies.

---

## 3. cdqnLang Specification

### 3.1 Purpose
cdqnLang is the **primary interface language** of the cdqn ecosystem, designed for beginners but powerful enough for advanced workflows. It is **compiled to Rust/WASM** by cdqnRuntime.

---

### 3.2 Syntax Cheat Sheet
| Construct | Symbol(s) | Example |
|-----------|-----------|---------|
| Assignment | `≔` | `x: i32 ≔ 5` |
| Equality | `=` | `x = 5` |
| Inequality | `≠` | `x ≠ 0` |
| Comparisons | `<`, `>`, `≤`, `≥` | `x ≤ 10` |
| Sequential flow | `:>>` | `step1() :>> step2()` |
| Mathematical power | Superscript UTF-8 | `x²`, `x³` |
| Sum/Product | `∑`, `∏` | `∑_{i=1}^{n} xᵢ` |
| Root/Integral | `√`, `∫` | `√x`, `∫ f(x) dx` |

---

### 3.3 Core Primitives in cdqnLang
- **CST (Causal System Timer)**  
  Auto-generated tag on all persistent entities. Combines:
    - Hybrid Logical Clock timestamp (1 epoch = 1 Earth year)
    - Spatial triangulation from OS timezone and node consensus
    - Device signature (CPU, storage, OS version hash)  
  Syntax: `cst()` returns CST value.
  
- **CDU (Context Data Unit)**  
  Immutable smallest persistence unit. Creation or derivation only.  
  Syntax:
```
let data: CDU ≔ createCDU(content) 
let newData: CDU ≔ deriveCDU(data, transform)
```
- **cduModel**  
Logical grouping of linked CDUs into higher-order structures.  
Syntax:
```
let model: cduModel ≔ linkCDUs(cduList)
```
---

### 3.4 Actor Model
All executable logic is expressed as **actors**.

#### Actor Rules
- Must be explicitly typed.
- No anonymous actors.
- May be autonomous (**agent** metatype) or reactive.

#### Actor Type Categories
1. **cdqnSys** – Reserved for ecosystem system functions.  
 Subtypes include:
   - `cdqnSysSecurity` – authentication, encryption, guardrails.
   - `cdqnSysDebug` – diagnostics, error tracing.
   - `cdqnSysNetworking` – P2P messaging, sync.
   - `cdqnSysData` – storage ops, indexing.
   - `cdqnSysPolicy` – rules enforcement, compliance.
   - `cdqnSysMonitoring` – system health and metrics.
   - `cdqnSysOrchestration` – actor coordination, scheduling.
2. **Custom** – User-defined actor types.
3. **agent** (metatype) – Tag applied to any autonomous actor.

#### Actor Definition Syntax
```
actor Sensor: cdqnSysData with agent ≔ { receive(input: CDU) :>> process(input) }
```
---

### 3.5 Control Structures
- **If/Else**
```
if x > 0 :>> positive(x) else :>> negative(x)
```
- **Loop**
```
for i: i32 in 1..n :>> process(i)
```
- **Workflow/Grouping with :>>**  
Replaces parentheses for nested calls:
```
operation1(a) :>> operation2(b) :>> finalize()
```
---

### 3.6 Integration with Ecosystem
- **ProxyAgent**: Sole natural-language interface; all user code is cdqnLang.  
- **cdqnRuntime**: Compiles and executes cdqnLang with WASM/WASI backend.  
- **cdqnDB**: Persists all CDUs/cduModels created in execution.  
- **Guardrails**: Enforced at syntax and execution level.

---

## 4. ProxyAgent Specification
- **Role**: Each user has exactly one ProxyAgent, which can attach to all their devices/nodes.  
- **Function**:  
  - Interpret user natural language into valid cdqnLang.  
  - Validate syntax, typing, and compliance.  
  - Route code to cdqnRuntime for execution.  
  - Present results to the user in cdqnLang.  
- **Integration**: Uses CST tagging for provenance; stores outputs in cdqnDB.

---

## 5. cdqnRuntime + cdqnCore Specification

### 5.1 cdqnCore
- Base framework defining primitives, actor rules, networking, persistence semantics.
- Guarantees that all ecosystem modules integrate consistently.
- Enforces:
  - No anonymous nodes.
  - Ephemeral key signing.
  - Guardrails for law/policy compliance.

### 5.2 cdqnRuntime
- **Execution Layer**: WASM/WASI for deterministic behavior.  
- **Actor Scheduler**: Local + distributed message passing.  
- **Persistence Layer**: Connects to cdqnDB.  
- **Networking Layer**: Hybrid P2P; offline-first.  
- **Guardrail Layer**: Policy enforcement before/during execution.

---

## 6. cdqnDB Specification
- **Type**: Immutable graph-based store.
- **Unit**: CDU; no updates, only derivations.
- **Structure**: Linked CDUs form cduModels; linked cduModels can form new cduModels.
- **Storage Priority**: Disk-first design to optimize cost, disk storage as memory.
- **Integration**: Directly consumes CST metadata.

---

## 7. Governance & Security
- All code CST-tagged and signed with ephemeral keys.
- Guardrails enforced at all layers.
- No execution of unvalidated actors.
- All persistent state stored in cdqnDB.
- Legal compliance checks before execution.

---

## 8. Example End-to-End Flow

User → ProxyAgent (natural language)
→ cdqnLang code generation
→ cdqnRuntime compilation to WASM/WASI
→ Actor Scheduler executes
→ CDU(s) created and persisted in cdqnDB with CST
→ Result in cdqnLang returned to ProxyAgent → User read the cdqnLang result

---

## 9. Example cdqnLang Program
```
let radius: f64 ≔ 5.0
let area: f64 ≔ π × radius²

actor CircleCalc: cdqnSysComputation ≔ { receive(r: f64) :>> return π × r² }

CircleCalc.receive(radius)
```
---

## License
This document and its content are licensed under the **BaDaaS License – The Agile Commercial Open-Core License (Doc 2)**.  
See Doc 2 for permitted uses, attribution, and partnership triggers.


---
