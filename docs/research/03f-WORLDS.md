# 03f-WORLDS: The Context & Reputation API

*   **File:** `docs/research/03f-WORLDS.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 5 Specification (Spectral Filters, Merkle Reputation, & The Membrane)
*   **Date:** December 12, 2025
*   **Status:** `v1.0 - Specification Draft`

> **The Lens of Reality.**
> *Layers 1-4 defined the internal physics of the machine. Layer 5 defines its relationship with the outside world. This API implements **Resonance Theory (`02f`)**: treating Contexts not as separate databases, but as **Spectral Filters** that allow or block data based on its frequency (Logic) and its source (Reputation). Crucially, it establishes the **Merkle Tree of Lineage**, allowing users to verify the provenance of their hardware and software without relying on a central authority.*

---

## 1. The World Mandate

1.  **Filter, Don't Segregate:** A "World" is a mask applied to the vector stream, blocking incompatible concepts (e.g., "Magic" in "Reality").
2.  **Verify, Don't Trust:** Identity is verified via a **Merkle Tree of Signatures**.
3.  **Reputation is Local:** The User decides which "Root Keys" (Manufacturers/Devs) they trust. The system enforces that trust.
4.  **Ancestry is Public:** Code provenance must be signed. Anonymity is allowed for users, but *forbidden* for system distributors.

---

## 2. Core Data Structures

### 2.1 The World Filter ( The Lens)
A bitmask that defines which dimensions and logic types are active.

```rust
pub struct WorldFilter {
    pub allow_magic: bool,       // High-Frequency/Low-Mass logic allowed?
    pub allow_mutation: bool,    // Can Deep Constants be melted?
    pub gravity: f32,            // Global Mass Multiplier (Reality = High)
    pub trusted_roots: MerkleRoot, // The whitelist of allowed authors
}
```

### 2.2 The Lineage Tree (The Chain of Custody)
Instead of a central list, we use a Merkle Tree stored in the Deep Lattice.
*   **Leaf:** A Developer/Manufacturer Public Key + Reputation Score.
*   **Branch:** A Consortium or Group (e.g., "CDQN Official", "Open Source Collective").
*   **Root:** The User's "Trust Anchor" (set at Genesis).

```rust
pub struct LineageNode {
    pub pub_key: [u8; 32],
    pub reputation_mass: u32,
    pub parent_signature: [u8; 64],
}
```

---

## 3. The Resonance API (The Gatekeeper)

This replaces "World Logic" with "Signal Processing."

### 3.1 The Input Filter
When data arrives (from a sensor, a network peer, or a code update), it must pass the filter.

```rust
/// Checks if an input resonates with the current World Context.
///
/// @param input: The incoming concept/code.
/// @param context: The active World Filter.
/// @return ResonanceFactor (0.0 to 1.0).
/// - 0.0: Blocked (Noise/Lies).
/// - 1.0: Accepted (Clear Signal).
fn lvm_check_resonance(
    input: &VectorBody, 
    context: &WorldFilter
) -> f32;
```
**Logic:**
1.  **Frequency Check:** Does the input violate Deep Constants? (e.g., $1+1=3$). If yes, Resonance = 0.
2.  **Origin Check:** Is the input signed by a key inside `context.trusted_roots`?
    *   If Yes: Resonance = 1.0.
    *   If No: Resonance = 0.1 (Noise).

### 3.2 The Membrane Transition
Moving data between worlds (e.g., Game $\to$ Reality).

```rust
/// Attempts to migrate a concept from Source World to Target World.
///
/// @param concept: The vector to move.
/// @param target: The destination filter.
/// @return Result<Success, Plasma>
fn lvm_membrane_pass(
    concept: &VectorBody, 
    target: &WorldFilter
) -> Result<Success, Plasma>;
```
**Logic:**
If `concept.mass * target.gravity` is too low, the concept evaporates. This prevents "Light" fiction from contaminating "Heavy" reality.

---

## 4. The Lineage API (Reputation Management)

This implements your directive: **"We verify sources."**

### 4.1 Signature Verification
Every critical update or hardware handshake must present a **Chain of Trust**.

```rust
/// Verifies that a code module or hardware component is signed by a recognized entity.
///
/// @param artifact_hash: Hash of the code/hardware ID.
/// @param signature: The crypto signature.
/// @param signer_key: The public key of the author.
/// @return true if signer is found in the local Merkle Tree.
fn lvm_verify_lineage(
    artifact_hash: [u8; 32],
    signature: [u8; 64],
    signer_key: [u8; 32]
) -> bool;
```

### 4.2 The Genesis Block (The CDQN Seed)
The LVM ships with a "Bootstrap Tree" containing the **CDQN Genesis Key**.
*   **The Promise:** "This code came from the CDQN Repository."
*   **The User Right:** The User can **prune** this tree. If they want to trust "Rebel Dev Corp" instead of "CDQN," they can replace the Root.
*   **Transparency:** The UI (Layer 9) will explicitly display:
    *   *"System Signed by: CDQN Official"* (Green)
    *   *"System Signed by: Unknown Dev"* (Red/Warning)

---

## 5. Implementation Notes for `libcdqn`

*   **No "Backdoor" Keys:** The CDQN Genesis Key is for **Identity Verification**, not Decryption. We can prove we wrote the code, but we cannot read the user's data.
*   **Immutable Logs:** Using the Ouroboros Ratchet (Layer 3), every update to the `trusted_roots` is logged. An attacker cannot silently add a malicious key; they must fork the history, which the user (or TEE) will detect.
*   **Hardware Binding:** On Tier 2 (Sovereign), the `trusted_roots` are pinned in the TEE. Modifying them requires the **Totem Ritual** (`02e`), preventing remote supply-chain attacks from silently swapping the OS.

---

### **Appendix: Test Vector Specification**

`docs/validation/03f_worlds.check`

**Example Test Case (`check/lineage.test`):**
```
// Test Case: Unsigned Code is blocked by Reality Filter.

// 1. GIVEN: A World Filter requiring "CDQN_ROOT" signature.
let filter = WorldFilter { trusted_roots: CDQN_MERKLE_ROOT, ... };

// 2. WHEN: An unsigned "Update Vector" tries to enter.
let unsigned_update = VectorBody { signature: None, ... };
let resonance = lvm_check_resonance(&unsigned_update, &filter);

// 3. THEN: Resonance is near zero. The update is ignored.
ASSERT resonance < 0.1;
```

**License:** Universal Sovereign Source License (USSL) v2.0.
