# Specs Doc 2: cdqnLang + ProxyAgent
**Version:** V1.0.0  
**Date:** 2025-08-11T00:00:00Z  
**Agent:** ChatGPT: OpenAI (GPT-5 2025-08-11)  
**Lead Author:** Christophe Duy Quang Nguyen  
**Human Contributors:** …  
**License:** BaDaaS License - The Agile Commercial Open-Core License (Doc 2)  

---

## Changelog
- **Version:** V1.0.0  
- **Date:** 2025-08-11T00:00:00Z  
- **Agent:** ChatGPT: OpenAI (GPT-5 2025-08-11)  
- **Summary:** Initial release of combined cdqnLang and ProxyAgent specification in compliance with Doc 1 structure and Doc 2 license.  
- **Sections Affected:** All  
- **Contact:** cdqn5249@gmail.com  

---

## Purpose
This document specifies the **full design of cdqnLang**—the high-abstraction language of the cdqn ecosystem—and the **ProxyAgent**, the AI interface that bridges beginners’ natural language with executable cdqnLang code.  
It merges both specifications so they can be implemented as a **single tightly-integrated layer** within the cdqn ecosystem.

---

## 1. Vision
Enable beginners to **vibe code** in natural language while learning the fundamentals of Rust through a high-abstraction DSL (cdqnLang).  
The ProxyAgent interprets natural language input, translates it into cdqnLang, and returns only cdqnLang output to the user, ensuring **one shared source of truth** for both AI and human.  
The long-term goal is to gradually transition users from natural language → cdqnLang → raw Rust.

---

## 2. cdqnLang Specification

### 2.1 Design Goals
- **Beginner-Friendly**: Low barrier to entry, natural readability.  
- **Mathematical Expressiveness**: UTF-8 math symbols for direct on-screen representation of formulas.  
- **Strong Typing**: Every value, actor, and function is explicitly typed.  
- **Actor Model Native**: Concurrency and message passing at the language core.  
- **Integration with cdqnDB**: Shared CDU (Context Data Unit) model for persistence.  
- **Immutable Event Flow**: All state changes derive from prior CDUs.  
- **Safety by Design**: Syntax and type system prevent ambiguity.  
- **Transition Path to Rust**: Compatible type system for seamless lowering.

### 2.2 Syntax Rules
- **Assignment**: `≔` (UTF-8)  
- **Equality**: `=` (UTF-8)  
- **Comparisons**: `≠`, `≤`, `≥`, `<`, `>`  
- **Flow operator**: `:>>` for sequential execution and replacing nested parentheses.  
- **Superscript powers**: `²`, `³` for exponents.  
- **UTF-8 math operators**: ∑, ∏, ∫, √, ∀, ∃, ∈, etc.  
- **Explicit typing**:
```cdqnLang
let x: f32 ≔ 3.14
```
### 2.3 Control Structures
- **Conditional**:
```cdqnLang
if value > 0 :>> positiveCase() else :>> negativeCase()
```
- **Loop**:
```cdqnLang
for i: i32 in 1..10 :>> process(i)
```
### 2.4 Actor Model
- **Actor Definition**:
```cdqnLang
actor Name: ActorType [with agent] ≔ { behaviour }
```
- **Actor Types**:
- **cdqnSys**: Reserved for system-level roles (security, monitoring, networking, data, policy, orchestration, etc.).  
- **Custom**: User-defined actor types for application logic.  
- **Metatype `agent`**: Tag marking an actor as autonomous (system or custom).  
- All actors typed; untyped actors are invalid.

---

## 3. ProxyAgent Specification

### 3.1 Role
ProxyAgent is an **AI intermediary** between user and cdqnLang:
- Translates user intent from **natural language** into **syntactically-valid cdqnLang**.  
- Returns **only cdqnLang** output to the user.  
- Maintains a **1:1 relationship** with each user account.  
- Can be linked to **all nodes** owned by that user.

### 3.2 Functions
- **Interpretation**: Parse and understand beginner’s natural language instructions.  
- **Code Generation**: Produce explicit cdqnLang code according to syntax and typing rules.  
- **Validation**: Enforce syntax, type, and actor model rules before execution.  
- **Security**: Apply guardrails for lawful and safe execution; prevent malicious or illegal code generation.  
- **Persistence**: Store all interactions as immutable CDUs in cdqnDB.

### 3.3 Behavioural Rules
- No hidden state outside of cdqnDB.  
- No bypass of guardrails.  
- Operates within user’s permissions and node policies.  
- Executes within cdqnRuntime sandbox.

### 3.4 Integration Points
- **cdqnRuntime**: Executes validated cdqnLang output from ProxyAgent.  
- **cdqnDB**: Stores code, results, and context as CDUs.  
- **CST**: All generated entities timestamped with Causal System Timer metadata.

---

## 4. Example Interaction

### User Input (Natural Language)
> “Create an actor that calculates the area of a circle given a radius.”

### ProxyAgent Output (cdqnLang)
```
let radius: f64 ≔ 5.0 
let area: f64 ≔ π × radius²
actor Calculator: cdqnSysComputation ≔ { receive(r: f64) :>> return π × r² }
Calculator.receive(radius)
```
---

## 5. Governance & Security
- No anonymous actors, variables, or nodes.  
- All entities signed with ephemeral keys.  
- All ProxyAgent actions logged as immutable CDUs.  
- Syntax and guardrails prevent unsafe constructs.

---

## License
This document and its content are licensed under the **BaDaaS License – The Agile Commercial Open-Core License (Doc 2)**.  
See **Doc 2** for permitted uses, attribution requirements, and partnership triggers.

---
