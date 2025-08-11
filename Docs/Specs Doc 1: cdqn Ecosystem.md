# Specs Doc 1: cdqn Ecosystem
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
- **Summary:** Initial release of full cdqn ecosystem specification in compliance with Doc 1 structure and Doc 2 license.  
- **Sections Affected:** All  
- **Contact:** cdqn5249@gmail.com  

---

## Purpose
This document provides the **full technical specification** for the cdqn (context datas query nodes) ecosystem, integrating all previously validated designs:  
- cdqnLang DSL  
- cdqnRuntime  
- cdqnDB  
- Agents & ProxyAgent  
- CST & CDU concepts  
- Ecosystem architecture  
- Vision statement  

It follows **Doc 1’s semantic versioning, auditability, and interoperability rules**, while being governed under the **BaDaaS License** in Doc 2.

---

## 1. Vision
To create a **vibe coding ecosystem** that eases beginner onboarding into the Rust community. The cdqnLang language provides a **high-abstraction layer** above Rust, allowing users to gradually transition from natural language (via ProxyAgent) to cdqnLang, and finally to raw Rust.

---

## 2. Core Components

### 2.1 cdqnCore
- **CST (Causal System Timer)**: Hybrid Logical Clock tuned for cdqn, including:
  - Time (1 epoch = 1 Earth year)  
  - Space (timezone triangulation)  
  - Device signature (ephemeral-key-based hash)  
- **CDU (Context Data Unit)**: Immutable atomic data entity.
- **cduModel**: Linked CDUs forming logical or functional structures.

### 2.2 cdqnLang
- Strongly typed, actor-model-based DSL derived from Rust.  
- **Syntax Rules:**
  - Assignment: `≔`  
  - Equality: `=`  
  - Flow operator: `:>>` (also replaces nested parentheses)  
  - Superscript notation for powers (`²`)  
  - Unicode math set (∑, ∏, ∫, √, ≤, ≥, etc.)  
  - Explicit typing for all variables and actors.  
- **Actors:**
  - **cdqnSys**: Predefined system actors for ecosystem needs.  
  - **agent**: Metatype tag for all autonomous actors.

### 2.3 cdqnRuntime
- WASM/WASI-based runtime executing cdqnLang code.
- Responsibilities:
  - Actor lifecycle and concurrency  
  - Guardrails for safety and legal compliance  
  - Offline-first operation with hybrid online support  
  - Integration with cdqnDB

### 2.4 cdqnDB
- Immutable graph/event-sourced database.  
- Disk-prioritized I/O with efficient persistence model.  
- Stores CDUs in content-addressable DAG format.

### 2.5 Agents
- Autonomous actors tagged with `agent`.  
- Language-model-backed (transformer, HRM, etc.)  
- Guardrails mandatory for lawful and safe execution.

### 2.6 ProxyAgent
- 1:1 with each user.  
- Translates natural language to cdqnLang.  
- Applies safety checks and persists interactions.

---

## 3. Architecture

User (Natural Language) ↓ ProxyAgent (NL → cdqnLang) ↓ cdqnRuntime (executes actors) ↓ cdqnDB (CDUs stored immutably)

- **Hybrid P2P** network with offline-first priority.
- **Hard drive over RAM** emphasis for persistence.

---

## 4. Governance & Security
- No anonymous nodes.
- All data is node-owned and signed with ephemeral keys.
- Guardrails at input, processing, and output stages.

---

## 5. Interoperability Guidelines
- All logs and events convertible to JSON/XML.
- Strict semantic versioning (V1.0.0 → V1.x.x).
- Changes tracked per **Doc 1** standards.

---

## License
This document and its content are licensed under the **BaDaaS License – The Agile Commercial Open-Core License (Doc 2)**.  
See **Doc 2** for permitted uses, attribution requirements, and partnership triggers.

---
