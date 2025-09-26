# K.CryptoCore

* **Module Path:** `src/kernel/crypto/`
* **Version:** 1.0.0
* **Date:** 26 September 2025
* **License:** **BaDaaS License**
* **Vibe Coding Engine:** Gemini Flash-Lite Latest, Google

## 1. Definition and Purpose

The **`K.CryptoCore`** is the **Kernel's (Layer 1) isolated, foundational component** and the **Sovereign Root of Trust** for every cdqn Node. It is the first module validated under the new lean design.

Its primary purpose is to enforce **Tenet 7 (Forward-Secret Cryptography)** of the cdqn manifesto, by providing secure, auditable mechanisms for **Identity Creation, Data Signing, and Signature Verification** for all `*.kdu` and `*.cdqn` files.

### Key Architectural Principle: Pure Logic Delegation

This module strictly separates its concerns to maximize testability and security:

1.  **`CryptoCoreEngine` (Stateless):** Contains the **pure, deterministic cryptographic computation** (hashing, signing primitives). This is the *logic* part of the Vibe Coding Engine's model.
2.  **`CryptoCoreManager` (Stateful):** Manages the sovereign identity's **`PrivateKey`** and delegates all computation to the Engine, ensuring the state remains secure and isolated.

## 2. Core Functionality

### A. Identity Management (Sovereignty)

*   **Creation:** A new sovereign identity is established by generating an **Ed25519 keypair** via the stateless engine.
*   **State:** The **`CryptoCoreManager`** holds the private key, making the node the sole authority over its signing capability, fulfilling the **Data Sovereignty** goal.

### B. Data Integrity (Immutability)

*   **Signing:** The `CryptoCoreManager::sign()` method is the only path to bind data to an identity. It ensures that every signed output (like a KDU header) is cryptographically linked to its originator.
*   **Verification:** The system relies on the public key derived from the manager to verify signatures, ensuring **Accountability (Tenet 4)** for every recorded event.

## 3. Source Code Paths for Reference

The following paths detail the validated, lean implementation of the **`K.CryptoCore`**:

| File | Purpose |
| :--- | :--- |
| `Cargo.toml` | Defines dependencies, confirming the use of secure crates like **`ed25519-dalek`**. |
| `src/types.rs` | Defines strongly-typed structures and the custom **`CryptoError`** enum, reflecting the new design's mandate for **Strict Error Handling**. |
| `src/engine.rs` | Contains the **pure, stateless** cryptographic primitives. |
| `src/manager.rs` | Contains the **stateful `CryptoCoreManager`** that holds the keys and orchestrates operations. |
| `src/lib.rs` | The module entry point. |
