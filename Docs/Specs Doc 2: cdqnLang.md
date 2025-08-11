# Specs Doc 2: cdqnLang
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
- **Summary:** Initial release of cdqnLang specification in compliance with Doc 1 structure and Doc 2 license.  
- **Sections Affected:** All  
- **Contact:** cdqn5249@gmail.com  

---

## Purpose
cdqnLang is the **domain-specific language** of the cdqn ecosystem.  
It is designed as a **high-abstraction layer** of Rust for beginners, enabling “vibe coding” with full explicit typing, mathematical notation, and actor-model constructs.  
It provides the common ground for communication between the user and the AI (ProxyAgent) and can be compiled to or interoperate with raw Rust code.

---

## 1. Design Goals
- **Beginner-Friendly**: Hide Rust’s low-level complexity, surface only essential constructs.  
- **Mathematical Expressiveness**: UTF-8 math symbols and notations for natural readability.  
- **Strong Typing**: Every variable, actor, and structure explicitly typed.  
- **Actor Model Native**: Concurrency and message passing built-in.  
- **Integration with cdqnDB**: Shares the CDU data model for persistence.  
- **Safety by Design**: Syntax enforces clarity and guardrails.  
- **Transition Path**: Code can be lowered to Rust for advanced users.

---

## 2. Core Syntax Rules

### 2.1 Variable and Value Binding
- Assignment: `≔`  
- Equality comparison: `=`  
- Other comparisons: `≠`, `≤`, `≥`, `<`, `>`  
- Explicit typing:

let speed: f32 ≔ 42.0

### 2.2 Mathematical Constructs
- UTF-8 notation only (no ASCII fallback)  
- Powers: `x²`, `x³`  
- Symbols: ∑, ∏, ∫, √, ∀, ∃, ∈, etc.  
- Operators respect precedence of standard mathematics.

### 2.3 Flow Control
- Sequential flow: `:>>`  
- Used for:
  - Stepwise execution  
  - Replacement of nested parentheses in grouped expressions

process1(data) :>> process2(result)

### 2.4 Actors
- Defined with explicit type and behavior.  
- Actor types:
 - **cdqnSys**: Reserved for ecosystem system functions.  
 - **Custom**: User-defined types.  
 - **agent**: Metatype tag for all autonomous actors (system or custom).

Example:

```cdqnLang
actor Sensor: cdqnSysDataCollector with agent ≔ { receive(input: CDU) :>> process(input) }
```
### 2.5 Control Structures
- If/Else:

```cdqnLang
if x > 0 :>> handlePositive(x) else :>> handleNegative(x)
```
- Loops:

```cdqnLang
for i: i32 in 1..10 :>> compute(i)
```
---

## 3. Integration with cdqn Ecosystem
- **cdqnRuntime**: Executes cdqnLang with WASM/WASI backend.  
- **cdqnDB**: Stores all persistent entities as CDUs (immutable).  
- **ProxyAgent**: Translates natural language to cdqnLang code and enforces syntax rules.  
- **Guardrails**: Syntax-level checks before runtime execution.

---

## 4. Example Program

```cdqnLang
let radius: f64 ≔ 5.0
let area: f64 ≔ π × radius²
actor Calculator: cdqnSysComputation ≔ { receive(r: f64) :>> return π × r² }
Calculator.receive(radius)
```
---

## 5. Governance & Security
- Only UTF-8 syntax accepted.  
- No anonymous actors or variables.  
- All actors and messages traceable via CST/CDU tagging.

---

## License
This document and its content are licensed under the **BaDaaS License – The Agile Commercial Open-Core License (Doc 2)**.  
See **Doc 2** for permitted uses, attribution requirements, and partnership triggers.

---
