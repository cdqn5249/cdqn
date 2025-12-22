# 02f-SECURITY: The Roadmap for Securing Verifiable Experience

*   **File:** `docs/research/legacy/02f-SECURITY.md`
*   **Context:** Theoretical Canon (Security Strategy & The 3 Tiers of Trust)
*   **Date:** December 14, 2025
*   **Status:** `v2.1` (The Asset Protection Standard)

> **The Chain of Custody for Human Value.**
> *As defined in `01a-COHESION`, the CDQN Lattice is the user's **Living Ledger**—their most valuable economic asset in the AI era. This document outlines the security roadmap required to protect that asset. We define three tiers of sovereignty, moving from "Best Effort" on compromised systems to a future vision of a fully attested hardware/software stack. This roadmap is our commitment to underwriting the economic value of your life's experience.*

---

## 1. The Premise: Verifying the Ledger

The value of the Living Ledger rests on its integrity. Our security promise is not "This system is unbreakable." Our promise is: **"The history of this Ledger is verifiable."**

We use a **Merkle Tree of Reputation** (to be specified in `03e-WORLDS`) to create a "Chain of Custody" for all system components, ensuring the user can always verify the provenance of the software that manages their experience.

---

## 2. Tier 1: Accountability on Untrusted Platforms

*   **Corresponds To:** `libcdqn` running on Windows, Android, Linux.
*   **Asset Value:** **"Draft" Experience.** Useful for personal growth and low-stakes collaboration.
*   **Promise:** **Accountability & Privacy.**
*   **Security Context:** We assume the Host OS could be compromised (a "God Mode" attacker at the OS level).
    *   **Defense:** We can't stop the attacker from reading memory. But because our history is an **Ouroboros Ratchet** (`02b-PHYSICS`), the attacker cannot *silently rewrite the past*. They can steal the data, but the cryptographic journal remains a valid record of what happened up to the point of compromise.
    *   **Attestation:** If the OS is pirated, `libcdqn` can detect this via heuristic checks. It cannot stop the OS, but it can **warn the user**. The UI will display "Red Alert: Running on Untrusted Substrate."
*   **Conclusion:** Tier 1 provides a robust "drafting table" for your experience, protected from casual fraud and peer-level snooping.

---

## 3. Tier 2: Sovereignty on Trusted Platforms

*   **Corresponds To:** `cdqnOS` running on commodity hardware with a TEE/TPM.
*   **Asset Value:** **"Notarized" Experience.** Secure enough for job applications, contracts, and high-stakes proof of skill.
*   **Promise:** **Sovereignty.**
*   **Security Context:** The **Sovereign Rituals** are active. The TEE protects the `GenesisSeed` from all software-level attacks. The ledger is protected from forgery.
    *   **Defense:** The TEE enforces the Time-Locks (`Hysteresis`) and Physical Presence checks (`Totem`) defined in the early research. Even a Rootkit cannot force a key export or instant wipe.
*   **The Gap:** This tier relies on trust in the TEE manufacturer. A hardware backdoor could compromise the notarization.
*   **Conclusion:** Tier 2 provides a cryptographically secure ledger, vulnerable only to nation-state level supply chain attacks or physical hardware disassembly.

---

## 4. Tier 3: The Vision - Verifiable Integrity (The Full Stack)

*   **Corresponds To:** A future `cdqnOS` running on **Certified Open Hardware** (e.g., a RISC-V board with a verifiable TEE implementation).
*   **Asset Value:** **"Gold Standard" Experience.** An audited, physically-backed, and continuously verified record of expertise. The highest level of trust.
*   **Promise:** **Verifiable End-to-End Integrity.**
*   **Mechanism:** **Remote Attestation.**
    *   At boot, the TEE creates a cryptographic hash of the entire software stack. It signs this measurement with a private key burned into the silicon. This is the **"Hardware and Software Binding."**
    *   The user's device can be challenged by user-chosen "Guardians." The device provides its signed attestation report. The Guardian checks the report against the "known good" measurements from the official CDQN repository.
    *   If a single bit of the kernel has been tampered with, the signature will not match. The system's integrity is continuously proven, not just promised.
*   **Conclusion:** Tier 3 is the ultimate goal. It closes the Supply Chain gap by replacing "Trust" with continuous **Verification**. It creates a true "Glass Box" where the economic value of your experience is underwritten by provable physics.

---

## 5. Security Roadmap Summary

| Tier | Platform | Asset Value | Defense Against | Known Vulnerability |
| :--- | :--- | :--- | :--- | :--- |
| **1** | `libcdqn` on Host OS | Draft Experience | Peer-level attacks | Host OS Compromise (Root) |
| **2** | `cdqnOS` on TEE | Notarized Experience | All Software Attacks | Hardware Supply Chain Attack |
| **3** | `cdqnOS` on Open HW | Gold Standard | Software & Detected HW Attacks | Undetected/Novel Hardware Attacks |

This roadmap now serves as the honest and complete security doctrine for protecting the most valuable asset the user will own.

---

**License:** Universal Sovereign Source License (USSL) v2.0.
